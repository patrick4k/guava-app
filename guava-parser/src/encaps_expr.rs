use crate::Capture;
use crate::ast::*;
use crate::FromCapture;
use crate::Rule;

impl FromCapture<'_> for EncapsExpr {
    fn from_cap<'a>(cap: Capture<'a>) -> Self {
        let mut encaps_type: Option<EncapsType> = None;
        let mut values: Option<Tuples> = None;

        for inner_cap in cap.into_inner() {
            match inner_cap.as_rule() {
                Rule::OPEN_PAREN => encaps_type = Some(EncapsType::Paren),
                Rule::OPEN_BRACE => encaps_type = Some(EncapsType::Brace),
                Rule::OPEN_BRACK => encaps_type = Some(EncapsType::Bracket),
                Rule::tuples => values = Some(Tuples::from_cap(inner_cap)),
                _ => {}
            }
        }

        let encaps_type = encaps_type.unwrap();
        let values = values.unwrap();

        EncapsExpr {
            encaps_type,
            values
        }
    }
}
