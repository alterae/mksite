//! Transforms and their application.

use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::{Error, Result};

/// A transform is a command or pipeline of command for transforming content.
/// Transforms take an input string on standard input and return an output
/// string on standard output.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
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

impl Transform {
    /// Tries to apply this transform to the given input, and returns the
    /// output.
    pub(crate) fn apply(&self, input: &[u8]) -> Result<Vec<u8>> {
        match self {
            Self::Single(command) => exec(input.into(), command),
            Self::Chain(commands) => commands.iter().try_fold(input.into(), exec),
        }
    }
}

impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "`{}'",
            match self {
                Transform::Single(cmd) => cmd.clone(),
                Transform::Chain(cmds) => cmds.join(" | "),
            }
        )
    }
}

/// Tries to run a shell command with the given input, and returns the output.
pub(crate) fn exec(input: Vec<u8>, command: &String) -> Result<Vec<u8>> {
    let argv = shell_words::split(command).map_err(|source| Error::Shell {
        command: command.clone(),
        source,
    })?;

    log::debug!("Runnging {argv:?}");

    let mut proc = Command::new(&argv[0])
        .args(&argv[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|source| Error::Io {
            msg: format!("Cannot run `{command}'"),
            source,
        })?;

    proc.stdin
        .take()
        .expect("Child process stdin was None, which should be impossible")
        .write_all(&input)
        .map_err(|source| Error::Io {
            msg: format!("Cannot pipe to `{command}'"),
            source,
        })?;

    let output = proc.wait_with_output().map_err(|source| Error::Io {
        msg: format!("Error while waiting on `{command}'"),
        source,
    })?;

    Ok(output.stdout)
}
