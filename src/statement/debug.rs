
use crate::statement::{Statement, List, UList, Sequence};

// reason for failure
enum EvaluateDebug {
    Length(Statement, Statement),
    Primitive(Statement, Statement),
    Type(Statement, Statement),
}


impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        if self.ispiped {
            let (head, tail) = self.items.split_at(self.items.len()-1);
            write!(f, "{:?}|{:?}", head, tail[0])?; // TODO this makes a copy, just format the vec properly
        } else {
            write!(f, "{:?}", self.items)?;
        }
        write!(f, "]")
    }
}


impl std::fmt::Debug for UList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        if self.ispiped {
            let (head, tail) = self.items.split_at(self.items.len()-1);
            write!(f, "{:?}|{:?}", head, tail[0])?; // TODO this makes a copy, just format the vec properly
        } else {
            write!(f, "{:?}", self.items)?;
        }
        write!(f, "}}")
    }
}

