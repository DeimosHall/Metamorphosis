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

    /// Returns the CreateDate tag value
    ///
    /// Format: "YYYY:MM:DD HH:MM:SS" (e.g., "2026:03:31 22:02:24")
    pub fn create_date(path: &Path) -> Option<String> {
        Self::read_tag(path, "CreateDate")
    }

    /// Sets the following tag values:
    /// - CreateDate
    /// - DateTimeOrginal
    /// - ModifyDate
    ///
    /// Format: "YYYY:MM:DD HH:MM:SS"
    pub fn set_all_dates(path: &Path, date: &str) -> Result<(), ExifToolError> {
        // TODO: also set gps date
        Self::write_tag(path, "AllDates", date)
    }

    /// Returns the OffSetTime tag value
    /// 
    /// Format: "HH:MM"
    pub fn offset_time(path: &Path) -> Option<String> {
        Self::read_tag(path, "OffsetTime")
    }

    /// Sets the following tag values:
    /// - OffsetTime
    /// - OffsetTimeOriginal
    /// - OffsetTimeDigitized
    ///
    /// Format: "HH:MM" (e.g., "02:00", "-06:00")
    pub fn set_all_offset_times(path: &Path, offset: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "OffsetTime", offset)?;
        Self::write_tag(path, "OffsetTimeOriginal", offset)?;
        Self::write_tag(path, "OffsetTimeDigitized", offset)?;
        Ok(())
    }
    
    /// Sets the ProcessingSoftware tag
    pub fn set_software(path: &Path) -> Result<(), ExifToolError> {
        let software = format!("Metamorphosis {}", env!("CARGO_PKG_VERSION"));
        // Self::write_tag(path, "Software", software.as_str())?;
        Self::write_tag(path, "ProcessingSoftware", software.as_str())?;
        Ok(())
    }
 
    /// Returns the ImageDescription tag value
    pub fn image_description(path: &Path) -> Option<String> {
        Self::read_tag(path, "ImageDescription")
    }
    
    /// Sets the ImageDescription tag
    pub fn set_image_description(path: &Path, description: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "ImageDescription", description)
    }

    /// Returns the Make tag value
    pub fn make(path: &Path) -> Option<String> {
        Self::read_tag(path, "Make")
    }

    /// Sets the Make tag value
    pub fn set_make(path: &Path, make: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "Make", make)
    }

    /// Returns the Model tag value
    pub fn model(path: &Path) -> Option<String> {
        Self::read_tag(path, "Model")
    }

    /// Sets the Model tag value
    pub fn set_model(path: &Path, model: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "Model", model)
    }
}
