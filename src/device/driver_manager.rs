use config::Config;
use device::driver::{Driver, DriverData};
use device::driver_spec::DriverSpec;
use resolve;
use error::DriverNotFoundError;

use failure::Error;

use std::collections::HashMap;
use std::collections::hash_map::Keys;

pub struct DriverManager {
    drivers: HashMap<Driver, DriverData>
}

impl DriverManager {
    pub fn new() -> Self {
        Self {
            drivers: HashMap::new()
        }
    }

    pub fn load_all(&mut self) -> Result<(), Error> {
        self.drivers = resolve::paths("RAMI_PKG_PATH", "rami.yml")?
            .enumerate()
            .map(|(i, path)| Ok((Driver::new(i), DriverData::new(path)?)))
            .collect::<Result<_, Error>>()?;
        Ok(())
    }

    pub fn driver_data(&self) -> impl Iterator<Item=(&Driver, &DriverData)> {
        self.drivers.iter()
    }

    pub fn drivers(&self) -> Keys<Driver, DriverData> {
        self.drivers.keys()
    }

    pub fn get_data(&self, drv: &Driver) -> Result<&DriverData, Error> {
        self.drivers.get(drv).ok_or(DriverNotFoundError.into())
    }

    pub fn suitable_drivers<'a>(&'a self, spec: &'a DriverSpec, conf: &'a Config) -> impl Iterator<Item=&'a Driver> {
        self.drivers.iter()
            .filter(move |(_, data)| spec.is_conforming(data))
            .filter(move |(_, data)| data.validate_config(conf))
            .map(|(drv, _)| drv)
    }
}
