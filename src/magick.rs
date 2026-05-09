use crate::models::filetypes::FileType;
use gettextrs::gettext;
use itertools::Itertools;
use std::sync::atomic::{AtomicUsize, Ordering};

pub async fn count_frames(path: String) -> Result<(usize, Option<(usize, usize)>), ()> {
    let command = tokio::process::Command::new("magick")
        .stdout(std::process::Stdio::piped())
        .arg("identify")
        .arg(path)
        .output()
        .await;

    match command {
        Ok(output) => match std::str::from_utf8(&output.stdout) {
            Ok(output_string) => {
                let lines = output_string.lines().collect_vec();
                let count = lines.len();
                let dims = lines
                    .first()
                    .and_then(|line| {
                        let dimension_match = regex::Regex::new(r" \d+x\d+ ").unwrap().find(line);

                        dimension_match.map(|m| {
                            let dims = m
                                .as_str()
                                .trim()
                                .split('x')
                                .map(|n| n.parse::<usize>())
                                .collect_vec();
                            match dims[..] {
                                [Ok(width), Ok(height)] => Some((width, height)),
                                _ => None,
                            }
                        })
                    })
                    .flatten();
                Ok((count, dims))
            }
            _ => Err(()),
        },
        _ => Err(()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JobFile {
    pub id: usize,
    pub desired_name: Option<String>,
    pub file_extension: FileType,
}

static FILE_COUNT: AtomicUsize = AtomicUsize::new(0);

impl JobFile {
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
                "TEMPORARY_SWITCHEROO_{}.{}",
                self.id,
                self.file_extension.as_extension()
            ),
        }
    }
}
