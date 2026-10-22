//! `.ASSERT` reports, evaluated on the exact accepted trajectory before
//! decimation and persisted field for field (analog state version 6).

use super::*;
use rspice_core::analysis::{AssertViolation, AssertionReport};
use rspice_core::netlist::{AssertAnalysis, AssertLevel, NetlistSourceLocation};

/// One violation: start, end, extreme value and where it was reached.
type AssertViolationPersistenceState = (Option<f64>, Option<f64>, Option<f64>, Option<f64>);

/// One report: name, analysis, target, instance, level, message, the card's
/// file and line, the violations, the X/Z stretch count and the error.
pub(crate) type AssertionPersistenceState = (
    String,
    String,
    String,
    Option<String>,
    String,
    Option<String>,
    Option<String>,
    usize,
    Vec<AssertViolationPersistenceState>,
    usize,
    Option<String>,
);

pub(crate) fn assertion_persistence_state(report: &AssertionReport) -> AssertionPersistenceState {
    (
        report.name.clone(),
        report.analysis.to_string(),
        report.target.clone(),
        report.instance.clone(),
        report.level.to_string(),
        report.message.clone(),
        report
            .origin
            .path
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned()),
        report.origin.line,
        report
            .violations
            .iter()
            .map(|violation| {
                (
                    violation.start,
                    violation.end,
                    violation.extreme,
                    violation.at,
                )
            })
            .collect(),
        report.unknown_stretches,
        report.error.clone(),
    )
}

pub(crate) fn rebuild_assertion(state: AssertionPersistenceState) -> PyResult<AssertionReport> {
    let (
        name,
        analysis,
        target,
        instance,
        level,
        message,
        file,
        line,
        violations,
        unknown_stretches,
        error,
    ) = state;
    let analysis = match analysis.as_str() {
        "op" => AssertAnalysis::Op,
        "dc" => AssertAnalysis::Dc,
        "tran" => AssertAnalysis::Tran,
        other => {
            return Err(crate::errors::value_error(format!(
                "pickled assertion {name} names unknown analysis '{other}'"
            )));
        }
    };
    let level = match level.as_str() {
        "notice" => AssertLevel::Notice,
        "warning" => AssertLevel::Warning,
        "error" => AssertLevel::Error,
        other => {
            return Err(crate::errors::value_error(format!(
                "pickled assertion {name} names unknown level '{other}'"
            )));
        }
    };
    Ok(AssertionReport {
        name,
        analysis,
        target,
        instance,
        level,
        message,
        origin: NetlistSourceLocation {
            path: file.map(std::path::PathBuf::from),
            line,
        },
        violations: violations
            .into_iter()
            .map(|(start, end, extreme, at)| AssertViolation {
                start,
                end,
                extreme,
                at,
            })
            .collect(),
        unknown_stretches,
        error,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::{IntoPyObject, Python, types::PyAnyMethods};

    #[test]
    fn an_assertion_report_survives_the_pickle_state_field_for_field() {
        Python::initialize();
        Python::attach(|py| {
            let report = AssertionReport {
                name: "mpsat".into(),
                analysis: AssertAnalysis::Tran,
                target: "@mp[vds]".into(),
                instance: Some("mp".into()),
                level: AssertLevel::Error,
                message: Some("PMOS out of saturation".into()),
                origin: NetlistSourceLocation::in_file("deck.cir", 7),
                violations: vec![
                    AssertViolation {
                        start: Some(1e-9),
                        end: Some(2e-9),
                        extreme: Some(-0.649),
                        at: Some(1.5e-9),
                    },
                    AssertViolation {
                        start: Some(3e-9),
                        end: None,
                        extreme: None,
                        at: None,
                    },
                ],
                unknown_stretches: 2,
                error: None,
            };
            let object = assertion_persistence_state(&report)
                .into_pyobject(py)
                .unwrap();
            let restored = rebuild_assertion(object.extract().unwrap()).unwrap();
            assert_eq!(restored, report);

            let unevaluated = AssertionReport {
                analysis: AssertAnalysis::Op,
                level: AssertLevel::Notice,
                instance: None,
                message: None,
                origin: NetlistSourceLocation::in_memory(3),
                violations: Vec::new(),
                unknown_stretches: 0,
                error: Some("V(nosuch) is not in the result".into()),
                ..report
            };
            let state = assertion_persistence_state(&unevaluated);
            assert_eq!(rebuild_assertion(state.clone()).unwrap(), unevaluated);

            let mut future = state;
            future.4 = "fatal".into();
            assert!(rebuild_assertion(future).is_err());
        });
    }
}
