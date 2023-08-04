use std::ffi::CString;

pub trait FFIString {
    fn to_c_ptr(self) -> Option<*mut i8>;
}


impl FFIString for &str {
    fn to_c_ptr(self) -> Option<*mut i8> {
        Some(CString::new(self).ok()?.into_raw())
    }
}


impl FFIString for String {
    fn to_c_ptr(self) -> Option<*mut i8> {
        Some(CString::new(self).ok()?.into_raw())
    }
}
