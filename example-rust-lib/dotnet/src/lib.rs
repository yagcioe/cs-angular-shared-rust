#[no_mangle]
pub extern "C" fn factorial_cs(num: i32) -> u64 {
    let mut f: u128 = 0;
    for _ in 0..10000000 {
        f = core::factorial(num as u128);
    }
    f as u64
}

#[no_mangle]
pub extern "C" fn add_one(num: i32) -> i32 {
    core::add_one(num)
}