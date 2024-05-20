use crate::fuzz::project::{FuzzProject, Manifest};
use crate::fuzz::{options::FuzzDirWrapper, RunCommand};
use anyhow::Result;
use clap::*;

use move_package::BuildConfig;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Parser)]
pub struct Add {
    #[clap(flatten)] 
    pub fuzz_dir_wrapper: FuzzDirWrapper,

    /// Name of the new fuzz target
    pub target: String,
}

impl RunCommand for Add {
    fn run_command(&mut self,  path: &Option<PathBuf>, config: &BuildConfig)-> Result<()> {
        let project = FuzzProject::new(self.fuzz_dir_wrapper.fuzz_dir.to_owned())?;
        let manifest = Manifest::parse()?;
        project.add_target(self, &manifest)
    }
}
