use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for DeclVar {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        let mut expr = None;
        for inner_cap in cap.into_inner() {
            match inner_cap.as_rule() {
                Rule::expr => {
                    expr = Some(inner_cap);
                }
                _ => {}
            }
        }
        let expr = Expr::from_cap(expr.unwrap());
        DeclVar {
            expr
        }
    }
}
