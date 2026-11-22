//! What a small design's run keeps with no outputs of its own: every node,
//! and every part's current, so a current can be plotted without running
//! again.

use super::block_cross_probe::run;
use super::pipeline::edit_the_transient;
use super::startup::rc;

/// The RC's run keeps the current through each of its parts beside its
/// nodes, in the data browser rather than on the plot, and the current is
/// the circuit's: started from rest, what VIN drives through R1 charges C1.
#[test]
fn a_small_designs_run_keeps_every_parts_current() {
    let mut app = rc();
    edit_the_transient(&mut app.state, |draft| draft.uic = true);
    run(&mut app);

    let run = &app.state.model.simulation.retained.runs[0];
    let waves = run
        .analyses
        .iter()
        .flat_map(|analysis| analysis.waveforms.iter())
        .collect::<Vec<_>>();
    let wave = |name: &str| {
        waves
            .iter()
            .find(|wave| wave.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| {
                panic!(
                    "the run keeps {name}: {:?}",
                    waves
                        .iter()
                        .map(|wave| wave.name.as_str())
                        .collect::<Vec<_>>()
                )
            })
    };
    for name in ["I(VIN)", "I(R1)", "I(C1)"] {
        assert!(!wave(name).visible, "{name} waits in the data browser");
    }
    assert!(
        wave("V(out)").visible,
        "the node the design names is plotted"
    );

    let resistor = wave("I(R1)");
    let capacitor = wave("I(C1)");
    let peak = resistor
        .y
        .samples(..)
        .fold(0.0_f64, |peak, current| peak.max(current.abs()));
    assert!(
        peak > 0.5e-3,
        "VIN charges C1 through R1: {peak:e} A at most"
    );
    let worst = resistor
        .y
        .samples(..)
        .zip(capacitor.y.samples(..))
        .map(|(through_r, into_c)| (through_r.abs() - into_c.abs()).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        worst < 1e-9,
        "one series current flows through R1 and C1: worst {worst:e} A"
    );
}
