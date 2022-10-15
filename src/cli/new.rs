//! The `mksite new` subcommand.

use std::{fs, io, path::Path};

use crate::config;

/// Creates a new directory with a given name, and then initializes a basic
/// project structure within, containing a `mksite.toml` file and a `src/`,
/// `static/`, and `layout/` directories.
pub(crate) fn cmd(name: String) -> io::Result<()> {
    fs::create_dir(&name)?;

    log::info!("Creating new project scaffold in {name}");

    fs::create_dir(Path::new(&name).join("src"))?;
    fs::create_dir(Path::new(&name).join("static"))?;
    fs::create_dir(Path::new(&name).join("layout"))?;

    config::generate(&Path::new(&name))
}
