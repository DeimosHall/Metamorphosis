use exiftool::ExifToolError;
use glib::{object::ObjectExt, subclass::types::ObjectSubclassIsExt};
use gtk::{glib, prelude::ButtonExt};
use log::{error, warn};

use crate::{components::image_thumbnail::ImageThumbnail, models::input_file::InputFile};

mod image_camera_lens;
mod image_date_time;
mod image_details;
mod image_location;

mod imp {
    use adw::subclass::prelude::*;
    use derivative::Derivative;
    use gtk::CompositeTemplate;

    use crate::views::image_metadata::{
        self, image_camera_lens::ImageCameraLensView, image_date_time::ImageDateTimeView,
        image_details::ImageDetailsView, image_location::ImageLocationView,
    };

    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derivative(Default)]
    #[template(resource = "/dev/deimoshall/Metamorphosis/ui/views/image_metadata/mod.ui")]
    pub struct ImageMetadataView {
        #[template_child]
        pub image_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub image_thumbnail: TemplateChild<ImageThumbnail>,
        #[template_child]
        pub image_camera_lens_view: TemplateChild<ImageCameraLensView>,
        #[template_child]
        pub image_date_time_view: TemplateChild<ImageDateTimeView>,
        #[template_child]
        pub image_details_view: TemplateChild<ImageDetailsView>,
        #[template_child]
        pub image_location_view: TemplateChild<ImageLocationView>,
        #[template_child]
        pub save_button: TemplateChild<gtk::Button>,
    }

    #[::glib::object_subclass]
    impl ObjectSubclass for ImageMetadataView {
        const NAME: &'static str = "ImageMetadataView";
        type Type = image_metadata::ImageMetadataView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageMetadataView {}
    impl WidgetImpl for ImageMetadataView {}
    impl BinImpl for ImageMetadataView {}
}

glib::wrapper! {
    pub struct ImageMetadataView(ObjectSubclass<imp::ImageMetadataView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ImageMetadataView {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ImageMetadataView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn stack(&self) -> adw::ViewStack {
        self.imp().image_stack.clone()
    }

    /// Helper method to show or hide the advanced tab container.
    ///
    /// Tabs have different heights because of the different
    /// amount of fields. This method addresses this issue.
    ///
    /// This doesn't work without the inner container.
    pub fn setup_tab_switch_listener(&self) {
        let view = self.clone();
        // Hide advanced tab at startup.
        // Comment it to see the height issue at least once.
        view.imp().image_camera_lens_view.hide();

        self.stack()
            .connect_visible_child_name_notify(move |stack| {
                if let Some(tab) = stack.visible_child_name() {
                    view.hide_all_tabs();

                    match tab.as_str() {
                        "date_time" => view.imp().image_date_time_view.show(),
                        "location" => view.imp().image_location_view.show(),
                        "camera_lens" => view.imp().image_camera_lens_view.show(),
                        "details" => view.imp().image_details_view.show(),
                        _ => warn!("Unhandled tab: {}", tab.as_str()),
                    }
                } else {
                    error!("Error getting the tab name");
                }
            });
    }

    fn hide_all_tabs(&self) {
        self.imp().image_camera_lens_view.hide();
        self.imp().image_date_time_view.hide();
        self.imp().image_location_view.hide();
    }

    pub fn current_tab(&self) -> Option<glib::GString> {
        self.stack().visible_child_name()
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
        F: Fn(&ImageMetadataView) + 'static,
    {
        let view = self.clone();
        self.imp()
            .image_thumbnail
            .connect_remove_clicked(move |_| on_remove(&view));
    }

    pub fn set_on_save<F>(&self, on_save: F)
    where
        // TODO: refactor this implementation
        F: Fn(&ImageMetadataView) + 'static,
    {
        let view = self.clone();
        self.imp().save_button.connect_clicked(move |_| {
            // Implemented on window.rs
            // Calls save_changes
            on_save(&view);
        });
    }

    /// Populate UI fields using exif data from the given file
    pub fn load_from_file(&self, path: &str) {
        self.imp().image_date_time_view.load_from_file(path);
        self.imp().image_location_view.load_from_file(path);
        self.imp().image_camera_lens_view.load_from_file(path);
        self.imp().image_details_view.load_file(path);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn save_changes(&self, path: &str) -> Result<(), Vec<ExifToolError>> {
        if let Some(current_tab) = self.current_tab() {
            return match current_tab.as_str() {
                "date_time" => self.imp().image_date_time_view.save_changes(path),
                "location" => self.imp().image_location_view.save_changes(path),
                "camera_lens" => self.imp().image_camera_lens_view.save_changes(path),
                "details" => self.imp().image_details_view.save_changes(path),
                _ => {
                    warn!("Unhandled tab: {}", current_tab);
                    Ok(())
                }
            };
        }

        warn!("This should never be printed");
        Ok(())
    }
}
