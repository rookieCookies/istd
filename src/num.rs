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

