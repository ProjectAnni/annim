use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AnnivSiteInfo {
    #[serde(rename(serialize = "site_name"))]
    name: String,
    description: String,
    #[serde(skip_deserializing, rename = "protocol_version", default = "protocol_version")]
    version: String,
    features: Vec<String>,
}

impl AnnivSiteInfo {
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }
}

fn protocol_version() -> String {
    "1".to_owned()
}