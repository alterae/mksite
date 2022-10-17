//! Custom error type.
use std::{io, path};

/// Custom error type for all the errors that our code can generate.
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    /// An I/O error occured.
    #[error("{msg}: {source}")]
    Io {
        /// A message detailing what context the error occured in.
        msg: String,

        /// The wrapped error that caused this error.
        source: io::Error,
    },

    // FIXME: improve error message
    /// Tera templating failed. Wraps [tera::Error].
    #[error(transparent)]
    Tera(#[from] tera::Error),

    /// Deserializing TOML failed. Wraps [toml::de::Error].
    #[error("Cannot deserialize {}: {0}", crate::config::FILE_NAME)]
    Toml(#[from] toml::de::Error),

    /// Initializing the logger failed because [log::set_logger] has already
    /// been called. Wraps [log::SetLoggerError].
    #[error(transparent)]
    Log(#[from] log::SetLoggerError),

    /// Parsing a shell command failed.
    #[error("Cannot parse `{command}': {source}")]
    Shell {
        /// The command that failed to parse.
        command: String,

        /// The wrapped error that caused this error.
        source: shell_words::ParseError,
    },

    /// Attempting to strip the prefix from a file path failed.
    #[error("Cannot strip prefix '{prefix}' from '{path}': {source}")]
    StripPath {
        /// The path that could not be stripped.
        path: path::PathBuf,

        /// The prefix that could not be stripped from the path.
        prefix: path::PathBuf,

        /// The wrapped error that caused this error.
        source: path::StripPrefixError,
    },

    /// Converting a string from a stream of bytes failed.
    #[error("{msg}: {source}")]
    FromUtf8 {
        /// A message detailing what context the error occured in.
        msg: String,

        /// The wrapped error that caused this error.
        source: std::string::FromUtf8Error,
    },

    /// An error occured while performing some operation via [fs_extra]. Wraps
    /// [fs_extra::error::Error].
    #[error("Cannot copy static directory contents: {0}")]
    FsExtra(#[from] fs_extra::error::Error),

    // debug instead of display bc escaping the things is actually useful here
    /// An path was invalid UTF-8. This is distinct from [Error::FromUtf8] in
    /// that it occurs when converting a _path_ to UTF-8, while [Error::FromUtf8]
    /// is for errors converting byte vecs.
    #[error("Invalid UTF-8 in path {0:?}")]
    PathConversion(path::PathBuf),
}

/// Custom result wrapper that represents either success or failure.
pub(crate) type Result<T> = std::result::Result<T, Error>;
