//! Measure-based transient gates for `measures` validation contracts.
//!
//! Decks under this contract compare extracted engineering quantities —
//! crossing times, windowed settled means, peak amplitudes, oscillation
//! periods — instead of pointwise waveform samples. Every measure is
//! computed by the same code from the reference table and from the
//! simulation, so the reference stays the sole source of expected values;
//! the sidecar (`<deck>.gates.tsv`) only declares which quantities matter
//! and their tolerances. This is the professional gate for waveforms whose
//! raw tail samples are chaotic amplifications below solver tolerance: the
//! reference simulator's own answers at such rows move tens of percent
//! under timestep refinement, so pointwise agreement there measures grid
//! reproduction, not physics. The same applies to a free-running
//! oscillator's phase, which is set by roundoff rather than by the circuit.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CrossDirection {
    Rise,
    Fall,
}

impl CrossDirection {
    fn as_str(self) -> &'static str {
        match self {
            CrossDirection::Rise => "rise",
            CrossDirection::Fall => "fall",
        }
    }
}

#[derive(Debug, Clone)]
enum MeasureKind {
    /// Time of the `n`-th crossing of `level` in the given direction;
    /// `tol` is an absolute time tolerance in seconds.
    Cross {
        level: f64,
        direction: CrossDirection,
        n: usize,
        tol: f64,
    },
    /// Trapezoidal time-average over `[t0, t1]`; tolerance is
    /// `tol_abs + tol_rel * |reference|`.
    Mean {
        t0: f64,
        t1: f64,
        tol_abs: f64,
        tol_rel: f64,
    },
    /// Maximum of `|y|` over `[t0, t1]`; tolerance is
    /// `tol_abs + tol_rel * |reference|`.
    PeakAbs {
        t0: f64,
        t1: f64,
        tol_abs: f64,
        tol_rel: f64,
    },
    /// Mean interval between successive crossings of `level` in the given
    /// direction inside `[t0, t1]`; tolerance is
    /// `tol_abs + tol_rel * |reference|`.
    ///
    /// This is the gate for a free-running oscillator, where the frequency
    /// is the physics and the phase is not: a ring started from its own
    /// metastable operating point grows out of roundoff, so two builds of
    /// the same reference simulator disagree on where the waveform is at a
    /// given time while agreeing on how fast it runs. Fewer than two
    /// crossings is a failure rather than a skip, so a stuck node cannot
    /// pass by never oscillating.
    Period {
        level: f64,
        direction: CrossDirection,
        t0: f64,
        t1: f64,
        tol_abs: f64,
        tol_rel: f64,
    },
}

#[derive(Debug, Clone)]
struct MeasureGate {
    signal: String,
    kind: MeasureKind,
    label: String,
}

impl TestRunner {
    pub(crate) fn compare_transient_measures(
        &self,
        cir_path: &Path,
        result: &rspice_core::engine::TransientResult,
    ) -> Result<Vec<ValueMismatch>, String> {
        let gates = Self::load_measure_gates(cir_path)?;
        if gates.is_empty() {
            return Err(format!(
                "measures contract for '{}' has an empty or missing gates sidecar",
                cir_path.display()
            ));
        }

        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["time"])? else {
            return Err(format!(
                "measures contract for '{}' requires a parseable time-axis reference table",
                cir_path.display()
            ));
        };

        let mut node_to_idx = HashMap::with_capacity(result.node_names.len() + 1);
        node_to_idx.insert("0".to_string(), 0usize);
        for (idx, name) in result.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx + 1);
        }

        let mut mismatches = Vec::new();
        for gate in &gates {
            let reference_series = reference
                .variables
                .iter()
                .find(|(name, _)| {
                    Self::normalize_variable_name(name)
                        == Self::normalize_variable_name(&gate.signal)
                })
                .map(|(_, series)| series)
                .ok_or_else(|| {
                    format!(
                        "measure gate signal '{}' is absent from the reference table of '{}'",
                        gate.signal,
                        cir_path.display()
                    )
                })?;

            let (node_pos, node_neg) =
                Self::parse_voltage_probe(&gate.signal).ok_or_else(|| {
                    format!(
                        "measure gate signal '{}' is not a voltage probe",
                        gate.signal
                    )
                })?;
            let idx_pos = Self::resolve_node_index(&node_to_idx, &node_pos).ok_or_else(|| {
                format!("measure gate node '{node_pos}' is absent from the simulation")
            })?;
            let idx_neg = node_neg
                .as_deref()
                .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                .unwrap_or(0);
            let pos_wave = Self::transient_node_waveform(result, idx_pos);
            let neg_wave = Self::transient_node_waveform(result, idx_neg);
            let sim_y: Vec<f64> = pos_wave
                .iter()
                .zip(neg_wave.iter())
                .map(|(a, b)| a - b)
                .collect();

            let expected =
                Self::evaluate_measure(&gate.kind, &reference_series.x, &reference_series.y)
                    .ok_or_else(|| {
                        format!(
                            "measure '{}' cannot be computed on the reference of '{}' -- the gate definition does not match the reference waveform",
                            gate.label,
                            cir_path.display()
                        )
                    })?;
            let actual = Self::evaluate_measure(&gate.kind, &result.time, &sim_y);

            let (tol_abs, tol_rel) = match gate.kind {
                MeasureKind::Cross { tol, .. } => (tol, 0.0),
                MeasureKind::Mean {
                    tol_abs, tol_rel, ..
                }
                | MeasureKind::PeakAbs {
                    tol_abs, tol_rel, ..
                }
                | MeasureKind::Period {
                    tol_abs, tol_rel, ..
                } => (tol_abs, tol_rel),
            };
            let allowed = tol_abs + tol_rel * expected.abs();

            match actual {
                Some(actual) if (actual - expected).abs() <= allowed => {}
                Some(actual) => {
                    let scale = expected.abs().max(actual.abs()).max(1e-30);
                    mismatches.push(ValueMismatch {
                        x_value: expected,
                        node: gate.label.clone(),
                        expected,
                        actual,
                        relative_error: (actual - expected).abs() / scale,
                    });
                }
                None => {
                    mismatches.push(ValueMismatch {
                        x_value: expected,
                        node: format!("{} (not attained in simulation)", gate.label),
                        expected,
                        actual: f64::NAN,
                        relative_error: f64::INFINITY,
                    });
                }
            }
        }

        Ok(mismatches)
    }

    fn load_measure_gates(cir_path: &Path) -> Result<Vec<MeasureGate>, String> {
        let sidecar = cir_path.with_extension("gates.tsv");
        let content = fs::read_to_string(&sidecar).map_err(|err| {
            format!(
                "failed to read measure gates sidecar '{}': {err}",
                sidecar.display()
            )
        })?;

        let mut gates = Vec::new();
        for (line_number, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut fields = line.split_whitespace();
            let signal = fields
                .next()
                .ok_or_else(|| {
                    format!("{}:{}: missing signal", sidecar.display(), line_number + 1)
                })?
                .to_string();
            let kind_token = fields.next().ok_or_else(|| {
                format!(
                    "{}:{}: missing measure kind",
                    sidecar.display(),
                    line_number + 1
                )
            })?;

            let mut args: HashMap<&str, &str> = HashMap::new();
            for token in fields {
                let (key, value) = token.split_once('=').ok_or_else(|| {
                    format!(
                        "{}:{}: expected key=value, got '{token}'",
                        sidecar.display(),
                        line_number + 1
                    )
                })?;
                args.insert(key, value);
            }
            let parse_value = |key: &str| -> Result<f64, String> {
                args.get(key)
                    .ok_or_else(|| {
                        format!(
                            "{}:{}: measure '{kind_token}' is missing '{key}='",
                            sidecar.display(),
                            line_number + 1
                        )
                    })?
                    .parse::<f64>()
                    .map_err(|err| {
                        format!(
                            "{}:{}: invalid '{key}': {err}",
                            sidecar.display(),
                            line_number + 1
                        )
                    })
            };
            let parse_optional = |key: &str| -> Result<f64, String> {
                match args.get(key) {
                    None => Ok(0.0),
                    Some(value) => value.parse::<f64>().map_err(|err| {
                        format!(
                            "{}:{}: invalid '{key}': {err}",
                            sidecar.display(),
                            line_number + 1
                        )
                    }),
                }
            };

            let parse_direction = || -> Result<CrossDirection, String> {
                match args.get("dir").copied() {
                    Some("rise") => Ok(CrossDirection::Rise),
                    Some("fall") => Ok(CrossDirection::Fall),
                    other => Err(format!(
                        "{}:{}: dir= must be rise or fall, got {:?}",
                        sidecar.display(),
                        line_number + 1,
                        other
                    )),
                }
            };

            let (kind, label) = match kind_token {
                "cross" => {
                    let level = parse_value("level")?;
                    let direction = parse_direction()?;
                    let n = parse_value("n")? as usize;
                    let tol = parse_value("tol")?;
                    let label = format!("{signal} cross level={level} {}#{n}", direction.as_str());
                    (
                        MeasureKind::Cross {
                            level,
                            direction,
                            n: n.max(1),
                            tol,
                        },
                        label,
                    )
                }
                "mean" => {
                    let t0 = parse_value("t0")?;
                    let t1 = parse_value("t1")?;
                    let tol_abs = parse_optional("tol_abs")?;
                    let tol_rel = parse_optional("tol_rel")?;
                    (
                        MeasureKind::Mean {
                            t0,
                            t1,
                            tol_abs,
                            tol_rel,
                        },
                        format!("{signal} mean [{t0:.3e},{t1:.3e}]"),
                    )
                }
                "peak_abs" => {
                    let t0 = parse_value("t0")?;
                    let t1 = parse_value("t1")?;
                    let tol_abs = parse_optional("tol_abs")?;
                    let tol_rel = parse_optional("tol_rel")?;
                    (
                        MeasureKind::PeakAbs {
                            t0,
                            t1,
                            tol_abs,
                            tol_rel,
                        },
                        format!("{signal} peak_abs [{t0:.3e},{t1:.3e}]"),
                    )
                }
                "period" => {
                    let level = parse_value("level")?;
                    let direction = parse_direction()?;
                    let t0 = parse_value("t0")?;
                    let t1 = parse_value("t1")?;
                    let tol_abs = parse_optional("tol_abs")?;
                    let tol_rel = parse_optional("tol_rel")?;
                    (
                        MeasureKind::Period {
                            level,
                            direction,
                            t0,
                            t1,
                            tol_abs,
                            tol_rel,
                        },
                        format!(
                            "{signal} period level={level} {} [{t0:.3e},{t1:.3e}]",
                            direction.as_str()
                        ),
                    )
                }
                other => {
                    return Err(format!(
                        "{}:{}: unknown measure kind '{other}'",
                        sidecar.display(),
                        line_number + 1
                    ));
                }
            };

            gates.push(MeasureGate {
                signal,
                kind,
                label,
            });
        }

        Ok(gates)
    }

    /// Interpolated times at which `y` crosses `level` in `direction`, in
    /// sample order. A crossing is placed linearly inside the sample
    /// interval that brackets it, which is what makes the gate independent
    /// of where the reference's own steps happened to land.
    fn crossing_times(x: &[f64], y: &[f64], level: f64, direction: CrossDirection) -> Vec<f64> {
        let mut times = Vec::new();
        for (x_window, y_window) in x.windows(2).zip(y.windows(2)) {
            let (t0, t1) = (x_window[0], x_window[1]);
            let (y0, y1) = (y_window[0], y_window[1]);
            if t1 <= t0 {
                continue;
            }
            let crosses = match direction {
                CrossDirection::Rise => y0 < level && y1 >= level,
                CrossDirection::Fall => y0 > level && y1 <= level,
            };
            if crosses {
                // Either arm brackets `level` strictly, so `y1 - y0` is
                // non-zero and the interpolation cannot divide by zero.
                let f = (level - y0) / (y1 - y0);
                times.push(t0 + f * (t1 - t0));
            }
        }
        times
    }

    fn evaluate_measure(kind: &MeasureKind, x: &[f64], y: &[f64]) -> Option<f64> {
        if x.len() != y.len() || x.len() < 2 {
            return None;
        }
        match *kind {
            MeasureKind::Cross {
                level,
                direction,
                n,
                ..
            } => Self::crossing_times(x, y, level, direction)
                .get(n.checked_sub(1)?)
                .copied(),
            MeasureKind::Period {
                level,
                direction,
                t0,
                t1,
                ..
            } => {
                if t1 <= t0 {
                    return None;
                }
                let crossings: Vec<f64> = Self::crossing_times(x, y, level, direction)
                    .into_iter()
                    .filter(|&t| t >= t0 && t <= t1)
                    .collect();
                // One crossing gives no interval, and no crossings is what a
                // node stuck at its operating point looks like. Both report
                // "not attained" rather than a vacuous pass.
                if crossings.len() < 2 {
                    return None;
                }
                let span = crossings[crossings.len() - 1] - crossings[0];
                Some(span / (crossings.len() - 1) as f64)
            }
            MeasureKind::Mean { t0, t1, .. } => {
                if t1 <= t0 {
                    return None;
                }
                let value_at = |t: f64| -> Option<f64> { Self::interpolate_series(x, y, t) };
                let mut area = 0.0;
                let mut prev_t = t0;
                let mut prev_v = value_at(t0)?;
                for (&t, &v) in x.iter().zip(y.iter()) {
                    if t <= t0 || t >= t1 {
                        continue;
                    }
                    area += 0.5 * (prev_v + v) * (t - prev_t);
                    prev_t = t;
                    prev_v = v;
                }
                let end_v = value_at(t1)?;
                area += 0.5 * (prev_v + end_v) * (t1 - prev_t);
                Some(area / (t1 - t0))
            }
            MeasureKind::PeakAbs { t0, t1, .. } => {
                if t1 <= t0 {
                    return None;
                }
                let mut peak: Option<f64> = None;
                let mut consider = |value: f64| {
                    let magnitude = value.abs();
                    if peak.is_none_or(|current| magnitude > current) {
                        peak = Some(magnitude);
                    }
                };
                if let Some(edge) = Self::interpolate_series(x, y, t0) {
                    consider(edge);
                }
                if let Some(edge) = Self::interpolate_series(x, y, t1) {
                    consider(edge);
                }
                for (&t, &v) in x.iter().zip(y.iter()) {
                    if t >= t0 && t <= t1 {
                        consider(v);
                    }
                }
                peak
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_deck(tag: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after the Unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("rspice_measure_gates_{tag}_{unique}"));
        fs::create_dir_all(&dir).expect("create gate sidecar fixture directory");
        dir.join("deck.cir")
    }

    /// A triangle wave of period `period`, sampled off the crossings so the
    /// gate has to interpolate to recover the period exactly.
    fn triangle(period: f64, cycles: usize, low: f64, high: f64) -> (Vec<f64>, Vec<f64>) {
        let mut x = Vec::new();
        let mut y = Vec::new();
        let samples_per_cycle = 7;
        let total = cycles * samples_per_cycle;
        for index in 0..=total {
            let t = period * index as f64 / samples_per_cycle as f64;
            let phase = (t / period).fract();
            let ramp = if phase < 0.5 {
                2.0 * phase
            } else {
                2.0 * (1.0 - phase)
            };
            x.push(t);
            y.push(low + (high - low) * ramp);
        }
        (x, y)
    }

    fn period_kind(level: f64, direction: CrossDirection, t0: f64, t1: f64) -> MeasureKind {
        MeasureKind::Period {
            level,
            direction,
            t0,
            t1,
            tol_abs: 0.0,
            tol_rel: 0.0,
        }
    }

    #[test]
    fn period_averages_the_interval_between_interpolated_crossings() {
        let (x, y) = triangle(2.0e-9, 4, 0.0, 2.0);
        let measured = TestRunner::evaluate_measure(
            &period_kind(1.0, CrossDirection::Rise, 0.0, 1.0e-8),
            &x,
            &y,
        )
        .expect("four cycles of a triangle wave have four rising crossings");
        assert!(
            (measured - 2.0e-9).abs() < 1.0e-21,
            "recovered period {measured:e} should be the 2 ns the wave was built with"
        );
    }

    /// The measure exists because phase is not portable, so a shifted copy of
    /// the same oscillation has to read back the same period.
    #[test]
    fn period_is_independent_of_phase() {
        let (x, y) = triangle(2.0e-9, 4, 0.0, 2.0);
        let shifted: Vec<f64> = x.iter().map(|t| t + 0.37e-9).collect();
        let kind = period_kind(1.0, CrossDirection::Rise, 0.0, 1.0e-8);
        let base = TestRunner::evaluate_measure(&kind, &x, &y).expect("base period");
        let moved = TestRunner::evaluate_measure(&kind, &shifted, &y).expect("shifted period");
        assert!(
            (base - moved).abs() < 1.0e-21,
            "period moved from {base:e} to {moved:e} under a 0.37 ns phase shift"
        );
    }

    #[test]
    fn period_measures_falling_crossings_too() {
        let (x, y) = triangle(2.0e-9, 4, 0.0, 2.0);
        let measured = TestRunner::evaluate_measure(
            &period_kind(1.0, CrossDirection::Fall, 0.0, 1.0e-8),
            &x,
            &y,
        )
        .expect("four cycles have four falling crossings");
        assert!((measured - 2.0e-9).abs() < 1.0e-21, "got {measured:e}");
    }

    /// The window has to be able to exclude a startup interval, because the
    /// first crossings of a ring emerging from metastability are not yet a
    /// period. Ten cycles of 2 ns, the first replaced by a 3 ns cycle.
    #[test]
    fn period_window_excludes_crossings_outside_it() {
        let (mut x, mut y) = triangle(2.0e-9, 3, 0.0, 2.0);
        let tail_start = *x.last().expect("non-empty");
        let (tail_x, tail_y) = triangle(4.0e-9, 2, 0.0, 2.0);
        x.extend(tail_x.iter().skip(1).map(|t| t + tail_start));
        y.extend(tail_y.iter().skip(1).copied());

        let fast = TestRunner::evaluate_measure(
            &period_kind(1.0, CrossDirection::Rise, 0.0, 6.0e-9),
            &x,
            &y,
        )
        .expect("the 2 ns section has three rising crossings");
        assert!((fast - 2.0e-9).abs() < 1.0e-20, "got {fast:e}");

        let slow = TestRunner::evaluate_measure(
            &period_kind(1.0, CrossDirection::Rise, 6.0e-9, 1.4e-8),
            &x,
            &y,
        )
        .expect("the 4 ns section has two rising crossings");
        assert!((slow - 4.0e-9).abs() < 1.0e-20, "got {slow:e}");
    }

    /// The failure that matters: a node parked at the metastable operating
    /// point never crosses, and must not pass by having nothing to compare.
    #[test]
    fn period_of_a_stuck_node_is_not_attained() {
        let x: Vec<f64> = (0..=50).map(|i| i as f64 * 1.0e-10).collect();
        let y = vec![0.958_048_954_7; x.len()];
        assert!(
            TestRunner::evaluate_measure(
                &period_kind(1.0, CrossDirection::Rise, 0.0, 5.0e-9),
                &x,
                &y
            )
            .is_none(),
            "a node stuck below the crossing level must report no period"
        );
    }

    #[test]
    fn period_of_a_single_crossing_is_not_attained() {
        let x = vec![0.0, 1.0e-9, 2.0e-9, 3.0e-9];
        let y = vec![0.0, 0.5, 1.5, 2.0];
        assert!(
            TestRunner::evaluate_measure(
                &period_kind(1.0, CrossDirection::Rise, 0.0, 5.0e-9),
                &x,
                &y
            )
            .is_none(),
            "one crossing spans no interval and cannot define a period"
        );
    }

    #[test]
    fn period_rejects_an_inverted_window() {
        let (x, y) = triangle(2.0e-9, 4, 0.0, 2.0);
        assert!(
            TestRunner::evaluate_measure(
                &period_kind(1.0, CrossDirection::Rise, 8.0e-9, 1.0e-9),
                &x,
                &y
            )
            .is_none(),
            "t1 <= t0 declares no window"
        );
    }

    #[test]
    fn sidecar_parses_a_period_gate() {
        let deck = temp_deck("parse");
        fs::write(
            deck.with_extension("gates.tsv"),
            "# comment\nv(18)\tperiod\tlevel=1.0\tdir=rise\tt0=1e-9\tt1=5e-9\ttol_abs=0\ttol_rel=0.007\n",
        )
        .expect("write gate sidecar");

        let gates = TestRunner::load_measure_gates(&deck).expect("sidecar parses");
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].signal, "v(18)");
        assert!(
            gates[0].label.contains("period level=1 rise"),
            "unexpected label '{}'",
            gates[0].label
        );
        match gates[0].kind {
            MeasureKind::Period {
                level,
                direction,
                t0,
                t1,
                tol_abs,
                tol_rel,
            } => {
                assert_eq!(level, 1.0);
                assert_eq!(direction, CrossDirection::Rise);
                assert_eq!(t0, 1.0e-9);
                assert_eq!(t1, 5.0e-9);
                assert_eq!(tol_abs, 0.0);
                assert_eq!(tol_rel, 0.007);
            }
            ref other => panic!("expected a period gate, got {other:?}"),
        }
    }

    #[test]
    fn sidecar_rejects_a_period_gate_without_a_direction() {
        let deck = temp_deck("no_dir");
        fs::write(
            deck.with_extension("gates.tsv"),
            "v(18)\tperiod\tlevel=1.0\tt0=0\tt1=5e-9\n",
        )
        .expect("write gate sidecar");

        let error = TestRunner::load_measure_gates(&deck).expect_err("dir= is required");
        assert!(error.contains("dir= must be rise or fall"), "got '{error}'");
    }

    #[test]
    fn sidecar_rejects_a_period_gate_without_a_window() {
        let deck = temp_deck("no_window");
        fs::write(
            deck.with_extension("gates.tsv"),
            "v(18)\tperiod\tlevel=1.0\tdir=rise\tt0=0\n",
        )
        .expect("write gate sidecar");

        let error = TestRunner::load_measure_gates(&deck).expect_err("t1= is required");
        assert!(error.contains("missing 't1='"), "got '{error}'");
    }

    /// The pre-existing kinds go through the same crossing helper now, so
    /// their behaviour is pinned here as well.
    #[test]
    fn cross_selects_the_nth_crossing_and_runs_out() {
        let (x, y) = triangle(2.0e-9, 4, 0.0, 2.0);
        let nth = |n: usize| {
            TestRunner::evaluate_measure(
                &MeasureKind::Cross {
                    level: 1.0,
                    direction: CrossDirection::Rise,
                    n,
                    tol: 0.0,
                },
                &x,
                &y,
            )
        };
        let first = nth(1).expect("first rising crossing");
        let third = nth(3).expect("third rising crossing");
        assert!((third - first - 4.0e-9).abs() < 1.0e-20);
        assert!(nth(9).is_none(), "there are only four rising crossings");
    }
}
