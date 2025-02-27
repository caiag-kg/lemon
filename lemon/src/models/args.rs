#[allow(dead_code)]
#[derive(Debug)]
pub struct CliArgs {
    pub source: String,
    pub target: String,
    pub include_dirs: Vec<String>,
    pub include_files: Vec<String>,
    pub exclude_dirs: Vec<String>,
    pub exclude_files: Vec<String>,
}

pub enum DirTypes {
    Exclude,
    Include,
}
