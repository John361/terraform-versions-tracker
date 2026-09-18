use anyhow::Result;
use crate::tracing::init_tracing;

pub fn run() -> Result<()> {
    init_tracing();

    Ok(())
}
