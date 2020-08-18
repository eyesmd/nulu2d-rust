// A 'point.rs' file defines a module called 'point' at crate root
// 'mod point;' loads said module 'point'
// Once loaded, the Point struct can be accesed via 'crate::point::Point'
// 'use' can be used to make a shortcuts

mod geometry;

pub use crate::geometry::Point;
pub use crate::geometry::Polygon;
pub use crate::geometry::Segment;
pub use crate::geometry::Vector;

mod physics;
