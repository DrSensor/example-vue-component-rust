extern crate nalgebra as na;

use na::{DMatrix};

#[no_mangle]
pub fn add(a: f32, b: f32) -> f32 {
    let matrix_a = DMatrix::from_diagonal_element(2, 2, a);
    let matrix_b = DMatrix::from_diagonal_element(2, 2, b);
    (matrix_a + matrix_b).determinant()
}

#[no_mangle]
pub fn substract(a: f32, b: f32) -> f32 {
    let matrix_a = DMatrix::from_diagonal_element(2, 2, a);
    let matrix_b = DMatrix::from_diagonal_element(2, 2, b);
    (matrix_a - matrix_b).determinant()
}

#[no_mangle]
pub fn multiply(a: f32, b: f32) -> f32 {
    let matrix_a = DMatrix::from_diagonal_element(2, 2, a);
    let matrix_b = DMatrix::from_diagonal_element(2, 2, b);
    (matrix_a * matrix_b).determinant()
}

#[no_mangle]
pub fn dot(a: f32, b: f32) -> f32 {
    let matrix_a = DMatrix::from_diagonal_element(2, 2, a);
    let matrix_b = DMatrix::from_diagonal_element(2, 2, b);
    matrix_a.dot(&matrix_b)
}

#[no_mangle]
pub fn tensor(a: f32, b: f32) -> f32 {
    let matrix_a = DMatrix::from_diagonal_element(2, 2, a);
    let matrix_b = DMatrix::from_diagonal_element(2, 2, b);
    matrix_a.kronecker(&matrix_b).determinant()
}