use anyhow::bail;
use std::collections::HashMap;
use std::str::FromStr;
use urlencoding::decode;

pub struct UriQueries {
    values: HashMap<String, Vec<String>>,
    sorted: Vec<String>,
}

impl FromStr for UriQueries {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: HashMap<String, Vec<String>> = HashMap::new();
        let mut sorted: Vec<String> = Vec::new();

        for item in s.trim().split('&') {
            if item.contains(';') {
                bail!("invalid semicolon separator in query")
            }

            let mut kv = item.split('=');
            let key = {
                if let Some(key) = kv.next() {
                    decode(key)?.to_string()
                } else {
                    continue;
                }
            };

            let vals = if let Some(vals) = values.get_mut(&key) {
                vals
            } else {
                sorted.push(key.clone());
                values.insert(key, Vec::new());

                values.get_mut(sorted.last().unwrap()).unwrap()
            };

            if let Some(val) = kv.next() {
                let value = decode(val)?.to_string();
                vals.push(value)
            }
        }

        Ok(UriQueries { values, sorted })
    }
}
