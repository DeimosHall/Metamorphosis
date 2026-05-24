use std::collections::HashSet;
use std::sync::atomic::AtomicUsize;

use crate::components::about_window::MetamorphosisAbout;
use crate::components::drag_overlay::DragOverlay;
use crate::config::APP_ID;
use crate::file_chooser::FileChooser;
use crate::input_file::InputFile;
use crate::magick::{JobFile, count_frames};
use crate::runtime;
use crate::services::exif::ExifService;
use adw::prelude::*;
use futures::future::join_all;
use gettextrs::gettext;
use glib::{MainContext, clone, idle_add_local_once};
use gtk::gdk::Texture;
use gtk::{gdk, gio, glib, subclass::prelude::*};
use itertools::Itertools;
use shared_child::SharedChild;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeFilter {
    Default,
    Point,
}

#[allow(dead_code)]
impl ResizeFilter {
    pub fn as_display_string(&self) -> Option<&str> {
        match self {
            ResizeFilter::Default => None,
            ResizeFilter::Point => Some("Point"),
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(ResizeFilter::Default),
            1 => Some(ResizeFilter::Point),
            _ => None,
        }
    }
}

mod imp {
    use std::{
        cell::{Cell, RefCell},
        sync::atomic::AtomicBool,
    };

    use crate::{config::PKGDATADIR, views::apply::Apply};

    use super::*;

    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use derivative::Derivative;
    use gtk::CompositeTemplate;

    #[derive(Debug, CompositeTemplate, Derivative)]
    #[derivative(Default)]
    #[template(resource = "/dev/deimoshall/Metamorphosis/ui/window.ui")]
    pub struct AppWindow {
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub drag_overlay: TemplateChild<DragOverlay>,
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub open_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub add_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub cancel_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub loading_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub progress_bar: TemplateChild<gtk::ProgressBar>,
        #[template_child]
        pub apply_view: TemplateChild<Apply>,
        #[template_child]
        pub view_switcher: TemplateChild<adw::ViewSwitcher>,

        #[template_child]
        pub navigation: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub help_overlay: TemplateChild<adw::ShortcutsDialog>,

        #[derivative(Default(value = "gio::ListStore::new::<InputFile>()"))]
        pub input_file_store: gio::ListStore,
        #[derivative(Default(value = "gio::Settings::new(APP_ID)"))]
        pub settings: gio::Settings,
        #[derivative(Default(value = "std::sync::Arc::new(AtomicBool::new(true))"))]
        pub is_canceled: std::sync::Arc<AtomicBool>,
        pub current_jobs: RefCell<Vec<Arc<SharedChild>>>,
        pub image_width: Cell<Option<u32>>,
        pub image_height: Cell<Option<u32>>,
        pub removed: RefCell<HashSet<u32>>,
        pub elements: Cell<usize>,
    }

    #[::glib::object_subclass]
    impl ObjectSubclass for AppWindow {
        const NAME: &'static str = "AppWindow";
        type Type = super::AppWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self::default()
        }
    }

    impl ObjectImpl for AppWindow {
        fn constructed(&self) {
            self.parent_constructed();

            // Load CSS
            let provider = gtk::CssProvider::new();
            provider.load_from_resource("/dev/deimoshall/Metamorphosis/style.css");

            if let Some(display) = gtk::gdk::Display::default() {
                gtk::style_context_add_provider_for_display(
                    &display,
                    &provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }

            let theme = gtk::IconTheme::for_display(
                &gtk::gdk::Display::default().expect("cannot find display"),
            );
            theme.add_search_path(PKGDATADIR.to_owned() + "/icons");

            let obj = self.obj();
            obj.load_window_size();
            obj.setup_gactions();
        }
    }

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {
        fn close_request(&self) -> glib::Propagation {
            if let Err(err) = self.obj().save_window_size() {
                dbg!("Failed to save window state, {}", &err);
            }

            if !self.is_canceled.load(std::sync::atomic::Ordering::SeqCst) {
                self.obj().close_dialog();
                glib::Propagation::Stop
            } else {
                // Pass close request on to the parent
                self.parent_close_request()
            }
        }
    }

    impl ApplicationWindowImpl for AppWindow {}
    impl AdwApplicationWindowImpl for AppWindow {}
}

glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::Widget, gtk::Window,  gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup,
                    gtk::Root, gtk::Native, gtk::ShortcutManager,
                    gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl AppWindow {
    pub fn new<P: glib::prelude::IsA<gtk::Application>>(app: &P) -> Self {
        let win = glib::Object::builder::<AppWindow>()
            .property("application", app)
            .build();

        win.setup_callbacks();
        win.setup_drop_target();

        win
    }

    /// Shows a basic toast with the given text.
    fn show_toast(&self, text: &str) {
        self.imp().toast_overlay.add_toast(adw::Toast::new(text));
    }

    fn setup_gactions(&self) {
        self.add_action_entries([
            gio::ActionEntry::builder("close")
                .activate(clone!(
                    #[weak(rename_to=window)]
                    self,
                    move |_, _, _| {
                        window.destroy();
                    }
                ))
                .build(),
            gio::ActionEntry::builder("add")
                .activate(clone!(
                    #[weak(rename_to=window)]
                    self,
                    move |_, _, _| {
                        window.add_dialog();
                    }
                ))
                .build(),
            gio::ActionEntry::builder("clear")
                .activate(clone!(
                    #[weak(rename_to=window)]
                    self,
                    move |_, _, _| {
                        window.clear();
                    }
                ))
                .build(),
            gio::ActionEntry::builder("exif")
                .activate(clone!(
                    #[weak(rename_to=window)]
                    self,
                    move |_, _, _| {
                        window.test_exiftool();
                    }
                ))
                .build(),
            gio::ActionEntry::builder("show-help-overlay")
                .activate(clone!(
                    #[weak(rename_to=window)]
                    self,
                    move |_, _, _| {
                        window.show_help_overlay();
                    }
                ))
                .build(),
            gio::ActionEntry::builder("about")
                .activate(clone!(
                    #[weak(rename_to=window)]
                    self,
                    move |_, _, _| {
                        window.show_about();
                    }
                ))
                .build(),
            gio::ActionEntry::builder("paste")
                .activate(clone!(
                    #[weak(rename_to=window)]
                    self,
                    move |_, _, _| {
                        window.load_clipboard();
                    }
                ))
                .build(),
        ]);
    }

    fn setup_callbacks(&self) {
        //load imp
        let imp = self.imp();

        imp.view_switcher.set_stack(Some(&imp.apply_view.stack()));
        
        imp.apply_view.setup_tab_switch_listener();

        imp.open_button.connect_clicked(clone!(
            #[weak(rename_to=this)]
            self,
            move |_| {
                this.add_dialog();
            }
        ));
        
        imp.add_button.connect_clicked(clone!(
            #[weak(rename_to=this)]
            self,
            move |_| {
                this.add_dialog();
            }
        ));

        // imp.image_container.set_filter_func(clone!(
        //     #[weak(rename_to=this)]
        //     self,
        //     #[upgrade_or_default]
        //     move |f| {
        //         return (f.index() as usize) >= this.imp().elements.get()
        //             || !this.imp().removed.borrow().contains(&(f.index() as u32));
        //     }
        // ));

        imp.cancel_button.connect_clicked(clone!(
            #[weak(rename_to=this)]
            self,
            move |_| {
                this.apply_cancel();
            }
        ));

        let apply_view = imp.apply_view.clone();

        apply_view.clone().set_on_remove(clone!(
            #[weak(rename_to=win)]
            self,
            move |_| {
                win.switch_to_stack_welcome();
            }
        ));
        
        // TODO: check why going though here takes much time
        apply_view.clone().set_on_apply(clone!(
            #[weak(rename_to=win)]
            self,
            move |_| {
                let path = win.files().first().unwrap().path();

                // Disable loading screen by now
                // TODO: implement it after batch processing
                if false {
                    win.switch_to_stack_applying();
                    win.set_convert_progress(0, 1);
                }

                glib::spawn_future_local(clone!(
                    #[weak(rename_to=win)]
                    win,
                    #[strong]
                    apply_view,
                    #[strong]
                    path,
                    async move {
                        let result = apply_view.apply_changes(path);

                        match result {
                            Ok(()) => {
                                if false {
                                    win.set_convert_progress(1, 1);
                                    win.switch_to_stack_apply();
                                }
                                win.show_toast(&gettext("Changes applied"));
                            }
                            Err(errors) => {
                                // TODO: use the right dialog
                                for error in errors {
                                    win.show_toast(&format!("{}", error));
                                }
                            }
                        }
                    }
                ));
            }
        ));
    }

    fn setup_drop_target(&self) {
        let drop_target = gtk::DropTarget::builder()
            .name("file-drop-target")
            .actions(gdk::DragAction::COPY)
            .formats(&gdk::ContentFormats::for_type(gdk::FileList::static_type()))
            .build();

        drop_target.connect_drop(clone!(
            #[weak(rename_to=win)]
            self,
            #[upgrade_or_default]
            move |_, value, _, _| {
                if let Ok(file_list) = value.get::<gdk::FileList>() {
                    if file_list.files().is_empty() {
                        win.show_toast(&gettext("Unable to access dropped files"));
                        return false;
                    }

                    let input_files = file_list.files().iter().map(InputFile::new).collect_vec();
                    win.open_files(input_files);
                    return true;
                }

                false
            }
        ));

        self.imp().drag_overlay.set_drop_target(&drop_target);
    }

    fn show_about(&self) {
        MetamorphosisAbout::show(self);
    }

    fn show_help_overlay(&self) {
        self.imp().help_overlay.present(Some(self));
    }

    fn close_dialog(&self) {
        let stop_converting_dialog = adw::AlertDialog::new(
            Some(&gettext("Stop converting?")),
            Some(&gettext("You will lose all progress.")),
        );

        stop_converting_dialog.add_response("cancel", &gettext("_Cancel"));
        stop_converting_dialog.add_response("stop", &gettext("_Stop"));
        stop_converting_dialog
            .set_response_appearance("stop", adw::ResponseAppearance::Destructive);
        stop_converting_dialog.connect_response(
            None,
            clone!(
                #[weak(rename_to=this)]
                self,
                move |_, response_id| {
                    if response_id == "stop" {
                        this.imp()
                            .is_canceled
                            .store(true, std::sync::atomic::Ordering::SeqCst);
                        let mut current_jobs = this.imp().current_jobs.borrow_mut();
                        for x in current_jobs.iter() {
                            match x.kill() {
                                Ok(_) => {}
                                Err(_) => {
                                    x.wait().ok();
                                }
                            }
                        }
                        current_jobs.clear();
                        this.close();
                    }
                }
            ),
        );
        stop_converting_dialog.present(Some(self));
    }

    fn set_convert_progress(&self, done: usize, total: usize) {
        let msg = format!("{done}/{total}");
        self.imp().progress_bar.set_text(Some(&msg));
        self.imp()
            .progress_bar
            .set_fraction((done as f64) / (total as f64));
    }

    pub fn load_clipboard(&self) {
        let clipboard = self.clipboard();
        if clipboard.formats().contain_mime_type("image/png") {
            MainContext::default().spawn_local(clone!(
                #[weak(rename_to=this)]
                self,
                async move {
                    let t = clipboard.read_texture_future().await;
                    if let Ok(Some(t)) = t {
                        let interim = JobFile::from_clipboard();
                        t.save_to_png(interim.as_filename()).ok();
                        let file =
                            InputFile::new(&gio::File::for_path(interim.as_filename())).unwrap();
                        this.open_success(vec![file]);
                    }
                }
            ));
        } else if clipboard
            .formats()
            .contain_mime_type("application/vnd.portal.files")
        {
            MainContext::default().spawn_local(clone!(
                #[weak(rename_to=this)]
                self,
                async move {
                    let t = clipboard.read_text_future().await.unwrap().unwrap();
                    let files = t
                        .lines()
                        .flat_map(|p| InputFile::new(&gio::File::for_path(p)))
                        .collect();
                    this.open_success(files);
                }
            ));
        }
    }

    fn open_success(&self, mut files: Vec<InputFile>) {
        let prev_files = self.active_files();
        let prev_files_paths = prev_files.iter().map(|f| f.path()).collect_vec();
        files = files
            .into_iter()
            .filter(|f| !prev_files_paths.contains(&f.path()))
            .chain(prev_files)
            .filter(|f| f.exists())
            .collect();

        // TODO: allow more than one file
        if files.len() > 1 {
            files.truncate(1);
        }

        self.imp().input_file_store.remove_all();
        self.imp().removed.replace(HashSet::new());

        self.switch_to_stack_loading_generally();

        for file in files.iter() {
            self.imp().input_file_store.append(file);
        }

        let _ = fdlimit::raise_fd_limit();

        self.load_frames();
    }

    fn load_frames(&self) {
        let files = self.files();
        let file_paths = files.iter().map(|f| f.path()).collect_vec();

        let (sender, receiver) = async_channel::bounded(1);

        std::thread::spawn(move || {
            let jobs = file_paths
                .into_iter()
                .map(|f| async move { count_frames(f).await.unwrap_or((1, None)) })
                .collect_vec();

            let res = runtime().block_on(join_all(jobs));

            sender.send_blocking(res).expect("Concurrency Issues");
        });

        glib::spawn_future_local(clone!(
            #[weak(rename_to=this)]
            self,
            async move {
                if let Ok(image_info) = receiver.recv().await {
                    let real_files = files.clone();
                    for (f, (frame, dims)) in real_files.iter().zip(image_info.iter()) {
                        f.set_frames(*frame);
                        let dims = *dims;
                        idle_add_local_once(clone!(
                            #[weak(rename_to=ff)]
                            f,
                            move || {
                                if let Some((width, height)) = dims {
                                    ff.set_width(width);
                                    ff.set_height(height);
                                }
                            }
                        ));
                        glib::MainContext::default().iteration(true);
                    }
                    idle_add_local_once(clone!(
                        #[weak(rename_to=these)]
                        this,
                        move || {
                            these.load_pixbuf();
                        }
                    ));
                }
            }
        ));
    }

    fn test_exiftool(&self) {
        println!("Testing exiftool");
        let paths = self.files().iter().map(|f| f.path()).collect_vec();

        glib::spawn_future_local(clone!(
            #[weak(rename_to=_this)]
            self,
            async move {
                for path in paths {
                    let exif = ExifService::new(&path);
                    exif.read_all();
                }
            }
        ));
    }

    pub fn clear(&self) {
        self.imp().input_file_store.remove_all();

        self.switch_to_stack_welcome();
    }

    fn active_files(&self) -> Vec<InputFile> {
        let removed = self.imp().removed.borrow().clone();
        self.files()
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !removed.contains(&(*i as u32)))
            .map(|(_, f)| f)
            .collect_vec()
    }

    fn files(&self) -> Vec<InputFile> {
        self.imp()
            .input_file_store
            .iter::<InputFile>()
            .flatten()
            .collect_vec()
    }

    fn load_pixbuf_finished(&self) {
        let imp = self.imp();

        let files_dims = self
            .active_files()
            .into_iter()
            .map(|f| f.dimensions())
            .unique()
            .collect_vec();

        if let Some((w, h)) = match files_dims[..] {
            [Some(d)] => Some(d),
            _ => None,
        } {
            imp.image_width.set(Some(w as u32));
            imp.image_height.set(Some(h as u32));
        } else {
            imp.image_width.set(None);
            imp.image_height.set(None);
        }

        let file = self.files().first().unwrap().clone();
        self.imp().apply_view.update_thumbnail(file);

        self.switch_back_from_loading();
        let path = self.files().first().unwrap().path();
        self.imp()
            .apply_view
            .load_from_file(path);

        if matches!(self.imp().navigation.visible_page().and_then(|x| x.tag()), Some(x) if x == "main")
        {
            self.switch_to_stack_apply();
        }
    }

    fn files_count(&self) -> usize {
        (self.imp().input_file_store.n_items() as usize) - self.imp().removed.borrow().len()
    }

    fn load_pixbuf(&self) {
        let files = self.active_files();

        let file_path_things = files
            .iter()
            .map(|f| {
                (
                    f.kind().supports_pixbuf()
                    // TODO: should I store full images or create downscale them to save memory?
                        && f.area().map(|x| x < 8000 * 8000).unwrap_or_default(), // image isn't too big
                    f.path(),
                )
            })
            .scan(0, |i, (b, path)| {
                // only load 10 images
                if b {
                    *i += 1;
                }

                if *i > 10 {
                    Some((false, path))
                } else {
                    Some((b, path))
                }
            })
            .collect_vec();

        let (sender, receiver) = async_channel::bounded(1);
        std::thread::spawn(move || {
            let file_paths_pixbuf = file_path_things
                .into_iter()
                .enumerate()
                .map(|(i, (b, path))| {
                    let sender = sender.clone();
                    async move {
                        sender
                            .send_blocking((
                                i,
                                match b {
                                    true => Some(Texture::from_filename(&path)),
                                    false => None,
                                },
                            ))
                            .expect("Concurrency Issues");
                    }
                })
                .collect_vec();

            runtime().block_on(join_all(file_paths_pixbuf));
        });

        let completed = std::sync::Arc::new(AtomicUsize::new(0));
        let total = self.files_count();

        glib::spawn_future_local(clone!(
            #[weak(rename_to=this)]
            self,
            async move {
                while let Ok((i, p)) = receiver.recv().await {
                    if let Some(Ok(p)) = p {
                        this.imp()
                            .input_file_store
                            .item(i as u32)
                            .and_downcast::<InputFile>()
                            .unwrap()
                            .set_pixbuf(p);
                    }
                    glib::MainContext::default().iteration(true);
                    let c = completed.clone();
                    let x = c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    if x + 1 == total {
                        this.load_pixbuf_finished();
                        break;
                    }
                }
            }
        ));
    }
}

pub trait FileOperations {
    fn add_dialog(&self);
    fn open_files(&self, files: Vec<Option<InputFile>>);
    fn open_load(&self);
    fn open_error(&self, error: Option<&str>);
    fn add_success_wrapper(&self, files: Vec<InputFile>);
}

trait StackNavigation {
    fn switch_to_stack_apply(&self);
    fn switch_to_stack_applying(&self);
    fn switch_to_stack_welcome(&self);
    fn switch_to_stack_invalid_image(&self);
    fn switch_to_stack_loading(&self);
    fn switch_back_from_loading(&self);
    fn switch_to_stack_loading_generally(&self);
}

pub trait WindowUI {
    fn apply_cancel(&self);
}

trait SettingsStore {
    fn save_window_size(&self) -> Result<(), glib::BoolError>;
    fn load_window_size(&self);
}

impl WindowUI for AppWindow {
    fn apply_cancel(&self) {}
}

impl StackNavigation for AppWindow {
    fn switch_to_stack_apply(&self) {
        self.imp().add_button.set_visible(true);
        self.imp().view_switcher.set_visible(true);
        self.imp().stack.set_visible_child_name("stack_apply");
    }

    fn switch_to_stack_applying(&self) {
        self.imp().add_button.set_visible(false);
        self.imp().view_switcher.set_visible(false);
        self.imp().stack.set_visible_child_name("stack_applying");
    }

    fn switch_to_stack_welcome(&self) {
        self.imp().add_button.set_visible(false);
        self.imp().view_switcher.set_visible(false);
        self.imp()
            .stack
            .set_visible_child_name("stack_welcome_page");
    }

    fn switch_to_stack_invalid_image(&self) {
        self.imp().add_button.set_visible(false);
        self.imp().view_switcher.set_visible(false);
        self.imp()
            .stack
            .set_visible_child_name("stack_invalid_image");
    }

    fn switch_to_stack_loading(&self) {
        self.imp().add_button.set_visible(false);
        self.imp().view_switcher.set_visible(false);
        self.imp().stack.set_visible_child_name("stack_loading");
        self.imp().loading_spinner.start();
    }

    fn switch_back_from_loading(&self) {
        self.imp().loading_spinner.stop();
    }

    fn switch_to_stack_loading_generally(&self) {
        self.switch_to_stack_loading();
    }
}

impl SettingsStore for AppWindow {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();

        imp.settings.set_int("window-width", width)?;
        imp.settings.set_int("window-height", height)?;

        imp.settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let imp = self.imp();

        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");
        let is_maximized = imp.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}

impl FileOperations for AppWindow {
    fn open_files(&self, files: Vec<Option<InputFile>>) {
        let files = files.into_iter().flatten().collect_vec();
        if files.is_empty() {
            self.show_toast(&gettext("Unsupported filetype"));
            return;
        }
        self.add_success_wrapper(files);
    }

    fn add_dialog(&self) {
        FileChooser::open_files_wrapper(
            self,
            vec![],
            AppWindow::open_load,
            AppWindow::add_success_wrapper,
            AppWindow::open_error,
        );
    }

    fn open_error(&self, error: Option<&str>) {
        if error.is_some() {
            self.switch_to_stack_invalid_image();
        }
    }

    fn open_load(&self) {
        self.switch_to_stack_loading_generally();
        self.imp().loading_spinner.start();
    }

    fn add_success_wrapper(&self, files: Vec<InputFile>) {
        self.open_success(files);
    }
}
