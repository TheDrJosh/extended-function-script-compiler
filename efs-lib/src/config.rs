use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub targets: Vec<PathBuf>,
    pub datapack_format: String,
    pub command_format: String,
}

impl Config {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: String::from("0.1.0"),
            targets: Default::default(),
            datapack_format: Default::default(),
            command_format: Default::default(),
        }
    }
}
