//! The `mksite clean` subcommand.

use std::fs;

use anyhow::Context;

use crate::config;

/// Deletes the `out/` directory and all its contents.
pub(crate) fn cmd() -> anyhow::Result<()> {
    anyhow::ensure!(
        config::exists(),
        "Cannot clean site: {} not found",
        config::FILE_NAME
    );

    let config = config::load()?;

    println!("Removing \"{}\"...", config.dirs.out);

    fs::remove_dir_all(config.dirs.out).context("No build output to clean")
}
