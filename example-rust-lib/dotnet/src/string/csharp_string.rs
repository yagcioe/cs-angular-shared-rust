use crate::array::csharp_array;

#[repr(C)]
pub struct CSharpString {
    ptr: *const u16,
    count: i32,
}

impl csharp_array::CSharpArrayTrait<u16> for CSharpString {
    fn ptr(&self) -> *const u16 {
        self.ptr
    }

    fn count(&self) -> i32 {
        self.count
    }
}

impl CSharpString {
    pub unsafe fn to_string(self) -> String {
        let vec: Vec<u16> = csharp_array::to_vec(self);
        String::from_utf16(&vec).unwrap()
    }
}
