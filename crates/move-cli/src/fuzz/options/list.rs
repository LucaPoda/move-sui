use crate::fuzz::{options::FuzzDirWrapper, project::FuzzProject, RunCommand};
use anyhow::Result;
use clap::Parser;

use move_package::BuildConfig;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Parser)]
pub struct List {
    #[clap(flatten)] 
    pub fuzz_dir_wrapper: FuzzDirWrapper,
}

impl RunCommand for List {
    fn run_command(&mut self,  path: &Option<PathBuf>, config: &BuildConfig)-> Result<()> {
        let project = FuzzProject::new(self.fuzz_dir_wrapper.fuzz_dir.to_owned())?;
        project.list_targets()
    }
}
