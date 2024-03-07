// IntermediateRepresentation
pub struct Code {
    types: Vec<TypeDef>,
    functions: Vec<Function>,
    instructions: Instructions
}

pub enum TypeDef {
    Class(Class),
    Interface(Interface),
    Primitive
}

pub struct Class {
    parent: Option<Box<ParentImpl>>,
    interface_impls: Vec<InterfaceImpl>,
    members: Vec<Member>
}

pub struct ParentImpl {
    parent: TypeDef,
    override_members: Vec<Member>
}

pub struct InterfaceImpl {
    interface: Interface,
    override_members: Vec<Member>
}

pub struct Interface {
    parent: Option<Box<TypeDef>>
}

pub struct Function {
    sig: Signature,
    instructions: Instructions
}

pub struct Signature {
    id: String,
    ret_type: TypeDef,
    param_provider: Box<dyn ParamProvider>
}

pub trait ParamProvider {
    fn is_valid(&self, args: Vec<TypeDef>) -> bool;
}

pub enum Member {
    Function { function: Function, overrideable: bool },
    VarDef(VarDef)
}

pub enum Expr {
    FunctionEval(Function, Vec<Expr>),
    Identifier(Identifier),
    Literal(Literal)
}

pub enum Identifier {
    Id(String),
    OpRef(String),
    TemplateIdentifier(String, Vec<TypeDef>)
}

pub enum Literal {
    String(String),
    Double(f32),
    Int(i32)
}

pub enum Instruction {
    Expr(Expr),
    VarDef(VarDef)
}

pub struct VarDef {
    id: String,
    typedef: TypeDef,
    init: Option<Expr>
}

pub type Instructions = Vec<Instruction>;
