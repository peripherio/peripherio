use device::libloading::Library;

use std::env;
use std::path::{Path, PathBuf};

pub struct Loader {
    path: PathBuf,
    lib: Library
}

#[derive(Debug, Clone)]
pub struct NotFoundError;

impl Loader {
    pub fn resolve(name: &str) -> Result<PathBuf, NotFoundError> {
        env::var("RAMIPATH").as_ref().map(|val| val.split(';').collect()).unwrap_or(vec![])
            .iter()
            .map(|path| Path::new(path).join(name).with_extension("so"))
            .find(|path| path.exists()).ok_or(NotFoundError)
    }

    pub fn new(name: &str) -> Result<Self, NotFoundError> {
        let path = Self::resolve(name)?;
        Ok(Loader {
            path: path.clone(),
            lib: Library::new(path).ok().ok_or(NotFoundError)? // TODO: FailedToLoadError
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
