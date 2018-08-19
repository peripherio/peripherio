use resolve::resolve;

use toml;
use failure::Error;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub struct Category {
    name: String,
    path: PathBuf,
    version: String,
    author: Option<String>,
    required_symbols: Vec<String>
}

#[derive(Deserialize)]
struct LibMetaData {
    name: String,
    version: String,
    author: Option<String>,
    symbols: Vec<String>,
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Category::new(s)
    }
}


impl Category {
    pub fn resolve(name: &str) -> Result<PathBuf, Error> {
        resolve(name, "RAMI_CTG_PATH", "category.toml")
    }

    pub fn new(name: &str) -> Result<Self, Error> {
        let path = Self::resolve(name)?;
        let mut file = File::open(&path.join("category.toml"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let metadata: LibMetaData = toml::from_str(&contents)?;
        Ok(Category {
            path: path.clone(),
            name: metadata.name,
            author: metadata.author,
            version: metadata.version,
            required_symbols: metadata.symbols
        })
    }

    pub fn required_symbols(&self) -> &Vec<String> {
        &self.required_symbols
    }
}
