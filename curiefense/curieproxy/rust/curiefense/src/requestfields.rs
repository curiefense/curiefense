use crate::config::contentfilter::Transformation;
use crate::utils::decoders::DecodingResult;
use std::collections::{hash_map, HashMap};

/// a newtype for user supplied data that can collide
/// more or less like a HashMap, but concatenates entries with a separator on insert
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestField {
    pub decoding: Vec<Transformation>,
    pub fields: HashMap<String, String>,
}

impl RequestField {
    fn base_add(&mut self, key: String, value: String) {
        self.fields
            .entry(key)
            .and_modify(|v| {
                v.push(' ');
                v.push_str(&value);
            })
            .or_insert(value);
    }

    pub fn add(&mut self, key: String, value: String) {
        let mut v = value.clone();
        // try to insert each value as its decoded base64 version, if it makes sense
        if !&v.is_empty() {
            let mut changed = false;
            for tr in self.decoding.iter() {
                match tr {
                    Transformation::Base64Decode => {
                        if let Ok(n) = crate::utils::decoders::base64dec_all_str(&v) {
                            v = n;
                            changed = true;
                        }
                    }
                    Transformation::UrlDecode => {
                        if let DecodingResult::Changed(ns) = crate::utils::decoders::urldecode_str(&v) {
                            v = ns;
                            changed = true;
                        }
                    }
                    Transformation::HtmlEntitiesDecode => {
                        // this code is not robust enough, as it fails on the first entity error, and will not decode anything
                        // ie. "foo &gt&gt;" will not be decoded, but it should return "foo &gt>"
                        if let DecodingResult::Changed(ns) = crate::utils::decoders::htmlentities(&v) {
                            v = ns;
                            changed = true;
                        }
                    }
                    Transformation::UnicodeDecode => {
                        if let DecodingResult::Changed(ns) = crate::utils::decoders::parse_unicode(&v) {
                            v = ns;
                            changed = true;
                        }
                    }
                }
            }
            if changed {
                self.base_add(key.clone() + ":decoded", v);
            }
        }
        self.base_add(key, value);
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.fields.get(k)
    }

    pub fn get_str(&self, k: &str) -> Option<&str> {
        self.fields.get(k).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn iter(&self) -> hash_map::Iter<'_, String, String> {
        self.fields.iter()
    }

    pub fn new(decoding: &[Transformation]) -> Self {
        RequestField {
            decoding: decoding.to_vec(),
            fields: HashMap::default(),
        }
    }

    pub fn singleton(decoding: &[Transformation], k: String, v: String) -> Self {
        let mut out = RequestField::new(decoding);
        out.add(k, v);
        out
    }

    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, String, String> {
        self.fields.iter_mut()
    }

    pub fn from_iterator<I: IntoIterator<Item = (String, String)>>(dec: &[Transformation], iter: I) -> Self {
        let mut out = RequestField::new(dec);
        for (k, v) in iter {
            out.add(k, v);
        }
        out
    }

    #[cfg(test)]
    pub fn raw_create(decoding: &[Transformation], content: &[(&str, &str)]) -> Self {
        RequestField {
            decoding: decoding.to_vec(),
            fields: content.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        }
    }
}
