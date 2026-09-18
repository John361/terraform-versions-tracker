use anyhow::Result;

use crate::cli::Cli;
use crate::tracing::init_tracing;

pub fn run() -> Result<()> {
    init_tracing();

    let _cli = Cli::load();

    Ok(())
}
