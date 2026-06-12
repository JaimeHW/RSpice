//! KLU solver micro-benchmark: factor / refactor / solve in isolation on
//! circuit-shaped matrices, plus fill statistics per ordering.
//!
//! Run: `cargo run --release -p rspice-core --example klu_bench`
//!
//! The macro rig (rspice-bench) measures whole processes; this isolates
//! the solver kernels so optimizations are attributable. Matrices mirror
//! the bench decks: a banded RC-ladder pattern and a denser ring-like
//! pattern with off-diagonal couplings, both diagonally dominant with a
//! Newton-style value drift between refactors.

use std::time::Instant;

use rspice_core::Value;
use rspice_core::solver::klu::KluSolver;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut v = self.0;
        v ^= v >> 12;
        v ^= v << 25;
        v ^= v >> 27;
        self.0 = v;
        v.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn unit(&mut self) -> Value {
        (self.next() >> 11) as Value / (1u64 << 53) as Value
    }
}

/// CSC builder from dense triplets.
struct Csc {
    n: usize,
    col_ptr: Vec<usize>,
    row_idx: Vec<usize>,
    values: Vec<Value>,
}

/// Ladder-like pattern: tridiagonal plus a ground-ish coupling every K
/// rows (mimics the RC ladder MNA with a source branch).
fn ladder_matrix(n: usize, rng: &mut Rng) -> Csc {
    let mut col_ptr = vec![0usize];
    let mut row_idx = Vec::new();
    let mut values = Vec::new();
    for j in 0..n {
        for i in j.saturating_sub(1)..=(j + 1).min(n - 1) {
            row_idx.push(i);
            values.push(if i == j {
                2.0 + rng.unit()
            } else {
                -(0.5 + 0.5 * rng.unit())
            });
        }
        if j % 16 == 0 && j + 8 < n {
            row_idx.push(j + 8);
            values.push(-0.1);
        }
        col_ptr.push(row_idx.len());
    }
    Csc {
        n,
        col_ptr,
        row_idx,
        values,
    }
}

/// Build CSC from per-column coupling lists (diagonal added).
fn from_couplings(n: usize, couplings: Vec<Vec<usize>>, rng: &mut Rng) -> Csc {
    let mut dense_cols: Vec<Vec<(usize, Value)>> = vec![Vec::new(); n];
    for (j, list) in couplings.into_iter().enumerate() {
        dense_cols[j].push((j, 4.0 + rng.unit()));
        for i in list {
            if i != j {
                dense_cols[j].push((i, -(0.2 + 0.6 * rng.unit())));
                dense_cols[i].push((j, -(0.2 + 0.6 * rng.unit())));
            }
        }
    }
    let mut col_ptr = vec![0usize];
    let mut row_idx = Vec::new();
    let mut values = Vec::new();
    for col in dense_cols.iter_mut() {
        col.sort_by_key(|(i, _)| *i);
        col.dedup_by_key(|(i, _)| *i);
        for &(i, v) in col.iter() {
            row_idx.push(i);
            values.push(v);
        }
        col_ptr.push(row_idx.len());
    }
    Csc {
        n,
        col_ptr,
        row_idx,
        values,
    }
}

/// Ring-oscillator-like pattern: stages couple to their neighbors (with
/// the wrap-around edge) plus a shared semi-dense supply rail — local
/// structure like real circuit matrices, not a random expander.
fn ring_osc_matrix(n: usize, rng: &mut Rng) -> Csc {
    let mut couplings: Vec<Vec<usize>> = vec![Vec::new(); n];
    for j in 1..n {
        couplings[j].push(j - 1);
        couplings[j].push((j + 1) % n);
        if j % 3 == 0 {
            couplings[j].push(0); // node 0 = supply rail
        }
    }
    from_couplings(n, couplings, rng)
}

/// Random expander: worst-case fill under *any* ordering (kept as the
/// pathological reference row; real circuits are local, not expanders).
fn expander_matrix(n: usize, rng: &mut Rng) -> Csc {
    let mut couplings: Vec<Vec<usize>> = vec![Vec::new(); n];
    for j in 0..n {
        for _ in 0..4 {
            couplings[j].push((rng.next() as usize) % n);
        }
    }
    from_couplings(n, couplings, rng)
}

fn drift(values: &mut [Value], rng: &mut Rng) {
    for v in values.iter_mut() {
        *v *= 1.0 + 0.02 * (rng.unit() - 0.5);
    }
}

fn bench_case(label: &str, mut m: Csc, refactors: usize) {
    let mut rng = Rng(0xBEEF ^ m.n as u64);
    let mut klu = KluSolver::new();

    let t0 = Instant::now();
    klu.analyze(m.n, &m.col_ptr, &m.row_idx);
    let t_analyze = t0.elapsed();

    let t0 = Instant::now();
    klu.factor(&m.col_ptr, &m.row_idx, &m.values)
        .expect("factor");
    let t_factor = t0.elapsed();

    // Warm refactor loop with Newton-style drift.
    let t0 = Instant::now();
    for _ in 0..refactors {
        drift(&mut m.values, &mut rng);
        match klu.refactor(&m.col_ptr, &m.row_idx, &m.values) {
            Ok(()) => {}
            Err(_) => klu
                .factor(&m.col_ptr, &m.row_idx, &m.values)
                .expect("re-factor"),
        }
    }
    let t_refactor = t0.elapsed();

    let b: Vec<Value> = (0..m.n).map(|i| ((i % 7) as Value) - 3.0).collect();
    let mut out = Vec::new();
    let t0 = Instant::now();
    for _ in 0..refactors {
        klu.solve(&b, &mut out).expect("solve");
    }
    let t_solve = t0.elapsed();

    let (l_nnz, u_nnz) = klu.factor_nnz();
    println!(
        "{label:<14} n={:<6} a_nnz={:<8} l+u_nnz={:<8} analyze={:>8.2?} factor={:>8.2?} refactor={:>9.3?}/iter solve={:>9.3?}/iter",
        m.n,
        m.values.len(),
        l_nnz + u_nnz,
        t_analyze,
        t_factor,
        t_refactor / refactors as u32,
        t_solve / refactors as u32,
    );
}

fn main() {
    let mut rng = Rng(7);
    println!("== KLU kernel micro-benchmark ==");
    for &n in &[100usize, 1000, 10000] {
        bench_case("ladder", ladder_matrix(n, &mut rng), 400);
    }
    for &n in &[100usize, 1000, 10000] {
        bench_case("ring_osc", ring_osc_matrix(n, &mut rng), 400);
    }
    bench_case("expander", expander_matrix(1000, &mut rng), 50);
}
