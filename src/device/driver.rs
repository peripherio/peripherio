use config::global::GLOBAL_SCHEMA;
use config::{Config, ConfigValue};
use device::category::{Category, Signature};
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

pub struct DriverData {
    path: PathBuf,
    name: String,
    version: String,
    author: Option<String>,
    vendor: Option<String>,
    category: Vec<Category>,
    requires: Vec<String>,
    detects: Vec<String>,
    merged_schemas: LinkedHashMap<String, serde_json::Value>,
    driver: Library,
}

pub const COMMON_SYMBOLS: [&str; 2] = ["init", "detect"];

#[derive(Deserialize, Serialize)]
struct LibMetaData {
    name: String,
    version: String,
    author: Option<String>,
    vendor: Option<String>,
    category: Vec<String>,
    driver: Option<String>,

    /// What user must specify
    requires: Option<Vec<String>>,

    /// Driver can detect them via `detect()`, also use can specify them via config
    detects: Option<Vec<String>>,

    /// Schema for user's specification
    schemas: Option<LinkedHashMap<String, serde_json::Value>>,
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

        let requires = metadata.requires.unwrap_or_default();
        let schemas = metadata.schemas.unwrap_or_default();
        let merged_schemas = requires
            .iter()
            .map(|key| {
                Ok((
                    key.clone(),
                    match (schemas.get(key), GLOBAL_SCHEMA.get(key)) {
                        (Some(schema), Some(v)) => {
                            let mut new_val = v.clone();
                            util::merge_value(&mut new_val, &schema);
                            new_val
                        }
                        (Some(schema), None) => schema.clone(),
                        (None, Some(v)) => v.clone(),
                        (None, None) => {
                            return Err(error::UnknownConfigError { name: key.clone() }.into())
                        }
                    },
                ))
            }).collect::<Result<LinkedHashMap<_, _>, Error>>()?;

        let inst = DriverData {
            path: path.as_ref().to_path_buf(),
            driver: Library::new(path.as_ref().join(driver_file))?,
            name: metadata.name,
            author: metadata.author,
            vendor: metadata.vendor,
            version: metadata.version,
            detects: metadata.detects.unwrap_or_default(),
            requires,
            merged_schemas,
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
            .flat_map(|ctg| ctg.required_symbols())
            .map(AsRef::as_ref)
            .chain(COMMON_SYMBOLS.into_iter().map(|e| *e))
            .all(|sym| unsafe { self.get::<fn(u32) -> u32>(sym) }.is_ok())
    }

    pub fn validate_config_value(&self, key: &str, value: &ConfigValue) -> bool {
        let mut scope = Scope::new();
        self.merged_schemas
            .get(key)
            .map(|schema_data| {
                let sschema = scope
                    .compile_and_return(schema_data.clone(), true)
                    .ok()
                    .unwrap();
                sschema.validate(value).is_valid()
            }).unwrap_or(true)
    }

    pub fn validate_config(&self, config: &Config) -> bool {
        config.iter().all(|(k, v)| self.validate_config_value(k, v))
    }

    pub fn dispatch(
        &self,
        command: &str,
        args: &HashMap<String, serde_json::Value>,
        conf: &Config,
    ) -> Result<HashMap<String, serde_json::Value>, Error> {
        let category: &Category = self
            .category
            .iter()
            .find(|v| {
                v.required_symbols()
                    .collect::<Vec<_>>()
                    .contains(&&command.to_string())
            }).ok_or(
                Error::from(error::UnknownCommandError {
                    name: command.to_string(),
                }),
            )?;

        let Signature { args: args_sig, returns: returns_sig } = category.signatures().get(command).unwrap();
        let args_schema = args_sig.clone().unwrap_or_default();
        let (conf_buf, conf_buf_size) = util::value_to_c_struct(&self.merged_schemas, conf)?;
        let (args_buf, args_buf_size) = util::value_to_c_struct(&args_schema, args)?;

        let command = unsafe { self.get::<fn(*const u8, *const u8) -> *const u8>(command)? };
        let returns = command(args_buf, conf_buf);
        unsafe { util::free(conf_buf, conf_buf_size) };
        unsafe { util::free(args_buf, args_buf_size) };
        let returns_schema = returns_sig.clone().unwrap_or_default();
        util::c_struct_to_value(&returns_schema, returns)
    }

    pub fn detect(&self, conf: &Config) -> Result<Vec<Config>, Error> {
        let (buf, entire_size) = util::value_to_c_struct(&self.merged_schemas, conf)?;
        let detect =
            unsafe { self.get::<fn(*const u8, *mut usize) -> *const *const u8>("detect")? };
        let mut ret_size: usize = 0;
        let res = detect(buf, &mut ret_size as *mut usize);
        unsafe { util::free(buf, entire_size) };
        let ary_of_conf = unsafe { slice::from_raw_parts(res, ret_size) };
        ary_of_conf
            .iter()
            .map(|ret_conf| util::c_struct_to_value(&self.merged_schemas, *ret_conf))
            .collect()
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
