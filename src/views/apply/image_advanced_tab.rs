use gtk::{glib, prelude::*, subclass::prelude::*};

use derivative::Derivative;
use gtk_macros::CompositeTemplate;

mod imp {

    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derive(Default)]
    #[template(resource = "/dev/deimoshall/Metamorphosis/ui/views/apply/image_advanced_tab.ui")]
    pub struct ImageAdvancedTab {
        // Dates
        #[template_child]
        pub modify_date_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub date_time_original_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub create_date_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub gps_date_stamp_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub gps_time_stamp_entry: TemplateChild<gtk::Entry>,

        // Fractional seconds
        #[template_child]
        pub sub_sec_time_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub sub_sec_time_original_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub sub_sec_time_digitized_entry: TemplateChild<gtk::Entry>,

        // Timezone offsets
        #[template_child]
        pub offset_time_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub offset_time_original_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub offset_time_digitized_entry: TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImageAdvancedTab {
        const NAME: &'static str = "ImageAdvancedTab";
        type Type = super::ImageAdvancedTab;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageAdvancedTab {}
    impl WidgetImpl for ImageAdvancedTab {}
    impl BoxImpl for ImageAdvancedTab {}
}

glib::wrapper! {
    pub struct ImageAdvancedTab(ObjectSubclass<imp::ImageAdvancedTab>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImageAdvancedTab {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
