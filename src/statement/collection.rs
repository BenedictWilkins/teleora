use std::ops::{Deref, Index};
use crate::statement::Statement;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub struct Collection<T> { 
    pub items: T,
}

pub type Sequence = Collection<Vec<Statement>>;
pub type SequenceRef<'a> = Collection<&'a [Statement]>;

impl<S, T: Deref<Target=[S]>> Collection<T> {
    // shared implementations go here
    pub fn iter(&self) -> std::slice::Iter<'_, S> {
        self.items.iter()
    }

    pub fn split_at(&self, index : usize) -> (Collection<&[S]>, Collection<&[S]>) {
        return (Collection { items : &self.items[..index]}, Collection {items : &self.items[index..]});
    }

    pub fn len(&self) -> usize {
        return self.items.len();
    }
}

impl Collection<Vec<Statement>> {
    //methods only for the owned version go here

    pub fn new(items : Vec<Statement>) -> Self {
        return Self { items : items};
    }

    pub fn as_ref(&self) -> SequenceRef  {
        return Collection{ items: &self.items}
    }    
}

impl Collection<&[Statement]> {
    
    pub fn to_vec(&self) -> Sequence {
        return Collection::new(self.items.to_vec());
    }
}

impl<S, T: Deref<Target=[S]>>  Index<usize> for Collection<T> {
    type Output = S;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.items[index];
    }
}

impl<'a, S : Debug, T : Deref<Target=[S]>> std::fmt::Debug for Collection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.items.iter();
        if let Some(item) = iter.next() {
            write!(f, "{:?}", item)?;
            for item in iter {
                write!(f, ", {:?}", item)?;
            }
        }
        write!(f, "")
    }
}


#[derive(Clone, PartialEq)]
pub struct List {
    pub items : Sequence,
    pub ispiped : bool,
}

#[derive(Clone, PartialEq)]
pub struct UList {
    pub items : Sequence,
    pub ispiped : bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object(pub (Sequence, bool));


impl Default for Sequence {
    fn default() -> Self { Self { items : Vec::new() }}
}

impl Default for UList {
    fn default() -> Self { Self { ..Default::default() }}
}

impl Default for List {
    fn default() -> Self { Self { ..Default::default() }}
}