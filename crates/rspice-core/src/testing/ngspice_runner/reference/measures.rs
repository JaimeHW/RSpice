//! Measure-based transient gates for `measures` validation contracts.
//!
//! Decks under this contract compare extracted engineering quantities —
//! crossing times, windowed settled means, peak amplitudes — instead of
//! pointwise waveform samples. Every measure is computed by the same code
//! from the reference table and from the simulation, so the reference stays
//! the sole source of expected values; the sidecar (`<deck>.gates.tsv`)
//! only declares which quantities matter and their tolerances. This is the
//! professional gate for waveforms whose raw tail samples are chaotic
//! amplifications below solver tolerance: the reference simulator's own
//! answers at such rows move tens of percent under timestep refinement, so
//! pointwise agreement there measures grid reproduction, not physics.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CrossDirection {
    Rise,
    Fall,
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
}

#[derive(Debug, Clone)]
struct MeasureGate {
    signal: String,
    kind: MeasureKind,
    label: String,
}

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn compare_transient_measures(
        &self,
        cir_path: &Path,
        result: &crate::engine::TransientResult,
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

            let (kind, label) = match kind_token {
                "cross" => {
                    let level = parse_value("level")?;
                    let direction = match args.get("dir").copied() {
                        Some("rise") => CrossDirection::Rise,
                        Some("fall") => CrossDirection::Fall,
                        other => {
                            return Err(format!(
                                "{}:{}: dir= must be rise or fall, got {:?}",
                                sidecar.display(),
                                line_number + 1,
                                other
                            ));
                        }
                    };
                    let n = parse_value("n")? as usize;
                    let tol = parse_value("tol")?;
                    let label = format!(
                        "{signal} cross level={level} {}#{n}",
                        if direction == CrossDirection::Rise {
                            "rise"
                        } else {
                            "fall"
                        }
                    );
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
            } => {
                let mut seen = 0usize;
                for window in x.windows(2).zip(y.windows(2)) {
                    let ((&t0, &t1), (&y0, &y1)) =
                        ((&window.0[0], &window.0[1]), (&window.1[0], &window.1[1]));
                    if t1 <= t0 {
                        continue;
                    }
                    let crosses = match direction {
                        CrossDirection::Rise => y0 < level && y1 >= level,
                        CrossDirection::Fall => y0 > level && y1 <= level,
                    };
                    if crosses {
                        seen += 1;
                        if seen == n {
                            let f = (level - y0) / (y1 - y0);
                            return Some(t0 + f * (t1 - t0));
                        }
                    }
                }
                None
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
