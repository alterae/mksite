//! The `mksite clean` subcommand.

use std::{fs, io};

use crate::config;

/// Deletes the `out/` directory and all its contents.
pub(crate) fn cmd() -> config::Result<()> {
    let config = config::load()?;

    log::info!("Removing \"{}\"", config.dirs.out);

    fs::remove_dir_all(&config.dirs.out).or_else(|e| match e.kind() {
        io::ErrorKind::NotFound => {
            log::warn!("Cannot remove \"{}\": {e}", config.dirs.out);
            Ok(())
        }
        _ => Err(e.into()),
    })
}
