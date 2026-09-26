// For more information visit the GPS Tags documentation from ExifTool: https://exiftool.org/TagNames/GPS.html

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

    /// Returns the GPSLatitude tag value
    pub fn gps_latitude(&self) -> Option<String> {
        self.read_raw_tag("GPSLatitude")
    }

    /// Sets the GPSLatitude tag value
    pub fn set_gps_latitude(&self, latitude: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSLatitude", latitude)
    }

    /// Returns the GPSLongitude tag value
    pub fn gps_longitude(&self) -> Option<String> {
        self.read_raw_tag("GPSLongitude")
    }

    /// Sets the GPSLongitude tag value
    pub fn set_gps_longitude(&self, longitude: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSLongitude", longitude)
    }

    /// Returns the GPSAltitude tag value
    pub fn gps_altitude(&self) -> Option<String> {
        self.read_raw_tag("GPSAltitude")
    }

    /// Sets the GPSAltitude tag value
    pub fn set_gps_altitude(&self, altitude: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSAltitude", altitude)
    }
}
