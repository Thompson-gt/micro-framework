use std::{ffi::OsStr, ffi::OsString, path::PathBuf};

use crate::types::custom_abstractions::file_type::FileType;

/// internal file type for static hosted files
#[derive(Debug, Clone)]
pub struct StaticFile {
    pub name: String,
    pub location: OsString,
    pub kind: FileType,
}

impl StaticFile {
    pub fn new(file: PathBuf) -> Self {
        let name = file
            .file_name()
            .expect("failed to get file name?")
            .to_owned();
        let name: Vec<_> = name
            .to_str()
            .unwrap()
            .split(".")
            .into_iter()
            .map(|chunk| chunk.to_string())
            .collect();
        let name = name
            .first()
            .expect("failed to get the name of static file?")
            .to_owned();
        let location = file.as_os_str().to_owned();
        let kind =
            match Self::get_file_type(file.extension().expect("failed to get the file extension?"))
            {
                Some(ft) => ft,
                None => panic!(
                    "invalid extension type given to `StaticFile`{:?}",
                    file.to_str().take().unwrap()
                ),
            };

        Self {
            name: name.to_owned(),
            location,
            kind,
        }
    }

    fn get_file_type(extension: &OsStr) -> Option<FileType> {
        match extension
            .to_str()
            .expect("failed to convert `OsStr` to `&str`")
        {
            "html" => Some(FileType::HTML),
            "png" => Some(FileType::PNG),
            "jpg" => Some(FileType::JPG),
            "jpeg" => Some(FileType::JPEG),
            "txt" => Some(FileType::TEXT),
            // this is just for testing purposes 'MAKE SURE TO REMOVE'
            "rs" => Some(FileType::RS),
            _ => None,
        }
    }
}
