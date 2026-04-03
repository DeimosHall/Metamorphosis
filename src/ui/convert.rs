use crate::{color::Color, filetypes::OutputType, magick::ResizeArgument, window::ResizeFilter};

pub trait ConvertArguments {
    fn get_quality_argument(&self) -> usize;
    fn get_dpi_argument(&self) -> usize;
    fn get_bgcolor_argument(&self) -> Color;
    fn get_filter_argument(&self) -> Option<ResizeFilter>;
    fn get_resize_argument(&self) -> ResizeArgument;
}

pub trait ConvertOperations {
    fn convert_start_wrapper(&self, save_format: OutputType, path: String);
    fn move_output(
        &self,
        save_format: OutputType,
        path: String,
        output_files: Vec<String>,
        dir_path: String,
    );
    fn convert_failed(&self, error_message: String, temp_dir_path: String);
    fn convert_success(&self, temp_dir_path: String, path: String, save_format: OutputType);
    fn convert_clean(&self, temp_dir_path: String);
    fn convert_cancel(&self);
}