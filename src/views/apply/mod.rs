use std::path::Path;

use exiftool::ExifToolError;
use glib::{object::ObjectExt, subclass::types::ObjectSubclassIsExt};
use gtk::{glib, prelude::ButtonExt};

use crate::{components::image_thumbnail::ImageThumbnail, input_file::InputFile};

mod image_advanced_tab;
mod image_basic_tab;

mod imp {
    use adw::subclass::prelude::*;
    use derivative::Derivative;
    use gtk::CompositeTemplate;

    use crate::views::apply::{self, image_advanced_tab::ImageAdvancedTab, image_basic_tab::ImageBasicTab};

    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derivative(Default)]
    #[template(resource = "/dev/deimoshall/Metamorphosis/ui/views/apply/mod.ui")]
    pub struct Apply {
        #[template_child]
        pub image_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub image_thumbnail: TemplateChild<ImageThumbnail>,
        #[template_child]
        pub image_basic_tab: TemplateChild<ImageBasicTab>,
        #[template_child]
        pub image_advanced_tab: TemplateChild<ImageAdvancedTab>,
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

    pub fn stack(&self) -> adw::ViewStack {
        self.imp().image_stack.clone()
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
        // TODO: refactor this implementation
        F: Fn(&Apply, String, String, String, String, String) + 'static,
    {
        let view = self.clone();
        self.imp().apply_button.connect_clicked(move |_| {
            let imp = view.imp();
            let date = imp.image_basic_tab.date();
            let offset = imp.image_basic_tab.offset();
            let manufacturer = imp.image_basic_tab.manufacturer();
            let model = imp.image_basic_tab.model();
            let description = imp.image_basic_tab.description();
            on_apply(&view, date, offset, manufacturer, model, description);
        });
    }

    /// Populate UI fields using exif data from the given file
    pub fn load_from_file(&self, path: String) {
        // TODO: improve arg to avoid cloning
        self.imp().image_basic_tab.load_from_file(path.clone());
        self.imp().image_advanced_tab.load_from_file(path);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn apply_changes(&self, path: String) -> Result<(), Vec<ExifToolError>> {
        self.imp().image_basic_tab.apply_changes(path)
    }
}
