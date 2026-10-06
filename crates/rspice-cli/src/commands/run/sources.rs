//! Reserve source directory entries before any result or failure report is written.
use super::*;
use rspice_core::abort_signal::AbortSignal;
use std::path::Path;

fn protect_file(path: &Path) -> Result<(), CliError> {
    publish::destinations::protect(path)?;
    if let Ok(canonical) = path.canonicalize() {
        publish::destinations::protect(&canonical)?;
    }
    Ok(())
}

fn check_abort(timeout: Option<f64>) -> Result<(), CliError> {
    if crate::abort::ProcessAbort.is_aborted() {
        Err(cancellation_cli_error(timeout))
    } else {
        Ok(())
    }
}

pub(super) fn protect(
    netlist: &Netlist,
    engine: &Engine,
    timeout: Option<f64>,
) -> Result<(), CliError> {
    check_abort(timeout)?;
    for path in netlist.included_source_paths() {
        check_abort(timeout)?;
        protect_file(path)?;
    }
    if let Some(directive) = &netlist.device_initial_conditions
        && let rspice_core::netlist::DeviceInitialConditionSource::File {
            resolved_path: Some(path),
            ..
        } = &directive.source
    {
        // INITCOND uses the execution directory's resolver, not the deck's.
        protect_file(path)?;
    }
    for measurement in &netlist.measurements {
        check_abort(timeout)?;
        if let rspice_core::netlist::measure::MeasureType::FileError { file, .. } =
            &measurement.measure_type
            && !file.path().contains("://")
        {
            // The parser has already resolved native measurement references.
            protect_file(Path::new(file.path()))?;
        }
    }
    // SPEF retains its authored spelling; the parser resolves it relative to
    // the path-backed root deck when importing the parasitics.
    let source_base = netlist
        .source_path
        .as_deref()
        .and_then(Path::parent)
        .unwrap_or_else(|| Path::new("."));
    for spelling in &netlist.spef_includes {
        check_abort(timeout)?;
        let path = Path::new(spelling);
        if path.is_absolute() {
            protect_file(path)?;
        } else {
            protect_file(&source_base.join(path))?;
        }
    }
    // Deferred independent sources retain their waveform grammar inside
    // subcircuits. Inspect filenames without evaluating instance parameters or
    // advancing statistical streams. The engine consumes these path spellings.
    let mut definitions: Vec<_> = netlist.subcircuits.iter().collect();
    let mut elements = netlist.elements.as_slice();
    loop {
        check_abort(timeout)?;
        for element in elements {
            check_abort(timeout)?;
            if let Some(path) =
                rspice_core::netlist::independent_source_file_dependency(&element.kind)
                    .map_err(crate::commands::map_parse_error)?
            {
                protect_file(Path::new(path.as_ref()))?;
            }
        }
        let Some(definition) = definitions.pop() else {
            break;
        };
        elements = &definition.elements;
        definitions.extend(&definition.nested_subcircuits);
    }
    if !netlist.veriloga_includes.is_empty() {
        for path in engine
            .veriloga_source_dependencies_with_abort(netlist, &crate::abort::ProcessAbort)
            .map_err(|error| {
                if matches!(error, rspice_core::SimulationError::Aborted) {
                    cancellation_cli_error(timeout)
                } else {
                    error.into()
                }
            })?
        {
            protect_file(&path)?;
        }
    }
    Ok(())
}
