use std::collections::HashMap;
use std::str::FromStr;

pub struct UriQueries {
    inner: HashMap<String, Vec<String>>,
}

impl FromStr for UriQueries {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}