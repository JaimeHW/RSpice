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
//! Two consumer relationships that a reader might expect are deliberately
//! absent, because the product does not have them:
//!
//! - **AC** names no source. An `.ac` card sweeps whatever sources carry an AC
//!   magnitude, so the binding lives on the instance, not in the analysis. It
//!   is derived here from the source's own `ac` parameter, and it is reported
//!   against every AC analysis in the plan rather than against one of them.
//! - **FOUR** names no source either. `.four` takes a fundamental *frequency*
//!   and an output node; nothing ties it to the instance that produced the
//!   tone. The one fundamental-to-source binding in the product is harmonic
//!   balance, which is listed.
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

/// Every independent source on this sheet, with what the plan reads it as.
///
/// Sources come back in reference order so the list is stable across edits
/// that do not add or remove one.
pub fn placed_sources(
    schematic: &SchematicState,
    plan: Option<&SimulationPlan>,
) -> Vec<PlacedSource> {
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
        let mut record = |role: &'static str| {
            consumers.push(SourceConsumer {
                analysis: analysis.clone(),
                role,
            });
        };
        match instance.draft() {
            AnalysisDraft::Ac(_) => {
                // The analysis names nothing; the instance carries the drive.
                if carries_ac_excitation(params, kind) {
                    record("AC excitation");
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
            // Corner and operating-point analyses carry a supply-source list on
            // their *config*, which is built at dispatch from the design rather
            // than authored in the draft. There is nothing to read here until a
            // draft holds one, and inventing the reference from the config would
            // report a binding the user never made.
            _ => {}
        }
    }
    consumers
}

/// Whether a plan's source field names this instance.
fn names(field: &str, reference: &str) -> bool {
    !field.trim().is_empty() && field.trim().eq_ignore_ascii_case(reference)
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
