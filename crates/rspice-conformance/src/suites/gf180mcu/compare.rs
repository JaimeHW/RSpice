//! Curve comparison against a vendored reference.

use super::*;

/// The worst-disagreeing point on a compared curve.
#[derive(Debug, Clone, Copy)]
pub struct DeviceMismatch {
    /// Bias voltage at which the deviation occurred.
    pub x: f64,
    /// Reference value at that point.
    pub expected: f64,
    /// RSpice's value at that point.
    pub actual: f64,
    /// Relative deviation, as a fraction.
    pub relative_error: f64,
}

/// Compare two curves point by point, returning the worst relative deviation.
///
/// Points whose reference magnitude falls below `floor` are skipped. That
/// matters more here than in a typical waveform comparison: a diode I-V sweep
/// spans a dozen decades, and near the zero crossing the reference itself is
/// at the numerical floor of the run that produced it. Relative error against
/// a value that is already noise manufactures enormous disagreements out of
/// currents that are physically identical, and a suite that reports those
/// teaches its readers to ignore it.
///
/// The floor is derived from the curve's own peak rather than fixed, because
/// these devices differ by orders of magnitude in scale: a fixed floor would
/// be permissive for the large diodes and exclude most of the small ones.
pub fn compare_series(
    reference: &[(f64, f64)],
    actual: &[f64],
    relative_floor: f64,
) -> Result<(f64, Option<DeviceMismatch>), String> {
    if reference.is_empty() {
        return Err("reference curve is empty".to_string());
    }
    if actual.len() < reference.len() {
        return Err(format!(
            "simulation produced {} points for a {}-point reference",
            actual.len(),
            reference.len()
        ));
    }

    let peak = reference
        .iter()
        .fold(0.0_f64, |peak, (_, value)| peak.max(value.abs()));
    let floor = peak * relative_floor;

    let mut worst_error = 0.0_f64;
    let mut worst = None;

    for (index, &(x, expected)) in reference.iter().enumerate() {
        let actual_value = actual[index];
        if !actual_value.is_finite() {
            return Err(format!("non-finite simulation value at bias {x}"));
        }
        if expected.abs() <= floor {
            continue;
        }
        let error = (actual_value.abs() - expected.abs()).abs() / expected.abs();
        if error > worst_error {
            worst_error = error;
            worst = Some(DeviceMismatch {
                x,
                expected,
                actual: actual_value,
                relative_error: error,
            });
        }
    }

    Ok((worst_error * 100.0, worst))
}

impl DeviceRunner {
    /// Load a case, run its sweep, and compare against the vendored reference.
    pub(super) fn evaluate(
        &self,
        case: &str,
        _allowed_pct: f64,
    ) -> Result<(f64, Option<DeviceMismatch>), String> {
        let deck_path = self.deck_path(case);
        let reference = self.load_reference(case)?;

        let source = std::fs::read_to_string(&deck_path)
            .map_err(|err| format!("unreadable deck: {err}"))?;
        // Parsed straight from source, letting `parse_with_path` do its own
        // include and `.lib` expansion. Pre-expanding with
        // `preprocess_includes` first — which the sibling suites do, because
        // their decks need the expanded text for directive scanning — loses
        // the binned MOS model families here: every `nmos_3p3.N` card in the
        // selected corner section collapses and instances fall back to
        // looking for a plain `nmos_3p3`.
        let netlist = Netlist::parse_with_path(&source, &deck_path)
            .map_err(|err| format!("parse: {err}"))?;

        // Selected rather than taken from the front: these decks set their
        // characterisation temperature with a `.temp` card, which the parser
        // files alongside the analyses, so the sweep is rarely first.
        let Some(rspice_core::netlist::AnalysisCommand::Dc {
            source: sweep_source,
            start,
            stop,
            step,
            sweep2,
            ..
        }) = netlist
            .analyses
            .iter()
            .find(|analysis| matches!(analysis, rspice_core::netlist::AnalysisCommand::Dc { .. }))
            .cloned()
        else {
            return Err("case deck does not request a DC sweep".to_string());
        };

        // Each case is a characterisation point at a specific temperature, and
        // the deck says which with a `.temp` card. Running them all at the
        // engine's default 27°C is not a small error: these devices are swept
        // from -40°C to 175°C, and at the cold and hot ends the mismatch shows
        // up as hundreds of percent in subthreshold current. It also hides
        // behind the one temperature that happens to agree — the 25°C cases
        // pass either way, which is exactly what makes the bug look like a
        // scattered physics disagreement rather than a missing setting.
        let temperature_c = netlist.analyses.iter().find_map(|analysis| {
            match analysis {
                rspice_core::netlist::AnalysisCommand::Temp { temperatures } => {
                    temperatures.first().copied()
                }
                _ => None,
            }
        });
        let engine = self.engine(temperature_c);
        let abort = DeadlineAbort::new(Instant::now(), self.config.max_time_per_case_ms);
        // The MOS cases are a family of curves: Id against Vds at six stepped
        // gate biases, which is one two-source sweep rather than six runs.
        // Dropping the outer source would silently compare a 67-point inner
        // sweep against a 402-point reference — a length mismatch that reads
        // as a broken reference rather than as a runner that ignored half the
        // directive.
        let swept = engine
            .run_dc_sweep2_with_abort(
                &netlist,
                &sweep_source,
                start,
                stop,
                step,
                sweep2.as_ref(),
                &abort,
            )
            .map_err(|err| format!("dc sweep: {err}"))?;

        // The reference records |I| through the sweep source, which is the
        // quantity the upstream deck printed.
        let branch = format!("{sweep_source}");
        let actual: Vec<f64> = swept
            .iter()
            .map(|(_, result)| {
                result
                    .branch_currents
                    .get(branch_index(result, &branch).unwrap_or(0))
                    .copied()
                    .unwrap_or(f64::NAN)
                    .abs()
            })
            .collect();

        compare_series(&reference, &actual, self.config.measurement_floor)
    }

    fn load_reference(&self, case: &str) -> Result<Vec<(f64, f64)>, String> {
        let path = self.reference_path(case);
        let content = std::fs::read_to_string(&path)
            .map_err(|err| format!("unreadable reference '{}': {err}", path.display()))?;

        let mut series = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut fields = line.split('\t');
            let (Some(x), Some(y)) = (fields.next(), fields.next()) else {
                continue;
            };
            match (x.trim().parse::<f64>(), y.trim().parse::<f64>()) {
                (Ok(x), Ok(y)) => series.push((x, y)),
                _ => return Err(format!("malformed reference row: {line}")),
            }
        }

        if series.is_empty() {
            return Err(format!("reference '{}' has no rows", path.display()));
        }
        Ok(series)
    }
}

/// Index of a named branch current in a solved result.
fn branch_index(result: &rspice_core::SimulationResult, name: &str) -> Option<usize> {
    result
        .branch_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_curves_have_no_deviation() {
        let reference = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)];
        let actual = vec![1.0, 2.0, 3.0];
        let (worst, mismatch) = compare_series(&reference, &actual, 1e-6).expect("compares");
        assert_eq!(worst, 0.0);
        assert!(mismatch.is_none());
    }

    #[test]
    fn worst_point_is_reported_with_its_bias() {
        let reference = vec![(-1.0, 1.0), (0.0, 2.0), (1.0, 4.0)];
        let actual = vec![1.0, 2.2, 4.0];
        let (worst, mismatch) = compare_series(&reference, &actual, 1e-6).expect("compares");
        assert!((worst - 10.0).abs() < 1e-9, "{worst}");
        let mismatch = mismatch.expect("a deviating point");
        assert_eq!(mismatch.x, 0.0);
    }

    /// The property that keeps a twelve-decade sweep from reporting nonsense.
    #[test]
    fn points_below_the_curve_floor_are_not_compared() {
        // A reference peaking at 1.0 puts the 1e-12 sample far below a 1e-6
        // relative floor, so its enormous relative deviation is excluded.
        let reference = vec![(0.0, 1.0), (1.0, 1e-12)];
        let actual = vec![1.0, 1.0];
        let (worst, mismatch) = compare_series(&reference, &actual, 1e-6).expect("compares");
        assert_eq!(worst, 0.0);
        assert!(mismatch.is_none());
    }

    #[test]
    fn a_short_simulation_is_an_error_rather_than_a_pass() {
        let reference = vec![(0.0, 1.0), (1.0, 2.0)];
        assert!(compare_series(&reference, &[1.0], 1e-6).is_err());
    }

    #[test]
    fn non_finite_simulation_values_are_an_error() {
        let reference = vec![(0.0, 1.0)];
        assert!(compare_series(&reference, &[f64::NAN], 1e-6).is_err());
    }
}
