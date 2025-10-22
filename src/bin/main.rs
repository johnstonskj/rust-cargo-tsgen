use cargo_tsgen::{
    cli::{Cli, Command},
    error::Error,
};
use std::process::ExitCode;

// ------------------------------------------------------------------------------------------------
// Main Function
// ------------------------------------------------------------------------------------------------

fn main() -> Result<ExitCode, Error> {
    let args = Cli::parse_args();

    args.execute()
}
