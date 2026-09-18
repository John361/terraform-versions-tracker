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
                let item = self.extract(path)?;
                result.push(item);
            }
        }

        Ok(result)
    }

    fn extract(&self, path: &Path) -> Result<ProviderVersion> {
        let content =
            std::fs::read_to_string(path).context(format!("Failed to read {}", path.display()))?;
        let hcl: Body =
            hcl::from_str(&content).context(format!("Failed to parse hcl {}", path.display()))?;

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
                                    "source" => source = v.to_string(),
                                    "version" => version = v.to_string(),
                                    _ => {}
                                }
                            }

                            return Ok(ProviderVersion {
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

        Err(anyhow!("Could not find required providers"))
    }
}
