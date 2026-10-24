use crate::models::metadata::{Metadata, exif_tag};

impl Metadata {
    pub fn dimensions(&self) -> Option<(u64, u64)> {
        let width = self.get("ImageWidth")?.as_u64();
        let height = self.get("ImageHeight")?.as_u64();
        width.zip(height)
    }

    pub fn sub_sec_time(&self) -> Option<u64> {
        self.get("SubSecTime")?.as_u64()
    }

    pub fn sub_sec_time_original(&self) -> Option<u64> {
        self.get("SubSecTimeOriginal")?.as_u64()
    }

    pub fn sub_sec_time_digitized(&self) -> Option<u64> {
        self.get("SubSecTimeDigitized")?.as_u64()
    }

    pub fn gps_latitude(&self) -> Option<f64> {
        self.get("GPSLatitude")?.as_f64()
    }

    pub fn gps_longitude(&self) -> Option<f64> {
        self.get("GPSLongitude")?.as_f64()
    }

    pub fn gps_altitude(&self) -> Option<f64> {
        self.get("GPSAltitude")?.as_f64()
    }

    exif_tag! {
        // Date & Time
        modify_date => "ModifyDate",
        offset_time => "OffsetTime",
        date_time_original => "DateTimeOriginal",
        offset_time_original => "OffsetTimeOriginal",
        create_date => "CreateDate",
        offset_time_digitized => "OffsetTimeDigitized",
        // Location
        gps_date_stamp => "GPSDateStamp",
        gps_time_stamp => "GPSTimeStamp",
        // Details
        image_description => "ImageDescription",
        artist => "Artist",
    }
}
