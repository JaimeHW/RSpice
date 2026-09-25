//! Portable browser-worker protocol identity and execution environment.

/// Per-task environment after process models have been materialized into the
/// prepared source. The worker applies these values before dispatch.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisExecutionEnvironment {
    pub temperature_celsius: f64,
    pub supply_voltage: Option<f64>,
    pub nominal_supply_voltage: Option<f64>,
    #[serde(default)]
    pub supply_source_names: Vec<String>,
}

/// Browser request protocol revisions.
/// 34: ERROR measurements carry captured and validated comparison tables.
/// 35: optimization objectives and constraints carry requested physical units.
/// 36: sampled-noise configuration preserves aperture and observation timing.
/// 37: retained HB dependencies carry separate behavioral integral spectra.
/// 38: QPSS dependencies retain typed behavioral integral coordinates.
/// 39: transient-noise seeds distinguish inherited settings from explicit zero.
/// 40: AC DATA carries authored parameter columns or netlist table ownership.
/// 41: Envelope can select multirate integration and its carrier/event controls.
/// 42: remove the unsupported Reliability analysis.
pub const WORKER_REQUEST_TRANSPORT_PROTOCOL: u8 = 42;

/// Browser response protocol revisions.
/// 18: transient-source convergence evidence survives result transport.
/// 19: exact DC curve identities, coordinates and traversal survive transport.
/// 20: retained PSS orbits carry canonical MNA branch-current samples.
/// 21: transient results retain exact current impulse histories and coverage.
/// 22: live samples carry sequenced current-impulse suffixes and loss accounting.
/// 23: a transient carries the spectra of the `.fft` cards it evaluated, and a
///     recorded FFT result is its own response variant.
/// 24: a sensitivity result is one study — its filter, every frequency it
///     solved, and a raw, normalized and phase column per variable — in place
///     of two maps read at a single point.
/// Earlier workers silently omit numerical quality or current observations.
/// 25: Monte Carlo retains complete parameter-stream trial identities and failed observations.
/// 26: optimization retains each weighted objective at the best candidate.
/// 27: optimization retains validated hard-constraint evidence and feasibility.
/// 28: OP configurations retain compatible-circuit previous-state policy.
/// 29: scalar measurements preserve physical units.
/// 30: noise spectra retain their physical or logarithmic output unit.
/// 31: optimization observations retain requested physical units.
/// 32: sampled noise retains the sampling configuration and spectra.
/// 33: retained HB results carry separate behavioral integral spectra.
/// 34: QPSS results retain typed behavioral integral coordinates.
/// 35: SOA reporting views retain the complete observation history.
/// 36: remove Reliability result payloads.
pub const WORKER_RESPONSE_TRANSPORT_PROTOCOL: u8 = 36;
