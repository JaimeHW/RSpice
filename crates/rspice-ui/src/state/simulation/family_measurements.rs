//! Measurements a result family's individual members produced.
//!
//! A run that solves one circuit retains one measurement per name, and a
//! specification bound to that name has one answer. A run that solves a family
//! — a parametric sweep, a corner sweep, a Monte Carlo trial set — has as many
//! answers as it has members, and reducing them to one before a limit sees them
//! is how a specification comes to pass while a retained member failed it.
//!
//! The point-family route already avoids that: it dispatches one authorized
//! task per declared point, so each point arrives as its own
//! [`AnalysisResult`](super::AnalysisResult) carrying its own `.MEAS` evaluation
//! and its own PVT attribution, and the ordinary worst-of join answers a limit
//! correctly without knowing a sweep happened. An in-analysis family cannot do
//! that — it is one task, and its members never become separate results — so it
//! states its members' measurements here instead, and the same join reads them.
//!
//! Two things travel with every member's measurements and neither is optional.
//! The member's own identity, because "the worst trial" is not a verdict unless
//! it can be named and re-run. And the member's own pass flag, because a member
//! whose measurement failed is not a member that passed — it is evidence the
//! family could not measure, which is a different thing from a value out of
//! bounds and must never be collapsed into one.

use serde::{Deserialize, Serialize};

/// Which member of a result family produced one measurement.
///
/// The variants are not interchangeable spellings of "index". A Monte Carlo
/// trial is reproduced from its seed and nothing else, so the seed is retained
/// with it; re-running trial 47 of a 500-trial analysis means re-deriving that
/// seed, and a trial index alone cannot do it. A swept point is named by the
/// value it was solved at, and a corner by the label its section carries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "member", rename_all = "snake_case", deny_unknown_fields)]
pub enum FamilyMemberId {
    /// One Monte Carlo trial, with the exact seed that reproduces it.
    ///
    /// `index` counts trials as the analysis requested them, not as it retained
    /// them: a trial that failed to converge is dropped from the distribution,
    /// so the retained members of a 500-trial analysis are not 0..500 and must
    /// never be renumbered into that range. A verdict that names trial 47 has
    /// to mean the trial the driver called 47.
    MonteCarloTrial { index: usize, seed: u64 },
    /// One point of a parametric sweep, named by the value it was solved at.
    SweepPoint { index: usize, value: f64 },
    /// One corner of a corner sweep, named by its section label.
    Corner { index: usize, label: String },
}

impl FamilyMemberId {
    /// Position in the family's own execution order.
    #[must_use]
    pub const fn index(&self) -> usize {
        match self {
            Self::MonteCarloTrial { index, .. }
            | Self::SweepPoint { index, .. }
            | Self::Corner { index, .. } => *index,
        }
    }

    /// How the member is named where a verdict reports it.
    ///
    /// A swept value is rendered at full `f64` precision rather than rounded
    /// for width: this string is the operator's route back to the exact point,
    /// and a rounded one can name a point the sweep never solved.
    #[must_use]
    pub fn label(&self) -> String {
        match self {
            Self::MonteCarloTrial { index, .. } => format!("Trial {index}"),
            Self::SweepPoint { value, .. } => format!("Point {value}"),
            Self::Corner { label, .. } => label.clone(),
        }
    }
}

/// One measurement one family member produced.
///
/// A retained projection of [`rspice_core::MeasureResult`] rather than the type
/// itself, because this is immutable persisted evidence and the engine's result
/// type is not serializable. Only the fields a verdict actually consults are
/// kept: the value, whether the member's own measurement succeeded, and why it
/// did not when it failed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyMeasurementEvidence {
    pub name: String,
    /// The measured value, when the member produced a finite one.
    ///
    /// `None` is the honest record of a measurement that ran and could not
    /// produce a number. It is never a zero and never a skipped member.
    pub value: Option<f64>,
    /// Whether this member's own measurement succeeded.
    pub passed: bool,
    /// Why the measurement failed, when it did.
    pub error: Option<String>,
}

impl FamilyMeasurementEvidence {
    /// Whether this evidence can answer a limit at all.
    #[must_use]
    pub const fn is_measured(&self) -> bool {
        self.passed && self.value.is_some()
    }
}

/// Everything one member of a result family measured, with its identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyMemberMeasurements {
    pub member: FamilyMemberId,
    pub measurements: Vec<FamilyMeasurementEvidence>,
}

impl FamilyMemberMeasurements {
    #[must_use]
    pub fn new(member: FamilyMemberId, measurements: Vec<FamilyMeasurementEvidence>) -> Self {
        Self {
            member,
            measurements,
        }
    }

    /// This member's evidence for one measurement name, matched the way every
    /// other specification join matches: case-insensitively.
    #[must_use]
    pub fn evidence_for(&self, name: &str) -> Option<&FamilyMeasurementEvidence> {
        self.measurements
            .iter()
            .find(|evidence| evidence.name.eq_ignore_ascii_case(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(name: &str, value: Option<f64>, passed: bool) -> FamilyMeasurementEvidence {
        FamilyMeasurementEvidence {
            name: name.to_owned(),
            value,
            passed,
            error: (!passed).then(|| "measurement failed".to_owned()),
        }
    }

    #[test]
    fn a_members_evidence_is_found_the_way_every_other_specification_join_matches() {
        let member = FamilyMemberMeasurements::new(
            FamilyMemberId::MonteCarloTrial { index: 3, seed: 91 },
            vec![evidence("V(out)", Some(1.8), true)],
        );

        assert!(member.evidence_for("v(OUT)").is_some());
        assert!(member.evidence_for("V(in)").is_none());
    }

    #[test]
    fn a_dropped_trial_keeps_the_number_the_driver_called_it() {
        // Trials 1 and 3 failed to converge and were never retained. A survivor
        // must keep the index the driver requested it under, or a verdict
        // naming "trial 2" sends an operator to re-run a different circuit.
        let member = FamilyMemberMeasurements::new(
            FamilyMemberId::MonteCarloTrial { index: 4, seed: 77 },
            vec![evidence("V(out)", Some(1.8), true)],
        );

        assert_eq!(member.member.index(), 4);
        assert_eq!(member.member.label(), "Trial 4");
    }

    #[test]
    fn a_swept_point_names_itself_by_the_value_it_was_solved_at() {
        let point = FamilyMemberId::SweepPoint {
            index: 2,
            value: 27.5,
        };

        assert_eq!(point.index(), 2);
        assert_eq!(point.label(), "Point 27.5");
    }

    #[test]
    fn a_corner_names_itself_by_its_section_label() {
        let corner = FamilyMemberId::Corner {
            index: 1,
            label: "SS_1.08V_125C".to_owned(),
        };

        assert_eq!(corner.label(), "SS_1.08V_125C");
    }

    #[test]
    fn evidence_with_no_value_cannot_answer_a_limit() {
        assert!(!evidence("gain", None, true).is_measured());
    }

    /// A failed measurement that kept its raw payload is still not an answer.
    ///
    /// A GOAL/TOL check can miss and retain the number it measured. That number
    /// is worth showing and must never be counted as a limit that held, so the
    /// value and the verdict are two fields rather than one nullable one.
    #[test]
    fn a_failed_measurement_is_not_measured_even_when_it_kept_its_payload() {
        let failed = evidence("gain", Some(12.0), false);

        assert_eq!(failed.value, Some(12.0));
        assert!(!failed.is_measured());
        assert_eq!(failed.error.as_deref(), Some("measurement failed"));
    }
}
