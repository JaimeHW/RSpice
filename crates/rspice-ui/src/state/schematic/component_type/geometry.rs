use super::super::point::Point;
use super::ComponentType;

impl ComponentType {
    /// Get the number of terminals for this component type.
    pub fn terminal_count(&self) -> usize {
        match self {
            ComponentType::Ground => 1,
            ComponentType::Port => 1,
            ComponentType::CellInstance => 2,
            ComponentType::Transformer => 4,
            ComponentType::NpnBjt | ComponentType::PnpBjt => 3,
            ComponentType::Njfet | ComponentType::Pjfet => 3,
            ComponentType::OpAmp => 3,
            ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos => 4,
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => 4,
            ComponentType::VSwitch | ComponentType::TransmissionLine => 4,
            ComponentType::CoupledInductor => 0,
            ComponentType::XspiceSummer
            | ComponentType::XspiceMultiplier
            | ComponentType::XspiceDivider => 3,
            ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate
            | ComponentType::XspiceTristate => 3,
            ComponentType::XspiceDFlipFlop | ComponentType::XspiceSrLatch => 4,
            ComponentType::XspiceJkFlipFlop => 5,
            _ => 2,
        }
    }

    /// Get terminal offsets relative to component position.
    ///
    /// Static per type — the coordinates are the `symbol_dimensions`
    /// half-extents (the unit test below keeps the tables honest), and
    /// returning `&'static` keeps per-frame and hit-test paths
    /// allocation-free.
    pub fn terminal_offsets(&self) -> &'static [(&'static str, Point)] {
        match self {
            // (40, 20) / (40, 40): hw 20
            ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor => {
                &[("+", Point { x: -20, y: 0 }), ("-", Point { x: 20, y: 0 })]
            }
            // (60, 80): hw 30, hh 40, pins inset 10 from the sides
            ComponentType::Transformer => &[
                ("P1", Point { x: -20, y: -40 }),
                ("P2", Point { x: -20, y: 40 }),
                ("S1", Point { x: 20, y: -40 }),
                ("S2", Point { x: 20, y: 40 }),
            ],
            // (40, 20): hw 20
            ComponentType::Diode => &[("A", Point { x: -20, y: 0 }), ("K", Point { x: 20, y: 0 })],
            // (28, 40): hh 20, vertical at R0
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
            | ComponentType::CurrentSourceNoise
            | ComponentType::BehavioralSource => {
                &[("+", Point { x: 0, y: -20 }), ("-", Point { x: 0, y: 20 })]
            }
            // (40, 80): hw 20, hh 40
            ComponentType::NpnBjt => &[
                ("C", Point { x: 20, y: -40 }),
                ("B", Point { x: -20, y: 0 }),
                ("E", Point { x: 20, y: 40 }),
            ],
            ComponentType::PnpBjt => &[
                ("C", Point { x: 20, y: 40 }),
                ("B", Point { x: -20, y: 0 }),
                ("E", Point { x: 20, y: -40 }),
            ],
            ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos => &[
                ("D", Point { x: 20, y: -40 }),
                ("G", Point { x: -20, y: 0 }),
                ("S", Point { x: 20, y: 40 }),
                ("B", Point { x: 20, y: 0 }),
            ],
            ComponentType::Njfet | ComponentType::Pjfet => &[
                ("D", Point { x: 20, y: -40 }),
                ("G", Point { x: -20, y: 0 }),
                ("S", Point { x: 20, y: 40 }),
            ],
            // (40, 20): hw 20
            ComponentType::SaturableInductor => {
                &[("+", Point { x: -20, y: 0 }), ("-", Point { x: 20, y: 0 })]
            }
            // (40, 40): hw 20, hh/2 10
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => &[
                ("O+", Point { x: -20, y: -10 }),
                ("O-", Point { x: -20, y: 10 }),
                ("C+", Point { x: 20, y: -10 }),
                ("C-", Point { x: 20, y: 10 }),
            ],
            // (40, 40): hw 20, inputs at ±10, output centered
            ComponentType::OpAmp => &[
                ("in+", Point { x: -20, y: -10 }),
                ("in-", Point { x: -20, y: 10 }),
                ("out", Point { x: 20, y: 0 }),
            ],
            // (40, 40): hw 20, hh 20 — switch path left/right, control top/bottom
            ComponentType::VSwitch => &[
                ("1", Point { x: -20, y: 0 }),
                ("2", Point { x: 20, y: 0 }),
                ("c+", Point { x: 0, y: -20 }),
                ("c-", Point { x: 0, y: 20 }),
            ],
            // (60, 40): hw 30, hh/2 10 — port a left, port b right
            ComponentType::TransmissionLine => &[
                ("a+", Point { x: -30, y: -10 }),
                ("a-", Point { x: -30, y: 10 }),
                ("b+", Point { x: 30, y: -10 }),
                ("b-", Point { x: 30, y: 10 }),
            ],
            ComponentType::CoupledInductor => &[],
            // (60, 40): hw 30
            ComponentType::CellInstance => {
                &[("1", Point { x: -30, y: 0 }), ("2", Point { x: 30, y: 0 })]
            }
            // (20, 20): hh 10
            ComponentType::Ground => &[("GND", Point { x: 0, y: -10 })],
            // (20, 20): hw 10 — single attachment point on the left, the
            // flag body extends to the right of it
            ComponentType::Port => &[("P", Point { x: -10, y: 0 })],
            // (40, 20): hw 20
            ComponentType::XspiceGain
            | ComponentType::XspiceLimiter
            | ComponentType::XspiceIntegrator
            | ComponentType::XspiceDifferentiator
            | ComponentType::XspiceInverter
            | ComponentType::XspiceBuffer
            | ComponentType::XspiceAdcBridge
            | ComponentType::XspiceDacBridge => &[
                ("in", Point { x: -20, y: 0 }),
                ("out", Point { x: 20, y: 0 }),
            ],
            // (40, 20): hw 20, hh/2 5
            ComponentType::XspiceSummer
            | ComponentType::XspiceMultiplier
            | ComponentType::XspiceDivider => &[
                ("in1", Point { x: -20, y: -5 }),
                ("in2", Point { x: -20, y: 5 }),
                ("out", Point { x: 20, y: 0 }),
            ],
            ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate => &[
                ("a", Point { x: -20, y: -5 }),
                ("b", Point { x: -20, y: 5 }),
                ("out", Point { x: 20, y: 0 }),
            ],
            ComponentType::XspiceTristate => &[
                ("in", Point { x: -20, y: 0 }),
                ("en", Point { x: 0, y: -10 }),
                ("out", Point { x: 20, y: 0 }),
            ],
            // (40, 40): hw 20, hh/2 10
            ComponentType::XspiceDFlipFlop => &[
                ("d", Point { x: -20, y: -10 }),
                ("clk", Point { x: -20, y: 10 }),
                ("q", Point { x: 20, y: -10 }),
                ("qbar", Point { x: 20, y: 10 }),
            ],
            ComponentType::XspiceJkFlipFlop => &[
                ("j", Point { x: -20, y: -10 }),
                ("k", Point { x: -20, y: 10 }),
                ("clk", Point { x: -20, y: 0 }),
                ("q", Point { x: 20, y: -10 }),
                ("qbar", Point { x: 20, y: 10 }),
            ],
            ComponentType::XspiceSrLatch => &[
                ("s", Point { x: -20, y: -10 }),
                ("r", Point { x: -20, y: 10 }),
                ("q", Point { x: 20, y: -10 }),
                ("qbar", Point { x: 20, y: 10 }),
            ],
        }
    }

    /// Get symbol dimensions in grid units (width, height).
    pub const fn symbol_dimensions(&self) -> (i32, i32) {
        match self {
            ComponentType::Resistor | ComponentType::Inductor => (40, 20),
            ComponentType::Transformer => (60, 80),
            ComponentType::Capacitor => (40, 40),
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
            | ComponentType::CurrentSourceNoise
            | ComponentType::BehavioralSource => (28, 40),
            ComponentType::Ground => (20, 20),
            ComponentType::Port => (20, 20),
            ComponentType::CellInstance => (60, 40),
            ComponentType::Diode => (40, 20),
            ComponentType::NpnBjt | ComponentType::PnpBjt => (40, 80),
            ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::Njfet
            | ComponentType::Pjfet
            | ComponentType::NVdmos
            | ComponentType::PVdmos => (40, 80),
            ComponentType::SaturableInductor => (40, 20),
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => (40, 40),
            ComponentType::OpAmp => (40, 40),
            ComponentType::VSwitch => (40, 40),
            ComponentType::TransmissionLine => (60, 40),
            ComponentType::CoupledInductor => (0, 0),
            ComponentType::XspiceGain
            | ComponentType::XspiceSummer
            | ComponentType::XspiceMultiplier
            | ComponentType::XspiceDivider
            | ComponentType::XspiceLimiter
            | ComponentType::XspiceIntegrator
            | ComponentType::XspiceDifferentiator => (40, 20),
            ComponentType::XspiceInverter
            | ComponentType::XspiceBuffer
            | ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate
            | ComponentType::XspiceTristate => (40, 20),
            ComponentType::XspiceDFlipFlop
            | ComponentType::XspiceJkFlipFlop
            | ComponentType::XspiceSrLatch => (40, 40),
            ComponentType::XspiceAdcBridge | ComponentType::XspiceDacBridge => (40, 20),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_TYPES: &[ComponentType] = &[
        ComponentType::Resistor,
        ComponentType::Capacitor,
        ComponentType::Inductor,
        ComponentType::Transformer,
        ComponentType::SaturableInductor,
        ComponentType::Diode,
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
        ComponentType::NpnBjt,
        ComponentType::PnpBjt,
        ComponentType::Nmos,
        ComponentType::Pmos,
        ComponentType::Njfet,
        ComponentType::Pjfet,
        ComponentType::NVdmos,
        ComponentType::PVdmos,
        ComponentType::Vcvs,
        ComponentType::Vccs,
        ComponentType::Ccvs,
        ComponentType::Cccs,
        ComponentType::OpAmp,
        ComponentType::BehavioralSource,
        ComponentType::VSwitch,
        ComponentType::TransmissionLine,
        ComponentType::CoupledInductor,
        ComponentType::CellInstance,
        ComponentType::Ground,
        ComponentType::Port,
        ComponentType::XspiceGain,
        ComponentType::XspiceLimiter,
        ComponentType::XspiceIntegrator,
        ComponentType::XspiceDifferentiator,
        ComponentType::XspiceInverter,
        ComponentType::XspiceBuffer,
        ComponentType::XspiceAndGate,
        ComponentType::XspiceOrGate,
        ComponentType::XspiceNandGate,
        ComponentType::XspiceNorGate,
        ComponentType::XspiceXorGate,
        ComponentType::XspiceTristate,
        ComponentType::XspiceSummer,
        ComponentType::XspiceMultiplier,
        ComponentType::XspiceDivider,
        ComponentType::XspiceDFlipFlop,
        ComponentType::XspiceJkFlipFlop,
        ComponentType::XspiceSrLatch,
        ComponentType::XspiceAdcBridge,
        ComponentType::XspiceDacBridge,
    ];

    /// The static terminal tables must stay consistent with the declared
    /// symbol dimensions and terminal counts.
    #[test]
    fn terminal_tables_match_symbol_dimensions() {
        for kind in ALL_TYPES {
            let offsets = kind.terminal_offsets();
            assert_eq!(
                offsets.len(),
                kind.terminal_count(),
                "{kind:?}: table length vs terminal_count"
            );

            let (w, h) = kind.symbol_dimensions();
            let (hw, hh) = (w / 2, h / 2);
            for (name, offset) in offsets {
                assert!(
                    offset.x.abs() <= hw && offset.y.abs() <= hh,
                    "{kind:?} terminal {name}: offset {offset:?} outside (+/-{hw}, +/-{hh})"
                );
                // Terminals sit on the symbol boundary, never inside corners.
                assert!(
                    offset.x.abs() == hw || offset.y.abs() == hh || offset.x.abs() == hw - 10,
                    "{kind:?} terminal {name}: offset {offset:?} floats inside the symbol"
                );
            }
        }
    }
}
