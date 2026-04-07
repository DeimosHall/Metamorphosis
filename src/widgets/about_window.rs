use adw::prelude::*;
use gettextrs::gettext;
use glib::object::IsA;
use gtk::License;

//code 'inspired' by https://gitlab.com/news-flash/news_flash_gtk/-/blob/master/src/about_dialog.rs

//this is non-translatable information, so it can be const
pub const DEVELOPERS: &[&str] = &["Deimos Hall <deimoshall@proton.me>"];

#[derive(Clone, Debug)]
pub struct MetamorphosisAbout;

impl MetamorphosisAbout {
    pub fn show<W: IsA<gtk::Widget>>(window: &W) {
        let about = adw::AboutDialog::from_appdata(
            "/io/dev/deimoshall/Metamorphosis/io.dev.deimoshall.Metamorphosis.metainfo.xml",
            Some(crate::config::VERSION),
        );
        about.set_developers(DEVELOPERS);
        about.set_translator_credits(&gettext("translator-credits"));

        // about.add_other_app(
        //     "io.dev.deimoshall.Impression",
        //     // Translators: Metainfo for the app Impression. <https://gitlab.com/deimoshall/Impression>
        //     &gettext("Impression"),
        //     // Translators: Metainfo for the app Impression. <https://gitlab.com/deimoshall/Impression>
        //     &gettext("Create bootable drives"),
        // );
        // about.add_other_app(
        //     "io.dev.deimoshall.Footage",
        //     // Translators: Metainfo for the app Footage. <https://gitlab.com/deimoshall/Footage>
        //     &gettext("Footage"),
        //     // Translators: Metainfo for the app Footage. <https://gitlab.com/deimoshall/Footage>
        //     &gettext("Polish your videos"),
        // );
        about.add_acknowledgement_section(
            Some(&gettext("Code and Design Borrowed from")),
            &[
                "Switcheroo https://gitlab.com/adhami3310/Switcheroo",
            ],
        );
        about.add_legal_section("ImageMagick", None, License::MitX11, None);
        about.add_legal_section("ExifTool", None, License::Artistic, None);
        about.set_copyright("Copyright © 2026 Deimos Hall");
        about.set_comments("View and edit Exif metadata");
        about.present(Some(window));
    }
}
