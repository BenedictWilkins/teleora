use pest::pratt_parser::{Assoc::*, Op, PrattParser};
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

mod ast;
use ast::{Expression, Operator, Statement,};

#[derive(Parser)]
#[grammar = "teleora.pest"]
pub struct TeleoraParser;

const TEST_STR:&str = include_str!("../test/test1.tela");


pub fn parse_expr(pairs: Pairs<Rule>,  pratt: &PrattParser<Rule>) -> Expression {
    pratt.map_primary(|primary| match primary.as_rule() {
            Rule::signed_integer => Expression::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::signed_float => Expression::Float(primary.as_str().parse::<f32>().unwrap()), 
            Rule::expr => parse_expr(primary.into_inner(), pratt),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Operator::Add,
                Rule::subtract => Operator::Subtract,
                Rule::multiply => Operator::Multiply,
                Rule::divide => Operator::Divide,
                Rule::modulo => Operator::Modulo,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expression::BinaryOperator {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::minus => Expression::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}

fn parse_statement(statement: Pair<Rule>,  pratt: &PrattParser<Rule>) -> Statement {
    match statement.as_rule() {
        
        Rule::head => Statement::Head { name: Box::new(String::from(statement.as_str()))}, 
        Rule::body => Statement::Body { }
        Rule::EOI => Statement::Empty,
        _ => unreachable!("{}", statement),
    }  
}


fn main() {
    use Rule::*;
    let pratt = PrattParser::<Rule>::new()
        .op(Op::infix(add, Left) | Op::infix(subtract, Left))
        .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
        .op(Op::prefix(minus));

    println!("{}", TEST_STR);
    match TeleoraParser::parse(Rule::document, TEST_STR) {
        Ok(mut pairs) => {
            //println!("---- {:#?}", pairs.next().unwrap().as_rule());
            // get all goals in document
            
            let goal_pairs = pairs.next().unwrap().into_inner();
            for pair in goal_pairs {
                parse_statement(pair, &pratt);
                //parse_goal(); 
            }

        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
        }
    }

        
    
    // parse the input using the rule 'document'
    //let parse = TeleoraParser::parse(Rule::document, TEST_STR).unwrap();
    //println!("{}", into_ascii_tree(parse).unwrap());

}

