//! The physical half of parser checks: what the parser read must reach the
//! device equation.
//!
//! A positional `AREA` or a reassigned instance field is something a parser
//! can get wrong without saying so. The parser's unit tests check the fields
//! it records; these run the deck through the engine and compare the
//! operating point with that of a deck writing the intended values out
//! explicitly. They live here, outside the crate, so the parser's own tests
//! never reach up into the engine.

use rspice_core::Engine;
use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::{Netlist, NetlistParseOptions};

#[test]
fn positional_device_area_reaches_the_diode_equation_in_instance_scope() {
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        // Check the actual diode current against the explicit, flattened
        // reference circuit, including independent sibling bindings.
        let current = |body: &str| {
            let netlist = Netlist::parse_with_options(
                &format!("Physical AREA\nV1 a 0 0.6\n{body}\n.model dm d (IS=1p)\n.end"),
                NetlistParseOptions {
                    expression_dialect: dialect,
                    ..Default::default()
                },
            )
            .unwrap();
            Engine::default()
                .run_dc_op(&netlist)
                .unwrap()
                .branch_currents[0]
        };
        assert_eq!(
            current("X1 a cell q=3\nX2 a cell q=7\n.subckt cell a q=11\nD1 a 0 dm 2*q\n.ends"),
            current("D1 a 0 dm 6\nD2 a 0 dm 14")
        );
    }
}

#[test]
fn winning_semiconductor_reassignments_reach_the_diode_equation() {
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        for nested in [false, true] {
            // The winning AREA and M must reach the actual diode equation.
            let current = |fields: &str| {
                let device = format!("D1 d 0 m {fields}\n.model m d (IS=1p)");
                let body = if nested {
                    format!("X1 d cell q=4\n.subckt cell d q=9\n{device}\n.ends")
                } else {
                    device
                };
                let netlist = Netlist::parse_with_options(
                    &format!("Physical assignment values\n.param q=4\nV1 d 0 0.6\n{body}\n.end"),
                    NetlistParseOptions {
                        expression_dialect: dialect,
                        ..Default::default()
                    },
                )
                .unwrap();
                Engine::default()
                    .run_dc_op(&netlist)
                    .unwrap()
                    .branch_currents[0]
            };
            assert_eq!(current("AREA=2 AREA={q} M=2 MULT=3"), current("AREA=4 M=3"));
        }
    }
}
