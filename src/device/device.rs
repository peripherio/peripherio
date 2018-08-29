use config::Config;
use driver::driver::Driver;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Device(usize);

impl Device {
    pub fn with_id(id: usize) -> Self {
        Device(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}
