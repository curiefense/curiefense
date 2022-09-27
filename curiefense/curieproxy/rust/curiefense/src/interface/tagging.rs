use crate::config::contentfilter::SectionIdx;
use serde::ser::SerializeMap;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Location {
    Request,
    Attributes,
    Ip,
    Uri,
    Path,
    Pathpart(usize),
    PathpartValue(usize, String),
    RefererPath,
    RefererPathpart(usize),
    RefererPathpartValue(usize, String),
    UriArgument(String),
    UriArgumentValue(String, String),
    RefererArgument(String),
    RefererArgumentValue(String, String),
    Body,
    BodyArgument(String),
    BodyArgumentValue(String, String),
    Headers,
    Header(String),
    HeaderValue(String, String),
    Cookies,
    Cookie(String),
    CookieValue(String, String),
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Location::*;
        match self {
            Request => write!(f, "request"),
            Attributes => write!(f, "attributes"),
            Ip => write!(f, "ip"),
            Uri => write!(f, "uri"),
            Path => write!(f, "path"),
            Pathpart(p) => write!(f, "path part {}", p),
            PathpartValue(p, v) => write!(f, "path part {}={}", p, v),
            UriArgument(a) => write!(f, "URI argument {}", a),
            UriArgumentValue(a, v) => write!(f, "URI argument {}={}", a, v),
            Body => write!(f, "body"),
            BodyArgument(a) => write!(f, "body argument {}", a),
            BodyArgumentValue(a, v) => write!(f, "body argument {}={}", a, v),
            Headers => write!(f, "headers"),
            Header(h) => write!(f, "header {}", h),
            HeaderValue(h, v) => write!(f, "header {}={}", h, v),
            Cookies => write!(f, "cookies"),
            Cookie(c) => write!(f, "cookie {}", c),
            CookieValue(c, v) => write!(f, "cookie {}={}", c, v),
            RefererArgument(a) => write!(f, "Referer argument {}", a),
            RefererArgumentValue(a, v) => write!(f, "Referer argument {}={}", a, v),
            RefererPath => write!(f, "referer path"),
            RefererPathpart(p) => write!(f, "referer path part {}", p),
            RefererPathpartValue(p, v) => write!(f, "referer path part {}={}", p, v),
        }
    }
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map: <S as serde::Serializer>::SerializeMap = serializer.serialize_map(None)?;
        self.serialize_with_parent::<S>(&mut map)?;
        map.end()
    }
}

impl Location {
    pub fn parent(&self) -> Option<Self> {
        use Location::*;
        match self {
            Request => None,
            Attributes => Some(Request),
            Ip => Some(Attributes),
            Uri => Some(Request),
            Path => Some(Uri),
            Pathpart(_) => Some(Path),
            PathpartValue(k, _) => Some(Pathpart(*k)),
            UriArgument(_) => Some(Uri),
            UriArgumentValue(n, _) => Some(UriArgument(n.clone())),
            Body => Some(Request),
            BodyArgument(_) => Some(Body),
            BodyArgumentValue(n, _) => Some(BodyArgument(n.clone())),
            Headers => Some(Request),
            Header(_) => Some(Headers),
            HeaderValue(n, _) => Some(Header(n.clone())),
            Cookies => Some(Request),
            Cookie(_) => Some(Cookies),
            CookieValue(n, _) => Some(Cookie(n.clone())),
            RefererArgument(_) => Some(Header("referer".to_string())),
            RefererArgumentValue(n, _) => Some(RefererArgument(n.clone())),
            RefererPath => Some(Header("referer".to_string())),
            RefererPathpart(_) => Some(RefererPath),
            RefererPathpartValue(k, _) => Some(RefererPathpart(*k)),
        }
    }

    pub fn get_locations(&self) -> HashSet<Self> {
        let mut out = HashSet::new();
        let mut start = self.clone();
        while let Some(p) = start.parent() {
            out.insert(start);
            start = p;
        }
        out.insert(start);
        out
    }

    pub fn request() -> HashSet<Self> {
        let mut out = HashSet::new();
        out.insert(Location::Request);
        out
    }

    pub fn body() -> HashSet<Self> {
        let mut out = HashSet::new();
        out.insert(Location::Body);
        out
    }

    pub fn from_value(idx: SectionIdx, name: &str, value: &str) -> Self {
        match idx {
            SectionIdx::Headers => Location::HeaderValue(name.to_string(), value.to_string()),
            SectionIdx::Cookies => Location::CookieValue(name.to_string(), value.to_string()),
            SectionIdx::Path => Location::Path,
            // TODO: track body / uri args
            SectionIdx::Args => Location::UriArgumentValue(name.to_string(), value.to_string()),
        }
    }
    pub fn from_name(idx: SectionIdx, name: &str) -> Self {
        match idx {
            SectionIdx::Headers => Location::Header(name.to_string()),
            SectionIdx::Cookies => Location::Cookie(name.to_string()),
            SectionIdx::Path => Location::Path,
            // TODO: track body / uri args
            SectionIdx::Args => Location::UriArgument(name.to_string()),
        }
    }
    pub fn from_section(idx: SectionIdx) -> Self {
        match idx {
            SectionIdx::Headers => Location::Headers,
            SectionIdx::Cookies => Location::Cookies,
            SectionIdx::Path => Location::Path,
            // TODO: track body / uri args
            SectionIdx::Args => Location::Uri,
        }
    }
    pub fn serialize_with_parent<S: serde::Serializer>(
        &self,
        map: &mut <S as serde::Serializer>::SerializeMap,
    ) -> Result<(), S::Error> {
        match self {
            Location::Request => (),
            Location::Attributes => {
                map.serialize_entry("section", "attributes")?;
            }
            Location::Ip => {
                map.serialize_entry("section", "ip")?;
            }
            Location::Uri => {
                map.serialize_entry("section", "uri")?;
            }
            Location::RefererPath => {
                map.serialize_entry("section", "referer path")?;
            }
            Location::RefererPathpart(part) => {
                map.serialize_entry("part", part)?;
            }
            Location::RefererPathpartValue(_, value) => {
                map.serialize_entry("value", value)?;
            }
            Location::Path => {
                map.serialize_entry("section", "path")?;
            }
            Location::Pathpart(part) => {
                map.serialize_entry("part", part)?;
            }
            Location::PathpartValue(_, value) => {
                map.serialize_entry("value", value)?;
            }
            Location::UriArgument(name) => {
                map.serialize_entry("name", name)?;
            }
            Location::UriArgumentValue(_, value) => {
                map.serialize_entry("value", value)?;
            }
            Location::RefererArgument(name) => {
                map.serialize_entry("name", name)?;
            }
            Location::RefererArgumentValue(_, value) => {
                map.serialize_entry("value", value)?;
            }
            Location::Body => {
                map.serialize_entry("section", "body")?;
            }
            Location::BodyArgument(name) => {
                map.serialize_entry("name", name)?;
            }
            Location::BodyArgumentValue(_, value) => {
                map.serialize_entry("value", value)?;
            }
            Location::Headers => {
                map.serialize_entry("section", "headers")?;
            }
            Location::Header(name) => {
                map.serialize_entry("name", name)?;
            }
            Location::HeaderValue(_, value) => {
                map.serialize_entry("value", value)?;
            }
            Location::Cookies => {
                map.serialize_entry("section", "cookies")?;
            }
            Location::Cookie(name) => {
                map.serialize_entry("name", name)?;
            }
            Location::CookieValue(_, value) => {
                map.serialize_entry("value", value)?;
            }
        }
        if let Some(p) = self.parent() {
            p.serialize_with_parent::<S>(map)?;
        }
        Ok(())
    }
}

/// computes all parents
pub fn all_parents(locs: HashSet<Location>) -> HashSet<Location> {
    let mut out = locs.clone();
    let mut to_compute = locs;
    loop {
        let to_compute_prime = to_compute.iter().filter_map(|l| l.parent()).collect::<HashSet<_>>();
        let diff = to_compute_prime.difference(&out).cloned().collect::<HashSet<_>>();
        if diff.is_empty() {
            break;
        }
        out.extend(diff.clone());
        to_compute = diff;
    }
    out
}

/// a newtype representing tags, to make sure they are tagified when inserted
#[derive(Debug, Clone, Default)]
pub struct Tags(pub HashMap<String, HashSet<Location>>);

fn tagify(tag: &str) -> String {
    fn filter_char(c: char) -> char {
        if c.is_ascii_alphanumeric() || c == ':' {
            c
        } else {
            '-'
        }
    }
    tag.to_lowercase().chars().map(filter_char).collect()
}

impl Serialize for Tags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.0.keys())
    }
}

impl Tags {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, value: &str, loc: Location) {
        let locs = std::iter::once(loc).collect();
        self.0.insert(tagify(value), locs);
    }

    pub fn insert_locs(&mut self, value: &str, locs: HashSet<Location>) {
        self.0.insert(tagify(value), locs);
    }

    pub fn insert_qualified(&mut self, id: &str, value: &str, loc: Location) {
        let locs = std::iter::once(loc).collect();
        self.insert_qualified_locs(id, value, locs);
    }

    pub fn insert_qualified_locs(&mut self, id: &str, value: &str, locs: HashSet<Location>) {
        let mut to_insert = id.to_string();
        to_insert.push(':');
        to_insert += &tagify(value);
        self.0.insert(to_insert, locs);
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0)
    }

    pub fn from_slice(slice: &[(String, Location)]) -> Self {
        Tags(
            slice
                .iter()
                .map(|(s, l)| (tagify(s), std::iter::once(l.clone()).collect()))
                .collect(),
        )
    }

    pub fn contains(&self, s: &str) -> bool {
        self.0.contains_key(s)
    }

    pub fn get(&self, s: &str) -> Option<&HashSet<Location>> {
        self.0.get(s)
    }

    pub fn as_hash_ref(&self) -> &HashMap<String, HashSet<Location>> {
        &self.0
    }

    pub fn selector(&self) -> String {
        let mut tvec: Vec<&str> = self.0.keys().map(|s| s.as_ref()).collect();
        tvec.sort_unstable();
        tvec.join("*")
    }

    pub fn intersect(&self, other: &HashSet<String>) -> HashMap<String, HashSet<Location>> {
        let mut out = HashMap::new();
        for (k, v) in &self.0 {
            if other.contains(k) {
                out.insert(k.clone(), v.clone());
            }
        }

        out
    }

    pub fn intersect_tags(&self, other: &HashSet<String>) -> Self {
        Tags(self.intersect(other))
    }

    pub fn has_intersection(&self, other: &HashSet<String>) -> bool {
        other.iter().any(|t| self.0.contains_key(t))
    }

    pub fn merge(&mut self, other: Self) {
        for (k, v) in other.0.into_iter() {
            let e = self.0.entry(k).or_default();
            (*e).extend(v);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RawTags(HashSet<String>);

impl RawTags {
    pub fn insert(&mut self, value: &str) {
        self.0.insert(tagify(value));
    }

    pub fn insert_qualified(&mut self, id: &str, value: &str) {
        let mut to_insert = id.to_string();
        to_insert.push(':');
        to_insert += &tagify(value);
        self.0.insert(to_insert);
    }

    pub fn as_hash_ref(&self) -> &HashSet<String> {
        &self.0
    }

    pub fn intersect<'t>(
        &'t self,
        other: &'t HashSet<String>,
    ) -> std::collections::hash_set::Intersection<'t, std::string::String, std::collections::hash_map::RandomState>
    {
        self.0.intersection(other)
    }

    pub fn has_intersection(&self, other: &HashSet<String>) -> bool {
        self.intersect(other).next().is_some()
    }

    pub fn with_loc(self, locations: &Location) -> Tags {
        Tags(
            self.0
                .into_iter()
                .map(|k| (k, std::iter::once(locations.clone()).collect()))
                .collect(),
        )
    }
    pub fn with_locs(self, locations: &HashSet<Location>) -> Tags {
        Tags(self.0.into_iter().map(|k| (k, locations.clone())).collect())
    }
}

impl std::iter::FromIterator<String> for RawTags {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut out = RawTags::default();
        for s in iter {
            out.insert(&s);
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tag_selector() {
        let tags = Tags::from_slice(&[
            ("ccc".to_string(), Location::Request),
            ("bbb".to_string(), Location::Request),
            ("aaa".to_string(), Location::Request),
        ]);
        assert_eq!(tags.selector(), "aaa*bbb*ccc");
    }

    #[test]
    fn tag_selector_r() {
        let tags = Tags::from_slice(&[
            ("aaa".to_string(), Location::Request),
            ("ccc".to_string(), Location::Request),
            ("bbb".to_string(), Location::Request),
        ]);
        assert_eq!(tags.selector(), "aaa*bbb*ccc");
    }
}
