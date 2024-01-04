use crate::errors::*;
use clap::{ArgAction, CommandFactory, Parser};
use clap_complete::Shell;
use std::io::stdout;
use std::path::PathBuf;

/// Flash an AVR firmware from an elf file with micronucleus
#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// Increase logging output (can be used multiple times)
    #[arg(short, long, global = true, action(ArgAction::Count))]
    pub verbose: u8,
    /// The elf file containing the firmware
    pub input: Option<PathBuf>,
    /// Instead of flashing, write the firmware to file
    pub output: Option<PathBuf>,
    /// Timeout in seconds for micronucleus device discovery
    #[arg(short, long)]
    pub timeout: Option<u64>,
    /// Generate shell completions
    #[arg(long, hide = true)]
    pub completions: Option<Shell>,
}

pub fn gen_completions(shell: Shell) -> Result<()> {
    clap_complete::generate(shell, &mut Args::command(), "elf2nucleus", &mut stdout());
    Ok(())
}
