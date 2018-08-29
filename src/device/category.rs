use resolve::resolve;

use failure::Error;
use linked_hash_map::LinkedHashMap;
use serde_json::value::Value;
use serde_yaml;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Signature {
    pub args: Option<LinkedHashMap<String, Value>>,
    pub returns: Option<LinkedHashMap<String, Value>>,
}

pub struct Category {
    name: String,
    path: PathBuf,
    version: String,
    author: Option<String>,
    required_signatures: HashMap<String, Signature>,
}

#[derive(Deserialize)]
struct LibMetaData {
    name: String,
    version: String,
    author: Option<String>,
    signatures: HashMap<String, Signature>,
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
            required_signatures: metadata.signatures,
        })
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn required_symbols(&self) -> impl Iterator<Item = &String> {
        self.required_signatures.keys()
    }

    pub fn signatures(&self) -> &HashMap<String, Signature> {
        &self.required_signatures
    }
}
