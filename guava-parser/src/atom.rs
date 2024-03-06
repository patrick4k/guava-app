use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for Atom {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        for inner_cap in cap.into_inner() {
            match inner_cap.as_rule() {
                Rule::identifier => Atom::Identifier(Identifier::from_cap(inner_cap)),
                Rule::COMPILER_ID => Atom::CompilerId(inner_cap.as_str().to_string()),
                Rule::DOUBLE => Atom::Double(inner_cap.as_str().parse().unwrap()),
                Rule::INT => Atom::Int(inner_cap.as_str().parse().unwrap()),
                Rule::parenexpr => Atom::ParenExpr(Expr::from_cap(inner_cap)),
                _ => panic!("Unexpected atom!")
            }
        }
    }
}
