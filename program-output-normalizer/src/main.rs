use clap::Parser;
use program_output_normalizer::*;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    let args = Program::parse();
    let version = args.version;
    let raw_command = args.command;

    let parts: Vec<&str> = raw_command.split(' ').collect();
    let program_name = parts[0];
    let mut program = Command::new(program_name);
    let cmd = program.args(parts[1..].to_vec());

    match cmd.output() {
        Err(err) => {
            let error = if err.raw_os_error() == Some(2) {
                if String::is_empty(&parts[0].to_string()) {
                    "No command provided".to_string()
                } else {
                    format!("Nonexistent command: {program_name}")
                }
            } else {
                err.to_string()
            };
            println!("{}", error::Error::new(version, error));
        }
        Ok(output) => {
            println!("=== Stdout ===\n{}", String::from_utf8(output.stdout)?);
            println!("=== Stderr ===\n{}", String::from_utf8(output.stderr)?);
        }
    }

    Ok(())
}
