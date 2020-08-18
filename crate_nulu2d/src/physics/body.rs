use crate::Point;
use crate::Polygon;
use crate::Vector;

pub struct Body {
    pub shape: Polygon,
    pub mass: f64,
    pub friction: f64,
    pub velocity: Vector,
    pub frictionless: bool,
    pub gravityless: bool,
}

impl Body {
    pub fn new(
        shape: Polygon,
        mass: f64,
        friction: f64,
        frictionless: bool,
        gravityless: bool,
    ) -> Body {
        Body {
            shape,
            mass,
            friction,
            velocity: Vector::new(0.0, 0.0),
            frictionless,
            gravityless,
        }
    }

    pub fn width(&self) -> f64 {
        self.shape.width()
    }
    pub fn height(&self) -> f64 {
        self.shape.height()
    }
    pub fn center(&self) -> Point {
        self.shape.center()
    }
    pub fn left(&self) -> f64 {
        self.shape.left()
    }
    pub fn right(&self) -> f64 {
        self.shape.right()
    }
    pub fn top(&self) -> f64 {
        self.shape.top()
    }
    pub fn bottom(&self) -> f64 {
        self.shape.bottom()
    }

    pub fn move_xy(&mut self, offset: Vector) {
        self.shape.move_xy(offset);
    }
    pub fn move_x(&mut self, offset_x: f64) {
        self.shape.move_x(offset_x);
    }
    pub fn move_y(&mut self, offset_y: f64) {
        self.shape.move_y(offset_y);
    }

    /*
    pub fn set_center(&mut self, new_center : Point) {
        self.shape.set_center(new_center);
    }
    pub fn set_left(&mut self, new_left : f64) {
        self.shape.set_left(new_left);
    }
    pub fn set_right(&mut self, new_right : f64) {
        self.shape.set_right(new_right);
    }
    pub fn set_top(&mut self, new_top : f64) {
        self.shape.set_top(new_top);
    }
    pub fn set_bottom(&mut self, new_bottom : f64) {
        self.shape.set_bottom(new_bottom);
    }
    */

    // # Heuristic to find the normal exerted from the floor
    // def floor_normal
    //   floor_normal = Nulu::Point.new(0, 0)
    //   normals.each do |normal|
    //     if normal.y > floor_normal.y
    //       floor_normal = normal
    //     end
    //   end
    //   return floor_normal
    // end
}
