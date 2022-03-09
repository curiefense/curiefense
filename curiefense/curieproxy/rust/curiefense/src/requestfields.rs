use crate::config::contentfilter::Transformation;
use crate::config::utils::{DataSource, XDataSource};
use crate::utils::decoders::DecodingResult;
use crate::utils::masker;
use std::collections::HashSet;
use std::collections::{hash_map, HashMap};

/// a newtype for user supplied data that can collide
/// more or less like a HashMap, but concatenates entries with a separator on insert
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestField {
    pub decoding: Vec<Transformation>,
    pub fields: HashMap<String, (String, HashSet<DataSource>)>,
}

impl RequestField {
    fn base_add(&mut self, key: String, ds: DataSource, value: String) {
        self.fields
            .entry(key)
            .and_modify(|(v, pds)| {
                v.push(' ');
                v.push_str(&value);
                pds.insert(ds.clone());
            })
            .or_insert({
                let mut hs = HashSet::new();
                hs.insert(ds);
                (value, hs)
            });
    }

    pub fn add(&mut self, key: String, ds: DataSource, value: String) {
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
                self.base_add(key.clone() + ":decoded", DataSource::DecodedFrom(key.clone()), v);
            }
        }
        self.base_add(key, ds, value);
    }

    pub fn mask(&mut self, masking_seed: &[u8], key: &str) -> HashSet<XDataSource> {
        let remask = self
            .fields
            .get_mut(key)
            .map(|(v, ds)| {
                *v = masker(masking_seed, v);
                ds.clone()
            })
            .unwrap_or_default();

        remask
            .into_iter()
            .flat_map(|d| match d {
                DataSource::Root => HashSet::new(),
                DataSource::DecodedFrom(fr) => self.mask(masking_seed, &fr),
                DataSource::X(x) => {
                    let mut o = HashSet::new();
                    o.insert(x);
                    o
                }
                DataSource::FromBody => self.mask(masking_seed, "RAW_BODY"),
            })
            .collect()
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.fields.get(k).map(|(v, _)| v)
    }

    pub fn get_str(&self, k: &str) -> Option<&str> {
        self.fields.get(k).map(|(s, _)| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.fields.iter().map(|(k, (v, _))| (k.as_str(), v.as_str()))
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            self.iter()
                .map(|(k, v)| (k.to_string(), serde_json::Value::String(v.to_string())))
                .collect(),
        )
    }

    pub fn new(decoding: &[Transformation]) -> Self {
        RequestField {
            decoding: decoding.to_vec(),
            fields: HashMap::default(),
        }
    }

    pub fn singleton(decoding: &[Transformation], k: String, ds: DataSource, v: String) -> Self {
        let mut out = RequestField::new(decoding);
        out.add(k, ds, v);
        out
    }

    /// a bit unsafe w.r.t. matching, but I don't know how to type this :(
    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, String, (String, HashSet<DataSource>)> {
        self.fields.iter_mut()
    }

    pub fn from_iterator<I: IntoIterator<Item = (String, DataSource, String)>>(
        dec: &[Transformation],
        iter: I,
    ) -> Self {
        let mut out = RequestField::new(dec);
        for (k, ds, v) in iter {
            out.add(k, ds, v);
        }
        out
    }

    #[cfg(test)]
    pub fn raw_create(decoding: &[Transformation], content: &[(&str, &DataSource, &str)]) -> Self {
        RequestField {
            decoding: decoding.to_vec(),
            fields: content
                .iter()
                .map(|(k, ds, v)| {
                    let mut hs: HashSet<DataSource> = HashSet::new();
                    hs.insert((*ds).clone());
                    (k.to_string(), (v.to_string(), hs))
                })
                .collect(),
        }
    }
}
