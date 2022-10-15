use colored::Colorize;

use std::{io, path};

// TODO: doc
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Tera templating error: {0}")]
    Tera(#[from] tera::Error),
    #[error("Error parsing config: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Cannot initialize logger as set_logger has already been called")]
    Log(#[from] log::SetLoggerError),
    #[error("Cannot parse tranform: {0}")]
    Shell(#[from] shell_words::ParseError),
    #[error("{0}\n{}: This is probably a bug. Please open an issue at https://github.com/alterae/mksite/issues.", "NOTE".bold())]
    Path(#[from] path::StripPrefixError),
    #[error("Cannot convert to UTF-8: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("Failed to copy directory: {0}")]
    FsExtra(#[from] fs_extra::error::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
