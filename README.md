# MOV to MP4 Converter

A command-line tool written in Rust to find and convert MOV files to MP4 format using FFmpeg.

## Features

- Find MOV files in a specified directory
- Recursive directory search
- Convert MOV files to MP4 format using FFmpeg
- Configurable output directory
- Force overwrite option
- Verbose logging
- Progress tracking
- Automatic FFmpeg availability check

## Installation

### Prerequisites

- **Rust** (latest stable version)
- **FFmpeg** (required for video conversion)

### Installing FFmpeg

#### Windows
```bash
# Using Chocolatey
choco install ffmpeg

# Using Scoop
scoop install ffmpeg

# Manual installation
# Download from https://ffmpeg.org/download.html#build-windows
```

#### macOS
```bash
# Using Homebrew
brew install ffmpeg

# Using MacPorts
sudo port install ffmpeg
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install ffmpeg
```

#### Linux (CentOS/RHEL/Fedora)
```bash
# CentOS/RHEL
sudo yum install ffmpeg

# Fedora
sudo dnf install ffmpeg
```

### Building from source

```bash
git clone <repository-url>
cd mov2mp4
cargo build --release
```

## Usage

```bash
# Basic usage - convert MOV files in current directory
cargo run -- sample/

# Convert with custom output directory
cargo run -- -i sample/ -o converted/

# Recursive search in subdirectories
cargo run -- -i sample/ -r

# Force overwrite existing files
cargo run -- -i sample/ -f

# Verbose output
cargo run -- -i sample/ -v

# Combine options
cargo run -- -i sample/ -o converted/ -r -f -v
```

### Command-line Options

- `-i, --input <DIR>`: Input directory containing MOV files (required)
- `-o, --output <DIR>`: Output directory for converted MP4 files (optional)
- `-r, --recursive`: Recursively search subdirectories
- `-f, --force`: Overwrite existing files
- `-v, --verbose`: Enable verbose logging
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## FFmpeg Conversion Settings

The tool uses the following FFmpeg settings for optimal quality and compatibility:

- **Video Codec**: H.264 (libx264)
- **Audio Codec**: AAC
- **Preset**: Medium (good balance of speed and quality)
- **Quality**: CRF 23 (good quality, reasonable file size)
- **Optimization**: Fast start enabled for web streaming

### Customizing FFmpeg Settings

To modify the conversion settings, edit the `convert_file` method in `src/converter.rs`:

```rust
cmd.args(&[
    "-i", input_path.to_string_lossy().as_ref(),
    "-c:v", "libx264",        // Change video codec
    "-c:a", "aac",            // Change audio codec
    "-preset", "medium",      // Change preset (fast, medium, slow, etc.)
    "-crf", "23",             // Change quality (18-28, lower = better)
    "-movflags", "+faststart", // Web optimization
    "-y",                     // Overwrite output files
]);
```

## Project Structure

```
src/
├── main.rs          # Application entry point
├── lib.rs           # Library root and module declarations
├── cli.rs           # Command-line interface definitions
├── finder.rs        # File discovery and filtering logic
└── converter.rs     # FFmpeg video conversion logic
```

## Testing

Run the test suite:

```bash
cargo test
```

Test FFmpeg availability:

```bash
cargo test test_check_ffmpeg_available
```

## Troubleshooting

### FFmpeg Not Found
If you get an error that FFmpeg is not found:
1. Make sure FFmpeg is installed
2. Ensure FFmpeg is in your system PATH
3. Restart your terminal/IDE after installation

### Conversion Failures
- Check that input files are valid MOV files
- Ensure you have sufficient disk space
- Try running with `-v` flag for detailed error messages

## License

[Add your license here]

## Contributing

[Add contribution guidelines here] 