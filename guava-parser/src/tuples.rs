use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for Tuples {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        let mut tuples: Tuples = vec![];
        for inner_cap in cap.into_inner() {
            match inner_cap.as_rule() {
               Rule::tuple => tuples.push(Tuple::from_cap(inner_cap)),
                _ => {}
            }
        }
        tuples
    }
}
