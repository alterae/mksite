//! The `mksite clean` subcommand.

use std::{fs, io};

use crate::{config, Result};

/// Deletes the `out/` directory and all its contents.
pub(crate) fn cmd() -> Result<()> {
    let config = config::load()?;

    log::info!("Removing '{}/'", config.dirs.out);

    fs::remove_dir_all(&config.dirs.out).or_else(|e| match e.kind() {
        io::ErrorKind::NotFound => {
            log::warn!("Cannot remove \"{}\": {e}", config.dirs.out);
            Ok(())
        }
        _ => Err(crate::Error::Io {
            msg: format!("Cannot remove \"{}\"", config.dirs.out),
            source: e,
        }),
    })
}
