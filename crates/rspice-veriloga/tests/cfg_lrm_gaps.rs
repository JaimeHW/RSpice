//! Where the constructs Phase 7 called "gaps" actually stand.
//!
//! The rebuild plan lists three things guard flattening could not express:
//! voltage contributions inside runtime loops, `$limit` under block-structured
//! control flow, and analog event ordering. These are the smallest modules that
//! need each, run through the front end, the CFG lowering and the AD pass.
//!
//! Written as status assertions rather than aspirations. Two of the three lower
//! today and are pinned here so they cannot silently regress; the third is
//! refused *by design* and the test says so, with the reason, so the plan stops
//! carrying it as unfinished work.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{AdSeed, CanonicalIrArtifact, differentiate};

fn compiled(name: &str, source: &str) -> CanonicalIrArtifact {
    VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name} must compile: {error:?}"))
}

/// Lower and differentiate, or say which step refused.
fn lower_and_differentiate(name: &str, source: &str) {
    let artifact = compiled(name, source);
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
        .unwrap_or_else(|diagnostics| panic!("{name}: lowering refused: {diagnostics:?}"));

    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain(
            (0..artifact.mir.branch_unknowns.len())
                .map(|index| AdSeed::BranchUnknownFlow(index.into())),
        )
        .collect();
    differentiate(&cfg.function, &lanes)
        .unwrap_or_else(|error| panic!("{name}: differentiation refused: {error}"));
}

/// A potential contribution built inside a loop, with the loop unrolled.
///
/// This is the case the plan meant. A potential contribution introduces a branch
/// unknown, so the loop is building the equation for a quantity the solver
/// solves for — and guard flattening could not represent that at all.
#[test]
fn a_voltage_contribution_inside_an_unrolled_loop_lowers() {
    lower_and_differentiate(
        "unrolled potential contribution",
        r#"
module looped_potential(a, b);
    inout a, b;
    electrical a, b;
    parameter real r = 100.0;
    integer index;
    analog begin
        for (index = 0; index < 3; index = index + 1) begin
            V(a, b) <+ r * I(a, b) / 3.0;
        end
    end
endmodule
"#,
    );
}

/// The same for a current contribution.
#[test]
fn a_current_contribution_inside_an_unrolled_loop_lowers() {
    lower_and_differentiate(
        "unrolled current contribution",
        r#"
module looped_current(a, b);
    inout a, b;
    electrical a, b;
    parameter real g = 1.0e-3;
    integer index;
    analog begin
        for (index = 0; index < 3; index = index + 1) begin
            I(a, b) <+ g * V(a, b);
        end
    end
endmodule
"#,
    );
}

/// A loop whose trip count is not known until the bias is: contributions inside
/// it are refused, and that is the design rather than a gap.
///
/// A stamp is a fixed program — a set of matrix entries decided at generation
/// time. A contribution executed a bias-dependent number of times has no fixed
/// set of entries to be, so there is nothing for the emitter to write. The
/// diagnostic says exactly this, and the check is in the semantic phase where
/// the trip count is known not to fold.
///
/// Pinned rather than left implicit, because "it does not lower" and "it must
/// not lower" look identical in a bug report.
#[test]
fn a_contribution_inside_a_runtime_bounded_loop_is_refused_by_design() {
    let source = r#"
module runtime_looped(a, b);
    inout a, b;
    electrical a, b;
    parameter real g = 1.0e-3;
    real bound;
    integer index;
    analog begin
        bound = 3.0 + V(a, b);
        index = 0;
        while (index < bound) begin
            I(a, b) <+ g * V(a, b);
            index = index + 1;
        end
    end
endmodule
"#;

    let error = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect_err("a bias-dependent trip count cannot carry a contribution");
    let rendered = format!("{error:?}");
    assert!(
        rendered.contains("compile-time-constant bounds"),
        "the refusal must name the reason, got {rendered}"
    );
}

/// `$limit` reached through a guard rather than at the top level.
///
/// The limiter carries per-instance state keyed by call site, so a call under a
/// branch keeps its slot whether or not the branch was taken on any particular
/// evaluation — which is what a flattened guard could not represent.
#[test]
fn limit_under_block_structured_control_flow_lowers() {
    lower_and_differentiate(
        "guarded limit",
        r#"
module guarded_limit(p, n);
    inout p, n;
    electrical p, n;
    parameter real is_sat = 1.0e-14;
    parameter real vt = 0.025852;
    parameter real vmax = 0.4;

    analog function real clampv;
        input proposed, previous, ceiling;
        real proposed, previous, ceiling;
        begin
            clampv = proposed > ceiling ? ceiling : proposed;
        end
    endfunction

    real vd;
    analog begin
        if (V(p, n) > 0.0) begin
            vd = $limit(V(p, n), "clampv", vmax);
        end else begin
            vd = V(p, n);
        end
        I(p, n) <+ is_sat * (exp(vd / vt) - 1.0);
    end
endmodule
"#,
    );
}

/// `$limit` inside an unrolled loop: a distinct call site per iteration, each
/// with its own state slot.
#[test]
fn limit_inside_an_unrolled_loop_lowers() {
    lower_and_differentiate(
        "looped limit",
        r#"
module looped_limit(p, n);
    inout p, n;
    electrical p, n;
    parameter real vt = 0.025852;
    parameter real vmax = 0.4;

    analog function real clampv;
        input proposed, previous, ceiling;
        real proposed, previous, ceiling;
        begin
            clampv = proposed > ceiling ? ceiling : proposed;
        end
    endfunction

    real vd;
    integer index;
    analog begin
        vd = V(p, n);
        for (index = 0; index < 2; index = index + 1) begin
            vd = $limit(vd, "clampv", vmax);
        end
        I(p, n) <+ 1.0e-14 * (exp(vd / vt) - 1.0);
    end
endmodule
"#,
    );
}

/// Analog events, and the order their bodies run in.
///
/// `initial_step` and `final_step` bodies are ordered against the main body
/// rather than folded into it, and both may assign variables the body reads.
#[test]
fn analog_event_bodies_lower_in_order() {
    lower_and_differentiate(
        "analog events",
        r#"
module events(a, b);
    inout a, b;
    electrical a, b;
    parameter real g = 1.0e-3;
    real gain;
    analog begin
        @(initial_step) begin
            gain = 2.0;
        end
        if (gain == 0.0) gain = 1.0;
        I(a, b) <+ g * gain * V(a, b);
        @(final_step) begin
            gain = 0.0;
        end
    end
endmodule
"#,
    );
}
