use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use hcl::{Body, Expression};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct ProviderVersion {
    pub name: String,
    pub source: String,
    pub version: String,
    pub path: PathBuf,
}

impl ProviderVersion {
    fn group_by_source(values: Vec<ProviderVersion>) -> HashMap<String, Vec<ProviderVersion>> {
        let mut map: HashMap<String, Vec<ProviderVersion>> = HashMap::new();

        for pv in values {
            map.entry(pv.source.clone()).or_default().push(pv);
        }

        map
    }
}

pub struct Collector {
    base_path: PathBuf,
}

impl Collector {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub fn collect(&self) -> Result<Vec<ProviderVersion>> {
        let mut result = Vec::new();

        for entry in WalkDir::new(&self.base_path) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && let Some(file_name) = path.file_name()
                && file_name.eq_ignore_ascii_case("versions.tf")
            {
                let items = self.extract(path)?;
                result.extend(items);
            }
        }

        Ok(result)
    }

    fn extract(&self, path: &Path) -> Result<Vec<ProviderVersion>> {
        let content =
            std::fs::read_to_string(path).context(format!("Failed to read {}", path.display()))?;
        let hcl: Body =
            hcl::from_str(&content).context(format!("Failed to parse hcl {}", path.display()))?;

        let mut result = Vec::new();

        for block in hcl.blocks() {
            if block.identifier() == "terraform" {
                for nested in block.body.blocks() {
                    if nested.identifier() == "required_providers" {
                        for attribute in nested.body().attributes() {
                            let name = attribute.key().to_string();

                            let Expression::Object(obj) = attribute.expr() else {
                                continue;
                            };

                            let mut source = String::new();
                            let mut version = String::new();

                            for (k, v) in obj.iter() {
                                match k.to_string().as_str() {
                                    "source" => source = v.to_string().replace("\"", ""),
                                    "version" => version = v.to_string().replace("\"", ""),
                                    _ => {}
                                }
                            }

                            result.push(ProviderVersion {
                                name,
                                source,
                                version,
                                path: path.to_path_buf(),
                            });
                        }
                    }
                }
            }
        }

        Ok(result)
    }
}
