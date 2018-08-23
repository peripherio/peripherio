use device::driver::Driver;
use resolve;

use failure::Error;

pub struct DriverManager {
    drivers: Vec<Driver>
}

impl DriverManager {
    pub fn new() -> Self {
        Self {
            drivers: Vec::new()
        }
    }

    pub fn load_all(&mut self) -> Result<(), Error> {
        self.drivers = resolve::paths("RAMI_PKG_PATH", "rami.yml")?
            .map(|path| Driver::new(path))
            .collect::<Result<_, _>>()?;
        Ok(())
    }
}
