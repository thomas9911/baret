use std::process::Output;

use tokio::io;
use tokio::process::Command;

use crate::error::Result;
use crate::Error;

use crate::Data;
use crate::SettingsStack;

struct CommandBuilder<'a> {
    function: &'a str,
}

impl<'a> CommandBuilder<'a> {
    fn new(function: &'a str) -> CommandBuilder<'a> {
        CommandBuilder { function }
    }

    async fn run(self, settings: &SettingsStack<'_, '_>) -> io::Result<Output> {
        let (program, program_args) = settings.command_with_args();
        let mut command = Command::new(program);

        if settings.clear_env() {
            command.env_clear();
        }

        command.envs(settings.env());
        command.args(program_args);
        command.arg(self.function);
        command.output().await
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => write!(f, "{}", error),
            Error::ExitCode(error) => {
                writeln!(f, "{}", &error.status)?;
                writeln!(
                    f,
                    "stdout:\n{}",
                    String::from_utf8(error.stdout.clone())
                        .unwrap_or(String::from("not utf8 string"))
                )?;
                writeln!(
                    f,
                    "stderr:\n{}",
                    String::from_utf8(error.stderr.clone())
                        .unwrap_or(String::from("not utf8 string"))
                )
            }
        }
    }
}

impl std::error::Error for Error {}

pub async fn pre_setup(data: &Data) -> Option<Result> {
    if let Some(before_all) = &data.setup.before_all {
        let settings = data.global.stack(&[]);
        return Some(run(before_all, &settings).await.into());
    }

    None
}

pub async fn post_setup(data: &Data) -> Option<Result> {
    if let Some(after_all) = &data.setup.after_all {
        let settings = data.global.stack(&[]);
        return Some(run(after_all, &settings).await.into());
    }

    None
}

pub async fn run<'a, 'b>(command: &str, settings: &SettingsStack<'a, 'b>) -> Result {
    // let result = Command::new(program)
    //     .args(program_args)
    //     .arg(command)
    let result = CommandBuilder::new(command).run(settings).await;

    match result {
        Ok(Output { status, .. }) if is_success(settings.should_fail(), status.success()) => Ok(()),
        Ok(error) => Err(Error::ExitCode(error)),
        Err(err) => Err(Error::IO(err)),
    }
}

fn is_success(inverse_result: bool, success: bool) -> bool {
    match (inverse_result, success) {
        (false, success) => success,
        (true, success) => !success,
    }
}
