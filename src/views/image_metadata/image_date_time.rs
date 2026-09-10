use glib::GString;
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

        // Dates
        #[template_child]
        pub modify_date_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub date_time_original_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub create_date_entry: TemplateChild<gtk::Entry>,

        // Fractional seconds
        #[template_child]
        pub sub_sec_time_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub sub_sec_time_original_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub sub_sec_time_digitized_entry: TemplateChild<gtk::Entry>,

        // Timezone offsets
        #[template_child]
        pub offset_time_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub offset_time_original_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub offset_time_digitized_entry: TemplateChild<gtk::Entry>,
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

    pub fn date(&self) -> GString {
        self.imp().creation_date_entry.text()
    }

    pub fn set_date(&self, date: &str) {
        self.imp().creation_date_entry.set_text(date);
    }

    pub fn modify_date(&self) -> GString {
        self.imp().modify_date_entry.text()
    }

    pub fn set_modify_date(&self, modify_date: &str) {
        self.imp().modify_date_entry.set_text(modify_date);
    }

    pub fn date_time_original(&self) -> GString {
        self.imp().date_time_original_entry.text()
    }

    pub fn set_date_time_original(&self, date_time: &str) {
        self.imp().date_time_original_entry.set_text(date_time);
    }

    pub fn create_date(&self) -> GString {
        self.imp().create_date_entry.text()
    }

    pub fn set_create_date(&self, create_date: &str) {
        self.imp().create_date_entry.set_text(create_date);
    }

    pub fn sub_sec_time(&self) -> GString {
        self.imp().sub_sec_time_entry.text()
    }

    pub fn set_sub_sec_time(&self, sub_sec_time: &str) {
        self.imp().sub_sec_time_entry.set_text(sub_sec_time);
    }

    pub fn sub_sec_time_original(&self) -> GString {
        self.imp().sub_sec_time_original_entry.text()
    }

    pub fn set_sub_sec_time_original(&self, sub_sec_time_original: &str) {
        self.imp()
            .sub_sec_time_original_entry
            .set_text(sub_sec_time_original);
    }

    pub fn sub_sec_time_digitized(&self) -> GString {
        self.imp().sub_sec_time_digitized_entry.text()
    }

    pub fn set_sub_sec_time_digitized(&self, sub_sec_time_digitized: &str) {
        self.imp()
            .sub_sec_time_digitized_entry
            .set_text(sub_sec_time_digitized);
    }

    pub fn offset_time(&self) -> GString {
        self.imp().offset_time_entry.text()
    }

    pub fn set_offset_time(&self, offset_time: &str) {
        self.imp().offset_time_entry.set_text(offset_time);
    }

    pub fn offset_time_original(&self) -> GString {
        self.imp().offset_time_original_entry.text()
    }

    pub fn set_offset_time_original(&self, offset_time_original: &str) {
        self.imp()
            .offset_time_original_entry
            .set_text(offset_time_original);
    }

    pub fn offset_time_digitized(&self) -> GString {
        self.imp().offset_time_digitized_entry.text()
    }

    pub fn set_offset_time_digitized(&self, offset_time_digitized: &str) {
        self.imp()
            .offset_time_digitized_entry
            .set_text(offset_time_digitized);
    }

    // TODO: maybe these methods should go in a trait
    /// Populate UI fields using exif data from the given file
    pub fn load_from_file(&self, path: &str) {
        let exif = ExifService::new(path);
        let date = exif.create_date().unwrap_or_default();

        let modify_date = exif.modify_date().unwrap_or_default();
        let date_time_original = exif.date_time_original().unwrap_or_default();
        let create_date = exif.create_date().unwrap_or_default();
        let sub_sec_time = exif.sub_sec_time().unwrap_or_default();
        let sub_sec_time_original = exif.sub_sec_time_original().unwrap_or_default();
        let sub_sec_time_digitized = exif.sub_sec_time_digitized().unwrap_or_default();
        let offset_time = exif.offset_time().unwrap_or_default();
        let offset_time_original = exif.offset_time_original().unwrap_or_default();
        let offset_time_digitized = exif.offset_time_digitized().unwrap_or_default();

        self.set_date(&date);

        self.set_modify_date(&modify_date);
        self.set_date_time_original(&date_time_original);
        self.set_create_date(&create_date);
        self.set_sub_sec_time(&sub_sec_time);
        self.set_sub_sec_time_original(&sub_sec_time_original);
        self.set_sub_sec_time_digitized(&sub_sec_time_digitized);
        self.set_offset_time(&offset_time);
        self.set_offset_time_original(&offset_time_original);
        self.set_offset_time_digitized(&offset_time_digitized);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn save_changes(&self, path: &str) -> Result<(), ExifToolError> {
        let exif = ExifService::new(path);

        exif.set_all_dates(self.date().as_str())?;
        exif.set_modify_date(self.modify_date().as_str())?;
        exif.set_date_time_original(self.date_time_original().as_str())?;
        exif.set_create_date(self.create_date().as_str())?;
        exif.set_sub_sec_time(self.sub_sec_time().as_str())?;
        exif.set_sub_sec_time_original(self.sub_sec_time_original().as_str())?;
        exif.set_sub_sec_time_digitized(self.sub_sec_time_digitized().as_str())?;
        exif.set_offset_time(self.offset_time().as_str())?;
        exif.set_offset_time_original(self.offset_time_original().as_str())?;
        exif.set_offset_time_digitized(self.offset_time_digitized().as_str())?;
        exif.set_software()?;

        Ok(())
    }
}
