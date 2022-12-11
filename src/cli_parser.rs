use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CliParser {
    paths_in: Vec<PathBuf>,
    path_out: PathBuf,
}

impl CliParser {
    pub fn paths_in(&self) -> &Vec<PathBuf> {
        &self.paths_in
    }
    pub fn path_out(&self) -> &PathBuf {
        &self.path_out
    }
}
