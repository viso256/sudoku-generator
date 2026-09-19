use std::{fs::File, num::NonZero};

use time::PlainDateTime;
use typst::{
    Library, LibraryExt, World,
    diag::FileResult,
    foundations::{Bytes, Datetime, Duration},
    syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot},
    text::{Font, FontBook},
    utils::LazyHash,
};

pub struct CustomWorld {
    library: LazyHash<Library>,
    font_book: LazyHash<FontBook>,
    main_file: Source,
    main_file_id: FileId,
    json_file: Bytes,
    json_file_id: FileId,
    fonts: [Font; 4],
    datetime: Option<PlainDateTime>,
}

impl CustomWorld {
    const LIBERATION_SANS_BOLD: &[u8] =
        include_bytes!("../pdf/fonts/liberation/LiberationSans-Bold.ttf");
    const LIBERATION_SANS_BOLD_ITALIC: &[u8] =
        include_bytes!("../pdf/fonts/liberation/LiberationSans-BoldItalic.ttf");
    const LIBERATION_SANS_ITALIC: &[u8] =
        include_bytes!("../pdf/fonts/liberation/LiberationSans-Italic.ttf");
    const LIBERATION_SANS_REGULAR: &[u8] =
        include_bytes!("../pdf/fonts/liberation/LiberationSans-Regular.ttf");

    pub fn new(json: String) -> Self {
        let library = LazyHash::from(Library::default());
        let fonts = [
            Font::new(Bytes::new(Self::LIBERATION_SANS_BOLD), 0)
                .expect("font should be able to load"),
            Font::new(Bytes::new(Self::LIBERATION_SANS_BOLD_ITALIC), 0)
                .expect("font should be able to load"),
            Font::new(Bytes::new(Self::LIBERATION_SANS_ITALIC), 0)
                .expect("font should be able to load"),
            Font::new(Bytes::new(Self::LIBERATION_SANS_REGULAR), 0)
                .expect("font should be able to load"),
        ];
        let font_book = LazyHash::from(FontBook::from_fonts(fonts.iter()));
        let main_file_id = FileId::new(RootedPath::new(
            VirtualRoot::Project,
            VirtualPath::new("sudoku.typ").unwrap(),
        ));
        let main_file = Source::new(main_file_id, include_str!("../pdf/sudoku.typ").to_owned());
        let json_file_id = FileId::new(RootedPath::new(
            VirtualRoot::Project,
            VirtualPath::new("sudoku.json").unwrap(),
        ));
        let json_file = Bytes::new(json);
        let datetime = None;
        Self {
            library,
            font_book,
            main_file,
            main_file_id,
            json_file,
            json_file_id,
            fonts,
            datetime,
        }
    }

    /// Set the current datetime to be used by the document, for platforms that do
    /// not support OffsetDateTime::now_local()
    pub fn with_datetime(&mut self, datetime: PlainDateTime) -> &mut Self {
        self.datetime = Some(datetime);
        self
    }
}

impl World for CustomWorld {
    #[doc = " The standard library."]
    #[doc = ""]
    #[doc = " Can be created through `Library::build()`."]
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    #[doc = " Metadata about all known fonts."]
    fn book(&self) -> &LazyHash<FontBook> {
        &self.font_book
    }

    #[doc = " Get the file id of the main source file."]
    fn main(&self) -> FileId {
        self.main_file_id
    }

    #[doc = " Try to access the specified file location as a source file."]
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_file_id {
            return Ok(self.main_file.clone());
        }
        Err(typst::diag::FileError::NotSource)
    }

    #[doc = " Try to access the specified file."]
    #[doc = ""]
    #[doc = " For file locations for which [`source`](Self::source) succeeds, this"]
    #[doc = " should also succeed. The [`Bytes`] can be cheaply created as a view into"]
    #[doc = " an existing [`Source`] through [`Bytes::from_string`]."]
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Ok(source) = self.source(id).map(Bytes::from_string) {
            return Ok(source);
        }

        if id == self.json_file_id {
            Ok(self.json_file.clone())
        } else {
            Err(typst::diag::FileError::Other(Some(
                format!("Not Available: {:?}", id).into(),
            )))
        }
    }

    #[doc = " Try to access the font with the given index in the font book."]
    #[doc = ""]
    #[doc = " Note that the index is not guaranteed to be in bounds of the font book"]
    #[doc = " returned by this world\'s `book()` function. This is the case because"]
    #[doc = " this function may be invoked with indices from an outdated or different"]
    #[doc = " font book during incremental compilation validation."]
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    #[doc = " Get the current date."]
    #[doc = ""]
    #[doc = " If no offset is specified, the local date should be chosen. Otherwise,"]
    #[doc = " the UTC date should be chosen with the corresponding offset."]
    #[doc = ""]
    #[doc = " If this function returns `None`, Typst\'s `datetime` function will"]
    #[doc = " return an error."]
    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        let offset: time::Duration = offset.map(time::Duration::from).unwrap_or_default();
        let now = time::OffsetDateTime::now_local().ok()? + offset;
        let datetime = PlainDateTime::new(now.date(), now.time());
        Some(Datetime::Datetime(datetime))
    }
}
