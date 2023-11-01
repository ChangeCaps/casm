use clap::Parser;

#[derive(Parser)]
struct BuildArgs {
    /// The input file to use
    input: String,
}

#[derive(Parser)]
enum SubCommand {
    /// Build the project
    #[clap(name = "build")]
    Build(BuildArgs),
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() {
    let args = Args::parse();
}
