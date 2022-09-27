use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    sequence::terminated,
    IResult,
};

/// this is actually not at all RFC complete, only works with URLs

#[derive(Debug, PartialEq, Eq)]
struct Uri<'t> {
    _scheme: &'t str,
    _userinfo: Option<&'t str>,
    _hostport: &'t str,
    remaining: &'t str,
}

fn unreserved(c: char) -> bool {
    ('a'..='z').contains(&c) || ('0'..='9').contains(&c) || c == '+' || c == '.' || c == '_' || c == '~'
}

fn sub_delims(c: char) -> bool {
    c == '!'
        || c == '$'
        || c == '&'
        || c == '\''
        || c == '('
        || c == ')'
        || c == '*'
        || c == '+'
        || c == ','
        || c == ';'
        || c == '='
}

fn userinfo(input: &str) -> IResult<&str, &str> {
    // hack, percent encoded character not properly decoded
    take_while(|c| unreserved(c) || sub_delims(c) || c == '%' || c == ':')(input)
}

fn phost(input: &str) -> IResult<&str, &str> {
    take_while(|c| c != '/')(input)
}

fn parse_uri(input: &str) -> IResult<&str, Uri<'_>> {
    let (input, scheme) = terminated(take_while(unreserved), tag("://"))(input)?;
    let (input, userinfo) = opt(terminated(userinfo, tag("@")))(input)?;
    let (input, hostport) = phost(input)?;
    Ok((
        input,
        Uri {
            _scheme: scheme,
            _userinfo: userinfo,
            _hostport: hostport,
            remaining: input,
        },
    ))
}

/// removes the scheme from an url
pub fn drop_scheme(uri: &str) -> &str {
    match parse_uri(uri) {
        Ok((_, parsed)) => parsed.remaining,
        Err(_) => uri,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check(uri: &str, scheme: &str, userinfo: Option<&str>, hostport: &str, rm: &str) {
        let expected = Uri {
            _scheme: scheme,
            _userinfo: userinfo,
            _hostport: hostport,
            remaining: rm,
        };
        match parse_uri(uri) {
            Err(rr) => panic!("could not parse {}: {:#?}", uri, rr),
            Ok((_, actual)) => assert_eq!(expected, actual),
        }
    }

    #[test]
    fn t1() {
        check(
            "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top",
            "https",
            Some("john.doe"),
            "www.example.com:123",
            "/forum/questions/?tag=networking&order=newest#top",
        );
    }

    #[test]
    fn t2() {
        check(
            "ldap://[2001:db8::7]/c=GB?objectClass?one",
            "ldap",
            None,
            "[2001:db8::7]",
            "/c=GB?objectClass?one",
        );
    }
}
