mod macros;

pub trait Similar<T = Self> {
    fn is_similar(self, other : Self, eps : f64) -> bool;
}

impl Similar for f64 {
    fn is_similar(self, other : f64, eps : f64) -> bool {
        return (self-other).abs() < eps;
    }
}

impl Similar for u32 {
    fn is_similar(self, other : u32, _ : f64) -> bool {
        return self == other;
    }
}
