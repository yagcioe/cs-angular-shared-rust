pub trait CSharpArrayTrait<T> {
    fn ptr(&self) -> *const T;
    fn count(&self) -> i32;
}

pub unsafe fn to_vec<TValue: Clone, TArray: CSharpArrayTrait<TValue>>(arr: TArray) -> Vec<TValue> 
{
    let len = usize::try_from(arr.count()).expect("i32 does not fit into usize");
    std::slice::from_raw_parts(arr.ptr(), len)
        .to_vec()
}
