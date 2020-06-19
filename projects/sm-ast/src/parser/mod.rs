use sm_parser::{Assoc, PrecClimber, Rule};
use crate::AST;
use std::collections::BTreeMap;

mod parse;
mod traits;

#[derive(Debug,Clone)]
pub struct ParserSettings {
    pub file: String,
    pub refine: bool,
}

#[allow(dead_code)]
pub(crate) enum ApplyOrSlice {
    Apply(Vec<AST>, BTreeMap<AST, AST>),
    Slice,
}

/// Determines the associativity and priority of operators
/// use Precedence in Mathematica
pub const OPERATORS: &[(Rule, u32, Assoc); 7] = &[
    // plus : a + b
    // minus: a - b
    (Rule::Plus, 310, Assoc::Left),
    (Rule::Minus, 310, Assoc::Left),
    //
    (Rule::Multiply, 400, Assoc::Left),
    (Rule::Divide, 470, Assoc::Left),
    //
    (Rule::Power, 590, Assoc::Right),
    // @
    (Rule::Map, 620, Assoc::Left),
    // dot: a.b
    (Rule::Dot, 900, Assoc::Left),
    // 1000 = Atom
];

pub static CLIMBER: PrecClimber<Rule> = PrecClimber::new_const(OPERATORS);