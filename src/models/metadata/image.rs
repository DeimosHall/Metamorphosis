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

    // Camera & Lens

    pub fn max_aperture_value(&self) -> Option<f64> {
        self.get("MaxApertureValue")?.as_f64()
    }

    pub fn exposure_time(&self) -> Option<&str> {
        self.get("ExposureTime")?.as_str()
    }

    pub fn fnumber(&self) -> Option<f64> {
        self.get("FNumber")?.as_f64()
    }

    // TODO: add conversion rules and add ExposureMode
    pub fn exposure_program(&self) -> Option<u64> {
        self.get("ExposureProgram")?.as_u64()
    }

    pub fn iso(&self) -> Option<u64> {
        self.get("ISO")?.as_u64()
    }

    // TODO: add conversion rules
    pub fn focal_length(&self) -> Option<&str> {
        self.get("FocalLength")?.as_str()
    }

    pub fn exposure_compensation(&self) -> Option<&str> {
        self.get("ExposureCompensation")?.as_str()
    }

    // TODO: add conversion rules
    pub fn metering_mode(&self) -> Option<&str> {
        self.get("MeteringMode")?.as_str()
    }

    // TODO: Add conversion rules
    pub fn flash(&self) -> Option<&str> {
        self.get("Flash")?.as_str()
    }

    // TODO: add conversion rules
    pub fn white_balance(&self) -> Option<&str> {
        self.get("WhiteBalance")?.as_str()
    }

    // Location

    pub fn gps_latitude(&self) -> Option<&str> {
        self.get("GPSLatitude")?.as_str()
    }

    pub fn gps_longitude(&self) -> Option<&str> {
        self.get("GPSLongitude")?.as_str()
    }

    pub fn gps_altitude(&self) -> Option<&str> {
        self.get("GPSAltitude")?.as_str()
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
        // Camera & Lens
        make => "Make",
        model => "Model",
        serial_number => "SerialNumber",
        owner_name => "OwnerName",
        camera_serial_number => "CameraSerialNumber",
        camera_firmware => "CameraFirmware",
        camera_label => "CameraLabel",
        image_unique_id => "ImageUniqueID",
        lens_make => "LensMake",
        lens_model => "LensModel",
        lens_serial_number => "LensSerialNumber",
        lens_info => "LensInfo", // TODO: apply conversion rules
        // Details
        image_description => "ImageDescription",
        artist => "Artist",
    }
}
