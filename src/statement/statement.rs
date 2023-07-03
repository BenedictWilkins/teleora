
use std::{rc::Rc, collections::HashMap};

use crate::statement::{BinaryOperator, UnaryOperator, Frame, Sequence, List, UList, Object};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Float(pub f32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Integer(pub i32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Boolean(pub bool);

#[derive(Debug, Clone, PartialEq)]
pub struct Atom(pub String);

/*
#[derive(Debug, Clone, PartialEq)]
pub struct Compound {
    pub name : String,
    pub arguments : Sequence,
} */


#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name : String
}

impl Variable {
    pub fn is_anonymous(&self) -> bool {
        return self.name == "_";
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Float(Float),
    Integer(Integer),
    Boolean(Boolean),
    Atom(Atom),
    //Compound(Compound),
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

impl Default for Variable {
    fn default() -> Self { Self { ..Default::default() }}
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



// pattern matching

pub trait Ground {
    fn ground(&self, other : &Statement) -> Option<&Self>;
}







// converting types to their corresponding Statement variants... could probably be done with a macro...
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

// Type conversions
impl From<UList> for List {
    fn from(item: UList) -> Self {
        return List { items : item.items, ispiped : item.ispiped };
    }
}

impl From<List> for UList {
    fn from(item: List) -> Self {
        return UList { items : item.items, ispiped : item.ispiped };
    }
}

impl List {
    pub fn new(items : Sequence, ispiped : bool) -> Self {
        return List { items : items, ispiped : ispiped };
    }

    pub fn len(&self) -> usize {
        return self.items.len();
    }
}

impl UList {
    pub fn new(items : Sequence, ispiped : bool) -> Self {
        return UList { items : items, ispiped : ispiped };
    }

    pub fn len(&self) -> usize {
        return self.items.len();
    }

}



