use std::rc::Rc;

pub mod index_map;
pub mod index_vec;
pub mod num;
pub mod bump_box;
pub mod strings;


#[macro_export]
macro_rules! static_assert {
    ($expr: expr, $l: ident) => {
        const $l : () = assert!($expr);
    }
}


#[macro_export]
macro_rules! static_assert_eq {
    ($e1: expr, $e2: expr, $l: ident) => {
        const $l : () = assert!($e1 == $e2);
    }
}


#[macro_export]
macro_rules! static_assert_neq {
    ($e1: expr, $e2: expr, $l: ident) => {
        const $l : () = assert!($e1 != $e2);
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


pub fn make_mut_slice<T: Clone>(rc: &mut Rc<[T]>) -> &mut [T] {
    if Rc::get_mut(rc).is_none() {
        *rc = Rc::from(&**rc);
    }

    let r = Rc::get_mut(rc);
    debug_assert!(r.is_some());

    unsafe { r.unwrap_unchecked() }
}


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Immutable<T>(T);

impl<T> Immutable<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
}


impl<T> core::ops::Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<T> AsRef<T> for Immutable<T> {
    fn as_ref(&self) -> &T {
        self
    }
}
