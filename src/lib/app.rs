use anyhow::Result;

use crate::cli::Cli;
use crate::collector::Collector;
use crate::printer::TablePrinter;
use crate::registry::RegistryClient;
use crate::tracing::init_tracing;

pub async fn run() -> Result<()> {
    init_tracing();

    let cli = Cli::load();

    let collector = Collector::new(cli.path);
    let collect = collector.collect()?;
    let sources = collect.keys().cloned().collect::<Vec<String>>();

    let registry_client = RegistryClient::new();
    let registry_responses = registry_client.do_requests(sources).await?;

    let printer = TablePrinter::default();
    let reports = printer.build_reports(collect, registry_responses);
    let table = printer.build_table(reports);

    println!("{table}");

    Ok(())
}
