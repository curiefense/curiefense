#[macro_use]

pub mod avltree;
pub mod sigset;

use mlua::prelude::*;
use mlua::serde::LuaSerdeExt;

use cidr::AnyIpCidr;
use maxminddb::geoip2;
use std::cmp::Ordering;
use std::net::IpAddr;
use std::str::FromStr;

use avltree::AVLTreeMap;
use sigset::{SigSet, SigSetError};

//////////////// MAXMIND GEOIP ////////////////

#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::enum_variant_names)]
enum GeoIPError {
    DBNotLoadedError,
    AddrParseError(std::net::AddrParseError),
    MMError(maxminddb::MaxMindDBError),
    LookupError(maxminddb::MaxMindDBError),
}

#[allow(clippy::upper_case_acronyms)]
pub struct GeoIP {
    city_db: Option<maxminddb::Reader<Vec<u8>>>,
    country_db: Option<maxminddb::Reader<Vec<u8>>>,
    asn_db: Option<maxminddb::Reader<Vec<u8>>>,
}

impl GeoIP {
    fn new() -> GeoIP {
        GeoIP {
            city_db: None,
            country_db: None,
            asn_db: None,
        }
    }

    fn load_city(&mut self, pth: &std::path::Path) -> Result<(), GeoIPError> {
        match maxminddb::Reader::open_readfile(pth) {
            Ok(db) => {
                self.city_db = Some(db);
                Ok(())
            }
            Err(x) => Err(GeoIPError::MMError(x)),
        }
    }

    fn load_country(&mut self, pth: &std::path::Path) -> Result<(), GeoIPError> {
        match maxminddb::Reader::open_readfile(pth) {
            Ok(db) => {
                self.country_db = Some(db);
                Ok(())
            }
            Err(x) => Err(GeoIPError::MMError(x)),
        }
    }

    fn load_asn(&mut self, pth: &std::path::Path) -> Result<(), GeoIPError> {
        match maxminddb::Reader::open_readfile(pth) {
            Ok(db) => {
                self.asn_db = Some(db);
                Ok(())
            }
            Err(x) => Err(GeoIPError::MMError(x)),
        }
    }

    fn lookup_asn(&self, ip_str: String) -> Result<(Option<u32>, Option<String>), GeoIPError> {
        match &self.asn_db {
            None => Err(GeoIPError::DBNotLoadedError),
            Some(db) => match FromStr::from_str(&ip_str) {
                Err(x) => Err(GeoIPError::AddrParseError(x)),
                Ok(ip) => match db.lookup(ip) {
                    Err(x) => Err(GeoIPError::LookupError(x)),
                    Ok(res) => {
                        let asn: geoip2::Asn = res;
                        let org = asn.autonomous_system_organization;
                        Ok((asn.autonomous_system_number, org))
                    }
                },
            },
        }
    }
    fn lookup_country(&self, ip_str: String) -> Result<Option<geoip2::Country>, GeoIPError> {
        match &self.country_db {
            None => Err(GeoIPError::DBNotLoadedError),
            Some(db) => match FromStr::from_str(&ip_str) {
                Err(x) => Err(GeoIPError::AddrParseError(x)),
                Ok(ip) => match db.lookup(ip) {
                    Err(x) => Err(GeoIPError::LookupError(x)),
                    Ok(res) => Ok(res),
                },
            },
        }
    }

    fn lookup_city(&self, ip_str: String) -> Result<Option<geoip2::City>, GeoIPError> {
        match &self.city_db {
            None => Err(GeoIPError::DBNotLoadedError),
            Some(db) => match FromStr::from_str(&ip_str) {
                Err(x) => Err(GeoIPError::AddrParseError(x)),
                Ok(ip) => match db.lookup(ip) {
                    Err(x) => Err(GeoIPError::LookupError(x)),
                    Ok(res) => Ok(res),
                },
            },
        }
    }
}

pub fn new_geoipdb(_: &Lua, _: ()) -> LuaResult<GeoIP> {
    Ok(GeoIP::new())
}

impl mlua::UserData for GeoIP {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("load_asn_db", |_, this: &mut GeoIP, pth: String| {
            match this.load_asn(std::path::Path::new(&pth)) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        });
        methods.add_method_mut("load_country_db", |_, this: &mut GeoIP, pth: String| {
            match this.load_country(std::path::Path::new(&pth)) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        });
        methods.add_method_mut("load_city_db", |_, this: &mut GeoIP, pth: String| {
            match this.load_city(std::path::Path::new(&pth)) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        });
        methods.add_method("lookup_asn", |lua: &Lua, this: &GeoIP, value: String| {
            let mut res = Vec::new();
            if let Ok((asn, org)) = this.lookup_asn(value) {
                res.push(asn.to_lua(lua).unwrap());
                res.push(org.to_lua(lua).unwrap());
            }
            Ok(mlua::MultiValue::from_vec(res))
        });
        methods.add_method("lookup_country", |lua: &Lua, this: &GeoIP, value: String| {
            match this.lookup_country(value) {
                Ok(Some(country)) => {
                    // Unlike for other lookup steps,
                    // we are returning the whole City object.
                    // We may want to do the same for ASN the lookup.
                    Ok(lua.to_value(&country).ok())
                }
                _ => Ok(None),
            }
        });
        methods.add_method("lookup_city", |lua: &Lua, this: &GeoIP, value: String| {
            match this.lookup_city(value) {
                Ok(Some(city)) => {
                    // Unlike for other lookup steps,
                    // we are returning the whole City object.
                    // We may want to do the same for ASN the lookup.
                    Ok(lua.to_value(&city).ok())
                }
                _ => Ok(None),
            }
        });
    }
}

//////////////// IP SET ////////////////

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct IPSet(AVLTreeMap<AnyIpCidr, String>);

fn cmp(net: &AnyIpCidr, ip: &AnyIpCidr) -> Ordering {
    let eq = match (
        ip.first_address(),
        ip.last_address(),
        net.first_address(),
        net.last_address(),
    ) {
        (Some(kf), Some(kl), Some(vf), Some(vl)) => ((kf <= vl) && (vf <= kl)),
        (_, _, None, None) => true,
        (None, None, Some(_), Some(_)) => true,
        (_, _, _, _) => panic!("internal error"),
    };
    if eq {
        Ordering::Equal
    } else {
        net.cmp(ip)
    }
}

impl IPSet {
    pub fn new() -> IPSet {
        IPSet(AVLTreeMap::new())
    }

    pub fn contains(&self, key: &AnyIpCidr) -> bool {
        self.0.contains_custom(key, cmp)
    }

    pub fn get(&self, key: &AnyIpCidr) -> Option<&String> {
        self.0.get_custom(key, cmp)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: AnyIpCidr, value: String) -> bool {
        self.0.insert(key, value)
    }
}

impl mlua::UserData for IPSet {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("len", |_, this: &IPSet, _: ()| Ok(this.len()));
        methods.add_method("contains", |_, this: &IPSet, value: String| {
            match AnyIpCidr::from_str(&value) {
                Ok(a) => Ok(Some(this.contains(&a))),
                Err(_) => Ok(None),
            }
        });
        methods.add_method("get", |_, this: &IPSet, value: String| {
            match AnyIpCidr::from_str(&value) {
                Ok(a) => match this.get(&a) {
                    Some(v) => Ok(Some(v.clone())),
                    None => Ok(None),
                },
                Err(_) => Ok(None),
            }
        });
        methods.add_method_mut(
            "add",
            |_, this: &mut IPSet, (key, value): (String, String)| match AnyIpCidr::from_str(&key) {
                Ok(k) => Ok(Some(this.insert(k, value))),
                Err(_) => Ok(None),
            },
        );
    }
}

pub fn new_ip_set(_: &Lua, _: ()) -> LuaResult<IPSet> {
    let ipset = IPSet::new();
    Ok(ipset)
}

//////////////// SIG SET ////////////////

pub fn new_sig_set(_: &Lua, _: ()) -> LuaResult<SigSet> {
    Ok(SigSet::new())
}

impl mlua::UserData for SigSet {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add", |_, this: &mut SigSet, (r, i): (String, String)| {
            match this.add(r, i) {
                Ok(_) => Ok(Some(true)),
                Err(_) => Ok(None),
            }
        });
        methods.add_method_mut("compile", |_, this: &mut SigSet, _: ()| match this.compile() {
            Ok(_) => Ok(Some(true)),
            Err(SigSetError::RegexCompileError(_)) => Ok(Some(false)),
            _ => Ok(None),
        });
        methods.add_method_mut("clear", |_, this: &mut SigSet, _: ()| {
            this.clear();
            Ok(())
        });
        methods.add_method("is_match", |_, this: &SigSet, m: String| match this.is_match(&m) {
            Ok(res) => Ok(Some(res)),
            Err(_) => Ok(None),
        });
        methods.add_method("is_match_id", |_, this: &SigSet, m: String| {
            match this.is_match_id(&m) {
                Ok(res) => match res {
                    None => Ok(None),
                    Some(x) => Ok(Some(x.clone())),
                },
                Err(_) => Ok(None),
            }
        });
        methods.add_method("is_match_ids", |_, this: &SigSet, m: String| {
            match this.is_match_ids(&m) {
                Ok(res) => {
                    let mut v = Vec::new();
                    for &r in res.iter() {
                        v.push(r.clone())
                    }
                    Ok(Some(v))
                }
                Err(_) => Ok(None),
            }
        });
    }
}

pub fn test_regex(_: &Lua, val: String) -> LuaResult<Option<String>> {
    match regex::Regex::new(&val) {
        Ok(_) => Ok(None),
        Err(x) => Ok(Some(format!("{:?}", x))),
    }
}

//////////////// MOD HASH ////////////////

pub fn modhash(_: &Lua, (val, m): (String, u32)) -> LuaResult<Option<u32>> {
    let digest = md5::compute(val);

    let res = match m {
        0 => None,
        m => {
            let h: u128 = u128::from_be_bytes(*digest);
            Some((h % (m as u128)) as u32)
        }
    };
    Ok(res)
}

//////////////// IP TO NUM ////////////////

pub fn iptonum(_: &Lua, ip: String) -> LuaResult<Option<String>> {
    match IpAddr::from_str(&ip) {
        Ok(r) => match r {
            IpAddr::V4(a) => Ok(Some(u32::from_be_bytes(a.octets()).to_string())),
            IpAddr::V6(a) => Ok(Some(u128::from_be_bytes(a.octets()).to_string())),
        },
        Err(_) => Ok(None),
    }
}

//////////////// DECODE URL ////////////////

pub fn decodeurl(lua: &Lua, args: LuaMultiValue) -> LuaResult<String> {
    let res: LuaResult<String> = FromLuaMulti::from_lua_multi(args, lua);
    match res {
        Ok(url_str) => Ok(curiefense::utils::url::urldecode_str(&url_str)),
        _ => Ok("".into()),
    }
}

pub fn encodeurl(lua: &Lua, args: LuaMultiValue) -> LuaResult<String> {
    let res: LuaResult<String> = FromLuaMulti::from_lua_multi(args, lua);
    match res {
        Ok(url_str) => Ok(urlencoding::encode(&url_str)),
        _ => Ok("".into()),
    }
}

/////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipset() {
        let mut ipset = IPSet::new();

        let ip1 = ["1.2.3.0/24", "1.1.1.1/32", "192.168.8.0/24", "10.0.0.0/8"];
        let ip2 = [
            "192.168.8.24",
            "192.168.0.0/16",
            "192.168.8.64/29",
            "1.2.2.0/23",
            "10.1.2.3",
            "10.0.0.0/7",
            "0.0.0.0/0",
        ];
        let ip3 = ["1.2.4.0/24", "1.1.1.2/32", "192.168.7.0/24", "2.0.0.0/8"];

        for n in ip1.iter() {
            let a = AnyIpCidr::from_str(&n).unwrap();
            println!("Inserting {:?}", a);
            ipset.0.insert(a, "hello".to_string());
        }
        assert!(ipset.len() == ip1.len());

        for n in ip1.iter() {
            let a = AnyIpCidr::from_str(&n).unwrap();
            println!("Testing {:?}", a);
            assert!(ipset.contains(&a));
        }

        for n in ip2.iter() {
            let a = AnyIpCidr::from_str(&n).unwrap();
            println!("Testing {:?}", a);
            assert!(ipset.contains(&a));
        }

        for n in ip3.iter() {
            let a = AnyIpCidr::from_str(&n).unwrap();
            println!("Testing {:?}", a);
            assert!(!ipset.contains(&a));
        }
    }
}
