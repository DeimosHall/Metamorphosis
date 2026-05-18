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

    // ****************** Dates ******************

    pub fn read_all(path: String) {
        let path = Path::new(path.as_str());
        let exif_data: ExifData = EXIFTOOL.read_metadata(path, &["-g2"]).unwrap();
        println!("Parsed data: \n{:#?}", exif_data);
    }

    pub fn modify_date(path: &Path) -> Option<String> {
        Self::read_tag(path, "ModifyDate")
    }

    pub fn set_modify_date(path: &Path, modify_date: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "ModifyDate", modify_date)
    }

    pub fn date_time_original(path: &Path) -> Option<String> {
        Self::read_tag(path, "DateTimeOriginal")
    }

    pub fn set_date_time_original(
        path: &Path,
        date_time_original: &str,
    ) -> Result<(), ExifToolError> {
        Self::write_tag(path, "DateTimeOriginal", date_time_original)
    }

    /// Returns the CreateDate tag value
    ///
    /// Format: "YYYY:MM:DD HH:MM:SS" (e.g., "2026:03:31 22:02:24")
    pub fn create_date(path: &Path) -> Option<String> {
        Self::read_tag(path, "CreateDate")
    }

    pub fn set_create_date(path: &Path, create_date: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "CreateDate", create_date)
    }

    pub fn gps_date_stamp(path: &Path) -> Option<String> {
        Self::read_tag(path, "GPSDateStamp")
    }

    pub fn set_gps_date_stamp(path: &Path, gps_date_stamp: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "GPSDateStamp", gps_date_stamp)
    }

    pub fn gps_time_stamp(path: &Path) -> Option<String> {
        Self::read_tag(path, "GPSTimeStamp")
    }

    pub fn set_gps_time_stamp(path: &Path, gps_time_stamp: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "GPSTimeStamp", gps_time_stamp)
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

    // ****************** Fractional seconds ******************
    pub fn sub_sec_time(path: &Path) -> Option<String> {
        Self::read_tag(path, "SubSecTime")
    }

    pub fn set_sub_sec_time(path: &Path, sub_sec_time: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "SubSecTime", sub_sec_time)
    }

    pub fn sub_sec_time_original(path: &Path) -> Option<String> {
        Self::read_tag(path, "SubSecTimeOriginal")
    }

    pub fn set_sub_sec_time_original(
        path: &Path,
        sub_sec_time_original: &str,
    ) -> Result<(), ExifToolError> {
        Self::write_tag(path, "SubSecTimeOriginal", sub_sec_time_original)
    }

    pub fn sub_sec_time_digitized(path: &Path) -> Option<String> {
        Self::read_tag(path, "SubSecTimeDigitized")
    }

    pub fn set_sub_sec_time_digitized(path: &Path, sub_sec_time_digitized: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "SubSecTimeDigitized", sub_sec_time_digitized)
    }

    // ****************** Timezone offsets ******************

    /// Returns the OffSetTime tag value
    ///
    /// Format: "HH:MM"
    pub fn offset_time(path: &Path) -> Option<String> {
        Self::read_tag(path, "OffsetTime")
    }

    pub fn set_offset_time(path: &Path, offset_time: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "OffsetTime", offset_time)
    }

    pub fn offset_time_original(path: &Path) -> Option<String> {
        Self::read_tag(path, "OffsetTimeOriginal")
    }

    pub fn set_offset_time_original(path: &Path, offset_time_original: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "OffsetTimeOriginal", offset_time_original)
    }

    pub fn offset_time_digitized(path: &Path) -> Option<String> {
        Self::read_tag(path, "OffsetTimeDigitized")
    }

    pub fn set_offset_time_digitized(path: &Path, offset_time_digitized: &str) -> Result<(), ExifToolError> {
        Self::write_tag(path, "OffsetTimeDigitized", offset_time_digitized)
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
