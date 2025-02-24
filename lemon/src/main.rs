#![allow(dead_code, unused_assignments, unused_variables)]
mod models;
mod service;

use service::DirPath;
use models::{BasePath, YesOrNo};
use crate::models::{CliArgs, DirTypes};

fn main() {
    println!("\n\t\t\"Lemon CLI\"");
    // Read args from console;
    let args = CliArgs::new().unwrap();
    
    match args.has_target() {
        true => {
            handle_copy(&args)
        }
        false => {
            handle_delete(&args);
        }
    }    
}


fn handle_delete(args: &CliArgs) {
    println!("\nDelete!");
}


fn handle_copy(args: &CliArgs) {
    // Check that source PATH is existed.
    let source_path = BasePath::new(args.source.clone(), false);

    // Check that target PATH is existed else create path tree.
    let target_path = BasePath::new(args.target.clone(), true);

    // Questionnaire: remove source folder?
    let question = format!("Do you want to remove the source directory: {} ? ", source_path.path);
    let remove_source = YesOrNo::ask(&question, true).unwrap();

    // Check that [include_files, include_dirs, exclude_dirs, exclude_files] are empty.
    // If empty then just coping.
    if args.has_dirs_files(DirTypes::Exclude) {
        let question = format!(
            "Do you want to remove: \nFolders: {:?}\nFiles: {:?}",
            &args.exclude_dirs,
            &args.exclude_files
        );
        todo!();
        println!("{}", question);
        return;
    }
    
    if args.has_dirs_files(DirTypes::Include) {
        let question = format!(
            "Do you want to remove all except: \nFolders: {:?}\nFiles: {:?}",
            &args.include_dirs,
            &args.include_files
        );
        todo!();
        println!("{}", question);
        return;
    }
    
    // Full copy:
    match source_path.copy_dir(&target_path, remove_source) {
        Ok(_) => {
            println!("\n[INFO] Copied successfully!\n");
        }
        Err(e) => {
            println!("\n[ERROR] Couldn't copy directory: {}\n", &target_path.path);
            println!("{}\n", e);
        }
    }
}
