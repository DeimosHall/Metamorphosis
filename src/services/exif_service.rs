use std::{path::Path, sync::LazyLock};

use exiftool::{ExifTool, ExifToolError, g2::ExifData};

static EXIFTOOL: LazyLock<ExifTool> =
    LazyLock::new(|| ExifTool::with_executable(Path::new("/app/exiftool")).unwrap());

pub struct ExifService;

impl ExifService {
    fn read_tag(path: &Path, tag: &str) -> Option<String> {
        EXIFTOOL.read_tag(path, tag, &[]).ok()?
    }

    fn write_tag(path: &Path, tag: &str, value: &str) -> Result<(), ExifToolError> {
        EXIFTOOL.write_tag(path, tag, value, &["-overwrite_original"])
    }

    pub fn read_all(path: String) {
        let path = Path::new(path.as_str());
        let exif_data: ExifData = EXIFTOOL.read_metadata(path, &["-g2"]).unwrap();
        println!("Parsed data: \n{:#?}", exif_data);
    }

    /// Get "Create Date" tag value
    ///
    /// Output Example: Some("2026:03:31 22:02:24")
    pub fn create_date(path: &Path) -> Option<String> {
        Self::read_tag(path, "CreateDate")
    }

    pub fn set_all_dates(path: &Path, date: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "AllDates", date)
    }

    pub fn offset_time(path: &Path) -> Option<String> {
        Self::read_tag(path, "OffsetTime")
    }

    pub fn set_all_offset_times(path: &Path, offset: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "OffsetTime", offset)?;
        Self::write_tag(path, "OffsetTimeOriginal", offset)?;
        Self::write_tag(path, "OffsetTimeDigitized", offset)?;
        Ok(())
    }
    
    pub fn set_software(path: &Path) -> Result<(), ExifToolError> {
        let software = format!("Metamorphosis {}", env!("CARGO_PKG_VERSION"));
        // Self::write_tag(path, "Software", software.as_str())?;
        Self::write_tag(path, "ProcessingSoftware", software.as_str())?;
        Ok(())
    }
}
