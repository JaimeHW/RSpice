use super::{MeasurementUnit, annotate};
use crate::analysis::measure_signals::measurements_for_analysis;
use crate::analysis::noise::NoiseInputQuantity;
use crate::netlist::{AnalysisCommand, ElementKind, Netlist};
use std::collections::HashMap;

fn known(symbol: &str) -> MeasurementUnit {
    MeasurementUnit::Known(symbol.to_owned())
}

pub(crate) fn annotate_native(
    netlist: &Netlist,
    family: &str,
    results: &mut [crate::MeasureResult],
    signal_names: impl Iterator<Item = impl AsRef<str>>,
    noise_input: Option<NoiseInputQuantity>,
) {
    let statements = measurements_for_analysis(netlist, family);
    let axis = match family {
        "TRAN" => known("s"),
        "AC" | "NOISE" => known("Hz"),
        "DC" => dc_axis(netlist),
        _ => MeasurementUnit::Unknown,
    };
    let mut signals = HashMap::from([("TIME".into(), axis.clone())]);
    if family == "NOISE" {
        let output = known("V^2/Hz");
        let input = noise_input.map_or(MeasurementUnit::Unknown, |input| {
            known(input.density_unit())
        });
        for name in ["ONOISE", "ONOISE()", "ONOISE_SPECTRUM"] {
            signals.insert(name.into(), output.clone());
        }
        for name in ["INOISE", "INOISE()", "INOISE_SPECTRUM"] {
            signals.insert(name.into(), input.clone());
        }
        for name in signal_names {
            let name = name.as_ref().to_ascii_uppercase();
            if name.starts_with("DNO(") {
                signals.insert(name, output.clone());
            } else if name.starts_with("DNI(") {
                signals.insert(name, input.clone());
            }
        }
    }
    annotate(&statements, results, Some(&axis), &signals);
}

fn dc_axis(netlist: &Netlist) -> MeasurementUnit {
    // Multiple DC cards can be retained by external callers. Only assign a
    // dimension if all possible primary sweeps agree.
    let mut axes = netlist.analyses.iter().filter_map(|analysis| {
        let AnalysisCommand::Dc { source, .. } = analysis else {
            return None;
        };
        Some(dc_source_unit(netlist, source))
    });
    let Some(first) = axes.next() else {
        return MeasurementUnit::Unknown;
    };
    if axes.all(|axis| axis == first) {
        first
    } else {
        MeasurementUnit::Unknown
    }
}

fn dc_source_unit(netlist: &Netlist, source: &str) -> MeasurementUnit {
    if source.eq_ignore_ascii_case("TEMP") || source.eq_ignore_ascii_case("TEMPERATURE") {
        return known("degC");
    }
    let (name, parameter) = if netlist
        .elements
        .iter()
        .any(|element| element.name.eq_ignore_ascii_case(source))
    {
        (source, "")
    } else {
        source
            .rsplit_once(':')
            .map_or((source, ""), |(name, parameter)| (name, parameter))
    };
    let Some(element) = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case(name))
    else {
        return MeasurementUnit::Unknown;
    };
    let parameter = parameter.to_ascii_uppercase();
    let symbol = match (&element.kind, parameter.as_str()) {
        (ElementKind::VoltageSource(_) | ElementKind::VoltageSourceDeferred(_), "" | "DC") => "V",
        (ElementKind::CurrentSource(_) | ElementKind::CurrentSourceDeferred(_), "" | "DC") => "A",
        (ElementKind::Resistor { .. }, "" | "R" | "RES" | "RESISTANCE" | "VALUE") => "ohm",
        (ElementKind::Capacitor { .. }, "" | "C" | "CAP" | "CAPACITANCE" | "VALUE") => "F",
        (
            ElementKind::Inductor { .. } | ElementKind::JilesAthertonInductor { .. },
            "" | "L" | "IND" | "INDUCTANCE" | "VALUE",
        ) => "H",
        (ElementKind::Capacitor { .. }, "IC") => "V",
        (_, "TEMP") => "degC",
        (_, "DTEMP") => "K",
        (_, "M" | "MULT" | "SCALE") => "1",
        _ => return MeasurementUnit::Unknown,
    };
    known(symbol)
}
