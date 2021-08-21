use std::process::Output;

use tokio::io;

use derive_more::From;

pub type Result = std::result::Result<(), Error>;

#[derive(Debug, From)]
pub enum Error {
    IO(io::Error),
    ExitCode(Output),
}
