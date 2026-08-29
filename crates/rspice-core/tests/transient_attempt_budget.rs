use rspice_core::{Engine, Netlist, SimulationConfig, SpiceDialect};
use std::sync::Arc;

#[test]
fn valid_locked_run_can_exceed_the_removed_global_attempt_heuristic() {
    // The former DELMAX-derived ceiling stopped at 50,000 outer attempts even
    // though every local Newton step converged and the explicit result budgets
    // still had ample room. One extra prescribed step is the smallest exact
    // counterexample to that accidental global policy.
    const STEP_ATTEMPTS: usize = 50_001;
    let grid = (0..=STEP_ATTEMPTS)
        .map(|index| index as f64 / STEP_ATTEMPTS as f64)
        .collect::<Vec<_>>();
    let netlist = Netlist::parse(
        "bounded transient attempts\n\
         I1 out 0 0\n\
         R1 out 0 1k\n\
         .TRAN 1 1\n\
         .END\n",
    )
    .expect("linear transient deck parses");
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        locked_time_grid: Some(Arc::new(grid.clone())),
        ..SimulationConfig::default()
    });

    let result = engine
        .run_tran(&netlist, 1.0, 1.0)
        .expect("explicit resource budgets, not a derived attempt guess, govern the run");

    assert_eq!(result.time, grid);
    assert_eq!(result.time.last().copied(), Some(1.0));
}
