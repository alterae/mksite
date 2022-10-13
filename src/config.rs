//! Config file generation, parsing, and loading.

use std::{collections::HashMap, fs, path};

/// The name of the config file to use.
pub(crate) const FILE_NAME: &str = "mksite.toml";

/// The configuration for a `mksite` project.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Config {
    /// The list of important directories.
    pub dirs: Dirs,
    /// Data to be passed to template rendering.
    pub data: HashMap<String, toml::Value>,
    /// The list of processors to apply, stored as a map of input formats to
    /// sub-maps of output formats and processors.
    pub processors: HashMap<String, HashMap<String, Processor>>,
}

/// The names of all the important directories needed to build a site.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Dirs {
    /// The src directory holds template files to be rendered and processed.
    pub src: String,
    /// The out directory is where generated content goes.
    pub out: String,
    /// Files in the static directory are copied as-is to the out directory.
    pub r#static: String,
    /// The layout directory is where layout files are stored.
    pub layout: String,
}

/// A processor is a command or pipeline of command for transforming content.
/// Processors take an input string on standard input and return an output
/// string on standard output.
#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum Processor {
    /// A processor with only one command.
    ///
    /// ## Example
    /// ```toml
    /// [processors]
    /// md.html = "pandoc -f markdown -t html"
    /// ```
    Single(String),
    /// A processor with multipe commands. The output of each command is piped
    /// as the input to the next.
    ///
    /// ## Example
    /// ```toml
    /// [processors]
    /// scd.html = [ "scdoc", "pandoc -f " ]
    Chain(Vec<String>),
}

/// Returns true if the `mksite.toml` config file exists in the current directory.
pub fn exists() -> bool {
    path::Path::new(FILE_NAME).exists()
}

/// Loads the `mksite.toml` config file from the current directory.
pub(crate) fn load() -> anyhow::Result<Config> {
    let res = toml::from_str(&fs::read_to_string(FILE_NAME)?)?;
    Ok(res)
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
