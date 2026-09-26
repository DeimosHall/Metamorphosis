use exiftool::ExifToolError;
use jiff::{Unit, Zoned};

use super::ExifService;

impl<'a> ExifService<'a> {
    // ****************** Dates ******************

    /// Returns the ModifyDate tag value
    /// Format: "YYYY:MM:DD HH:MM:SS"
    pub fn modify_date(&self) -> Option<String> {
        self.read_tag("ModifyDate")
    }

    /// Sets the ModifyDate tag value
    /// Format: "YYYY:MM:DD HH:MM:SS"
    pub fn set_modify_date(&self, modify_date: &str) -> Result<(), ExifToolError> {
        self.write_tag("ModifyDate", modify_date)
    }

    /// Sets the ModifyDate tag value as today
    // TODO: move this out from the service layer
    pub fn set_modify_date_as_today(&self) -> Result<(), ExifToolError> {
        let now = Zoned::now().round(Unit::Second).expect("Should never fail");
        let current_date = now.date().to_string().replace("-", ":");
        let current_time = now.time();
        let date_time = format!("{} {}", current_date, current_time);
        self.set_modify_date(date_time.as_str())
    }

    /// Returns the DateTimeOriginal tag value
    /// Format: "YYYY:MM:DD HH:MM:SS"
    pub fn date_time_original(&self) -> Option<String> {
        self.read_tag("DateTimeOriginal")
    }

    /// Sets the DateTimeOriginal tag value
    /// Format: "YYYY:MM:DD HH:MM:SS"
    pub fn set_date_time_original(&self, date_time_original: &str) -> Result<(), ExifToolError> {
        self.write_tag("DateTimeOriginal", date_time_original)
    }

    /// Returns the CreateDate tag value
    /// Format: "YYYY:MM:DD HH:MM:SS" (e.g., "2026:03:31 22:02:24")
    pub fn create_date(&self) -> Option<String> {
        self.read_tag("CreateDate")
    }

    /// Sets the CreateDate tag value
    /// Format: "YYYY:MM:DD HH:MM:SS"
    pub fn set_create_date(&self, create_date: &str) -> Result<(), ExifToolError> {
        self.write_tag("CreateDate", create_date)
    }

    /// Sets the following tag values:
    /// - `CreateDate` to the given date
    /// - `DateTimeOrginal` to the given date
    /// - `ModifyDate` as today
    ///
    /// Date format: "YYYY:MM:DD HH:MM:SS"
    // TODO: move this out of the service layer
    pub fn set_all_dates(&self, date: &str) -> Result<(), ExifToolError> {
        self.write_tag("AllDates", date)?;
        self.set_modify_date_as_today()
    }

    // ****************** Fractional seconds ******************

    /// Returns the SubSecTime tag value
    pub fn sub_sec_time(&self) -> Option<String> {
        self.read_tag("SubSecTime")
    }

    /// Sets the SubSecTime tag value
    pub fn set_sub_sec_time(&self, sub_sec_time: &str) -> Result<(), ExifToolError> {
        self.write_tag("SubSecTime", sub_sec_time)
    }

    /// Returns the SubSecTimeOriginal tag value
    pub fn sub_sec_time_original(&self) -> Option<String> {
        self.read_tag("SubSecTimeOriginal")
    }

    /// Sets the SubSecTimeOriginal tag value
    pub fn set_sub_sec_time_original(
        &self,
        sub_sec_time_original: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("SubSecTimeOriginal", sub_sec_time_original)
    }

    /// Returns the SubSecTimeDigitized tag value
    pub fn sub_sec_time_digitized(&self) -> Option<String> {
        self.read_tag("SubSecTimeDigitized")
    }

    /// Sets the SubSecTimeDigitized tag value
    pub fn set_sub_sec_time_digitized(
        &self,
        sub_sec_time_digitized: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("SubSecTimeDigitized", sub_sec_time_digitized)
    }

    // ****************** Timezone offsets ******************

    /// Returns the OffSetTime tag value (from ModifyDate)
    /// Format: "HH:MM"
    pub fn offset_time(&self) -> Option<String> {
        self.read_tag("OffsetTime")
    }

    /// Sets the OffsetTime tag value (for ModifyDate)
    /// Offset time format: "HH:MM"
    pub fn set_offset_time(&self, offset_time: &str) -> Result<(), ExifToolError> {
        self.write_tag("OffsetTime", offset_time)
    }

    /// Returns the OffsetTimeOriginal tag value (from DateTimeOrginal)
    /// Format: "HH:MM"
    pub fn offset_time_original(&self) -> Option<String> {
        self.read_tag("OffsetTimeOriginal")
    }

    /// Sets the OffsetTimeOriginal tag value (for DateTimeOrginal)
    /// Offset time format: "HH:MM"
    pub fn set_offset_time_original(
        &self,
        offset_time_original: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("OffsetTimeOriginal", offset_time_original)
    }

    /// Returns the OffsetTimeDigitized tag value (from CreateDate)
    /// Format: "HH:MM"
    pub fn offset_time_digitized(&self) -> Option<String> {
        self.read_tag("OffsetTimeDigitized")
    }

    /// Sets the OffsetTimeDigitized tag value (for CreateDate)
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
    // TODO: move this out of the service layer
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
}
