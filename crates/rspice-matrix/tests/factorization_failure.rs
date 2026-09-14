use rspice_matrix::{RealSolverBackend, SolverError, SolverOptions, StaticMatrix};

fn matrix(entries: &[(usize, usize, f64)]) -> StaticMatrix {
    StaticMatrix::from_triplets_with_options(
        2,
        2,
        entries,
        SolverOptions {
            real_backend: RealSolverBackend::Faer,
            ..SolverOptions::default()
        },
    )
    .unwrap()
}

#[test]
fn empty_patterns_return_singular_errors_before_using_numeric_lu() {
    for transpose in [false, true] {
        let mut single = matrix(&[]);
        let result = if transpose {
            single.solve_transpose(&[0.0, 0.0])
        } else {
            single.solve(&[0.0, 0.0])
        };
        assert!(matches!(result, Err(SolverError::SingularMatrix)));
        let mut batch = matrix(&[]);
        let mut output = vec![];
        let result = if transpose {
            batch.solve_many_transpose_into(&[0.0; 4], 2, &mut output)
        } else {
            batch.solve_many_into(&[0.0; 4], 2, &mut output)
        };
        assert!(matches!(result, Err(SolverError::SingularMatrix)));
    }
}

#[test]
fn a_failed_numeric_factorization_cannot_poison_the_old_value_cache() {
    for transpose in [false, true] {
        for batch in [false, true] {
            let mut matrix = matrix(&[(0, 0, 2.0), (1, 1, 3.0)]);
            let mut output = vec![];
            let mut solve = |matrix: &mut StaticMatrix| {
                if batch {
                    if transpose {
                        matrix.solve_many_transpose_into(&[4.0, 9.0, 4.0, 9.0], 2, &mut output)
                    } else {
                        matrix.solve_many_into(&[4.0, 9.0, 4.0, 9.0], 2, &mut output)
                    }
                } else if transpose {
                    matrix.solve_transpose_into(&[4.0, 9.0], &mut output)
                } else {
                    matrix.solve_into(&[4.0, 9.0], &mut output)
                }
            };
            solve(&mut matrix).unwrap();
            matrix.clear_values();
            assert!(solve(&mut matrix).is_err());
            matrix.add(0, 0, 2.0);
            matrix.add(1, 1, 3.0);
            solve(&mut matrix).unwrap();
            assert_eq!(&output[..2], &[2.0, 3.0]);
        }
    }
}
