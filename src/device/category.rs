use resolve::resolve;

use serde_yaml;
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
        let path = Category::resolve(s)?;
        Category::new(path)
    }
}


impl Category {
    pub fn resolve(name: &str) -> Result<PathBuf, Error> {
        resolve(name, "RAMI_CTG_PATH", "category.yml")
    }

    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = File::open(&path.as_ref().join("category.yml"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let metadata: LibMetaData = serde_yaml::from_str(&contents)?;
        Ok(Category {
            path: path.as_ref().to_path_buf(),
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
