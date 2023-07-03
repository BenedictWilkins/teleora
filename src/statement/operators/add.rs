use crate::statement::{Integer, Float};

pub trait Add<Rhs = Self> {
    type Output;
    fn add(&self, rhs: &Rhs) -> Self::Output;
}

impl Add<Integer> for Integer {
    type Output = Integer;
    fn add(&self, rhs: &Integer) -> Integer { return Integer(self.0 + rhs.0); }
}

impl Add<Float> for Integer {
    type Output = Float;
    fn add(&self, rhs: &Float) -> Float { return Float((self.0 as f32) + rhs.0); }
}

impl Add<Integer> for Float {
    type Output = Float;
    fn add(&self, rhs: &Integer) -> Float { return Float(self.0 + (rhs.0 as f32)); }
}

impl Add<Float> for Float {
    type Output = Float;
    fn add(&self, rhs: &Float) -> Float { return Float(self.0 + rhs.0); }
}