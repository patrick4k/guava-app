mod util;
mod decltype;
mod declvar;
mod line;
mod expr;
mod atom;
mod identifier;
mod encaps_expr;
mod ast;

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

struct CodeAst<'a> {
    pub declarations: Vec<Capture<'a>>,
    pub instructions: Vec<Capture<'a>>
}

enum LineAst<'a> {
    Instruction(Capture<'a>),
    Declaration(Capture<'a>)
}

enum Instruction<'a> {
    Rule(Capture<'a>),
    ScopeIn,
    ScopeOut
}

fn parse_code(s: &str) -> Code {
    let mut declarations: Vec<Capture> = vec![];
    let mut instructions: Vec<Capture> = vec![];

    let code_parse_result = GuavaParser::parse(Rule::code, s);
    match code_parse_result {
        Err(e) => panic!("{}", e),
        _ => {}
    }

    let code = code_parse_result.unwrap().next().unwrap().into_inner();
    for line in code {
        match line.as_rule() {
            Rule::line => {
                let line_ast = parse_line(line);
                match line_ast {
                    LineAst::Instruction(inst) => instructions.push(inst),
                    LineAst::Declaration(decl) => declarations.push(decl)
                }
            },
            _ => {}
        }
    }

    let declarations: Vec<DeclType> = declarations.into_iter().map(DeclType::from_cap).collect();
    let instructions: Vec<Line> = instructions.into_iter().map(Line::from_cap).collect();

    todo!("Create Code AST")
}

fn parse_line(line: Capture) -> LineAst {
    let mut lines = line.into_inner();
    let line = lines.next().unwrap();
    match line.as_rule() {
        Rule::statement => {
            let decl = line.into_inner().next().unwrap();
            match decl.as_rule() {
                Rule::decltype => return LineAst::Declaration(decl),
                _ => return LineAst::Instruction(decl)
            }
        }
        Rule::expr => {
            return LineAst::Instruction(line);
        }
        _ => panic!("Unexpected capture!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: add test
}
