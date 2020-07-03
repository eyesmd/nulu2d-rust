use std::f64::consts;

pub struct Point {
    x : f64,
    y : f64,
}

impl Point {
    // Constructors
    pub fn new(x : f64, y : f64) -> Point{
        return Point{
            x: x,
            y: y,
        };
    }

    // Accessors
    pub fn angle(&self) -> f64 {
        return (self.y.atan2(self.x) + 2.0*consts::PI) % (2.0*consts::PI);
    }
    pub fn norm(&self) -> f64 {
        return (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
    }

    // Mutators
    pub fn set_angle(&mut self, angle : f64) {
        self.direct_to(angle, self.norm());
    }
    pub fn direct_to(&mut self, angle : f64, norm : f64) {
        self.x = angle.cos() * norm;
        self.y = angle.sin() * norm;
    }
    pub fn point_to(&mut self, x : f64, y : f64) {
        self.x = x;
        self.y = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_in_delta(a: f64, b : f64) {
        // TODO: hacerla macro, creo
        let delta = 1e-5;
        if (a-b).abs() > delta {
            panic!("{} != {} (DELTA={})",a,b,delta);
        }
    }

    #[test]
    fn new_point_works() {
        let point = Point::new(-10.0, 15.5);
        assert_in_delta(point.x, -10.0);
        assert_in_delta(point.y, 15.5);
    }
    #[test]
    fn point_polar_read() {
        let point = Point::new(-1.0, -1.0);
        assert_in_delta(point.angle(), consts::PI*5.0/4.0);
        assert_in_delta(point.norm(), 2.0_f64.sqrt());

        let point = Point::new(1.0, 0.00001);
        assert_in_delta(point.angle(), 0.0);

        let point = Point::new(1.0, -0.00001);
        let expected = 2.0*consts::PI;
        assert_in_delta(point.angle(), expected);
    }
    #[test]
    fn point_polar_write() {
        let mut point = Point::new(1.0, 0.0);
        point.set_angle(consts::PI/4.0);
        assert_in_delta(point.x, 2.0_f64.sqrt()/2.0);
        assert_in_delta(point.y, 2.0_f64.sqrt()/2.0);
    }
}
