
use crate::statement::{BinaryOperator, UnaryOperator};

#[derive(Debug, Copy, Clone)]
pub struct Float(pub f32);
#[derive(Debug, Copy, Clone)]
pub struct Integer(pub i32);
#[derive(Debug, Copy, Clone)]
pub struct Boolean(pub bool);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable(pub String);
#[derive(Clone)]
pub struct Sequence(pub Vec<Statement>);
#[derive(Clone)]
pub struct List(pub (Sequence, bool));
#[derive(Clone)]
pub struct UList(pub (Sequence, bool));
#[derive(Clone, Debug)]
pub struct Object(pub (Sequence, bool));

#[derive(Debug, Clone)]
pub enum Statement {
    Float(Float),
    Integer(Integer),
    Boolean(Boolean),
    Atom(Atom),
    Variable(Variable),
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    Sequence(Sequence),
    List(List),
    UList(UList),
    Object(Object),
    Empty,
}

impl Default for Atom {
    fn default() -> Self { Self(Default::default())}
}

impl Default for Sequence {
    fn default() -> Self { Self(Vec::new())}
}

impl Default for List {
    fn default() -> Self { Self((Sequence::default(),false))}
}

impl Default for Variable {
    fn default() -> Self { Self(Default::default())}
}

impl Sequence {
    pub fn len(&self) -> usize {
        return self.0.len();
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Statement> {
        self.0.iter()
    }

    pub fn split_at(&self, index : usize) -> (&[Statement], &[Statement])  {
        return self.0.split_at(index);
    }
}


impl Statement {
    pub fn evaluate(&self) -> Statement {
        let z = match self {
            Statement::Float(x) => Statement::Float(*x),
            Statement::Integer(x) => Statement::Integer(*x),
            Statement::BinaryOperator(x) => x.evaluate(),
            Statement::UnaryOperator(x) => x.evaluate(),
            _ => unreachable!(),
        };
        return z;
    }
}


pub trait AsStatement {
    fn as_statement(self) -> Statement ;
}

impl AsStatement for Variable {
    fn as_statement(self) -> Statement {
        return Statement::Variable(self);
    }
}

impl AsStatement for Sequence {
    fn as_statement(self) -> Statement {
        return Statement::Sequence(self);
    }
}

impl AsStatement for List {
    fn as_statement(self) -> Statement {
        return Statement::List(self);
    }
}

impl AsStatement for UList {
    fn as_statement(self) -> Statement {
        return Statement::UList(self);
    }
}

impl AsStatement for Object {
    fn as_statement(self) -> Statement {
        return Statement::Object(self);
    }
}

impl AsStatement for Atom {
    fn as_statement(self) -> Statement {
        return Statement::Atom(self);
    }
}

impl AsStatement for Float {
    fn as_statement(self) -> Statement {
        return Statement::Float(self);
    }
}

impl AsStatement for Integer {
    fn as_statement(self)  -> Statement  {
        return Statement::Integer(self);
    }
}

impl AsStatement for f32 {
    fn as_statement(self)  -> Statement  {
        return Statement::Float(Float(self));
    }
}

impl AsStatement for i32 {
    fn as_statement(self)  -> Statement {
        return Statement::Integer(Integer(self));
    }
}

impl From<UList> for List {
    fn from(item: UList) -> Self {
        return List((item.0.0, item.0.1));
    }
}

impl From<List> for UList {
    fn from(item: List) -> Self {
        return UList((item.0.0, item.0.1));
    }
}



