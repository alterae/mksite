//! Config file generation, parsing, and loading.

use std::{collections::HashMap, fs, io, path};

use thiserror::Error;

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
    pub dirs: Dirs,
    /// Data to be passed to template rendering.
    pub data: HashMap<String, toml::Value>,
    /// The list of transforms to apply, stored as a map of input formats to
    /// sub-maps of output formats and transforms.
    pub transforms: HashMap<String, HashMap<String, transform::Transform>>,
}

/// The names of all the important directories needed to build a site.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Dirs {
    /// The src directory holds template files to be rendered, transformed, and
    /// inserted into layouts.
    pub src: String,
    /// The out directory is where generated content goes.
    pub out: String,
    /// Files in the static directory are copied as-is to the out directory.
    pub r#static: String,
    /// The layout directory is where layout files are stored.
    pub layout: String,
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

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to deserialize config: {0}")]
    Deserialization(#[from] toml::de::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Returns true if the `mksite.toml` config file exists in the current directory.
pub fn exists() -> bool {
    path::Path::new(FILE_NAME).exists()
}

/// Loads the `mksite.toml` config file from the current directory.
pub(crate) fn load() -> Result<Config> {
    toml::from_str(&fs::read_to_string(FILE_NAME)?).map_err(Error::from)
}

/// Generates the `mksite.toml` config file in the specified directory.
/// `path` must be a directory.
///
/// The contents of this file are copied verbatim from `mksite.default.toml`
/// via `include_str`.
pub(crate) fn generate(path: &impl AsRef<path::Path>) -> anyhow::Result<()> {
    anyhow::ensure!(
        fs::metadata(path)?.is_dir(),
        "{:?} is not a directory",
        path.as_ref()
    );

    anyhow::ensure!(
        !path.as_ref().join(FILE_NAME).exists(),
        "Config file {FILE_NAME} already exists"
    );

    fs::write(
        path.as_ref().join(FILE_NAME),
        include_str!("../mksite.default.toml"),
    )?;

    Ok(())
}
