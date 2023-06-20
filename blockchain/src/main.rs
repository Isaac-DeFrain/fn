use clap::{Parser, Subcommand};
use canonical_chain_discovery::{backward_discovery, forward_discovery, common};

mod canonical_chain_discovery;

#[derive(Parser, Debug)]
#[command(name = "chain-discovery", author, version, about, long_about = Some("Mina canonical chain discovery"))]
struct Cli {
    #[command(subcommand)]
    command: CliSubcommand,
}

#[derive(Subcommand, Debug)]
enum CliSubcommand {
    /// Use the backward canonical chain discovery algorithm (fast)
    Backward(common::SubcommandArgs),
    /// Use the forward canonical chain discovery algorithm (slow)
    Forward(common::SubcommandArgs),
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    match Cli::parse().command {
        CliSubcommand::Backward(args) => {
            backward_discovery::main(&args)
        }
        CliSubcommand::Forward(args) => {
            forward_discovery::main(&args)
        }
    }
}

