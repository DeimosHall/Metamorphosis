use glib::{ParamSpec, ParamSpecEnum, ParamSpecObject, ParamSpecString, Value};
use gtk::{
    gdk::{Texture, gdk_pixbuf::Pixbuf},
    gio, glib,
    prelude::*,
    subclass::prelude::*,
};

use log::debug;
use once_cell::sync::Lazy;
use std::{
    cell::{Cell, Ref, RefCell},
    sync::Arc,
};

use crate::models::filetypes::FileType;
use crate::models::{input_file, metadata::Metadata};

mod imp {
    use super::*;

    pub struct InputFile {
        pub path: RefCell<String>,
        pub kind: Cell<FileType>,
        pub pixbuf: RefCell<Option<Texture>>,
        pub metadata: RefCell<Option<Arc<Metadata>>>,
    }

    #[::glib::object_subclass]
    impl ObjectSubclass for InputFile {
        const NAME: &'static str = "MetamorphosisInputFile";
        type Type = input_file::InputFile;

        fn new() -> Self {
            Self {
                path: RefCell::new("/invalid-path".to_string()),
                kind: Cell::new(FileType::Unknown),
                pixbuf: RefCell::new(None),
                metadata: RefCell::new(None),
            }
        }
    }

    impl ObjectImpl for InputFile {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("path").readwrite().build(),
                    ParamSpecEnum::builder::<FileType>("kind")
                        .readwrite()
                        .build(),
                    ParamSpecObject::builder::<Pixbuf>("pixbuf")
                        .write_only()
                        .build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "path" => {
                    let p = value.get::<String>().expect("Value must be a String");
                    self.path.replace(p);
                }
                "kind" => {
                    let p = value.get::<FileType>().expect("Value must be a filetype");
                    self.kind.set(p);
                }
                "pixbuf" => {
                    let p = value.get::<Texture>().expect("Value must be a Pixbuf");
                    self.pixbuf.replace(Some(p));
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "path" => self.path.borrow().to_value(),
                "kind" => self.kind.get().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct InputFile(ObjectSubclass<imp::InputFile>);
}

impl InputFile {
    pub fn new(file: &gio::File) -> Option<Self> {
        let path = file.path()?;
        debug!("Path: {:?}", path);

        let file_info = file
            .query_info(
                gio::FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE,
                gio::FileQueryInfoFlags::NONE,
                gio::Cancellable::NONE,
            )
            .ok()?;

        let mimetype = file_info.content_type()?.as_str().to_owned();

        let extension = FileType::from_mimetype(&mimetype);

        extension.map(|extension| {
            glib::Object::builder::<Self>()
                .property("path", path.to_str().expect("Should convert path to str"))
                .property("kind", extension)
                .build()
        })
    }

    pub fn pixbuf(&self) -> Ref<'_, Option<Texture>> {
        self.imp().pixbuf.borrow()
    }

    pub fn set_pixbuf(&self, p: Texture) {
        self.imp().pixbuf.replace(Some(p));
    }

    pub fn area(&self) -> Option<u64> {
        let (width, height) = self.metadata()?.dimensions()?;
        width.checked_mul(height) // None if overflow
    }

    pub fn path(&self) -> String {
        self.imp().path.borrow().to_string()
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.path()).exists()
    }

    pub fn kind(&self) -> FileType {
        self.imp().kind.get()
    }

    pub fn metadata(&self) -> Option<Arc<Metadata>> {
        self.imp().metadata.borrow().clone()
    }

    pub fn set_metadata(&self, metadata: Arc<Metadata>) {
        self.imp().metadata.replace(Some(metadata));
    }
}
