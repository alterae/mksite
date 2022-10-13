//! Command-line interface definition and argument handling.

use std::{fs, path::Path};

use anyhow::Context;
use tera::Tera;

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
                    config::exists(),
                    "Cannot build site: {} not found",
                    config::FILE_NAME
                );

                let config = config::load()?;

                let context = tera::Context::from_serialize(config.metadata)?;

                let tera = Tera::new("src/**/*")?;

                for template in tera.get_template_names() {
                    let output = tera.render(template, &context)?;

                    let path = Path::new("out").join(template);

                    if let Some(p) = path.parent() {
                        fs::create_dir_all(p)?;
                    }

                    // TODO: process output, apply layouts

                    fs::write(path, output)?;
                }

                Ok(())
            }
            Self::Clean => {
                anyhow::ensure!(
                    config::exists(),
                    "Cannot clean site: {} not found",
                    config::FILE_NAME
                );

                fs::remove_dir_all("out").context("No build output to clean")
            }
            Self::Init => config::generate_config_file(&Path::new(".")),
            Self::New { name } => {
                fs::create_dir(&name).with_context(|| {
                    format!("Cannot create directory {name} as it already exists")
                })?;
                fs::create_dir(Path::new(&name).join("src"))?;
                config::generate_config_file(&Path::new(&name))
            }
        }
    }
}
