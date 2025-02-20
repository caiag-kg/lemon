use std::error::Error;
use clap::{Arg, ArgMatches, Command};
use crate::models::CliArgs;


impl CliArgs {
    pub fn new() -> Result<CliArgs, Box<dyn Error>> {
        let matches = Self::get_matches()?;
        Ok(
            CliArgs{
                source: String::from(matches.get_one::<String>("source").unwrap()),
                target: String::from(matches.get_one::<String>("target").unwrap()),
                include_dirs: matches
                    .get_many::<String>("include_dirs")
                    .unwrap_or_default()
                    .map(|s| s.to_string())
                    .collect(),
                include_files: matches
                    .get_many::<String>("include_dirs")
                    .unwrap_or_default()
                    .map(|s| s.to_string())
                    .collect(),
                exclude_dirs: matches
                    .get_many::<String>("include_dirs")
                    .unwrap_or_default()
                    .map(|s| s.to_string())
                    .collect(),
                exclude_files: matches
                    .get_many::<String>("include_dirs")
                    .unwrap_or_default()
                    .map(|s| s.to_string())
                    .collect(),
            }
        )
    }

    pub fn has_target(&self) -> bool {
        !self.target.is_empty()
    }

    pub fn has_exclude_include_dirs(&self) -> bool {
        !self.exclude_dirs.is_empty() ||
        !self.exclude_files.is_empty() ||
        !self.include_dirs.is_empty() ||
        !self.include_files.is_empty()
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
    fn test_has_exclude_include_dirs() {
        todo!(" ");
        let args = CliArgs{
            source: "Some".to_string(),
            target: "Another".to_string(),
            include_dirs: vec!["".to_string()],
            include_files: vec!["".to_string()],
            exclude_dirs: vec!["d".to_string()],
            exclude_files: vec!["".to_string()],
        }.has_exclude_include_dirs();
        assert!(args);
    }
}