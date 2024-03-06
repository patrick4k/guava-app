use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for Tuple {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        let mut tuple: Tuple = vec![];
        for inner_cap in cap.into_inner() {
            match inner_cap.as_rule() {
                Rule::expr => tuple.push(Expr::from_cap(inner_cap)),
                _ => {}
            }
        }
        tuple
    }
}
