use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar/guava.pest"]
pub struct GuaveParser;

pub fn add(left: usize, right: usize) -> usize {
    let p = GuaveParser::parse(Rule::code, "").unwrap();
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
