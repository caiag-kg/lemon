use std::error::Error;
use clap::{Arg, ArgMatches, Command};
use clap::parser::ValuesRef;
use crate::models::{CliArgs, DirTypes};


impl CliArgs {
    pub fn new() -> Result<CliArgs, Box<dyn Error>> {
        let matches = Self::get_matches()?;
        let include_dirs: Vec<String> = matches
                .get_many::<String>("include_dirs")
                .unwrap_or_default()
                .cloned()
                .collect();
        let include_files: Vec<String> = matches
                .get_many::<String>("include_files")
                .unwrap_or_default()
                .cloned()
                .collect();
        let exclude_dirs: Vec<String> = matches
                .get_many::<String>("exclude_dirs")
                .unwrap_or_default()
                .cloned()
                .collect();
        let exclude_files: Vec<String> = matches
                .get_many::<String>("exclude_files")
                .unwrap_or_default()
                .cloned()
                .collect();
        Ok(
            CliArgs{
                source: String::from(matches.get_one::<String>("source").unwrap()),
                target: String::from(matches.get_one::<String>("target").unwrap()),
                include_dirs,
                include_files,
                exclude_dirs,
                exclude_files
            }
        )
    }

    pub fn has_target(&self) -> bool {
        !self.target.trim().is_empty()
    }
    
    pub fn has_dirs_files(&self, dir_types: DirTypes) -> bool {
        let mut ress: Vec<bool> = Vec::new();
        let dirs_files = match dir_types {
            DirTypes::Exclude => {
                [&self.exclude_dirs, &self.exclude_files]
            }
            DirTypes::Include => {
                [&self.include_dirs, &self.exclude_dirs]
            }
        };
        for label_value in dirs_files.iter() {
            if label_value.len() == 1 && label_value[0].trim().is_empty() {
                ress.push(false);
            } else {
                ress.push(true);
            }
        }
        ress.iter().any(|&x| x)
    }

    pub fn get_matches() -> Result<ArgMatches, Box<dyn Error>> {
        let matches = Command::new("lemon")
            .version("0.1.0")
            .author("Author: <Kubanychbek uulu Kairat>")
            .about("A CLI tool to copy a directory structure from source to destination, \
        with options to include or exclude specific directories and files.")
            .arg(
                Arg::new("source")
                    .value_name("SOURCE_PATH")
                    .help("Specify the source directory to copy")
                    .index(1)
                    .required(true)
            )
            .arg(
                Arg::new("target")
                    .value_name("TARGET_PATH")
                    .help("Specify the target directory to copy")
                    .index(2)
                    .default_value("")
                    .required(false)
            )
            .arg(
                Arg::new("include_dirs")
                    .long("ind")
                    .short('a')
                    .value_name("INCLUDE_DIRS")
                    .help("Include directories")
                    .num_args(1..)
                    .default_value("")
                    .required(false)
            )
            .arg(
                Arg::new("include_files")
                    .long("inf")
                    .short('b')
                    .value_name("INCLUDE_FILES")
                    .help("Include files")
                    .num_args(1..)
                    .default_value("")
                    .required(false)
            )
            .arg(
                Arg::new("exclude_dirs")
                    .long("xd")
                    .short('c')
                    .value_name("EXCLUDE_DIRS")
                    .help("Exclude directories")
                    .num_args(1..)
                    .default_value("")
                    .required(false)
            )
            .arg(
                Arg::new("exclude_files")
                    .long("xf")
                    .short('d')
                    .value_name("EXCLUDE_FILES")
                    .help("Exclude files")
                    .num_args(1..)
                    .default_value("")
                    .required(false)

            ).get_matches();
        Ok(matches)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_exclude_include_dirs_with_true() {
        let must_be_true = CliArgs{
            source: "Some".to_string(),
            target: "Another".to_string(),
            include_dirs: vec!["".to_string()],
            include_files: vec!["".to_string()],
            exclude_dirs: vec!["some".to_string()],
            exclude_files: vec!["".to_string()],
        }.has_dirs_files(DirTypes::Exclude);
    
        assert!(must_be_true);
    }

    #[test]
    fn test_has_exclude_include_dirs_with_false() {
        let must_be_false = CliArgs{
            source: "Some".to_string(),
            target: "Another".to_string(),
            include_dirs: vec!["".to_string()],
            include_files: vec!["".to_string()],
            exclude_dirs: vec!["".to_string()],
            exclude_files: vec!["".to_string()],
        }.has_dirs_files(DirTypes::Exclude);

        assert!(!must_be_false);
    }

    #[test]
    fn test_cli_args() {
        let cli_args = CliArgs{
            source: "Some".to_string(),
            target: "Another".to_string(),
            include_dirs: vec!["".to_string()],
            include_files: vec!["".to_string()],
            exclude_dirs: vec!["a".to_string()],
            exclude_files: vec!["".to_string()],
        };
        
        assert_eq!(cli_args.source, String::from("Some"), "Error in source");
        assert_eq!(cli_args.target, String::from("Another"), "Error in target");
        assert_eq!(cli_args.exclude_dirs[0], String::from("a"), "Error in exclude dirs");
    }
}