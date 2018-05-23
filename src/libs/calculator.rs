#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub fn substract(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[no_mangle]
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

#[no_mangle]
pub fn power(a: i32, b: i32) -> i32 {
    a ^ b
}

#[no_mangle]
pub fn remainder(a: i32, b: i32) -> i32 {
    a % b
}