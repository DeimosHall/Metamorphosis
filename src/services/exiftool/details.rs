use exiftool::ExifToolError;

use super::ExifService;

impl<'a> ExifService<'a> {
    /// Sets the ProcessingSoftware tag value
    pub fn set_software(&self) -> Result<(), ExifToolError> {
        let software = format!("Metamorphosis {}", env!("CARGO_PKG_VERSION"));
        // self.write_tag("Software", software.as_str())?;
        self.write_tag("ProcessingSoftware", software.as_str())?;
        Ok(())
    }

    /// Sets the ImageDescription tag value
    pub fn set_image_description(&self, description: &str) -> Result<(), ExifToolError> {
        self.write_tag("ImageDescription", description)
    }

    /// Sets the Artist tag value
    pub fn set_artist(&self, artist: &str) -> Result<(), ExifToolError> {
        self.write_tag("Artist", artist)
    }
}
