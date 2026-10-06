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

pub(super) fn protect(
    netlist: &Netlist,
    engine: &Engine,
    timeout: Option<f64>,
) -> Result<(), CliError> {
    for path in netlist.included_source_paths() {
        protect_file(path)?;
    }
    // Deferred independent sources retain their waveform grammar inside
    // subcircuits. Inspect filenames without evaluating instance parameters or
    // advancing statistical streams. The engine consumes these path spellings.
    let mut definitions: Vec<_> = netlist.subcircuits.iter().collect();
    let mut elements = netlist.elements.as_slice();
    loop {
        if crate::abort::ProcessAbort.is_aborted() {
            return Err(cancellation_cli_error(timeout));
        }
        for element in elements {
            if crate::abort::ProcessAbort.is_aborted() {
                return Err(cancellation_cli_error(timeout));
            }
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
