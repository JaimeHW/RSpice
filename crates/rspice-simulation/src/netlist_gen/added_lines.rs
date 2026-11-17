//! A design's Added lines, appended to the netlist generated for it.
//!
//! Every deck generated for a design (each plan run, the Netlist workspace's
//! generated netlist, File ▸ Export ▸ Netlist) ends with the design's Added
//! lines, between two comment lines, before `.end`. Only the root design's
//! block applies. A card the engine reads as an analysis is refused on its
//! line, never dropped: analyses, sweeps and temperatures belong to the plan.

use std::fmt;

use rspice_design::added_lines::{AddedLines, added_lines_of};
use rspice_design_model::cell_view::CellViewRef;

/// A card the Added lines may not hold, on its line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddedLineRefusal {
    /// The line of the Added lines text, counted from 1.
    pub line: usize,
    pub message: String,
}

/// Why a design's Added lines stopped its deck: every refused card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddedLinesRefused {
    pub design: CellViewRef,
    pub refusals: Vec<AddedLineRefusal>,
}

impl fmt::Display for AddedLinesRefused {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Added lines of {}", self.design.display_path())?;
        for (index, refusal) in self.refusals.iter().enumerate() {
            let separator = if index == 0 { ", " } else { "; " };
            write!(
                formatter,
                "{separator}line {}: {}",
                refusal.line, refusal.message
            )?;
        }
        Ok(())
    }
}

impl std::error::Error for AddedLinesRefused {}

/// The cards `text` may not hold, each on its line: every card the engine
/// reads as an analysis, and an `.end` that would hide the lines after it.
pub fn added_lines_refusals(text: &str) -> Vec<AddedLineRefusal> {
    text.lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let keyword = line.split_whitespace().next()?.to_ascii_lowercase();
            let message = if keyword == ".end" {
                "The netlist ends after the Added lines: delete this .end".to_owned()
            } else if !rspice_core::netlist::is_analysis_directive(&keyword) {
                return None;
            } else if matches!(keyword.as_str(), ".step" | ".temp" | ".mc") {
                format!(
                    "Sweeps and temperatures belong to the plan: set {keyword} in Simulate ▸ PVT, sweeps & variation"
                )
            } else {
                format!("Analyses belong to the plan: add {keyword} in Simulate ▸ Analyses")
            };
            Some(AddedLineRefusal {
                line: index + 1,
                message,
            })
        })
        .collect()
}

/// `netlist`, generated for `design`, with the design's Added lines before
/// its `.end`. A design with no Added lines leaves the deck as it was.
pub fn append_added_lines(
    netlist: String,
    design: &CellViewRef,
    blocks: &[AddedLines],
) -> Result<String, AddedLinesRefused> {
    let Some(text) = added_lines_of(blocks, design) else {
        return Ok(netlist);
    };
    let refusals = added_lines_refusals(text);
    if !refusals.is_empty() {
        return Err(AddedLinesRefused {
            design: design.clone(),
            refusals,
        });
    }
    let block = format!(
        "* Added lines ({})\n{}\n* End of added lines\n\n",
        design.display_path(),
        text.trim_end()
    );
    Ok(crate::netlist_preparation::splice_before_terminal_end_card(
        &netlist, &block,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::added_lines::set_added_lines;

    fn design() -> CellViewRef {
        CellViewRef::new("work", "amp", "schematic")
    }

    fn blocks(text: &str) -> Vec<AddedLines> {
        let mut blocks = Vec::new();
        set_added_lines(&mut blocks, &design(), text.to_owned());
        blocks
    }

    #[test]
    fn the_block_lands_before_end_between_its_two_comment_lines() {
        let deck = append_added_lines(
            "amp\nR1 in out 1k\n.tran 1n 1u\n\n.end".to_owned(),
            &design(),
            &blocks(".nodeset v(out)=1\nK1 L1 L2 0.9\n\n"),
        )
        .expect("nothing in the block is refused");
        assert_eq!(
            deck,
            "amp\nR1 in out 1k\n.tran 1n 1u\n\n* Added lines (work/amp/schematic)\n\
             .nodeset v(out)=1\nK1 L1 L2 0.9\n* End of added lines\n\n.end"
        );
    }

    #[test]
    fn only_the_designs_own_block_applies() {
        let other = CellViewRef::new("work", "bias", "schematic");
        let deck = "amp\nR1 in out 1k\n.end".to_owned();
        assert_eq!(
            append_added_lines(deck.clone(), &other, &blocks(".nodeset v(out)=1")),
            Ok(deck)
        );
    }

    #[test]
    fn an_analysis_card_is_refused_on_its_line() {
        let refusals = added_lines_refusals(
            "* start-up\n.nodeset v(out)=1\n  .TRAN 1n 1u\n.step param r 1k 2k 1k\n.end",
        );
        assert_eq!(
            refusals,
            vec![
                AddedLineRefusal {
                    line: 3,
                    message: "Analyses belong to the plan: add .tran in Simulate ▸ Analyses"
                        .to_owned(),
                },
                AddedLineRefusal {
                    line: 4,
                    message: "Sweeps and temperatures belong to the plan: set .step in Simulate ▸ PVT, sweeps & variation"
                        .to_owned(),
                },
                AddedLineRefusal {
                    line: 5,
                    message: "The netlist ends after the Added lines: delete this .end".to_owned(),
                },
            ]
        );

        let refused = append_added_lines(
            "amp\n.end".to_owned(),
            &design(),
            &blocks(".nodeset v(out)=1\n.tran 1n 1u"),
        )
        .expect_err("a .tran is refused, not dropped");
        assert_eq!(
            refused.to_string(),
            "Added lines of work/amp/schematic, line 2: Analyses belong to the plan: add .tran in Simulate ▸ Analyses"
        );
    }
}
