use exiftool::ExifToolError;
use gtk::{glib, prelude::*, subclass::prelude::*};

use derivative::Derivative;
use gtk_macros::CompositeTemplate;
use adw::subclass::bin::BinImpl;

use crate::services::exif::ExifService;

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derive(Default)]
    #[template(resource = "/dev/deimoshall/Metamorphosis/ui/views/apply/image_advanced_tab.ui")]
    pub struct ImageAdvancedTab {
        #[template_child]
        pub container: TemplateChild<gtk::Box>,
        // Dates
        #[template_child]
        pub modify_date_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub date_time_original_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub create_date_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub gps_date_stamp_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub gps_time_stamp_entry: TemplateChild<gtk::Entry>,

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
    impl ObjectSubclass for ImageAdvancedTab {
        const NAME: &'static str = "ImageAdvancedTab";
        type Type = super::ImageAdvancedTab;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageAdvancedTab {}
    impl WidgetImpl for ImageAdvancedTab {}
    impl BinImpl for ImageAdvancedTab {}
}

glib::wrapper! {
    pub struct ImageAdvancedTab(ObjectSubclass<imp::ImageAdvancedTab>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImageAdvancedTab {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn show(&self) {
        self.imp().container.set_visible(true);
    }

    pub fn hide(&self) {
        self.imp().container.set_visible(false);
    }

    pub fn modify_date(&self) -> String {
        self.imp().modify_date_entry.text().to_string()
    }

    pub fn set_modify_date(&self, modify_date: &str) {
        self.imp().modify_date_entry.set_text(modify_date);
    }

    pub fn date_time_original(&self) -> String {
        self.imp().date_time_original_entry.text().to_string()
    }

    pub fn set_date_time_original(&self, date_time: &str) {
        self.imp().date_time_original_entry.set_text(date_time);
    }

    pub fn create_date(&self) -> String {
        self.imp().create_date_entry.text().to_string()
    }

    pub fn set_create_date(&self, create_date: &str) {
        self.imp().create_date_entry.set_text(create_date);
    }

    pub fn gps_date_stamp(&self) -> String {
        self.imp().gps_date_stamp_entry.text().to_string()
    }

    pub fn set_gps_date_stamp(&self, gps_date_stamp: &str) {
        self.imp().gps_date_stamp_entry.set_text(gps_date_stamp);
    }

    pub fn gps_time_stamp(&self) -> String {
        self.imp().gps_time_stamp_entry.text().to_string()
    }

    pub fn set_gps_time_stamp(&self, gps_time_stamp: &str) {
        self.imp().gps_time_stamp_entry.set_text(gps_time_stamp);
    }

    pub fn sub_sec_time(&self) -> String {
        self.imp().sub_sec_time_entry.text().to_string()
    }

    pub fn set_sub_sec_time(&self, sub_sec_time: &str) {
        self.imp().sub_sec_time_entry.set_text(sub_sec_time);
    }

    pub fn sub_sec_time_original(&self) -> String {
        self.imp().sub_sec_time_original_entry.text().to_string()
    }

    pub fn set_sub_sec_time_original(&self, sub_sec_time_original: &str) {
        self.imp().sub_sec_time_original_entry.set_text(sub_sec_time_original);
    }

    pub fn sub_sec_time_digitized(&self) -> String {
        self.imp().sub_sec_time_digitized_entry.text().to_string()
    }

    pub fn set_sub_sec_time_digitized(&self, sub_sec_time_digitized: &str) {
        self.imp().sub_sec_time_digitized_entry.set_text(sub_sec_time_digitized);
    }

    pub fn offset_time(&self) -> String {
        self.imp().offset_time_entry.text().to_string()
    }

    pub fn set_offset_time(&self, offset_time: &str) {
        self.imp().offset_time_entry.set_text(offset_time);
    }

    pub fn offset_time_original(&self) -> String {
        self.imp().offset_time_original_entry.text().to_string()
    }

    pub fn set_offset_time_original(&self, offset_time_original: &str) {
        self.imp().offset_time_original_entry.set_text(offset_time_original);
    }

    pub fn offset_time_digitized(&self) -> String {
        self.imp().offset_time_digitized_entry.text().to_string()
    }

    pub fn set_offset_time_digitized(&self, offset_time_digitized: &str) {
        self.imp().offset_time_digitized_entry.set_text(offset_time_digitized);
    }

    pub fn load_from_file(&self, path: String) {
        let exif = ExifService::new(&path);
        let modify_date = exif.modify_date().unwrap_or_default();
        let date_time_original = exif.date_time_original().unwrap_or_default();
        let create_date = exif.create_date().unwrap_or_default();
        let gps_date_stamp = exif.gps_date_stamp().unwrap_or_default();
        let gps_time_stamp = exif.gps_time_stamp().unwrap_or_default();
        let sub_sec_time = exif.sub_sec_time().unwrap_or_default();
        let sub_sec_time_original = exif.sub_sec_time_original().unwrap_or_default();
        let sub_sec_time_digitized = exif.sub_sec_time_digitized().unwrap_or_default();
        let offset_time = exif.offset_time().unwrap_or_default();
        let offset_time_original = exif.offset_time_original().unwrap_or_default();
        let offset_time_digitized = exif.offset_time_digitized().unwrap_or_default();

        self.set_modify_date(&modify_date);
        self.set_date_time_original(&date_time_original);
        self.set_create_date(&create_date);
        self.set_gps_date_stamp(&gps_date_stamp);
        self.set_gps_time_stamp(&gps_time_stamp);
        self.set_sub_sec_time(&sub_sec_time);
        self.set_sub_sec_time_original(&sub_sec_time_original);
        self.set_sub_sec_time_digitized(&sub_sec_time_digitized);
        self.set_offset_time(&offset_time);
        self.set_offset_time_original(&offset_time_original);
        self.set_offset_time_digitized(&offset_time_digitized);
    }

    pub fn apply_changes(&self, path: &String) -> Result<(), Vec<ExifToolError>> {
        let exif = ExifService::new(&path);
        let modify_date = self.modify_date();
        let date_time_original = self.date_time_original();
        let create_date = self.create_date();
        let gps_date_stamp = self.gps_date_stamp();
        let gps_time_stamp = self.gps_time_stamp();
        let sub_sec_time = self.sub_sec_time();
        let sub_sec_time_original = self.sub_sec_time_original();
        let sub_sec_time_digitized = self.sub_sec_time_digitized();
        let offset_time = self.offset_time();
        let offset_time_original = self.offset_time_original();
        let offset_time_digitized = self.offset_time_digitized();

        let mut errors = Vec::new();

        if let Err(e) = exif.set_modify_date(modify_date.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_date_time_original(date_time_original.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_create_date(create_date.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_gps_date_stamp(gps_date_stamp.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_gps_time_stamp(gps_time_stamp.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_sub_sec_time(sub_sec_time.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_sub_sec_time_original(sub_sec_time_original.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_sub_sec_time_digitized(sub_sec_time_digitized.as_str()) {           
            errors.push(e);
        }

        if let Err(e) = exif.set_offset_time(offset_time.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_offset_time_original(offset_time_original.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_offset_time_digitized(offset_time_digitized.as_str()) {
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
