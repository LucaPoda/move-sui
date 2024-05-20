use crate::fuzz::{
    options::{BuildMode, BuildOptions, FuzzDirWrapper},
    project::FuzzProject,
    RunCommand,
};
use anyhow::Result;
use clap::Parser;
use crate::fuzz::options::{CargoBuildOptions, MoveBuildOptions};

use move_package::BuildConfig;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Parser)]
pub struct Build {
    #[clap(flatten)]  
    pub build: BuildOptions,

    #[clap(flatten)] 
    pub fuzz_dir_wrapper: FuzzDirWrapper,

    /// Name of the fuzz target to build, or build all targets if not supplied
    pub target: Option<String>,

    
}

impl RunCommand for Build {
    fn run_command(&mut self,  path: &Option<PathBuf>, config: &BuildConfig)-> Result<()> {
        let project = FuzzProject::new(self.fuzz_dir_wrapper.fuzz_dir.to_owned())?;
        project.exec_build(BuildMode::Build, &self.build, self.target.as_deref(), path, config)
    }
}
