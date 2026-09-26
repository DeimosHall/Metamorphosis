use exiftool::ExifToolError;
use glib::GString;
use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::services::exiftool::ExifService;

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
        pub gps_date_stamp_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub gps_time_stamp_entry: TemplateChild<adw::EntryRow>,

        #[template_child]
        pub gps_latitude_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub gps_longitude_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub gps_altitude_entry: TemplateChild<adw::EntryRow>,
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
    pub fn show(&self) {
        self.imp().container.set_visible(true);
    }

    pub fn hide(&self) {
        self.imp().container.set_visible(false);
    }

    pub fn gps_date_stamp(&self) -> GString {
        self.imp().gps_date_stamp_entry.text()
    }

    pub fn set_gps_date_stamp(&self, gps_date_stamp: &str) {
        self.imp().gps_date_stamp_entry.set_text(gps_date_stamp);
    }

    pub fn gps_time_stamp(&self) -> GString {
        self.imp().gps_time_stamp_entry.text()
    }

    pub fn set_gps_time_stamp(&self, gps_time_stamp: &str) {
        self.imp().gps_time_stamp_entry.set_text(gps_time_stamp);
    }

    pub fn gps_latitude(&self) -> GString {
        self.imp().gps_latitude_entry.text()
    }

    pub fn set_gps_latitude(&self, gps_latitude: &str) {
        self.imp().gps_latitude_entry.set_text(gps_latitude);
    }

    pub fn gps_longitude(&self) -> GString {
        self.imp().gps_longitude_entry.text()
    }

    pub fn set_gps_longitude(&self, gps_longitude: &str) {
        self.imp().gps_longitude_entry.set_text(gps_longitude);
    }

    pub fn gps_altitude(&self) -> GString {
        self.imp().gps_altitude_entry.text()
    }

    pub fn set_gps_altitude(&self, gps_altitude: &str) {
        self.imp().gps_altitude_entry.set_text(gps_altitude);
    }

    pub fn load_from_file(&self, path: &str) {
        let exif = ExifService::new(path);
        let gps_date_stamp = exif.gps_date_stamp().unwrap_or_default();
        let gps_time_stamp = exif.gps_time_stamp().unwrap_or_default();
        let gps_latitude = exif.gps_latitude().unwrap_or_default();
        let gps_longitude = exif.gps_longitude().unwrap_or_default();
        let gps_altitude = exif.gps_altitude().unwrap_or_default();

        self.set_gps_date_stamp(&gps_date_stamp);
        self.set_gps_time_stamp(&gps_time_stamp);
        self.set_gps_latitude(&gps_latitude);
        self.set_gps_longitude(&gps_longitude);
        self.set_gps_altitude(&gps_altitude);
    }

    pub fn save_changes(&self, path: &str) -> Result<(), ExifToolError> {
        let exif = ExifService::new(path);

        exif.set_gps_date_stamp(self.gps_date_stamp().as_str())?;
        exif.set_gps_time_stamp(self.gps_time_stamp().as_str())?;
        exif.set_gps_latitude(self.gps_latitude().as_str())?;
        exif.set_gps_longitude(self.gps_longitude().as_str())?;
        exif.set_gps_altitude(self.gps_altitude().as_str())?;

        Ok(())
    }
}
