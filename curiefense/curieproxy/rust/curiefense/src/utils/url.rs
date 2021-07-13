use crate::requestfields::RequestField;
use itertools::Itertools;

#[inline]
fn from_hex_digit(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'A'..=b'F' => Some(digit - b'A' + 10),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        _ => None,
    }
}

/// decodes an url encoded string into a binary vector
fn urldecode(input: &str) -> Vec<u8> {
    urldecode_bytes(input.as_bytes())
}

fn urldecode_bytes(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut bytes = input.iter().copied();
    while let Some(mut b) = bytes.next() {
        loop {
            if b == b'%' {
                if let Some(h) = bytes.next() {
                    if let Some(hv) = from_hex_digit(h) {
                        if let Some(l) = bytes.next() {
                            if let Some(lv) = from_hex_digit(l) {
                                out.push(hv * 16 + lv);
                                break;
                            } else {
                                out.push(b);
                                out.push(h);
                                b = l;
                            }
                        } else {
                            out.push(b);
                            out.push(h);
                            break;
                        }
                    } else {
                        out.push(b);
                        b = h;
                    }
                } else {
                    out.push(b);
                    break;
                }
            } else {
                out.push(b);
                break;
            }
        }
    }
    out
}

/// decodes an url encoded string into a string, which can contain REPLACEMENT CHARACTER on decoding failure
pub fn urldecode_str(input: &str) -> String {
    String::from_utf8_lossy(&urldecode(input)).into_owned()
}

/// parses query parameters, that look like a=b&c=d
pub fn parse_urlencoded_params(args: &mut RequestField, query: &str) {
    for kv in query.split('&') {
        let (k, v) = match kv.splitn(2, '=').collect_tuple() {
            Some((k, v)) => (urldecode_str(k), urldecode_str(v)),
            None => (urldecode_str(kv), String::new()),
        };
        args.add(k, v);
    }
}

fn urldecode_bytes_str(input: &[u8]) -> String {
    String::from_utf8_lossy(&urldecode_bytes(input)).into_owned()
}

/// parses query parameters, that look like a=b&c=d
pub fn parse_urlencoded_params_bytes(args: &mut RequestField, query: &[u8]) {
    for kv in query.split(|x| *x == b'&') {
        let (k, v) = match kv.splitn(2, |x| *x == b'=').collect_tuple() {
            Some((k, v)) => (urldecode_bytes_str(k), urldecode_bytes_str(v)),
            None => (urldecode_bytes_str(kv), String::new()),
        };
        args.add(k, v);
    }
}

#[cfg(test)]
mod test_lib {
    use super::urldecode_str;

    #[test]
    fn test_urldecode_normal() {
        assert!(urldecode_str("ABCD") == "ABCD");
        assert!(urldecode_str("ABCD%40") == "ABCD@");
        assert!(urldecode_str("ABCD%40EFG") == "ABCD@EFG");
        assert!(urldecode_str("%27%28%29%2a%2b%2C%2D%2e%2F") == "'()*+,-./");
        assert!(urldecode_str("ABCD+EFG") == "ABCD+EFG");
        assert!(
            urldecode_str("http://www.example.com/foo/bar?x=AB%20CD%3d~~F%7C%60G")
                == "http://www.example.com/foo/bar?x=AB CD=~~F|`G"
        );
    }

    #[test]
    fn test_urldecode_utf8() {
        assert!(urldecode_str("%F0%9F%91%BE%20Exterminate%21") == "👾 Exterminate!");
    }

    #[test]
    fn test_urldecode_incorrect() {
        assert!(urldecode_str("%") == "%");
        assert!(urldecode_str("%a") == "%a");
        assert!(urldecode_str("%p1") == "%p1");
        assert!(urldecode_str("%ap") == "%ap");
        assert!(urldecode_str("%%41") == "%A");
        assert!(urldecode_str("%a%41") == "%aA");
        assert!(urldecode_str("%F0%9F%91%BE%20Exterminate%21%") == "👾 Exterminate!%");
        assert!(urldecode_str("%F0%9F%BE%20%21%") == "� !%");
    }
}
