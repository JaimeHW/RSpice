//! Stamp-throughput benchmark for the generated Verilog-A built-ins.
//!
//! `rspice-bench` has a gate for the native JIT and one for generated source
//! size, but nothing measured the thing the generated Rust actually exists to
//! do: evaluate a device and stamp its contributions inside the Newton loop.
//! Without that number, a change to the emitted stamp code cannot be justified
//! or regression-gated, so this module supplies it.
//!
//! Devices are driven directly rather than through a netlist. A compact model
//! needs a full model card to be physically meaningful, and no fixture like
//! that exists for all forty-two built-ins; what the backend's cost depends on
//! is the emitted operation count and derivative width, which default
//! parameters exercise faithfully. Every terminal is left ungrounded so the
//! Jacobian block is dense, which is the worst case and, more importantly, the
//! same case for every model and every revision.
//!
//! Results are per-stamp nanoseconds reported as median/p95/min over samples.
//! The median is the comparison statistic; p95 is retained because a stamp that
//! is usually fast and occasionally slow still stalls a Newton iteration.

use std::time::Instant;

use super::{
    BuiltinVerilogAInstance, GeneratedAnalysisKind, GeneratedSimulationParameters,
    GeneratedStaticStampCache, builtins,
};
use crate::Value;
use crate::solver::StaticMatrix;
use std::sync::Arc;

/// Bias applied to every ungrounded node, in volts.
///
/// Small enough to keep exponential junction terms in range for every built-in,
/// large enough to leave the models off their exact-zero fast paths.
const BENCH_NODE_BIAS: Value = 0.35;

/// Seed current for branch unknowns, in amperes.
const BENCH_BRANCH_CURRENT: Value = 1.0e-6;

#[derive(Debug, Clone)]
pub struct GeneratedStampBenchConfig {
    /// Stamp calls per timed sample.
    pub iterations: usize,
    /// Timed samples. Reported statistics are taken across these.
    pub samples: usize,
    /// Restrict the run to these model names; empty means every built-in.
    pub models: Vec<String>,
}

impl Default for GeneratedStampBenchConfig {
    fn default() -> Self {
        Self {
            iterations: 2_000,
            samples: 7,
            models: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedStampBenchResult {
    pub model_name: &'static str,
    pub node_count: usize,
    pub branch_count: usize,
    /// CSC locations the device writes through, after linking.
    pub linked_slot_count: usize,
    pub ns_per_stamp_median: f64,
    pub ns_per_stamp_p95: f64,
    pub ns_per_stamp_min: f64,
}

#[derive(Debug, Clone)]
pub enum GeneratedStampBenchError {
    UnknownModel(String),
    Setup {
        model_name: String,
        detail: String,
    },
    /// The stamp produced nothing, so the timing would measure an empty path.
    NoContribution {
        model_name: String,
    },
    /// The stamp reported a runtime evaluation failure at the bench bias.
    Evaluation {
        model_name: String,
        detail: String,
    },
}

impl std::fmt::Display for GeneratedStampBenchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownModel(name) => {
                write!(f, "'{name}' is not a compiled-in generated built-in")
            }
            Self::Setup { model_name, detail } => {
                write!(f, "{model_name}: benchmark setup failed: {detail}")
            }
            Self::NoContribution { model_name } => write!(
                f,
                "{model_name}: stamp wrote no matrix or right-hand-side entries at the benchmark bias; the timing would be meaningless"
            ),
            Self::Evaluation { model_name, detail } => {
                write!(f, "{model_name}: stamp reported {detail}")
            }
        }
    }
}

impl std::error::Error for GeneratedStampBenchError {}

/// Benchmark every requested built-in, one result (or error) per model.
///
/// A model that fails to set up or evaluate does not abort the run: the point
/// of the sweep is a table across the whole corpus, and one unusable model
/// should not hide the other forty-one.
pub fn run_generated_stamp_benchmarks(
    config: &GeneratedStampBenchConfig,
) -> Vec<Result<GeneratedStampBenchResult, GeneratedStampBenchError>> {
    let available = builtins::builtin_names();
    let selected: Vec<&'static str> = if config.models.is_empty() {
        available.to_vec()
    } else {
        let mut selected = Vec::with_capacity(config.models.len());
        for requested in &config.models {
            match available
                .iter()
                .find(|name| name.eq_ignore_ascii_case(requested))
            {
                Some(name) => selected.push(*name),
                None => {
                    // Preserve the caller's ordering by emitting the failure in
                    // place rather than collecting unknown names separately.
                    selected.push("");
                }
            }
        }
        selected
    };

    selected
        .into_iter()
        .zip(
            config
                .models
                .iter()
                .map(Some)
                .chain(std::iter::repeat(None)),
        )
        .map(|(name, requested)| {
            if name.is_empty() {
                let requested = requested.map(String::as_str).unwrap_or_default();
                return Err(GeneratedStampBenchError::UnknownModel(
                    requested.to_string(),
                ));
            }
            benchmark_model(name, config)
        })
        .collect()
}

fn benchmark_model(
    model_name: &'static str,
    config: &GeneratedStampBenchConfig,
) -> Result<GeneratedStampBenchResult, GeneratedStampBenchError> {
    let setup_error = |detail: String| GeneratedStampBenchError::Setup {
        model_name: model_name.to_string(),
        detail,
    };

    let node_count = builtins::total_node_count(model_name)
        .ok_or_else(|| setup_error("model exposes no node count".to_string()))?;
    let branch_count = builtins::branch_count(model_name).unwrap_or(0);

    // Matrix rows 0..node_count hold the ungrounded nodes; branch unknowns
    // follow, matching `GeneratedStaticStampCache`'s index convention.
    let nodes: Vec<usize> = (1..=node_count).collect();
    let branches: Vec<usize> = (1..=branch_count).collect();
    let size = node_count + branch_count;
    if size == 0 {
        return Err(setup_error("model has no unknowns".to_string()));
    }

    let mut triplets = Vec::with_capacity(size * size);
    for row in 0..size {
        for col in 0..size {
            triplets.push((row, col, 0.0));
        }
    }
    let mut matrix = StaticMatrix::from_triplets(size, size, &triplets)
        .map_err(|error| setup_error(format!("matrix assembly failed: {error}")))?;

    let kind = builtins::instantiate(model_name, &nodes, &branches, &[])
        .map_err(|error| setup_error(format!("instantiation failed: {error}")))?
        .ok_or_else(|| setup_error("model is not compiled into this binary".to_string()))?;

    let mut instance = BuiltinVerilogAInstance {
        model_name,
        instance_name: format!("xbench_{model_name}"),
        nodes,
        branches,
        temperature: crate::constants::TEMP_REFERENCE,
        analysis_initial_step: false,
        analysis_final_step: false,
        static_stamp_cache: Arc::new(GeneratedStaticStampCache::default()),
        kind,
    };
    instance.link_static_stamps(&matrix, node_count);
    let linked_slot_count = instance.static_stamp_cache.linked_slot_count();

    let mut voltages = vec![BENCH_NODE_BIAS; size];
    for value in voltages.iter_mut().skip(node_count) {
        *value = BENCH_BRANCH_CURRENT;
    }
    let mut rhs = vec![0.0 as Value; size];
    let simparams = GeneratedSimulationParameters::new();

    // One untimed stamp: it populates the model's instance- and
    // temperature-static caches, which every later call reuses. Timing it would
    // charge one-time setup to the steady-state cost.
    instance
        .stamp(
            &mut matrix,
            &mut rhs,
            &voltages,
            node_count,
            GeneratedAnalysisKind::Dc,
            simparams,
        )
        .map_err(|error| GeneratedStampBenchError::Evaluation {
            model_name: model_name.to_string(),
            detail: error.to_string(),
        })?;

    if !stamp_produced_contribution(&mut matrix, &rhs) {
        return Err(GeneratedStampBenchError::NoContribution {
            model_name: model_name.to_string(),
        });
    }

    let samples = config.samples.max(1);
    let iterations = config.iterations.max(1);
    let mut per_stamp_ns = Vec::with_capacity(samples);
    for _ in 0..samples {
        let started = Instant::now();
        for iteration in 0..iterations {
            // Walk the bias by a sub-millivolt step per call. Newton never
            // stamps the same point twice, and holding the vector fixed would
            // let any future bias-keyed caching report throughput this backend
            // does not actually have.
            voltages[0] = BENCH_NODE_BIAS + (iteration % 64) as Value * 1.0e-5;
            instance
                .stamp(
                    &mut matrix,
                    &mut rhs,
                    &voltages,
                    node_count,
                    GeneratedAnalysisKind::Dc,
                    simparams,
                )
                .map_err(|error| GeneratedStampBenchError::Evaluation {
                    model_name: model_name.to_string(),
                    detail: error.to_string(),
                })?;
        }
        let elapsed = started.elapsed();
        per_stamp_ns.push(elapsed.as_secs_f64() * 1.0e9 / iterations as f64);
    }

    per_stamp_ns.sort_by(f64::total_cmp);
    Ok(GeneratedStampBenchResult {
        model_name,
        node_count,
        branch_count,
        linked_slot_count,
        ns_per_stamp_median: percentile(&per_stamp_ns, 0.50),
        ns_per_stamp_p95: percentile(&per_stamp_ns, 0.95),
        ns_per_stamp_min: per_stamp_ns[0],
    })
}

/// Whether the device wrote anything the solver would see.
///
/// Guards against timing a model that silently evaluates to nothing at the
/// bench bias, which would otherwise report an impressively small number.
fn stamp_produced_contribution(matrix: &mut StaticMatrix, rhs: &[Value]) -> bool {
    matrix.values_mut().iter().any(|value| *value != 0.0) || rhs.iter().any(|value| *value != 0.0)
}

/// Nearest-rank percentile over an ascending slice.
fn percentile(ascending: &[f64], fraction: f64) -> f64 {
    debug_assert!(!ascending.is_empty(), "percentile of an empty sample set");
    let rank = (fraction * ascending.len() as f64).ceil() as usize;
    ascending[rank.saturating_sub(1).min(ascending.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentile_picks_nearest_rank() {
        let samples = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(percentile(&samples, 0.50), 2.0);
        assert_eq!(percentile(&samples, 0.95), 4.0);
        assert_eq!(percentile(&samples, 0.0), 1.0);
    }

    #[test]
    fn percentile_handles_a_single_sample() {
        assert_eq!(percentile(&[7.0], 0.50), 7.0);
        assert_eq!(percentile(&[7.0], 0.95), 7.0);
    }

    #[test]
    fn unknown_model_names_are_reported_not_skipped() {
        let config = GeneratedStampBenchConfig {
            iterations: 1,
            samples: 1,
            models: vec!["definitely_not_a_builtin".to_string()],
        };
        let results = run_generated_stamp_benchmarks(&config);
        assert_eq!(results.len(), 1);
        assert!(matches!(
            results[0],
            Err(GeneratedStampBenchError::UnknownModel(_))
        ));
    }
}
