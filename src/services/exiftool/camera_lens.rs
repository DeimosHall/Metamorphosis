use exiftool::ExifToolError;

use super::ExifService;

impl<'a> ExifService<'a> {
    // ****************** Device ******************

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

    /// Returns the SerialNumber tag value
    pub fn serial_number(&self) -> Option<String> {
        self.read_tag("SerialNumber")
    }

    /// Sets the SerialNumber tag value
    pub fn set_serial_number(&self, serial_number: &str) -> Result<(), ExifToolError> {
        self.write_tag("SerialNumber", serial_number)
    }

    /// Returns the OwnerName tag value
    pub fn owner_name(&self) -> Option<String> {
        self.read_tag("OwnerName")
    }

    /// Sets the OwnerName tag value
    pub fn set_owner_name(&self, owner_name: &str) -> Result<(), ExifToolError> {
        self.write_tag("OwnerName", owner_name)
    }

    /// Returns the CameraSerialNumber tag value
    pub fn camera_serial_number(&self) -> Option<String> {
        self.read_tag("CameraSerialNumber")
    }

    /// Sets the CameraSerialNumber tag value
    pub fn set_camera_serial_number(
        &self,
        camera_serial_number: &str,
    ) -> Result<(), ExifToolError> {
        self.write_tag("CameraSerialNumber", camera_serial_number)
    }

    /// Returns the CameraFirmware tag value
    pub fn camera_firmware(&self) -> Option<String> {
        self.read_tag("CameraFirmware")
    }

    /// Sets the CameraFirmware tag value
    pub fn set_camera_firmware(&self, camera_firmware: &str) -> Result<(), ExifToolError> {
        self.write_tag("CameraFirmware", camera_firmware)
    }

    /// Returns the CameraLabel tag value
    pub fn camera_label(&self) -> Option<String> {
        self.read_tag("CameraLabel")
    }

    /// Sets the CameraLabel tag value
    pub fn set_camera_label(&self, camera_label: &str) -> Result<(), ExifToolError> {
        self.write_tag("CameraLabel", camera_label)
    }

    /// Returns the ImageUniqueID tag value
    pub fn image_unique_id(&self) -> Option<String> {
        self.read_tag("ImageUniqueID")
    }

    /// Sets the ImageUniqueID tag value
    pub fn set_image_unique_id(&self, image_unique_id: &str) -> Result<(), ExifToolError> {
        self.write_tag("ImageUniqueID", image_unique_id)
    }

    // ****************** Exposure Settings ******************

    /// Returns the ExposureTime tag value
    pub fn exposure_time(&self) -> Option<String> {
        self.read_tag("ExposureTime")
    }

    /// Sets the ExposureTime tag value
    pub fn set_exposure_time(&self, exposure_time: &str) -> Result<(), ExifToolError> {
        self.write_tag("ExposureTime", exposure_time)
    }
}
