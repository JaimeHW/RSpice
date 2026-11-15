//! The symbol's pins as the rows of a table: a column for each field the
//! inspector edits on one pin, read and written by the inspector's own
//! rules, so a value the inspector refuses the table refuses too.
//!
//! A table edit sets one column of every pin it is given, which is how a
//! multi-row edit works: select rows, edit a cell, every selected row takes
//! the value. Each edit is one undo step, named for what it did.

use std::collections::BTreeMap;

use rspice_design::symbol::{SymbolDocument, SymbolPinElectricalKind};
use rspice_design_model::symbol_pin::{SymbolPinSide, SymbolPinStyle};
use rspice_ui_kit::accessibility::counted;

use super::inspector::{lead_length_accepted, pin_name_accepted};

/// The pin table's identity among the engineering tables, which its saved
/// views are kept under.
pub const PIN_TABLE_GRID_ID: &str = "symbol-pins";

/// The Show name column's value for a pin whose name is lettered.
pub const NAME_SHOWN: &str = "shown";
/// The Show name column's value for a pin whose name is not lettered.
pub const NAME_HIDDEN: &str = "hidden";

/// One column of the pin table: one field of a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinTableColumn {
    Name,
    ElectricalType,
    X,
    Y,
    Side,
    LeadLength,
    Style,
    ShowName,
    NetlistOrder,
}

impl PinTableColumn {
    /// Every column, in the inspector's row order.
    pub const ALL: [Self; 9] = [
        Self::Name,
        Self::ElectricalType,
        Self::X,
        Self::Y,
        Self::Side,
        Self::LeadLength,
        Self::Style,
        Self::ShowName,
        Self::NetlistOrder,
    ];

    /// The column's identity in a table view.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::ElectricalType => "electrical-type",
            Self::X => "x",
            Self::Y => "y",
            Self::Side => "side",
            Self::LeadLength => "lead-length",
            Self::Style => "style",
            Self::ShowName => "show-name",
            Self::NetlistOrder => "netlist-order",
        }
    }

    /// The column's heading: the inspector's row label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Name => "Name",
            Self::ElectricalType => "Electrical type",
            Self::X => "X",
            Self::Y => "Y",
            Self::Side => "Side",
            Self::LeadLength => "Lead length",
            Self::Style => "Style",
            Self::ShowName => "Show name",
            Self::NetlistOrder => "Netlist order",
        }
    }

    /// What an undo step calls the field it set.
    const fn noun(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::ElectricalType => "electrical type",
            Self::X => "X",
            Self::Y => "Y",
            Self::Side => "side",
            Self::LeadLength => "lead length",
            Self::Style => "style",
            Self::ShowName => "name visibility",
            Self::NetlistOrder => "netlist order",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|column| column.id() == id)
    }

    /// Whether the column holds numbers, which sort and filter as numbers.
    pub const fn numeric(self) -> bool {
        matches!(
            self,
            Self::X | Self::Y | Self::LeadLength | Self::NetlistOrder
        )
    }

    /// What the column's numbers are counted in, where they are lengths.
    pub const fn unit(self) -> Option<&'static str> {
        match self {
            Self::X | Self::Y | Self::LeadLength => Some("grid"),
            _ => None,
        }
    }

    /// The values a cell of this column takes, where it is one of a few.
    pub fn choices(self) -> Option<Vec<&'static str>> {
        match self {
            Self::ElectricalType => Some(
                SymbolPinElectricalKind::ALL
                    .map(SymbolPinElectricalKind::label)
                    .to_vec(),
            ),
            Self::Side => Some(SymbolPinSide::ALL.map(SymbolPinSide::label).to_vec()),
            Self::Style => Some(SymbolPinStyle::ALL.map(SymbolPinStyle::label).to_vec()),
            Self::ShowName => Some(vec![NAME_SHOWN, NAME_HIDDEN]),
            Self::Name | Self::X | Self::Y | Self::LeadLength | Self::NetlistOrder => None,
        }
    }

    /// The column's value for the pin at netlist place `index`, as the
    /// table shows it. An unplaced pin has no X or Y.
    pub fn value(self, document: &SymbolDocument, index: usize) -> String {
        let pin = &document.pins[index];
        match self {
            Self::Name => pin.name.clone(),
            Self::ElectricalType => SymbolPinElectricalKind::from_pin(pin).label().to_owned(),
            Self::X => pin
                .position
                .map(|tip| tip.x.to_string())
                .unwrap_or_default(),
            Self::Y => pin
                .position
                .map(|tip| tip.y.to_string())
                .unwrap_or_default(),
            Self::Side => pin.side.label().to_owned(),
            Self::LeadLength => pin.length.to_string(),
            Self::Style => pin.style.label().to_owned(),
            Self::ShowName => if pin.name_hidden {
                NAME_HIDDEN
            } else {
                NAME_SHOWN
            }
            .to_owned(),
            Self::NetlistOrder => (index + 1).to_string(),
        }
    }

    /// The column's value for the pin at `index` as a number, where it is
    /// one.
    pub fn number(self, document: &SymbolDocument, index: usize) -> Option<f64> {
        let pin = &document.pins[index];
        match self {
            Self::X => pin.position.map(|tip| f64::from(tip.x)),
            Self::Y => pin.position.map(|tip| f64::from(tip.y)),
            Self::LeadLength => Some(f64::from(pin.length)),
            Self::NetlistOrder => Some((index + 1) as f64),
            _ => None,
        }
    }
}

/// One edit the table made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinTableEdit {
    /// The undo step's name: "Set side of 4 pins".
    pub step: String,
    /// Each renamed pin's old name and its new one, so every placed
    /// instance's terminal follows the pin.
    pub renames: BTreeMap<String, String>,
    /// The edited pins' names after the edit, in netlist order: the
    /// selection the table and the canvas keep.
    pub pins: Vec<String>,
}

/// Set `column` of every pin `pins` names to `text`.
///
/// Every pin takes the one value, so a multi-row edit is this called with
/// the selected rows. Returns `Ok(None)` when every pin already holds the
/// value, and the reason when the value cannot be taken; the document is
/// unchanged then.
pub fn set_pin_cells(
    document: &mut SymbolDocument,
    pins: &[String],
    column: PinTableColumn,
    text: &str,
) -> Result<Option<PinTableEdit>, String> {
    let indices = pin_indices(document, pins);
    if indices.is_empty() {
        return Ok(None);
    }
    let text = text.trim();
    let before = document.clone();
    let subject = subject(document, &indices);
    let mut renames = BTreeMap::new();
    let step = match column {
        PinTableColumn::Name => {
            let [order] = indices[..] else {
                return Err("A name names one pin: select one row to rename its pin.".to_owned());
            };
            if !pin_name_accepted(document, order, text) {
                return Err(format!(
                    "\"{text}\" cannot name this pin: a pin name is unique, not empty and \
                     has no control characters."
                ));
            }
            let old = std::mem::replace(&mut document.pins[order].name, text.to_owned());
            if old == text {
                return Ok(None);
            }
            renames.insert(old.clone(), text.to_owned());
            format!("Rename pin {old} to {text}")
        }
        PinTableColumn::ElectricalType => {
            let kind = choice(column, text, &SymbolPinElectricalKind::ALL, |kind| {
                kind.label()
            })?;
            let (electrical_type, direction) = kind.contract();
            for &index in &indices {
                if SymbolPinElectricalKind::from_pin(&document.pins[index]) != kind {
                    document.pins[index].set_electrical_contract(electrical_type, direction);
                }
            }
            format!("Set {} of {subject}", column.noun())
        }
        PinTableColumn::X | PinTableColumn::Y => {
            let value = integer(column, text)?;
            for &index in &indices {
                let pin = &mut document.pins[index];
                let Some(tip) = pin.position.as_mut() else {
                    let name = pin.name.clone();
                    *document = before;
                    return Err(format!(
                        "Pin {name} is not placed, so it has no {}.",
                        column.label()
                    ));
                };
                if column == PinTableColumn::X {
                    tip.x = value;
                } else {
                    tip.y = value;
                }
            }
            format!("Set {} of {subject}", column.noun())
        }
        PinTableColumn::Side => {
            let side = choice(column, text, &SymbolPinSide::ALL, |side| side.label())?;
            for &index in &indices {
                document.pins[index].side = side;
            }
            format!("Set {} of {subject}", column.noun())
        }
        PinTableColumn::LeadLength => {
            let length = integer(column, text)?;
            if !lead_length_accepted(length) {
                return Err("Lead length is 0, a bare connection point, or longer.".to_owned());
            }
            for &index in &indices {
                document.pins[index].length = length;
            }
            format!("Set {} of {subject}", column.noun())
        }
        PinTableColumn::Style => {
            let style = choice(column, text, &SymbolPinStyle::ALL, |style| style.label())?;
            for &index in &indices {
                document.pins[index].style = style;
            }
            format!("Set {} of {subject}", column.noun())
        }
        PinTableColumn::ShowName => {
            let hidden = choice(column, text, &[false, true], |hidden| {
                if *hidden { NAME_HIDDEN } else { NAME_SHOWN }
            })?;
            for &index in &indices {
                document.pins[index].name_hidden = hidden;
            }
            if hidden {
                format!("Hide the name of {subject}")
            } else {
                format!("Show the name of {subject}")
            }
        }
        PinTableColumn::NetlistOrder => {
            let total = document.pins.len();
            let place = text
                .parse::<usize>()
                .ok()
                .filter(|place| (1..=total).contains(place))
                .ok_or_else(|| format!("Netlist order is a place from 1 to {total}."))?;
            let moving = indices
                .iter()
                .rev()
                .map(|&index| document.pins.remove(index))
                .collect::<Vec<_>>();
            let at = (place - 1).min(document.pins.len());
            document.pins.splice(at..at, moving.into_iter().rev());
            format!("Move {subject} to netlist place {place}")
        }
    };
    if *document == before {
        return Ok(None);
    }
    Ok(Some(PinTableEdit {
        step,
        renames: renames.clone(),
        pins: edited_names(document, pins, &renames),
    }))
}

/// Move every pin `pins` names one netlist place earlier, or later: the
/// table's rows reordered. A pin already first, or last, stays, and the
/// selected pins keep their order among themselves.
pub fn move_pins(
    document: &mut SymbolDocument,
    pins: &[String],
    later: bool,
) -> Option<PinTableEdit> {
    let indices = pin_indices(document, pins);
    if indices.is_empty() {
        return None;
    }
    let subject = subject(document, &indices);
    let mut selected = vec![false; document.pins.len()];
    for &index in &indices {
        selected[index] = true;
    }
    let mut moved = false;
    let mut swap = |document: &mut SymbolDocument, index: usize, other: usize| {
        if selected[index] && !selected[other] {
            document.pins.swap(index, other);
            selected.swap(index, other);
            moved = true;
        }
    };
    if later {
        for index in (0..document.pins.len().saturating_sub(1)).rev() {
            swap(document, index, index + 1);
        }
    } else {
        for index in 1..document.pins.len() {
            swap(document, index, index - 1);
        }
    }
    moved.then(|| PinTableEdit {
        step: format!(
            "Move {subject} {} in netlist order",
            if later { "later" } else { "earlier" }
        ),
        renames: BTreeMap::new(),
        pins: edited_names(document, pins, &BTreeMap::new()),
    })
}

/// The netlist places of the pins `pins` names, in netlist order.
fn pin_indices(document: &SymbolDocument, pins: &[String]) -> Vec<usize> {
    document
        .pins
        .iter()
        .enumerate()
        .filter(|(_, pin)| pins.iter().any(|name| name.eq_ignore_ascii_case(&pin.name)))
        .map(|(index, _)| index)
        .collect()
}

/// What an undo step calls the pins it edited: "pin IN", or "4 pins".
fn subject(document: &SymbolDocument, indices: &[usize]) -> String {
    match indices {
        [index] => format!("pin {}", document.pins[*index].name),
        _ => counted(indices.len(), "pin", "pins"),
    }
}

/// The edited pins' names after an edit that applied `renames`, in netlist
/// order.
fn edited_names(
    document: &SymbolDocument,
    pins: &[String],
    renames: &BTreeMap<String, String>,
) -> Vec<String> {
    let wanted = pins
        .iter()
        .map(|name| renames.get(name).unwrap_or(name))
        .collect::<Vec<_>>();
    document
        .pins
        .iter()
        .filter(|pin| {
            wanted
                .iter()
                .any(|name| name.eq_ignore_ascii_case(&pin.name))
        })
        .map(|pin| pin.name.clone())
        .collect()
}

fn integer(column: PinTableColumn, text: &str) -> Result<i32, String> {
    text.parse::<i32>()
        .map_err(|_| format!("{} is a whole number of grid units.", column.label()))
}

/// The one of `options` whose label is `text`, ignoring case.
fn choice<T: Copy>(
    column: PinTableColumn,
    text: &str,
    options: &[T],
    label: impl Fn(&T) -> &'static str,
) -> Result<T, String> {
    options
        .iter()
        .find(|option| label(option).eq_ignore_ascii_case(text))
        .copied()
        .ok_or_else(|| {
            format!(
                "{} is one of: {}.",
                column.label(),
                options.iter().map(&label).collect::<Vec<_>>().join(", ")
            )
        })
}

#[cfg(test)]
mod tests;
