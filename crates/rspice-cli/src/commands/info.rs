//! Info Command - Display netlist information

use crate::cli::{CliError, InfoArgs};
use rspice_core::Netlist;

/// Execute the info command
pub fn execute(args: InfoArgs, _verbose: bool, quiet: bool) -> Result<(), CliError> {
    if !args.input.exists() {
        return Err(CliError::InputNotFound {
            path: args.input.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    let netlist = Netlist::parse_file(&args.input).map_err(|e| CliError::ParseError {
        message: e.to_string(),
        line: None,
        suggestion: None,
    })?;

    if args.json {
        print_json(&netlist, &args)?;
    } else {
        print_summary(&netlist, &args, quiet)?;
    }

    Ok(())
}

fn print_summary(netlist: &Netlist, args: &InfoArgs, _quiet: bool) -> Result<(), CliError> {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║  Netlist: {:<56} ║", truncate(&netlist.title, 56));
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let counts = count_elements(netlist);
    println!("Elements ({} total):", counts.total);
    if counts.resistors > 0 {
        println!("  Resistors:      {:>6}", counts.resistors);
    }
    if counts.capacitors > 0 {
        println!("  Capacitors:     {:>6}", counts.capacitors);
    }
    if counts.inductors > 0 {
        println!("  Inductors:      {:>6}", counts.inductors);
    }
    if counts.diodes > 0 {
        println!("  Diodes:         {:>6}", counts.diodes);
    }
    if counts.bjts > 0 {
        println!("  BJTs:           {:>6}", counts.bjts);
    }
    if counts.mosfets > 0 {
        println!("  MOSFETs:        {:>6}", counts.mosfets);
    }
    if counts.voltage_sources > 0 {
        println!("  Voltage Sources:{:>6}", counts.voltage_sources);
    }
    if counts.current_sources > 0 {
        println!("  Current Sources:{:>6}", counts.current_sources);
    }
    if counts.subcircuits > 0 {
        println!("  Subcircuits:    {:>6}", counts.subcircuits);
    }
    if counts.other > 0 {
        println!("  Other:          {:>6}", counts.other);
    }
    println!();

    if !netlist.analyses.is_empty() {
        println!("Analyses ({}):", netlist.analyses.len());
        for analysis in &netlist.analyses {
            println!("  • {:?}", analysis);
        }
        println!();
    }

    if args.detailed {
        print_detailed_elements(netlist);
    }

    if args.models && !netlist.models.is_empty() {
        println!("Models ({}):", netlist.models.len());
        for model in &netlist.models {
            println!("  {} ({})", model.name, model.model_type);
        }
        println!();
    }

    if args.hierarchy && !netlist.subcircuits.is_empty() {
        println!("Subcircuits ({}):", netlist.subcircuits.len());
        for subckt in &netlist.subcircuits {
            println!(
                "  {} ({} ports, {} elements)",
                subckt.name,
                subckt.ports.len(),
                subckt.elements.len()
            );
        }
        println!();
    }

    if !netlist.measurements.is_empty() {
        println!("Measurements ({}):", netlist.measurements.len());
        for meas in &netlist.measurements {
            println!("  {}", meas.name);
        }
        println!();
    }

    Ok(())
}

fn print_detailed_elements(netlist: &Netlist) {
    println!("Elements:");
    for elem in &netlist.elements {
        println!("  {}: {:?}", elem.name, elem.kind);
    }
    println!();
}

fn print_json(netlist: &Netlist, args: &InfoArgs) -> Result<(), CliError> {
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
        "models": if args.models { Some(netlist.models.iter().map(|m| &m.name).collect::<Vec<_>>()) } else { None },
        "subcircuits": if args.hierarchy { Some(netlist.subcircuits.iter().map(|s| &s.name).collect::<Vec<_>>()) } else { None },
        "measurements": netlist.measurements.len(),
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&json).unwrap_or_default()
    );
    Ok(())
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
            rspice_core::netlist::ElementKind::Inductor { .. } => counts.inductors += 1,
            rspice_core::netlist::ElementKind::Diode { .. } => counts.diodes += 1,
            rspice_core::netlist::ElementKind::Bjt { .. } => counts.bjts += 1,
            rspice_core::netlist::ElementKind::Mosfet { .. } => counts.mosfets += 1,
            rspice_core::netlist::ElementKind::VoltageSource(_) => counts.voltage_sources += 1,
            rspice_core::netlist::ElementKind::CurrentSource(_) => counts.current_sources += 1,
            rspice_core::netlist::ElementKind::Subcircuit { .. } => counts.subcircuits += 1,
            _ => counts.other += 1,
        }
    }

    counts
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
