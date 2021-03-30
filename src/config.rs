use crate::db::AnnivDbConfig;
use std::path::Path;
use serde::Deserialize;
use crate::models::info::AnnivSiteInfo;

#[derive(Deserialize)]
pub struct AnnivConfig {
    pub database: AnnivDbConfig,
    #[serde(rename = "site")]
    pub site_info: AnnivSiteInfo,
}

impl AnnivConfig {
    pub fn from_file<P: AsRef<Path>>(p: P) -> anyhow::Result<AnnivConfig> {
        let string = std::fs::read_to_string(p)?;
        let result = toml::from_str(&string)?;
        Ok(result)
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.site_info.has_feature(feature)
    }
}