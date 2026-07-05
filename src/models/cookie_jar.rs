use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use url::{form_urlencoded, form_urlencoded::Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieJar {
    #[serde(rename = "cookies")]
    map: BTreeMap<String, Value>,
}

impl CookieJar {
    pub fn new() -> CookieJar {
        CookieJar {
            map: BTreeMap::new(),
        }
    }

    pub fn extend(&mut self, cj: &CookieJar) -> CookieJar {
        for (k, v) in cj.to_map() {
            self.add(k, v);
        }
        self.clone()
    }

    pub fn add(&mut self, key: impl Into<String>, val: Value) -> Option<Value> {
        let cookie = key.into();
        self.insert(cookie, val)
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.map.get(key).map(|cookie| cookie.clone())
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        if let Value::String(val) = self.get(key)? {
            Some(val)
        } else {
            None
        }
    }

    pub fn get_bytes(&self, key: &str) -> Option<Vec<u8>> {
        self.get_string(key).map(|s| s.as_bytes().to_vec())
    }

    pub fn get_u64(&self, key: &str) -> Option<u64> {
        if let Value::Number(val) = self.get(key)? {
            val.as_u64()
        } else {
            None
        }
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        if let Value::Number(val) = self.get(key)? {
            val.as_f64()
        } else {
            None
        }
    }

    pub fn get_map(&self, key: &str) -> Option<Map<String, Value>> {
        if let Value::Object(val) = self.get(key)? {
            Some(val)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, val: Value) -> Option<Value> {
        let cookie = key.into();
        self.map.insert(cookie, val)
    }

    pub fn remove(&mut self, key: impl Into<String>) -> Option<Value> {
        self.map.remove(&key.into())
    }

    pub fn from_str(data: &str) -> CookieJar {
        let mut jar = CookieJar::new();
        for data in data.split(';') {
            for (k, v) in form_urlencoded::parse(data.as_bytes()) {
                let k = k.trim().to_string();
                let v = serde_json::from_str::<Value>(v.trim())
                    .or(serde_json::from_str::<Value>(&format!("\"{}\"", v)))
                    .unwrap_or_else(|v| Value::String(v.to_string()));
                jar.insert(k, v).map(|_| ()).unwrap_or(());
            }
        }
        jar
    }

    pub fn parse(data: &str) -> ::std::result::Result<CookieJar, String> {
        Ok(CookieJar::from_str(data))
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn to_map(&self) -> BTreeMap<String, Value> {
        self.map.clone()
    }

    pub fn to_string_map(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::<String, String>::new();
        for (k, v) in self.map.iter().map(|(k, v)| (k.to_string(), v.to_string())) {
            map.insert(k, v);
        }
        map
    }

    pub fn to_string(&self) -> String {
        let mut v = String::new();
        let mut s = &mut Serializer::new(&mut v);
        for (k, v) in &self.map {
            let v = serde_json::to_string_pretty(v).unwrap_or(v.to_string());
            s = s.append_pair(&k, &v);
        }
        v
    }
}
