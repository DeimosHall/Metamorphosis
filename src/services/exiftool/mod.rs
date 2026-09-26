use std::{path::Path, sync::LazyLock};

use exiftool::{ExifTool, ExifToolError, g2::ExifData};

mod camera_lens;
mod date_time;
mod details;
mod location;

static EXIFTOOL: LazyLock<ExifTool> =
    LazyLock::new(|| ExifTool::with_executable(Path::new("/app/exiftool")).unwrap());

pub struct ExifService<'a> {
    path: &'a Path,
}

impl<'a> ExifService<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            path: Path::new(path),
        }
    }

    /// Returns a tag value in String format
    /// Converts numbers and bools to String
    fn read_tag_with_args(&self, tag: &str, args: &[&str]) -> Option<String> {
        let value = EXIFTOOL.json_tag(self.path, tag, args).ok()?;
        match value {
            serde_json::Value::String(value) => Some(value),
            serde_json::Value::Number(value) => Some(value.to_string()),
            serde_json::Value::Bool(value) => Some(value.to_string()),
            _ => None,
        }
    }

    /// Returns a tag value in human-readable format
    /// Example:
    ///     - GPSLatitude: 19 deg 43' 15.49" N
    fn read_tag(&self, tag: &str) -> Option<String> {
        self.read_tag_with_args(tag, &[])
    }

    /// Returns a tag value in raw format
    /// Example:
    ///     - GPSLatitude: 19.7209694444444
    fn read_raw_tag(&self, tag: &str) -> Option<String> {
        self.read_tag_with_args(tag, &["-n"])
    }

    /// Writes a value to a tag
    fn write_tag(&self, tag: &str, value: &str) -> Result<(), ExifToolError> {
        EXIFTOOL.write_tag(self.path, tag, value, &["-overwrite_original"])
    }

    pub fn width(&self) -> Option<usize> {
        self.read_tag("ImageWidth").and_then(|w| w.parse().ok())
    }

    pub fn height(&self) -> Option<usize> {
        self.read_tag("ImageHeight").and_then(|h| h.parse().ok())
    }

    pub fn dimensions(&self) -> Option<(usize, usize)> {
        self.width().zip(self.height())
    }

    // TODO: used for testing purposes, delete later
    pub fn read_all(&self) {
        let exif_data: ExifData = EXIFTOOL.read_metadata(self.path, &["-g2"]).unwrap();
        println!("Parsed data: \n{:#?}", exif_data);
    }
}
