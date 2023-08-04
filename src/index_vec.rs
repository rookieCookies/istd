
/// Creates an index map with the given names
/// containing the given type
#[macro_export]
macro_rules! index_vec {
    ($map: ident, $key: ident, $ty: ty) => {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Debug, Hash)]
        pub struct $key(usize);

        
        #[derive(Debug, PartialEq)]
        pub struct $map {
            vec: Vec<$ty>,
        }


        impl $map {
            pub fn new() -> Self {
                Self { vec: Vec::new() }
            }


            #[inline(always)]
            pub fn with_capacity(cap: usize) -> Self {
                Self { vec: Vec::with_capacity(cap) }
            }
            
            
            #[inline(always)]
            pub fn push(&mut self, value: $ty) -> $key {
                self.vec.push(value);
                $key(self.vec.len() - 1)
            }


            #[inline(always)]
            pub fn get(&self, index: $key) -> Option<&$ty> {
                self.vec.get(index.0)
            }

            
            #[inline(always)]
            pub fn get_mut(&mut self, index: $key) -> Option<&mut $ty> {
                self.vec.get_mut(index.0)
            }


            #[inline(always)]
            pub fn len(&self) -> usize {
                self.vec.len()
            }


            #[inline(always)]
            pub fn capacity(&self) -> usize {
                self.vec.capacity()
            }


            #[inline(always)]
            pub fn is_empty(&self) -> bool {
                self.vec.is_empty()
            }


            #[inline(always)]
            pub fn as_slice(&self) -> &[$ty] {
                &self.vec
            }
        }


        impl core::ops::Index<$key> for $map {
            type Output = $ty;
            
            fn index(&self, key: $key) -> &Self::Output {
                &self.vec[key.0]
            }
        }


        impl core::ops::IndexMut<$key> for $map {
            fn index_mut(&mut self, key: $key) -> &mut Self::Output {
                &mut self.vec[key.0]
            }
        }

    }
}

