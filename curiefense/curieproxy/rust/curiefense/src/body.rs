/// body parsing functions
///
/// This module contains body parsing for the following mime types:
///
///  * json
///  * xml
///  * multipart/form-data
///  * urlencoded forms
///
/// The main function, parse_body, is the only exported function.
///
use multipart::server::Multipart;
use serde_json::Value;
use std::io::Read;
use xmlparser::{ElementEnd, EntityDefinition, ExternalId, Token};

use crate::logs::Logs;
use crate::requestfields::RequestField;
use crate::utils::url::parse_urlencoded_params_bytes;

fn json_path(prefix: &[String]) -> String {
    if prefix.is_empty() {
        "JSON_ROOT".to_string()
    } else {
        prefix.join("_")
    }
}

/// flatten a JSON tree into the RequestField key/value store
/// key values are build by joining all path names with "_", where path names are:
///   * keys for objects ;
///   * indices for lists.
///
/// Scalar values are converted to string, with lowercase booleans and null values.
fn flatten_json(args: &mut RequestField, prefix: &mut Vec<String>, value: Value) {
    match value {
        Value::Array(array) => {
            prefix.push(String::new());
            let idx = prefix.len() - 1;
            for (i, v) in array.into_iter().enumerate() {
                prefix[idx] = format!("{}", i);
                flatten_json(args, prefix, v);
            }
            prefix.pop();
        }
        Value::Object(mp) => {
            prefix.push(String::new());
            let idx = prefix.len() - 1;
            for (k, v) in mp.into_iter() {
                prefix[idx] = k;
                flatten_json(args, prefix, v);
            }
            prefix.pop();
        }
        Value::String(str) => {
            args.add(json_path(prefix), str);
        }
        Value::Bool(b) => {
            args.add(json_path(prefix), (if b { "true" } else { "false" }).to_string());
        }
        Value::Number(n) => {
            args.add(json_path(prefix), format!("{}", n));
        }
        Value::Null => {
            args.add(json_path(prefix), "null".to_string());
        }
    }
}

/// This should work with a stream of json items, not deserialize all at once
///
/// I tried qjsonrs, but it was approximatively 10x slower for small maps (but faster with larger maps)
/// qjronsrs -> serde_json benches:
///  * map/1 -> -98.83%
///  * map/100 -> -43.516%
///  * map/10000 -> +33.534%
///
/// next idea: adapting https://github.com/Geal/nom/blob/master/examples/json_iterator.rs
fn json_body(args: &mut RequestField, body: &[u8]) -> Result<(), String> {
    let value: Value = serde_json::from_slice(body).map_err(|rr| format!("Invalid JSON body: {}", rr))?;

    let mut prefix = Vec::new();
    flatten_json(args, &mut prefix, value);
    Ok(())
}

/// builds the XML path for a given stack, by appending key names with their indices
fn xml_path(stack: &[(String, u64)]) -> String {
    let mut out = String::new();
    for (s, i) in stack {
        out += s;
        // if i == 0, this means we are working with xml attributes
        if *i > 0 {
            out.extend(format!("{}", i).chars());
        }
    }
    out
}

/// pop the stack and checks for errors when closing an element
fn close_xml_element(
    args: &mut RequestField,
    stack: &mut Vec<(String, u64)>,
    close_name: Option<&str>,
) -> Result<(), String> {
    match stack.pop() {
        None => {
            return Err(format!("Invalid XML, extraneous element end: {:?}", close_name));
        }
        Some((openname, idx)) => {
            if let Some(local) = close_name {
                if openname != local {
                    return Err(format!(
                        "Invalid XML, wrong closing element. Expected: {}, got {}",
                        openname, local
                    ));
                }
            }
            if idx == 0 {
                // empty XML element, save it with an empty string
                let path = xml_path(&stack) + openname.as_str() + "1";
                args.add(path, String::new());
            }
            Ok(())
        }
    }
}

fn xml_increment_last(stack: &mut Vec<(String, u64)>) -> u64 {
    if let Some(curtop) = stack.last_mut() {
        let prev = curtop.1;
        curtop.1 = prev + 1;
        return prev;
    }
    0
}

/// Parses the XML body by iterating on the token stream
///
/// This checks the following errors, in addition to the what the lexer gets:
///   * mismatched opening and closing tags
///   * premature end of document
fn xml_body(args: &mut RequestField, body: &[u8]) -> Result<(), String> {
    let body_utf8 = String::from_utf8_lossy(body);
    let mut stack: Vec<(String, u64)> = Vec::new();
    for rtoken in xmlparser::Tokenizer::from(body_utf8.as_ref()) {
        let token = rtoken.map_err(|rr| format!("XML parsing error: {}", rr))?;
        match token {
            Token::ProcessingInstruction { .. } => (),
            Token::Comment { .. } => (),
            Token::Declaration { .. } => (),
            Token::DtdStart { .. } => (),
            Token::DtdEnd { .. } => (),
            Token::EmptyDtd { .. } => (),
            Token::EntityDeclaration { name, definition, .. } => match definition {
                EntityDefinition::EntityValue(span) => {
                    args.add("_XMLENTITY_VALUE_".to_string() + name.as_str(), span.to_string())
                }
                EntityDefinition::ExternalId(ExternalId::System(span)) => {
                    args.add("_XMLENTITY_SYSTEMID_".to_string() + name.as_str(), span.to_string())
                }
                EntityDefinition::ExternalId(ExternalId::Public(p1, p2)) => args.add(
                    "_XMLENTITY_PUBLICID_".to_string() + name.as_str(),
                    p1.to_string() + "/" + p2.as_str(),
                ),
            },
            Token::ElementStart { local, .. } => {
                // increment element index for the current element
                xml_increment_last(&mut stack);
                // and push the new element
                stack.push((local.to_string(), 0))
            }
            Token::ElementEnd { end, .. } => match end {
                //  <foo/>
                ElementEnd::Empty => close_xml_element(args, &mut stack, None)?,
                //  <foo>
                ElementEnd::Open => (),
                //  </foo>
                ElementEnd::Close(_, local) => close_xml_element(args, &mut stack, Some(local.as_str()))?,
            },
            Token::Attribute { local, value, .. } => {
                let path = xml_path(&stack) + local.as_str();
                args.add(path, value.to_string());
            }
            Token::Text { text } => {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    xml_increment_last(&mut stack);
                    args.add(xml_path(&stack), trimmed.to_string());
                }
            }
            Token::Cdata { text, .. } => {
                xml_increment_last(&mut stack);
                args.add(xml_path(&stack), text.to_string());
            }
        }
    }
    if stack.is_empty() {
        Ok(())
    } else {
        Err("XML error: premature end of document".to_string())
    }
}

/// parses bodies that are url encoded forms, like query params
fn forms_body(args: &mut RequestField, body: &[u8]) -> Result<(), String> {
    // TODO: body is traversed twice here, this is inefficient
    if body.contains(&b'=') && body.iter().all(|x| *x > 0x20 && *x < 0x7f) {
        parse_urlencoded_params_bytes(args, body);
        Ok(())
    } else {
        Err("Body is not forms encoded".to_string())
    }
}

/// reuses the multipart crate to parse these bodies
///
/// will not work properly with binary data
fn multipart_form_encoded(boundary: &str, args: &mut RequestField, body: &[u8]) -> Result<(), String> {
    let mut multipart = Multipart::with_body(body, boundary);
    multipart
        .foreach_entry(|mut entry| {
            let mut content = Vec::new();
            let _ = entry.data.read_to_end(&mut content);
            let name = entry.headers.name.to_string();
            let scontent = String::from_utf8_lossy(&content);
            args.add(name, scontent.to_string());
        })
        .map_err(|rr| format!("Could not parse multipart body: {}", rr))
}

/// body parsing function
///
/// fails if the
pub fn parse_body(
    logs: &mut Logs,
    args: &mut RequestField,
    mcontent_type: Option<&str>,
    body: &[u8],
) -> Result<(), String> {
    logs.debug("body parsing started");

    if let Some(content_type) = mcontent_type {
        logs.debug(format!("parsing content type: {}", content_type));
        if let Some(boundary) = content_type.strip_prefix("multipart/form-data; boundary=") {
            return multipart_form_encoded(boundary, args, body);
        }

        if content_type.ends_with("/json") {
            return json_body(args, body);
        }

        if content_type.ends_with("/xml") {
            return xml_body(args, body);
        }

        if content_type == "application/x-www-form-urlencoded" {
            return forms_body(args, body);
        }
    }

    // unhandled content type, default to json and forms_body
    json_body(args, body).or_else(|_| forms_body(args, body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logs::LogLevel;

    fn test_parse_ok(mcontent_type: Option<&str>, body: &[u8]) -> RequestField {
        let mut logs = Logs::default();
        let mut args = RequestField::default();
        parse_body(&mut logs, &mut args, mcontent_type, body).unwrap();
        for lg in logs.logs {
            if lg.level > LogLevel::Debug {
                panic!("unexpected log: {:?}", lg);
            }
        }
        args
    }

    fn test_parse_bad(mcontent_type: Option<&str>, body: &[u8]) {
        let mut logs = Logs::default();
        let mut args = RequestField::default();
        assert!(parse_body(&mut logs, &mut args, mcontent_type, body).is_err());
    }

    fn test_parse(mcontent_type: Option<&str>, body: &[u8], expected: &[(&str, &str)]) {
        let args = test_parse_ok(mcontent_type, body);
        for (k, v) in expected {
            match args.get_str(k) {
                None => panic!("Argument not set {}", k),
                Some(actual) => assert_eq!(actual, *v),
            }
        }
        if args.len() != expected.len() {
            for (k, v) in args.iter() {
                if !expected.iter().any(|(ek, _)| ek == k) {
                    println!("Spurious argument {}: {}", k, v);
                }
            }
            panic!("Spurious arguments");
        }
    }

    #[test]
    fn json_empty_body() {
        test_parse(Some("application/json"), br#"{}"#, &[]);
    }

    #[test]
    fn json_scalar() {
        test_parse(Some("application/json"), br#""scalar""#, &[("JSON_ROOT", "scalar")]);
    }

    #[test]
    fn json_simple_object() {
        test_parse(
            Some("application/json"),
            br#"{"a": "b", "c": "d"}"#,
            &[("a", "b"), ("c", "d")],
        );
    }

    #[test]
    fn json_bad() {
        test_parse_bad(Some("application/json"), br#"{"a": "b""#);
    }

    #[test]
    fn json_collision() {
        test_parse(
            Some("application/json"),
            br#"{"a": {"b": "1"}, "a_b": "2"}"#,
            &[("a_b", "1 2")],
        );
    }

    #[test]
    fn json_simple_array() {
        test_parse(Some("application/json"), br#"["a", "b"]"#, &[("0", "a"), ("1", "b")]);
    }

    #[test]
    fn json_nested_objects() {
        test_parse(
            Some("application/json"),
            br#"{"a": [true,null,{"z": 0.2}], "c": {"d": 12}}"#,
            &[("a_0", "true"), ("a_1", "null"), ("a_2_z", "0.2"), ("c_d", "12")],
        );
    }

    #[test]
    fn arguments_collision() {
        let mut logs = Logs::default();
        let mut args = RequestField::default();
        args.add("a".to_string(), "query_arg".to_string());
        parse_body(&mut logs, &mut args, Some("application/json"), br#"{"a": "body_arg"}"#).unwrap();
        assert_eq!(args.get_str("a"), Some("query_arg body_arg"));
    }

    #[test]
    fn xml_simple() {
        test_parse(Some("text/xml"), br#"<a>content</a>"#, &[("a1", "content")]);
    }

    #[test]
    fn xml_bad1() {
        test_parse_bad(Some("text/xml"), br#"<a>"#);
    }

    #[test]
    fn xml_bad2() {
        test_parse_bad(Some("text/xml"), br#"<a>x</b>"#);
    }

    #[test]
    fn xml_bad3() {
        test_parse_bad(Some("text/xml"), br#"<a 1x="12">x</a>"#);
    }

    #[test]
    fn xml_nested() {
        test_parse(
            Some("text/xml"),
            br#"<a>a<b foo="bar">xxx</b>z</a>"#,
            &[("a1", "a"), ("a3", "z"), ("a2bfoo", "bar"), ("a2b1", "xxx")],
        );
    }

    #[test]
    fn xml_cdata() {
        test_parse(
            Some("text/xml"),
            br#"<a ><![CDATA[ <script>alert("test");</script> ]]></a >"#,
            &[("a1", r#" <script>alert("test");</script> "#)],
        );
    }

    #[test]
    fn xml_nested_empty() {
        test_parse(Some("text/xml"), br#"<a><b><c></c></b></a>"#, &[("a1b1c1", "")]);
    }

    #[test]
    fn xml_nested_empty_b() {
        test_parse(
            Some("application/xml"),
            br#"<a> <b> <c> </c></b></a>"#,
            &[("a1b1c1", "")],
        );
    }

    #[test]
    fn xml_entity_a() {
        test_parse(
            Some("application/xml"),
            br#"<!DOCTYPE foo [ <!ENTITY myentity "my entity value" > ]><a>xx</a>"#,
            &[("a1", "xx"), ("_XMLENTITY_VALUE_myentity", "my entity value")],
        );
    }

    #[test]
    fn xml_entity_b() {
        test_parse(
            Some("application/xml"),
            br#"<!DOCTYPE foo [ <!ENTITY ext SYSTEM "http://website.com" > ]><a>xx</a>"#,
            &[("a1", "xx"), ("_XMLENTITY_SYSTEMID_ext", "http://website.com")],
        );
    }

    #[test]
    fn xml_spaces() {
        test_parse(
            Some("text/xml"),
            br#"<a>a <b><c> c </c>  </b>  </a>"#,
            &[("a1", "a"), ("a2b1c1", "c")],
        );
    }

    #[test]
    fn xml_space_in_attribute() {
        test_parse(
            Some("application/xml"),
            br#"<a foo1=" ab c "><foo>abc</foo></a>"#,
            &[("afoo1", " ab c "), ("a1foo1", "abc")],
        );
    }

    #[test]
    fn xml_indent() {
        test_parse(
            Some("text/xml"),
            br#"
    <a>x1
      <b>x2</b>
    </a>
    "#,
            &[("a1", "x1"), ("a2b1", "x2")],
        );
    }

    #[test]
    fn multipart() {
        let content = [
            "--------------------------28137e3917e320b3",
            "Content-Disposition: form-data; name=\"foo\"",
            "",
            "bar",
            "--------------------------28137e3917e320b3",
            "Content-Disposition: form-data; name=\"baz\"",
            "",
            "qux",
            "--------------------------28137e3917e320b3--",
            "",
        ];
        test_parse(
            Some("multipart/form-data; boundary=------------------------28137e3917e320b3"),
            content.join("\r\n").as_bytes(),
            &[("foo", "bar"), ("baz", "qux")],
        );
    }

    #[test]
    fn urlencoded() {
        test_parse(
            Some("application/x-www-form-urlencoded"),
            b"a=1&b=2&c=3",
            &[("a", "1"), ("b", "2"), ("c", "3")],
        );
    }

    #[test]
    fn urlencoded_default() {
        test_parse(None, b"a=1&b=2&c=3", &[("a", "1"), ("b", "2"), ("c", "3")]);
    }

    #[test]
    fn json_default() {
        test_parse(None, br#"{"a": "b", "c": "d"}"#, &[("a", "b"), ("c", "d")]);
    }
}
