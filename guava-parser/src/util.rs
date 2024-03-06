use crate::Rule;

pub fn rule2str(r: Rule) -> String {
    match r {
        Rule::statement => "statement",
        Rule::expr => "expr",
        Rule::exprTail => "exprTail",
        Rule::decltype => "decltype",
        Rule::declvar => "declvar",
        Rule::identifier => "identifier",
        Rule::opidentifier => "opidentifier",
        Rule::templateidentifier => "templateidentifier",
        Rule::encapsScope => "encapsScope",
        Rule::encapsExpr => "encapsExpr",
        Rule::tuple => "tuple",
        Rule::tuples => "tuples",
        Rule::atom => "atom",
        Rule::parenexpr => "parenexpr",
        _ => "UNKNOWN"
    }.to_string()
}
