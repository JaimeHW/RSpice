//! The linearized period's monodromy, reduced once to Hessenberg form.
//!
//! Every frequency's periodicity solve is `(ρI − M)·x = r`, or its transpose
//! for the adjoint, with `ρ = e^{jωT}`. `M` is dense: one period couples every
//! state. Householder reflections write it once as `M = Q·H·Qᵀ`, `H` upper
//! Hessenberg and `Q` orthogonal, so each solve is `(ρI − H)·y = Qᵀ·r`,
//! `x = Q·y`: elimination with adjacent-row pivoting on `H`, O(n²) per
//! frequency in place of a dense O(n³) factorization.

use crate::{Complex64, Value};

/// `M = Q·H·Qᵀ`.
#[derive(Default)]
pub(super) struct Monodromy {
    n: usize,
    /// `H`, row-major.
    hessenberg: Vec<Value>,
    /// `Q`'s reflectors `I − β·v·vᵀ` in the order they were applied, the
    /// k-th acting on rows `k + 1..`; an empty `v` is the identity.
    reflectors: Vec<(Vec<Value>, Value)>,
}

impl Monodromy {
    /// Reduce `matrix`, `n × n` row-major.
    pub(super) fn reduce(mut matrix: Vec<Value>, n: usize) -> Self {
        let mut reflectors = Vec::with_capacity(n.saturating_sub(2));
        for k in 0..n.saturating_sub(2) {
            let mut v: Vec<Value> = (k + 1..n).map(|row| matrix[row * n + k]).collect();
            let norm = v.iter().map(|value| value * value).sum::<Value>().sqrt();
            let alpha = if v[0] >= 0.0 { -norm } else { norm };
            v[0] -= alpha;
            let length = v.iter().map(|value| value * value).sum::<Value>();
            if length == 0.0 {
                reflectors.push((Vec::new(), 0.0));
                continue;
            }
            let beta = 2.0 / length;
            for col in k..n {
                let dot: Value = v
                    .iter()
                    .enumerate()
                    .map(|(i, value)| value * matrix[(k + 1 + i) * n + col])
                    .sum();
                let scale = beta * dot;
                if scale != 0.0 {
                    for (i, value) in v.iter().enumerate() {
                        matrix[(k + 1 + i) * n + col] -= scale * value;
                    }
                }
            }
            for row in 0..n {
                let span = &mut matrix[row * n + k + 1..(row + 1) * n];
                let dot: Value = v.iter().zip(span.iter()).map(|(a, b)| a * b).sum();
                let scale = beta * dot;
                if scale != 0.0 {
                    for (entry, value) in span.iter_mut().zip(&v) {
                        *entry -= scale * value;
                    }
                }
            }
            matrix[(k + 1) * n + k] = alpha;
            for row in k + 2..n {
                matrix[row * n + k] = 0.0;
            }
            reflectors.push((v, beta));
        }
        Self {
            n,
            hessenberg: matrix,
            reflectors,
        }
    }

    /// Solve `(ρI − M)·x = rhs`, or `(ρI − Mᵀ)·x = rhs` when `transposed`;
    /// `None` when `ρ` is a multiplier of `M` to working precision.
    pub(super) fn solve(
        &self,
        rotation: Complex64,
        mut rhs: Vec<Complex64>,
        transposed: bool,
    ) -> Option<Vec<Complex64>> {
        let n = self.n;
        let zero = Complex64::new(0.0, 0.0);
        for (k, (v, beta)) in self.reflectors.iter().enumerate() {
            reflect(&mut rhs[k + 1..], v, *beta);
        }
        let mut u: Vec<Complex64> = self
            .hessenberg
            .iter()
            .map(|value| Complex64::new(-value, 0.0))
            .collect();
        for k in 0..n {
            u[k * n + k] += rotation;
        }
        let scale = u.iter().fold(0.0, |m: Value, v| m.max(v.norm()));
        let floor = 1.0e-14 * scale.max(Value::MIN_POSITIVE);
        let mut swaps = vec![false; n];
        let mut multipliers = vec![zero; n];
        for k in 0..n.saturating_sub(1) {
            if u[(k + 1) * n + k].norm() > u[k * n + k].norm() {
                for col in k..n {
                    u.swap(k * n + col, (k + 1) * n + col);
                }
                swaps[k] = true;
            }
            if u[k * n + k].norm() <= floor {
                return None;
            }
            let multiplier = u[(k + 1) * n + k] / u[k * n + k];
            multipliers[k] = multiplier;
            u[(k + 1) * n + k] = zero;
            if multiplier != zero {
                for col in k + 1..n {
                    let value = u[k * n + col];
                    u[(k + 1) * n + col] -= multiplier * value;
                }
            }
        }
        if u[n * n - 1].norm() <= floor {
            return None;
        }
        if transposed {
            for row in 0..n {
                let mut value = rhs[row];
                for col in 0..row {
                    value -= u[col * n + row] * rhs[col];
                }
                rhs[row] = value / u[row * n + row];
            }
            for k in (0..n.saturating_sub(1)).rev() {
                let value = rhs[k + 1];
                rhs[k] -= multipliers[k] * value;
                if swaps[k] {
                    rhs.swap(k, k + 1);
                }
            }
        } else {
            for k in 0..n.saturating_sub(1) {
                if swaps[k] {
                    rhs.swap(k, k + 1);
                }
                let value = rhs[k];
                rhs[k + 1] -= multipliers[k] * value;
            }
            for row in (0..n).rev() {
                let mut value = rhs[row];
                for col in row + 1..n {
                    value -= u[row * n + col] * rhs[col];
                }
                rhs[row] = value / u[row * n + row];
            }
        }
        for (k, (v, beta)) in self.reflectors.iter().enumerate().rev() {
            reflect(&mut rhs[k + 1..], v, *beta);
        }
        Some(rhs)
    }
}

/// `x ← (I − β·v·vᵀ)·x`.
fn reflect(x: &mut [Complex64], v: &[Value], beta: Value) {
    if v.is_empty() {
        return;
    }
    let dot: Complex64 = v.iter().zip(x.iter()).map(|(a, b)| *b * *a).sum();
    let scale = dot * beta;
    for (entry, value) in x.iter_mut().zip(v) {
        *entry -= scale * *value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `(ρI − M)·x` and `(ρI − Mᵀ)·x` for a dense row-major `M`.
    fn apply(
        matrix: &[Value],
        n: usize,
        rotation: Complex64,
        x: &[Complex64],
        transposed: bool,
    ) -> Vec<Complex64> {
        (0..n)
            .map(|row| {
                let mut value = rotation * x[row];
                for col in 0..n {
                    let entry = if transposed {
                        matrix[col * n + row]
                    } else {
                        matrix[row * n + col]
                    };
                    value -= x[col] * entry;
                }
                value
            })
            .collect()
    }

    #[test]
    fn both_periodicity_solves_invert_their_systems() {
        let n = 7;
        let matrix: Vec<Value> = (0..n * n)
            .map(|index| {
                let (row, col) = (index / n, index % n);
                ((row * 5 + col * 3) % 11) as Value / 13.0 - 0.3
                    + if row == col { 0.2 } else { 0.0 }
            })
            .collect();
        let reduced = Monodromy::reduce(matrix.clone(), n);
        let rotation = Complex64::from_polar(1.0, 0.7);
        let rhs: Vec<Complex64> = (0..n)
            .map(|row| Complex64::new(row as Value - 2.0, 0.5 * row as Value))
            .collect();
        for transposed in [false, true] {
            let x = reduced
                .solve(rotation, rhs.clone(), transposed)
                .expect("regular");
            let back = apply(&matrix, n, rotation, &x, transposed);
            for (a, b) in back.iter().zip(&rhs) {
                assert!((a - b).norm() < 1.0e-12, "{a} against {b}");
            }
        }
    }

    #[test]
    fn a_multiplier_on_the_unit_circle_is_refused() {
        // A rotation by 0.7 rad: its multipliers are e^{±0.7j}.
        let (c, s) = (0.7_f64.cos(), 0.7_f64.sin());
        let matrix = vec![c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 0.5];
        let reduced = Monodromy::reduce(matrix, 3);
        let rhs = vec![Complex64::new(1.0, 0.0); 3];
        assert!(
            reduced
                .solve(Complex64::from_polar(1.0, 0.7), rhs, false)
                .is_none()
        );
    }
}
