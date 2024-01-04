use std::path::PathBuf;

/// Flash an AVR firmware from an elf file with micronucleus
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// The elf file containing the firmware
    pub input: PathBuf,
    /// Instead of flashing, write the firmware to file
    pub output: Option<PathBuf>,
    /// Timeout in seconds for micronucleus device discovery
    #[arg(short, long)]
    pub timeout: Option<u64>,
}
