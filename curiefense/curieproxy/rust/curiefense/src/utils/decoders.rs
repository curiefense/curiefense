use crate::config::utils::{DataSource, XDataSource};
use crate::requestfields::RequestField;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_while, take_while_m_n};
use nom::character::complete::anychar;
use nom::character::complete::char;
use nom::combinator::iterator;
use nom::combinator::map_opt;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::success;
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub enum DecodingResult<T> {
    NoChange,
    Changed(T),
}

#[inline]
fn from_hex_digit(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'A'..=b'F' => Some(digit - b'A' + 10),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        _ => None,
    }
}

#[inline]
fn is_hex_char(digit: char) -> bool {
    matches!(digit, '0'..='9' | 'A'..='F' | 'a'..='f')
}

/// decodes an url encoded string into a binary vector
fn urldecode(input: &str) -> DecodingResult<Vec<u8>> {
    urldecode_bytes(input.as_bytes())
}

fn urldecode_bytes(input: &[u8]) -> DecodingResult<Vec<u8>> {
    // scan for the first '+' or '%' ... there is no find method for &[u8] ?
    fn find_start(i: &[u8]) -> Option<usize> {
        for (idx, c) in i.iter().enumerate() {
            if *c == b'+' || *c == b'%' {
                return Some(idx);
            }
        }
        None
    }
    let (prefix, input) = match find_start(input) {
        None => return DecodingResult::NoChange,
        Some(p) => (&input[0..p], &input[p..]),
    };

    let mut changed = false;
    let mut out = prefix.to_vec();
    let mut bytes = input.iter().copied();
    while let Some(mut b) = bytes.next() {
        loop {
            if b == b'+' {
                changed = true;
                out.push(32);
                break;
            } else if b == b'%' {
                if let Some(h) = bytes.next() {
                    if let Some(hv) = from_hex_digit(h) {
                        if let Some(l) = bytes.next() {
                            if let Some(lv) = from_hex_digit(l) {
                                changed = true;
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
    if changed {
        DecodingResult::Changed(out)
    } else {
        DecodingResult::NoChange
    }
}

fn base64dec_all(input: &str) -> Result<Vec<u8>, &str> {
    const BAD_PADDING_MESSAGE: &str = "bad padding";
    if input.len() % 4 == 1 {
        return Err(BAD_PADDING_MESSAGE);
    }
    let mut i = 4;
    let mut v: u32 = 0;
    let mut res: Vec<u8> = Vec::default();
    let mut pad = 0;
    for c in input.chars() {
        let n = match c {
            '0'..='9' => 52 + (c as u8) - b'0',
            'A'..='Z' => (c as u8) - b'A',
            'a'..='z' => 26 + (c as u8) - b'a',
            '+' | '-' => 62,
            '/' | '_' => 63,
            '=' => {
                if (pad >= 2) || (i >= 3) {
                    return Err("bad padding");
                }
                pad += 1;
                0
            }
            _ => return Err("invalid baase64 character"),
        } as u32;
        v <<= 6;
        v |= n;
        i -= 1;
        if i == 0 {
            res.push((v >> 16) as u8);
            if pad < 2 {
                res.push((v >> 8) as u8);
            }
            if pad < 1 {
                res.push(v as u8);
            }
            i = 4;
            v = 0;
        }
    }
    if (i == 3) || ((pad > 0) && (i != 4)) {
        return Err(BAD_PADDING_MESSAGE);
    }
    if i == 1 {
        res.push((v >> 10) as u8);
        res.push((v >> 2) as u8);
    } else if i == 2 {
        res.push((v >> 4) as u8);
    }
    Ok(res)
}

/// decodes an url encoded string into a string, which can contain REPLACEMENT CHARACTER on decoding failure
pub fn base64dec_all_str(input: &str) -> Result<String, &str> {
    match base64dec_all(input) {
        Ok(d) => Ok(String::from_utf8_lossy(&d).into_owned()),
        Err(e) => Err(e),
    }
}

/// decodes an url encoded string into a string, which can contain REPLACEMENT CHARACTER on decoding failure
/// no changes if the source string did not contain '+' or '%'
pub fn urldecode_str(input: &str) -> DecodingResult<String> {
    match urldecode(input) {
        DecodingResult::NoChange => DecodingResult::NoChange,
        DecodingResult::Changed(r) => DecodingResult::Changed(String::from_utf8_lossy(&r).into_owned()),
    }
}

/// same as urldecode_str, but defaults to the input string when no change happeneds
fn urldecode_str_def(input: &str) -> String {
    match urldecode_str(input) {
        DecodingResult::Changed(s) => s,
        DecodingResult::NoChange => input.to_string(),
    }
}

/// parses query parameters, that look like a=b&c=d
pub fn parse_urlencoded_params(args: &mut RequestField, query: &str) {
    for kv in query.split('&') {
        let (k, v) = match kv.splitn(2, '=').collect_tuple() {
            Some((k, v)) => (urldecode_str_def(k), urldecode_str_def(v)),
            None => (urldecode_str_def(kv), String::new()),
        };
        args.add(k, DataSource::X(XDataSource::Uri), v);
    }
}

fn urldecode_bytes_str(input: &[u8]) -> String {
    match urldecode_bytes(input) {
        DecodingResult::NoChange => String::from_utf8_lossy(input).into_owned(),
        DecodingResult::Changed(r) => String::from_utf8_lossy(&r).into_owned(),
    }
}

/// parses query parameters, that look like a=b&c=d
pub fn parse_urlencoded_params_bytes(args: &mut RequestField, query: &[u8]) {
    for kv in query.split(|x| *x == b'&') {
        let (k, v) = match kv.splitn(2, |x| *x == b'=').collect_tuple() {
            Some((k, v)) => (urldecode_bytes_str(k), urldecode_bytes_str(v)),
            None => (urldecode_bytes_str(kv), String::new()),
        };
        args.add(k, DataSource::X(XDataSource::Uri), v);
    }
}

fn from_hex(input: &str) -> Result<char, &'static str> {
    let a = u32::from_str_radix(input, 16).map_err(|_| "invalid hex digit")?;
    char::from_u32(a).ok_or("invalid char")
}

fn from_decimal(input: &str) -> Result<char, &'static str> {
    let a = input.parse::<u32>().map_err(|_| "invalid hex digit")?;
    char::from_u32(a).ok_or("invalid char")
}

/// parses \uXXXX, \XXXX and \UXXXXXXXX
pub fn parse_unicode(input: &str) -> DecodingResult<String> {
    // scan for the first '\\'
    let p = input.find('\\');
    let (prefix, input) = match p {
        None => return DecodingResult::NoChange,
        Some(p) => (&input[0..p], &input[p..]),
    };
    let mut changed = false;

    fn parse_lower(i: &str) -> IResult<&str, u32> {
        let (i, _) = tag("\\u")(i)?;
        map_res(take_while_m_n(4, 4, is_hex_char), |x| u32::from_str_radix(x, 16))(i)
    }
    // https://www.w3.org/TR/CSS2/syndata.html#characters
    fn parse_css(i: &str) -> IResult<&str, u32> {
        let (i, _) = tag("\\")(i)?;
        let (i, o) = map_res(take_while(is_hex_char), |x| u32::from_str_radix(x, 16))(i)?;
        let (i, _) = opt(char(' '))(i)?;
        Ok((i, o))
    }
    fn parse_upper<'t>(c: &mut bool, i: &'t str) -> IResult<&'t str, char> {
        let (i, _) = tag("\\U")(i)?;
        let (i, r) = map_res(take_while_m_n(8, 8, is_hex_char), from_hex)(i)?;
        *c = true;
        Ok((i, r))
    }

    // can parse codepoints > 0x10000, combining surrogate pairs
    fn utf16<'t>(c: &mut bool, i: &'t str) -> IResult<&'t str, char> {
        let (i, n1) = alt((parse_lower, parse_css))(i)?;
        let (i, r) = if (0xd800..=0xdfff).contains(&n1) {
            let (i, n2) = alt((parse_lower, parse_css))(i)?;
            let x = 0x10000 + (n1 - 0xd800) * 0x400 + (n2 - 0xdc00);
            map_opt(success(x), char::from_u32)(i)
        } else {
            map_opt(success(n1), char::from_u32)(i)
        }?;
        *c = true;
        Ok((i, r))
    }
    fn parseone<'t>(c: &mut bool, i: &'t str) -> IResult<&'t str, char> {
        let mut c1 = false;
        let mut c2 = false;
        let r = alt((|x| parse_upper(&mut c1, x), |x| utf16(&mut c2, x), anychar))(i);
        *c |= c1 | c2;
        r
    }

    let mut it = iterator(input, |x| parseone(&mut changed, x));
    let mut out = prefix.to_string();

    for x in &mut it {
        out.push(x);
    }
    let _ = it.finish();

    if changed {
        DecodingResult::Changed(out)
    } else {
        DecodingResult::NoChange
    }
    // this parser can't fail, errors are not checked
}

pub fn htmlentities(input: &str) -> DecodingResult<String> {
    // scan for the first '&'
    let p = input.find('&');
    let (prefix, input) = match p {
        None => return DecodingResult::NoChange,
        Some(p) => (&input[0..p], &input[p..]),
    };

    let mut changed = false;
    fn get_entity(i: &str) -> Option<char> {
        match i.to_lowercase().as_str() {
            "quot" => Some('"'),
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "nbsp" => Some(' '), // this is not a space, this is ua0
            _ => None,
        }
    }
    fn hex_entity(i: &str) -> IResult<&str, char> {
        let (i, _) = is_a("xX")(i)?;
        map_res(take_while(is_hex_char), from_hex)(i)
    }
    fn decimal_entity(i: &str) -> IResult<&str, char> {
        map_res(take_while(|c| ('0'..='9').contains(&c)), from_decimal)(i)
    }
    fn num_entity(i: &str) -> IResult<&str, char> {
        let (i, _) = char('#')(i)?;
        alt((hex_entity, decimal_entity))(i)
    }
    fn named_entity(i: &str) -> IResult<&str, char> {
        map_opt(take_while(|c| c != ';'), get_entity)(i)
    }

    fn parse_char<'t>(c: &mut bool, i: &'t str) -> IResult<&'t str, char> {
        let (i, _) = char('&')(i)?;
        let (i, entity) = alt((num_entity, named_entity))(i)?;
        let (i, _) = char(';')(i)?;
        *c = true;
        Ok((i, entity))
    }

    let mut it = iterator(input, alt((|x| parse_char(&mut changed, x), anychar)));
    let mut out = prefix.to_string();

    for x in &mut it {
        out.push(x);
    }
    let _ = it.finish();

    if changed {
        DecodingResult::Changed(out)
    } else {
        DecodingResult::NoChange
    }
}

#[cfg(test)]
mod test_lib {
    use super::*;

    #[test]
    fn test_urldecode_normal() {
        assert!(urldecode_str("ABCD") == DecodingResult::NoChange);
        assert!(urldecode_str("ABCD%40") == DecodingResult::Changed("ABCD@".to_string()));
        assert!(urldecode_str("ABCD%40EFG") == DecodingResult::Changed("ABCD@EFG".to_string()));
        assert!(urldecode_str("%27%28%29%2a%2b%2C%2D%2e%2F") == DecodingResult::Changed("'()*+,-./".to_string()));
        assert!(urldecode_str("ABCD+EFG") == DecodingResult::Changed("ABCD EFG".to_string()));
        assert!(
            urldecode_str("http://www.example.com/foo/bar?x=AB%20CD%3d~~F%7C%60G")
                == DecodingResult::Changed("http://www.example.com/foo/bar?x=AB CD=~~F|`G".to_string())
        );
    }

    #[test]
    fn test_urldecode_bytes() {
        let cases: &[(&[u8], &[u8])] = &[
            (b"ABCD", b"ABCD"),
            (b"ABCD%40", b"ABCD@"),
            (b"%40ABCD%40%%4", b"@ABCD@%%4"),
            (b"%4%4%4", b"%4%4%4"),
        ];
        for (input, output) in cases.iter() {
            let r = urldecode_bytes(*input);
            if input == output {
                assert_eq!(r, DecodingResult::NoChange);
            } else {
                assert_eq!(r, DecodingResult::Changed(output.to_vec()));
            }
        }
    }

    #[test]
    fn test_urldecode_utf8() {
        assert!(urldecode_str_def("%F0%9F%91%BE%20Exterminate%21") == "👾 Exterminate!");
    }

    #[test]
    fn test_urldecode_incorrect() {
        assert!(urldecode_str_def("%") == "%");
        assert!(urldecode_str_def("%a") == "%a");
        assert!(urldecode_str_def("%p1") == "%p1");
        assert!(urldecode_str_def("%ap") == "%ap");
        assert!(urldecode_str_def("%%41") == "%A");
        assert!(urldecode_str_def("%a%41") == "%aA");
        assert!(urldecode_str_def("%F0%9F%91%BE%20Exterminate%21%") == "👾 Exterminate!%");
        assert!(urldecode_str_def("%F0%9F%BE%20%21%") == "� !%");
    }

    #[test]
    fn test_ok_base64dec_all_str() {
        for (input, output) in [
            ("", ""),
            ("Zm9v", "foo"),
            ("QQ==", "A"),
            ("QQ", "A"),
            ("QUI=", "AB"),
            ("QUI", "AB"),
            ("QUE+", "AA>"),
            ("QUE-", "AA>"),
            ("QUE/", "AA?"),
            ("QUE_", "AA?"),
            (
                "bTxLTFNqZGhmPkxLSkhGemVsSWpmZXpsbsOpw6Bkw6vDhGtmZQ==",
                "m<KLSjdhf>LKJHFzelIjfezlnéàdëÄkfe",
            ),
        ]
        .iter()
        {
            println!(
                "base64 dec {:?} => {:?} / expect {:?}",
                input,
                base64dec_all_str(input),
                output
            );
            assert!(base64dec_all_str(input) == Ok(output.to_string()));
        }
    }
    #[test]
    fn test_err_base64dec_all_str() {
        for (input, output) in [
            ("A", "bad padding"),
            ("A==", "bad padding"),
            ("ABC==", "bad padding"),
            ("ABCD==", "bad padding"),
            ("ABCDE==", "bad padding"),
            ("QUIDA", "bad padding"),
            ("QUIDE=", "bad padding"),
            ("QUIDE=A", "bad padding"),
            ("QUID===", "bad padding"),
        ]
        .iter()
        {
            assert!(base64dec_all_str(input) == Err(output));
        }
    }

    #[test]
    fn tests_unicode() {
        for (input, output) in [
            ("nothing", "nothing"),
            ("<DIV STYLE=\"background-image:\\0075\\0072\\006C\\0028'\\006a\\0061\\0076\\0061\\0073\\0063\\0072\\0069\\0070", "<DIV STYLE=\"background-image:url('javascrip"),
            ("\\0020\\u0020\\U00000020", "   "),
            ("\\uD83D\\uDC36", "🐶"),
            ("\\D83D\\uDC36", "🐶"),
            ("\\D83D\\DC36", "🐶"),
            ("\\uD83D\\uDC69\\u200D\\u2764\\uFE0F\\u200D\\uD83D\\uDC69", "👩‍❤️‍👩"),
            ("\\U0001f469\\u200D\\u2764\\u200D\\U0001f469", "\u{1F469}\u{200D}\u{2764}\u{200D}\u{1F469}"),
            ("\\01F469 \\200D \\2764 \\200D \\01F469", "\u{1F469}\u{200D}\u{2764}\u{200D}\u{1F469}"),
        ]
        .iter()
        {
            let r = parse_unicode(input);
            if input == output {
                assert_eq!(r, DecodingResult::NoChange);
            } else {
                assert_eq!(r, DecodingResult::Changed(output.to_string()));
            }
        }
    }

    #[test]
    fn test_entities() {
        for (input, output) in [
            ("nothing", "nothing"),
            ("&gt;", ">"),
            ("&#32;", " "),
            ("&#x20;", " "),
            ("&#x20", "&#x20"),
            ("&gt;&gt&gt;", ">&gt>"),
            ("&#x0000000020;", " "),
            ("&#x000000000020;", " "),
            // overflow
            ("&#x010000000020;", "&#x010000000020;"),
            ("&#128512;", "😀"),
            (
                "&#128105;&#8205;&#10084;&#8205;&#128105;",
                "\u{1F469}\u{200D}\u{2764}\u{200D}\u{1F469}",
            ),
        ]
        .iter()
        {
            let r = htmlentities(input);
            if input == output {
                assert_eq!(r, DecodingResult::NoChange);
            } else {
                assert_eq!(r, DecodingResult::Changed(output.to_string()));
            }
        }
    }
}
