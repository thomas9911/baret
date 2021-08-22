use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

use serde_yaml::from_reader;
use structopt::StructOpt;

use futures::stream::{self, StreamExt};
use tokio::runtime::Builder;

use indicatif::ProgressBar;

use baret_lib::command;
use baret_lib::Data;

const PROGRESS_BAR_COLOR_TEMPLATE: &'static str =
    "[{elapsed_precise}] {pos:.cyan.bold.bright}/{len:.white.bold.bright} {bar:.cyan/blue}";
const PROGRESS_BAR_TEMPLATE: &'static str = "[{elapsed_precise}] {pos}/{len} {bar}";

#[derive(Debug, StructOpt)]
#[structopt(name = "baret", about = "Bash and Rust End-to-end Testing.")]
struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str), default_value = "baret.yaml")]
    config: PathBuf,

    /// Verify the input file
    #[structopt(long)]
    verify: bool,

    /// Output example config
    #[structopt(long)]
    example: bool,

    /// dont show the progress bar
    #[structopt(short, long)]
    quiet: bool,

    /// enable colors in the progress bar
    #[structopt(long)]
    color: bool,
}

fn main() {
    let opt = Opt::from_args();

    if opt.example {
        let example =
            serde_yaml::to_string(&Data::dump_example()).expect("unable to create example");
        println!("{}", example);
        return;
    }

    let file = match File::open(&opt.config) {
        Ok(x) => x,
        Err(_) => {
            eprintln!("config {} not found.", opt.config.display());
            std::process::exit(1);
        }
    };

    let data: Data = match from_reader(BufReader::new(file)) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if opt.verify {
        return;
    }

    let pb = create_progression_bar(&opt, data.test.len() as u64);
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    match runtime.block_on(main_loop(data, pb)) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn create_progression_bar(opt: &Opt, amount_of_tasks: u64) -> ProgressBar {
    let pb = if opt.quiet {
        indicatif::ProgressBar::hidden()
    } else {
        indicatif::ProgressBar::new(amount_of_tasks)
    };
    // pb.set_style(
    //     indicatif::ProgressStyle::default_bar()
    //         .template("[{elapsed_precise}] {pos}/{len} [{bar:.cyan/blue}]")
    //         .progress_chars("#>-"),
    // );
    if opt.color {
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(PROGRESS_BAR_COLOR_TEMPLATE)
                .progress_chars("█░░"),
        );
    } else {
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(PROGRESS_BAR_TEMPLATE)
                .progress_chars("█░░"),
        );
    }
    pb.set_draw_delta(amount_of_tasks / 100);
    pb
}

async fn main_loop(data: Data, pb: ProgressBar) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(result) = command::pre_setup(&data).await {
        result?
    }

    let global_settings = Arc::new(data.global.clone());

    let tests = data.test.clone();
    let mut tasks = stream::iter(tests)
        .map(|(test_name, test)| {
            let global_settings = global_settings.clone();
            tokio::spawn(async move {
                match test.run_arc_settings(global_settings).await {
                    Ok(()) => false,
                    Err(err) => {
                        eprintln!("Failed test: '{}'", test_name);
                        eprintln!("{}", err);
                        true
                    }
                }
            })
        })
        .buffer_unordered(global_settings.max_test_concurrency());

    let mut successes = 0usize;
    let mut errors = 0usize;
    while let Some(task) = tasks.next().await {
        if task? {
            errors += 1;
        } else {
            successes += 1;
            pb.inc(1);
        }
    }
    pb.finish();

    if let Some(result) = command::post_setup(&data).await {
        result?
    }

    if errors != 0 {
        Err(format!(
            "Error: {} {} had errors out of {} {}",
            errors,
            test_or_tests(errors),
            successes + errors,
            test_or_tests(successes + errors)
        )
        .into())
    } else {
        Ok(())
    }
}

fn test_or_tests(amount: usize) -> &'static str {
    if amount == 1 {
        "test"
    } else {
        "tests"
    }
}
