use gettextrs::gettext;
use glib::GString;
use gtk::{INVALID_LIST_POSITION, StringList, glib, prelude::*, subclass::prelude::*};

use adw::{
    prelude::{ActionRowExt, ComboRowExt},
    subclass::bin::BinImpl,
};
use derivative::Derivative;
use exiftool::ExifToolError;
use gtk_macros::CompositeTemplate;
use log::{debug, error};

use crate::{models::timezone::TIMEZONES, services::exiftool::ExifService};

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derivative(Default)]
    #[template(
        resource = "/dev/deimoshall/Metamorphosis/ui/views/image_metadata/image_date_time.ui"
    )]
    pub struct ImageDateTimeView {
        #[template_child]
        pub container: TemplateChild<gtk::Box>,

        // General fields
        #[template_child]
        pub general_fields_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub creation_date_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub creation_time_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub timezone_combo: TemplateChild<adw::ComboRow>,
        pub timezone_options: StringList,

        #[template_child]
        pub advanced_options_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub advanced_revelear: TemplateChild<gtk::Revealer>,

        // Modify Date fields
        #[template_child]
        pub modify_date_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub sub_sec_time_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub offset_time_entry: TemplateChild<adw::EntryRow>,

        // Date Time Original fields
        #[template_child]
        pub date_time_original_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub sub_sec_time_original_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub offset_time_original_entry: TemplateChild<adw::EntryRow>,

        // Create Date fields
        #[template_child]
        pub create_date_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub sub_sec_time_digitized_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub offset_time_digitized_entry: TemplateChild<adw::EntryRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImageDateTimeView {
        const NAME: &'static str = "ImageDateTimeView";
        type Type = super::ImageDateTimeView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageDateTimeView {}
    impl WidgetImpl for ImageDateTimeView {}
    impl BinImpl for ImageDateTimeView {}
}

glib::wrapper! {
    pub struct ImageDateTimeView(ObjectSubclass<imp::ImageDateTimeView>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImageDateTimeView {
    pub fn show(&self) {
        self.imp().container.set_visible(true);
    }

    pub fn hide(&self) {
        self.imp().container.set_visible(false);
    }

    fn timezone_combo(&self) -> &adw::ComboRow {
        &self.imp().timezone_combo
    }

    fn timezone_options(&self) -> &StringList {
        &self.imp().timezone_options
    }

    fn append_timezone_option(&self, option: &str) {
        self.timezone_options().append(option);
    }

    fn remove_invalid_timezone_option(&self) {
        let invalid_position = self.imp().timezone_options.find("Invalid");
        if invalid_position != INVALID_LIST_POSITION {
            debug!("Invalid position removed");
            self.timezone_options().remove(invalid_position);
        }
    }

    fn setup_advanced_switch_listener(&self) {
        let view = self.clone();
        self.imp()
            .advanced_options_switch
            .connect_active_notify(move |switch| {
                // Show/hide advanced options
                view.imp()
                    .advanced_revelear
                    .set_reveal_child(switch.is_active());
                // Enables/disables general fields
                view.imp()
                    .general_fields_group
                    .set_sensitive(!switch.is_active());
            });
    }

    /// Populates the StringList used in the timezone_combo
    /// with a "None" value at the beggining and the TIMEZONES
    /// after it
    fn setup_timezone_combo(&self) {
        self.append_timezone_option(&gettext("None"));

        for timezone in TIMEZONES.iter() {
            self.append_timezone_option(timezone);
        }

        self.timezone_combo()
            .set_model(Some(self.timezone_options()));
    }

    pub fn setup(&self) {
        self.setup_advanced_switch_listener();
        self.setup_timezone_combo();
    }

    pub fn creation_date(&self) -> GString {
        self.imp().creation_date_entry.text()
    }

    pub fn set_creation_date(&self, creation_date: &str) {
        self.imp().creation_date_entry.set_text(creation_date);
    }

    pub fn creation_time(&self) -> GString {
        self.imp().creation_time_entry.text()
    }

    pub fn set_creation_time(&self, creation_time: &str) {
        self.imp().creation_time_entry.set_text(creation_time);
    }

    /// Returns the timezone value from the list
    /// Should always be a valid value
    pub fn timezone(&self) -> GString {
        let index = self.timezone_combo().selected();

        if let Some(timezone) = self.timezone_options().string(index) {
            return timezone;
        }

        error!("Error getting timezone. Index {} out of bounds!", index);
        GString::new()
    }

    /// Sets the time zone value in the combo row
    /// Picks a value from the list based on the criteria:
    /// Empty -> "None"
    /// Valid -> One from the list
    /// Invalid -> "Invalid"
    pub fn set_timezone(&self, timezone: &str) {
        debug!("Setting time zone: {}", timezone);
        self.remove_invalid_timezone_option();

        if timezone.is_empty() {
            debug!("No time zone in metadata");
            let none_position = self.imp().timezone_options.find("None");
            self.timezone_combo().set_subtitle("");
            self.timezone_combo().set_selected(none_position);
            return;
        }

        // Weird API that makes me validate against INVALID_LIST_POSITION
        let position = self.imp().timezone_options.find(timezone);

        if position != INVALID_LIST_POSITION {
            self.timezone_combo().set_subtitle("");
            self.timezone_combo().set_selected(position);
        } else {
            debug!("The '{}' time zone is not a valid value", timezone);
            self.imp().timezone_options.append(&gettext("Invalid"));
            let invalid_position = self.imp().timezone_options.find("Invalid");
            self.timezone_combo()
                .set_subtitle(&gettext("Invalid time zone found"));
            self.timezone_combo().set_selected(invalid_position);
        }
    }

    pub fn modify_date(&self) -> GString {
        self.imp().modify_date_entry.text()
    }

    pub fn set_modify_date(&self, modify_date: &str) {
        self.imp().modify_date_entry.set_text(modify_date);
    }

    pub fn date_time_original(&self) -> GString {
        self.imp().date_time_original_entry.text()
    }

    pub fn set_date_time_original(&self, date_time: &str) {
        self.imp().date_time_original_entry.set_text(date_time);
    }

    pub fn create_date(&self) -> GString {
        self.imp().create_date_entry.text()
    }

    pub fn set_create_date(&self, create_date: &str) {
        self.imp().create_date_entry.set_text(create_date);
    }

    pub fn sub_sec_time(&self) -> GString {
        self.imp().sub_sec_time_entry.text()
    }

    pub fn set_sub_sec_time(&self, sub_sec_time: &str) {
        self.imp().sub_sec_time_entry.set_text(sub_sec_time);
    }

    pub fn sub_sec_time_original(&self) -> GString {
        self.imp().sub_sec_time_original_entry.text()
    }

    pub fn set_sub_sec_time_original(&self, sub_sec_time_original: &str) {
        self.imp()
            .sub_sec_time_original_entry
            .set_text(sub_sec_time_original);
    }

    pub fn sub_sec_time_digitized(&self) -> GString {
        self.imp().sub_sec_time_digitized_entry.text()
    }

    pub fn set_sub_sec_time_digitized(&self, sub_sec_time_digitized: &str) {
        self.imp()
            .sub_sec_time_digitized_entry
            .set_text(sub_sec_time_digitized);
    }

    pub fn offset_time(&self) -> GString {
        self.imp().offset_time_entry.text()
    }

    pub fn set_offset_time(&self, offset_time: &str) {
        self.imp().offset_time_entry.set_text(offset_time);
    }

    pub fn offset_time_original(&self) -> GString {
        self.imp().offset_time_original_entry.text()
    }

    pub fn set_offset_time_original(&self, offset_time_original: &str) {
        self.imp()
            .offset_time_original_entry
            .set_text(offset_time_original);
    }

    pub fn offset_time_digitized(&self) -> GString {
        self.imp().offset_time_digitized_entry.text()
    }

    pub fn set_offset_time_digitized(&self, offset_time_digitized: &str) {
        self.imp()
            .offset_time_digitized_entry
            .set_text(offset_time_digitized);
    }

    fn split_date_time(date_time: &str) -> Option<(&str, &str)> {
        date_time.split_once(' ')
    }

    fn join_date_time(date: &str, time: &str) -> String {
        format!("{} {}", date, time)
    }

    // TODO: maybe these methods should go in a trait
    /// Populate UI fields using exif data from the given file
    pub fn load_from_file(&self, path: &str) {
        let exif = ExifService::new(path);

        let modify_date = exif.modify_date().unwrap_or_default();
        let date_time_original = exif.date_time_original().unwrap_or_default();
        let create_date = exif.create_date().unwrap_or_default();
        let sub_sec_time = exif.sub_sec_time().unwrap_or_default();
        let sub_sec_time_original = exif.sub_sec_time_original().unwrap_or_default();
        let sub_sec_time_digitized = exif.sub_sec_time_digitized().unwrap_or_default();
        let offset_time = exif.offset_time().unwrap_or_default();
        let offset_time_original = exif.offset_time_original().unwrap_or_default();
        let offset_time_digitized = exif.offset_time_digitized().unwrap_or_default();

        let (creation_date, creation_time) =
            ImageDateTimeView::split_date_time(&create_date).unwrap_or_default();

        self.set_creation_date(creation_date);
        self.set_creation_time(creation_time);
        debug!("Loading offset from file: {}", offset_time_digitized);
        self.set_timezone(&offset_time_digitized);

        self.set_modify_date(&modify_date);
        self.set_date_time_original(&date_time_original);
        self.set_create_date(&create_date);
        self.set_sub_sec_time(&sub_sec_time);
        self.set_sub_sec_time_original(&sub_sec_time_original);
        self.set_sub_sec_time_digitized(&sub_sec_time_digitized);
        self.set_offset_time(&offset_time);
        self.set_offset_time_original(&offset_time_original);
        self.set_offset_time_digitized(&offset_time_digitized);
    }

    /// Take the values from the UI fields and apply them to a file
    pub fn save_changes(&self, path: &str) -> Result<(), ExifToolError> {
        let exif = ExifService::new(path);

        if !self.imp().advanced_options_switch.is_active() {
            let date = self.creation_date();
            let time = self.creation_time();
            let date_time = ImageDateTimeView::join_date_time(date.as_str(), time.as_str());
            let offset = self.timezone();

            exif.set_all_dates(date_time.as_str())?;
            exif.set_all_offset_times(offset.as_str())?;
        } else {
            exif.set_modify_date(self.modify_date().as_str())?;
            exif.set_date_time_original(self.date_time_original().as_str())?;
            exif.set_create_date(self.create_date().as_str())?;
            exif.set_sub_sec_time(self.sub_sec_time().as_str())?;
            exif.set_sub_sec_time_original(self.sub_sec_time_original().as_str())?;
            exif.set_sub_sec_time_digitized(self.sub_sec_time_digitized().as_str())?;
            exif.set_offset_time(self.offset_time().as_str())?;
            exif.set_offset_time_original(self.offset_time_original().as_str())?;
            exif.set_offset_time_digitized(self.offset_time_digitized().as_str())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_date_time() {
        let date_time = "2026:09:10 16:41:04";
        let (date, time) = ImageDateTimeView::split_date_time(date_time).unwrap();
        assert_eq!(date, "2026:09:10");
        assert_eq!(time, "16:41:04");
    }

    #[test]
    fn joins_date_time() {
        let date = "2026:09:10";
        let time = "16:41:04";
        let date_time = ImageDateTimeView::join_date_time(date, time);
        assert_eq!(date_time, "2026:09:10 16:41:04");
    }
}
