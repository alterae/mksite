//! The `mksite new` subcommand.

use std::{fs, path::Path};

use anyhow::Context;

use crate::config;

/// Creates a new directory with a given name, and then initializes a basic
/// project structure within, containing a `mksite.toml` file and a `src/`,
/// `static/`, and `layout/` directories.
pub(crate) fn cmd(name: String) -> anyhow::Result<()> {
    fs::create_dir(&name)
        .with_context(|| format!("Cannot create directory {name} as it already exists"))?;

    log::info!("Creating new project scaffold in {name}...");

    fs::create_dir(Path::new(&name).join("src"))?;
    fs::create_dir(Path::new(&name).join("static"))?;
    fs::create_dir(Path::new(&name).join("layout"))?;

    config::generate(&Path::new(&name))
}
