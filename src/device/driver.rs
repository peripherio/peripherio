use config::{Config, ConfigValue};
use device::category::Category;
use device::libloading::{Library, Symbol};
use error;
use resolve::resolve;
use util;

use failure::Error;
use linked_hash_map::LinkedHashMap;
use serde_json;
use serde_yaml;
use valico::json_schema::schema::{self, CompilationSettings, Schema, ScopedSchema};
use valico::json_schema::{self, keywords, Scope};

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fmt, mem, ptr, slice};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Driver(usize);

impl Driver {
    pub fn new(id: usize) -> Self {
        Driver(id)
    }
}

pub struct Requirement {
    detects: bool,
    schema: Option<Schema>,
    type_str: String,
}

impl Requirement {
    pub fn schema(&self) -> &Option<Schema> {
        &self.schema
    }

    pub fn detects(&self) -> bool {
        self.detects
    }

    pub fn type_str(&self) -> &String {
        &self.type_str
    }
}

pub struct DriverData {
    path: PathBuf,
    name: String,
    version: String,
    author: Option<String>,
    vendor: Option<String>,
    category: Vec<Category>,
    requires: LinkedHashMap<String, Requirement>,
    driver: Library,
}

const COMMON_SYMBOLS: [&str; 2] = ["init", "detect"];

#[derive(Deserialize, Serialize)]
struct RequirementData {
    detects: Option<bool>,
    schema: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
struct LibMetaData {
    name: String,
    version: String,
    author: Option<String>,
    vendor: Option<String>,
    category: Vec<String>,
    driver: Option<String>,
    requires: LinkedHashMap<String, RequirementData>,
}

impl DriverData {
    pub fn resolve(name: &str) -> Result<PathBuf, Error> {
        resolve(name, "RAMI_PKG_PATH", "rami.yml")
    }

    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = File::open(&path.as_ref().join("rami.yml"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let metadata: LibMetaData = serde_yaml::from_str(&contents)?;

        let driver_file = metadata.driver.unwrap_or(format!("{}.so", metadata.name));
        let category = metadata
            .category
            .iter()
            .map(|c| c.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let requires = metadata
            .requires
            .into_iter()
            .map(|(k, v)| {
                let type_str = v
                    .schema
                    .as_ref()
                    .and_then(|schema| schema["type"].as_str())
                    .unwrap_or("integer")
                    .to_string();
                let compiled_schema = v
                    .schema
                    .map(|schema| {
                        schema::compile(
                            schema,
                            None,
                            CompilationSettings::new(&keywords::default(), true),
                        ).map_err(|e| error::SchemaError::from(e))
                    }).map_or(Ok(None), |r| r.map(Some))?;
                Ok((
                    k,
                    Requirement {
                        detects: v.detects.unwrap_or(false),
                        schema: compiled_schema,
                        type_str,
                    },
                ))
            }).collect::<Result<LinkedHashMap<String, Requirement>, Error>>()?;
        let inst = DriverData {
            path: path.as_ref().to_path_buf(),
            driver: Library::new(path.as_ref().join(driver_file))?,
            name: metadata.name,
            author: metadata.author,
            vendor: metadata.vendor,
            version: metadata.version,
            requires,
            category,
        };
        if !inst.validate_symbols() {
            return Err(error::SymbolsNotEnoughError {
                name: inst.name,
                requires: inst
                    .category
                    .iter()
                    .flat_map(|v| v.required_symbols())
                    .cloned()
                    .collect::<Vec<_>>(),
                common: format!("{:?}", COMMON_SYMBOLS),
            }.into());
        }
        Ok(inst)
    }

    pub fn validate_symbols(&self) -> bool {
        self.category
            .iter()
            .flat_map(|ctg| ctg.required_symbols().iter())
            .map(AsRef::as_ref)
            .chain(COMMON_SYMBOLS.into_iter().map(|e| *e))
            .all(|sym| unsafe { self.get::<fn(u32) -> u32>(sym) }.is_ok())
    }

    pub fn validate_config_value(&self, key: &str, value: &ConfigValue) -> bool {
        let scope = Scope::new();
        self.requires
            .get(key)
            .and_then(|req| req.schema().as_ref())
            .map(|schema| {
                let sschema = ScopedSchema::new(&scope, &schema);
                sschema.validate(value).is_valid()
            }).unwrap_or(true)
    }

    pub fn validate_config(&self, config: &Config) -> bool {
        config.iter().all(|(k, v)| self.validate_config_value(k, v))
    }

    pub fn detect(&self, conf: &Config) -> Result<Vec<Config>, Error> {
        let entire_size: usize = self
            .requires
            .iter()
            .fold(0, |sum, (_, v)| sum + util::size_of_type(v.type_str()));
        let buf = unsafe { util::alloc(entire_size) };
        let mut filled_size: usize = 0;
        for (k, v) in &self.requires {
            let size = util::size_of_type(v.type_str());
            if let Some(val) = conf.get(k) {
                unsafe {
                    let ptr = util::cast_to_ptr(v.type_str(), val)?;
                    ptr::copy_nonoverlapping(ptr, buf.offset(filled_size as isize), size);
                }
                filled_size += size;
            } else {
                unsafe { ptr::write_bytes(buf.offset(filled_size as isize), 0, size) };
                filled_size += size;
            }
        }
        let detect =
            unsafe { self.get::<fn(*const u8, *mut usize) -> *const *const u8>("detect")? };
        let mut ret_size: usize = 0;
        let res = detect(buf, &mut ret_size as *mut usize);
        unsafe { util::free(buf, entire_size) };
        let ary_of_conf = unsafe { slice::from_raw_parts(res, ret_size) };
        ary_of_conf
            .iter()
            .map(|ret_conf| {
                let mut newconf = conf.clone();
                let mut retrieved_size: usize = 0;
                for (k, v) in &self.requires {
                    let size = util::size_of_type(v.type_str());
                    let val = unsafe {
                        let buf = util::alloc(size);
                        ptr::copy_nonoverlapping(
                            ret_conf.offset(retrieved_size as isize),
                            buf,
                            size,
                        );
                        let val = util::cast_from_ptr(v.type_str(), buf)?.clone();
                        util::free(buf, size);
                        val
                    };
                    retrieved_size += size;
                    newconf.insert(k.to_string(), val);
                }
                Ok(newconf)
            }).collect()
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn category(&self) -> &Vec<Category> {
        &self.category
    }

    pub fn vendor(&self) -> &Option<String> {
        &self.vendor
    }

    pub unsafe fn get<'lib, T: 'lib>(&'lib self, name: &str) -> Result<Symbol<'lib, T>, Error> {
        self.driver.get(name.as_bytes()).map_err(|e| e.into())
    }
}
