//! Every excitation placed in the design, and what reads each one.
//!
//! Two questions are answered together here because answering them apart is
//! what produced the dead end this replaces: a list of sources with no way to
//! tell which are actually driving anything, beside a plan whose source fields
//! name strings that may not correspond to a placed instance at all.
//!
//! The consumer side has exactly one derivation, [`consumers_for`], and it is
//! private. Both surfaces that show this list — the Simulation Studio page and
//! the Design navigator's rail — call [`placed_sources`] and render what it
//! returns; neither re-derives a reference, so neither can disagree with the
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
        match instance.draft() {
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
            _ => {}
        }
    }
    consumers
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
    params.get("ac").is_some_and(|magnitude| {
        !matches!(
            magnitude.trim().to_ascii_lowercase().as_str(),
            "" | "0" | "+0" | "-0" | "0.0"
        )
    })
}

/// The waveform family label, or `None` for anything that is not an
/// independent source.
///
/// Controlled sources, behavioural sources and RF ports are excluded: none of
/// them is an excitation a plan can name as a source, and listing them would
/// make the count of excitations wrong.
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

    #[test]
    fn only_independent_sources_are_listed() {
        let schematic = schematic_with(vec![
            source(1, ComponentType::VoltageSourceSin, "V1", "freq=1k"),
            source(2, ComponentType::Resistor, "R1", ""),
            source(3, ComponentType::Vcvs, "E1", ""),
            source(4, ComponentType::CurrentSourcePulse, "I1", "per=1m"),
        ]);
        let listed = placed_sources(&schematic, None);
        let references: Vec<&str> = listed
            .iter()
            .map(|source| source.reference.as_str())
            .collect();
        assert_eq!(references, vec!["I1", "V1"]);
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
