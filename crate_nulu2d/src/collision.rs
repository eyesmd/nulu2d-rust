use crate::Point;
use crate::Polygon;
use crate::Segment;
use crate::Vector;

fn mtv(a: &Polygon, b: &Polygon) -> Option<Vector> {
    let mut mtv = Vector::new(f64::INFINITY, f64::INFINITY);
    let axes = a
        .segments()
        .into_iter()
        .chain(b.segments().into_iter())
        .map(|s| s.direction().perp().unit());

    for axis in axes {
        let mut mina = a
            .vertices
            .iter()
            .map(|v| *v * axis)
            .fold(f64::INFINITY, f64::min);
        let mut maxa = a
            .vertices
            .iter()
            .map(|v| *v * axis)
            .fold(f64::NEG_INFINITY, f64::max);
        let mut minb = b
            .vertices
            .iter()
            .map(|v| *v * axis)
            .fold(f64::INFINITY, f64::min);
        let mut maxb = b
            .vertices
            .iter()
            .map(|v| *v * axis)
            .fold(f64::NEG_INFINITY, f64::max);

        let mut overlap: f64;
        let mut neg = false;

        if mina >= minb {
            neg = !neg;
            let (_mina, _maxa) = (-maxa, -mina);
            let (_minb, _maxb) = (-maxb, -minb);
            mina = _mina;
            minb = _minb;
            maxa = _maxa;
            maxb = _maxb;
        }

        // find overlap
        if maxb <= maxa {
            overlap = maxb - minb;
            if minb - mina < maxa - maxb {
                overlap += minb - mina;
            } else {
                overlap += maxa - maxb;
                neg = !neg;
            }
        } else if minb <= maxa {
            overlap = maxa - minb
        } else {
            return None;
        }

        // assure separation
        overlap += 1e-9;

        // mtv update
        if overlap < mtv.norm() {
            mtv = axis * (if neg { -overlap } else { overlap });
        }
    }
    return Some(mtv);
}

fn containing(shape: &Polygon, point: Point) -> bool {
    let shape_point = shape.center();
    for segment in shape.segments().iter() {
        let orthogonal = segment.direction().perp().unit();

        let axis_projection = segment.a.vector_projection_to(orthogonal);
        let shape_point_projection = shape_point.vector_projection_to(orthogonal);
        let point_projection = point.vector_projection_to(orthogonal);

        let shape_point_difference = shape_point_projection - axis_projection;
        let point_difference = point_projection - axis_projection;

        if point_difference * shape_point_difference < 0.0 {
            return false;
        }
    }
    return true;
}

fn parametric_intersection(la: Segment, lb: Segment) -> Option<(f64, f64)> {
    let c = la.center();
    let v = la.direction();
    let d = lb.center();
    let w = lb.direction();

    let ndet = v.x * w.y - v.y * w.x;
    if ndet.abs() > f64::EPSILON {
        return Some((
            (w.y * (d.x - c.x) - w.x * (d.y - c.y)) / ndet,
            (v.y * (d.x - c.x) - v.x * (d.y - c.y)) / ndet,
        ));
    } else {
        return None;
    }
}

fn intersection(la: Segment, lb: Segment) -> Option<Point> {
    match parametric_intersection(la, lb) {
        Some((t1, t2))
            if (t1 >= 0.0 - f64::EPSILON && t1 <= 1.0 + f64::EPSILON)
                && (t2 >= 0.0 - f64::EPSILON && t2 <= 1.0 + f64::EPSILON) =>
        {
            return Some(la.a + (la.b - la.a) * t1)
        }
        _ => return None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;
    use similar::assert_similar;
    use similar::Similar;

    #[test]
    fn parametric_intersection_regular() {
        let l1 = Segment::from_arrow(Point::new(0.0, 0.0), Point::new(-0.5, -1.0));
        let l2 = Segment::from_arrow(Point::new(0.0, 3.0), Point::new(1.0, -1.0));

        let inter = parametric_intersection(l1, l2).unwrap();
        assert_eq!(-2.0, inter.0);
        assert_eq!(1.0, inter.1);

        let inter = parametric_intersection(l2, l1).unwrap();
        assert_eq!(1.0, inter.0);
        assert_eq!(-2.0, inter.1);
    }

    #[test]
    fn test_parametric_intersection_parallel() {
        let l1 = Segment::from_arrow(Point::new(0.0, 3.0), Point::new(1.0, -1.0));
        let l2 = Segment::from_arrow(Point::new(1.0, 0.0), Point::new(1.0, -1.0));

        assert_eq!(parametric_intersection(l2, l1), None);
        assert_eq!(parametric_intersection(l1, l2), None);
    }

    #[test]
    fn intersection() {
        let a = Point::new(0.0, 1.0);
        let b = Point::new(1.0, 1.0);
        let c = Point::new(1.0, 0.0);
        let d = Point::new(0.0, 0.0);
        let e = Point::new(2.0, 2.0);

        let s1 = Segment::from_points(a, c);
        let s2 = Segment::from_points(d, e);
        assert_eq!(super::intersection(s1, s2), Some(Point::new(0.5, 0.5)));

        let s1 = Segment::from_points(d, c);
        let s2 = Segment::from_points(a, b);
        assert_eq!(super::intersection(s1, s2), None);

        let s1 = Segment::from_points(a, c);
        let s2 = Segment::from_points(e, b);
        assert_eq!(super::intersection(s1, s2), None);
    }

    #[test]
    fn mtv_none() {
        let p1 = Polygon::new(&vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 2.0),
            Point::new(2.0, 2.0),
            Point::new(2.0, 0.0),
        ]);
        let p2 = Polygon::new(&vec![
            Point::new(3.0, 3.0),
            Point::new(3.0, 5.0),
            Point::new(4.0, 4.0),
            Point::new(5.0, 3.0),
        ]);
        assert_similar!(mtv(&p1, &p2), Option::<Point>::None);
    }

    #[test]
    fn mtv_up() {
        let p = Polygon::new(&vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 4.0),
            Point::new(4.0, 4.0),
            Point::new(4.0, 0.0),
        ]);
        let q = Polygon::new(&vec![
            Point::new(2.0, 3.0),
            Point::new(5.0, 3.0),
            Point::new(5.0, 5.0),
            Point::new(2.0, 5.0),
        ]);
        assert_similar!(mtv(&p, &q), Some(Point::new(0.0, 1.0)));
    }

    #[test]
    fn mtv_right() {
        let p = Polygon::new(&vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 4.0),
            Point::new(4.0, 4.0),
            Point::new(4.0, 0.0),
        ]);
        let q = Polygon::new(&vec![
            Point::new(3.0, 2.0),
            Point::new(6.0, 2.0),
            Point::new(6.0, 4.0),
            Point::new(3.0, 4.0),
        ]);
        assert_similar!(mtv(&p, &q), Some(Point::new(1.0, 0.0)));
    }

    #[test]
    fn mtv_right_up() {
        let p = Polygon::new(&vec![
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(-1.0, 0.0),
            Point::new(0.0, -1.0),
        ]);
        let q = Polygon::new(&vec![
            Point::new(0.0 + 0.25, 0.0 + 0.25),
            Point::new(2.0 + 0.25, 0.0 + 0.25),
            Point::new(2.0 + 0.25, 2.0 + 0.25),
            Point::new(0.0 + 0.25, 2.0 + 0.25),
        ]);
        assert_similar!(mtv(&p, &q), Some(Point::new(0.25, 0.25)));
    }

    #[test]
    fn mtv_reverse() {
        let p = Polygon::new(&vec![
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(-1.0, 0.0),
            Point::new(0.0, -1.0),
        ]);
        let q = Polygon::new(&vec![
            Point::new(0.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 2.0),
            Point::new(0.0, 2.0),
        ]);
        assert_similar!(mtv(&p, &q), Some(Point::new(0.5, 0.5)));
    }

    #[test]
    fn border_collision() {
        let p = Polygon::new(&vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0),
        ]);
        let q = Polygon::new(&vec![
            Point::new(0.0, 1.0),
            Point::new(0.0, 2.0),
            Point::new(1.0, 2.0),
            Point::new(1.0, 1.0),
        ]);

        assert_similar!(mtv(&p, &q), Some(Point::new(0.0, 0.0)));
        assert!(mtv(&p, &q).unwrap().norm() > 0.0);
    }

    #[test]
    fn contains_false() {
        let shape = Polygon::new(&vec![
            Point::new(0.0, 1.0),
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(3.0, 1.0),
            Point::new(3.0, 2.0),
            Point::new(2.0, 3.0),
            Point::new(1.0, 3.0),
            Point::new(0.0, 2.0),
        ]);
        let point = Point::new(0.5 - 0.1, 0.5 - 0.1);
        assert!(!containing(&shape, point));
    }

    #[test]
    fn contains_true() {
        let shape = Polygon::new(&vec![
            Point::new(0.0, 1.0),
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(3.0, 1.0),
            Point::new(3.0, 2.0),
            Point::new(2.0, 3.0),
            Point::new(1.0, 3.0),
            Point::new(0.0, 2.0),
        ]);
        let point = Point::new(0.5 + 0.1, 0.5 + 0.1);
        assert!(containing(&shape, point));
    }
}
