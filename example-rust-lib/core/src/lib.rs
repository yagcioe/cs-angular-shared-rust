pub fn factorial(num: i64) -> i64 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

pub fn add_one(num: i32) -> i32 {
    return num+1;
}