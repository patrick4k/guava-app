use crate::Capture;

pub struct Code {
    pub declarations: Vec<DeclType>,
    pub instructions: Vec<Line>
}

pub struct DeclType {
    pub compiler_tags: Vec<String>,
    pub prefix_identifiers: Vec<Identifier>,
    pub args: Option<EncapsExpr>,
    pub identifier: Identifier,
    pub body: Line
}

pub struct DeclVar {
    pub expr: Expr
}

pub enum Identifier {
    Id(String),
    OpRef(String),
    TemplateIdentifier(String, Vec<Expr>)
}

pub enum Line {
    DeclVar(DeclVar),
    Scope(Vec<Line>),
    Expr(Expr)
}

pub enum ExprItem {
    Op(String),
    Atom(Atom)
}

pub type Expr = Vec<ExprItem>;

pub enum Atom {
    Identifier(Identifier),
    CompilerId(String),
    Double(f32),
    Int(i32),
    String(String),
    ParenExpr(Expr)
}

pub enum EncapsType {
    Bracket,
    Brace,
    Paren
}

pub struct EncapsExpr {
    pub encaps_type: EncapsType,
    pub values: Vec<Vec<Expr>>
}
