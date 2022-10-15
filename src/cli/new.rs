//! The `mksite new` subcommand.

use std::{fs, path::Path};

use crate::{config, Error};

/// Creates a new directory with a given name, and then initializes a basic
/// project structure within, containing a `mksite.toml` file and a `src/`,
/// `static/`, and `layout/` directories.
pub(crate) fn cmd(name: String) -> crate::Result<()> {
    fs::create_dir(&name).map_err(|source| Error::Io {
        msg: format!("Cannot create {name}/src"),
        source,
    })?;

    log::info!("Creating new project scaffold in {name}");

    for dir in ["src", "static", "layout"] {
        let path = Path::new(&name).join(dir);
        fs::create_dir(path).map_err(|source| Error::Io {
            msg: format!("Cannot create {name}/{dir}"),
            source,
        })?;
    }

    config::generate(Path::new(&name))
}
