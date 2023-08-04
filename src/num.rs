pub trait OrdUtils: Ord + Sized {
    #[inline(always)]
    fn at_least(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    fn at_most(self, other: Self) -> Self {
        self.min(other)
    }
}

impl<T: Ord + Sized> OrdUtils for T {}


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PackedBool(u8);

impl PackedBool {
    #[inline(always)]
    pub fn new(markers: [bool; 8]) -> Self {
        let mut packed_value  = PackedBool(0);
        for (i, &marker) in markers.iter().enumerate() {
            packed_value.set(i as u8, marker);
        }
        packed_value
    }


    #[inline(always)]
    pub fn set(&mut self, index: u8, val: bool) {
        assert!(index < 8);
        self.0 |= (val as u8) << index;
    }


    #[inline(always)]
    pub fn get(&self, index: u8) -> bool {
        assert!(index < 8);
        (self.0 & (1 << index)) != 0
    }


    #[inline(always)]
    pub fn any(&self) -> bool {
        self.0 != 0
    }


    #[inline(always)]
    pub fn all(&self) -> bool {
        self.0 == u8::MAX
    }
}


#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_packed_bool() {
    let packed = PackedBool::new([true, false, false, false,
                                  false, true, true, false]);
    println!("{:b}", packed.0);


    assert_eq!(packed.get(0), true);
    assert_eq!(packed.get(1), false);
    assert_eq!(packed.get(2), false);
    assert_eq!(packed.get(3), false);
    assert_eq!(packed.get(4), false);
    assert_eq!(packed.get(5), true);
    assert_eq!(packed.get(6), true);
    assert_eq!(packed.get(7), false);
}
