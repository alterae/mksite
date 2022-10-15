use std::{io, path};

// TODO: doc
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("{msg}: {source}")]
    Io { msg: String, source: io::Error },
    #[error(transparent)]
    Tera(#[from] tera::Error),
    /// FIXME: hardcoded filename
    #[error("Cannot deserialize mksite.toml: {0}")]
    Toml(#[from] toml::de::Error),
    #[error(transparent)]
    Log(#[from] log::SetLoggerError),
    #[error("Cannot parse `{command}': {source}")]
    Shell {
        command: String,
        source: shell_words::ParseError,
    },
    #[error("Cannot strip prefix \"{prefix}\" from {path}: {source}")]
    StripPath {
        path: std::path::PathBuf,
        prefix: String,
        source: path::StripPrefixError,
    },
    #[error("{msg}: {source}")]
    FromUtf8 {
        msg: String,
        source: std::string::FromUtf8Error,
    },
    #[error("Cannot copy static directory contents: {0}")]
    FsExtra(#[from] fs_extra::error::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
