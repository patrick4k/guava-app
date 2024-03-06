use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for Identifier {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        let mut is_op_ref = false;
        let mut id: Option<String> = None;
        for inner_cap in cap.clone().into_inner() {
            match inner_cap.as_rule() {
                Rule::OPREFSYMBOL => is_op_ref = true,
                Rule::ID => id = Some(inner_cap.as_str().to_string()),
                Rule::op => id = Some(inner_cap.as_str().to_string()),
                _ => {}
            }
        }

        if id.is_some() {
            if is_op_ref {
                return Identifier::OpRef(id.unwrap());
            }
            return Identifier::Id(id.unwrap());
        }

        let cap = cap.into_inner().next().unwrap();
        let mut id: Option<String> = None;
        let mut exprs: Vec<Expr> = vec![];
        for inner_cap in cap.into_inner() {
            match inner_cap.as_rule() {
                Rule::ID => id = Some(inner_cap.as_str().to_string()),
                Rule::op => id = Some(inner_cap.as_str().to_string()),
                Rule::expr => exprs.push(Expr::from_cap(inner_cap)),
                _ => {}
            }
        }
        Identifier::TemplateIdentifier(id.unwrap(), exprs)
    }
}
