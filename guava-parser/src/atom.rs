use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;
use std::str::FromStr;

impl FromCapture<'_> for Atom {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        for inner_cap in cap.into_inner() {
            return match inner_cap.as_rule() {
                Rule::identifier => Atom::Identifier(Identifier::from_cap(inner_cap)),
                Rule::COMPILER_ID => Atom::CompilerId(inner_cap.as_str().to_string()),
                Rule::DOUBLE => Atom::Double(f32::from_str(inner_cap.as_str()).unwrap()),
                Rule::INT => Atom::Int(i32::from_str(inner_cap.as_str()).unwrap()),
                Rule::STRING => Atom::String(inner_cap.as_str().to_string()),
                Rule::parenexpr => Atom::ParenExpr(Expr::from_cap(inner_cap)),
                _ => panic!("Unexpected atom!")
            };
        }
        panic!("Unknown Atom!")
    }
}
