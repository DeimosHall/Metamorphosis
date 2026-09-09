use exiftool::ExifToolError;
use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::services::exif::ExifService;

mod imp {
    use adw::subclass::bin::BinImpl;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(
        resource = "/dev/deimoshall/Metamorphosis/ui/views/image_metadata/image_location.ui"
    )]
    pub struct ImageLocationView {
        #[template_child]
        pub container: TemplateChild<gtk::Box>,
        #[template_child]
        pub gps_date_stamp_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub gps_time_stamp_entry: TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImageLocationView {
        const NAME: &'static str = "ImageLocationView";
        type Type = super::ImageLocationView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageLocationView {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for ImageLocationView {}
    impl BinImpl for ImageLocationView {}
}

glib::wrapper! {
    pub struct ImageLocationView(ObjectSubclass<imp::ImageLocationView>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImageLocationView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn show(&self) {
        self.imp().container.set_visible(true);
    }

    pub fn hide(&self) {
        self.imp().container.set_visible(false);
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

    pub fn load_from_file(&self, path: &str) {
        let exif = ExifService::new(path);
        let gps_date_stamp = exif.gps_date_stamp().unwrap_or_default();
        let gps_time_stamp = exif.gps_time_stamp().unwrap_or_default();

        self.set_gps_date_stamp(&gps_date_stamp);
        self.set_gps_time_stamp(&gps_time_stamp);
    }

    pub fn save_changes(&self, path: &str) -> Result<(), Vec<ExifToolError>> {
        let exif = ExifService::new(path);
        let gps_date_stamp = self.gps_date_stamp();
        let gps_time_stamp = self.gps_time_stamp();

        let mut errors = Vec::new();

        if let Err(e) = exif.set_gps_date_stamp(gps_date_stamp.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_gps_time_stamp(gps_time_stamp.as_str()) {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
