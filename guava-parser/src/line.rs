use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for Line {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        match cap.as_rule() {
            Rule::declvar => Line::DeclVar(DeclVar::from_cap(cap)),
            Rule::encapsScope => Line::Scope(todo!("Parse Encaps Scope")),
            Rule::expr => Line::Expr(Expr::from_cap(cap)),
            _ => panic!("Unexpected line!")
        }
    }
}
