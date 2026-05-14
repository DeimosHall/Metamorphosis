use std::path::Path;

use exiftool::ExifToolError;
use glib::{object::ObjectExt, subclass::types::ObjectSubclassIsExt};
use gtk::{
    glib,
    prelude::{ButtonExt, EditableExt},
};

use crate::{
    components::image_thumbnail::ImageThumbnail, input_file::InputFile,
    services::exif::ExifService,
};

mod imp {
    use adw::subclass::prelude::*;
    use derivative::Derivative;
    use gtk::CompositeTemplate;

    use crate::views::apply;

    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derivative(Default)]
    #[template(resource = "/dev/deimoshall/Metamorphosis/ui/views/apply/mod.ui")]
    pub struct Apply {
        #[template_child]
        pub image_thumbnail: TemplateChild<ImageThumbnail>,
        #[template_child]
        pub create_date_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub offset_time_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub manufacturer_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub model_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub image_description_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub apply_button: TemplateChild<gtk::Button>,
    }

    #[::glib::object_subclass]
    impl ObjectSubclass for Apply {
        const NAME: &'static str = "ApplyView";
        type Type = apply::Apply;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Apply {}
    impl WidgetImpl for Apply {}
    impl BinImpl for Apply {}
}

glib::wrapper! {
    pub struct Apply(ObjectSubclass<imp::Apply>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for Apply {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl Apply {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn update_thumbnail(&self, file: InputFile) {
        let imp = self.imp();
        let file_type = file.kind();
        let dimensions = file.dimensions();

        let caption = match dimensions {
            Some((w, h)) => {
                format!("{} · {}×{}", file_type.as_display_string(), w, h,)
            }
            None => file_type.as_display_string().to_owned(),
        };

        let (w, h) = dimensions.unwrap_or_default();

        imp.image_thumbnail
            .set_property("image", file.pixbuf().as_ref());
        imp.image_thumbnail.set_property("content", caption);
        imp.image_thumbnail.set_property("width", w as u32);
        imp.image_thumbnail.set_property("height", h as u32);

        // imp.image_thumbnail.connect_remove_clicked(clone!(
        //     #[weak(rename_to=this)]
        //     self,
        //     move |_| {
        //         // Switch to stack welcome here
        //     }
        // ));
    }

    pub fn set_on_apply<F>(&self, on_apply: F)
    where
        F: Fn(&Apply, String, String, String, String, String) + 'static,
    {
        let view = self.clone();
        self.imp().apply_button.connect_clicked(move |_| {
            let imp = view.imp();
            let date = imp.create_date_entry.text().to_string();
            let offset = imp.offset_time_entry.text().to_string();
            let manufacturer = imp.manufacturer_entry.text().to_string();
            let model = imp.manufacturer_entry.text().to_string();
            let description = imp.image_description_entry.text().to_string();
            on_apply(&view, date, offset, manufacturer, model, description);
        });
    }

    pub fn date(&self) -> String {
        self.imp().create_date_entry.text().to_string()
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
        self.imp().create_date_entry.set_text(date);
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

    /// Populate UI fields using exif data from the given file
    pub fn load_from_file(&self, path: &Path) {
        let date = ExifService::create_date(path).unwrap_or_default();
        let offset = ExifService::offset_time(path).unwrap_or_default();
        let manufacturer = ExifService::make(path).unwrap_or_default();
        let model = ExifService::model(path).unwrap_or_default();
        let description = ExifService::image_description(path).unwrap_or_default();

        self.set_date(&date);
        self.set_offset(&offset);
        self.set_manufacturer(&manufacturer);
        self.set_model(&model);
        self.set_description(&description);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn apply_changes(&self, path: String) -> Result<(), Vec<ExifToolError>> {
        let path = Path::new(path.as_str());
        let date = self.date();
        let offset = self.offset();
        let manufacturer = self.manufacturer();
        let model = self.model();
        let description = self.description();

        let mut errors = Vec::new();

        if let Err(e) = ExifService::set_all_dates(path, date.as_str()) {
            errors.push(e);
        }

        if let Err(e) = ExifService::set_all_offset_times(path, offset.as_str()) {
            errors.push(e);
        }

        if let Err(e) = ExifService::set_make(path, manufacturer.as_str()) {
            errors.push(e);
        }

        if let Err(e) = ExifService::set_model(path, model.as_str()) {
            errors.push(e);
        }

        if let Err(e) = ExifService::set_image_description(path, description.as_str()) {
            errors.push(e);
        }

        if let Err(e) = ExifService::set_software(path) {
            errors.push(e);
        }

        if errors.is_empty() {
            return Ok(());
        } else {
            Err(errors)
        }
    }
}
