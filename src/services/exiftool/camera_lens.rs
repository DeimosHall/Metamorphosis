use exiftool::ExifToolError;

use crate::services::exiftool::exif_tag;

use super::ExifService;

impl<'a> ExifService<'a> {
    // ****************** Device ******************
    /// Sets the Make tag value
    pub fn set_make(&self, make: &str) -> Result<(), ExifToolError> {
        self.write_tag("Make", make)
    }

    /// Sets the Model tag value
    pub fn set_model(&self, model: &str) -> Result<(), ExifToolError> {
        self.write_tag("Model", model)
    }

    /// Sets the SerialNumber tag value
    pub fn set_serial_number(&self, serial_number: &str) -> Result<(), ExifToolError> {
        self.write_tag("SerialNumber", serial_number)
    }

    /// Sets the OwnerName tag value
    pub fn set_owner_name(&self, owner_name: &str) -> Result<(), ExifToolError> {
        self.write_tag("OwnerName", owner_name)
    }

    /// Sets the CameraSerialNumber tag value
    pub fn set_camera_serial_number(
        &self,
        camera_serial_number: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("CameraSerialNumber", camera_serial_number)
    }

    /// Sets the CameraFirmware tag value
    pub fn set_camera_firmware(&self, camera_firmware: &str) -> Result<(), ExifToolError> {
        self.write_tag("CameraFirmware", camera_firmware)
    }

    /// Sets the CameraLabel tag value
    pub fn set_camera_label(&self, camera_label: &str) -> Result<(), ExifToolError> {
        self.write_tag("CameraLabel", camera_label)
    }

    /// Sets the ImageUniqueID tag value
    pub fn set_image_unique_id(&self, image_unique_id: &str) -> Result<(), ExifToolError> {
        self.write_tag("ImageUniqueID", image_unique_id)
    }

    exif_tag! {
        // Lens
        lens_make => "LensMake",
        lens_model => "LensModel",
        lens_serial_number => "LensSerialNumber",
        lens_info => "LensInfo",
        max_aperture_value => "MaxApertureValue",
        // Exposure Settings
        exposure_time => "ExposureTime",
        fnumber => "FNumber",
        exposure_program => "ExposureProgram",
        iso => "ISO",
        focal_length => "FocalLength",
        exposure_compensation => "ExposureCompensation",
        metering_mode => "MeteringMode",
        flash => "Flash",
        white_balance => "WhiteBalance",
    }
}
