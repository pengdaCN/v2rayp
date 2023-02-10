use anyhow::bail;
use std::collections::HashMap;
use std::str::FromStr;
use urlencoding::decode;

pub struct UriQueries {
    values: HashMap<String, uri_query::Value>,
    seq: i32,
}

mod uri_query {
    pub struct Value {
        pub score: i32,
        pub data: Vec<String>,
    }

    impl Value {
        pub fn new(score: i32) -> Self {
            Value {
                score,
                data: Vec::new(),
            }
        }
    }
}

impl UriQueries {
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.values.get(key).map(|v| &v.data)
    }
}

impl FromStr for UriQueries {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seq = 0;
        let mut values = HashMap::new();
        for item in s.trim().split('&') {
            if item.contains(';') {
                bail!("invalid semicolon separator in query")
            }

            let mut kv = item.split('=');
            let key = {
                if let Some(raw) = kv.next() {
                    decode(raw.trim())?.to_string()
                } else {
                    continue;
                }
            };

            let vals = if let Some(v) = values.get_mut(&key) {
                v
            } else {
                let vals = uri_query::Value::new(seq);
                seq += 1;

                let key2 = key.clone();
                values.insert(key, vals);
                values.get_mut(&key2).unwrap()
            };

            if let Some(raw) = kv.next() {
                vals.data.push(decode(raw.trim())?.to_string());
            }
        }

        Ok(UriQueries { seq, values })
    }
}
