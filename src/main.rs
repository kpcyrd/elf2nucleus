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
use std::process::Command;
use std::process::Stdio;

fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = "info";
    env_logger::init_from_env(Env::default().default_filter_or(log_level));
    debug!("Started with arguments: {args:?}");

    let output = Command::new("avr-objcopy")
        .args([
            OsStr::new("--output-target=ihex"),
            args.input.as_ref(),
            OsStr::new("/dev/stdout"),
        ])
        .output()
        .context("Failed to execute avr-objcopy")?;

    if !output.status.success() {
        anyhow::bail!("avr-objcopy exited with error: {:?}", output.status);
    }

    let hex = &output.stdout;

    if let Some(output) = args.output {
        fs::write(&output, hex)
            .with_context(|| anyhow!("Failed to write firmware to file {output:?}"))?;
    } else {
        let mut argv = Vec::new();
        if let Some(timeout) = args.timeout {
            argv.push(Cow::Borrowed("--timeout"));
            argv.push(Cow::Owned(timeout.to_string()));
        }
        argv.extend([
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
            stdin.write_all(hex)?;
        };

        child.wait()?;
    }

    Ok(())
}
