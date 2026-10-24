use exiftool::ExifToolError;
use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::{
    models::metadata::Metadata, services::exiftool::ExifService,
    views::image_metadata::text_field_accesors,
};

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

    text_field_accesors! {
        // Date & Time
        gps_date_stamp => gps_date_stamp_entry,
        gps_time_stamp => gps_time_stamp_entry,
        // Location
        gps_latitude => gps_latitude_entry,
        gps_longitude => gps_longitude_entry,
        gps_altitude => gps_altitude_entry,
    }

    pub fn set_metadata(&self, metadata: &Metadata) {
        self.set_gps_date_stamp(metadata.gps_date_stamp().unwrap_or_default());
        self.set_gps_time_stamp(metadata.gps_time_stamp().unwrap_or_default());

        if let Some(gps_latitude) = metadata.gps_latitude() {
            self.set_gps_latitude(&gps_latitude.to_string());
        }

        if let Some(gps_longitude) = metadata.gps_longitude() {
            self.set_gps_longitude(&gps_longitude.to_string());
        }

        if let Some(gps_altitude) = metadata.gps_altitude() {
            self.set_gps_altitude(&gps_altitude.to_string());
        }
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
