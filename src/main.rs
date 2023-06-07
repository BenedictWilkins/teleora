
use pest::Parser;
use pest_derive::Parser;

#[macro_use]
mod utils;

mod interpret;

mod statement;
use statement::{Statement};

use itertools::Itertools;

#[derive(Parser)]
#[grammar = "teleora.pest"]
pub struct TeleoraParser;

const TEST_STR:&str = include_str!("../test/test1.tela");

fn main() {
    println!("{}", TEST_STR);
    match TeleoraParser::parse(Rule::document, TEST_STR) {
        Ok(pairs) => {
            interpret::interpret(pairs);
        }
        Err(e) => {
            println!("Parse failed: {:?}", e);
        }
    }
}

/*
fn evaluate(entry_point : Goal, arguments : Vec<Statement> ) -> Option<Action> {
    println!("{}", entry_point.get_name());

    // match arguments with the goal
   
    entry_point.evaluate(arguments); 
    
    return Option::None;

} */





