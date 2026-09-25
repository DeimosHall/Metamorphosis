use exiftool::ExifToolError;

use super::ExifService;

impl<'a> ExifService<'a> {
    /// Returns the GPSDateStamp tag value
    /// Format: "YYYY:MM:DD"
    pub fn gps_date_stamp(&self) -> Option<String> {
        self.read_tag("GPSDateStamp")
    }

    /// Sets the GPSDateStamp tag value
    /// Format: "YYYY:MM:DD"
    pub fn set_gps_date_stamp(&self, gps_date_stamp: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSDateStamp", gps_date_stamp)
    }

    /// Returns the GPSTimeStamp tag value
    /// Format: "HH:MM:SS"
    pub fn gps_time_stamp(&self) -> Option<String> {
        self.read_tag("GPSTimeStamp")
    }

    /// Sets the GPSTimeStamp tag value
    /// Format: "HH:MM:SS"
    pub fn set_gps_time_stamp(&self, gps_time_stamp: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSTimeStamp", gps_time_stamp)
    }
}
