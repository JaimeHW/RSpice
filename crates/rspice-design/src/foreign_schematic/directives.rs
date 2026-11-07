//! Where each directive card of a foreign drawing belongs in a project.
//!
//! A drawing's directive blocks are deck text. A project keeps the same
//! content in typed places: analyses in the simulation plan, constant
//! parameters as design variables, model and subcircuit text in a model
//! library of the project, and the files a deck includes as model libraries
//! bound in place. Each card is sorted here by the place it belongs; a card
//! with no such place stays on the sheet as text.

use rspice_design_model::Point;

use super::import::lettered_note;
use super::{ForeignDirective, ForeignLettering};
use crate::netlist_document::card_tokens;
use crate::schematic::design_note::{DesignNote, DesignNoteKind};
use crate::schematic::document::SchematicDocument;
use crate::schematic::identity::SchematicIdentity;

/// The project place a directive card belongs in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForeignCardRole {
    /// `.op`, `.tran`, `.ac`, `.dc`, `.noise`, `.tf`: an analysis of the plan.
    Analysis,
    /// `.param`: design variables, or model-library text for an expression.
    Parameter,
    /// `.options`: the plan's simulator options.
    Options,
    /// `.temp`: the plan's temperature.
    Temperature,
    /// `.step`: a sweep of the plan.
    Step,
    /// `.meas`: a measurement.
    Measurement,
    /// `.model`, `.func` and a whole `.subckt` … `.ends`: model-library text.
    ModelText,
    /// `.include` and `.lib`: a model file bound in place.
    Include,
    /// Read by the source program alone and meaningless to any other
    /// (`.backanno`, `.end`).
    SourceOnly,
    /// Anything else, which stays on the sheet.
    Other,
}

/// One logical card of a directive block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignCard {
    /// Where the block holding the card was drawn.
    pub position: Point,
    /// How the block holding the card was lettered.
    pub lettering: ForeignLettering,
    pub role: ForeignCardRole,
    /// The card as written; a subcircuit is every line through its `.ends`.
    pub text: String,
    /// The card's words with continuation lines joined; for a subcircuit,
    /// those of its `.subckt` line.
    pub tokens: Vec<String>,
}

impl ForeignCard {
    /// The card's keyword, lower case (`.tran`).
    pub fn keyword(&self) -> String {
        self.tokens
            .first()
            .map(|word| word.to_ascii_lowercase())
            .unwrap_or_default()
    }
}

/// Every card of the drawing's directive blocks, in drawing order.
pub fn foreign_directive_cards(directives: &[ForeignDirective]) -> Vec<ForeignCard> {
    let mut cards = Vec::new();
    for directive in directives {
        let mut logical: Vec<(String, String)> = Vec::new();
        for line in directive.text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('*') || trimmed.starts_with(';') {
                continue;
            }
            match (trimmed.strip_prefix('+'), logical.last_mut()) {
                (Some(rest), Some((text, joined))) => {
                    text.push('\n');
                    text.push_str(line.trim_end());
                    joined.push(' ');
                    joined.push_str(rest);
                }
                _ => logical.push((line.trim_end().to_owned(), trimmed.to_owned())),
            }
        }
        let mut subcircuit: Option<(String, Vec<String>, usize)> = None;
        for (text, joined) in logical {
            let tokens = card_tokens(&joined);
            let keyword = tokens
                .first()
                .map(|word| word.to_ascii_lowercase())
                .unwrap_or_default();
            if let Some((body, _, depth)) = subcircuit.as_mut() {
                body.push('\n');
                body.push_str(&text);
                match keyword.as_str() {
                    ".subckt" => *depth += 1,
                    ".ends" => *depth -= 1,
                    _ => {}
                }
                if *depth == 0
                    && let Some((body, tokens, _)) = subcircuit.take()
                {
                    cards.push(ForeignCard {
                        position: directive.position,
                        lettering: directive.lettering,
                        role: ForeignCardRole::ModelText,
                        text: body,
                        tokens,
                    });
                }
                continue;
            }
            if keyword == ".subckt" {
                subcircuit = Some((text, tokens, 1));
                continue;
            }
            cards.push(ForeignCard {
                position: directive.position,
                lettering: directive.lettering,
                role: role(&keyword),
                text,
                tokens,
            });
        }
        // A subcircuit the block never closes is still the block's text.
        if let Some((body, tokens, _)) = subcircuit {
            cards.push(ForeignCard {
                position: directive.position,
                lettering: directive.lettering,
                role: ForeignCardRole::Other,
                text: body,
                tokens,
            });
        }
    }
    cards
}

fn role(keyword: &str) -> ForeignCardRole {
    match keyword {
        ".op" | ".tran" | ".ac" | ".dc" | ".noise" | ".tf" => ForeignCardRole::Analysis,
        ".param" | ".params" => ForeignCardRole::Parameter,
        ".option" | ".options" | ".opt" => ForeignCardRole::Options,
        ".temp" => ForeignCardRole::Temperature,
        ".step" => ForeignCardRole::Step,
        ".meas" | ".measure" => ForeignCardRole::Measurement,
        ".model" | ".func" => ForeignCardRole::ModelText,
        ".include" | ".inc" | ".lib" => ForeignCardRole::Include,
        ".backanno" | ".end" => ForeignCardRole::SourceOnly,
        _ => ForeignCardRole::Other,
    }
}

/// The model names a drawing's own model text defines, lower case.
pub fn defined_model_names(cards: &[ForeignCard]) -> Vec<String> {
    cards
        .iter()
        .filter(|card| card.role == ForeignCardRole::ModelText)
        .filter_map(|card| match card.keyword().as_str() {
            ".model" | ".subckt" => card.tokens.get(1).map(|name| name.to_ascii_lowercase()),
            _ => None,
        })
        .collect()
}

/// Writes directive text RSpice keeps on the sheet as a note where its block
/// was drawn, lettered as the block was. Like a comment, it is lettering
/// alone, with no anchor mark.
pub fn keep_directive_text(
    document: &mut SchematicDocument,
    position: Point,
    text: &str,
    lettering: ForeignLettering,
) -> Result<(), String> {
    let id = SchematicIdentity::with_cursor(1).allocate(document);
    let note = DesignNote::new(id, position, DesignNoteKind::PlainText, text)
        .map_err(|error| error.to_string())?;
    document
        .design_notes
        .push(lettered_note(note, position, &lettering));
    Ok(())
}
