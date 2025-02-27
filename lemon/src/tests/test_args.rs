
use super::super::*;

#[test]
fn test_merge_x_includes() {
    let mut cli_args = CliArgs {
        source: "Some".to_string(),
        target: "Another".to_string(),
        include_dirs: vec!["123".to_string(), "456".to_string(), "789".to_string()],
        exclude_dirs: vec!["some".to_string(), "123".to_string(), "456".to_string()],
        include_files: vec![
            "".to_string(),
            "dir".to_string(),
            "some/another".to_string(),
        ],
        exclude_files: vec!["".to_string(), "dir".to_string(), "another/123".to_string()],
    };
    cli_args.merge_x_includes().unwrap();

    let target_inc_dirs: Vec<String> = vec!["789".to_string()];
    let target_x_dirs: Vec<String> = vec!["some".to_string()];
    let target_inc_files: Vec<String> = vec!["some/another".to_string()];
    let target_x_files: Vec<String> = vec!["another/123".to_string()];

    assert_eq!(cli_args.include_dirs, target_inc_dirs, "\n[ERROR] inc_dirs");
    assert_eq!(
        cli_args.include_files, target_inc_files,
        "\n[ERROR] inc_files"
    );
    assert_eq!(cli_args.exclude_dirs, target_x_dirs, "\n[ERROR] x_dirs");
    assert_eq!(cli_args.exclude_files, target_x_files, "\n[ERROR] x_files");
}

#[test]
fn test_has_exclude_include_dirs_with_true() {
    let must_be_true = CliArgs {
        source: "Some".to_string(),
        target: "Another".to_string(),
        include_dirs: vec!["".to_string()],
        include_files: vec!["".to_string()],
        exclude_dirs: vec!["some".to_string()],
        exclude_files: vec!["".to_string()],
    }
        .has_add_args(DirTypes::Exclude);

    assert!(must_be_true);
}

#[test]
fn test_has_exclude_include_dirs_with_false() {
    let must_be_false = CliArgs {
        source: "Some".to_string(),
        target: "Another".to_string(),
        include_dirs: vec!["".to_string()],
        include_files: vec!["".to_string()],
        exclude_dirs: vec!["".to_string()],
        exclude_files: vec!["".to_string()],
    }
        .has_add_args(DirTypes::Exclude);

    assert!(!must_be_false);
}

#[test]
fn test_cli_args() {
    let cli_args = CliArgs {
        source: "Some".to_string(),
        target: "Another".to_string(),
        include_dirs: vec!["".to_string()],
        include_files: vec!["".to_string()],
        exclude_dirs: vec!["a".to_string()],
        exclude_files: vec!["".to_string()],
    };

    assert_eq!(cli_args.source, String::from("Some"), "Error in source");
    assert_eq!(cli_args.target, String::from("Another"), "Error in target");
    assert_eq!(
        cli_args.exclude_dirs[0],
        String::from("a"),
        "Error in exclude dirs"
    );
}