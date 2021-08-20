use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

use serde_yaml::from_reader;
use structopt::StructOpt;

use baret_lib::command;
use baret_lib::Data;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    if opt.example {
        let example =
            serde_yaml::to_string(&Data::dump_example()).expect("unable to create example");
        println!("{}", example);
        return Ok(());
    }

    let file = match File::open(&opt.config) {
        Ok(x) => x,
        Err(_) => return Err(format!("config {} not found.", opt.config.display()).into()),
    };

    let data: Data = match from_reader(BufReader::new(file)) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string().into()),
    };

    if opt.verify {
        return Ok(());
    }

    if let Some(result) = command::pre_setup(&data).await {
        result?
    }

    let global_settings = Arc::new(data.global.clone());
    let mut tasks = Vec::new();

    let tests = data.test.clone();
    for (test_name, test) in tests {
        let global_settings = global_settings.clone();
        let task = tokio::spawn(async move {
            match test.run_arc_settings(global_settings).await {
                Ok(()) => false,
                Err(err) => {
                    eprintln!("Failed test: '{}'", test_name);
                    eprintln!("{}", err);
                    true
                }
            }
        });
        tasks.push(task);
    }

    let mut had_error = false;
    for task in tasks {
        had_error |= task.await?;
    }

    if let Some(result) = command::post_setup(&data).await {
        result?
    }

    if had_error {
        Err("Some tests had errors".into())
    } else {
        Ok(())
    }
}
