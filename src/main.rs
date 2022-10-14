use clap::Parser;

mod cli;
mod config;
mod transform;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    stderrlog::new()
        .module(module_path!())
        .show_module_names(true)
        .quiet(args.quiet)
        // + 1 to show warnings by default
        .verbosity(args.verbose as usize + 1)
        .init()?;

    args.command.run()
}
