use ipnet::IpNet;
use lazy_static::lazy_static;
use maxminddb::{
    geoip2::{Asn, City, Country},
    Reader,
};
#[cfg(not(test))]
use std::ops::Deref;
use std::{net::IpAddr, path::PathBuf};

struct GeoStruct {
    asn: Reader<Vec<u8>>,
    country: Reader<Vec<u8>>,
    city: Reader<Vec<u8>>,
}

lazy_static! {
    // as they are lazy, these loads will not be triggered in test mode
    static ref GEO: Result<GeoStruct, maxminddb::MaxMindDBError> = {
        let maxmind_root = std::env::var("MAXMIND_ROOT").unwrap_or_else(|_| "/cf-config/current/config/maxmind".to_string());
        let maxmind_asn = std::env::var("MAXMIND_ASN").unwrap_or_else(|_| "GeoLite2-ASN.mmdb".to_string());
        let maxmind_country = std::env::var("MAXMIND_COUNTRY").unwrap_or_else(|_| "GeoLite2-Country.mmdb".to_string());
        let maxmind_city = std::env::var("MAXMIND_CITY").unwrap_or_else(|_| "GeoLite2-City.mmdb".to_string());
        let root_path = PathBuf::from(maxmind_root);
        let mut asn_path = root_path.clone();
        asn_path.push(maxmind_asn);
        let mut country_path = root_path.clone();
        country_path.push(maxmind_country);
        let mut city_path = root_path;
        city_path.push(maxmind_city);
        Reader::open_readfile(asn_path)
            .and_then(|asn| Reader::open_readfile(country_path)
            .and_then(|country| Reader::open_readfile(city_path)
            .map(|city| GeoStruct { asn, country, city } )))
    };
}

#[cfg(not(test))]
fn compute_network<T>(data: T, addr: IpAddr, prefix_len: usize) -> (T, Option<IpNet>) {
    let network = IpNet::new(addr, prefix_len as u8).ok();
    (data, network)
}

/// Retrieves the english name of the country associated with this IP
#[cfg(not(test))]
pub fn get_country(addr: IpAddr) -> Result<(Country<'static>, Option<IpNet>), String> {
    match GEO.deref() {
        Err(rr) => Err(format!("could not read country db: {}", rr)),
        Ok(db) => match db.country.lookup_prefix(addr) {
            Ok((country, prefix_len)) => Ok(compute_network::<Country>(country, addr, prefix_len)),
            Err(rr) => Err(format!("{}", rr)),
        },
    }
}

#[cfg(not(test))]
pub fn get_asn(addr: IpAddr) -> Result<(Asn<'static>, Option<IpNet>), String> {
    match GEO.deref() {
        Err(rr) => Err(format!("could not read ASN db: {}", rr)),
        Ok(db) => match db.asn.lookup_prefix(addr) {
            Ok((asn, prefix_len)) => Ok(compute_network::<Asn>(asn, addr, prefix_len)),
            Err(rr) => Err(format!("{}", rr)),
        },
    }
}

#[cfg(not(test))]
pub fn get_city(addr: IpAddr) -> Result<(City<'static>, Option<IpNet>), String> {
    match GEO.deref() {
        Err(rr) => Err(format!("could not read city db: {}", rr)),
        Ok(db) => match db.city.lookup_prefix(addr) {
            Ok((city, prefix_len)) => Ok(compute_network::<City>(city, addr, prefix_len)),
            Err(rr) => Err(format!("{}", rr)),
        },
    }
}

#[cfg(test)]
pub fn get_country(_addr: IpAddr) -> Result<(Country<'static>, Option<IpNet>), String> {
    Err("TEST".into())
}

#[cfg(test)]
pub fn get_asn(_addr: IpAddr) -> Result<(Asn<'static>, Option<IpNet>), String> {
    Err("TEST".into())
}

#[cfg(test)]
pub fn get_city(_addr: IpAddr) -> Result<(City<'static>, Option<IpNet>), String> {
    Err("TEST".into())
}
