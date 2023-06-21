/// Creates an index map with the given names
/// containing the given type
#[macro_export]
macro_rules! index_map {
    ($map: ident, $key: ident, $ty: ty) => {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Debug, Hash)]
        pub struct $key(usize);

        
        #[derive(Debug, PartialEq)]
        pub struct $map {
            vec: Vec<$ty>,
        }


        impl $map {
            pub fn push(&mut self, value: $ty) -> $key {
                self.vec.push(value);
                $key(self.vec.len() - 1)
            }


            pub fn get(&self, index: $key) -> Option<&$ty> {
                self.vec.get(index.0)
            }

            
            pub fn get_mut(&mut self, index: $key) -> Option<&mut $ty> {
                self.vec.get_mut(index.0)
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
