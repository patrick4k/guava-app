use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for EncapsScope {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        let mut encaps_scope: EncapsScope = vec![];
        for inner_cap in cap.into_inner() {
            match inner_cap.as_rule() {
                Rule::line => encaps_scope.push(Line::from_cap(inner_cap)),
                _ => {}
            }
        }
        encaps_scope
    }
}
