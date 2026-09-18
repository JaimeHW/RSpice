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
//! — **or** a fourth route holds instead of 2 and 3: **the engine reads the
//! parsed package record directly.** Several of Xyce's option packages never
//! touch `SimulationConfig` at all; the engine takes them off
//! `netlist.options` at the site that needs them, which is where the output
//! schedule and the harmonic-balance initial state are read. Such an entry's
//! [`OptionSpec::consumer`] names *that* site, and the harness proves it by
//! parsing the emitted deck and asserting the parsed package record moved.
//! Condition 1 still holds, and an option with no reader is still not
//! admitted: the route widens what counts as a reader, not whether one is
//! required.
//!
//! Every entry below states its consumer, so the last condition is checked by
//! reading the table rather than by remembering. `numeric_override::tests`
//! then proves the whole rule mechanically: it emits each option, parses the
//! deck, resolves it, and fails on any option that leaves both the resolved
//! `SimulationConfig` and the parsed `SimulationOptions` unchanged.
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
//! - **`HBINT NUMFREQ`** — read, but not on the route a Studio run takes. The
//!   only reader is `HbConfig::from_hb_card`
//!   (`rspice-core/src/analysis/harmonic_balance/config.rs:328`), which the
//!   command line, the Python bindings, the browser API and the engine adapter
//!   use to translate a deck. The Studio builds its `HbConfig` from the HB
//!   form's own per-tone Harmonics field instead
//!   (`services/simulation_runner/hb.rs:359`), so a harmonic order authored
//!   here would be emitted and then ignored by the run that carried it.
//! - **`HBINT SAVEICDATA`, `LINSOL-HB PREC_TYPE`, `DEVICE DEBUGLEVEL`,
//!   `TIMEINT DEBUGLEVEL`, `MEASURE MEASFAIL`, `MEASURE USE_CONT_FILES`,
//!   `LINSOL TR_PARTITION`** — parsed onto a `SimulationOptions` field and
//!   dropped there. Neither the resolver nor anything under
//!   `rspice-core/src/engine/` reads any of them, so every one of these fails
//!   the fourth route as squarely as it fails the second.
//! - **`OUTPUT PRINTHEADER`, `OUTPUT PRINTFOOTER`** — read only by the Xyce
//!   `.prn` file writer (`rspice-core/src/io/xyce_prn.rs:541`). No Studio run
//!   path writes a `.prn`, so both are controls over a file this application
//!   does not produce.
//! - **`NONLIN-HB MAXSTEP`** — read at `engine/hb.rs:1137`, and read *after*
//!   the HB form's own Max iterations field has already set
//!   `HbConfig::max_iterations`. Admitting it would give one Newton budget two
//!   editors on one form, which is the defect `MAXTIMESTEP` is excluded for.

mod global;
mod hbint;
mod nonlin_tran;
mod output;
mod timeint;

use crate::simulation::accuracy::AnalysisAccuracy;
use crate::simulation::dialog::{
    DampingStrategy, HbTimeDomainMode, IntegrationMethod, MatrixSolver, OpHomotopy, format_si_value,
};

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
    // Transient Newton (`NONLIN-TRAN`)
    TransientNewtonReltol,
    TransientNewtonAbstol,
    TransientNewtonUpdateBound,
    TransientNewtonResidualBound,
    TransientNewtonBudget,
    TransientDeviceConvergence,
    TransientNoxSolver,
    // Output schedule (`OUTPUT`)
    StrobeInterval,
    OutputTimePoints,
    RetainEverySignal,
    // Harmonic-balance integration (`HBINT`)
    HbInitialState,
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
    TransientNewton,
    Output,
    HarmonicBalance,
    Matrix,
    DeviceBypass,
}

impl OverrideSection {
    /// Every section, in the order the editor stacks them.
    ///
    /// The three sections in the middle are each one analysis family's own:
    /// the transient's Newton package, the output schedule a time-domain run
    /// samples on, and the harmonic-balance initial state. They sit after the
    /// integration section because that is the order a reader meets them in —
    /// how the step is bounded, then how the step's solve is bounded, then
    /// which of the solved steps are reported — and before the matrix section,
    /// which is the plan's own policy and belongs last.
    pub const ALL: [Self; 8] = [
        Self::Convergence,
        Self::Charge,
        Self::Integration,
        Self::TransientNewton,
        Self::Output,
        Self::HarmonicBalance,
        Self::Matrix,
        Self::DeviceBypass,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Convergence => "Convergence",
            Self::Charge => "Charge",
            Self::Integration => "Integration",
            Self::TransientNewton => "Transient Newton",
            Self::Output => "Output",
            Self::HarmonicBalance => "Harmonic balance",
            Self::Matrix => "Matrix",
            Self::DeviceBypass => "Device bypass",
        }
    }

    /// This section's place in [`Self::ALL`].
    ///
    /// `PartialEq` is not usable in a `const fn`, and the catalog is assembled
    /// by one, so the order comes out of this rather than out of a comparison.
    const fn rank(self) -> usize {
        match self {
            Self::Convergence => 0,
            Self::Charge => 1,
            Self::Integration => 2,
            Self::TransientNewton => 3,
            Self::Output => 4,
            Self::HarmonicBalance => 5,
            Self::Matrix => 6,
            Self::DeviceBypass => 7,
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
    NonlinTran,
    Output,
    Hbint,
}

impl OptionPackage {
    /// Every package, in the order the emitter writes their cards.
    ///
    /// The emitter iterates this rather than a list of its own, so a package
    /// added to the catalog cannot be left without a card. The global one is
    /// first because it re-scopes nothing and every scoped card that follows
    /// re-scopes itself.
    pub const ALL: [Self; 5] = [
        Self::Global,
        Self::Timeint,
        Self::NonlinTran,
        Self::Output,
        Self::Hbint,
    ];

    /// The `.OPTIONS` card header this package's keys ride on.
    ///
    /// `NONLIN-TRAN` rather than `NONLIN-TRANSIENT`: the parser scopes both
    /// spellings onto the same keys, and the shorter one is what Xyce's own
    /// documentation writes.
    #[must_use]
    pub const fn header(self) -> &'static str {
        match self {
            Self::Global => ".OPTIONS",
            Self::Timeint => ".OPTIONS TIMEINT",
            Self::NonlinTran => ".OPTIONS NONLIN-TRAN",
            Self::Output => ".OPTIONS OUTPUT",
            Self::Hbint => ".OPTIONS HBINT",
        }
    }

    /// The package name alone, which is what the parser matches. Empty for the
    /// global set, which has no name because it has no selector.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Global => "",
            Self::Timeint => "TIMEINT",
            Self::NonlinTran => "NONLIN-TRAN",
            Self::Output => "OUTPUT",
            Self::Hbint => "HBINT",
        }
    }

    /// This package's table, in the order it declares its own keys.
    const fn table(self) -> &'static [OptionSpec] {
        match self {
            Self::Global => &global::GLOBAL,
            Self::Timeint => &timeint::TIMEINT,
            Self::NonlinTran => &nonlin_tran::NONLIN_TRAN,
            Self::Output => &output::OUTPUT,
            Self::Hbint => &hbint::HBINT,
        }
    }
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
    /// One of [`HbTimeDomainMode::all`].
    TimeDomainMode,
    /// An increasing list of times, each finite and at or above zero.
    ///
    /// The one option that takes a list rather than a bound. It is a value
    /// kind of its own because the well it is typed into validates the whole
    /// sequence — a repeated or out-of-order stop is refused by the engine's
    /// own breakpoint schedule, and a control that accepted one would author a
    /// deck that fails to run.
    TimeList,
}

/// Why an analysis of some kind cannot carry an option.
///
/// Two things make an option inapplicable: the solve never reads it, or some
/// other record already owns it. Both are worded once here so a reader meets
/// the same sentence wherever the refusal surfaces.
pub(super) const NOT_TIME_STEPPED: &str =
    "this analysis never advances time, so a time-integration bound would not reach its solve";
pub(super) const NOT_HARMONIC_BALANCE: &str =
    "only a harmonic-balance solve reads this package, and this analysis does not run one";
pub(super) const TRANSIENT_OWNS_STEP_CEILING: &str =
    "the transient's own Max step field owns this, and one bound cannot have two copies";
/// `.OPTIONS METHOD` on a PSS analysis.
///
/// A shooting solve never reads it. `Engine::pss_integration_method`
/// (`rspice-core/src/engine/pss.rs:662`) selects the formula from the `.PSS`
/// card's own `METHOD=`, and with that keyword absent it falls to the PSS
/// default — trapezoidal on a fixed shooting grid, otherwise TrapGear's
/// adaptive choice. `SimulationConfig::method` is not consulted anywhere in
/// `engine/pss.rs`. So the option is not merely a second editor of the form's
/// `Integration method` chooser; it is a control a PSS run would ignore, and
/// the chooser is the one that reaches the solve.
pub(super) const PSS_OWNS_ITS_INTEGRATION_FORMULA: &str = "a shooting solve reads the formula off its own .PSS card, never .OPTIONS METHOD; the PSS \
     form's own Integration method chooser is the control that reaches it";
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
    /// Where `rspice-core` reads that field, or — under the fourth admission
    /// route — where it reads the parsed package record. This is the whole
    /// admission argument for the entry, so it names a file and a line rather
    /// than a subsystem.
    pub consumer: &'static str,
    /// Which solves read this option at all.
    pub reach: OptionReach,
}

/// Which solves read one option.
///
/// A kind whose solve never reaches an option's consumer earns no control for
/// it, and the refusal says which solve does. One field rather than a bool per
/// family, so an option cannot claim two reaches at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionReach {
    /// Every kind's solve reads it.
    EverySolve,
    /// Only a solve that advances time, which is [`AnalysisKind::advances_time`].
    TimeStepped,
    /// Only the harmonic-balance family's solve.
    ///
    /// Three kinds run one today: harmonic balance itself, and HBSP and
    /// HBNOISE, which both solve an HB fixed point first
    /// (`services/simulation_runner/hbnoise.rs:279`). The quasi-periodic kinds
    /// will join them when their solves exist; until one runs, a control on
    /// its form would author a value no run reads.
    HarmonicBalanceFamily,
}

impl OptionReach {
    /// Why this kind cannot reach an option with this reach, if it cannot.
    const fn refusal_for(self, kind: AnalysisKind) -> Option<&'static str> {
        match self {
            Self::EverySolve => None,
            Self::TimeStepped => {
                if kind.advances_time() {
                    None
                } else {
                    Some(NOT_TIME_STEPPED)
                }
            }
            Self::HarmonicBalanceFamily => {
                if matches!(
                    kind,
                    AnalysisKind::HarmonicBalance | AnalysisKind::Hbsp | AnalysisKind::Hbnoise
                ) {
                    None
                } else {
                    Some(NOT_HARMONIC_BALANCE)
                }
            }
        }
    }
}

/// The catalog. Order here is the order the ledger and the editor report.
///
/// The package tables are interleaved rather than appended, because that order
/// is by *section*: the four `TIMEINT` keys are the integration section's own,
/// and they report after the global keys that open that section and before the
/// matrix section's. The tables carry ownership; this carries the order.
pub(super) const SPECS: &[OptionSpec] = &LEDGER_ORDER;

/// How many options the catalog holds, across every package.
const CATALOG_LEN: usize = catalog_len();

const fn catalog_len() -> usize {
    let mut total = 0;
    let mut package = 0;
    while package < OptionPackage::ALL.len() {
        total += OptionPackage::ALL[package].table().len();
        package += 1;
    }
    total
}

const LEDGER_ORDER: [OptionSpec; CATALOG_LEN] = ledger_order();

/// Every package table, section by section.
///
/// Section-major rather than table-major, and derived rather than written
/// down: a global key added to the convergence section would otherwise push
/// four `TIMEINT` rows past the matrix section with nothing saying so, and a
/// whole package appended at the end would report its keys after the device
/// bypass ones whatever section they claim.
const fn ledger_order() -> [OptionSpec; CATALOG_LEN] {
    let mut catalog = [global::GLOBAL[0]; CATALOG_LEN];
    let mut written = 0;
    let mut rank = 0;
    while rank < OverrideSection::ALL.len() {
        let mut package = 0;
        while package < OptionPackage::ALL.len() {
            let table = OptionPackage::ALL[package].table();
            let mut index = 0;
            while index < table.len() {
                if table[index].section.rank() == rank {
                    catalog[written] = table[index];
                    written += 1;
                }
                index += 1;
            }
            package += 1;
        }
        rank += 1;
    }
    assert!(
        written == CATALOG_LEN,
        "every catalog entry names a section in OverrideSection::ALL"
    );
    catalog
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
        match self.packaged_name() {
            Some(name) => name,
            None => self.spec().key,
        }
    }

    /// The qualified spelling for a packaged key, as a literal so the return
    /// stays `'static`.
    ///
    /// Every option whose package has a name earns one, and the catalog test
    /// checks that it reads `<package> <key>`, so a new entry that forgets its
    /// arm here is reported by its bare key and caught rather than shipped
    /// looking like a global one.
    const fn packaged_name(self) -> Option<&'static str> {
        match self {
            Self::LteReltol => Some("TIMEINT RELTOL"),
            Self::LteAbstol => Some("TIMEINT ABSTOL"),
            Self::MinTimestep => Some("TIMEINT MINTIMESTEP"),
            Self::MaximumTimestep => Some("TIMEINT DELMAX"),
            Self::TransientNewtonReltol => Some("NONLIN-TRAN RELTOL"),
            Self::TransientNewtonAbstol => Some("NONLIN-TRAN ABSTOL"),
            Self::TransientNewtonUpdateBound => Some("NONLIN-TRAN DELTAXTOL"),
            Self::TransientNewtonResidualBound => Some("NONLIN-TRAN RHSTOL"),
            Self::TransientNewtonBudget => Some("NONLIN-TRAN MAXSTEP"),
            Self::TransientDeviceConvergence => Some("NONLIN-TRAN ENFORCEDEVICECONV"),
            Self::TransientNoxSolver => Some("NONLIN-TRAN NOX"),
            Self::StrobeInterval => Some("OUTPUT INITIAL_INTERVAL"),
            Self::OutputTimePoints => Some("OUTPUT OUTPUTTIMEPOINTS"),
            Self::RetainEverySignal => Some("OUTPUT SNAPSHOTS"),
            Self::HbInitialState => Some("HBINT TAHB"),
            _ => None,
        }
    }

    /// The card this option's key rides on.
    #[must_use]
    pub fn package(self) -> OptionPackage {
        self.spec().package
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
        if let Some(reason) = self.spec().reach.refusal_for(kind) {
            return Some(reason);
        }
        match self {
            Self::MaximumTimestep if matches!(kind, AnalysisKind::Transient) => {
                Some(TRANSIENT_OWNS_STEP_CEILING)
            }
            // The one kind that advances time and still reads its integration
            // formula from somewhere other than `.OPTIONS METHOD`.
            Self::IntegrationMethod if matches!(kind, AnalysisKind::Pss) => {
                Some(PSS_OWNS_ITS_INTEGRATION_FORMULA)
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
///
/// `Clone` rather than `Copy`: one option states a list of times, and a list
/// is not a register. The alternative was a fixed-length inline array with a
/// count, which would have put an arbitrary ceiling on how many output stops
/// an analysis may name — the engine's own ceiling is the run's analysis-point
/// budget, and that is the only one worth enforcing.
#[derive(Debug, Clone, PartialEq)]
pub enum OverrideValue {
    Real(f64),
    Count(usize),
    Flag(bool),
    Method(IntegrationMethod),
    Damping(DampingStrategy),
    Solver(MatrixSolver),
    TimeDomainMode(HbTimeDomainMode),
    /// An increasing list of output stops, in seconds.
    TimeList(Vec<f64>),
}

impl OverrideValue {
    /// The text the deck carries for this value.
    ///
    /// Reals are written at full precision rather than the two-digit form the
    /// plan-level emitter uses: a plan value is a preset the reader recognizes,
    /// whereas these are exactly what someone typed, and a deck that rounded
    /// them would resolve to a bound the ledger does not report.
    #[must_use]
    pub fn to_deck_text(&self) -> Option<String> {
        Some(match self {
            Self::Real(value) => format!("{value:e}"),
            Self::Count(value) => value.to_string(),
            Self::Flag(value) => u8::from(*value).to_string(),
            Self::Method(method) => method.spice_name().to_owned(),
            Self::Damping(strategy) => strategy.spice_name().to_owned(),
            // `Lu` is the automatic setting and names no backend, so it
            // cannot be stated; the record refuses it at authoring time.
            Self::Solver(solver) => solver.spice_name()?.to_owned(),
            Self::TimeDomainMode(mode) => mode.spice_name().to_owned(),
            // Comma-separated, which is the one separator the parser's time
            // vector reads without the tokenizer joining two stops into one.
            Self::TimeList(times) => times
                .iter()
                .map(|time| format!("{time:e}"))
                .collect::<Vec<_>>()
                .join(","),
        })
    }

    /// How the ledger and the option fields report this value.
    ///
    /// Engineering notation for every real, the same spelling the preset it is
    /// reported beside is written in.
    #[must_use]
    pub fn to_report_text(&self) -> String {
        match self {
            Self::Real(value) => format_si_value(*value),
            Self::Count(value) => value.to_string(),
            Self::Flag(value) => if *value { "on" } else { "off" }.to_owned(),
            Self::Method(method) => method.spice_name().to_owned(),
            Self::Damping(strategy) => strategy.display_name().to_owned(),
            Self::Solver(solver) => solver.display_name().to_owned(),
            Self::TimeDomainMode(mode) => mode.display_name().to_owned(),
            // Spaces rather than the deck's commas: a well a reader types a
            // schedule into reads as a schedule, and both separators are
            // accepted back.
            Self::TimeList(times) => times
                .iter()
                .copied()
                .map(format_si_value)
                .collect::<Vec<_>>()
                .join(" "),
        }
    }
}
