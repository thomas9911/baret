use std::process::Output;

use tokio::process::Command;

use crate::error::Result;
use crate::Error;

use crate::Data;
use crate::SettingsStack;

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
    let (program, program_args) = settings.command_with_args();

    let result = Command::new(program)
        .args(program_args)
        .arg(command)
        .output()
        .await;

    match result {
        Ok(Output { status, .. }) if status.success() => Ok(()),
        Ok(error) => Err(Error::ExitCode(error)),
        Err(err) => Err(Error::IO(err)),
    }
}
