use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct Metadata {
    #[serde(flatten)]
    tags: BTreeMap<String, Value>,
}

impl Metadata {
    pub fn debug(&self) {
        for (tag, value) in &self.tags {
            log::debug!("{}: {}", tag, value);
        }
    }

    pub fn get(&self, tag: &str) -> Option<&Value> {
        self.tags.get(tag)
    }

    pub fn dimensions(&self) -> Option<(u64, u64)> {
        let width = self.get("ImageWidth")?.as_u64();
        let height = self.get("ImageHeight")?.as_u64();
        width.zip(height)
    }

    pub fn image_descripton(&self) -> Option<&str> {
        self.get("ImageDescription")?.as_str()
    }

    pub fn artist(&self) -> Option<&str> {
        self.get("Artist")?.as_str()
    }
}
