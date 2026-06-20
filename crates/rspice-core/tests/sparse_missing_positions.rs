use num_complex::Complex64;
use rspice_core::solver::{ComplexMatrix, StaticMatrix};

fn single_entry_real_matrix() -> StaticMatrix {
    StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0)]).expect("valid static matrix")
}

#[test]
fn static_matrix_add_reports_missing_position_on_solve() {
    let mut matrix = single_entry_real_matrix();

    matrix.add(0, 1, 2.0);
    let message = matrix.solve(&[1.0, 0.0]).unwrap_err().to_string();

    assert!(
        message.contains("StaticMatrix::add missing matrix position (0, 1)"),
        "unexpected error: {message}"
    );
}

#[test]
fn static_matrix_add_reports_missing_position_on_dense_solve() {
    let mut matrix = single_entry_real_matrix();

    matrix.add(0, 1, 2.0);
    let message = matrix.solve_dense(&[1.0, 0.0]).unwrap_err().to_string();

    assert!(
        message.contains("StaticMatrix::add missing matrix position (0, 1)"),
        "unexpected error: {message}"
    );
}

#[test]
fn complex_matrix_add_real_reports_missing_position_on_solve() {
    let real_matrix = single_entry_real_matrix();
    let mut matrix = ComplexMatrix::from_real_structure(&real_matrix);

    matrix.add_real(0, 1, 2.0);
    let message = matrix
        .solve(&[Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)])
        .unwrap_err()
        .to_string();

    assert!(
        message.contains("ComplexMatrix::add_real missing matrix position (0, 1)"),
        "unexpected error: {message}"
    );
}

#[test]
fn complex_matrix_add_reports_missing_position_on_solve() {
    let real_matrix = single_entry_real_matrix();
    let mut matrix = ComplexMatrix::from_real_structure(&real_matrix);

    matrix.add(0, 1, Complex64::new(2.0, 3.0));
    let message = matrix
        .solve(&[Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)])
        .unwrap_err()
        .to_string();

    assert!(
        message.contains("ComplexMatrix::add missing matrix position (0, 1)"),
        "unexpected error: {message}"
    );
}

#[test]
fn complex_matrix_add_imag_reports_missing_position_on_solve() {
    let real_matrix = single_entry_real_matrix();
    let mut matrix = ComplexMatrix::from_real_structure(&real_matrix);

    matrix.add_imag(0, 1, 3.0);
    let message = matrix
        .solve(&[Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)])
        .unwrap_err()
        .to_string();

    assert!(
        message.contains("ComplexMatrix::add_imag missing matrix position (0, 1)"),
        "unexpected error: {message}"
    );
}
