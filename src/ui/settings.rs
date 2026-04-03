use crate::filetypes::{CompressionType, FileType};

pub trait SettingsStore {
    fn save_window_size(&self) -> Result<(), glib::BoolError>;
    fn load_window_size(&self);
    fn save_options(&self) -> Result<(), glib::BoolError>;
    fn load_options(&self);
    fn save_selected_output(&self) -> Result<(), glib::BoolError>;
    fn load_selected_output(&self) -> FileType;
    fn save_selected_compression(&self) -> Result<(), glib::BoolError>;
    fn load_selected_compression(&self) -> CompressionType;
}