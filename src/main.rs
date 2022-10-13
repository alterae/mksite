use clap::Parser;

mod cli;
mod config;
mod transform;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    args.command.run()
}
