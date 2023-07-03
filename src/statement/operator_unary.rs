
use crate::statement::{Statement, Float, Integer};
use crate::statement::operators::Negate;

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Minus(Box<Statement>),
    Not(Box<Statement>), // TODO
}


impl UnaryOperator { 
    pub fn evaluate(&self) -> Statement {
        let y = match self {
            UnaryOperator::Minus(x) =>  UnaryOperator::negate(&x.evaluate()),
            _ => unreachable!(),
        };
        return y;
    }
    
    fn negate(x : &Statement) -> Statement {
        let z = match x {
            Statement::Boolean(xx)    => Statement::Boolean(xx.negate()),
            Statement::Float(xx)   => Statement::Float(xx.negate()),
            Statement::Integer(xx) => Statement::Integer(xx.negate()),
            _ => unreachable!(),
        };
        return z;
    }

    
}
