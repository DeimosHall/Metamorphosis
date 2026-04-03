pub trait WindowUI {
    fn update_options(&self);
    fn update_output_options(&self);
    fn update_compression_options(&self);
    fn update_advanced_options(&self);
    fn update_width_from_height(&self);
    fn update_height_from_width(&self);
    fn update_resize(&self);
    fn update_full_image_container(&self);
    fn update_image_container(&self, count: usize, remaining_visible: bool);
}