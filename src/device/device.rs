#[derive(Copy, Clone, Eq, Hash)]
struct Device(u64);

struct DeviceData(&Driver, Config);

pub struct DeviceManager {
    devices: HashMap<Device, DeviceData>,
    names: HashMap<Device, String>
}

impl DeviceManager {
    pub fn add(&self, drv: &Driver, conf: Config) -> Result<Device, Error> {
        let device = Device(self.devices.len());
        self.devices.insert(device, DeviceData(drv, conf));
        self.names.insert(device, self.generate_name());
        device
    }

    fn generate_name(&self) -> String {
        "TODO: Implement deveice name generation!".to_string()
    }
}
