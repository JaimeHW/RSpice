//! Every excitation placed in the design, and what reads each one.
//!
//! Two questions are answered together here because answering them apart is
//! what produced the dead end this replaces: a list of sources with no way to
//! tell which are actually driving anything, beside a plan whose source fields
//! name strings that may not correspond to a placed instance at all.
//!
//! The consumer side has exactly one walk of the plan, [`attribute_plan`], and
//! the two match-arm sets it is handed — [`consumers_for`] for sources,
//! [`port_consumers_for`] for RF ports — are private. Both surfaces that show
//! these lists — the Simulation Studio page and the Design navigator's rail —
//! call [`placed_sources`] and [`placed_rf_ports`] and render what they
//! return; neither re-derives a reference, so neither can disagree with the
//! other about what an analysis is reading.
//!
//! A consumer comes in one of two shapes, and the difference is what a reader
//! needs in order to decide whether a row is a finding:
//!
//! - **Named.** The analysis's own draft holds this instance's reference: a DC
//!   sweep variable, a noise input, a PSS tone, an envelope modulation source.
//!   Rename the instance and the binding breaks.
//! - **Whole-design.** The analysis reads every placed source and names none of
//!   them. An `.ac` card sweeps whatever carries an AC magnitude, so that
//!   binding is derived from the instance's own `ac` parameter and reported
//!   against every AC analysis in the plan. A transient — and every analysis
//!   built on one — re-evaluates every source at every accepted timestep
//!   (`VoltageSources::update_transient_rhs`,
//!   `rspice-core/src/circuit/storage/sources.rs:845-867`;
//!   `CurrentSources::update_transient_rhs`, same file `:2280-2305`), and an
//!   operating point reads every source's DC value
//!   (`rspice-core/src/engine/source_values.rs`, whose `extract_dc_value`
//!   match over `SourceSpec` is total). Those are recorded against every
//!   source, because without them a PULSE in a transient-only plan reads as one
//!   that nothing looks at.
//!
//! `.four` is not a third shape. The Fourier analysis runs a transient and
//! decomposes one output of it (`run_fourier_analysis_with_abort` in
//! `services::simulation_runner::envelope_fourier`), so it reads every source
//! and names none; the one fundamental-to-source binding in the product is
//! harmonic balance, which is listed as a named consumer.
//!
//! Matching is case-insensitive against [`Component::spice_instance_name`],
//! which is the spelling the deck carries and therefore the spelling a plan's
//! source field was picked from.
//!
//! RF ports are derived beside the sources rather than among them, by
//! [`placed_rf_ports`]. A port is not a waveform: it is a Z0 termination an
//! S-parameter run indexes by port number, and only a port carrying a drive
//! spec is a generator behind that impedance. Counting one as a source would
//! make the number of placed excitations wrong — but leaving it out of this
//! module altogether is what let an S-parameter testbench, whose every
//! excitation is a placed port, report that the design places nothing at all.
//! So a port gets the same two answers a source gets, off the same walk of the
//! plan, in a list of its own.
//!
//! Each list is answered from two different questions, and the difference is
//! which design is being asked about.
//!
//! - **One buffer.** [`placed_sources`] and [`placed_rf_ports`] read the
//!   schematic handed to them: the sheet in front of the reader, live, with
//!   uncommitted edits included. That is what the Design navigator's rails
//!   list first, because the rail's first duty is the drawing on screen.
//! - **The whole design.** [`design_sources`] and [`design_rf_ports`] read the
//!   frozen [`DesignProjection`]: every occurrence the execution plan binds,
//!   each master's own placed excitations tagged with the occurrence path the
//!   run reaches them through. A source drawn inside a child master is driven
//!   by the run and was in neither list, so the root of a hierarchical design
//!   reported that it places no sources at all — on the studio page whose one
//!   job is to say what drives the circuit.
//!
//! Multiplicity is per occurrence rather than per drawing, because that is
//! what the run has. A run flattens the hierarchy, so one source drawn in a
//! master two instances reach is two cards in the deck: `XA.V1` and `XB.V1`
//! sit across different nodes, bias different devices, and are separately
//! selectable. Listing the drawing once would state a number no run ever has,
//! and would leave the reader no row to click through to the second instance.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Weak};

use crate::simulation::netlist_gen::{design_nets, projection_nets};
use crate::simulation::plan::{AnalysisDraft, SimulationPlan};
use crate::state::workspace::DesignProjection;
use crate::state::{
    CellViewRef, Component, ComponentType, InstancePath, LibraryManager, SchematicState,
};

/// Net names keyed by (component id, terminal name), as
/// [`net_names_by_terminal`] resolves them for one schematic.
type TerminalNets = HashMap<(u64, String), String>;

/// One analysis that reads a source, and what it reads it as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceConsumer {
    /// The analysis instance's own display name, not its kind's label: a plan
    /// may hold several analyses of one kind and the row has to say which.
    pub analysis: String,
    /// The part the source plays in that analysis.
    pub role: &'static str,
    /// Whether the instance is enabled in the plan.
    ///
    /// A disabled instance is not a reader: the run it would belong to does not
    /// contain it, so a source only it names is a source this plan runs without
    /// looking at. It is still listed, dimmed, because deleting the row instead
    /// would make a disabled transient look like a plan that never read the
    /// source at all — and re-enabling it is the one edit that changes the
    /// answer.
    pub enabled: bool,
}

impl SourceConsumer {
    /// Whether this consumer's analysis is in the run the plan would dispatch.
    #[must_use]
    pub const fn reads(&self) -> bool {
        self.enabled
    }
}

/// One independent source placed in the design.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedSource {
    pub component_id: u64,
    /// The deck spelling, which is what a plan's source field holds.
    pub reference: String,
    pub is_voltage: bool,
    /// The waveform family, which for these components is the kind itself.
    pub family: &'static str,
    /// The one number that identifies this source at a glance.
    pub key_figure: String,
    /// Terminal nets in pin order.
    pub nets: Vec<String>,
    pub consumers: Vec<SourceConsumer>,
    /// The occurrence the run reaches this instance through, or `None` when
    /// the list was derived from one schematic buffer and therefore has no
    /// hierarchy to place it in.
    ///
    /// `None` is not the root. A buffer read on its own is its own root as far
    /// as that reading goes, and saying so is what keeps a rail listing the
    /// sheet in front of the reader when the configured design does not
    /// resolve — but it is not a claim about where the run reaches it, which
    /// is what [`design_sources`] answers.
    pub occurrence: Option<InstancePath>,
}

impl PlacedSource {
    /// Whether the run this plan would dispatch reads this source.
    ///
    /// Disabled instances do not count. `consumers` used to be walked as a
    /// whole here, so one disabled transient in the plan marked every placed
    /// source "read" — which is the opposite of the finding this list exists to
    /// surface.
    #[must_use]
    pub fn is_read(&self) -> bool {
        self.consumers.iter().any(SourceConsumer::reads)
    }

    /// The consumers the run actually contains.
    pub fn reading_consumers(&self) -> impl Iterator<Item = &SourceConsumer> {
        self.consumers.iter().filter(|consumer| consumer.reads())
    }
    /// The quantity letter, for a column that has room for one character.
    pub const fn quantity(&self) -> &'static str {
        if self.is_voltage { "V" } else { "I" }
    }

    /// `PULSE · PER 2 µs`, the row's one-line identity.
    pub fn summary(&self) -> String {
        if self.key_figure.is_empty() {
            self.family.to_owned()
        } else {
            format!("{} · {}", self.family, self.key_figure)
        }
    }

    /// What an Occurrence column states for this row.
    #[must_use]
    pub fn occurrence_label(&self) -> String {
        occurrence_label(self.occurrence.as_ref())
    }
}

/// What a placed RF port does in the design it sits in.
///
/// One port element covers the whole span from a passive load to a
/// large-signal generator, and which of those a row is decides whether an
/// unread port is a finding or the ordinary state. The order is the netlist
/// generator's own precedence (`netlist_gen::instances`, the `RfPort` arm): a
/// power drive outranks an AC magnitude, which outranks a DC bias, and a port
/// carrying none of the three is a termination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RfPortMode {
    /// `PWR=` — an available-power generator behind Z0.
    PowerDrive,
    /// `AC` — a small-signal magnitude behind Z0.
    AcDrive,
    /// `DC` — a bias behind Z0, with no signal of its own.
    DcBias,
    /// No source spec at all: the port is a Z0 load.
    Termination,
}

impl RfPortMode {
    /// The word a column with room for one has to carry.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PowerDrive => "drive",
            Self::AcDrive => "AC drive",
            Self::DcBias => "DC bias",
            Self::Termination => "term",
        }
    }
}

/// One RF port placed in the design.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedRfPort {
    pub component_id: u64,
    /// The deck spelling, which is what the `P` card carries.
    pub reference: String,
    /// The index an S-parameter run addresses this port by. Two ports may
    /// carry the same one; see [`duplicate_port_numbers`].
    pub port_number: u32,
    /// Reference impedance, re-formatted rather than echoed so `5e1` and `50`
    /// print identically.
    pub z0: String,
    pub mode: RfPortMode,
    /// Terminal nets in pin order.
    pub nets: Vec<String>,
    pub consumers: Vec<SourceConsumer>,
    /// The occurrence the run reaches this port through, read exactly as
    /// [`PlacedSource::occurrence`] is.
    pub occurrence: Option<InstancePath>,
}

impl PlacedRfPort {
    /// Whether the run this plan would dispatch reads this port.
    ///
    /// Run-scoped for the same reason [`PlacedSource::is_read`] is: a disabled
    /// S-parameter analysis is not in the run, so the ports it would have
    /// indexed are terminations for the length of that run.
    #[must_use]
    pub fn is_read(&self) -> bool {
        self.consumers.iter().any(SourceConsumer::reads)
    }

    /// The quantity letter, for a column that has room for one character.
    ///
    /// The card's own letter, as a source row states `V` or `I`.
    pub const fn quantity(&self) -> &'static str {
        "P"
    }

    /// `term · Z0 50`, the row's one-line identity.
    ///
    /// What the port does and the one number that tells two of them apart,
    /// which is the same pair [`PlacedSource::summary`] states.
    pub fn summary(&self) -> String {
        if self.z0.is_empty() {
            self.mode.label().to_owned()
        } else {
            format!("{} \u{00b7} Z0 {}", self.mode.label(), self.z0)
        }
    }

    /// What an Occurrence column states for this row.
    #[must_use]
    pub fn occurrence_label(&self) -> String {
        occurrence_label(self.occurrence.as_ref())
    }
}

/// The Occurrence column's one spelling, for both row kinds.
///
/// `/` for an instance the design root places directly, and for a row derived
/// from one buffer — a sheet read on its own is its own root, so `/` is what
/// it is being read as. Blank was the other candidate and says less: an empty
/// cell in a ledger reads as a value the page failed to find, and `/` is the
/// spelling the rest of the product already renders the design root in
/// (`InstancePath`'s own `Display`, the breadcrumb, every stored path).
fn occurrence_label(occurrence: Option<&InstancePath>) -> String {
    occurrence.map_or_else(|| InstancePath::root().to_string(), InstancePath::to_string)
}

/// How many independent sources this sheet places.
///
/// The same predicate [`placed_sources`] admits a component under, asked
/// without resolving anything: resolving the list to read its length walks the
/// design's nets, parses every source's parameters and sorts the result, for a
/// count that only the set of placed components can change.
///
/// This is one sheet, so it is what [`whole_design_source_count`] falls back
/// to when the configured hierarchy does not resolve — never what a surface
/// states about the design, which is a different and larger number wherever a
/// source is drawn below the open sheet.
#[must_use]
pub fn placed_source_count(schematic: &SchematicState) -> usize {
    schematic
        .components
        .iter()
        .filter(|component| source_family(component.kind).is_some())
        .count()
}

/// Every independent source on this sheet, with what the plan reads it as.
///
/// Sources come back in reference order so the list is stable across edits
/// that do not add or remove one.
///
/// Not cached, and not free: the net map below walks the whole design. The
/// studio's Excitations page and that page's heading share one resolution;
/// nothing else on a frame needs the list rather than
/// [`placed_source_count`]. There is no schematic revision to key a shared
/// answer on, so what is left is paid where it is spent rather than hidden
/// behind a cache that could serve a stale count. The one case it does not pay
/// for is a design that places no source at all, which is every sheet still
/// being drawn.
pub fn placed_sources(
    schematic: &SchematicState,
    plan: Option<&SimulationPlan>,
) -> Vec<PlacedSource> {
    #[cfg(test)]
    crate::simulation::cost_probe::record(crate::simulation::cost_probe::Derivation::PlacedSources);
    if !schematic
        .components
        .iter()
        .any(|component| source_family(component.kind).is_some())
    {
        return Vec::new();
    }
    let nets = net_names_by_terminal(schematic);
    let mut sources: Vec<PlacedSource> = schematic
        .components
        .iter()
        .filter_map(|component| {
            let (source, drive) = placed_source(component, &nets)?;
            Some(with_source_consumers(source, drive, plan))
        })
        .collect();
    sources.sort_by_key(source_order);
    sources
}

/// Every independent source the whole design places, with the occurrence each
/// one is reached through and what the plan reads it as.
///
/// This is the list the run has. The plan the projection carries is the one
/// authority on which occurrences exist, so the walk is over its bindings
/// rather than over the library: a cell view nothing instantiates places no
/// source in this design, and a master two instances reach places its sources
/// twice.
///
/// Sources come back by occurrence and then by reference, which puts the
/// design root's own first and keeps each occurrence's sources in the order
/// [`placed_sources`] lists a sheet's.
pub fn design_sources(
    libraries: &LibraryManager,
    projection: &Arc<DesignProjection>,
    plan: Option<&SimulationPlan>,
) -> Vec<PlacedSource> {
    design_excitations(libraries, projection)
        .sources
        .iter()
        .map(|(source, drive)| with_source_consumers(source.clone(), *drive, plan))
        .collect()
}

/// Every RF port the whole design places, read exactly as [`design_sources`]
/// reads its sources.
///
/// Ports stay in port-number order across the whole design rather than being
/// grouped by occurrence, because the number is what an S-parameter run
/// addresses them by and the run has one index for the flattened design. That
/// is also why [`duplicate_port_numbers`] over this list is a real finding: two
/// occurrences each carrying `P1` claim one index of one matrix.
pub fn design_rf_ports(
    libraries: &LibraryManager,
    projection: &Arc<DesignProjection>,
    plan: Option<&SimulationPlan>,
) -> Vec<PlacedRfPort> {
    let consumers = plan.map(port_consumers_for).unwrap_or_default();
    design_excitations(libraries, projection)
        .ports
        .iter()
        .map(|port| PlacedRfPort {
            consumers: consumers.clone(),
            ..port.clone()
        })
        .collect()
}

/// The whole design's excitations, with the reading a surface falls back to
/// when the configured hierarchy does not resolve.
///
/// [`design_sources`] and [`design_rf_ports`] need a projection, and a project
/// can be in a state that has none — an unresolved binding, a missing root.
/// Answering "nothing drives this circuit" because a binding elsewhere is
/// unresolved is the wrong answer to a different question, so the editor's own
/// buffer is read instead and the rows say so by carrying no occurrence.
///
/// One function rather than that choice spelled out per surface: the Studio's
/// Excitations page lists what this returns and the navigator rail that opens
/// it counts what [`whole_design_source_count`] returns, and a rail that
/// resolved the design differently from the page behind it is a number the
/// reader cannot act on.
pub fn whole_design_excitations(
    libraries: &LibraryManager,
    workspace: &crate::state::ProjectWorkspace,
    active_schematic: &SchematicState,
    plan: Option<&SimulationPlan>,
) -> (Vec<PlacedSource>, Vec<PlacedRfPort>) {
    match workspace.design_projection(libraries, &workspace.active_view, active_schematic) {
        Ok(projection) => (
            design_sources(libraries, &projection, plan),
            design_rf_ports(libraries, &projection, plan),
        ),
        Err(_) => (
            placed_sources(active_schematic, plan),
            placed_rf_ports(active_schematic, plan),
        ),
    }
}

/// How many independent sources the whole design places.
///
/// The rail counted the open sheet while the page it opens counted the design,
/// so a root drawing one supply over two instances that each place a source
/// was offered "1" on the way into a heading reading `DESIGN · 3 sources`.
/// Both now state this number, and it is the length of exactly the list
/// [`whole_design_excitations`] hands the page.
///
/// Affordable on every frame because nothing here is recomputed while the
/// design stands still: the projection is memoized on its own key — a digest
/// over every cell view, the live editor buffer included — and the walk over
/// the projection is memoized on the projection, so a frame that asks for both
/// the count and the rows walks the design once and a frame that changes
/// nothing walks it not at all. The plan is deliberately not consulted: a plan
/// decides what *reads* a source, never whether one is placed.
#[must_use]
pub fn whole_design_source_count(
    libraries: &LibraryManager,
    workspace: &crate::state::ProjectWorkspace,
    active_schematic: &SchematicState,
) -> usize {
    match workspace.design_projection(libraries, &workspace.active_view, active_schematic) {
        Ok(projection) => design_excitations(libraries, &projection).sources.len(),
        Err(_) => placed_source_count(active_schematic),
    }
}

/// Every excitation the whole design places, before any plan is read against
/// it — the half the design projection decides.
///
/// The consumer lists are deliberately not part of it. A consumer is decided
/// by the simulation plan, which is not an input to the projection and carries
/// no revision this module may key a memo on: the plan is edited through
/// several paths, and a memo trusting a counter would serve a role from before
/// the edit that changed it. The plan-side walk therefore runs per call, which
/// is a walk of the plan's instances per row over two small collections — what
/// [`placed_sources`] already paid on every frame, beside the net resolution
/// that is now paid once.
struct DesignExcitations {
    /// Each placed source with an empty consumer list, paired with what its
    /// own annotations drive. The parameter string those answers were read
    /// from is not retained; those bits are all the plan-side walk needs.
    sources: Vec<(PlacedSource, SourceDrive)>,
    /// Each placed port with an empty consumer list. Nothing about which port
    /// it is decides its readership, so there is no second half to keep.
    ports: Vec<PlacedRfPort>,
}

thread_local! {
    /// The whole-design half of both lists, against the projection that
    /// decided it.
    ///
    /// Keyed on the projection's identity rather than on a digest of its
    /// contents, because identity *is* the epoch here.
    /// `ProjectWorkspace::design_projection` digests every authority and every
    /// cell view on each call and mints a new `Arc` whenever one of them moved,
    /// so two calls that see one `Arc` saw one design — the same discipline
    /// `netlist_gen::projection_nets` gets by retaining its extraction inside
    /// the projection. A projection the workspace could not key, because its
    /// authorities do not serialize, is a fresh `Arc` every frame and therefore
    /// misses every frame, which is exactly the answer the projection itself
    /// gives for that design.
    ///
    /// The key is a `Weak` rather than a bare pointer: a weak reference keeps
    /// the allocation alive after the projection is dropped, so no later
    /// projection can be minted at that address while it is still the key, and
    /// equal pointers therefore mean one projection rather than two that
    /// happened to reuse an address. It holds nothing else alive.
    ///
    /// Per thread, because that is where a frame is painted and because two
    /// threads deriving against two projections would otherwise evict each
    /// other on every call.
    static DESIGN_EXCITATIONS: RefCell<Option<(Weak<DesignProjection>, Arc<DesignExcitations>)>> =
        const { RefCell::new(None) };
}

/// The retained whole-design half, rebuilt only for a projection this thread
/// has not already answered about.
fn design_excitations(
    libraries: &LibraryManager,
    projection: &Arc<DesignProjection>,
) -> Arc<DesignExcitations> {
    let key: *const DesignProjection = Arc::as_ptr(projection);
    let retained = DESIGN_EXCITATIONS.with_borrow(|slot| {
        slot.as_ref()
            .filter(|(against, _)| std::ptr::eq(against.as_ptr(), key))
            .map(|(_, excitations)| Arc::clone(excitations))
    });
    if let Some(excitations) = retained {
        return excitations;
    }
    let excitations = Arc::new(walk_design(libraries, projection));
    DESIGN_EXCITATIONS.with_borrow_mut(|slot| {
        *slot = Some((Arc::downgrade(projection), Arc::clone(&excitations)));
    });
    excitations
}

/// One walk of every occurrence the execution plan binds.
///
/// Masters are resolved once each and shared by their occurrences: the net
/// summary comes from `projection_nets`, which the projection itself retains,
/// and the terminal map built over it is kept for the length of this walk. A
/// design instantiating one master a hundred times therefore resolves its nets
/// once, not a hundred times.
fn walk_design(
    libraries: &LibraryManager,
    projection: &Arc<DesignProjection>,
) -> DesignExcitations {
    #[cfg(test)]
    crate::simulation::cost_probe::record(crate::simulation::cost_probe::Derivation::PlacedSources);
    let mut terminals: HashMap<String, Arc<TerminalNets>> = HashMap::new();
    let mut sources = Vec::new();
    let mut ports = Vec::new();
    for binding in projection.plan().bindings() {
        let master = binding.resolved_reference();
        // A binding whose resolved view is not a schematic — a stop boundary,
        // a SPICE or Verilog-A view — has no drawing to place an excitation
        // in, and the projection is the authority on which buffers exist.
        let Some(schematic) = materialized(projection, master) else {
            continue;
        };
        if !places_excitation(schematic) {
            continue;
        }
        let nets = Arc::clone(
            terminals
                .entry(master.key().to_ascii_lowercase())
                .or_insert_with(|| {
                    Arc::new(projection_terminal_nets(
                        libraries,
                        projection,
                        &master.key(),
                    ))
                }),
        );
        let occurrence = binding.instance_path().clone();
        for component in &schematic.components {
            if let Some((mut source, drive)) = placed_source(component, &nets) {
                source.occurrence = Some(occurrence.clone());
                sources.push((source, drive));
            } else if component.kind == ComponentType::RfPort {
                let mut port = placed_rf_port(component, &nets);
                port.occurrence = Some(occurrence.clone());
                ports.push(port);
            }
        }
    }
    sources.sort_by(|(left, _), (right, _)| {
        (
            occurrence_order(left.occurrence.as_ref()),
            source_order(left),
        )
            .cmp(&(
                occurrence_order(right.occurrence.as_ref()),
                source_order(right),
            ))
    });
    ports.sort_by_key(port_order);
    DesignExcitations { sources, ports }
}

/// Whether a master places anything either list would carry. Asked before the
/// net summary is resolved, because resolving it is the expensive half and a
/// master that places no excitation has no use for it.
fn places_excitation(schematic: &SchematicState) -> bool {
    schematic.components.iter().any(|component| {
        source_family(component.kind).is_some() || component.kind == ComponentType::RfPort
    })
}

/// The projection's materialized buffer for one cell view.
///
/// The projection keys its buffers in the design's own spelling, so the lookup
/// folds case rather than assuming one — the same rule the hierarchy tree
/// reads its masters by.
fn materialized<'a>(
    projection: &'a DesignProjection,
    reference: &CellViewRef,
) -> Option<&'a SchematicState> {
    let key = reference.key();
    projection
        .schematic_buffers()
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(&key))
        .map(|(_, schematic)| schematic)
}

/// The row order within one occurrence: reference, case-folded.
fn source_order(source: &PlacedSource) -> String {
    source.reference.to_ascii_uppercase()
}

/// The row order across occurrences: the folded path, which sorts the design
/// root first because `/` is a prefix of every path below it.
fn occurrence_order(occurrence: Option<&InstancePath>) -> String {
    occurrence.map(InstancePath::fold_key).unwrap_or_default()
}

/// The row order of a port: its matrix index first, then where it is, then its
/// reference.
fn port_order(port: &PlacedRfPort) -> (u32, String, String) {
    (
        port.port_number,
        occurrence_order(port.occurrence.as_ref()),
        port.reference.to_ascii_uppercase(),
    )
}

/// What this instance's own annotations drive, read once from its parameter
/// string: the `.ac` magnitude and the two distortion tones. Retaining these
/// three bits is what lets the parameter string itself go unretained.
#[derive(Clone, Copy)]
struct SourceDrive {
    ac: bool,
    distof1: bool,
    distof2: bool,
}

/// One component read as a source, with what its annotations drive, or `None`
/// for anything that is not an independent source.
fn placed_source(
    component: &Component,
    nets: &HashMap<(u64, String), String>,
) -> Option<(PlacedSource, SourceDrive)> {
    let family = source_family(component.kind)?;
    let params = crate::state::parse_params_string(&component.params);
    Some((
        PlacedSource {
            component_id: component.id,
            reference: component.spice_instance_name(),
            is_voltage: is_voltage_source(component.kind),
            family,
            key_figure: key_figure(component, &params),
            nets: terminal_nets(component, nets),
            consumers: Vec::new(),
            occurrence: None,
        },
        SourceDrive {
            ac: carries_ac_excitation(&params, component.kind),
            distof1: carries_value(params.get("distof1_mag").map(String::as_str)),
            distof2: carries_value(params.get("distof2_mag").map(String::as_str)),
        },
    ))
}

/// One component read as an RF port. The caller has already established that
/// it is one.
fn placed_rf_port(component: &Component, nets: &HashMap<(u64, String), String>) -> PlacedRfPort {
    let params = crate::state::parse_params_string(&component.params);
    PlacedRfPort {
        component_id: component.id,
        reference: component.spice_instance_name(),
        port_number: port_number(&params),
        z0: reference_impedance(&params),
        mode: rf_port_mode(component, &params),
        nets: terminal_nets(component, nets),
        consumers: Vec::new(),
        occurrence: None,
    }
}

/// The one place a resolved source is paired with what a plan reads it as.
fn with_source_consumers(
    mut source: PlacedSource,
    drive: SourceDrive,
    plan: Option<&SimulationPlan>,
) -> PlacedSource {
    source.consumers = plan
        .map(|plan| consumers_for(plan, &source.reference, drive))
        .unwrap_or_default();
    source
}

/// Every RF port on this sheet, with what the plan reads it as.
///
/// Ports come back in port-number order, and by reference within a number, so
/// the list reads in the order an S-parameter matrix is indexed rather than in
/// the order the ports were drawn. A repeated number is reported rather than
/// refused — see [`duplicate_port_numbers`].
///
/// Costed like [`placed_sources`]: the net map walks the whole design, so the
/// function returns before building it on a sheet that places no port, which is
/// every design that is not an RF testbench.
pub fn placed_rf_ports(
    schematic: &SchematicState,
    plan: Option<&SimulationPlan>,
) -> Vec<PlacedRfPort> {
    if !schematic
        .components
        .iter()
        .any(|component| component.kind == ComponentType::RfPort)
    {
        return Vec::new();
    }
    let nets = net_names_by_terminal(schematic);
    // Every analysis that reads a port reads all of them, so the plan is walked
    // once for the whole sheet and the answer cloned onto each row rather than
    // re-derived per port.
    let consumers = plan.map(port_consumers_for).unwrap_or_default();
    let mut ports: Vec<PlacedRfPort> = schematic
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::RfPort)
        .map(|component| PlacedRfPort {
            consumers: consumers.clone(),
            ..placed_rf_port(component, &nets)
        })
        .collect();
    ports.sort_by_key(port_order);
    ports
}

/// The port numbers more than one placed port claims, in ascending order.
///
/// Not an error here, and deliberately not resolved here either: an
/// S-parameter run addresses a port by number, so two ports sharing one is a
/// design defect — but which of them wins, and whether the run is refused, is
/// the dispatching surface's answer to give. This states the collision so that
/// surface has one set to state it about.
#[must_use]
pub fn duplicate_port_numbers(ports: &[PlacedRfPort]) -> Vec<u32> {
    let mut claims: HashMap<u32, usize> = HashMap::new();
    for port in ports {
        *claims.entry(port.port_number).or_default() += 1;
    }
    let mut duplicates: Vec<u32> = claims
        .into_iter()
        .filter(|(_, claimed)| *claimed > 1)
        .map(|(number, _)| number)
        .collect();
    duplicates.sort_unstable();
    duplicates
}

/// The one walk of the plan both consumer derivations are built on.
///
/// `arms` is handed each instance's draft and a recorder that already carries
/// that instance's display name and enabled flag, so an arm states only the
/// role it is claiming. Two loops over `plan.instances()` would be two places
/// for an analysis kind to be attributed, and the surfaces that render the
/// result show sources and ports in one table — the moment the two disagreed
/// about whether an instance is enabled, the table would say both.
fn attribute_plan<F>(plan: &SimulationPlan, mut arms: F) -> Vec<SourceConsumer>
where
    F: FnMut(&AnalysisDraft, &mut dyn FnMut(&'static str)),
{
    let mut consumers = Vec::new();
    for instance in plan.instances() {
        let analysis = instance.display_name().to_owned();
        let enabled = instance.enabled();
        let mut record = |role: &'static str| {
            consumers.push(SourceConsumer {
                analysis: analysis.clone(),
                role,
                enabled,
            });
        };
        arms(instance.draft(), &mut record);
    }
    consumers
}

/// What the plan reads every placed RF port as.
///
/// Whole-design by construction: an S-parameter run drives and measures the
/// ports the design places, indexing them by their own port numbers rather than
/// naming references in the draft. Nothing here depends on which port it is, so
/// the answer is derived once per sheet.
///
/// Only `.sp` claims a port. The other analyses that reach a port reach it as a
/// two-terminal element the netlist already carries — a termination loads a
/// transient exactly as a resistor does — and recording that as readership
/// would mean every plan reads every port, which is the reading that makes the
/// unread state say nothing at all.
fn port_consumers_for(plan: &SimulationPlan) -> Vec<SourceConsumer> {
    attribute_plan(plan, |draft, record| {
        if matches!(draft, AnalysisDraft::SParameter(_)) {
            record("S-parameter port");
        }
    })
}

/// The one place a plan is read for source references.
///
/// Every arm names a field that exists on the draft; a draft whose analysis
/// takes no source contributes nothing rather than an empty row.
fn consumers_for(
    plan: &SimulationPlan,
    reference: &str,
    drive: SourceDrive,
) -> Vec<SourceConsumer> {
    attribute_plan(plan, |draft, record| {
        match draft {
            AnalysisDraft::Ac(_) => {
                // The analysis names nothing; the instance carries the drive.
                if drive.ac {
                    record("AC excitation");
                }
            }
            AnalysisDraft::Disto(disto) => {
                // The analysis names nothing; the instance carries the tone. A
                // distortion run takes its excitation only from the
                // `DISTOF1`/`DISTOF2` annotations the instance's distortion
                // properties emit — `build_distortion_rhs` in
                // `rspice-core/src/engine/distortion.rs` skips every source
                // whose spec carries no tone — so an AC magnitude contributes
                // nothing here. The second-tone vector is built only by a run
                // whose draft requests a ratio, so a source carrying only
                // `DISTOF2` is read by exactly those runs.
                if drive.distof1 || (requests_second_tone(&disto.f2_over_f1) && drive.distof2) {
                    record("distortion drive");
                }
            }
            // Whole-design readers. A transient re-evaluates every source at
            // every accepted timestep, and Fourier and transient-noise runs are
            // transients with a projection bolted on; an operating point reads
            // every source's DC value. None of them names an instance, so the
            // binding is recorded against all of them.
            AnalysisDraft::Transient(_) => record("transient drive"),
            AnalysisDraft::TransientNoise(_) => record("noise-transient drive"),
            AnalysisDraft::Fourier(_) => record("Fourier transient drive"),
            AnalysisDraft::OperatingPoint(_) => record("operating-point bias"),
            AnalysisDraft::Envelope(envelope) => {
                // Both shapes at once: the envelope run is a transient, and the
                // sources it names are the ones whose modulation it tracks. The
                // named role wins so the row states the stronger relationship.
                if names_any(&envelope.modulation_sources, reference) {
                    record("envelope modulation source");
                } else {
                    record("envelope transient drive");
                }
            }
            AnalysisDraft::Pss(pss) => {
                if names_any(&pss.tone_sources, reference) {
                    record("PSS tone");
                }
            }
            AnalysisDraft::Noise(noise) => {
                if names(&noise.input, reference) {
                    record("noise input");
                }
            }
            AnalysisDraft::DcSweep(dc) => {
                if names(&dc.source, reference) {
                    record("DC sweep variable");
                }
                if dc.nested && names(&dc.source2, reference) {
                    record("DC inner sweep");
                }
            }
            AnalysisDraft::Stb(stb) => {
                if names(&stb.probe_source, reference) {
                    record("loop probe source");
                }
            }
            AnalysisDraft::TransferFunction(xf) => {
                if names(&xf.input_source, reference) {
                    record("transfer-function input");
                }
            }
            AnalysisDraft::Pac(pac) => {
                if names(&pac.input_source, reference) {
                    record("periodic AC input");
                }
            }
            AnalysisDraft::Pnoise(pnoise) => {
                if names(&pnoise.input_source, reference) {
                    record("periodic noise input");
                }
            }
            AnalysisDraft::Pxf(pxf) => {
                if names(&pxf.input_source, reference) {
                    record("periodic transfer input");
                }
            }
            AnalysisDraft::HarmonicBalance(hb) => {
                if names(&hb.fundamental_source, reference) {
                    record("HB fundamental");
                } else if hb
                    .additional_tones
                    .iter()
                    .any(|tone| names(&tone.source, reference))
                {
                    record("HB tone");
                }
            }
            AnalysisDraft::Hbnoise(hbnoise) => {
                if names(&hbnoise.input_source, reference) {
                    record("HB noise input");
                }
            }
            AnalysisDraft::Qpac(qpac) => {
                if names(&qpac.input_source, reference) {
                    record("quasi-periodic AC input");
                }
            }
            AnalysisDraft::Qpnoise(qpnoise) => {
                if names(&qpnoise.input_source, reference) {
                    record("quasi-periodic noise input");
                }
            }
            AnalysisDraft::Qpxf(qpxf) => {
                if names(&qpxf.input_source, reference) {
                    record("quasi-periodic transfer input");
                }
            }
            // Everything else contributes only what its draft names, and these
            // drafts name nothing. A corner run carries a supply-source list on
            // its *config*, which is built at dispatch from the design rather
            // than authored in the draft; inventing the reference from the
            // config would report a binding the user never made. The periodic
            // and quasi-periodic steady-state solves reach a transient too, but
            // the plan's control over which sources they read is the tone list
            // this file already reads, so claiming the rest would state a
            // relationship the user cannot see or change.
            _ => {}
        }
    })
}

/// Whether a plan's source field names this instance.
fn names(field: &str, reference: &str) -> bool {
    !field.trim().is_empty() && field.trim().eq_ignore_ascii_case(reference)
}

/// Whether a plan's *list*-valued source field names this instance.
///
/// Two drafts hold a list rather than one reference, and each splits it in its
/// own `to_config`: `PssDialogState::tone_sources` on `,`, `;` and whitespace
/// (`simulation::dialog::pss::parse_tone_sources`), and
/// `EnvelopeDialogState::modulation_sources` on `,`, `;` and newline before
/// trimming (`simulation::dialog::envelope::parse_source_list`). Splitting on
/// the union and then on whitespace accepts exactly what both accept, because
/// a name either parser would keep can hold none of those characters —
/// `validate_modulation_sources` rejects a name with surrounding whitespace and
/// the deck spelling of an instance never contains a separator.
fn names_any(field: &str, reference: &str) -> bool {
    field
        .split([',', ';'])
        .flat_map(str::split_whitespace)
        .any(|token| names(token, reference))
}

/// Whether a distortion draft's ratio field requests a second tone.
///
/// Empty and `auto` select single-tone harmonic distortion — the same reading
/// `sim_setup` applies when it validates and dispatches the draft — and only a
/// two-tone run builds the `DISTOF2` excitation vector.
fn requests_second_tone(ratio: &str) -> bool {
    let ratio = ratio.trim();
    !ratio.is_empty() && !ratio.eq_ignore_ascii_case("auto")
}

/// Whether an `.ac` run would drive this source.
///
/// A dedicated AC source always carries a magnitude; any other source carries
/// one only when its `ac` parameter is set to something that is not zero, which
/// is the same test the netlist generator applies before emitting the `AC`
/// annotation.
fn carries_ac_excitation(params: &HashMap<String, String>, kind: ComponentType) -> bool {
    if matches!(
        kind,
        ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc
    ) {
        return true;
    }
    carries_value(params.get("ac").map(String::as_str))
}

/// Whether a parameter is set to something the deck would carry.
///
/// Absent, blank and every spelling of zero are all "not set", because a source
/// spec of zero is the absence of that spec: a port with `AC 0` behind its Z0
/// is a termination, not a drive.
fn carries_value(raw: Option<&str>) -> bool {
    raw.is_some_and(|value| {
        !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "" | "0" | "+0" | "-0" | "0.0"
        )
    })
}

/// The index an S-parameter run addresses this port by.
///
/// One, when the parameter is absent or holds something a port number cannot
/// be. That is the registry's own default and its own floor (the `port`
/// property is bounded 1–64), and a row reporting port 0 for a field the
/// property sheet will not accept would state a port the run can never
/// address.
fn port_number(params: &HashMap<String, String>) -> u32 {
    params
        .get("port")
        .and_then(|raw| crate::quantity::parse_engineering_value(raw).ok())
        .filter(|number| *number >= 1.0)
        .map_or(1, |number| number.round() as u32)
}

/// The port's reference impedance, as the deck would carry it.
///
/// Re-formatted through the same parse-and-format path a source's key figure
/// takes, so `5e1` and `50` print identically. An impedance authored as an
/// expression parses as neither, and is echoed rather than dropped: the field
/// is what the port was given, and a blank cell would read as a port with no
/// reference impedance at all.
fn reference_impedance(params: &HashMap<String, String>) -> String {
    let raw = params
        .get("z0")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .unwrap_or("50");
    crate::quantity::parse_engineering_value(raw)
        .map(crate::state::format_engineering)
        .unwrap_or_else(|_| raw.to_owned())
}

/// What this port does in the design.
///
/// The precedence is the netlist generator's own (`netlist_gen::instances`,
/// the `RfPort` arm): `PWR` outranks `AC`, which outranks `DC`. The `dc`
/// parameter falls back to the component's value exactly as the emitted card
/// does, so a port biased through its value field is not reported as a
/// termination the deck then biases.
fn rf_port_mode(component: &Component, params: &HashMap<String, String>) -> RfPortMode {
    let param = |key: &str| params.get(key).map(String::as_str);
    if carries_value(param("pwr")) {
        RfPortMode::PowerDrive
    } else if carries_value(param("ac_mag")) {
        RfPortMode::AcDrive
    } else if carries_value(
        param("dc")
            .filter(|dc| !dc.trim().is_empty())
            .or(Some(component.value.as_str())),
    ) {
        RfPortMode::DcBias
    } else {
        RfPortMode::Termination
    }
}

/// The waveform family label, or `None` for anything that is not an
/// independent source.
///
/// Controlled sources, behavioural sources and RF ports are excluded: none of
/// them is an excitation a plan can name as a source, and listing them would
/// make the count of excitations wrong. An RF port is not thereby unlisted —
/// it is indexed by number rather than named, so it has a list of its own,
/// [`placed_rf_ports`].
const fn source_family(kind: ComponentType) -> Option<&'static str> {
    Some(match kind {
        ComponentType::VoltageSource | ComponentType::CurrentSource => "DC",
        ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc => "AC",
        ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => "PULSE",
        ComponentType::VoltageSourceSin | ComponentType::CurrentSourceSin => "SIN",
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => "PWL",
        ComponentType::VoltageSourcePwlFile | ComponentType::CurrentSourcePwlFile => "PWL FILE",
        ComponentType::VoltageSourceExp | ComponentType::CurrentSourceExp => "EXP",
        ComponentType::VoltageSourceSffm | ComponentType::CurrentSourceSffm => "SFFM",
        ComponentType::VoltageSourceAm | ComponentType::CurrentSourceAm => "AM",
        ComponentType::VoltageSourcePat | ComponentType::CurrentSourcePat => "PAT",
        ComponentType::VoltageSourceNoise | ComponentType::CurrentSourceNoise => "TRNOISE",
        _ => return None,
    })
}

const fn is_voltage_source(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourcePwlFile
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm
            | ComponentType::VoltageSourceAm
            | ComponentType::VoltageSourcePat
            | ComponentType::VoltageSourceNoise
    )
}

/// The one number that tells two sources of the same family apart.
///
/// Which number that is differs by family: a pulse train is identified by its
/// period, a sinusoid by its frequency, a modulated source by its carrier. The
/// value is re-formatted rather than echoed so a row reads the same whether the
/// field was typed as `1e6` or `1Meg`.
fn key_figure(component: &Component, params: &HashMap<String, String>) -> String {
    let quantity = if is_voltage_source(component.kind) {
        "V"
    } else {
        "A"
    };
    let figure = |key: &str, label: &str, unit: &str| -> Option<String> {
        let raw = params.get(key)?;
        let value = crate::quantity::parse_engineering_value(raw).ok()?;
        Some(if label.is_empty() {
            format!("{}{unit}", crate::state::format_engineering(value))
        } else {
            format!("{label} {}{unit}", crate::state::format_engineering(value))
        })
    };
    match component.kind {
        ComponentType::VoltageSource | ComponentType::CurrentSource => {
            crate::quantity::parse_engineering_value(&component.value)
                .ok()
                .map(|value| format!("{}{quantity}", crate::state::format_engineering(value)))
                .unwrap_or_default()
        }
        ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc => {
            crate::quantity::parse_engineering_value(&component.value)
                .ok()
                .map(|value| format!("{}{quantity} AC", crate::state::format_engineering(value)))
                .unwrap_or_default()
        }
        ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => {
            figure("per", "PER", "s")
                .or_else(|| figure("period", "PER", "s"))
                .or_else(|| figure("pw", "PW", "s"))
                .unwrap_or_default()
        }
        ComponentType::VoltageSourceSin | ComponentType::CurrentSourceSin => {
            figure("freq", "", "Hz").unwrap_or_default()
        }
        ComponentType::VoltageSourceExp | ComponentType::CurrentSourceExp => {
            figure("tau1", "TAU1", "s").unwrap_or_default()
        }
        ComponentType::VoltageSourceSffm
        | ComponentType::CurrentSourceSffm
        | ComponentType::VoltageSourceAm
        | ComponentType::CurrentSourceAm => figure("fc", "FC", "Hz").unwrap_or_default(),
        ComponentType::VoltageSourcePat | ComponentType::CurrentSourcePat => {
            figure("tsample", "TSAMPLE", "s").unwrap_or_default()
        }
        ComponentType::VoltageSourceNoise | ComponentType::CurrentSourceNoise => {
            figure("nt", "NT", "s").unwrap_or_default()
        }
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => {
            let data = params
                .get("pwl_data")
                .map(String::as_str)
                .filter(|data| !data.trim().is_empty())
                .unwrap_or(component.value.as_str());
            let points = data.split_whitespace().count() / 2;
            if points == 0 {
                String::new()
            } else {
                format!("{points} point{}", if points == 1 { "" } else { "s" })
            }
        }
        ComponentType::VoltageSourcePwlFile | ComponentType::CurrentSourcePwlFile => params
            .get("file")
            .map(String::as_str)
            .filter(|path| !path.trim().is_empty())
            .unwrap_or(component.value.as_str())
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or_default()
            .to_owned(),
        _ => String::new(),
    }
}

/// Net name for every terminal in the design, keyed by instance and pin.
fn net_names_by_terminal(schematic: &SchematicState) -> HashMap<(u64, String), String> {
    keyed_by_terminal(design_nets(schematic))
}

/// The same map for one cell view of a frozen projection.
///
/// Built on `projection_nets`, which resolves the hierarchy the way the run
/// does and is itself retained by the projection — so the terminals a source
/// inside a child master reports are the nets of that master, named as the
/// deck names them, rather than whatever the editor's own buffer would say.
fn projection_terminal_nets(
    libraries: &LibraryManager,
    projection: &DesignProjection,
    cell_view_key: &str,
) -> HashMap<(u64, String), String> {
    keyed_by_terminal(
        projection_nets(libraries, projection, cell_view_key)
            .iter()
            .cloned(),
    )
}

fn keyed_by_terminal(
    nets: impl IntoIterator<Item = crate::simulation::netlist_gen::DesignNet>,
) -> HashMap<(u64, String), String> {
    nets.into_iter()
        .flat_map(|net| {
            let name = net.name.clone();
            net.terminals
                .into_iter()
                .map(move |terminal| ((terminal.component_id, terminal.pin), name.clone()))
        })
        .collect()
}

/// This source's nets, in the order its pins are declared.
///
/// A terminal with no net is reported as unconnected rather than skipped: a
/// source driving nothing is exactly what a reader scanning this list is
/// looking for, and dropping the entry would hide it.
fn terminal_nets(component: &Component, nets: &HashMap<(u64, String), String>) -> Vec<String> {
    component
        .terminal_positions()
        .into_iter()
        .map(|(pin, _)| {
            nets.get(&(component.id, pin.to_owned()))
                .cloned()
                .unwrap_or_else(|| "unconnected".to_owned())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::AnalysisKind;
    use crate::state::Point;

    fn source(id: u64, kind: ComponentType, name: &str, params: &str) -> Component {
        let mut component = Component::new(id, kind, Point::origin()).with_name_value(name, "0");
        component.params = params.to_owned();
        component
    }

    fn schematic_with(components: Vec<Component>) -> SchematicState {
        let mut schematic = SchematicState::default();
        schematic.components = components;
        schematic
    }

    /// An RF port with no value of its own, which is how one is placed.
    fn port(id: u64, name: &str, params: &str) -> Component {
        let mut component =
            Component::new(id, ComponentType::RfPort, Point::origin()).with_name_value(name, "");
        component.params = params.to_owned();
        component
    }

    fn sp_plan(enabled: bool) -> SimulationPlan {
        let mut plan = SimulationPlan::empty();
        let (sp, _) = plan
            .insert(AnalysisKind::SParameter)
            .expect("an S-parameter analysis inserts");
        plan.set_enabled(sp, enabled)
            .expect("the fixture S-parameter analysis takes its enabled flag");
        plan
    }

    #[test]
    fn only_independent_sources_are_listed() {
        let schematic = schematic_with(vec![
            source(1, ComponentType::VoltageSourceSin, "V1", "freq=1k"),
            source(2, ComponentType::Resistor, "R1", ""),
            source(3, ComponentType::Vcvs, "E1", ""),
            source(4, ComponentType::CurrentSourcePulse, "I1", "per=1m"),
            port(5, "P1", "port=1"),
        ]);
        let listed = placed_sources(&schematic, None);
        let references: Vec<&str> = listed
            .iter()
            .map(|source| source.reference.as_str())
            .collect();
        assert_eq!(references, vec!["I1", "V1"]);
    }

    /// The list an S-parameter testbench is made of.
    ///
    /// A port is excluded from `placed_sources` above and was in no list at
    /// all, so a design whose every excitation is a placed port reported that
    /// it places none.
    #[test]
    fn a_placed_port_is_listed_with_its_number_impedance_and_mode() {
        let schematic = schematic_with(vec![
            source(1, ComponentType::Resistor, "R1", ""),
            port(2, "P1", "port=2 z0=5e1"),
        ]);
        let listed = placed_rf_ports(&schematic, None);
        assert_eq!(listed.len(), 1, "{listed:?}");
        assert_eq!(listed[0].reference, "P1");
        assert_eq!(listed[0].port_number, 2);
        assert_eq!(listed[0].z0, "50", "5e1 and 50 are one impedance");
        assert_eq!(listed[0].mode, RfPortMode::Termination);
        assert_eq!(listed[0].summary(), "term \u{00b7} Z0 50");
        assert_eq!(listed[0].quantity(), "P");
    }

    /// A port with no `port` parameter is port 1, which is the registry's own
    /// default, and a port number below the registry's floor is not a port
    /// number.
    #[test]
    fn a_port_number_defaults_to_the_registrys_own_default() {
        let schematic = schematic_with(vec![port(1, "P1", ""), port(2, "P2", "port=0")]);
        let listed = placed_rf_ports(&schematic, None);
        assert_eq!(listed[0].port_number, 1);
        assert_eq!(listed[1].port_number, 1);
        assert_eq!(listed[0].z0, "50", "an unstated impedance is 50");
    }

    /// Each of the four things a port element can be, in the netlist
    /// generator's own precedence.
    #[test]
    fn a_ports_mode_follows_the_source_spec_the_card_would_carry() {
        let schematic = schematic_with(vec![
            port(1, "P1", "port=1 pwr=-10 ac_mag=1 dc=2"),
            port(2, "P2", "port=2 ac_mag=1 dc=2"),
            port(3, "P3", "port=3 dc=2"),
            port(4, "P4", "port=4"),
            // Every spelling of zero is the absence of the spec, because the
            // port it describes is electrically a termination.
            port(5, "P5", "port=5 pwr=0 ac_mag=0.0 dc=-0"),
        ]);
        let modes: Vec<RfPortMode> = placed_rf_ports(&schematic, None)
            .iter()
            .map(|port| port.mode)
            .collect();
        assert_eq!(
            modes,
            vec![
                RfPortMode::PowerDrive,
                RfPortMode::AcDrive,
                RfPortMode::DcBias,
                RfPortMode::Termination,
                RfPortMode::Termination,
            ]
        );
    }

    /// The emitted card takes its DC from the component's value when the
    /// parameter is unset, so a port biased that way is not a termination.
    #[test]
    fn a_port_biased_through_its_value_field_is_not_a_termination() {
        let component =
            Component::new(1, ComponentType::RfPort, Point::origin()).with_name_value("P1", "2");
        let listed = placed_rf_ports(&schematic_with(vec![component]), None);
        assert_eq!(listed[0].mode, RfPortMode::DcBias);
    }

    /// One entry per pin, in pin order, exactly as a source's row reports its
    /// terminals.
    ///
    /// Every pin gets a name here even with nothing drawn against it, because
    /// `design_nets` names the degenerate one-terminal net a lone pin forms;
    /// `unconnected` is what [`terminal_nets`] reports for a pin the net map
    /// does not carry at all. The port list must not invent a different answer
    /// from the source list directly above it in the same table.
    #[test]
    fn a_ports_terminals_are_reported_in_pin_order() {
        let schematic = schematic_with(vec![port(1, "P1", "port=1")]);
        let listed = placed_rf_ports(&schematic, None);
        assert_eq!(
            listed[0].nets.len(),
            schematic.components[0].terminal_positions().len()
        );
        assert!(
            listed[0].nets.iter().all(|net| !net.is_empty()),
            "{:?}",
            listed[0].nets
        );
    }

    /// Port-number order, not drawing order: the list reads the way the
    /// S-parameter matrix it feeds is indexed.
    #[test]
    fn ports_are_listed_in_port_number_order() {
        let schematic = schematic_with(vec![
            port(1, "PB", "port=2"),
            port(2, "PC", "port=1"),
            port(3, "PA", "port=1"),
        ]);
        let references: Vec<String> = placed_rf_ports(&schematic, None)
            .into_iter()
            .map(|port| port.reference)
            .collect();
        assert_eq!(references, vec!["PA", "PC", "PB"]);
    }

    /// Two ports claiming one number is a design defect this list reports
    /// rather than refuses: which of them an `.sp` run would address is the
    /// dispatching surface's answer, and it needs the collision set to give it.
    #[test]
    fn a_repeated_port_number_is_reported_as_a_collision() {
        let schematic = schematic_with(vec![
            port(1, "P1", "port=1"),
            port(2, "P2", "port=3"),
            port(3, "P3", "port=3"),
            port(4, "P4", "port=2"),
            port(5, "P5", "port=2"),
        ]);
        let listed = placed_rf_ports(&schematic, None);
        assert_eq!(listed.len(), 5, "every port is still listed");
        assert_eq!(duplicate_port_numbers(&listed), vec![2, 3]);

        let unique = schematic_with(vec![port(1, "P1", "port=1"), port(2, "P2", "port=2")]);
        assert!(duplicate_port_numbers(&placed_rf_ports(&unique, None)).is_empty());
    }

    /// An `.sp` run indexes the ports the design places; it names none of them
    /// in its draft, so the binding is recorded against all of them.
    #[test]
    fn an_s_parameter_analysis_reads_every_placed_port() {
        let schematic = schematic_with(vec![
            port(1, "P1", "port=1"),
            port(2, "P2", "port=2"),
            source(3, ComponentType::VoltageSourcePulse, "V1", "per=1m"),
        ]);
        let plan = sp_plan(true);
        let listed = placed_rf_ports(&schematic, Some(&plan));
        assert!(
            listed.iter().all(|port| port
                .consumers
                .iter()
                .all(|consumer| consumer.role == "S-parameter port")
                && port.is_read()),
            "{listed:?}"
        );
        assert!(
            placed_sources(&schematic, Some(&plan))[0]
                .consumers
                .is_empty(),
            "an S-parameter run reads ports, not independent sources"
        );
    }

    /// A disabled `.sp` run is not in the run the plan would dispatch, so the
    /// ports it would have indexed are terminations for its length. The row
    /// still names it, because re-enabling it is the one edit that changes the
    /// answer.
    #[test]
    fn a_disabled_s_parameter_analysis_is_listed_but_does_not_read_a_port() {
        let schematic = schematic_with(vec![port(1, "P1", "port=1")]);

        let listed = placed_rf_ports(&schematic, Some(&sp_plan(false)));
        assert_eq!(
            listed[0]
                .consumers
                .iter()
                .map(|consumer| consumer.role)
                .collect::<Vec<_>>(),
            vec!["S-parameter port"],
            "the disabled instance is still named"
        );
        assert!(!listed[0].is_read());

        let listed = placed_rf_ports(&schematic, Some(&sp_plan(true)));
        assert!(listed[0].is_read());
    }

    /// A transient loads a termination exactly as it loads a resistor, which is
    /// not readership: if it were, every plan would read every port and the
    /// unread state would say nothing at all.
    #[test]
    fn a_plan_that_names_no_port_reader_reads_no_port() {
        let plan = SimulationPlan::new();
        let schematic = schematic_with(vec![port(1, "P1", "port=1")]);
        let listed = placed_rf_ports(&schematic, Some(&plan));
        assert!(listed[0].consumers.is_empty(), "{:?}", listed[0].consumers);
    }

    #[test]
    fn a_row_states_its_family_and_one_figure() {
        let schematic = schematic_with(vec![source(
            1,
            ComponentType::VoltageSourceSin,
            "V1",
            "freq=1000",
        )]);
        let listed = placed_sources(&schematic, None);
        assert_eq!(listed[0].summary(), "SIN · 1kHz");
        assert_eq!(listed[0].quantity(), "V");
    }

    /// A DC source is authored with its unit far more often than without it,
    /// and the row used to drop the whole figure when it was: the parser
    /// refused `5V`, `key_figure` swallowed the error with `.ok()?`, and the
    /// page whose one job is to say what drives the circuit showed the bare
    /// word `DC`.
    #[test]
    fn a_dc_row_states_a_value_authored_with_its_unit() {
        let component = Component::new(1, ComponentType::VoltageSource, Point::origin())
            .with_name_value("V1", "5V");
        let listed = placed_sources(&schematic_with(vec![component]), None);
        assert_eq!(listed[0].summary(), "DC · 5V");
    }

    /// The ampere that made atto unaffordable: `1A` is one ampere on every
    /// surface that reads it, not 1e-18.
    #[test]
    fn a_current_source_authored_in_amperes_reads_as_amperes() {
        let component = Component::new(1, ComponentType::CurrentSource, Point::origin())
            .with_name_value("I1", "1A");
        let listed = placed_sources(&schematic_with(vec![component]), None);
        assert_eq!(listed[0].summary(), "DC · 1A");
        assert_eq!(listed[0].quantity(), "I");
    }

    /// Parameter figures carry units too: a pulse period authored `1ms` and a
    /// sinusoid frequency authored `10kHz` both used to leave the row's figure
    /// empty.
    #[test]
    fn a_parameter_figure_authored_with_its_unit_reaches_the_row() {
        let schematic = schematic_with(vec![
            source(1, ComponentType::CurrentSourcePulse, "I1", "per=1ms"),
            source(2, ComponentType::VoltageSourceSin, "V1", "freq=10kHz"),
        ]);
        let listed = placed_sources(&schematic, None);
        let summaries: Vec<String> = listed.iter().map(PlacedSource::summary).collect();
        assert_eq!(summaries, vec!["PULSE · PER 1ms", "SIN · 10kHz"]);
    }

    #[test]
    fn a_pulse_row_is_identified_by_its_period() {
        let schematic = schematic_with(vec![source(
            1,
            ComponentType::CurrentSourcePulse,
            "I1",
            "per=1m pw=100u",
        )]);
        let listed = placed_sources(&schematic, None);
        assert_eq!(listed[0].summary(), "PULSE · PER 1ms");
        assert_eq!(listed[0].quantity(), "I");
    }

    /// The point list is the primary property, so it lives in `value` rather
    /// than in the parameter string, and the count has to read it from there.
    #[test]
    fn a_pwl_row_counts_its_points() {
        let component = Component::new(1, ComponentType::VoltageSourcePwl, Point::origin())
            .with_name_value("V1", "0 0 1u 1 2u 0");
        let listed = placed_sources(&schematic_with(vec![component]), None);
        assert_eq!(listed[0].summary(), "PWL · 3 points");
    }

    /// One entry per pin, in pin order, so the two terminals of a source line
    /// up with the columns that show them however the design is wired.
    #[test]
    fn every_terminal_is_reported_in_pin_order() {
        let schematic = schematic_with(vec![source(
            1,
            ComponentType::VoltageSourceSin,
            "V1",
            "freq=1k",
        )]);
        let listed = placed_sources(&schematic, None);
        let pins = listed[0].nets.len();
        assert_eq!(pins, schematic.components[0].terminal_positions().len());
        assert!(
            listed[0].nets.iter().all(|net| !net.is_empty()),
            "{:?}",
            listed[0].nets
        );
    }

    /// A source with no AC magnitude is not excited by an AC run, so it must
    /// not be reported as one of that analysis's inputs.
    #[test]
    fn ac_excitation_follows_the_instance_and_not_the_analysis() {
        let mut params = HashMap::new();
        assert!(!carries_ac_excitation(
            &params,
            ComponentType::VoltageSourceSin
        ));
        params.insert("ac".to_owned(), "0".to_owned());
        assert!(!carries_ac_excitation(
            &params,
            ComponentType::VoltageSourceSin
        ));
        params.insert("ac".to_owned(), "1".to_owned());
        assert!(carries_ac_excitation(
            &params,
            ComponentType::VoltageSourceSin
        ));
        assert!(carries_ac_excitation(
            &HashMap::new(),
            ComponentType::VoltageSourceAc
        ));
    }

    /// A distortion run is not an AC run with harmonics bolted on.
    ///
    /// Its excitation comes only from `DISTOF1`/`DISTOF2` annotations
    /// (`rspice-core` `engine::distortion::build_distortion_rhs` skips a source
    /// whose spec carries no tone and refuses a run whose vector is zero), so
    /// an AC magnitude — or a distortion magnitude of zero, which is the
    /// absence of the tone — claims nothing, and a source carrying only the
    /// second tone is read by exactly the runs that request a second-tone
    /// ratio.
    #[test]
    fn a_distortion_run_claims_the_sources_carrying_its_tones() {
        let mut plan = SimulationPlan::empty();
        let (disto, _) = plan
            .insert(AnalysisKind::Disto)
            .expect("a distortion analysis inserts");
        let schematic = schematic_with(vec![
            source(1, ComponentType::VoltageSourceAc, "V1", "distof1_mag=0"),
            source(
                2,
                ComponentType::VoltageSourceSin,
                "V2",
                "ac=1 freq=1k distof1_mag=1m",
            ),
            source(3, ComponentType::VoltageSource, "V3", "distof2_mag=2m"),
        ]);

        let roles = |listed: &[PlacedSource], index: usize| -> Vec<&'static str> {
            listed[index]
                .consumers
                .iter()
                .map(|consumer| consumer.role)
                .collect()
        };

        let listed = placed_sources(&schematic, Some(&plan));
        assert!(
            listed[0].consumers.is_empty(),
            "an AC magnitude and a zero tone do not drive a distortion run: {listed:?}"
        );
        assert_eq!(roles(&listed, 1), vec!["distortion drive"]);
        assert!(
            listed[2].consumers.is_empty(),
            "a single-tone run never builds the DISTOF2 vector: {listed:?}"
        );

        plan.edit(disto, |draft| {
            let AnalysisDraft::Disto(draft) = draft else {
                panic!("expected a Disto draft");
            };
            draft.f2_over_f1 = "0.9".to_owned();
        })
        .expect("Disto draft edits");
        let listed = placed_sources(&schematic, Some(&plan));
        assert_eq!(
            roles(&listed, 2),
            vec!["distortion drive"],
            "a two-tone run reads its second-tone source"
        );
    }

    #[test]
    fn a_plan_field_matches_its_instance_case_insensitively() {
        assert!(names("v1", "V1"));
        assert!(names(" V1 ", "V1"));
        assert!(!names("", "V1"));
        assert!(!names("V10", "V1"));
    }

    /// The tone list is the one control a PSS run has over which sources define
    /// its period, so a source named there is the least "unread" row on the
    /// page. Before this arm existed it rendered as `not read`.
    #[test]
    fn a_pss_tone_source_resolves_to_its_pss_analysis() {
        let mut plan = SimulationPlan::empty();
        let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
        plan.edit(pss, |draft| {
            let AnalysisDraft::Pss(draft) = draft else {
                panic!("expected a PSS draft");
            };
            draft.osc_mode = false;
            draft.tone_sources = "VLO, V1".to_owned();
        })
        .expect("PSS draft edits");
        let schematic = schematic_with(vec![
            source(1, ComponentType::VoltageSourceSin, "V1", "freq=1k"),
            source(2, ComponentType::VoltageSourceSin, "V2", "freq=1k"),
        ]);
        let listed = placed_sources(&schematic, Some(&plan));
        assert_eq!(
            listed[0]
                .consumers
                .iter()
                .map(|consumer| consumer.role)
                .collect::<Vec<_>>(),
            vec!["PSS tone"],
            "V1 is named in the tone list"
        );
        assert!(
            listed[1].consumers.is_empty(),
            "V2 is not named and a PSS names no other source: {:?}",
            listed[1].consumers
        );
    }

    #[test]
    fn an_envelope_modulation_source_resolves_to_its_envelope_analysis() {
        let mut plan = SimulationPlan::empty();
        let (envelope, _) = plan
            .insert(AnalysisKind::Envelope)
            .expect("Envelope inserts");
        plan.edit(envelope, |draft| {
            let AnalysisDraft::Envelope(draft) = draft else {
                panic!("expected an Envelope draft");
            };
            draft.modulation_sources = "VMOD;V1".to_owned();
        })
        .expect("Envelope draft edits");
        let schematic = schematic_with(vec![
            source(1, ComponentType::VoltageSourceSin, "V1", "freq=1k"),
            source(2, ComponentType::VoltageSourceSin, "V2", "freq=1k"),
        ]);
        let listed = placed_sources(&schematic, Some(&plan));
        assert_eq!(
            listed[0].consumers[0].role, "envelope modulation source",
            "V1 is named in the modulation list"
        );
        assert_eq!(
            listed[1].consumers[0].role, "envelope transient drive",
            "an envelope run is still a transient, and a transient reads every \
             source it is not told about"
        );
    }

    /// A transient re-evaluates every source at every accepted timestep
    /// (`VoltageSources::update_transient_rhs`), so a PULSE in a transient-only
    /// plan is read by that transient. The page said `not read`.
    #[test]
    fn a_pulse_in_a_transient_only_plan_is_read() {
        let plan = SimulationPlan::new();
        assert_eq!(
            plan.instances().len(),
            1,
            "a fresh plan holds one transient"
        );
        let schematic = schematic_with(vec![source(
            1,
            ComponentType::VoltageSourcePulse,
            "V1",
            "per=1m pw=100u",
        )]);
        let listed = placed_sources(&schematic, Some(&plan));
        assert_eq!(
            listed[0]
                .consumers
                .iter()
                .map(|consumer| consumer.role)
                .collect::<Vec<_>>(),
            vec!["transient drive"]
        );
    }

    /// An operating point reads every source's DC value
    /// (`rspice-core` `engine::source_values::extract_dc_value` matches every
    /// `SourceSpec`), including the initial value of a waveform it never sweeps.
    #[test]
    fn an_operating_point_reads_every_placed_source() {
        let mut plan = SimulationPlan::empty();
        plan.insert(AnalysisKind::OperatingPoint)
            .expect("OP inserts");
        let schematic = schematic_with(vec![
            source(1, ComponentType::VoltageSourcePulse, "V1", "per=1m"),
            source(2, ComponentType::CurrentSourceSin, "I1", "freq=1k"),
        ]);
        let listed = placed_sources(&schematic, Some(&plan));
        assert!(
            listed.iter().all(|source| source
                .consumers
                .iter()
                .any(|consumer| consumer.role == "operating-point bias")),
            "{listed:?}"
        );
    }

    /// The unread state still exists, and still means what the page says: an
    /// AC-only plan reads nothing about a source that carries no AC magnitude.
    #[test]
    fn a_source_no_analysis_names_or_drives_stays_unread() {
        let mut plan = SimulationPlan::empty();
        plan.insert(AnalysisKind::Ac).expect("AC inserts");
        let schematic = schematic_with(vec![source(
            1,
            ComponentType::VoltageSourcePulse,
            "V1",
            "per=1m",
        )]);
        let listed = placed_sources(&schematic, Some(&plan));
        assert!(listed[0].consumers.is_empty(), "{:?}", listed[0].consumers);
    }

    /// Both list-valued fields are split by their drafts on separators a source
    /// reference can never contain, so one matcher accepts exactly both.
    #[test]
    fn a_list_valued_plan_field_matches_any_of_its_items() {
        assert!(names_any("VLO, V1", "v1"));
        assert!(names_any("VLO;V1", "V1"));
        assert!(names_any("VLO V1", "V1"));
        assert!(names_any("VLO,\nV1", "V1"));
        assert!(!names_any("VLO, V10", "V1"));
        assert!(!names_any("", "V1"));
    }

    /// A disabled analysis is not a reader.
    ///
    /// The consumer walk read every instance the plan holds, so one disabled
    /// transient — a whole-design reader — marked every placed source "read".
    /// That is the exact opposite of the finding this list exists to surface:
    /// the run the plan would dispatch does not contain that analysis, so a
    /// source only it names is one the run drives without looking at.
    ///
    /// The row still lists it, dimmed, because re-enabling the instance is the
    /// one edit that changes the answer.
    #[test]
    fn a_disabled_analysis_is_listed_but_does_not_read_a_source() {
        let mut plan = SimulationPlan::empty();
        let (transient, _) = plan
            .insert(AnalysisKind::Transient)
            .expect("a transient inserts");
        plan.set_enabled(transient, false)
            .expect("the fixture transient disables");
        let schematic = schematic_with(vec![source(
            1,
            ComponentType::VoltageSourcePulse,
            "V1",
            "per=1m",
        )]);

        let listed = placed_sources(&schematic, Some(&plan));

        assert_eq!(
            listed[0]
                .consumers
                .iter()
                .map(|consumer| consumer.role)
                .collect::<Vec<_>>(),
            vec!["transient drive"],
            "the disabled instance is still named"
        );
        assert!(
            !listed[0].is_read(),
            "a disabled instance is not in the run, so it does not read anything"
        );
        assert_eq!(
            listed[0].reading_consumers().count(),
            0,
            "and it is not one of the run's readers"
        );

        plan.set_enabled(transient, true)
            .expect("the fixture transient re-enables");
        let listed = placed_sources(&schematic, Some(&plan));
        assert!(
            listed[0].is_read(),
            "re-enabling the instance is what makes the source read"
        );
        assert_eq!(listed[0].reading_consumers().count(), 1);
    }

    #[test]
    fn sources_are_listed_in_reference_order() {
        let schematic = schematic_with(vec![
            source(1, ComponentType::VoltageSourceSin, "V2", "freq=1k"),
            source(2, ComponentType::VoltageSourceSin, "V1", "freq=1k"),
        ]);
        let listed = placed_sources(&schematic, None);
        assert_eq!(listed[0].reference, "V1");
        assert_eq!(listed[1].reference, "V2");
    }

    /// A list derived from one buffer states no occurrence, because it has no
    /// hierarchy to place the buffer in. That is not the root: the root is a
    /// claim about where the run reaches an instance, and this reading never
    /// asked.
    #[test]
    fn a_single_buffer_reading_claims_no_occurrence() {
        let schematic = schematic_with(vec![
            source(1, ComponentType::VoltageSourceSin, "V1", "freq=1k"),
            port(2, "P1", "port=1"),
        ]);
        assert_eq!(placed_sources(&schematic, None)[0].occurrence, None);
        assert_eq!(placed_rf_ports(&schematic, None)[0].occurrence, None);
        // The column still states something, because a sheet read on its own
        // is being read as its own root.
        assert_eq!(placed_sources(&schematic, None)[0].occurrence_label(), "/");
        assert_eq!(placed_rf_ports(&schematic, None)[0].occurrence_label(), "/");
    }

    /// What the whole design places, as opposed to what one sheet does.
    mod whole_design {
        use super::*;
        use crate::simulation::cost_probe::{Derivation, count, reset};
        use crate::state::{
            Cell, Library, LibraryCellInstance, LibraryManager, Point, ProjectWorkspace, View,
            ViewType,
        };

        /// One placed hierarchical instance, under the name the design gives
        /// it — which is the segment the occurrence path is spelled with.
        fn instance(id: u64, name: &str, cell: &str) -> Component {
            let mut component = Component::new(id, ComponentType::CellInstance, Point::origin())
                .with_library_cell(LibraryCellInstance::new("work", cell, "schematic"));
            component.name = name.to_owned();
            component
        }

        /// A workspace whose root drawing is `root` and whose `work` library
        /// holds one schematic master per entry of `masters`.
        struct Design {
            libraries: LibraryManager,
            workspace: ProjectWorkspace,
            active: CellViewRef,
            root: SchematicState,
        }

        impl Design {
            fn new(root: SchematicState, masters: &[(&str, SchematicState)]) -> Self {
                let mut libraries = LibraryManager::new();
                let mut work = Library::new("work");
                let mut workspace = ProjectWorkspace::default();
                for (cell, schematic) in masters {
                    let mut master = Cell::new(*cell);
                    master.add_view(View::new("schematic", ViewType::Schematic));
                    work.add_cell(master);
                    workspace.schematic_buffers.insert(
                        CellViewRef::new("work", *cell, "schematic").key(),
                        schematic.clone(),
                    );
                }
                libraries.add_library(work);
                let active = workspace.active_view.clone();
                let mut top = Library::new(&active.library);
                let mut top_cell = Cell::new(&active.cell);
                top_cell.add_view(View::new(&active.view, ViewType::Schematic));
                top.add_cell(top_cell);
                libraries.add_library(top);
                workspace
                    .schematic_buffers
                    .insert(active.key(), root.clone());
                Self {
                    libraries,
                    workspace,
                    active,
                    root,
                }
            }

            fn projection(&self) -> Arc<DesignProjection> {
                self.workspace
                    .configuration_execution_projection(&self.libraries, &self.active, &self.root)
                    .expect("the fixture design projects")
            }

            fn sources(&self, plan: Option<&SimulationPlan>) -> Vec<PlacedSource> {
                design_sources(&self.libraries, &self.projection(), plan)
            }

            fn ports(&self, plan: Option<&SimulationPlan>) -> Vec<PlacedRfPort> {
                design_rf_ports(&self.libraries, &self.projection(), plan)
            }
        }

        /// Every row's occurrence path and reference, in listed order.
        fn placements(sources: &[PlacedSource]) -> Vec<(String, &str)> {
            sources
                .iter()
                .map(|source| (source.occurrence_label(), source.reference.as_str()))
                .collect()
        }

        /// The finding this lane exists for: a source drawn inside a child
        /// master is netlisted, biases the circuit and is read by the
        /// analyses, and neither surface could see it.
        #[test]
        fn a_source_inside_a_child_master_is_listed_under_the_occurrence_that_reaches_it() {
            let mut root = SchematicState::default();
            root.components
                .push(source(1, ComponentType::VoltageSource, "VDD", ""));
            root.components.push(instance(2, "XAFE", "afe"));
            let mut child = SchematicState::default();
            child
                .components
                .push(source(10, ComponentType::VoltageSourceSin, "V1", "freq=1k"));

            let design = Design::new(root, &[("afe", child)]);

            assert_eq!(
                placements(&design.sources(None)),
                vec![("/".to_owned(), "VDD"), ("/XAFE".to_owned(), "V1")],
                "the design root's own source leads, and the child's states where it is"
            );
        }

        /// Per occurrence, not per drawing.
        ///
        /// A run flattens the hierarchy: one source drawn in a master two
        /// instances reach becomes `XA.V1` and `XB.V1`, across different
        /// nodes, biasing different devices, separately selectable. Listing
        /// the drawing once would state a number no run has, and would leave
        /// the reader no row to reach the second instance through.
        #[test]
        fn one_drawn_source_reached_twice_is_two_rows() {
            let mut root = SchematicState::default();
            root.components.push(instance(1, "XA", "afe"));
            root.components.push(instance(2, "XB", "afe"));
            let mut child = SchematicState::default();
            child
                .components
                .push(source(10, ComponentType::VoltageSourceSin, "V1", "freq=1k"));
            child.components.push(port(11, "P1", "port=1"));

            let design = Design::new(root, &[("afe", child)]);

            assert_eq!(
                placements(&design.sources(None)),
                vec![("/XA".to_owned(), "V1"), ("/XB".to_owned(), "V1")],
                "the deck carries one card per occurrence, so the page states one row per card"
            );
            let ports = design.ports(None);
            assert_eq!(
                ports
                    .iter()
                    .map(|port| (port.occurrence_label(), port.reference.as_str()))
                    .collect::<Vec<_>>(),
                vec![("/XA".to_owned(), "P1"), ("/XB".to_owned(), "P1")]
            );
            assert_eq!(
                duplicate_port_numbers(&ports),
                vec![1],
                "two occurrences claiming port 1 claim one index of one S-parameter matrix"
            );
        }

        /// The whole-design reading is what a plan is read against, so a
        /// source only a child master places is read by the plan's
        /// whole-design analyses exactly as a root-placed one is.
        #[test]
        fn a_child_master_source_is_read_by_the_plans_whole_design_analyses() {
            let mut root = SchematicState::default();
            root.components.push(instance(1, "XAFE", "afe"));
            let mut child = SchematicState::default();
            child.components.push(source(
                10,
                ComponentType::VoltageSourcePulse,
                "V1",
                "per=1m",
            ));

            let design = Design::new(root, &[("afe", child)]);
            let listed = design.sources(Some(&SimulationPlan::new()));

            assert_eq!(
                listed[0]
                    .consumers
                    .iter()
                    .map(|consumer| consumer.role)
                    .collect::<Vec<_>>(),
                vec!["transient drive"]
            );
            assert!(listed[0].is_read());
        }

        /// A master that places no excitation is skipped before its nets are
        /// resolved, and a binding with no schematic behind it — a stop
        /// boundary, a SPICE or Verilog-A view — contributes nothing rather
        /// than an error.
        #[test]
        fn a_master_that_places_nothing_contributes_nothing() {
            let mut root = SchematicState::default();
            root.components.push(instance(1, "XEMPTY", "empty"));
            root.components.push(instance(2, "XGONE", "absent"));
            root.components
                .push(source(3, ComponentType::VoltageSource, "V1", ""));

            let design = Design::new(root, &[("empty", SchematicState::default())]);

            assert_eq!(
                placements(&design.sources(None)),
                vec![("/".to_owned(), "V1")]
            );
        }

        /// The walk is retained against the projection that decided it.
        ///
        /// Counted rather than read, because a rail that resolves the whole
        /// design twice a frame paints exactly the same pixels as one that
        /// resolves it once — the failure this probe exists for. The count is
        /// per build of the whole-design half, so an unchanged frame advances
        /// it not at all.
        #[test]
        fn the_whole_design_walk_is_not_repeated_for_one_projection() {
            let mut root = SchematicState::default();
            root.components.push(instance(1, "XA", "afe"));
            root.components.push(instance(2, "XB", "afe"));
            let mut child = SchematicState::default();
            child
                .components
                .push(source(10, ComponentType::VoltageSourceSin, "V1", "freq=1k"));
            let design = Design::new(root, &[("afe", child)]);
            let projection = design.projection();

            reset();
            let first = design_sources(&design.libraries, &projection, None);
            assert_eq!(count(Derivation::PlacedSources), 1, "the first call walks");

            let again = design_sources(&design.libraries, &projection, None);
            let ports = design_rf_ports(&design.libraries, &projection, None);
            assert_eq!(
                count(Derivation::PlacedSources),
                1,
                "a second reading of one projection re-walks nothing, and neither does the \
                 other list"
            );
            assert_eq!(placements(&first), placements(&again));
            assert!(ports.is_empty());

            // The one thing that must invalidate it: a design the workspace
            // digests differently is a different projection, and a different
            // projection is a different answer.
            let mut edited = design;
            edited
                .root
                .components
                .push(source(4, ComponentType::VoltageSource, "VDD", ""));
            let moved = edited.projection();
            assert!(
                !Arc::ptr_eq(&projection, &moved),
                "an edit mints a new projection"
            );
            assert_eq!(
                placements(&design_sources(&edited.libraries, &moved, None)),
                vec![
                    ("/".to_owned(), "VDD"),
                    ("/XA".to_owned(), "V1"),
                    ("/XB".to_owned(), "V1"),
                ]
            );
            assert_eq!(count(Derivation::PlacedSources), 2, "and it is walked once");
        }
    }
}
