use anyhow::Result;

use crate::cli::Cli;
use crate::collector::Collector;
use crate::tracing::init_tracing;

pub fn run() -> Result<()> {
    init_tracing();

    let cli = Cli::load();
    let collector = Collector::new(cli.path);
    let collect = collector.collect()?;

    println!("{:#?}", collect);

    Ok(())
}
