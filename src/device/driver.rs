use device::category::Category;
use device::libloading::{Library, Symbol};
use error;
use resolve::resolve;
use config::ConfigValue;

use serde_yaml;
use serde_json;
use valico::json_schema::{self, Scope};
use valico::json_schema::schema::{self, ScopedSchema, Schema, CompilationSettings};
use failure::Error;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::fmt;

pub struct Requirement {
    detects: bool,
    schema: Option<Schema>
}

pub struct Driver {
    path: PathBuf,
    name: String,
    version: String,
    author: Option<String>,
    category: Vec<Category>,
    requires: HashMap<String, Requirement>,
    driver: Library
}

const COMMON_SYMBOLS: [&str; 2] = ["init", "detect"];

#[derive(Deserialize, Serialize)]
struct RequirementData {
    detects: Option<bool>,
    schema: Option<serde_json::Value>
}

#[derive(Deserialize, Serialize)]
struct LibMetaData {
    name: String,
    version: String,
    author: Option<String>,
    category: Vec<String>,
    driver: Option<String>,
    requires: HashMap<String, RequirementData>
}

impl Driver {
    pub fn resolve(name: &str) -> Result<PathBuf, Error> {
        resolve(name, "RAMI_PKG_PATH", "rami.yml")
    }

    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = File::open(&path.as_ref().join("rami.yml"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let metadata: LibMetaData = serde_yaml::from_str(&contents)?;

        let driver_file = metadata.driver.unwrap_or(format!("{}.so", metadata.name));
        let category = metadata.category.iter().map(|c| c.parse()).collect::<Result<Vec<_>, _>>()?;
        let requires = metadata.requires.into_iter().map(|(k, v)| {
            let compiled_schema = v.schema.map(|schema| {
                schema::compile(schema, None, CompilationSettings::new(&HashMap::new(), true)).map_err(|e| error::SchemaError::from(e))
            }).map_or(Ok(None), |r| r.map(Some))?;
            Ok((k, Requirement {
                detects: v.detects.unwrap_or(false),
                schema: compiled_schema
            }))
        }).collect::<Result<HashMap<String, Requirement>, Error>>()?;
        Ok(Driver {
            path: path.as_ref().to_path_buf(),
            driver: Library::new(path.as_ref().join(driver_file))?,
            name: metadata.name,
            author: metadata.author,
            version: metadata.version,
            requires,
            category
        })
    }

    pub fn validate_symbols(&self) -> bool {
        self.category.iter().flat_map(|ctg| ctg.required_symbols().iter()).map(AsRef::as_ref)
            .chain(COMMON_SYMBOLS.into_iter().map(|e|*e))
            .all(|sym| unsafe { self.get::<fn(u32) -> u32>(sym) }.is_ok())
    }

    pub fn validate_config(&self, key: &str, value: &ConfigValue) -> bool {
        let scope = Scope::new();
        self.requires.get(key).and_then(|req| req.schema.as_ref()).map(|schema| {
            let sschema = ScopedSchema::new(&scope, &schema);
            sschema.validate(value).is_valid()
        }).unwrap_or(true)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub unsafe fn get<'lib, T: 'lib>(&'lib self, name: &str) -> Result<Symbol<'lib, T>, Error> {
        self.driver.get(name.as_bytes()).map_err(|e| e.into())
    }
}
