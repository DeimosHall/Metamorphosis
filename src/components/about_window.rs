use adw::prelude::*;
use gettextrs::gettext;
use glib::object::IsA;
use gtk::License;

// Code 'inspired' by https://gitlab.com/news-flash/news_flash_gtk/-/blob/master/src/about_dialog.rs

// This is non-translatable information, so it can be const
pub const DEVELOPERS: &[&str] = &["Deimos Hall <deimoshall@proton.me>"];
pub const ARTISTS: &[&str] = &["Hylke Bons https://planetpeanut.studio"];

#[derive(Clone, Debug)]
pub struct MetamorphosisAbout;

impl MetamorphosisAbout {
    pub fn show<W: IsA<gtk::Widget>>(window: &W) {
        let about = adw::AboutDialog::from_appdata(
            "/dev/deimoshall/Metamorphosis/dev.deimoshall.Metamorphosis.metainfo.xml",
            Some(crate::config::VERSION),
        );
        about.set_developers(DEVELOPERS);
        about.set_artists(ARTISTS);
        about.set_translator_credits(&gettext("translator-credits"));

        about.add_other_app(
            "io.dev.deimoshall.DecryptIt",
            "Decrypt It",
            &gettext("Decrypt DLC files"),
        );

        about.add_acknowledgement_section(
            Some(&gettext("Code and Design Borrowed from")),
            &["Switcheroo https://gitlab.com/adhami3310/Switcheroo"],
        );

        about.add_legal_section("ExifTool", None, License::Artistic, None);
        about.set_copyright("Copyright © 2026 Deimos Hall");
        about.set_comments("Edit metadata");
        about.present(Some(window));
    }
}
