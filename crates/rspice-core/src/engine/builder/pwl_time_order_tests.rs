//! A PWL table whose times decrease: the default refuses it, as LTspice
//! does; ngspice warns and plays it as written, and Xyce plays it as written
//! without a word.

use crate::config::{SimulationConfig, SpiceDialect};

fn build(
    dialect: SpiceDialect,
    source: &str,
) -> Result<crate::circuit::CircuitData, crate::engine::SimulationError> {
    crate::engine::Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
        .build_circuit(
            &crate::Netlist::parse(&format!("pwl order\n{source}\nR1 a 0 1k\n.end\n"))
                .expect("the deck parses"),
        )
}

#[test]
fn the_default_dialect_refuses_a_pwl_time_that_decreases() {
    for source in [
        "V1 a 0 PWL(0 0 2m 1 1m 2 3m 3)",
        "I1 0 a DC 0 PWL(0 0 2m 1 1m 2 3m 3)",
    ] {
        let error = build(SpiceDialect::BestAvailable, source)
            .err()
            .unwrap_or_else(|| panic!("{source} builds"))
            .to_string();
        assert!(
            error.contains(&format!("source '{}'", &source[..2]))
                && error.contains("PWL time 0.001 follows time 0.002"),
            "{error}"
        );
    }
    // A repeated time is a vertical step, which LTspice plays.
    build(
        SpiceDialect::BestAvailable,
        "V1 a 0 PWL(0 0 1m 1 1m 2 2m 3)",
    )
    .expect("a repeated PWL time builds");
}

#[test]
fn ngspice_and_xyce_play_a_decreasing_table_as_written() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let circuit =
            build(dialect, "V1 a 0 PWL(0 0 2m 1 1m 2 3m 3)").expect("the table is played");
        // ngspice-46 measures these on the same deck.
        for (time, ngspice) in [
            (0.5e-3, 0.25),
            (1e-3, 0.5),
            (1.5e-3, 0.75),
            (2.5e-3, 2.75),
            (3.5e-3, 3.0),
        ] {
            let value = circuit.voltage_sources.transient_value_at(0, time);
            assert!(
                (value - ngspice).abs() <= 1e-12,
                "{dialect:?} at {time:e}: {value}"
            );
        }
    }
}
