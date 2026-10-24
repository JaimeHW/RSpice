//! A deck's `.VEC` vector files drive their input columns.
//!
//! Each input bit becomes an ideal piecewise-linear voltage source, named
//! `VVEC_<BIT>`, from the bit's node to its VREF node
//! (`netlist::hspice_vec`). The file is read here, when the circuit is built,
//! so a prepared run reads the bytes it sealed and registered under the
//! card's spelling, and a deck run from its file reads it from beside the
//! deck. The expected outputs are checked after a transient by the `.ASSERT`
//! evaluator, which reads the same file.

use super::SimulationError;
use crate::Netlist;
use crate::netlist::canonical_symbol;

/// The prefix of a vector input's source name.
const SOURCE_PREFIX: &str = "vvec_";

/// The deck with every `.VEC` file's input sources added and its cards
/// consumed, so an elaboration of the result does not add them again.
pub(super) fn with_vector_inputs(
    netlist: &Netlist,
    limit: usize,
) -> Result<Netlist, SimulationError> {
    let mut lowered = netlist.clone();
    let references = std::mem::take(&mut lowered.vector_files);
    let mut names: std::collections::HashMap<String, String> = lowered
        .elements
        .iter()
        .map(|element| (canonical_symbol(&element.name), "the deck".to_owned()))
        .collect();
    for reference in &references {
        let file = crate::analysis::load_vector_file(reference, &netlist.params, limit)
            .map_err(SimulationError::Circuit)?;
        let sources = file.input_sources(SOURCE_PREFIX).map_err(|error| {
            SimulationError::Circuit(format!(
                "{}:{}: {} (vector file of the .VEC card at {})",
                reference.path, error.line, error.message, reference.origin
            ))
        })?;
        for source in sources {
            let owner = format!("the .VEC card at {}", reference.origin);
            if let Some(previous) = names.insert(canonical_symbol(&source.name), owner.clone()) {
                return Err(SimulationError::Circuit(format!(
                    "{owner}: the input source {} it adds for node {} is already named by {previous}",
                    source.name, source.nodes[0]
                )));
            }
            lowered.elements.push(source);
        }
    }
    Ok(lowered)
}
