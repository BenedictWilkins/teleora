use std::rc::Rc;

#[derive(Debug)]
pub struct Goal {
    pub head : Rc<Statement>,
    pub body : Statement
}



#[derive(Debug)]
pub enum Statement {
    Head {
        name : Box<Statement>,
        args : Box<Statement> // List
    },
    Body { 
        conditions : Box<Statement>,    // List
        actions : Box<Statement>,       // List
    },
    List(Vec<Statement>),
    UList(Vec<Statement>),
    Obj(Vec<(Statement,Statement)>),
    Variable(String),
    Atom(String),
    Integer(i32),
    Float(f32),
    Empty,
    BinaryOperator {
        lhs: Box<Statement>,
        op: Operator,
        rhs: Box<Statement>,
    },
}

#[derive(Debug)]
pub enum Expression {
    Integer(i32),
    Float(f32),
    UnaryMinus(Box<Expression>),
    BinaryOperator {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Pipe,
}
