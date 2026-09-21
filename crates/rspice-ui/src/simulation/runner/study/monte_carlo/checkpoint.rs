//! Studio trial journals retain measurement verdicts beside exact core samples.
//!
//! Partial/sparse journals are deliberately separate from a completed result's
//! contiguous `FamilyMemberMeasurements` roster. Numbers are encoded only once,
//! in the core binary; every accepted row also has a complete verdict record.

use crate::simulation::runner::SimulationError;
use crate::state::FamilyMeasurementEvidence;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::MonteCarloCheckpoint;
use rspice_core::{ResourceKind, ResourceLimits};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, HashSet};

const MAGIC: &[u8] = b"RSPICE-STUDIO-MC\0";
const VERSION: u32 = 1;
pub(super) type Observations = BTreeMap<usize, Vec<FamilyMeasurementEvidence>>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StudyMonteCarloCheckpoint {
    pub(super) numerical: MonteCarloCheckpoint,
    measurements: Vec<String>,
    pub(super) observations: Observations,
}

fn invalid(message: &str) -> SimulationError {
    SimulationError::InvalidConfig(format!("Monte Carlo checkpoint: {message}"))
}
fn bound(kind: ResourceKind, requested: usize, limit: usize) -> Result<(), SimulationError> {
    if requested > limit {
        Err(SimulationError::ResourceLimit {
            resource: kind.as_str().into(),
            requested,
            limit,
        })
    } else {
        Ok(())
    }
}
fn core_error(error: rspice_core::SimulationError) -> SimulationError {
    crate::simulation::engine_bridge::EngineBridge::new().translate_error(error)
}
fn poll(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    crate::simulation::runner::spec::ensure_not_aborted(abort)
}

pub(super) fn failed_observations(names: &[String], error: &str) -> Vec<FamilyMeasurementEvidence> {
    names
        .iter()
        .map(|name| FamilyMeasurementEvidence {
            name: name.clone(),
            value: None,
            passed: false,
            error: Some(error.into()),
        })
        .collect()
}

impl StudyMonteCarloCheckpoint {
    pub(crate) fn population_identity(&self) -> [u8; 32] {
        self.numerical.population_identity()
    }
    pub(crate) fn completed_trials(&self) -> usize {
        self.numerical.completed_trials()
    }
    pub(crate) fn completed_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.numerical.completed_indices()
    }

    /// Capture only committed numerical rows. Other workers may already have
    /// evaluated a row without committing it; that row is not a checkpoint yet.
    pub(super) fn capture(
        numerical: &MonteCarloCheckpoint,
        measurements: &[String],
        evaluated: &Observations,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        poll(abort)?;
        bound(
            ResourceKind::ResultValues,
            numerical
                .completed_trials()
                .max(1)
                .saturating_mul(measurements.len().saturating_add(2))
                .saturating_mul(3),
            limits.max_result_values,
        )?;
        let mut observations = BTreeMap::new();
        for index in numerical.completed_indices() {
            poll(abort)?;
            let row = match numerical.trial(index).expect("committed index") {
                Some(_) => evaluated
                    .get(&index)
                    .ok_or_else(|| invalid("observed trial is missing its verdicts"))?
                    .clone(),
                None => evaluated
                    .get(&index)
                    .filter(|row| row.iter().all(|value| value.value.is_none()))
                    .cloned()
                    .unwrap_or_else(|| {
                        failed_observations(measurements, "Monte Carlo trial did not converge")
                    }),
            };
            observations.insert(index, row);
        }
        let captured = Self {
            numerical: numerical.clone(),
            measurements: measurements.to_vec(),
            observations,
        };
        captured.validate(limits, abort)?;
        Ok(captured)
    }

    pub(super) fn validate(
        &self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        poll(abort)?;
        bound(
            ResourceKind::BatchRuns,
            self.completed_trials(),
            limits.max_batch_runs,
        )?;
        bound(
            ResourceKind::ResultValues,
            self.completed_trials()
                .max(1)
                .saturating_mul(self.measurements.len().saturating_add(2))
                .saturating_mul(3),
            limits.max_result_values,
        )?;
        if self.measurements.len() != self.numerical.measurement_count()
            || self.observations.len() != self.completed_trials()
        {
            return Err(invalid("incomplete measurement or trial roster"));
        }
        let mut names = HashSet::new();
        // Includes conservative fixed overhead for both nested headers/checksums.
        let mut bytes = 256usize;
        for name in &self.measurements {
            poll(abort)?;
            if name.trim().is_empty()
                || name.chars().any(char::is_control)
                || !names.insert(name.to_ascii_lowercase())
            {
                return Err(invalid("invalid or repeated measurement name"));
            }
            bytes = bytes.saturating_add(8).saturating_add(name.len());
        }
        for index in self.numerical.completed_indices() {
            poll(abort)?;
            let row = self
                .observations
                .get(&index)
                .ok_or_else(|| invalid("trial is missing its verdicts"))?;
            let numerical = self.numerical.trial(index).expect("committed index");
            if row.len() != self.measurements.len() {
                return Err(invalid("incomplete verdict row"));
            }
            bytes = bytes.saturating_add(17);
            for (column, observation) in row.iter().enumerate() {
                poll(abort)?;
                let expected = numerical.map(|values| values[column].to_bits());
                if observation.name != self.measurements[column]
                    || observation.value.map(f64::to_bits) != expected
                    || observation.value.is_some_and(|value| !value.is_finite())
                    || if observation.passed {
                        observation.value.is_none() || observation.error.is_some()
                    } else {
                        observation
                            .error
                            .as_ref()
                            .is_none_or(|error| error.trim().is_empty())
                    }
                {
                    return Err(invalid(
                        "measurement value or verdict disagrees with its committed trial",
                    ));
                }
                bytes = bytes
                    .saturating_add(1)
                    .saturating_add(usize::from(numerical.is_some()) * 8)
                    .saturating_add(
                        observation
                            .error
                            .as_ref()
                            .map_or(0, |error| 8usize.saturating_add(error.len())),
                    );
            }
        }
        bound(
            ResourceKind::ExternalDataBytes,
            bytes,
            limits.max_external_data_bytes,
        )
    }

    /// Pool identical populations atomically. Overlaps must agree in both exact
    /// numbers and pass/fail evidence, and contribute only one trial.
    pub(crate) fn merge_with_limits(
        &mut self,
        other: &Self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        self.validate(limits, abort)?;
        other.validate(limits, abort)?;
        if self.measurements != other.measurements
            || self.population_identity() != other.population_identity()
        {
            return Err(invalid("cannot pool different population contracts"));
        }
        for (index, row) in &other.observations {
            poll(abort)?;
            if self.observations.get(index).is_some_and(|existing| {
                existing.iter().zip(row).any(|(a, b)| {
                    a.name != b.name
                        || a.value.map(f64::to_bits) != b.value.map(f64::to_bits)
                        || a.passed != b.passed
                        || a.error != b.error
                })
            }) {
                return Err(invalid(
                    "overlapping trials have conflicting measurement verdicts",
                ));
            }
        }
        let count = self.completed_trials().saturating_add(
            other
                .observations
                .keys()
                .filter(|index| !self.observations.contains_key(index))
                .count(),
        );
        bound(
            ResourceKind::ResultValues,
            count
                .saturating_mul(self.measurements.len().saturating_add(2))
                .saturating_mul(6),
            limits.max_result_values,
        )?;
        let mut merged = self.clone();
        merged
            .numerical
            .merge_with_limits(&other.numerical, limits, abort)
            .map_err(core_error)?;
        for (index, row) in &other.observations {
            poll(abort)?;
            merged
                .observations
                .entry(*index)
                .or_insert_with(|| row.clone());
        }
        merged.validate(limits, abort)?;
        *self = merged;
        Ok(())
    }

    /// Portable envelope for files, project blobs and transferable worker bytes.
    /// Floating-point values stay in the core's exact binary representation.
    pub(crate) fn to_bytes_with_limits(
        &self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<u8>, SimulationError> {
        self.validate(limits, abort)?;
        let numerical = self
            .numerical
            .to_bytes_with_limits(limits, abort)
            .map_err(core_error)?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        write_size(&mut bytes, numerical.len());
        bytes.extend_from_slice(&numerical);
        write_size(&mut bytes, self.measurements.len());
        for name in &self.measurements {
            write_text(&mut bytes, name);
        }
        write_size(&mut bytes, self.observations.len());
        for (index, row) in &self.observations {
            poll(abort)?;
            write_size(&mut bytes, *index);
            for observation in row {
                poll(abort)?;
                bytes.push(u8::from(observation.passed));
                if let Some(error) = &observation.error {
                    write_text(&mut bytes, error);
                }
            }
        }
        let digest = Sha256::digest(&bytes);
        bytes.extend_from_slice(&digest);
        bound(
            ResourceKind::ExternalDataBytes,
            bytes.len(),
            limits.max_external_data_bytes,
        )?;
        Ok(bytes)
    }

    pub(crate) fn from_bytes_with_limits(
        bytes: &[u8],
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        poll(abort)?;
        bound(
            ResourceKind::ExternalDataBytes,
            bytes.len(),
            limits.max_external_data_bytes,
        )?;
        if bytes.len() < MAGIC.len() + 4 + 8 + 32 {
            return Err(invalid("truncated envelope"));
        }
        let (body, digest) = bytes.split_at(bytes.len() - 32);
        if Sha256::digest(body).as_slice() != digest {
            return Err(invalid("envelope checksum mismatch"));
        }
        let mut reader = Reader { remaining: body };
        if reader.take(MAGIC.len())? != MAGIC || reader.take(4)? != VERSION.to_le_bytes() {
            return Err(invalid("unsupported envelope version"));
        }
        let size = reader.size()?;
        let numerical =
            MonteCarloCheckpoint::from_bytes_with_limits(reader.take(size)?, limits, abort)
                .map_err(core_error)?;
        let columns = reader.size()?;
        if columns != numerical.measurement_count() || columns > reader.remaining.len() / 8 {
            return Err(invalid("invalid column count"));
        }
        bound(
            ResourceKind::ResultValues,
            numerical
                .completed_trials()
                .max(1)
                .saturating_mul(columns.saturating_add(2))
                .saturating_mul(3),
            limits.max_result_values,
        )?;
        let mut measurements = Vec::with_capacity(columns);
        for _ in 0..columns {
            poll(abort)?;
            measurements.push(reader.text()?);
        }
        let count = reader.size()?;
        if count != numerical.completed_trials()
            || count > reader.remaining.len() / columns.saturating_add(8)
        {
            return Err(invalid("invalid verdict count"));
        }
        let mut observations = BTreeMap::new();
        for expected in numerical.completed_indices() {
            poll(abort)?;
            let index = reader.size()?;
            if index != expected {
                return Err(invalid("verdict indices disagree with completed trials"));
            }
            let values = numerical.trial(index).expect("committed index");
            let mut row = Vec::with_capacity(columns);
            for (column, name) in measurements.iter().enumerate() {
                poll(abort)?;
                let (passed, error) = match reader.take(1)?[0] {
                    0 => (false, Some(reader.text()?)),
                    1 => (true, None),
                    _ => return Err(invalid("invalid verdict tag")),
                };
                row.push(FamilyMeasurementEvidence {
                    name: name.clone(),
                    value: values.map(|values| values[column]),
                    passed,
                    error,
                });
            }
            observations.insert(index, row);
        }
        if !reader.remaining.is_empty() {
            return Err(invalid("trailing envelope data"));
        }
        let checkpoint = Self {
            numerical,
            measurements,
            observations,
        };
        checkpoint.validate(limits, abort)?;
        Ok(checkpoint)
    }
}

fn write_size(bytes: &mut Vec<u8>, value: usize) {
    bytes.extend_from_slice(&(value as u64).to_le_bytes());
}
fn write_text(bytes: &mut Vec<u8>, value: &str) {
    write_size(bytes, value.len());
    bytes.extend_from_slice(value.as_bytes());
}
struct Reader<'a> {
    remaining: &'a [u8],
}
impl<'a> Reader<'a> {
    fn take(&mut self, size: usize) -> Result<&'a [u8], SimulationError> {
        if size > self.remaining.len() {
            return Err(invalid("truncated envelope data"));
        }
        let (value, tail) = self.remaining.split_at(size);
        self.remaining = tail;
        Ok(value)
    }
    fn size(&mut self) -> Result<usize, SimulationError> {
        usize::try_from(u64::from_le_bytes(
            self.take(8)?.try_into().expect("fixed length"),
        ))
        .map_err(|_| invalid("envelope size exceeds this platform"))
    }
    fn text(&mut self) -> Result<String, SimulationError> {
        let size = self.size()?;
        std::str::from_utf8(self.take(size)?)
            .map(str::to_owned)
            .map_err(|_| invalid("invalid UTF-8 metadata"))
    }
}

#[cfg(test)]
mod tests;
