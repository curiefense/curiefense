use serde::Serialize;
use std::collections::HashMap;

/// newtype for the BT format
pub struct NameValue<'t, K: Eq + std::hash::Hash, V> {
    inner: &'t HashMap<K, V>,
}

impl<'t, K: Eq + std::hash::Hash, V> NameValue<'t, K, V> {
    pub fn new(inner: &'t HashMap<K, V>) -> Self {
        NameValue { inner }
    }
}

impl<'t, K: Eq + std::hash::Hash + std::fmt::Display, V: Serialize> Serialize for NameValue<'t, K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.inner.iter().map(|(k, v)| BigTableKV {
            name: k.to_string(),
            value: v,
        }))
    }
}

/// newtype for big tables KV format
#[derive(Serialize)]
pub struct BigTableKV<K, V> {
    pub name: K,
    pub value: V,
}
