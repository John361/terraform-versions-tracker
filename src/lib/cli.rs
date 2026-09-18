use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    name = "terraform-versions-tracker",
    bin_name = "terraform-versions-tracker"
)]
pub struct Cli {
    #[arg(long = "path", required = true, help = "Base path to scan (required)")]
    pub path: PathBuf,
}

impl Cli {
    pub fn load() -> Self {
        Self::try_load_from(std::env::args_os()).unwrap_or_else(|e| e.exit())
    }

    pub fn try_load_from<I, T>(args: I) -> Result<Self, clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        Cli::try_parse_from(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;

    #[test]
    fn test_cli_load_success() {
        let args = vec!["vault-secrets", "--path", "/tmp"];

        let cli = Cli::try_load_from(args).unwrap();
        assert_eq!(cli.path, PathBuf::from("/tmp"));
    }

    #[test]
    fn test_cli_load_error() {
        let args = vec!["vault-secrets", "--paths", "/tmp"];

        let err = Cli::try_load_from(args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnknownArgument);
    }
}
