use std::path::{Path, PathBuf};
use anyhow::Result;
use tracing::{info, warn, error};
use tokio::process::Command;
use std::process::Stdio;

pub struct Converter {
    force_overwrite: bool,
}

impl Converter {
    pub fn new(force_overwrite: bool) -> Self {
        Self { force_overwrite }
    }

    pub async fn convert_file(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        info!("Converting {} to {}", input_path.display(), output_path.display());

        // Check if output file already exists
        if output_path.exists() && !self.force_overwrite {
            warn!("Output file {} already exists. Use --force to overwrite.", output_path.display());
            return Ok(());
        }

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Build FFmpeg command
        let mut cmd = Command::new("ffmpeg");
        
        cmd.args(&[
            "-i", input_path.to_string_lossy().as_ref(),
            "-c:v", "libx264",        // Video codec: H.264
            "-c:a", "aac",            // Audio codec: AAC
            "-preset", "medium",      // Encoding preset (fast, medium, slow, etc.)
            "-crf", "23",             // Constant Rate Factor (18-28 is good, lower = better quality)
            "-movflags", "+faststart", // Optimize for web streaming
            "-y",                     // Overwrite output files
        ]);

        // Add output path
        cmd.arg(output_path.to_string_lossy().as_ref());

        // Set up process
        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped());

        info!("Running FFmpeg command: {:?}", cmd);

        // Execute FFmpeg
        let output = cmd.output().await?;

        if output.status.success() {
            info!("Conversion completed successfully");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg failed: {}", stderr);
            Err(anyhow::anyhow!("FFmpeg conversion failed: {}", stderr))
        }
    }

    pub fn generate_output_path(&self, input_path: &Path, output_dir: Option<&Path>) -> PathBuf {
        let stem = input_path.file_stem().unwrap_or_default();
        let mut output_path = PathBuf::new();
        
        if let Some(dir) = output_dir {
            output_path.push(dir);
        } else {
            output_path.push(input_path.parent().unwrap_or_else(|| Path::new(".")));
        }
        
        output_path.push(format!("{}.mp4", stem.to_string_lossy()));
        output_path
    }

    /// Check if FFmpeg is available on the system
    pub async fn check_ffmpeg_available() -> Result<bool> {
        let output = Command::new("ffmpeg")
            .arg("-version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        Ok(output.is_ok() && output.unwrap().status.success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_output_path() {
        let converter = Converter::new(false);
        let input = Path::new("videos/test.MOV");
        let output = converter.generate_output_path(input, None);
        assert_eq!(output, PathBuf::from("videos/test.mp4"));
    }

    #[test]
    fn test_generate_output_path_with_custom_dir() {
        let converter = Converter::new(false);
        let input = Path::new("videos/test.MOV");
        let output_dir = Path::new("converted");
        let output = converter.generate_output_path(input, Some(output_dir));
        assert_eq!(output, PathBuf::from("converted/test.mp4"));
    }

    #[tokio::test]
    async fn test_check_ffmpeg_available() {
        let available = Converter::check_ffmpeg_available().await.unwrap_or(false);
        // This test will pass if FFmpeg is available, fail gracefully if not
        println!("FFmpeg available: {}", available);
    }
} 