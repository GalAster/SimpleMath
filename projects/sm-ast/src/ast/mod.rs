use bigdecimal::BigDecimal;
use num::BigInt;
use std::collections::BTreeMap;

mod traits;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AST {
    EmptyStatement,
    //
    Program(Vec<AST>),
    //
    /// function call
    /// function(name, *args, **kwargs)
    Function(Symbol, Vec<Parameter>),
    //
    /// true or false
    Boolean(bool),
    Integer(BigInt),
    Decimal(BigDecimal),
    Symbol(Symbol),
    String(String),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parameter {
    pub arguments: Vec<AST>,
    pub options: BTreeMap<AST, AST>,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub file: String,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expression {
    Source { raw: String, input: AST, eos: bool },
    Executed { raw: String, input: AST, output: AST },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    pub name_space: Vec<String>,
    pub name: String,
    pub kind: SymbolKind,
    /// maybe use bit flag
    pub attributes: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolKind {
    /// raw symbol
    Normal,
    /// alias of another one
    Alias,
    ///
    Prefix(Box<str>),
    /// infix operator with Precedence
    Infix(Box<str>, u8),
    Suffix(Box<str>),
}
