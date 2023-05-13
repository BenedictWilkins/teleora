use pest::{Parser};
use pest_derive::Parser;
use pest_ascii_tree::into_ascii_tree;

#[derive(Parser)]
#[grammar = "teleora.pest"]
pub struct TeleoraParser;

const TEST_STR:&str = include_str!("../test/test1.tela");

fn main() {
    // parse the input using the rule 'document'
    let parse = TeleoraParser::parse(Rule::document, TEST_STR).unwrap();
    println!("{}", into_ascii_tree(parse).unwrap());

}

