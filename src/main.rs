//! Binary crate root for `mksite`.
#![warn(clippy::missing_docs_in_private_items)]

use clap::Parser;

mod cli;
mod config;
mod error;
mod site;
mod transform;
mod util;

pub(crate) use error::*;

fn main() {
    let args = cli::Args::parse();

    setup_logger(args.log_level, args.quiet).unwrap_or_else(|e| log::error!("{e}"));

    args.command.run().unwrap_or_else(|e| log::error!("{e}"))
}

/// Initializes the global logger via `fern` to the specified verbosity level.
/// If `quiet` is true, no output will be printed.
fn setup_logger(
    level: log::LevelFilter,
    quiet: bool,
) -> std::result::Result<(), log::SetLoggerError> {
    use colored::{Color, Colorize};

    fern::Dispatch::new()
        .format(move |out, message, record| {
            let color = match record.level() {
                log::Level::Error => Color::BrightRed,
                log::Level::Warn => Color::BrightYellow,
                log::Level::Trace => Color::BrightBlack,
                _ => Color::White,
            };

            out.finish(format_args!(
                "{}{message}",
                match record.level() {
                    log::Level::Info => "".to_owned(),
                    level => format!(
                        "{}: ",
                        level.as_str().to_ascii_lowercase().bold().color(color)
                    ),
                },
            ))
        })
        .level(if quiet { log::LevelFilter::Off } else { level })
        .chain(std::io::stderr())
        .apply()
}
