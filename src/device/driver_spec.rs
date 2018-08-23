pub struct DriverSpec {
    vendor: Option<String>,
    category: Option<String>,
    name: Option<String>
}

impl DriverSpec {
    pub fn new(vendor: Option<String>, category: Option<String>, name: Option<String>) -> Self {
        DriverSpec {
            vendor, category. name
        }
    }
}
