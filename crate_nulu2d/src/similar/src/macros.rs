#[macro_export]
macro_rules! assert_similar {
    ($left:expr, $right:expr, $delta:expr) => {
        assert!(
            ($left).is_similar($right, $delta),
            "{:?} !~ {:?}",
            $left,
            $right
        );
    };
    ($left:expr, $right:expr) => {
        assert_similar!($left, $right, 1e-3);
    };
}
