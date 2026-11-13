//! A Spectre `statistics` block is executed from the run coordinate.
//!
//! Monte Carlo stamps its own statistical coordinate per trial. Every other
//! coordinate — plain, `.STEP`, `.TEMP`, or DATA — gets one from the canonical
//! deck-plan materializer, so a swept statistical deck draws a distinct sample
//! per point and replays bit-for-bit from the same plan.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::execution::DeckPlan;
use rspice_core::netlist::Netlist;

struct TempDeckDir(PathBuf);

impl TempDeckDir {
    fn new(test_name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is after UNIX epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "rspice_spectre_statistics_coordinates_{}_{}_{}",
            test_name,
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&dir).expect("create temp deck dir");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDeckDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

const STATISTICS_LIBRARY: &str = "simulator lang=spectre\n\
                                  statistics {\n\
                                  process {\n\
                                  vary rtop dist=gauss std=100\n\
                                  }\n\
                                  }\n";

/// A resistive divider whose top resistor is the varied process parameter.
/// Neither resistor declares a temperature coefficient, so `V(out)` moves
/// only when the statistical draw moves.
fn swept_statistical_deck(directory: &Path, sweep: &str) -> Netlist {
    let library = directory.join("process.scs");
    fs::write(&library, STATISTICS_LIBRARY).expect("write Spectre statistics library");
    let deck = directory.join("divider.cir");
    fs::write(
        &deck,
        format!(
            "Spectre statistics under an authored sweep\n\
             .param rtop=1k\n\
             .include \"{}\"\n\
             V1 in 0 1\n\
             R1 in out {{rtop}}\n\
             R2 out 0 1k\n\
             {sweep}\n\
             .op\n\
             .end\n",
            library.display().to_string().replace('\\', "/")
        ),
    )
    .expect("write statistical deck");
    Netlist::parse_file(&deck).expect("statistical deck parses")
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

/// `V(out)` at every planned coordinate, as raw bits so replay equality is
/// exact rather than approximate.
fn swept_output_bits(netlist: &Netlist) -> Vec<u64> {
    let engine = engine();
    let plan = DeckPlan::from_netlist(netlist, &engine.config().resource_limits)
        .expect("deck plan is canonical");
    let materializer = engine
        .prepare_deck_plan_materializer(netlist, &plan)
        .expect("deck plan materializes");
    assert!(
        materializer.len() >= 2,
        "the sweep must plan at least two coordinates, got {}",
        materializer.len()
    );
    (0..materializer.len())
        .map(|index| {
            let run = materializer
                .materialize_run(index)
                .expect("coordinate materializes");
            let result = engine
                .run_dc_op(run.netlist())
                .expect("statistical operating point converges");
            result
                .try_voltage_named("out")
                .expect("V(out) is reported")
                .to_bits()
        })
        .collect()
}

#[test]
fn temperature_sweep_draws_one_sample_per_coordinate_and_replays_exactly() {
    let directory = TempDeckDir::new("temp_sweep");
    let netlist = swept_statistical_deck(directory.path(), ".TEMP 20 40 60");
    assert!(
        !netlist.spectre_statistics.variations.is_empty(),
        "the included Spectre library must contribute a statistical plan"
    );

    let first = swept_output_bits(&netlist);
    let replay = swept_output_bits(&netlist);
    assert_eq!(
        first, replay,
        "the same plan and seed must replay bit-for-bit"
    );

    assert_eq!(first.len(), 3);
    assert_ne!(
        first[0], first[1],
        "two temperature coordinates must draw different samples"
    );
    assert_ne!(
        first[1], first[2],
        "two temperature coordinates must draw different samples"
    );

    // The nominal divider is exactly 0.5; a coordinate that never sampled
    // would land there.
    for (index, bits) in first.iter().enumerate() {
        let voltage = f64::from_bits(*bits);
        assert!(
            (voltage - 0.5).abs() > 1e-6,
            "coordinate {index} ran nominal instead of sampling: V(out)={voltage}"
        );
        assert!(
            voltage.is_finite() && voltage > 0.0 && voltage < 1.0,
            "coordinate {index} produced an implausible divider output {voltage}"
        );
    }
}

#[test]
fn parameter_step_draws_one_sample_per_coordinate_and_replays_exactly() {
    let directory = TempDeckDir::new("param_step");
    let netlist = swept_statistical_deck(directory.path(), ".STEP PARAM rtop 1k 3k 1k");

    let first = swept_output_bits(&netlist);
    let replay = swept_output_bits(&netlist);
    assert_eq!(
        first, replay,
        "the same plan and seed must replay bit-for-bit"
    );
    assert_eq!(first.len(), 3);
    assert_ne!(first[0], first[1]);
    assert_ne!(first[1], first[2]);

    // Each coordinate re-centres the distribution on its own swept nominal,
    // so the outputs must fall in the swept order even after perturbation:
    // a 100-ohm standard deviation cannot cross a 1k step.
    let voltages = first
        .iter()
        .map(|bits| f64::from_bits(*bits))
        .collect::<Vec<_>>();
    assert!(
        voltages[0] > voltages[1] && voltages[1] > voltages[2],
        "sampled dividers must stay ordered by their swept nominal: {voltages:?}"
    );
}

#[test]
fn a_deck_without_a_statistics_block_is_untouched_by_coordinate_stamping() {
    let directory = TempDeckDir::new("no_statistics");
    let deck = directory.path().join("plain.cir");
    fs::write(
        &deck,
        "plain divider\n\
         .param rtop=1k\n\
         V1 in 0 1\n\
         R1 in out {rtop}\n\
         R2 out 0 1k\n\
         .TEMP 20 40\n\
         .op\n\
         .end\n",
    )
    .expect("write plain deck");
    let netlist = Netlist::parse_file(&deck).expect("plain deck parses");

    for bits in swept_output_bits(&netlist) {
        let voltage = f64::from_bits(bits);
        assert!(
            (voltage - 0.5).abs() <= 1e-9,
            "a deck with no statistics block must stay nominal, got {voltage}"
        );
    }
}
