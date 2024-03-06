use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for Identifier {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        let mut is_op_ref = false;
        let mut id: Option<String> = None;
        for inner_cap in cap.into_inner() {
            match inner_cap.as_rule() {
                Rule::OPREFSYMBOL => is_op_ref = true,
                Rule::ID => id = Some(inner_cap.as_str().to_string()),
                Rule::op => id = Some(inner_cap.as_str().to_string())
            }
        }

        if id.is_some() {
            if is_op_ref {
                return Identifier::OpRef(id.unwrap());
            }
            return Identifier::Id(id.unwrap());
        }

        todo!("Impl template id AST")
    }
}
