use derive_more::From;
use glob::{GlobError, PatternError};
use std::process::Output;
use tokio::io;

pub type Result = std::result::Result<(), Error>;

#[derive(Debug, From)]
pub enum Error {
    IO(io::Error),
    ExitCode(Output),
    PatternError(PatternError),
    GlobError(GlobError),
}
