use regex::Regex;
use std::fmt::Debug;
use crate::lexer::Token::{
    CLosingParenthesis, Comma, Def, Delimiter, Extern, Ident, Number, OpeningParenthesis, Operator,
};

#[allow(unused)]
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Def,
    Extern,
    Delimiter,
    OpeningParenthesis,
    CLosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String),
}

#[allow(unused)]
pub fn tokenize(input: &str) -> Vec<Token> {
    let comment_re = Regex::new(r"(?m)#.*\n").unwrap();
    //remove comments from input
    let preprocessed = comment_re.replace_all(input, "\n");
    let mut result = Vec::new();

    let token_re = Regex::new(concat!(
    r"(?P<ident>\p{Alphabetic}\w*)|",
    r"(?P<number>\d+\.?\d*)|",
    r"(?P<delimiter>;)|",
    r"(?P<oppar>\()|",
    r"(?P<clpar>\))|",
    r"(?P<comma>,)|",
    r"(?P<operator>\S)"
    ))
        .unwrap();
    for cap in token_re.captures_iter(preprocessed.as_ref()) {
        let token = if let Some(match_ident) = cap.name("ident") {
            match match_ident.as_str() {
                "def" => Def,
                "extern" => Extern,
                ident => Ident(ident.to_string()),
            }
        } else if let Some(match_num) = cap.name("number") {
            Number(
                match_num
                    .as_str()
                    .parse()
                    .expect("fail to convert string to float"),
            )
        } else if cap.name("delimiter").is_some() {
            Delimiter
        } else if cap.name("oppar").is_some() {
            OpeningParenthesis
        } else if cap.name("clpar").is_some() {
            CLosingParenthesis
        } else if cap.name("comma").is_some() {
            Comma
        } else {
            Operator(cap.name("operator").unwrap().as_str().to_string())
        };
        result.push(token);
    }

    result
}

#[cfg(test)]
mod test_lexer {
    use super::*;

    #[test]
    fn test_tokenize() {
        let res = tokenize("123456 lpc + (123)");
        println!("{:?}", res);
    }
}
