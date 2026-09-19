//! Retained evidence of one `.SENS` study: the output that was
//! differentiated, the filter that chose the variables, the points it was
//! solved at, and one column of derivatives per variable per point.
//!
//! The Studio differentiates nothing. Every number below is what
//! `Engine::run_sensitivity_dc_complete_with_abort` or
//! `…_ac_complete_with_abort` returned, under the engine's own vector names
//! (`R1`, `M1_W`, `MOD:VTO`, `PARAM:GAIN`), so a deck run here and the same
//! deck run by `rspice run` are one answer rather than two.
//!
//! One shape carries all three cases a study can have — a DC operating point,
//! an AC sweep of one frequency, and an AC sweep of many — because they differ
//! only in how many points they hold. A reader who cannot tell a one-point
//! sweep from an operating point cannot tell which question was asked, so the
//! basis is part of the evidence and not a flag beside it.
//!
//! The filter travels with the numbers for the same reason. A ranked table of
//! four rows selected out of two thousand is not the report a table of four
//! rows out of four is, and the engine's default — every device and model
//! parameter, no design parameter — is spelled by the empty string rather than
//! guessed from the row names.

use rspice_core::analysis::sensitivity::{SensitivityUnavailability, SensitivityValue};

use super::ComplexResultValue;

/// The points one study was solved at, and the nominal output at each.
///
/// The nominal output is what every normalized column divides by, so it is
/// retained rather than recomputed: a sheet that re-derives it would be
/// stating its own arithmetic as the engine's.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "basis", rename_all = "snake_case", deny_unknown_fields)]
pub enum SensitivityBasisEvidence {
    /// One DC operating point. The nominal output is real by construction.
    Dc { output: f64 },
    /// An AC sweep. One frequency is an ordinary sweep of length one, which
    /// is exactly what the card `AC DEC 1 f f` asks for.
    #[serde(rename_all = "camelCase")]
    Ac {
        frequencies_hz: Vec<f64>,
        output: Vec<ComplexResultValue>,
    },
}

impl SensitivityBasisEvidence {
    /// How many points every column of this study has.
    pub fn point_count(&self) -> usize {
        match self {
            Self::Dc { .. } => 1,
            Self::Ac { frequencies_hz, .. } => frequencies_hz.len(),
        }
    }

    /// The frequency of one point, or `None` at a DC operating point.
    pub fn frequency_at(&self, index: usize) -> Option<f64> {
        match self {
            Self::Dc { .. } => None,
            Self::Ac { frequencies_hz, .. } => frequencies_hz.get(index).copied(),
        }
    }

    /// Magnitude of the nominal output at one point.
    ///
    /// This is the quantity the engine's zero-output rule tests, so both
    /// domains answer it the same way and nothing downstream branches on the
    /// basis to ask "was the output zero here".
    pub fn output_magnitude_at(&self, index: usize) -> Option<f64> {
        match self {
            Self::Dc { output } => (index == 0).then(|| output.abs()),
            Self::Ac { output, .. } => output
                .get(index)
                .map(|value| value.real.hypot(value.imaginary)),
        }
    }

    /// How the basis names itself in a table cell or an export column.
    pub fn mode_tag(&self) -> &'static str {
        match self {
            Self::Dc { .. } => "dc",
            Self::Ac { .. } => "ac",
        }
    }
}

/// One variable's derivative columns, one entry per point of the study.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SensitivityStudyRow {
    /// The engine's vector name for this variable. A device or model variable
    /// is spelled as `.SENS` spells it; a design parameter is `PARAM:<NAME>`.
    pub parameter: String,
    /// The variable's own value at the point the study was linearized about.
    pub nominal_value: f64,
    /// DC: `d(output)/dp`. AC: `d|output|/dp`.
    pub raw: Vec<SensitivityValue<f64>>,
    /// `(p / |output|) * d|output|/dp`, which is dimensionless in both domains.
    pub normalized: Vec<SensitivityValue<f64>>,
    /// `d(angle of output)/dp` in radians. Empty for a DC study, where the
    /// output has no phase to differentiate.
    pub phase: Vec<SensitivityValue<f64>>,
}

/// What one `.SENS` run answered.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SensitivityStudyEvidence {
    /// The probe as the run spelled it, for example `V(OUT,IN)` or `I(V1)`.
    pub output: String,
    /// The filter list as it was run: canonical, upper-case, space separated.
    /// Empty is the engine's default and means every device and model
    /// parameter — never "no filter was recorded".
    pub filter: String,
    pub basis: SensitivityBasisEvidence,
    /// One row per selected variable, strictly sorted by the engine's vector
    /// name, which is the order the engine returns them in.
    pub rows: Vec<SensitivityStudyRow>,
}

impl SensitivityStudyEvidence {
    /// How many points every column of this study has.
    pub fn point_count(&self) -> usize {
        self.basis.point_count()
    }

    /// The frequency of one point, or `None` at a DC operating point.
    pub fn frequency_at(&self, index: usize) -> Option<f64> {
        self.basis.frequency_at(index)
    }

    /// Whether the study holds more than one point, which is the one thing
    /// that decides whether a reader needs to be told which point a table is
    /// read at.
    pub fn is_swept(&self) -> bool {
        self.point_count() >= 2
    }

    /// How the filter reads to someone who did not type it.
    pub fn filter_label(&self) -> &str {
        if self.filter.is_empty() {
            "every device and model parameter"
        } else {
            self.filter.as_str()
        }
    }

    /// Refuse a study the engine could not have produced.
    ///
    /// Every rule is a property of the engine's own answer: the grid is the
    /// one an AC sweep produces, every column is as long as the grid, phase
    /// exists only where the output has one, the rows are the engine's sorted
    /// vector names, and a normalized column is unavailable for "zero output"
    /// at exactly the points whose nominal output is zero. A payload that
    /// breaks one of them is not a sensitivity study, and a sheet ranking it
    /// would rank numbers that answer no question.
    pub fn validate(&self) -> Result<(), String> {
        if self.output.trim().is_empty() {
            return Err("sensitivity study evidence has no output probe".to_owned());
        }
        if self.filter.chars().any(char::is_control) {
            return Err("sensitivity study filter contains a control character".to_owned());
        }
        let points = self.point_count();
        if points == 0 {
            return Err("sensitivity study evidence was solved at no points".to_owned());
        }
        if let SensitivityBasisEvidence::Ac {
            frequencies_hz,
            output,
        } = &self.basis
        {
            let mut previous: Option<f64> = None;
            for frequency in frequencies_hz {
                if !frequency.is_finite() || *frequency <= 0.0 {
                    return Err(format!(
                        "sensitivity study was solved at {frequency} Hz, which is not a positive \
                         finite frequency"
                    ));
                }
                if previous.is_some_and(|previous| previous >= *frequency) {
                    return Err(
                        "sensitivity study frequencies must be strictly increasing".to_owned()
                    );
                }
                previous = Some(*frequency);
            }
            if output.len() != points {
                return Err(format!(
                    "sensitivity study retained {} nominal outputs for {points} frequencies",
                    output.len()
                ));
            }
            if output
                .iter()
                .any(|value| !value.real.is_finite() || !value.imaginary.is_finite())
            {
                return Err("sensitivity study has a non-finite nominal output".to_owned());
            }
        } else if let SensitivityBasisEvidence::Dc { output } = &self.basis
            && !output.is_finite()
        {
            return Err("sensitivity study has a non-finite nominal output".to_owned());
        }

        let has_phase = matches!(self.basis, SensitivityBasisEvidence::Ac { .. });
        let mut previous_name: Option<&str> = None;
        for row in &self.rows {
            if row.parameter.trim().is_empty() {
                return Err("sensitivity study row has no parameter name".to_owned());
            }
            if previous_name.is_some_and(|previous| previous >= row.parameter.as_str()) {
                return Err(
                    "sensitivity study rows must have unique, strictly sorted parameter names"
                        .to_owned(),
                );
            }
            previous_name = Some(&row.parameter);
            if !row.nominal_value.is_finite() {
                return Err(format!(
                    "sensitivity study parameter '{}' has a non-finite nominal value",
                    row.parameter
                ));
            }
            if row.raw.len() != points || row.normalized.len() != points {
                return Err(format!(
                    "sensitivity study parameter '{}' has {} raw and {} normalized values for \
                     {points} points",
                    row.parameter,
                    row.raw.len(),
                    row.normalized.len()
                ));
            }
            let expected_phase = if has_phase { points } else { 0 };
            if row.phase.len() != expected_phase {
                return Err(format!(
                    "sensitivity study parameter '{}' has {} phase values where {expected_phase} \
                     are possible",
                    row.parameter,
                    row.phase.len()
                ));
            }
            for column in [&row.raw, &row.normalized, &row.phase] {
                for value in column {
                    match value {
                        SensitivityValue::Available(value) if !value.is_finite() => {
                            return Err(format!(
                                "sensitivity study parameter '{}' has a non-finite value",
                                row.parameter
                            ));
                        }
                        SensitivityValue::Unavailable {
                            unavailable: SensitivityUnavailability::InvalidInput,
                        } => {
                            return Err(format!(
                                "sensitivity study parameter '{}' was refused as invalid input",
                                row.parameter
                            ));
                        }
                        _ => {}
                    }
                }
            }
            // The engine's own normalization rule: a zero nominal output has
            // no scale to divide by, and it says so at that point and only
            // there. Any other spelling would let a sheet present an
            // arbitrary ratio as the engine's answer.
            for (index, value) in row.normalized.iter().enumerate() {
                let zero_output = self
                    .basis
                    .output_magnitude_at(index)
                    .is_some_and(|magnitude| magnitude == 0.0);
                let says_zero_output =
                    value.reason() == Some(SensitivityUnavailability::ZeroOutput);
                if zero_output != says_zero_output {
                    return Err(format!(
                        "sensitivity study parameter '{}' disagrees with its own nominal output \
                         about whether point {index} had one",
                        row.parameter
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn dc_fixture() -> SensitivityStudyEvidence {
        SensitivityStudyEvidence {
            output: "I(V1)".to_owned(),
            filter: "PARAM:*".to_owned(),
            basis: SensitivityBasisEvidence::Dc { output: -1.0 },
            rows: vec![SensitivityStudyRow {
                parameter: "PARAM:DRIVE".to_owned(),
                nominal_value: 2.0,
                raw: vec![SensitivityValue::Available(-0.5)],
                normalized: vec![SensitivityValue::Available(1.0)],
                phase: Vec::new(),
            }],
        }
    }

    pub(crate) fn ac_fixture() -> SensitivityStudyEvidence {
        SensitivityStudyEvidence {
            output: "V(OUT)".to_owned(),
            filter: String::new(),
            basis: SensitivityBasisEvidence::Ac {
                frequencies_hz: vec![10.0, 100.0, 1000.0],
                output: vec![
                    ComplexResultValue {
                        real: 1.0,
                        imaginary: 0.0,
                    },
                    ComplexResultValue {
                        real: 0.5,
                        imaginary: -0.5,
                    },
                    ComplexResultValue {
                        real: 0.0,
                        imaginary: -0.25,
                    },
                ],
            },
            rows: vec![
                SensitivityStudyRow {
                    parameter: "C1".to_owned(),
                    nominal_value: 1.0e-9,
                    raw: vec![
                        SensitivityValue::Available(-1.0),
                        SensitivityValue::Available(-2.0),
                        SensitivityValue::Available(-3.0),
                    ],
                    normalized: vec![
                        SensitivityValue::Available(-0.1),
                        SensitivityValue::Available(-0.2),
                        SensitivityValue::Available(-0.3),
                    ],
                    phase: vec![
                        SensitivityValue::Available(0.0),
                        SensitivityValue::Available(-0.5),
                        SensitivityValue::Available(-1.0),
                    ],
                },
                SensitivityStudyRow {
                    parameter: "R1".to_owned(),
                    nominal_value: 1000.0,
                    raw: vec![
                        SensitivityValue::Available(1.0),
                        SensitivityValue::unavailable(
                            SensitivityUnavailability::NondifferentiableMagnitude,
                        ),
                        SensitivityValue::Available(3.0),
                    ],
                    normalized: vec![
                        SensitivityValue::Available(0.1),
                        SensitivityValue::Available(0.2),
                        SensitivityValue::Available(0.3),
                    ],
                    phase: vec![
                        SensitivityValue::Available(0.0),
                        SensitivityValue::Available(0.5),
                        SensitivityValue::Available(1.0),
                    ],
                },
            ],
        }
    }

    #[test]
    fn one_shape_carries_an_operating_point_and_a_sweep() {
        let dc = dc_fixture();
        dc.validate().expect("the DC fixture is valid evidence");
        assert_eq!(dc.point_count(), 1);
        assert!(!dc.is_swept());
        assert_eq!(dc.frequency_at(0), None);
        assert_eq!(dc.filter_label(), "PARAM:*");

        let ac = ac_fixture();
        ac.validate().expect("the AC fixture is valid evidence");
        assert_eq!(ac.point_count(), 3);
        assert!(ac.is_swept());
        assert_eq!(ac.frequency_at(2), Some(1000.0));
        assert_eq!(ac.frequency_at(3), None);
        assert_eq!(ac.filter_label(), "every device and model parameter");

        let text = serde_json::to_string(&ac).expect("evidence serializes");
        let restored: SensitivityStudyEvidence =
            serde_json::from_str(&text).expect("evidence restores");
        assert_eq!(ac, restored);
    }

    #[test]
    fn a_column_that_does_not_span_the_grid_is_refused() {
        let mut broken = ac_fixture();
        broken.rows[0].raw.pop();
        assert!(broken.validate().is_err());

        let mut broken = ac_fixture();
        broken.rows[0].phase.clear();
        assert!(broken.validate().is_err());

        // A DC study has no phase to differentiate, so a phase column is a
        // number that cannot have been computed.
        let mut broken = dc_fixture();
        broken.rows[0].phase = vec![SensitivityValue::Available(0.0)];
        assert!(broken.validate().is_err());
    }

    #[test]
    fn a_grid_the_engine_could_not_have_solved_is_refused() {
        let mut broken = ac_fixture();
        let SensitivityBasisEvidence::Ac { frequencies_hz, .. } = &mut broken.basis else {
            panic!("the AC fixture is an AC study");
        };
        frequencies_hz[2] = 100.0;
        assert!(broken.validate().is_err());

        let mut broken = ac_fixture();
        let SensitivityBasisEvidence::Ac { output, .. } = &mut broken.basis else {
            panic!("the AC fixture is an AC study");
        };
        output.pop();
        assert!(broken.validate().is_err());
    }

    #[test]
    fn rows_out_of_the_engines_order_and_repeated_names_are_refused() {
        let mut broken = ac_fixture();
        broken.rows.swap(0, 1);
        assert!(broken.validate().is_err());

        let mut broken = ac_fixture();
        broken.rows[1].parameter = "C1".to_owned();
        assert!(broken.validate().is_err());
    }

    /// The normalized column and the nominal output must agree about which
    /// points had an output to divide by. Either one alone is a claim.
    #[test]
    fn a_normalized_column_must_agree_with_its_own_nominal_output() {
        let mut zeroed = ac_fixture();
        let SensitivityBasisEvidence::Ac { output, .. } = &mut zeroed.basis else {
            panic!("the AC fixture is an AC study");
        };
        output[1] = ComplexResultValue {
            real: 0.0,
            imaginary: 0.0,
        };
        assert!(
            zeroed.validate().is_err(),
            "a zero nominal output with an available normalized value is not the engine's answer"
        );
        for row in &mut zeroed.rows {
            row.normalized[1] =
                SensitivityValue::unavailable(SensitivityUnavailability::ZeroOutput);
        }
        zeroed
            .validate()
            .expect("a zero output normalized as zero-output is the engine's own answer");

        let mut invented = ac_fixture();
        invented.rows[0].normalized[0] =
            SensitivityValue::unavailable(SensitivityUnavailability::ZeroOutput);
        assert!(invented.validate().is_err());
    }

    #[test]
    fn a_refused_input_and_a_non_finite_value_are_not_evidence() {
        let mut broken = dc_fixture();
        broken.rows[0].raw[0] =
            SensitivityValue::unavailable(SensitivityUnavailability::InvalidInput);
        assert!(broken.validate().is_err());

        let mut broken = dc_fixture();
        broken.rows[0].raw[0] = SensitivityValue::Available(f64::NAN);
        assert!(broken.validate().is_err());

        let mut broken = dc_fixture();
        broken.rows[0].nominal_value = f64::INFINITY;
        assert!(broken.validate().is_err());

        let mut broken = dc_fixture();
        broken.filter = "PARAM:*\n".to_owned();
        assert!(broken.validate().is_err());

        let mut broken = dc_fixture();
        broken.output = "  ".to_owned();
        assert!(broken.validate().is_err());
    }
}
