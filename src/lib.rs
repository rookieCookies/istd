pub mod index_map;
pub mod num;
pub mod bump_box;


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



#[test]
fn test() {
    bump_box!(boxes, PreAllocMap, PreAllocBox, u32);

    PreAllocMap::init(10);
    for i in 0..10 {
        PreAllocBox::new(i);
    }
}
