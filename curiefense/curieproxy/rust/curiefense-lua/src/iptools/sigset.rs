use regex::RegexSet;

#[derive(Debug)]
pub struct SigSet {
    regex_set: Option<regex::RegexSet>,
    regex_list: Vec<String>,
    id_list: Vec<String>,
}

#[derive(Debug)]
pub enum SigSetError {
    RegexSetNotCompiled,
    RegexSetAlreadyCompiled,
    RegexCompileError(regex::Error),
}

impl std::string::ToString for SigSetError {
    fn to_string(&self) -> String {
        match self {
            SigSetError::RegexSetAlreadyCompiled => "SigSet already compiled".to_string(),
            SigSetError::RegexSetNotCompiled => "SigSet not compiled".to_string(),
            SigSetError::RegexCompileError(_) => "Compilation error".to_string(),
        }
    }
}

impl SigSet {
    pub fn new() -> SigSet {
        SigSet {
            regex_set: None,
            regex_list: Vec::new(),
            id_list: Vec::new(),
        }
    }

    pub fn add(&mut self, regex: String, id: String) -> Result<(), SigSetError> {
        match &self.regex_set {
            Some(_) => Err(SigSetError::RegexSetAlreadyCompiled),
            None => {
                self.id_list.push(id);
                self.regex_list.push(regex);
                Ok(())
            }
        }
    }

    pub fn compile(&mut self) -> Result<(), SigSetError> {
        match &self.regex_set {
            Some(_) => Err(SigSetError::RegexSetAlreadyCompiled),
            None => match RegexSet::new(&self.regex_list) {
                Ok(res) => {
                    self.regex_set = Some(res);
                    Ok(())
                }
                Err(x) => Err(SigSetError::RegexCompileError(x)),
            },
        }
    }

    pub fn clear(&mut self) {
        self.regex_set = None;
        self.regex_list = Vec::new();
        self.id_list = Vec::new();
    }

    pub fn is_match(&self, s: &str) -> Result<bool, SigSetError> {
        match &self.regex_set {
            None => Err(SigSetError::RegexSetNotCompiled),
            Some(rs) => Ok(rs.is_match(s)),
        }
    }

    pub fn is_match_id(&self, s: &str) -> Result<Option<&String>, SigSetError> {
        match &self.regex_set {
            None => Err(SigSetError::RegexSetNotCompiled),
            Some(rs) => {
                let matches: Vec<_> = rs.matches(&s).into_iter().collect();
                if matches.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(&self.id_list[matches[0]]))
                }
            }
        }
    }

    pub fn is_match_ids(&self, s: &str) -> Result<Vec<&String>, SigSetError> {
        match &self.regex_set {
            None => Err(SigSetError::RegexSetNotCompiled),
            Some(rs) => {
                let matches: Vec<_> = rs.matches(&s).into_iter().collect();
                Ok(matches.into_iter().map(|x| &self.id_list[x]).collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! s {
        ( $e:expr ) => {
            ($e).to_string()
        };
    }
    macro_rules! testT {
        ( $e:expr ) => {
            let r = ($e);
            println!("expect True: {:?}", r);
            assert!(r.unwrap())
        };
    }
    macro_rules! testF {
        ( $e:expr ) => {
            let r = ($e);
            println!("expect False: {:?}", r);
            assert!(!r.unwrap())
        };
    }
    macro_rules! testOk {
        ( $e:expr ) => {
            let r = ($e);
            println!("expect Ok: {:?}", r);
            assert!(r.is_ok())
        };
    }
    macro_rules! testErr {
        ( $e:expr ) => {
            let r = ($e);
            println!("expect Err: {:?}", r);
            assert!(r.is_err())
        };
    }
    macro_rules! testEq {
        ( $e:expr , $f:expr ) => {
            let r = ($e);
            println!("expect ${:?}: {:?}", ($f), r);
            assert!(r.unwrap() == ($f))
        };
    }

    #[test]
    fn test_add() {
        let mut ss = SigSet::new();
        testOk!(ss.add(s!("^A+$"), s!("only As")));
        testOk!(ss.add(s!("^B+$"), s!("only Bs")));
        testOk!(ss.add(s!("^C+$"), s!("only Cs")));
    }
    #[test]
    fn test_compile() {
        let mut ss = SigSet::new();
        testOk!(ss.add(s!("^A+$"), s!("only As")));
        testOk!(ss.add(s!("^B+$"), s!("only Bs")));
        testOk!(ss.add(s!("^C+$"), s!("only Cs")));
        testOk!(ss.compile());
    }

    #[test]
    fn test_recompile() {
        let mut ss = SigSet::new();
        testOk!(ss.add(s!("^A+$"), s!("only As")));
        testOk!(ss.compile());
        testErr!(ss.compile());
    }

    #[test]
    fn test_add_err_after_compile() {
        let mut ss = SigSet::new();
        testOk!(ss.add(s!("^A+$"), s!("only As")));
        testOk!(ss.add(s!("^B+$"), s!("only Bs")));
        testOk!(ss.compile());
        testErr!(ss.add(s!("^C+$"), s!("only Cs")));
    }

    #[test]
    fn test_compile_err() {
        let mut ss = SigSet::new();
        testOk!(ss.add(s!("^A+$"), s!("only As")));
        testOk!(ss.add(s!("+[]+^B+$^"), s!("only Bs")));
        testErr!(ss.compile());
    }

    #[test]
    fn test_is_match() {
        let mut ss = SigSet::new();
        testOk!(ss.add(s!("^A+$"), s!("only As")));
        testOk!(ss.add(s!("^B+$"), s!("only Bs")));
        testOk!(ss.add(s!("^C+$"), s!("only Cs")));
        testOk!(ss.compile());
        testT!(ss.is_match(&s!("AAAAA")));
        testT!(ss.is_match(&s!("BBB")));
        testF!(ss.is_match(&s!("BBC")));
    }

    #[test]
    fn test_clear() {
        let mut ss = SigSet::new();
        testOk!(ss.add(s!("^A+$"), s!("only As")));
        testOk!(ss.compile());
        testT!(ss.is_match(&s!("AAAAA")));
        ss.clear();
        testErr!(ss.is_match(&s!("AAAAA")));
        testOk!(ss.add(s!("^B+$"), s!("only Bs")));
        testOk!(ss.compile());
        testF!(ss.is_match(&s!("AAAAA")));
        testT!(ss.is_match(&s!("BBB")));
    }

    #[test]
    fn test_match_id() {
        let mut ss = SigSet::new();
        testOk!(ss.add(s!("^A+$"), s!("only As")));
        testOk!(ss.add(s!("^B+$"), s!("only Bs")));
        testOk!(ss.add(s!("^C+$"), s!("only Cs")));
        testOk!(ss.compile());
        let r = ss.is_match_id(&s!("AAAAA"));
        assert!(r.unwrap().unwrap() == &s!("only As"));
        let r = ss.is_match_id(&s!("CCC"));
        assert!(r.unwrap().unwrap() == &s!("only Cs"));
    }

    #[test]
    fn test_match_ids() {
        let mut ss = SigSet::new();
        testOk!(ss.add(s!("^A+"), s!("starts with As")));
        testOk!(ss.add(s!("B+$"), s!("ends with Bs")));
        testOk!(ss.add(s!("^C+$"), s!("only Cs")));
        testOk!(ss.compile());

        testEq!(ss.is_match_ids(&s!("AAAAA")), vec!["starts with As"]);
        testEq!(
            ss.is_match_ids(&s!("AAAAABBBB")),
            vec!["starts with As", "ends with Bs"]
        );
    }
}
