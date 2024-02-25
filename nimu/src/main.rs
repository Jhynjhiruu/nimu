use anyhow::Result;
use clap::Parser;

use std::fs::read;
use std::path::PathBuf;

use nimu::Nimu;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Bootrom file
    #[arg(short, long)]
    bootrom: PathBuf,

    /// Virage0 file
    #[arg(short, long)]
    virage0: PathBuf,

    /// Virage1 file
    #[arg(short, long)]
    virage1: PathBuf,

    /// Virage2 file
    #[arg(short, long)]
    virage2: PathBuf,

    /// NAND file
    #[arg(short, long)]
    nand: PathBuf,

    /// Spare data file
    #[arg(short, long)]
    spare: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let bootrom = read(cli.bootrom)?;
    let v0 = read(cli.virage0)?;
    let v1 = read(cli.virage1)?;
    let v2 = read(cli.virage2)?;
    let nand = read(cli.nand)?;
    let spare = read(cli.spare)?;

    let mut nimu = Nimu::new(bootrom, v0, v1, v2, nand, spare);

    nimu.run();

    Ok(())
}
