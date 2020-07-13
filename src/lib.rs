
// A 'point.rs' file defines a module called 'point' at crate root
// 'mod point;' loads said module 'point'
// Once loaded, the Point struct can be accesed via 'crate::point::Point'
// 'use' can be used to make a shortcuts

mod point;
mod segment;
mod polygon;
mod collision;

pub use crate::point::Point;
pub use crate::point::Vector;
pub use crate::segment::Segment;
pub use crate::polygon::Polygon;
