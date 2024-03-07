extern crate guava_util;

mod decltype;
mod declvar;
mod line;
mod expr;
mod atom;
mod identifier;
mod encaps_expr;
mod encaps_scope;
mod tuples;
mod tuple;

pub mod ast;

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::*;

use crate::ast::*;

#[derive(Parser)]
#[grammar = "./grammar/guava.pest"]
pub struct GuavaParser;

type Capture<'a> = Pair<'a, Rule>;

pub trait FromCapture<'a> {
    fn from_cap(cap: Capture<'a>) -> Self;
}

pub fn parse(s: &str) -> Code {
    parse_code(s)
}

fn parse_code(s: &str) -> Code {
    let mut declarations: Vec<DeclType> = vec![];
    let mut instructions: Vec<Line> = vec![];

    let code_parse_result = GuavaParser::parse(Rule::code, s);
    match code_parse_result {
        Err(e) => panic!("{}", e),
        _ => {}
    }

    let code = code_parse_result.unwrap().next().unwrap().into_inner();
    for line in code {
        match line.as_rule() {
            Rule::line => {
                let line_of_code = parse_line(line);
                match line_of_code {
                    LineOfCode::Line(inst) => instructions.push(inst),
                    LineOfCode::DeclType(decl) => declarations.push(decl)
                }
            },
            _ => {}
        }
    }

    Code {
        declarations,
        instructions
    }
}

enum LineOfCode {
    DeclType(DeclType),
    Line(Line)
}

fn parse_line(line: Capture) -> LineOfCode {
    let mut line = line.into_inner().next().unwrap();
    match line.as_rule() {
        Rule::statement => {

            line = line.into_inner().next().unwrap();
            match line.as_rule() {
                Rule::decltype => return LineOfCode::DeclType(DeclType::from_cap(line)),
                _ => {}
            }
        }
        Rule::expr => {}
        _ => panic!("Unexpected capture!")
    }
    LineOfCode::Line(Line::from_cap(line))
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: add test
}
