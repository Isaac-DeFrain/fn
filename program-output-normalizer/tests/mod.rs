use clap::Parser;
use program_output_normalizer::{version::Version, Program};

#[test]
fn parse_echo() -> anyhow::Result<()> {
    let version = "1";
    let config = "minimina";
    let cmd = "echo hello";
    let res = Program::parse_from(vec![
        "program-output-normalizer",
        "-v",
        version,
        "--config",
        config,
        "-c",
        cmd,
    ]);

    assert_eq!(res.version, Version::new(version.parse::<u32>()?));
    assert_eq!(res.config, config.to_string());
    assert_eq!(res.command, cmd.to_string());

    Ok(())
}
