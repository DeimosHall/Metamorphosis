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
    #[template(resource = "/dev/deimoshall/Metamorphosis/ui/views/image_metadata/image_details.ui")]
    pub struct ImageDetailsView {
        #[template_child]
        pub container: TemplateChild<gtk::Box>,
        #[template_child]
        pub image_description_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub artist_entry: TemplateChild<adw::EntryRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImageDetailsView {
        const NAME: &'static str = "ImageDetailsView";
        type Type = super::ImageDetailsView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageDetailsView {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for ImageDetailsView {}
    impl BinImpl for ImageDetailsView {}
}

glib::wrapper! {
    pub struct ImageDetailsView(ObjectSubclass<imp::ImageDetailsView>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImageDetailsView {
    pub fn show(&self) {
        self.imp().container.set_visible(true);
    }

    pub fn hide(&self) {
        self.imp().container.set_visible(false);
    }

    // Getters and setters
    text_field_accesors! {
        image_description => image_description_entry,
        artist => artist_entry,
    }

    pub fn set_metadata(&self, metadata: &Metadata) {
        self.set_image_description(metadata.image_description().unwrap_or_default());
        self.set_artist(metadata.artist().unwrap_or_default());
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn save_changes(&self, path: &str) -> Result<(), ExifToolError> {
        let exif = ExifService::new(path);

        exif.set_image_description(self.image_description().as_str())?;
        exif.set_artist(self.artist().as_str())?;
        // TODO: Let the user configure the software name
        exif.set_software()?;

        Ok(())
    }
}
