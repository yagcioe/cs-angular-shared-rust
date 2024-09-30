use crate::array::rust_array::{self, RustArrayTrait};

#[repr(C)]
pub struct RustString {
    ptr: *mut u8,
    count: i32,
    capacity: i32,
}

impl RustArrayTrait<u8> for RustString {
    fn ptr(&self) -> *mut u8 {
        self.ptr
    }

    fn count(&self) -> i32 {
        self.count
    }

    fn capacity(&self) -> i32 {
        self.capacity
    }

    fn new(ptr: *mut u8, count: i32, capacity: i32) -> Self {
        Self {
            ptr,
            capacity,
            count,
        }
    }

    fn free(self) {
        rust_array::free(self);
    }
}

impl RustString {
    pub fn from_string(s: String) -> Self {
        rust_array::from_vec(s.into_bytes())
    }
}
