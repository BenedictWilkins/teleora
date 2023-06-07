
use crate::statement::{List, UList, Sequence};

impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        if self.0.1 {
            let (head, tail) = self.0.0.split_at(self.0.0.len()-1);
            write!(f, "{:?}|{:?}", Sequence(head.to_vec()), tail[0])?; // TODO this makes a copy, just format the vec properly
        } else {
            write!(f, "{:?}", self.0.0)?;
        }
        write!(f, "]")
    }
}


impl std::fmt::Debug for UList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        if self.0.1 {
            let (head, tail) = self.0.0.split_at(self.0.0.len()-1);
            write!(f, "{:?}|{:?}", Sequence(head.to_vec()), tail[0])?; // TODO this makes a copy, just format the vec properly
        } else {
            write!(f, "{:?}", self.0.0)?;
        }
        write!(f, "}}")
    }
}


impl std::fmt::Debug for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "[")?;
        let mut iter = self.0.iter();
        if let Some(item) = iter.next() {
            write!(f, "{:?}", item)?;
            for item in iter {
                write!(f, ", {:?}", item)?;
            }
        }
        write!(f, "")
    }
}