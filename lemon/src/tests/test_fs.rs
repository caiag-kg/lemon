use std::fs;
use std::fs::File;
use std::path::Path;
use super::super::*;


#[test]
fn test_new_existing_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let path_str = temp_dir.path().to_str().unwrap().to_string();

    let base_path = BasePath::new(path_str.clone(), false);
    assert_eq!(base_path.path, path_str);
}

#[test]
#[should_panic(expected = "[WARNING] Invalid path, must exist!")]
fn test_new_non_existing_directory_without_create() {
    let path_str = "./non_existent_dir".to_string();
    BasePath::new(path_str, false);
}

#[test]
fn test_new_creates_directory_when_create_is_true() {
    let temp_dir = tempfile::tempdir().unwrap();
    let new_path = temp_dir.path().join("new_dir").to_str().unwrap().to_string();

    let _base_path = BasePath::new(new_path.clone(), true);
    assert!(Path::new(&new_path).exists());
}

#[test]
fn test_create_dir() {
    let temp_dir = tempfile::tempdir().unwrap();
    let new_dir = temp_dir.path().join("test_create");

    assert!(!new_dir.exists());
    BasePath::create_dir(new_dir.to_str().unwrap()).unwrap();
    assert!(new_dir.exists());
}

#[test]
fn test_is_exist() {
    let temp_dir = tempfile::tempdir().unwrap();
    assert!(BasePath::is_exist(temp_dir.path().to_str().unwrap()));

    let non_existing = "./does_not_exist";
    assert!(!BasePath::is_exist(non_existing));
}

#[test]
fn test_is_dir() {
    let temp_dir = tempfile::tempdir().unwrap();
    assert!(BasePath::is_dir(temp_dir.path().to_str().unwrap()));

    let temp_file = temp_dir.path().join("temp.txt");
    File::create(&temp_file).unwrap();
    assert!(!BasePath::is_dir(temp_file.to_str().unwrap()));
}

#[test]
fn test_copy_dir() {
    let temp_dir = tempfile::tempdir().unwrap();
    let src_dir = temp_dir.path().join("src");
    let dest_dir = temp_dir.path().join("dest");

    fs::create_dir(&src_dir).unwrap();
    File::create(src_dir.join("file.txt")).unwrap();

    let src_base = BasePath::new(src_dir.to_str().unwrap().to_string(), false);
    let dest_base = BasePath::new(dest_dir.to_str().unwrap().to_string(), true);

    src_base.copy_dir(&dest_base, false).unwrap();

    assert!(dest_dir.exists());
    assert!(dest_dir.join("file.txt").exists());
}

#[test]
fn test_copy_dir_with_remove_source() {
    let temp_dir = tempfile::tempdir().unwrap();
    let src_dir = temp_dir.path().join("src");
    let dest_dir = temp_dir.path().join("dest");

    fs::create_dir(&src_dir).unwrap();
    File::create(src_dir.join("file.txt")).unwrap();

    let src_base = BasePath::new(src_dir.to_str().unwrap().to_string(), false);
    let dest_base = BasePath::new(dest_dir.to_str().unwrap().to_string(), true);

    src_base.copy_dir(&dest_base, true).unwrap();

    assert!(!src_dir.exists()); // Source should be removed
    assert!(dest_dir.exists());
    assert!(dest_dir.join("file.txt").exists());
}

#[test]
fn test_rm_dir() {
    let temp_dir = tempfile::tempdir().unwrap();
    let dir_to_remove = temp_dir.path().join("to_remove");

    fs::create_dir(&dir_to_remove).unwrap();
    assert!(dir_to_remove.exists());

    let mut base_path = BasePath::new(temp_dir.path().to_str().unwrap().to_string(), false);
    base_path.rm_dir(dir_to_remove.to_str().unwrap()).unwrap();

    assert!(!dir_to_remove.exists());
}

#[test]
fn test_rm_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_to_remove = temp_dir.path().join("file.txt");

    File::create(&file_to_remove).unwrap();
    assert!(file_to_remove.exists());

    let mut base_path = BasePath::new(temp_dir.path().to_str().unwrap().to_string(), false);
    base_path.rm_file(file_to_remove.to_str().unwrap()).unwrap();

    assert!(!file_to_remove.exists());
}