//! Convergence helpers for Newton-Raphson iteration
//!
//! This module provides:
//! - GMIN stepping for difficult circuits
//! - Source stepping for convergence
//! - Linear and nonlinear solver interfaces

use super::{DampingStrategy, Engine, SimulationError};
use crate::abort_signal::AbortSignal;
use crate::device::NonlinearConvergenceCriteria;
use crate::diagnostics::{
    ConvergenceDiagnostic, ConvergenceFailureClass, ConvergenceSite, ConvergenceSiteKind,
};
use crate::solver::{
    ArcLengthConfig, ArcLengthContinuation, PseudoTransient, SolverError, StaticMatrix,
};
use crate::{CircuitData, Value};

/// The trial point an operating-point probe evaluates at: the candidate
/// solution, the analysis time it is dated to, the analysis kind whose
/// evaluation phase applies, and the junction conductance floor added to every
/// nonlinear branch. A probe given the solution without the floor would be
/// reading a different circuit than the one being solved.
#[derive(Clone, Copy)]
pub(in crate::engine::convergence) struct OperatingPointProbe<'a> {
    pub solution: &'a [Value],
    pub time: Value,
    pub analysis: crate::xspice::AnalysisType,
    pub junction_gmin: Value,
}

/// One corrector run: where it starts, the damping state it carries across
/// its Newton steps, and the iteration ceiling it must finish inside.
pub(in crate::engine::convergence) struct CorrectorRun<'a> {
    pub initial_solution: &'a [Value],
    pub damping_state: &'a mut NewtonDampingState,
    pub max_iterations: usize,
}

/// One damping decision: the accepted point, the Newton proposal from it, and
/// the damping state that remembers what the previous steps did.
pub(in crate::engine::convergence) struct DampingStep<'a> {
    pub old: &'a [Value],
    pub proposal: &'a [Value],
    pub damping_state: &'a mut NewtonDampingState,
}

mod continuation;
mod damping;
mod fallback;
mod residuals;
mod solve;
mod stamping;
mod tolerances;

/// Linear equation family used by an accepted transient startup state.
///
/// Xyce Core windings can require the current-seeded inductor equations when
/// the ordinary ideal-short operating-point system is singular. Keeping this
/// provenance typed prevents a later audit from reconstructing a different
/// system and turning that mismatch into a topology error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::engine) enum TransientOperatingPointLinearSystem {
    IdealInductorShorts,
    CurrentSeededInductors,
}

#[derive(Debug, Clone, Copy)]
pub(in crate::engine) struct AcceptedTransientOperatingPointContract {
    pub(in crate::engine) linear_system: TransientOperatingPointLinearSystem,
    pub(in crate::engine) nodal_gmin: Value,
    /// Present when the state was accepted by the nonlinear transient
    /// operating-point equations. Linear startup has no nonlinear probe.
    pub(in crate::engine) junction_gmin: Option<Value>,
}

pub(in crate::engine) struct TransientOperatingPointSolution {
    pub(in crate::engine) values: Vec<Value>,
    /// `None` denotes a startup-directive-constrained state rather than an
    /// accepted unconstrained operating point. This includes a NODESET
    /// recovery seed and Xyce's hard-constrained `.IC` transient state.
    pub(in crate::engine) accepted_contract: Option<AcceptedTransientOperatingPointContract>,
}

#[derive(Debug, Clone, Copy)]
struct NewtonDampingState {
    pub(in crate::engine::convergence) bank_rose_alpha: Value,
    pub(in crate::engine::convergence) prev_step_norm: Option<Value>,
}

impl Default for NewtonDampingState {
    fn default() -> Self {
        Self {
            bank_rose_alpha: 1.0,
            prev_step_norm: None,
        }
    }
}

impl Engine {
    const MAX_NODE_VOLTAGE: Value = 1000.0;
    const MAX_DELTA_VOLTAGE_LIMIT: Value = 0.5;
    const BANK_ROSE_ALPHA_MIN: Value = 0.1;
    const BANK_ROSE_ALPHA_MAX: Value = 1.0;
    const ARMIJO_C1: Value = 1e-4;
    const LINE_SEARCH_BACKTRACK: Value = 0.5;
    const LINE_SEARCH_MAX_ITERS: usize = 8;
    const ARC_LENGTH_MAX_STEPS: usize = 128;
    const ABORT_POLL_MASK: usize = 0x7;
    const DC_RESIDUAL_STALL_LIMIT: usize = 3;
    const DC_LIMIT_CYCLE_HISTORY: usize = 64;
    const DC_LIMIT_CYCLE_HIT_LIMIT: usize = 3;
    const DC_LIMIT_CYCLE_MIN_PERIOD: usize = 4;
    const DC_LIMIT_CYCLE_MAX_TRACKED_VALUES: usize = 1024;
    const MAX_CONTINUATION_CORRECTOR_ITERS: usize = 512;
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod failure_attribution_tests {
    use super::*;
    use crate::Netlist;
    use crate::engine::SimulationConfig;
    use solve::SINGULAR_ROWS_SHOWN;

    /// Resolved so the deck cannot re-resolve the convergence aids a test
    /// deliberately turned off.
    fn engine(config: SimulationConfig) -> Engine {
        Engine::new_with_resolved_config(config)
    }

    fn parse(deck: &str) -> Netlist {
        Netlist::parse(deck).expect("test deck parses")
    }

    fn site_names(diagnostic: &ConvergenceDiagnostic) -> Vec<String> {
        diagnostic
            .sites
            .iter()
            .map(|site| site.name.to_ascii_lowercase())
            .collect()
    }

    #[test]
    fn a_floating_current_drive_names_the_node_with_no_dc_path() {
        let netlist = parse(
            "current-driven floating node\n\
             i1 0 out dc 1m\n\
             c1 out 0 1u\n\
             .op\n\
             .end\n",
        );
        let engine = engine(SimulationConfig::default());
        let error = engine
            .run_dc_op(&netlist)
            .expect_err("a current-driven floating node has no operating point");

        let rendered = error.to_string();
        assert!(
            rendered.contains("no DC path to ground"),
            "prose must be unchanged: {rendered}"
        );

        let diagnostic = engine
            .convergence_quality()
            .failure_diagnostic
            .expect("the refusal names its nodes");
        assert_eq!(
            diagnostic.class,
            ConvergenceFailureClass::NoDcPathToGround,
            "{diagnostic:?}"
        );
        assert!(
            diagnostic.describes(&rendered),
            "the attribution must pair with the error it was built for: {diagnostic:?}"
        );
        assert!(
            site_names(&diagnostic).contains(&"out".to_string()),
            "the floating node must be named: {diagnostic:?}"
        );
        assert!(
            diagnostic
                .sites
                .iter()
                .all(|site| site.kind == ConvergenceSiteKind::Node),
            "a DC-path refusal names nodes, not branches: {diagnostic:?}"
        );
        assert_eq!(diagnostic.elided_sites, 0);
    }

    /// No convergence aid may rescue the solve, so the failure the test
    /// inspects is the one the deck actually produced.
    fn unaided_config() -> SimulationConfig {
        let mut config = SimulationConfig::default();
        config.convergence_config.gmin_stepping = false;
        config.convergence_config.source_stepping = false;
        config.convergence_config.pseudo_transient = false;
        config.convergence_config.arc_length = false;
        config
    }

    #[test]
    fn an_unconditioned_floating_island_names_the_unconstrained_row() {
        let mut config = unaided_config();
        // Without the nodal conditioning floor the island's KCL row is
        // structurally empty, which is the singular system this attributes.
        config.convergence_config.gmin_target = 0.0;
        let netlist = parse(
            "floating island with no numerical conditioning\n\
             v1 in 0 5\n\
             r1 in mid 1k\n\
             r2 mid 0 1k\n\
             c1 island 0 1u\n\
             .op\n\
             .end\n",
        );
        let engine = engine(config);
        let error = engine
            .run_dc_op(&netlist)
            .expect_err("an unconditioned floating island has no equation");

        let rendered = error.to_string();
        let diagnostic = engine
            .convergence_quality()
            .failure_diagnostic
            .expect("a singular system names its deficient rows");
        assert_eq!(
            diagnostic.class,
            ConvergenceFailureClass::SingularSystem,
            "{rendered}"
        );
        assert!(diagnostic.describes(&rendered), "{diagnostic:?}");
        assert!(
            !diagnostic.sites.is_empty(),
            "the deficient rows must be named: {rendered}"
        );
        // Every named row the prose shows must be the same row the
        // attribution shows; one owner, two readers.
        for site in diagnostic.sites.iter().take(SINGULAR_ROWS_SHOWN) {
            assert!(
                rendered.contains(&site.name),
                "prose omits attributed row {}: {rendered}",
                site.name
            );
        }
    }

    #[test]
    fn a_newton_abort_names_its_worst_residual_nodes() {
        let mut config = unaided_config();
        config.max_iterations = 5;
        // The source demands a full amp through a reverse-biased junction
        // and a teraohm. The bias that would satisfy it is far outside the
        // solver's physical voltage clamp, so Newton cycles against the
        // clamp instead of settling.
        let netlist = parse(
            "reverse-driven junction with no reachable bias\n\
             i1 out 0 1\n\
             d1 out 0 dmod\n\
             r1 out 0 1e12\n\
             .model dmod d(is=1e-18 n=1)\n\
             .op\n\
             .end\n",
        );
        let engine = engine(config);
        let error = engine
            .run_dc_op(&netlist)
            .expect_err("no reachable bias satisfies the demanded current");
        assert!(
            matches!(error, SimulationError::ConvergenceFailed(_)),
            "unexpected failure class: {error}"
        );

        let rendered = error.to_string();
        let diagnostic = engine
            .convergence_quality()
            .failure_diagnostic
            .expect("a Newton abort names the equations it left violated");
        assert_eq!(
            diagnostic.class,
            ConvergenceFailureClass::NewtonNonConvergence
        );
        assert!(diagnostic.describes(&rendered), "{diagnostic:?}");
        assert!(!diagnostic.sites.is_empty());
        assert!(
            diagnostic
                .sites
                .iter()
                .all(|site| site.residual.is_some_and(|value| value.is_finite())),
            "a residual-ranked site must carry the residual it was ranked by: {diagnostic:?}"
        );
        let residuals: Vec<Value> = diagnostic
            .sites
            .iter()
            .filter_map(|site| site.residual)
            .collect();
        assert!(
            residuals.windows(2).all(|pair| pair[0] >= pair[1]),
            "sites must be ordered worst first: {residuals:?}"
        );
    }

    #[test]
    fn a_solve_that_succeeds_attributes_nothing() {
        let netlist = parse(
            "resistive divider\n\
             v1 in 0 5\n\
             r1 in out 1k\n\
             r2 out 0 1k\n\
             .op\n\
             .end\n",
        );
        let engine = engine(SimulationConfig::default());
        engine.run_dc_op(&netlist).expect("a divider solves");

        assert!(
            engine.convergence_quality().failure_diagnostic.is_none(),
            "a passing run must not allocate an attribution"
        );
    }

    #[test]
    fn an_attribution_refuses_an_error_it_was_not_built_for() {
        let diagnostic = ConvergenceDiagnostic {
            class: ConvergenceFailureClass::NoDcPathToGround,
            sites: vec![ConvergenceSite {
                name: "out".to_string(),
                kind: ConvergenceSiteKind::Node,
                residual: None,
            }],
            elided_sites: 0,
            failure_message: "no DC path to ground from node(s) OUT".to_string(),
        };

        assert!(diagnostic.describes(
            "Circuit error: .STEP PARAM R = 1: no DC path to ground from node(s) OUT and more"
        ));
        assert!(
            !diagnostic.describes("Convergence failed after 100 iterations"),
            "an unrelated failure must not borrow this attribution"
        );
    }
}
