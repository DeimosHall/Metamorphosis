use crate::input_file::InputFile;

pub trait FileOperations {
    fn add_dialog(&self);
    fn open_files(&self, files: Vec<Option<InputFile>>);
    fn save_error(&self, error: Option<&str>);
    fn save_files(&self);
    fn open_load(&self);
    fn open_error(&self, error: Option<&str>);
    fn add_success_wrapper(&self, files: Vec<InputFile>);
}