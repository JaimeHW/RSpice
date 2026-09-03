use num_complex::Complex64;
use rspice_matrix::{
    ComplexMatrix, KluSolver, RealSolverBackend, SolverOptions, StaticMatrix, Value,
};

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut value = self.0;
        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;
        self.0 = value;
        value.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn unit(&mut self) -> Value {
        (self.next() >> 11) as Value / (1_u64 << 53) as Value
    }

    fn signed(&mut self) -> Value {
        2.0 * self.unit() - 1.0
    }
}

fn options(real_backend: RealSolverBackend) -> SolverOptions {
    SolverOptions {
        real_backend,
        ..SolverOptions::default()
    }
}

fn componentwise_backward_error(dense: &[Vec<Value>], rhs: &[Value], solution: &[Value]) -> Value {
    dense
        .iter()
        .zip(rhs)
        .map(|(row, &b)| {
            let ax = row
                .iter()
                .zip(solution)
                .map(|(&a, &x)| a * x)
                .sum::<Value>();
            let denominator = b.abs()
                + row
                    .iter()
                    .zip(solution)
                    .map(|(&a, &x)| a.abs() * x.abs())
                    .sum::<Value>();
            (b - ax).abs() / denominator.max(Value::MIN_POSITIVE)
        })
        .fold(0.0, Value::max)
}

#[test]
fn real_backends_agree_on_random_scaled_sparse_systems() {
    let mut rng = Rng(0x9d7f_4a7c_15e3_8821);

    for trial in 0..120 {
        let n = 2 + rng.next() as usize % 28;
        let row_scale: Vec<Value> = (0..n)
            .map(|_| 10.0_f64.powi((rng.next() % 17) as i32 - 8))
            .collect();
        let col_scale: Vec<Value> = (0..n)
            .map(|_| 10.0_f64.powi((rng.next() % 17) as i32 - 8))
            .collect();

        let mut base = vec![vec![0.0; n]; n];
        for (row, values) in base.iter_mut().enumerate() {
            values[row] = 4.0 + rng.unit();
        }
        for col in 0..n {
            for _ in 0..3 {
                let row = rng.next() as usize % n;
                if row != col {
                    base[row][col] += 0.15 * rng.signed();
                }
            }
        }

        let mut dense = vec![vec![0.0; n]; n];
        let mut triplets = Vec::new();
        for col in 0..n {
            for row in 0..n {
                if base[row][col] != 0.0 {
                    let value = row_scale[row] * base[row][col] * col_scale[col];
                    dense[row][col] = value;
                    triplets.push((row, col, value));
                }
            }
        }

        let expected: Vec<Value> = (0..n).map(|col| rng.signed() / col_scale[col]).collect();
        let rhs: Vec<Value> = dense
            .iter()
            .map(|row| row.iter().zip(&expected).map(|(&a, &x)| a * x).sum())
            .collect();

        let mut klu = StaticMatrix::from_triplets_with_options(
            n,
            n,
            &triplets,
            options(RealSolverBackend::Klu),
        )
        .unwrap();
        let mut faer = StaticMatrix::from_triplets_with_options(
            n,
            n,
            &triplets,
            options(RealSolverBackend::Faer),
        )
        .unwrap();

        let klu_solution = klu
            .solve(&rhs)
            .unwrap_or_else(|error| panic!("KLU policy failed trial {trial}: {error}"));
        let faer_solution = faer
            .solve(&rhs)
            .unwrap_or_else(|error| panic!("faer policy failed trial {trial}: {error}"));

        let tolerance = 64.0 * Value::EPSILON * (n as Value + 1.0);
        assert!(
            componentwise_backward_error(&dense, &rhs, &klu_solution) <= tolerance,
            "KLU policy returned an unstable result on trial {trial}"
        );
        assert!(
            componentwise_backward_error(&dense, &rhs, &faer_solution) <= tolerance,
            "faer policy returned an unstable result on trial {trial}"
        );
        for ((&actual, &reference), &wanted) in
            klu_solution.iter().zip(&faer_solution).zip(&expected)
        {
            let scale = actual.abs().max(reference.abs()).max(wanted.abs()).max(1.0);
            assert!(
                (actual - reference).abs() <= 2.0e-9 * scale,
                "backend disagreement on trial {trial}: {actual} vs {reference}"
            );
        }
    }
}

fn dense_complex_solve(
    mut matrix: Vec<Vec<Complex64>>,
    mut rhs: Vec<Complex64>,
) -> Option<Vec<Complex64>> {
    let n = rhs.len();
    for col in 0..n {
        let pivot = (col..n).max_by(|&left, &right| {
            matrix[left][col]
                .norm_sqr()
                .total_cmp(&matrix[right][col].norm_sqr())
        })?;
        if matrix[pivot][col] == Complex64::new(0.0, 0.0) {
            return None;
        }
        matrix.swap(col, pivot);
        rhs.swap(col, pivot);
        let pivot_value = matrix[col][col];
        let pivot_rhs = rhs[col];
        for row in col + 1..n {
            let factor = matrix[row][col] / pivot_value;
            for index in col..n {
                let pivot_entry = matrix[col][index];
                matrix[row][index] -= factor * pivot_entry;
            }
            rhs[row] -= factor * pivot_rhs;
        }
    }
    for col in (0..n).rev() {
        rhs[col] /= matrix[col][col];
        let solved = rhs[col];
        for row in 0..col {
            rhs[row] -= matrix[row][col] * solved;
        }
    }
    Some(rhs)
}

#[test]
fn complex_sparse_solve_matches_an_independent_dense_reference() {
    let mut rng = Rng(0x42f0_e1eb_a9ea_3693);
    for trial in 0..80 {
        let n = 2 + rng.next() as usize % 10;
        let mut dense = vec![vec![Complex64::new(0.0, 0.0); n]; n];
        for row in 0..n {
            dense[row][row] = Complex64::new(5.0 + rng.unit(), 0.25 * rng.signed());
        }
        for col in 0..n {
            for _ in 0..2 {
                let row = rng.next() as usize % n;
                if row != col {
                    dense[row][col] += Complex64::new(0.2 * rng.signed(), 0.2 * rng.signed());
                }
            }
        }

        let structure: Vec<(usize, usize, Value)> = (0..n)
            .flat_map(|col| {
                let dense = &dense;
                (0..n).filter_map(move |row| {
                    (dense[row][col] != Complex64::new(0.0, 0.0)).then_some((row, col, 0.0))
                })
            })
            .collect();
        let real = StaticMatrix::from_triplets(n, n, &structure).unwrap();
        let mut sparse = ComplexMatrix::from_real_structure(&real);
        for col in 0..n {
            for row in 0..n {
                let value = dense[row][col];
                if value != Complex64::new(0.0, 0.0) {
                    let index = real.get_index(row, col).unwrap();
                    sparse.stamp_direct_real(index, value.re);
                    sparse.stamp_direct_imag(index, value.im);
                }
            }
        }

        let rhs: Vec<Complex64> = (0..n)
            .map(|_| Complex64::new(rng.signed(), rng.signed()))
            .collect();
        let expected = dense_complex_solve(dense, rhs.clone()).unwrap();
        let actual = sparse
            .solve(&rhs)
            .unwrap_or_else(|error| panic!("complex sparse solve failed trial {trial}: {error}"));
        for (&actual, &expected) in actual.iter().zip(&expected) {
            let scale = actual.norm().max(expected.norm()).max(1.0);
            assert!(
                (actual - expected).norm() <= 2.0e-10 * scale,
                "complex disagreement on trial {trial}: {actual} vs {expected}"
            );
        }
    }
}

#[test]
fn arbitrary_public_klu_inputs_never_panic() {
    let mut rng = Rng(0xda94_2042_e4dd_58b5);
    for _ in 0..5_000 {
        let n = rng.next() as usize % 24;
        let pointer_len = rng.next() as usize % 30;
        let row_len = rng.next() as usize % 80;
        let col_ptr: Vec<usize> = (0..pointer_len).map(|_| rng.next() as usize % 90).collect();
        let rows: Vec<usize> = (0..row_len).map(|_| rng.next() as usize % 30).collect();
        let value_len = rng.next() as usize % 90;
        let values: Vec<Value> = (0..value_len)
            .map(|index| {
                if index % 31 == 0 {
                    Value::NAN
                } else {
                    rng.signed()
                }
            })
            .collect();
        let rhs: Vec<Value> = (0..(rng.next() as usize % 30))
            .map(|_| rng.signed())
            .collect();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut klu = KluSolver::new();
            let _ = klu.analyze(n, &col_ptr, &rows);
            let _ = klu.factor(&values);
            let _ = klu.refactor(&values);
            let _ = klu.solve(&rhs, &mut Vec::new());
        }));
        assert!(result.is_ok(), "public KLU API panicked on arbitrary input");
    }
}

#[test]
fn klu_matching_and_refactor_survive_random_row_permutations() {
    let mut rng = Rng(0x7f4a_7c15_9e37_79b9);
    for trial in 0..100 {
        let n = 3 + rng.next() as usize % 23;
        let mut row_permutation: Vec<usize> = (0..n).collect();
        for index in (1..n).rev() {
            let other = rng.next() as usize % (index + 1);
            row_permutation.swap(index, other);
        }

        let mut columns = vec![Vec::<(usize, Value)>::new(); n];
        for col in 0..n {
            columns[col].push((row_permutation[col], 4.0 + rng.unit()));
            let next = (col + 1) % n;
            columns[col].push((row_permutation[next], 0.1 * rng.signed()));
            let previous = (col + n - 1) % n;
            columns[col].push((row_permutation[previous], 0.1 * rng.signed()));
            columns[col].sort_by_key(|(row, _)| *row);
        }

        let mut col_ptr = Vec::with_capacity(n + 1);
        let mut rows = Vec::with_capacity(3 * n);
        let mut values = Vec::with_capacity(3 * n);
        col_ptr.push(0);
        for column in columns {
            for (row, value) in column {
                rows.push(row);
                values.push(value);
            }
            col_ptr.push(rows.len());
        }
        let expected: Vec<Value> = (0..n).map(|_| rng.signed()).collect();
        let mut rhs = vec![0.0; n];
        for col in 0..n {
            for entry in col_ptr[col]..col_ptr[col + 1] {
                rhs[rows[entry]] += values[entry] * expected[col];
            }
        }

        let mut klu = KluSolver::new();
        klu.analyze(n, &col_ptr, &rows)
            .unwrap_or_else(|error| panic!("analysis failed trial {trial}: {error}"));
        klu.factor(&values)
            .unwrap_or_else(|error| panic!("factor failed trial {trial}: {error}"));
        let mut actual = Vec::new();
        klu.solve(&rhs, &mut actual)
            .unwrap_or_else(|error| panic!("solve failed trial {trial}: {error}"));
        for (&actual, &expected) in actual.iter().zip(&expected) {
            assert!(
                (actual - expected).abs() <= 2.0e-9 * expected.abs().max(1.0),
                "factor solution mismatch trial {trial}: {actual} vs {expected}"
            );
        }

        let mut expected_after_refactor = expected.clone();
        for col in 0..n {
            let scale = 0.75 + 0.5 * rng.unit();
            expected_after_refactor[col] /= scale;
            for value in &mut values[col_ptr[col]..col_ptr[col + 1]] {
                *value *= scale;
            }
        }
        if klu.refactor(&values).is_err() {
            klu.factor(&values).unwrap();
        }
        klu.solve(&rhs, &mut actual).unwrap();
        for (&actual, &expected) in actual.iter().zip(&expected_after_refactor) {
            assert!(
                (actual - expected).abs() <= 2.0e-8 * expected.abs().max(1.0),
                "refactor solution mismatch trial {trial}: {actual} vs {expected}"
            );
        }
    }
}
