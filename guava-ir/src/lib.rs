extern crate guava_parser;

mod signature;
mod typedef;

use crate::ir::{Code, TypeDef};

use guava_parser::ast;

pub mod ir;

pub fn convert_ast(mut ast: ast::Code) -> Code {
    let decltypes: Vec<TypeDef> = ast.declarations.into_iter().map(TypeDef::from).collect();
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;
}
