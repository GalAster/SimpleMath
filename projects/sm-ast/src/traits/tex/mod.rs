mod utils;

use crate::{ast::Parameter, ToTex, AST};
use itertools::Itertools;
use utils::{infix_tex, omit_brackets_function};

#[rustfmt::skip]
pub trait BoxArea {
    fn height(&self) -> usize { 1 }
    fn width(&self) -> usize { 1 }
}

impl ToTex for AST {
    fn to_tex(&self) -> String {
        match (*self).clone() {
            AST::EmptyStatement => format!(""),
            AST::Program(_) => unimplemented!(),
            AST::Integer(i) => format!("{}", i),
            AST::Decimal(f) => format!("{}", f),
            AST::Symbol(s) => format!("{}", s.name),
            AST::String(s) => format!(r"\\text{{{}}}", s),
            AST::Function(s, p) => {
                match s.name_space.iter().map(|e| e.as_str()).collect_vec().as_slice() {
                    ["std", "prefix"] => unimplemented!(),
                    ["std", "infix"] => return infix_tex(&s, &p[0]),
                    ["std", "suffix"] => unimplemented!(),
                    _ => (),
                }
                match s.name.as_ref() {
                    "sin" | "cos" | "tan" | "cot" | "sec" | "csc" | "arcsin" | "arccos" | "arctan" => {
                        format!(r"\\{}{}", s, omit_brackets_function(&p[0]))
                    }
                    "arccot" | "arcsec" | "arccsc" | "arcsinh" | "arccosh" | "arctanh" | "arccoth" | "arcsech" | "arccsch" => {
                        format!(r"\\operatorname{{{}}}{}", s, omit_brackets_function(&p[0]))
                    }
                    "List" => {
                        let max = p[0].arguments.iter().map(|e| e.height()).max().unwrap();
                        let e = &p[0].arguments.iter().map(AST::to_tex).collect_vec();
                        if max > 1 { format!(r"\\left\\{{{}\\right\\}}", e.join(", ")) } else { format!(r"\\{{{}\\}}", e.join(", ")) }
                    }
                    _ => {
                        let inner = p.iter().map(|e| e.to_tex()).collect_vec().join("");
                        format!(r"\\operatorname{{{}}}{}", s.name, inner)
                    }
                }
            }
            #[rustfmt::skip]
            AST::Boolean(b) => if b { format!(r"\\mathtt{{true}}") } else { format!(r"\\mathtt{{false}}") },
        }
    }
}

impl ToTex for Parameter {
    fn to_tex(&self) -> String {
        let terms = self.arguments.iter().map(|e| e.to_tex()).collect_vec().join(", ");
        format!(r"\\left({}\\right)", terms)
    }
}

impl BoxArea for AST {
    fn height(&self) -> usize {
        match self {
            AST::EmptyStatement => 0,
            AST::Program(_) => 1,
            AST::Function { .. } => 1,
            _ => 1,
        }
    }
    fn width(&self) -> usize {
        match self {
            AST::EmptyStatement => 0,
            AST::Program(_) => 1,
            AST::Function(s, p) => match s.name_space.iter().map(|e| e.as_str()).collect_vec().as_slice() {
                ["std", "prefix"] => unimplemented!(),
                ["std", "infix"] => p[0].arguments.len(),
                ["std", "suffix"] => unimplemented!(),
                _ => 1,
            },
            _ => 1,
        }
    }
}

impl BoxArea for Parameter {
    fn height(&self) -> usize {
        if self.arguments.len() == 0 {
            return 1;
        };
        self.arguments.iter().map(|e| e.height()).max().unwrap()
    }
    fn width(&self) -> usize {
        self.arguments.len()
    }
}
