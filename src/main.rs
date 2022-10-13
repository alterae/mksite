use clap::Parser;

mod cli;
mod config;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    args.command.run()
}
