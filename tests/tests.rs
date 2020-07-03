use nulu2d;

#[test]
fn point_works() {
    let point = nulu2d::Point{
        x: 10,
        y: 15,
    };
    assert_eq!(point.x, 10);
    assert_eq!(point.y, 15);
}