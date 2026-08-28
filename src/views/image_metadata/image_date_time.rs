use gtk::{glib, prelude::*, subclass::prelude::*};

use adw::subclass::bin::BinImpl;
use derivative::Derivative;
use exiftool::ExifToolError;
use gtk_macros::CompositeTemplate;

use crate::services::exif::ExifService;

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derivative(Default)]
    #[template(
        resource = "/dev/deimoshall/Metamorphosis/ui/views/image_metadata/image_date_time.ui"
    )]
    pub struct ImageDateTimeView {
        #[template_child]
        pub container: TemplateChild<gtk::Box>,
        #[template_child]
        pub creation_date_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub offset_time_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub manufacturer_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub model_entry: TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImageDateTimeView {
        const NAME: &'static str = "ImageDateTimeView";
        type Type = super::ImageDateTimeView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageDateTimeView {}
    impl WidgetImpl for ImageDateTimeView {}
    impl BinImpl for ImageDateTimeView {}
}

glib::wrapper! {
    pub struct ImageDateTimeView(ObjectSubclass<imp::ImageDateTimeView>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImageDateTimeView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn show(&self) {
        self.imp().container.set_visible(true);
    }

    pub fn hide(&self) {
        self.imp().container.set_visible(false);
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

    // TODO: maybe these methods should go in a trait
    /// Populate UI fields using exif data from the given file
    pub fn load_from_file(&self, path: &str) {
        let exif = ExifService::new(path);
        let date = exif.create_date().unwrap_or_default();
        let offset = exif.offset_time_digitized().unwrap_or_default();
        let manufacturer = exif.make().unwrap_or_default();
        let model = exif.model().unwrap_or_default();

        self.set_date(&date);
        self.set_offset(&offset);
        self.set_manufacturer(&manufacturer);
        self.set_model(&model);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn save_changes(&self, path: &str) -> Result<(), Vec<ExifToolError>> {
        let exif = ExifService::new(path);
        let date = self.date();
        let offset = self.offset();
        let manufacturer = self.manufacturer();
        let model = self.model();

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
