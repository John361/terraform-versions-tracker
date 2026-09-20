use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};

use crate::collector::ProviderVersion;
use crate::registry::RegistryProviderResponse;

pub struct ProviderReport {
    pub name: String,
    pub current_versions: Vec<String>,
    pub latest_version: String,
    pub paths: Vec<PathBuf>,
}

#[derive(Default)]
pub struct TablePrinter {}

impl TablePrinter {
    pub fn build_reports(
        &self,
        local: HashMap<String, Vec<ProviderVersion>>,
        registry: HashMap<String, RegistryProviderResponse>,
    ) -> Vec<ProviderReport> {
        let mut reports = Vec::new();

        for (source, versions) in local {
            let name = versions[0].name.clone();

            let current_versions: Vec<String> = versions
                .iter()
                .map(|v| v.version.clone())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

            let paths: Vec<PathBuf> = versions
                .iter()
                .map(|v| v.path.clone())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

            let latest = registry
                .get(&source)
                .map(|r| r.version.clone())
                .unwrap_or_else(|| "?".to_string());

            reports.push(ProviderReport {
                name,
                current_versions,
                latest_version: latest,
                paths,
            });
        }

        reports.sort_by(|a, b| a.name.cmp(&b.name));
        reports
    }

    pub fn build_table(&self, reports: Vec<ProviderReport>) -> Table {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Provider").add_attribute(Attribute::Bold),
                Cell::new("Version").add_attribute(Attribute::Bold),
                Cell::new("Paths").add_attribute(Attribute::Bold),
            ]);

        for report in reports {
            let current = report.current_versions.join(", ");

            let version_cell = if report
                .current_versions
                .iter()
                .all(|v| v == &report.latest_version)
            {
                Cell::new(format!("{} ✓", report.latest_version)).fg(Color::Green)
            } else {
                Cell::new(format!("{} → {}", current, report.latest_version)).fg(Color::Red)
            };

            let paths_cell = Cell::new(
                report
                    .paths
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            );

            table.add_row(vec![Cell::new(&report.name), version_cell, paths_cell]);
        }

        table
    }
}
