use exiftool::ExifToolError;
use glib::{object::ObjectExt, subclass::types::ObjectSubclassIsExt};
use gtk::{glib, prelude::ButtonExt};

use crate::{components::image_thumbnail::ImageThumbnail, input_file::InputFile};

mod image_advanced_tab;
mod image_general_tab;

mod imp {
    use adw::subclass::prelude::*;
    use derivative::Derivative;
    use gtk::CompositeTemplate;

    use crate::views::apply::{
        self, image_advanced_tab::ImageAdvancedTab, image_general_tab::ImageGeneralTab,
    };

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
        pub image_general_tab: TemplateChild<ImageGeneralTab>,
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

    /// Helper method to show or hide the advanced tab container.
    ///
    /// Both tabs have different heights because of the different
    /// amount of fields. This makes the general tab have a height
    /// equivalent to the advanced one. This method addresses this
    /// issue.
    ///
    /// This doesn't work without the inner container.
    pub fn setup_tab_switch_listener(&self) {
        let view = self.clone();
        // Hide advanced tab at startup.
        // Comment it to see the height issue at least once.
        view.imp().image_advanced_tab.hide();

        self.stack()
            .connect_visible_child_name_notify(move |stack| {
                if let Some(tab) = stack.visible_child_name() {
                    match tab.as_str() {
                        "general" => view.imp().image_advanced_tab.hide(),
                        "advanced" => view.imp().image_advanced_tab.show(),
                        _ => {}
                    }
                }
            });
    }

    pub fn current_tab(&self) -> Option<glib::GString> {
        Some(self.stack().visible_child_name()?)
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
    }

    /// Sets a callback for the remove action.
    ///
    /// The user can perform it on the trash icon
    /// placed on the top right of the image.
    pub fn set_on_remove<F>(&self, on_remove: F)
    where
        F: Fn(&Apply) + 'static,
    {
        let view = self.clone();
        self.imp()
            .image_thumbnail
            .connect_remove_clicked(move |_| on_remove(&view));
    }

    pub fn set_on_apply<F>(&self, on_apply: F)
    where
        // TODO: refactor this implementation
        F: Fn(&Apply) + 'static,
    {
        let view = self.clone();
        self.imp().apply_button.connect_clicked(move |_| {
            // Implemented on window.rs
            // Calls apply_changes
            on_apply(&view);
        });
    }

    /// Populate UI fields using exif data from the given file
    pub fn load_from_file(&self, path: String) {
        // TODO: improve arg to avoid cloning
        self.imp().image_general_tab.load_from_file(path.clone());
        self.imp().image_advanced_tab.load_from_file(path);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn apply_changes(&self, path: String) -> Result<(), Vec<ExifToolError>> {
        if let Some(current_tab) = self.current_tab() {
            return match current_tab.as_str() {
                "general" => self.imp().image_general_tab.apply_changes(&path),
                "advanced" => self.imp().image_advanced_tab.apply_changes(&path),
                _ => Ok(()), // TODO: return an error here
            };
        }

        println!("This should never be printed");
        Ok(())
    }
}
