use device::driver::DriverData;

pub struct DriverSpec {
    vendor: Option<String>,
    category: Option<String>,
    name: Option<String>,
}

impl DriverSpec {
    pub fn new(vendor: Option<String>, category: Option<String>, name: Option<String>) -> Self {
        DriverSpec {
            vendor,
            category,
            name,
        }
    }

    pub fn is_conforming(&self, driver: &DriverData) -> bool {
        self.name
            .as_ref()
            .map(|n| n == driver.name())
            .unwrap_or(true)
            && self
                .category
                .as_ref()
                .map(|n| {
                    driver
                        .category()
                        .into_iter()
                        .map(|c| c.name())
                        .collect::<Vec<_>>()
                        .contains(&n)
                }).unwrap_or(true)
            && self
                .vendor
                .as_ref()
                .map(|n| Some(n) == driver.vendor().as_ref())
                .unwrap_or(true)
    }
}
