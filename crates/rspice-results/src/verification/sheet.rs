//! One row per specification, one cell per point, judged by the sealed judge.

use rspice_vocabulary::product::{AnalysisInstanceId, SimulationPlanId, SpecificationId};

use super::points::{Point, PointAxis, RunPoints};
use super::statistics::{JointYield, Statistics};
use crate::analysis_result::AnalysisResult;
use crate::family_measurements::FamilyMemberId;
use crate::run::{EvidenceDomain, SimulationRun};
use crate::specification::{
    PreparedSpecification, PreparedSpecificationPolicy, SpecPointScope, SpecificationComparison,
    SpecificationRole, SpecificationWaiver,
};
use crate::specification_verdict::{
    Candidate, SpecificationVerdict, SpecificationVerdictStatus, blocks_acceptance, candidates_for,
    status_of, verdict_over,
};
use crate::waveform::RetainedWaveform;

/// A passing value within this fraction of its limit is near it: of the
/// limit's own magnitude for a one-sided limit, of the span for a range.
pub const NEAR_LIMIT_FRACTION: f64 = 0.05;

/// What one point says about one specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellStatus {
    /// Measured, and inside the guarded limit when there is one.
    Pass,
    /// A pass within [`NEAR_LIMIT_FRACTION`] of the limit.
    Near,
    /// Outside the guarded limit.
    Fail,
    /// The measurement itself failed.
    Error,
    /// The specification's scope does not admit this point.
    Excluded,
    /// The point gave no value for the specification.
    Absent,
}

/// One point's answer to one specification.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub status: CellStatus,
    /// The worst value the point gave, in the specification's unit.
    pub value: Option<f64>,
    /// That value's margin to the guarded limit; negative once broken.
    pub signed_margin: Option<f64>,
    /// How many values answered at this point: more than one where analyses
    /// or Monte Carlo trials share it.
    pub evidence_count: usize,
}

/// What a specification's row says of the run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowStatus {
    Pass,
    /// A pass whose worst value is within [`NEAR_LIMIT_FRACTION`] of its limit.
    Near,
    Fail,
    /// The worst evidence is a measurement that failed.
    Error,
    /// No evidence answered it.
    NotMeasured,
    /// It has a waiver and would otherwise fail.
    Waived,
    /// It has no limit, so its values are recorded and not judged.
    Tracked,
}

/// The value that decided a row, and where it came from.
#[derive(Debug, Clone, PartialEq)]
pub struct Worst {
    pub value: Option<f64>,
    pub signed_margin: Option<f64>,
    /// The point it was solved at.
    pub point: Option<usize>,
    /// The family member that gave it, when one did: a corner, a sweep point
    /// or a Monte Carlo trial.
    pub member: Option<FamilyMemberId>,
}

/// One specification across every point of the run.
#[derive(Debug, Clone)]
pub struct Row {
    pub specification_id: Option<SpecificationId>,
    /// The requirement's name, or its measurement when it has none.
    pub name: String,
    pub requirement_key: Option<String>,
    pub measurement: String,
    pub comparison: SpecificationComparison,
    pub unit: String,
    pub role: SpecificationRole,
    pub scope: SpecPointScope,
    pub waiver: Option<SpecificationWaiver>,
    pub guard_band: f64,
    pub producing_analysis: Option<AnalysisInstanceId>,
    /// The value at the run's nominal point.
    pub nominal_value: Option<f64>,
    pub worst: Option<Worst>,
    /// One cell per point of the sheet, in the sheet's point order.
    pub cells: Vec<Cell>,
    /// `None` when the run does not answer this plan's limits.
    pub status: Option<RowStatus>,
    /// The sealed judge's verdict, which `status` restates.
    pub verdict: Option<SpecificationVerdict>,
    /// The Monte Carlo trials that answered it, when any did.
    pub statistics: Option<Statistics>,
}

/// How many rows say each thing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StatusCounts {
    pub pass: usize,
    pub near: usize,
    pub fail: usize,
    pub error: usize,
    pub not_measured: usize,
    pub waived: usize,
    pub tracked: usize,
}

impl StatusCounts {
    fn add(&mut self, status: RowStatus) {
        let count = match status {
            RowStatus::Pass => &mut self.pass,
            RowStatus::Near => &mut self.near,
            RowStatus::Fail => &mut self.fail,
            RowStatus::Error => &mut self.error,
            RowStatus::NotMeasured => &mut self.not_measured,
            RowStatus::Waived => &mut self.waived,
            RowStatus::Tracked => &mut self.tracked,
        };
        *count += 1;
    }

    /// The rows that carry a verdict: every row but the tracked ones.
    #[must_use]
    pub const fn judged(&self) -> usize {
        self.pass + self.near + self.fail + self.error + self.not_measured + self.waived
    }
}

/// Whether the run can be accepted against the plan's policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Acceptance {
    pub blocked: bool,
    /// The rows that block it, in row order.
    pub blocking: Vec<usize>,
}

/// Whether the plan's limits are still the ones the run sealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitsSinceRun {
    Unchanged,
    /// The plan's specifications or policy differ from the ones sealed with
    /// the run; the sheet judges the run against the plan's.
    Edited,
    /// The run sealed no limits of this plan to compare with.
    NotSealed,
}

/// The sheet's roll-up.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    /// Whose evidence the run is; only this plan's (or history from before
    /// runs recorded an owner) is judged.
    pub evidence: EvidenceDomain,
    pub counts: StatusCounts,
    /// `None` when the run does not answer this plan's limits.
    pub acceptance: Option<Acceptance>,
    pub limits: LimitsSinceRun,
}

/// One run judged against one plan's specifications.
#[derive(Debug, Clone)]
pub struct Sheet {
    /// The points the run solved, in run order with the nominal point first.
    pub points: Vec<Point>,
    /// The coordinates that vary across the points, in heading order.
    pub axes: Vec<PointAxis>,
    /// One row per specification, in plan order.
    pub rows: Vec<Row>,
    pub summary: Summary,
    /// Trials that passed every specification with a yield; `None` when no
    /// specification has one.
    pub joint_yield: Option<JointYield>,
}

impl Sheet {
    /// Judge one run's retained evidence against a plan's specifications and
    /// policy, as they stand now.
    ///
    /// Every verdict is the sealed judge's: the cells are its own candidates,
    /// placed at the point each came from, and a row's status restates the
    /// verdict it gives on the same evidence. A run another plan owns, or a
    /// manual deck, is shown with its values and no verdict.
    #[must_use]
    pub fn build<A, W>(
        specifications: &[PreparedSpecification],
        policy: &PreparedSpecificationPolicy,
        run: Option<&SimulationRun<A>>,
        plan_id: Option<SimulationPlanId>,
    ) -> Self
    where
        A: AsRef<AnalysisResult<W>>,
        W: AsRef<RetainedWaveform>,
    {
        let evidence = run.map_or(EvidenceDomain::NoDataset, |run| {
            run.evidence_domain(plan_id)
        });
        let judged = evidence.answers_a_plan_limit();
        let analyses = run.map_or_else(Vec::new, |run| {
            run.analyses.iter().map(AsRef::as_ref).collect::<Vec<_>>()
        });
        let points = RunPoints::collect(analyses.iter().copied(), |analysis| {
            analysis.provenance().is_some()
        });
        let rows = specifications
            .iter()
            .map(|specification| row(specification, &analyses, &points, judged))
            .collect::<Vec<_>>();

        let mut counts = StatusCounts::default();
        for status in rows.iter().filter_map(|row| row.status) {
            counts.add(status);
        }
        let acceptance = judged.then(|| {
            let blocking = specifications
                .iter()
                .zip(&rows)
                .enumerate()
                .filter_map(|(index, (specification, row))| {
                    let verdict = row.verdict.as_ref()?;
                    blocks_acceptance(
                        specification,
                        policy.policy(),
                        verdict,
                        analyses.iter().copied(),
                    )
                    .then_some(index)
                })
                .collect::<Vec<_>>();
            Acceptance {
                blocked: !blocking.is_empty(),
                blocking,
            }
        });
        let limits = match (evidence, run.and_then(SimulationRun::prepared_receipt)) {
            (EvidenceDomain::ThisPlan, Some(receipt)) => {
                if receipt.specifications() == specifications
                    && receipt.specification_policy() == policy
                {
                    LimitsSinceRun::Unchanged
                } else {
                    LimitsSinceRun::Edited
                }
            }
            _ => LimitsSinceRun::NotSealed,
        };
        let joint_yield = super::statistics::joint_yield(&rows);
        Self {
            axes: points.varying_axes(),
            points: points.points,
            rows,
            summary: Summary {
                evidence,
                counts,
                acceptance,
                limits,
            },
            joint_yield,
        }
    }

    /// Whether the rows carry verdicts: the run answers this plan's limits.
    #[must_use]
    pub const fn is_judged(&self) -> bool {
        self.summary.evidence.answers_a_plan_limit()
    }

    /// Why the rows carry no verdict, when they do not.
    #[must_use]
    pub const fn refusal(&self) -> Option<&'static str> {
        self.summary.evidence.refusal()
    }
}

fn row<W: AsRef<RetainedWaveform>>(
    specification: &PreparedSpecification,
    analyses: &[&AnalysisResult<W>],
    points: &RunPoints,
    judged: bool,
) -> Row {
    let entry = specification.entry();
    let definition = specification.definition();
    let limited = entry.min.is_some() || entry.max.is_some();
    let candidates = candidates_for(specification, analyses.iter().copied());

    let mut at_point: Vec<Vec<&Candidate>> = vec![Vec::new(); points.points.len()];
    for candidate in &candidates {
        if let Some(point) = points.of_candidate(candidate.analysis, candidate.member.as_ref()) {
            at_point[point].push(candidate);
        }
    }
    let cells = at_point
        .into_iter()
        .enumerate()
        .map(|(point, mut evidence)| {
            if evidence.is_empty() {
                let admitted = points.analyses_at(point).iter().any(|position| {
                    entry.scope.admits(
                        analyses[*position]
                            .provenance()
                            .and_then(|provenance| provenance.pvt_point()),
                    )
                });
                return Cell {
                    status: if admitted {
                        CellStatus::Absent
                    } else {
                        CellStatus::Excluded
                    },
                    value: None,
                    signed_margin: None,
                    evidence_count: 0,
                };
            }
            let status = status_of(&mut evidence);
            let worst = evidence[0];
            Cell {
                status: cell_status(status, worst, entry.min, entry.max, judged),
                value: worst.value,
                signed_margin: worst.signed_margin,
                evidence_count: evidence.len(),
            }
        })
        .collect::<Vec<_>>();

    let mut ordered = candidates.iter().collect::<Vec<_>>();
    let verdict = verdict_over(specification, &mut ordered);
    let worst = ordered.first().map(|worst| Worst {
        value: worst.value,
        signed_margin: worst.signed_margin,
        point: points.of_candidate(worst.analysis, worst.member.as_ref()),
        member: worst.member.clone(),
    });
    let status = judged.then(|| {
        let base = match verdict.status() {
            SpecificationVerdictStatus::Pass => {
                if ordered
                    .first()
                    .is_some_and(|worst| is_near(worst.value, entry.min, entry.max))
                {
                    RowStatus::Near
                } else {
                    RowStatus::Pass
                }
            }
            SpecificationVerdictStatus::BoundFailure => RowStatus::Fail,
            SpecificationVerdictStatus::MeasurementFailure => RowStatus::Error,
            SpecificationVerdictStatus::MissingEvidence => RowStatus::NotMeasured,
        };
        let waived = definition.is_some_and(|definition| definition.waiver.is_some());
        // A failed measurement is never hidden: having no limit leaves a row
        // unjudged, not unchecked.
        match base {
            RowStatus::Fail | RowStatus::Error if waived => RowStatus::Waived,
            RowStatus::Error => RowStatus::Error,
            _ if !limited => RowStatus::Tracked,
            base => base,
        }
    });
    let statistics = super::statistics::row_statistics(
        &entry.measurement,
        (judged && limited).then_some((entry.min, entry.max)),
        &candidates,
        points,
    );

    Row {
        specification_id: definition.map(|definition| definition.id),
        name: definition
            .map(|definition| definition.requirement_name.trim())
            .filter(|name| !name.is_empty())
            .unwrap_or(entry.measurement.as_str())
            .to_owned(),
        requirement_key: definition.map(|definition| definition.requirement_key.clone()),
        measurement: entry.measurement.clone(),
        comparison: definition.map_or_else(
            || match (entry.min, entry.max) {
                (None, None) => SpecificationComparison::Tracked,
                (Some(limit), None) => SpecificationComparison::Minimum { limit },
                (None, Some(limit)) => SpecificationComparison::Maximum { limit },
                (Some(minimum), Some(maximum)) => {
                    SpecificationComparison::Range { minimum, maximum }
                }
            },
            |definition| definition.comparison.clone(),
        ),
        unit: entry.unit.clone(),
        role: definition.map_or(SpecificationRole::Blocking, |definition| definition.role),
        scope: entry.scope.clone(),
        waiver: definition.and_then(|definition| definition.waiver.clone()),
        guard_band: specification.guard_band(),
        producing_analysis: definition.and_then(|definition| definition.producing_analysis),
        nominal_value: points
            .points
            .iter()
            .position(|point| point.nominal)
            .and_then(|point| cells[point].value),
        worst,
        cells,
        status,
        verdict: judged.then_some(verdict),
        statistics,
    }
}

/// One point's status from the judge's status over its evidence. A run that
/// does not answer this plan's limits is read as if the row had none.
fn cell_status(
    status: SpecificationVerdictStatus,
    worst: &Candidate,
    minimum: Option<f64>,
    maximum: Option<f64>,
    judged: bool,
) -> CellStatus {
    match status {
        SpecificationVerdictStatus::MeasurementFailure => CellStatus::Error,
        SpecificationVerdictStatus::BoundFailure if judged => CellStatus::Fail,
        SpecificationVerdictStatus::Pass if judged && is_near(worst.value, minimum, maximum) => {
            CellStatus::Near
        }
        SpecificationVerdictStatus::Pass
        | SpecificationVerdictStatus::BoundFailure
        | SpecificationVerdictStatus::MissingEvidence => CellStatus::Pass,
    }
}

/// Whether a passing value sits within [`NEAR_LIMIT_FRACTION`] of its limit.
fn is_near(value: Option<f64>, minimum: Option<f64>, maximum: Option<f64>) -> bool {
    value
        .and_then(|value| normalized_margin(minimum, maximum, value))
        .is_some_and(|margin| margin < NEAR_LIMIT_FRACTION)
}

/// Room before the limit, as a fraction of it: of the limit's magnitude for a
/// one-sided limit, of the span for a range. A limit with no width, or none,
/// has no fraction to state.
fn normalized_margin(minimum: Option<f64>, maximum: Option<f64>, value: f64) -> Option<f64> {
    const SCALE_FLOOR: f64 = 1.0e-30;
    match (minimum, maximum) {
        (Some(minimum), Some(maximum)) if maximum > minimum => {
            Some((value - minimum).min(maximum - value) / (maximum - minimum).max(SCALE_FLOOR))
        }
        (Some(minimum), None) => Some((value - minimum) / minimum.abs().max(SCALE_FLOOR)),
        (None, Some(maximum)) => Some((maximum - value) / maximum.abs().max(SCALE_FLOOR)),
        (Some(_), Some(_)) | (None, None) => None,
    }
}
