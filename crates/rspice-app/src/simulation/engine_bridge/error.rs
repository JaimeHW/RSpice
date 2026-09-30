//! Translating engine errors into user-facing ones.
//!
//! Engine diagnostics name internal nodes and matrix rows. This maps them
//! back onto the design objects a user can act on.

use super::EngineBridge;
use rspice_simulation::error::SimulationError;

impl EngineBridge {
    pub(crate) fn translate_error(&self, err: rspice_core::SimulationError) -> SimulationError {
        SimulationError::from_engine(&self.engine, err)
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
