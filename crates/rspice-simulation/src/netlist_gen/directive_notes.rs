//! SPICE directive notes, written into the deck of the design they are
//! drawn in.
//!
//! A note's lines are written as they stand, between an opening comment that
//! names the note and its cell view and a closing one, so any line of the
//! deck between the two reads back to the note's own line. On the design the
//! deck is written for, they are deck cards. On a block's schematic they are
//! written inside the block's `.subckt` body, so they apply at every
//! occurrence with the block's own names, as a card inside a subcircuit does
//! in SPICE. A card SPICE applies only to a whole deck cannot be scoped that
//! way, and a `.param` naming one of the block's own parameters would hide
//! the value every instance passes, so a block refuses both on their lines.

use super::*;
use crate::preparation::{PreparationError, PreparationStage};
use rspice_design::schematic::design_note::{DesignNote, DesignNoteKind};
use rspice_design::schematic::spice_directive::{directive_keyword, refused_directive_lines};

/// Where one line of a SPICE directive note is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectiveNoteLine {
    /// The cell view whose schematic draws the note.
    pub cell_view: Option<CellViewRef>,
    pub note: u64,
    /// The note's line, counted from 1.
    pub line: usize,
}

const OPENING: &str = "* SPICE directive note ";
const CLOSING: &str = "* End of SPICE directive note ";

/// Cards SPICE reads for the whole deck, which a `.subckt` body cannot scope.
const DECK_WIDE_CARDS: [&str; 8] = [
    ".options", ".option", ".opt", ".temp", ".global", ".include", ".inc", ".lib",
];

/// Where a schematic's directive notes are written.
pub(super) enum DirectiveScope<'p> {
    /// Into the deck itself: the schematic is the design the deck is for.
    Deck,
    /// Into a block's `.subckt` body, whose header declares `parameters`.
    Block { parameters: &'p [String] },
}

impl NetlistGenerator<'_> {
    /// Write every SPICE directive note the schematic draws, or refuse the
    /// lines of each that cannot join the deck here.
    pub(super) fn generate_directive_notes(&mut self, scope: DirectiveScope<'_>) {
        let notes = self
            .schematic
            .design_notes
            .iter()
            .filter(|note| note.kind == DesignNoteKind::SpiceDirective)
            .collect::<Vec<_>>();
        for note in notes {
            let refusals = directive_note_refusals(note, &scope);
            if !refusals.is_empty() {
                for (line, message) in refusals {
                    let defect = NetlistDefect::DirectiveLine {
                        place: DirectiveNoteLine {
                            cell_view: self.cell_view.clone(),
                            note: note.id,
                            line,
                        },
                        message,
                    };
                    self.errors.push(defect.to_string());
                    self.defects.push(defect);
                }
                continue;
            }
            let origin = WrittenLine {
                note: Some(note.id),
                ..self.cell_origin()
            };
            let place = self
                .cell_view
                .as_ref()
                .map(CellViewRef::display_path)
                .unwrap_or_default();
            self.lines
                .push_for(format!("{OPENING}{} ({place})", note.id), origin.clone());
            self.lines.push_for(note.text.clone(), origin.clone());
            self.lines.push_for(format!("{CLOSING}{}", note.id), origin);
        }
    }
}

/// The lines of `note` that cannot be written in `scope`, each with why.
fn directive_note_refusals(note: &DesignNote, scope: &DirectiveScope<'_>) -> Vec<(usize, String)> {
    let mut refusals = refused_directive_lines(&note.text)
        .into_iter()
        .map(|refused| (refused.line, refused.reason()))
        .collect::<Vec<_>>();
    if let DirectiveScope::Block { parameters } = scope {
        for (line, keyword, card) in directive_cards(&note.text) {
            if DECK_WIDE_CARDS.contains(&keyword.as_str()) {
                refusals.push((
                    line,
                    format!(
                        "{keyword} applies to the whole netlist, so a block's schematic cannot \
                         hold it: put it on the top design's sheet"
                    ),
                ));
            } else if matches!(keyword.as_str(), ".param" | ".params") {
                for name in defined_parameters(&card) {
                    if parameters
                        .iter()
                        .any(|parameter| parameter.eq_ignore_ascii_case(&name))
                    {
                        refusals.push((
                            line,
                            format!(
                                "{name} is a parameter of this block, which each instance \
                                 sets: change its default in the block's parameters"
                            ),
                        ));
                    }
                }
            }
        }
    }
    refusals.sort_by_key(|(line, _)| *line);
    refusals
}

/// Each dot card of `text`: the line it starts on (from 1), its keyword,
/// and its words with continuation lines joined.
pub fn directive_cards(text: &str) -> Vec<(usize, String, String)> {
    let mut cards: Vec<(usize, String, String)> = Vec::new();
    let mut continues = false;
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix('+') {
            if continues && let Some((_, _, card)) = cards.last_mut() {
                card.push(' ');
                card.push_str(rest.trim());
            }
            continue;
        }
        continues = false;
        if let Some(keyword) = directive_keyword(line) {
            cards.push((index + 1, keyword, trimmed.trim_end().to_owned()));
            continues = true;
        }
    }
    cards
}

/// The names a `.param` card defines, in order: `r` and `f` for
/// `.param r=1k f(x)={x*2}`.
pub fn defined_parameters(card: &str) -> Vec<String> {
    let body = card
        .split_once(char::is_whitespace)
        .map_or("", |(_, body)| body);
    let characters = body.chars().collect::<Vec<_>>();
    let mut names = Vec::new();
    let mut depth = 0_i32;
    let mut quoted = false;
    for (index, &character) in characters.iter().enumerate() {
        match character {
            '\'' | '"' => quoted = !quoted,
            '{' | '(' | '[' if !quoted => depth += 1,
            '}' | ')' | ']' if !quoted => depth -= 1,
            '=' if !quoted && depth == 0 => {
                let mut end = index;
                while end > 0 && characters[end - 1].is_whitespace() {
                    end -= 1;
                }
                // A function's name stands before its argument list.
                if end > 0 && characters[end - 1] == ')' {
                    let mut open = 0_i32;
                    while end > 0 {
                        end -= 1;
                        match characters[end] {
                            ')' => open += 1,
                            '(' => {
                                open -= 1;
                                if open == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                let mut start = end;
                while start > 0
                    && (characters[start - 1].is_alphanumeric() || characters[start - 1] == '_')
                {
                    start -= 1;
                }
                if start < end {
                    names.push(characters[start..end].iter().collect());
                }
            }
            _ => {}
        }
    }
    names
}

/// What the plan states that a directive note on the design it runs may not
/// state again: nothing silently wins, so a value both state is refused on
/// the note's line, naming where the plan states it.
#[derive(Debug, Clone, Copy)]
pub struct PlanStatements<'a> {
    /// The plan's design variables, which its sweeps and corners vary.
    pub design_variables: &'a [DesignVariable],
    /// The plan's own `.OPTIONS` block, as the run writes it.
    pub options_block: &'a str,
    /// Whether the plan sets a temperature of its own: a reference other than
    /// the default, or a run set that steps temperature.
    pub temperature: bool,
}

/// Every line of the design's directive notes that states again what the
/// plan states, as the defect that refuses it.
pub fn plan_clashes(
    document: &SchematicDocument,
    cell_view: &CellViewRef,
    plan: PlanStatements<'_>,
) -> Vec<NetlistDefect> {
    let plan_options = plan
        .options_block
        .lines()
        .filter_map(|line| line.trim().strip_prefix('+'))
        .filter_map(|setting| setting.split('=').next())
        .map(|key| key.trim().to_ascii_uppercase())
        .collect::<Vec<_>>();
    let mut defects = Vec::new();
    for note in document
        .design_notes
        .iter()
        .filter(|note| note.kind == DesignNoteKind::SpiceDirective)
    {
        let place = |line| DirectiveNoteLine {
            cell_view: Some(cell_view.clone()),
            note: note.id,
            line,
        };
        for (line, keyword, card) in directive_cards(&note.text) {
            let mut refuse = |message: String| {
                defects.push(NetlistDefect::DirectiveLine {
                    place: place(line),
                    message,
                });
            };
            match keyword.as_str() {
                ".param" | ".params" => {
                    for name in defined_parameters(&card) {
                        if plan
                            .design_variables
                            .iter()
                            .any(|variable| variable.name.eq_ignore_ascii_case(&name))
                        {
                            refuse(format!(
                                "{name} is also a design variable of the plan: delete one of them"
                            ));
                        }
                    }
                }
                ".temp" if plan.temperature => refuse(
                    "the plan sets the temperature in Simulate ▸ PVT, sweeps & variation:                      delete this .temp, or set the temperature there"
                        .to_owned(),
                ),
                ".options" | ".option" | ".opt" => {
                    for word in card.split_whitespace().skip(1) {
                        let key = word.split('=').next().unwrap_or(word).to_ascii_uppercase();
                        if plan_options.contains(&key) {
                            refuse(format!(
                                "the plan's options already set {key}: delete it here, or                                  change it in Simulate ▸ Solver & convergence"
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    defects
}

/// The failure that stops a deck the generator raised `errors` and
/// `defects` for, on the directive note line of the first defect a note's
/// line raised. `None` when nothing stops it.
pub fn refusal_of(errors: &[String], defects: &[NetlistDefect]) -> Option<PreparationError> {
    let mut said = errors.to_vec();
    for defect in defects {
        let text = defect.to_string();
        if !said.contains(&text) {
            said.push(text);
        }
    }
    if said.is_empty() {
        return None;
    }
    let error = PreparationError::new(PreparationStage::Netlist, said.join("; "));
    Some(
        match defects.iter().find_map(|defect| match defect {
            NetlistDefect::DirectiveLine { place, .. } => Some(place.clone()),
            _ => None,
        }) {
            Some(place) => error.on_directive_note(place),
            None => error,
        },
    )
}

/// The directive note line `line` (counted from 1) of `deck` stands on, if
/// it is a line a note wrote.
pub fn directive_note_at_deck_line(deck: &str, line: usize) -> Option<DirectiveNoteLine> {
    let lines = deck.lines().collect::<Vec<_>>();
    let at = line.checked_sub(1)?;
    lines.get(at)?;
    for (index, text) in lines[..at].iter().enumerate().rev() {
        if text.starts_with(CLOSING) {
            return None;
        }
        let Some(rest) = text.strip_prefix(OPENING) else {
            continue;
        };
        let (note, place) = rest.split_once(' ')?;
        let place = place.strip_prefix('(')?.strip_suffix(')')?;
        let cell_view = match place.split('/').collect::<Vec<_>>().as_slice() {
            [library, cell, view] => Some(CellViewRef::new(*library, *cell, *view)),
            _ => None,
        };
        return Some(DirectiveNoteLine {
            cell_view,
            note: note.parse().ok()?,
            line: at - index,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_param_card_names_each_parameter_and_function_it_defines() {
        assert_eq!(
            defined_parameters(".param r = 1k f(x,y)={x*max(y,1)} c='2*r' t={a=b}"),
            ["r", "f", "c", "t"]
        );
    }

    #[test]
    fn continuation_lines_join_the_card_they_continue() {
        assert_eq!(
            directive_cards("* bias\n.param a=1\n+ b=2\nK1 L1 L2 1\n.ic v(out)=0"),
            [
                (2, ".param".to_owned(), ".param a=1 b=2".to_owned()),
                (5, ".ic".to_owned(), ".ic v(out)=0".to_owned()),
            ]
        );
    }

    #[test]
    fn a_deck_line_inside_a_note_reads_back_to_the_note_and_its_line() {
        let deck = "title\nR1 a 0 1k\n* SPICE directive note 7 (work/amp/schematic)\n\
                    .param r=1k\n.ic v(a)=1\n* End of SPICE directive note 7\n.end";
        assert_eq!(
            directive_note_at_deck_line(deck, 5),
            Some(DirectiveNoteLine {
                cell_view: Some(CellViewRef::new("work", "amp", "schematic")),
                note: 7,
                line: 2,
            })
        );
        assert_eq!(directive_note_at_deck_line(deck, 2), None);
        assert_eq!(directive_note_at_deck_line(deck, 7), None);
    }
}
