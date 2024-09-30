use array::rust_array::RustArrayTrait;
use string::{csharp_string::CSharpString, rust_string::RustString};
use test_case::{TestStruct, TestStructArray};

#[allow(dead_code)]
mod array;

#[allow(dead_code)]
mod string;

#[allow(dead_code)]
mod test_case;

#[no_mangle]
pub extern "C" fn add_one(num: i32) -> i32 {
    core::add_one(num)
}

#[no_mangle]
pub extern "C" fn alloc_u8_string() -> RustString {
    let str = format!("foo bar baz");
    let buf = RustString::from_string(str);
    buf
}

#[no_mangle]
pub unsafe extern "C" fn free_u8_string(buffer: RustString) {
    buffer.free();
}

#[no_mangle]
pub unsafe extern "C" fn get_csharp_string(cs_string: CSharpString) -> RustString {
    let mut str = cs_string.to_string();
    str.insert(0, 'c');

    RustString::from_string(str)
}

#[no_mangle]
pub unsafe extern "C" fn get_test_mut(mut test: TestStruct) ->  TestStruct {
    test.key += 1;

    test
}

#[no_mangle]
pub unsafe extern "C" fn get_test_mut_arr(test: TestStructArray) ->  TestStructArray {
    let mut arr = test.to_vec();
    arr.push(TestStruct::default());

    TestStructArray::from_vec(arr)
}

#[no_mangle]
pub unsafe extern "C" fn free_test_arr(test: TestStructArray) {
    test.free();
}
