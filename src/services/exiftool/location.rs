// For more information visit the GPS Tags documentation from ExifTool: https://exiftool.org/TagNames/GPS.html

use exiftool::ExifToolError;

use super::ExifService;

impl<'a> ExifService<'a> {
    /// Sets the GPSDateStamp tag value
    /// Format: "YYYY:MM:DD"
    pub fn set_gps_date_stamp(&self, gps_date_stamp: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSDateStamp", gps_date_stamp)
    }

    /// Sets the GPSTimeStamp tag value
    /// Format: "HH:MM:SS"
    pub fn set_gps_time_stamp(&self, gps_time_stamp: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSTimeStamp", gps_time_stamp)
    }

    /// Sets the GPSLatitude tag value
    pub fn set_gps_latitude(&self, latitude: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSLatitude", latitude)
    }

    /// Sets the GPSLongitude tag value
    pub fn set_gps_longitude(&self, longitude: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSLongitude", longitude)
    }

    /// Sets the GPSAltitude tag value
    pub fn set_gps_altitude(&self, altitude: &str) -> Result<(), ExifToolError> {
        self.write_tag("GPSAltitude", altitude)
    }
}
