use std::rc::Rc;

use pest::pratt_parser::{Assoc::*, Op, PrattParser};
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

mod ast;
use ast::{Expression, Operator, Statement, Goal};

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


fn parse_goal(goal: Pair<Rule>,  pratt: &PrattParser<Rule>) -> Vec<Goal> {
    //println!("-----{:?}", goal);
    match goal.as_rule() {
        Rule::goal => {
            let mut pairs = goal.into_inner();
            let head = Rc::new(parse_statement(pairs.next().unwrap(), pratt)); // TODO here we should convert head into normal form (convert into goal conditions for later processing)
            pairs
                .map(|p| parse_statement(p, pratt))
                .map(|b| Goal { head : Rc::clone(&head), body : b}).collect()
        },
        Rule::EOI => Vec::new(),
        _ => unreachable!("Expected rule: 'goal' but found {:?}", goal.as_rule())
    }
}

fn parse_statement(statement: Pair<Rule>,  pratt: &PrattParser<Rule>) -> Statement {
    println!("{:?}", statement.as_rule());
    match statement.as_rule() {
        Rule::head => {
            let mut pairs = statement.into_inner();
            let name = parse_statement(pairs.next().unwrap(), pratt);
            let args = Statement::List(pairs.map(|p| parse_statement(p, pratt)).collect());
            Statement::Head { name : Box::new(name), args : Box::new(args) }
        },
        Rule::body => {
            //println!("---{:?}", statement);
            let mut pairs = statement.into_inner();
            let conditions = Statement::List(pairs.next().unwrap().into_inner().map(|p| parse_statement(p, pratt)).collect()); 
            let actions = Statement::List(pairs.next().unwrap().into_inner().map(|p| parse_statement(p, pratt)).collect()); 
            Statement::Body { conditions: Box::new(conditions), actions: Box::new(actions) }
        },
        Rule::list => { Statement::List(statement.into_inner().map(|p| parse_statement(p, pratt)).collect()) }
        Rule::variable => Statement::Variable(statement.as_str().to_string()),
        Rule::atom => Statement::Atom(statement.as_str().to_string()),
        Rule::signed_integer => Statement::Integer(statement.as_str().parse::<i32>().unwrap()),
        Rule::signed_float => Statement::Float(statement.as_str().parse::<f32>().unwrap()),
        
        
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
                let goals = parse_goal(pair, &pratt);
                println!("{:?}", goals);
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

