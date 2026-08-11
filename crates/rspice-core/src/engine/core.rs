//! Core engine type and shared orchestration helpers.

use super::{
    SimulationConfig, SimulationConfigError, SimulationConfigOverrides, SimulationError,
    resolve_simulation_config,
};

#[derive(Clone, Copy)]
pub(in crate::engine) enum DcOpStartup<'a> {
    Automatic,
    ForceInitialConditions,
    PreviousSolution(&'a [Value]),
    Zero,
}
use crate::diagnostics::ConvergenceQuality;
use crate::netlist::{ElementKind, SubcircuitDef};
use crate::resource::{ResourceKind, ResourceLimitError};
use crate::{CircuitData, Netlist, Value};
use std::collections::{HashMap, HashSet};

/// Main simulation engine
pub struct Engine {
    pub(crate) config: SimulationConfig,
    config_error: Option<SimulationConfigError>,
    /// True when the configuration already includes the target netlist's
    /// options and any higher-precedence runtime overrides.
    config_is_resolved: bool,
    #[cfg(feature = "parallel")]
    parallel_pool: std::sync::OnceLock<Result<rayon::ThreadPool, String>>,
    /// Reusable narrow pool for the short, memory-bound classic-MOS kernel.
    /// Frequency/corner parallelism keeps the general host-wide pool above.
    #[cfg(feature = "parallel")]
    classic_mos_parallel_pool: std::sync::OnceLock<Result<rayon::ThreadPool, String>>,
    /// Convergence-quality metrics for the most recent analysis.
    ///
    /// Behind a lock rather than threaded through the solve chain because the
    /// analysis entry points take `&self` and `&Engine` is shared across rayon
    /// workers during parallel sweeps. Every recording site is a *fallback* —
    /// gmin stepping, source stepping, a force-accepted point, a rejected
    /// step — or once-per-solve. None is in the Newton inner loop, so the lock
    /// is uncontended in the hot path.
    convergence: std::sync::Arc<std::sync::Mutex<ConvergenceQuality>>,
}

impl Engine {
    fn validated_startup_netlist<'a>(
        &self,
        netlist: &'a Netlist,
    ) -> Option<std::borrow::Cow<'a, Netlist>> {
        if netlist.startup_directives.is_empty() {
            return Some(std::borrow::Cow::Borrowed(netlist));
        }
        let mut validated = netlist.clone();
        match crate::netlist::validate_startup_directives(&mut validated) {
            Ok(()) => Some(std::borrow::Cow::Owned(validated)),
            Err(error) => {
                // Circuit construction owns the typed fatal error path. Hint
                // collection remains infallible for established internal API
                // callers, but never applies unvalidated sidecar entries.
                log::debug!("startup validation failed before hint collection: {error}");
                None
            }
        }
    }

    /// Create a new engine with the given configuration
    pub fn new(config: SimulationConfig) -> Self {
        let config_error = config.validate().err();
        Self {
            config,
            config_error,
            config_is_resolved: false,
            #[cfg(feature = "parallel")]
            parallel_pool: std::sync::OnceLock::new(),
            #[cfg(feature = "parallel")]
            classic_mos_parallel_pool: std::sync::OnceLock::new(),
            convergence: std::sync::Arc::new(std::sync::Mutex::new(ConvergenceQuality::new())),
        }
    }

    /// Create an engine only when its complete configuration is valid.
    ///
    /// Prefer this constructor at trust boundaries such as CLI configuration,
    /// language bindings, job queues, and services. [`Self::new`] remains
    /// available for compatibility with callers that construct known-good
    /// configurations internally.
    pub fn try_new(config: SimulationConfig) -> Result<Self, SimulationConfigError> {
        config.validate()?;
        Ok(Self {
            config,
            config_error: None,
            config_is_resolved: false,
            #[cfg(feature = "parallel")]
            parallel_pool: std::sync::OnceLock::new(),
            #[cfg(feature = "parallel")]
            classic_mos_parallel_pool: std::sync::OnceLock::new(),
            convergence: std::sync::Arc::new(std::sync::Mutex::new(ConvergenceQuality::new())),
        })
    }

    /// Create an engine from a fully resolved authoritative configuration.
    ///
    /// Analyses will not apply `.OPTIONS` again. Frontends use this after
    /// resolving `defaults < deck options < explicit run overrides`, which
    /// prevents a second resolution pass from overwriting the explicit
    /// temperature, tolerance, or other execution-owned values.
    pub fn new_with_resolved_config(config: SimulationConfig) -> Self {
        let config_error = config.validate().err();
        Self {
            config,
            config_error,
            config_is_resolved: true,
            #[cfg(feature = "parallel")]
            parallel_pool: std::sync::OnceLock::new(),
            #[cfg(feature = "parallel")]
            classic_mos_parallel_pool: std::sync::OnceLock::new(),
            convergence: std::sync::Arc::new(std::sync::Mutex::new(ConvergenceQuality::new())),
        }
    }

    /// Validating form of [`Self::new_with_resolved_config`].
    pub fn try_new_with_resolved_config(
        config: SimulationConfig,
    ) -> Result<Self, SimulationConfigError> {
        config.validate()?;
        Ok(Self {
            config,
            config_error: None,
            config_is_resolved: true,
            #[cfg(feature = "parallel")]
            parallel_pool: std::sync::OnceLock::new(),
            #[cfg(feature = "parallel")]
            classic_mos_parallel_pool: std::sync::OnceLock::new(),
            convergence: std::sync::Arc::new(std::sync::Mutex::new(ConvergenceQuality::new())),
        })
    }

    /// Convergence-quality metrics for the most recently completed analysis.
    ///
    /// Reports what the solver had to do to get an answer, as distinct from
    /// whether it got one: how often it fell back to gmin or source stepping,
    /// how many transient points were force-accepted without converging, and
    /// how many steps were rejected and retried. A run that returns clean
    /// numbers while reporting force-accepted points is telling you the
    /// waveform is not trustworthy at those points.
    ///
    /// Each analysis entry point clears this first, so the value describes one
    /// run. Read it before starting the next.
    ///
    /// This is orthogonal to [`Self::health_check`], which exercises a fixed
    /// three-line probe deck to answer whether the engine works at all.
    pub fn convergence_quality(&self) -> ConvergenceQuality {
        self.convergence
            .lock()
            .map(|quality| quality.clone())
            .unwrap_or_default()
    }

    /// Clear the metrics at the start of an analysis.
    pub(crate) fn reset_convergence_quality(&self) {
        if let Ok(mut quality) = self.convergence.lock() {
            *quality = ConvergenceQuality::new();
        }
    }

    /// Run `record` against the metrics, ignoring a poisoned lock.
    ///
    /// Diagnostics must never be the reason a simulation fails: a panic on
    /// another thread has already lost the run, and turning that into a second
    /// panic here would replace the real error with this one.
    #[inline]
    pub(crate) fn record_convergence<F>(&self, record: F)
    where
        F: FnOnce(&mut ConvergenceQuality),
    {
        if let Ok(mut quality) = self.convergence.lock() {
            record(&mut quality);
        }
    }

    /// Return the construction-time configuration error, if any.
    ///
    /// This lets compatibility callers that use [`Self::new`] preflight an
    /// engine without starting simulation work. Every circuit-building run
    /// also enforces the same check.
    pub fn configuration_error(&self) -> Option<&SimulationConfigError> {
        self.config_error.as_ref()
    }

    pub(crate) fn ensure_valid_configuration(&self) -> Result<(), SimulationError> {
        match &self.config_error {
            Some(error) => Err(error.clone().into()),
            None => Ok(()),
        }
    }

    #[inline]
    /// Check a requested analysis-point count against the configured limit.
    ///
    /// Callers that materialise sweep points themselves — batch planners, UI
    /// sweep editors — should pre-validate here so an oversized sweep is
    /// rejected before the points are allocated rather than during the run.
    pub fn ensure_analysis_points(&self, requested: usize) -> Result<(), SimulationError> {
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            requested,
            self.config.resource_limits.max_analysis_points,
        )?;
        Ok(())
    }

    #[inline]
    pub(crate) fn ensure_result_values(&self, requested: usize) -> Result<(), SimulationError> {
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            requested,
            self.config.resource_limits.max_result_values,
        )?;
        Ok(())
    }

    #[inline]
    /// Check a requested batch-run count against the configured limit.
    pub fn ensure_batch_runs(&self, requested: usize) -> Result<(), SimulationError> {
        ResourceLimitError::ensure(
            ResourceKind::BatchRuns,
            requested,
            self.config.resource_limits.max_batch_runs,
        )?;
        Ok(())
    }

    /// Effective parallel work width for an operation, honoring both the
    /// engine policy and any bounded Rayon pool already driving this engine.
    #[cfg(feature = "parallel")]
    pub(crate) fn parallel_worker_count(&self, work_items: usize) -> usize {
        let available = if rayon::current_thread_index().is_some() {
            rayon::current_num_threads()
        } else {
            std::thread::available_parallelism()
                .map(|count| count.get())
                .unwrap_or(1)
        };
        available
            .min(self.config.resource_limits.max_parallel_workers)
            .clamp(1, work_items.max(1))
    }

    /// Execute Rayon work without initializing its unbounded global pool.
    ///
    /// A frontend-owned Rayon worker (for example a CLI multi-run plan) is
    /// reused so nested analyses cannot multiply thread counts. Standalone
    /// engine calls lazily create one pool bounded by the shared resource
    /// policy and reuse it for the engine's lifetime.
    #[cfg(feature = "parallel")]
    pub(crate) fn install_parallel<R: Send>(
        &self,
        operation: impl FnOnce() -> R + Send,
    ) -> Result<R, SimulationError> {
        if rayon::current_thread_index().is_some() {
            return Ok(operation());
        }

        let pool = self.parallel_pool.get_or_init(|| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(self.parallel_worker_count(usize::MAX))
                .thread_name(|index| format!("rspice-core-{index}"))
                .build()
                .map_err(|error| format!("failed to create bounded analysis worker pool: {error}"))
        });
        let pool = pool.as_ref().map_err(|message| {
            SimulationError::Solver(crate::solver::SolverError::InvalidCircuit(message.clone()))
        })?;
        Ok(pool.install(operation))
    }

    /// Execute the memory-bound classic-MOS update on its measured efficient
    /// width without constraining host-wide frequency or corner parallelism.
    ///
    /// When an outer frontend pool already owns this engine, reuse it to
    /// preserve the no-oversubscription contract for parallel run plans.
    #[cfg(feature = "parallel")]
    pub(crate) fn install_classic_mos_parallel<R: Send>(
        &self,
        operation: impl FnOnce() -> R + Send,
    ) -> Result<R, SimulationError> {
        if rayon::current_thread_index().is_some() {
            return Ok(operation());
        }

        const MAX_CLASSIC_MOS_WORKERS: usize = 8;
        let pool = self.classic_mos_parallel_pool.get_or_init(|| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(
                    self.parallel_worker_count(usize::MAX)
                        .min(MAX_CLASSIC_MOS_WORKERS),
                )
                .thread_name(|index| format!("rspice-classic-mos-{index}"))
                .build()
                .map_err(|error| {
                    format!("failed to create classic-MOS analysis worker pool: {error}")
                })
        });
        let pool = pool.as_ref().map_err(|message| {
            SimulationError::Solver(crate::solver::SolverError::InvalidCircuit(message.clone()))
        })?;
        Ok(pool.install(operation))
    }

    pub(crate) fn ensure_result_shape(
        &self,
        points: usize,
        values_per_point: usize,
    ) -> Result<(), SimulationError> {
        self.ensure_result_values(points.saturating_mul(values_per_point))
    }

    pub(crate) fn simulation_result_value_count(result: &crate::solver::SimulationResult) -> usize {
        result
            .node_voltages
            .len()
            .saturating_add(result.branch_currents.len())
            .saturating_add(result.dc_observables.len())
            .saturating_add(result.time_points.len())
            .saturating_add(
                result
                    .voltage_waveforms
                    .iter()
                    .map(Vec::len)
                    .fold(0usize, usize::saturating_add),
            )
    }

    /// Get a reference to the simulation configuration
    pub fn config(&self) -> &SimulationConfig {
        &self.config
    }

    /// Resolve effective simulation configuration for a specific netlist.
    ///
    /// Applies `.OPTIONS` on top of the engine's base configuration.
    pub(crate) fn resolved_for_netlist(&self, netlist: &Netlist) -> Self {
        let mut resolved_engine = if self.config_is_resolved {
            Self::new_with_resolved_config(self.config.clone())
        } else {
            let resolved = resolve_simulation_config(
                &self.config,
                Some(&netlist.options),
                &SimulationConfigOverrides::default(),
            );
            Self::new_with_resolved_config(resolved)
        };
        // The analyses run on this engine, not on `self`, so it has to record
        // into the same metrics. Without sharing the handle every measurement
        // would be dropped with the temporary and `convergence_quality` would
        // report a clean run no matter what the solver actually did.
        resolved_engine.convergence = std::sync::Arc::clone(&self.convergence);
        resolved_engine
    }

    /// Refuse analyses that require native BSIM4 charge equations when a
    /// card selects a charge model not yet ported. DC evaluation remains
    /// valid for these cards because BSIM4's DC path is capmod-independent.
    pub(crate) fn ensure_supported_bsim4_dynamic_charges(
        circuit: &crate::CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        for dev in &circuit.bsim4v8.devices {
            let model = &dev.core.model;
            if !(0..=2).contains(&model.cap_mod) {
                return Err(SimulationError::Circuit(format!(
                    "{analysis} analysis requires native BSIM4 charge model equations; BSIM4 '{}' \
                     selects CAPMOD={} (only CAPMOD=0, 1, or 2 is implemented for charge-based analyses; \
                     DC operating point is supported)",
                    dev.name, model.cap_mod
                )));
            }
            if !model.cvcharge_mod_supported_for_charges() {
                return Err(SimulationError::Circuit(format!(
                    "{analysis} analysis requires native BSIM4 charge model equations; BSIM4 '{}' \
                     selects CVCHARGEMOD={} (only integer CVCHARGEMOD=0, 1, 2, or 3 is implemented for \
                     charge-based analyses; DC operating point is supported)",
                    dev.name, model.cvcharge_mod_value
                )));
            }
        }
        Ok(())
    }

    fn unsupported_b3soi_capmod_error(
        analysis: &str,
        family: &str,
        name: &str,
        selected: i32,
        supported: &str,
    ) -> SimulationError {
        SimulationError::Circuit(format!(
            "{analysis} analysis requires native {family} charge model equations; {family} '{name}' \
             selects CAPMOD={selected} (only {supported} is implemented for charge-based \
             analyses; DC operating point is supported)"
        ))
    }

    /// Refuse analyses that require B3SOI charge equations when a card selects
    /// a charge model not yet ported. DC evaluation remains valid because the
    /// SOI DC paths are capmod-independent.
    pub(crate) fn ensure_supported_b3soi_dynamic_charges(
        circuit: &crate::CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                continue;
            }
            if dev.model.cap_mod != 2 && dev.model.cap_mod != 3 {
                return Err(Self::unsupported_b3soi_capmod_error(
                    analysis,
                    "B3SOIFD",
                    &dev.name,
                    dev.model.cap_mod,
                    "CAPMOD=2 or 3",
                ));
            }
        }
        for dev in &circuit.b3soi.devices {
            if dev.charges_suppressed() {
                continue;
            }
            if dev.model.cap_mod != 2 && dev.model.cap_mod != 3 {
                return Err(Self::unsupported_b3soi_capmod_error(
                    analysis,
                    "B3SOIDD",
                    &dev.name,
                    dev.model.cap_mod,
                    "CAPMOD=2 or 3",
                ));
            }
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                continue;
            }
            if dev.model.cap_mod != 2 && dev.model.cap_mod != 3 {
                return Err(Self::unsupported_b3soi_capmod_error(
                    analysis,
                    "B3SOIPD",
                    &dev.name,
                    dev.model.cap_mod,
                    "CAPMOD=2 or 3",
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn ensure_supported_ekv3_dynamic_charges(
        circuit: &crate::CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        for dev in &circuit.ekv3s.devices {
            if analysis.eq_ignore_ascii_case("Noise") {
                continue;
            }
            return Err(SimulationError::Circuit(format!(
                "{analysis} analysis requires native EKV3 dynamic charge/stability equations; \
                 EKV3 '{}' is supported for the VA-Models source-backed LEVEL=301 \
                 NMOS150 DC operating-point/DC sweep slice and the Xyce-validated LEVEL=301 \
                 NMOS150 VANOISE small-signal/noise slice. AC, transient, STB, and pole-zero \
                 remain fail-closed until validated against Xyce dynamic oracles",
                dev.name
            )));
        }
        Ok(())
    }

    pub(crate) fn ensure_supported_dynamic_charges(
        circuit: &crate::CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        Self::ensure_supported_bsim4_dynamic_charges(circuit, analysis)?;
        Self::ensure_supported_b3soi_dynamic_charges(circuit, analysis)?;
        Self::ensure_supported_ekv3_dynamic_charges(circuit, analysis)?;
        Self::ensure_supported_xyce_memristor_small_signal(circuit, analysis)
    }

    pub(crate) fn ensure_supported_ac_dynamic_charges(
        circuit: &crate::CircuitData,
    ) -> Result<(), SimulationError> {
        Self::ensure_supported_bsim4_dynamic_charges(circuit, "AC")?;
        Self::ensure_supported_b3soi_dynamic_charges(circuit, "AC")?;
        Self::ensure_supported_ekv3_dynamic_charges(circuit, "AC")?;
        Self::ensure_supported_xyce_memristor_small_signal(circuit, "AC")
    }

    pub(crate) fn ensure_supported_transient_dynamic_charges(
        circuit: &crate::CircuitData,
    ) -> Result<(), SimulationError> {
        Self::ensure_supported_bsim4_dynamic_charges(circuit, "Transient")?;
        Self::ensure_supported_b3soi_dynamic_charges(circuit, "Transient")?;
        Self::ensure_supported_ekv3_dynamic_charges(circuit, "Transient")
    }

    pub(in crate::engine) fn ensure_supported_xyce_memristor_small_signal(
        circuit: &crate::CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        if let Some(device) = circuit.xyce_memristors.first() {
            let contract = if analysis.eq_ignore_ascii_case("PSS") {
                "periodic dynamic-state formulation"
            } else {
                "small-signal dynamic-state linearization"
            };
            return Err(SimulationError::Circuit(format!(
                "{analysis} analysis for native Xyce {} memristor '{}' remains fail-closed until its {contract} is validated against an authoritative oracle; DC and transient are supported",
                device.device.family_name(),
                device.name
            )));
        }
        Ok(())
    }

    /// Node names a probe string may refer to, in the order they are tried.
    ///
    /// Frontends resolving a user-typed probe (`V(out)`, `v(x1.n3)`) need the
    /// same candidate list the engine uses, so that a miss can be reported
    /// against what was actually searched.
    pub fn node_lookup_candidates(netlist: &Netlist, node_name: &str) -> Vec<String> {
        let canonical = netlist.ground_policy().canonical_node(node_name);
        if canonical == "0" {
            return vec![canonical.to_string()];
        }
        let mut candidates = Vec::new();
        Self::push_unique_node_lookup_candidate(&mut candidates, node_name.to_string());

        let normalized = Self::normalize_hierarchical_node_name(node_name).replace(':', ".");
        Self::push_unique_node_lookup_candidate(&mut candidates, normalized);

        if let Some(resolved) = Self::resolve_hierarchical_node_name(netlist, node_name) {
            Self::push_unique_node_lookup_candidate(&mut candidates, resolved);
        }

        candidates
    }

    /// Resolve a hierarchical probe name against a flattened netlist.
    ///
    /// Returns the flattened node name, or `None` when nothing matches any
    /// candidate from [`Self::node_lookup_candidates`].
    pub fn resolve_hierarchical_node_name(netlist: &Netlist, node_name: &str) -> Option<String> {
        let normalized = Self::normalize_hierarchical_node_name(node_name);
        Self::resolve_hierarchical_node_name_with_delimiter(netlist, &normalized, ':').or_else(
            || Self::resolve_hierarchical_node_name_with_delimiter(netlist, &normalized, '.'),
        )
    }

    fn resolve_hierarchical_node_name_with_delimiter(
        netlist: &Netlist,
        normalized: &str,
        delimiter: char,
    ) -> Option<String> {
        let parts = normalized.split(delimiter).collect::<Vec<_>>();
        if parts.len() < 2 {
            return None;
        }
        let delimiter = delimiter.to_string();

        for instance_count in (1..parts.len()).rev() {
            if parts[..instance_count].iter().any(|part| part.is_empty()) {
                continue;
            }

            let local_node = parts[instance_count..].join(&delimiter);
            if local_node.is_empty() {
                continue;
            }

            if let Some(resolved) = Self::resolve_hierarchical_node_parts(
                netlist,
                &parts[..instance_count],
                &local_node,
            ) {
                return Some(resolved);
            }
        }

        None
    }

    fn resolve_hierarchical_node_parts(
        netlist: &Netlist,
        instance_names: &[&str],
        local_node: &str,
    ) -> Option<String> {
        let mut elements = netlist.elements.as_slice();
        let root_subcircuits = netlist.subcircuits.as_slice();
        let mut subcircuits = root_subcircuits;
        let mut prefix = String::new();
        let mut node_map: HashMap<String, String> = HashMap::new();

        for instance_name in instance_names {
            let instance = elements.iter().find(|element| {
                element.name.eq_ignore_ascii_case(instance_name)
                    && matches!(element.kind, ElementKind::Subcircuit { .. })
            })?;
            let ElementKind::Subcircuit { subckt_name, .. } = &instance.kind else {
                return None;
            };
            let subckt = Self::find_subcircuit_definition(subcircuits, subckt_name)
                .or_else(|| Self::find_subcircuit_definition(root_subcircuits, subckt_name))?;

            let mut child_node_map = HashMap::new();
            for (port, external_node) in subckt.ports.iter().zip(instance.nodes.iter()) {
                child_node_map.insert(
                    Self::normalize_hierarchical_node_name(port),
                    Self::remap_hierarchical_node_name(
                        external_node,
                        &prefix,
                        &node_map,
                        &netlist.global_nodes,
                    ),
                );
            }

            prefix = if prefix.is_empty() {
                instance.name.clone()
            } else {
                format!("{}.{}", prefix, instance.name)
            };
            node_map = child_node_map;
            elements = subckt.elements.as_slice();
            subcircuits = subckt.nested_subcircuits.as_slice();
        }

        Some(Self::remap_hierarchical_node_name(
            local_node,
            &prefix,
            &node_map,
            &netlist.global_nodes,
        ))
    }

    fn startup_node_id(
        netlist: &Netlist,
        circuit: &crate::CircuitData,
        node_name: &str,
    ) -> Option<usize> {
        Self::node_lookup_candidates(netlist, node_name)
            .into_iter()
            .find_map(|candidate| circuit.get_node_by_name(&candidate))
    }

    fn push_unique_node_lookup_candidate(candidates: &mut Vec<String>, candidate: String) {
        if !candidates
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(&candidate))
        {
            candidates.push(candidate);
        }
    }

    fn normalize_hierarchical_node_name(node_name: &str) -> String {
        node_name
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase()
    }

    fn find_subcircuit_definition<'a>(
        subcircuits: &'a [SubcircuitDef],
        name: &str,
    ) -> Option<&'a SubcircuitDef> {
        for subckt in subcircuits {
            if subckt.name.eq_ignore_ascii_case(name) {
                return Some(subckt);
            }
            if let Some(nested) = Self::find_subcircuit_definition(&subckt.nested_subcircuits, name)
            {
                return Some(nested);
            }
        }
        None
    }

    fn remap_hierarchical_node_name(
        node: &str,
        prefix: &str,
        node_map: &HashMap<String, String>,
        global_nodes: &HashSet<String>,
    ) -> String {
        if node == "0" {
            return "0".to_string();
        }

        if global_nodes.contains(&node.to_ascii_uppercase()) {
            return node.to_string();
        }

        if let Some(mapped) = node_map.get(&Self::normalize_hierarchical_node_name(node)) {
            return mapped.clone();
        }

        if prefix.is_empty() {
            node.to_string()
        } else {
            format!("{prefix}.{node}")
        }
    }
}

#[cfg(test)]
mod convergence_quality_tests {
    use super::*;

    const DIVIDER: &str = "divider\nV1 1 0 10\nR1 1 0 1k\n.OP\n.end\n";

    #[test]
    fn a_well_behaved_deck_reports_clean_convergence() {
        let netlist = Netlist::parse(DIVIDER).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        engine.run_dc_op(&netlist).expect("divider solves");

        let quality = engine.convergence_quality();
        assert!(
            !quality.has_issues(),
            "a linear divider needs no convergence aids: {}",
            quality.summary()
        );
        assert_eq!(quality.summary(), "Clean convergence");
    }

    /// The analyses do not run on the engine the caller holds — every entry
    /// point rebuilds a resolved engine from the deck's `.OPTIONS`. If that
    /// engine does not share the metrics handle, everything it records is
    /// dropped with the temporary and this feature reports a clean run no
    /// matter what the solver did.
    #[test]
    fn the_resolved_engine_records_into_the_callers_metrics() {
        let netlist = Netlist::parse(DIVIDER).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let resolved = engine.resolved_for_netlist(&netlist);

        resolved.record_convergence(|quality| quality.record_force_accept(7));

        let quality = engine.convergence_quality();
        assert_eq!(
            quality.force_accepted_points, 1,
            "the resolved engine must record into the caller's metrics"
        );
        assert_eq!(quality.force_accepted_indices, vec![7]);
    }

    #[test]
    fn each_analysis_starts_from_a_clean_slate() {
        let netlist = Netlist::parse(DIVIDER).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());

        engine.record_convergence(|quality| quality.record_gmin_stepping());
        assert!(engine.convergence_quality().has_issues());

        engine.run_dc_op(&netlist).expect("divider solves");
        assert!(
            !engine.convergence_quality().has_issues(),
            "metrics describe one run, so an analysis must clear the previous one"
        );
    }

    #[test]
    fn recorded_aids_are_summarised_for_the_caller() {
        let engine = Engine::new(SimulationConfig::default());
        engine.record_convergence(|quality| {
            quality.record_gmin_stepping();
            quality.record_force_accept(3);
            quality.record_timestep_reduction();
        });

        let quality = engine.convergence_quality();
        assert!(quality.has_issues());
        assert_eq!(quality.timestep_reductions, 1);
        let summary = quality.summary();
        assert!(summary.contains("force-accepted"), "{summary}");
        assert!(summary.contains("GMIN"), "{summary}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hierarchical_node_lookup_resolves_nested_subcircuit_port() {
        let netlist = Netlist::parse(
            "\
nested hierarchical lookup
X2 4 5 IC_SubSubckt
.SUBCKT IC_Subckt in out
R1 in mid 10
C1 mid out 1u
.ENDS
.SUBCKT IC_SubSubckt in out
R1 in a 1
X1 a b IC_Subckt
R2 b out 1
.ENDS
.END
",
        )
        .expect("test deck parses");

        let resolved = Engine::resolve_hierarchical_node_name(&netlist, "X2:X1:out")
            .expect("hierarchical node resolves");

        assert_eq!(resolved.to_ascii_lowercase(), "x2.b");
    }

    #[test]
    fn hierarchical_node_lookup_resolves_dot_delimited_subcircuit_port() {
        let netlist = Netlist::parse(
            "\
dot-delimited hierarchical lookup
X2 4 5 IC_Subckt
.SUBCKT IC_Subckt in out
R1 in mid 10
C1 mid out 1u
.ENDS
.END
",
        )
        .expect("test deck parses");

        let resolved = Engine::resolve_hierarchical_node_name(&netlist, "X2.out")
            .expect("dot-delimited hierarchical node resolves");

        assert_eq!(resolved, "5");
    }

    #[test]
    fn hierarchical_node_lookup_preserves_colon_suffix_local_nodes() {
        let netlist = Netlist::parse(
            "\
colon local node lookup
X1 2 0 sub1
.SUBCKT sub1 a b
V1 1: 0 1
V2 : 0 1
X2 3 0 sub2
.ENDS
.SUBCKT sub2 c d
V1 1: 0 1
V2 : 0 1
.ENDS
.END
",
        )
        .expect("test deck parses");

        assert_eq!(
            Engine::resolve_hierarchical_node_name(&netlist, "X1:1:")
                .expect("colon-suffix local node resolves"),
            "X1.1:"
        );
        assert_eq!(
            Engine::resolve_hierarchical_node_name(&netlist, "X1::")
                .expect("literal colon local node resolves"),
            "X1.:"
        );
        assert_eq!(
            Engine::resolve_hierarchical_node_name(&netlist, "X1:X2:1:")
                .expect("nested colon-suffix local node resolves"),
            "X1.X2.1:"
        );
        assert_eq!(
            Engine::resolve_hierarchical_node_name(&netlist, "X1:X2::")
                .expect("nested literal colon local node resolves"),
            "X1.X2.:"
        );
    }

    #[test]
    fn hierarchical_startup_directives_apply_to_flattened_node_ids() {
        let netlist = Netlist::parse(
            "\
hierarchical startup directives
V1 1 0 0
X2 1 2 IC_SubSubckt
RLOAD 2 0 1k
.NODESET V(X2:X1:out)=0.1
.IC V(X2:X1:out)=0.25
.SUBCKT IC_Subckt in out
R1 in mid 10
C1 mid out 1u
.ENDS
.SUBCKT IC_SubSubckt in out
R1 in a 1
X1 a b IC_Subckt
R2 b out 1
.ENDS
.END
",
        )
        .expect("test deck parses");
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let flattened_node = circuit
            .get_node_by_name("X2.B")
            .expect("flattened internal node exists");

        let ic_hints = engine.collect_initial_condition_hints(&netlist, &circuit);
        assert_eq!(ic_hints, vec![(flattened_node, 0.25)]);

        let voltage_hints = engine.collect_node_voltage_hints(&netlist, &circuit);
        assert_eq!(voltage_hints, vec![(flattened_node, 0.25)]);
    }

    #[test]
    fn dot_delimited_subcircuit_port_initial_condition_targets_connected_node() {
        let netlist = Netlist::parse(
            "\
dot-delimited hierarchical startup directive
V1 1 0 0
X2 1 3 RC
CLOAD 3 0 1u
.IC V(X2.B)=0.5
.SUBCKT RC a b
R1 a mid 10
C1 mid b 1u
.ENDS
.END
",
        )
        .expect("test deck parses");
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let connected_node = circuit
            .get_node_by_name("3")
            .expect("connected top-level node exists");

        let ic_hints = engine.collect_initial_condition_hints(&netlist, &circuit);
        assert_eq!(ic_hints, vec![(connected_node, 0.5)]);
    }

    #[test]
    fn infallible_hint_collectors_never_apply_failed_startup_validation() {
        let mut netlist = Netlist::parse(
            "invalid startup hints\n\
             V1 1 0 1\n\
             .IC V(1)=0.25\n\
             .NODESET V(1)=0.75\n\
             .OP\n\
             .END\n",
        )
        .expect("default/ngspice mode permits both startup modes");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("permissive-mode circuit builds");
        netlist
            .params
            .set_expression_dialect(crate::config::ExpressionDialect::Xyce);

        assert!(
            engine
                .collect_initial_condition_hints(&netlist, &circuit)
                .is_empty()
        );
        assert!(
            engine
                .collect_node_voltage_hints(&netlist, &circuit)
                .is_empty()
        );
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(SimulationConfig::default())
    }
}

impl Engine {
    fn scoped_startup_directives(
        &self,
        netlist: &Netlist,
    ) -> (
        Vec<crate::netlist::InitialCondition>,
        Vec<crate::netlist::NodeSet>,
    ) {
        if netlist.subcircuits.is_empty() {
            return (Vec::new(), Vec::new());
        }

        match crate::netlist::flatten_netlist_with_models(netlist) {
            Ok(flattened) => (
                flattened.scoped_initial_conditions,
                flattened.scoped_node_sets,
            ),
            Err(error) => {
                log::debug!(
                    "could not collect scoped startup directives from flattened netlist: {error}"
                );
                (Vec::new(), Vec::new())
            }
        }
    }

    /// Collect node-voltage hints from .NODESET and .IC directives.
    ///
    /// .IC entries override .NODESET entries for the same node.
    pub(crate) fn collect_node_voltage_hints(
        &self,
        netlist: &Netlist,
        circuit: &crate::CircuitData,
    ) -> Vec<(usize, Value)> {
        let Some(validated) = self.validated_startup_netlist(netlist) else {
            return Vec::new();
        };
        let netlist = validated.as_ref();
        let mut by_node: Vec<Option<Value>> = vec![None; circuit.num_nodes() + 1];
        let (scoped_initial_conditions, scoped_node_sets) = self.scoped_startup_directives(netlist);

        for nodeset in netlist.node_sets.iter().chain(scoped_node_sets.iter()) {
            if !nodeset.voltage.is_finite() {
                continue;
            }
            if let Some(node_id) = Self::startup_node_id(netlist, circuit, &nodeset.node)
                && node_id > 0
                && node_id <= circuit.num_nodes()
            {
                by_node[node_id] = Some(nodeset.voltage);
            }
        }

        for ic in netlist
            .initial_conditions
            .iter()
            .chain(scoped_initial_conditions.iter())
        {
            if !ic.voltage.is_finite() {
                continue;
            }
            if let Some(node_id) = Self::startup_node_id(netlist, circuit, &ic.node)
                && node_id > 0
                && node_id <= circuit.num_nodes()
            {
                by_node[node_id] = Some(ic.voltage);
            }
        }

        by_node
            .into_iter()
            .enumerate()
            .skip(1)
            .filter_map(|(node_id, voltage)| voltage.map(|v| (node_id, v)))
            .collect()
    }

    /// Collect node-voltage initial conditions from .IC directives only.
    pub(crate) fn collect_initial_condition_hints(
        &self,
        netlist: &Netlist,
        circuit: &crate::CircuitData,
    ) -> Vec<(usize, Value)> {
        let Some(validated) = self.validated_startup_netlist(netlist) else {
            return Vec::new();
        };
        let netlist = validated.as_ref();
        let (scoped_initial_conditions, _) = self.scoped_startup_directives(netlist);
        netlist
            .initial_conditions
            .iter()
            .chain(scoped_initial_conditions.iter())
            .filter_map(|ic| {
                if !ic.voltage.is_finite() {
                    return None;
                }
                let node_id = Self::startup_node_id(netlist, circuit, &ic.node)?;
                if node_id == 0 || node_id > circuit.num_nodes() {
                    return None;
                }
                Some((node_id, ic.voltage))
            })
            .collect()
    }

    /// Apply .IC node overrides to an operating-point solution vector.
    ///
    /// Returns the number of nodes overridden.
    pub(crate) fn apply_initial_condition_overrides(
        &self,
        netlist: &Netlist,
        circuit: &crate::CircuitData,
        solution: &mut [Value],
    ) -> usize {
        let hints = self.collect_initial_condition_hints(netlist, circuit);
        let mut applied = 0usize;

        for (node_id, voltage) in hints {
            let idx = node_id - 1;
            if idx < solution.len() {
                solution[idx] = voltage;
                applied += 1;
            }
        }

        applied
    }

    /// Solve the DC operating point with optional node-voltage hints.
    #[cfg(test)]
    pub(crate) fn solve_dc_operating_point(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        self.solve_dc_operating_point_with_abort(
            netlist,
            circuit,
            matrix,
            &crate::abort_signal::NoAbort,
        )
    }

    /// Solve the DC operating point with optional node-voltage hints and abort support.
    pub(crate) fn solve_dc_operating_point_with_abort(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        abort: &dyn crate::abort_signal::AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        self.solve_dc_operating_point_with_startup_and_abort(
            netlist,
            circuit,
            matrix,
            DcOpStartup::Automatic,
            abort,
        )
    }

    pub(in crate::engine) fn solve_dc_operating_point_with_startup_and_abort(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        startup: DcOpStartup<'_>,
        abort: &dyn crate::abort_signal::AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let force_initial_conditions = matches!(startup, DcOpStartup::ForceInitialConditions);
        if !force_initial_conditions {
            self.ensure_dc_paths_to_ground(circuit)?;
        }
        let solution = match startup {
            DcOpStartup::ForceInitialConditions => {
                let hints = self.collect_initial_condition_hints(netlist, circuit);
                if hints.is_empty() {
                    return Err(SimulationError::Circuit(
                        "forced .IC operating point requires at least one valid .IC node voltage"
                            .to_owned(),
                    ));
                }
                let seed = vec![0.0; circuit.matrix_size()];
                self.solve_nonlinear_nodeset_dc_startup_with_abort(
                    circuit, matrix, &seed, &hints, abort,
                )
            }
            DcOpStartup::PreviousSolution(solution) => {
                if solution.len() != circuit.matrix_size()
                    || solution.iter().any(|value| !value.is_finite())
                {
                    return Err(SimulationError::Circuit(
                        "previous operating-point state is incompatible with the current circuit"
                            .to_owned(),
                    ));
                }
                if circuit.has_nonlinear_devices() || !circuit.generic_switches.is_empty() {
                    self.solve_nonlinear_with_guess_and_abort(
                        circuit,
                        matrix,
                        Some(solution),
                        abort,
                    )
                } else {
                    self.solve_linear(circuit, matrix)
                }
            }
            DcOpStartup::Zero => {
                if circuit.has_nonlinear_devices() || !circuit.generic_switches.is_empty() {
                    let seed = vec![0.0; circuit.matrix_size()];
                    self.solve_nonlinear_with_guess_and_abort(circuit, matrix, Some(&seed), abort)
                } else {
                    self.solve_linear(circuit, matrix)
                }
            }
            DcOpStartup::Automatic => {
                if circuit.has_nonlinear_devices() || !circuit.generic_switches.is_empty() {
                    let hints = self.collect_node_voltage_hints(netlist, circuit);
                    if hints.is_empty() {
                        self.solve_nonlinear_with_node_hints_and_abort(circuit, matrix, &[], abort)
                    } else {
                        self.solve_nonlinear_with_node_hints_and_abort(
                            circuit, matrix, &hints, abort,
                        )
                    }
                } else {
                    if abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    self.solve_linear(circuit, matrix)
                }
            }
        }?;
        // A forced `.IC` solve deliberately replaces authored node KCL rows
        // with ideal voltage constraints. It is a constrained state, not a
        // physical unconstrained DC equilibrium, so reconstructing ordinary
        // DC KCL here would apply the wrong equation contract.
        if !force_initial_conditions {
            self.ensure_solved_dc_paths_to_ground(circuit, matrix, &solution)?;
        }
        Ok(solution)
    }

    /// Refuse a current-driven floating component whose operating point the
    /// numerical conditioning shunt would invent rather than the circuit
    /// determine.
    ///
    /// This check belongs at the shared DC-solve boundary: operating-point,
    /// AC, noise, distortion, pole-zero, PSS, STB, sensitivity, and ordinary
    /// transient startup all pass through here. A `.TRAN ... UIC` run skips
    /// this boundary because it deliberately skips the operating point.
    ///
    /// Purely passive floating islands preserve historical SPICE behavior:
    /// construction reports their no-path warning and the solve may choose
    /// their irrelevant common mode. `.OPTIONS RSHUNT` is a physical escape
    /// hatch because it adds an author-sized path from every node to ground.
    pub(in crate::engine) fn ensure_dc_paths_to_ground(
        &self,
        circuit: &CircuitData,
    ) -> Result<(), SimulationError> {
        let floating =
            circuit.independent_dc_drive_nodes(self.current_abstol(), self.residual_reltol());
        self.ensure_named_dc_paths_to_ground(&floating)
    }

    pub(in crate::engine) fn ensure_solved_dc_paths_to_ground(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &[Value],
    ) -> Result<(), SimulationError> {
        let violating = self.physical_dc_kcl_violation_nodes(circuit, matrix, solution)?;
        self.ensure_no_conditioning_dependent_kcl(circuit, &violating, "DC operating point")
    }

    pub(in crate::engine) fn ensure_solved_transient_operating_point_paths_to_ground(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &[Value],
        time: Value,
        contract: super::convergence::AcceptedTransientOperatingPointContract,
    ) -> Result<(), SimulationError> {
        let violating = self.physical_transient_operating_point_kcl_violation_nodes(
            circuit, matrix, solution, time, contract,
        )?;
        self.ensure_no_conditioning_dependent_kcl(circuit, &violating, "transient operating point")
    }

    fn ensure_no_conditioning_dependent_kcl(
        &self,
        circuit: &CircuitData,
        violating: &[String],
        operating_point_kind: &str,
    ) -> Result<(), SimulationError> {
        if violating.is_empty() {
            return Ok(());
        }
        let shown: Vec<&str> = violating.iter().take(8).map(String::as_str).collect();
        let suffix = if violating.len() > shown.len() {
            format!(" (and {} more)", violating.len() - shown.len())
        } else {
            String::new()
        };
        let remediation = if circuit.has_global_shunt() {
            "Reduce .OPTIONS RSHUNT to provide a stronger physical shunt, or add a stronger explicit DC path."
        } else {
            "Add a physical DC path, or set .OPTIONS RSHUNT=<ohms> to shunt every node to ground."
        };
        Err(SimulationError::Circuit(format!(
            "{operating_point_kind} at node(s) {}{} depends materially on the simulator's numerical \
             nodal conditioning: the accepted bias has no DC path to ground strong enough to \
             satisfy the installed circuit's physical KCL at the configured tolerances after that \
             conditioning is removed. {remediation}",
            shown.join(", "),
            suffix
        )))
    }

    fn ensure_named_dc_paths_to_ground(&self, floating: &[String]) -> Result<(), SimulationError> {
        if floating.is_empty() {
            return Ok(());
        }
        let shown: Vec<&str> = floating.iter().take(8).map(String::as_str).collect();
        let suffix = if floating.len() > shown.len() {
            format!(" (and {} more)", floating.len() - shown.len())
        } else {
            String::new()
        };
        Err(SimulationError::Circuit(format!(
            "no DC path to ground from node(s) {}{}: capacitors and current sources do not \
             conduct at DC, so nothing in the circuit sets their operating-point voltage. \
             Connect them through a conducting element, or set .OPTIONS RSHUNT=<ohms> to \
             shunt every node to ground with a resistor of that value.",
            shown.join(", "),
            suffix
        )))
    }
}
