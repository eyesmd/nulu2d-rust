pub trait Similar<T = Self> {
    fn is_similar(self, other : Self) -> bool;
}

impl Similar for f64 {
    fn is_similar(self, other : f64) -> bool {
        return (self-other).abs() < 1e-5;
    }
}

impl Similar for u32 {
    fn is_similar(self, other : u32) -> bool {
        return self == other;
    }
}
