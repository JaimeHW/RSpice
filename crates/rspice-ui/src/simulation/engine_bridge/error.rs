//! Translating engine errors into user-facing ones.
//!
//! Engine diagnostics name internal nodes and matrix rows. This maps them
//! back onto the design objects a user can act on.

use super::EngineBridge;
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Translate core engine error to UI error.
    ///
    /// When the engine could name the circuit objects behind this failure,
    /// the translation keeps the prose and adds them. The engine records an
    /// attribution wherever a solve gives up, and a later convergence aid may
    /// still rescue that solve, so the recorded attribution is used only when
    /// it says it belongs to the error actually being translated.
    pub(super) fn translate_error(&self, err: rspice_core::SimulationError) -> SimulationError {
        let attribution = self.attribution_for(&err);
        let translated = self.translate_unattributed(err);
        match attribution {
            Some(attribution) => SimulationError::Attributed {
                message: translated.to_string(),
                attribution,
            },
            None => translated,
        }
    }

    /// The engine's attribution for `err`, if it recorded one for this error.
    ///
    /// The analyses run against engines resolved from `self.engine`, which
    /// share its metrics, so the bridge's own engine is where the record
    /// lands whichever entry point produced the failure.
    fn attribution_for(
        &self,
        err: &rspice_core::SimulationError,
    ) -> Option<crate::state::ConvergenceAttribution> {
        let rendered = err.to_string();
        let diagnostic = self.engine.convergence_quality().failure_diagnostic?;
        diagnostic
            .describes(&rendered)
            .then(|| crate::state::ConvergenceAttribution::from(&diagnostic))
    }

    fn translate_unattributed(&self, err: rspice_core::SimulationError) -> SimulationError {
        match err {
            rspice_core::SimulationError::Configuration(
                rspice_core::SimulationConfigError::ResourceLimit(error),
            )
            | rspice_core::SimulationError::ResourceLimit(error) => {
                SimulationError::ResourceLimit {
                    resource: error.resource.as_str().to_string(),
                    requested: error.requested,
                    limit: error.limit,
                }
            }
            rspice_core::SimulationError::Configuration(error) => {
                SimulationError::InvalidConfig(error.to_string())
            }
            rspice_core::SimulationError::BehavioralReference(error) => {
                SimulationError::BehavioralReference {
                    owner_name: error.owner_name,
                    canonical_owner_name: error.canonical_owner_name,
                    dependency_name: error.dependency_name,
                    canonical_dependency_name: error.canonical_dependency_name,
                    reason: error.reason.as_str().to_string(),
                }
            }
            rspice_core::SimulationError::Circuit(msg) => SimulationError::CircuitError(msg),
            rspice_core::SimulationError::Solver(solver_err) => {
                SimulationError::SolverError(solver_err.to_string())
            }
            rspice_core::SimulationError::Netlist(msg) => SimulationError::ParseError(msg),
            rspice_core::SimulationError::ConvergenceFailed(iterations) => {
                SimulationError::ConvergenceFailed {
                    iterations,
                    message: "Newton-Raphson iteration limit exceeded".to_string(),
                }
            }
            rspice_core::SimulationError::Aborted => SimulationError::Aborted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::config::AnalysisConfig;
    use crate::simulation::dialog::OpConfig;

    /// A node driven only by a current source and a capacitor: the operating
    /// point has no conductive path from it to ground, which is one of the
    /// two refusals `engine/core.rs:1309,1335` records an attribution for.
    const NO_DC_PATH_DECK: &str = "current-driven floating node\n\
         i1 0 out dc 1m\n\
         c1 out 0 1u\n\
         .op\n\
         .end\n";

    #[test]
    fn behavioral_reference_error_preserves_typed_fields() {
        let core_error = rspice_core::SimulationError::BehavioralReference(Box::new(
            rspice_core::device::BehavioralReferenceError {
                owner_name: "b2".to_string(),
                canonical_owner_name: "B2".to_string(),
                dependency_name: "b1".to_string(),
                canonical_dependency_name: "B1".to_string(),
                reason:
                    rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
            },
        ));
        let translated = EngineBridge::new().translate_error(core_error);

        assert_eq!(
            translated,
            SimulationError::BehavioralReference {
                owner_name: "b2".to_string(),
                canonical_owner_name: "B2".to_string(),
                dependency_name: "b1".to_string(),
                canonical_dependency_name: "B1".to_string(),
                reason: "lead_current_not_solution_variable".to_string(),
            }
        );
        assert_eq!(
            translated.to_string(),
            "Device instance B2: Problem with value for B1 in B2 \
             (lead_current_not_solution_variable)"
        );
    }

    #[test]
    fn a_topology_refusal_reaches_the_gui_naming_its_node() {
        let netlist = rspice_core::Netlist::parse(NO_DC_PATH_DECK).expect("test deck parses");
        let bridge = EngineBridge::new();

        // The prose a reader must be shown, read off the engine before any
        // attribution is in play.
        let expected_message = format!("Circuit error: {}", {
            let core_error = bridge
                .engine_for_netlist(&netlist)
                .run_dc_op(&netlist)
                .expect_err("a current-driven floating node has no operating point");
            let rspice_core::SimulationError::Circuit(message) = &core_error else {
                panic!("expected a circuit refusal, got {core_error}");
            };
            message.clone()
        });

        // Exactly the sequence an analysis runs, entered where the queue
        // enters it: `run_request` parses, `dispatch_analysis` picks the
        // operating-point arm, and that arm builds its own engine from a
        // configuration it resolved itself. If that engine stopped recording
        // into the bridge's metrics this would arrive as a bare message.
        let translated = bridge
            .run(&AnalysisConfig::DcOp(OpConfig::default()), NO_DC_PATH_DECK)
            .expect_err("the operating-point dispatch must refuse the same deck");

        let SimulationError::Attributed {
            message,
            attribution,
        } = &translated
        else {
            panic!("the refusal must arrive attributed, got {translated:?}");
        };
        assert_eq!(
            message, &expected_message,
            "the prose a person reads must be byte-identical to the unattributed form"
        );
        assert_eq!(translated.to_string(), expected_message);
        assert_eq!(
            attribution.class,
            crate::state::ConvergenceFailureClass::NoDcPathToGround
        );
        assert!(
            attribution
                .nets()
                .any(|net| net.eq_ignore_ascii_case("out")),
            "the floating node must reach the GUI by name: {attribution:?}"
        );
    }

    #[test]
    fn a_failure_the_engine_could_not_name_is_left_exactly_as_it_was() {
        let bridge = EngineBridge::new();

        let translated = bridge.translate_error(rspice_core::SimulationError::Netlist(
            "unterminated .subckt".to_owned(),
        ));

        assert_eq!(
            translated,
            SimulationError::ParseError("unterminated .subckt".to_owned()),
            "a failure that names no conductor must not gain an empty attribution"
        );
    }
}
