use std::path::PathBuf;

use crate::fuzz::{
    options::{BuildOptions, FuzzDirWrapper},
    project::FuzzProject,
    RunCommand,
};
use anyhow::{bail, Result};
use clap::Parser;

use move_package::BuildConfig;

#[derive(Clone, Debug, Parser)]
pub struct Coverage {
    #[clap(flatten)] 
    pub build: BuildOptions,

    #[clap(flatten)] 
    pub fuzz_dir_wrapper: FuzzDirWrapper,

    /// Sets the path to the LLVM bin directory. By default, it will use the one installed with rustc
    #[clap(long)]
    pub llvm_path: Option<PathBuf>,

    /// Name of the fuzz target
    pub target: String,

    /// Custom corpus directories or artifact files
    pub corpus: Vec<String>,

    #[clap(last(true))]
    /// Additional libFuzzer arguments passed through to the binary
    pub args: Vec<String>,
}

impl RunCommand for Coverage {
    fn run_command(&mut self,  path: &Option<PathBuf>, config: &BuildConfig)-> Result<()> {
        if self.build.cargo_options.build_std {
            bail!(
                "-Zbuild-std is currently incompatible with -Zinstrument-coverage, \
                see https://github.com/rust-lang/wg-cargo-std-aware/issues/63"
            );
        }
        let project = FuzzProject::new(self.fuzz_dir_wrapper.fuzz_dir.to_owned())?;
        self.build.cargo_options.coverage = true;
        project.exec_coverage(self, path, config)
    }
}
