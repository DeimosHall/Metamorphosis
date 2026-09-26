use exiftool::ExifToolError;

use super::ExifService;

impl<'a> ExifService<'a> {
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
