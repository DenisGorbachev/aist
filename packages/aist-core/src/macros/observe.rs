#[macro_export]
macro_rules! observe {
    ($report:expr, $result:expr) => {
        match $result {
            Ok(value) => value,
            Err(source) => {
                $report.error(stringify!($result), source);
                return;
            }
        }
    };
}
