use super::*;
use crate::abort_signal::CountingAbort;
use crate::{Engine, Netlist};

fn accepted_values(resistors: &Resistors) -> Vec<(Value, [Value; 7])> {
    resistors
        .thermal
        .iter()
        .enumerate()
        .filter_map(|(index, state)| {
            state.as_ref().map(|state| {
                (
                    resistors.conductances[index],
                    [
                        state.temperature_celsius,
                        state.resistivity,
                        state.heat_capacity,
                        state.thermal_heat_capacity,
                        state.reported_resistance,
                        state.output_resistance,
                        state.output_conductance,
                    ],
                )
            })
        })
        .collect()
}

#[test]
fn cancellation_at_every_material_boundary_preserves_accepted_state() {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=4 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    let netlist = Netlist::parse(&format!(
        "* cancellable thermal preparation\n{functions}\n\
         R1 a 0 rm L=1 A=1\nR2 b 0 rm L=1 A=1\n\
         .MODEL rm R(LEVEL=2 RESISTIVITY={{IF(TEMP>27,f4(),100)}} HEATCAPACITY=1u)\n.END\n"
    ))
    .unwrap();
    let mut circuit = Engine::default().build_circuit(&netlist).unwrap();
    let solution = vec![1.0; circuit.matrix_size()];
    let before = accepted_values(&circuit.resistors);
    assert_eq!(before.len(), 2);
    let completed = CountingAbort::new(usize::MAX);
    let prepared = circuit
        .resistors
        .prepare_thermal_step(&solution, 1e-3, &completed)
        .unwrap();
    assert_eq!(accepted_values(&circuit.resistors), before);
    assert!(
        completed.count() > 20,
        "the material expressions must execute"
    );

    for boundary in 1..=completed.count() {
        let abort = CountingAbort::new(boundary - 1);
        let result = circuit
            .resistors
            .prepare_thermal_step(&solution, 1e-3, &abort);
        assert!(
            matches!(result, Err(ThermalUpdateError::Aborted)),
            "boundary {boundary}"
        );
        assert_eq!(abort.polls_after_abort(), 0, "boundary {boundary}");
        assert_eq!(
            accepted_values(&circuit.resistors),
            before,
            "boundary {boundary}"
        );
    }

    circuit.resistors.commit_thermal_step(prepared);
    for (_, values) in accepted_values(&circuit.resistors) {
        assert!(values[0] > 27.0);
        assert_eq!(values[1], 16.0);
    }
}
