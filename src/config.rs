use crate::db::AnnivDbConfig;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnnivConfig {
    pub(crate) database: AnnivDbConfig,
}

impl AnnivConfig {
    pub fn from_file<P: AsRef<Path>>(p: P) -> anyhow::Result<AnnivConfig> {
        let string = std::fs::read_to_string(p)?;
        let result = toml::from_str(&string)?;
        Ok(result)
    }
}