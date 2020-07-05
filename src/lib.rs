use std::f64::consts;
use std::ops;

#[derive(Copy, Clone)]
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

    pub fn angle(&self) -> f64 {
        return (self.y.atan2(self.x) + 2.0*consts::PI) % (2.0*consts::PI);
    }
    pub fn norm(&self) -> f64 {
        return (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
    }
    pub fn unit(&self) -> Point {
        return *self / self.norm();
    }
    pub fn rotated(&self, angle : f64) -> Point {
        let mut rotated = *self;
        rotated.set_angle(angle);
        return rotated;
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

// TODO: tal vez es una buena idea implementar estos traits para &Point,
// para que no consuman la instancia.
// https://doc.rust-lang.org/std/ops/index.html
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        return Point::new(self.x + other.x, self.y + other.y);
    }
}
impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        return Point::new(self.x - other.x, self.y - other.y);
    }
}
impl ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, scalar: f64) -> Point {
        return Point::new(self.x*scalar, self.y*scalar);
    }
}
impl ops::Mul<Point> for Point {
    type Output = f64;
    fn mul(self, other: Point) -> f64 {
        return self.x * other.x + self.y * other.y;
    }
}
impl ops::Div<f64> for Point {
    type Output = Point;
    fn div(self, scalar: f64) -> Point {
        return Point::new(self.x/scalar, self.y/scalar);
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    fn assert_in_delta(a: f64, b : f64) {
        // TODO: hacerla macro, creo
        let delta = 1e-5;
        if (a-b).abs() > delta {
            panic!("{} != {} (DELTA={})",a,b,delta);
        }
    }

    #[test]
    fn new_works() {
        let point = Point::new(-10.0, 15.5);
        assert_in_delta(point.x, -10.0);
        assert_in_delta(point.y, 15.5);
    }
    #[test]
    fn polar_read() {
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
    fn polar_write() {
        let mut point = Point::new(1.0, 0.0);
        point.set_angle(consts::PI/4.0);
        assert_in_delta(point.x, 2.0_f64.sqrt()/2.0);
        assert_in_delta(point.y, 2.0_f64.sqrt()/2.0);
    }
    #[test]
    fn add() {
        let point = Point::new(1.0, -1.0);
        let other = Point::new(12.0, 1.5);
        let sum = point + other;
        assert_in_delta(sum.x, 13.0);
        assert_in_delta(sum.y, 0.5);
    }
    #[test]
    fn sub() {
        let point = Point::new(1.0, -1.0);
        let other = Point::new(12.0, 1.5);
        let sub = point - other;
        assert_in_delta(sub.x, -11.0);
        assert_in_delta(sub.y, -2.5);
    }
    #[test]
    fn mul_scalar() {
        let point = Point::new(4.0, -1.0);
        let point = point * 2.0;
        assert_in_delta(point.x, 8.0);
        assert_in_delta(point.y, -2.0);
    }
    #[test]
    fn dot_product() {
        let point = Point::new(2.0, 2.0);
        let other = Point::new(1.0, -3.0);
        let dot_prod = point * other;
        assert_in_delta(dot_prod, -4.0);
    }
    #[test]
    fn div() {
        let point = Point::new(4.0, -1.0);
        let point = point / 2.0;
        assert_in_delta(point.x, 2.0);
        assert_in_delta(point.y, -0.5);
    }
    #[test]
    fn unit() {
        let point = Point::new(3.3, -1.12);
        let unit = point.unit();
        assert_in_delta(unit.norm(), 1.0);
        assert_in_delta(unit.angle(), point.angle());
    }
    #[test]
    fn rotated() {
        let point = Point::new(1.0, 0.0);
        let rotated = point.rotated(consts::PI/4.0);
        assert_in_delta(rotated.x, 0.5_f64.sqrt());
        assert_in_delta(rotated.y, 0.5_f64.sqrt());
    }
}
