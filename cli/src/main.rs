mod build;

use std::io;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum SubCommand {
    /// Build the project
    #[clap(name = "build")]
    Build(build::BuildArgs),
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.subcmd {
        SubCommand::Build(args) => build::build(args),
    }
}
