//! The model-library Verilog-A each process corner's deck installs.
//!
//! A library's corner sections may bind one alias to a different source each:
//! `amp` is `amp_tt.va` in `tt` and `amp_ss.va` in `ss`. The run's deck names
//! the runtimes its reference sections read; a deck materialized for a corner
//! names that corner's instead, and every task installs the set its own deck
//! names ([`crate::veriloga::PreparedVerilogARuntimeSet::named_by`]).

use super::*;
use crate::veriloga::PreparedVerilogARuntimeSet;

/// The `.veriloga` lines of a deck, as the directives they are.
pub(super) fn directive_lines(deck: &str) -> std::collections::BTreeSet<String> {
    deck.lines()
        .map(|line| line.trim().to_ascii_lowercase())
        .filter(|line| line.starts_with(".veriloga "))
        .collect()
}

/// The model-library runtimes the run's deck names for its reference model
/// selection, and the ones each process corner's sections name instead.
#[derive(Debug, Clone, Default)]
pub(crate) struct CornerVerilogARuntimes {
    reference: PreparedVerilogARuntimeSet,
    by_process: Vec<(ProcessCorner, PreparedVerilogARuntimeSet)>,
}

impl CornerVerilogARuntimes {
    /// `reference` must be the model-library runtimes whose directives the
    /// run's deck carries; `by_process` one set for every process a corner
    /// contract of the run materializes.
    pub(crate) fn new(
        reference: PreparedVerilogARuntimeSet,
        by_process: Vec<(ProcessCorner, PreparedVerilogARuntimeSet)>,
    ) -> Self {
        Self {
            reference,
            by_process,
        }
    }

    /// Every set a deck of the run may name a source from, beside the run's
    /// own.
    pub(super) fn sets(&self) -> impl Iterator<Item = &PreparedVerilogARuntimeSet> {
        self.by_process.iter().map(|(_, set)| set)
    }

    /// Replace the reference sections' directives in a deck materialized for
    /// `process` with that corner's own.
    pub(super) fn bind_process(
        &self,
        deck: String,
        process: ProcessCorner,
    ) -> Result<String, PreparationError> {
        let corner = match self
            .by_process
            .iter()
            .find(|(candidate, _)| *candidate == process)
        {
            Some((_, corner)) => corner,
            None if self.reference.is_empty() => return Ok(deck),
            None => {
                return Err(PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "The {} process corner's model-library Verilog-A was not prepared",
                        process.short_name()
                    ),
                ));
            }
        };
        let directives = |set: &PreparedVerilogARuntimeSet| {
            set.sources()
                .map(|source| {
                    crate::netlist_preparation::project_veriloga_directive(
                        source.source_key(),
                        source.netlist_alias(),
                    )
                })
                .collect::<Vec<_>>()
        };
        let reference = directives(&self.reference);
        if reference == directives(corner) {
            return Ok(deck);
        }
        let mut bound = deck
            .split_inclusive('\n')
            .filter(|line| {
                !reference
                    .iter()
                    .any(|directive| line.trim().eq_ignore_ascii_case(directive))
            })
            .collect::<String>();
        for source in corner.sources() {
            crate::netlist_preparation::append_project_veriloga_directive(
                &mut bound,
                source.source_key(),
                source.netlist_alias(),
            );
        }
        Ok(bound)
    }
}
