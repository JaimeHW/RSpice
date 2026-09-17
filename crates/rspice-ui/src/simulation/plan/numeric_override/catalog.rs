//! The advanced options an analysis may depart from the plan on, and the
//! engine site that reads each one.
//!
//! ## The admission rule
//!
//! An option is in this catalog only if all three of these hold:
//!
//! 1. **The deck can carry it.** It is a key `rspice_core`'s `.OPTIONS` parser
//!    reads, because a per-analysis record's only route to the engine is a
//!    second `.OPTIONS` block spliced into that task's deck.
//! 2. **The resolver maps it.** `rspice_core::resolve_simulation_config` moves
//!    the parsed key onto a `SimulationConfig` field. A key the resolver has no
//!    arm for is parsed and then dropped.
//! 3. **The engine reads that field.** [`OptionSpec::consumer`] names the exact
//!    site. A field that is only ever written is not read, and an option that
//!    reaches only a field like that is a dead control wearing a number.
//!
//! Every entry below states its consumer, so the third condition is checked by
//! reading the table rather than by remembering. `numeric_override::tests`
//! then proves the whole rule mechanically: it emits each option, parses the
//! deck, resolves it, and fails on any option that leaves the resolved
//! `SimulationConfig` unchanged.
//!
//! ## What was excluded, and why
//!
//! These are the keys a reader will look for and not find. Each was rejected
//! against the rule above, not overlooked:
//!
//! - **`ITL2`, `ITL6`** — parsed into `SimulationOptions` and dropped there.
//!   `config_resolver.rs` has no arm for either, and neither name occurs
//!   anywhere under `rspice-core/src/engine/`. Condition 2 fails.
//! - **`TEMP`** — a per-analysis temperature is already owned by the run
//!   space's TEMP/Corner axis. A second owner is exactly the defect this
//!   record exists to avoid.
//! - **`TNOM`** — the model-parameter reference temperature. It reaches
//!   devices through `netlist::param_scope`, not through
//!   `resolve_simulation_config`, and it describes the model library rather
//!   than one analysis's numerics.
//! - **`SEED`** — the parser seeds the statistical stream before any parameter
//!   is evaluated. This record's block is spliced in just before the terminal
//!   `.end` card, which is after every parameter, so a seed stated here would
//!   be read too late to govern the draws it names.
//! - **`ABSTOL`** — the other spelling of the current floor. Both it and
//!   `IABSTOL` resolve onto `current_abstol`, and the resolver reads
//!   `opts.iabstol.or(opts.abstol)` — field precedence rather than card order,
//!   so only `IABSTOL` can win against a plan that states either.
//!   [`NumericOverrideOption::Abstol`] emits that one. One field, one owner.
//! - **`MAXTIMESTEP`** — `config.max_timestep` is a third step ceiling, and the
//!   transient engine composes all three by `min` (`engine/transient.rs:1540`),
//!   so it can only restate a ceiling the analysis can already state through
//!   [`NumericOverrideOption::MaximumTimestep`] or the transient form's own Max
//!   step field. One bound, one copy.
//! - **`RSHUNT`, `CSHUNT`** — read by the engine, but they add real resistors
//!   and capacitors to every node. They change the circuit, not one analysis's
//!   numerics, and the plan states no policy for them to depart from.

mod global;
mod timeint;

use crate::simulation::accuracy::AnalysisAccuracy;
use crate::simulation::dialog::{DampingStrategy, IntegrationMethod, MatrixSolver, OpHomotopy};

use crate::simulation::plan::AnalysisKind;

/// One solver option an analysis may depart from the plan on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericOverrideOption {
    // Convergence
    Reltol,
    Abstol,
    Vntol,
    ResidualReltol,
    Gmin,
    Itl1,
    Itl4,
    GminStepping,
    SourceStepping,
    PseudoTransient,
    ArcLength,
    Damping,
    // Charge
    Chgtol,
    // Integration
    Trtol,
    IntegrationMethod,
    LteReltol,
    LteAbstol,
    MinTimestep,
    MaximumTimestep,
    // Matrix
    Pivrel,
    Pivtol,
    Solver,
    // Device bypass
    Bypass,
    BypassReltol,
    BypassAbstol,
}

/// Where an option sits in the sectioned editor.
///
/// The sections are the engine's own divisions, not a tidy-up: an option in
/// [`Self::Convergence`] bounds the Newton solve, one in [`Self::Integration`]
/// bounds the time step, and mixing them is what makes an options page
/// unreadable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OverrideSection {
    Convergence,
    Charge,
    Integration,
    Matrix,
    DeviceBypass,
}

impl OverrideSection {
    /// Every section, in the order the editor stacks them.
    pub const ALL: [Self; 5] = [
        Self::Convergence,
        Self::Charge,
        Self::Integration,
        Self::Matrix,
        Self::DeviceBypass,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Convergence => "Convergence",
            Self::Charge => "Charge",
            Self::Integration => "Integration",
            Self::Matrix => "Matrix",
            Self::DeviceBypass => "Device bypass",
        }
    }
}

/// The card an option's key rides on.
///
/// The parser's package selector stays in force for the rest of the `.OPTIONS`
/// command it appears on, so a scoped key placed among the global ones would
/// re-scope every key after it. Each package therefore gets its own card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptionPackage {
    Global,
    Timeint,
}

/// What an authored value has to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverrideValueKind {
    /// A finite value strictly greater than zero.
    PositiveReal,
    /// A finite value at or above zero. `GMIN=0` is meaningful: it asks for no
    /// junction conductance floor at all.
    NonNegativeReal,
    /// A whole count of at least one.
    IterationCount,
    /// On or off.
    Flag,
    /// One of [`IntegrationMethod::all`].
    Method,
    /// One of [`DampingStrategy::all`].
    Damping,
    /// One of [`MatrixSolver::all`] that names an explicit backend.
    Solver,
}

/// Why an analysis of some kind cannot carry an option.
///
/// Two things make an option inapplicable: the solve never reads it, or some
/// other record already owns it. Both are worded once here so a reader meets
/// the same sentence wherever the refusal surfaces.
pub(super) const NOT_TIME_STEPPED: &str =
    "this analysis never advances time, so a time-integration bound would not reach its solve";
pub(super) const TRANSIENT_OWNS_STEP_CEILING: &str =
    "the transient's own Max step field owns this, and one bound cannot have two copies";
/// The continuation aids and the damping strategy under a tier that assigns
/// them. `Fast` clears every aid and `Robust` sets every one, both after the
/// deck has been resolved, so an authored value under either is read and then
/// discarded. `Balanced` and `Accurate` inherit and own nothing.
pub(super) const FAST_TIER_OWNS_CONTINUATION: &str = "the Fast accuracy tier turns every continuation aid off after the deck's options are \
     resolved; set this analysis's tier to Balanced to author the aid here";
pub(super) const ROBUST_TIER_OWNS_CONTINUATION: &str = "the Robust accuracy tier turns every continuation aid on after the deck's options are \
     resolved; set this analysis's tier to Balanced to author the aid here";
/// The four stepping flags under an explicit operating-point homotopy. The
/// homotopy control is applied after the tier, so it is the last writer.
pub(super) const HOMOTOPY_OWNS_CONTINUATION: &str = "this operating point's Homotopy control assigns the continuation aids after the deck's \
     options are resolved; set Homotopy to Adaptive to author the aid here";

/// Everything the editor, the emitter, the digest and the ledger need to know
/// about one option.
///
/// `Copy` because the catalog is assembled from the package tables by a
/// `const fn`, which reads each entry out of its table by index.
#[derive(Clone, Copy)]
pub struct OptionSpec {
    pub option: NumericOverrideOption,
    /// The `.OPTIONS` key, exactly as the engine's parser spells it.
    pub key: &'static str,
    pub package: OptionPackage,
    pub section: OverrideSection,
    pub label: &'static str,
    pub value_kind: OverrideValueKind,
    /// What an authored value must look like, for the input's placeholder.
    pub value_hint: &'static str,
    /// The `SimulationConfig` field `resolve_simulation_config` writes.
    pub config_field: &'static str,
    /// Where `rspice-core` reads that field. This is the whole admission
    /// argument for the entry, so it names a file and a line rather than a
    /// subsystem.
    pub consumer: &'static str,
    /// True when the option is only read on a time-stepped path.
    pub time_stepped_only: bool,
}

/// The catalog. Order here is the order the ledger and the editor report.
///
/// The two package tables are spliced rather than appended, because that
/// order is by *section*: the four `TIMEINT` keys are the integration
/// section's own, and they report after the global keys that open that
/// section and before the matrix section's. The tables carry ownership; this
/// carries the order.
pub(super) const SPECS: &[OptionSpec] = &LEDGER_ORDER;

/// How many options the catalog holds, across every package.
const CATALOG_LEN: usize = global::GLOBAL.len() + timeint::TIMEINT.len();

const LEDGER_ORDER: [OptionSpec; CATALOG_LEN] = ledger_order();

/// Both package tables, in the order the ledger reports them.
const fn ledger_order() -> [OptionSpec; CATALOG_LEN] {
    let seam = integration_seam();
    let mut catalog = [global::GLOBAL[0]; CATALOG_LEN];
    let mut written = 0;
    let mut index = 0;
    while index < seam {
        catalog[written] = global::GLOBAL[index];
        written += 1;
        index += 1;
    }
    index = 0;
    while index < timeint::TIMEINT.len() {
        catalog[written] = timeint::TIMEINT[index];
        written += 1;
        index += 1;
    }
    index = seam;
    while index < global::GLOBAL.len() {
        catalog[written] = global::GLOBAL[index];
        written += 1;
        index += 1;
    }
    catalog
}

/// Where the integration package's table splices into the global one: after
/// the last global key of the same section.
///
/// Derived rather than written down. A global key added to the convergence
/// section would otherwise push the integration package past the matrix
/// section with nothing saying so.
const fn integration_seam() -> usize {
    let mut seam = 0;
    let mut index = 0;
    while index < global::GLOBAL.len() {
        if matches!(global::GLOBAL[index].section, OverrideSection::Integration) {
            seam = index + 1;
        }
        index += 1;
    }
    seam
}

impl NumericOverrideOption {
    /// Why an analysis carrying an accuracy tier cannot be given ITL1, and
    /// equally why the Solver ledger does not report the plan's ITL1 as that
    /// analysis's effective Newton budget.
    ///
    /// [`crate::simulation::accuracy::AccuracyPolicy::apply`] assigns
    /// `max_iterations` from the tier *after* the deck's `.OPTIONS` have been
    /// resolved, so an ITL1 written into either options block — the plan's or
    /// this record's — is overwritten before the first Newton step. One
    /// sentence states it, and both the refusal and the ledger's origin cell
    /// use this one.
    pub const ACCURACY_TIER_OWNS_ITERATIONS: &'static str =
        "its accuracy tier owns the Newton budget and is applied after the deck's options";

    /// This option's catalog entry.
    #[must_use]
    pub fn spec(self) -> &'static OptionSpec {
        SPECS
            .iter()
            .find(|spec| spec.option == self)
            .expect("every option has exactly one catalog entry; the catalog test proves it")
    }

    /// Every option, in the order the ledger reports them.
    ///
    /// `Iterator` is already `#[must_use]`, so the attribute is not repeated
    /// here; a dropped iterator is refused by the trait's own annotation.
    pub fn all() -> impl Iterator<Item = Self> {
        SPECS.iter().map(|spec| spec.option)
    }

    /// How the option is named on the ledger and in refusals.
    #[must_use]
    pub fn label(self) -> &'static str {
        self.spec().label
    }

    /// Short identity used by the option picker and by refusal messages.
    ///
    /// Two options spell the same key under different packages — the global
    /// `RELTOL` bounds the Newton update and `TIMEINT RELTOL` bounds the
    /// accepted truncation error — so this is the *qualified* name, and
    /// [`OptionSpec::key`] is the bare key the deck carries.
    #[must_use]
    pub fn key(self) -> &'static str {
        let spec = self.spec();
        match spec.package {
            OptionPackage::Global => spec.key,
            OptionPackage::Timeint => match self.option_timeint_name() {
                Some(name) => name,
                None => spec.key,
            },
        }
    }

    /// The qualified spelling for a packaged key, as a literal so the return
    /// stays `'static`.
    const fn option_timeint_name(self) -> Option<&'static str> {
        match self {
            Self::LteReltol => Some("TIMEINT RELTOL"),
            Self::LteAbstol => Some("TIMEINT ABSTOL"),
            Self::MinTimestep => Some("TIMEINT MINTIMESTEP"),
            Self::MaximumTimestep => Some("TIMEINT DELMAX"),
            _ => None,
        }
    }

    /// What an authored value must look like, for the input's placeholder.
    #[must_use]
    pub fn value_hint(self) -> &'static str {
        self.spec().value_hint
    }

    #[must_use]
    pub fn section(self) -> OverrideSection {
        self.spec().section
    }

    /// Where `rspice-core` reads the field this option resolves onto.
    #[must_use]
    pub fn consumer(self) -> &'static str {
        self.spec().consumer
    }

    /// The `SimulationConfig` field this option resolves onto.
    #[must_use]
    pub fn config_field(self) -> &'static str {
        self.spec().config_field
    }

    #[must_use]
    pub fn value_kind(self) -> OverrideValueKind {
        self.spec().value_kind
    }

    /// Why this kind cannot carry this option, if it cannot.
    ///
    /// A refusal is the whole point of the gate: an override that is accepted,
    /// persisted and then ignored by the solve is indistinguishable from one
    /// that works, which is the failure this record exists to avoid.
    ///
    /// This answers for the kind alone, which is all a kind-level reader — the
    /// catalogue ratchets, the projection tests — can ask. A kind that offers
    /// an accuracy tier or a homotopy control owns further options *per
    /// instance*, and a caller holding one must ask
    /// [`Self::refusal_for_instance`] instead.
    #[must_use]
    pub fn refusal_for(self, kind: AnalysisKind) -> Option<&'static str> {
        if self.spec().time_stepped_only && !kind.advances_time() {
            return Some(NOT_TIME_STEPPED);
        }
        match self {
            Self::MaximumTimestep if matches!(kind, AnalysisKind::Transient) => {
                Some(TRANSIENT_OWNS_STEP_CEILING)
            }
            // Both kinds that offer an accuracy tier, not just the operating
            // point: the transfer function resolves its tier through the same
            // `AccuracyPolicy::apply`, so an ITL1 recorded against it would be
            // accepted, persisted, reported — and then overwritten.
            Self::Itl1
                if matches!(
                    kind,
                    AnalysisKind::OperatingPoint | AnalysisKind::TransferFunction
                ) =>
            {
                Some(Self::ACCURACY_TIER_OWNS_ITERATIONS)
            }
            _ => None,
        }
    }

    /// Why *this instance* cannot carry this option, if it cannot.
    ///
    /// [`Self::refusal_for`] plus the rules only the instance can answer. The
    /// tier and the homotopy control are both applied after the deck's
    /// `.OPTIONS` have been resolved (`simulation/engine_bridge/dc.rs`
    /// `resolved_op_config`), so which of them is selected decides whether an
    /// authored continuation aid survives to the solve at all. Answering that
    /// from the kind alone would either refuse a value that works under
    /// `Balanced`/`Adaptive` or accept one that is discarded.
    #[must_use]
    pub fn refusal_for_instance(
        self,
        kind: AnalysisKind,
        ownership: SolverOwnership,
    ) -> Option<&'static str> {
        self.refusal_for(kind).or_else(|| match self {
            Self::GminStepping | Self::SourceStepping | Self::PseudoTransient | Self::ArcLength => {
                ownership.continuation_aid_owner()
            }
            Self::Damping => ownership.damping_owner(),
            _ => None,
        })
    }

    /// The options this instance may carry, for the authoring picker.
    ///
    /// An instance whose own controls assign nothing passes
    /// [`SolverOwnership::NONE`], and then only the kind decides.
    #[must_use]
    pub fn applicable_to_instance(kind: AnalysisKind, ownership: SolverOwnership) -> Vec<Self> {
        Self::all()
            .filter(|option| option.refusal_for_instance(kind, ownership).is_none())
            .collect()
    }
}

/// What one analysis instance's own controls assign after the deck is read.
///
/// The deck's `.OPTIONS` are not the last word on an operating point or a
/// transfer function. Both resolve an accuracy tier through
/// [`crate::simulation::accuracy::AccuracyPolicy::apply`], and the operating
/// point additionally resolves a homotopy choice through
/// [`OpHomotopy::apply`] — each on top of the fully resolved configuration.
/// Five catalog options land on fields those two assign, so whether such an
/// option reaches the solve is a property of the instance, not of its kind.
///
/// [`Self::NONE`] is the honest answer for every kind that offers neither
/// control: nothing overwrites, so nothing is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SolverOwnership {
    /// The tier this instance carries, for the kinds that offer one.
    pub accuracy: Option<AnalysisAccuracy>,
    /// The homotopy choice this instance carries, for the operating point.
    pub homotopy: Option<OpHomotopy>,
}

impl SolverOwnership {
    /// An instance whose own controls assign nothing after the deck.
    pub const NONE: Self = Self {
        accuracy: None,
        homotopy: None,
    };

    /// Who assigns the four continuation flags, when someone does.
    ///
    /// The homotopy control is named ahead of the tier because it is applied
    /// after it: under `Robust` with `Gmin stepping` selected, the value that
    /// actually runs is the homotopy's, and naming the tier would send a
    /// reader to change the control that is not deciding.
    #[must_use]
    pub fn continuation_aid_owner(self) -> Option<&'static str> {
        if self
            .homotopy
            .is_some_and(OpHomotopy::owns_continuation_aids)
        {
            return Some(HOMOTOPY_OWNS_CONTINUATION);
        }
        self.damping_owner()
    }

    /// Who assigns the damping strategy, when someone does.
    ///
    /// The tier alone: no homotopy choice touches damping, so a reader who
    /// selected one keeps whatever damping the deck resolved to.
    #[must_use]
    pub fn damping_owner(self) -> Option<&'static str> {
        match self.accuracy {
            Some(AnalysisAccuracy::Fast) => Some(FAST_TIER_OWNS_CONTINUATION),
            Some(AnalysisAccuracy::Robust) => Some(ROBUST_TIER_OWNS_CONTINUATION),
            // `Balanced` and `Accurate` inherit the resolved aids, and a kind
            // that offers no tier states nothing about them either.
            _ => None,
        }
    }
}

/// One authored value, in the type its option stores.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverrideValue {
    Real(f64),
    Count(usize),
    Flag(bool),
    Method(IntegrationMethod),
    Damping(DampingStrategy),
    Solver(MatrixSolver),
}

impl OverrideValue {
    /// The text the deck carries for this value.
    ///
    /// Reals are written at full precision rather than the two-digit form the
    /// plan-level emitter uses: a plan value is a preset the reader recognizes,
    /// whereas these are exactly what someone typed, and a deck that rounded
    /// them would resolve to a bound the ledger does not report.
    #[must_use]
    pub fn to_deck_text(self) -> Option<String> {
        Some(match self {
            Self::Real(value) => format!("{value:e}"),
            Self::Count(value) => value.to_string(),
            Self::Flag(value) => u8::from(value).to_string(),
            Self::Method(method) => method.spice_name().to_owned(),
            Self::Damping(strategy) => strategy.spice_name().to_owned(),
            // `Lu` is the automatic setting and names no backend, so it
            // cannot be stated; the record refuses it at authoring time.
            Self::Solver(solver) => solver.spice_name()?.to_owned(),
        })
    }
}
