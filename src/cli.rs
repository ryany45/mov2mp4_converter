use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mov2mp4")]
#[command(about = "Convert MOV files to MP4 format")]
#[command(version)]
pub struct Cli {
    /// Input directory containing MOV files
    #[arg(short, long, value_name = "DIR")]
    pub input: PathBuf,

    /// Output directory for converted MP4 files
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// Recursively search subdirectories
    #[arg(short, long)]
    pub recursive: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Overwrite existing files
    #[arg(short, long)]
    pub force: bool,
} 