use config::Config;
use driver::driver::{Driver, DriverData, DRIVER_ENV, DRIVER_FILE};
use driver::driver_spec::DriverSpec;
use error::DriverNotFoundError;
use resolve;

use failure::Error;

use std::collections::hash_map::{Iter, Keys};
use std::collections::HashMap;

pub struct DriverManager {
    drivers: HashMap<Driver, DriverData>,
}

impl DriverManager {
    pub fn new() -> Self {
        Self {
            drivers: HashMap::new(),
        }
    }

    pub fn load_all(&mut self) -> Result<(), Error> {
        self.drivers = resolve::paths(DRIVER_ENV, DRIVER_FILE)?
            .enumerate()
            .map(|(i, path)| Ok((Driver::new(i), DriverData::new(path)?)))
            .collect::<Result<_, Error>>()?;
        Ok(())
    }

    pub fn driver_data(&self) -> Iter<'_, Driver, DriverData> {
        self.drivers.iter()
    }

    pub fn drivers(&self) -> Keys<Driver, DriverData> {
        self.drivers.keys()
    }

    pub fn get_data(&self, drv: &Driver) -> Result<&DriverData, Error> {
        self.drivers.get(drv).ok_or(DriverNotFoundError.into())
    }

    pub fn suitable_drivers(&self, spec: &DriverSpec, conf: &Config) -> Vec<Driver> {
        self.drivers
            .iter()
            .filter(move |(_, data)| spec.is_conforming(data))
            .filter(move |(_, data)| data.validate_config(conf))
            .map(|(drv, _)| drv)
            .cloned()
            .collect()
    }
}
