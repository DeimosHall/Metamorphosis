use exiftool::ExifToolError;
use gtk::{glib, prelude::*, subclass::prelude::*};

use adw::subclass::bin::BinImpl;
use derivative::Derivative;
use gtk_macros::CompositeTemplate;

use crate::{
    models::metadata::Metadata, services::exiftool::ExifService,
    views::image_metadata::text_field_accesors,
};

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate, Derivative, Default)]
    #[template(
        resource = "/dev/deimoshall/Metamorphosis/ui/views/image_metadata/image_camera_lens.ui"
    )]
    pub struct ImageCameraLensView {
        #[template_child]
        pub container: TemplateChild<gtk::Box>,

        // Device
        #[template_child]
        pub make_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub model_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub serial_number_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub owner_name_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub camera_serial_number_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub camera_firmware_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub camera_label_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub image_unique_id_entry: TemplateChild<adw::EntryRow>,

        // Lens
        #[template_child]
        pub lens_make_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub lens_model_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub lens_serial_number_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub lens_info_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub max_aperture_value_entry: TemplateChild<adw::EntryRow>,

        // Exposure Settings
        #[template_child]
        pub exposure_time_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub fnumber_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub exposure_program_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub iso_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub focal_length_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub exposure_compensation_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub metering_mode_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub flash_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub white_balance_entry: TemplateChild<adw::EntryRow>,
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
    pub fn show(&self) {
        self.imp().container.set_visible(true);
    }

    pub fn hide(&self) {
        self.imp().container.set_visible(false);
    }

    // Getters and setters
    text_field_accesors! {
        // Device
        make => make_entry,
        model => model_entry,
        serial_number => serial_number_entry,
        owner_name => owner_name_entry,
        camera_serial_number => camera_serial_number_entry,
        camera_firmware => camera_firmware_entry,
        camera_label => camera_label_entry,
        image_unique_id => image_unique_id_entry,
        // Lens
        lens_make => lens_make_entry,
        lens_model => lens_model_entry,
        lens_serial_number => lens_serial_number_entry,
        lens_info => lens_info_entry,
        max_aperture_value => max_aperture_value_entry,
        // Exposure Settings
        exposure_time => exposure_time_entry,
        fnumber => fnumber_entry,
        exposure_program => exposure_program_entry,
        iso => iso_entry,
        focal_length => focal_length_entry,
        exposure_compensation => exposure_compensation_entry,
        metering_mode => metering_mode_entry,
        flash => flash_entry,
        white_balance => white_balance_entry,
    }

    /// Returns the given value in String format if it is
    /// Some, returns an empty String otherwise.
    /// Useful to "clean" the UI fields when the previous
    /// image contains a field, but the new one doesn't.
    fn as_text<T: ToString>(value: Option<T>) -> String {
        value.map(|v| v.to_string()).unwrap_or_default()
    }

    pub fn set_metadata(&self, metadata: &Metadata) {
        self.set_make(metadata.make().unwrap_or_default());
        self.set_model(metadata.model().unwrap_or_default());
        self.set_serial_number(metadata.serial_number().unwrap_or_default());
        self.set_owner_name(metadata.owner_name().unwrap_or_default());
        self.set_camera_serial_number(metadata.camera_serial_number().unwrap_or_default());
        self.set_camera_firmware(metadata.camera_firmware().unwrap_or_default());
        self.set_camera_label(metadata.camera_label().unwrap_or_default());
        self.set_image_unique_id(metadata.image_unique_id().unwrap_or_default());

        self.set_lens_make(metadata.lens_make().unwrap_or_default());
        self.set_lens_model(metadata.lens_model().unwrap_or_default());
        self.set_lens_serial_number(metadata.lens_serial_number().unwrap_or_default());
        self.set_lens_info(metadata.lens_info().unwrap_or_default());

        self.set_max_aperture_value(&Self::as_text(metadata.max_aperture_value()));
        self.set_exposure_time(&Self::as_text(metadata.exposure_time()));
        self.set_fnumber(&Self::as_text(metadata.fnumber()));
        self.set_exposure_program(&Self::as_text(metadata.exposure_program()));
        self.set_iso(&Self::as_text(metadata.iso()));
        self.set_focal_length(&Self::as_text(metadata.focal_length()));
        self.set_exposure_compensation(&Self::as_text(metadata.exposure_compensation()));
        self.set_metering_mode(&Self::as_text(metadata.metering_mode()));
        self.set_flash(&Self::as_text(metadata.flash()));
        self.set_white_balance(&Self::as_text(metadata.white_balance()));
    }

    pub fn save_changes(&self, path: &str) -> Result<(), ExifToolError> {
        let exif = ExifService::new(path);

        exif.set_make(self.make().as_str())?;
        exif.set_model(self.model().as_str())?;
        exif.set_serial_number(self.serial_number().as_str())?;
        exif.set_owner_name(self.owner_name().as_str())?;
        exif.set_camera_serial_number(self.camera_serial_number().as_str())?;
        exif.set_camera_firmware(self.camera_firmware().as_str())?;
        exif.set_camera_label(self.camera_label().as_str())?;
        exif.set_image_unique_id(self.image_unique_id().as_str())?;

        exif.set_lens_make(self.lens_make().as_str())?;
        exif.set_lens_model(self.lens_model().as_str())?;
        exif.set_lens_serial_number(self.lens_serial_number().as_str())?;
        exif.set_lens_info(self.lens_info().as_str())?;
        exif.set_max_aperture_value(self.max_aperture_value().as_str())?;

        exif.set_exposure_time(self.exposure_time().as_str())?;
        exif.set_fnumber(self.fnumber().as_str())?;
        exif.set_exposure_program(self.exposure_program().as_str())?;
        exif.set_iso(self.iso().as_str())?;
        exif.set_focal_length(self.focal_length().as_str())?;
        exif.set_exposure_compensation(self.exposure_compensation().as_str())?;
        exif.set_metering_mode(self.metering_mode().as_str())?;
        exif.set_flash(self.flash().as_str())?;
        exif.set_white_balance(self.white_balance().as_str())?;

        Ok(())
    }
}
