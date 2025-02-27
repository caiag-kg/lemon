#![allow(dead_code)]
use crate::models::BasePath;
use fs_extra::dir::{copy, CopyOptions};
use std::{
    error::Error,
    fmt::Debug,
    fs,
    io::{stdin, stdout, BufRead, Result, Write},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};
use walkdir::WalkDir;

impl BasePath {
    pub fn new(path: String, create: bool) -> Self {
        match Self::is_exist(&path) {
            true => {
                assert!(
                    Self::is_dir(&path),
                    "\n[WARNING] Invalid path, must be directory!\n"
                );
            } // if PATH exist.
            false => {
                match create {
                    true => Self::create_dir(path.as_str()).unwrap(), // if create is true.
                    false => {
                        assert!(
                            Self::is_exist(&path),
                            "\n[WARNING] Invalid path, must exist!\n"
                        );
                    }
                }
            }
        }
        BasePath { path }
    }

    pub fn create_dir(dir_path: &str) -> Result<()> {
        let dir_path = Path::new(dir_path);
        fs::create_dir_all(dir_path)?;
        Ok(())
    }

    pub fn is_exist(path: &str) -> bool {
        let path = Path::new(path);
        path.exists()
    }

    pub fn is_dir(path: &str) -> bool {
        let path = Path::new(path);
        path.is_dir()
    }
}

impl BasePath {
    pub fn copy_dir(&self, destination: &BasePath, remove_source: bool) -> Result<()> {
        let source = Path::new(&self.path);
        for entry in WalkDir::new(source) {
            let entry = entry?;
            let rel_path = entry.path().strip_prefix(source).unwrap();
            let dest = Path::new(&destination.path);
            let dest_path = dest.join(rel_path);

            if entry.file_type().is_dir() {
                fs::create_dir_all(dest_path)?;
            } else {
                fs::copy(entry.path(), dest_path)?;
            }
        }
        // Remove source directory if remove_source is true:
        if remove_source {
            fs::remove_dir_all(&self.path)?;
        }
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn copy_dir_with_excludes(
        &self,
        include_dirs: Vec<String>,
        include_files: Vec<String>,
        exclude_dirs: Vec<String>,
        exclude_files: Vec<String>,
    ) -> Result<u64> {
        if let Some(entry) = WalkDir::new(&self.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .next()
        {
            let entry_dirs = entry.path();
            for dir in entry_dirs.iter() {
                println!("Dir: {}", dir.to_string_lossy());
            }
            return Ok(12_u64);
        }
        Ok(123_u64)
    }
}

impl BasePath {
    pub fn rm_dir(&mut self, dir: &str) -> Result<()> {
        let path = Path::new(dir);
        if path.exists() && path.is_dir() {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    pub fn rm_file(&mut self, file: &str) -> Result<()> {
        let path = Path::new(file);
        if path.exists() && path.is_file() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
