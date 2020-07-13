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

impl<T> Similar for Option<T> where T : Similar {
    fn is_similar(self, other : Option<T>, eps : f64) -> bool {
        match (self, other) {
            (Some(x), Some(y)) => return x.is_similar(y, eps),
            (None, None) => return true,
            _ => return false,
        }
    }
}

/* NOTE:
The generic implementetion for IntoIterator trait produces conflicts with the
implementations for f64 and u32. AFAIU that's because somewhere and somewhen
f64/u32 may implement the IntoIterator trait. The unstable channel has a
solution for this: https://github.com/rust-lang/rust/issues/31844.

I the meantime I implemented Vec and slice separately, though it might be
a good idea to build a macro and avoid code duplication (TODO).
*/

// impl<I> Similar for I where I: IntoIterator, I::Item: Similar {
//     fn is_similar(self, other : I, eps : f64) -> bool {
//         let mut mine = self.into_iter();
//         let mut theirs = other.into_iter();
//         loop {
//             match (mine.next(), theirs.next()) {
//                 (None, None) => return true,
//                 (Some(v), Some(u)) => {
//                     if !u.is_similar(v, eps) {
//                         return false;
//                     }
//                 },
//                 _ => return false,
//             }
//         }
//     }
// }

impl<T> Similar for &Vec<T> where T : Similar + Copy {
    fn is_similar(self, other : &Vec<T>, eps : f64) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            if !self[i].is_similar(other[i], eps) {
                return false;
            }
        }
        return true;
    }
}

impl<T> Similar for &[T] where T : Similar + Copy {
    fn is_similar(self, other : &[T], eps : f64) -> bool {
    if self.len() != other.len() {
        return false;
    }
    for i in 0..self.len() {
            if !self[i].is_similar(other[i], eps) {
                    return false;
                }
            }
            return true;
        }
}
