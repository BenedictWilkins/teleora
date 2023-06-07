
use crate::statement::{Float, Integer, Boolean};

pub trait Negate {
    fn negate(self) -> Self;
}

impl Negate for Float {
    fn negate(self) -> Self { return Float(-self.0); }
}

impl Negate for Integer {
    fn negate(self) -> Self { return Integer(-self.0); }
}

impl Negate for Boolean {
    fn negate(self) -> Self { return Boolean(!self.0); }
}
