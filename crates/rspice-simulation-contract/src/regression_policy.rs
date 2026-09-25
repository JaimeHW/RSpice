//! Authored regression tolerance policy attached to a simulation plan.

use std::collections::HashMap;

use rspice_app_types::product::AnalysisInstanceId;
use rspice_results::provenance::AnalysisResultSourceDomain;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionComparisonMethod {
    AbsoluteRelativeEnvelope,
    PointwiseRelative,
}

impl RegressionComparisonMethod {
    pub const ALL: [Self; 2] = [Self::AbsoluteRelativeEnvelope, Self::PointwiseRelative];

    pub const fn label(self) -> &'static str {
        match self {
            Self::AbsoluteRelativeEnvelope => "Absolute + relative envelope",
            Self::PointwiseRelative => "Pointwise relative",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionTargetKind {
    Measurement,
    Waveform,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionTargetSelector {
    pub source_domain: AnalysisResultSourceDomain,
    pub source_instance_id: AnalysisInstanceId,
    pub kind: RegressionTargetKind,
    pub name: String,
    pub occurrence: u32,
}

impl RegressionTargetSelector {
    fn validate(&self) -> Result<(), String> {
        if self.source_domain == AnalysisResultSourceDomain::LegacyUnclassified {
            return Err(
                "legacy-unclassified result sources cannot own regression policy".to_owned(),
            );
        }
        if self.name.trim().is_empty() {
            return Err("target name must not be empty".to_owned());
        }
        if self.name != self.name.trim() {
            return Err("target name must not have surrounding whitespace".to_owned());
        }
        if self.name.chars().any(char::is_control) {
            return Err("target name must not contain control characters".to_owned());
        }
        if self.name.graphemes(true).count() > 256 {
            return Err("target name exceeds 256 grapheme clusters".to_owned());
        }
        Ok(())
    }

    /// Remap a plan-owned source when the plan's analysis identities are cloned.
    pub fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        if self.source_domain == AnalysisResultSourceDomain::SimulationPlan {
            cloned.source_instance_id = analysis_identity_map
                .get(&self.source_instance_id)
                .copied()
                .ok_or(self.source_instance_id)?;
        }
        Ok(cloned)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionComparisonWindow {
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionToleranceRule {
    pub target: RegressionTargetSelector,
    pub method: RegressionComparisonMethod,
    /// Absolute value-domain tolerance in the target's retained base unit.
    pub absolute_tolerance: f64,
    /// Relative tolerance as a fraction (`0.005` = `0.5%`).
    pub relative_tolerance: f64,
    /// Maximum horizontal displacement in the waveform X-axis base unit.
    pub time_skew_allowance: f64,
    /// Optional inclusive X-axis comparison window. Measurements use `None`.
    pub comparison_window: Option<RegressionComparisonWindow>,
}

impl RegressionToleranceRule {
    pub fn validate(&self) -> Result<(), String> {
        self.target.validate()?;
        for (label, value) in [
            ("absolute tolerance", self.absolute_tolerance),
            ("relative tolerance", self.relative_tolerance),
            ("time-skew allowance", self.time_skew_allowance),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(format!("{label} must be finite and nonnegative"));
            }
        }
        if self.target.kind == RegressionTargetKind::Measurement
            && (self.time_skew_allowance != 0.0 || self.comparison_window.is_some())
        {
            return Err(
                "measurement targets cannot define time skew or a comparison window".to_owned(),
            );
        }
        if let Some(window) = self.comparison_window {
            if !window.start.is_finite() || !window.end.is_finite() {
                return Err("comparison-window bounds must be finite".to_owned());
            }
            if window.start > window.end {
                return Err("comparison-window start must not exceed its end".to_owned());
            }
        }
        Ok(())
    }

    /// Preserve a rule while remapping its plan-owned analysis target.
    pub fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.target = self.target.cloned_for_new_plan(analysis_identity_map)?;
        Ok(cloned)
    }
}
