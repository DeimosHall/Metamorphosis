use std::{path::Path, sync::LazyLock};

use exiftool::{ExifTool, ExifToolError, g2::ExifData};

static EXIFTOOL: LazyLock<ExifTool> =
    LazyLock::new(|| ExifTool::with_executable(Path::new("/app/exiftool")).unwrap());

pub struct ExifService;

impl ExifService {
    pub fn read_all(path: String) {
        let path = Path::new(path.as_str());
        let exif_data: ExifData = EXIFTOOL.read_metadata(path, &["-g2"]).unwrap();
        println!("Parsed data: \n{:#?}", exif_data);
    }

    /// Get "Create Date" tag value
    ///
    /// Output Example: Some("2026:03:31 22:02:24")
    pub fn create_date(path: String) -> Option<String> {
        let path = Path::new(path.as_str());
        EXIFTOOL.read_tag(path, "CreateDate", &[]).ok()?
    }

    pub fn set_all_dates(path: String, date: String) -> Result<(), ExifToolError> {
        let path = Path::new(path.as_str());
        Ok(EXIFTOOL.write_tag(&path, "AllDates", date.as_str(), &["-overwrite_original"])?)
    }

    pub fn offset_time(path: String) -> Option<String> {
        let path = Path::new(path.as_str());
        EXIFTOOL.read_tag(path, "OffsetTime", &[]).ok()?
    }

    pub fn set_all_offset_times(path: String, offset: String) -> Result<(), ExifToolError> {
        let path = Path::new(path.as_str());
        EXIFTOOL.write_tag(path, "OffsetTime", offset.as_str(), &[])?;
        EXIFTOOL.write_tag(path, "OffsetTimeOriginal", offset.as_str(), &[])?;
        EXIFTOOL.write_tag(path, "OffsetTimeDigitized", offset.as_str(), &[])?;
        Ok(())
    }
}
