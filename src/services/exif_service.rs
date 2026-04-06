use std::{path::Path, process::Command};

use exiftool::{ExifTool, ExifToolError, g2::ExifData};

pub struct ExifService;

impl ExifService {
    pub async fn exiftool_version() {
        let command = Command::new("/app/exiftool").arg("-ver").output().expect("Not found");
        println!("ExifTool version: {}", String::from_utf8_lossy(&command.stdout));
    }
    
    pub async fn read_all(path: String) {
        let path = Path::new(path.as_str());
        let exiftool = ExifTool::with_executable(Path::new("/app/exiftool")).unwrap();

        let exif_data: ExifData = exiftool.read_metadata(path, &["-g2"]).unwrap();
        println!("Parsed data: \n{:#?}", exif_data);
    }
    
    /// Get "Create Date" tag value
    /// 
    /// Output Example: Some("2026:03:31 22:02:24")
    pub async fn create_date(path: String) -> Option<String> {
        let exiftool = ExifTool::with_executable(Path::new("/app/exiftool")).unwrap();
        let path = Path::new(path.as_str());
        exiftool.read_tag(path, "CreateDate", &[]).ok()?
    }
    
    pub async fn set_create_date(path: String, date: String) -> Result<(), ExifToolError> {
        let path = Path::new(path.as_str());
        let exiftool = ExifTool::with_executable(Path::new("/app/exiftool"))?;
        Ok(exiftool.write_tag(&path, "CreateDate", date.as_str(), &["-overwrite_original"])?)
    }
}
