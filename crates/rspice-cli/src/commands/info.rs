//! Info Command - Display netlist information

use crate::cli::{CliError, Config, InfoArgs};
use crate::commands::truncate;
use rspice_core::Netlist;
use std::io::Write;
use std::path::Path;

mod elements;
mod hierarchy;
mod parameters;

/// Execute the info command
pub fn execute(
    args: InfoArgs,
    config: &Config,
    _verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    let netlist = crate::commands::parse_netlist_input(&args.input, &args.netlist_options, config)?;

    let mut out = std::io::stdout().lock();
    if args.json {
        print_json(&mut out, &netlist, &args)?;
    } else {
        crate::commands::emit_netlist_diagnostics(&netlist, quiet);
        print_summary(&mut out, &netlist, &args)
            .map_err(|error| CliError::output_error(Path::new("stdout"), error))?;
    }

    out.flush()
        .map_err(|error| CliError::output_error(Path::new("stdout"), error))
}

fn print_summary(out: &mut impl Write, netlist: &Netlist, args: &InfoArgs) -> std::io::Result<()> {
    writeln!(
        out,
        "╔══════════════════════════════════════════════════════════════════╗"
    )?;
    writeln!(out, "║  Netlist: {:<56} ║", truncate(&netlist.title, 56))?;
    writeln!(
        out,
        "╚══════════════════════════════════════════════════════════════════╝"
    )?;
    writeln!(out)?;

    let counts = count_elements(netlist);
    writeln!(out, "Elements ({} total):", counts.total)?;
    if counts.resistors > 0 {
        writeln!(out, "  Resistors:      {:>6}", counts.resistors)?;
    }
    if counts.capacitors > 0 {
        writeln!(out, "  Capacitors:     {:>6}", counts.capacitors)?;
    }
    if counts.inductors > 0 {
        writeln!(out, "  Inductors:      {:>6}", counts.inductors)?;
    }
    if counts.diodes > 0 {
        writeln!(out, "  Diodes:         {:>6}", counts.diodes)?;
    }
    if counts.bjts > 0 {
        writeln!(out, "  BJTs:           {:>6}", counts.bjts)?;
    }
    if counts.mosfets > 0 {
        writeln!(out, "  MOSFETs:        {:>6}", counts.mosfets)?;
    }
    if counts.voltage_sources > 0 {
        writeln!(out, "  Voltage Sources:{:>6}", counts.voltage_sources)?;
    }
    if counts.current_sources > 0 {
        writeln!(out, "  Current Sources:{:>6}", counts.current_sources)?;
    }
    if counts.subcircuits > 0 {
        writeln!(out, "  Subcircuits:    {:>6}", counts.subcircuits)?;
    }
    if counts.other > 0 {
        writeln!(out, "  Other:          {:>6}", counts.other)?;
    }
    writeln!(out)?;

    if !netlist.analyses.is_empty() {
        writeln!(out, "Analyses ({}):", netlist.analyses.len())?;
        for analysis in &netlist.analyses {
            writeln!(out, "  • {:?}", analysis)?;
        }
        writeln!(out)?;
    }

    if args.detailed {
        print_detailed_elements(out, netlist)?;
    }

    if args.models {
        writeln!(out, "Models ({}):", netlist.models.len())?;
        for model in &netlist.models {
            writeln!(out, "  {} ({})", model.name, model.model_type)?;
            for parameter in parameters::model_parameters(model) {
                writeln!(out, "    {parameter}")?;
            }
        }
        writeln!(out)?;
    }

    if args.params {
        let mut params = netlist.params.all_params();
        params.sort_by(|a, b| a.0.cmp(&b.0));
        if params.is_empty() {
            writeln!(out, "Parameters: none")?;
            writeln!(out)?;
        } else {
            writeln!(out, "Parameters ({}):", params.len())?;
            for (name, value) in &params {
                writeln!(out, "  {} = {}", name, value)?;
            }
            writeln!(out)?;
        }
    }

    if args.hierarchy {
        writeln!(out, "Subcircuit definitions and instance references:")?;
        hierarchy::Hierarchy::new(&netlist.elements, &netlist.subcircuits, args.detailed)
            .write(out, 2)?;
        writeln!(out)?;
    }

    if !netlist.measurements.is_empty() {
        writeln!(out, "Measurements ({}):", netlist.measurements.len())?;
        for meas in &netlist.measurements {
            writeln!(out, "  {}", meas.name)?;
        }
        writeln!(out)?;
    }

    Ok(())
}

fn print_detailed_elements(out: &mut impl Write, netlist: &Netlist) -> std::io::Result<()> {
    writeln!(out, "Elements:")?;
    for elem in &netlist.elements {
        elements::ElementDetails::new(elem).write(out, 2)?;
    }
    writeln!(out)
}

fn print_json(out: &mut impl Write, netlist: &Netlist, args: &InfoArgs) -> Result<(), CliError> {
    let counts = count_elements(netlist);

    let json = serde_json::json!({
        "title": netlist.title,
        "elements": {
            "total": counts.total,
            "resistors": counts.resistors,
            "capacitors": counts.capacitors,
            "inductors": counts.inductors,
            "diodes": counts.diodes,
            "bjts": counts.bjts,
            "mosfets": counts.mosfets,
            "voltage_sources": counts.voltage_sources,
            "current_sources": counts.current_sources,
            "subcircuits": counts.subcircuits,
            "other": counts.other,
        },
        "analyses": netlist.analyses.len(),
        "element_details": args.detailed.then(|| netlist.elements.iter().map(elements::ElementDetails::new).collect::<Vec<_>>()),
        "models": if args.models { Some(netlist.models.iter().map(|m| &m.name).collect::<Vec<_>>()) } else { None },
        "model_definitions": args.models.then(|| netlist.models.iter().map(|model| serde_json::json!({
            "name": model.name,
            "model_type": model.model_type,
            "parameters": parameters::model_parameters(model),
        })).collect::<Vec<_>>()),
        "params": if args.params {
            let mut params = netlist.params.all_params();
            params.sort_by(|a, b| a.0.cmp(&b.0));
            Some(params.into_iter().map(|(name, value)| {
                serde_json::json!({"name": name, "value": value})
            }).collect::<Vec<_>>())
        } else { None },
        "subcircuits": if args.hierarchy { Some(netlist.subcircuits.iter().map(|s| &s.name).collect::<Vec<_>>()) } else { None },
        "hierarchy": args.hierarchy.then(|| hierarchy::Hierarchy::new(&netlist.elements, &netlist.subcircuits, args.detailed)),
        "measurements": netlist.measurements.len(),
        "diagnostics": netlist.diagnostics.iter().map(|diagnostic| serde_json::json!({
            "severity": match diagnostic.severity {
                rspice_core::netlist::DiagnosticSeverity::Warning => "warning",
            },
            "line": diagnostic.line,
            "code": &diagnostic.code,
            "message": &diagnostic.message,
        })).collect::<Vec<_>>(),
    });
    let json = crate::observability::envelope("rspice.info", json);

    serde_json::to_writer_pretty(&mut *out, &json)
        .map_err(|error| CliError::output_json_error(Path::new("stdout"), error))?;
    writeln!(out).map_err(|error| CliError::output_error(Path::new("stdout"), error))
}

struct ElementCounts {
    total: usize,
    resistors: usize,
    capacitors: usize,
    inductors: usize,
    diodes: usize,
    bjts: usize,
    mosfets: usize,
    voltage_sources: usize,
    current_sources: usize,
    subcircuits: usize,
    other: usize,
}

fn count_elements(netlist: &Netlist) -> ElementCounts {
    let mut counts = ElementCounts {
        total: netlist.elements.len(),
        resistors: 0,
        capacitors: 0,
        inductors: 0,
        diodes: 0,
        bjts: 0,
        mosfets: 0,
        voltage_sources: 0,
        current_sources: 0,
        subcircuits: 0,
        other: 0,
    };

    for elem in &netlist.elements {
        match &elem.kind {
            rspice_core::netlist::ElementKind::Resistor { .. } => counts.resistors += 1,
            rspice_core::netlist::ElementKind::Capacitor { .. } => counts.capacitors += 1,
            rspice_core::netlist::ElementKind::Inductor { .. }
            | rspice_core::netlist::ElementKind::JilesAthertonInductor { .. } => {
                counts.inductors += 1
            }
            rspice_core::netlist::ElementKind::Diode { .. } => counts.diodes += 1,
            rspice_core::netlist::ElementKind::Bjt { .. } => counts.bjts += 1,
            rspice_core::netlist::ElementKind::Mosfet { .. } => counts.mosfets += 1,
            rspice_core::netlist::ElementKind::VoltageSource(_)
            | rspice_core::netlist::ElementKind::VoltageSourceDeferred(_) => {
                counts.voltage_sources += 1
            }
            rspice_core::netlist::ElementKind::CurrentSource(_)
            | rspice_core::netlist::ElementKind::CurrentSourceDeferred(_) => {
                counts.current_sources += 1
            }
            rspice_core::netlist::ElementKind::Subcircuit { .. } => counts.subcircuits += 1,
            _ => counts.other += 1,
        }
    }

    counts
}
