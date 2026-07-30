//! In-process KLU solver-kernel benchmark gate.
//!
//! This subcommand complements the process-level `run` benchmark. It times
//! `analyze` / `factor` / `refactor` / `solve` in isolation on circuit-shaped
//! matrices so solver optimizations are attributable to a phase, and reports
//! fill (`(L+U) nnz / A nnz`) per case.
//!
//! Matrices mirror the bench decks: a banded RC-ladder pattern and a denser
//! ring-like pattern with off-diagonal couplings, both diagonally dominant
//! with a Newton-style value drift between refactors. A random expander is
//! measured as the pathological reference row and never gated -- real circuit
//! matrices are local, not expanders.
//!
//! Budgets are normalized per `(L+U) nnz` rather than absolute per-iteration,
//! because a sparse direct solve is proportional to factor nonzeros: one
//! threshold then covers the whole size sweep. Every budget is off unless
//! passed explicitly, so the gate cannot fail before baselines exist.

use crate::error::BenchError;
use clap::Args;
use rspice_matrix::{KluSolver, Value};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

/// Arguments for the `klu` subcommand.
#[derive(Args, Debug)]
pub struct KluArgs {
    /// Matrix dimensions swept for each circuit-shaped pattern.
    #[arg(long, value_delimiter = ',', default_values_t = vec![100usize, 1_000, 10_000])]
    pub sizes: Vec<usize>,

    /// Refactor and solve iterations per timed sample, circuit-shaped cases.
    #[arg(long, default_value_t = 400)]
    pub refactors: usize,

    /// Dimension of the pathological random-expander reference case.
    #[arg(long, default_value_t = 1_000)]
    pub expander_size: usize,

    /// Refactor and solve iterations per timed sample, expander case.
    #[arg(long, default_value_t = 50)]
    pub expander_refactors: usize,

    /// Timed samples per case; the report gates on the median, not the fastest.
    #[arg(long, default_value_t = 7)]
    pub samples: usize,

    /// Refactor budget in ns per (L+U) nonzero. Off unless set.
    #[arg(long, value_name = "NS")]
    pub max_refactor_ns_per_lu_nnz: Option<f64>,

    /// Solve budget in ns per (L+U) nonzero. Off unless set.
    #[arg(long, value_name = "NS")]
    pub max_solve_ns_per_lu_nnz: Option<f64>,

    /// Fill budget as `(L+U) nnz / A nnz`. Off unless set.
    #[arg(long, value_name = "RATIO")]
    pub max_fill_ratio: Option<f64>,

    /// Optional JSON report path.
    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,
}

/// Whole-run KLU kernel report.
#[derive(Debug, Serialize)]
pub struct KluReport {
    /// Timed samples per case.
    pub samples: usize,
    /// Refactor budget in effect, if any.
    pub max_refactor_ns_per_lu_nnz: Option<f64>,
    /// Solve budget in effect, if any.
    pub max_solve_ns_per_lu_nnz: Option<f64>,
    /// Fill budget in effect, if any.
    pub max_fill_ratio: Option<f64>,
    /// One entry per measured pattern and size.
    pub cases: Vec<KluCase>,
    /// False when any gated case exceeded a budget.
    pub passed: bool,
}

/// Per-case measurement. Times are medians over the sample count.
#[derive(Debug, Serialize)]
pub struct KluCase {
    /// Pattern label.
    pub name: String,
    /// Matrix dimension.
    pub n: usize,
    /// Structural nonzeros in `A`.
    pub a_nnz: usize,
    /// Nonzeros in the `L` and `U` factors combined.
    pub lu_nnz: usize,
    /// `lu_nnz / a_nnz`.
    pub fill_ratio: f64,
    /// Symbolic analysis, ns.
    pub analyze_ns: f64,
    /// First numeric factorization, ns.
    pub factor_ns: f64,
    /// Refactor, ns per iteration (drift excluded from the timed region).
    pub refactor_ns_per_iter: f64,
    /// Triangular solve, ns per iteration.
    pub solve_ns_per_iter: f64,
    /// Refactor normalized by factor nonzeros -- the gated quantity.
    pub refactor_ns_per_lu_nnz: f64,
    /// Solve normalized by factor nonzeros -- the gated quantity.
    pub solve_ns_per_lu_nnz: f64,
    /// Refactor and solve iterations behind each timed sample.
    pub refactors: usize,
    /// Pathological reference cases are measured but exempt from budgets.
    pub gated: bool,
    /// True when the case is ungated or met every budget.
    pub passed: bool,
    /// Which budget was exceeded, when `passed` is false.
    pub failure: Option<String>,
}

/// Deterministic xorshift, so a case is byte-identical between runs.
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

/// Seeds a case from its own identity rather than from sweep position, so
/// changing `--sizes` cannot silently change another case's matrix.
fn case_seed(label: &str, n: usize) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in label.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash ^= n as u64;
    hash.wrapping_mul(0x0000_0100_0000_01b3)
}

/// Compressed-sparse-column matrix with a pristine value vector.
struct Csc {
    n: usize,
    col_ptr: Vec<usize>,
    row_idx: Vec<usize>,
    values: Vec<Value>,
}

/// Ladder-like pattern: tridiagonal plus a ground-ish coupling every 16 rows
/// (mimics the RC ladder MNA with a source branch).
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

/// Builds CSC from per-column coupling lists, adding the diagonal.
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

/// Ring-oscillator-like pattern: stages couple to their neighbors (with the
/// wrap-around edge) plus a shared semi-dense supply rail -- local structure
/// like real circuit matrices, not a random expander.
fn ring_osc_matrix(n: usize, rng: &mut Rng) -> Csc {
    let mut couplings: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (j, coupling) in couplings.iter_mut().enumerate().take(n).skip(1) {
        coupling.push(j - 1);
        coupling.push((j + 1) % n);
        if j % 3 == 0 {
            coupling.push(0); // node 0 = supply rail
        }
    }
    from_couplings(n, couplings, rng)
}

/// Random expander: worst-case fill under *any* ordering, kept as the
/// pathological reference row.
fn expander_matrix(n: usize, rng: &mut Rng) -> Csc {
    let mut couplings: Vec<Vec<usize>> = vec![Vec::new(); n];
    for coupling in couplings.iter_mut().take(n) {
        for _ in 0..4 {
            coupling.push((rng.next() as usize) % n);
        }
    }
    from_couplings(n, couplings, rng)
}

/// Newton-style value drift between refactors.
fn drift(values: &mut [Value], rng: &mut Rng) {
    for v in values.iter_mut() {
        *v *= 1.0 + 0.02 * (rng.unit() - 0.5);
    }
}

/// Median of a non-empty sample set.
fn median(samples: &mut [f64]) -> f64 {
    samples.sort_by(f64::total_cmp);
    let mid = samples.len() / 2;
    if samples.len().is_multiple_of(2) {
        (samples[mid - 1] + samples[mid]) / 2.0
    } else {
        samples[mid]
    }
}

fn elapsed_ns(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1e9
}

/// One pattern at one size: `samples` independent repeats, each from the
/// pristine values so accumulated drift cannot bias later samples.
fn bench_case(
    label: &str,
    matrix: &Csc,
    refactors: usize,
    samples: usize,
    gated: bool,
) -> Result<KluCase, BenchError> {
    let mut analyze = Vec::with_capacity(samples);
    let mut factor = Vec::with_capacity(samples);
    let mut refactor = Vec::with_capacity(samples);
    let mut solve = Vec::with_capacity(samples);
    let mut lu_nnz = 0usize;

    let rhs: Vec<Value> = (0..matrix.n).map(|i| ((i % 7) as Value) - 3.0).collect();
    let mut out = Vec::new();

    for _ in 0..samples {
        let mut values = matrix.values.clone();
        let mut rng = Rng(case_seed(label, matrix.n) ^ 0x5EED);
        let mut klu = KluSolver::new();

        let start = Instant::now();
        klu.analyze(matrix.n, &matrix.col_ptr, &matrix.row_idx);
        analyze.push(elapsed_ns(start));

        let start = Instant::now();
        klu.factor(&matrix.col_ptr, &matrix.row_idx, &values)
            .map_err(|source| BenchError::Klu {
                message: format!("{label} n={}: initial factor failed: {source}", matrix.n),
            })?;
        factor.push(elapsed_ns(start));

        // Time the refactor calls only -- drift is test-fixture work, not
        // solver work, and at these sizes it is a measurable fraction.
        let mut refactor_ns = 0.0;
        for _ in 0..refactors {
            drift(&mut values, &mut rng);
            let start = Instant::now();
            let outcome = klu.refactor(&matrix.col_ptr, &matrix.row_idx, &values);
            refactor_ns += elapsed_ns(start);
            if outcome.is_err() {
                // A refactor that rejects the drifted values falls back to a
                // full factor, outside the timed region.
                klu.factor(&matrix.col_ptr, &matrix.row_idx, &values)
                    .map_err(|source| BenchError::Klu {
                        message: format!("{label} n={}: re-factor failed: {source}", matrix.n),
                    })?;
            }
        }
        refactor.push(refactor_ns / refactors as f64);

        let start = Instant::now();
        for _ in 0..refactors {
            klu.solve(&rhs, &mut out)
                .map_err(|source| BenchError::Klu {
                    message: format!("{label} n={}: solve failed: {source}", matrix.n),
                })?;
        }
        solve.push(elapsed_ns(start) / refactors as f64);

        let (l_nnz, u_nnz) = klu.factor_nnz();
        lu_nnz = l_nnz + u_nnz;
    }

    let a_nnz = matrix.values.len();
    let refactor_ns_per_iter = median(&mut refactor);
    let solve_ns_per_iter = median(&mut solve);
    let per_lu_nnz = |ns: f64| if lu_nnz == 0 { 0.0 } else { ns / lu_nnz as f64 };

    Ok(KluCase {
        name: label.to_owned(),
        n: matrix.n,
        a_nnz,
        lu_nnz,
        fill_ratio: if a_nnz == 0 {
            0.0
        } else {
            lu_nnz as f64 / a_nnz as f64
        },
        analyze_ns: median(&mut analyze),
        factor_ns: median(&mut factor),
        refactor_ns_per_iter,
        solve_ns_per_iter,
        refactor_ns_per_lu_nnz: per_lu_nnz(refactor_ns_per_iter),
        solve_ns_per_lu_nnz: per_lu_nnz(solve_ns_per_iter),
        refactors,
        gated,
        passed: true,
        failure: None,
    })
}

/// Applies the budgets that were set, leaving ungated cases untouched.
fn apply_budgets(case: &mut KluCase, args: &KluArgs) {
    if !case.gated {
        return;
    }
    let mut failures = Vec::new();
    if let Some(budget) = args.max_refactor_ns_per_lu_nnz
        && case.refactor_ns_per_lu_nnz > budget
    {
        failures.push(format!(
            "refactor {:.4} ns/lu_nnz exceeds budget {budget:.4}",
            case.refactor_ns_per_lu_nnz
        ));
    }
    if let Some(budget) = args.max_solve_ns_per_lu_nnz
        && case.solve_ns_per_lu_nnz > budget
    {
        failures.push(format!(
            "solve {:.4} ns/lu_nnz exceeds budget {budget:.4}",
            case.solve_ns_per_lu_nnz
        ));
    }
    if let Some(budget) = args.max_fill_ratio
        && case.fill_ratio > budget
    {
        failures.push(format!(
            "fill {:.3}x exceeds budget {budget:.3}x",
            case.fill_ratio
        ));
    }
    if !failures.is_empty() {
        case.passed = false;
        case.failure = Some(failures.join("; "));
    }
}

pub fn run(args: &KluArgs) -> Result<ExitCode, BenchError> {
    if args.samples == 0 {
        return Err(BenchError::BenchmarkPolicy {
            message: "--samples must be at least 1; a median needs a sample".into(),
        });
    }
    if args.refactors == 0 || args.expander_refactors == 0 {
        return Err(BenchError::BenchmarkPolicy {
            message: "--refactors and --expander-refactors must be at least 1".into(),
        });
    }
    if let Some(&small) = args.sizes.iter().chain([&args.expander_size]).min()
        && small < 2
    {
        return Err(BenchError::BenchmarkPolicy {
            message: format!("matrix dimensions must be at least 2; got {small}"),
        });
    }

    // (label, n, refactors, gated)
    let mut plan: Vec<(&str, usize, usize, bool)> = Vec::new();
    for &n in &args.sizes {
        plan.push(("ladder", n, args.refactors, true));
    }
    for &n in &args.sizes {
        plan.push(("ring_osc", n, args.refactors, true));
    }
    plan.push((
        "expander",
        args.expander_size,
        args.expander_refactors,
        false,
    ));

    let mut cases = Vec::with_capacity(plan.len());
    for (label, n, refactors, gated) in plan {
        let mut rng = Rng(case_seed(label, n));
        let matrix = match label {
            "ladder" => ladder_matrix(n, &mut rng),
            "ring_osc" => ring_osc_matrix(n, &mut rng),
            "expander" => expander_matrix(n, &mut rng),
            _ => return Err(BenchError::Internal("unknown KLU benchmark pattern")),
        };
        let mut case = bench_case(label, &matrix, refactors, args.samples, gated)?;
        apply_budgets(&mut case, args);
        cases.push(case);
    }

    let report = KluReport {
        samples: args.samples,
        max_refactor_ns_per_lu_nnz: args.max_refactor_ns_per_lu_nnz,
        max_solve_ns_per_lu_nnz: args.max_solve_ns_per_lu_nnz,
        max_fill_ratio: args.max_fill_ratio,
        passed: cases.iter().all(|case| case.passed),
        cases,
    };

    println!(
        "klu samples={} refactors={} expander-refactors={}",
        report.samples, args.refactors, args.expander_refactors
    );
    for case in &report.cases {
        println!(
            "  {name:<10} n={n:<6} a_nnz={a_nnz:<8} l+u_nnz={lu_nnz:<9} fill={fill:>6.2}x  analyze={analyze:>10.1} ns factor={factor:>10.1} ns  refactor={refactor:>10.1} ns/iter ({refactor_norm:>6.3} ns/nnz)  solve={solve:>10.1} ns/iter ({solve_norm:>6.3} ns/nnz)  [{status}]",
            name = case.name,
            n = case.n,
            a_nnz = case.a_nnz,
            lu_nnz = case.lu_nnz,
            fill = case.fill_ratio,
            analyze = case.analyze_ns,
            factor = case.factor_ns,
            refactor = case.refactor_ns_per_iter,
            refactor_norm = case.refactor_ns_per_lu_nnz,
            solve = case.solve_ns_per_iter,
            solve_norm = case.solve_ns_per_lu_nnz,
            status = if !case.gated {
                "reference"
            } else if case.passed {
                "ok"
            } else {
                "failed"
            },
        );
        if let Some(failure) = &case.failure {
            println!("    {failure}");
        }
    }

    if let Some(path) = &args.out {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|source| {
                BenchError::io(
                    format!("create KLU report dir `{}`", parent.display()),
                    source,
                )
            })?;
        }
        let json = serde_json::to_string_pretty(&report).map_err(|source| BenchError::Json {
            context: "serialize KLU benchmark report".into(),
            source,
        })?;
        fs::write(path, json).map_err(|source| {
            BenchError::io(format!("write KLU report `{}`", path.display()), source)
        })?;
    }

    Ok(if report.passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}
