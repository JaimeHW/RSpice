use crate::state::ComponentType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentPaletteEntry {
    pub kind: ComponentType,
    pub label: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentPaletteSection {
    pub title: &'static str,
    pub entries: &'static [ComponentPaletteEntry],
}

const PASSIVES: &[ComponentPaletteEntry] = &[
    ComponentPaletteEntry {
        kind: ComponentType::Resistor,
        label: "Resistor",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Capacitor,
        label: "Capacitor",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Inductor,
        label: "Inductor",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Transformer,
        label: "Transformer",
    },
    ComponentPaletteEntry {
        kind: ComponentType::SaturableInductor,
        label: "Saturable Inductor",
    },
];

const SOURCES: &[ComponentPaletteEntry] = &[
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSource,
        label: "Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourceAc,
        label: "AC Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourcePulse,
        label: "Pulse Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourceSin,
        label: "Sinusoidal Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourcePwl,
        label: "PWL Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourceExp,
        label: "Exponential Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourceSffm,
        label: "SFFM Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSource,
        label: "Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourceAc,
        label: "AC Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourcePulse,
        label: "Pulse Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourceSin,
        label: "Sinusoidal Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourcePwl,
        label: "PWL Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourceExp,
        label: "Exponential Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourceNoise,
        label: "Noise Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Ground,
        label: "Ground",
    },
];

const SEMICONDUCTORS: &[ComponentPaletteEntry] = &[
    ComponentPaletteEntry {
        kind: ComponentType::Diode,
        label: "Diode",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Nmos,
        label: "NMOS",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Pmos,
        label: "PMOS",
    },
    ComponentPaletteEntry {
        kind: ComponentType::NpnBjt,
        label: "NPN BJT",
    },
    ComponentPaletteEntry {
        kind: ComponentType::PnpBjt,
        label: "PNP BJT",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Njfet,
        label: "N-JFET",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Pjfet,
        label: "P-JFET",
    },
    ComponentPaletteEntry {
        kind: ComponentType::NVdmos,
        label: "N-VDMOS",
    },
    ComponentPaletteEntry {
        kind: ComponentType::PVdmos,
        label: "P-VDMOS",
    },
];

const PALETTE: &[ComponentPaletteSection] = &[
    ComponentPaletteSection {
        title: "Passives",
        entries: PASSIVES,
    },
    ComponentPaletteSection {
        title: "Sources",
        entries: SOURCES,
    },
    ComponentPaletteSection {
        title: "Semiconductors",
        entries: SEMICONDUCTORS,
    },
];

pub fn component_palette() -> &'static [ComponentPaletteSection] {
    PALETTE
}

#[cfg(test)]
fn palette_contains(kind: ComponentType) -> bool {
    PALETTE
        .iter()
        .flat_map(|section| section.entries.iter())
        .any(|entry| entry.kind == kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_palette_includes_transformer() {
        assert!(palette_contains(ComponentType::Transformer));
    }

    #[test]
    fn test_palette_excludes_metadata_only_coupling_primitive() {
        assert!(!palette_contains(ComponentType::CoupledInductor));
    }

    #[test]
    fn test_palette_covers_supported_symbol_backed_devices() {
        let actual = component_palette()
            .iter()
            .flat_map(|section| section.entries.iter().map(|entry| format!("{:?}", entry.kind)))
            .collect::<BTreeSet<_>>();

        let expected = [
            ComponentType::Resistor,
            ComponentType::Capacitor,
            ComponentType::Inductor,
            ComponentType::Transformer,
            ComponentType::SaturableInductor,
            ComponentType::VoltageSource,
            ComponentType::VoltageSourceAc,
            ComponentType::VoltageSourcePulse,
            ComponentType::VoltageSourceSin,
            ComponentType::VoltageSourcePwl,
            ComponentType::VoltageSourceExp,
            ComponentType::VoltageSourceSffm,
            ComponentType::CurrentSource,
            ComponentType::CurrentSourceAc,
            ComponentType::CurrentSourcePulse,
            ComponentType::CurrentSourceSin,
            ComponentType::CurrentSourcePwl,
            ComponentType::CurrentSourceExp,
            ComponentType::CurrentSourceNoise,
            ComponentType::Ground,
            ComponentType::Diode,
            ComponentType::Nmos,
            ComponentType::Pmos,
            ComponentType::NpnBjt,
            ComponentType::PnpBjt,
            ComponentType::Njfet,
            ComponentType::Pjfet,
            ComponentType::NVdmos,
            ComponentType::PVdmos,
        ]
        .into_iter()
        .map(|kind| format!("{:?}", kind))
        .collect::<BTreeSet<_>>();

        assert_eq!(actual, expected);
    }
}
