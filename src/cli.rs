use std::{fs, path};

use anyhow::Context;

use crate::config;

/// A format-agnostic static site generator.
#[derive(clap::Parser)]
#[command(version)]
pub(crate) struct Args {
    /// The subcommand to run.
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

impl Command {
    /// Runs the given command.
    pub(crate) fn run(self) -> anyhow::Result<()> {
        match self {
            Self::Build => {
                anyhow::ensure!(
                    path::Path::new(config::FILE_NAME).exists(),
                    "Cannot build site: {} not found",
                    config::FILE_NAME
                );

                todo!()
            }
            Self::Clean => {
                anyhow::ensure!(
                    path::Path::new(config::FILE_NAME).exists(),
                    "Cannot clean site: {} not found",
                    config::FILE_NAME
                );

                fs::remove_dir_all("out").context("No build output to clean")?;

                todo!()
            }
            Self::Init => config::generate_config_file(&path::Path::new(".")),
            Self::New { name } => {
                fs::create_dir(&name).with_context(|| {
                    format!("Cannot create directory {name} as it already exists")
                })?;
                fs::create_dir(path::Path::new(&name).join("src"))?;
                config::generate_config_file(&path::Path::new(&name))
            }
        }
    }
}
