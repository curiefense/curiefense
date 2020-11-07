#[macro_use]
extern crate mlua_derive;
use mlua::prelude::*;
use std::time::{SystemTime,UNIX_EPOCH};
use aes::Aes128;
use ofb::Ofb;
use ofb::cipher::{NewStreamCipher, SyncStreamCipher};
use hex_literal::hex;
use base64;
//use hex;
use std::error::Error;
use getrandom;

type AesOfb = Ofb<Aes128>;

const KEY: [u8; 16]  = hex!("0aac364ef2d29f9649b714099e8a231f");


const JSAPP: &str = include_str!("jsapp.js");
const JSBIO: &str = include_str!("jsbio.js");

const DELIM: &str = "@@@";
const ZEBRA_DELIM: &str = ";$(hash);_xcalc(arguments.calle);";

const CONTRACT: u32 = 3;

fn hello(_: &Lua, name: String) -> LuaResult<()> {
    println!("hello, {}!", name);
    Ok(())
}

fn js_app(_: &Lua, _:()) -> LuaResult<String> {
    Ok(JSAPP.to_string())
}

fn js_bio(_: &Lua, _:()) -> LuaResult<String> {
    Ok(JSBIO.to_string())
}

fn gen_new_seed(_: &Lua, ua:String) -> LuaResult<String> {

//    let mut iv: [u8;16] = [0; 16 ];

    let mut iv = vec![0; 16 ];
    getrandom::getrandom(&mut iv).expect("Failed to get random bytes from system");

    let now = SystemTime::now();
    let now_i = now.duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    let now_s: String = now_i.to_string();

    let token: String = now_s + DELIM + &ua;

    let mut buf = token.into_bytes();

    let mut cipher = AesOfb::new_var(&KEY, &iv).expect("AES OFB cipher creation failed");
    cipher.apply_keystream(&mut buf);

    iv.extend(&buf);

    let b64 = base64::encode(&iv);
    Ok(b64)
}


fn verify_workproof(_: &Lua, (zebra, ua):(String,String)) -> LuaResult<Option<String>> {
    match do_verify_workproof(zebra, ua) {
        Ok(s) => Ok(Some(s)),
        Err(e) => { println!("Error occured: {}", e); Ok(None) },
    }
}

fn count_leading_zero_bits(bytes:&[u8]) -> u32 {
    let mut n:u32 = 0;
    for b in bytes {
        if *b == 0 {
            n = n + 8;
        } else {
            for i in 0..7 {
                if (*b << i) & 0x80 == 0 {
                    n = n + 1
                } else {
                    return n
                }
            }
        }
    }
    n
}


#[test]
fn test_clzb() {
    assert_eq!(count_leading_zero_bits(&[0,0xff]),      8);
    assert_eq!(count_leading_zero_bits(&[0x10, 0, 0]),  3);
    assert_eq!(count_leading_zero_bits(&[0,0,0,0]),     32);
    assert_eq!(count_leading_zero_bits(&[0,0,4]),       21);
}


fn do_verify_workproof(enc_zebra:String, ua:String) -> Result<String, Box<dyn Error>> {

    // DECODE ZEBRA

    let zebra_v: Vec<u8> = base64::decode(&enc_zebra)?;
    let zebra_s: String = String::from_utf8(zebra_v)?;
    let zebra: Vec<&str> = zebra_s.split(ZEBRA_DELIM).collect();
    println!("Zebra splitted = {:?}", zebra);
    if zebra.len() != 5 {
        return Err(From::from("invalid zebra format"))
    }

    // CHECK COUNTER
    let _counter = &zebra[1];
    // XXX


    // NO ZEBRA TZOFFSET CHECK


    // CHECK HASHCASH
    let hashcash = hex::decode(&zebra[0])?;
    if count_leading_zero_bits(&hashcash) < CONTRACT {
        return Err(From::from("contract not fulfilled"));
    }


    // CHECK BROWSER SIG
    let _browser_sig = &zebra[3];
    // XXX


    // CHECK TOKEN
    let mut token_enc = base64::decode(&zebra[4])?;

    let iv: &[u8] = &token_enc[0..16];
    let mut cipher = AesOfb::new_var(&KEY, iv).expect("AES OFB cipher creation failed");
    cipher.apply_keystream(&mut token_enc[16..]);
    let token: Vec<u8> = token_enc[16..].to_vec();

    let token_s = String::from_utf8(token)?;
    let token_v: Vec<&str> = token_s.split(DELIM).collect();
    println!("Token splitted = {:?}", token_v);
    if token_v.len() != 2 {
        return Err(From::from("invalid token format"))
    }

    // check token timestamp
    let timestamp = token_v[0].parse()?;
    let now = SystemTime::now();
    let now_s = now.duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;

    if (now_s < timestamp)  || (now_s - timestamp) > 33 {
        return Err(From::from("bad token timestamp"))
    };
    // check ua
    let token_ua = token_v[1];
    if token_ua != ua {
        return Err(From::from("bad token user agent"))
    }

    let mut ua_v = ua.into_bytes();

    let mut iv = vec![0; 16 ];
    getrandom::getrandom(&mut iv).expect("Failed to get random bytes from system");
    let mut cipher = AesOfb::new_var(&KEY, &iv).expect("AES OFB cipher creation failed");
    cipher.apply_keystream(&mut ua_v);

    iv.extend(&ua_v);
    let b64 = base64::encode(&iv);

    Ok(b64)
}

fn parse_rbzid(_: &Lua, (rbzid, ua): (String, String)) -> LuaResult<bool> {

    let mut rbzid_enc = match base64::decode(&rbzid) {
        Ok(s) => s,
        Err(e) => { println!("Error occured: {}", e); return Ok(false) },
    };

    let iv: &[u8] = &rbzid_enc[0..16];
    let mut cipher = AesOfb::new_var(&KEY, iv).expect("AES OFB cipher creation failed");
    cipher.apply_keystream(&mut rbzid_enc[16..]);
    let rbzid_dec: Vec<u8> = rbzid_enc[16..].to_vec();

    let rbzid_s = match String::from_utf8(rbzid_dec) {
        Ok(s) => s,
        Err(e) => { println!("Error occured: {}", e); return Ok(false) },
    };

    Ok(ua == rbzid_s)
}



#[lua_module]
fn grasshopper(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("hello", lua.create_function(hello)?)?;
    exports.set("js_app", lua.create_function(js_app)?)?;
    exports.set("js_bio", lua.create_function(js_bio)?)?;
    exports.set("gen_new_seed", lua.create_function(gen_new_seed)?)?;
    exports.set("verify_workproof", lua.create_function(verify_workproof)?)?;
    exports.set("parse_rbzid", lua.create_function(parse_rbzid)?)?;
    Ok(exports)
}
