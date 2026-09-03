//! Frequency-domain result schema: one table, proved point by point.
//!
//! An AC-shaped result is addressed through one set of node and branch names.
//! This module is where that claim is checked, so `AcResult` and the
//! distortion responses cannot disagree about what "the same schema" means.

use super::*;

/// One frequency point's complete signal schema, as `V(name)`/`I(name)`.
fn ac_point_signals(point: &AcResult) -> Vec<String> {
    point
        .node_names
        .iter()
        .map(|name| format!("V({name})"))
        .chain(point.branch_names.iter().map(|name| format!("I({name})")))
        .collect()
}

/// Adopt the first point's signal schema, having proved every point matches it.
///
/// A frequency sweep is one table whose columns are fixed by the circuit, not
/// by the point. Reading the names off `results[0]` and then indexing later
/// rows through a fallible lookup leaves a solver that changed its mind
/// looking like a well-formed table with a few NaNs in it. Divergence is a
/// typed failure naming the point and the signals instead.
pub(crate) fn validated_ac_schema(
    analysis: &str,
    points: &[AcResult],
) -> Result<(Vec<String>, Vec<String>), String> {
    let Some(first) = points.first() else {
        return Ok((Vec::new(), Vec::new()));
    };
    let expected = ac_point_signals(first);
    for (index, point) in points.iter().enumerate() {
        let where_ = format!("point {} ({:.16e} Hz)", index + 1, point.frequency);
        if point.voltages.len() != point.node_names.len()
            || point.currents.len() != point.branch_names.len()
        {
            return Err(format!(
                "malformed {analysis} result at {where_}: {} voltages for {} node names and {} currents for {} branch names",
                point.voltages.len(),
                point.node_names.len(),
                point.currents.len(),
                point.branch_names.len()
            ));
        }
        if index == 0 {
            continue;
        }
        let actual = ac_point_signals(point);
        if actual == expected {
            continue;
        }
        let expected_set = expected.iter().collect::<std::collections::BTreeSet<_>>();
        let actual_set = actual.iter().collect::<std::collections::BTreeSet<_>>();
        let missing = expected_set
            .difference(&actual_set)
            .map(|name| name.as_str())
            .collect::<Vec<_>>();
        let unexpected = actual_set
            .difference(&expected_set)
            .map(|name| name.as_str())
            .collect::<Vec<_>>();
        if missing.is_empty() && unexpected.is_empty() {
            let first_difference = expected
                .iter()
                .zip(&actual)
                .position(|(left, right)| left != right)
                .unwrap_or(0);
            return Err(format!(
                "{analysis} result schema is reordered at {where_}: column {first_difference} is '{}' but the first point published '{}'",
                actual
                    .get(first_difference)
                    .map_or("<missing>", String::as_str),
                expected
                    .get(first_difference)
                    .map_or("<missing>", String::as_str)
            ));
        }
        return Err(format!(
            "{analysis} result schema changes at {where_}: missing [{}]; unexpected [{}]",
            missing.join(", "),
            unexpected.join(", ")
        ));
    }
    Ok((first.node_names.clone(), first.branch_names.clone()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::Complex64;

    fn ac_row(frequency: f64, voltages: Vec<Complex64>, currents: Vec<Complex64>) -> AcResult {
        AcResult {
            frequency,
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            voltages,
            currents,
        }
    }

    /// Assemble the struct without the schema proof, to exercise the
    /// per-row guards that construction now makes unreachable from Python.
    fn unchecked(frequencies: Vec<f64>, results: Vec<AcResult>) -> PyAcResult {
        let node_names = results[0].node_names.clone();
        let branch_names = results[0].branch_names.clone();
        PyAcResult {
            frequencies,
            results,
            node_names,
            branch_names,
            evidence: None,
        }
    }

    #[test]
    fn construction_refuses_a_row_whose_width_contradicts_its_own_names() {
        let message = PyAcResult::checked(
            vec![1.0, 2.0],
            vec![
                ac_row(
                    1.0,
                    vec![Complex64::new(1.0, 0.0)],
                    vec![Complex64::new(0.0, 1.0)],
                ),
                ac_row(2.0, Vec::new(), Vec::new()),
            ],
        )
        .err()
        .expect("a row that does not fill its own schema is malformed");

        assert!(
            message.contains("point 2 (2.0000000000000000e0"),
            "{message}"
        );
        assert!(message.contains("0 voltages for 1 node names"), "{message}");
    }

    #[test]
    fn construction_names_the_signals_a_diverging_point_added_and_dropped() {
        let mut diverged = ac_row(
            2.0,
            vec![Complex64::new(1.0, 0.0)],
            vec![Complex64::new(0.0, 1.0)],
        );
        diverged.node_names = vec!["elsewhere".to_string()];

        let message = PyAcResult::checked(
            vec![1.0, 2.0],
            vec![
                ac_row(
                    1.0,
                    vec![Complex64::new(1.0, 0.0)],
                    vec![Complex64::new(0.0, 1.0)],
                ),
                diverged,
            ],
        )
        .err()
        .expect("a point that publishes different signals is malformed");

        assert!(
            message.contains("AC result schema changes at point 2"),
            "{message}"
        );
        assert!(message.contains("missing [V(out)]"), "{message}");
        assert!(message.contains("unexpected [V(elsewhere)]"), "{message}");
    }

    #[test]
    fn construction_refuses_a_frequency_axis_the_solve_did_not_fill() {
        let message = PyAcResult::checked(
            vec![1.0, 2.0],
            vec![ac_row(
                1.0,
                vec![Complex64::new(1.0, 0.0)],
                vec![Complex64::new(0.0, 1.0)],
            )],
        )
        .err()
        .expect("a short solve must not be padded out");

        assert!(
            message.contains("1 solved points for 2 requested frequencies"),
            "{message}"
        );
    }

    #[test]
    fn ac_voltage_access_rejects_short_later_rows() {
        let ac = unchecked(
            vec![1.0, 2.0],
            vec![
                ac_row(1.0, vec![Complex64::new(1.0, 0.0)], Vec::new()),
                ac_row(2.0, Vec::new(), Vec::new()),
            ],
        );

        let message = ac
            .voltage_phasor_from_row(1, &ac.results[1], 1)
            .unwrap_err();
        assert!(message.contains("malformed AC result row 1"), "{message}");
        assert!(message.contains("missing voltage"), "{message}");
    }

    #[test]
    fn ac_branch_access_rejects_short_later_rows() {
        let ac = unchecked(
            vec![1.0, 2.0],
            vec![
                ac_row(
                    1.0,
                    vec![Complex64::new(1.0, 0.0)],
                    vec![Complex64::new(0.0, 1.0)],
                ),
                ac_row(2.0, vec![Complex64::new(1.0, 0.0)], Vec::new()),
            ],
        );

        let message = ac
            .branch_current_from_row(1, &ac.results[1], 0)
            .unwrap_err();
        assert!(message.contains("malformed AC result row 1"), "{message}");
        assert!(message.contains("missing current"), "{message}");
    }
}
