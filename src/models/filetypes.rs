use std::slice::Iter;

#[derive(Clone, Copy, Debug, glib::Enum, PartialEq, Default, Eq, Hash)]
#[enum_type(name = "MetamorphosisFiletype")]
pub enum FileType {
    #[enum_value(name = "PNG")]
    Png,
    #[enum_value(name = "JPEG")]
    Jpeg,
    #[enum_value(name = "JPG")]
    Jpg,
    #[enum_value(name = "WEBP")]
    Webp,
    #[enum_value(name = "SVG")]
    Svg,
    #[enum_value(name = "HEIF")]
    Heif,
    #[enum_value(name = "HEIC")]
    Heic,
    #[enum_value(name = "BMP")]
    Bmp,
    #[enum_value(name = "AVIF")]
    Avif,
    #[enum_value(name = "JXL")]
    Jxl,
    #[enum_value(name = "TIFF")]
    Tiff,
    #[enum_value(name = "PDF")]
    Pdf,
    #[enum_value(name = "GIF")]
    Gif,
    #[enum_value(name = "ICO")]
    Ico,
    #[enum_value(name = "DDS")]
    Dds,
    #[enum_value(name = "Unknown")]
    #[default]
    Unknown,
}

use FileType::*;

impl FileType {
    pub fn is_input(&self) -> bool {
        matches!(
            self,
            Png | Jpg
                | Webp
                | Svg
                | Heif
                | Heic
                | Bmp
                | Avif
                | Jxl
                | Tiff
                | Pdf
                | Gif
                | Ico
                | Jpeg
                | Dds
        )
    }

    pub fn supports_pixbuf(&self) -> bool {
        !matches!(self, Pdf | Dds | Ico)
    }

    pub fn input_formats() -> Iter<'static, Self> {
        static FILETYPES: [FileType; 15] = [
            Png, Jpg, Jpeg, Webp, Svg, Heif, Heic, Bmp, Avif, Jxl, Tiff, Pdf, Gif, Ico, Dds,
        ];
        FILETYPES.iter()
    }

    pub fn as_mime(&self) -> &'static str {
        match self {
            Png => "image/png",
            Jpg => "image/jpeg",
            Jpeg => "image/jpeg",
            Webp => "image/webp",
            Svg => "image/svg+xml",
            Heif => "image/heif",
            Heic => "image/heic",
            Bmp => "image/bmp",
            Avif => "image/avif",
            Jxl => "image/jxl",
            Tiff => "image/tiff",
            Pdf => "application/pdf",
            Gif => "image/gif",
            Ico => "image/x-icon",
            Dds => "image/vnd-ms.dds",
            Unknown => "",
        }
    }

    pub fn from_mimetype(mimetype: &str) -> Option<Self> {
        match mimetype {
            "image/png" => Some(Png),
            "image/jpeg" => Some(Jpg),
            "image/jpg" => Some(Jpg),
            "image/webp" => Some(Webp),
            "image/svg+xml" => Some(Svg),
            "image/heif" => Some(Heif),
            "image/heic" => Some(Heic),
            "image/bmp" => Some(Bmp),
            "image/avif" => Some(Avif),
            "image/jxl" => Some(Jxl),
            "image/tiff" => Some(Tiff),
            "application/pdf" => Some(Pdf),
            "image/gif" => Some(Gif),
            "image/x-icon" => Some(Ico),
            "image/vnd-ms.dds" => Some(Dds),
            _ => None,
        }
    }

    pub fn as_extension(&self) -> &str {
        match self {
            Png => "png",
            Jpg => "jpg",
            Jpeg => "jpeg",
            Webp => "webp",
            Svg => "svg",
            Heif => "heif",
            Heic => "heic",
            Bmp => "bmp",
            Avif => "avif",
            Jxl => "jxl",
            Tiff => "tiff",
            Pdf => "pdf",
            Gif => "gif",
            Ico => "ico",
            Dds => "dds",
            Unknown => "",
        }
    }

    pub fn as_display_string(&self) -> String {
        self.as_extension().to_uppercase()
    }
}
