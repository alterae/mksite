/// A format-agnostic static site generator.
#[derive(clap::Parser)]
#[command(version)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Command,
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
