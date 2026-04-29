use super::super::point::Point;
use super::ComponentType;

impl ComponentType {
    /// Get the number of terminals for this component type.
    pub fn terminal_count(&self) -> usize {
        match self {
            ComponentType::Ground => 1,
            ComponentType::CellInstance => 2,
            ComponentType::Transformer => 4,
            ComponentType::NpnBjt | ComponentType::PnpBjt => 3,
            ComponentType::Njfet | ComponentType::Pjfet => 3,
            ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos => 4,
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => 4,
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
    pub fn terminal_offsets(&self) -> Vec<(&'static str, Point)> {
        let (w, h) = self.symbol_dimensions();
        let hw = w / 2;
        let hh = h / 2;

        match self {
            ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor => {
                vec![("+", Point::new(-hw, 0)), ("-", Point::new(hw, 0))]
            }
            ComponentType::Transformer => {
                let pin_inset = 10;
                vec![
                    ("P1", Point::new(-hw + pin_inset, -hh)),
                    ("P2", Point::new(-hw + pin_inset, hh)),
                    ("S1", Point::new(hw - pin_inset, -hh)),
                    ("S2", Point::new(hw - pin_inset, hh)),
                ]
            }
            ComponentType::Diode => vec![("A", Point::new(-hw, 0)), ("K", Point::new(hw, 0))],
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
                vec![("+", Point::new(0, -hh)), ("-", Point::new(0, hh))]
            }
            ComponentType::NpnBjt => vec![
                ("C", Point::new(hw, -hh)),
                ("B", Point::new(-hw, 0)),
                ("E", Point::new(hw, hh)),
            ],
            ComponentType::PnpBjt => vec![
                ("C", Point::new(hw, hh)),
                ("B", Point::new(-hw, 0)),
                ("E", Point::new(hw, -hh)),
            ],
            ComponentType::Nmos | ComponentType::Pmos => vec![
                ("D", Point::new(hw, -hh)),
                ("G", Point::new(-hw, 0)),
                ("S", Point::new(hw, hh)),
                ("B", Point::new(hw, 0)),
            ],
            ComponentType::Njfet | ComponentType::Pjfet => vec![
                ("D", Point::new(hw, -hh)),
                ("G", Point::new(-hw, 0)),
                ("S", Point::new(hw, hh)),
            ],
            ComponentType::NVdmos | ComponentType::PVdmos => vec![
                ("D", Point::new(hw, -hh)),
                ("G", Point::new(-hw, 0)),
                ("S", Point::new(hw, hh)),
                ("B", Point::new(hw, 0)),
            ],
            ComponentType::SaturableInductor => {
                vec![("+", Point::new(-hw, 0)), ("-", Point::new(hw, 0))]
            }
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => vec![
                ("O+", Point::new(-hw, -hh / 2)),
                ("O-", Point::new(-hw, hh / 2)),
                ("C+", Point::new(hw, -hh / 2)),
                ("C-", Point::new(hw, hh / 2)),
            ],
            ComponentType::CoupledInductor => vec![],
            ComponentType::CellInstance => {
                vec![("1", Point::new(-hw, 0)), ("2", Point::new(hw, 0))]
            }
            ComponentType::Ground => vec![("GND", Point::new(0, -hh))],
            ComponentType::XspiceGain
            | ComponentType::XspiceLimiter
            | ComponentType::XspiceIntegrator
            | ComponentType::XspiceDifferentiator => {
                vec![("in", Point::new(-hw, 0)), ("out", Point::new(hw, 0))]
            }
            ComponentType::XspiceSummer => vec![
                ("in1", Point::new(-hw, -hh / 2)),
                ("in2", Point::new(-hw, hh / 2)),
                ("out", Point::new(hw, 0)),
            ],
            ComponentType::XspiceMultiplier | ComponentType::XspiceDivider => vec![
                ("in1", Point::new(-hw, -hh / 2)),
                ("in2", Point::new(-hw, hh / 2)),
                ("out", Point::new(hw, 0)),
            ],
            ComponentType::XspiceInverter | ComponentType::XspiceBuffer => {
                vec![("in", Point::new(-hw, 0)), ("out", Point::new(hw, 0))]
            }
            ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate => vec![
                ("a", Point::new(-hw, -hh / 2)),
                ("b", Point::new(-hw, hh / 2)),
                ("out", Point::new(hw, 0)),
            ],
            ComponentType::XspiceTristate => vec![
                ("in", Point::new(-hw, 0)),
                ("en", Point::new(0, -hh)),
                ("out", Point::new(hw, 0)),
            ],
            ComponentType::XspiceDFlipFlop => vec![
                ("d", Point::new(-hw, -hh / 2)),
                ("clk", Point::new(-hw, hh / 2)),
                ("q", Point::new(hw, -hh / 2)),
                ("qbar", Point::new(hw, hh / 2)),
            ],
            ComponentType::XspiceJkFlipFlop => vec![
                ("j", Point::new(-hw, -hh / 2)),
                ("k", Point::new(-hw, hh / 2)),
                ("clk", Point::new(-hw, 0)),
                ("q", Point::new(hw, -hh / 2)),
                ("qbar", Point::new(hw, hh / 2)),
            ],
            ComponentType::XspiceSrLatch => vec![
                ("s", Point::new(-hw, -hh / 2)),
                ("r", Point::new(-hw, hh / 2)),
                ("q", Point::new(hw, -hh / 2)),
                ("qbar", Point::new(hw, hh / 2)),
            ],
            ComponentType::XspiceAdcBridge => {
                vec![("in", Point::new(-hw, 0)), ("out", Point::new(hw, 0))]
            }
            ComponentType::XspiceDacBridge => {
                vec![("in", Point::new(-hw, 0)), ("out", Point::new(hw, 0))]
            }
        }
    }

    /// Get symbol dimensions in grid units (width, height).
    pub fn symbol_dimensions(&self) -> (i32, i32) {
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
            | ComponentType::CurrentSourceNoise => (28, 40),
            ComponentType::Ground => (20, 20),
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
