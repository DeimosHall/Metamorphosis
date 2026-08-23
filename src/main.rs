mod application;
#[rustfmt::skip]
mod config;
mod components;
mod dialogs;
mod input_file;
mod models;
mod services;
mod views;
mod window;

use std::sync::OnceLock;

use gettextrs::{LocaleCategory, gettext};
use glib::ExitCode;
use gtk::{gio, glib};
use tokio::runtime::Runtime;

use self::application::App;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Setting up tokio runtime needs to succeed.")
    })
}

fn main() -> ExitCode {
    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Metamorphosis"));

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    // Set debug level for logs when building with devel profile
    let mut builder = pretty_env_logger::formatted_builder();
    match std::env::var("RUST_LOG") {
        Ok(value) => {
            builder.parse_filters(&value);
        }
        Err(_) => {
            let filter = if config::PROFILE == "Devel" {
                "debug"
            } else {
                "error"
            };
            builder.parse_filters(filter);
        }
    }

    builder.init();

    let app = App::new();
    app.run()
}
