pub mod add;
pub mod build;
pub mod check;
pub mod cmin;
pub mod coverage;
pub mod fmt;
pub mod init;
pub mod list;
pub mod run;
pub mod tmin;

pub use self::{
    add::Add, build::Build, check::Check, cmin::Cmin, coverage::Coverage, fmt::Fmt, init::Init,
    list::List, run::Run, tmin::Tmin,
};

use clap::*;
use std::str::FromStr;
use std::{fmt as stdfmt, path::PathBuf};
use std::fmt::Debug;
use move_package::BuildConfig;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Sanitizer {
    Address,
    Leak,
    Memory,
    Thread,
    None,
}

impl FromStr for Sanitizer {
    type Err = &'static str; // Or any other error type you prefer

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "address" => Ok(Sanitizer::Address),
            "leak" => Ok(Sanitizer::Leak),
            "memory" => Ok(Sanitizer::Memory),
            "thread" => Ok(Sanitizer::Thread),
            "none" => Ok(Sanitizer::None),
            _ => Err("Invalid sanitizer variant"),
        }
    }
}


impl stdfmt::Display for Sanitizer {
    fn fmt(&self, f: &mut stdfmt::Formatter) -> stdfmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sanitizer::Address => "address",
                Sanitizer::Leak => "leak",
                Sanitizer::Memory => "memory",
                Sanitizer::Thread => "thread",
                Sanitizer::None => "",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BuildMode {
    Build,
    Check,
}

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct BuildOptions {
    #[clap(name = "cargo-dev", short = 'D', long = "cargo-dev", global = true)]
    /// Build artifacts in development mode, without optimizations
    pub dev: bool,

    /// Build target with verbose output from `cargo build`
    #[clap(short = 'v', long)]
    pub verbose: bool,

    #[clap(flatten)] 
    /// cargo-specific build options
    pub cargo_options: CargoBuildOptions,
}

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct CargoBuildOptions {
    #[clap(short = 'O', long, conflicts_with = "cargo-dev")]
    /// Build artifacts in release mode, with optimizations
    pub release: bool,

    #[clap(short = 'a', long)]
    /// Build artifacts with debug assertions and overflow checks enabled (default if not -O)
    pub debug_assertions: bool,

    #[clap(long, conflicts_with_all = &["no-default-features", "features"])]
    /// Build artifacts with all Cargo features enabled
    pub all_features: bool,

    #[clap(name="no-default-features", long)]
    /// Build artifacts with default Cargo features disabled
    pub no_default_features: bool,

    #[clap(long)]
    /// Build artifacts with given Cargo feature enabled
    pub features: Option<String>,

    #[clap(short, long, default_value = "address")]
    /// Use a specific sanitizer
    pub sanitizer: Sanitizer,

    #[clap(long = "build-std")]
    /// Pass -Zbuild-std to Cargo, which will build the standard library with all the build
    /// settings for the fuzz target, including debug assertions, and a sanitizer if requested.
    /// Currently this conflicts with coverage instrumentation but -Zbuild-std enables detecting
    /// more bugs so this option defaults to true, but when using `cargo fuzz coverage` it
    /// defaults to false.
    pub build_std: bool,

    #[clap(short, long = "careful")]
    /// enable "careful" mode: inspired by https://github.com/RalfJung/cargo-careful, this enables
    /// building the fuzzing harness along with the standard library (implies --build-std) with
    /// debug assertions and extra const UB and init checks.
    pub careful_mode: bool,

    #[clap(long = "target", default_value(crate::fuzz::utils::default_target()))]
    /// Target triple of the fuzz targetJust
    pub triple: String,

    #[clap(short = 'Z', value_name = "FLAG")]
    /// Unstable (nightly-only) flags to Cargo
    pub unstable_flags: Vec<String>,

    #[clap(skip = false)]
    /// Instrument program code with source-based code coverage information.
    /// This build option will be automatically used when running `cargo fuzz coverage`.
    /// The option will not be shown to the user, which is ensured by the `skip` attribute.
    /// The attribute takes a default value `false`, ensuring that by default,
    /// the coverage option will be disabled).
    pub coverage: bool,

    /// Dead code is linked by default to prevent a potential error with some
    /// optimized targets. This flag allows you to opt out of it.
    #[clap(long)]
    pub strip_dead_code: bool,

    /// By default the 'cfg(fuzzing)' compilation configuration is set. This flag
    /// allows you to opt out of it.
    #[clap(long)]
    pub no_cfg_fuzzing: bool,

    #[clap(long)]
    /// Don't build with the `sanitizer-coverage-trace-compares` LLVM argument
    ///
    ///  Using this may improve fuzzer throughput at the cost of worse coverage accuracy.
    /// It also allows older CPUs lacking the `popcnt` instruction to use `cargo-fuzz`;
    /// the `*-trace-compares` instrumentation assumes that the instruction is
    /// available.
    pub no_trace_compares: bool,

    #[clap(long)]
    /// Cargo Home directoru todo: ckdn
    pub cargo_home: Option<String>,
    
    #[clap(long)]
    /// Cargo Home directoru todo: ckdn
    pub cargo_target_dir: Option<String>
}


impl FromStr for BuildOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let build_options: Self = s.parse()?;
        Ok(build_options)
    }
}

impl FromStr for CargoBuildOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cargo_build_options: Self = s.parse()?;
        Ok(cargo_build_options)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct MoveBuildOptions {
    #[clap(long)]
    /// Bytecode version to compile move code
    pub(crate) bytecode_version: Option<u32>,
    #[clap(long)]
    /// Only fetch dependency repos to MOVE_HOME
    pub(crate) fetch_deps_only: bool,
    #[clap(long)]
    /// Force recompilation of all packages
    pub(crate) force: bool,
    #[clap(long)]
    /// Skip fetching latest git dependencies
    pub(crate) skip_fetch_latest_git_deps: bool,
}

impl std::fmt::Display for BuildOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cargo_options)?;

        if self.dev {
            write!(f, " -D")?;
        }

        if self.verbose {
            write!(f, " -v")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for CargoBuildOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.release {
            write!(f, " -O")?;
        }

        if self.no_default_features {
            write!(f, " --no-default-features")?;
        }

        if self.all_features {
            write!(f, " --all-features")?;
        }

        if let Some(feature) = &self.features {
            write!(f, " --features={}", feature)?;
        }

        // Handling sanitizer
        match self.sanitizer {
            Sanitizer::None => write!(f, " --sanitizer=none")?,
            Sanitizer::Address => {}
            _ => write!(f, " --sanitizer={}", self.sanitizer)?,
        }

        if self.build_std {
            write!(f, " --build-std")?;
        }

        if self.careful_mode {
            write!(f, " --careful")?;
        }

        if self.coverage {
            write!(f, " --coverage")?;
        }

        if self.debug_assertions {
            write!(f, " --debug-assertions")?;
        }

        if self.strip_dead_code {
            write!(f, " --strip-dead-code")?;
        }

        if self.no_cfg_fuzzing {
            write!(f, " --no-cfg-fuzzing")?;
        }

        if self.no_trace_compares {
            write!(f, " --no-trace-compares")?;
        }

        if self.triple != crate::fuzz::utils::default_target() {
            write!(f, " --target={}", self.triple)?;
        }

        if let Some(cargo_home) = &self.cargo_home {
            write!(f, " --cargo-home={}", cargo_home)?;
        }
        if let Some(cargo_target_dir) = &self.cargo_target_dir {
            write!(f, " --cargo-home={}", cargo_target_dir)?;
        }

        for flag in &self.unstable_flags {
            write!(f, " -Z{}", flag)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct FuzzDirWrapper {
    /// The path to the fuzz project directory.
    #[clap(long)]
    pub fuzz_dir: Option<PathBuf>,
}

impl stdfmt::Display for FuzzDirWrapper {
    fn fmt(&self, f: &mut stdfmt::Formatter) -> stdfmt::Result {
        if let Some(ref elem) = self.fuzz_dir {
            write!(f, " --fuzz-dir={}", elem.display())?;
        }

        Ok(())
    }
}

impl FromStr for FuzzDirWrapper {
    type Err = anyhow::Error; // Or any other error type you prefer

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse the string as a path
        let path = if s.is_empty() {
            None
        } else {
            Some(PathBuf::from(s))
        };

        Ok(FuzzDirWrapper { fuzz_dir: path })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_build_options() {
        let default_cargo_opts = CargoBuildOptions {
            release: false,
            debug_assertions: false,
            no_default_features: false,
            all_features: false,
            features: None,
            sanitizer: Sanitizer::Address,
            build_std: false,
            careful_mode: false,
            triple: String::from(crate::fuzz::utils::default_target()),
            unstable_flags: Vec::new(),
            coverage: false,
            strip_dead_code: false,
            no_cfg_fuzzing: false,
            no_trace_compares: false,
            cargo_home: None,
            cargo_target_dir: None
        };

        let default_opts = BuildOptions {
            dev: false,
            verbose: false,
            cargo_options: default_cargo_opts.clone(),
        };

        let opts = vec![
            default_opts.clone(),
            BuildOptions {
                dev: true,
                ..default_opts.clone()
            },
            BuildOptions {
                cargo_options: CargoBuildOptions {
                    release: true,
                    ..default_cargo_opts.clone()
                },
                ..default_opts.clone()
            },
            BuildOptions {
                cargo_options: CargoBuildOptions {
                    debug_assertions: true,
                    ..default_cargo_opts.clone()
                },
                ..default_opts.clone()
            },
            BuildOptions {
                verbose: true,
                ..default_opts.clone()
            },
            BuildOptions {
                cargo_options: CargoBuildOptions {
                    no_default_features: true,
                    ..default_cargo_opts.clone()
                },
                ..default_opts.clone()
            },
            BuildOptions {
                cargo_options: CargoBuildOptions {
                    all_features: true,
                    ..default_cargo_opts.clone()
                },
                ..default_opts.clone()
            },
            BuildOptions {
                cargo_options: CargoBuildOptions {
                    features: Some(String::from("features")),
                    ..default_cargo_opts.clone()
                },
                ..default_opts.clone()
            },
            BuildOptions {
                cargo_options: CargoBuildOptions {
                    sanitizer: Sanitizer::None,
                    ..default_cargo_opts.clone()
                },
                ..default_opts.clone()
            },
            BuildOptions {
                cargo_options: CargoBuildOptions {
                    triple: String::from("custom_triple"),
                    ..default_cargo_opts.clone()
                },
                ..default_opts.clone()
            },
            BuildOptions {
                cargo_options: CargoBuildOptions {
                    unstable_flags: vec![String::from("unstable"), String::from("flags")],
                    ..default_cargo_opts.clone()
                },
                ..default_opts.clone()
            },
            BuildOptions {
                ..default_opts.clone()
            },
            default_opts.clone(), // With coverage false
        ];

        let mut i = 0;
        for case in opts {
            println!("{i}");
            i+=1;
            println!("{:?}", case);
            println!();
            println!("{:?}", BuildOptions::parse_from(case.to_string().split(' ')));
            println!();
            assert_eq!(case, BuildOptions::parse_from(case.to_string().split(' ')));
        }
    }
}