use exiftool::ExifToolError;
use gtk::{glib, prelude::*, subclass::prelude::*};

use adw::subclass::bin::BinImpl;
use derivative::Derivative;
use gtk_macros::CompositeTemplate;

use crate::services::exif::ExifService;

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative, Default)]
    #[template(
        resource = "/dev/deimoshall/Metamorphosis/ui/views/image_metadata/image_camera_lens.ui"
    )]
    pub struct ImageCameraLensView {
        #[template_child]
        pub container: TemplateChild<gtk::Box>,
        #[template_child]
        pub manufacturer_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub model_entry: TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImageCameraLensView {
        const NAME: &'static str = "ImageCameraLensView";
        type Type = super::ImageCameraLensView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageCameraLensView {}
    impl WidgetImpl for ImageCameraLensView {}
    impl BinImpl for ImageCameraLensView {}
}

glib::wrapper! {
    pub struct ImageCameraLensView(ObjectSubclass<imp::ImageCameraLensView>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImageCameraLensView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn show(&self) {
        self.imp().container.set_visible(true);
    }

    pub fn hide(&self) {
        self.imp().container.set_visible(false);
    }

    pub fn manufacturer(&self) -> String {
        self.imp().manufacturer_entry.text().to_string()
    }

    pub fn set_manufacturer(&self, manufacturer: &str) {
        self.imp().manufacturer_entry.set_text(manufacturer);
    }

    pub fn model(&self) -> String {
        self.imp().model_entry.text().to_string()
    }

    pub fn set_model(&self, model: &str) {
        self.imp().model_entry.set_text(model);
    }

    pub fn load_from_file(&self, path: &str) {
        let exif = ExifService::new(path);
        let manufacturer = exif.make().unwrap_or_default();
        let model = exif.model().unwrap_or_default();

        self.set_manufacturer(&manufacturer);
        self.set_model(&model);
    }

    pub fn save_changes(&self, path: &str) -> Result<(), Vec<ExifToolError>> {
        let exif = ExifService::new(path);
        let manufacturer = self.manufacturer();
        let model = self.model();

        let mut errors = Vec::new();

        if let Err(e) = exif.set_make(manufacturer.as_str()) {
            errors.push(e);
        }

        if let Err(e) = exif.set_model(model.as_str()) {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
