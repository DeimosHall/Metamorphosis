use exiftool::ExifToolError;
use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::services::exif::ExifService;

mod imp {
    use adw::subclass::bin::BinImpl;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/deimoshall/Metamorphosis/ui/views/image_metadata/image_details.ui")]
    pub struct ImageDetailsView {
        #[template_child]
        pub container: TemplateChild<gtk::Box>,
        #[template_child]
        pub image_description_entry: TemplateChild<gtk::Entry>,
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
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn show(&self) {
        self.imp().container.set_visible(true);
    }

    pub fn hide(&self) {
        self.imp().container.set_visible(false);
    }

    pub fn description(&self) -> String {
        self.imp().image_description_entry.text().to_string()
    }

    pub fn set_description(&self, description: &str) {
        self.imp().image_description_entry.set_text(description);
    }

    pub fn load_file(&self, path: &str) {
        let exif = ExifService::new(path);
        let description = exif.image_description().unwrap_or_default();

        self.set_description(&description);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn save_changes(&self, path: &str) -> Result<(), Vec<ExifToolError>> {
        let exif = ExifService::new(path);
        let description = self.description();

        let mut errors = Vec::new();

        if let Err(e) = exif.set_image_description(description.as_str()) {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
