#![allow(dead_code, unused_assignments, unused_variables)]
mod models;
mod service;

use service::DirPath;
use models::BasePath;
use crate::models::CliArgs;

fn main() {
    // Read args from console;
    let args = CliArgs::new().unwrap();
    
    // Check that source PATH is existed.
    let source_path = BasePath::new(args.source.clone(), false);
    
    // Check that target PATH is existed else create path tree.
    let target = args.target.clone();
    let target_path = BasePath::new(target.clone(), true);
    
    println!("Has_target: {}", args.has_target());
    println!("Has exclude & includes: {}", args.has_exclude_include_dirs());

    // 
    // let question = format!("Do you want to remove the source directory: {} ? ", source_path.path);
    // let remove_source = yes_or_no(&question).unwrap();
    // println!("Remove? {}", remove_source.value_as_string().unwrap());
    
    // // Check that [include_files, include_dirs, exclude_dirs, exclude_files] are empty.
    // // If empty then just coping.
    // if ![include_files, include_dirs, exclude_dirs, exclude_files]
    //     .iter()
    //     .all(|i| i.iter().all(|x| {!x.is_empty()})
    //     ) {
    //     source_path.copy_dir(&target_path).unwrap();
    //     println!("[INFO] Copied successfully!");
    //     println!("Target: {}", target_path.path);
    // }
}
