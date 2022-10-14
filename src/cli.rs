//! Command-line interface definition and argument handling.

pub mod build;
mod clean;
mod init;
mod new;

/// A file format-agnostic static site generator.
#[derive(clap::Parser)]
#[command(version)]
pub(crate) struct Args {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Command,
    /// Do not print log messages.
    #[arg(short, long)]
    pub quiet: bool,
    /// Use verbose output (-v, -vv, -vvv etc.).
    #[arg(short, long, action=clap::ArgAction::Count)]
    pub verbose: u8,
}

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
    pub(crate) fn run(self) -> anyhow::Result<()> {
        match self {
            Self::Build => build::cmd(),
            Self::Clean => clean::cmd(),
            Self::Init => init::cmd(),
            Self::New { name } => new::cmd(name),
        }
    }
}
