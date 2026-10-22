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

    pub fn dimensions(&self) -> Option<(u64, u64)> {
        let width = self.tags.get("ImageWidth")?.as_number()?.as_u64();
        let height = self.tags.get("ImageHeight")?.as_number()?.as_u64();
        width.zip(height)
    }
}
