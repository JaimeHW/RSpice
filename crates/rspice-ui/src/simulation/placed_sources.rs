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

use std::collections::HashMap;

use crate::simulation::netlist_gen::design_nets;
use crate::simulation::plan::{AnalysisDraft, SimulationPlan};
use crate::state::{Component, ComponentType, SchematicState};

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

    /// The consumers the run actually contains.
    pub fn reading_consumers(&self) -> impl Iterator<Item = &SourceConsumer> {
        self.consumers.iter().filter(|consumer| consumer.reads())
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
}

/// How many independent sources this sheet places.
///
/// The same predicate [`placed_sources`] admits a component under, asked
/// without resolving anything: the navigator's rail states this number and
/// nothing else, and resolving the list to read its length walked the design's
/// nets, parsed every source's parameters and sorted the result — on every
/// frame the rail was drawn, for a count that only the set of placed
/// components can change.
#[must_use]
pub fn placed_source_count(schematic: &SchematicState) -> usize {
    schematic
        .components
        .iter()
        .filter(|component| source_family(component.kind).is_some())
        .count()
}

/// How many RF ports this sheet places.
///
/// The same predicate [`placed_rf_ports`] admits a component under, asked
/// without resolving anything, for the same reason [`placed_source_count`]
/// exists: a rail that states a number and nothing else must not walk the
/// design's nets to learn it.
#[must_use]
pub fn placed_rf_port_count(schematic: &SchematicState) -> usize {
    schematic
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::RfPort)
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
            let family = source_family(component.kind)?;
            let reference = component.spice_instance_name();
            let params = crate::state::parse_params_string(&component.params);
            let consumers = plan
                .map(|plan| consumers_for(plan, &reference, &params, component.kind))
                .unwrap_or_default();
            Some(PlacedSource {
                component_id: component.id,
                is_voltage: is_voltage_source(component.kind),
                family,
                key_figure: key_figure(component, &params),
                nets: terminal_nets(component, &nets),
                consumers,
                reference,
            })
        })
        .collect();
    sources.sort_by(|left, right| {
        left.reference
            .to_ascii_uppercase()
            .cmp(&right.reference.to_ascii_uppercase())
    });
    sources
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
        .map(|component| {
            let params = crate::state::parse_params_string(&component.params);
            PlacedRfPort {
                component_id: component.id,
                reference: component.spice_instance_name(),
                port_number: port_number(&params),
                z0: reference_impedance(&params),
                mode: rf_port_mode(component, &params),
                nets: terminal_nets(component, &nets),
                consumers: consumers.clone(),
            }
        })
        .collect();
    ports.sort_by(|left, right| {
        left.port_number.cmp(&right.port_number).then_with(|| {
            left.reference
                .to_ascii_uppercase()
                .cmp(&right.reference.to_ascii_uppercase())
        })
    });
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
    params: &HashMap<String, String>,
    kind: ComponentType,
) -> Vec<SourceConsumer> {
    attribute_plan(plan, |draft, record| {
        match draft {
            AnalysisDraft::Ac(_) => {
                // The analysis names nothing; the instance carries the drive.
                if carries_ac_excitation(params, kind) {
                    record("AC excitation");
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
            //
            // `.disto` is the one that looks like it belongs with `.ac` above
            // and does not. A distortion run takes its excitation only from
            // explicit `DISTOF1`/`DISTOF2` annotations on a source card:
            // `build_distortion_rhs` in
            // `rspice-core/src/engine/distortion.rs` skips every source whose
            // spec carries no tone, and the run is refused outright when the
            // vector it builds is zero. An AC magnitude contributes nothing to
            // it. No property this product can author emits either annotation,
            // so no placed source drives a distortion run at all, and recording
            // an AC source as its drive would name a binding the run itself
            // rejects.
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
    design_nets(schematic)
        .into_iter()
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
        assert_eq!(placed_rf_port_count(&schematic), 1);
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
        assert_eq!(listed[0].reading_consumers().count(), 0);

        let listed = placed_rf_ports(&schematic, Some(&sp_plan(true)));
        assert!(listed[0].is_read());
        assert_eq!(listed[0].reading_consumers().count(), 1);
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
    /// whose spec carries no tone and refuses a run whose vector is zero), and
    /// no property this product can author emits one. Recording an AC source as
    /// a distortion drive would put a role on the row that dispatching the plan
    /// then rejects, which is the same wrong answer as calling an unread source
    /// read.
    #[test]
    fn a_distortion_run_claims_no_placed_source() {
        let mut plan = SimulationPlan::empty();
        plan.insert(AnalysisKind::Disto)
            .expect("a distortion analysis inserts");
        let schematic = schematic_with(vec![
            source(1, ComponentType::VoltageSourceAc, "V1", ""),
            source(2, ComponentType::VoltageSourceSin, "V2", "ac=1 freq=1k"),
        ]);
        let listed = placed_sources(&schematic, Some(&plan));
        assert!(
            listed.iter().all(|source| source.consumers.is_empty()),
            "an AC magnitude does not drive a distortion run: {listed:?}"
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
}
