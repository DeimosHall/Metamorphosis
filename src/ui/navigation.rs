pub trait StackNavigation {
    fn switch_to_stack_convert(&self);
    fn switch_to_stack_converting(&self);
    fn switch_to_stack_welcome(&self);
    fn switch_to_stack_invalid_image(&self);
    fn switch_to_stack_loading(&self);
    fn switch_back_from_loading(&self);
    fn switch_to_stack_loading_generally(&self);
}