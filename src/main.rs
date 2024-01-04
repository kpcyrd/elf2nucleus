pub mod args;
pub mod errors;

use crate::args::Args;
use crate::errors::*;
use anyhow::Context;
use clap::Parser;
use env_logger::Env;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

fn parse_from_elf<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let output = Command::new("avr-objcopy")
        .args([
            OsStr::new("--output-target=ihex"),
            path.as_ref().as_ref(),
            OsStr::new("/dev/stdout"),
        ])
        .output()
        .context("Failed to execute avr-objcopy")?;

    if !output.status.success() {
        anyhow::bail!("avr-objcopy exited with error: {:?}", output.status);
    }

    let mut buf = Vec::new();
    let ihex = String::from_utf8(output.stdout)?;
    let reader = ihex::Reader::new(&ihex);

    for item in reader {
        if let Ok(ihex::Record::Data { offset: _, value }) = item {
            buf.extend(value);
        }
    }

    Ok(buf)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = match args.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    env_logger::init_from_env(Env::default().default_filter_or(log_level));
    trace!("Started with arguments: {args:?}");

    match (&args.input, args.completions) {
        (Some(input), None) => {
            let hex = parse_from_elf(input)?;

            if let Some(output) = args.output {
                fs::write(&output, &hex)
                    .with_context(|| anyhow!("Failed to write firmware to file {output:?}"))?;
            } else {
                let mut argv = Vec::new();
                if let Some(timeout) = args.timeout {
                    argv.push(Cow::Borrowed("--timeout"));
                    argv.push(Cow::Owned(timeout.to_string()));
                }
                argv.extend([
                    Cow::Borrowed("--type"),
                    Cow::Borrowed("raw"),
                    Cow::Borrowed("--run"),
                    Cow::Borrowed("--no-ansi"),
                    Cow::Borrowed("-"),
                ]);
                debug!("Starting micronucleus {args:?}");
                let mut child = Command::new("micronucleus")
                    .args(argv.iter().map(|x| x.as_ref()))
                    .stdin(Stdio::piped())
                    .spawn()
                    .context("Failed to execute micronucleus")?;

                {
                    let mut stdin = child.stdin.take().unwrap();
                    stdin.write_all(&hex)?;
                };

                child.wait()?;
            }
        }
        (None, Some(shell)) => {
            args::gen_completions(shell)?;
        }
        _ => bail!("Usage: elf2nucleus <input>"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_elf() -> Result<()> {
        let elf = parse_from_elf("test_data/attiny85-hello-world.elf")?;
        assert_eq!(
            elf,
            [
                14, 192, 29, 192, 28, 192, 27, 192, 26, 192, 25, 192, 24, 192, 23, 192, 22, 192,
                21, 192, 20, 192, 19, 192, 18, 192, 17, 192, 16, 192, 17, 36, 31, 190, 207, 229,
                210, 224, 222, 191, 205, 191, 32, 224, 160, 230, 176, 224, 1, 192, 29, 146, 161,
                54, 178, 7, 225, 247, 2, 208, 188, 192, 224, 207, 1, 208, 181, 208, 207, 146, 223,
                146, 239, 146, 255, 146, 15, 147, 31, 147, 143, 183, 248, 148, 144, 145, 96, 0,
                144, 48, 9, 240, 160, 192, 145, 224, 144, 147, 96, 0, 143, 191, 193, 152, 185, 154,
                193, 154, 64, 224, 80, 224, 42, 239, 63, 225, 140, 227, 146, 224, 124, 1, 108, 231,
                116, 224, 236, 225, 241, 224, 140, 227, 157, 224, 108, 1, 193, 154, 138, 1, 202, 1,
                217, 1, 17, 151, 241, 247, 161, 224, 9, 55, 17, 5, 132, 7, 149, 7, 8, 240, 161, 45,
                161, 112, 15, 95, 31, 79, 143, 79, 159, 79, 160, 48, 121, 247, 199, 1, 1, 151, 241,
                247, 193, 152, 138, 1, 202, 1, 217, 1, 17, 151, 241, 247, 161, 224, 3, 63, 17, 5,
                132, 7, 149, 7, 8, 240, 161, 45, 161, 112, 15, 95, 31, 79, 143, 79, 159, 79, 160,
                48, 121, 247, 203, 1, 1, 151, 241, 247, 193, 154, 138, 1, 202, 1, 217, 1, 17, 151,
                241, 247, 161, 224, 12, 51, 17, 5, 132, 7, 149, 7, 8, 240, 161, 45, 161, 112, 15,
                95, 31, 79, 143, 79, 159, 79, 160, 48, 121, 247, 207, 1, 1, 151, 241, 247, 193,
                152, 138, 1, 202, 1, 217, 1, 17, 151, 241, 247, 161, 224, 7, 49, 17, 5, 132, 7,
                149, 7, 8, 240, 161, 45, 161, 112, 15, 95, 31, 79, 143, 79, 159, 79, 160, 48, 121,
                247, 198, 1, 1, 151, 241, 247, 193, 154, 138, 1, 202, 1, 217, 1, 17, 151, 241, 247,
                161, 224, 12, 51, 17, 5, 132, 7, 149, 7, 8, 240, 161, 45, 161, 112, 15, 95, 31, 79,
                143, 79, 159, 79, 160, 48, 121, 247, 207, 1, 1, 151, 241, 247, 193, 152, 138, 1,
                202, 1, 217, 1, 17, 151, 241, 247, 161, 224, 3, 63, 17, 5, 132, 7, 149, 7, 8, 240,
                161, 45, 161, 112, 15, 95, 31, 79, 143, 79, 159, 79, 160, 48, 121, 247, 203, 1, 1,
                151, 241, 247, 117, 207, 143, 191, 3, 208, 5, 208, 3, 208, 3, 208, 253, 223, 1,
                208, 255, 207, 129, 224, 144, 224, 248, 148, 0, 192, 248, 148, 255, 207
            ]
        );
        Ok(())
    }
}
