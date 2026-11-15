//! A transient run written through `StoreSink` reads back from its segment as exactly the columns
//! the same run returns in memory: the time axis, every saved node voltage and branch current, NaN
//! and signed zero included.

use rspice_core::NoAbort;
use rspice_core::engine::{Engine, TransientResult, TransientStartupMode};
use rspice_core::netlist::Netlist;
use rspice_results_store::{MemoryIo, RunStatus, SegmentReader, SegmentState, StoreSink};

/// An RC step, a rectifier, a capacitor `IC=` start, and an RC driven through XSPICE bridges.
const DECKS: &[(&str, &str, f64, f64, TransientStartupMode)] = &[
    (
        "rc",
        "rc step\nv1 in 0 pulse(0 1 10n 1n 1n 200n 400n)\nr1 in out 1k\nc1 out 0 100p\n.end\n",
        1e-6,
        5e-9,
        TransientStartupMode::OperatingPoint,
    ),
    (
        "rectifier",
        "rectifier\nv1 in 0 sin(0 5 1meg)\nd1 in out dmod\nr1 out 0 1k\nc1 out 0 10n\n.model dmod d(is=1e-14 cjo=2p)\n.end\n",
        3e-6,
        10e-9,
        TransientStartupMode::OperatingPoint,
    ),
    (
        "capacitor ic",
        "capacitor ic\nc1 a 0 1n ic=1\nr1 a b 1k\nc2 b 0 2n ic=-0.5\nr2 b 0 2k\n.end\n",
        5e-6,
        20e-9,
        TransientStartupMode::Uic,
    ),
    (
        "xspice bridges",
        "xspice bridges\nvin a 0 pwl(0 0 1n 0 1.1n 3.3)\na_adc [a] [d] adc\na_dac [d] [drv] dac\n.model adc adc_bridge (in_low=0.8 in_high=2.0 rise_delay=1p fall_delay=1p)\n.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)\nr1 drv out 1k\nc1 out 0 1p\n.end\n",
        5e-9,
        0.05e-9,
        TransientStartupMode::OperatingPoint,
    ),
];

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// The saved columns of an in-memory result, in the order a run writes them to its sink: node
/// voltages in node order, then branch currents in branch order. A column output retention left
/// out is empty.
fn saved_columns(result: &TransientResult) -> Vec<(&str, &[f64])> {
    let voltages = result
        .node_names
        .iter()
        .zip(&result.voltages)
        .filter(|(_, column)| !column.is_empty());
    let currents = result
        .branch_names
        .iter()
        .zip(&result.branch_currents)
        .filter(|(_, column)| !column.is_empty());
    voltages
        .chain(currents)
        .map(|(name, column)| (name.as_str(), column.as_slice()))
        .collect()
}

#[test]
fn a_transient_written_through_the_store_reads_back_as_the_in_memory_result() {
    for &(label, deck, tstop, max_step, startup) in DECKS {
        let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("{label}: {error}"));
        let engine = Engine::default();
        let memory = engine
            .run_tran_with_startup_mode_and_abort(&netlist, tstop, max_step, startup, &NoAbort)
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        let mut store = StoreSink::new(MemoryIo::new());
        let (streamed, receipt) = engine
            .run_tran_into_sink_with_startup_mode_and_abort(
                &netlist, tstop, max_step, startup, &NoAbort, &mut store,
            )
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        assert!(
            streamed.time.is_empty(),
            "{label}: the rows went to the sink"
        );
        assert!(
            streamed
                .voltages
                .iter()
                .chain(&streamed.branch_currents)
                .all(Vec::is_empty),
            "{label}: the waveforms went to the sink"
        );
        assert_eq!(
            bits(&streamed.step_sizes),
            bits(&memory.step_sizes),
            "{label}: the same accepted steps"
        );
        assert_eq!(
            streamed.digital_traces, memory.digital_traces,
            "{label}: event traces"
        );

        let columns = saved_columns(&memory);
        assert_eq!(receipt.rows, memory.time.len() as u64, "{label}: rows");
        let mut reader = SegmentReader::open(store.into_io().unwrap()).unwrap();
        assert!(
            matches!(
                reader.state(),
                SegmentState::Complete {
                    status: RunStatus::Completed,
                    ..
                }
            ),
            "{label}: the segment ended"
        );
        assert_eq!(reader.slots(), columns.len(), "{label}: slots");
        let rows = memory.time.len();
        assert_eq!(
            bits(&reader.read_x(0, rows).unwrap()),
            bits(&memory.time),
            "{label}: time"
        );
        for (slot, (name, column)) in columns.iter().enumerate() {
            assert_eq!(
                reader.schema().signals[slot].name,
                *name,
                "{label}: slot {slot}"
            );
            assert_eq!(
                bits(&reader.read_slot(slot, 0, rows).unwrap()),
                bits(column),
                "{label}: {name}"
            );
        }
    }
}

#[test]
fn a_deck_that_reads_whole_waveforms_after_the_run_is_refused_on_the_sink_route() {
    let netlist = Netlist::parse(
        "fft\nv1 in 0 sin(0 1 1meg)\nr1 in 0 1k\n.tran 1n 4u\n.fft v(in) np=1024\n.end\n",
    )
    .unwrap();
    let mut store = StoreSink::new(MemoryIo::new());
    let error = Engine::default()
        .run_tran_into_sink_with_startup_mode_and_abort(
            &netlist,
            4e-6,
            1e-9,
            TransientStartupMode::OperatingPoint,
            &NoAbort,
            &mut store,
        )
        .unwrap_err();
    assert!(
        error
            .to_string()
            .contains(".FFT reads the transient waveforms"),
        "{error}"
    );
    assert!(store.into_io().unwrap().bytes().is_empty());
}
