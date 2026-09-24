//! The options that ride on an `HBINT` card of their own.
//!
//! Xyce's harmonic-balance integration package. One of its keys is admitted:
//! `TAHB`, which decides how the HB solve builds its first iterate. That
//! choice is the difference between a strongly driven amplifier converging and
//! not converging, and it is not derivable from the tones — which is why it is
//! an authored control rather than a heuristic.
//!
//! Admitted under the fourth route. `resolve_simulation_config` has no arm for
//! it; `Engine::hb_initial_state_strategy` reads the parsed record on the way
//! into a fresh HB solve, including Envelope initialization. HBSP and HBNOISE
//! reuse an existing carrier, whose producer owns this choice.
//!
//! The package's other two keys are excluded, and `catalog.rs`'s header says
//! why: `NUMFREQ` is read only by the deck translators the command line and
//! the bindings use, while the Studio's HB run builds its harmonic orders from
//! the form's own per-tone field; `SAVEICDATA` is parsed and dropped.

use super::{
    NumericOverrideOption, OptionPackage, OptionReach, OptionSpec, OverrideSection,
    OverrideValueKind,
};

pub(super) const HBINT: [OptionSpec; 1] = [OptionSpec {
    option: NumericOverrideOption::HbInitialState,
    key: "TAHB",
    package: OptionPackage::Hbint,
    section: OverrideSection::HarmonicBalance,
    label: "Initial state · HBINT TAHB",
    value_kind: OverrideValueKind::TimeDomainMode,
    value_hint: "how the first HB iterate is built",
    config_field: "netlist.options.hb_time_domain_mode",
    consumer: "engine/hb.rs:1150 · hb_initial_state_strategy",
    reach: OptionReach::HarmonicBalanceFamily,
}];
