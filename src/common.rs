use anyhow::bail;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use urlencoding::{decode, encode};

#[derive(Default)]
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.values.get(key).map(|v| &v.data)
    }

    pub fn append(&mut self, key: &str, value: String) {
        let vals = if let Some(vals) = self.values.get_mut(key) {
            vals
        } else {
            self.values
                .insert(String::from(key), uri_query::Value::new(self.seq));
            self.seq += 1;

            self.values.get_mut(key).unwrap()
        };

        vals.data.push(value)
    }

    pub fn put(&mut self, key: &str, value: String) -> Option<Vec<String>> {
        let data = self.values.remove(key);

        let seq = if let Some(old) = &data {
            old.score
        } else {
            let seq = self.seq;

            self.seq += 1;

            seq
        };

        let mut vals = uri_query::Value::new(seq);
        vals.data.push(value);

        self.values.insert(String::from(key), vals);

        data.map(|x| x.data)
    }

    pub fn del(&mut self, key: &str) -> Option<Vec<String>> {
        self.values.remove(key).map(|v| v.data)
    }
}

impl Display for UriQueries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut items: Vec<_> = self.values.iter().collect();
        items.sort_by(|a, b| a.1.score.cmp(&b.1.score));

        let kvs: Vec<_> = items
            .into_iter()
            .flat_map(|x| {
                x.1.data
                    .iter()
                    .map(|v| format!("{key}={val}", key = encode(x.0), val = encode(v)))
                    .collect::<Vec<_>>()
            })
            .collect();

        write!(f, "{}", kvs.join("&"))
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
