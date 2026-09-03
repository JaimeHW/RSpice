//! Every analysis family answers an authored output symbol the same way.
//!
//! The projection is one decision procedure, so a `.SAVE` symbol a result
//! cannot supply must be a typed `RequestedSignalUnavailable` in *every*
//! [`AnalysisResultKind`] — not only in the families whose exporters happened
//! to check. The matrix is exhaustive over `AnalysisResultKind::ALL`, so a new
//! result family cannot be added without deciding this.

use std::borrow::Cow;

use rspice_core::execution::{
    AnalysisResultKind, ProjectionSource, ProjectionSourceSignal, ProjectionValues,
    SignalProjection, projection_analysis_kind,
};
use rspice_core::netlist::Netlist;
use rspice_core::{NoAbort, SimulationError, SimulationErrorCode, Value};

const AUTHORED_SYMBOL: &str = "@R1[NotAParameter]";

fn deck_saving(symbol: &str) -> Netlist {
    Netlist::parse(&format!(
        "* one authored output symbol\nV1 in 0 1\nR1 in 0 1k\n.OP\n.SAVE {symbol}\n.END\n"
    ))
    .expect("projection matrix deck parses")
}

fn real_signal(
    display: &str,
    registry: &str,
    values: Vec<Value>,
) -> ProjectionSourceSignal<'static> {
    ProjectionSourceSignal::new(
        display,
        registry,
        rspice_core::execution::SignalKind::Voltage,
        ProjectionValues::Real(Cow::Owned(values)),
    )
    .expect("valid real source signal")
}

fn complex_signal(
    display: &str,
    registry: &str,
    real: Vec<Value>,
    imag: Vec<Value>,
) -> ProjectionSourceSignal<'static> {
    ProjectionSourceSignal::new(
        display,
        registry,
        rspice_core::execution::SignalKind::Voltage,
        ProjectionValues::Complex {
            real: Cow::Owned(real),
            imag: Cow::Owned(imag),
        },
    )
    .expect("valid complex source signal")
}

fn observable(display: &str, values: Vec<Value>) -> ProjectionSourceSignal<'static> {
    ProjectionSourceSignal::new(
        display,
        display,
        rspice_core::execution::SignalKind::DeviceObservable,
        ProjectionValues::Real(Cow::Owned(values)),
    )
    .expect("valid device-observable source signal")
}

fn assert_unavailable(error: SimulationError, instance: &str) {
    assert_eq!(
        error.descriptor().code,
        SimulationErrorCode::RequestedSignalUnavailable,
        "projection must fail typed, got: {error}"
    );
    let SimulationError::RequestedSignalUnavailable(detail) = error else {
        panic!("expected a typed unavailable-signal error");
    };
    assert_eq!(detail.signal, AUTHORED_SYMBOL);
    assert_eq!(detail.analysis_label, instance);
}

#[test]
fn an_unsupplied_authored_symbol_is_typed_in_every_analysis_family() {
    let netlist = deck_saving(AUTHORED_SYMBOL);
    let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");

    for kind in AnalysisResultKind::ALL {
        let instance = kind.tag();

        let real = ProjectionSource::new(kind, instance)
            .with_axis(vec![0.0, 1.0])
            .with_signals(vec![real_signal("V(in)", "in", vec![1.0, 1.0])]);
        let error = projection
            .project(&netlist.params, &real, &NoAbort)
            .unwrap_err_or_else(kind, "real");
        assert_unavailable(error, instance);

        let complex = ProjectionSource::new(kind, instance)
            .with_axis(vec![0.0, 1.0])
            .with_signals(vec![complex_signal(
                "V(in)",
                "in",
                vec![1.0, 1.0],
                vec![0.0, 0.0],
            )]);
        let error = projection
            .project(&netlist.params, &complex, &NoAbort)
            .unwrap_err_or_else(kind, "complex");
        assert_unavailable(error, instance);
    }
}

#[test]
fn a_supplied_authored_symbol_projects_in_every_analysis_family() {
    let netlist = deck_saving(AUTHORED_SYMBOL);
    let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");

    for kind in AnalysisResultKind::ALL {
        let source = ProjectionSource::new(kind, kind.tag())
            .with_axis(vec![0.0, 1.0])
            .with_signals(vec![
                real_signal("V(in)", "in", vec![1.0, 1.0]),
                observable(AUTHORED_SYMBOL, vec![2.0, 3.0]),
            ]);
        let projected = projection
            .project(&netlist.params, &source, &NoAbort)
            .unwrap_or_else(|error| {
                panic!("{} must project a supplied symbol: {error}", kind.tag())
            });
        let names = projected
            .signals()
            .iter()
            .map(|signal| signal.descriptor().display_name().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [AUTHORED_SYMBOL],
            "{} must export exactly the authored selection",
            kind.tag()
        );
        assert!(
            projected.signals()[0].validity().iter().all(|valid| *valid),
            "{} projected an absent sample without saying so",
            kind.tag()
        );
    }
}

#[test]
fn an_ordered_print_card_only_selects_its_own_family() {
    // `.PRINT TRAN` must not contribute columns to an AC projection, and an
    // unqualified card must contribute to both.
    let netlist = Netlist::parse(
        "* qualified and unqualified print cards\n\
         V1 in 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 1n\n\
         .TRAN 1u 10u\n\
         .AC DEC 1 1 10\n\
         .PRINT TRAN V(out)\n\
         .PRINT V(in)\n\
         .END\n",
    )
    .expect("qualified print deck parses");
    let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");

    let ac = ProjectionSource::new(AnalysisResultKind::Ac, "AC")
        .with_axis(vec![1.0])
        .with_signals(vec![
            complex_signal("V(in)", "in", vec![1.0], vec![0.0]),
            complex_signal("V(out)", "out", vec![0.5], vec![0.25]),
        ]);
    let projected = projection
        .project(&netlist.params, &ac, &NoAbort)
        .expect("AC projects its own card");
    let names = projected
        .signals()
        .iter()
        .map(|signal| signal.descriptor().display_name().to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        ["V(in)"],
        "a TRAN-qualified card must not add a column to an AC export"
    );

    let tran = ProjectionSource::new(AnalysisResultKind::Transient, "TRAN")
        .with_axis(vec![0.0, 1.0])
        .with_signals(vec![
            real_signal("V(in)", "in", vec![1.0, 1.0]),
            real_signal("V(out)", "out", vec![0.5, 0.5]),
        ]);
    let projected = projection
        .project(&netlist.params, &tran, &NoAbort)
        .expect("TRAN projects both the qualified and unqualified cards");
    let names = projected
        .signals()
        .iter()
        .map(|signal| signal.descriptor().display_name().to_string())
        .collect::<Vec<_>>();
    assert_eq!(names, ["V(out)", "V(in)"], "cards project in deck order");

    assert_eq!(
        projection_analysis_kind(AnalysisResultKind::Ac),
        Some(rspice_core::netlist::OutputAnalysisKind::Ac)
    );
}

/// Fail with the family that produced the surprise instead of a bare unwrap.
trait ExpectProjectionFailure {
    fn unwrap_err_or_else(self, kind: AnalysisResultKind, shape: &str) -> SimulationError;
}

impl<T> ExpectProjectionFailure for Result<T, SimulationError> {
    fn unwrap_err_or_else(self, kind: AnalysisResultKind, shape: &str) -> SimulationError {
        match self {
            Ok(_) => panic!(
                "{} accepted an unsupplied authored symbol in a {shape} result",
                kind.tag()
            ),
            Err(error) => error,
        }
    }
}
