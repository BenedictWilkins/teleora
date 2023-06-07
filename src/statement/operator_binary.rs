

use crate::statement::{Statement, Float, Integer};
use crate::statement::add::Add;


#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add(Box<Statement>, Box<Statement>),
    Subtract(Box<Statement>, Box<Statement>), // TODO
    Multiply(Box<Statement>, Box<Statement>),
    Divide(Box<Statement>, Box<Statement>),
    Modulo(Box<Statement>, Box<Statement>),
    And(Box<Statement>, Box<Statement>),
    Or(Box<Statement>, Box<Statement>),
    GreaterThan(Box<Statement>, Box<Statement>),
    GreaterThanEqual(Box<Statement>, Box<Statement>),
    LessThan(Box<Statement>, Box<Statement>),
    LessThanEqual(Box<Statement>, Box<Statement>),
    Equal(Box<Statement>, Box<Statement>),
}

impl BinaryOperator { 
    pub fn evaluate(&self) -> Statement {
        let y = match self {
            BinaryOperator::Add(x,y) =>  BinaryOperator::add(&x.evaluate(), &y.evaluate()),
            _ => unreachable!(),
        };
        return y;
    }
    
    fn add(x : &Statement , y : &Statement ) -> Statement {
        let z = match (x, y) {
            (Statement::Float(xx),   Statement::Float(yy))   => Statement::Float(xx.add(yy)),
            (Statement::Float(xx),   Statement::Integer(yy)) => Statement::Float(xx.add(yy)),
            (Statement::Integer(xx), Statement::Float(yy))   => Statement::Float(xx.add(yy)),
            (Statement::Integer(xx), Statement::Integer(yy)) => Statement::Integer(xx.add(yy)),
            _ => unreachable!(),
        };
        return z;
    }

    
}


fn main() {

    //let x1 = Statement::BinaryOperator(BinaryOperator::Add(&1.0.as_statement(), &1.as_statement()));
    //let x2 = BinaryOperator::Add(&1.0.as_statement(), &x1);
    //let s = Statement::BinaryOperator(x2);
    //println!("{:?}", s.evaluate());
}
