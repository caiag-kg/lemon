use std::error::Error;
use clap::{Arg, ArgMatches, Command};


pub fn get_matches() -> Result<ArgMatches, Box<dyn Error>> {
    todo!("Read args without label!");
    let matches = Command::new("lemon")
        .version("0.1.0")
        .author("Author: <Kubanychbek uulu Kairat>")
        .about("A CLI tool to copy a directory structure from source to destination, \
        with options to include or exclude specific directories and files.")
        .arg(
            Arg::new("source directory PATH")
                .short('s')
                .long("source")
                .value_name("SOURCE_PATH")
                .help("Specify the source directory to copy")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::new("target directory PATH")
            .short('t')
            .long("target")
            .value_name("TARGET_PATH")
            .help("Specify the target directory to copy")
            .index(2)
            .required(true)
        )        
        .arg(
            Arg::new("include_dirs")
                .long("include_dirs")
                .short('a')
                .value_name("INCLUDE_DIRS")
                .help("Include directories")
                .default_value(None)
                .index(3)
                .required(false)
        )
        .arg(
            Arg::new("include_files")
                .long("include_files")
                .short('b')
                .value_name("INCLUDE_FILES")
                .help("Include files")
                .default_value(None)
                .index(4)
                .required(false)
        )
        .arg(
            Arg::new("exclude_dirs")
                .long("exclude_dirs")
                .short('c')
                .value_name("EXCLUDE_DIRS")
                .help("Exclude directories")
                .default_value(None)
                .index(5)
                .required(false)
        )
        .arg(
            Arg::new("exclude_files")
                .long("exclude_files")
                .short('d')
                .value_name("EXCLUDE_FILES")
                .help("Exclude files")
                .default_value(None)
                .index(6)
                .required(false)

        ).get_matches();
    Ok(matches)
}