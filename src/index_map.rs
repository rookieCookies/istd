
/// Creates an index map with the given names
/// containing the given type
#[macro_export]
macro_rules! index_map {
    ($map: ident, $key: ident, $ty: ty) => {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Debug, Hash)]
        pub struct $key(usize);

        
        #[derive(Debug, PartialEq)]
        pub struct $map {
            map: std::collections::HashMap<std::rc::Rc<$ty>, $key>,
            vec: Vec<std::rc::Rc<$ty>>,
        }


        impl $map {
            pub fn new() -> Self {
                Self { 
                    vec: Vec::new(), 
                    map: std::collections::HashMap::new()
                }
            }


            #[inline(always)]
            pub fn with_capacity(cap: usize) -> Self {
                Self { 
                    vec: Vec::with_capacity(cap),
                    map: std::collections::HashMap::with_capacity(cap)
                }
            }
            
            
            #[inline(always)]
            pub fn insert(&mut self, value: $ty) -> $key {
                if let Some(v) = self.map.get(&value) {
                    return *v
                }

                
                let rc = std::rc::Rc::new(value);
                self.vec.push(rc.clone());
                let key = $key(self.vec.len() - 1);
                self.map.insert(rc, key);
                key
            }


            #[inline(always)]
            pub fn get(&self, index: $key) -> &$ty {
                self.vec.get(index.0).map(|x| &**x).unwrap()
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
        }


        impl Default for $map {
            fn default() -> Self {
                Self::new()
            }
        }
        

        impl core::ops::Index<$key> for $map {
            type Output = $ty;
            
            fn index(&self, key: $key) -> &Self::Output {
                &self.vec[key.0]
            }
        }

    }
}

