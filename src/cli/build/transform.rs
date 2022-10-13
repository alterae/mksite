//! Transforms and their application.

use std::{
    io::Write,
    process::{Command, Stdio},
};

/// A transform is a command or pipeline of command for transforming content.
/// Transforms take an input string on standard input and return an output
/// string on standard output.
#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum Transform {
    /// A transform with only one command.
    ///
    /// ## Example
    /// ```toml
    /// [transforms]
    /// md.html = "pandoc -f markdown -t html"
    /// ```
    Single(String),
    /// A transforms with multipe commands. The output of each command is piped
    /// as the input to the next.
    ///
    /// ## Example
    /// ```toml
    /// [transforms]
    /// scd.html = [ "scdoc", "pandoc -f " ]
    Chain(Vec<String>),
}

// TODO: move to own module
impl Transform {
    pub(crate) fn apply(&self, input: &[u8]) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::Single(command) => exec(input.into(), command),
            Self::Chain(commands) => commands.iter().try_fold(input.into(), exec),
        }
    }
}

// TODO: move to own module
pub(crate) fn exec(input: Vec<u8>, command: &String) -> anyhow::Result<Vec<u8>> {
    println!("    Applying `{command}'...");

    let argv = shell_words::split(command)?;

    let mut proc = Command::new(&argv[0])
        .args(&argv[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    proc.stdin.take().unwrap().write_all(&input)?;

    Ok(proc.wait_with_output()?.stdout)
}
