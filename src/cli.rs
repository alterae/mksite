//! Command-line interface definition and argument handling.

pub(crate) mod build;
mod clean;
mod init;
mod new;

/// A file format-agnostic static site generator.
#[derive(clap::Parser)]
#[command(version)]
pub(crate) struct Args {
    /// The subcommand to run.
    #[command(subcommand)]
    pub(crate) command: Command,

    /// Do not print log messages.
    #[arg(short, long, conflicts_with = "log_level")]
    pub(crate) quiet: bool,

    /// What level of logging to enable (error, warn, info, debug, or trace).
    #[arg(long, default_value = "info")]
    pub(crate) log_level: log::LevelFilter,
}

/// Enum of subcommands.
#[derive(clap::Subcommand)]
pub(crate) enum Command {
    /// Build the site according to `mksite.toml`.
    Build,
    /// Delete all build outputs.
    Clean,
    /// Initialize a `mksite.toml` file in the current directory.
    Init,
    /// Scaffold an empty site in a new directory.
    New {
        /// The name of the directory to create.
        name: String,
    },
}

impl Command {
    /// Runs the given command.
    pub(crate) fn run(self) -> crate::Result<()> {
        match self {
            Self::Build => build::cmd(),
            Self::Clean => clean::cmd(),
            Self::Init => init::cmd(),
            Self::New { name } => new::cmd(name),
        }
    }
}
