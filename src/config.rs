//! Config file generation, parsing, and loading.

use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{self, PathBuf},
};

use crate::{Error, Result};

use crate::transform;

/// The name of the config file to use.
pub(crate) const FILE_NAME: &str = "mksite.toml";

/// The configuration for a `mksite` project.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub(crate) struct Config {
    /// The list of important directories.
    #[serde(default)]
    pub(crate) dirs: Dirs,

    /// The list of pages to ignore in the templating, transforming, and layout
    /// steps.
    #[serde(default)]
    pub(crate) ignores: Ignores,

    /// Data to be passed to template rendering.
    #[serde(default)]
    pub(crate) data: HashMap<String, toml::Value>,

    /// The list of transforms to apply, stored as a map of input formats to
    /// sub-maps of output formats and transforms.
    #[serde(default)]
    pub(crate) transforms: HashMap<String, HashMap<String, transform::Transform>>,
}

/// The names of all the important directories needed to build a site.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Dirs {
    /// The src directory holds template files to be rendered, transformed, and
    /// inserted into layouts.
    #[serde(default = "Dirs::default_src")]
    pub(crate) src: PathBuf,

    /// The out directory is where generated content goes.
    #[serde(default = "Dirs::default_out")]
    pub(crate) out: PathBuf,

    /// Files in the static directory are copied as-is to the out directory.
    #[serde(default = "Dirs::default_static")]
    pub(crate) r#static: PathBuf,

    /// The layout directory is where layout files are stored.
    #[serde(default = "Dirs::default_layout")]
    pub(crate) layout: PathBuf,
}

// TODO: use git-style globs for ignore paths
// TODO: document in readme
/// The paths to files to be ignored during the templating, transform, and
/// layout steps.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub(crate) struct Ignores {
    /// Paths to source pages (eg `src/index.html`) to be ignored turing
    /// templating. Pages ignored this way will not be passed through Tera, and
    /// as such do not have to be valid UTF-8.
    #[serde(default)]
    pub(crate) template: Vec<PathBuf>,

    /// Paths to source pages (eg `src/index.html`) to be ignored during the
    /// transform step. Pages ignored this way will not be transformed, and
    /// their file extension will remain preserved, as if no transform were
    /// defined for them.
    #[serde(default)]
    pub(crate) transform: Vec<PathBuf>,

    /// Paths to _output_ pages (eg `out/index.html`) to be ignored during the
    /// layout step. Pages ignored this way will not have layouts applied to
    /// them, and will be written to the output directory as-is, as if no layout
    /// were defined from them. As a result, they do not have to be valid UTF-8.
    #[serde(default)]
    pub(crate) layout: Vec<PathBuf>,
}

impl Dirs {
    /// Returns the default 'src/' directory.
    fn default_src() -> PathBuf {
        "src".into()
    }

    /// Returns the default 'out/' directory.
    fn default_out() -> PathBuf {
        "out".into()
    }

    /// Returns the default 'static/' directory.
    fn default_static() -> PathBuf {
        "static".into()
    }

    /// Returns the default 'layout/' directory.
    fn default_layout() -> PathBuf {
        "layout".into()
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
