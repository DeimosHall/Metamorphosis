use gtk::{glib, prelude::*, subclass::prelude::*};

use derivative::Derivative;
use exiftool::ExifToolError;
use gtk_macros::CompositeTemplate;
use adw::subclass::bin::BinImpl;

use crate::services::exif::ExifService;

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derivative(Default)]
    #[template(resource = "/dev/deimoshall/Metamorphosis/ui/views/apply/image_general_tab.ui")]
    pub struct ImageGeneralTab {
        #[template_child]
        pub creation_date_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub offset_time_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub manufacturer_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub model_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub image_description_entry: TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImageGeneralTab {
        const NAME: &'static str = "ImageGeneralTab";
        type Type = super::ImageGeneralTab;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageGeneralTab {}
    impl WidgetImpl for ImageGeneralTab {}
    impl BinImpl for ImageGeneralTab {}
}

glib::wrapper! {
    pub struct ImageGeneralTab(ObjectSubclass<imp::ImageGeneralTab>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImageGeneralTab {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn date(&self) -> String {
        self.imp().creation_date_entry.text().to_string()
    }

    pub fn offset(&self) -> String {
        self.imp().offset_time_entry.text().to_string()
    }

    pub fn manufacturer(&self) -> String {
        self.imp().manufacturer_entry.text().to_string()
    }

    pub fn model(&self) -> String {
        self.imp().model_entry.text().to_string()
    }

    pub fn description(&self) -> String {
        self.imp().image_description_entry.text().to_string()
    }

    pub fn set_date(&self, date: &str) {
        self.imp().creation_date_entry.set_text(date);
    }

    pub fn set_offset(&self, offset: &str) {
        self.imp().offset_time_entry.set_text(offset);
    }

    pub fn set_manufacturer(&self, manufacturer: &str) {
        self.imp().manufacturer_entry.set_text(manufacturer);
    }

    pub fn set_model(&self, model: &str) {
        self.imp().model_entry.set_text(model);
    }

    pub fn set_description(&self, description: &str) {
        self.imp().image_description_entry.set_text(description);
    }

    // TODO: maybe these methods should go in a trait
    /// Populate UI fields using exif data from the given file
    pub fn load_from_file(&self, path: String) {
        let exif = ExifService::new(&path);
        let date = exif.create_date().unwrap_or_default();
        let offset = exif.offset_time().unwrap_or_default();
        let manufacturer = exif.make().unwrap_or_default();
        let model = exif.model().unwrap_or_default();
        let description = exif.image_description().unwrap_or_default();

        self.set_date(&date);
        self.set_offset(&offset);
        self.set_manufacturer(&manufacturer);
        self.set_model(&model);
        self.set_description(&description);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn apply_changes(&self, path: &String) -> Result<(), Vec<ExifToolError>> {
        let exif = ExifService::new(&path);
        let date = self.date();
        let offset = self.offset();
        let manufacturer = self.manufacturer();
        let model = self.model();
        let description = self.description();

        let mut errors = Vec::new();

        if let Err(e) = exif.set_all_dates(date.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_all_offset_times(offset.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_make(manufacturer.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_model(model.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_image_description(description.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_software() {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
