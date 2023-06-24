#[macro_export]
macro_rules! bump_box {
    ($mod: ident, $map: ident, $box: ident, $ty: ty) => {
        pub use $mod::{$map, $box};
        mod $mod {
            static MAP : std::sync::OnceLock<$map> = std::sync::OnceLock::new();

            
            #[derive(Debug)]
            pub struct $map {
                ptr: std::sync::atomic::AtomicPtr<$ty>,
                end: std::sync::atomic::AtomicPtr<$ty>,
            }

            
            pub struct $box {
                ptr: *mut $ty,
            }


            impl $map {
                pub fn init(cap: usize) {
                    unsafe {
                        MAP.get_or_init(|| {
                            let layout = std::alloc::Layout::array::<$ty>(cap).unwrap();
                            let allocation = std::alloc::alloc(layout) as *mut $ty;

                            let ptr = std::sync::atomic::AtomicPtr::new(allocation);
                            let end = std::sync::atomic::AtomicPtr::new(allocation.add(cap));

                            Self {
                                ptr,
                                end,
                            }
                        });
                    };

                    dbg!(&MAP);
                }
            }


            impl $box {
                pub fn new(val: $ty) -> Self {
                    unsafe {
                        let map = MAP.get().expect("map isn't initialised yet");
                        let ptr = map.ptr.fetch_update(
                            std::sync::atomic::Ordering::Release,
                            std::sync::atomic::Ordering::Acquire,
                            |p| (p < map.end.load(std::sync::atomic::Ordering::Relaxed)).then(|| p.add(1))
                        );

                        match ptr {
                            Ok(ptr) => {
                                *ptr.sub(1) = val;
                                Self { ptr }
                            },
                            Err(_) => panic!("map will overflow if we add this box"),
                        }
                    }
                }
            }


            impl std::ops::Deref for $box {
                type Target = $ty;

                fn deref(&self) -> &Self::Target {
                    unsafe { &*self.ptr }
                }
            }


            impl std::ops::DerefMut for $box {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { &mut *self.ptr }
                }
            }


            impl std::convert::AsMut<$ty> for $box {
                fn as_mut(&mut self) -> &mut $ty {
                    &mut *self
                }
            }


            impl std::convert::AsRef<$ty> for $box {
                fn as_ref(&self) -> &$ty {
                    &*self
                }
            }


            impl Clone for $box {
                fn clone(&self) -> Self {
                    Self::new(unsafe { &*self.ptr }.clone())
                }
            }


            impl std::fmt::Debug for $box {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}({:?})", stringify!($box), &*self)
                }
            }

        }
    }
}
