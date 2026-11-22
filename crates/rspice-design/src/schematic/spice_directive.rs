//! What a SPICE directive note may hold.
//!
//! A directive note's lines join the netlist of the design it is drawn in,
//! as written. The plan owns analyses and sweeps, so a card the engine reads
//! as one is refused on its line, never dropped; `.temp` is the one such card
//! a note may carry, as the temperature of a design whose plan states none.
//! An `.end` would hide every card after the note, so it is refused too.

use std::fmt;

/// The analysis cards a typed line can be written into the plan from.
pub const PLAN_ANALYSIS_CARDS: [&str; 6] = [".op", ".tran", ".ac", ".dc", ".noise", ".tf"];

/// Why one line of a directive note cannot join the netlist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectiveRefusal {
    /// An analysis, which belongs to the plan.
    Analysis,
    /// A sweep or a Monte Carlo run, which belongs to the plan's run set.
    Sweep,
    /// `.end`, which would end the netlist at the note.
    End,
}

/// A line of a directive note the netlist cannot take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusedDirectiveLine {
    /// The note's line, counted from 1.
    pub line: usize,
    /// The card's keyword, lower case (`.tran`).
    pub keyword: String,
    /// The line as written, trimmed.
    pub card: String,
    pub refusal: DirectiveRefusal,
}

impl RefusedDirectiveLine {
    /// Whether "Add to the plan" can write this card into the plan as the
    /// Analyses page would.
    pub fn adds_to_plan(&self) -> bool {
        self.refusal == DirectiveRefusal::Analysis
            && PLAN_ANALYSIS_CARDS.contains(&self.keyword.as_str())
    }

    /// What is wrong with the line, without where it is.
    pub fn reason(&self) -> String {
        let keyword = &self.keyword;
        match self.refusal {
            DirectiveRefusal::Analysis if self.adds_to_plan() => {
                format!("{keyword} is an analysis, and the plan owns analyses")
            }
            DirectiveRefusal::Analysis => format!(
                "{keyword} is an analysis, and the plan owns analyses: add it in Simulate ▸ Analyses"
            ),
            DirectiveRefusal::Sweep => format!(
                "{keyword} is a sweep, and the plan owns sweeps: set it in Simulate ▸ PVT, sweeps & variation"
            ),
            DirectiveRefusal::End => "the netlist would end at this .end: delete it".to_owned(),
        }
    }
}

impl fmt::Display for RefusedDirectiveLine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Line {}: {}", self.line, self.reason())
    }
}

/// The dot card a line starts, lower case: `.param` for `.PARAM R=1k`. A
/// comment, a continuation or an element card starts none.
pub fn directive_keyword(line: &str) -> Option<String> {
    let word = line.split_whitespace().next()?;
    word.starts_with('.').then(|| word.to_ascii_lowercase())
}

/// Every line of `text` a directive note may not hold.
pub fn refused_directive_lines(text: &str) -> Vec<RefusedDirectiveLine> {
    text.lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let keyword = directive_keyword(line)?;
            let refusal = if keyword == ".end" {
                DirectiveRefusal::End
            } else if keyword == ".temp" || !rspice_core::netlist::is_analysis_directive(&keyword) {
                return None;
            } else if matches!(keyword.as_str(), ".step" | ".mc") {
                DirectiveRefusal::Sweep
            } else {
                DirectiveRefusal::Analysis
            };
            Some(RefusedDirectiveLine {
                line: index + 1,
                keyword,
                card: line.trim().to_owned(),
                refusal,
            })
        })
        .collect()
}

/// `text` without its line `line` (counted from 1), as when that card has
/// gone to the plan.
pub fn without_line(text: &str, line: usize) -> String {
    text.lines()
        .enumerate()
        .filter(|(index, _)| index + 1 != line)
        .map(|(_, line)| line)
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyses_sweeps_and_end_are_refused_on_their_lines_and_temp_is_carried() {
        let refused = refused_directive_lines(
            "* start-up\n.param r=1k\n  .TRAN 1n 1u\n.temp 85\n.step param r 1k 2k 1k\n.four 1k v(out)\n.end",
        );
        let lines = refused
            .iter()
            .map(|refused| {
                (
                    refused.line,
                    refused.keyword.as_str(),
                    refused.adds_to_plan(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            lines,
            [
                (3, ".tran", true),
                (5, ".step", false),
                (6, ".four", false),
                (7, ".end", false)
            ]
        );
        assert_eq!(
            refused[0].to_string(),
            "Line 3: .tran is an analysis, and the plan owns analyses"
        );
        assert_eq!(refused[0].card, ".TRAN 1n 1u");
        assert_eq!(
            refused[1].to_string(),
            "Line 5: .step is a sweep, and the plan owns sweeps: set it in Simulate ▸ PVT, sweeps & variation"
        );
    }

    #[test]
    fn a_card_gone_to_the_plan_leaves_the_other_lines_as_they_were() {
        assert_eq!(
            without_line(".param r=1k\n.tran 1n 1u\n.ic v(out)=1", 2),
            ".param r=1k\n.ic v(out)=1"
        );
    }
}
