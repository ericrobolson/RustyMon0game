//use std::fs::File;
//use std::io::prelude::*;

// This is an abstraction for the file system.
// TODO: expose a 'future' type construct for loading resources.
// Done to enable non blocking file io

const RES: &'static str = "../res/";

/// Abstracted file system.
pub struct FileSystem {}
impl FileSystem {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Attempts to load the given file.
    /// Will return a 'future' in the event that it can be done asynchronously.
    /// This is an abstracted method that attempts to load from the 'res' folder.
    /// As it can be platform dependant, this gets around that.
    pub fn load<'a>(&mut self, file: &'a str) -> Result<FileLoad, FileOpenError> {
        todo!()

        //let mut file = match File::open(format!("{}{}", RES, file)) {
        //    Ok(f) => f,
        //    Err(e) => {
        //        return Err(FileOpenError::DoesNotExist);
        //    }
        //};
        //
        //let mut buffer = vec![];
        //file.read_to_end(&mut buffer).unwrap();
        //Ok(FileLoad::File(buffer))
    }

    ///
    pub fn save<'a>(&mut self, file_name: &'a str, contents: &[u8]) -> Result<(), FileSaveError> {
        todo!()
    }
}

/// Operation for loading a file
pub enum FileLoad {
    Loading,
    File(alloc::vec::Vec<u8>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FileOpenError {
    DoesNotExist,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FileSaveError {
    NotSupported,
}
