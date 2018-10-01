use driver::driver::DriverData;
use protos::peripherio as protos;

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
                })
                .unwrap_or(true)
            && self
                .vendor
                .as_ref()
                .map(|n| Some(n) == driver.vendor().as_ref())
                .unwrap_or(true)
    }
}

impl<'a> From<&'a protos::DriverSpecification> for DriverSpec {
    fn from(p_spec: &'a protos::DriverSpecification) -> Self {
        let empty_or = |v| {
            if v == "" {
                None
            } else {
                Some(v)
            }
        };
        let vendor = p_spec.get_vendor().to_string();
        let category = p_spec.get_category().to_string();
        let name = p_spec.get_name().to_string();
        DriverSpec::new(empty_or(vendor), empty_or(category), empty_or(name))
    }
}

impl<'a> From<Option<&'a protos::DriverSpecification>> for DriverSpec {
    fn from(p_spec: Option<&'a protos::DriverSpecification>) -> Self {
        if let Some(p) = p_spec {
            DriverSpec::from(p)
        } else {
            DriverSpec::new(None, None, None)
        }
    }
}
