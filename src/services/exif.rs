use std::{path::Path, sync::LazyLock};

use exiftool::{ExifTool, ExifToolError, g2::ExifData};
use jiff::{Unit, Zoned};

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
    ///
    /// Converts numbers and bools to String
    fn read_tag(&self, tag: &str) -> Option<String> {
        let value = EXIFTOOL.json_tag(self.path, tag, &[]).ok()?;
        match value {
            serde_json::Value::String(value) => Some(value),
            serde_json::Value::Number(value) => Some(value.to_string()),
            serde_json::Value::Bool(value) => Some(value.to_string()),
            _ => None,
        }
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

    // ****************** Dates ******************

    // TODO: used for testing purposes, delete later
    pub fn read_all(&self) {
        let exif_data: ExifData = EXIFTOOL.read_metadata(self.path, &["-g2"]).unwrap();
        println!("Parsed data: \n{:#?}", exif_data);
    }

    /// Returns the ModifyDate tag value
    ///
    /// Format: "YYYY:MM:DD HH:MM:SS"
    pub fn modify_date(&self) -> Option<String> {
        self.read_tag("ModifyDate")
    }

    /// Sets the ModifyDate tag value
    ///
    /// Date format: "YYYY:MM:DD HH:MM:SS"
    pub fn set_modify_date(&self, modify_date: &str) -> Result<(), ExifToolError> {
        self.write_tag("ModifyDate", modify_date)
    }

    /// Sets the ModifyDate tag value as today
    pub fn set_modify_date_as_today(&self) -> Result<(), ExifToolError> {
        let now = Zoned::now().round(Unit::Second).expect("Should never fail");
        let current_date = now.date().to_string().replace("-", ":");
        let current_time = now.time();
        let date_time = format!("{} {}", current_date, current_time);
        self.set_modify_date(date_time.as_str())
    }

    pub fn date_time_original(&self) -> Option<String> {
        self.read_tag("DateTimeOriginal")
    }

    pub fn set_date_time_original(&self, date_time_original: &str) -> Result<(), ExifToolError> {
        self.write_tag("DateTimeOriginal", date_time_original)
    }

    /// Returns the CreateDate tag value
    ///
    /// Format: "YYYY:MM:DD HH:MM:SS" (e.g., "2026:03:31 22:02:24")
    pub fn create_date(&self) -> Option<String> {
        self.read_tag("CreateDate")
    }

    pub fn set_create_date(&self, create_date: &str) -> Result<(), ExifToolError> {
        self.write_tag("CreateDate", create_date)
    }

    pub fn gps_date_stamp(&self) -> Option<String> {
        self.read_tag("GPSDateStamp")
    }

    pub fn set_gps_date_stamp(&self, gps_date_stamp: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSDateStamp", gps_date_stamp)
    }

    pub fn gps_time_stamp(&self) -> Option<String> {
        self.read_tag("GPSTimeStamp")
    }

    pub fn set_gps_time_stamp(&self, gps_time_stamp: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSTimeStamp", gps_time_stamp)
    }

    /// Sets the following tag values:
    /// - `CreateDate` to the given date
    /// - `DateTimeOrginal` to the given date
    /// - `ModifyDate` as today
    ///
    /// Date format: "YYYY:MM:DD HH:MM:SS"
    pub fn set_all_dates(&self, date: &str) -> Result<(), ExifToolError> {
        // TODO: also set gps date if selected by user
        self.write_tag("AllDates", date)?;
        self.set_modify_date_as_today()
    }

    // ****************** Fractional seconds ******************
    pub fn sub_sec_time(&self) -> Option<String> {
        self.read_tag("SubSecTime")
    }

    pub fn set_sub_sec_time(&self, sub_sec_time: &str) -> Result<(), ExifToolError> {
        self.write_tag("SubSecTime", sub_sec_time)
    }

    pub fn sub_sec_time_original(&self) -> Option<String> {
        self.read_tag("SubSecTimeOriginal")
    }

    pub fn set_sub_sec_time_original(
        &self,
        sub_sec_time_original: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("SubSecTimeOriginal", sub_sec_time_original)
    }

    pub fn sub_sec_time_digitized(&self) -> Option<String> {
        self.read_tag("SubSecTimeDigitized")
    }

    pub fn set_sub_sec_time_digitized(
        &self,
        sub_sec_time_digitized: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("SubSecTimeDigitized", sub_sec_time_digitized)
    }

    // ****************** Timezone offsets ******************

    /// Returns the OffSetTime tag value (from ModifyDate)
    ///
    /// Format: "HH:MM"
    pub fn offset_time(&self) -> Option<String> {
        self.read_tag("OffsetTime")
    }

    /// Sets the OffsetTime tag value (for ModifyDate)
    ///
    /// Offset time format: "HH:MM"
    pub fn set_offset_time(&self, offset_time: &str) -> Result<(), ExifToolError> {
        self.write_tag("OffsetTime", offset_time)
    }

    /// Returns the OffsetTimeOriginal tag value (from DateTimeOrginal)
    ///
    /// Format: "HH:MM"
    pub fn offset_time_original(&self) -> Option<String> {
        self.read_tag("OffsetTimeOriginal")
    }

    /// Sets the OffsetTimeOriginal tag value (for DateTimeOrginal)
    ///
    /// Offset time format: "HH:MM"
    pub fn set_offset_time_original(
        &self,
        offset_time_original: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("OffsetTimeOriginal", offset_time_original)
    }

    /// Returns the OffsetTimeDigitized tag value (from CreateDate)
    ///
    /// Format: "HH:MM"
    pub fn offset_time_digitized(&self) -> Option<String> {
        self.read_tag("OffsetTimeDigitized")
    }

    /// Sets the OffsetTimeDigitized tag value (for CreateDate)
    ///
    /// Offset time format: "HH:MM"
    pub fn set_offset_time_digitized(
        &self,
        offset_time_digitized: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("OffsetTimeDigitized", offset_time_digitized)
    }

    /// Sets the following tag values:
    /// - OffsetTime (ModifyDate) as the local time offset
    /// - OffsetTimeOriginal (DateTimeOrginal) as the given value
    /// - OffsetTimeDigitized (CreateDate) as the given value
    ///
    /// Offset time format: "HH:MM" (e.g., "02:00", "-06:00")
    pub fn set_all_offset_times(&self, offset: &str) -> Result<(), ExifToolError> {
        let now = Zoned::now().round(Unit::Second).expect("Should never fail");
        let local_offset = now.offset().to_string();

        // Append ":00" if for example, the given value is like "+05"
        let local_offset = if local_offset.len() <= 3 {
            format!("{}:00", local_offset)
        } else {
            local_offset
        };

        self.set_offset_time(local_offset.as_str())?;
        self.set_offset_time_original(offset)?;
        self.set_offset_time_digitized(offset)
    }

    /// Sets the ProcessingSoftware tag
    pub fn set_software(&self) -> Result<(), ExifToolError> {
        let software = format!("Metamorphosis {}", env!("CARGO_PKG_VERSION"));
        // self.write_tag("Software", software.as_str())?;
        self.write_tag("ProcessingSoftware", software.as_str())?;
        Ok(())
    }

    /// Returns the ImageDescription tag value
    pub fn image_description(&self) -> Option<String> {
        self.read_tag("ImageDescription")
    }

    /// Sets the ImageDescription tag
    pub fn set_image_description(&self, description: &str) -> Result<(), ExifToolError> {
        self.write_tag("ImageDescription", description)
    }

    /// Returns the Make tag value
    pub fn make(&self) -> Option<String> {
        self.read_tag("Make")
    }

    /// Sets the Make tag value
    pub fn set_make(&self, make: &str) -> Result<(), ExifToolError> {
        self.write_tag("Make", make)
    }

    /// Returns the Model tag value
    pub fn model(&self) -> Option<String> {
        self.read_tag("Model")
    }

    /// Sets the Model tag value
    pub fn set_model(&self, model: &str) -> Result<(), ExifToolError> {
        self.write_tag("Model", model)
    }
}
