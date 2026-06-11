//! `.MEAS` evaluation against bridge waveforms.
//!
//! The engine parses `.MEAS TRAN|AC|DC` statements into the netlist; this
//! module evaluates the ones matching a finished analysis and returns the
//! results for the run record (specs matrix, console echo). Signal names
//! are aliased both ways — plain node keys gain a `V(node)` alias and
//! wrapped keys gain the inner name — so statements written either way
//! resolve. Complex (AC) waveforms are measured on magnitude, the SPICE
//! convention for `V(x)` in `.MEAS AC`.

use std::collections::HashMap;

use crate::simulation::results::WaveformData;

/// Evaluate the netlist's `.MEAS <analysis>` statements against one
/// analysis' waveforms. Returns an empty vector when the netlist carries
/// no matching statements.
pub(super) fn evaluate_measurements(
    netlist: &rspice_core::Netlist,
    analysis: &str,
    x: &[f64],
    waveforms: &HashMap<String, WaveformData>,
) -> Vec<rspice_core::MeasureResult> {
    let statements: Vec<_> = netlist
        .measurements
        .iter()
        .filter(|m| m.analysis.eq_ignore_ascii_case(analysis))
        .collect();
    if statements.is_empty() || x.is_empty() {
        return Vec::new();
    }

    // Owned magnitude arrays for complex waveforms; the signals map borrows
    // from these and from real waveforms' y arrays.
    let magnitudes: Vec<(&String, Vec<f64>)> = waveforms
        .iter()
        .filter_map(|(name, wf)| {
            let imag = wf.y_imag.as_ref()?;
            let mag = wf
                .y_values
                .iter()
                .zip(imag)
                .map(|(re, im)| re.hypot(*im))
                .collect();
            Some((name, mag))
        })
        .collect();

    let mut signals: HashMap<String, &[f64]> = HashMap::new();
    for (name, mag) in &magnitudes {
        insert_aliases(&mut signals, name, mag.as_slice());
    }
    for (name, wf) in waveforms {
        if wf.y_imag.is_some() {
            continue; // magnitude alias already inserted
        }
        if wf.y_values.len() != x.len() {
            continue; // length-mismatched trace would misindex the x grid
        }
        insert_aliases(&mut signals, name, wf.y_values.as_slice());
    }

    let mut engine = rspice_core::MeasureEngine::new();
    for statement in statements {
        engine.add(statement.clone());
    }
    engine.evaluate(x, &signals)
}

/// Insert one trace under its own key plus the spelling variants a `.MEAS`
/// statement may use: wrapped keys gain the inner name, plain node keys
/// gain `V(node)`. First insertion wins on alias collisions.
fn insert_aliases<'a>(signals: &mut HashMap<String, &'a [f64]>, name: &str, values: &'a [f64]) {
    signals.insert(name.to_owned(), values);
    if let Some(inner) = wrapped_inner(name) {
        signals.entry(inner.to_owned()).or_insert(values);
    } else if !name.contains('(') {
        signals.entry(format!("V({name})")).or_insert(values);
    }
}

/// `V(out)` / `I(v1)` → `out` / `v1`.
fn wrapped_inner(name: &str) -> Option<&str> {
    let (head, rest) = name.split_once('(')?;
    if !(head.eq_ignore_ascii_case("V") || head.eq_ignore_ascii_case("I")) {
        return None;
    }
    rest.strip_suffix(')')
}
