//! Config file generation, parsing, and loading.

use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path,
};

use crate::Result;

use crate::transform;

/// The name of the config file to use.
pub(crate) const FILE_NAME: &str = "mksite.toml";

/// The configuration for a `mksite` project.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Config {
    /// The list of important directories.
    ///
    /// FIXME: The whole key can be omitted, but if any of them are specified
    /// manually, all of them have to be.
    #[serde(default)]
    pub(crate) dirs: Dirs,
    /// Data to be passed to template rendering.
    pub(crate) data: HashMap<String, toml::Value>,
    /// The list of transforms to apply, stored as a map of input formats to
    /// sub-maps of output formats and transforms.
    pub(crate) transforms: HashMap<String, HashMap<String, transform::Transform>>,
}

/// The names of all the important directories needed to build a site.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Dirs {
    /// The src directory holds template files to be rendered, transformed, and
    /// inserted into layouts.
    pub(crate) src: String,
    /// The out directory is where generated content goes.
    pub(crate) out: String,
    /// Files in the static directory are copied as-is to the out directory.
    pub(crate) r#static: String,
    /// The layout directory is where layout files are stored.
    pub(crate) layout: String,
}

impl Default for Dirs {
    fn default() -> Self {
        Self {
            src: "src".into(),
            out: "out".into(),
            r#static: "static".into(),
            layout: "layout".into(),
        }
    }
}

/// Loads the `mksite.toml` config file from the current directory.
pub(crate) fn load() -> Result<Config> {
    toml::from_str(&fs::read_to_string(FILE_NAME)?).map_err(|e| e.into())
}

/// Generates the `mksite.toml` config file in the specified directory.
/// `path` must be a directory.
///
/// The contents of this file are copied verbatim from `mksite.default.toml`
/// via `include_str`.
pub(crate) fn generate(path: &path::Path) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path.join(FILE_NAME))?;

    file.write_all(include_str!("../mksite.default.toml").as_bytes())
}
