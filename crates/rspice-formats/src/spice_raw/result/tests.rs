use super::*;
use crate::WaveformDomain;
use crate::spice_raw::{RawReadError, decode_spice_raw, decode_spice_raw_plots};
use crate::waveform_io::result::EXPORT_TEST_LIMITS;
use rspice_results::events::{
    DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
    DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
};

const NO_EVENTS: &dyn Fn(&str) -> bool = &|_| false;
const EVERY_EVENT: &dyn Fn(&str) -> bool = &|_| true;

fn encode(
    analysis: &AnalysisResult<RetainedWaveform>,
    format: RawFormat,
    events: &dyn Fn(&str) -> bool,
) -> RawExport {
    encode_result_raw(
        analysis,
        &analysis.waveforms.iter().collect::<Vec<_>>(),
        events,
        format,
        EXPORT_TEST_LIMITS,
    )
    .unwrap()
}

/// A transient written in either encoding reads back exactly: every bit of
/// every sample, the names and the units.
#[test]
fn a_transient_reads_back_bit_for_bit_in_both_encodings() {
    let adjacent = f64::from_bits(1.0_f64.to_bits() + 1);
    let analysis =
        AnalysisResult::new(1, AnalysisType::Transient, "RC step", 0.0).with_waveforms(vec![
            RetainedWaveform::new(
                "V(out)",
                vec![0.0, 1e-9, 2e-9],
                vec![-0.0, adjacent, 1e-300],
            )
            .with_unit("V"),
            RetainedWaveform::new("I(R1)", vec![0.0, 1e-9, 2e-9], vec![1e-3, -2e-3, 0.5])
                .with_unit("A"),
        ]);
    for format in [RawFormat::Binary, RawFormat::Ascii] {
        let export = encode(&analysis, format, NO_EVENTS);
        assert_eq!(export.plot_name, "Transient Analysis");
        assert_eq!((export.points, export.variables), (3, 3));
        assert!(!export.complex);
        let decoded = decode_spice_raw(&export.bytes, Default::default()).unwrap();
        assert_eq!(decoded.domain, WaveformDomain::Transient);
        assert_eq!(decoded.coordinate_name, "time");
        assert_eq!(decoded.coordinate_unit.as_deref(), Some("s"));
        assert_eq!(decoded.coordinate, [0.0, 1e-9, 2e-9]);
        assert_eq!(decoded.signals.len(), 2);
        assert_eq!(decoded.signals[0].name, "V(out)");
        assert_eq!(decoded.signals[0].unit.as_deref(), Some("V"));
        let bits = |values: &[f64]| {
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            bits(&decoded.signals[0].real),
            bits(&[-0.0, adjacent, 1e-300]),
            "{format:?}"
        );
        assert_eq!(decoded.signals[1].name, "I(R1)");
        assert_eq!(decoded.signals[1].unit.as_deref(), Some("A"));
        assert_eq!(decoded.signals[1].real, [1e-3, -2e-3, 0.5]);
    }
}

/// An AC sweep is a complex plot of the retained rectangular samples, as
/// ngspice writes one, and a real trace in it reads back real.
#[test]
fn an_ac_sweep_is_a_complex_plot_and_keeps_a_real_trace_real() {
    let analysis = AnalysisResult::new(2, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
        RetainedWaveform::new("mag(V(out))", vec![1.0, 10.0], vec![99.0, 99.0])
            .with_complex_components("V(out)", vec![0.5, -0.25], vec![-0.5, 0.125])
            .with_unit("V"),
        RetainedWaveform::new("gain", vec![1.0, 10.0], vec![2.0, 3.0]),
    ]);
    for format in [RawFormat::Binary, RawFormat::Ascii] {
        let export = encode(&analysis, format, NO_EVENTS);
        assert_eq!(export.plot_name, "AC Analysis");
        assert!(export.complex);
        let text = String::from_utf8_lossy(&export.bytes);
        assert!(text.contains("Flags: complex"), "{text}");
        assert!(text.contains("\t1\tV(out)\tvoltage\n"), "{text}");
        let decoded = decode_spice_raw(&export.bytes, Default::default()).unwrap();
        assert_eq!(decoded.domain, WaveformDomain::Ac);
        assert_eq!(decoded.coordinate, [1.0, 10.0]);
        assert_eq!(decoded.signals[0].name, "V(out)");
        assert_eq!(decoded.signals[0].real, [0.5, -0.25]);
        assert_eq!(
            decoded.signals[0].imag.as_deref(),
            Some([-0.5, 0.125].as_slice())
        );
        assert_eq!(decoded.signals[1].name, "gain");
        assert_eq!(decoded.signals[1].real, [2.0, 3.0]);
        assert!(decoded.signals[1].imag.is_none(), "{format:?}");
    }
}

/// A DC sweep is declared on its swept source, and a name a rawfile cannot
/// declare is escaped there and read back exactly.
#[test]
fn a_dc_sweep_names_its_source_and_a_spaced_name_survives() {
    use rspice_results::dc_sweep::{
        DcCurveSelection, DcSweepDirection, DcSweepEvidence, DcSweepFamily,
    };
    let mut analysis =
        AnalysisResult::new(3, AnalysisType::DcSweep, "DC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("V(a) - V(b)", vec![0.0, 0.5], vec![0.0, 0.25]).with_unit("V"),
        ]);
    analysis.result_payload = Some(AnalysisResultPayload::DcSweep {
        evidence: std::sync::Arc::new(DcSweepEvidence {
            source: "V1".to_owned(),
            direction: DcSweepDirection::Ascending,
            quantities: Vec::new(),
            family: DcSweepFamily::Single,
            selection: DcCurveSelection::All,
        }),
    });
    let export = encode(&analysis, RawFormat::Ascii, NO_EVENTS);
    let text = String::from_utf8_lossy(&export.bytes);
    assert!(text.contains("\t0\tV1\tvoltage\n"), "{text}");
    assert!(text.contains("\t1\tV(a)%20-%20V(b)\tvoltage\n"), "{text}");
    let decoded = decode_spice_raw(&export.bytes, Default::default()).unwrap();
    assert_eq!(decoded.domain, WaveformDomain::DcSweep);
    assert_eq!(decoded.coordinate_name, "V1");
    assert_eq!(decoded.signals[0].name, "V(a) - V(b)");
}

/// A result that is not a sampled transient, AC or DC sweep is refused by
/// its own name rather than published under one of theirs.
#[test]
fn a_noise_spectrum_is_refused_by_name() {
    let analysis =
        AnalysisResult::new(4, AnalysisType::Noise, "Output noise", 0.0).with_waveforms(vec![
            RetainedWaveform::new("onoise", vec![1.0, 10.0], vec![1e-9, 1e-9]),
        ]);
    let error = encode_result_raw(
        &analysis,
        &analysis.waveforms.iter().collect::<Vec<_>>(),
        NO_EVENTS,
        RawFormat::Binary,
        EXPORT_TEST_LIMITS,
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("'Output noise' is none of those"), "{error}");
}

fn mixed_transient() -> AnalysisResult<RetainedWaveform> {
    let digital = |name: &str, codes: &[(f64, u8)]| DigitalEventTraceEvidence {
        node_name: name.to_owned(),
        points: codes
            .iter()
            .map(|&(time_s, value_code)| DigitalEventPointEvidence { time_s, value_code })
            .collect(),
    };
    AnalysisResult::new(5, AnalysisType::Transient, "Mixed", 0.0)
        .with_waveforms(vec![
            RetainedWaveform::new("V(a)", vec![0.0, 1e-9], vec![0.0, 1.8]).with_unit("V"),
        ])
        .with_result_payload(AnalysisResultPayload::TransientEvents {
            current_impulses: None,
            voltage_impulses: None,
            digital_traces: vec![
                digital("q1", &[(0.0, 0), (5e-10, 1)]),
                digital("q0", &[(0.0, 1), (7e-10, 0)]),
            ],
            real_traces: vec![RealEventTraceEvidence {
                node_name: "level".to_owned(),
                points: vec![RealEventPointEvidence {
                    time_s: 2e-10,
                    value: 0.75,
                }],
            }],
            digital_buses: vec![DigitalBusEvidence {
                name: "q".to_owned(),
                msb: 1,
                lsb: 0,
                members: vec!["q1".to_owned(), "q0".to_owned()],
                source: DigitalBusSourceEvidence::Engine,
            }],
            hdl_sources: Vec::new(),
        })
}

/// A transient's event histories follow the analysis plot as the engine's
/// own node and bus plots, and decode back to the histories retained.
#[test]
fn event_histories_ride_along_as_the_engines_own_plots() {
    let analysis = mixed_transient();
    for format in [RawFormat::Binary, RawFormat::Ascii] {
        let export = encode(&analysis, format, EVERY_EVENT);
        assert_eq!((export.event_plots, export.bus_plots), (3, 1));
        let file =
            rspice_core::io::parse_raw_plots_bytes_with_limits(&export.bytes, Default::default())
                .unwrap();
        assert_eq!(file.plots.len(), 5, "analysis, three nodes and one bus");
        assert_eq!(file.plots[0].header.plotname, "Transient Analysis");
        let traces = rspice_core::execution::decode_event_plots(&file).unwrap();
        let names = traces
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, ["q1", "q0"]);
        assert_eq!(traces.digital_traces[0].points.len(), 2);
        assert_eq!(traces.digital_traces[0].points[1].time, 5e-10);
        assert_eq!(traces.real_traces[0].points[0].value, 0.75);
        assert_eq!(traces.digital_buses[0].name, "q");
    }
}

/// Only the nodes the caller names are written, and a bus only with all of
/// its members; without events the file is the analysis plot alone.
#[test]
fn only_the_named_event_nodes_are_written() {
    let analysis = mixed_transient();
    let export = encode(&analysis, RawFormat::Binary, &|name| name == "q1");
    assert_eq!((export.event_plots, export.bus_plots), (1, 0));
    let without = encode(&analysis, RawFormat::Binary, NO_EVENTS);
    assert_eq!((without.event_plots, without.bus_plots), (0, 0));
    let decoded = decode_spice_raw(&without.bytes, Default::default()).unwrap();
    assert_eq!(decoded.signals[0].real, [0.0, 1.8]);
}

/// The multi-plot reader takes a file with event lanes back as the one
/// transient it is, carrying the histories its event plots hold, where the
/// one-dataset reader refuses the file rather than drop them.
#[test]
fn a_rawfile_with_event_lanes_reads_back_as_one_transient_with_its_events() {
    let analysis = mixed_transient();
    for format in [RawFormat::Binary, RawFormat::Ascii] {
        let export = encode(&analysis, format, EVERY_EVENT);
        let plots = decode_spice_raw_plots(&export.bytes, Default::default()).unwrap();
        assert_eq!(plots.len(), 1, "event plots are not analyses");
        let [plot] = plots.as_slice() else {
            unreachable!()
        };
        assert_eq!(plot.plot_name, "Transient Analysis");
        assert!(!plot.scalar);
        assert_eq!(plot.dataset.domain, WaveformDomain::Transient);
        assert_eq!(plot.dataset.signals[0].real, [0.0, 1.8]);
        let events = plot
            .events
            .as_ref()
            .expect("the transient carries the events");
        let names = events
            .digital_traces
            .iter()
            .map(|trace| (trace.node_name.as_str(), trace.points.len()))
            .collect::<Vec<_>>();
        assert_eq!(names, [("q1", 2), ("q0", 2)]);
        assert_eq!(events.digital_traces[1].points[1].time, 7e-10);
        assert_eq!(events.real_traces[0].points[0].value, 0.75);
        assert_eq!(events.digital_buses[0].members, ["q1", "q0"]);
        assert!(matches!(
            decode_spice_raw(&export.bytes, Default::default()),
            Err(RawReadError::MultiplePlots(5))
        ));
    }
}

/// Event plots belong to exactly one transient: beside two, or beside none,
/// the file is refused rather than guessed at.
#[test]
fn event_plots_with_no_single_transient_are_refused() {
    let export = encode(&mixed_transient(), RawFormat::Ascii, EVERY_EVENT);
    let text = std::str::from_utf8(&export.bytes).unwrap();
    let events_start = text.match_indices("Title:").nth(1).unwrap().0;
    let (analysis_plot, event_plots) = text.split_at(events_start);
    for (source, transients) in [
        (format!("{analysis_plot}{text}"), 2),
        (event_plots.to_owned(), 0),
    ] {
        let error = decode_spice_raw_plots(source.as_bytes(), Default::default()).unwrap_err();
        assert!(
            matches!(error, RawReadError::UnattributedEvents { transient_plots } if transient_plots == transients),
            "{error}"
        );
    }
}
