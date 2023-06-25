use clap::{Parser, Subcommand};

mod contiguous;
mod new_only;

#[derive(Parser, Debug)]
#[command(name = "mina-gsutil", author, version, about, long_about = Some("Download Mina blocks with ease!"))]
struct Entrypoint {
    /// Only download the new blocks absent from your block dir
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Only download the most recent blocks absent from your block dir
    NewOnly(new_only::NewArgs),
    /// Download a contiguous collection blocks
    Contiguous(contiguous::ContiguousArgs),
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    match Entrypoint::parse().command {
        Command::Contiguous(args) => contiguous::main(args),
        Command::NewOnly(args) => new_only::main(args),
    }
}
