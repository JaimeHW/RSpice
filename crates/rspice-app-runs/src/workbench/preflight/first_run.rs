//! The analysis the first Run of a plan nobody has edited runs, chosen once
//! from the sources the design places and written into the plan.
//!
//! A new project's plan holds a one-millisecond transient whatever the
//! drawing is, which shows a megahertz sine as a solid band, a 50 Hz mains
//! source for a twentieth of one period, and an AC-only source or a DC bias
//! as flat lines. The drawing already says what it wants: a source with only
//! an AC magnitude asks for an AC sweep; a pulse, sine or piecewise-linear
//! source asks for a transient long enough to see it repeat, five periods of
//! the slowest; anything else settles to an operating point.

use rspice_app_state::state::{Component, ComponentType};
use rspice_app_state::workbench::app_state::design_history::SetupParts;
use rspice_app_state::workbench::app_state::model::Model;
use rspice_design::parameters::{carries_source_value, parse_params_string};
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::analysis_kind::AnalysisKind;
use rspice_vocabulary::quantity::{format_engineering_value, parse_engineering_value};

/// How many periods of the slowest source the first transient shows.
const PERIODS: f64 = 5.0;

/// How many output steps the first transient's stop time is divided into.
const STEPS: f64 = 1000.0;

/// What the first run of a plan nobody has edited runs.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FirstRun {
    /// `source` carries an AC magnitude and no waveform in time.
    Ac { source: String },
    /// Five periods of `source`, the slowest pulse, sine or piecewise-linear
    /// source: `stop` seconds.
    Transient { stop: f64, source: String },
    /// Nothing on the drawing changes with time or frequency.
    OperatingPoint,
}

/// The first run `components` ask for.
pub(crate) fn first_run<'a>(components: impl IntoIterator<Item = &'a Component>) -> FirstRun {
    let mut ac = None;
    let mut slowest: Option<(f64, &str)> = None;
    for component in components {
        if let Some(period) = period(component) {
            if slowest.is_none_or(|(longest, _)| period > longest) {
                slowest = Some((period, component.name.as_str()));
            }
        } else if ac.is_none() && carries_only_an_ac_magnitude(component) {
            ac = Some(component.name.as_str());
        }
    }
    if let Some(source) = ac {
        return FirstRun::Ac {
            source: source.to_owned(),
        };
    }
    slowest.map_or(FirstRun::OperatingPoint, |(period, source)| {
        FirstRun::Transient {
            stop: PERIODS * period,
            source: source.to_owned(),
        }
    })
}

/// Whether an `.ac` run would drive `component` and nothing in time does: a
/// dedicated AC source, or a DC source whose `ac` parameter is set, which is
/// the test the netlist generator applies before it writes the `AC` field.
fn carries_only_an_ac_magnitude(component: &Component) -> bool {
    match component.kind {
        ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc => true,
        ComponentType::VoltageSource | ComponentType::CurrentSource => carries_source_value(
            parse_params_string(&component.params)
                .get("ac")
                .map(String::as_str),
        ),
        _ => false,
    }
}

/// How long one repeat of a pulse, sine or piecewise-linear source lasts, in
/// seconds, read as the netlist generator writes the source: a pulse train's
/// period (2 µs when none is set), a sine's 1/f (1 MHz when none is set), and
/// the span of a piecewise-linear table.
fn period(component: &Component) -> Option<f64> {
    let params = parse_params_string(&component.params);
    let number = |keys: &[&str]| {
        keys.iter()
            .filter_map(|key| params.get(*key))
            .find(|raw| !raw.trim().is_empty())
            .and_then(|raw| parse_engineering_value(raw).ok())
    };
    // A source placed before its fields were parameters keeps its whole
    // waveform in the value, `SIN(0 1 1k)`, and the generator writes that
    // text as it stands when no field is set.
    let positional = |family: &str, index: usize, keys: &[&str]| {
        if keys.iter().any(|key| params.contains_key(*key)) {
            return None;
        }
        let value = component.value.trim();
        let (head, rest) = value.split_once('(')?;
        if !head.trim().eq_ignore_ascii_case(family) {
            return None;
        }
        rest.trim_end_matches(')')
            .split(|character: char| character.is_whitespace() || character == ',')
            .filter(|field| !field.is_empty())
            .nth(index)
            .and_then(|field| parse_engineering_value(field).ok())
    };
    let seconds = match component.kind {
        ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => {
            const PULSE: &[&str] = &["v1", "v2", "i1", "i2", "td", "tr", "tf", "pw", "per"];
            positional("PULSE", 6, PULSE)
                .or_else(|| number(&["per", "period"]))
                .or(Some(2e-6))
        }
        ComponentType::VoltageSourceSin | ComponentType::CurrentSourceSin => {
            const SIN: &[&str] = &["vo", "va", "io", "ia", "freq", "td", "theta", "phase"];
            positional("SIN", 2, SIN)
                .or_else(|| number(&["freq"]))
                .or(Some(1e6))
                .filter(|frequency| *frequency > 0.0)
                .map(|frequency| 1.0 / frequency)
        }
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => {
            let table = params
                .get("pwl_data")
                .map(String::as_str)
                .filter(|table| !table.trim().is_empty())
                .unwrap_or(component.value.as_str());
            let table = table.trim();
            let table = table
                .strip_prefix("PWL(")
                .or_else(|| table.strip_prefix("pwl("))
                .map_or(table, |inner| inner.trim_end_matches(')'));
            table
                .split(|character: char| character.is_whitespace() || character == ',')
                .filter(|field| !field.is_empty())
                .step_by(2)
                .filter_map(|time| parse_engineering_value(time).ok())
                .filter(|time| time.is_finite())
                .reduce(f64::max)
        }
        _ => None,
    };
    seconds.filter(|seconds| seconds.is_finite() && *seconds > 0.0)
}

/// Choose the first run's analysis when nobody has edited the active plan,
/// and write it into the plan as one step the reader can undo. Returns what
/// the status line says of the choice, or `None` when the plan was edited
/// before or the design places nothing yet.
pub(crate) fn choose_first_analysis(model: &mut Model) -> Option<String> {
    if !model
        .sim_setup
        .stable_analysis_plan()
        .is_ok_and(|plan| plan.is_untouched())
    {
        return None;
    }
    // The design the run starts from, not the sheet on screen: a reader
    // descended into a block still runs the testbench above it.
    let choice = {
        let root = model
            .workspace
            .simulation_root_schematic(&model.workspace.content.active_view, &model.schematic)?;
        let components = &root.document().components;
        if components.is_empty() {
            return None;
        }
        first_run(components)
    };
    let step = model.setup_snapshot(SetupParts::ANALYSES);
    let plan = model.sim_setup.stable_analysis_plan_mut().ok()?;
    // Every step lands on a copy, and only a copy that took them all becomes
    // the plan: a refused step leaves the plan as nobody edited it.
    let mut candidate = plan.clone();
    let transient = candidate
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Transient)?
        .id();
    let words = match &choice {
        FirstRun::Transient { stop, source } => {
            let stop_text = format_engineering_value(*stop);
            let step_text = format_engineering_value(stop / STEPS);
            candidate
                .edit(transient, |draft| {
                    if let AnalysisDraft::Transient(setup) = draft {
                        setup.stop.clone_from(&stop_text);
                        setup.step = step_text;
                    }
                })
                .ok()?;
            format!(
                "Run chose Transient, {}: five periods of {source}. The run chip changes it.",
                rspice_results::verification::format_bound(*stop, "s")
            )
        }
        FirstRun::Ac { source } => {
            // A sweep linearizes about an operating point, which the plan
            // runs first and binds to it.
            let (ac, _) = candidate.insert(AnalysisKind::Ac).ok()?;
            candidate.repair_dependencies(ac).ok()?;
            candidate.remove(transient, Vec::new()).ok()?;
            format!(
                "Run chose an AC sweep: {source} carries an AC magnitude. The run chip changes it."
            )
        }
        FirstRun::OperatingPoint => {
            candidate.insert(AnalysisKind::OperatingPoint).ok()?;
            candidate.remove(transient, Vec::new()).ok()?;
            "Run chose an operating point: no source changes with time. The run chip changes it."
                .to_owned()
        }
    };
    *plan = candidate;
    model.sim_setup.refresh_legacy_analysis_projections();
    model.record_setup_change("first analysis", step);
    Some(words)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_app_state::state::Point;

    fn source(kind: ComponentType, name: &str, params: &str) -> Component {
        let mut component = Component::new(1, kind, Point::new(0, 0));
        name.clone_into(&mut component.name);
        params.clone_into(&mut component.params);
        component
    }

    #[test]
    fn the_slowest_periodic_source_sets_five_periods() {
        let fast = source(ComponentType::VoltageSourceSin, "V1", "freq=10k");
        let slow = source(ComponentType::VoltageSourcePulse, "V2", "per=1m pw=500u");
        let FirstRun::Transient { stop, source } = first_run([&fast, &slow]) else {
            panic!("a periodic source asks for a transient");
        };
        assert_eq!(source, "V2");
        assert!((stop - 5e-3).abs() < 1e-12, "{stop}");
    }

    #[test]
    fn a_source_reads_its_period_as_the_netlist_writes_it() {
        // A fresh sine runs at the generator's 1 MHz, a pulse with no period
        // repeats every 2 µs, a table lasts its last time, and a waveform kept
        // in the value is read from the value.
        let cases = [
            (source(ComponentType::VoltageSourceSin, "V1", ""), 5e-6),
            (
                source(ComponentType::CurrentSourcePulse, "I1", "pw=1u"),
                10e-6,
            ),
            (
                source(
                    ComponentType::VoltageSourcePwl,
                    "V3",
                    &rspice_design::parameters::format_params_string(
                        &[("pwl_data".to_owned(), "0 0 1m 1 3m 0".to_owned())].into(),
                    ),
                ),
                15e-3,
            ),
        ];
        for (component, expected) in cases {
            let FirstRun::Transient { stop, .. } = first_run([&component]) else {
                panic!("{} asks for a transient", component.name);
            };
            assert!(
                (stop - expected).abs() < expected * 1e-9,
                "{}: {stop}",
                component.name
            );
        }
        let mut legacy = source(ComponentType::VoltageSourceSin, "V4", "");
        "SIN(0 1 50)".clone_into(&mut legacy.value);
        let FirstRun::Transient { stop, .. } = first_run([&legacy]) else {
            panic!("a sine kept in the value asks for a transient");
        };
        assert!((stop - 0.1).abs() < 1e-12, "{stop}");
    }

    #[test]
    fn an_ac_only_source_asks_for_an_ac_sweep_and_a_dc_bias_for_an_operating_point() {
        let ac = source(ComponentType::VoltageSourceAc, "VAC", "");
        let dc = source(ComponentType::VoltageSource, "VDD", "");
        assert_eq!(
            first_run([&dc, &ac]),
            FirstRun::Ac {
                source: "VAC".to_owned()
            }
        );
        assert_eq!(first_run([&dc]), FirstRun::OperatingPoint);
        let biased = source(ComponentType::VoltageSource, "VIN", "ac=1");
        assert_eq!(
            first_run([&dc, &biased]),
            FirstRun::Ac {
                source: "VIN".to_owned()
            }
        );
        // An AC field on a pulse does not make it AC-only: it still repeats.
        let pulse = source(ComponentType::VoltageSourcePulse, "VCLK", "per=1u ac=1");
        assert!(matches!(first_run([&pulse]), FirstRun::Transient { .. }));
    }
}
