//! Create Pins fields over an editor draft and resolved display text.

use egui::Id;
use rspice_design::schematic::port::{PortDiscipline, PortSignalType};
use rspice_design_model::port::PortDirection;
use rspice_ui_kit::widgets::FormRows;

/// Editable pin names and interface choices; no document or modal authority.
#[derive(Debug, Clone)]
pub struct PinPlacementFields {
    pub names: String,
    pub direction: PortDirection,
    pub signal_type: PortSignalType,
    pub discipline: PortDiscipline,
    /// An explicit discipline choice stops automatic signal-type following.
    pub discipline_touched: bool,
}

impl Default for PinPlacementFields {
    fn default() -> Self {
        Self {
            names: String::new(),
            direction: PortDirection::In,
            signal_type: PortSignalType::Analog,
            discipline: PortDiscipline::Electrical,
            discipline_touched: false,
        }
    }
}

impl PinPlacementFields {
    pub fn set_direction(&mut self, direction: PortDirection) {
        self.direction = direction;
        coerce_from_direction(self.direction, &mut self.signal_type);
        if !self.discipline_touched {
            self.discipline = discipline_for(self.signal_type);
        }
    }

    pub fn set_signal_type(&mut self, signal_type: PortSignalType) {
        self.signal_type = signal_type;
        coerce_from_signal(self.signal_type, &mut self.direction);
        if !self.discipline_touched {
            self.discipline = discipline_for(self.signal_type);
        }
    }

    pub fn set_discipline(&mut self, discipline: PortDiscipline) {
        self.discipline = discipline;
        self.discipline_touched = true;
    }

    pub fn show(
        &mut self,
        rows: &mut FormRows<'_>,
        declaration: Option<&str>,
        bits: Option<&str>,
    ) -> Option<Id> {
        let disciplines = PortDiscipline::ALL.map(|entry| entry.keyword().to_owned());

        let names = rows.text(NAMES_LABEL, names_field_id(), &mut self.names, NAMES_HINT);
        if let Some(declaration) = declaration {
            rows.derived(declaration);
        }
        if let Some(bits) = bits {
            rows.derived(bits);
        }
        let mut direction = DIRECTIONS
            .iter()
            .position(|entry| *entry == self.direction)
            .unwrap_or(0);
        if rows.segmented(
            DIRECTION_LABEL,
            "rspice.create-pins.direction",
            &DIRECTION_SEGMENTS,
            &mut direction,
        ) {
            self.set_direction(DIRECTIONS[direction]);
        }
        let mut signal = SIGNALS
            .iter()
            .position(|entry| *entry == self.signal_type)
            .unwrap_or(0);
        if rows.segmented(
            SIGNAL_LABEL,
            "rspice.create-pins.signal",
            &SIGNAL_SEGMENTS,
            &mut signal,
        ) {
            self.set_signal_type(SIGNALS[signal]);
        }
        if let Some(picked) = rows.select(
            DISCIPLINE_LABEL,
            "rspice.create-pins.discipline",
            self.discipline.keyword(),
            &disciplines,
        ) {
            self.set_discipline(PortDiscipline::ALL[picked]);
        }
        Some(names.id)
    }
}

const NAMES_LABEL: &str = "Names";
const NAMES_HINT: &str = "IN OUT VDD  or  DATA[7:0]";
const DIRECTION_LABEL: &str = "Direction";
const SIGNAL_LABEL: &str = "Signal";
const DISCIPLINE_LABEL: &str = "Discipline";
/// Directions in the order the segmented control offers them.
const DIRECTIONS: [PortDirection; 4] = [
    PortDirection::In,
    PortDirection::Out,
    PortDirection::InOut,
    PortDirection::Supply,
];
const DIRECTION_SEGMENTS: [&str; 4] = ["Input", "Output", "Inout", "Supply"];

/// Signal types in the order the segmented control offers them.
const SIGNALS: [PortSignalType; 3] = [
    PortSignalType::Analog,
    PortSignalType::Logic,
    PortSignalType::Power,
];
const SIGNAL_SEGMENTS: [&str; 3] = ["Analog", "Logic", "Power"];

fn names_field_id() -> egui::Id {
    egui::Id::new("rspice.create-pins.names")
}

/// Keep direction and signal type coherent by moving the *other* field.
///
/// The matrix the model accepts pairs power with inout and supply only, so a
/// reader who picks Supply has already said Power, and one who picks Power has
/// said the pin is a rail. Refusing either would be telling the reader that the
/// thing they just asked for is not allowed, when what they meant is plain.
fn coerce_from_direction(direction: PortDirection, signal: &mut PortSignalType) {
    match direction {
        PortDirection::Supply => *signal = PortSignalType::Power,
        // Leaving Supply for a one-way direction leaves Power behind with it:
        // only a bidirectional pin can carry power without being a rail.
        PortDirection::In | PortDirection::Out if *signal == PortSignalType::Power => {
            *signal = PortSignalType::Analog;
        }
        _ => {}
    }
}

fn coerce_from_signal(signal: PortSignalType, direction: &mut PortDirection) {
    match signal {
        PortSignalType::Power if *direction != PortDirection::Supply => {
            *direction = PortDirection::InOut;
        }
        PortSignalType::Analog | PortSignalType::Logic if *direction == PortDirection::Supply => {
            *direction = PortDirection::InOut;
        }
        _ => {}
    }
}

/// The discipline a signal type implies, until the reader picks one.
fn discipline_for(signal: PortSignalType) -> PortDiscipline {
    match signal {
        PortSignalType::Logic => PortDiscipline::Logic,
        PortSignalType::Analog | PortSignalType::Power => PortDiscipline::Electrical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_choices_keep_direction_signal_and_explicit_discipline_consistent() {
        let mut fields = PinPlacementFields::default();
        fields.set_direction(PortDirection::Supply);
        assert_eq!(fields.signal_type, PortSignalType::Power);
        assert_eq!(fields.discipline, PortDiscipline::Electrical);
        fields.set_direction(PortDirection::InOut);
        assert_eq!(fields.signal_type, PortSignalType::Power);
        fields.set_direction(PortDirection::In);
        assert_eq!(fields.signal_type, PortSignalType::Analog);
        fields.set_signal_type(PortSignalType::Power);
        assert_eq!(fields.direction, PortDirection::InOut);
        fields.set_direction(PortDirection::Supply);
        fields.set_signal_type(PortSignalType::Logic);
        assert_eq!(fields.direction, PortDirection::InOut);
        assert_eq!(fields.discipline, PortDiscipline::Logic);
        assert!(!fields.discipline_touched);
        fields.set_discipline(PortDiscipline::Wreal);
        fields.set_signal_type(PortSignalType::Analog);
        assert_eq!(fields.discipline, PortDiscipline::Wreal);
        fields.set_direction(PortDirection::Supply);
        assert_eq!(fields.signal_type, PortSignalType::Power);
        assert_eq!(fields.discipline, PortDiscipline::Wreal);
        assert!(fields.discipline_touched);
    }
}
