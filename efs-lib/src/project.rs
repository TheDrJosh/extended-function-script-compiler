use std::{
    collections::HashMap,
    fs::{self, DirBuilder},
    path::PathBuf,
};

use crate::config::Config;

pub struct Project {
    pub files: HashMap<PathBuf, String>,
    pub config: Config,
}

impl Project {
    pub fn new(name: String, path: PathBuf) -> anyhow::Result<()> {
        DirBuilder::new().recursive(true).create(path.join("src"))?;

        let config = Config::new(name);

        fs::write(
            path.join("efs-config.toml"),
            toml::to_string_pretty(&config)?,
        )?;
        Ok(())
    }

    pub fn open(path: PathBuf) -> anyhow::Result<Self> {
        let config: Config = toml::from_str(&fs::read_to_string(path.join("efs-config.toml"))?)?;

        let paths = glob::glob(format!("{}/src/**/*.efs", path.display()).as_str())?;

        let files = paths
            .filter_map(Result::ok)
            .map(|path| (path.clone(), fs::read_to_string(path).unwrap_or_default()))
            .collect();

        Ok(Self { files, config })
    }
}
