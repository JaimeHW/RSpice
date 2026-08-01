//! What a stamp writes, and what it has stopped writing.
//!
//! The claim being checked is narrow and worth stating: a device should not
//! write matrix entries it knows are zero. In the output this replaces, 202 of
//! 931 stamp arguments are literal `multiplicity * 0.0`, and the two-node CMC
//! resistor spends two of its three stamper calls writing nothing at all.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{AdSeed, CanonicalIrArtifact, differentiate, optimize_cfg};
use rspice_veriloga::rust_backend::stamp_plan::StampPlan;

#[test]
fn a_stamp_writes_only_the_entries_it_can_reach() {
    // Two independent branches sharing no unknown: each residual reaches its
    // own two nodes and neither reaches the other's, so a dense Jacobian would
    // be half zeros and this one should carry none of them.
    let source = r#"
module pair(a, b, c, d);
    inout a, b, c, d;
    electrical a, b, c, d;
    parameter real r1 = 100.0;
    parameter real r2 = 220.0;
    analog begin
        I(a, b) <+ V(a, b) / r1;
        I(c, d) <+ V(c, d) / r2;
    end
endmodule
"#;
    let plan = plan(source, "pair");

    assert!(
        plan.structurally_absent > 0,
        "each residual reaches two of the four nodes, so half the dense \
         Jacobian should never have been built; plan was {plan:?}"
    );
    for row in &plan.rows {
        assert!(
            row.derivatives.len() <= 2,
            "a two-terminal branch has at most two live entries, got {}",
            row.derivatives.len()
        );
    }
}

/// A contribution that depends on no unknown is still a contribution — it has a
/// residual and an equation — but it has no Jacobian row at all.
///
/// This is the mechanism that does the work, and it is worth separating from
/// the arithmetic one. `0.0 * V(a,c)` deliberately does *not* fold here: `x * 0`
/// is false for NaN and this backend does not apply that identity, so an entry
/// written that way survives as an expression and is written. What gets dropped
/// is what the derivative pass never built.
#[test]
fn a_contribution_that_reads_no_unknown_writes_no_row() {
    let source = r#"
module leak(a, c);
    inout a, c;
    electrical a, c;
    parameter real g = 1.0e-3;
    parameter real bias = 5.0e-6;
    analog begin
        I(a, c) <+ g * V(a, c);
        I(a, c) <+ bias;
    end
endmodule
"#;
    let plan = plan(source, "leak");

    let empty = plan
        .rows
        .iter()
        .filter(|row| row.derivatives.is_empty())
        .count();
    assert_eq!(
        empty, 1,
        "the constant contribution reaches no unknown, so its whole row should \
         be absent; plan was {plan:?}"
    );
    assert!(
        plan.entries() > 0,
        "the conductive contribution is real and must survive"
    );
    assert!(
        plan.structurally_absent > 0,
        "the absent row is what `structurally_absent` counts"
    );
}

fn plan(source: &str, name: &str) -> StampPlan {
    let artifact = artifact(source);
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
        .unwrap_or_else(|diagnostics| panic!("{name}: {diagnostics:?}"));

    let seeds: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain(
            (0..artifact.mir.branch_unknowns.len())
                .map(|index| AdSeed::BranchUnknownFlow(index.into())),
        )
        .collect();
    let mut differentiated =
        differentiate(&cfg.function, &seeds).unwrap_or_else(|error| panic!("{name}: {error}"));

    // Every read-out before anything evaluates or emits: taking a lane appends
    // an instruction to the function.
    let rows: Vec<Vec<Option<_>>> = artifact
        .mir
        .equations
        .iter()
        .map(|equation| {
            differentiated.derivative_row(cfg.residuals[usize::from(equation.contribution)])
        })
        .collect();

    // No fixture here limits, so no lane is reserved for the correction.
    let mut plan = StampPlan::new(&artifact.mir, &cfg.residuals, &rows, None);
    let (optimized, wanted) = optimize_cfg(&differentiated.function, &plan.wanted());
    plan.remap(&wanted);
    plan.drop_zeros(&optimized);
    plan
}

fn artifact(source: &str) -> CanonicalIrArtifact {
    VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("fixture must compile to canonical IR")
}
