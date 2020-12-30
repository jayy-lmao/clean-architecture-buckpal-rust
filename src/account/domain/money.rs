#[derive(Debug, Clone, Copy)]
pub struct Money(f32);

impl Money {
    pub fn to_f32(&self) -> f32 {
        self.0
    }
    pub fn new(val: f32) -> Self {
        Self(val.to_owned())
    }
    pub fn negate(&mut self) -> Money {
        self.0 *= -1.;
        *self
    }
    pub fn is_positive(&self) -> bool {
        self.0 > 0.
    }
    pub fn add(val_a: Money, val_b: Money) -> Self {
        Self(val_a.0 + val_b.0)
    }
}
