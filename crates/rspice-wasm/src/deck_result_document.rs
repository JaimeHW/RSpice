//! Retained, versioned results for canonical authored-deck execution.
//!
//! Unlike the scalar result handles, this document preserves the complete
//! coordinate/analysis identity product.  Each analog result keeps its own
//! coordinate-local schema; consumers request its descriptors and bounded
//! numeric windows independently.

use std::collections::HashSet;

use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AxisAssignment, AxisKind, DeckPlan, RunAxisValue,
    RunCoordinate, StepAxisTarget,
};
use rspice_core::{AbortSignal, NoAbort};
use serde::Serialize;

use crate::{
    AnalogResultDocument, AnalogResultMetadata, AnalogResultWindow, TransientFftHarmonicsSnapshot,
    TransientFftMetricsSnapshot, TransientFftSnapshot,
};

pub const DECK_RESULT_SCHEMA: &str = "rspice-deck-result";
pub const DECK_RESULT_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckAxisDescriptor {
    pub kind: String,
    pub name: String,
    pub target: Option<String>,
    pub value_count: usize,
    pub data_bindings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckPlannedAnalysisDescriptor {
    pub analysis_instance_id: String,
    pub kind: String,
    pub ordinal: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckDataBinding {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DeckAxisValue {
    Numeric { value: f64 },
    DataRow { bindings: Vec<DeckDataBinding> },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckAxisAssignment {
    pub kind: String,
    pub name: String,
    pub target: Option<String>,
    pub value_index: usize,
    pub value: DeckAxisValue,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckCoordinateDescriptor {
    pub index: usize,
    pub id: String,
    pub namespace: String,
    pub assignments: Vec<DeckAxisAssignment>,
}

#[derive(Debug)]
pub struct DeckAnalogResult {
    pub coordinate_index: usize,
    pub analysis_instance_id: String,
    pub output_namespace: String,
    pub checkpoint_namespace: String,
    pub document: AnalogResultDocument,
}

#[derive(Debug)]
pub struct DeckFftResult {
    pub coordinate_index: usize,
    pub parent_result_index: usize,
    pub output_namespace: String,
    pub snapshot: TransientFftSnapshot,
}

#[derive(Debug)]
pub struct DeckResultDocument {
    pub axes: Vec<DeckAxisDescriptor>,
    pub planned_analyses: Vec<DeckPlannedAnalysisDescriptor>,
    pub coordinates: Vec<DeckCoordinateDescriptor>,
    pub results: Vec<DeckAnalogResult>,
    pub fft_results: Vec<DeckFftResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckResultSummary {
    pub index: usize,
    pub coordinate_index: usize,
    pub coordinate_id: String,
    pub analysis_instance_id: String,
    pub output_namespace: String,
    pub checkpoint_namespace: String,
    pub kind: crate::AnalogAnalysisKind,
    pub point_count: usize,
    pub signal_count: usize,
    pub device_state_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckFftSummary {
    pub index: usize,
    pub coordinate_index: usize,
    pub coordinate_id: String,
    pub parent_result_index: usize,
    pub analysis_id: String,
    pub parent_analysis_id: String,
    pub output_namespace: String,
    pub bin_count: usize,
    pub harmonic_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckResultMetadata {
    pub schema: String,
    pub schema_version: u32,
    pub axes: Vec<DeckAxisDescriptor>,
    pub planned_analyses: Vec<DeckPlannedAnalysisDescriptor>,
    pub coordinates: Vec<DeckCoordinateDescriptor>,
    pub results: Vec<DeckResultSummary>,
    pub fft_results: Vec<DeckFftSummary>,
    pub maximum_window_values: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckFftMetadata {
    pub schema: String,
    pub schema_version: u32,
    pub coordinate_id: String,
    pub parent_result_index: usize,
    pub output_namespace: String,
    pub analysis_id: String,
    pub parent_analysis_id: String,
    pub ordinal: usize,
    pub source_kind: String,
    pub source_text: String,
    pub authored_output: String,
    pub output_name: String,
    pub physical_type: String,
    pub value_unit: Option<String>,
    pub start_time: f64,
    pub stop_time: f64,
    pub sample_interval: f64,
    pub point_count: usize,
    pub accurate_sampling: bool,
    pub format: String,
    pub mode: String,
    pub window: String,
    pub window_name: String,
    pub alpha: f64,
    pub coherent_gain: f64,
    pub frequency_resolution: f64,
    pub fundamental_bin: usize,
    pub minimum_metric_bin: usize,
    pub maximum_metric_bin: usize,
    pub bin_count: usize,
    pub metrics: Option<DeckFftMetricsMetadata>,
    pub maximum_window_values: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckFftMetricsMetadata {
    pub fundamental_magnitude: f64,
    pub thd_ratio: f64,
    pub thd_db: f64,
    pub sndr_db: f64,
    pub enob_bits: f64,
    pub snr_db: f64,
    pub sfdr_db: f64,
    pub sfdr_spur_bin: Option<usize>,
    pub sfdr_spur_frequency: Option<f64>,
    pub harmonic_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckFftBinWindow {
    pub schema_version: u32,
    pub analysis_id: String,
    pub coordinate_id: String,
    pub start: usize,
    pub end: usize,
    pub point_count: usize,
    pub indices: Vec<usize>,
    pub frequencies: Vec<f64>,
    pub real: Vec<f64>,
    pub imaginary: Vec<f64>,
    pub magnitudes: Vec<f64>,
    pub phase_degrees: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckFftHarmonicWindow {
    pub schema_version: u32,
    pub analysis_id: String,
    pub coordinate_id: String,
    pub start: usize,
    pub end: usize,
    pub point_count: usize,
    pub ranks: Vec<usize>,
    pub bins: Vec<usize>,
    pub frequencies: Vec<f64>,
    pub magnitudes: Vec<f64>,
    pub magnitudes_db: Vec<f64>,
    pub phase_degrees: Vec<f64>,
}

impl DeckResultDocument {
    pub fn new(plan: &DeckPlan) -> Result<Self, String> {
        let mut axes = try_vec(plan.axes().len(), "deck axis descriptors")?;
        for axis in plan.axes() {
            let mut data_bindings = Vec::new();
            if let Some(RunAxisValue::DataRow(bindings)) = axis.values().first() {
                data_bindings = try_vec(bindings.len(), "deck DATA axis binding descriptors")?;
                for binding in bindings {
                    data_bindings.push(binding.name().to_owned());
                }
            }
            axes.push(DeckAxisDescriptor {
                kind: axis_kind_name(axis.kind())?.to_owned(),
                name: axis.name().to_owned(),
                target: axis.step_target().map(step_target_name).transpose()?,
                value_count: axis.values().len(),
                data_bindings,
            });
        }
        let mut planned_analyses =
            try_vec(plan.analyses().len(), "deck planned-analysis descriptors")?;
        for analysis in plan.analyses() {
            planned_analyses.push(DeckPlannedAnalysisDescriptor {
                analysis_instance_id: analysis.id().tag(),
                kind: analysis_kind_name(analysis.id().kind())?.to_owned(),
                ordinal: analysis.id().ordinal() as usize + 1,
            });
        }
        Ok(Self {
            axes,
            planned_analyses,
            coordinates: Vec::new(),
            results: Vec::new(),
            fft_results: Vec::new(),
        })
    }

    pub fn push_coordinate(&mut self, coordinate: &RunCoordinate) -> Result<usize, String> {
        let index = self.coordinates.len();
        if coordinate.ordinal() != index {
            return Err(format!(
                "canonical coordinate ordinal {} does not match aggregate index {index}",
                coordinate.ordinal()
            ));
        }
        let mut assignments = try_vec(
            coordinate.assignments().len(),
            "deck coordinate assignments",
        )?;
        for assignment in coordinate.assignments() {
            assignments.push(assignment_descriptor(assignment)?);
        }
        self.coordinates
            .try_reserve(1)
            .map_err(|_| "could not allocate a deck coordinate descriptor".to_owned())?;
        self.coordinates.push(DeckCoordinateDescriptor {
            index,
            id: coordinate.stable_id().to_string(),
            namespace: coordinate.stable_tag(),
            assignments,
        });
        Ok(index)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate_with_abort(&NoAbort)
    }

    pub fn validate_with_abort(&self, abort: &dyn AbortSignal) -> Result<(), String> {
        ensure_not_aborted(abort)?;
        if self.coordinates.is_empty() || self.results.is_empty() {
            return Err(
                "deck results require at least one coordinate and one analog result".into(),
            );
        }
        if self.planned_analyses.is_empty() {
            return Err("deck results require at least one planned analysis".into());
        }
        validate_axis_descriptors(&self.axes)?;
        let expected_coordinate_count = self.axes.iter().try_fold(1usize, |total, axis| {
            total
                .checked_mul(axis.value_count)
                .ok_or_else(|| "deck coordinate cardinality overflowed usize".to_owned())
        })?;
        if self.coordinates.len() != expected_coordinate_count {
            return Err(format!(
                "deck has {} coordinates; its axis product requires {expected_coordinate_count}",
                self.coordinates.len()
            ));
        }
        let mut coordinate_ids = HashSet::new();
        coordinate_ids
            .try_reserve(self.coordinates.len())
            .map_err(|_| "could not allocate deck coordinate validation set".to_owned())?;
        let mut coordinate_index_tuples = HashSet::new();
        coordinate_index_tuples
            .try_reserve(self.coordinates.len())
            .map_err(|_| "could not allocate deck coordinate-index validation set".to_owned())?;
        for (index, coordinate) in self.coordinates.iter().enumerate() {
            poll_abort(abort, index)?;
            if coordinate.index != index
                || !valid_coordinate_id(&coordinate.id)
                || coordinate.namespace != format!("run-{}", coordinate.id)
                || !coordinate_ids.insert(coordinate.id.as_str())
                || coordinate.assignments.len() != self.axes.len()
            {
                return Err("deck coordinate identity or shape is invalid".into());
            }
            let mut expected_ordinal = index;
            let mut index_tuple = try_vec(self.axes.len(), "deck coordinate-index tuple")?;
            for (axis, assignment) in self.axes.iter().zip(&coordinate.assignments) {
                validate_assignment(axis, assignment)?;
                let expected_value_index = expected_ordinal % axis.value_count;
                expected_ordinal /= axis.value_count;
                if assignment.value_index != expected_value_index {
                    return Err(
                        "deck coordinate assignments are not in canonical Cartesian order".into(),
                    );
                }
                index_tuple.push(assignment.value_index);
            }
            if expected_ordinal != 0 || !coordinate_index_tuples.insert(index_tuple) {
                return Err("deck coordinate index tuple is invalid or duplicated".into());
            }
        }
        let mut planned_ids = HashSet::new();
        planned_ids
            .try_reserve(self.planned_analyses.len())
            .map_err(|_| "could not allocate planned-analysis validation set".to_owned())?;
        let mut next_ordinals = std::collections::BTreeMap::<&str, usize>::new();
        for planned in &self.planned_analyses {
            let next = next_ordinals.entry(planned.kind.as_str()).or_insert(1);
            let _analog_kind = planned_analog_kind(&planned.kind)?;
            if planned.analysis_instance_id.trim().is_empty()
                || planned.kind.trim().is_empty()
                || planned.ordinal == 0
                || planned.ordinal != *next
                || planned.analysis_instance_id
                    != planned_analysis_tag(&planned.kind, planned.ordinal)?
                || !planned_ids.insert(planned.analysis_instance_id.as_str())
            {
                return Err("planned analysis identity is invalid".into());
            }
            *next = next
                .checked_add(1)
                .ok_or_else(|| "planned analysis ordinal overflowed usize".to_owned())?;
        }
        let expected_result_count = self
            .coordinates
            .len()
            .checked_mul(self.planned_analyses.len())
            .ok_or_else(|| "deck result cardinality overflowed usize".to_owned())?;
        if self.results.len() != expected_result_count {
            return Err(format!(
                "deck has {} coordinate-local analog results; expected {expected_result_count}",
                self.results.len()
            ));
        }
        let mut result_keys = HashSet::new();
        result_keys
            .try_reserve(self.results.len())
            .map_err(|_| "could not allocate deck result validation set".to_owned())?;
        for (index, result) in self.results.iter().enumerate() {
            poll_abort(abort, index)?;
            let expected_coordinate_index = index / self.planned_analyses.len();
            let expected_analysis = &self.planned_analyses[index % self.planned_analyses.len()];
            let expected_kind = planned_analog_kind(&expected_analysis.kind)?;
            let coordinate = self
                .coordinates
                .get(result.coordinate_index)
                .ok_or_else(|| format!("deck result {index} references an absent coordinate"))?;
            result.document.validate()?;
            if result.coordinate_index != expected_coordinate_index
                || result.analysis_instance_id != expected_analysis.analysis_instance_id
                || result.document.coordinate_id.as_deref() != Some(coordinate.id.as_str())
                || result.document.analysis.id != result.analysis_instance_id
                || result.document.analysis.ordinal != expected_analysis.ordinal
                || result.document.analysis.kind != expected_kind
                || result.output_namespace
                    != format!("{}/{}", coordinate.namespace, result.analysis_instance_id)
                || result.checkpoint_namespace != result.output_namespace
                || !result_keys.insert((
                    result.coordinate_index,
                    result.analysis_instance_id.as_str(),
                ))
            {
                return Err("deck result identity or namespace is invalid".into());
            }
        }
        let mut fft_keys = HashSet::new();
        fft_keys
            .try_reserve(self.fft_results.len())
            .map_err(|_| "could not allocate deck FFT validation set".to_owned())?;
        let mut previous_fft_parent = None;
        let mut expected_fft_ordinal = 1usize;
        for (index, fft) in self.fft_results.iter().enumerate() {
            poll_abort(abort, index)?;
            let coordinate = self.coordinates.get(fft.coordinate_index).ok_or_else(|| {
                format!("deck FFT result {index} references an absent coordinate")
            })?;
            let parent = self.results.get(fft.parent_result_index).ok_or_else(|| {
                format!("deck FFT result {index} references an absent parent result")
            })?;
            match previous_fft_parent {
                Some(previous) if previous == fft.parent_result_index => {
                    expected_fft_ordinal = expected_fft_ordinal
                        .checked_add(1)
                        .ok_or_else(|| "deck FFT ordinal overflowed usize".to_owned())?;
                }
                Some(previous) if previous > fft.parent_result_index => {
                    return Err("deck FFT results are not in canonical parent order".into());
                }
                _ => expected_fft_ordinal = 1,
            }
            previous_fft_parent = Some(fft.parent_result_index);
            if parent.coordinate_index != fft.coordinate_index
                || parent.document.analysis.kind != crate::AnalogAnalysisKind::Transient
                || fft.snapshot.parent_analysis_id != parent.analysis_instance_id
                || fft.snapshot.ordinal != expected_fft_ordinal
                || fft.output_namespace
                    != format!(
                        "{}/{}/{}",
                        coordinate.namespace, parent.analysis_instance_id, fft.snapshot.analysis_id
                    )
                || !fft_keys.insert((
                    fft.coordinate_index,
                    fft.parent_result_index,
                    fft.snapshot.analysis_id.as_str(),
                ))
            {
                return Err("deck FFT parent identity or namespace is invalid".into());
            }
            validate_fft_snapshot(&fft.snapshot)?;
        }
        Ok(())
    }

    pub fn retained_numeric_value_count(&self) -> Result<usize, String> {
        self.retained_numeric_value_count_with_abort(&NoAbort)
    }

    pub fn retained_numeric_value_count_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<usize, String> {
        let analog = self.results.iter().try_fold(0usize, |total, result| {
            ensure_not_aborted(abort)?;
            total
                .checked_add(result.document.retained_numeric_value_count())
                .ok_or_else(|| "deck analog retained-value count overflowed usize".to_owned())
        })?;
        self.fft_results.iter().try_fold(analog, |total, result| {
            ensure_not_aborted(abort)?;
            total
                .checked_add(fft_retained_numeric_value_count(&result.snapshot)?)
                .ok_or_else(|| "deck retained-value count overflowed usize".to_owned())
        })
    }

    pub fn metadata(&self, maximum_window_values: usize) -> Result<DeckResultMetadata, String> {
        self.validate()?;
        let mut results = try_vec(self.results.len(), "deck result metadata")?;
        for (index, result) in self.results.iter().enumerate() {
            let coordinate = self
                .coordinates
                .get(result.coordinate_index)
                .ok_or_else(|| format!("deck result {index} references an absent coordinate"))?;
            results.push(DeckResultSummary {
                index,
                coordinate_index: result.coordinate_index,
                coordinate_id: coordinate.id.clone(),
                analysis_instance_id: result.analysis_instance_id.clone(),
                output_namespace: result.output_namespace.clone(),
                checkpoint_namespace: result.checkpoint_namespace.clone(),
                kind: result.document.analysis.kind,
                point_count: result.document.point_count,
                signal_count: result.document.signals.len(),
                device_state_count: result.document.device_states.len(),
            });
        }
        let mut fft_results = try_vec(self.fft_results.len(), "deck FFT metadata")?;
        for (index, result) in self.fft_results.iter().enumerate() {
            let coordinate = self
                .coordinates
                .get(result.coordinate_index)
                .ok_or_else(|| {
                    format!("deck FFT result {index} references an absent coordinate")
                })?;
            fft_results.push(DeckFftSummary {
                index,
                coordinate_index: result.coordinate_index,
                coordinate_id: coordinate.id.clone(),
                parent_result_index: result.parent_result_index,
                analysis_id: result.snapshot.analysis_id.clone(),
                parent_analysis_id: result.snapshot.parent_analysis_id.clone(),
                output_namespace: result.output_namespace.clone(),
                bin_count: result.snapshot.bins.indices.len(),
                harmonic_count: harmonic_count(&result.snapshot),
            });
        }
        Ok(DeckResultMetadata {
            schema: DECK_RESULT_SCHEMA.to_owned(),
            schema_version: DECK_RESULT_VERSION,
            axes: self.axes.clone(),
            planned_analyses: self.planned_analyses.clone(),
            coordinates: self.coordinates.clone(),
            results,
            fft_results,
            maximum_window_values,
        })
    }

    pub fn result_metadata(
        &self,
        index: usize,
        maximum_window_values: usize,
    ) -> Result<AnalogResultMetadata, String> {
        let result = self.validated_analog_result(index)?;
        Ok(result.document.metadata(maximum_window_values))
    }

    pub fn result_window(
        &self,
        index: usize,
        start: usize,
        count: usize,
        maximum_window_values: usize,
    ) -> Result<AnalogResultWindow, String> {
        let result = self.validated_analog_result(index)?;
        result.document.window(start, count, maximum_window_values)
    }

    pub fn fft_metadata(
        &self,
        index: usize,
        maximum_window_values: usize,
    ) -> Result<DeckFftMetadata, String> {
        let (result, coordinate) = self.validated_fft_result(index)?;
        let fft = &result.snapshot;
        Ok(DeckFftMetadata {
            schema: DECK_RESULT_SCHEMA.to_owned(),
            schema_version: DECK_RESULT_VERSION,
            coordinate_id: coordinate.id.clone(),
            parent_result_index: result.parent_result_index,
            output_namespace: result.output_namespace.clone(),
            analysis_id: fft.analysis_id.clone(),
            parent_analysis_id: fft.parent_analysis_id.clone(),
            ordinal: fft.ordinal,
            source_kind: fft.source_kind.clone(),
            source_text: fft.source_text.clone(),
            authored_output: fft.authored_output.clone(),
            output_name: fft.output_name.clone(),
            physical_type: fft.physical_type.clone(),
            value_unit: fft.value_unit.clone(),
            start_time: fft.start_time,
            stop_time: fft.stop_time,
            sample_interval: fft.sample_interval,
            point_count: fft.point_count,
            accurate_sampling: fft.accurate_sampling,
            format: fft.format.clone(),
            mode: fft.mode.clone(),
            window: fft.window.clone(),
            window_name: fft.window_name.clone(),
            alpha: fft.alpha,
            coherent_gain: fft.coherent_gain,
            frequency_resolution: fft.frequency_resolution,
            fundamental_bin: fft.fundamental_bin,
            minimum_metric_bin: fft.minimum_metric_bin,
            maximum_metric_bin: fft.maximum_metric_bin,
            bin_count: fft.bins.indices.len(),
            metrics: fft.metrics.as_ref().map(fft_metrics_metadata),
            maximum_window_values,
        })
    }

    pub fn fft_bin_window(
        &self,
        index: usize,
        start: usize,
        count: usize,
        maximum_window_values: usize,
    ) -> Result<DeckFftBinWindow, String> {
        let (result, coordinate) = self.validated_fft_result(index)?;
        let bins = &result.snapshot.bins;
        let end = checked_window(start, count, bins.indices.len(), maximum_window_values, 6)?;
        Ok(DeckFftBinWindow {
            schema_version: DECK_RESULT_VERSION,
            analysis_id: result.snapshot.analysis_id.clone(),
            coordinate_id: coordinate.id.clone(),
            start,
            end,
            point_count: bins.indices.len(),
            indices: bins.indices[start..end].to_vec(),
            frequencies: bins.frequencies[start..end].to_vec(),
            real: bins.real[start..end].to_vec(),
            imaginary: bins.imaginary[start..end].to_vec(),
            magnitudes: bins.magnitudes[start..end].to_vec(),
            phase_degrees: bins.phase_degrees[start..end].to_vec(),
        })
    }

    pub fn fft_harmonic_window(
        &self,
        index: usize,
        start: usize,
        count: usize,
        maximum_window_values: usize,
    ) -> Result<DeckFftHarmonicWindow, String> {
        let (result, coordinate) = self.validated_fft_result(index)?;
        let harmonics = result
            .snapshot
            .metrics
            .as_ref()
            .map(|metrics| &metrics.largest_harmonics)
            .ok_or_else(|| format!("FFT result {index} has no harmonic metrics"))?;
        let end = checked_window(
            start,
            count,
            harmonics.ranks.len(),
            maximum_window_values,
            6,
        )?;
        Ok(DeckFftHarmonicWindow {
            schema_version: DECK_RESULT_VERSION,
            analysis_id: result.snapshot.analysis_id.clone(),
            coordinate_id: coordinate.id.clone(),
            start,
            end,
            point_count: harmonics.ranks.len(),
            ranks: harmonics.ranks[start..end].to_vec(),
            bins: harmonics.bins[start..end].to_vec(),
            frequencies: harmonics.frequencies[start..end].to_vec(),
            magnitudes: harmonics.magnitudes[start..end].to_vec(),
            magnitudes_db: harmonics.magnitudes_db[start..end].to_vec(),
            phase_degrees: harmonics.phase_degrees[start..end].to_vec(),
        })
    }

    fn validated_analog_result(&self, index: usize) -> Result<&DeckAnalogResult, String> {
        let result = self.results.get(index).ok_or_else(|| {
            format!(
                "analog result index {index} is outside 0..{}",
                self.results.len()
            )
        })?;
        let analyses_per_coordinate = self.planned_analyses.len();
        if analyses_per_coordinate == 0 {
            return Err("deck result has no planned analysis descriptors".into());
        }
        let expected_coordinate = index / analyses_per_coordinate;
        let expected_analysis = &self.planned_analyses[index % analyses_per_coordinate];
        let expected_kind = planned_analog_kind(&expected_analysis.kind)?;
        let coordinate = self
            .coordinates
            .get(result.coordinate_index)
            .ok_or_else(|| format!("analog result {index} references an absent coordinate"))?;
        result.document.validate()?;
        if result.coordinate_index != expected_coordinate
            || result.analysis_instance_id != expected_analysis.analysis_instance_id
            || result.document.coordinate_id.as_deref() != Some(coordinate.id.as_str())
            || result.document.analysis.id != result.analysis_instance_id
            || result.document.analysis.kind != expected_kind
            || result.output_namespace
                != format!("{}/{}", coordinate.namespace, result.analysis_instance_id)
            || result.checkpoint_namespace != result.output_namespace
        {
            return Err(format!(
                "analog result {index} has an invalid deck identity"
            ));
        }
        Ok(result)
    }

    fn validated_fft_result(
        &self,
        index: usize,
    ) -> Result<(&DeckFftResult, &DeckCoordinateDescriptor), String> {
        let result = self.fft_results.get(index).ok_or_else(|| {
            format!(
                "FFT result index {index} is outside 0..{}",
                self.fft_results.len()
            )
        })?;
        validate_fft_snapshot(&result.snapshot)?;
        let coordinate = self
            .coordinates
            .get(result.coordinate_index)
            .ok_or_else(|| format!("FFT result {index} references an absent coordinate"))?;
        let parent = self
            .results
            .get(result.parent_result_index)
            .ok_or_else(|| format!("FFT result {index} references an absent parent result"))?;
        if parent.coordinate_index != result.coordinate_index
            || parent.document.analysis.kind != crate::AnalogAnalysisKind::Transient
            || result.snapshot.parent_analysis_id != parent.analysis_instance_id
            || result.output_namespace
                != format!(
                    "{}/{}/{}",
                    coordinate.namespace, parent.analysis_instance_id, result.snapshot.analysis_id
                )
        {
            return Err(format!(
                "FFT result {index} has an invalid deck parent identity"
            ));
        }
        Ok((result, coordinate))
    }
}

pub fn set_execution_identity(
    document: &mut AnalogResultDocument,
    coordinate: &RunCoordinate,
    analysis_id: AnalysisInstanceId,
) {
    document.coordinate_id = Some(coordinate.stable_id().to_string());
    document.analysis.id = analysis_id.tag();
    document.analysis.ordinal = analysis_id.ordinal() as usize + 1;
}

pub(crate) fn fft_retained_numeric_value_count(
    snapshot: &TransientFftSnapshot,
) -> Result<usize, String> {
    let bin_values = snapshot
        .bins
        .indices
        .len()
        .checked_mul(6)
        .ok_or_else(|| "deck FFT retained-value count overflowed usize".to_owned())?;
    let harmonic_values = snapshot
        .metrics
        .as_ref()
        .map_or(0, |metrics| metrics.largest_harmonics.ranks.len())
        .checked_mul(6)
        .ok_or_else(|| "deck FFT retained-value count overflowed usize".to_owned())?;
    let metric_values = snapshot.metrics.as_ref().map_or(0usize, |metrics| {
        7usize
            .saturating_add(usize::from(metrics.sfdr_spur_bin.is_some()))
            .saturating_add(usize::from(metrics.sfdr_spur_frequency.is_some()))
    });
    bin_values
        .checked_add(harmonic_values)
        .and_then(|value| value.checked_add(metric_values))
        .ok_or_else(|| "deck FFT retained-value count overflowed usize".to_owned())
}

fn assignment_descriptor(assignment: &AxisAssignment) -> Result<DeckAxisAssignment, String> {
    let value = match assignment.value() {
        RunAxisValue::Numeric(value) => DeckAxisValue::Numeric { value: *value },
        RunAxisValue::DataRow(bindings) => {
            let mut retained = try_vec(bindings.len(), "deck DATA bindings")?;
            retained.extend(bindings.iter().map(|binding| DeckDataBinding {
                name: binding.name().to_owned(),
                value: binding.value(),
            }));
            DeckAxisValue::DataRow { bindings: retained }
        }
        RunAxisValue::AlterVariant { .. } => {
            return Err("ALTER coordinates are not supported by the browser deck API".into());
        }
        _ => {
            return Err(
                "unknown coordinate value kind is not supported by the browser deck API".into(),
            );
        }
    };
    Ok(DeckAxisAssignment {
        kind: axis_kind_name(assignment.kind())?.to_owned(),
        name: assignment.name().to_owned(),
        target: assignment.step_target().map(step_target_name).transpose()?,
        value_index: assignment.value_index(),
        value,
    })
}

fn axis_kind_name(kind: AxisKind) -> Result<&'static str, String> {
    match kind {
        AxisKind::Alter => Ok("alter"),
        AxisKind::Data => Ok("data"),
        AxisKind::Step => Ok("step"),
        AxisKind::Temperature => Ok("temperature"),
        _ => Err("unknown canonical axis kind is not supported by the browser deck API".into()),
    }
}

fn step_target_name(target: &StepAxisTarget) -> Result<String, String> {
    match target {
        StepAxisTarget::Parameter { name } => Ok(format!("parameter:{name}")),
        StepAxisTarget::Device { name, parameter } => Ok(parameter.as_ref().map_or_else(
            || format!("device:{name}"),
            |parameter| format!("device:{name}:{parameter}"),
        )),
        StepAxisTarget::Model { name, parameter } => Ok(format!("model:{name}:{parameter}")),
        StepAxisTarget::Temperature => Ok("temperature".to_owned()),
        _ => Err("unknown STEP target is not supported by the browser deck API".into()),
    }
}

fn analysis_kind_name(kind: AnalysisKind) -> Result<&'static str, String> {
    match kind {
        AnalysisKind::ImplicitOp => Ok("implicit_op"),
        AnalysisKind::Op => Ok("op"),
        AnalysisKind::Dc => Ok("dc"),
        AnalysisKind::Ac => Ok("ac"),
        AnalysisKind::Tran => Ok("tran"),
        AnalysisKind::Noise => Ok("noise"),
        AnalysisKind::Sp => Ok("sp"),
        AnalysisKind::Stb => Ok("stb"),
        AnalysisKind::Distortion => Ok("distortion"),
        AnalysisKind::PoleZero => Ok("pole_zero"),
        AnalysisKind::Sensitivity => Ok("sensitivity"),
        AnalysisKind::TransferFunction => Ok("transfer_function"),
        AnalysisKind::Pss => Ok("pss"),
        AnalysisKind::Pac => Ok("pac"),
        AnalysisKind::PNoise => Ok("pnoise"),
        AnalysisKind::HarmonicBalance => Ok("harmonic_balance"),
        AnalysisKind::Envelope => Ok("envelope"),
        AnalysisKind::MonteCarlo => Ok("monte_carlo"),
        AnalysisKind::Fourier => Ok("fourier"),
        AnalysisKind::Fft => Ok("fft"),
        _ => Err("unknown canonical analysis kind is not supported by the browser deck API".into()),
    }
}

fn validate_assignment(
    axis: &DeckAxisDescriptor,
    assignment: &DeckAxisAssignment,
) -> Result<(), String> {
    if assignment.kind != axis.kind
        || assignment.name != axis.name
        || assignment.target != axis.target
        || assignment.value_index >= axis.value_count
    {
        return Err("deck coordinate assignment does not match its axis descriptor".into());
    }
    match (&assignment.kind[..], &assignment.value) {
        ("step" | "temperature", DeckAxisValue::Numeric { value }) if value.is_finite() => Ok(()),
        ("data", DeckAxisValue::DataRow { bindings })
            if !bindings.is_empty()
                && bindings.len() == axis.data_bindings.len()
                && bindings
                    .iter()
                    .map(|binding| binding.name.as_str())
                    .eq(axis.data_bindings.iter().map(String::as_str))
                && bindings.iter().all(|binding| {
                    !binding.name.trim().is_empty() && binding.value.is_finite()
                }) =>
        {
            Ok(())
        }
        _ => Err("deck coordinate assignment value is invalid for its axis kind".into()),
    }
}

fn validate_axis_descriptors(axes: &[DeckAxisDescriptor]) -> Result<(), String> {
    let mut identities = HashSet::new();
    identities
        .try_reserve(axes.len())
        .map_err(|_| "could not allocate deck axis validation set".to_owned())?;
    let mut binding_owners = HashSet::new();
    binding_owners
        .try_reserve(axes.len())
        .map_err(|_| "could not allocate deck binding validation set".to_owned())?;
    let mut previous_rank = 0usize;
    for (index, axis) in axes.iter().enumerate() {
        let rank = match axis.kind.as_str() {
            "data" => 1,
            "step" => 2,
            "temperature" => 3,
            _ => return Err(format!("unsupported deck axis kind {:?}", axis.kind)),
        };
        if axis.name.trim().is_empty()
            || axis.value_count == 0
            || (index != 0 && rank < previous_rank)
            || !identities.insert(axis.name.to_ascii_lowercase())
        {
            return Err("deck axis identity, order, or cardinality is invalid".into());
        }
        previous_rank = rank;
        match axis.kind.as_str() {
            "data" => {
                if axis.target.is_some() || axis.data_bindings.is_empty() {
                    return Err("DATA axis target or binding descriptors are invalid".into());
                }
                let mut previous: Option<&str> = None;
                for binding in &axis.data_bindings {
                    if binding.trim().is_empty()
                        || binding.trim() != binding
                        || binding != &binding.to_ascii_lowercase()
                        || previous.is_some_and(|previous| previous >= binding.as_str())
                        || !binding_owners.insert(binding.clone())
                    {
                        return Err(
                            "DATA axis bindings must be unique canonical plan bindings".into()
                        );
                    }
                    previous = Some(binding);
                }
            }
            "step" => {
                let target = axis
                    .target
                    .as_deref()
                    .ok_or_else(|| "STEP axis target descriptor is absent".to_owned())?;
                let (expected_axis_name, binding_name) = step_descriptor_identity(target)?;
                if axis.name != expected_axis_name
                    || !axis.data_bindings.is_empty()
                    || !binding_owners.insert(binding_name)
                {
                    return Err("STEP axis target descriptor is invalid".into());
                }
            }
            "temperature" => {
                if axis.name != "temperature"
                    || axis
                        .target
                        .as_deref()
                        .is_some_and(|target| target != "temperature")
                    || !axis.data_bindings.is_empty()
                    || !binding_owners.insert("temperature".to_owned())
                {
                    return Err("temperature axis target descriptor is invalid".into());
                }
            }
            _ => unreachable!("axis kind was checked above"),
        }
    }
    Ok(())
}

fn step_descriptor_identity(target: &str) -> Result<(String, String), String> {
    if target.trim() != target || target != target.to_ascii_lowercase() {
        return Err("STEP axis target is not a canonical normalized identity".into());
    }
    if let Some(name) = target.strip_prefix("parameter:") {
        if name.is_empty() || name.contains(':') {
            return Err("STEP parameter target is invalid".into());
        }
        return Ok((format!("param:{name}"), name.to_owned()));
    }
    if let Some(name) = target.strip_prefix("device:") {
        if name.is_empty() {
            return Err("STEP device target is invalid".into());
        }
        return Ok((target.to_owned(), target.to_owned()));
    }
    if let Some(model_and_parameter) = target.strip_prefix("model:") {
        if model_and_parameter
            .split_once(':')
            .is_none_or(|(model, parameter)| model.is_empty() || parameter.is_empty())
        {
            return Err("STEP model target is invalid".into());
        }
        return Ok((target.to_owned(), target.to_owned()));
    }
    Err("STEP axis target kind is unsupported".into())
}

fn validate_fft_snapshot(fft: &TransientFftSnapshot) -> Result<(), String> {
    let bins = &fft.bins;
    let bin_count = bins.indices.len();
    if fft.analysis_id != format!("fft-{:03}", fft.ordinal)
        || fft.parent_analysis_id.trim().is_empty()
        || fft.ordinal == 0
        || fft.point_count == 0
        || fft.source_text.trim().is_empty()
        || fft.authored_output.trim().is_empty()
        || fft.output_name.trim().is_empty()
        || !matches!(fft.source_kind.as_str(), "probe" | "expression")
        || !fft.start_time.is_finite()
        || !fft.stop_time.is_finite()
        || !fft.sample_interval.is_finite()
        || fft.sample_interval <= 0.0
        || !fft.alpha.is_finite()
        || !fft.coherent_gain.is_finite()
        || !fft.frequency_resolution.is_finite()
        || fft.frequency_resolution <= 0.0
        || [
            bins.frequencies.len(),
            bins.real.len(),
            bins.imaginary.len(),
            bins.magnitudes.len(),
            bins.phase_degrees.len(),
        ]
        .into_iter()
        .any(|count| count != bin_count)
        || bins
            .frequencies
            .iter()
            .chain(&bins.real)
            .chain(&bins.imaginary)
            .chain(&bins.magnitudes)
            .chain(&bins.phase_degrees)
            .any(|value| !value.is_finite())
        || bins.frequencies.iter().any(|value| *value < 0.0)
        || bins.magnitudes.iter().any(|value| *value < 0.0)
        || bins
            .indices
            .iter()
            .enumerate()
            .any(|(expected, index)| *index != expected || *index >= fft.point_count)
        || bins.frequencies.windows(2).any(|pair| pair[0] >= pair[1])
        || bins
            .frequencies
            .iter()
            .enumerate()
            .any(|(index, frequency)| {
                let expected = index as f64 * fft.frequency_resolution;
                !approximately_equal(*frequency, expected)
            })
        || fft.fundamental_bin >= bin_count
        || fft.minimum_metric_bin >= bin_count
        || fft.maximum_metric_bin >= bin_count
        || fft.minimum_metric_bin > fft.maximum_metric_bin
    {
        return Err("deck FFT result identity, shape, or value is invalid".into());
    }
    if let Some(metrics) = &fft.metrics {
        if [
            metrics.fundamental_magnitude,
            metrics.thd_ratio,
            metrics.thd_db,
            metrics.sndr_db,
            metrics.enob_bits,
            metrics.snr_db,
            metrics.sfdr_db,
        ]
        .into_iter()
        .any(|value| !value.is_finite())
            || metrics
                .sfdr_spur_frequency
                .is_some_and(|value| !value.is_finite() || value < 0.0)
            || metrics
                .sfdr_spur_bin
                .is_some_and(|value| value >= bin_count)
            || metrics.sfdr_spur_bin.is_some() != metrics.sfdr_spur_frequency.is_some()
        {
            return Err("deck FFT scalar metric is invalid".into());
        }
        validate_harmonics(&metrics.largest_harmonics, fft)?;
    }
    Ok(())
}

fn validate_harmonics(
    harmonics: &TransientFftHarmonicsSnapshot,
    fft: &TransientFftSnapshot,
) -> Result<(), String> {
    let count = harmonics.ranks.len();
    if [
        harmonics.bins.len(),
        harmonics.frequencies.len(),
        harmonics.magnitudes.len(),
        harmonics.magnitudes_db.len(),
        harmonics.phase_degrees.len(),
    ]
    .into_iter()
    .any(|length| length != count)
        || harmonics
            .frequencies
            .iter()
            .chain(&harmonics.magnitudes)
            .chain(&harmonics.magnitudes_db)
            .chain(&harmonics.phase_degrees)
            .any(|value| !value.is_finite())
        || harmonics
            .ranks
            .iter()
            .enumerate()
            .any(|(index, rank)| *rank != index + 1)
        || harmonics
            .frequencies
            .iter()
            .any(|frequency| *frequency < 0.0)
        || harmonics
            .magnitudes
            .iter()
            .any(|magnitude| *magnitude < 0.0)
    {
        return Err("deck FFT harmonic shape or value is invalid".into());
    }
    let mut bins = HashSet::new();
    bins.try_reserve(count)
        .map_err(|_| "could not allocate FFT harmonic validation set".to_owned())?;
    for (index, bin) in harmonics.bins.iter().copied().enumerate() {
        if bin >= fft.bins.indices.len()
            || !bins.insert(bin)
            || !approximately_equal(harmonics.frequencies[index], fft.bins.frequencies[bin])
            || !approximately_equal(harmonics.magnitudes[index], fft.bins.magnitudes[bin])
        {
            return Err("deck FFT harmonic bin identity is invalid".into());
        }
    }
    Ok(())
}

fn planned_analysis_tag(kind: &str, ordinal: usize) -> Result<String, String> {
    let prefix = match kind {
        "implicit_op" => "implicit-op",
        "op" => "op",
        "dc" => "dc",
        "ac" => "ac",
        "tran" => "tran",
        "noise" => "noise",
        "sp" => "sp",
        "stb" => "stb",
        "distortion" => "disto",
        "pole_zero" => "pz",
        "sensitivity" => "sens",
        "transfer_function" => "tf",
        "pss" => "pss",
        "pac" => "pac",
        "pnoise" => "pnoise",
        "harmonic_balance" => "hb",
        "envelope" => "env",
        "monte_carlo" => "mc",
        "fourier" => "four",
        "fft" => "fft",
        _ => return Err(format!("unknown planned analysis kind {kind:?}")),
    };
    Ok(format!("{prefix}-{ordinal:03}"))
}

fn planned_analog_kind(kind: &str) -> Result<crate::AnalogAnalysisKind, String> {
    match kind {
        "implicit_op" | "op" => Ok(crate::AnalogAnalysisKind::OperatingPoint),
        "dc" => Ok(crate::AnalogAnalysisKind::DcSweep),
        "ac" => Ok(crate::AnalogAnalysisKind::AcSmallSignal),
        "tran" => Ok(crate::AnalogAnalysisKind::Transient),
        "noise" => Ok(crate::AnalogAnalysisKind::Noise),
        _ => Err(format!(
            "planned analysis kind {kind:?} is not mapped by deck result schema v1"
        )),
    }
}

fn approximately_equal(first: f64, second: f64) -> bool {
    let tolerance = first.abs().max(second.abs()).mul_add(1.0e-12, 1.0e-15);
    (first - second).abs() <= tolerance
}

fn valid_coordinate_id(id: &str) -> bool {
    let Some((semantic, occurrence)) = id.split_once('-') else {
        return false;
    };
    if semantic.len() != 32
        || !semantic
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        || occurrence.len() < 3
        || !occurrence.bytes().all(|byte| byte.is_ascii_digit())
        || (occurrence.len() > 3 && occurrence.starts_with('0'))
    {
        return false;
    }
    occurrence
        .parse::<u64>()
        .is_ok_and(|value| value >= 1 && value <= u64::from(u32::MAX) + 1)
}

fn fft_metrics_metadata(metrics: &TransientFftMetricsSnapshot) -> DeckFftMetricsMetadata {
    DeckFftMetricsMetadata {
        fundamental_magnitude: metrics.fundamental_magnitude,
        thd_ratio: metrics.thd_ratio,
        thd_db: metrics.thd_db,
        sndr_db: metrics.sndr_db,
        enob_bits: metrics.enob_bits,
        snr_db: metrics.snr_db,
        sfdr_db: metrics.sfdr_db,
        sfdr_spur_bin: metrics.sfdr_spur_bin,
        sfdr_spur_frequency: metrics.sfdr_spur_frequency,
        harmonic_count: metrics.largest_harmonics.ranks.len(),
    }
}

fn harmonic_count(fft: &TransientFftSnapshot) -> usize {
    fft.metrics
        .as_ref()
        .map_or(0, |metrics| metrics.largest_harmonics.ranks.len())
}

fn checked_window(
    start: usize,
    count: usize,
    point_count: usize,
    maximum_window_values: usize,
    columns: usize,
) -> Result<usize, String> {
    let end = start
        .checked_add(count)
        .ok_or_else(|| "result window range overflows usize".to_owned())?;
    if count == 0 || start >= point_count || end > point_count {
        return Err(format!(
            "result window [{start}, {end}) is outside 0..{point_count}"
        ));
    }
    let requested = count
        .checked_mul(columns)
        .ok_or_else(|| "result window value count overflows usize".to_owned())?;
    if requested > maximum_window_values {
        return Err(format!(
            "result window requires {requested} values but the transfer limit is {maximum_window_values}"
        ));
    }
    Ok(end)
}

fn try_vec<T>(capacity: usize, object: &'static str) -> Result<Vec<T>, String> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(capacity)
        .map_err(|_| format!("could not allocate {object}"))?;
    Ok(values)
}

fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), String> {
    if abort.is_aborted() {
        Err("deck result processing was aborted".into())
    } else {
        Ok(())
    }
}

fn poll_abort(abort: &dyn AbortSignal, index: usize) -> Result<(), String> {
    if index.is_multiple_of(64) {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::DeckAxisDescriptor;

    const REPEATED: &str = "malformed deck document fixture\n\
        V1 out 0 DC 1 AC 1\n\
        R1 out 0 1k\n\
        .op\n\
        .ac lin 3 1 10\n\
        .end\n";

    #[test]
    fn malformed_public_deck_documents_fail_without_indexing_panics() {
        let mut missing_coordinate =
            crate::run_authored_deck_document_detailed(REPEATED).expect("fixture executes");
        missing_coordinate.coordinates.clear();
        assert!(missing_coordinate.metadata(64).is_err());
        assert!(missing_coordinate.result_metadata(0, 64).is_err());

        let mut wrong_order =
            crate::run_authored_deck_document_detailed(REPEATED).expect("fixture executes");
        wrong_order.results.swap(0, 1);
        assert!(wrong_order.validate().is_err());
        assert!(wrong_order.metadata(64).is_err());

        let mut duplicate_plan =
            crate::run_authored_deck_document_detailed(REPEATED).expect("fixture executes");
        duplicate_plan.planned_analyses[1] = duplicate_plan.planned_analyses[0].clone();
        assert!(duplicate_plan.validate().is_err());

        let mut mismatched_plan =
            crate::run_authored_deck_document_detailed(REPEATED).expect("fixture executes");
        mismatched_plan.planned_analyses[0].kind = "tran".to_owned();
        mismatched_plan.planned_analyses[0].analysis_instance_id = "tran-001".to_owned();
        mismatched_plan.results[0].analysis_instance_id = "tran-001".to_owned();
        mismatched_plan.results[0].document.analysis.id = "tran-001".to_owned();
        mismatched_plan.results[0].output_namespace =
            format!("{}/tran-001", mismatched_plan.coordinates[0].namespace);
        mismatched_plan.results[0].checkpoint_namespace =
            mismatched_plan.results[0].output_namespace.clone();
        assert!(mismatched_plan.validate().is_err());

        let mut malformed_coordinate =
            crate::run_authored_deck_document_detailed(REPEATED).expect("fixture executes");
        malformed_coordinate.coordinates[0].id = "0000000000000000000000000000000A-001".to_owned();
        malformed_coordinate.coordinates[0].namespace =
            format!("run-{}", malformed_coordinate.coordinates[0].id);
        for result in &mut malformed_coordinate.results {
            result.document.coordinate_id = Some(malformed_coordinate.coordinates[0].id.clone());
            result.output_namespace = format!(
                "{}/{}",
                malformed_coordinate.coordinates[0].namespace, result.analysis_instance_id
            );
            result.checkpoint_namespace = result.output_namespace.clone();
        }
        assert!(malformed_coordinate.validate().is_err());

        let mut malformed_axis =
            crate::run_authored_deck_document_detailed(REPEATED).expect("fixture executes");
        malformed_axis.axes.push(DeckAxisDescriptor {
            kind: "step".to_owned(),
            name: "empty".to_owned(),
            target: Some("parameter:empty".to_owned()),
            value_count: 2,
            data_bindings: Vec::new(),
        });
        assert!(malformed_axis.validate().is_err());
    }

    #[test]
    fn malformed_fft_columns_and_identities_fail_before_window_slicing() {
        let deck = "malformed FFT fixture\n\
            V1 out 0 SIN(0 1 1k)\n\
            R1 out 0 1k\n\
            .tran 1u 1m\n\
            .fft v(out) np=32 format=unorm window=hann\n\
            .end\n";
        let mut document =
            crate::run_authored_deck_document_detailed(deck).expect("FFT fixture executes");
        assert_eq!(document.fft_results.len(), 1);
        document.fft_results[0].snapshot.bins.frequencies.pop();
        assert!(document.validate().is_err());
        assert!(document.fft_metadata(0, 64).is_err());
        assert!(document.fft_bin_window(0, 0, 1, 64).is_err());

        let mut invalid_parent =
            crate::run_authored_deck_document_detailed(REPEATED).expect("analog fixture executes");
        let mut fft = crate::run_authored_deck_document_detailed(deck)
            .expect("FFT fixture executes")
            .fft_results
            .remove(0);
        fft.parent_result_index = 0;
        fft.coordinate_index = 0;
        fft.snapshot.parent_analysis_id = "op-001".to_owned();
        fft.output_namespace =
            format!("{}/op-001/fft-001", invalid_parent.coordinates[0].namespace);
        invalid_parent.fft_results.push(fft);
        assert!(invalid_parent.validate().is_err());
    }
}
