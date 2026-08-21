use crate::models::filetypes::FileType;
use gettextrs::gettext;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JobFile {
    pub id: usize,
    pub desired_name: Option<String>,
    pub file_extension: FileType,
}

static FILE_COUNT: AtomicUsize = AtomicUsize::new(0);

impl JobFile {
    // TODO: refactor this into a service
    pub fn from_clipboard() -> Self {
        let id = FILE_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        Self {
            id,
            desired_name: Some(format!("{}.png", gettext("Pasted Image"))),
            file_extension: FileType::Png,
        }
    }

    pub fn as_filename(&self) -> String {
        match &self.desired_name {
            Some(desired_name) => desired_name.to_owned(),
            None => format!(
                "TEMPORARY_METAMORPHOSIS_{}.{}",
                self.id,
                self.file_extension.as_extension()
            ),
        }
    }
}
