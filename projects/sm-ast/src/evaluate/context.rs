use crate::{evaluate::Context, AST, ToWolfram, ToTex};
use num::{traits::Pow, ToPrimitive};
use std::collections::{BTreeMap, VecDeque};
use crate::ast::{Symbol, Position};
use crate::internal;

impl AST {
    pub fn forward(&mut self, ctx: &mut Context) {
        match self {
            AST::Expression { base, .. } => base.forward(ctx),
            AST::FunctionCall { name, ref arguments, ref options, ref position } => {
                name.forward(ctx);
                match **name {
                    AST::Symbol(ref s)=>{
                        if s.name == "Sequence" {
                            ()
                        }
                        else {
                            *self = evaluate_function(s, arguments, options, position.clone(), ctx)
                        }
                    }
                    _ => unimplemented!()
                }

            }
            AST::MultiplicativeExpression { expressions, .. } => {
                *self = evaluate_multiplicative(&expressions, ctx);
            }
            AST::List(v) => {
                *self = AST::List(evaluate_list_omit(v,ctx));
            }
            AST::UnaryOperators { .. } => {}
            AST::InfixOperators { infix, lhs, rhs, .. } => {
                lhs.forward(ctx);
                rhs.forward(ctx);
                // ,  `Vec<&mut Box<AST>>` ->  `&mut [AST]`
                match infix.as_str() {
                    "+" => {
                        *self = evaluate_additive(&vec![*lhs.clone(), *rhs.clone()], ctx);
                    }
                    "*" | "×" => {
                        *self = evaluate_multiplicative(&vec![*lhs.clone(), *rhs.clone()], ctx);
                    }
                    "^" => {
                        *self = evaluate_power(&vec![*lhs.clone(), *rhs.clone()], ctx);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

fn evaluate_list_omit(v: &mut VecDeque<AST>,ctx:&mut Context)->VecDeque<AST>{
    let mut new = VecDeque::with_capacity(v.len());
    for e in v {
        e.forward(ctx);
        match e {
            AST::Symbol(ref s)=>{
                if s.name == "Nothing" {
                    continue
                }
                else {
                    new.push_back(e.clone())
                }
            },
            AST::FunctionCall { name, arguments,.. }=>{
                match *name.clone() {
                    AST::Symbol(s)=>{
                        if s.name == "Sequence" {
                            let args = evaluate_list_omit(&mut VecDeque::from(arguments.clone()),ctx);
                            new.extend(args)
                        }
                        else {
                            new.push_back(e.clone())
                        }
                    },
                    _ =>new.push_back(e.clone())
                }
            }
            _ =>new.push_back(e.clone())
        }
    }
    return new;
}

fn evaluate_function(f: &Symbol, args:&Vec<AST>,kws: &BTreeMap<AST,AST>, position:Position,_: &mut Context) -> AST {
    match f.name.as_str() {
        "wolfram_form" => {
            AST::string(&args[0].to_wolfram_string())
        },
        "tex_form" => {
            AST::string(&args[0].to_tex())
        },
        "first" => {
            internal::first(&args[0]).unwrap()
        },
        "last" => {
            internal::last(&args[0]).unwrap()
        },
        "length" => {
            internal::length(&args[0]).unwrap()
        },
        "factorial"=> {
            internal::factorial(&args[0]).unwrap()
        },
        "fibonacci"=> {
            internal::fibonacci(&args[0]).unwrap()
        },
        _ => {
            println!("{:?}", f);
            unimplemented!()
        }
    }
}

fn evaluate_additive(vec: &[AST], _: &mut Context) -> AST {
    match vec {
        [AST::Integer(lhs), AST::Integer(rhs)] => AST::Integer(lhs + rhs),
        _ => {
            println!("{:?}", vec);
            unimplemented!()
        }
    }
}

fn evaluate_multiplicative(vec: &[AST], _: &mut Context) -> AST {
    match vec {
        [AST::Integer(lhs), AST::Integer(rhs)] => AST::Integer(lhs * rhs),
        _ => {
            println!("{:?}", vec);
            unimplemented!()
        }
    }
}

fn evaluate_power(vec: &[AST], _: &mut Context) -> AST {
    match vec {
        [AST::Integer(lhs), AST::Integer(rhs)] => match rhs.to_u64() {
            None => AST::Integer(lhs.clone()),
            Some(s) => AST::Integer(lhs.pow(s)),
        },
        _ => {
            println!("{:?}", vec);
            unimplemented!()
        }
    }
}
