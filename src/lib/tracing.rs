pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("changeme=warn".parse().unwrap()),
        )
        .init();

    tracing::debug!("Tracing initialized");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_init_tracing() {
        let _ = panic::catch_unwind(init_tracing);
    }
}
