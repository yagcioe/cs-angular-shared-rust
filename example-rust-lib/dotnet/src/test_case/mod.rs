use crate::array::{csharp_array, rust_array};

#[repr(C)]
#[derive(Default, Clone)]
pub struct TestStruct {
    pub value: i32,
    pub value1: i32,
    pub value2: i32,
    pub value3: i32,
    pub value4: i32,
    pub value5: i32,
    pub value6: i32,
    pub value7: i32,
    pub value8: i32,
    pub value9: i32,
    pub value10: i32,
    pub value11: i32,
    pub value12: i32,
    pub value13: i32,
    pub value14: i32,
    pub value15: i32,
    pub value16: i32,
    pub value17: i32,
    pub value18: i32,
    pub key: i32,
}

#[repr(C)]
pub struct TestStructArray {
    ptr: *mut TestStruct,
    length: i32,
    capacity: i32,
}

impl csharp_array::CSharpArrayTrait<TestStruct> for TestStructArray {
    fn ptr(&self) -> *const TestStruct {
        self.ptr
    }

    fn count(&self) -> i32 {
        self.length
    }
}

impl rust_array::RustArrayTrait<TestStruct> for TestStructArray {
    fn ptr(&self) -> *mut TestStruct {
        self.ptr
    }

    fn count(&self) -> i32 {
        self.length
    }

    fn capacity(&self) -> i32 {
        self.capacity
    }

    fn new(ptr: *mut TestStruct, length: i32, capacity: i32) -> Self {
        Self {
            ptr,
            length,
            capacity
        }
    }

    fn free(self) {
        rust_array::free(self);
    }
}

impl TestStructArray {
    pub unsafe fn to_vec(self) -> Vec<TestStruct> {
        csharp_array::to_vec(self)
    }

    pub unsafe fn from_vec(vec: Vec<TestStruct>) -> TestStructArray {
        rust_array::from_vec(vec)
    }
}
