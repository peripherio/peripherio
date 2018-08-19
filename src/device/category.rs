use failure::Error;

use std::str::FromStr;

pub struct Category {
    name: String,
    required_symbols: Vec<String>
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Category {
            name: s.to_owned(),
            required_symbols: Vec::new()
        })
    }
}

impl Category {
    pub fn required_symbols(&self) -> &Vec<String> {
        self.requried_symbols
    }
}
