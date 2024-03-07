use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for DeclType {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        let mut compiler_tags: Vec<String> = vec![];
        let mut identifiers: Vec<Identifier> = vec![];
        let mut args: Option<EncapsExpr> = None;
        let mut body: Option<Line> = None;
        for inner_cap in cap.into_inner() {
            match inner_cap.as_rule() {
                Rule::COMPILER_TAG => compiler_tags.push(inner_cap.as_str().to_string()),
                Rule::identifier => identifiers.push(Identifier::from_cap(inner_cap)),
                Rule::encapsExpr => args = Some(EncapsExpr::from_cap(inner_cap)),
                Rule::expr => body = Some(Line::from_cap(inner_cap)),
                Rule::encapsScope => body = Some(Line::from_cap(inner_cap)),
                _ => {}
            }
        }

        let identifier = identifiers.pop().unwrap();
        let prefix_identifiers = identifiers;
        let body = body.unwrap();
        DeclType {
            compiler_tags,
            prefix_identifiers,
            args,
            identifier,
            body
        }
    }
}
