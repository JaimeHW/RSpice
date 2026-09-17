//! Transient device noise: every device's own noise spectral density injected
//! as a seeded current in the time domain.
//!
//! `.TRAN … NOISEFMAX=f` runs an ordinary transient with one extra current
//! between the terminals of every noise mechanism the device models export —
//! the same mechanisms `.NOISE` sums, collected by the same code
//! ([`Engine::try_collect_noise_sources`]), so nothing here invents a density.
//! This is Spectre's `tran noisefmax=…`.
//!
//! # What is injected
//!
//! For mechanism `k`,
//!
//! ```text
//! i_k(t) = NOISESCALE · a_k(bias(t)) · xi_k(t)
//! ```
//!
//! where `xi_k` is a unit-density sample-and-hold process on the `k·NT` grid
//! (see [`crate::circuit::device_noise`] for its spectrum and why it holds
//! rather than interpolates) and `a_k` is re-derived from the accepted
//! solution at every accepted step, which is what makes the injected noise
//! track the instantaneous bias instead of the `t = 0` operating point.
//!
//! Two shapes of density cover every mechanism that can be rendered:
//!
//! * **Frequency-flat** — thermal (`4kT/R`), shot (`2qI`), an explicit
//!   Verilog-A `white_noise(pwr)`, and a `1/f^ef` law with `ef = 0`. A flat
//!   one-sided density `S` is a held standard normal with
//!   `a_k = sqrt(S·fmax)`, which makes both the in-band density `S` and the
//!   total mean square `S·fmax` exact.
//!
//! * **`A/f^ef`** — every flicker mechanism, including the BSIM3 and BSIM4
//!   physical channel-flicker models, whose densities are also exactly
//!   proportional to `f^-ef`. The unit process is the Kasdin
//!   fractional-integration sequence already used by `TRNOISE`
//!   ([`super::noise::kasdin_one_over_f`], ngspice's `f_alpha` construction in
//!   `src/frontend/trannoise/1-f-code.c:44-53`, whose `Q_d` is the standard
//!   deviation of the white input), driven by white noise of variance
//!
//!   ```text
//!   q = (2·pi·NT)^ef / (2·NT)
//!   ```
//!
//!   which is the normalization that makes the held sequence's one-sided
//!   density exactly `1/f^ef` A²/Hz: the filter `H(z) = (1 - z^-1)^(-ef/2)`
//!   has `|H|² = |2 sin(w/2)|^-ef -> (2·pi·f·NT)^-ef` at low frequency, and a
//!   held sequence of discrete density `P(w)` has continuous one-sided density
//!   `2·NT·P(2·pi·f·NT)·sinc²(f·NT)`. Then `a_k = sqrt(A)` with `A` the
//!   mechanism's own density at 1 Hz. At `ef = 0` the formula degenerates to
//!   `q = fmax`, agreeing with the flat case.
//!
//!   Ngspice's `NA`/`NAMP` carry no spectral claim — `isrcload.c:347-352`
//!   interpolates raw amplitudes — so there is no ngspice normalization to
//!   agree with; only the generator is shared.
//!
//! # What is refused
//!
//! A mechanism whose density is neither flat nor a power law cannot be
//! rendered from what the model states, and guessing the missing degree of
//! freedom would be inventing physics. Those are refused by name rather than
//! approximated:
//!
//! * a tabulated density (Verilog-A `noise_table`), which is an arbitrary
//!   spectrum;
//! * burst/popcorn noise, a Lorentzian plateau `P0` with corner `fb`: a
//!   random-telegraph process reproduces it exactly, but `(P0, fb)` fixes only
//!   two of its three parameters, and choosing the third is an assumption the
//!   model card did not make;
//! * BSIM4 correlated channel/gate thermal noise (`tnoiMod=2`), whose pair is
//!   defined by a cross-spectrum rather than two independent currents;
//! * a mechanism injected at a noise-only private row — BJT distributed base
//!   noise adds rows beyond the MNA system, which the transient does not
//!   solve for.
//!
//! A device with no noise model contributes nothing, silently, exactly as in
//! `.NOISE`.

use std::collections::HashMap;
use std::sync::Arc;

use super::noise::{
    DEFAULT_NOISE_SEED, MAX_NOISE_SAMPLES, SplitMix64, check_noise_abort,
    checked_noise_sample_count, kasdin_one_over_f,
};
use crate::abort_signal::AbortSignal;
use crate::analysis::noise::{NoiseSource, NoiseSourceIdentity, NoiseSourceType};
use crate::circuit::device_noise::{InjectedNoiseSource, NoiseTrain, TransientDeviceNoise};
use crate::config::SpiceDialect;
use crate::engine::{Engine, SimulationError};
use crate::netlist::TransientNoiseConfig;
use crate::numerics::fnv1a;
use crate::{CircuitData, Value};

/// Total materialized `1/f^ef` samples one run may hold, across every source.
///
/// A flat mechanism costs no memory at all, so this bounds only the flicker
/// trains: 16M values is 128 MB, past which a deck should raise `NOISEFMAX`'s
/// reciprocal or shorten the run rather than have the simulator quietly take
/// the machine.
const MAX_TOTAL_FLICKER_SAMPLES: usize = 1 << 24;

/// The largest `1/f^ef` exponent the fractional integrator represents.
///
/// `ef = 2` is already a random walk; above it the filter's low-frequency
/// power grows faster than any finite record can represent and the "lowest
/// represented frequency" stops meaning anything.
const MAX_FLICKER_EXPONENT: Value = 2.0;

/// How one mechanism's amplitude is derived from its spectral density.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AmplitudeLaw {
    /// Frequency-flat: `sqrt(S · fmax)`.
    Flat,
    /// `A/f^ef`: `sqrt(A)`, with `A` the density at 1 Hz.
    PowerLaw,
}

/// Which law renders one mechanism, or why none does.
///
/// `Ok(None)` is a mechanism that carries no density of its own — the BSIM4
/// correlated placeholder, whose content lives in the pair this run has
/// already refused — and injects nothing.
fn rendering_law(source: &NoiseSource) -> Result<Option<AmplitudeLaw>, String> {
    let named = describe(&source.identity);
    match source.noise_type {
        NoiseSourceType::Thermal | NoiseSourceType::Shot | NoiseSourceType::White => {
            Ok(Some(AmplitudeLaw::Flat))
        }
        NoiseSourceType::Flicker
        | NoiseSourceType::Bsim3Flicker
        | NoiseSourceType::Bsim4Flicker => {
            let ef = source.ef;
            if !ef.is_finite() || !(0.0..=MAX_FLICKER_EXPONENT).contains(&ef) {
                return Err(format!(
                    "transient noise cannot render '{named}' with a 1/f exponent of {ef}; the \
                     fractional integrator represents 0 to {MAX_FLICKER_EXPONENT}"
                ));
            }
            Ok(Some(if ef == 0.0 {
                AmplitudeLaw::Flat
            } else {
                AmplitudeLaw::PowerLaw
            }))
        }
        NoiseSourceType::Table => Err(format!(
            "transient noise cannot render '{named}': a tabulated spectral density is neither \
             frequency-flat nor a power law, and rendering it as either would report a spectrum \
             the model does not state. Use .NOISE for this device."
        )),
        NoiseSourceType::Burst => Err(format!(
            "transient noise cannot render '{named}': its Lorentzian plateau and corner \
             frequency fix two of the three parameters of the random-telegraph process that \
             would reproduce it, and choosing the third is an assumption the model card did not \
             make. Use .NOISE for this device."
        )),
        NoiseSourceType::Bsim4CorrelatedThermal => Ok(None),
    }
}

/// What the accepted-step refresh needs to re-derive one injected amplitude.
struct AmplitudeEntry {
    identity: NoiseSourceIdentity,
    noise_type: NoiseSourceType,
    node_pos: usize,
    node_neg: usize,
    law: AmplitudeLaw,
    /// Absolute temperature the model reports for this mechanism, when it
    /// reports one; otherwise the analysis temperature is used.
    absolute_temperature: Option<Value>,
}

impl AmplitudeEntry {
    /// The structural key a fallback match uses when the collected catalog's
    /// shape has changed under a bias-dependent guard.
    fn key(&self) -> (&str, Option<&str>, NoiseSourceType, usize, usize) {
        (
            self.identity.device.as_str(),
            self.identity.mechanism.as_deref(),
            self.noise_type,
            self.node_pos,
            self.node_neg,
        )
    }
}

/// The run-local half of transient noise: everything needed to refresh the
/// amplitudes the circuit stamps.
pub(super) struct TransientNoiseRuntime {
    dialect: SpiceDialect,
    ambient_temperature: Value,
    scale: Value,
    fmax: Value,
    /// The seed the whole run was derived from, for the run log.
    seed: u64,
    entries: Vec<AmplitudeEntry>,
}

impl TransientNoiseRuntime {
    /// How many mechanisms are injected.
    pub(super) fn source_count(&self) -> usize {
        self.entries.len()
    }

    /// The seed every injected stream was derived from.
    pub(super) fn seed(&self) -> u64 {
        self.seed
    }

    /// Re-derive every injected amplitude from an accepted solution.
    ///
    /// Re-collecting the catalog is what makes the density the device model's
    /// own at the present bias rather than a frozen operating-point value.
    /// The catalog's order is a function of the circuit's fixed device arrays,
    /// so the common case matches position for position; the structural check
    /// below is what proves that rather than assuming it, and a mechanism
    /// whose bias-dependent guard has flipped falls back to a keyed match.
    pub(super) fn refresh(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
    ) -> Result<(), SimulationError> {
        if self.entries.is_empty() {
            return Ok(());
        }
        let mut collected = Engine::try_collect_noise_sources(circuit, solution, self.dialect)?;
        Engine::configure_noise_physical_constants(
            &mut collected.elementary,
            &mut collected.correlated,
            self.dialect,
        );
        self.apply(circuit, &collected.elementary)
    }

    /// Re-derive the amplitudes from a catalog the caller has already
    /// collected and given this run's physical constants.
    fn apply(
        &self,
        circuit: &mut CircuitData,
        collected: &[NoiseSource],
    ) -> Result<(), SimulationError> {
        let positional = collected.len() == self.entries.len()
            && collected.iter().zip(&self.entries).all(|(source, entry)| {
                source.noise_type == entry.noise_type
                    && source.node_pos == entry.node_pos
                    && source.node_neg == entry.node_neg
            });
        let fallback = if positional {
            None
        } else {
            let mut index = HashMap::with_capacity(collected.len());
            for (position, source) in collected.iter().enumerate() {
                index
                    .entry((
                        source.identity.device.as_str(),
                        source.identity.mechanism.as_deref(),
                        source.noise_type,
                        source.node_pos,
                        source.node_neg,
                    ))
                    .or_insert(position);
            }
            Some(index)
        };

        let mut amplitudes = Vec::with_capacity(self.entries.len());
        for (position, entry) in self.entries.iter().enumerate() {
            let source = match &fallback {
                None => collected.get(position),
                Some(index) => index.get(&entry.key()).and_then(|&at| collected.get(at)),
            };
            // A mechanism the present bias has switched off injects nothing
            // at this step, which is the density its own model reports.
            let Some(source) = source else {
                amplitudes.push(0.0);
                continue;
            };
            amplitudes.push(self.amplitude(source, entry)?);
        }

        let Some(plan) = circuit.transient_device_noise_mut() else {
            return Err(SimulationError::Circuit(
                "transient noise amplitudes were refreshed without an installed injection plan"
                    .to_string(),
            ));
        };
        for (injected, amplitude) in plan.sources_mut().iter_mut().zip(amplitudes) {
            injected.amplitude = amplitude;
        }
        Ok(())
    }

    /// The complete multiplier one mechanism's unit process is scaled by.
    fn amplitude(
        &self,
        source: &NoiseSource,
        entry: &AmplitudeEntry,
    ) -> Result<Value, SimulationError> {
        let temperature = Engine::elementary_noise_temperature(
            self.ambient_temperature,
            entry.absolute_temperature,
        );
        // Both laws are evaluated at 1 Hz: a flat density ignores the
        // frequency, and a power law's value there is its coefficient.
        let density = source
            .try_spectral_density(1.0, temperature)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "transient noise source '{}' is not evaluable: {error}",
                    describe(&entry.identity)
                ))
            })?;
        let coefficient = match entry.law {
            AmplitudeLaw::Flat => density * self.fmax,
            AmplitudeLaw::PowerLaw => density,
        };
        if !coefficient.is_finite() || coefficient < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "transient noise source '{}' produced a {coefficient} noise power",
                describe(&entry.identity)
            )));
        }
        Ok(self.scale * coefficient.sqrt())
    }
}

/// A device and mechanism, as a diagnostic names it.
fn describe(identity: &NoiseSourceIdentity) -> String {
    match &identity.mechanism {
        Some(mechanism) => format!("{}:{mechanism}", identity.device),
        None => identity.device.clone(),
    }
}

/// The stream seed of one mechanism.
///
/// Content-addressed so that reruns are bit-identical and adding an unrelated
/// device cannot reshuffle another device's train: the run seed is mixed with
/// an FNV hash of the mechanism's own identity, its kind, and how many earlier
/// sources shared all three. The ordinal is there because several elementary
/// sources of one device may legitimately share an identity — `.NOISE` sums
/// them — and two of them sharing a train would make independent mechanisms
/// perfectly correlated. Solution rows are deliberately *not* in the key: a
/// deck that gains an unrelated device renumbers rows, and a stream that moved
/// with the numbering would change a device's train for a reason that has
/// nothing to do with that device.
fn stream_seed(
    run_seed: u64,
    identity: &NoiseSourceIdentity,
    noise_type: NoiseSourceType,
    ordinal: usize,
) -> u64 {
    let key = format!(
        "{}|{}|{}|{ordinal}",
        identity.device.to_ascii_uppercase(),
        identity.mechanism.as_deref().unwrap_or(""),
        noise_type.label(),
    );
    run_seed ^ fnv1a(&key)
}

impl Engine {
    /// Build the injection plan for a transient-noise run and install it on
    /// the circuit.
    ///
    /// Returns the run-local half, or `None` when the deck did not ask for
    /// transient noise. A deck that asked for it and has no noise mechanism at
    /// all runs as an ordinary deterministic transient, and the run log says
    /// so rather than the request disappearing.
    pub(super) fn install_transient_device_noise(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
        config: TransientNoiseConfig,
        tstop: Value,
        options_seed: Option<u64>,
        abort: &dyn AbortSignal,
    ) -> Result<TransientNoiseRuntime, SimulationError> {
        config.validate().map_err(SimulationError::Circuit)?;
        if !(tstop.is_finite() && tstop > 0.0) {
            return Err(SimulationError::Circuit(format!(
                "transient noise requires a finite positive stop time, found {tstop}"
            )));
        }
        check_noise_abort(abort)?;

        let nt = 1.0 / (2.0 * config.fmax);
        if !(nt.is_finite() && nt > 0.0) {
            return Err(SimulationError::Circuit(format!(
                "NOISEFMAX={} does not give a representable sample interval",
                config.fmax
            )));
        }
        let sample_count = checked_noise_sample_count(
            "transient noise",
            "the .TRAN card",
            tstop / nt,
            1,
            "tstop·2·NOISEFMAX",
            "Lower NOISEFMAX or shorten the transient.",
        )
        .map_err(SimulationError::Circuit)?;

        let mut collected =
            Self::try_collect_noise_sources(circuit, solution, self.config.spice_dialect)?;
        Self::configure_noise_physical_constants(
            &mut collected.elementary,
            &mut collected.correlated,
            self.config.spice_dialect,
        );
        if !collected.correlated.is_empty() {
            let mut named: Vec<String> = collected
                .correlated
                .iter()
                .map(|pair| pair.identity.device.clone())
                .collect();
            named.sort_unstable();
            named.dedup();
            return Err(SimulationError::Circuit(format!(
                "transient noise cannot inject correlated channel/gate thermal noise, which is \
                 defined by a cross-spectrum rather than by independent currents: {}. Select \
                 tnoiMod=0 or 1 on those models, or use .NOISE.",
                named.join(", ")
            )));
        }

        let matrix_rows = circuit.matrix_size();
        let run_seed = config.seed.or(options_seed).unwrap_or(DEFAULT_NOISE_SEED);
        let mut injected = Vec::new();
        let mut entries = Vec::new();
        let mut flicker_budget = MAX_TOTAL_FLICKER_SAMPLES;
        let mut ordinals: HashMap<(String, Option<String>, NoiseSourceType), usize> =
            HashMap::new();
        for (position, source) in collected.elementary.iter().enumerate() {
            if position.is_multiple_of(64) {
                check_noise_abort(abort)?;
            }
            let identity = &source.identity;
            if source.node_pos > matrix_rows || source.node_neg > matrix_rows {
                return Err(SimulationError::Circuit(format!(
                    "transient noise cannot inject '{}': its mechanism lives on a noise-only \
                     private row that a transient does not solve for. Use .NOISE for that \
                     device, or remove its distributed base resistance.",
                    describe(identity)
                )));
            }
            let Some(law) = rendering_law(source).map_err(SimulationError::Circuit)? else {
                continue;
            };
            let absolute_temperature = collected
                .elementary_absolute_temperatures
                .get(position)
                .copied()
                .flatten();
            let ordinal = ordinals
                .entry((
                    identity.device.to_ascii_uppercase(),
                    identity.mechanism.clone(),
                    source.noise_type,
                ))
                .or_insert(0);
            let seed = stream_seed(run_seed, identity, source.noise_type, *ordinal);
            *ordinal += 1;
            let train = match law {
                AmplitudeLaw::Flat => NoiseTrain::Flat { stream_seed: seed },
                AmplitudeLaw::PowerLaw => {
                    let samples = Self::flicker_unit_train(
                        source.ef,
                        nt,
                        sample_count,
                        config.fmin.unwrap_or(1.0 / tstop),
                        seed,
                        &mut flicker_budget,
                        identity,
                        abort,
                    )?;
                    NoiseTrain::PowerLaw {
                        samples: Arc::new(samples),
                    }
                }
            };
            injected.push(InjectedNoiseSource {
                node_pos: source.node_pos,
                node_neg: source.node_neg,
                train,
                amplitude: 0.0,
            });
            entries.push(AmplitudeEntry {
                identity: identity.clone(),
                noise_type: source.noise_type,
                node_pos: source.node_pos,
                node_neg: source.node_neg,
                law,
                absolute_temperature,
            });
        }

        circuit.install_transient_device_noise(TransientDeviceNoise::new(
            nt,
            sample_count,
            injected,
        ));
        let runtime = TransientNoiseRuntime {
            dialect: self.config.spice_dialect,
            ambient_temperature: self.config.temperature,
            scale: config.scale,
            fmax: config.fmax,
            seed: run_seed,
            entries,
        };
        // The catalog in hand is the one the operating point produced, so the
        // first amplitudes come from it rather than from a second collection.
        runtime.apply(circuit, &collected.elementary)?;
        Ok(runtime)
    }

    /// Make every noise sample boundary a solver stop.
    ///
    /// A sample-and-hold source steps at each boundary, so the integrator has
    /// to land on them rather than integrate across them — the same treatment
    /// an authored `TRNOISE` train gets, where the boundaries arrive as PWL
    /// breakpoints. Clamping `tmax` instead would leave the discontinuity
    /// inside a step.
    pub(super) fn add_transient_noise_breakpoints(
        breakpoints: &mut crate::numerics::integration::BreakpointManager,
        circuit: &CircuitData,
        tstop: Value,
        max_points: usize,
    ) -> Result<(), SimulationError> {
        let Some(plan) = circuit.transient_device_noise() else {
            return Ok(());
        };
        let nt = plan.sample_interval();
        let count = plan.sample_count();
        if count > max_points {
            return Err(SimulationError::Circuit(format!(
                "transient noise needs {count} sample breakpoints, past the {max_points}-point \
                 analysis limit. Lower NOISEFMAX, shorten the transient, or raise \
                 .OPTIONS MAXANALYSISPOINTS."
            )));
        }
        for index in 1..=count {
            Self::add_breakpoint_if_in_range(breakpoints, index as Value * nt, tstop);
        }
        Ok(())
    }

    /// One mechanism's unit `1/f^ef` train: one-sided density `1/f^ef` A²/Hz
    /// once held on the sample grid.
    ///
    /// The fractional integrator's impulse response is truncated after the
    /// taps that `fmin` covers, and that many leading samples are generated
    /// and discarded, so the retained record is stationary rather than opening
    /// with the filter's warm-up.
    #[allow(clippy::too_many_arguments)]
    fn flicker_unit_train(
        ef: Value,
        nt: Value,
        sample_count: usize,
        fmin: Value,
        seed: u64,
        budget: &mut usize,
        identity: &NoiseSourceIdentity,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let periods = 1.0 / (fmin * nt);
        if !(periods.is_finite() && periods >= 1.0) {
            return Err(SimulationError::Circuit(format!(
                "NOISEFMIN={fmin} is not representable on the {nt} s noise sample grid"
            )));
        }
        let taps = (periods.ceil() as usize).min(sample_count);
        let total = sample_count
            .checked_add(taps)
            .filter(|total| *total <= MAX_NOISE_SAMPLES)
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "transient noise source '{}' needs {sample_count} samples plus a \
                     {taps}-sample warm-up, past the {MAX_NOISE_SAMPLES} limit. Raise \
                     NOISEFMIN, lower NOISEFMAX, or shorten the transient.",
                    describe(identity)
                ))
            })?;
        if total > *budget {
            return Err(SimulationError::Circuit(format!(
                "transient noise flicker trains exceed the {MAX_TOTAL_FLICKER_SAMPLES}-sample \
                 budget at source '{}'. Raise NOISEFMIN, lower NOISEFMAX, or shorten the \
                 transient.",
                describe(identity)
            )));
        }
        *budget -= total;

        let variance = (2.0 * std::f64::consts::PI * nt).powf(ef) / (2.0 * nt);
        if !(variance.is_finite() && variance > 0.0) {
            return Err(SimulationError::Circuit(format!(
                "transient noise source '{}' has no representable 1/f^{ef} normalization at a \
                 {nt} s sample interval",
                describe(identity)
            )));
        }
        let mut rng = SplitMix64::new(seed);
        let mut train = kasdin_one_over_f(total, ef, variance.sqrt(), taps, &mut rng, abort)?;
        train.drain(..taps);
        if train.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(format!(
                "transient noise source '{}' generated a non-finite 1/f sample",
                describe(identity)
            )));
        }
        Ok(train)
    }
}

#[cfg(test)]
mod tests;
