#[derive(Debug, Clone)]
pub struct Money(f64);

impl Money {
    pub fn new(val: f64) -> Self {
        Self(val.to_owned())
    }
    pub fn negate(&self) -> Money {
        self.0 = -1. * self.0;
        *self
    }
    pub fn isPositive(&self) -> bool {
        self.0 > 0.
    }
    pub fn add(val_a: Money, val_b: Money) -> Self {
        Self(val_a.0 + val_b.0)
    }
}
