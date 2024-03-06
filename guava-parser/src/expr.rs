use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for Expr {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        let mut expr: Expr = vec![];
        for inner_cap in cap.into_inner() {
            expr.push(ExprItem::from_cap(inner_cap));
        }
        expr
    }
}

impl FromCapture<'_> for ExprItem {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        match cap.as_rule() {
            Rule::op => ExprItem::Op(cap.as_str().to_string()),
            Rule::atom => ExprItem::Atom(Atom::from_cap(cap)),
            _ => panic!("Unexpected item!")
        }
    }
}
