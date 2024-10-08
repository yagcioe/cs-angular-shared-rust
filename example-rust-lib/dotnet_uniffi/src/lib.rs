use std::time::Duration;
// You must call this once
uniffi::setup_scaffolding!();

// What follows is an intentionally ridiculous whirlwind tour of how you'd expose a complex API to UniFFI.

#[derive(Copy, Clone, Debug, PartialEq, uniffi::Record)]
pub struct ComputationResult {
    pub value: i64,
    pub computation_time: Duration,
}

impl ComputationResult {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            computation_time: Duration::from_millis(1000),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, uniffi::Record)]
pub struct ComputationRequest {
    pub value: i64,
}

#[uniffi::export]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[uniffi::export]
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[uniffi::export]
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

#[uniffi::export]
pub fn test(num: i32) -> i32 {
    core::add_one(num)
}

#[uniffi::export]
pub fn test_str(mut num: String) -> String {
    num.push('c');
    num
}

#[uniffi::export]
pub fn test_panic() {
    panic!("lol")
}

#[uniffi::export]
pub fn factorial(num: i64) -> i64 {
    core::factorial(num)
}

#[uniffi::export]
pub fn last(num: Vec<i64>) -> i64 {
    *(num.last().unwrap())
}

#[uniffi::export]
pub fn valid(value: String) -> bool {
    core::validate(value)
}

#[uniffi::export]
pub fn compute(value: ComputationRequest) -> ComputationResult {
    ComputationResult::new(value.value)
}

#[uniffi::export]
pub fn compute_all(value: Vec<ComputationRequest>) -> Vec<ComputationResult> {
    let result = value.iter().map(|value|{ComputationResult::new(value.value)}).collect();
    result
}

#[uniffi::export]
pub fn get_factorial(num: i64) -> String {
    let mut f: i64 = 0;
    for _ in 0..10000000 {
        f = core::factorial(num);
    }
    f.to_string()
}
