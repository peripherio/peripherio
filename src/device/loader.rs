use device::category::Category;

use device::libloading::{Library, Symbol};
use toml;
use error::LibraryNotFoundError;

use failure::Error;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;

pub struct Loader {
    path: PathBuf,
    name: String,
    version: String,
    author: Option<String>,
    category: Vec<Category>,
    driver: Library
}

#[derive(Deserialize)]
struct LibMetaData {
    name: String,
    version: String,
    author: Option<String>,
    category: Vec<String>,
    driver: Option<String>
}

impl Loader {
    pub fn resolve(name: &str) -> Result<PathBuf, Error> {
        env::var("RAMI_PKG_PATH").as_ref().map(|val| val.split(';').collect()).unwrap_or(vec![])
            .iter()
            .map(|path| Path::new(path).join(name).join("rami.toml"))
            .find(|path| path.is_file()).and_then(|path| path.parent())
            .map(|path| path.to_path_buf())
            .ok_or(LibraryNotFoundError{name: name.to_owned()}.into())
    }

    pub fn new(name: &str) -> Result<Self, Error> {
        let path = Self::resolve(name)?;
        let file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let metadata: LibMetaData = toml::from_str(&contents)?;

        let driver_file = metadata.driver.unwrap_or(format!("{}.so", metadata.name));
        let category = metadata.category.iter().map(|c| c.parse()).collect::<Result<Vec<_>, _>>()?;
        Ok(Loader {
            path: path.clone(),
            driver: Library::new(path.join(driver_file))?,
            name: metadata.name,
            author: metadata.author,
            version: metadata.version,
            category
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub unsafe fn get<'lib, T: 'lib>(&'lib self, name: &str) -> Result<Symbol<'lib, T>, Error> {
        self.driver.get(name.as_bytes()).map_err(|e| e.into())
    }
}
