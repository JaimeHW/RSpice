//! Unit tests for the expression engine.
//!
//! The statistical-function tests pin both the distribution properties
//! (moments, bounds) and the determinism contract: a given seed must
//! reproduce the identical draw sequence on every platform and run.

use super::*;

fn eval_with(ctx: &ParamContext, input: &str) -> Value {
    eval_expression(input, ctx).unwrap_or_else(|e| panic!("eval `{input}` failed: {e}"))
}

#[test]
fn bare_named_constants_parse_as_numbers() {
    let ctx = ParamContext::new();
    assert!((eval_with(&ctx, "pi") - std::f64::consts::PI).abs() < 1.0e-15);
    assert!((eval_with(&ctx, "exp") - std::f64::consts::E).abs() < 1.0e-15);
    assert!((eval_with(&ctx, "2*exp") - 2.0 * std::f64::consts::E).abs() < 1.0e-15);
    assert!((eval_with(&ctx, "exp(1)") - std::f64::consts::E).abs() < 1.0e-15);
}

#[test]
fn modulo_operator_matches_xyce_precedence() {
    let ctx = ParamContext::new();
    assert_eq!(eval_with(&ctx, "2 + 6*5/2%4 - 1"), 4.0);
    assert_eq!(eval_with(&ctx, "8%3*2"), 2.0);
}

#[test]
fn table_function_clamps_outside_defined_range() {
    let ctx = ParamContext::new();
    assert_eq!(eval_with(&ctx, "table(110n%120n,0,0,60n,3.3,100n,0)"), 0.0);
    assert!((eval_with(&ctx, "table(200n%120n,0,0,60n,3.3,100n,0)") - 1.65).abs() < 1.0e-12);
}

#[test]
fn function_arguments_accept_comparison_and_ternary_expressions() {
    let mut ctx = ParamContext::new();
    ctx.set("A", 1.0);
    ctx.set("B", 2.0);
    ctx.set("C", 3.0);
    ctx.set("D", 4.0);

    assert_eq!(eval_with(&ctx, "if(A==B,C,D)"), 4.0);
    assert_eq!(eval_with(&ctx, "if(A<=B,C,D)"), 3.0);
    assert_eq!(eval_with(&ctx, "if(A!=B,(A>B)?C:D,D)"), 4.0);
}

#[test]
fn xyce_single_character_logical_ops_match_double_character_forms() {
    let mut ctx = ParamContext::new();
    ctx.set("BEDROCK", 0.033e-6);
    ctx.set("SLAGHEAP", 0.035e-6);

    assert_eq!(
        eval_with(
            &ctx,
            "0.034u <= slagheap & slagheap < 0.036u & 0.032u <= bedrock & bedrock < 0.034u ? 0.001u : 0",
        ),
        0.001e-6
    );
    assert_eq!(eval_with(&ctx, "0 | 0"), 0.0);
    assert_eq!(eval_with(&ctx, "0 | 5"), 1.0);
    assert_eq!(eval_with(&ctx, "1 && 0 || 2"), 1.0);
}

#[test]
fn xyce_user_functions_rebind_arguments_as_expressions() {
    let mut ctx = ParamContext::new();
    ctx.set("MRSLATE", 1.0);
    ctx.set("BEDROCK", 1.0);
    ctx.set("PEBBLES", 2.0);
    ctx.define_function(
        "betty",
        vec!["bedrock".to_string(), "slagheap".to_string()],
        "bedrock == 0 ? 0 : 7",
    );
    ctx.define_function(
        "great_gazoo",
        vec![
            "bedrock".to_string(),
            "pebbles".to_string(),
            "bammbamm".to_string(),
            "hatrock".to_string(),
        ],
        "mrSlate ? betty(bedrock, pebbles) : 0",
    );
    ctx.define_function(
        "wilma",
        vec![
            "bedrock".to_string(),
            "pebbles".to_string(),
            "bammbamm".to_string(),
            "slaghoople".to_string(),
            "hatrock".to_string(),
        ],
        "great_gazoo(bedrock, pebbles, bammbamm, hatrock)",
    );
    ctx.define_function(
        "dino",
        vec![
            "bedrock".to_string(),
            "pebbles".to_string(),
            "bammbamm".to_string(),
            "slaghoople".to_string(),
            "hatrock".to_string(),
        ],
        "wilma(bedrock + great_gazoo(bedrock, pebbles, bammbamm, hatrock), pebbles, bammbamm, slaghoople, hatrock)",
    );

    assert_eq!(eval_with(&ctx, "betty(bedrock, pebbles)"), 7.0);
    assert_eq!(eval_with(&ctx, "great_gazoo(bedrock, pebbles, 0, 0)"), 7.0);
    assert_eq!(eval_with(&ctx, "dino(bedrock, pebbles, 0, 0, 0)"), 0.0);
    assert_eq!(eval_with(&ctx, "great_gazoo(bedrock, pebbles, 0, 0)"), 7.0);
}

#[test]
fn xyce_special_character_function_names_evaluate() {
    let mut ctx = ParamContext::new();
    for name in ["#func", "@func", "`func", "$func"] {
        ctx.define_function(name, vec!["X".to_string()], "X+4");
        assert_eq!(eval_with(&ctx, &format!("{name}(1)")), 5.0);
    }

    ctx.set("A", 0.0);
    ctx.set("B", 2.0);
    ctx.define_function("C", vec!["X".to_string()], "X+3");
    assert_eq!(eval_with(&ctx, "A ? 1 : 2"), 2.0);
    assert_eq!(eval_with(&ctx, "A?B:C(1)"), 4.0);
}

#[test]
fn parameter_expression_pow_pwr_and_pwrs_keep_distinct_spice_sign_semantics() {
    let ctx = ParamContext::new();
    assert_eq!(eval_with(&ctx, "pow(-2,2)"), 4.0);
    assert_eq!(eval_with(&ctx, "pow(-2,3)"), -8.0);
    assert_eq!(eval_with(&ctx, "pwr(-2,3)"), 8.0);
    assert_eq!(eval_with(&ctx, "pwrs(-2,3)"), -8.0);
}

#[test]
fn behavioral_preparation_expands_functions_without_substituting_probe_names() {
    let mut ctx = ParamContext::new();
    ctx.set("scale", 2.0);
    ctx.set("node", 9.0);
    ctx.set("vsense", 7.0);
    ctx.define_function("gain", vec!["x".to_string(), "y".to_string()], "scale*x*y");

    let prepared = prepare_behavioral_expression("gain(V(node), I(vsense))", &ctx)
        .expect("behavioral expression prepares");

    assert_eq!(prepared, "((2*V(node))*I(vsense))");
}

#[test]
fn behavioral_preparation_treats_xyce_function_names_as_identifiers() {
    let mut ctx = ParamContext::new();
    ctx.define_function("#V", vec!["X".to_string()], "X+2");
    ctx.define_function("@I", vec!["X".to_string()], "X+3");

    let prepared = prepare_behavioral_expression("#V(1)+@I(2)", &ctx)
        .expect("special-character function expression prepares");

    assert_eq!(prepared, "((1+2)+(2+3))");
}

#[test]
fn behavioral_preparation_expands_unknown_function_arguments() {
    let mut ctx = ParamContext::new();
    ctx.set("V1", 1.1);
    ctx.set("V2", 2.0);
    ctx.set("TD", 0.5e-12);
    ctx.set("TR", 0.3e-12);
    ctx.set("TF", 0.4e-12);
    ctx.set("PW", 10.0e-12);

    let prepared = prepare_behavioral_expression("spice_pulse(v1,v2,td,tr,tf,pw)", &ctx)
        .expect("behavioral expression prepares");

    assert_eq!(
        prepared,
        "SPICE_PULSE(1.1,2,0.0000000000005,0.0000000000003,0.0000000000004,0.00000000001)"
    );
}

#[test]
fn behavioral_preparation_preserves_digit_leading_probe_nodes() {
    let ctx = ParamContext::new();

    let prepared =
        prepare_behavioral_expression("V(2a)+V(1)", &ctx).expect("behavioral expression prepares");

    assert_eq!(prepared, "(V(2a)+V(1))");
    assert!(!prepared.contains("0.000000000000000002"));
}

#[test]
fn behavioral_preparation_expands_xyce_ordered_poly_expression() {
    let ctx = ParamContext::new();

    let prepared = prepare_behavioral_expression("POLY(1) V(2) 3 2 1", &ctx)
        .expect("POLY expression prepares");

    assert!(!prepared.to_ascii_uppercase().contains("POLY"));
    assert!(prepared.contains("V(2)"));
    assert!(prepared.contains("3"));
    assert!(prepared.contains("2"));
}

#[test]
fn behavioral_preparation_evaluates_xyce_ordered_poly1_coefficients_by_degree() {
    let mut ctx = ParamContext::new();
    for (name, value) in [
        ("C0", 3.0),
        ("C1", 2.0),
        ("C2", 1.0),
        ("C3", 3.0),
        ("C4", 4.0),
    ] {
        ctx.set(name, value);
    }
    ctx.define_function(
        "polyVersion",
        vec!["X1".to_string()],
        "POLY(1) X1 C0 C1 C2 C3 C4",
    );

    for (input, expected) in [(-2.0, 43.0), (-1.5, 12.375), (-1.0, 3.0), (2.0, 99.0)] {
        let expression = format!("polyVersion({input})");
        let prepared =
            prepare_behavioral_expression(&expression, &ctx).expect("POLY function prepares");
        let actual = eval_expression(&prepared, &ParamContext::new())
            .unwrap_or_else(|err| panic!("prepared POLY expression `{prepared}` evaluates: {err}"));
        assert!(
            (actual - expected).abs() < 1.0e-12,
            "POLY(1) at {input} prepared as `{prepared}` produced {actual}, expected {expected}"
        );
    }
}

#[test]
fn behavioral_preparation_compiles_xyce_ordered_poly1_with_voltage_probe_argument() {
    let mut ctx = ParamContext::new();
    for (name, value) in [
        ("C0", 3.0),
        ("C1", 2.0),
        ("C2", 1.0),
        ("C3", 3.0),
        ("C4", 4.0),
    ] {
        ctx.set(name, value);
    }
    ctx.define_function(
        "polyVersion",
        vec!["X1".to_string()],
        "POLY(1) X1 C0 C1 C2 C3 C4",
    );

    let prepared = prepare_behavioral_expression("3.0*(polyVersion(V(2)))", &ctx)
        .expect("POLY function with voltage probe prepares");
    let ast = crate::expr::parse_expression_strict(&prepared)
        .unwrap_or_else(|err| panic!("prepared expression `{prepared}` parses: {err}"));
    let program = crate::expr::compile(&ast);
    let node_index = *program
        .node_map
        .get("2")
        .expect("compiled expression references V(2)");
    let mut vm = crate::expr::Vm::new();

    for (input, expected) in [(-2.0, 129.0), (-1.5, 37.125), (-1.0, 9.0), (2.0, 297.0)] {
        let mut voltages = vec![0.0; program.node_map.len()];
        voltages[node_index] = input;
        let actual = vm.execute(&program, &crate::expr::Context::dc(&voltages, &[]));
        assert!(
            (actual - expected).abs() < 1.0e-12,
            "prepared expression `{prepared}` at V(2)={input} produced {actual}, expected {expected}"
        );
    }
}

#[test]
fn parsed_xyce_poly1_function_body_prepares_with_voltage_probe_argument() {
    let deck = r#"
.param C0=3.0
.param C1=2.0
.param C2=1.0
.param C3=3.0
.param C4=4.0
.func polyVersion(x1) {POLY(1) x1 C0 C1 C2 C3 C4}
B 3 0 V ={3.0*(polyVersion(V(2)))}
.op
.end
"#;
    let netlist = crate::netlist::Netlist::parse(deck).expect("deck parses");
    let expression = netlist
        .elements
        .iter()
        .find_map(|element| match &element.kind {
            crate::netlist::ElementKind::BehavioralVoltage { expression, .. } => {
                Some(expression.as_str())
            }
            _ => None,
        })
        .expect("behavioral voltage source parsed");
    let prepared = prepare_behavioral_expression(expression, &netlist.params)
        .expect("parsed POLY function with voltage probe prepares");
    let ast = crate::expr::parse_expression_strict(&prepared)
        .unwrap_or_else(|err| panic!("prepared expression `{prepared}` parses: {err}"));
    let program = crate::expr::compile(&ast);
    let node_index = *program
        .node_map
        .get("2")
        .expect("compiled expression references V(2)");
    let mut voltages = vec![0.0; program.node_map.len()];
    voltages[node_index] = -2.0;
    let mut vm = crate::expr::Vm::new();
    let actual = vm.execute(&program, &crate::expr::Context::dc(&voltages, &[]));

    assert!(
        (actual - 129.0).abs() < 1.0e-12,
        "parsed expression `{expression}` prepared as `{prepared}` produced {actual}"
    );
}

#[test]
fn behavioral_preparation_expands_xyce_ordered_poly_function_body() {
    let mut ctx = ParamContext::new();
    for (name, value) in [
        ("C0", 0.1),
        ("C1", 0.2),
        ("C2", 0.3),
        ("C11", 0.4),
        ("C12", 0.5),
        ("C21", 0.6),
        ("C22", 0.7),
        ("C111", 0.8),
        ("C112", 0.9),
        ("C121", 0.10),
        ("C122", 0.11),
        ("C211", 0.12),
        ("C212", 0.13),
        ("C221", 0.14),
        ("C222", 0.15),
    ] {
        ctx.set(name, value);
    }
    ctx.define_function(
        "polyVersion",
        vec!["X1".to_string(), "X2".to_string()],
        "POLY(2) X1 X2 C0 C1 C2 C11 C12 C21 C22 C111 C112 C121 C122 C211 C212 C221 C222",
    );

    let prepared = prepare_behavioral_expression("polyVersion(V(2), V(1))", &ctx)
        .expect("POLY function prepares");

    assert!(!prepared.to_ascii_uppercase().contains("POLY"));
    assert!(prepared.contains("V(2)"));
    assert!(prepared.contains("V(1)"));
    assert!(prepared.contains("0.8"));
    assert!(prepared.contains("0.15"));
}

#[test]
fn behavioral_preparation_preserves_digit_leading_probe_nodes_through_function_args() {
    let mut ctx = ParamContext::new();
    ctx.set("C0", 0.1);
    ctx.set("C1", 0.2);
    ctx.set("C2", 0.3);
    ctx.define_function(
        "exprVersion",
        vec!["X1".to_string(), "X2".to_string()],
        "C0 + C1*X1 + C2*X2",
    );

    let prepared = prepare_behavioral_expression("exprVersion(V(2a),V(1))", &ctx)
        .expect("function call expression prepares");

    assert!(prepared.contains("V(2a)"));
    assert!(prepared.contains("V(1)"));
    assert!(!prepared.contains("0.000000000000000002"));
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
    assert_ne!(
        first, third,
        "different seeds must give different sequences"
    );
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
    assert!(
        (var - 1.0).abs() < 0.05,
        "normal variance {var} too far from 1"
    );
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
fn gauss_two_arg_defaults_sigma_to_one_like_xyce() {
    let ctx = ParamContext::new();
    let nom = 100.0;
    let expected_std = 5.0;
    let mut sum = 0.0;
    let mut sum2 = 0.0;
    let n = 10_000;
    for _ in 0..n {
        let v = eval_with(&ctx, "gauss(100, 0.05)");
        sum += v;
        sum2 += v * v;
    }
    let mean = sum / n as Value;
    let var = sum2 / n as Value - mean * mean;
    let std = var.sqrt();
    assert!((mean - nom).abs() < 0.1, "gauss mean {mean} != {nom}");
    assert!(
        (std - expected_std).abs() < 0.15,
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
    assert!((std - 0.1).abs() < 0.005, "agauss std {std} != 0.3/3 = 0.1");
}

#[test]
fn agauss_two_arg_defaults_sigma_to_one_like_xyce() {
    let ctx = ParamContext::new();
    let nom = 1.5;
    let expected_std = 0.3;
    let mut sum = 0.0;
    let mut sum2 = 0.0;
    let n = 10_000;
    for _ in 0..n {
        let v = eval_with(&ctx, "agauss(1.5, 0.3)");
        sum += v;
        sum2 += v * v;
    }
    let mean = sum / n as Value;
    let var = sum2 / n as Value - mean * mean;
    let std = var.sqrt();
    assert!((mean - nom).abs() < 0.005, "agauss mean {mean} != {nom}");
    assert!(
        (std - expected_std).abs() < 0.005,
        "agauss std {std} != {expected_std}"
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
        assert!(
            (-3.5..-2.5).contains(&w),
            "aunif draw {w} outside [-3.5, -2.5)"
        );
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
    assert!(
        above > 500 && below > 500,
        "unif draws one-sided: +{above}/-{below}"
    );
}

#[test]
fn nominal_statistical_mode_matches_xyce_non_sampling_semantics() {
    let mut ctx = ParamContext::new();
    ctx.set_statistical_mode(StatisticalParamMode::Nominal);

    assert_eq!(eval_with(&ctx, "agauss(1, 1, 1)"), 1.0);
    assert_eq!(eval_with(&ctx, "agauss(1, 1)"), 1.0);
    assert_eq!(eval_with(&ctx, "gauss(1, 1, 1)"), 1.0);
    assert_eq!(eval_with(&ctx, "gauss(1, 1)"), 1.0);
    assert_eq!(eval_with(&ctx, "2 * rand()"), 1.0);
    assert_eq!(eval_with(&ctx, "unif(1, 1)"), 1.0);
    assert_eq!(eval_with(&ctx, "aunif(1, 1)"), 1.0);
    assert_eq!(eval_with(&ctx, "limit(5, 0.5)"), 5.0);

    let replay = ParamContext::new();
    assert_eq!(
        ctx.random().next_uniform(),
        replay.random().next_uniform(),
        "nominal evaluation must not consume the statistical draw stream"
    );
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
        eval_expression("agauss(1)", &ctx),
        Err(ExprError::WrongArgCount(_))
    ));
    assert!(matches!(
        eval_expression("gauss(1, 0.1, 1, 2)", &ctx),
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
fn ngspice_comparison_spellings() {
    // numparam accepts `=` as equality and `<>` as inequality.
    let ctx = ParamContext::new();
    assert_eq!(eval_with(&ctx, "(3 = 3)"), 1.0);
    assert_eq!(eval_with(&ctx, "(3 = 4)"), 0.0);
    assert_eq!(eval_with(&ctx, "(3 <> 4)"), 1.0);
    assert_eq!(eval_with(&ctx, "(3 <> 3)"), 0.0);
    // The two-character spellings keep working.
    assert_eq!(eval_with(&ctx, "(3 == 3)"), 1.0);
    assert_eq!(eval_with(&ctx, "(3 != 3)"), 0.0);
    // And `<`/`<=` are not shadowed by `<>`.
    assert_eq!(eval_with(&ctx, "(3 < 4)"), 1.0);
    assert_eq!(eval_with(&ctx, "(4 <= 4)"), 1.0);
}

#[test]
fn statistical_functions_compose_with_parameters_and_expressions() {
    let mut ctx = ParamContext::new();
    ctx.set("W", 2.0e-6);
    ctx.set_random_seed(5);
    let v = eval_with(&ctx, "W * (1 + agauss(0, 0.01, 1))");
    assert!(v > 1.8e-6 && v < 2.2e-6, "composed draw {v} implausible");
}

#[test]
fn power_operator_matches_ngspice_numparam() {
    // Every value below pins an ngspice-46 .param oracle result: power
    // binds tighter than unary minus, chains fold left, and a sign in
    // exponent position applies to the immediate operand only.
    let ctx = ParamContext::new();
    assert_eq!(eval_with(&ctx, "-2^2"), -4.0);
    assert_eq!(eval_with(&ctx, "3-2^2"), -1.0);
    assert_eq!(eval_with(&ctx, "2^-2"), 0.25);
    assert_eq!(eval_with(&ctx, "2^3^2"), 64.0);
    assert_eq!(eval_with(&ctx, "4^3^0.5"), 8.0);
    assert_eq!(eval_with(&ctx, "2^-3^2"), 0.015625);
    assert_eq!(eval_with(&ctx, "2^-3^2*4"), 0.0625);
    assert_eq!(eval_with(&ctx, "2^-3-1"), -0.875);
    assert_eq!(eval_with(&ctx, "2**-2"), 0.25);
    assert_eq!(eval_with(&ctx, "-2**2"), -4.0);
    assert_eq!(eval_with(&ctx, "2**3**2"), 64.0);
}

#[test]
fn xyce_complex_log10_negative_param_projects_components() {
    let mut ctx = ParamContext::new();
    let value = eval_expression_complex("log10(-1)", &ctx)
        .unwrap_or_else(|e| panic!("complex log10 failed: {e}"));
    let expected_im = -std::f64::consts::PI / std::f64::consts::LN_10;

    assert!(value.re.abs() < 1.0e-14, "real part {}", value.re);
    assert!(
        (value.im - expected_im).abs() < 1.0e-14,
        "imaginary part {}",
        value.im
    );

    ctx.set_complex("r0", value);
    assert!(eval_with(&ctx, "r0").abs() < 1.0e-14);
    assert!(eval_with(&ctx, "re(r0)").abs() < 1.0e-14);
    assert!((eval_with(&ctx, "img(r0)") - expected_im).abs() < 1.0e-14);
}

#[test]
fn log_function_follows_expression_dialect() {
    let mut ctx = ParamContext::new();
    assert!((eval_with(&ctx, "log(100)") - 100.0_f64.ln()).abs() < 1.0e-14);
    assert!((eval_with(&ctx, "ln(100)") - 100.0_f64.ln()).abs() < 1.0e-14);
    assert!((eval_with(&ctx, "log10(100)") - 2.0).abs() < 1.0e-14);

    ctx.set_expression_dialect(crate::netlist::ExpressionDialect::Xyce);
    assert!((eval_with(&ctx, "log(100)") - 2.0).abs() < 1.0e-14);
    assert!((eval_with(&ctx, "ln(100)") - 100.0_f64.ln()).abs() < 1.0e-14);
    assert!((eval_with(&ctx, "log10(100)") - 2.0).abs() < 1.0e-14);
}

#[test]
fn xyce_hyperbolic_functions_follow_expression_dialect() {
    let mut ctx = ParamContext::new();
    assert!(eval_with(&ctx, "atanh(1)").is_infinite());
    assert_eq!(eval_with(&ctx, "tanh(21)"), 21.0_f64.tanh());

    ctx.set_expression_dialect(crate::netlist::ExpressionDialect::Xyce);

    let upper_saturated_atanh = (1.0 - 1.0e-12_f64).atanh();
    let lower_saturated_atanh = (1.0e-12_f64 - 1.0).atanh();
    assert!((eval_with(&ctx, "atanh(1)") - upper_saturated_atanh).abs() < 1.0e-14);
    assert!((eval_with(&ctx, "atanh(2)") - upper_saturated_atanh).abs() < 1.0e-14);
    assert!((eval_with(&ctx, "atanh(-2)") - lower_saturated_atanh).abs() < 1.0e-14);
    assert_eq!(eval_with(&ctx, "tanh(21)"), 1.0);
    assert_eq!(eval_with(&ctx, "tanh(-21)"), -1.0);
}

#[test]
fn xyce_complex_literals_and_projection_functions() {
    let ctx = ParamContext::new();
    let literal = eval_expression_complex("3.0+2.0J", &ctx)
        .unwrap_or_else(|e| panic!("complex literal failed: {e}"));
    assert!((literal.re - 3.0).abs() < 1.0e-14);
    assert!((literal.im - 2.0).abs() < 1.0e-14);
    assert!((eval_with(&ctx, "m(3.0+2.0J)") - 13.0_f64.sqrt()).abs() < 1.0e-14);

    let sqrt_negative = eval_expression_complex("sqrt(-1.0)", &ctx)
        .unwrap_or_else(|e| panic!("complex sqrt failed: {e}"));
    assert!(sqrt_negative.re.abs() < 1.0e-14);
    assert!((sqrt_negative.im + 1.0).abs() < 1.0e-14);
    assert!((eval_with(&ctx, "img(sqrt(-1.0))") + 1.0).abs() < 1.0e-14);
}

#[test]
fn parser_preserves_complex_param_storage() {
    let netlist = crate::netlist::Netlist::parse(
        "complex params\n\
         .param r0={log10(-1)}\n\
         .param realPart=1.0e-4\n\
         .param imagPart=2.0e-4\n\
         .param par1={realPart + imagPart*1.0J}\n\
         .param alias=par1\n\
         .END\n",
    )
    .expect("complex parameter deck parses");

    let r0 = netlist.params.get_complex("r0").expect("r0 exists");
    let expected_im = -std::f64::consts::PI / std::f64::consts::LN_10;
    assert!(r0.re.abs() < 1.0e-14);
    assert!((r0.im - expected_im).abs() < 1.0e-14);

    let par1 = netlist.params.get_complex("par1").expect("par1 exists");
    assert!((par1.re - 1.0e-4).abs() < 1.0e-18);
    assert!((par1.im - 2.0e-4).abs() < 1.0e-18);
    assert_eq!(netlist.params.get_complex("alias"), Some(par1));
}

#[test]
fn number_suffixes_match_ngspice_numparam() {
    // numparam swallows letters after a number, applies the scale factor
    // even after a scientific exponent, and has no `mil` unit (`1mil` is
    // milli). All values pinned against ngspice-46.
    let ctx = ParamContext::new();
    assert_eq!(eval_with(&ctx, "10kohm"), 10_000.0);
    assert_eq!(eval_with(&ctx, "1MegHz"), 1e6);
    assert_eq!(eval_with(&ctx, "1mil"), 1e-3);
    assert_eq!(eval_with(&ctx, "1e3k"), 1e6);
    // ngspice computes `mantissa * scale`, so pin the same product (one
    // ulp off the decimal literal 2.5e-6).
    assert_eq!(eval_with(&ctx, "2.5u"), 2.5 * 1e-6);
    assert_eq!(eval_with(&ctx, "1a"), 1e-18);
    assert_eq!(eval_with(&ctx, "5xyz"), 5.0);
    assert_eq!(eval_with(&ctx, "10k + 1"), 10_001.0);
    assert_eq!(eval_with(&ctx, "3meg"), 3e6);
    assert_eq!(eval_with(&ctx, "3x"), 3e6);
    assert_eq!(eval_with(&ctx, "3X"), 3e6);
}
