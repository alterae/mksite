//! Config file generation, parsing, and loading.

use std::{collections::HashMap, fs, io::Write, path};

use crate::{Error, Result};

use crate::transform;

/// The name of the config file to use.
pub(crate) const FILE_NAME: &str = "mksite.toml";

/// The configuration for a `mksite` project.
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub(crate) struct Config {
    /// The list of important directories.
    ///
    /// FIXME: The whole key can be omitted, but if any of them are specified
    /// manually, all of them have to be.
    #[serde(default)]
    pub(crate) dirs: Dirs,

    /// Data to be passed to template rendering.
    #[serde(default)]
    pub(crate) data: HashMap<String, toml::Value>,

    /// The list of transforms to apply, stored as a map of input formats to
    /// sub-maps of output formats and transforms.
    #[serde(default)]
    pub(crate) transforms: HashMap<String, HashMap<String, transform::Transform>>,
}

/// The names of all the important directories needed to build a site.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Dirs {
    /// The src directory holds template files to be rendered, transformed, and
    /// inserted into layouts.
    #[serde(default = "Dirs::default_src")]
    pub(crate) src: String,

    /// The out directory is where generated content goes.
    #[serde(default = "Dirs::default_out")]
    pub(crate) out: String,

    /// Files in the static directory are copied as-is to the out directory.
    #[serde(default = "Dirs::default_static")]
    pub(crate) r#static: String,

    /// The layout directory is where layout files are stored.
    #[serde(default = "Dirs::default_layout")]
    pub(crate) layout: String,
}

impl Dirs {
    /// Returns the default 'src/' directory.
    fn default_src() -> String {
        "src".to_owned()
    }

    /// Returns the default 'out/' directory.
    fn default_out() -> String {
        "out".to_owned()
    }

    /// Returns the default 'static/' directory.
    fn default_static() -> String {
        "static".to_owned()
    }

    /// Returns the default 'layout/' directory.
    fn default_layout() -> String {
        "layout".to_owned()
    }
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
    let config = fs::read_to_string(FILE_NAME).map_err(|source| Error::Io {
        msg: format!("Cannot read {FILE_NAME}"),
        source,
    })?;

    toml::from_str(&config).map_err(|source| source.into())
}

/// Generates the `mksite.toml` config file in the specified directory.
/// `path` must be a directory.
///
/// The contents of this file are copied verbatim from `mksite.default.toml`
/// via `include_str`.
pub(crate) fn generate(path: &path::Path) -> Result<()> {
    let file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path.join(FILE_NAME));

    let mut file = match file {
        Ok(file) => file,
        Err(source) => {
            return Err(Error::Io {
                msg: format!("Cannot create '{}'", path.display()),
                source,
            })
        }
    };

    file.write_all(include_str!("../mksite.default.toml").as_bytes())
        .map_err(|source| Error::Io {
            msg: format!("Cannot write '{}'", path.display()),
            source,
        })
}
