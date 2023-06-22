pub mod index_map;
pub mod num;


#[macro_export]
macro_rules! static_assert {
    ($expr: expr) => {
        const _ : () = assert!($expr);
    }
}


#[macro_export]
macro_rules! static_assert_eq {
    ($e1: expr, $e2: expr) => {
        const _ : () = assert!($e1 == $e2);
    }
}


#[macro_export]
macro_rules! static_assert_neq {
    ($e1: expr, $e2: expr) => {
        const _ : () = assert!($e1 != $e2);
    }
}

