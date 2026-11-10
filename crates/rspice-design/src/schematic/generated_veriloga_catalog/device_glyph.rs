//! The house device glyph each shipped compact model draws.
//!
//! A compact model is a device, so its instance draws the device's own
//! glyph, not a block. The core glyph carries the device's terminals
//! unlabeled, as every house device symbol does; a back gate or thermal node
//! on the core is named. Any other terminal is an extra drawn at one of the
//! core's extra slots: a plain lead ending in a contact bar, or a thermal tap.
//!
//! Terminal points never depend on polarity, so changing a model's type
//! parameter never pulls a wire's end off its terminal: the N and P glyphs of
//! a family put every terminal at the same point. Which of them draws follows
//! the sign of the polarity parameter at draw time.

use rspice_core_devices::device::veriloga_builtins::generated_veriloga_model_descriptor;
use rspice_design_model::Point;
use rspice_design_model::symbol_pin::SymbolPinSide;

use crate::schematic::component::LibraryCellInstance;
use crate::schematic::port::PortSpec;
use crate::symbol_generation::{GeneratedPin, GeneratedSymbol};

/// A core glyph family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Core {
    Mos,
    SoiFourTerminal,
    Soi,
    Hemt,
    Mesfet,
    Bjt,
    BjtSubstrate,
    BjtThermal,
    Diode,
    Resistor,
    Varactor,
}

impl Core {
    /// The N (or only) artwork, and the P artwork when the family has one.
    const fn assets(self) -> (&'static str, Option<&'static str>) {
        match self {
            Self::Mos => (
                "mos_n_chan_enh_no_substrate.svg",
                Some("mos_p_chan_enh_no_substrate.svg"),
            ),
            Self::SoiFourTerminal => ("mos_n_soi_4t.svg", Some("mos_p_soi_4t.svg")),
            Self::Soi => ("mos_n_soi.svg", Some("mos_p_soi.svg")),
            Self::Hemt => ("hemt_n_4t.svg", Some("hemt_p_4t.svg")),
            Self::Mesfet => ("mesfet_n_chan.svg", Some("mesfet_p_chan.svg")),
            Self::Bjt => ("bjt_npn.svg", Some("bjt_pnp_collector_top.svg")),
            Self::BjtSubstrate => (
                "bjt_npn_substrate.svg",
                Some("bjt_pnp_substrate_collector_top.svg"),
            ),
            Self::BjtThermal => (
                "bjt_npn_thermal.svg",
                Some("bjt_pnp_thermal_collector_top.svg"),
            ),
            Self::Diode => ("diode.svg", None),
            Self::Resistor => ("resistor.svg", None),
            Self::Varactor => ("varactor_mos.svg", None),
        }
    }

    /// The box the glyph and its extras occupy.
    const fn frame(self) -> (i32, i32) {
        match self {
            Self::Diode | Self::Resistor | Self::Varactor => (40, 40),
            _ => (40, 80),
        }
    }

    /// The size the artwork is drawn at, centred in the box.
    const fn artwork(self) -> (i32, i32) {
        match self {
            Self::Diode | Self::Resistor => (40, 20),
            Self::Varactor => (40, 40),
            _ => (40, 80),
        }
    }
}

/// Where one terminal sits, in box coordinates (origin top left).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terminal {
    /// On the core artwork, unlabeled.
    Core(i32, i32),
    /// On the core artwork and named: a back gate or a thermal node.
    Named(i32, i32),
    /// A plain extra: a lead ending in a contact bar.
    Plain(i32, i32),
    /// A thermal extra: the tap's lead, tick and bar.
    Thermal(i32, i32),
}

impl Terminal {
    const fn at(self) -> (i32, i32) {
        match self {
            Self::Core(x, y) | Self::Named(x, y) | Self::Plain(x, y) | Self::Thermal(x, y) => {
                (x, y)
            }
        }
    }

    const fn on_core(self) -> bool {
        matches!(self, Self::Core(..) | Self::Named(..))
    }
}

/// How a model states its polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Polarity {
    /// The model has one polarity.
    Fixed,
    /// The sign of this parameter: non-negative N or NPN, negative P or PNP.
    Parameter(&'static str),
}

/// One shipped compact model: its module and terminals as compiled, and how
/// its glyph draws them.
#[derive(Debug)]
struct CompactModelGlyph {
    module: &'static str,
    terminals: &'static [&'static str],
    core: Core,
    placement: &'static [Terminal],
    polarity: Polarity,
}

const fn join<const A: usize, const B: usize, const S: usize>(
    core: [Terminal; A],
    extras: [Terminal; B],
) -> [Terminal; S] {
    let mut out = [Terminal::Core(0, 0); S];
    let mut index = 0;
    while index < A {
        out[index] = core[index];
        index += 1;
    }
    while index < S {
        out[index] = extras[index - A];
        index += 1;
    }
    out
}

/// Drain, gate, source and body or back gate on a 40 x 80 core; collector,
/// base, emitter and substrate on a BJT's.
const MOS: [Terminal; 4] = [
    Terminal::Core(40, 0),
    Terminal::Core(0, 40),
    Terminal::Core(40, 80),
    Terminal::Core(40, 40),
];
const SOI4: [Terminal; 4] = [
    Terminal::Core(40, 0),
    Terminal::Core(0, 40),
    Terminal::Core(40, 80),
    Terminal::Named(40, 40),
];
/// The SOI glyph with its body contact lower left.
const SOI5: [Terminal; 5] = join(SOI4, [Terminal::Core(0, 80)]);
/// Drain, gate and source; collector, base and emitter.
const THREE: [Terminal; 3] = [
    Terminal::Core(40, 0),
    Terminal::Core(0, 40),
    Terminal::Core(40, 80),
];
const BJT_THERMAL: [Terminal; 5] = join(MOS, [Terminal::Named(0, 80)]);
/// The two ends of a 40 x 40 box's horizontal glyph.
const TWO: [Terminal; 2] = [Terminal::Core(0, 20), Terminal::Core(40, 20)];
/// The extra slots: plain at the upper left corner and the top centre;
/// thermal at the lower left corner and the bottom centre of either box.
const PLAIN_CORNER: Terminal = Terminal::Plain(0, 0);
const PLAIN_TOP: Terminal = Terminal::Plain(20, 0);
const THERMAL_CORNER: Terminal = Terminal::Thermal(0, 80);
const THERMAL_BOTTOM: Terminal = Terminal::Thermal(20, 80);
const THERMAL_SMALL: Terminal = Terminal::Thermal(20, 40);

const MOS_T: [Terminal; 5] = join(MOS, [THERMAL_CORNER]);
const MOS_SUB: [Terminal; 5] = join(MOS, [PLAIN_CORNER]);
const MOS_SUB_T: [Terminal; 6] = join(MOS, [PLAIN_CORNER, THERMAL_CORNER]);
const SOI4_T: [Terminal; 5] = join(SOI4, [THERMAL_CORNER]);
const SOI_B_T: [Terminal; 7] = join(SOI5, [PLAIN_CORNER, THERMAL_BOTTOM]);
const SOI_T: [Terminal; 6] = join(SOI5, [THERMAL_BOTTOM]);
const SOI_N6: [Terminal; 6] = join(SOI5, [PLAIN_CORNER]);
const FET_T_RF: [Terminal; 5] = join(THREE, [THERMAL_CORNER, PLAIN_CORNER]);
const BJT_T: [Terminal; 4] = join(THREE, [THERMAL_CORNER]);
const TWO_T: [Terminal; 3] = join(TWO, [THERMAL_SMALL]);
const RESISTOR_NC_T: [Terminal; 4] = [
    Terminal::Core(0, 20),
    PLAIN_TOP,
    Terminal::Core(40, 20),
    THERMAL_SMALL,
];
const VARACTOR: [Terminal; 3] = [
    Terminal::Core(0, 20),
    Terminal::Plain(20, 40),
    Terminal::Core(40, 20),
];

macro_rules! glyph {
    ($module:literal, [$($terminal:literal),+], $core:ident, $placement:ident, $polarity:expr) => {
        CompactModelGlyph {
            module: $module,
            terminals: &[$($terminal),+],
            core: Core::$core,
            placement: &$placement,
            polarity: $polarity,
        }
    };
}

const TYPE: Polarity = Polarity::Parameter("TYPE");
const FIXED: Polarity = Polarity::Fixed;

/// Every shipped compact model, by module and compiled terminals.
static COMPACT_MODELS: &[CompactModelGlyph] = &[
    glyph!("PSP104VA", ["d", "g", "s", "b"], Mos, MOS, TYPE),
    glyph!("PSPNQS104VA", ["d", "g", "s", "b"], Mos, MOS, TYPE),
    glyph!("PSP104TVA", ["d", "g", "s", "b", "dt"], Mos, MOS_T, TYPE),
    glyph!("bsimbulk", ["d", "g", "s", "b", "t"], Mos, MOS_T, TYPE),
    glyph!("ekv_va", ["d", "g", "s", "b"], Mos, MOS, TYPE),
    glyph!(
        "ekv3_rf",
        ["d", "g", "s", "b"],
        Mos,
        MOS,
        Polarity::Parameter("SIGN")
    ),
    glyph!("hisimhv_n4_va", ["d", "g", "s", "b"], Mos, MOS, TYPE),
    glyph!(
        "hisimhv_n5_va",
        ["d", "g", "s", "b", "sub"],
        Mos,
        MOS_SUB,
        TYPE
    ),
    glyph!(
        "hisimhv_va",
        ["d", "g", "s", "b", "sub", "temp"],
        Mos,
        MOS_SUB_T,
        TYPE
    ),
    // L-UTSOI's b is its back gate.
    glyph!(
        "l_utsoi",
        ["d", "g", "s", "b", "tnode"],
        SoiFourTerminal,
        SOI4_T,
        TYPE
    ),
    glyph!(
        "bsimcmg_va",
        ["d", "g", "s", "e", "t"],
        SoiFourTerminal,
        SOI4_T,
        TYPE
    ),
    glyph!(
        "bsimimg",
        ["d", "fg", "s", "bg", "t"],
        SoiFourTerminal,
        SOI4_T,
        TYPE
    ),
    glyph!(
        "bsimsoi_va",
        ["d", "g", "s", "e"],
        SoiFourTerminal,
        SOI4,
        TYPE
    ),
    glyph!(
        "bsimsoi",
        ["d", "g", "s", "e", "p", "b", "t"],
        Soi,
        SOI_B_T,
        TYPE
    ),
    glyph!("bsimsoi", ["d", "g", "s", "e", "b", "t"], Soi, SOI_T, TYPE),
    glyph!(
        "hisimsoi_va",
        ["d", "g", "s", "e"],
        SoiFourTerminal,
        SOI4,
        TYPE
    ),
    glyph!("hisimsoi_va", ["d", "g", "s", "e", "bc"], Soi, SOI5, TYPE),
    glyph!(
        "hisimsoi_va",
        ["d", "g", "s", "e", "bc", "n6"],
        Soi,
        SOI_N6,
        TYPE
    ),
    glyph!(
        "hisimsotb_va",
        ["d", "g", "s", "e"],
        SoiFourTerminal,
        SOI4,
        TYPE
    ),
    glyph!("asmhemt", ["d", "g", "s", "b", "dt"], Hemt, MOS_T, FIXED),
    glyph!("mvsg_cmc", ["d", "g", "s", "b", "dt"], Hemt, MOS_T, TYPE),
    glyph!(
        "EPFL_HEMT_10a",
        ["d", "g", "s", "b", "t"],
        Hemt,
        MOS_T,
        TYPE
    ),
    glyph!("angelov", ["d", "g", "s"], Mesfet, THREE, FIXED),
    glyph!(
        "angelov_gan",
        ["d", "g", "s", "t", "rf"],
        Mesfet,
        FET_T_RF,
        FIXED
    ),
    glyph!("vbic13", ["c", "b", "e"], Bjt, THREE, TYPE),
    glyph!("vbic13_3t_et", ["c", "b", "e", "dt"], Bjt, BJT_T, TYPE),
    glyph!("vbic13_4t", ["c", "b", "e", "s"], BjtSubstrate, MOS, TYPE),
    glyph!(
        "vbic_4T_et_cf",
        ["c", "b", "e", "s", "dt"],
        BjtThermal,
        BJT_THERMAL,
        FIXED
    ),
    glyph!("bjtd504va", ["c", "b", "e"], Bjt, THREE, TYPE),
    glyph!("bjtd504tva", ["c", "b", "e", "dt"], Bjt, BJT_T, TYPE),
    glyph!("bjt504va", ["c", "b", "e", "s"], BjtSubstrate, MOS, TYPE),
    glyph!(
        "bjt504tva",
        ["c", "b", "e", "s", "dt"],
        BjtThermal,
        BJT_THERMAL,
        TYPE
    ),
    glyph!("bjtd505_va", ["c", "b", "e"], Bjt, THREE, TYPE),
    glyph!("bjtd505t_va", ["c", "b", "e", "dt"], Bjt, BJT_T, TYPE),
    glyph!("bjt505_va", ["c", "b", "e", "s"], BjtSubstrate, MOS, TYPE),
    glyph!(
        "bjt505t_va",
        ["c", "b", "e", "s", "dt"],
        BjtThermal,
        BJT_THERMAL,
        TYPE
    ),
    glyph!(
        "hicumL0va",
        ["c", "b", "e", "s", "tnode"],
        BjtThermal,
        BJT_THERMAL,
        TYPE
    ),
    glyph!(
        "hicumL2va",
        ["c", "b", "e", "s", "tnode"],
        BjtThermal,
        BJT_THERMAL,
        TYPE
    ),
    glyph!("asmesd", ["c", "b", "e", "dt"], Bjt, BJT_T, TYPE),
    // The ESD diode's b is its anode and e its cathode.
    glyph!("asmesd_dio", ["b", "e", "dt"], Diode, TWO_T, FIXED),
    glyph!("DIODE_CMC", ["a", "k"], Diode, TWO, FIXED),
    glyph!("JUNCAP200", ["a", "k"], Diode, TWO, FIXED),
    glyph!("r2_cmc", ["n1", "n2"], Resistor, TWO, FIXED),
    glyph!("r2_et_cmc", ["n1", "n2", "dt"], Resistor, TWO_T, FIXED),
    glyph!(
        "r3_cmc",
        ["n1", "nc", "n2", "dt"],
        Resistor,
        RESISTOR_NC_T,
        FIXED
    ),
    glyph!("mosvar", ["g", "bi", "b"], Varactor, VARACTOR, FIXED),
];

/// A plain extra's lead, and the half-length of its contact bar.
const PLAIN_LEAD: i32 = 10;
const PLAIN_BAR_HALF: i32 = 5;
/// A thermal tap's lead from a corner slot, its tick, and its bar.
const THERMAL_LEAD: i32 = 12;
const THERMAL_TICK: i32 = 7;
const THERMAL_BAR_HALF: i32 = 3;
/// How far a terminal's name stands off its terminal along the lead, and
/// off the lead's axis.
const NAME_GAP: i32 = 3;
const NAME_LIFT: i32 = 1;

/// A terminal name a device glyph letters beside its terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlyphName {
    pub text: String,
    /// The name's anchor in symbol coordinates.
    pub at: Point,
    /// Which way the name's box reaches from the anchor on each axis:
    /// 1 toward positive, -1 toward negative.
    pub reach: Point,
}

/// The glyph a placement of a shipped compact model draws.
#[derive(Debug, Clone, Copy)]
pub struct DeviceGlyph {
    model: &'static CompactModelGlyph,
}

/// The device glyph for a placement of a shipped compact model, matched by
/// its module and its terminals as compiled; `None` for any other binding.
pub fn compact_model_glyph(binding: &LibraryCellInstance) -> Option<DeviceGlyph> {
    let generated = binding.generated_veriloga.as_ref()?;
    COMPACT_MODELS
        .iter()
        .find(|model| {
            model.module.eq_ignore_ascii_case(&generated.module_name)
                && model.terminals.len() == binding.terminal_order.len()
                && model
                    .terminals
                    .iter()
                    .zip(&binding.terminal_order)
                    .all(|(expected, bound)| expected.eq_ignore_ascii_case(bound))
        })
        .map(|model| DeviceGlyph { model })
}

impl DeviceGlyph {
    fn centred(&self, (x, y): (i32, i32)) -> Point {
        let (width, height) = self.model.core.frame();
        Point::new(x - width / 2, y - height / 2)
    }

    /// The artwork a placement whose instance parameters are `params` draws:
    /// P when its polarity parameter, stated or by default, is negative.
    pub fn asset(&self, binding: &LibraryCellInstance, params: &str) -> &'static str {
        let (n, p) = self.model.core.assets();
        let (Some(p), Polarity::Parameter(name)) = (p, self.model.polarity) else {
            return n;
        };
        let stated = crate::parameters::parse_params_string(params)
            .get(&name.to_ascii_lowercase())
            .and_then(|value| value.trim().parse::<f64>().ok());
        let by_default = || {
            let generated = binding.generated_veriloga.as_ref()?;
            generated_veriloga_model_descriptor(&generated.model_name)?
                .parameters
                .iter()
                .find(|parameter| {
                    parameter.name.eq_ignore_ascii_case(name)
                        || parameter
                            .aliases
                            .iter()
                            .any(|alias| alias.eq_ignore_ascii_case(name))
                })?
                .default
        };
        if stated.or_else(by_default).is_some_and(|sign| sign < 0.0) {
            p
        } else {
            n
        }
    }

    /// The size the artwork is drawn at, centred on the symbol origin.
    pub fn artwork_size(&self) -> (i32, i32) {
        self.model.core.artwork()
    }

    /// The glyph's terminals as a block: pins in interface order on the
    /// box's edges, the box as the body.
    pub fn block(&self, ports: &[PortSpec]) -> GeneratedSymbol {
        let (width, height) = self.model.core.frame();
        let pins = ports
            .iter()
            .zip(self.model.placement)
            .map(|(port, terminal)| {
                let (x, y) = terminal.at();
                GeneratedPin {
                    name: port.name.clone(),
                    direction: port.direction,
                    side: if x == 0 {
                        SymbolPinSide::Left
                    } else if x == width {
                        SymbolPinSide::Right
                    } else if y == 0 {
                        SymbolPinSide::Top
                    } else {
                        SymbolPinSide::Bottom
                    },
                    offset: self.centred((x, y)),
                }
            })
            .collect();
        GeneratedSymbol {
            body_extent: (self.centred((0, 0)), self.centred((width, height))),
            ..GeneratedSymbol::fixed_block(pins, width, height)
        }
    }

    /// The terminals the core artwork reaches, in interface order.
    pub fn core_offsets(&self) -> Vec<Point> {
        self.model
            .placement
            .iter()
            .filter(|terminal| terminal.on_core())
            .map(|terminal| self.centred(terminal.at()))
            .collect()
    }

    /// The extras drawn beside the core artwork, as line segments in symbol
    /// coordinates.
    pub fn extra_segments(&self) -> Vec<(Point, Point)> {
        let (width, _) = self.model.core.frame();
        let mut segments = Vec::new();
        for terminal in self.model.placement {
            let (x, y) = terminal.at();
            let at = |dx: i32, dy: i32| self.centred((x + dx, y + dy));
            // Into the box: across it from a side slot, down or up into it
            // from a top or bottom one.
            let side = x == 0 || x == width;
            let inward = if side {
                (if x == 0 { 1 } else { -1 }, 0)
            } else {
                (0, if y == 0 { 1 } else { -1 })
            };
            match terminal {
                Terminal::Core(..) | Terminal::Named(..) => {}
                Terminal::Plain(..) => {
                    let (end_x, end_y) = (inward.0 * PLAIN_LEAD, inward.1 * PLAIN_LEAD);
                    let (across_x, across_y) =
                        (inward.1 * PLAIN_BAR_HALF, inward.0 * PLAIN_BAR_HALF);
                    segments.push((at(0, 0), at(end_x, end_y)));
                    segments.push((
                        at(end_x - across_x, end_y - across_y),
                        at(end_x + across_x, end_y + across_y),
                    ));
                }
                Terminal::Thermal(..) => {
                    // A corner tap runs along the box's edge before rising;
                    // a centre tap rises straight off its terminal.
                    let lead = if side { inward.0 * THERMAL_LEAD } else { 0 };
                    let tick = if y == 0 { THERMAL_TICK } else { -THERMAL_TICK };
                    if lead != 0 {
                        segments.push((at(0, 0), at(lead, 0)));
                    }
                    segments.push((at(lead, 0), at(lead, tick)));
                    segments.push((
                        at(lead - THERMAL_BAR_HALF, tick),
                        at(lead + THERMAL_BAR_HALF, tick),
                    ));
                }
            }
        }
        segments
    }

    /// The names the glyph letters: its back gates, thermal nodes and
    /// extras, each just outside its terminal and clear of the lead.
    pub fn names(&self, ports: &[PortSpec]) -> Vec<GlyphName> {
        let (width, _) = self.model.core.frame();
        ports
            .iter()
            .zip(self.model.placement)
            .filter(|(_, terminal)| !matches!(terminal, Terminal::Core(..)))
            .map(|(port, terminal)| {
                let (x, y) = terminal.at();
                let (step, reach) = if x == 0 {
                    ((-NAME_GAP, -NAME_LIFT), Point::new(-1, -1))
                } else if x == width {
                    ((NAME_GAP, -NAME_LIFT), Point::new(1, -1))
                } else if y == 0 {
                    ((NAME_GAP, -NAME_GAP), Point::new(1, -1))
                } else {
                    ((NAME_GAP, NAME_GAP), Point::new(1, 1))
                };
                let terminal = self.centred((x, y));
                GlyphName {
                    text: port.name.clone(),
                    at: Point::new(terminal.x + step.0, terminal.y + step.1),
                    reach,
                }
            })
            .collect()
    }
}

#[cfg(all(
    test,
    feature = "generated-veriloga-catalog",
    feature = "symbol-artwork"
))]
mod tests {
    use super::*;
    use crate::schematic::component::Component;
    use crate::schematic::component_type::ComponentType;
    use crate::schematic::generated_veriloga_catalog::{
        generated_veriloga_devices, generated_veriloga_library_binding,
    };
    use crate::symbol_artwork::SymbolLibrary;

    /// Every shipped compact model draws a device glyph whose artwork is in
    /// the build and reaches the glyph's core terminals, in both polarities,
    /// and whose polarity parameter the model declares.
    #[test]
    fn every_shipped_compact_model_resolves_to_a_device_glyph() {
        let library = SymbolLibrary::load_embedded().expect("the embedded symbols load");
        let mut drawn = vec![false; COMPACT_MODELS.len()];
        for descriptor in generated_veriloga_devices() {
            let binding = generated_veriloga_library_binding(descriptor).expect("a binding");
            let glyph = compact_model_glyph(&binding).unwrap_or_else(|| {
                panic!(
                    "{} ({}, {:?}) draws no device glyph",
                    descriptor.model_name, descriptor.module_name, binding.terminal_order
                )
            });
            let entry = COMPACT_MODELS
                .iter()
                .position(|model| std::ptr::eq(model, glyph.model))
                .expect("a table entry");
            drawn[entry] = true;
            let (n, p) = glyph.model.core.assets();
            let (width, height) = glyph.artwork_size();
            let placed = Component::new(1, ComponentType::CellInstance, Point::origin())
                .with_library_cell(binding.clone());
            for asset in std::iter::once(n).chain(p) {
                assert!(
                    library.artwork_fits(&placed, asset, width as f32, height as f32),
                    "{asset} does not reach the terminals of {}",
                    descriptor.model_name
                );
            }
            if let Polarity::Parameter(name) = glyph.model.polarity {
                assert!(
                    descriptor.parameters.iter().any(|parameter| {
                        parameter.name.eq_ignore_ascii_case(name)
                            || parameter
                                .aliases
                                .iter()
                                .any(|alias| alias.eq_ignore_ascii_case(name))
                    }),
                    "{} declares no {name}",
                    descriptor.model_name
                );
            }
            let ports = binding.interface().expect("an interface");
            for pin in glyph.block(&ports).pins {
                assert_eq!(
                    (pin.offset.x % 10, pin.offset.y % 10),
                    (0, 0),
                    "{}: {pin:?} is off the grid",
                    descriptor.model_name
                );
            }
        }
        // No entry is for a model the build does not ship.
        assert!(drawn.iter().all(|drawn| *drawn), "{drawn:?}");
    }

    fn binding(model: &str) -> LibraryCellInstance {
        let descriptor = generated_veriloga_devices()
            .iter()
            .find(|descriptor| descriptor.model_name == model)
            .expect("the model is shipped");
        generated_veriloga_library_binding(descriptor).expect("a binding")
    }

    /// BSIM-CMG draws the four-terminal SOI glyph with its back gate and
    /// thermal node named, the P glyph when its type is negative, and the
    /// same terminal points either way.
    #[test]
    fn bsim_cmg_draws_the_soi_glyph_and_follows_its_type() {
        let binding = binding("bsimcmg_va");
        let glyph = compact_model_glyph(&binding).expect("a device glyph");
        assert_eq!(glyph.asset(&binding, ""), "mos_n_soi_4t.svg");
        assert_eq!(glyph.asset(&binding, "type=-1"), "mos_p_soi_4t.svg");
        assert_eq!(glyph.asset(&binding, "TYPE=1 L=30n"), "mos_n_soi_4t.svg");
        let ports = binding.interface().expect("an interface");
        let block = glyph.block(&ports);
        let at = |name: &str| {
            block
                .pins
                .iter()
                .find(|pin| pin.name.eq_ignore_ascii_case(name))
                .map(|pin| pin.offset)
                .expect("the pin")
        };
        assert_eq!(at("d"), Point::new(20, -40));
        assert_eq!(at("g"), Point::new(-20, 0));
        assert_eq!(at("s"), Point::new(20, 40));
        assert_eq!(at("e"), Point::new(20, 0));
        assert_eq!(at("t"), Point::new(-20, 40));
        let names = glyph
            .names(&ports)
            .into_iter()
            .map(|name| name.text.to_ascii_lowercase())
            .collect::<Vec<_>>();
        assert_eq!(names, ["e", "t"]);
        // The thermal tap: along the bottom edge, a tick up, and its bar.
        assert_eq!(
            glyph.extra_segments(),
            [
                (Point::new(-20, 40), Point::new(-8, 40)),
                (Point::new(-8, 40), Point::new(-8, 33)),
                (Point::new(-11, 33), Point::new(-5, 33)),
            ]
        );
    }

    /// A PNP draws collector up, so its terminals are the NPN's.
    #[test]
    fn a_pnp_keeps_the_npn_terminal_points() {
        let binding = binding("hicumL2va");
        let glyph = compact_model_glyph(&binding).expect("a device glyph");
        assert_eq!(
            glyph.asset(&binding, "type=-1"),
            "bjt_pnp_thermal_collector_top.svg"
        );
        assert_eq!(glyph.asset(&binding, "type=1"), "bjt_npn_thermal.svg");
        let names = glyph.names(&binding.interface().expect("an interface"));
        assert_eq!(names.len(), 1);
        assert_eq!(names[0].at, Point::new(-23, 39));
        assert_eq!(names[0].reach, Point::new(-1, -1));
    }
}
