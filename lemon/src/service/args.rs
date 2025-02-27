use crate::models::{CliArgs, DirTypes};
use crate::service::questionnaire;
use clap::parser::ValuesRef;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;

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

        // let has_add_args = Self.has_add_args(DirTypes::Exclude)
        let mut cli_args = CliArgs {
            source: String::from(matches.get_one::<String>("source").unwrap()),
            target: String::from(matches.get_one::<String>("target").unwrap()),
            include_dirs,
            include_files,
            exclude_dirs,
            exclude_files,
        };

        // Merge excluded and included folders and files if they are exists:
        if cli_args.has_add_args(DirTypes::Exclude) && cli_args.has_add_args(DirTypes::Include) {
            cli_args.merge_x_includes()?;
        }

        Ok(cli_args)
    }

    pub fn get_matches() -> Result<ArgMatches, Box<dyn Error>> {
        let matches = Command::new("lemon")
            .version("0.1.0")
            .author("Author: <Kubanychbek uulu Kairat>")
            .about(
                "A CLI tool to copy a directory structure from source to destination, \
        with options to include or exclude specific directories and files.",
            )
            .arg(
                Arg::new("source")
                    .value_name("SOURCE_PATH")
                    .help("Specify the source directory to copy")
                    .index(1)
                    .required(true),
            )
            .arg(
                Arg::new("target")
                    .value_name("TARGET_PATH")
                    .help("Specify the target directory to copy")
                    .index(2)
                    .default_value("")
                    .required(false),
            )
            .arg(
                Arg::new("include_dirs")
                    .long("ind")
                    .short('a')
                    .value_name("INCLUDE_DIRS")
                    .help("Include directories")
                    .num_args(1..)
                    .default_value("")
                    .required(false),
            )
            .arg(
                Arg::new("include_files")
                    .long("inf")
                    .short('b')
                    .value_name("INCLUDE_FILES")
                    .help("Include files")
                    .num_args(1..)
                    .default_value("")
                    .required(false),
            )
            .arg(
                Arg::new("exclude_dirs")
                    .long("xd")
                    .short('c')
                    .value_name("EXCLUDE_DIRS")
                    .help("Exclude directories")
                    .num_args(1..)
                    .default_value("")
                    .required(false),
            )
            .arg(
                Arg::new("exclude_files")
                    .long("xf")
                    .short('d')
                    .value_name("EXCLUDE_FILES")
                    .help("Exclude files")
                    .num_args(1..)
                    .default_value("")
                    .required(false),
            )
            .get_matches();
        Ok(matches)
    }

    pub(crate) fn merge_x_includes(&mut self) -> Result<(), Box<dyn Error>> {
        // Will be called in self.has_add_args method!

        // Get similar dir names:
        let mut common_dirs: Vec<String> = Vec::new();
        for dir in &self.include_dirs {
            if self.exclude_dirs.contains(dir) {
                common_dirs.push(dir.clone());
            }
        }
        // Delete similar dir names from excluded & included dirs:
        self.include_dirs.retain(|x| !common_dirs.contains(x));
        self.exclude_dirs.retain(|x1| !common_dirs.contains(x1));

        // Get similar dir names:
        let mut common_files: Vec<String> = Vec::new();
        for f in &self.include_files {
            if self.exclude_files.contains(f) {
                common_files.push(f.clone());
            }
        }
        // Delete similar dir names from excluded & included files:
        self.include_files.retain(|x| !common_files.contains(x));
        self.exclude_files.retain(|x1| !common_files.contains(x1));

        Ok(())
    }
}

impl CliArgs {
    pub fn has_target(&self) -> bool {
        !self.target.trim().is_empty()
    }

    pub fn has_add_args(&mut self, dir_types: DirTypes) -> bool {
        let mut ress: Vec<bool> = Vec::new();
        let dirs_files = match dir_types {
            DirTypes::Exclude => [&self.exclude_dirs, &self.exclude_files],
            DirTypes::Include => [&self.include_dirs, &self.exclude_dirs],
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
}
