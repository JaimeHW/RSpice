//! Execution phases shared by the compiler and simulator backends.

/// Declaration assignments precede analog initial blocks, and both precede
/// ordinary analog evaluation. These phases repeat when a new analysis starts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum AnalogEvaluationPhase {
    #[default]
    Evaluation = 0,
    Declarations = 1,
    Initialization = 2,
}

/// Solver phase within one physical analysis. Equilibrium includes the
/// operating/initial-condition solve preceding AC, noise, or transient work.
/// This is independent of initial/final-step events and initializer lifetime.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum AnalogAnalysisPhase {
    #[default]
    Point = 0,
    Equilibrium = 1,
    Nodeset = 2,
}

impl AnalogAnalysisPhase {
    #[inline]
    pub const fn is_equilibrium(self) -> bool {
        !matches!(self, Self::Point)
    }
}

/// Stable query IDs shared by bytecode, native code, Wasm, and generated Rust.
/// IDs 7/8 are global event flags; 10–14 identify the physical analysis for
/// filtering those events, rather than matching solver-phase qualifiers.
pub const ANALYSIS_QUERY_COUNT: u8 = 15;

const ANALYSIS_NAMES: &[(&str, u8)] = &[
    ("dc", 0),
    ("op", 0),
    ("ac", 1),
    ("tran", 2),
    ("transient", 2),
    ("noise", 3),
    ("ic", 4),
    ("static", 5),
    ("smallsig", 6),
    ("smallsignal", 6),
    ("small_signal", 6),
    ("__rspice_initial_step", 7),
    ("__rspice_final_step", 8),
    ("nodeset", 9),
    ("__rspice_scope_dc", 10),
    ("__rspice_scope_op", 10),
    ("__rspice_scope_ac", 11),
    ("__rspice_scope_tran", 12),
    ("__rspice_scope_transient", 12),
    ("__rspice_scope_noise", 13),
    ("__rspice_scope_ic", 14),
];
#[inline]
pub fn analysis_query_id(name: &str) -> Option<u8> {
    ANALYSIS_NAMES
        .iter()
        .find_map(|&(candidate, id)| name.eq_ignore_ascii_case(candidate).then_some(id))
}

/// Canonical spelling for a query ID; aliases share the first spelling.
pub fn analysis_query_name(id: u8) -> Option<&'static str> {
    ANALYSIS_NAMES
        .iter()
        .find_map(|&(name, candidate)| (candidate == id).then_some(name))
}

/// Every active spelling, for evaluators whose input contract uses name sets.
pub fn active_analysis_query_names(mask: u32) -> impl Iterator<Item = &'static str> {
    ANALYSIS_NAMES
        .iter()
        .filter_map(move |&(name, id)| (mask & (1 << id) != 0).then_some(name))
}

/// Analysis predicates from VAMS-2023 4.6.1, plus RSpice's existing aliases.
/// The physical code remains 0=DC, 1=AC, 2=tran, 3=noise, 4=explicit IC.
#[inline]
pub const fn analysis_query_mask(
    analysis: u8,
    phase: AnalogAnalysisPhase,
    initial_step: bool,
    final_step: bool,
) -> u32 {
    if analysis > 4 {
        return 0;
    }
    let mut mask = (1_u32 << analysis) | (1_u32 << (10 + analysis));
    if matches!(analysis, 0 | 4) || phase.is_equilibrium() {
        mask |= 1 << 5;
    }
    if analysis == 2 && phase.is_equilibrium() {
        mask |= 1 << 4;
    }
    if matches!(analysis, 1 | 3) {
        mask |= 1 << 6;
    }
    if initial_step {
        mask |= 1 << 7;
    }
    if final_step {
        mask |= 1 << 8;
    }
    if matches!(phase, AnalogAnalysisPhase::Nodeset) {
        mask |= 1 << 9;
    }
    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_analysis_and_solver_phase_follow_the_lrm_table() {
        use AnalogAnalysisPhase::{Equilibrium, Nodeset, Point};
        // Public predicates in bit order: DC, AC, tran, noise, IC, static,
        // small-signal alias, initial flag, final flag, nodeset.
        for (analysis, phase, expected) in [
            (0, Point, 0b0000100001),
            (0, Nodeset, 0b1000100001),
            (1, Point, 0b0001000010),
            (1, Equilibrium, 0b0001100010),
            (1, Nodeset, 0b1001100010),
            (2, Point, 0b0000000100),
            (2, Equilibrium, 0b0000110100),
            (2, Nodeset, 0b1000110100),
            (3, Point, 0b0001001000),
            (3, Equilibrium, 0b0001101000),
            (3, Nodeset, 0b1001101000),
            (4, Point, 0b0000110000),
        ] {
            let mask = analysis_query_mask(analysis, phase, false, false);
            assert_eq!(
                mask & 0x3ff,
                expected,
                "analysis {analysis}, phase {phase:?}"
            );
            assert_eq!(
                mask >> 10,
                1 << analysis,
                "global events retain the physical identity"
            );
            assert_eq!(
                analysis_query_mask(analysis, phase, true, true),
                mask | (1 << 7) | (1 << 8)
            );
        }
        assert_eq!(analysis_query_mask(255, Nodeset, true, true), 0);
    }

    #[test]
    fn aliases_and_global_event_filters_use_one_query_vocabulary() {
        assert_eq!(analysis_query_id("AC"), Some(1));
        assert_eq!(analysis_query_id("transient"), Some(2));
        assert_eq!(analysis_query_id("nodeset"), Some(9));
        assert_eq!(analysis_query_id("__rspice_scope_ac"), Some(11));
        assert_eq!(analysis_query_id("__rspice_scope_static"), None);
        assert_eq!(analysis_query_id("unimplemented_analysis"), None);
    }

    #[test]
    fn generated_context_observes_solver_phase_independently_of_step_flags() {
        use crate::{GeneratedAnalysisKind, GeneratedEvalContext};
        let queries = [
            "dc",
            "ac",
            "tran",
            "noise",
            "ic",
            "static",
            "smallsig",
            "__rspice_initial_step",
            "__rspice_final_step",
            "nodeset",
            "__rspice_scope_dc",
            "__rspice_scope_ac",
            "__rspice_scope_tran",
            "__rspice_scope_noise",
            "__rspice_scope_ic",
        ];
        for kind in [
            GeneratedAnalysisKind::Dc,
            GeneratedAnalysisKind::Ac,
            GeneratedAnalysisKind::Tran,
            GeneratedAnalysisKind::Noise,
            GeneratedAnalysisKind::Ic,
        ] {
            for phase in [
                AnalogAnalysisPhase::Point,
                AnalogAnalysisPhase::Equilibrium,
                AnalogAnalysisPhase::Nodeset,
            ] {
                for (initial, final_step) in
                    [(false, false), (true, false), (false, true), (true, true)]
                {
                    let ctx = GeneratedEvalContext::with_analysis_step(
                        &[],
                        300.15,
                        0,
                        kind,
                        initial,
                        final_step,
                    )
                    .with_analysis_phase(phase);
                    let mask = analysis_query_mask(kind.code(), phase, initial, final_step);
                    for (id, query) in queries.iter().enumerate() {
                        assert_eq!(
                            ctx.analysis(query),
                            mask & (1 << id) != 0,
                            "{kind:?} {phase:?} {query}"
                        );
                    }
                    assert_eq!(ctx.analysis_ic(), ctx.analysis("ic"));
                    assert_eq!(ctx.analysis_static(), ctx.analysis("static"));
                    assert_eq!(ctx.analysis_nodeset(), ctx.analysis("nodeset"));
                    assert!(!ctx.analysis("unknown_analysis"));
                }
            }
        }
    }
}
