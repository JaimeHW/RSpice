use num_complex::Complex64;
use rspice_core::solver::{ComplexMatrix, StaticMatrix};

fn single_entry_real_matrix() -> StaticMatrix {
    StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0)]).expect("valid static matrix")
}

#[test]
#[should_panic(expected = "StaticMatrix::add missing matrix position (0, 1)")]
fn static_matrix_add_panics_for_missing_position() {
    let mut matrix = single_entry_real_matrix();

    matrix.add(0, 1, 2.0);
}

#[test]
#[should_panic(expected = "ComplexMatrix::add_real missing matrix position (0, 1)")]
fn complex_matrix_add_real_panics_for_missing_position() {
    let real_matrix = single_entry_real_matrix();
    let mut matrix = ComplexMatrix::from_real_structure(&real_matrix);

    matrix.add_real(0, 1, 2.0);
}

#[test]
#[should_panic(expected = "ComplexMatrix::add missing matrix position (0, 1)")]
fn complex_matrix_add_panics_for_missing_position() {
    let real_matrix = single_entry_real_matrix();
    let mut matrix = ComplexMatrix::from_real_structure(&real_matrix);

    matrix.add(0, 1, Complex64::new(2.0, 3.0));
}

#[test]
#[should_panic(expected = "ComplexMatrix::add_imag missing matrix position (0, 1)")]
fn complex_matrix_add_imag_panics_for_missing_position() {
    let real_matrix = single_entry_real_matrix();
    let mut matrix = ComplexMatrix::from_real_structure(&real_matrix);

    matrix.add_imag(0, 1, 3.0);
}
