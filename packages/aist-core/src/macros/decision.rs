/// Allows the actor who reads the code to see what `$actual` decision has been made by `$expected` function.
#[macro_export]
macro_rules! decision {
    ($actual:expr, $expected:expr) => {
        let actual = $actual;
        debug_assert_eq!(&actual, &$expected);
        actual
    };
}
