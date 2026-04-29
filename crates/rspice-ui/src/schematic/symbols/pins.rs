use crate::state::ComponentType;

use super::types::{PinDirection, Symbol, SymbolPin};

/// Add default pin positions based on component type
pub(super) fn add_default_pins(symbol: &mut Symbol, component_type: ComponentType) {
    let (cx, cy) = symbol.center();
    let w = symbol.width() / 2.0;
    let h = symbol.height() / 2.0;

    match component_type {
        ComponentType::Resistor
        | ComponentType::Capacitor
        | ComponentType::Inductor
        | ComponentType::SaturableInductor => {
            // Two-terminal horizontal component
            symbol.pins = vec![
                SymbolPin {
                    name: "1".to_string(),
                    position: (cx - w, cy),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "2".to_string(),
                    position: (cx + w, cy),
                    direction: PinDirection::Right,
                },
            ];
        }
        ComponentType::Transformer => {
            let pin_inset = 10.0;
            symbol.pins = vec![
                SymbolPin {
                    name: "P1".to_string(),
                    position: (cx - w + pin_inset, cy - h),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "P2".to_string(),
                    position: (cx - w + pin_inset, cy + h),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "S1".to_string(),
                    position: (cx + w - pin_inset, cy - h),
                    direction: PinDirection::Right,
                },
                SymbolPin {
                    name: "S2".to_string(),
                    position: (cx + w - pin_inset, cy + h),
                    direction: PinDirection::Right,
                },
            ];
        }
        ComponentType::VoltageSource
        | ComponentType::VoltageSourceAc
        | ComponentType::VoltageSourcePulse
        | ComponentType::VoltageSourceSin
        | ComponentType::VoltageSourcePwl
        | ComponentType::VoltageSourceExp
        | ComponentType::VoltageSourceSffm
        | ComponentType::CurrentSource
        | ComponentType::CurrentSourceAc
        | ComponentType::CurrentSourcePulse
        | ComponentType::CurrentSourceSin
        | ComponentType::CurrentSourcePwl
        | ComponentType::CurrentSourceExp
        | ComponentType::CurrentSourceNoise => {
            // Vertical source
            symbol.pins = vec![
                SymbolPin {
                    name: "+".to_string(),
                    position: (cx, cy - h),
                    direction: PinDirection::Up,
                },
                SymbolPin {
                    name: "-".to_string(),
                    position: (cx, cy + h),
                    direction: PinDirection::Down,
                },
            ];
        }
        ComponentType::Ground => {
            symbol.pins = vec![SymbolPin {
                name: "0".to_string(),
                position: (cx, cy - h),
                direction: PinDirection::Up,
            }];
        }
        ComponentType::Diode => {
            symbol.pins = vec![
                SymbolPin {
                    name: "A".to_string(),
                    position: (cx - w, cy),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "K".to_string(),
                    position: (cx + w, cy),
                    direction: PinDirection::Right,
                },
            ];
        }
        ComponentType::Nmos
        | ComponentType::Pmos
        | ComponentType::NVdmos
        | ComponentType::PVdmos => {
            symbol.pins = vec![
                SymbolPin {
                    name: "G".to_string(),
                    position: (cx - w, cy),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "D".to_string(),
                    position: (cx, cy - h),
                    direction: PinDirection::Up,
                },
                SymbolPin {
                    name: "S".to_string(),
                    position: (cx, cy + h),
                    direction: PinDirection::Down,
                },
            ];
        }
        ComponentType::Njfet | ComponentType::Pjfet => {
            symbol.pins = vec![
                SymbolPin {
                    name: "G".to_string(),
                    position: (cx - w, cy),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "D".to_string(),
                    position: (cx, cy - h),
                    direction: PinDirection::Up,
                },
                SymbolPin {
                    name: "S".to_string(),
                    position: (cx, cy + h),
                    direction: PinDirection::Down,
                },
            ];
        }
        ComponentType::NpnBjt | ComponentType::PnpBjt => {
            symbol.pins = vec![
                SymbolPin {
                    name: "B".to_string(),
                    position: (cx - w, cy),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "C".to_string(),
                    position: (cx, cy - h),
                    direction: PinDirection::Up,
                },
                SymbolPin {
                    name: "E".to_string(),
                    position: (cx, cy + h),
                    direction: PinDirection::Down,
                },
            ];
        }
        _ => {}
    }
}
