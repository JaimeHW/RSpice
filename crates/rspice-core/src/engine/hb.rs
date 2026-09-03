//! Harmonic Balance (HB) Analysis Engine Integration
//!
//! This module provides Harmonic Balance analysis for finding
//! periodic steady-state solutions in the frequency domain.
//!
//! # Overview
//!
//! HB solves directly for the Fourier coefficients of node voltages, making it
//! ideal for RF/MW circuits with slow time constants where transient would be
//! prohibitively slow.
//!
//! # Algorithm
//!
//! 1. **Circuit setup**: Build admittance matrices G (conductance) and C (capacitance)
//! 2. **Source stamping**: Extract DC and AC source spectra
//! 3. **Newton iteration**: Solve for spectral coefficients via Newton-Raphson
//!    - Linear part: (G + jω*C) * X
//!    - Nonlinear part: FFT ↔ time-domain evaluation ↔ IFFT
//! 4. **Result construction**: Build HbResult with spectral voltages and harmonics

use super::{Engine, SimulationError, TransientStartupMode};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::harmonic_balance::{ExactPeriodicNetwork, HbDcSeedPolicy, HbFft};
use crate::analysis::{HbConfig, HbResult, HbSolver, HbSolverState};
use crate::circuit::CircuitData;
use crate::engine::periodic_capability;
use crate::engine::transient::netlist_checkpoint_identity;
use crate::netlist::SourceSpec;
use crate::{Netlist, Value};
use num_complex::Complex64;
use std::collections::BTreeSet;

mod drive;
mod pac;
mod pnoise;
#[cfg(test)]
mod retained_auth_tests;
mod stamping;
mod state;

pub use pac::PacAnalysisResult;
pub use pnoise::PnoiseAnalysisResult;
pub use state::{HbEnvelopeContinuationState, HbEnvelopeStateGuarantee};

const HB_OPERATING_POINT_IDENTITY_VERSION: u32 = 2;

fn hb_identity_field(hasher: &mut blake3::Hasher, name: &str, bytes: &[u8]) {
    hasher.update(&(name.len() as u64).to_le_bytes());
    hasher.update(name.as_bytes());
    hasher.update(&(bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

fn hb_config_identity(config: &HbConfig) -> String {
    let mut hasher = blake3::Hasher::new();
    // Version 2 authenticates the dialect-selected source projection. The
    // preceding contract produced analytic PULSE coefficients in every
    // dialect, while Xyce APFT now projects single-tone sources on its finite
    // collocation grid. Retained states from those transforms are not
    // interchangeable even when every authored HbConfig field is identical.
    hasher.update(b"rspice-hb-source-transform-config-v2\0");
    hb_identity_field(
        &mut hasher,
        "fundamental_freq",
        &config.fundamental_freq.to_bits().to_le_bytes(),
    );
    hb_identity_field(
        &mut hasher,
        "num_harmonics",
        &(config.num_harmonics as u64).to_le_bytes(),
    );
    hb_identity_field(
        &mut hasher,
        "tone_count",
        &(config.tones.len() as u64).to_le_bytes(),
    );
    for (index, tone) in config.tones.iter().enumerate() {
        hb_identity_field(
            &mut hasher,
            &format!("tone[{index}].frequency"),
            &tone.frequency.to_bits().to_le_bytes(),
        );
        hb_identity_field(
            &mut hasher,
            &format!("tone[{index}].num_harmonics"),
            &(tone.num_harmonics as u64).to_le_bytes(),
        );
        hb_identity_field(
            &mut hasher,
            &format!("tone[{index}].name"),
            tone.name.as_bytes(),
        );
        hb_identity_field(
            &mut hasher,
            &format!("tone[{index}].source_name"),
            tone.source_name.as_deref().unwrap_or("<none>").as_bytes(),
        );
    }
    for (name, value) in [
        ("tolerance", config.tolerance),
        ("abstol", config.abstol),
        ("damping", config.damping),
        ("min_damping", config.min_damping),
    ] {
        hb_identity_field(&mut hasher, name, &value.to_bits().to_le_bytes());
    }
    for (name, value) in [
        ("max_iterations", config.max_iterations),
        ("oversample_factor", config.oversample_factor),
        ("max_mixing_order", config.max_mixing_order),
        ("gmres_restart", config.gmres_restart),
    ] {
        hb_identity_field(&mut hasher, name, &(value as u64).to_le_bytes());
    }
    hb_identity_field(
        &mut hasher,
        "collocation_points",
        &(config
            .collocation_points
            .map(|value| value as u64)
            .unwrap_or(u64::MAX))
        .to_le_bytes(),
    );
    for (name, value) in [
        ("use_krylov", config.use_krylov),
        ("source_stepping", config.source_stepping),
        ("use_exact_jacobian", config.use_exact_jacobian),
        ("verbose", config.verbose),
    ] {
        hb_identity_field(&mut hasher, name, &[u8::from(value)]);
    }
    hasher.finalize().to_hex().to_string()
}

#[allow(clippy::too_many_arguments)]
fn hb_retained_state_identity(
    config: &HbConfig,
    node_names: &[String],
    spectral_state: &[Vec<Complex64>],
    mna_branch_names: &[String],
    mna_branch_spectral_state: &[Vec<Complex64>],
    iterations: usize,
    residual_norm: Value,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"rspice-hb-retained-state-v1\0");
    hb_identity_field(
        &mut hasher,
        "hb_config_identity",
        hb_config_identity(config).as_bytes(),
    );
    hb_identity_field(
        &mut hasher,
        "iterations",
        &(iterations as u64).to_le_bytes(),
    );
    hb_identity_field(
        &mut hasher,
        "residual_norm",
        &residual_norm.to_bits().to_le_bytes(),
    );
    hb_identity_field(
        &mut hasher,
        "node_count",
        &(node_names.len() as u64).to_le_bytes(),
    );
    for (index, (node, spectrum)) in node_names.iter().zip(spectral_state).enumerate() {
        hb_identity_field(&mut hasher, &format!("node[{index}].name"), node.as_bytes());
        hb_identity_field(
            &mut hasher,
            &format!("node[{index}].coefficient_count"),
            &(spectrum.len() as u64).to_le_bytes(),
        );
        for (harmonic, value) in spectrum.iter().enumerate() {
            hb_identity_field(
                &mut hasher,
                &format!("node[{index}].coefficient[{harmonic}].real"),
                &value.re.to_bits().to_le_bytes(),
            );
            hb_identity_field(
                &mut hasher,
                &format!("node[{index}].coefficient[{harmonic}].imaginary"),
                &value.im.to_bits().to_le_bytes(),
            );
        }
    }
    hb_identity_field(
        &mut hasher,
        "mna_branch_count",
        &(mna_branch_names.len() as u64).to_le_bytes(),
    );
    for (index, (branch, spectrum)) in mna_branch_names
        .iter()
        .zip(mna_branch_spectral_state)
        .enumerate()
    {
        hb_identity_field(
            &mut hasher,
            &format!("mna_branch[{index}].name"),
            branch.as_bytes(),
        );
        hb_identity_field(
            &mut hasher,
            &format!("mna_branch[{index}].coefficient_count"),
            &(spectrum.len() as u64).to_le_bytes(),
        );
        for (harmonic, value) in spectrum.iter().enumerate() {
            hb_identity_field(
                &mut hasher,
                &format!("mna_branch[{index}].coefficient[{harmonic}].real"),
                &value.re.to_bits().to_le_bytes(),
            );
            hb_identity_field(
                &mut hasher,
                &format!("mna_branch[{index}].coefficient[{harmonic}].imaginary"),
                &value.im.to_bits().to_le_bytes(),
            );
        }
    }
    hasher.finalize().to_hex().to_string()
}

fn hb_resolved_simulation_identity(config: &super::SimulationConfig) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"rspice-hb-resolved-simulation-config-v1\0");
    for (name, value) in [
        ("tolerance", config.tolerance),
        ("temperature", config.temperature),
        ("matrix_pivot_tolerance", config.matrix_pivot_tolerance),
        (
            "matrix_absolute_pivot_tolerance",
            config.matrix_absolute_pivot_tolerance,
        ),
    ] {
        hb_identity_field(&mut hasher, name, &value.to_bits().to_le_bytes());
    }
    hb_identity_field(
        &mut hasher,
        "max_iterations",
        &(config.max_iterations as u64).to_le_bytes(),
    );
    for (name, value) in [
        ("b3soi_gmin_scaling", config.b3soi_gmin_scaling),
        ("device_voltage_limiting", config.device_voltage_limiting),
        ("bypass.enabled", config.bypass_config.enabled),
    ] {
        hb_identity_field(&mut hasher, name, &[u8::from(value)]);
    }
    for (name, value) in [("rshunt", config.rshunt), ("cshunt", config.cshunt)] {
        hb_identity_field(
            &mut hasher,
            name,
            &value.map(Value::to_bits).unwrap_or(u64::MAX).to_le_bytes(),
        );
    }
    for (name, value) in [
        ("bypass.reltol", config.bypass_config.reltol),
        ("bypass.abstol", config.bypass_config.abstol),
        ("gmin_initial", config.convergence_config.gmin_initial),
        ("gmin_target", config.convergence_config.gmin_target),
        (
            "junction_gmin_target",
            config.convergence_config.junction_gmin_target,
        ),
        ("voltage_reltol", config.convergence_config.voltage_reltol),
        ("voltage_abstol", config.convergence_config.voltage_abstol),
        ("current_abstol", config.convergence_config.current_abstol),
        ("charge_abstol", config.convergence_config.charge_abstol),
        ("residual_reltol", config.convergence_config.residual_reltol),
    ] {
        hb_identity_field(&mut hasher, name, &value.to_bits().to_le_bytes());
    }
    for (name, value) in [
        ("gmin_stepping", config.convergence_config.gmin_stepping),
        ("source_stepping", config.convergence_config.source_stepping),
        (
            "pseudo_transient",
            config.convergence_config.pseudo_transient,
        ),
        ("arc_length", config.convergence_config.arc_length),
    ] {
        hb_identity_field(&mut hasher, name, &[u8::from(value)]);
    }
    for (name, value) in [
        ("spice_dialect", format!("{:?}", config.spice_dialect)),
        (
            "jfet_level2_model",
            format!("{:?}", config.resolved_jfet_level2_model()),
        ),
        ("matrix_solver", format!("{:?}", config.matrix_solver)),
        (
            "nonlinear_continuation",
            format!("{:?}", config.convergence_config.nonlinear_continuation),
        ),
        (
            "damping_strategy",
            format!("{:?}", config.convergence_config.damping_strategy),
        ),
    ] {
        hb_identity_field(&mut hasher, name, value.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

fn is_canonical_blake3_identity(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Versioned semantic producer identity for a retained HB operating point.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct HbOperatingPointIdentity {
    version: u32,
    semantic_netlist_identity: String,
    resolved_simulation_identity: String,
    hb_source_transform_identity: String,
    #[cfg_attr(feature = "veriloga", serde(default))]
    retained_state_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HbOperatingPointProducerInputs {
    semantic_netlist_identity: String,
    resolved_simulation_identity: String,
    hb_source_transform_identity: String,
}

impl HbOperatingPointIdentity {
    /// Identity schema version.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Fully elaborated semantic netlist identity.
    pub fn semantic_netlist_identity(&self) -> &str {
        &self.semantic_netlist_identity
    }

    /// Resolved HB-relevant simulation configuration identity.
    pub fn resolved_simulation_identity(&self) -> &str {
        &self.resolved_simulation_identity
    }

    /// Exact HB basis and source-transform configuration identity.
    pub fn hb_source_transform_identity(&self) -> &str {
        &self.hb_source_transform_identity
    }

    /// Canonical identity of the exact retained numerical payload.
    pub fn retained_state_identity(&self) -> &str {
        &self.retained_state_identity
    }

    fn capture(
        netlist: &Netlist,
        simulation_config: &super::SimulationConfig,
        hb_config: &HbConfig,
    ) -> Result<HbOperatingPointProducerInputs, SimulationError> {
        let semantic_netlist_identity = netlist_checkpoint_identity(netlist).ok_or_else(|| {
            SimulationError::Circuit(
                "HB producer netlist has no canonical semantic identity".to_owned(),
            )
        })?;
        Ok(HbOperatingPointProducerInputs {
            semantic_netlist_identity,
            resolved_simulation_identity: hb_resolved_simulation_identity(simulation_config),
            hb_source_transform_identity: hb_config_identity(hb_config),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn bind(
        producer: HbOperatingPointProducerInputs,
        config: &HbConfig,
        node_names: &[String],
        spectral_state: &[Vec<Complex64>],
        mna_branch_names: &[String],
        mna_branch_spectral_state: &[Vec<Complex64>],
        iterations: usize,
        residual_norm: Value,
    ) -> Self {
        Self {
            version: HB_OPERATING_POINT_IDENTITY_VERSION,
            semantic_netlist_identity: producer.semantic_netlist_identity,
            resolved_simulation_identity: producer.resolved_simulation_identity,
            hb_source_transform_identity: producer.hb_source_transform_identity,
            retained_state_identity: hb_retained_state_identity(
                config,
                node_names,
                spectral_state,
                mna_branch_names,
                mna_branch_spectral_state,
                iterations,
                residual_norm,
            ),
        }
    }

    fn validate(&self) -> Result<(), SimulationError> {
        if self.version != HB_OPERATING_POINT_IDENTITY_VERSION {
            return Err(SimulationError::Circuit(format!(
                "retained HB producer identity version {} is unsupported; expected {}",
                self.version, HB_OPERATING_POINT_IDENTITY_VERSION
            )));
        }
        for (name, value) in [
            ("semantic netlist", &self.semantic_netlist_identity),
            ("resolved simulation", &self.resolved_simulation_identity),
            ("HB source-transform", &self.hb_source_transform_identity),
            ("retained state", &self.retained_state_identity),
        ] {
            if !is_canonical_blake3_identity(value) {
                return Err(SimulationError::Circuit(format!(
                    "retained HB {name} identity is not a canonical BLAKE3 digest"
                )));
            }
        }
        Ok(())
    }
}

/// Exact converged harmonic-balance numerical state retained for dependent
/// periodic small-signal analyses.
///
/// Display spectra are not an execution contract: they may be filtered or
/// converted to magnitude/phase. This payload keeps the frozen HB basis and
/// the solver's complex Fourier coefficients in canonical node order so a
/// dependent analysis never has to re-solve or infer the large-signal state.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct HbOperatingPoint {
    config: HbConfig,
    node_names: Vec<String>,
    spectral_state: Vec<Vec<Complex64>>,
    /// Canonical circuit-MNA branch order authenticated alongside
    /// `mna_branch_spectral_state`. An empty pair is the legacy node-only
    /// representation and is never accepted for a circuit that has branches.
    #[cfg_attr(feature = "veriloga", serde(default))]
    mna_branch_names: Vec<String>,
    #[cfg_attr(feature = "veriloga", serde(default))]
    mna_branch_spectral_state: Vec<Vec<Complex64>>,
    /// Versioned semantic producer authentication. `None` is a parseable
    /// legacy artifact that must never enter a dependent numerical solve.
    #[cfg_attr(feature = "veriloga", serde(default))]
    producer_identity: Option<HbOperatingPointIdentity>,
    iterations: usize,
    residual_norm: Value,
}

impl HbOperatingPoint {
    /// Frozen HB configuration that produced this state.
    pub fn config(&self) -> &HbConfig {
        &self.config
    }

    /// Canonical non-ground node order used by every spectral row.
    pub fn node_names(&self) -> &[String] {
        &self.node_names
    }

    /// Solver Fourier coefficients indexed `[node][harmonic]`.
    pub fn spectral_state(&self) -> &[Vec<Complex64>] {
        &self.spectral_state
    }

    /// Canonical MNA branch order for the retained current spectra.
    ///
    /// An empty slice denotes a legacy node-only artifact, not evidence that
    /// an elaborated circuit has no branch unknowns. The consuming engine
    /// makes that distinction against the circuit's canonical MNA registry.
    pub fn mna_branch_names(&self) -> &[String] {
        &self.mna_branch_names
    }

    /// Solver Fourier coefficients indexed `[branch][harmonic]`, with current
    /// oriented from the authored positive terminal to the negative terminal.
    pub fn mna_branch_spectral_state(&self) -> &[Vec<Complex64>] {
        &self.mna_branch_spectral_state
    }

    /// Authenticated semantic identity minted by the producing HB engine.
    ///
    /// `None` identifies a legacy or caller-assembled artifact. Such an
    /// artifact remains inspectable for compatibility, but PAC and PNoise
    /// reject it before numerical reuse.
    pub fn producer_identity(&self) -> Option<&HbOperatingPointIdentity> {
        self.producer_identity.as_ref()
    }

    /// Number of nonlinear iterations used by the producer solve.
    pub fn iterations(&self) -> usize {
        self.iterations
    }

    /// Final producer residual norm.
    pub fn residual_norm(&self) -> Value {
        self.residual_norm
    }

    /// Highest retained positive harmonic.
    pub fn spectral_harmonic_capacity(&self) -> usize {
        self.config.num_harmonics
    }

    /// Reconstruct an HB operating point after authenticated transport.
    /// Shape, finiteness, and canonical-name checks are repeated at this
    /// boundary before the state can enter a numerical kernel.
    pub fn try_from_parts(
        config: HbConfig,
        node_names: Vec<String>,
        spectral_state: Vec<Vec<Complex64>>,
        iterations: usize,
        residual_norm: Value,
    ) -> Result<Self, SimulationError> {
        Self::try_from_parts_with_mna_branches(
            config,
            node_names,
            spectral_state,
            Vec::new(),
            Vec::new(),
            iterations,
            residual_norm,
        )
    }

    /// Reconstruct an HB operating point with authenticated circuit-MNA
    /// branch-current spectra.
    ///
    /// `mna_branch_names` and `mna_branch_spectral_state` must be in the exact
    /// canonical branch order of the elaborated circuit. The consumer repeats
    /// that circuit-specific identity check before numerical reuse.
    #[allow(clippy::too_many_arguments)]
    pub fn try_from_parts_with_mna_branches(
        config: HbConfig,
        node_names: Vec<String>,
        spectral_state: Vec<Vec<Complex64>>,
        mna_branch_names: Vec<String>,
        mna_branch_spectral_state: Vec<Vec<Complex64>>,
        iterations: usize,
        residual_norm: Value,
    ) -> Result<Self, SimulationError> {
        Self::try_from_parts_internal(
            config,
            node_names,
            spectral_state,
            mna_branch_names,
            mna_branch_spectral_state,
            iterations,
            residual_norm,
            None,
        )
    }

    /// Reconstruct a transported HB operating point together with the exact
    /// semantic producer identity that was minted by the HB engine.
    ///
    /// This validates the identity's schema and digests but does not trust a
    /// caller assertion by itself. A dependent PAC or PNoise run always
    /// compares the identity with the currently elaborated deck and resolved
    /// engine configuration immediately before numerical reuse.
    #[allow(clippy::too_many_arguments)]
    pub fn try_from_authenticated_parts_with_mna_branches(
        producer_identity: HbOperatingPointIdentity,
        config: HbConfig,
        node_names: Vec<String>,
        spectral_state: Vec<Vec<Complex64>>,
        mna_branch_names: Vec<String>,
        mna_branch_spectral_state: Vec<Vec<Complex64>>,
        iterations: usize,
        residual_norm: Value,
    ) -> Result<Self, SimulationError> {
        Self::try_from_parts_internal(
            config,
            node_names,
            spectral_state,
            mna_branch_names,
            mna_branch_spectral_state,
            iterations,
            residual_norm,
            Some(producer_identity),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn try_from_parts_internal(
        config: HbConfig,
        node_names: Vec<String>,
        spectral_state: Vec<Vec<Complex64>>,
        mna_branch_names: Vec<String>,
        mna_branch_spectral_state: Vec<Vec<Complex64>>,
        iterations: usize,
        residual_norm: Value,
        producer_identity: Option<HbOperatingPointIdentity>,
    ) -> Result<Self, SimulationError> {
        config.validate().map_err(|error| {
            SimulationError::Circuit(format!(
                "retained HB state has an invalid configuration: {error}"
            ))
        })?;
        if let Some(identity) = producer_identity.as_ref() {
            identity.validate()?;
        }
        if !residual_norm.is_finite() || residual_norm < 0.0 {
            return Err(SimulationError::Circuit(
                "retained HB state has an invalid residual norm".to_owned(),
            ));
        }
        if node_names.is_empty() || spectral_state.len() != node_names.len() {
            return Err(SimulationError::Circuit(format!(
                "retained HB state contains {} spectral row(s) for {} node name(s)",
                spectral_state.len(),
                node_names.len()
            )));
        }
        let expected_harmonics = config.num_harmonics.checked_add(1).ok_or_else(|| {
            SimulationError::Circuit("retained HB harmonic basis exceeds this platform".to_owned())
        })?;
        let mut seen = std::collections::HashSet::with_capacity(node_names.len());
        for (node, spectrum) in node_names.iter().zip(&spectral_state) {
            if node.is_empty() || node.trim() != node {
                return Err(SimulationError::Circuit(
                    "retained HB state contains a non-canonical node name".to_owned(),
                ));
            }
            if !seen.insert(node.to_ascii_uppercase()) {
                return Err(SimulationError::Circuit(format!(
                    "retained HB state contains duplicate node name '{node}'"
                )));
            }
            if spectrum.len() != expected_harmonics {
                return Err(SimulationError::Circuit(format!(
                    "retained HB node '{node}' contains {} coefficients; the frozen basis requires {expected_harmonics}",
                    spectrum.len()
                )));
            }
            if spectrum
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(SimulationError::Circuit(format!(
                    "retained HB node '{node}' contains a non-finite coefficient"
                )));
            }
            if spectrum.first().is_some_and(|value| value.im != 0.0) {
                return Err(SimulationError::Circuit(format!(
                    "retained HB node '{node}' has a nonzero imaginary DC coefficient"
                )));
            }
        }
        if mna_branch_spectral_state.len() != mna_branch_names.len() {
            return Err(SimulationError::Circuit(format!(
                "retained HB state contains {} MNA branch spectral row(s) for {} branch name(s)",
                mna_branch_spectral_state.len(),
                mna_branch_names.len()
            )));
        }
        let mut seen_branches = std::collections::HashSet::with_capacity(mna_branch_names.len());
        for (branch, spectrum) in mna_branch_names.iter().zip(&mna_branch_spectral_state) {
            if branch.is_empty() || branch.trim() != branch {
                return Err(SimulationError::Circuit(
                    "retained HB state contains a non-canonical MNA branch name".to_owned(),
                ));
            }
            if !seen_branches.insert(branch.to_ascii_uppercase()) {
                return Err(SimulationError::Circuit(format!(
                    "retained HB state contains duplicate MNA branch name '{branch}'"
                )));
            }
            if spectrum.len() != expected_harmonics {
                return Err(SimulationError::Circuit(format!(
                    "retained HB MNA branch '{branch}' contains {} coefficients; the frozen basis requires {expected_harmonics}",
                    spectrum.len()
                )));
            }
            if spectrum
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(SimulationError::Circuit(format!(
                    "retained HB MNA branch '{branch}' contains a non-finite coefficient"
                )));
            }
            if spectrum.first().is_some_and(|value| value.im != 0.0) {
                return Err(SimulationError::Circuit(format!(
                    "retained HB MNA branch '{branch}' has a nonzero imaginary DC coefficient"
                )));
            }
        }
        let point = Self {
            config,
            node_names,
            spectral_state,
            mna_branch_names,
            mna_branch_spectral_state,
            producer_identity,
            iterations,
            residual_norm,
        };
        if let Some(identity) = point.producer_identity.as_ref()
            && identity.retained_state_identity
                != hb_retained_state_identity(
                    &point.config,
                    &point.node_names,
                    &point.spectral_state,
                    &point.mna_branch_names,
                    &point.mna_branch_spectral_state,
                    point.iterations,
                    point.residual_norm,
                )
        {
            return Err(SimulationError::Circuit(
                "retained HB numerical payload does not match its authenticated producer identity"
                    .to_owned(),
            ));
        }
        Ok(point)
    }

    fn authenticate_for_reuse(
        &self,
        netlist: &Netlist,
        simulation_config: &super::SimulationConfig,
        hb_config: &HbConfig,
    ) -> Result<(), SimulationError> {
        let retained = self.producer_identity.as_ref().ok_or_else(|| {
            SimulationError::Circuit(
                "retained HB operating point is a legacy identityless artifact and is not trusted for dependent numerical reuse"
                    .to_owned(),
            )
        })?;
        retained.validate()?;
        if retained.retained_state_identity
            != hb_retained_state_identity(
                &self.config,
                &self.node_names,
                &self.spectral_state,
                &self.mna_branch_names,
                &self.mna_branch_spectral_state,
                self.iterations,
                self.residual_norm,
            )
        {
            return Err(SimulationError::Circuit(
                "retained HB numerical payload does not match its authenticated producer identity"
                    .to_owned(),
            ));
        }
        let current = HbOperatingPointIdentity::capture(netlist, simulation_config, hb_config)?;
        if retained.semantic_netlist_identity != current.semantic_netlist_identity {
            return Err(SimulationError::Circuit(
                "retained HB semantic circuit identity does not match the currently elaborated netlist"
                    .to_owned(),
            ));
        }
        if retained.resolved_simulation_identity != current.resolved_simulation_identity {
            return Err(SimulationError::Circuit(
                "retained HB resolved simulation configuration does not match the current engine configuration"
                    .to_owned(),
            ));
        }
        if retained.hb_source_transform_identity != current.hb_source_transform_identity {
            return Err(SimulationError::Circuit(
                "retained HB source-transform configuration does not match the current HB configuration"
                    .to_owned(),
            ));
        }
        Ok(())
    }

    fn to_solver_state(
        &self,
        expected_node_names: &[String],
        expected_mna_branch_names: &[String],
    ) -> Result<HbSolverState, SimulationError> {
        if self.node_names != expected_node_names {
            return Err(SimulationError::Circuit(format!(
                "retained HB node basis does not match the elaborated circuit: expected {:?}, received {:?}",
                expected_node_names, self.node_names
            )));
        }
        if self.mna_branch_names != expected_mna_branch_names {
            let detail = if self.mna_branch_names.is_empty() {
                "the retained artifact is node-only"
            } else {
                "the retained branch basis differs"
            };
            return Err(SimulationError::Circuit(format!(
                "retained HB MNA branch basis does not match the elaborated circuit ({detail}): expected {:?}, received {:?}",
                expected_mna_branch_names, self.mna_branch_names
            )));
        }
        let mut state = HbSolverState::new(self.node_names.len(), self.config.num_harmonics);
        state.x.clone_from(&self.spectral_state);
        state
            .mna_branch_currents
            .clone_from(&self.mna_branch_spectral_state);
        state.iteration = self.iterations;
        state.total_iterations = self.iterations;
        state.residual_norm = self.residual_norm;
        state.converged = true;
        Ok(state)
    }
}

/// HB-specific error types
#[derive(Debug, Clone)]
pub enum HbError {
    /// Newton iteration did not converge
    ConvergenceFailed { iterations: usize, residual: Value },
    /// Circuit has no reactive elements
    NoReactiveElements,
    /// Invalid configuration
    InvalidConfig(String),
    /// Matrix is singular
    SingularMatrix,
    /// Circuit contains nonlinear/advanced devices not yet supported by HB runtime.
    UnsupportedNonlinearDevices(String),
}

impl std::fmt::Display for HbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConvergenceFailed {
                iterations,
                residual,
            } => {
                write!(
                    f,
                    "HB convergence failed after {} iterations (residual: {:.3e})",
                    iterations, residual
                )
            }
            Self::NoReactiveElements => write!(f, "Circuit has no capacitors or inductors"),
            Self::InvalidConfig(msg) => write!(f, "Invalid HB config: {}", msg),
            Self::SingularMatrix => write!(f, "Singular admittance matrix"),
            Self::UnsupportedNonlinearDevices(summary) => {
                write!(f, "HB runtime does not yet support {}", summary)
            }
        }
    }
}

impl std::error::Error for HbError {}

impl From<HbError> for SimulationError {
    fn from(e: HbError) -> Self {
        match e {
            HbError::ConvergenceFailed { iterations, .. } => {
                SimulationError::ConvergenceFailed(iterations)
            }
            _ => SimulationError::Circuit(e.to_string()),
        }
    }
}

/// HB analysis result with detailed info
#[derive(Debug)]
pub struct HbAnalysisResult {
    /// The HB solution
    pub result: HbResult,
    /// Fundamental frequency
    pub fundamental_freq: Value,
    /// Number of harmonics
    pub num_harmonics: usize,
    /// Whether solution converged
    pub converged: bool,
    /// Exact spectral operating point consumed by HB-dependent analyses.
    pub operating_point: HbOperatingPoint,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HbDriveTone {
    harmonic: usize,
    source_filter: Option<String>,
}

#[derive(Debug, Clone)]
struct HbSourceSpectrum {
    dc: Value,
    /// Physical peak amplitudes and phases for positive harmonics. The HB
    /// solver's stamping boundary converts these to one-sided Fourier
    /// coefficients.
    harmonics: Vec<(usize, Value, Value)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HbInitialStateStrategy {
    /// Historical RSpice behavior for an omitted Xyce package: use the HB
    /// kernel's best-effort DC seed and retain its continuation fallback.
    DefaultDcSeed,
    /// Xyce `TAHB=0`: enter HB from the caller-supplied zero spectrum.
    Direct,
    /// Xyce `TAHB=1`: transform one accepted first-tone transient period.
    TransientAssisted,
    /// Xyce `TAHB=2`: repeat the exact DC operating point over the HB grid.
    DcOperatingPoint,
}

impl HbDriveTone {
    fn broadcast(harmonic: usize) -> Self {
        Self {
            harmonic,
            source_filter: None,
        }
    }

    fn matches_source(&self, source_name: &str) -> bool {
        match &self.source_filter {
            None => true,
            Some(filter) => filter.eq_ignore_ascii_case(source_name),
        }
    }
}

impl Engine {
    /// Project an authenticated shooting-PSS orbit into the HB spectral basis
    /// used by periodic small-signal kernels. This is a representation change,
    /// not an operating-point solve: every coefficient is sampled from the
    /// retained orbit and no Newton or linear large-signal solve is run.
    fn hb_state_from_pss_operating_point(
        &self,
        operating_point: &super::PssOperatingPoint,
        config: &HbConfig,
        node_names: &[String],
    ) -> Result<HbSolverState, SimulationError> {
        config.validate().map_err(|error| {
            SimulationError::Circuit(format!("dependent HB configuration is invalid: {error}"))
        })?;
        let analysis = operating_point.analysis();
        let result = &analysis.result;
        if !result.frequency.is_finite() || result.frequency <= 0.0 {
            return Err(SimulationError::Circuit(
                "periodic operating point has an invalid fundamental frequency".to_owned(),
            ));
        }
        let relative_frequency_error =
            ((result.frequency - config.fundamental_freq) / result.frequency).abs();
        if relative_frequency_error > 1.0e-9 {
            return Err(SimulationError::Circuit(format!(
                "periodic operating-point frequency {:.16e} Hz does not match the dependent analysis basis {:.16e} Hz",
                result.frequency, config.fundamental_freq
            )));
        }
        if node_names.len() != result.waveforms.len() {
            return Err(SimulationError::Circuit(format!(
                "periodic operating point contains {} node waveforms for a {}-node dependent circuit",
                result.waveforms.len(),
                node_names.len()
            )));
        }

        let fft_size = config.checked_fft_size().map_err(|error| {
            SimulationError::Circuit(format!("dependent HB collocation grid is invalid: {error}"))
        })?;
        let mut fft = HbFft::try_with_size(config.num_harmonics, fft_size).map_err(|error| {
            SimulationError::Circuit(format!("dependent HB FFT construction failed: {error}"))
        })?;
        let sample_count = fft.size();
        let mut state = HbSolverState::new(node_names.len(), config.num_harmonics);
        for (target_index, target_name) in node_names.iter().enumerate() {
            let source_index = result
                .node_names
                .iter()
                .position(|source_name| source_name.eq_ignore_ascii_case(target_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic operating point has no waveform for dependent-circuit node '{target_name}'"
                    ))
                })?;
            let waveform = &result.waveforms[source_index];
            let samples = (0..sample_count)
                .map(|sample_index| {
                    let time = analysis.period * sample_index as Value / sample_count as Value;
                    waveform.interpolate(&result.time, time, analysis.period)
                })
                .collect::<Vec<_>>();
            state.x[target_index] = fft.to_frequency_domain(&samples);
        }
        state.iteration = analysis.iterations.max(1);
        state.total_iterations = analysis.iterations;
        state.residual_norm = analysis.final_residual;
        state.converged = true;
        Ok(state)
    }

    /// Run Harmonic Balance analysis
    ///
    /// This is the main entry point for HB simulation. It builds the circuit,
    /// extracts admittance matrices, and solves for spectral coefficients.
    ///
    /// # Arguments
    /// * `netlist` - The circuit netlist
    /// * `config` - HB analysis configuration
    ///
    /// # Returns
    /// * `Ok(HbAnalysisResult)` - Successful analysis with spectral voltages
    /// * `Err(SimulationError)` - Analysis failed
    ///
    /// # Example
    /// ```ignore
    /// use rspice_core::{Engine, Netlist};
    /// use rspice_core::analysis::HbConfig;
    ///
    /// let netlist = Netlist::parse("...")?;
    /// let engine = Engine::default();
    /// let config = HbConfig::new(1e9).with_harmonics(9);
    /// let result = engine.run_hb(&netlist, config)?;
    /// ```
    pub fn run_hb(
        &self,
        netlist: &Netlist,
        config: HbConfig,
    ) -> Result<HbAnalysisResult, SimulationError> {
        self.run_hb_with_abort(netlist, config, &NoAbort)
    }

    /// Run harmonic balance with cooperative cancellation.
    pub fn run_hb_with_abort(
        &self,
        netlist: &Netlist,
        config: HbConfig,
        abort: &dyn AbortSignal,
    ) -> Result<HbAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        let config = engine.hb_config_for_netlist(netlist, config)?;
        engine.hb_validate_config(&config)?;
        let producer_identity =
            HbOperatingPointIdentity::capture(netlist, &engine.config, &config)?;

        // Build circuit using SoA architecture
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::ensure_no_mixed_signal_analysis(&circuit, "harmonic-balance analysis")?;
        if producer_identity != HbOperatingPointIdentity::capture(netlist, &engine.config, &config)?
        {
            return Err(SimulationError::Circuit(
                "HB semantic producer inputs changed while the circuit was being elaborated"
                    .to_owned(),
            ));
        }
        engine.run_hb_with_prebuilt_circuit_abort(
            netlist,
            circuit,
            config,
            Some(producer_identity),
            abort,
        )
    }

    /// Apply analysis-local options authored for Xyce's HB packages.
    ///
    /// This deliberately returns a derived `HbConfig` instead of modifying
    /// `Engine::config`: `NONLIN-HB MAXSTEP` is a Newton limit for the HB
    /// nonlinear system and must never leak into DC, transient, or PSS.
    pub(super) fn hb_config_for_netlist(
        &self,
        netlist: &Netlist,
        mut config: HbConfig,
    ) -> Result<HbConfig, SimulationError> {
        Self::hb_dc_seed_policy(netlist)?;
        if let Some(maxstep) = netlist.options.nonlin_hb_maxstep {
            if maxstep == 0 {
                return Err(HbError::InvalidConfig(
                    ".OPTIONS NONLIN-HB MAXSTEP must be at least 1".to_string(),
                )
                .into());
            }
            config.max_iterations = maxstep;
        }
        Ok(config)
    }

    fn hb_initial_state_strategy(netlist: &Netlist) -> HbInitialStateStrategy {
        let Some(mode) = netlist.options.hb_time_domain_mode else {
            return HbInitialStateStrategy::DefaultDcSeed;
        };
        match mode.xyce_value() {
            0 => HbInitialStateStrategy::Direct,
            1 => HbInitialStateStrategy::TransientAssisted,
            2 => HbInitialStateStrategy::DcOperatingPoint,
            _ => unreachable!("typed Xyce TAHB mode has no accepted integer spelling"),
        }
    }

    fn hb_dc_seed_policy(netlist: &Netlist) -> Result<HbDcSeedPolicy, SimulationError> {
        Ok(match Self::hb_initial_state_strategy(netlist) {
            HbInitialStateStrategy::DefaultDcSeed => HbDcSeedPolicy::Enabled,
            HbInitialStateStrategy::Direct
            | HbInitialStateStrategy::TransientAssisted
            | HbInitialStateStrategy::DcOperatingPoint => HbDcSeedPolicy::Disabled,
        })
    }

    fn hb_seed_dc_operating_point(
        &self,
        netlist: &Netlist,
        state: &mut HbSolverState,
        node_names: &[String],
        branch_names: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let dc = self
            .run_dc_op_with_abort(netlist, abort)
            .map_err(|error| match error {
                SimulationError::Aborted => SimulationError::Aborted,
                other => SimulationError::Circuit(format!(
                    "TAHB=2 DC operating-point initial-state construction failed: {other}"
                )),
            })?;
        for (node_index, node_name) in node_names.iter().enumerate() {
            let source_index = dc
                .node_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(node_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "TAHB=2 DC operating point has no value for HB node '{node_name}'"
                    ))
                })?;
            let value = dc.node_voltages.get(source_index).copied().ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "TAHB=2 DC operating point lost the value for HB node '{node_name}'"
                ))
            })?;
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "TAHB=2 DC operating point produced a non-finite value for HB node '{node_name}'"
                )));
            }
            state.x[node_index][0] = Complex64::new(value, 0.0);
        }
        for (branch_index, branch_name) in branch_names.iter().enumerate() {
            let source_index = dc
                .branch_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(branch_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "TAHB=2 cannot represent periodic branch '{branch_name}' from the retained DC operating point"
                    ))
                })?;
            let value = dc
                .branch_currents
                .get(source_index)
                .copied()
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "TAHB=2 DC operating point lost periodic branch '{branch_name}'"
                    ))
                })?;
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "TAHB=2 DC operating point produced a non-finite current for periodic branch '{branch_name}'"
                )));
            }
            state.mna_branch_currents[branch_index][0] = Complex64::new(value, 0.0);
        }
        Ok(())
    }

    fn hb_interpolate_transient_value(
        times: &[Value],
        values: &[Value],
        time: Value,
        context: &str,
    ) -> Result<Value, SimulationError> {
        if times.len() < 2 || values.len() != times.len() {
            return Err(SimulationError::Circuit(format!(
                "{context} has {} values on a {}-point transient grid",
                values.len(),
                times.len()
            )));
        }
        if !time.is_finite()
            || times
                .windows(2)
                .any(|pair| !pair[0].is_finite() || pair[0] >= pair[1])
            || !times.last().copied().unwrap_or(Value::NAN).is_finite()
        {
            return Err(SimulationError::Circuit(format!(
                "{context} has an invalid interpolation grid"
            )));
        }
        match times.binary_search_by(|candidate| candidate.total_cmp(&time)) {
            Ok(index) => Ok(values[index]),
            Err(0) => Err(SimulationError::Circuit(format!(
                "{context} does not cover requested time {time:.16e} s"
            ))),
            Err(index) if index >= times.len() => Err(SimulationError::Circuit(format!(
                "{context} does not cover requested time {time:.16e} s"
            ))),
            Err(index) => {
                let t0 = times[index - 1];
                let t1 = times[index];
                let fraction = (time - t0) / (t1 - t0);
                let value = values[index - 1] + fraction * (values[index] - values[index - 1]);
                if value.is_finite() {
                    Ok(value)
                } else {
                    Err(SimulationError::Circuit(format!(
                        "{context} interpolation produced a non-finite value"
                    )))
                }
            }
        }
    }

    fn hb_seed_transient_assisted(
        &self,
        netlist: &Netlist,
        config: &HbConfig,
        state: &mut HbSolverState,
        node_names: &[String],
        branch_names: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let first_tone_frequency = config
            .tones
            .first()
            .map(|tone| tone.frequency)
            .unwrap_or(config.fundamental_freq);
        if !first_tone_frequency.is_finite() || first_tone_frequency <= 0.0 {
            return Err(HbError::InvalidConfig(
                "TAHB=1 first authored tone must be finite and positive".to_string(),
            )
            .into());
        }
        let transient_period = first_tone_frequency.recip();
        let collocation_points = config
            .checked_fft_size()
            .map_err(|error| HbError::InvalidConfig(error.to_string()))?;
        let integration_intervals = collocation_points.checked_mul(4).ok_or_else(|| {
            HbError::InvalidConfig(
                "TAHB=1 transient integration-point count exceeds this platform".to_string(),
            )
        })?;
        self.ensure_analysis_points(integration_intervals.saturating_add(1))?;
        let max_step = transient_period / integration_intervals as Value;
        if !transient_period.is_finite() || !max_step.is_finite() || max_step <= 0.0 {
            return Err(HbError::InvalidConfig(
                "TAHB=1 first-tone transient period or timestep is not representable".to_string(),
            )
            .into());
        }
        let transient = self
            .run_tran_with_startup_mode_and_abort(
                netlist,
                transient_period,
                max_step,
                TransientStartupMode::OperatingPoint,
                abort,
            )
            .map_err(|error| match error {
                SimulationError::Aborted => SimulationError::Aborted,
                other => SimulationError::Circuit(format!(
                    "TAHB=1 first-tone transient initial-state construction failed: {other}"
                )),
            })?;
        let hb_period = config.fundamental_freq.recip();
        let sample_times = (0..collocation_points)
            .map(|sample| {
                (hb_period * sample as Value / collocation_points as Value)
                    .rem_euclid(transient_period)
            })
            .collect::<Vec<_>>();
        let mut fft = HbFft::try_with_size(config.num_harmonics, collocation_points)
            .map_err(|error| HbError::InvalidConfig(error.to_string()))?;

        for (node_index, node_name) in node_names.iter().enumerate() {
            let source_index = transient
                .node_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(node_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "TAHB=1 transient has no retained waveform for HB node '{node_name}'"
                    ))
                })?;
            let waveform = transient.voltages.get(source_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "TAHB=1 transient lost the waveform for HB node '{node_name}'"
                ))
            })?;
            let samples = sample_times
                .iter()
                .map(|time| {
                    Self::hb_interpolate_transient_value(
                        &transient.time,
                        waveform,
                        *time,
                        &format!("TAHB=1 node '{node_name}'"),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            state.x[node_index] = fft.to_frequency_domain(&samples);
        }
        for (branch_index, branch_name) in branch_names.iter().enumerate() {
            let source_index = transient
                .branch_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(branch_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "TAHB=1 cannot represent periodic branch '{branch_name}' from the retained transient state"
                    ))
                })?;
            let waveform = transient.branch_currents.get(source_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "TAHB=1 transient lost periodic branch '{branch_name}'"
                ))
            })?;
            let samples = sample_times
                .iter()
                .map(|time| {
                    Self::hb_interpolate_transient_value(
                        &transient.time,
                        waveform,
                        *time,
                        &format!("TAHB=1 branch '{branch_name}'"),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            state.mna_branch_currents[branch_index] = fft.to_frequency_domain(&samples);
        }
        Ok(())
    }

    fn hb_validate_config(&self, config: &HbConfig) -> Result<(), SimulationError> {
        config
            .validate()
            .map_err(|error| HbError::InvalidConfig(error.to_string()))?;
        let fft_size = config
            .checked_fft_size()
            .map_err(|error| HbError::InvalidConfig(error.to_string()))?;
        let spectral_components = config.num_harmonics.checked_add(1).ok_or_else(|| {
            HbError::InvalidConfig("num_harmonics exceeds the addressable spectrum".to_owned())
        })?;
        self.ensure_analysis_points(fft_size)?;
        self.ensure_analysis_points(spectral_components)?;
        Ok(())
    }

    /// Solve an already elaborated circuit. HB-specific clients use this
    /// boundary when they must authenticate a source transformation before
    /// the periodic solve; ordinary callers always enter through
    /// [`Self::run_hb_with_abort`].
    fn run_hb_with_prebuilt_circuit_abort(
        &self,
        netlist: &Netlist,
        circuit: CircuitData,
        config: HbConfig,
        producer_inputs: Option<HbOperatingPointProducerInputs>,
        abort: &dyn AbortSignal,
    ) -> Result<HbAnalysisResult, SimulationError> {
        // Get node count (excluding ground)
        let num_nodes = circuit.num_nodes();
        if num_nodes == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }

        // No reactive-element gate: junction devices carry their own charge
        // storage, and HB on a resistive nonlinear circuit is legitimate
        // distortion analysis (every harmonic system is solvable regardless).
        if let Some(summary) =
            periodic_capability::summarize(&periodic_capability::periodic_residual_gaps(&circuit))
        {
            return Err(HbError::UnsupportedNonlinearDevices(summary).into());
        }
        let has_supported_nonlinear =
            periodic_capability::has_exact_periodic_nonlinear_devices(&circuit);
        if let Some(summary) =
            periodic_capability::summarize(&periodic_capability::periodic_descriptor_gaps(&circuit))
        {
            return Err(SimulationError::Circuit(format!(
                "exact HB MNA is unavailable because the circuit contains {summary}"
            )));
        }
        let periodic_branches = circuit
            .num_branches()
            .checked_add(Self::hb_periodic_extra_branch_count(&circuit)?)
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "HB canonical and distributed-network branch count overflows this platform"
                        .to_string(),
                )
            })?;
        let mna_unknowns = num_nodes.checked_add(periodic_branches).ok_or_else(|| {
            SimulationError::Circuit(
                "HB node and canonical branch count overflows this platform".to_string(),
            )
        })?;
        let one_sided_scalar_coordinates = config
            .num_harmonics
            .checked_mul(2)
            .and_then(|count| count.checked_add(1))
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "HB realified spectral coordinate count overflows this platform".to_string(),
                )
            })?;
        let matrix_unknowns = mna_unknowns
            .checked_mul(one_sided_scalar_coordinates)
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "HB realified MNA dimension overflows this platform".to_string(),
                )
            })?;
        self.ensure_matrix_unknowns(matrix_unknowns)?;
        let retained_complex_values = config
            .num_harmonics
            .checked_add(1)
            .and_then(|harmonics| harmonics.checked_mul(mna_unknowns))
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "HB retained complex-value count overflows this platform".to_string(),
                )
            })?;
        let retained_scalar_values = retained_complex_values.checked_mul(2).ok_or_else(|| {
            SimulationError::Circuit(
                "HB retained scalar-value count overflows this platform".to_string(),
            )
        })?;
        self.ensure_result_values(retained_scalar_values)?;
        let initial_state_strategy = Self::hb_initial_state_strategy(netlist);
        let dc_seed_policy = Self::hb_dc_seed_policy(netlist)?;
        let drive_tones = Self::hb_collect_drive_tones(&config)?;
        Self::hb_validate_drive_tone_sources(&circuit, &drive_tones)?;

        // Create solver
        let mut solver = HbSolver::try_new(config.clone(), num_nodes).map_err(|error| {
            SimulationError::Circuit(format!("HB solver construction failed: {error}"))
        })?;

        // Set node names from circuit's node map
        let node_names = self.hb_build_node_names(&circuit, num_nodes);
        solver.set_node_names(node_names.clone());

        // Stamp linear circuit elements into HB solver
        self.hb_stamp_resistors(&circuit, &mut solver);
        self.hb_stamp_capacitors(&circuit, &mut solver);
        self.hb_stamp_voltage_sources(&circuit, &mut solver, &config, &drive_tones)?;
        self.hb_stamp_periodic_mna_branches(&circuit, &mut solver)?;
        self.hb_stamp_current_sources(&circuit, &mut solver, &config, &drive_tones)?;
        if has_supported_nonlinear {
            self.hb_stamp_supported_nonlinear_devices(&circuit, &mut solver, num_nodes);
        }
        let periodic_branch_names = solver.try_periodic_mna_branch_names().map_err(|error| {
            SimulationError::Circuit(format!(
                "HB initial-state branch metadata construction failed: {error}"
            ))
        })?;

        // Create solver state
        let mut state = HbSolverState::new(num_nodes, config.num_harmonics);
        state
            .try_prepare_mna_branches(solver.exact_mna_branches().len(), config.num_harmonics)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "HB canonical MNA state allocation failed: {error}"
                ))
            })?;
        let retained_state_values = state.try_total_unknowns().map_err(|error| {
            SimulationError::Circuit(format!("HB state qualification failed: {error}"))
        })?;
        let expected_retained_state_values = mna_unknowns
            .checked_mul(config.num_harmonics + 1)
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "HB retained state dimension exceeds this platform".to_string(),
                )
            })?;
        if retained_state_values != expected_retained_state_values {
            return Err(SimulationError::Circuit(format!(
                "HB retained state contains {retained_state_values} complex coordinates; resource qualification authorized {expected_retained_state_values}"
            )));
        }

        // Construct the authored initial trajectory before Newton.  Explicit
        // Xyce modes are strict: TAHB=1/2 either produce the complete retained
        // node/branch spectrum or fail closed.  Only the omitted historical
        // RSpice policy keeps its best-effort DC fallback.
        match initial_state_strategy {
            HbInitialStateStrategy::TransientAssisted => self.hb_seed_transient_assisted(
                netlist,
                &config,
                &mut state,
                &node_names,
                &periodic_branch_names,
                abort,
            )?,
            HbInitialStateStrategy::DcOperatingPoint => self.hb_seed_dc_operating_point(
                netlist,
                &mut state,
                &node_names,
                &periodic_branch_names,
                abort,
            )?,
            HbInitialStateStrategy::DefaultDcSeed if has_supported_nonlinear => {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                match self.run_dc_op_with_abort(netlist, abort) {
                    Ok(dc) => {
                        for node in 0..num_nodes {
                            if node < state.x.len() && !state.x[node].is_empty() {
                                let v = dc.node_voltages.get(node + 1).copied().unwrap_or(0.0);
                                state.x[node][0] = Complex64::new(v, 0.0);
                            }
                        }
                    }
                    Err(err) => {
                        log::warn!(
                            "HB: DC operating point for the harmonic-0 seed failed ({err}); \
                             starting from zero"
                        );
                    }
                }
            }
            HbInitialStateStrategy::DefaultDcSeed | HbInitialStateStrategy::Direct => {}
        }

        if has_supported_nonlinear {
            solver
                .solve_newton_with_abort_seed_policy(&mut state, abort, dc_seed_policy)
                .map_err(|e| match e {
                    crate::analysis::HbError::Aborted => SimulationError::Aborted,
                    crate::analysis::HbError::ConvergenceFailed {
                        iterations,
                        residual,
                    } => HbError::ConvergenceFailed {
                        iterations,
                        residual,
                    }
                    .into(),
                    other => {
                        SimulationError::Circuit(format!("HB nonlinear solve failed: {}", other))
                    }
                })?;
        } else {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            // Solve linear HB system
            solver
                .solve_linear(&mut state)
                .map_err(|error| match error {
                    crate::analysis::HbError::Aborted => SimulationError::Aborted,
                    crate::analysis::HbError::ConvergenceFailed {
                        iterations,
                        residual,
                    } => HbError::ConvergenceFailed {
                        iterations,
                        residual,
                    }
                    .into(),
                    crate::analysis::HbError::SingularMatrix => HbError::SingularMatrix.into(),
                    other => SimulationError::Circuit(format!("HB linear solve failed: {other}")),
                })?;
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
        }

        // Build result
        let mut result = solver.build_result(&state).map_err(|error| {
            SimulationError::Circuit(format!("HB result construction failed: {error}"))
        })?;
        self.hb_attach_periodic_state(&circuit, &mut result)?;

        let mna_branch_names = periodic_branch_names;
        let operating_point = if let Some(producer) = producer_inputs {
            if producer != HbOperatingPointIdentity::capture(netlist, &self.config, &config)? {
                return Err(SimulationError::Circuit(
                    "HB semantic producer inputs changed during the periodic solve".to_owned(),
                ));
            }
            let identity = HbOperatingPointIdentity::bind(
                producer,
                &config,
                &result.node_names,
                &state.x,
                &mna_branch_names,
                &state.mna_branch_currents,
                state.total_iterations.max(state.iteration),
                state.residual_norm,
            );
            HbOperatingPoint::try_from_authenticated_parts_with_mna_branches(
                identity,
                config.clone(),
                result.node_names.clone(),
                state.x.clone(),
                mna_branch_names,
                state.mna_branch_currents.clone(),
                state.total_iterations.max(state.iteration),
                state.residual_norm,
            )?
        } else {
            HbOperatingPoint::try_from_parts_with_mna_branches(
                config.clone(),
                result.node_names.clone(),
                state.x.clone(),
                mna_branch_names,
                state.mna_branch_currents.clone(),
                state.total_iterations.max(state.iteration),
                state.residual_norm,
            )?
        };
        Ok(HbAnalysisResult {
            result,
            fundamental_freq: config.fundamental_freq,
            num_harmonics: config.num_harmonics,
            converged: state.converged,
            operating_point,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SimulationConfig, SpiceDialect};

    #[test]
    fn authored_nonlin_hb_budget_changes_only_the_derived_hb_config() {
        let netlist = Netlist::parse(
            "typed HB runtime options\n\
             V1 out 0 1\n\
             R1 out 0 1k\n\
             .options hbint tahb=0\n\
             .options nonlin-hb maxstep=2\n\
             .hb 1k\n\
             .end\n",
        )
        .expect("typed HB deck parses");
        let mut simulation = SimulationConfig::default();
        simulation.max_iterations = 37;
        let engine = Engine::new(simulation);
        let caller_config = HbConfig::new(1.0e3).with_max_iterations(19);

        let effective = engine
            .hb_config_for_netlist(&netlist, caller_config.clone())
            .expect("TAHB=0 and NONLIN-HB MAXSTEP are supported");

        assert_eq!(
            Engine::hb_dc_seed_policy(&netlist).expect("direct policy resolves"),
            HbDcSeedPolicy::Disabled
        );
        assert_eq!(effective.max_iterations, 2);
        assert_eq!(caller_config.max_iterations, 19);
        assert_eq!(engine.config.max_iterations, 37);

        let analysis = engine
            .run_hb(&netlist, caller_config)
            .expect("typed direct HB options run through the production entry point");
        assert_eq!(analysis.operating_point.config().max_iterations, 2);

        let without_authored_budget =
            Netlist::parse("caller HB budget\nV1 out 0 1\nR1 out 0 1k\n.end\n")
                .expect("base deck parses");
        let unchanged = engine
            .hb_config_for_netlist(
                &without_authored_budget,
                HbConfig::new(1.0e3).with_max_iterations(19),
            )
            .expect("an omitted package leaves the caller's HB budget intact");
        assert_eq!(
            Engine::hb_dc_seed_policy(&without_authored_budget)
                .expect("omitted TAHB policy resolves"),
            HbDcSeedPolicy::Enabled
        );
        assert_eq!(unchanged.max_iterations, 19);
    }

    #[test]
    fn explicit_tahb_direct_runs_nonlinear_hb_without_a_dc_seed_policy() {
        let netlist = Netlist::parse(
            "direct nonlinear HB\n\
             V1 in 0 SIN(0 0.01 1k)\n\
             R1 in out 1k\n\
             D1 out 0 DMOD\n\
             C1 out 0 1n\n\
             .model DMOD D IS=1e-14\n\
             .options hbint tahb=0\n\
             .hb 1k\n\
             .end\n",
        )
        .expect("direct nonlinear HB deck parses");
        assert_eq!(
            Engine::hb_dc_seed_policy(&netlist).expect("direct policy resolves"),
            HbDcSeedPolicy::Disabled
        );

        let analysis = Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(1.0e3).with_harmonics(3))
            .expect("direct frequency-domain nonlinear HB converges from the supplied zero state");
        assert!(analysis.converged);
        assert!(analysis.result.is_valid());
        assert!(
            analysis.operating_point.iterations() > 0,
            "direct nonlinear HB must execute Newton rather than publishing its zero initializer"
        );
        let input = analysis
            .result
            .spectral_voltages
            .iter()
            .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case("in"))
            .expect("driven input spectrum is retained");
        assert!(
            input.coefficients[1].norm() > 4.0e-3,
            "direct nonlinear HB lost its exact periodic source constraint: {:?}",
            input.coefficients
        );
    }

    #[test]
    fn tahb_initializers_construct_exact_frequency_domain_seed_shapes() {
        let transient_netlist = Netlist::parse(
            "transient-assisted seed\n\
             V1 in 0 SIN(0 1 2k)\n\
             R1 in 0 1k\n\
             .options hbint tahb=1\n\
             .hb 2k 3k\n\
             .end\n",
        )
        .expect("TAHB=1 deck parses");
        let config = HbConfig::multi_tone(vec![
            crate::analysis::harmonic_balance::HbTone::new(2.0e3, 2),
            crate::analysis::harmonic_balance::HbTone::new(3.0e3, 2),
        ])
        .with_collocation_points(13);
        let engine = Engine::new(SimulationConfig::default());
        let mut transient_seed = HbSolverState::new(1, config.num_harmonics);
        transient_seed
            .try_prepare_mna_branches(1, config.num_harmonics)
            .expect("one ideal-source branch fits the qualified seed");
        engine
            .hb_seed_transient_assisted(
                &transient_netlist,
                &config,
                &mut transient_seed,
                &["in".to_string()],
                &["V1".to_string()],
                &NoAbort,
            )
            .expect("TAHB=1 transforms the accepted first-tone trajectory");
        // Independent closed-form oracle. The ideal source fixes
        // v(in)=sin(2*pi*2 kHz*t), whose one-sided Fourier coefficient on the
        // 1 kHz common grid is -j/2 at k=2. KCL through the 1 kohm resistor
        // fixes the source current to -v(in)/1 kohm, hence +j/2000 at k=2.
        // This pins both retained coordinate families without asking another
        // simulator or another RSpice analysis for the expected values.
        let expected_voltage = Complex64::new(0.0, -0.5);
        let expected_source_current = Complex64::new(0.0, 0.5e-3);
        // The initializer deliberately samples the accepted transient through
        // piecewise-linear interpolation. For a sine, the interpolation error
        // is bounded by max(|v''|)*h^2/8. A normalized DFT coefficient cannot
        // exceed the largest sample error, so this is an analytic bound on
        // both the wanted coefficient error and leakage into every other bin.
        let angular_frequency = std::f64::consts::TAU * 2.0e3;
        let maximum_step = 0.5e-3 / (13 * 4) as Value;
        let interpolation_bound = angular_frequency.powi(2) * maximum_step.powi(2) / 8.0;
        let voltage_tolerance = interpolation_bound + 1.0e-10;
        let current_tolerance = interpolation_bound / 1.0e3 + 1.0e-12;
        assert!(
            (transient_seed.x[0][2] - expected_voltage).norm() < voltage_tolerance,
            "TAHB=1 node spectrum differs from the sine oracle: expected {expected_voltage:?}, got {:?}",
            transient_seed.x[0][2]
        );
        assert!(
            (transient_seed.mna_branch_currents[0][2] - expected_source_current).norm()
                < current_tolerance,
            "TAHB=1 branch spectrum differs from the KCL oracle: expected {expected_source_current:?}, got {:?}",
            transient_seed.mna_branch_currents[0][2]
        );
        assert!(
            transient_seed.x[0][1].norm() < 1.0e-6,
            "first-tone trajectory leaked onto the common-grid fundamental: {:?}",
            transient_seed.x[0]
        );
        for (harmonic, coefficient) in transient_seed.x[0].iter().enumerate() {
            if harmonic != 2 {
                assert!(
                    coefficient.norm() < voltage_tolerance,
                    "TAHB=1 node seed has non-oracle harmonic {harmonic}: {coefficient:?}"
                );
            }
        }
        for (harmonic, coefficient) in transient_seed.mna_branch_currents[0].iter().enumerate() {
            if harmonic != 2 {
                assert!(
                    coefficient.norm() < current_tolerance,
                    "TAHB=1 branch seed has non-oracle harmonic {harmonic}: {coefficient:?}"
                );
            }
        }

        let dc_netlist = Netlist::parse(
            "DC trajectory seed\nV1 out 0 1.25\nR1 out 0 1k\n.options hbint tahb=2\n.hb 1k\n.end\n",
        )
        .expect("TAHB=2 deck parses");
        let mut dc_seed = HbSolverState::new(1, 3);
        dc_seed
            .try_prepare_mna_branches(1, 3)
            .expect("one ideal-source branch fits the qualified seed");
        engine
            .hb_seed_dc_operating_point(
                &dc_netlist,
                &mut dc_seed,
                &["out".to_string()],
                &["V1".to_string()],
                &NoAbort,
            )
            .expect("TAHB=2 repeats the converged DC point");
        assert!((dc_seed.x[0][0].re - 1.25).abs() < 1.0e-12);
        assert_eq!(dc_seed.x[0][0].im, 0.0);
        assert!(
            dc_seed.x[0][1..]
                .iter()
                .all(|value| *value == Complex64::new(0.0, 0.0))
        );
        // KCL independently fixes i(V1)=-1.25 V / 1 kohm=-1.25 mA. A
        // constant trajectory has no non-DC coefficients.
        assert!(
            (dc_seed.mna_branch_currents[0][0] - Complex64::new(-1.25e-3, 0.0)).norm() < 1.0e-12,
            "TAHB=2 branch spectrum differs from the DC KCL oracle: {:?}",
            dc_seed.mna_branch_currents[0][0]
        );
        assert!(
            dc_seed.mna_branch_currents[0][1..]
                .iter()
                .all(|value| *value == Complex64::new(0.0, 0.0))
        );
    }

    #[test]
    fn parsed_xyce_multitone_pulse_reaches_the_engine_apft_path() {
        let netlist = Netlist::parse(
            "parsed APFT integration\n\
             VDRIVE in 0 PULSE(-0.2 0.8 17n 23n 31n 400n 500n)\n\
             R1 in 0 1k\n\
             .options hbint numfreq=2 numfreq2=2 tahb=1\n\
             .hb 2Meg 3Meg\n\
             .end\n",
        )
        .expect("typed multi-tone Xyce deck parses");
        let frequencies = match &netlist.analyses[0] {
            crate::netlist::AnalysisCommand::Hb { frequencies } => frequencies,
            other => panic!("expected parsed HB command, got {other:?}"),
        };
        assert_eq!(frequencies, &[2.0e6, 3.0e6]);
        assert_eq!(netlist.options.hb_num_frequencies, [2, 2]);
        assert_eq!(
            netlist.options.hb_time_domain_mode,
            Some(crate::netlist::XyceHbTimeDomainMode::TransientAssisted)
        );
        let config = HbConfig::multi_tone(
            frequencies
                .iter()
                .zip(&netlist.options.hb_num_frequencies)
                .map(|(frequency, order)| {
                    crate::analysis::harmonic_balance::HbTone::new(*frequency, *order)
                })
                .collect(),
        )
        .with_collocation_points(13);
        let mut simulation = SimulationConfig::default();
        simulation.spice_dialect = SpiceDialect::Xyce;
        let analysis = Engine::new(simulation)
            .run_hb(&netlist, config)
            .expect("parser-authored TAHB/APFT deck runs through the production engine");
        let input = analysis
            .result
            .spectral_voltages
            .iter()
            .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case("in"))
            .expect("input spectrum is retained");
        assert!(input.coefficients[2].norm() > 0.1);
        assert!(input.coefficients.iter().all(|value| value.is_finite()));
    }

    #[test]
    fn retained_inductor_state_requires_a_complete_exact_mna_spectrum() {
        let netlist = Netlist::parse(
            "retained exact inductor state\n\
             V1 in 0 DC 1\n\
             R1 in out 1k\n\
             LSTATE out 0 1m\n\
             .end\n",
        )
        .expect("exact inductor deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let exact = engine
            .run_hb(&netlist, HbConfig::new(1.0e3).with_harmonics(2))
            .expect("exact branch-aware HB converges")
            .result;

        let mut missing = exact.clone();
        missing.reactive_spectra.clear();
        missing
            .mna_branch_currents
            .retain(|branch| !branch.device_name.eq_ignore_ascii_case("LSTATE"));
        let error = engine
            .hb_attach_periodic_state(&circuit, &mut missing)
            .expect_err("missing exact inductor branch state must fail closed");
        assert!(
            error
                .to_string()
                .contains("lost exact MNA current spectrum")
        );

        let mut malformed = exact;
        malformed.reactive_spectra.clear();
        malformed
            .mna_branch_currents
            .iter_mut()
            .find(|branch| branch.device_name.eq_ignore_ascii_case("LSTATE"))
            .expect("exact inductor branch is retained")
            .coefficients
            .pop();
        let error = engine
            .hb_attach_periodic_state(&circuit, &mut malformed)
            .expect_err("truncated exact inductor branch state must fail closed");
        assert!(error.to_string().contains("is malformed"));
    }

    #[test]
    fn explicit_tahb_modes_are_hb_local_and_execute_the_authored_initializer() {
        for mode in [1, 2] {
            let netlist = Netlist::parse(&format!(
                "supported TAHB mode\nV1 out 0 PULSE(0 1 0 10u 10u 480u 1m)\nR1 out 0 1k\n.options hbint tahb={mode}\n.options nonlin-hb maxstep=2\n.hb 1k\n.end\n"
            ))
            .expect("known Xyce TAHB modes remain typed");
            let engine = Engine::new(SimulationConfig::default());

            let dc = engine
                .run_dc_op(&netlist)
                .expect("HB-local options must not affect DC");
            assert!(dc.node_voltages[1].abs() < 1.0e-12);

            assert_eq!(
                Engine::hb_dc_seed_policy(&netlist).expect("typed policy resolves"),
                HbDcSeedPolicy::Disabled,
                "an explicit initializer must not be overwritten by the legacy kernel DC seed"
            );
            let analysis = engine
                .run_hb(&netlist, HbConfig::new(1.0e3).with_harmonics(3))
                .unwrap_or_else(|error| panic!("TAHB={mode} failed: {error}"));
            assert!(analysis.converged);
            assert!(analysis.result.is_valid());
        }
    }

    #[test]
    fn manually_constructed_zero_nonlin_hb_budget_fails_closed() {
        let mut netlist = Netlist::parse("invalid HB budget\nV1 1 0 1\nR1 1 0 1k\n.end\n")
            .expect("base deck parses");
        netlist.options.nonlin_hb_maxstep = Some(0);
        let error = Engine::new(SimulationConfig::default())
            .hb_config_for_netlist(&netlist, HbConfig::new(1.0e3))
            .expect_err("invalid typed AST must be rejected at the runtime boundary");
        assert!(error.to_string().contains("must be at least 1"));
    }

    #[test]
    fn pulse_source_coefficients_are_analytic_and_grid_invariant() {
        let minimal_grid = HbConfig::new(10.0e3)
            .with_harmonics(50)
            .with_collocation_points(101);
        let oversized_grid = minimal_grid.clone().with_collocation_points(401);
        let pulse = SourceSpec::Pulse {
            v1: 1.0,
            v2: 2.0,
            delay: 1.234_567e-6,
            rise: 10.0e-6,
            fall: 10.0e-6,
            width: 40.0e-6,
            period: 100.0e-6,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };
        let spectrum = Engine::hb_source_spectrum(
            1.0,
            0.0,
            0.0,
            Some(&pulse),
            &minimal_grid,
            &[1],
            SpiceDialect::BestAvailable,
        )
        .expect("periodic pulse spectrum");
        let oversized = Engine::hb_source_spectrum(
            1.0,
            0.0,
            0.0,
            Some(&pulse),
            &oversized_grid,
            &[1],
            SpiceDialect::BestAvailable,
        )
        .expect("same pulse on an oversized collocation grid");

        assert_eq!(spectrum.dc, oversized.dc);
        assert_eq!(spectrum.harmonics, oversized.harmonics);
        assert!((spectrum.dc - 1.5).abs() < 1.0e-15);
        let (_, h1_amplitude, h1_phase) = spectrum.harmonics[0];
        let h1 = Complex64::from_polar(h1_amplitude, h1_phase);
        let envelope = (2.0 / std::f64::consts::PI) * (std::f64::consts::PI / 10.0).sin()
            / (std::f64::consts::PI / 10.0);
        let center = 30.0e-6 + 1.234_567e-6;
        let expected_h1 =
            Complex64::from_polar(envelope, -std::f64::consts::TAU * center / 100.0e-6);
        assert!(
            (h1 - expected_h1).norm() < 1.0e-14,
            "h1={h1:?}, expected={expected_h1:?}"
        );
        let h2_norm = spectrum
            .harmonics
            .iter()
            .find(|(harmonic, _, _)| *harmonic == 2)
            .map(|(_, amplitude, phase)| Complex64::from_polar(*amplitude, *phase))
            .map_or(0.0, |phasor| phasor.norm());
        assert!(h2_norm < 1.0e-14, "symmetric trapezoid h2={h2_norm}");
    }

    #[test]
    fn xyce_pulse_source_uses_the_requested_collocation_projection() {
        let config = HbConfig::new(1.0e6)
            .with_harmonics(101)
            .with_collocation_points(203);
        let pulse = SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 0.0,
            rise: 5.0e-9,
            fall: 5.0e-9,
            width: 0.49e-6,
            period: 1.0e-6,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };
        let spectrum = Engine::hb_source_spectrum(
            0.0,
            0.0,
            0.0,
            Some(&pulse),
            &config,
            &[1],
            SpiceDialect::Xyce,
        )
        .expect("Xyce-compatible periodic pulse spectrum");

        let samples = (0..203)
            .map(|sample| {
                let time = sample as Value / (203.0 * 1.0e6);
                if time < 5.0e-9 {
                    time / 5.0e-9
                } else if time < 495.0e-9 {
                    1.0
                } else if time < 500.0e-9 {
                    (500.0e-9 - time) / 5.0e-9
                } else {
                    0.0
                }
            })
            .collect::<Vec<_>>();
        let expected_dc = samples.iter().sum::<Value>() / samples.len() as Value;
        let expected_h1 = samples
            .iter()
            .enumerate()
            .map(|(sample, value)| {
                Complex64::from_polar(
                    *value,
                    -std::f64::consts::TAU * sample as Value / samples.len() as Value,
                )
            })
            .sum::<Complex64>()
            / samples.len() as Value
            * 2.0;
        let (_, amplitude, phase) = spectrum.harmonics[0];
        let actual_h1 = Complex64::from_polar(amplitude, phase);
        assert!((spectrum.dc - expected_dc).abs() < 1.0e-15);
        assert!((actual_h1 - expected_h1).norm() < 1.0e-14);
        assert!((spectrum.dc - 0.495).abs() > 1.0e-6);
    }

    #[test]
    fn xyce_multi_tone_pulse_uses_a_certified_nonuniform_apft_transform() {
        let config = HbConfig::multi_tone(vec![
            crate::analysis::harmonic_balance::HbTone::new(2.0e6, 2),
            crate::analysis::harmonic_balance::HbTone::new(3.0e6, 2),
        ])
        .with_collocation_points(13);
        let pulse = SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 17.0e-9,
            rise: 23.0e-9,
            fall: 31.0e-9,
            width: 0.40e-6,
            period: 0.5e-6,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };

        let spectrum = Engine::hb_source_spectrum(
            0.0,
            0.0,
            0.0,
            Some(&pulse),
            &config,
            &[2],
            SpiceDialect::Xyce,
        )
        .expect("multi-tone PULSE has an exact finite APFT projection");

        let period = config.fundamental_freq.recip();
        let rotation = 0.5 * (5.0_f64.sqrt() - 1.0);
        let mut time_points = (0..13)
            .map(|index| (((index as Value + 0.5) * rotation).fract()) * period)
            .collect::<Vec<_>>();
        time_points.sort_by(Value::total_cmp);
        for time in time_points {
            let expected =
                crate::circuit::VoltageSources::evaluate_source_spec_at_time_with_dialect(
                    &pulse,
                    time,
                    period / 13.0,
                    period,
                    SpiceDialect::Xyce,
                );
            let reconstructed = spectrum.dc
                + spectrum
                    .harmonics
                    .iter()
                    .map(|(harmonic, amplitude, phase)| {
                        amplitude
                            * (std::f64::consts::TAU
                                * *harmonic as Value
                                * config.fundamental_freq
                                * time
                                + *phase)
                                .cos()
                    })
                    .sum::<Value>();
            assert!(
                (reconstructed - expected).abs() < 1.0e-10,
                "APFT reconstruction at {time:.16e}: {reconstructed:.16e} != {expected:.16e}"
            );
        }
    }

    #[test]
    fn xyce_apft_rejects_aliased_inexact_and_mismatched_grids() {
        let pulse = SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 0.0,
            rise: 10.0e-9,
            fall: 10.0e-9,
            width: 0.48e-6,
            period: 0.5e-6,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };
        let aliased = HbConfig::multi_tone(vec![
            crate::analysis::harmonic_balance::HbTone::new(2.0e6, 2),
            crate::analysis::harmonic_balance::HbTone::new(2.0e6, 2),
        ])
        .with_collocation_points(9);
        let error = Engine::hb_source_spectrum(
            0.0,
            0.0,
            0.0,
            Some(&pulse),
            &aliased,
            &[1],
            SpiceDialect::Xyce,
        )
        .expect_err("duplicate authored APFT axes must not collapse silently");
        assert!(error.to_string().contains("alias common-grid harmonic"));

        let exact = HbConfig::multi_tone(vec![
            crate::analysis::harmonic_balance::HbTone::new(2.0e6, 2),
            crate::analysis::harmonic_balance::HbTone::new(3.0e6, 2),
        ])
        .with_collocation_points(13);
        let mut inexact = exact.clone();
        inexact.tones[0].frequency += 1.0e-4;
        let error = Engine::hb_source_spectrum(
            0.0,
            0.0,
            0.0,
            Some(&pulse),
            &inexact,
            &[2],
            SpiceDialect::Xyce,
        )
        .expect_err("nearby but distinct tones must not be rounded onto the APFT grid");
        assert!(error.to_string().contains("cannot be represented exactly"));

        let sparse_authored_grid = HbConfig::multi_tone(vec![
            crate::analysis::harmonic_balance::HbTone::new(2.0e6, 2),
            crate::analysis::harmonic_balance::HbTone::new(5.0e6, 2),
        ]);
        let error = Engine::hb_source_spectrum(
            0.0,
            0.0,
            0.0,
            Some(&pulse),
            &sparse_authored_grid,
            &[2],
            SpiceDialect::Xyce,
        )
        .expect_err("a sparse authored APFT lattice must not be silently densified");
        assert!(error.to_string().contains("authored signed-frequency grid"));

        let mismatched = exact.with_collocation_points(15);
        let error = Engine::hb_source_spectrum(
            0.0,
            0.0,
            0.0,
            Some(&pulse),
            &mismatched,
            &[2],
            SpiceDialect::Xyce,
        )
        .expect_err("an authored APFT grid cannot be silently resized");
        assert!(error.to_string().contains("authored grid requests 15"));
    }

    #[test]
    fn xyce_single_filtered_tone_uses_the_single_tone_collocation_transform() {
        let ordinary = HbConfig::new(1.0e6)
            .with_harmonics(2)
            .with_collocation_points(5);
        let filtered = HbConfig::multi_tone(vec![
            crate::analysis::harmonic_balance::HbTone::new(1.0e6, 2).with_source("VDRIVE"),
        ])
        .with_collocation_points(5);
        let pulse = SourceSpec::Pulse {
            v1: -0.25,
            v2: 0.75,
            delay: 20.0e-9,
            rise: 30.0e-9,
            fall: 40.0e-9,
            width: 0.35e-6,
            period: 1.0e-6,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };

        let ordinary_spectrum = Engine::hb_source_spectrum(
            -0.25,
            0.0,
            0.0,
            Some(&pulse),
            &ordinary,
            &[1],
            SpiceDialect::Xyce,
        )
        .expect("ordinary single-tone Xyce projection");
        let filtered_spectrum = Engine::hb_source_spectrum(
            -0.25,
            0.0,
            0.0,
            Some(&pulse),
            &filtered,
            &[1],
            SpiceDialect::Xyce,
        )
        .expect("source-filtered single-tone Xyce projection");

        assert_eq!(filtered_spectrum.dc, ordinary_spectrum.dc);
        assert_eq!(filtered_spectrum.harmonics, ordinary_spectrum.harmonics);
    }

    #[test]
    fn sin_source_is_converted_from_sine_to_cosine_reference() {
        let config = HbConfig::new(1.0e3).with_harmonics(3);
        let sin = SourceSpec::Sin {
            offset: 2.0,
            amplitude: 3.0,
            frequency: 1.0e3,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };
        let spectrum = Engine::hb_source_spectrum(
            2.0,
            0.0,
            0.0,
            Some(&sin),
            &config,
            &[1],
            SpiceDialect::BestAvailable,
        )
        .expect("periodic sine spectrum");

        assert_eq!(spectrum.dc, 2.0);
        let (_, amplitude, phase) = spectrum.harmonics[0];
        let phasor = Complex64::from_polar(amplitude, phase);
        assert!(phasor.re.abs() < 1.0e-12, "phasor={phasor:?}");
        assert!((phasor.im + 3.0).abs() < 1.0e-12, "phasor={phasor:?}");
    }

    #[test]
    fn transient_waveform_takes_precedence_over_small_signal_ac_annotation() {
        let config = HbConfig::new(1.0e3).with_harmonics(3);
        let source = SourceSpec::DcAcTransient {
            dc_value: 0.0,
            ac_magnitude: 99.0,
            ac_phase: 1.0,
            transient: Box::new(SourceSpec::Sin {
                offset: 1.0,
                amplitude: 2.0,
                frequency: 1.0e3,
                delay: 0.0,
                damping: 0.0,
                phase: 0.0,
            }),
        };
        let spectrum = Engine::hb_source_spectrum(
            0.0,
            99.0,
            1.0,
            Some(&source),
            &config,
            &[1],
            SpiceDialect::BestAvailable,
        )
        .expect("periodic transient spectrum");

        assert_eq!(spectrum.dc, 1.0);
        let (_, amplitude, phase) = spectrum.harmonics[0];
        let phasor = Complex64::from_polar(amplitude, phase);
        assert!(phasor.re.abs() < 1.0e-12, "phasor={phasor:?}");
        assert!((phasor.im + 2.0).abs() < 1.0e-12, "phasor={phasor:?}");
    }

    #[test]
    fn unmatched_periodic_source_contributes_dc_but_no_harmonics() {
        let config = HbConfig::new(1.0e3).with_harmonics(3);
        let sin = SourceSpec::Sin {
            offset: 2.0,
            amplitude: 3.0,
            frequency: 1.0e3,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };
        let spectrum = Engine::hb_source_spectrum(
            2.0,
            0.0,
            0.0,
            Some(&sin),
            &config,
            &[],
            SpiceDialect::BestAvailable,
        )
        .expect("filtered periodic source");

        assert_eq!(spectrum.dc, 2.0);
        assert!(spectrum.harmonics.is_empty());
    }

    #[test]
    fn periodic_source_rejects_non_finite_parameters() {
        let config = HbConfig::new(1.0e3).with_harmonics(3);
        let sin = SourceSpec::Sin {
            offset: 0.0,
            amplitude: f64::NAN,
            frequency: 1.0e3,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };
        let err = Engine::hb_source_spectrum(
            0.0,
            0.0,
            0.0,
            Some(&sin),
            &config,
            &[1],
            SpiceDialect::BestAvailable,
        )
        .expect_err("non-finite source parameters must fail");
        assert!(err.to_string().contains("amplitude must be finite"));
    }

    #[test]
    fn invalid_exact_collocation_grid_fails_before_solver_construction() {
        let netlist =
            Netlist::parse("invalid HB grid\nV1 1 0 1\nR1 1 0 1k\n.end\n").expect("deck parses");
        let config = HbConfig::new(1.0e3)
            .with_harmonics(5)
            .with_collocation_points(10);
        let err = Engine::new(SimulationConfig::default())
            .run_hb(&netlist, config)
            .expect_err("even/undersized collocation grid must fail");
        assert!(err.to_string().contains("collocation grid"));
    }

    #[test]
    fn non_finite_fundamental_is_rejected() {
        let netlist = Netlist::parse("invalid HB frequency\nV1 1 0 1\nR1 1 0 1k\n.end\n")
            .expect("deck parses");
        let err = Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(f64::NAN))
            .expect_err("NaN fundamental must fail");
        assert!(err.to_string().contains("finite and positive"));
    }
}
