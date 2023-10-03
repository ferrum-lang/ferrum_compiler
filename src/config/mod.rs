use std::{default, path::PathBuf};

pub const DEFAULT_SRC_DIR: &str = "src";
pub const DEFAULT_FERRUM_OUT_DIR: &str = ".ferrum";
pub const DEFAULT_RUST_GEN_DIR: &str = ".ferrum/compiled_rust";
pub const DEFAULT_BUILDS_DIR: &str = ".ferrum/builds";

#[derive(Debug, Clone)]
pub struct Config {
    pub src_dir: PathBuf,
    pub ferrum_out_dir: PathBuf,
    pub rust_gen_dir: PathBuf,
    pub builds_dir: PathBuf,
    pub binary_file: PathBuf,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        return ConfigBuilder::default();
    }

    pub fn default_from_project_root(root: impl Into<PathBuf>) -> Self {
        return Self::builder().default_from_project_root(root).build();
    }
}

impl default::Default for Config {
    fn default() -> Self {
        return ConfigBuilder::default().build();
    }
}

impl<P: Into<PathBuf>> From<P> for Config {
    fn from(value: P) -> Self {
        return Self::default_from_project_root(value.into());
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConfigBuilder {
    src_dir: Option<PathBuf>,
    ferrum_out_dir: Option<PathBuf>,
    rust_gen_dir: Option<PathBuf>,
    builds_dir: Option<PathBuf>,
    binary_file: Option<PathBuf>,
}

impl ConfigBuilder {
    pub fn default_from_project_root(mut self, root: impl Into<PathBuf>) -> Self {
        let root: PathBuf = root.into();

        self.src_dir = Some(root.join(DEFAULT_SRC_DIR));
        self.ferrum_out_dir = Some(root.join(DEFAULT_FERRUM_OUT_DIR));
        self.rust_gen_dir = Some(root.join(DEFAULT_RUST_GEN_DIR));
        self.builds_dir = Some(root.join(DEFAULT_BUILDS_DIR));

        // TODO
        self.binary_file = Some(root.join(DEFAULT_BUILDS_DIR).join("dev/out"));

        return self;
    }

    pub fn build(self) -> Config {
        return Config {
            src_dir: self.src_dir.unwrap_or(DEFAULT_SRC_DIR.into()),
            ferrum_out_dir: self.ferrum_out_dir.unwrap_or(DEFAULT_FERRUM_OUT_DIR.into()),
            rust_gen_dir: self.rust_gen_dir.unwrap_or(DEFAULT_RUST_GEN_DIR.into()),
            builds_dir: self.builds_dir.unwrap_or(DEFAULT_BUILDS_DIR.into()),

            // TODO
            binary_file: self
                .binary_file
                .unwrap_or(PathBuf::from(DEFAULT_BUILDS_DIR).join("dev/out")),
        };
    }
}
