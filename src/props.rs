use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    String(String),
    Bool(bool),
    I32(i32),
    F64(f64),
    Nothing,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::I32(i) => write!(f, "{}", i),
            Value::F64(n) => write!(f, "{}", n),
            Value::Nothing => write!(f, "null"),
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(b) = s.parse() {
            return Ok(Value::Bool(b));
        }

        if let Ok(i) = s.parse() {
            return Ok(Value::I32(i));
        }

        if let Ok(f) = s.parse() {
            return Ok(Value::F64(f));
        }

        Ok(Value::String(s.to_string()))
    }
}


// Implement From for each type
impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Properties {
    properties: HashMap<String, Value>,
}

impl Properties {
    pub fn new() -> Self {
        Properties {
            properties: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }

    pub fn set(&mut self, key: impl ToString, value: impl Into<Value>) {
        self.properties.insert(
            key.to_string(),
            value.into(),
        );
    }

    pub fn join(&self, other: &Self) -> Self {
        let mut properties = self.properties.clone();
        for (key, value) in &other.properties {
            properties.insert(key.clone(), value.clone());
        }
        Properties { properties }
    }

    pub fn append(&mut self, other: &mut Self) {
        for (key, value) in other.properties.drain() {
            self.properties.insert(key, value);
        }
    }

    pub fn remove(&mut self, key: &str) {
        self.properties.remove(key);
    }

    pub fn extend(&mut self, properties: impl IntoIterator<Item=(String, Value)>) {
        for (key, value) in properties {
            self.properties.insert(key, value);
        }
    }
}

impl Display for Properties {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[")?;
        for (key, value) in &self.properties {
            write!(f, "{}={}, ", key, value)?;
        }
        write!(f, "]")
    }
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            properties: HashMap::new(),
        }
    }
}

impl<T: AsRef<str>> Index<T> for Properties {
    type Output = Value;

    fn index(&self, key: T) -> &Self::Output {
        self.properties.get(key.as_ref()).expect("Key not found")
    }
}

impl<T: AsRef<str>> IndexMut<T> for Properties {
    fn index_mut(&mut self, key: T) -> &mut Self::Output {
        // If the key does not exist, insert a new key with a default value
        self.properties.entry(key.as_ref().to_string()).or_insert(Value::Nothing)
    }
}

impl IntoIterator for Properties {
    type Item = (String, Value);
    type IntoIter = std::collections::hash_map::IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.properties.into_iter()
    }
}

impl<'a> IntoIterator for &'a Properties {
    type Item = (&'a String, &'a Value);
    type IntoIter = std::collections::hash_map::Iter<'a, String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.properties.iter()
    }
}