use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::Result;

pub struct FileFinder {
    recursive: bool,
}

impl FileFinder {
    pub fn new(recursive: bool) -> Self {
        Self { recursive }
    }

    pub fn find_mov_files(&self, directory: &Path) -> Result<Vec<PathBuf>> {
        let mut mov_files = Vec::new();

        let walker = if self.recursive {
            WalkDir::new(directory)
        } else {
            WalkDir::new(directory).max_depth(1)
        };

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && self.is_mov_file(path) {
                mov_files.push(path.to_path_buf());
            }
        }

        Ok(mov_files)
    }

    fn is_mov_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            extension.to_string_lossy().to_lowercase() == "mov"
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_mov_file() {
        let finder = FileFinder::new(false);
        
        assert!(finder.is_mov_file(Path::new("test.MOV")));
        assert!(finder.is_mov_file(Path::new("test.mov")));
        assert!(!finder.is_mov_file(Path::new("test.mp4")));
        assert!(!finder.is_mov_file(Path::new("test")));
    }
} 