//! Exact Monte Carlo trial journals, bounded persistence, and population pooling.
//!
//! A journal records completed observations (or statistical failures), never a
//! partially solved circuit. Restart reruns only missing trials. Its population
//! identity binds the frozen source, elaborated circuit, engine configuration,
//! sampler, environment, and the caller's complete analysis/measurement contract.

use super::{Engine, MonteCarloStudyConfig, MonteCarloTrialJournal, MonteCarloVariationSource};
use crate::abort_signal::AbortSignal;
use crate::analysis::monte_carlo::{MeanConfidenceMethod, MonteCarloResult};
use crate::engine::SimulationError;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use crate::{Netlist, Value};
use std::collections::BTreeMap;
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};

const MAGIC: &[u8] = b"RSPICE-MC-CP\0";
const VERSION: u32 = 1;

fn invalid(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("Monte Carlo checkpoint: {}", message.into()))
}
fn check_abort(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::from_abort(abort))
    } else {
        Ok(())
    }
}

/// Completed trial evidence from one immutable statistical population.
///
/// Indices may be sparse after parallel cancellation or when pooling disjoint
/// batches. A stored failed trial is complete and is not retried on resume.
/// Fields are private; portable decoding checks both content integrity and shape.
#[derive(Debug, Clone, PartialEq)]
pub struct MonteCarloCheckpoint {
    population: [u8; 32],
    columns: usize,
    rows: BTreeMap<usize, Option<Vec<Value>>>,
}

impl MonteCarloCheckpoint {
    pub fn population_identity(&self) -> [u8; 32] {
        self.population
    }
    /// Number of scalar measurements in every observed trial.
    pub fn measurement_count(&self) -> usize {
        self.columns
    }
    pub fn completed_trials(&self) -> usize {
        self.rows.len()
    }
    pub fn completed_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.rows.keys().copied()
    }
    /// None is unfinished; Some(None) is a completed statistical failure.
    pub fn trial(&self, index: usize) -> Option<Option<&[Value]>> {
        self.rows.get(&index).map(|row| row.as_deref())
    }

    fn validate(
        &self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        check_abort(abort)?;
        if self.columns == 0 {
            return Err(invalid("no measurement columns"));
        }
        ResourceLimitError::ensure(
            ResourceKind::BatchRuns,
            self.rows.len(),
            limits.max_batch_runs,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            self.rows
                .len()
                .max(1)
                .saturating_mul(self.columns.saturating_add(2)),
            limits.max_result_values,
        )?;
        for row in self.rows.values() {
            check_abort(abort)?;
            if let Some(values) = row {
                if values.len() != self.columns {
                    return Err(invalid("incomplete measurement row"));
                }
                for value in values {
                    check_abort(abort)?;
                    if !value.is_finite() {
                        return Err(invalid("nonfinite measurement"));
                    }
                }
            }
        }
        Ok(())
    }

    /// Combine trial populations without double-counting overlap. Conflicting
    /// outcomes or different population contracts are refused atomically.
    pub fn merge_with_limits(
        &mut self,
        other: &Self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        self.validate(limits, abort)?;
        other.validate(limits, abort)?;
        if self.population != other.population || self.columns != other.columns {
            return Err(invalid("cannot pool different population contracts"));
        }
        let mut count = self.rows.len();
        for (index, row) in &other.rows {
            check_abort(abort)?;
            if let Some(existing) = self.rows.get(index) {
                let same = match (existing, row) {
                    (None, None) => true,
                    (Some(left), Some(right)) => left
                        .iter()
                        .zip(right)
                        .all(|(a, b)| a.to_bits() == b.to_bits()),
                    _ => false,
                };
                if !same {
                    return Err(invalid(format!("conflicting outcomes for trial {index}")));
                }
            } else {
                count = count.saturating_add(1);
            }
        }
        ResourceLimitError::ensure(ResourceKind::BatchRuns, count, limits.max_batch_runs)?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            count
                .saturating_mul(self.columns.saturating_add(2))
                .saturating_mul(2),
            limits.max_result_values,
        )?;
        let mut merged = BTreeMap::new();
        for (index, row) in self.rows.iter().chain(&other.rows) {
            check_abort(abort)?;
            if !merged.contains_key(index) {
                merged.insert(*index, row.clone());
            }
        }
        check_abort(abort)?;
        self.rows = merged;
        Ok(())
    }

    /// Portable binary representation. Values are exact IEEE-754 bits; the
    /// checksum covers the version, population, identities, failures and values.
    pub fn to_bytes_with_limits(
        &self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<u8>, SimulationError> {
        self.validate(limits, abort)?;
        let successes = self.rows.values().filter(|row| row.is_some()).count();
        let bytes = MAGIC
            .len()
            .saturating_add(4 + 32 + 8 + 8 + 32)
            .saturating_add(self.rows.len().saturating_mul(9))
            .saturating_add(successes.saturating_mul(self.columns).saturating_mul(8));
        ResourceLimitError::ensure(
            ResourceKind::ExternalDataBytes,
            bytes,
            limits.max_external_data_bytes,
        )?;
        let mut output = Vec::with_capacity(bytes);
        output.extend_from_slice(MAGIC);
        output.extend_from_slice(&VERSION.to_le_bytes());
        output.extend_from_slice(&self.population);
        output.extend_from_slice(&(self.columns as u64).to_le_bytes());
        output.extend_from_slice(&(self.rows.len() as u64).to_le_bytes());
        for (index, row) in &self.rows {
            check_abort(abort)?;
            output.extend_from_slice(&(*index as u64).to_le_bytes());
            output.push(u8::from(row.is_some()));
            if let Some(values) = row {
                for value in values {
                    check_abort(abort)?;
                    output.extend_from_slice(&value.to_bits().to_le_bytes());
                }
            }
        }
        let digest = blake3::hash(&output);
        output.extend_from_slice(digest.as_bytes());
        Ok(output)
    }

    pub fn from_bytes_with_limits(
        bytes: &[u8],
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        check_abort(abort)?;
        ResourceLimitError::ensure(
            ResourceKind::ExternalDataBytes,
            bytes.len(),
            limits.max_external_data_bytes,
        )?;
        if bytes.len() < MAGIC.len() + 4 + 32 + 8 + 8 + 32 {
            return Err(invalid("truncated header"));
        }
        let (body, digest) = bytes.split_at(bytes.len() - 32);
        if blake3::hash(body).as_bytes() != digest {
            return Err(invalid("content digest mismatch"));
        }
        let mut reader = CheckpointReader {
            bytes: body,
            offset: 0,
        };
        if reader.take(MAGIC.len())? != MAGIC || reader.take(4)? != VERSION.to_le_bytes() {
            return Err(invalid("unsupported format/version"));
        }
        let population = reader.take(32)?.try_into().expect("fixed length");
        let columns = reader.usize()?;
        let count = reader.usize()?;
        ResourceLimitError::ensure(ResourceKind::BatchRuns, count, limits.max_batch_runs)?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            count.max(1).saturating_mul(columns.saturating_add(2)),
            limits.max_result_values,
        )?;
        if columns == 0 || count > body.len().saturating_sub(reader.offset) / 9 {
            return Err(invalid("invalid shape"));
        }
        let mut rows = BTreeMap::new();
        let mut previous = None;
        for _ in 0..count {
            check_abort(abort)?;
            let index = reader.usize()?;
            if previous.is_some_and(|last| index <= last) {
                return Err(invalid("trial indices must be unique and ordered"));
            }
            previous = Some(index);
            let row = match reader.take(1)?[0] {
                0 => None,
                1 => {
                    if columns > body.len().saturating_sub(reader.offset) / 8 {
                        return Err(invalid("truncated measurements"));
                    }
                    let mut values = Vec::with_capacity(columns);
                    for _ in 0..columns {
                        check_abort(abort)?;
                        let value = Value::from_bits(u64::from_le_bytes(
                            reader.take(8)?.try_into().expect("fixed length"),
                        ));
                        if !value.is_finite() {
                            return Err(invalid("nonfinite measurement"));
                        }
                        values.push(value);
                    }
                    Some(values)
                }
                _ => return Err(invalid("unknown trial outcome")),
            };
            rows.insert(index, row);
        }
        if reader.offset != body.len() {
            return Err(invalid("unexpected trailing data"));
        }
        Ok(Self {
            population,
            columns,
            rows,
        })
    }
}

struct CheckpointReader<'a> {
    bytes: &'a [u8],
    offset: usize,
}
impl<'a> CheckpointReader<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8], SimulationError> {
        let end = self
            .offset
            .checked_add(count)
            .ok_or_else(|| invalid("size overflow"))?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| invalid("truncated data"))?;
        self.offset = end;
        Ok(value)
    }
    fn usize(&mut self) -> Result<usize, SimulationError> {
        usize::try_from(u64::from_le_bytes(
            self.take(8)?.try_into().expect("fixed length"),
        ))
        .map_err(|_| invalid("index exceeds this platform's supported range"))
    }
}

impl Engine {
    /// Start a journal against one frozen deck. `evaluation_identity` must hash
    /// the caller's complete configured analysis/prerequisite/measurement contract;
    /// the engine cannot inspect the arbitrary evaluator closure. Trial range,
    /// histogram and mean-confidence reporting may change without changing draws.
    pub fn new_monte_carlo_checkpoint(
        &self,
        netlist: &Netlist,
        study: &MonteCarloStudyConfig,
        evaluation_identity: [u8; 32],
        abort: &dyn AbortSignal,
    ) -> Result<MonteCarloCheckpoint, SimulationError> {
        let population =
            self.monte_carlo_population_identity(netlist, study, evaluation_identity, abort)?;
        let checkpoint = MonteCarloCheckpoint {
            population,
            columns: study.measurements.len(),
            rows: BTreeMap::new(),
        };
        checkpoint.validate(self.config.resource_limits, abort)?;
        Ok(checkpoint)
    }

    fn monte_carlo_population_identity(
        &self,
        netlist: &Netlist,
        study: &MonteCarloStudyConfig,
        evaluation_identity: [u8; 32],
        abort: &dyn AbortSignal,
    ) -> Result<[u8; 32], SimulationError> {
        check_abort(abort)?;
        self.ensure_valid_configuration()?;
        let mut hash = blake3::Hasher::new();
        hash.update(b"rspice-monte-carlo-population-v1\0");
        let mut field = |value: &[u8]| {
            hash.update(&(value.len() as u64).to_le_bytes());
            hash.update(value);
        };
        field(&evaluation_identity);
        field(
            crate::engine::transient::netlist_checkpoint_identity(netlist)
                .ok_or_else(|| invalid("circuit identity unavailable"))?
                .as_bytes(),
        );
        // Parsed nominal values alone cannot identify statistical expressions.
        field(netlist.source_text.as_deref().unwrap_or("").as_bytes());
        field(format!("{:?}", netlist.spectre_statistics).as_bytes());
        field(format!("{:?}", netlist.spectre_statistical_coordinate).as_bytes());
        field(format!("{:?}", netlist.ast_overlay).as_bytes());
        let mut config = self.config.clone();
        config.resource_limits = ResourceLimits::default();
        field(format!("{config:?}").as_bytes());
        field(crate::engine::transient::simulation_checkpoint_identity(&config).as_bytes());
        field(&study.seed.to_le_bytes());
        field(format!("{:?}", study.variation_source).as_bytes());
        if study.variation_source == MonteCarloVariationSource::ParameterTolerance
            && netlist.spectre_statistics.variations.is_empty()
        {
            field(format!("{:?}", study.distribution).as_bytes());
        }
        let mut filter = study
            .parameter_filter
            .iter()
            .map(|name| name.trim().to_ascii_uppercase())
            .collect::<Vec<_>>();
        filter.sort();
        filter.dedup();
        field(format!("{filter:?}").as_bytes());
        field(format!("{:?}", study.environment).as_bytes());
        field(format!("{:?}", study.measurements).as_bytes());
        check_abort(abort)?;
        Ok(*hash.finalize().as_bytes())
    }

    /// Resume only unfinished trials and rebuild statistics over the requested
    /// range. Accepted rows remain in `checkpoint` even on cancellation, timeout,
    /// resource failure or publication failure. The publication hook runs serially
    /// after each newly completed trial and can persist at a caller-chosen cadence.
    /// Statistical failures are retained; interrupted/fatal trials remain missing.
    pub fn run_monte_carlo_measurements_checkpointed_with_abort<F, P>(
        &self,
        netlist: &Netlist,
        study: &MonteCarloStudyConfig,
        evaluation_identity: [u8; 32],
        checkpoint: &mut MonteCarloCheckpoint,
        abort: &dyn AbortSignal,
        evaluate: F,
        publish: P,
    ) -> Result<MonteCarloResult, SimulationError>
    where
        F: Fn(&Engine, &Netlist, usize, &dyn AbortSignal) -> Result<Vec<Value>, SimulationError>
            + Sync,
        P: Fn(&MonteCarloCheckpoint) -> Result<(), SimulationError> + Sync,
    {
        checkpoint.validate(self.config.resource_limits, abort)?;
        let identity =
            self.monte_carlo_population_identity(netlist, study, evaluation_identity, abort)?;
        if checkpoint.population != identity || checkpoint.columns != study.measurements.len() {
            return Err(invalid(
                "population contract does not match the requested study",
            ));
        }
        let end = study
            .first_trial
            .checked_add(study.num_runs)
            .ok_or_else(|| invalid("trial range overflow"))?;
        let overlap = checkpoint.rows.range(study.first_trial..end).count();
        let count = checkpoint
            .rows
            .len()
            .saturating_add(study.num_runs)
            .saturating_sub(overlap);
        self.ensure_batch_runs(count)?;
        let retained = count
            .saturating_mul(checkpoint.columns.saturating_add(2))
            .saturating_mul(2)
            .saturating_add(
                study
                    .num_runs
                    .saturating_mul(checkpoint.columns.saturating_mul(2).saturating_add(1)),
            )
            .saturating_add(
                study
                    .histogram_bins
                    .saturating_mul(2)
                    .saturating_add(8)
                    .saturating_mul(checkpoint.columns),
            );
        let scratch = match study.confidence_method {
            MeanConfidenceMethod::StudentT => 0,
            MeanConfidenceMethod::PercentileBootstrap { resamples, .. } => resamples,
        };
        self.ensure_result_values(retained.saturating_add(scratch))?;
        let previous = checkpoint.rows.clone();
        let current = Mutex::new(checkpoint);
        let publication_failed = AtomicBool::new(false);
        let restore = |index| previous.get(&index).cloned();
        let commit = |index, outcome: &Option<Vec<Value>>| {
            let mut current = current.lock().expect("MC checkpoint lock");
            current.rows.insert(index, outcome.clone());
            if publication_failed.load(Ordering::Acquire) {
                return Err(invalid("checkpoint publication already failed"));
            }
            if let Err(error) = publish(&current) {
                publication_failed.store(true, Ordering::Release);
                return Err(error);
            }
            Ok(())
        };
        self.run_monte_carlo_measurements_journaled_with_abort(
            netlist,
            study,
            abort,
            Some(MonteCarloTrialJournal {
                restore: &restore,
                commit: &commit,
            }),
            evaluate,
        )
    }
}

#[cfg(test)]
mod tests;
