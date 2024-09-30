pub trait RustArrayTrait<T> {
    fn ptr(&self) -> *mut T;
    fn count(&self) -> i32;
    fn capacity(&self) -> i32;
    fn new(ptr: *mut T, length: i32, capacity: i32) -> Self;
    fn free(self);
}

pub fn from_vec<TValue, TArray: RustArrayTrait<TValue>>(mut vec: Vec<TValue>) -> TArray {
    let length = i32::try_from(vec.len()).expect("usize does not fit into i32");
    let capacity =
        i32::try_from(vec.capacity()).expect("usize does not fit into i32");

    TArray::new(vec.as_mut_ptr(), length, capacity)
}

fn destroy_into_vec<TValue, TArray: RustArrayTrait<TValue>>(arr: TArray) -> Vec<TValue> {
    if arr.ptr().is_null() {
        vec![]
    } else {
        let len = usize::try_from(arr.count()).expect("i32 does not fit into usize");
        let capacity = usize::try_from(arr.capacity()).expect("i32 does not fit into usize");
        unsafe { Vec::from_raw_parts(arr.ptr(), len, capacity) }
    }
}

pub fn free<TValue, TArray: RustArrayTrait<TValue>>(arr: TArray) {
    drop(destroy_into_vec(arr));
}
