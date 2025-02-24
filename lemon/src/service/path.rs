#![allow(dead_code)]
use fs_extra::dir::{copy, CopyOptions};
use walkdir::WalkDir;
use std::{
    path::{Path, PathBuf},
    error::Error,
    fmt::Debug,
    io::{stdout, stdin, Result, BufRead, Write},
    fs,
    thread,
    time::Duration,
};
use crate::models::BasePath;


pub trait DirPath {
    #[allow(dead_code)]
    fn new(path: String, create: bool) -> Self;
    fn create_dir(dir_path: &str) -> Result<()>;
    fn copy_dir(&self, other: &BasePath, remove_source: bool) -> Result<()>;
    fn copy_dir_with_excludes(
                &self,
                include_dirs: Vec<String>,
                include_files: Vec<String>,
                exclude_dirs: Vec<String>,
                exclude_files: Vec<String>,) -> Result<u64>;
    fn is_exist(path: &str) -> bool;
    fn is_dir(path: &str) -> bool;
}

impl DirPath for BasePath {
    fn new(path: String, create: bool) -> Self {
        match Self::is_exist(&path) {
            true => {
                assert!(Self::is_dir(&path), "\n[WARNING] Invalid path, must be directory!\n");
            } // if PATH exist.
            false => {
                match create {
                    true => {
                        Self::create_dir(path.as_str()).unwrap()
                    } // if create is true.
                    false => {
                        assert!(Self::is_exist(&path), "\n[WARNING] Invalid path, must exist!\n");
                    }
                }
            }
        }
        BasePath { path }
    }
    
    fn create_dir(dir_path: &str) -> Result<()> {
        let dir_path = Path::new(dir_path);
        fs::create_dir_all(dir_path)?;
        Ok(())
    }

    fn copy_dir(&self, destination: &BasePath, remove_source: bool) -> Result<()> {
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
    fn copy_dir_with_excludes(
        &self,
        include_dirs: Vec<String>,
        include_files: Vec<String>,
        exclude_dirs: Vec<String>,
        exclude_files: Vec<String>,
    ) -> Result<u64> {        
        let mut delete = String::new();
        println!("Do you want to delete? (y/n): {:#?}\n{:#?}", exclude_dirs, exclude_files);
        stdout().flush()?;
        stdin().read_line(&mut delete)?;
        if delete.trim() == "y" {
            println!("Deleted directory: {}", self.path);
        }
        
        if let Some(entry) = WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok()).next() {
            let entry_dirs = entry.path();
            for dir in entry_dirs.iter() {
                println!("Dir: {}", dir.to_string_lossy());
            }
            return Ok(12_u64);
        }
        Ok(123_u64)
    }

    fn is_exist(path: &str) -> bool {
        let path = Path::new(path);
        path.exists()
    }

    fn is_dir(path: &str) -> bool {
        let path = Path::new(path);
        path.is_dir()
    }
}



// #[cfg(test)]
// mod tests {
//     use super::*;
//     
//     fn fixture() -> (BasePath, BasePath) {
//         let source = BasePath::new("./".to_string(), false);
//         let destination = BasePath::new("./test".to_string(), false);
//         
//         (source, destination)
//     }
//     #[test]
//     fn test_copy_dir() {
//         let (source, destination) = fixture();
//         assert!(source.copy_dir(&destination, true).is_ok());
//     }
// }