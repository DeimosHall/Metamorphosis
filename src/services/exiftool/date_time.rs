use exiftool::ExifToolError;
use jiff::{Unit, Zoned};

use super::ExifService;

impl<'a> ExifService<'a> {
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

    /// Sets the DateTimeOriginal tag value
    /// Format: "YYYY:MM:DD HH:MM:SS"
    pub fn set_date_time_original(&self, date_time_original: &str) -> Result<(), ExifToolError> {
        self.write_tag("DateTimeOriginal", date_time_original)
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

    /// Sets the SubSecTime tag value
    pub fn set_sub_sec_time(&self, sub_sec_time: &str) -> Result<(), ExifToolError> {
        self.write_tag("SubSecTime", sub_sec_time)
    }

    /// Sets the SubSecTimeOriginal tag value
    pub fn set_sub_sec_time_original(
        &self,
        sub_sec_time_original: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("SubSecTimeOriginal", sub_sec_time_original)
    }

    /// Sets the SubSecTimeDigitized tag value
    pub fn set_sub_sec_time_digitized(
        &self,
        sub_sec_time_digitized: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("SubSecTimeDigitized", sub_sec_time_digitized)
    }

    // ****************** Timezone offsets ******************

    /// Sets the OffsetTime tag value (for ModifyDate)
    /// Offset time format: "HH:MM"
    pub fn set_offset_time(&self, offset_time: &str) -> Result<(), ExifToolError> {
        self.write_tag("OffsetTime", offset_time)
    }
    /// Sets the OffsetTimeOriginal tag value (for DateTimeOrginal)
    /// Offset time format: "HH:MM"
    pub fn set_offset_time_original(
        &self,
        offset_time_original: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("OffsetTimeOriginal", offset_time_original)
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
