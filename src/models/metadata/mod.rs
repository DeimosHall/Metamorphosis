use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

mod image;

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
}

macro_rules! exif_tag {
    ($($field:ident => $tag:literal),+ $(,)?) => {
        pastey::paste! {
            $(
                pub fn $field(&self) -> Option<&str> {
                    self.get($tag)?.as_str()
                }
            )+
        }
    };
}

pub(crate) use exif_tag;
