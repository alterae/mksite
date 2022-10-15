//! The `mksite init` subcommand.

use std::{io, path::Path};

use crate::config;

/// Generates a skeleton `mksite.toml` config file in the current directory.
pub(crate) fn cmd() -> io::Result<()> {
    log::info!("Writing default config file to ./{}", config::FILE_NAME);
    config::generate(Path::new("."))
}
