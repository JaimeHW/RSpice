//! Browser-safe resource policy, execution options, and compression policy.
//!
//! Every JavaScript export accepts the same additive options object. Unknown
//! fields are rejected so a misspelled control cannot silently widen a limit.

use rspice_core::ResourceLimits;

use serde::{Deserialize, Serialize};

use crate::DetailedWasmResult;
use crate::errors::WasmError;

pub(crate) const MEBIBYTE: usize = 1024 * 1024;
pub(crate) const MAX_TIMEOUT_MILLISECONDS: u32 = 86_400_000;
pub(crate) const DEFAULT_MAX_TRANSFER_VALUES: usize = 262_144;
/// Largest lossless JSON export one retained result may produce.
pub(crate) const DEFAULT_MAX_RESULT_JSON_BYTES: u64 = 64 * 1024 * 1024;

pub(crate) fn browser_resource_limits() -> ResourceLimits {
    let mut limits = ResourceLimits::default();
    limits.max_netlist_bytes = 8 * MEBIBYTE;
    limits.max_netlist_lines = 250_000;
    limits.max_expanded_source_bytes = 16 * MEBIBYTE;
    limits.max_dependency_source_bytes = 16 * MEBIBYTE;
    limits.max_external_data_bytes = 16 * MEBIBYTE;
    limits.max_external_data_values = 2_000_000;
    limits.max_shared_cache_bytes = 64 * MEBIBYTE;
    limits.max_include_depth = 16;
    limits.max_hierarchy_depth = 64;
    limits.max_flattened_elements = 20_000;
    limits.max_circuit_nodes = 2_000;
    limits.max_matrix_unknowns = 2_000;
    limits.max_analysis_points = 200_000;
    limits.max_result_values = 2_000_000;
    limits.max_parallel_workers = 1;
    limits.max_batch_runs = 1_000;
    limits
}

/// Browser-facing resource policy. JavaScript field names use camelCase.
///
/// Partial objects inherit every omitted field from the browser-safe defaults.
/// Unknown fields are rejected so a misspelled security control never appears
/// to have been applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct WasmResourceLimits {
    pub max_netlist_bytes: usize,
    pub max_netlist_lines: usize,
    pub max_expanded_source_bytes: usize,
    pub max_dependency_source_bytes: usize,
    pub max_external_data_bytes: usize,
    pub max_external_data_values: usize,
    pub max_shared_cache_bytes: usize,
    pub max_include_depth: usize,
    pub max_hierarchy_depth: usize,
    pub max_flattened_elements: usize,
    pub max_circuit_nodes: usize,
    pub max_matrix_unknowns: usize,
    pub max_analysis_points: usize,
    pub max_result_values: usize,
    pub max_parallel_workers: usize,
    pub max_batch_runs: usize,
}

impl WasmResourceLimits {
    pub(crate) fn from_core(limits: ResourceLimits) -> Self {
        Self {
            max_netlist_bytes: limits.max_netlist_bytes,
            max_netlist_lines: limits.max_netlist_lines,
            max_expanded_source_bytes: limits.max_expanded_source_bytes,
            max_dependency_source_bytes: limits.max_dependency_source_bytes,
            max_external_data_bytes: limits.max_external_data_bytes,
            max_external_data_values: limits.max_external_data_values,
            max_shared_cache_bytes: limits.max_shared_cache_bytes,
            max_include_depth: limits.max_include_depth,
            max_hierarchy_depth: limits.max_hierarchy_depth,
            max_flattened_elements: limits.max_flattened_elements,
            max_circuit_nodes: limits.max_circuit_nodes,
            max_matrix_unknowns: limits.max_matrix_unknowns,
            max_analysis_points: limits.max_analysis_points,
            max_result_values: limits.max_result_values,
            max_parallel_workers: limits.max_parallel_workers,
            max_batch_runs: limits.max_batch_runs,
        }
    }

    pub(crate) fn to_core(&self) -> ResourceLimits {
        let mut limits = ResourceLimits::default();
        limits.max_netlist_bytes = self.max_netlist_bytes;
        limits.max_netlist_lines = self.max_netlist_lines;
        limits.max_expanded_source_bytes = self.max_expanded_source_bytes;
        limits.max_dependency_source_bytes = self.max_dependency_source_bytes;
        limits.max_external_data_bytes = self.max_external_data_bytes;
        limits.max_external_data_values = self.max_external_data_values;
        limits.max_shared_cache_bytes = self.max_shared_cache_bytes;
        limits.max_include_depth = self.max_include_depth;
        limits.max_hierarchy_depth = self.max_hierarchy_depth;
        limits.max_flattened_elements = self.max_flattened_elements;
        limits.max_circuit_nodes = self.max_circuit_nodes;
        limits.max_matrix_unknowns = self.max_matrix_unknowns;
        limits.max_analysis_points = self.max_analysis_points;
        limits.max_result_values = self.max_result_values;
        limits.max_parallel_workers = self.max_parallel_workers;
        limits.max_batch_runs = self.max_batch_runs;
        limits
    }
}

impl Default for WasmResourceLimits {
    fn default() -> Self {
        Self::from_core(browser_resource_limits())
    }
}

/// Extensible options object accepted by every JavaScript export.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct WasmExecutionOptions {
    pub resource_limits: WasmResourceLimits,
    /// Waveform compression applied to every authored `.TRAN` this call runs.
    ///
    /// Omitting it publishes the full accepted grid. Compression is a browser
    /// transfer policy: the solver and the authored output projection are
    /// identical either way, and the published result always carries the
    /// compression certificate that says which grid it is on.
    pub transient_compression: Option<WasmCompressionOptions>,
}

impl WasmExecutionOptions {
    /// The core compression policy this call asked for, if any.
    pub(crate) fn compression_config(
        &self,
    ) -> DetailedWasmResult<Option<rspice_core::engine::CompressionConfig>> {
        self.transient_compression
            .as_ref()
            .map(WasmCompressionOptions::to_core)
            .transpose()
    }
}

/// Browser-facing transient compression policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct WasmCompressionOptions {
    /// Absolute interpolation error in each channel's native units.
    pub absolute_tolerance: f64,
    /// Relative interpolation error as a fraction of the actual value.
    pub relative_tolerance: f64,
    /// Maximum retained time-axis gap. Zero disables the gap ceiling.
    pub maximum_interval: f64,
    /// Set false to preserve every accepted point while retaining explicit
    /// compression provenance.
    pub enabled: bool,
}

impl Default for WasmCompressionOptions {
    fn default() -> Self {
        let defaults = rspice_core::engine::CompressionConfig::default();
        Self {
            absolute_tolerance: defaults.abs_tol,
            relative_tolerance: defaults.rel_tol,
            maximum_interval: defaults.maximum_retained_interval,
            enabled: defaults.enabled,
        }
    }
}

impl WasmCompressionOptions {
    pub(crate) fn to_core(&self) -> DetailedWasmResult<rspice_core::engine::CompressionConfig> {
        for (name, value) in [
            ("absoluteTolerance", self.absolute_tolerance),
            ("relativeTolerance", self.relative_tolerance),
            ("maximumInterval", self.maximum_interval),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(Box::new(WasmError::invalid_argument(format!(
                    "transient compression {name} must be finite and non-negative, got {value}"
                ))));
            }
        }
        Ok(rspice_core::engine::CompressionConfig {
            abs_tol: self.absolute_tolerance,
            rel_tol: self.relative_tolerance,
            enabled: self.enabled,
            maximum_retained_interval: self.maximum_interval,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Browser defaults are stricter than the desktop defaults, because a page
    /// cannot be allowed to exhaust a tab's memory.
    #[test]
    fn browser_defaults_are_stricter_than_the_desktop_defaults() {
        let browser = browser_resource_limits();
        let desktop = ResourceLimits::default();
        assert!(browser.max_netlist_bytes <= desktop.max_netlist_bytes);
        assert!(browser.max_circuit_nodes <= desktop.max_circuit_nodes);
        assert!(browser.max_analysis_points <= desktop.max_analysis_points);
        assert!(browser.max_result_values <= desktop.max_result_values);
        assert_eq!(browser.max_parallel_workers, 1);
    }

    /// A partial options object inherits every omitted browser default, and a
    /// misspelled control is rejected rather than silently ignored.
    #[test]
    fn partial_options_inherit_defaults_and_unknown_fields_are_rejected() {
        let options: WasmExecutionOptions =
            serde_json::from_str(r#"{"resourceLimits":{"maxCircuitNodes":8}}"#)
                .expect("a partial options object decodes");
        assert_eq!(options.resource_limits.max_circuit_nodes, 8);
        assert_eq!(
            options.resource_limits.max_netlist_bytes,
            WasmResourceLimits::default().max_netlist_bytes
        );
        assert!(options.transient_compression.is_none());

        serde_json::from_str::<WasmExecutionOptions>(r#"{"resourceLimits":{"maxNodes":8}}"#)
            .expect_err("a misspelled resource control must be rejected");
        serde_json::from_str::<WasmExecutionOptions>(r#"{"resourceLimit":{}}"#)
            .expect_err("a misspelled option must be rejected");
    }

    /// Compression tolerances must be finite and non-negative; a nonsense
    /// policy fails before any solve.
    #[test]
    fn compression_options_fail_closed_on_impossible_tolerances() {
        for policy in [
            WasmCompressionOptions {
                absolute_tolerance: -1.0,
                ..WasmCompressionOptions::default()
            },
            WasmCompressionOptions {
                relative_tolerance: f64::NAN,
                ..WasmCompressionOptions::default()
            },
            WasmCompressionOptions {
                maximum_interval: f64::INFINITY,
                ..WasmCompressionOptions::default()
            },
        ] {
            let error = *policy
                .to_core()
                .expect_err("an impossible compression policy must fail closed");
            assert_eq!(error.code, "invalid_argument");
            assert_eq!(error.category, "input_validation");
        }
        assert!(WasmCompressionOptions::default().to_core().is_ok());
    }
}
