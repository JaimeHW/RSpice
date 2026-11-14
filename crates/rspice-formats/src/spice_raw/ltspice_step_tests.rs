//! An LTspice `stepped` rawfile, `.step param r list 1k 2k 4k` over an RC
//! transient, written by LTspice 17.2.4 with its log beside it.

use super::*;

fn fixture(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../rspice-core-base/tests/fixtures/ltspice")
        .join(name);
    std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

#[test]
fn a_stepped_rawfile_reads_as_one_dataset_per_run_named_by_its_log() {
    let log = fixture("rc_step.log");
    let plots = decode_spice_raw_plots_with_log(
        &fixture("rc_step.raw"),
        Some(&log),
        rspice_core::ResourceLimits::default(),
    )
    .unwrap();
    let runs: Vec<_> = plots
        .iter()
        .map(|plot| {
            (
                plot.plot_name.as_str(),
                plot.step.clone().unwrap(),
                plot.dataset.coordinate.len(),
            )
        })
        .collect();
    let step = |index, r: f64| RawStep {
        index,
        coordinate: vec![("r".to_owned(), r)],
    };
    assert_eq!(
        runs,
        [
            ("Transient Analysis", step(0, 1000.0), 55),
            ("Transient Analysis", step(1, 2000.0), 31),
            ("Transient Analysis", step(2, 4000.0), 19),
        ]
    );
    for (plot, r) in plots.iter().zip([1e3, 2e3, 4e3]) {
        assert_eq!(plot.dataset.domain, crate::WaveformDomain::Transient);
        let time = &plot.dataset.coordinate;
        assert_eq!(time.first(), Some(&0.0));
        assert!(time.windows(2).all(|pair| pair[0] < pair[1]));
        let out = plot
            .dataset
            .signals
            .iter()
            .find(|signal| signal.name == "V(out)")
            .unwrap();
        // Each run charges its 1 nF through its own resistor. The deck caps
        // the step at 50 ns, which leaves LTspice's 1 us run up to 5.4e-4
        // off its closed form near the start: within its reltol of 1e-3 of
        // the 1 V swing.
        for (&t, &v) in time.iter().zip(&out.real) {
            let expected = 1.0 - (-t / (r * 1e-9)).exp();
            assert!(
                (v - expected).abs() < 1e-3,
                "r = {r}: V(out)({t:e}) = {v}, {expected}"
            );
        }
    }
}

#[test]
fn a_stepped_rawfile_without_its_log_numbers_its_runs() {
    let plots = decode_spice_raw_plots(
        &fixture("rc_step.raw"),
        rspice_core::ResourceLimits::default(),
    )
    .unwrap();
    let steps: Vec<_> = plots
        .iter()
        .map(|plot| plot.step.clone().unwrap())
        .collect();
    assert_eq!(
        steps,
        (0..3)
            .map(|index| RawStep {
                index,
                coordinate: Vec::new()
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn a_log_that_names_other_runs_names_none() {
    let mut log = fixture("rc_step.log");
    log.extend(".step r=8000\n".encode_utf16().flat_map(u16::to_le_bytes));
    let plots = decode_spice_raw_plots_with_log(
        &fixture("rc_step.raw"),
        Some(&log),
        rspice_core::ResourceLimits::default(),
    )
    .unwrap();
    assert!(plots.iter().all(|plot| {
        plot.step
            .as_ref()
            .is_some_and(|step| step.coordinate.is_empty())
    }));
}

#[test]
fn an_unstepped_ltspice_transient_is_one_dataset() {
    let plots = decode_spice_raw_plots(
        &fixture("rc_tran.raw"),
        rspice_core::ResourceLimits::default(),
    )
    .unwrap();
    assert_eq!(plots.len(), 1);
    assert_eq!(plots[0].step, None);
    assert_eq!(plots[0].dataset.coordinate.len(), 23);
}
