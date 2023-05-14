#[derive(Debug)]
pub enum Statement {
    Goal {
        head: Box<Statement>,
        conditions : Box<Statement>,
        action : Box<Statement>
    },
    Head { 
        name : Box<String>,
        //args : Vec<Primitive>
    },
    Body { 
        condition : Box<Statement>,
        action : Box<Statement>,
    },
    Primitive(Primitive),
    Empty,
}

#[derive(Debug)]
pub enum Primitive {
    Variable(Box<String>),
    Atom(Box<String>),
    Integer(i32),
    Float(f32),
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
}
