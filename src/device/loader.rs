use device::libloading::{Library, Symbol};
use error::LibraryNotFoundError;

use failure::Error;

use std::env;
use std::path::{Path, PathBuf};

pub struct Loader {
    path: PathBuf,
    lib: Library
}

impl Loader {
    pub fn resolve(name: &str) -> Result<PathBuf, Error> {
        env::var("RAMI_PKG_PATH").as_ref().map(|val| val.split(';').collect()).unwrap_or(vec![])
            .iter()
            .map(|path| Path::new(path).join(name).with_extension("so"))
            .find(|path| path.exists()).ok_or(LibraryNotFoundError{name: name.to_owned()}.into())
    }

    pub fn new(name: &str) -> Result<Self, Error> {
        let path = Self::resolve(name)?;
        Ok(Loader {
            path: path.clone(),
            lib: Library::new(path)?
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub unsafe fn get<'lib, T: 'lib>(&'lib self, name: &str) -> Result<Symbol<'lib, T>, Error> {
        self.lib.get(name.as_bytes()).map_err(|e| e.into())
    }
}
