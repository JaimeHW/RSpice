//! Unit tests for the expression engine.
//!
//! The statistical-function tests pin both the distribution properties
//! (moments, bounds) and the determinism contract: a given seed must
//! reproduce the identical draw sequence on every platform and run.

use super::*;

fn eval_with(ctx: &ParamContext, input: &str) -> Value {
    eval_expression(input, ctx).unwrap_or_else(|e| panic!("eval `{input}` failed: {e}"))
}

//=============================================================================
// RandomState: determinism and distribution
//=============================================================================

#[test]
fn random_stream_is_deterministic_per_seed() {
    let a = RandomState::new(42);
    let b = RandomState::new(42);
    let first: Vec<Value> = (0..64).map(|_| a.next_uniform()).collect();
    let second: Vec<Value> = (0..64).map(|_| b.next_uniform()).collect();
    assert_eq!(first, second, "same seed must reproduce the same sequence");

    let c = RandomState::new(43);
    let third: Vec<Value> = (0..64).map(|_| c.next_uniform()).collect();
    assert_ne!(first, third, "different seeds must give different sequences");
}

#[test]
fn random_uniform_is_in_unit_interval_with_sane_moments() {
    let r = RandomState::new(7);
    let n = 20_000;
    let mut sum = 0.0;
    for _ in 0..n {
        let u = r.next_uniform();
        assert!((0.0..1.0).contains(&u), "uniform draw {u} outside [0,1)");
        sum += u;
    }
    let mean = sum / n as Value;
    assert!(
        (mean - 0.5).abs() < 0.01,
        "uniform mean {mean} too far from 0.5"
    );
}

#[test]
fn random_standard_normal_has_unit_moments_and_stays_finite() {
    let r = RandomState::new(1234);
    let n = 50_000;
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    for _ in 0..n {
        let z = r.next_standard_normal();
        assert!(z.is_finite(), "normal draw must be finite");
        sum += z;
        sum_sq += z * z;
    }
    let mean = sum / n as Value;
    let var = sum_sq / n as Value - mean * mean;
    assert!((mean).abs() < 0.02, "normal mean {mean} too far from 0");
    assert!((var - 1.0).abs() < 0.05, "normal variance {var} too far from 1");
}

#[test]
fn reseeding_restarts_the_stream() {
    let mut ctx = ParamContext::new();
    ctx.set_random_seed(99);
    let first = eval_with(&ctx, "agauss(0, 1, 1)");
    let second = eval_with(&ctx, "agauss(0, 1, 1)");
    assert_ne!(first, second, "consecutive draws must differ");

    ctx.set_random_seed(99);
    let replay = eval_with(&ctx, "agauss(0, 1, 1)");
    assert_eq!(first, replay, "reseeding must restart the stream");
}

#[test]
fn cloned_contexts_share_one_netlist_wide_stream() {
    // Clones pull from a single shared sequence: interleaved draws across
    // the original and a clone must replay exactly as sequential draws on a
    // fresh context with the same seed.
    let mut ctx = ParamContext::new();
    ctx.set_random_seed(11);
    let cloned = ctx.clone();
    let interleaved = [
        eval_with(&ctx, "aunif(0, 1)"),
        eval_with(&cloned, "aunif(0, 1)"),
        eval_with(&ctx, "aunif(0, 1)"),
    ];

    let mut replay_ctx = ParamContext::new();
    replay_ctx.set_random_seed(11);
    let sequential = [
        eval_with(&replay_ctx, "aunif(0, 1)"),
        eval_with(&replay_ctx, "aunif(0, 1)"),
        eval_with(&replay_ctx, "aunif(0, 1)"),
    ];
    assert_eq!(interleaved, sequential);
    // And the draws are genuinely distinct values, not a stuck stream.
    assert_ne!(interleaved[0], interleaved[1]);
    assert_ne!(interleaved[1], interleaved[2]);
}

#[test]
fn adopt_random_joins_the_source_stream() {
    let mut parent = ParamContext::new();
    parent.set_random_seed(21);
    let mut child = ParamContext::new();
    child.adopt_random(parent.random());

    let interleaved = [
        eval_with(&parent, "aunif(0, 1)"),
        eval_with(&child, "aunif(0, 1)"),
    ];

    let mut replay = ParamContext::new();
    replay.set_random_seed(21);
    let sequential = [
        eval_with(&replay, "aunif(0, 1)"),
        eval_with(&replay, "aunif(0, 1)"),
    ];
    assert_eq!(interleaved, sequential);
}

//=============================================================================
// Statistical functions: semantics
//=============================================================================

#[test]
fn gauss_matches_relative_deviation_semantics() {
    // gauss(nom, rvar, sigma): std dev = nom * rvar / sigma.
    let ctx = ParamContext::new();
    let nom = 100.0;
    let rvar = 0.05;
    let sigma = 2.0;
    let n = 50_000;
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    for _ in 0..n {
        let v = eval_with(&ctx, "gauss(100, 0.05, 2)");
        sum += v;
        sum_sq += v * v;
    }
    let mean = sum / n as Value;
    let std = (sum_sq / n as Value - mean * mean).sqrt();
    let expected_std = nom * rvar / sigma;
    assert!((mean - nom).abs() < 0.05, "gauss mean {mean} != {nom}");
    assert!(
        (std - expected_std).abs() / expected_std < 0.05,
        "gauss std {std} != {expected_std}"
    );
}

#[test]
fn agauss_matches_absolute_deviation_semantics() {
    // agauss(nom, avar, sigma): std dev = avar / sigma.
    let ctx = ParamContext::new();
    let n = 50_000;
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    for _ in 0..n {
        let v = eval_with(&ctx, "agauss(1.5, 0.3, 3)");
        sum += v;
        sum_sq += v * v;
    }
    let mean = sum / n as Value;
    let std = (sum_sq / n as Value - mean * mean).sqrt();
    assert!((mean - 1.5).abs() < 0.005, "agauss mean {mean} != 1.5");
    assert!(
        (std - 0.1).abs() < 0.005,
        "agauss std {std} != 0.3/3 = 0.1"
    );
}

#[test]
fn unif_and_aunif_respect_bounds() {
    let ctx = ParamContext::new();
    for _ in 0..5_000 {
        // unif: nom * (1 ± rvar) bounds.
        let v = eval_with(&ctx, "unif(10, 0.2)");
        assert!((8.0..12.0).contains(&v), "unif draw {v} outside [8, 12)");
        // aunif: nom ± avar bounds.
        let w = eval_with(&ctx, "aunif(-3, 0.5)");
        assert!((-3.5..-2.5).contains(&w), "aunif draw {w} outside [-3.5, -2.5)");
    }
}

#[test]
fn unif_covers_both_sides_of_nominal() {
    let ctx = ParamContext::new();
    let mut above = 0usize;
    let mut below = 0usize;
    for _ in 0..2_000 {
        let v = eval_with(&ctx, "unif(1, 0.1)");
        if v > 1.0 {
            above += 1;
        } else if v < 1.0 {
            below += 1;
        }
    }
    assert!(above > 500 && below > 500, "unif draws one-sided: +{above}/-{below}");
}

#[test]
fn limit_two_arg_is_worst_case_two_point() {
    let ctx = ParamContext::new();
    let mut hi = 0usize;
    let mut lo = 0usize;
    for _ in 0..1_000 {
        let v = eval_with(&ctx, "limit(5, 0.5)");
        if (v - 5.5).abs() < 1e-12 {
            hi += 1;
        } else if (v - 4.5).abs() < 1e-12 {
            lo += 1;
        } else {
            panic!("limit(5, 0.5) produced {v}, expected exactly 4.5 or 5.5");
        }
    }
    assert!(hi > 0 && lo > 0, "limit never hit one side: +{hi}/-{lo}");
}

#[test]
fn limit_three_arg_clamp_is_unchanged() {
    let ctx = ParamContext::new();
    assert_eq!(eval_with(&ctx, "limit(5, 0, 2)"), 2.0);
    assert_eq!(eval_with(&ctx, "limit(-5, 0, 2)"), 0.0);
    assert_eq!(eval_with(&ctx, "limit(1, 0, 2)"), 1.0);
}

#[test]
fn statistical_functions_validate_arguments() {
    let ctx = ParamContext::new();
    assert!(matches!(
        eval_expression("gauss(1, 0.1, 0)", &ctx),
        Err(ExprError::InvalidArgument(_))
    ));
    assert!(matches!(
        eval_expression("gauss(1, 0.1)", &ctx),
        Err(ExprError::WrongArgCount(_))
    ));
    assert!(matches!(
        eval_expression("agauss(1)", &ctx),
        Err(ExprError::WrongArgCount(_))
    ));
    assert!(matches!(
        eval_expression("unif(1)", &ctx),
        Err(ExprError::WrongArgCount(_))
    ));
    assert!(matches!(
        eval_expression("limit(1)", &ctx),
        Err(ExprError::WrongArgCount(_))
    ));
}

#[test]
fn statistical_functions_compose_with_parameters_and_expressions() {
    let mut ctx = ParamContext::new();
    ctx.set("W", 2.0e-6);
    ctx.set_random_seed(5);
    let v = eval_with(&ctx, "W * (1 + agauss(0, 0.01, 1))");
    assert!(v > 1.8e-6 && v < 2.2e-6, "composed draw {v} implausible");
}
