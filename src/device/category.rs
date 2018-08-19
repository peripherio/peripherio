use failure::Error;

use std::str::FromStr;

pub struct Category {
    name: String,
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Category {
            name: s.to_owned()
        })
    }
}
