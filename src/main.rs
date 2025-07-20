use mov2mp4::{Cli, FileFinder, Converter};
use tracing::{info, error};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        tracing_subscriber::fmt::init();
    }

    info!("MOV to MP4 Converter starting...");
    info!("Input directory: {}", cli.input.display());
    
    if let Some(output) = &cli.output {
        info!("Output directory: {}", output.display());
    } else {
        info!("Output directory: same as input");
    }
    
    info!("Recursive search: {}", cli.recursive);
    info!("Force overwrite: {}", cli.force);

    // Check if FFmpeg is available
    if !Converter::check_ffmpeg_available().await? {
        error!("FFmpeg is not installed or not found in PATH");
        error!("Please install FFmpeg from: https://ffmpeg.org/download.html");
        error!("Or use a package manager:");
        error!("  Windows: choco install ffmpeg");
        error!("  macOS: brew install ffmpeg");
        error!("  Ubuntu/Debian: sudo apt install ffmpeg");
        return Err(anyhow::anyhow!("FFmpeg not found"));
    }

    info!("FFmpeg is available");

    // Find MOV files
    let finder = FileFinder::new(cli.recursive);
    let mov_files = finder.find_mov_files(&cli.input)?;
    
    info!("Found {} MOV files", mov_files.len());
    
    if mov_files.is_empty() {
        println!("No MOV files found in {}", cli.input.display());
        return Ok(());
    }

    // Convert files
    let converter = Converter::new(cli.force);
    let mut success_count = 0;
    let mut error_count = 0;
    let total_files = mov_files.len();

    for mov_file in mov_files {
        let output_path = converter.generate_output_path(&mov_file, cli.output.as_deref());
        
        match converter.convert_file(&mov_file, &output_path).await {
            Ok(_) => {
                success_count += 1;
                println!("✓ Converted: {} -> {}", mov_file.display(), output_path.display());
            }
            Err(e) => {
                error_count += 1;
                error!("Failed to convert {}: {}", mov_file.display(), e);
            }
        }
    }

    println!("\nConversion summary:");
    println!("  Successfully converted: {}", success_count);
    println!("  Failed conversions: {}", error_count);
    println!("  Total files processed: {}", total_files);

    Ok(())
}
