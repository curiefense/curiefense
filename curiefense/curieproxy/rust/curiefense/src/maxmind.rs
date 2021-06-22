use lazy_static::lazy_static;
use maxminddb::{
    geoip2::{Asn, City, Country},
    Reader,
};
use std::net::IpAddr;
#[cfg(not(test))]
use std::ops::Deref;

lazy_static! {
    // as they are lazy, these loads will not be triggered in test mode
    static ref ASN: Result<Reader<Vec<u8>>, maxminddb::MaxMindDBError> =
        Reader::open_readfile("/config/current/config/maxmind/GeoLite2-ASN.mmdb");
    static ref COUNTRY: Result<Reader<Vec<u8>>, maxminddb::MaxMindDBError> =
        Reader::open_readfile("/config/current/config/maxmind/GeoLite2-Country.mmdb");
    static ref CITY: Result<Reader<Vec<u8>>, maxminddb::MaxMindDBError> =
        Reader::open_readfile("/config/current/config/maxmind/GeoLite2-City.mmdb");
}

/// Retrieves the english name of the country associated with this IP
#[cfg(not(test))]
pub fn get_country(addr: IpAddr) -> Result<Country, String> {
    match COUNTRY.deref() {
        Err(rr) => Err(format!("could not read country db: {}", rr)),
        Ok(db) => db.lookup(addr).map_err(|rr| format!("{}", rr)),
    }
}

#[cfg(not(test))]
pub fn get_asn(addr: IpAddr) -> Result<Asn, String> {
    match ASN.deref() {
        Err(rr) => Err(format!("could not read ASN db: {}", rr)),
        Ok(db) => db.lookup(addr).map_err(|rr| format!("{}", rr)),
    }
}

#[cfg(not(test))]
pub fn get_city(addr: IpAddr) -> Result<City, String> {
    match CITY.deref() {
        Err(rr) => Err(format!("could not read city db: {}", rr)),
        Ok(db) => db.lookup(addr).map_err(|rr| format!("{}", rr)),
    }
}

#[cfg(test)]
pub fn get_country(_addr: IpAddr) -> Result<Country, String> {
    Err("TEST".into())
}

#[cfg(test)]
pub fn get_asn(_addr: IpAddr) -> Result<Asn, String> {
    Err("TEST".into())
}

#[cfg(test)]
pub fn get_city(_addr: IpAddr) -> Result<City, String> {
    Err("TEST".into())
}
