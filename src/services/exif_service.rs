use std::{path::Path, process::Command};

use exiftool::{ExifTool, g2::ExifData};

pub struct ExifService;

impl ExifService {
    pub fn exiftool_version() {
        let command = Command::new("/app/exiftool").arg("-ver").output().expect("Not found");
        println!("ExifTool version: {}", String::from_utf8_lossy(&command.stdout));
    }
    
    pub fn read_all(path: String) {
        let path = Path::new(path.as_str());
        let exiftool = ExifTool::with_executable(Path::new("/app/exiftool")).unwrap();

        let exif_data: ExifData = exiftool.read_metadata(path, &["-g2"]).unwrap();
        println!("Parsed data: \n{:#?}", exif_data);
    }
}
