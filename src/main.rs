use std::mem;
use std::rc::Rc;

use pest::pratt_parser::{Assoc::*, Op, PrattParser};
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

mod ast;
use ast::{Operator, Statement, Goal, Method, Sequence};

use crate::ast::Parse;

#[derive(Parser)]
#[grammar = "teleora.pest"]
pub struct TeleoraParser;

const TEST_STR:&str = include_str!("../test/test1.tela");

fn main() {
    println!("{}", TEST_STR);
    match TeleoraParser::parse(Rule::document, TEST_STR) {
        Ok(mut pairs) => {
            //println!("{:?}", pairs.len());
            let goals = pairs.next().unwrap();
            // parse all goals
            for pair in goals.into_inner() {
                //println!("_goal: \n{:?}", pair);
                let goals = Statement::parse(pair);
                println!("goal: \n{:?}", goals);
            }
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
        }
    }
}





