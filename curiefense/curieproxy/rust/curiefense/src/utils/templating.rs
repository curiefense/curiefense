use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1, take_while, take_while1},
    combinator::{all_consuming, map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::config::matchers::RequestSelector;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TVar {
    Selector(RequestSelector),
    Tag(String), // match for a specific tag
}

#[derive(Debug, PartialEq, Eq)]
pub enum TemplatePartT<'t, A> {
    Raw(&'t str),
    Var(A),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TemplatePart<A> {
    Raw(String),
    Var(A),
}

impl<'t, A> TemplatePartT<'t, A> {
    pub fn owned(self) -> TemplatePart<A> {
        match self {
            TemplatePartT::Raw(s) => TemplatePart::Raw(s.to_string()),
            TemplatePartT::Var(v) => TemplatePart::Var(v),
        }
    }
}

fn variable<F, A>(sub: F, input: &str) -> IResult<&str, A>
where
    F: Fn(&str) -> IResult<&str, A>,
{
    delimited(tag("${"), sub, tag("}"))(input)
}

fn nonvariable(input: &str) -> IResult<&str, &str> {
    recognize(pair(take(1usize), take_while(|c| c != '$' && c != '\\')))(input)
}

fn escaped(input: &str) -> IResult<&str, &str> {
    preceded(tag("\\"), take(1usize))(input)
}

fn templates<F, A>(sub: F, input: &str) -> IResult<&str, Vec<TemplatePartT<'_, A>>>
where
    F: Fn(&str) -> IResult<&str, A>,
{
    all_consuming(many0(alt((
        map(escaped, TemplatePartT::Raw),
        map(|i| variable(&sub, i), TemplatePartT::Var),
        map(nonvariable, TemplatePartT::Raw),
    ))))(input)
}

pub fn parse_template<F, A>(sub: F, i: &str) -> Vec<TemplatePartT<A>>
where
    F: Fn(&str) -> IResult<&str, A>,
{
    match templates(sub, i) {
        Ok((_, r)) => r,
        _ => vec![TemplatePartT::Raw(i)],
    }
}

fn parse_tvar(input: &str) -> IResult<&str, TVar> {
    let (input, selp1) = take_while1(|c| ('a'..='z').contains(&c))(input)?;
    let (input, oselp2) = opt(preceded(tag("."), take_till1(|c| c == '}')))(input)?;
    match (selp1, oselp2) {
        (_, None) => {
            if let Some(rs) = RequestSelector::decode_attribute(selp1) {
                Ok((input, TVar::Selector(rs)))
            } else {
                nom::combinator::fail(input)
            }
        }
        ("tags", Some(tagname)) => Ok((input, TVar::Tag(tagname.to_string()))),
        (_, Some(selp2)) => match RequestSelector::resolve_selector_raw(selp1, selp2) {
            Err(_) => nom::combinator::fail(input),
            Ok(t) => Ok((input, TVar::Selector(t))),
        },
    }
}

pub type RequestTemplate = Vec<TemplatePart<TVar>>;

pub fn parse_request_template(i: &str) -> RequestTemplate {
    parse_template(parse_tvar, i).into_iter().map(|p| p.owned()).collect()
}

#[cfg(test)]
mod test {
    use nom::bytes::complete::take_till1;

    use super::*;

    fn parse_simple(i: &str) -> Vec<TemplatePartT<String>> {
        parse_template(|x| map(take_till1(|c| c == '}'), |s: &str| s.to_string())(x), i)
    }

    #[test]
    fn empty() {
        assert_eq!(parse_simple(""), Vec::new())
    }

    #[test]
    fn no_variables() {
        assert_eq!(parse_simple("test"), vec![TemplatePartT::Raw("test")])
    }

    #[test]
    fn mixed() {
        use TemplatePartT::*;
        assert_eq!(
            parse_simple("test${a} test2${b.c.d}"),
            vec![
                Raw("test"),
                Var("a".to_string()),
                Raw(" test2"),
                Var("b.c.d".to_string())
            ]
        )
    }

    #[test]
    fn escaped() {
        use TemplatePartT::*;
        assert_eq!(
            parse_simple("test\\${a} test2${b.c.d}"),
            vec![Raw("test"), Raw("$"), Raw("{a} test2"), Var("b.c.d".to_string())]
        )
    }

    #[test]
    fn last_escape() {
        use TemplatePartT::*;
        assert_eq!(
            parse_simple("${a}xxx\\"),
            vec![Var("a".to_string()), Raw("xxx"), Raw("\\")]
        )
    }

    #[test]
    fn selector_a() {
        use TVar::*;
        use TemplatePart::*;
        assert_eq!(
            parse_request_template("${ip} - ${headers.foo-bar}"),
            vec![
                Var(Selector(RequestSelector::Ip)),
                Raw(" - ".to_string()),
                Var(Selector(RequestSelector::Header("foo-bar".to_string())))
            ]
        )
    }
}
