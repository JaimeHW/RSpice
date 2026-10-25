//! Engine reports a run retains: the device operating-point report and the
//! ranked noise summary save and reopen with every label the engine emits,
//! and text outside their vocabulary still refuses the write.

use super::*;

/// Solve a deck and hand back the operating-point report the engine really
/// produced, so the round trip below is judged against emitted labels rather
/// than against labels a fixture chose.
fn solved_device_op_report(deck: &str) -> rspice_core::circuit::DeviceOpReport {
    let netlist = rspice_core::netlist::Netlist::parse(deck).expect("op deck parses");
    let (_, report) =
        rspice_core::engine::Engine::new(rspice_core::engine::SimulationConfig::default())
            .run_dc_op_with_report(&netlist)
            .expect("operating point solves");
    report
}

fn run_retaining(report: rspice_core::circuit::DeviceOpReport) -> SimulationRun {
    let mut run = SimulationRun::new(61);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcOp, "Operating point").with_device_op(report),
    );
    seal_legacy_unattributed(&mut run);
    run
}

/// A retained operating-point report is engine output, and every label in it
/// has to be one the reader can restore, because a label it cannot resolve
/// fails validation and refuses the whole project.
///
/// Both families here were refused: the diode reports a junction capacitance
/// and the VDMOS reports a device family, an operating region and a dissipation
/// figure that the reader's vocabulary never carried, so an operating point
/// containing either could not be written at all.
#[test]
fn a_run_retaining_a_device_operating_point_saves_and_reopens() {
    let diode = "\
* junction diode operating point
v1 in 0 dc 5
r1 in a 1k
d1 a 0 dmod
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let vdmos = "\
* power VDMOS operating point
vd d 0 dc 10
vg g 0 dc 5
m1 d g 0 0 irfmod W=0.386 L=2.5u
.MODEL irfmod NMOS LEVEL=18 VTO=3.5 RS=0.005 M=3
.op
.end
";

    for (family, deck, quantity) in [("DIODE", diode, "cd"), ("VDMOS", vdmos, "power")] {
        let report = solved_device_op_report(deck);
        let emitted = report
            .entries
            .iter()
            .find(|entry| entry.device_kind == family)
            .unwrap_or_else(|| panic!("the {family} deck reports that family"));
        assert!(
            emitted.params.iter().any(|(name, _)| *name == quantity),
            "the {family} entry reports {quantity}, the quantity the reader used to refuse"
        );

        let mut simulation = SimulationState::default();
        simulation.retained.next_run_id = 61;
        simulation.retained.runs = vec![run_retaining(report.clone())].into();

        let persisted = crate::io::capture_simulation_results(&simulation);
        persisted
            .validate()
            .unwrap_or_else(|error| panic!("a {family} operating point is persistable: {error}"));

        let mut reloaded = SimulationState::default();
        crate::io::restore_simulation_results(persisted, &mut reloaded)
            .unwrap_or_else(|error| panic!("a saved {family} operating point reopens: {error}"));

        let restored = reloaded.retained.runs[0].analyses[0]
            .device_op
            .as_ref()
            .expect("the reopened run still carries its operating-point report");
        assert_eq!(restored.entries.len(), report.entries.len());
        for (before, after) in report.entries.iter().zip(&restored.entries) {
            assert_eq!(before.name, after.name);
            assert_eq!(before.device_kind, after.device_kind);
            assert_eq!(before.region, after.region);
            assert_eq!(
                before
                    .params
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>(),
                after
                    .params
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>(),
                "{family} quantity names survive the round trip"
            );
        }
    }
}

/// Completing the vocabulary is not the same as dropping the check: text the
/// engine has no label for still refuses the write, because restoring it would
/// invent a quantity the report never carried.
#[test]
fn an_operating_point_label_outside_the_vocabulary_is_still_refused() {
    let report = rspice_core::circuit::DeviceOpReport {
        entries: vec![rspice_core::circuit::DeviceOpEntry {
            name: "XU1".to_owned(),
            device_kind: "DIODE",
            region: None,
            params: vec![("vd", 0.7), ("vendor-quantity", 1.0)],
        }],
    };
    let mut simulation = SimulationState::default();
    simulation.retained.next_run_id = 61;
    simulation.retained.runs = vec![run_retaining(report)].into();

    let error = crate::io::capture_simulation_results(&simulation)
        .validate()
        .expect_err("an unrecognized quantity refuses the write");
    assert!(
        error.contains("unknown static label 'vendor-quantity'"),
        "{error}"
    );
}

/// Solve a deck's noise analysis and hand back the ranked contributor summary
/// the engine really produced, so the round trip below is judged against
/// emitted mechanisms rather than against mechanisms a fixture chose.
fn solved_noise_summary(deck: &str, output: &str, input: &str) -> crate::state::NoiseSummary {
    let netlist = rspice_core::netlist::Netlist::parse(deck).expect("noise deck parses");
    let frequencies = [1.0e1, 1.0e2, 1.0e3, 1.0e4, 1.0e5];
    let results =
        rspice_core::engine::Engine::new(rspice_core::engine::SimulationConfig::default())
            .run_noise_named_with_input_source(&netlist, output, None, input, &frequencies, 300.15)
            .expect("noise analysis runs");
    let integrated = rspice_core::analysis::IntegratedNoise::new(results);
    crate::state::NoiseSummary {
        input_quantity: None,
        conversion: None,
        noise_figure: None,
        rows: integrated
            .contribution_summary()
            .into_iter()
            .map(|contribution| crate::state::NoiseContributorRow {
                device: contribution.device_name,
                mechanism: contribution.mechanism,
                power: contribution.integrated_power,
                share_pct: contribution.percentage,
            })
            .collect(),
        total_rms: Some(integrated.total_output_noise()),
        input_rms: Some(integrated.total_input_referred_noise()),
        band: (frequencies[0], frequencies[frequencies.len() - 1]),
    }
}

fn run_retaining_noise(summary: crate::state::NoiseSummary) -> SimulationRun {
    let mut run = SimulationRun::new(62);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Noise, "Noise").with_noise_summary(summary),
    );
    seal_legacy_unattributed(&mut run);
    run
}

/// A ranked noise summary is engine output, and its mechanisms come from the
/// device model rather than from the broad physical class the reader used to
/// accept. A MOSFET names its channel and series-resistance sources, a bipolar
/// names its transport, base and parasitic-resistance sources, and retaining
/// the top contributors is the default, so a run containing either refused the
/// whole project on save.
#[test]
fn a_run_retaining_a_noise_summary_saves_and_reopens() {
    let mosfet = "\
* classic level-1 MOSFET noise
vdd dd 0 dc 5
rl dd d 10k
vin g 0 dc 2 ac 1
m1 d g 0 0 nmod w=10u l=1u
.model nmod NMOS (LEVEL=1 VTO=1 KP=100u RD=10 RS=10 KF=1e-24 AF=1)
.end
";
    let bipolar = "\
* Gummel-Poon bipolar noise
vcc cc 0 dc 10
rl cc c 10k
vin bb 0 dc 0.75 ac 1
rb bb b 1k
q1 c b 0 qmod
.model qmod NPN (IS=1e-16 BF=100 RB=100 RC=10 RE=1 KF=1e-14 AF=1)
.end
";

    for (family, deck, output, mechanisms) in [
        ("MOSFET", mosfet, "d", ["ID", "FN", "RD", "RS"].as_slice()),
        (
            "BJT",
            bipolar,
            "c",
            ["IC", "IB", "FN", "RB", "RC", "RE"].as_slice(),
        ),
    ] {
        let summary = solved_noise_summary(deck, output, "vin");
        let emitted = summary
            .rows
            .iter()
            .map(|row| row.mechanism.as_str())
            .collect::<Vec<_>>();
        for mechanism in mechanisms {
            assert!(
                emitted.contains(mechanism),
                "the {family} deck contributes {mechanism}, one of the mechanisms the reader refused; got {emitted:?}"
            );
        }

        let mut simulation = SimulationState::default();
        simulation.retained.next_run_id = 62;
        simulation.retained.runs = vec![run_retaining_noise(summary.clone())].into();

        let persisted = crate::io::capture_simulation_results(&simulation);
        persisted
            .validate()
            .unwrap_or_else(|error| panic!("a {family} noise summary is persistable: {error}"));

        let mut reloaded = SimulationState::default();
        crate::io::restore_simulation_results(persisted, &mut reloaded)
            .unwrap_or_else(|error| panic!("a saved {family} noise summary reopens: {error}"));

        let restored = reloaded.retained.runs[0].analyses[0]
            .noise_summary
            .as_ref()
            .expect("the reopened run still carries its noise summary");
        assert_eq!(
            restored
                .rows
                .iter()
                .map(|row| (row.device.as_str(), row.mechanism.as_str()))
                .collect::<Vec<_>>(),
            summary
                .rows
                .iter()
                .map(|row| (row.device.as_str(), row.mechanism.as_str()))
                .collect::<Vec<_>>(),
            "{family} contributor identities survive the round trip"
        );
    }
}

/// Bounding the mechanism is not the same as dropping the check: text no
/// emitter can compose still refuses the write, because a summary is not a
/// place to put free prose or unbounded input.
#[test]
fn a_noise_mechanism_outside_the_persistable_shape_is_still_refused() {
    for mechanism in [
        String::new(),
        "channel thermal".to_owned(),
        "a".repeat(rspice_core::analysis::NOISE_MECHANISM_MAX_BYTES + 1),
    ] {
        let summary = crate::state::NoiseSummary {
            input_quantity: None,
            conversion: None,
            noise_figure: None,
            rows: vec![crate::state::NoiseContributorRow {
                device: "M1".to_owned(),
                mechanism,
                power: 1e-18,
                share_pct: 100.0,
            }],
            total_rms: Some(1e-9),
            input_rms: Some(1e-9),
            band: (1.0, 1.0e5),
        };
        let mut simulation = SimulationState::default();
        simulation.retained.next_run_id = 62;
        simulation.retained.runs = vec![run_retaining_noise(summary)].into();

        let error = crate::io::capture_simulation_results(&simulation)
            .validate()
            .expect_err("a mechanism outside the shape refuses the write");
        assert!(error.contains("is not a noise mechanism"), "{error}");
    }
}
