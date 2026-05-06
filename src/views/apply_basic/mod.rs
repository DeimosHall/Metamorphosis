use std::path::Path;

use exiftool::ExifToolError;
use glib::{clone, subclass::types::ObjectSubclassIsExt};
use gtk::{
    accessible::Property,
    glib,
    prelude::{AccessibleExtManual, ButtonExt, EditableExt, FlowBoxChildExt, WidgetExt},
};

use crate::{
    components::image_thumbnail::ImageThumbnail, input_file::InputFile,
    services::exif_service::ExifService,
};

mod imp {
    use adw::subclass::prelude::*;
    use derivative::Derivative;
    use gtk::CompositeTemplate;

    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derivative(Default)]
    #[template(resource = "/dev/deimoshall/Metamorphosis/blueprints/apply_basic.ui")]
    pub struct ApplyBasic {
        #[template_child]
        pub image_container: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub create_date_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub offset_time_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub image_description_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub apply_button: TemplateChild<gtk::Button>,
    }

    #[::glib::object_subclass]
    impl ObjectSubclass for ApplyBasic {
        const NAME: &'static str = "ApplyBasicView";
        type Type = super::ApplyBasic;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ApplyBasic {}
    impl WidgetImpl for ApplyBasic {}
    impl BinImpl for ApplyBasic {}
}

glib::wrapper! {
    pub struct ApplyBasic(ObjectSubclass<imp::ApplyBasic>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ApplyBasic {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ApplyBasic {
    pub fn new() -> Self {
        glib::Object::new()
    }

    // Should it go here?
    pub fn update_image_container(&self, file: InputFile) {
        let imp = self.imp();
        let file_type = file.kind();
        let dims = file.dimensions();

        while let Some(child) = imp.image_container.first_child() {
            imp.image_container.remove(&child);
        }

        let f = file;
        let caption = match dims {
            Some((w, h)) => {
                format!("{} · {}×{}", file_type.as_display_string(), w, h,)
            }
            None => file_type.as_display_string().to_owned(),
        };

        let (w, h) = dims.unwrap_or_default();

        let image_thumbnail =
            ImageThumbnail::new(f.pixbuf().as_ref(), &caption, w as u32, h as u32);

        let image_flow_box_child = gtk::FlowBoxChild::new();
        image_flow_box_child.set_child(Some(&image_thumbnail));

        image_flow_box_child.update_property(&[Property::Label(&caption)]);

        imp.image_container.append(&image_flow_box_child);
        image_thumbnail.connect_remove_clicked(clone!(
            #[weak(rename_to=this)]
            self,
            move |_| {
                this.imp().image_container.invalidate_filter();
            }
        ));
    }

    pub fn set_on_apply<F>(&self, on_apply: F)
    where
        F: Fn(&ApplyBasic, String, String, String) + 'static,
    {
        let view = self.clone();
        self.imp().apply_button.connect_clicked(move |_| {
            let imp = view.imp();
            let date = imp.create_date_entry.text().to_string();
            let offset = imp.offset_time_entry.text().to_string();
            let description = imp.image_description_entry.text().to_string();
            on_apply(&view, date, offset, description);
        });
    }

    pub fn date(&self) -> String {
        self.imp().create_date_entry.text().to_string()
    }

    pub fn offset(&self) -> String {
        self.imp().offset_time_entry.text().to_string()
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

    pub fn set_description(&self, description: &str) {
        self.imp().image_description_entry.set_text(description);
    }

    /// Populate UI fields using exif data from the given file
    pub fn load_from_file(&self, path: &Path) {
        let date = ExifService::create_date(path).unwrap_or_default();
        let offset = ExifService::offset_time(path).unwrap_or_default();
        let description = ExifService::image_description(path).unwrap_or_default();

        self.set_date(&date);
        self.set_offset(&offset);
        self.set_description(&description);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn apply_changes(&self, path: String) -> Result<(), Vec<ExifToolError>> {
        let path = Path::new(path.as_str());
        let date = self.date();
        let offset = self.offset();
        let description = self.description();

        let mut errors = Vec::new();

        if let Err(e) = ExifService::set_all_dates(path, date.as_str()) {
            errors.push(e);
        }

        if let Err(e) = ExifService::set_all_offset_times(path, offset.as_str()) {
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
