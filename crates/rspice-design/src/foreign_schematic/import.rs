//! Turning a foreign drawing into an RSpice schematic.
//!
//! The drawing is scaled onto RSpice's grid: one LTspice grid step (16
//! units) becomes two RSpice steps, and one QSPICE step (100 units) one.
//! Every place a source pin, wire end or flag sits keeps its scaled position,
//! so the source's wiring stands as it was drawn. A part RSpice draws natively
//! is turned and placed where its terminals best meet the source pins, and a
//! straight lead joins any terminal that does not land on its pin. A part
//! RSpice has no component for keeps the source symbol's own drawing as a
//! cell of the project, so its pins land exactly.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

use rspice_design_model::Point;
use rspice_design_model::port::{PortDirection, PortSpec};

use super::behaviour::qspice_behaviour;
use super::directives::{defined_model_names, foreign_directive_cards};
use super::functional::{Functional, FunctionalPlan, SourceNets, ltspice_gate, ltspice_opamp};
use super::mapping::{
    LTSPICE_AWAITING_NATIVE_PARTS, NativeKind, NativeMapping, ltspice_builtin_definition,
    native_mapping, qspice_refusal, source_value, spice_text, spice_tokens,
};
use super::qspice::DEVICE_TYPE;
use super::{
    ForeignAttribute, ForeignLettering, ForeignPin, ForeignPoint, ForeignPortDirection,
    ForeignSchematic, ForeignShape, ForeignSymbol, ForeignSymbolDefinition, ForeignTextKind,
    ForeignTool, ForeignWindow,
};
use crate::schematic::component::{Component, LibraryCellInstance};
use crate::schematic::component_display::{
    ComponentDisplayMode, SchematicParameterLabelVisibility,
};
use crate::schematic::component_type::ComponentType;
use crate::schematic::design_note::{DESIGN_NOTE_TEXT_OFFSET_WORLD, DesignNote, DesignNoteKind};
use crate::schematic::document::SchematicDocument;
use crate::schematic::document_policy::{NetNamingPolicy, SchematicDocumentPolicy};
use crate::schematic::documentation_shape::{DocumentationShape, DocumentationShapeGeometry};
use crate::schematic::functional_catalog::{
    conform_functional_inputs, functional_library_binding, functional_part, placement_value,
};
use crate::schematic::label_layout::{
    INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD, LayoutPoint, LayoutRect, NET_LABEL_FONT_WORLD,
    PlacedText, TextEdge, instance_label_layout, net_label_layout, supply_glyph_rect, text_box,
};
use crate::schematic::label_position::{LabelPosition, ValuePiece};
use crate::schematic::net_label::{Junction, NetLabel, NetLabelKind, NetLabelSide, SupplyGlyph};
use crate::schematic::rotation::Rotation;
use crate::schematic::terminal_connection::rebuild_connections_from_terminals;
use crate::schematic::value_label::{component_value_label, value_and_parameters, value_pieces};
use crate::schematic::wire::Wire;
use crate::symbol::{PinNamePlacement, SymbolDocument, SymbolPin, SymbolShape};

/// RSpice's schematic grid step.
const GRID: i32 = 10;
/// The places of an LTspice A-device's instance line.
const DEVICE_PLACES: usize = 8;

/// The texts a source's drawing letters its waveform in, by LTspice's
/// window number: its value, its second value (`AC 1`) and its settings
/// (`Rser=1`).
const SOURCE_TEXTS: [(u32, &str); 4] = [
    (3, "Value"),
    (123, "Value2"),
    (39, "SpiceLine"),
    (40, "SpiceLine2"),
];

/// The air an instance's name or value keeps from a body RSpice draws for
/// it, as the shared layout keeps it.
const LABEL_CLEARANCE: f32 = 4.0;

/// The device letters whose value names a `.model` card.
const MODEL_PREFIXES: [&str; 7] = ["D", "Q", "M", "J", "Z", "S", "W"];

/// What the importer may read beside the drawing or from the source
/// program's installed libraries. Files are read in place and never copied.
pub trait ForeignLibrary {
    /// The definition of a symbol the drawing places, by the name the drawing
    /// writes (`OpAmps\LT1001`).
    fn symbol(&self, name: &str) -> Option<ForeignSymbolSource>;
    /// Where a model file a symbol names (`SpiceModel`, `ModelFile`) lives.
    fn model_file(&self, name: &str) -> Option<ForeignModelFile>;
    /// The `.model` card the source program finds for a model a part names
    /// without the drawing defining it (LTspice's `standard.dio`, QSPICE's
    /// `Diode.txt`).
    fn model_card(&self, name: &str) -> Option<String>;
}

/// A symbol definition found for a placed symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSymbolSource {
    pub definition: ForeignSymbolDefinition,
    /// Where the definition was read, for the record.
    pub location: String,
    /// The schematic beside the symbol that defines its insides, when the
    /// symbol is a hierarchical block.
    pub schematic: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignModelFile {
    pub path: PathBuf,
    /// The file is encrypted, so no simulator but its vendor's can read it.
    pub encrypted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForeignNoticeSeverity {
    /// Imported, with something the reader should know.
    Notice,
    /// Part of the drawing did not come across.
    Refusal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignImportNotice {
    pub severity: ForeignNoticeSeverity,
    pub message: String,
}

/// A directive block of the drawing, where and how it was lettered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignDirective {
    pub position: Point,
    pub text: String,
    pub lettering: ForeignLettering,
}

/// A symbol RSpice has no component for, as a cell for the project library:
/// its drawing and pins scaled to the grid, its pins in instance-line order.
#[derive(Debug, Clone, PartialEq)]
pub struct ForeignSymbolCell {
    pub cell: String,
    pub symbol: SymbolDocument,
    pub ports: Vec<PortSpec>,
    /// The symbol name the drawing wrote.
    pub source_symbol: String,
    /// Where its definition was read; `None` for a generated box.
    pub location: Option<String>,
    /// The schematic that defines a hierarchical block's insides.
    pub schematic: Option<PathBuf>,
    /// The model or subcircuit its instances write; `None` for a
    /// hierarchical block.
    pub model: Option<String>,
    /// The instance line of a part whose device letter is not `X`.
    pub template: Option<String>,
    /// The file that defines its model, when one does: the cell then runs
    /// through a `spice` view over that file.
    pub source: Option<ForeignModelSource>,
    /// Ports for instance-line places the source symbol gives no pin: each
    /// is a symbol pin that touches nothing, so it writes a node of its own,
    /// as LTspice writes `MP_nn` there. An A-device's share its common's pin,
    /// or one grounded point when it draws none.
    pub unconnected: Vec<String>,
}

/// A file, and the `.lib` section of it, that defines a cell's model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignModelSource {
    pub path: PathBuf,
    pub section: Option<String>,
}

impl ForeignSymbolCell {
    /// Takes the first of `files` whose text defines this cell's model as
    /// its source, when it has none. Each file is `(path, section, text)`.
    pub fn find_source(&mut self, files: &[(PathBuf, Option<String>, String)]) {
        let Some(model) = self.model.as_deref().filter(|_| self.source.is_none()) else {
            return;
        };
        let keyword = if self.template.is_some() {
            ".model"
        } else {
            ".subckt"
        };
        self.source = files
            .iter()
            .find(|(_, _, text)| defines(text, keyword, model))
            .map(|(path, section, _)| ForeignModelSource {
                path: path.clone(),
                section: section.clone(),
            });
    }
}

/// Whether a line of `text` opens `keyword name`.
fn defines(text: &str, keyword: &str, name: &str) -> bool {
    text.lines().any(|line| {
        let mut words = line.split_ascii_whitespace();
        words
            .next()
            .is_some_and(|word| word.eq_ignore_ascii_case(keyword))
            && words
                .next()
                .is_some_and(|word| word.eq_ignore_ascii_case(name))
    })
}

/// Moves every instance of a cell with a source file onto that cell's
/// `spice` view, bound to the file.
pub fn bind_instances_to_sources(
    document: &mut SchematicDocument,
    library: &str,
    cells: &[ForeignSymbolCell],
) {
    for component in &mut document.components {
        let Some(binding) = component.library_cell.as_mut() else {
            continue;
        };
        let Some(source) = cells
            .iter()
            .filter(|cell| binding.library == library && binding.cell == cell.cell)
            .find_map(|cell| cell.source.as_ref())
        else {
            continue;
        };
        binding.view = "spice".to_owned();
        binding.source_path = Some(source.path.clone());
        binding.model_section.clone_from(&source.section);
    }
}

/// The imported schematic and everything that did not become sheet content.
#[derive(Debug, Clone)]
pub struct ForeignImport {
    pub document: SchematicDocument,
    pub cells: Vec<ForeignSymbolCell>,
    pub directives: Vec<ForeignDirective>,
    /// The source program's own cards for models the parts name and the
    /// drawing does not define.
    pub model_cards: Vec<String>,
    pub notices: Vec<ForeignImportNotice>,
}

/// Imports a foreign drawing; `library` is the project library the
/// drawing's own symbols are created in.
pub fn import_foreign_schematic(
    drawing: &ForeignSchematic,
    library: &str,
    access: &dyn ForeignLibrary,
) -> ForeignImport {
    let mut builder = Builder {
        drawing,
        library,
        access,
        // Both programs name nets as SPICE does: case-blind, with any
        // printable character (`V+`, `+5V`).
        document: SchematicDocument {
            document_policy: SchematicDocumentPolicy {
                net_naming: NetNamingPolicy::SpiceCompatibleRelaxed,
                ..SchematicDocumentPolicy::default()
            },
            ..SchematicDocument::default()
        },
        next_id: 1,
        cells: Vec::new(),
        cell_index: HashMap::new(),
        notices: Vec::new(),
        directives: Vec::new(),
        leads: Vec::new(),
        source_points: HashSet::new(),
        source_segments: Vec::new(),
        source_nets: SourceNets::default(),
        anchors: HashSet::new(),
        terminals: Vec::new(),
        names: BTreeSet::new(),
        models: BTreeSet::new(),
        pin_points: HashMap::new(),
        pin_leads: HashMap::new(),
        lettered: Vec::new(),
        free_names: Vec::new(),
        device_ties: Vec::new(),
        behaviours: Vec::new(),
    };
    let resolved = drawing
        .symbols
        .iter()
        .map(|symbol| builder.resolve(symbol))
        .collect::<Vec<_>>();
    // LTspice's jumper joins its two pins' nets as a wire does.
    let jumpers = drawing
        .symbols
        .iter()
        .zip(&resolved)
        .filter(|(symbol, _)| {
            drawing.tool == ForeignTool::Ltspice
                && symbol.base_name().eq_ignore_ascii_case("jumper")
        })
        .filter_map(|(symbol, (definition, ..))| {
            let [a, b] = definition.as_ref()?.pins.as_slice() else {
                return None;
            };
            let at = |pin: &ForeignPin| {
                builder.scale(
                    symbol
                        .position
                        .offset(symbol.orientation.apply(pin.position)),
                )
            };
            Some((at(a), at(b)))
        })
        .collect::<Vec<_>>();
    builder.collect_source_geometry(&jumpers);
    // Every pin is known before any lead is routed, so no lead runs through
    // a pin of a part placed after it.
    for (symbol, (definition, ..)) in drawing.symbols.iter().zip(&resolved) {
        let Some(definition) = definition else {
            continue;
        };
        let centre = body_centre(definition);
        for pin in &definition.pins {
            let at = builder.scale(
                symbol
                    .position
                    .offset(symbol.orientation.apply(pin.position)),
            );
            *builder.pin_points.entry(at).or_default() += 1;
            let inward = symbol.orientation.apply(ForeignPoint::new(
                centre.x - pin.position.x,
                centre.y - pin.position.y,
            ));
            builder
                .pin_leads
                .entry(at)
                .or_default()
                .push(Point::new(inward.x, inward.y));
        }
    }
    for (symbol, resolved) in drawing.symbols.iter().zip(resolved) {
        builder.place_symbol(symbol, resolved);
    }
    builder.tie_device_pins();
    builder.place_flags();
    builder.clear_net_names();
    builder.clear_lettering();
    builder.place_texts();
    builder.place_shapes();
    builder.place_wires();
    builder.finish()
}

struct Builder<'a> {
    drawing: &'a ForeignSchematic,
    library: &'a str,
    access: &'a dyn ForeignLibrary,
    document: SchematicDocument,
    next_id: u64,
    cells: Vec<ForeignSymbolCell>,
    cell_index: HashMap<String, usize>,
    notices: Vec<ForeignImportNotice>,
    directives: Vec<ForeignDirective>,
    /// Leads from RSpice terminals to the source pins they stand for.
    leads: Vec<(Point, Point)>,
    /// Every point the source drawing's wires, flags and junctions join at,
    /// and its wire segments, on RSpice's grid: what a lead must not touch.
    source_points: HashSet<Point>,
    source_segments: Vec<(Point, Point)>,
    /// The nets the source drawing's wires and flags make.
    source_nets: SourceNets,
    /// Every point the source drawing joins things at.
    anchors: HashSet<Point>,
    /// Every placed terminal, for the wire-connection cache.
    terminals: Vec<(u64, String, Point)>,
    names: BTreeSet<String>,
    /// The models native parts name, for the source program's cards.
    models: BTreeSet<String>,
    /// How many placed symbols have a pin at each point.
    pin_points: HashMap<Point, usize>,
    /// The way each source pin's lead runs from its point into its body.
    pin_leads: HashMap<Point, Vec<Point>>,
    /// The placed parts whose name and value stand where the drawing
    /// lettered them, by index.
    lettered: Vec<usize>,
    /// Net labels, by index, whose name may stand on another side where it
    /// lands on a body, with those sides in the order to try them.
    free_names: Vec<(usize, Vec<NetLabelSide>)>,
    /// Terminals of LTspice A-devices that LTspice grounds when nothing
    /// else reaches them.
    device_ties: Vec<DeviceTie>,
    /// The subcircuits written for QSPICE's behavioural devices.
    behaviours: Vec<String>,
}

type Resolved = (
    Option<ForeignSymbolDefinition>,
    Option<String>,
    Option<PathBuf>,
);

/// A terminal of an LTspice A-device, tied to ground unless its source pin
/// (`None`: it has none) meets a wire, a flag or another pin.
struct DeviceTie {
    source: Option<Point>,
    terminal: Point,
    /// The part's body, which the ground faces away from.
    body: Body,
    /// The net the terminal joins, ground when there is none.
    net: Option<String>,
}

/// The middle of a placed part's body and half its width and height.
#[derive(Debug, Clone, Copy)]
struct Body {
    center: Point,
    half: Point,
}

impl Body {
    fn of(symbol: &SymbolDocument, component: &Component, terminals: &[Point]) -> Self {
        let (low, high) = symbol.body_bounds();
        if low == high {
            return Self {
                center: centre(terminals),
                half: Point::new(1, 1),
            };
        }
        let [low, high] =
            [low, high].map(|corner| component.pos + component.transform_point(corner));
        Self {
            center: Point::new((low.x + high.x) / 2, (low.y + high.y) / 2),
            half: Point::new(
                ((high.x - low.x) / 2).abs().max(1),
                ((high.y - low.y) / 2).abs().max(1),
            ),
        }
    }

    /// The way a ground on `terminal` faces: out of the body edge the
    /// terminal stands on.
    fn outward(self, terminal: Point) -> Rotation {
        let away = terminal - self.center;
        // Measured in half-widths, so a pin on a long side reads as on it.
        if i64::from(away.x.abs()) * i64::from(self.half.y)
            > i64::from(away.y.abs()) * i64::from(self.half.x)
        {
            if away.x > 0 {
                Rotation::R270
            } else {
                Rotation::R90
            }
        } else if away.y < 0 {
            Rotation::R180
        } else {
            Rotation::R0
        }
    }
}

impl Builder<'_> {
    fn id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn notice(&mut self, message: impl Into<String>) {
        self.notices.push(ForeignImportNotice {
            severity: ForeignNoticeSeverity::Notice,
            message: message.into(),
        });
    }

    fn refuse(&mut self, message: impl Into<String>) {
        self.notices.push(ForeignImportNotice {
            severity: ForeignNoticeSeverity::Refusal,
            message: message.into(),
        });
    }

    /// A drawing point on RSpice's grid.
    fn scale(&self, point: ForeignPoint) -> Point {
        Point::new(self.scale_length(point.x), self.scale_length(point.y))
    }

    fn scale_length(&self, length: i32) -> i32 {
        let (numerator, denominator) = match self.drawing.tool {
            ForeignTool::Ltspice => (5_i64, 4_i64),
            ForeignTool::Qspice => (1, 10),
        };
        let scaled = i64::from(length) * numerator;
        let step = denominator * i64::from(GRID);
        let snapped = (scaled + scaled.signum() * step / 2) / step * i64::from(GRID);
        i32::try_from(snapped).unwrap_or(if snapped < 0 { i32::MIN } else { i32::MAX })
    }

    /// A drawing point at full precision, for artwork that need not sit on
    /// the grid.
    fn scale_fine(&self, point: ForeignPoint) -> Point {
        let (numerator, denominator) = match self.drawing.tool {
            ForeignTool::Ltspice => (5, 4),
            ForeignTool::Qspice => (1, 10),
        };
        let fine = |value: i32| {
            let scaled = i64::from(value) * numerator;
            i32::try_from((scaled + scaled.signum() * denominator / 2) / denominator).unwrap_or(0)
        };
        Point::new(fine(point.x), fine(point.y))
    }

    /// A placed symbol's definition, where it was read, and the schematic
    /// of a hierarchical block.
    fn resolve(&self, symbol: &ForeignSymbol) -> Resolved {
        match &symbol.embedded {
            Some(definition) => (Some(definition.clone()), None, None),
            None => match self.access.symbol(&symbol.symbol) {
                Some(source) => (
                    Some(source.definition),
                    Some(source.location),
                    source.schematic,
                ),
                None => (ltspice_builtin_definition(symbol.base_name()), None, None),
            },
        }
    }

    fn place_symbol(&mut self, symbol: &ForeignSymbol, resolved: Resolved) {
        let tool = self.drawing.tool;
        let label = self.symbol_label(symbol);
        if tool == ForeignTool::Qspice
            && let Some(definition) = &symbol.embedded
            && self.place_behaviour(symbol, definition)
        {
            return;
        }
        if tool == ForeignTool::Qspice
            && let Some(reason) = symbol.embedded.as_ref().and_then(|definition| {
                qspice_refusal(definition.symbol_type.trim(), &qspice_kind(symbol))
            })
        {
            self.refuse(format!("{label} {reason}; it was left out."));
            self.place_marker(symbol, symbol.embedded.as_ref(), &label);
            return;
        }
        // LTspice names its frequency-response probes `@n`; only LTspice
        // runs them.
        if tool == ForeignTool::Ltspice
            && symbol
                .attribute("InstName")
                .is_some_and(|name| name.trim().starts_with('@'))
        {
            self.refuse(format!(
                "{label} is an LTspice frequency-response probe, which only LTspice runs; it was left out, so the loop it was inserted in is open."
            ));
            self.place_marker(symbol, resolved.0.as_ref(), &label);
            return;
        }
        let (definition, location, schematic) = resolved;
        let Some(definition) = definition else {
            self.refuse(format!(
                "{label}: no symbol file `{}.asy` was found beside the schematic or in LTspice's symbol library, so its pins are unknown; it is drawn as a box with no pins.",
                symbol.symbol
            ));
            self.place_unknown_box(symbol);
            return;
        };
        let attributes = merged_attributes(&definition, symbol);
        // LTspice letters only the attributes a WINDOW shows; a symbol read
        // from its file says which. A stand-in for a missing file says
        // nothing, so its part keeps RSpice's own lettering.
        let windowed = tool == ForeignTool::Ltspice && location.is_some();
        let pins = definition
            .ordered_pins()
            .into_iter()
            .map(|pin| {
                self.scale(
                    symbol
                        .position
                        .offset(symbol.orientation.apply(pin.position)),
                )
            })
            .collect::<Vec<_>>();
        self.anchors.extend(pins.iter().copied());
        // LTspice's jumper joins its two nets and is no device: a wire.
        if tool == ForeignTool::Ltspice
            && symbol.base_name().eq_ignore_ascii_case("jumper")
            && let [from, to] = pins.as_slice()
        {
            self.lead(*from, *to);
            return;
        }
        let placed = self.document.components.len();
        match native_mapping(tool, symbol.base_name(), Some(&definition)) {
            Some(mapping) if schematic.is_none() => {
                self.place_native(symbol, &attributes, mapping, &pins);
            }
            _ => {
                if schematic.is_none()
                    && let Some(plan) =
                        self.functional_plan(symbol, &definition, &attributes, &pins)
                {
                    self.place_functional(symbol, &attributes, plan, &pins);
                } else {
                    if tool == ForeignTool::Ltspice
                        && LTSPICE_AWAITING_NATIVE_PARTS
                            .contains(&symbol.base_name().to_ascii_lowercase().as_str())
                    {
                        self.notice(format!(
                            "{label} keeps LTspice's own symbol and model until RSpice carries this part natively."
                        ));
                    }
                    self.place_cell(symbol, &definition, &attributes, location, schematic, &pins);
                }
            }
        }
        if self.document.components.len() > placed {
            self.letter_like_the_drawing(symbol, &definition, windowed);
        }
    }

    /// Places a QSPICE ¥-device gate or Ã-device Gm amplifier as the
    /// behavioural subcircuit written for it, which goes into the project's
    /// model library. Returns whether it did.
    fn place_behaviour(
        &mut self,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
    ) -> bool {
        let Some(device) = definition.symbol_type.trim().chars().next() else {
            return false;
        };
        let value = format!(
            "{} {}",
            symbol.attribute(DEVICE_TYPE).unwrap_or_default(),
            symbol.attribute("Value").unwrap_or_default()
        );
        let ordered = definition.ordered_pins();
        let pins = ordered
            .iter()
            .map(|pin| self.pin_source(symbol, pin))
            .collect::<Vec<_>>();
        // A pin with a net of its own joins it when nothing is wired to it.
        let reached = ordered
            .iter()
            .zip(&pins)
            .map(|(pin, point)| pin.default_net.is_some() || self.reached(*point))
            .collect::<Vec<_>>();
        let model = format!(
            "{}_{}",
            cell_name(&symbol.symbol),
            self.behaviours.len() + 1
        );
        let Some(behaviour) = qspice_behaviour(device, &value, &reached, &model) else {
            return false;
        };
        let label = self.symbol_label(symbol);
        let unmodelled = if behaviour.unmodelled.is_empty() {
            String::new()
        } else {
            format!(
                "; its {} setting(s) are not modelled",
                behaviour.unmodelled.join(", ")
            )
        };
        self.notice(format!(
            "{label} is QSPICE's {device}-device {}; it comes across as the behavioural subcircuit `{model}`, written into the project's model library, with its outputs between its Vdd and Vss pins{unmodelled}.",
            behaviour.kind
        ));
        self.behaviours.push(behaviour.card);
        // The device letter is QSPICE's; RSpice names the part as a
        // subcircuit.
        let name = symbol
            .attribute("InstName")
            .unwrap_or_default()
            .trim_start_matches(device);
        let attributes = [
            ForeignAttribute {
                name: "InstName".to_owned(),
                value: format!("X{name}"),
            },
            ForeignAttribute {
                name: "Value".to_owned(),
                value: model,
            },
        ];
        self.anchors.extend(pins.iter().copied());
        let placed = self.document.components.len();
        self.place_cell(symbol, definition, &attributes, None, None, &pins);
        if self.document.components.len() > placed {
            self.letter_like_the_drawing(symbol, definition, false);
        }
        true
    }

    /// Puts the last placed part's name and value where the drawing
    /// lettered them: at the instance's own `WINDOW`, else its symbol's,
    /// turned with the instance. The text stays upright and hangs off its
    /// anchor on the side the drawing's run grows toward. When `windowed`,
    /// a name or value with no window at all is not lettered, as LTspice
    /// letters none.
    fn letter_like_the_drawing(
        &mut self,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
        windowed: bool,
    ) {
        let window = |attribute: u32| {
            symbol
                .windows
                .iter()
                .chain(&definition.windows)
                .find(|window| window.attribute == attribute)
        };
        let (name, value) = (window(0), window(3));
        let Some(component) = self.document.components.last() else {
            return;
        };
        let origin = component.pos;
        let place = |window: Option<&ForeignWindow>| {
            window
                .filter(|window| !window.hidden)
                .map(|window| self.label_position(symbol, window, origin))
        };
        let (name_position, value_position) = (place(name), place(value));
        let mut pieces = symbol
            .value_pieces
            .iter()
            .map(|piece| {
                (
                    piece.text.clone(),
                    self.label_position(symbol, &piece.window, origin),
                    piece.size_percent,
                )
            })
            .collect::<Vec<_>>();
        // A source's drawing letters its waveform as written, on a line,
        // where RSpice letters a line for each parameter: each text the
        // drawing shows keeps its words and its place.
        let source = component.kind.is_source();
        if source && pieces.is_empty() {
            let attributes = merged_attributes(definition, symbol);
            pieces = SOURCE_TEXTS
                .into_iter()
                .filter_map(|(number, key)| {
                    let window = window(number).filter(|window| !window.hidden)?;
                    let text =
                        attribute(&attributes, key).filter(|text| !text.trim().is_empty())?;
                    Some((
                        text.trim().to_owned(),
                        self.label_position(symbol, window, origin),
                        100,
                    ))
                })
                .collect();
        }
        let stands_for = source.then(|| value_and_parameters(component));
        let unshown =
            |window: Option<&ForeignWindow>| window.map_or(windowed, |window| window.hidden);
        let hidden = (unshown(name), unshown(value));
        self.lettered.push(self.document.components.len() - 1);
        let Some(component) = self.document.components.last_mut() else {
            return;
        };
        if let Some(position) = name_position {
            component.name_label_pos = position;
        }
        if let Some(position) = value_position {
            component.value_label_pos = position;
        }
        // A value the drawing letters in texts of its own keeps each where
        // the drawing put it, at its own size, while they say what the part
        // has.
        component.value_pieces = pieces
            .into_iter()
            .map(|(text, position, size_percent)| ValuePiece {
                text,
                position,
                size_percent,
                stands_for: stands_for.clone(),
            })
            .collect();
        if value_pieces(component).is_none() {
            component.value_pieces.clear();
        }
        component.display_mode = match hidden {
            (true, true) => ComponentDisplayMode::Hidden,
            (true, false) => ComponentDisplayMode::ValueOnly,
            (false, true) => ComponentDisplayMode::NameOnly,
            (false, false) => component.display_mode,
        };
    }

    /// A part RSpice draws with its own artwork has a body of another size
    /// than the source symbol's, so a name or value the drawing stood
    /// beside the source body can land on RSpice's body or on a
    /// neighbour's. Such a text keeps its side and its line and moves out
    /// along the way it reads to the nearest place where it lands on no
    /// body, no other part's text and no net label. A text the drawing
    /// itself set on another text stays where it was set.
    fn clear_lettering(&mut self) {
        let bodies = self
            .document
            .components
            .iter()
            .filter(|component| component.kind != ComponentType::CellInstance)
            .map(native_body)
            .collect::<Vec<_>>();
        let net_labels = self
            .document
            .net_labels
            .iter()
            .flat_map(net_label_boxes)
            .collect::<Vec<_>>();
        for &index in &self.lettered {
            if self.document.components[index].kind == ComponentType::CellInstance {
                continue;
            }
            for label in [Lettering::Name, Lettering::Value] {
                let pieces = match label {
                    Lettering::Name => 1,
                    Lettering::Value => value_pieces(&self.document.components[index])
                        .map_or(1, <[ValuePiece]>::len),
                };
                for piece in 0..pieces {
                    let components = &self.document.components;
                    let component = &components[index];
                    let Some((position, text, size)) = label.text(component, piece) else {
                        continue;
                    };
                    let origin = component.pos;
                    let Some(placed) = position
                        .placed_text(LayoutPoint::new(origin.x as f32, origin.y as f32), 1.0)
                    else {
                        continue;
                    };
                    let own = text_box(placed, &text, size);
                    if !bodies.iter().any(|body| overlaps(own, *body)) {
                        continue;
                    }
                    let obstacles = bodies
                        .iter()
                        .copied()
                        .chain(net_labels.iter().copied())
                        .chain(components.iter().enumerate().flat_map(|(other, part)| {
                            lettering_boxes(part)
                                .into_iter()
                                .filter(move |(which, _)| {
                                    other != index || *which != (label, piece)
                                })
                                .map(|(_, rect)| rect)
                        }))
                        .collect::<Vec<_>>();
                    // A text may cross its own part's body, and no further:
                    // a text with no clear place beside its part stays by
                    // it rather than wander off past other parts.
                    let body = native_body(component);
                    let reach = (
                        body.max.x - body.min.x + own.max.x - own.min.x + 2.0 * LABEL_CLEARANCE,
                        body.max.y - body.min.y + own.max.y - own.min.y + 2.0 * LABEL_CLEARANCE,
                    );
                    let cleared = clear_of(position, origin, &text, size, &obstacles, reach);
                    label.set(&mut self.document.components[index], piece, cleared);
                }
            }
        }
    }

    fn label_position(
        &self,
        symbol: &ForeignSymbol,
        window: &ForeignWindow,
        origin: Point,
    ) -> LabelPosition {
        let anchor = self.scale_fine(
            symbol
                .position
                .offset(symbol.orientation.apply(window.offset)),
        );
        let reach = symbol.orientation.apply(window.reach);
        let edge = |step: i32| match step.signum() {
            1 => TextEdge::Start,
            -1 => TextEdge::End,
            _ => TextEdge::Center,
        };
        let (horizontal, vertical) = (edge(reach.x), edge(reach.y));
        LabelPosition::Placed {
            offset: anchor - origin,
            horizontal,
            vertical,
        }
    }

    /// The part as a notice names it: its name and symbol (a QSPICE block
    /// may have a name and no symbol).
    fn symbol_label(&self, symbol: &ForeignSymbol) -> String {
        let drawn = symbol.symbol.trim();
        match symbol.attribute("InstName") {
            Some(name) if drawn.is_empty() => name.to_owned(),
            Some(name) => format!("{name} ({drawn})"),
            None if drawn.is_empty() => format!("The symbol on line {}", symbol.line),
            None => format!("Symbol {drawn} on line {}", symbol.line),
        }
    }

    /// A distinct instance name: the drawing's, or the prefix and the next
    /// free number.
    fn instance_name(&mut self, wanted: Option<&str>, prefix: &str) -> String {
        if let Some(name) = wanted.map(str::trim).filter(|name| !name.is_empty())
            && self.names.insert(name.to_ascii_lowercase())
        {
            return name.to_owned();
        }
        let prefix = if prefix.is_empty() { "U" } else { prefix };
        (1..)
            .map(|number| format!("{prefix}{number}"))
            .find(|name| self.names.insert(name.to_ascii_lowercase()))
            .expect("an unbounded range finds a free name")
    }

    fn place_native(
        &mut self,
        symbol: &ForeignSymbol,
        attributes: &[ForeignAttribute],
        mapping: NativeMapping,
        pins: &[Point],
    ) {
        let label = self.symbol_label(symbol);
        let mut value = attribute(attributes, "Value")
            .unwrap_or_default()
            .to_owned();
        let mut words = Vec::new();
        // QSPICE writes a passive's settings after its value in one text
        // (`100µ Rser=.1`): the value is its first word.
        if self.drawing.tool == ForeignTool::Qspice
            && matches!(
                mapping.kind,
                NativeKind::Fixed(
                    ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor
                )
            )
        {
            let tokens = spice_tokens(&spice_text(&value));
            if let Some((first, rest)) = tokens.split_first() {
                value.clone_from(first);
                words.extend(rest.iter().cloned());
            }
        }
        for key in ["Value2", "SpiceLine", "SpiceLine2"] {
            if let Some(text) = attribute(attributes, key).filter(|text| !text.trim().is_empty()) {
                words.extend(spice_tokens(&spice_text(text)));
            }
        }
        let (kind, value, params, extra) = match mapping.kind {
            NativeKind::Fixed(kind) => {
                let mut params = Vec::new();
                let mut extra = Vec::new();
                for word in words {
                    match word.split_once('=') {
                        // `Value2` and `SpiceLine` may both set a field; the
                        // source program writes both and the later one holds.
                        Some((key, value)) if !key.is_empty() && !value.is_empty() => {
                            params.retain(|(written, _): &(String, String)| {
                                !written.eq_ignore_ascii_case(key)
                            });
                            params.push((key.to_owned(), value.to_owned()));
                        }
                        // LTspice writes `x2` for two parts in parallel.
                        _ if parallel_count(&word).is_some() => {
                            params.push(("m".to_owned(), word[1..].to_owned()));
                        }
                        _ => extra.push(word),
                    }
                }
                let value = spice_text(&value);
                // QSPICE reads a passive's value as an expression, braces or
                // not (`2.21/3`, `Rload`); RSpice reads one in braces.
                let value = if self.drawing.tool == ForeignTool::Qspice
                    && matches!(
                        kind,
                        ComponentType::Resistor
                            | ComponentType::Capacitor
                            | ComponentType::Inductor
                    )
                    && !value.is_empty()
                    && !value.starts_with('{')
                    && !is_spice_number(&value)
                {
                    format!("{{{value}}}")
                } else {
                    value
                };
                (kind, value, params, extra)
            }
            NativeKind::VoltageSource | NativeKind::CurrentSource => {
                let voltage = mapping.kind == NativeKind::VoltageSource;
                let mut text = value.clone();
                for word in words {
                    text.push(' ');
                    text.push_str(&word);
                }
                let source = source_value(voltage, &text);
                (source.kind, source.value, source.params, source.extra)
            }
        };
        // Words with no field of their own follow the value on the card, where
        // the source simulator read them.
        let value = if extra.is_empty() {
            value
        } else {
            if kind != ComponentType::BehavioralSource {
                self.notice(format!(
                    "{label}: `{}` has no RSpice field and follows its value on the card.",
                    extra.join(" ")
                ));
            }
            format!("{value} {}", extra.join(" ")).trim().to_owned()
        };
        let prefix = kind.spice_prefix();
        if MODEL_PREFIXES.contains(&prefix)
            && let Some(model) = value.split_whitespace().next()
        {
            self.models.insert(model.to_owned());
        }
        let name = self.instance_name(attribute(attributes, "InstName"), prefix);
        let id = self.id();
        let mut component = Component::new(id, kind, Point::default()).with_name_value(name, value);
        component.params = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join(" ");
        if crate::parameters::validate_parameter_text(&component.params).is_err() {
            self.notice(format!(
                "{label}: its parameters `{}` are kept as written but do not read as name=value pairs.",
                component.params
            ));
        }
        let offsets = |candidate: &Component| -> Vec<Point> {
            let placed = candidate.terminal_positions();
            mapping
                .terminals
                .iter()
                .map(|terminal| {
                    placed
                        .iter()
                        .find(|(name, _)| name == terminal)
                        .map_or(Point::default(), |(_, point)| *point)
                })
                .collect()
        };
        self.place_fitted(&mut component, pins, offsets);
        let placed = component.terminal_positions();
        if let Some((from, to)) = mapping.tie {
            let point = |terminal: &str| {
                placed
                    .iter()
                    .find(|(name, _)| *name == terminal)
                    .map(|(_, point)| *point)
            };
            if let (Some(from), Some(to)) = (point(from), point(to)) {
                self.lead(from, to);
            }
        }
        self.terminals.extend(
            placed
                .into_iter()
                .map(|(name, point)| (id, name.to_owned(), point)),
        );
        self.document.components.push(component);
    }

    /// The functional part an LTspice part becomes, when RSpice carries it
    /// as one; a part that stays LTspice's own device says why.
    fn functional_plan(
        &mut self,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
        attributes: &[ForeignAttribute],
        pins: &[Point],
    ) -> Option<FunctionalPlan> {
        let opamp = symbol.base_name().eq_ignore_ascii_case("UniversalOpAmp2")
            && definition.pins.len() == 5;
        if self.drawing.tool != ForeignTool::Ltspice
            || !opamp
                && !attribute(attributes, "Prefix")
                    .is_some_and(|prefix| prefix.trim().eq_ignore_ascii_case("A"))
        {
            return None;
        }
        let mut words = Vec::new();
        let fallback = attribute(attributes, "SpiceModel")
            .unwrap_or_default()
            .trim();
        let model = match attribute(attributes, "Value").filter(|value| !value.trim().is_empty()) {
            Some(value) => {
                let (model, rest) = split_model(&spice_text(value), fallback);
                words.extend(rest);
                model
            }
            None => fallback.to_owned(),
        };
        for key in ["Value2", "SpiceLine", "SpiceLine2"] {
            if let Some(text) = attribute(attributes, key).filter(|text| !text.trim().is_empty()) {
                words.extend(spice_tokens(&spice_text(text)));
            }
        }
        let nets = &self.source_nets;
        let net = |index: usize| pins.get(index).and_then(|point| nets.net(*point));
        let is_ground = |net: usize| nets.is_ground(net);
        let functional = if opamp {
            ltspice_opamp(&model, &words)
        } else {
            ltspice_gate(&model, &definition.ordered_pins(), &net, &is_ground, &words)?
        };
        match functional {
            Functional::Part(plan) => Some(plan),
            Functional::Kept(reason) => {
                let label = self.symbol_label(symbol);
                self.notice(format!("{label} {reason}."));
                None
            }
        }
    }

    /// Places RSpice's functional part for a source part, its pins drawn to
    /// the source pins the plan names.
    fn place_functional(
        &mut self,
        symbol: &ForeignSymbol,
        attributes: &[ForeignAttribute],
        plan: FunctionalPlan,
        pins: &[Point],
    ) {
        let Some(descriptor) = functional_part(plan.stable_id) else {
            return;
        };
        let id = self.id();
        let mut component = Component::new(id, ComponentType::CellInstance, Point::default())
            .with_library_cell(functional_library_binding(descriptor));
        component.params = plan
            .params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join(" ");
        if let Err(error) = conform_functional_inputs(&mut component) {
            let label = self.symbol_label(symbol);
            self.notice(format!("{label}: {error}."));
        }
        let Some(binding) = component.library_cell.clone() else {
            return;
        };
        let name = self.instance_name(attribute(attributes, "InstName"), "X");
        component.name = name;
        component.value = placement_value(&binding);
        let targets = plan
            .pins
            .iter()
            .map(|index| pins[*index])
            .collect::<Vec<_>>();
        let offsets = |candidate: &Component| -> Vec<Point> {
            candidate
                .terminal_positions()
                .into_iter()
                .map(|(_, point)| point)
                .collect()
        };
        let points = self.place_fitted(&mut component, &targets, offsets);
        for (terminal, point) in binding.terminal_order.iter().zip(points) {
            self.terminals.push((id, terminal.clone(), point));
        }
        self.document.components.push(component);
    }

    fn place_cell(
        &mut self,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
        attributes: &[ForeignAttribute],
        location: Option<String>,
        schematic: Option<PathBuf>,
        pins: &[Point],
    ) {
        let label = self.symbol_label(symbol);
        let prefix = attribute(attributes, "Prefix")
            .map(str::trim)
            .filter(|prefix| !prefix.is_empty())
            .unwrap_or("X")
            .to_ascii_uppercase();
        let prefix = if prefix
            .chars()
            .all(|character| character.is_ascii_alphabetic())
        {
            prefix
        } else {
            self.notice(format!(
                "{label}: its device prefix `{prefix}` is not a SPICE letter; it is placed as a subcircuit."
            ));
            "X".to_owned()
        };
        let subcircuit = schematic.is_some();
        // A subcircuit's `SpiceModel`, when it names a model and not a file,
        // is the model its line writes, ahead of `Value` and `Value2`.
        let spice_model = attribute(attributes, "SpiceModel")
            .map(str::trim)
            .filter(|model| prefix == "X" && !model.is_empty() && !looks_like_file(model));
        // With no value, or a value of settings alone, the symbol's own
        // model is the one its line writes.
        let fallback = attribute(attributes, "SpiceModel")
            .map(str::trim)
            .filter(|model| !model.is_empty() && !looks_like_file(model))
            .unwrap_or(symbol.base_name());
        let written = (!subcircuit).then(|| match spice_model {
            Some(model) => ["Value", "Value2"]
                .into_iter()
                .filter_map(|key| attribute(attributes, key))
                .filter(|text| !text.trim().is_empty())
                .fold(model.to_owned(), |line, text| {
                    format!("{line} {}", spice_text(text))
                }),
            None => attribute(attributes, "Value")
                .filter(|value| !value.trim().is_empty())
                .or_else(|| attribute(attributes, "Value2"))
                .map(spice_text)
                .unwrap_or_else(|| fallback.to_owned()),
        });
        let mut words = Vec::new();
        for key in ["SpiceLine", "SpiceLine2"] {
            if let Some(text) = attribute(attributes, key).filter(|text| !text.trim().is_empty()) {
                words.extend(spice_tokens(&spice_text(text)));
            }
        }
        if !subcircuit
            && prefix != "X"
            && let Some(value2) = attribute(attributes, "Value2").filter(|value2| {
                !value2.trim().is_empty() && attribute(attributes, "Value").is_some()
            })
        {
            words.extend(spice_tokens(&spice_text(value2)));
        }
        let (model, model_words) = written
            .as_deref()
            .map(|text| split_model(text, fallback))
            .unwrap_or_default();
        words.splice(0..0, model_words);
        // One cell per model: a cell's `spice` view runs one model.
        // LTspice's A-devices write all eight places of their line, and
        // ground each place nothing reaches.
        let device = prefix == "A" && !subcircuit && self.drawing.tool == ForeignTool::Ltspice;
        let cell_index = self.symbol_cell(
            symbol,
            definition,
            location,
            schematic.clone(),
            (!subcircuit).then_some(model.as_str()),
            device,
        );
        let cell = self.cells[cell_index].clone();
        let model = if subcircuit { cell.cell.clone() } else { model };
        let params = words
            .iter()
            .filter(|word| word.contains('='))
            .cloned()
            .collect::<Vec<_>>();
        let loose = words
            .iter()
            .filter(|word| !word.contains('='))
            .cloned()
            .collect::<Vec<_>>();
        let mut binding = LibraryCellInstance::new(self.library, &cell.cell, "symbol");
        binding.terminal_order = cell.ports.iter().map(|port| port.name.clone()).collect();
        binding.terminal_dirs = cell.ports.iter().map(|port| port.direction).collect();
        binding.module_name = Some(model.clone());
        binding.reference_prefix = Some(prefix.clone());
        if prefix != "X" {
            binding.netlist_template =
                Some(format!("{prefix}{{name}} {{nodes}} {{model}} {{params}}"));
            self.cells[cell_index]
                .template
                .clone_from(&binding.netlist_template);
            binding.parameter_order = params
                .iter()
                .filter_map(|word| word.split_once('=').map(|(key, _)| key.to_owned()))
                .collect();
        }
        if !subcircuit {
            for key in ["ModelFile", "SpiceModel"] {
                let Some(file) = attribute(attributes, key).map(str::trim) else {
                    continue;
                };
                if file.is_empty() || !looks_like_file(file) {
                    continue;
                }
                match self.access.model_file(file) {
                    Some(found) if found.encrypted => {
                        self.refuse(format!(
                            "{label}: its model file `{file}` is encrypted, so RSpice cannot read model `{model}`; the model is unresolved."
                        ));
                    }
                    Some(found) => {
                        self.cells[cell_index].source.get_or_insert(ForeignModelSource {
                            path: found.path.clone(),
                            section: None,
                        });
                        binding.source_path = Some(found.path);
                    }
                    None => self.refuse(format!(
                        "{label}: its model file `{file}` was not found, so model `{model}` is unresolved."
                    )),
                }
                break;
            }
            // A subcircuit the drawing's own model text or included files
            // define is referenced by name.
            if prefix == "X" && binding.source_path.is_none() {
                binding.netlist_template = Some("X{name} {nodes} {model} {params}".to_owned());
                binding.parameter_order = params
                    .iter()
                    .filter_map(|word| word.split_once('=').map(|(key, _)| key.to_owned()))
                    .collect();
            }
        }
        if !loose.is_empty() {
            self.notice(format!(
                "{label}: `{}` follows the model on its instance line and is kept there.",
                loose.join(" ")
            ));
        }
        let wanted = attribute(attributes, "InstName").map(str::to_owned);
        let wanted = match wanted {
            // A device letter other than X names the card itself.
            Some(name) if prefix != "X" && !name.to_ascii_uppercase().starts_with(&prefix) => {
                Some(format!("{prefix}{name}"))
            }
            other => other,
        };
        let name = self.instance_name(wanted.as_deref(), &prefix);
        let id = self.id();
        let mut component = Component::new(id, ComponentType::CellInstance, Point::default())
            .with_name_value(name, model)
            .with_library_cell(binding);
        component.params = params.join(" ");
        if !loose.is_empty() {
            component.params = format!("{} {}", component.params, loose.join(" "))
                .trim()
                .to_owned();
        }
        let drawn = |name: &String| !cell.unconnected.contains(name);
        let local = cell
            .symbol
            .pins
            .iter()
            .filter(|pin| drawn(&pin.name))
            .map(|pin| pin.position.unwrap_or_default())
            .collect::<Vec<_>>();
        let offsets = |candidate: &Component| -> Vec<Point> {
            local
                .iter()
                .map(|point| candidate.transform_point(*point))
                .collect()
        };
        let points = self.place_fitted(&mut component, pins, offsets);
        if device {
            self.tie_device(&cell, &component, pins, &points);
        }
        if self.drawing.tool == ForeignTool::Qspice {
            let body = Body::of(&cell.symbol, &component, &points);
            self.tie_default_nets(symbol, definition, body, pins, &points);
        }
        for (port, point) in cell
            .ports
            .iter()
            .filter(|port| drawn(&port.name))
            .zip(points)
        {
            self.terminals.push((id, port.name.clone(), point));
        }
        self.document.components.push(component);
    }

    /// Records the terminals of an LTspice A-device that are ground when
    /// nothing reaches them: each drawn pin, and the places its symbol gives
    /// no pin when they do not share its common's.
    fn tie_device(
        &mut self,
        cell: &ForeignSymbolCell,
        component: &Component,
        sources: &[Point],
        terminals: &[Point],
    ) {
        let body = Body::of(&cell.symbol, component, terminals);
        for (source, terminal) in sources.iter().zip(terminals) {
            self.device_ties.push(DeviceTie {
                source: Some(*source),
                terminal: *terminal,
                body,
                net: None,
            });
        }
        let placeholder = |pin: &&SymbolPin| cell.unconnected.contains(&pin.name);
        let common = cell
            .symbol
            .pins
            .get(7)
            .filter(|pin| !placeholder(pin))
            .and_then(|pin| pin.position);
        let spare = cell
            .symbol
            .pins
            .iter()
            .filter(placeholder)
            .filter_map(|pin| pin.position)
            .find(|position| Some(*position) != common);
        if let Some(position) = spare {
            self.device_ties.push(DeviceTie {
                source: None,
                terminal: component.pos + component.transform_point(position),
                body,
                net: None,
            });
        }
    }

    /// Records the pins of a QSPICE part that join a net of their own
    /// (`Vdd`, `GND`) when nothing is wired to them, as QSPICE connects them.
    fn tie_default_nets(
        &mut self,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
        body: Body,
        sources: &[Point],
        terminals: &[Point],
    ) {
        for pin in definition.ordered_pins() {
            let Some(net) = &pin.default_net else {
                continue;
            };
            let source = self.pin_source(symbol, pin);
            let Some(terminal) = sources
                .iter()
                .position(|point| *point == source)
                .and_then(|index| terminals.get(index))
            else {
                continue;
            };
            let ground = net == "0" || net.eq_ignore_ascii_case("GND");
            self.device_ties.push(DeviceTie {
                source: Some(source),
                terminal: *terminal,
                body,
                net: (!ground).then(|| net.clone()),
            });
        }
    }

    /// Where a symbol's pin lies on the drawing, scaled to the sheet.
    fn pin_source(&self, symbol: &ForeignSymbol, pin: &ForeignPin) -> Point {
        self.scale(
            symbol
                .position
                .offset(symbol.orientation.apply(pin.position)),
        )
    }

    /// Whether a wire, a label or another pin reaches the drawing's point.
    fn reached(&self, point: Point) -> bool {
        self.source_nets.net(point).is_some()
            || self.pin_points.get(&point).copied().unwrap_or_default() > 1
    }

    /// Grounds each recorded A-device terminal whose source pin nothing
    /// reaches, as LTspice writes node `0` there: a ground symbol on the
    /// terminal, facing away from the part.
    fn tie_device_pins(&mut self) {
        for tie in std::mem::take(&mut self.device_ties) {
            if tie.source.is_some_and(|source| self.reached(source)) {
                continue;
            }
            if let Some(net) = tie.net {
                let id = self.id();
                let mut label = NetLabel::new(id, tie.terminal, net.clone());
                if is_supply_rail(&net) {
                    label.kind = NetLabelKind::Supply {
                        glyph: SupplyGlyph::for_name(&net),
                    };
                }
                self.document.net_labels.push(label);
                continue;
            }
            let rotation = tie.body.outward(tie.terminal);
            let id = self.id();
            let mut ground = Component::new(id, ComponentType::Ground, Point::default());
            ground.rotation = rotation;
            ground.name = format!("GND{id}");
            let offset = ground
                .terminal_positions()
                .into_iter()
                .next()
                .map(|(_, point)| point)
                .unwrap_or_default();
            ground.pos = tie.terminal - offset;
            self.terminals.extend(
                ground
                    .terminal_positions()
                    .into_iter()
                    .map(|(name, point)| (id, name.to_owned(), point)),
            );
            self.document.components.push(ground);
        }
    }

    /// The project cell for a symbol, made the first time the symbol is
    /// placed.
    fn symbol_cell(
        &mut self,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
        location: Option<String>,
        schematic: Option<PathBuf>,
        model: Option<&str>,
        device: bool,
    ) -> usize {
        // QSPICE embeds a symbol per component, so two components under one
        // name may differ; their drawings decide.
        let key = format!(
            "{}|{:?}|{}",
            symbol.symbol.to_ascii_lowercase(),
            definition.pins,
            model.unwrap_or_default().to_ascii_lowercase()
        );
        if let Some(index) = self.cell_index.get(&key) {
            return *index;
        }
        let mut name = cell_name(&symbol.symbol);
        let mut suffix = 2;
        while self
            .cells
            .iter()
            .any(|cell| cell.cell.eq_ignore_ascii_case(&name))
        {
            name = format!("{}_{suffix}", cell_name(&symbol.symbol));
            suffix += 1;
        }
        // A hierarchical block's ports are its schematic's; only a symbol
        // over a model leaves places of its instance line without a pin.
        let mut slots = if schematic.is_none() && self.drawing.tool == ForeignTool::Ltspice {
            spice_order_slots(definition)
        } else {
            definition.ordered_pins().into_iter().map(Some).collect()
        };
        // An A-device's places without a pin share its common's, which
        // LTspice's own symbols all draw and its circuits ground.
        let mut shared = None;
        if device {
            slots.resize(slots.len().max(DEVICE_PLACES), None);
            shared = slots
                .get(DEVICE_PLACES - 1)
                .copied()
                .flatten()
                .map(|pin| self.scale(pin.position));
        }
        let ports = unique_pin_names(&slots);
        let drawn = slots
            .iter()
            .flatten()
            .map(|pin| self.scale(pin.position))
            .collect::<Vec<_>>();
        let mut spare = unconnected_pin_points(&drawn);
        let mut unconnected = Vec::new();
        let mut gaps = Vec::new();
        let mut pins = Vec::new();
        for ((order, slot), port) in (1..).zip(&slots).zip(&ports) {
            let position = match slot {
                Some(pin) => self.scale(pin.position),
                None => {
                    unconnected.push(port.name.clone());
                    gaps.push(order.to_string());
                    if device && shared.is_none() {
                        shared = spare.next();
                    }
                    shared.or_else(|| spare.next()).unwrap_or_default()
                }
            };
            // A place the symbol draws no pin for letters no name either:
            // several share one point. Nor does a pin the symbol names
            // nothing: the port's name is made up for its place.
            let name_offset = slot
                .and_then(|pin| pin.name_offset)
                .map(|offset| self.scale_fine(ForeignPoint::new(offset, 0)).x);
            pins.push(
                SymbolPin::new(port.name.clone(), port.direction, Some(position))
                    .with_name_hidden(
                        slot.is_none_or(|pin| pin.name_hidden || pin.name.trim().is_empty()),
                    )
                    .with_name_offset(name_offset)
                    .with_name_at(slot.and_then(|pin| pin.name_at).map(|(offset, reach)| {
                        PinNamePlacement {
                            offset: self.scale_fine(offset),
                            reach: Point::new(reach.x, reach.y),
                        }
                    })),
            );
        }
        if !gaps.is_empty() && !device {
            self.notice(format!(
                "{}: its symbol has no pin for place {} of the instance line; RSpice leaves an unconnected terminal there, as LTspice does, so the model's nodes keep their order.",
                symbol.symbol,
                gaps.join(", ")
            ));
        }
        let mut document = SymbolDocument {
            pins,
            body: definition
                .shapes
                .iter()
                .filter_map(|shape| self.symbol_shape(shape))
                .collect(),
            ..SymbolDocument::default()
        };
        let anchor = |attribute: u32| {
            definition
                .windows
                .iter()
                .find(|window| window.attribute == attribute)
                .map(|window| self.scale_fine(window.offset))
        };
        if let Some(point) = anchor(0) {
            document.name_anchor = point;
        }
        if let Some(point) = anchor(3) {
            document.value_anchor = point;
        }
        self.cells.push(ForeignSymbolCell {
            cell: name,
            symbol: document,
            ports,
            source_symbol: symbol.symbol.clone(),
            location,
            schematic,
            model: model.map(str::to_owned),
            template: None,
            source: None,
            unconnected,
        });
        let index = self.cells.len() - 1;
        self.cell_index.insert(key, index);
        index
    }

    fn symbol_shape(&self, shape: &ForeignShape) -> Option<SymbolShape> {
        Some(match shape {
            ForeignShape::Line(a, b) => SymbolShape::Polyline {
                points: vec![self.scale_fine(*a), self.scale_fine(*b)],
                closed: false,
            },
            ForeignShape::Rectangle(a, b) => {
                let (a, b) = (self.scale_fine(*a), self.scale_fine(*b));
                SymbolShape::Polyline {
                    points: vec![a, Point::new(b.x, a.y), b, Point::new(a.x, b.y)],
                    closed: true,
                }
            }
            ForeignShape::Ellipse(a, b) => {
                let (a, b) = (self.scale_fine(*a), self.scale_fine(*b));
                let (width, height) = ((b.x - a.x).abs(), (b.y - a.y).abs());
                if width == height {
                    SymbolShape::Circle {
                        center: Point::new((a.x + b.x) / 2, (a.y + b.y) / 2),
                        radius: (width / 2).max(1),
                    }
                } else {
                    SymbolShape::Polyline {
                        points: ellipse_points(a, b, 0.0, std::f64::consts::TAU, 24),
                        closed: true,
                    }
                }
            }
            ForeignShape::Arc([a, b, start, end]) => {
                let (from, sweep) = arc_angles(
                    self.scale_fine(*a),
                    self.scale_fine(*b),
                    self.scale_fine(*start),
                    self.scale_fine(*end),
                )?;
                SymbolShape::Polyline {
                    points: ellipse_points(
                        self.scale_fine(*a),
                        self.scale_fine(*b),
                        from,
                        sweep,
                        16,
                    ),
                    closed: false,
                }
            }
        })
    }

    /// A box marking where a symbol with no known pins stood.
    fn place_unknown_box(&mut self, symbol: &ForeignSymbol) {
        let definition = ForeignSymbolDefinition {
            symbol_type: "BLOCK".to_owned(),
            pins: Vec::new(),
            shapes: vec![ForeignShape::Rectangle(
                ForeignPoint::new(0, 0),
                ForeignPoint::new(
                    self.drawing.tool.units_per_grid() * 4,
                    self.drawing.tool.units_per_grid() * 4,
                ),
            )],
            attributes: Vec::new(),
            windows: Vec::new(),
        };
        let attributes = merged_attributes(&definition, symbol);
        self.place_cell(symbol, &definition, &attributes, None, None, &[]);
    }

    /// A note where a refused component stood, so the gap reads as one: it
    /// hangs from the top left of the body, its label under the words, so it
    /// stays on the body's own ground rather than across its neighbours.
    /// It marks no point, so it draws no anchor mark: a mark at the corner
    /// would touch a flag standing on the refused part's pin.
    fn place_marker(
        &mut self,
        symbol: &ForeignSymbol,
        definition: Option<&ForeignSymbolDefinition>,
        label: &str,
    ) {
        let id = self.id();
        let corner = definition
            .into_iter()
            .flat_map(|definition| {
                let shapes = definition.shapes.iter().flat_map(|shape| match shape {
                    ForeignShape::Line(a, b)
                    | ForeignShape::Rectangle(a, b)
                    | ForeignShape::Ellipse(a, b) => vec![*a, *b],
                    // An arc's box is its whole ellipse's, often wider than
                    // the body.
                    ForeignShape::Arc(_) => Vec::new(),
                });
                shapes.chain(definition.pins.iter().map(|pin| pin.position))
            })
            .map(|point| symbol.orientation.apply(point))
            .reduce(|a, b| ForeignPoint::new(a.x.min(b.x), a.y.min(b.y)))
            .unwrap_or_default();
        let text = format!("Not imported:\n{label}");
        if let Ok(note) = DesignNote::new(
            id,
            self.scale(symbol.position.offset(corner)),
            DesignNoteKind::PlainText,
            text,
        ) {
            self.document
                .design_notes
                .push(note.with_anchor_mark_hidden(true));
        }
    }

    /// A straight or once-bent lead from a placed terminal to the source pin
    /// it stands for.
    fn lead(&mut self, from: Point, to: Point) {
        let route = lead_routes(from, to).into_iter().next().unwrap_or_default();
        self.push_leads(&route);
    }

    fn push_leads(&mut self, leads: &[(Point, Point)]) {
        for pair in leads.windows(2) {
            // A bend joins its two legs.
            if pair[0].1 == pair[1].0 {
                self.anchors.insert(pair[0].1);
            }
        }
        self.leads.extend_from_slice(leads);
    }

    fn collect_source_geometry(&mut self, joins: &[(Point, Point)]) {
        let drawing = self.drawing;
        for (a, b) in drawing.wires.iter().chain(&drawing.bus_taps) {
            let (a, b) = (self.scale(*a), self.scale(*b));
            self.source_points.extend([a, b]);
            self.source_segments.push((a, b));
        }
        for flag in &drawing.flags {
            self.source_points.insert(self.scale(flag.position));
        }
        for point in &drawing.junctions {
            self.source_points.insert(self.scale(*point));
        }
        let flags = drawing
            .flags
            .iter()
            .map(|flag| {
                let name = if flag.is_ground() {
                    "0"
                } else {
                    flag.name.as_str()
                };
                (self.scale(flag.position), name)
            })
            .collect::<Vec<_>>();
        let segments = self
            .source_segments
            .iter()
            .chain(joins)
            .copied()
            .collect::<Vec<_>>();
        self.source_nets = SourceNets::new(&segments, &flags);
    }

    /// Places `component` where its terminals best meet the source `pins`
    /// and draws a lead from each terminal to its pin, returning the
    /// terminal points in pin order. The cheapest placement whose leads all
    /// run clear of every other pin, wire and lead wins, so a lead joins
    /// nothing but its own pin; when none runs clear, the cheapest placement
    /// keeps its plain leads.
    fn place_fitted(
        &mut self,
        component: &mut Component,
        pins: &[Point],
        offsets: impl Fn(&Component) -> Vec<Point>,
    ) -> Vec<Point> {
        let candidates = placements(component, pins, &offsets);
        // Leads bent at most once first; a lead that must cross another's
        // path jogs around it only when no placement avoids that.
        let clear = [false, true].into_iter().find_map(|detour| {
            candidates
                .iter()
                .find_map(|(position, rotation, mirrored, local)| {
                    let terminals = local
                        .iter()
                        .map(|offset| *position + *offset)
                        .collect::<Vec<_>>();
                    self.clear_leads(&terminals, pins, detour)
                        .map(|leads| (*position, *rotation, *mirrored, terminals, leads))
                })
        });
        let (position, rotation, mirrored, terminals, leads) = clear.unwrap_or_else(|| {
            let (position, rotation, mirrored, local) =
                candidates.into_iter().next().unwrap_or_else(|| {
                    let mut upright = component.clone();
                    upright.pos = Point::default();
                    upright.rotation = Rotation::R0;
                    upright.mirror_h = false;
                    upright.mirror_v = false;
                    let anchor = pins.first().copied().unwrap_or_default();
                    (anchor, Rotation::R0, false, offsets(&upright))
                });
            let terminals = local
                .iter()
                .map(|offset| position + *offset)
                .collect::<Vec<_>>();
            // Each lead still takes a clear route where it has one.
            let mut leads = Vec::new();
            for (from, to) in terminals.iter().zip(pins) {
                let others = terminals
                    .iter()
                    .chain(pins)
                    .copied()
                    .filter(|point| point != from && point != to)
                    .collect::<Vec<_>>();
                let mut routes = lead_routes(*from, *to);
                routes.extend(detour_routes(*from, *to));
                let route = routes
                    .iter()
                    .find(|route| self.route_is_clear(route, *from, *to, &others, &leads))
                    .or(routes.first())
                    .cloned()
                    .unwrap_or_default();
                leads.extend(route);
            }
            (position, rotation, mirrored, terminals, leads)
        });
        component.pos = position;
        component.rotation = rotation;
        component.mirror_h = mirrored;
        self.push_leads(&leads);
        terminals
    }

    /// Leads from each terminal to its pin that touch no other pin,
    /// terminal, joining point, wire or lead, or `None` when one cannot.
    fn clear_leads(
        &self,
        terminals: &[Point],
        pins: &[Point],
        detour: bool,
    ) -> Option<Vec<(Point, Point)>> {
        let mut routed: Vec<(Point, Point)> = Vec::new();
        for (from, to) in terminals.iter().zip(pins) {
            let others = terminals
                .iter()
                .chain(pins)
                .copied()
                .filter(|point| point != from && point != to)
                .collect::<Vec<_>>();
            if from == to {
                if others.contains(from) {
                    return None;
                }
                continue;
            }
            let mut routes = lead_routes(*from, *to);
            if detour {
                routes.extend(detour_routes(*from, *to));
            }
            let route = routes
                .into_iter()
                .find(|route| self.route_is_clear(route, *from, *to, &others, &routed))?;
            routed.extend(route);
        }
        Some(routed)
    }

    fn route_is_clear(
        &self,
        route: &[(Point, Point)],
        from: Point,
        to: Point,
        others: &[Point],
        routed: &[(Point, Point)],
    ) -> bool {
        let touches_point = self
            .anchors
            .iter()
            .chain(&self.source_points)
            .chain(self.pin_points.keys())
            .chain(self.terminals.iter().map(|(_, _, point)| point))
            .chain(others)
            .filter(|point| **point != from && **point != to)
            .any(|point| route.iter().any(|(a, b)| on_segment(*a, *b, *point)));
        // A wire or lead that reaches the pin itself is already its net.
        let touches_segment = self
            .source_segments
            .iter()
            .chain(&self.leads)
            .chain(routed)
            .filter(|(c, d)| !on_segment(*c, *d, to))
            .any(|(c, d)| {
                on_segment(*c, *d, from)
                    || route.iter().any(|(a, b)| {
                        overlap(*a, *b, *c, *d) || (*b != to && on_segment(*c, *d, *b))
                    })
            });
        !touches_point && !touches_segment
    }

    fn place_flags(&mut self) {
        let mut ports = 0;
        for flag in &self.drawing.flags {
            let at = self.scale(flag.position);
            self.anchors.insert(at);
            let id = self.id();
            if flag.is_ground() {
                let mut ground = Component::new(id, ComponentType::Ground, Point::default());
                ground.rotation = match flag.facing.map(|run| (run.x.signum(), run.y.signum())) {
                    Some((1, _)) => Rotation::R270,
                    Some((-1, _)) => Rotation::R90,
                    Some((0, -1)) => Rotation::R180,
                    _ => Rotation::R0,
                };
                let terminal = ground
                    .terminal_positions()
                    .into_iter()
                    .next()
                    .map(|(_, point)| point)
                    .unwrap_or_default();
                ground.pos = at - terminal;
                ground.name = format!("GND{id}");
                self.terminals.extend(
                    ground
                        .terminal_positions()
                        .into_iter()
                        .map(|(name, point)| (id, name.to_owned(), point)),
                );
                self.document.components.push(ground);
            } else if let Some(direction) = flag.port {
                ports += 1;
                let mut port = Component::new(id, ComponentType::Port, at + Point::new(10, 0));
                port.name = format!("P{ports}");
                port.value = flag.name.clone();
                port.params = format!(
                    "dir={}",
                    match direction {
                        ForeignPortDirection::Input => "in",
                        ForeignPortDirection::Output => "out",
                        ForeignPortDirection::Bidirectional => "inout",
                    }
                );
                self.terminals.extend(
                    port.terminal_positions()
                        .into_iter()
                        .map(|(name, point)| (id, name.to_owned(), point)),
                );
                self.document.components.push(port);
            } else {
                let mut label = NetLabel::new(id, at, flag.name.clone());
                let run = flag.facing.or_else(|| self.away_from_attachment(at));
                label.side = match run.map(|run| (run.x.signum(), run.y.signum())) {
                    Some((-1, _)) => NetLabelSide::Left,
                    Some((0, 1)) => NetLabelSide::Below,
                    Some((0, -1)) => NetLabelSide::Above,
                    _ => NetLabelSide::Right,
                };
                if is_supply_rail(&flag.name) {
                    let reaches = match flag.facing {
                        Some(facing) => vec![ForeignPoint::new(-facing.x, -facing.y)],
                        None => self.reaches(at),
                    };
                    label.kind = NetLabelKind::Supply {
                        glyph: rail_glyph(&flag.name, &reaches),
                    };
                }
                // A side RSpice chose, or a name QSPICE centres over its
                // point, gives way to another where it lands on a body.
                let others = match (run, label.side) {
                    _ if matches!(label.kind, NetLabelKind::Supply { .. }) => Vec::new(),
                    (None, _) => vec![NetLabelSide::Left, NetLabelSide::Above, NetLabelSide::Below],
                    (Some(_), NetLabelSide::Above) if flag.facing.is_some() => {
                        vec![NetLabelSide::Right, NetLabelSide::Left]
                    }
                    _ => Vec::new(),
                };
                if !others.is_empty() {
                    self.free_names
                        .push((self.document.net_labels.len(), others));
                }
                self.document.net_labels.push(label);
            }
        }
    }

    /// Names each net whose label may stand on another side on the first
    /// side where its name lands on no part's body. RSpice draws its own
    /// bodies, and grounds, which can reach into a name the drawing centred
    /// beside them; and a flag on the middle of a wire says nothing of the
    /// side its name reads to.
    fn clear_net_names(&mut self) {
        let natives = self
            .document
            .components
            .iter()
            .filter(|component| component.kind != ComponentType::CellInstance)
            .map(native_body);
        let cells = self.document.components.iter().filter_map(|component| {
            let binding = component.library_cell.as_ref()?;
            let cell = self.cells.iter().find(|cell| cell.cell == binding.cell)?;
            cell.symbol.drawn_body_bounds()?;
            let body = Body::of(&cell.symbol, component, &[]);
            Some(LayoutRect {
                min: LayoutPoint::new(
                    (body.center.x - body.half.x) as f32,
                    (body.center.y - body.half.y) as f32,
                ),
                max: LayoutPoint::new(
                    (body.center.x + body.half.x) as f32,
                    (body.center.y + body.half.y) as f32,
                ),
            })
        });
        let bodies = natives.chain(cells).collect::<Vec<_>>();
        let lands = |label: &NetLabel| {
            net_label_boxes(label)
                .first()
                .is_some_and(|name| bodies.iter().any(|body| overlaps(*name, *body)))
        };
        for (index, sides) in std::mem::take(&mut self.free_names) {
            let Some(label) = self.document.net_labels.get(index) else {
                continue;
            };
            if !lands(label) {
                continue;
            }
            let clear = sides.into_iter().find(|side| {
                let mut moved = label.clone();
                moved.side = *side;
                !lands(&moved)
            });
            if let Some(side) = clear {
                self.document.net_labels[index].side = side;
            }
        }
    }

    /// The way a flag's name reads when the drawing does not say: away from
    /// the one side its wires and pin leads reach it from, as LTspice letters
    /// a flag, so a flag at the foot of a supply pin names the rail below the
    /// pin rather than across the part. A flag reached from more than one
    /// side, or from the middle of a wire, says nothing.
    fn away_from_attachment(&self, at: Point) -> Option<ForeignPoint> {
        let mut sides = self.reaches(at).into_iter();
        let first = sides.next()?;
        sides
            .all(|side| side == first)
            .then(|| ForeignPoint::new(-first.x, -first.y))
    }

    /// The sides wires and pin leads reach `at` from, each a unit step
    /// along the axis it mostly runs on: both ways along a wire `at` stands
    /// on the middle of.
    fn reaches(&self, at: Point) -> Vec<ForeignPoint> {
        let mut reaches = Vec::new();
        for (a, b) in &self.source_segments {
            if *a == at {
                reaches.push(*b - at);
            } else if *b == at {
                reaches.push(*a - at);
            } else if strictly_inside(*a, *b, at) {
                reaches.extend([*a - at, *b - at]);
            }
        }
        reaches.extend(self.pin_leads.get(&at).into_iter().flatten().copied());
        reaches
            .into_iter()
            .filter_map(|reach| {
                match (
                    reach.x.abs() >= reach.y.abs(),
                    reach.x.signum(),
                    reach.y.signum(),
                ) {
                    (true, 0, _) => None,
                    (true, x, _) => Some(ForeignPoint::new(x, 0)),
                    (false, _, y) => Some(ForeignPoint::new(0, y)),
                }
            })
            .collect()
    }

    /// Places a mutual-inductance card (`K1 L1 L2 1`) whose windings are
    /// inductors of the drawing as a coupling part at `position`. Returns
    /// whether it did.
    fn place_coupling(&mut self, position: Point, line: &str) -> bool {
        let words = spice_tokens(line);
        let [name, windings @ .., coupling] = words.as_slice() else {
            return false;
        };
        if !name.to_ascii_uppercase().starts_with('K') || windings.len() < 2 {
            return false;
        }
        let inductors = windings
            .iter()
            .map(|winding| {
                self.document
                    .components
                    .iter()
                    .find(|component| {
                        component.kind == ComponentType::Inductor
                            && component.name.eq_ignore_ascii_case(winding)
                    })
                    .map(|component| component.name.clone())
            })
            .collect::<Option<Vec<_>>>();
        let Some(inductors) = inductors else {
            return false;
        };
        let name = self.instance_name(Some(name), "K");
        let id = self.id();
        let mut component = Component::new(id, ComponentType::CoupledInductor, position)
            .with_name_value(name, coupling.clone());
        component.params = format!("inductors=\"{}\"", inductors.join(" "));
        self.document.components.push(component);
        true
    }

    fn place_texts(&mut self) {
        for text in &self.drawing.texts {
            let position = self.scale(text.position);
            match text.kind {
                ForeignTextKind::Directive => {
                    let lettering = text.lettering;
                    let mut couplings = 0;
                    let text = spice_text_lines(&text.text)
                        .lines()
                        .filter(|line| {
                            let at = position + Point::new(0, 20 * couplings);
                            let placed = self.place_coupling(at, line);
                            couplings += i32::from(placed);
                            !placed
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    if !text.trim().is_empty() {
                        self.directives.push(ForeignDirective {
                            position,
                            text,
                            lettering,
                        });
                    }
                }
                ForeignTextKind::Comment => {
                    let id = self.id();
                    match DesignNote::new(id, position, DesignNoteKind::PlainText, text.text.trim())
                    {
                        Ok(note) => {
                            let note = lettered_note(note, position, &text.lettering);
                            self.document.design_notes.push(note);
                        }
                        Err(error) => self.notice(format!(
                            "The comment on line {} was not placed: {error}.",
                            text.line
                        )),
                    }
                }
            }
        }
        for flag in &self.drawing.data_flags {
            let id = self.id();
            let position = self.scale(flag.position);
            if let Ok(note) = DesignNote::new(
                id,
                position,
                DesignNoteKind::PlainText,
                format!("{} readout: {}", self.drawing.tool.label(), flag.expression),
            ) {
                self.document.design_notes.push(note);
            }
        }
        if !self.drawing.data_flags.is_empty() {
            self.notice(format!(
                "{} waveform readout flag(s) became notes; place a probe to read a value.",
                self.drawing.data_flags.len()
            ));
        }
    }

    fn place_shapes(&mut self) {
        let mut skipped = 0;
        for shape in &self.drawing.shapes {
            let geometry = match shape {
                ForeignShape::Line(a, b) => DocumentationShapeGeometry::Line {
                    start: self.scale_fine(*a),
                    end: self.scale_fine(*b),
                },
                ForeignShape::Rectangle(a, b) => DocumentationShapeGeometry::Rectangle {
                    first: self.scale_fine(*a),
                    opposite: self.scale_fine(*b),
                },
                ForeignShape::Ellipse(a, b) => DocumentationShapeGeometry::Polygon {
                    points: ellipse_points(
                        self.scale_fine(*a),
                        self.scale_fine(*b),
                        0.0,
                        std::f64::consts::TAU,
                        32,
                    ),
                },
                ForeignShape::Arc([a, b, start, end]) => {
                    let (a, b) = (self.scale_fine(*a), self.scale_fine(*b));
                    match arc_angles(a, b, self.scale_fine(*start), self.scale_fine(*end)) {
                        Some((from, sweep)) => {
                            let points = ellipse_points(a, b, from, sweep, 2);
                            DocumentationShapeGeometry::Arc {
                                start: points[0],
                                through: points[1],
                                end: points[2],
                            }
                        }
                        None => {
                            skipped += 1;
                            continue;
                        }
                    }
                }
            };
            let id = self.id();
            match DocumentationShape::new(id, geometry) {
                Ok(shape) => self.document.documentation_shapes.push(shape),
                Err(_) => skipped += 1,
            }
        }
        if skipped > 0 {
            self.notice(format!(
                "{skipped} drawing shape(s) of zero size were left out."
            ));
        }
    }

    fn place_wires(&mut self) {
        let mut segments = Vec::new();
        for (a, b) in &self.drawing.wires {
            let (a, b) = (self.scale(*a), self.scale(*b));
            self.anchors.insert(a);
            self.anchors.insert(b);
            segments.push((a, b));
        }
        for (a, b) in &self.drawing.bus_taps {
            let (a, b) = (self.scale(*a), self.scale(*b));
            self.anchors.insert(a);
            self.anchors.insert(b);
            segments.push((a, b));
        }
        if !self.drawing.bus_taps.is_empty() {
            self.notice(format!(
                "{} bus tap(s) became plain wires; RSpice buses are declared, not drawn.",
                self.drawing.bus_taps.len()
            ));
        }
        for point in &self.drawing.junctions {
            let point = self.scale(*point);
            self.anchors.insert(point);
        }
        segments.extend(self.leads.iter().copied());
        // A wire end, a pin or a flag on the middle of a wire joins it there,
        // as the source simulator reads the drawing; RSpice joins wires at
        // their ends, so the wire is cut at that point.
        let mut pieces = BTreeSet::new();
        for (a, b) in segments {
            if a == b {
                continue;
            }
            let mut cuts = self
                .anchors
                .iter()
                .copied()
                .filter(|point| strictly_inside(a, b, *point))
                .collect::<Vec<_>>();
            cuts.sort_by_key(|point| {
                (i64::from(point.x - a.x)).abs() + (i64::from(point.y - a.y)).abs()
            });
            let mut from = a;
            for cut in cuts.into_iter().chain(std::iter::once(b)) {
                let (from_key, cut_key) = ((from.x, from.y), (cut.x, cut.y));
                pieces.insert(if from_key <= cut_key {
                    (from_key, cut_key)
                } else {
                    (cut_key, from_key)
                });
                from = cut;
            }
        }
        let mut ends = BTreeMap::<(i32, i32), usize>::new();
        for (a, b) in &pieces {
            *ends.entry(*a).or_default() += 1;
            *ends.entry(*b).or_default() += 1;
        }
        for (_, _, point) in &self.terminals {
            if let Some(count) = ends.get_mut(&(point.x, point.y)) {
                *count += 1;
            }
        }
        for (a, b) in pieces {
            let id = self.id();
            self.document.wires.push(Wire::segment(
                id,
                Point::new(a.0, a.1),
                Point::new(b.0, b.1),
            ));
        }
        for ((x, y), count) in ends {
            if count >= 3 {
                let id = self.id();
                self.document
                    .junctions
                    .push(Junction::new(id, Point::new(x, y)));
            }
        }
    }

    fn finish(mut self) -> ForeignImport {
        rebuild_connections_from_terminals(&mut self.document, &self.terminals);
        bind_instances_to_sources(&mut self.document, self.library, &self.cells);
        let defined = defined_model_names(&foreign_directive_cards(&self.directives));
        let model_cards = self
            .models
            .iter()
            .filter(|model| !defined.contains(&model.to_ascii_lowercase()))
            .filter_map(|model| self.access.model_card(model))
            .chain(self.behaviours)
            .collect();
        ForeignImport {
            document: self.document,
            cells: self.cells,
            directives: self.directives,
            model_cards,
            notices: self.notices,
        }
    }
}

/// The definition's default attributes under the placed symbol's own.
/// The middle of a symbol's drawn body in its own frame, or of its pins when
/// it draws nothing.
fn body_centre(definition: &ForeignSymbolDefinition) -> ForeignPoint {
    let drawn = definition.shapes.iter().flat_map(|shape| match shape {
        ForeignShape::Line(a, b) | ForeignShape::Rectangle(a, b) | ForeignShape::Ellipse(a, b) => {
            vec![*a, *b]
        }
        ForeignShape::Arc([a, b, ..]) => vec![*a, *b],
    });
    let points = drawn.collect::<Vec<_>>();
    let points = if points.is_empty() {
        definition.pins.iter().map(|pin| pin.position).collect()
    } else {
        points
    };
    let (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) = (
        points.iter().map(|point| point.x).min(),
        points.iter().map(|point| point.x).max(),
        points.iter().map(|point| point.y).min(),
        points.iter().map(|point| point.y).max(),
    ) else {
        return ForeignPoint::default();
    };
    ForeignPoint::new((min_x + max_x) / 2, (min_y + max_y) / 2)
}

/// The box a part RSpice draws natively stands in: its symbol's extent,
/// turned with it, about its position.
fn native_body(component: &Component) -> LayoutRect {
    let (width, height) = component.symbol_dimensions();
    let (width, height) = if component.rotation.is_vertical() {
        (height, width)
    } else {
        (width, height)
    };
    let (x, y) = (component.pos.x as f32, component.pos.y as f32);
    let (half_width, half_height) = (width as f32 * 0.5, height as f32 * 0.5);
    LayoutRect {
        min: LayoutPoint::new(x - half_width, y - half_height),
        max: LayoutPoint::new(x + half_width, y + half_height),
    }
}

/// Which of a part's texts the importer is placing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lettering {
    Name,
    Value,
}

impl Lettering {
    /// The text's placement, its words and its size; for a value lettered
    /// in pieces, the piece at `piece`. None for a text the part does not
    /// show or a drawing did not place.
    fn text(self, component: &Component, piece: usize) -> Option<(LabelPosition, String, f32)> {
        let shown = SchematicParameterLabelVisibility::NamesAndValues;
        let (position, text, size) = match self {
            Self::Name if component.display_mode.show_name(shown) => (
                component.name_label_pos,
                component.name.clone(),
                INSTANCE_NAME_WORLD,
            ),
            Self::Value if component.display_mode.show_value(shown) => {
                match value_pieces(component) {
                    Some(pieces) => {
                        let piece = pieces.get(piece)?;
                        (piece.position, piece.text.clone(), piece.size_world())
                    }
                    None => (
                        component.value_label_pos,
                        component_value_label(component).into_owned(),
                        INSTANCE_VALUE_WORLD,
                    ),
                }
            }
            _ => return None,
        };
        (matches!(position, LabelPosition::Placed { .. }) && !text.trim().is_empty())
            .then_some((position, text, size))
    }

    fn set(self, component: &mut Component, piece: usize, position: LabelPosition) {
        match self {
            Self::Name => component.name_label_pos = position,
            Self::Value if value_pieces(component).is_some() => {
                if let Some(piece) = component.value_pieces.get_mut(piece) {
                    piece.position = position;
                }
            }
            Self::Value => component.value_label_pos = position,
        }
    }
}

/// The boxes a part's shown name and value take where they are placed,
/// each with which text it is.
fn lettering_boxes(component: &Component) -> Vec<((Lettering, usize), LayoutRect)> {
    let origin = LayoutPoint::new(component.pos.x as f32, component.pos.y as f32);
    let layout = instance_label_layout(origin, 1.0, component);
    let shown = SchematicParameterLabelVisibility::NamesAndValues;
    let mut boxes = Vec::new();
    if component.kind == ComponentType::Ground {
        return boxes;
    }
    if component.display_mode.show_name(shown) && !component.name.trim().is_empty() {
        boxes.push((
            (Lettering::Name, 0),
            text_box(layout.name, &component.name, INSTANCE_NAME_WORLD),
        ));
    }
    if !component.display_mode.show_value(shown) {
        return boxes;
    }
    match value_pieces(component) {
        Some(pieces) => boxes.extend(pieces.iter().enumerate().filter_map(|(index, piece)| {
            let placed = piece.position.placed_text(origin, 1.0)?;
            Some((
                (Lettering::Value, index),
                text_box(placed, &piece.text, piece.size_world()),
            ))
        })),
        None => {
            let value = component_value_label(component);
            if !value.trim().is_empty() {
                boxes.push((
                    (Lettering::Value, 0),
                    text_box(layout.value, &value, INSTANCE_VALUE_WORLD),
                ));
            }
        }
    }
    boxes
}

/// The boxes a net label's name, connector tab and supply symbol take.
fn net_label_boxes(label: &NetLabel) -> Vec<LayoutRect> {
    let layout = net_label_layout(label);
    let name = text_box(
        PlacedText {
            anchor: layout.corner,
            horizontal: TextEdge::Start,
            vertical: TextEdge::End,
        },
        layout.name,
        NET_LABEL_FONT_WORLD,
    );
    [
        Some(name),
        layout.tab.map(|(tab, _)| tab),
        supply_glyph_rect(label),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn overlaps(a: LayoutRect, b: LayoutRect) -> bool {
    a.min.x < b.max.x && b.min.x < a.max.x && a.min.y < b.max.y && b.min.y < a.max.y
}

/// A placed text moved along an axis its box reaches out on to the
/// nearest place where it stands [`LABEL_CLEARANCE`] clear of every
/// obstacle it would land on: the shortest such move, or the shortest of
/// all four for a text centred on its anchor. A text with no such place
/// within `reach`, across and down, stays where it was.
fn clear_of(
    position: LabelPosition,
    origin: Point,
    text: &str,
    size: f32,
    obstacles: &[LayoutRect],
    reach: (f32, f32),
) -> LabelPosition {
    let LabelPosition::Placed {
        offset,
        horizontal,
        vertical,
    } = position
    else {
        return position;
    };
    let Some(placed) =
        position.placed_text(LayoutPoint::new(origin.x as f32, origin.y as f32), 1.0)
    else {
        return position;
    };
    let text = text_box(placed, text, size);
    let across = match horizontal {
        TextEdge::Start => vec![Point::new(1, 0)],
        TextEdge::End => vec![Point::new(-1, 0)],
        TextEdge::Center => Vec::new(),
    };
    let along = match vertical {
        TextEdge::Start => vec![Point::new(0, 1)],
        TextEdge::End => vec![Point::new(0, -1)],
        TextEdge::Center => Vec::new(),
    };
    let directions = if across.is_empty() && along.is_empty() {
        vec![
            Point::new(1, 0),
            Point::new(-1, 0),
            Point::new(0, 1),
            Point::new(0, -1),
        ]
    } else {
        across.into_iter().chain(along).collect()
    };
    // How far the text must move along `direction` to stand clear of
    // `obstacle` beyond it.
    let past = |direction: Point, obstacle: &LayoutRect| match (direction.x, direction.y) {
        (1, _) => (obstacle.max.x + LABEL_CLEARANCE - text.min.x).ceil(),
        (-1, _) => (text.max.x - obstacle.min.x + LABEL_CLEARANCE).ceil(),
        (_, 1) => (obstacle.max.y + LABEL_CLEARANCE - text.min.y).ceil(),
        _ => (text.max.y - obstacle.min.y + LABEL_CLEARANCE).ceil(),
    };
    let moved = |direction: Point, distance: f32| LayoutRect {
        min: LayoutPoint::new(
            text.min.x + direction.x as f32 * distance,
            text.min.y + direction.y as f32 * distance,
        ),
        max: LayoutPoint::new(
            text.max.x + direction.x as f32 * distance,
            text.max.y + direction.y as f32 * distance,
        ),
    };
    let limit = |direction: Point| if direction.x == 0 { reach.1 } else { reach.0 };
    let clear_along = |direction: Point| {
        let mut distance = 0.0_f32;
        for _ in 0..=obstacles.len() {
            if distance > limit(direction) {
                return None;
            }
            let landed = moved(direction, distance);
            match obstacles
                .iter()
                .find(|obstacle| overlaps(landed, **obstacle))
            {
                None => return Some(distance),
                Some(obstacle) => distance = distance.max(past(direction, obstacle)),
            }
        }
        None
    };
    let Some((direction, distance)) = directions
        .into_iter()
        .filter_map(|direction| clear_along(direction).map(|distance| (direction, distance)))
        .min_by(|a, b| a.1.total_cmp(&b.1))
    else {
        return position;
    };
    LabelPosition::Placed {
        offset: offset + Point::new(direction.x * distance as i32, direction.y * distance as i32),
        horizontal,
        vertical,
    }
}

fn merged_attributes(
    definition: &ForeignSymbolDefinition,
    symbol: &ForeignSymbol,
) -> Vec<ForeignAttribute> {
    let mut attributes = definition.attributes.clone();
    for attribute in &symbol.attributes {
        match attributes
            .iter_mut()
            .find(|existing| existing.name.eq_ignore_ascii_case(&attribute.name))
        {
            Some(existing) => existing.value = attribute.value.clone(),
            None => attributes.push(attribute.clone()),
        }
    }
    attributes
}

/// The count of an LTspice parallel-count word (`x2`).
fn parallel_count(word: &str) -> Option<u32> {
    word.strip_prefix(['x', 'X'])
        .and_then(|count| count.parse::<u32>().ok())
        .filter(|count| *count > 0)
}

/// An attribute's text; LTspice writes an emptied attribute as `""`.
fn attribute<'a>(attributes: &'a [ForeignAttribute], name: &str) -> Option<&'a str> {
    attributes
        .iter()
        .find(|attribute| attribute.name.eq_ignore_ascii_case(name))
        .map(|attribute| match attribute.value.trim() {
            "\"\"" => "",
            _ => attribute.value.as_str(),
        })
}

/// A subcircuit name followed by words of its own instance line splits into
/// the name and those words.
/// The model a value names and the words after it; a value that opens
/// with a setting names none, and `fallback` is the model.
fn split_model(text: &str, fallback: &str) -> (String, Vec<String>) {
    let tokens = spice_tokens(text);
    match tokens.split_first() {
        Some((name, rest)) if !name.contains('=') => (name.clone(), rest.to_vec()),
        _ => (fallback.to_owned(), tokens),
    }
}

/// Whether `text` is a SPICE number: digits, an optional fraction and
/// exponent, then letters only (a scale or a unit, `10k`, `4.7uF`).
pub(super) fn is_spice_number(text: &str) -> bool {
    let unsigned = text.strip_prefix(['+', '-']).unwrap_or(text);
    let digits = unsigned
        .find(|character: char| !(character.is_ascii_digit() || character == '.'))
        .unwrap_or(unsigned.len());
    let (number, rest) = unsigned.split_at(digits);
    if !number.chars().any(|character| character.is_ascii_digit())
        || number.matches('.').count() > 1
    {
        return false;
    }
    let rest = match rest.strip_prefix(['e', 'E']) {
        Some(exponent)
            if exponent
                .trim_start_matches(['+', '-'])
                .starts_with(|c: char| c.is_ascii_digit()) =>
        {
            exponent
                .trim_start_matches(['+', '-'])
                .trim_start_matches(|c: char| c.is_ascii_digit())
        }
        _ => rest,
    };
    rest.chars()
        .all(|character| character.is_ascii_alphabetic())
}

fn looks_like_file(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    [
        ".lib", ".sub", ".mod", ".cir", ".txt", ".inc", ".bjt", ".dio", ".mos", ".jft", ".sp",
        ".ckt",
    ]
    .iter()
    .any(|extension| lower.ends_with(extension))
        || lower.contains(['\\', '/'])
}

/// A project cell name from a symbol name: letters, digits and underscores.
fn cell_name(symbol: &str) -> String {
    let mut name = String::new();
    for character in symbol.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            name.push(character);
        } else if !name.ends_with('_') {
            name.push('_');
        }
    }
    let name = name.trim_matches('_');
    if name.is_empty() {
        "imported_symbol".to_owned()
    } else if name.starts_with(|character: char| character.is_ascii_digit()) {
        format!("s_{name}")
    } else {
        name.to_owned()
    }
}

/// The definition's pins in instance-line order, with `None` at each place
/// below the last that no pin takes.
fn spice_order_slots(definition: &ForeignSymbolDefinition) -> Vec<Option<&ForeignPin>> {
    let mut slots = Vec::new();
    let mut next = 1;
    for pin in definition.ordered_pins() {
        while next < pin.spice_order {
            slots.push(None);
            next += 1;
        }
        slots.push(Some(pin));
        next = next.max(pin.spice_order + 1);
    }
    slots
}

/// Places for pins that must touch nothing, near the middle of the drawn
/// pins: each an odd number of grid steps on both axes, where no wire of a
/// drawing on LTspice's grid (two RSpice steps) can run, and off every
/// drawn pin.
fn unconnected_pin_points(drawn: &[Point]) -> impl Iterator<Item = Point> + '_ {
    let middle = |values: Vec<i32>| {
        let (low, high) = (
            values.iter().copied().min().unwrap_or_default(),
            values.iter().copied().max().unwrap_or_default(),
        );
        ((low + high) / 2).div_euclid(2 * GRID) * 2 * GRID + GRID
    };
    let x = middle(drawn.iter().map(|point| point.x).collect());
    let y = middle(drawn.iter().map(|point| point.y).collect());
    (0..)
        .map(move |step| Point::new(x + 2 * GRID * step, y))
        .filter(|point| !drawn.contains(point))
}

/// Ports in instance-line order, each with a name of its own; a place no
/// pin takes is named for its number.
fn unique_pin_names(slots: &[Option<&ForeignPin>]) -> Vec<PortSpec> {
    let mut seen = BTreeSet::new();
    (1..)
        .zip(slots)
        .map(|(order, slot)| {
            let base = match slot {
                Some(pin) if !pin.name.trim().is_empty() => pin.name.trim().to_owned(),
                Some(pin) => format!("P{}", pin.spice_order),
                None => format!("NC{order}"),
            };
            let mut name = base.clone();
            let mut suffix = 2;
            while !seen.insert(name.to_ascii_lowercase()) {
                name = format!("{base}_{suffix}");
                suffix += 1;
            }
            PortSpec {
                name,
                direction: PortDirection::InOut,
            }
        })
        .collect()
}

/// How far a comment's text stands off the point its drawing stood it on by
/// an edge, so a word standing on a wire keeps its descenders clear of it.
const NOTE_STANDOFF_WORLD: f32 = 3.0;

/// `note` lettered the way its drawing lettered it, standing on `point`,
/// with no anchor mark: the drawing marks none.
pub(super) fn lettered_note(
    note: DesignNote,
    point: Point,
    lettering: &ForeignLettering,
) -> DesignNote {
    let note = note
        .with_text_size_percent(lettering.size_percent)
        .with_anchor_mark_hidden(true);
    let pos = note_anchor(&note, point, lettering);
    DesignNote { pos, ..note }
}

/// Where a note's anchor goes so its text's box stands on `point` the way
/// the source drawing stood it: by its start, end or centre across, and by
/// its top, bottom or middle down. A text of several lines LTspice centres
/// down stands on its point by its first line, the rest running below; its
/// drawings lay their notes out by that.
fn note_anchor(note: &DesignNote, point: Point, lettering: &ForeignLettering) -> Point {
    let (width, height) = note.text_extent_world(&note.text);
    let middle = if lettering.by_first_line {
        note.text_extent_world("").1
    } else {
        height
    };
    let reach = |edge: TextEdge, extent: f32, middle: f32| match edge {
        TextEdge::Start => -NOTE_STANDOFF_WORLD,
        TextEdge::Center => middle * 0.5,
        TextEdge::End => extent + NOTE_STANDOFF_WORLD,
    };
    let (across, down) = lettering.justification;
    let (offset_x, offset_y) = DESIGN_NOTE_TEXT_OFFSET_WORLD;
    Point::new(
        point.x - (reach(across, width, width) + offset_x).round() as i32,
        point.y - (reach(down, height, middle) + offset_y).round() as i32,
    )
}

/// The glyph a rail flag draws: its name's by convention, unless that would
/// stand on what reaches the flag. A rail reached from above and not from
/// below hangs its bar below its point, and one reached from below and not
/// from above stands its bar or arrow over it, as a drafter turns a rail
/// symbol away from its wire; `reaches` are the sides wires and leads reach
/// the flag from.
fn rail_glyph(name: &str, reaches: &[ForeignPoint]) -> SupplyGlyph {
    let glyph = SupplyGlyph::for_name(name);
    let from = |y: i32| reaches.iter().any(|reach| reach.x == 0 && reach.y == y);
    match glyph {
        SupplyGlyph::Down if from(1) && !from(-1) => SupplyGlyph::Bar,
        SupplyGlyph::Bar | SupplyGlyph::Arrow if from(-1) && !from(1) => SupplyGlyph::Down,
        _ => glyph,
    }
}

/// The middle of a part's terminals.
fn centre(terminals: &[Point]) -> Point {
    let count = i32::try_from(terminals.len().max(1)).unwrap_or(1);
    let sum = terminals
        .iter()
        .fold(Point::default(), |sum, point| sum + *point);
    Point::new(sum.x / count, sum.y / count)
}

/// The type a QSPICE behavioural device states (`D-FLOP`, `RROPAMP`): the
/// first word of the type its symbol draws, else of its value, that is a
/// name and not a setting or a number.
fn qspice_kind(symbol: &ForeignSymbol) -> String {
    [symbol.attribute(DEVICE_TYPE), symbol.attribute("Value")]
        .into_iter()
        .flatten()
        .find_map(|text| {
            spice_tokens(&spice_text(text)).into_iter().find(|word| {
                !word.contains('=') && word.chars().next().is_some_and(char::is_alphabetic)
            })
        })
        .unwrap_or_default()
}

/// VDD, VCC, VSS and VEE, alone or numbered, are supply rails.
fn is_supply_rail(name: &str) -> bool {
    let upper = name.trim().to_ascii_uppercase();
    ["VDD", "VCC", "VSS", "VEE"].iter().any(|rail| {
        upper
            .strip_prefix(rail)
            .is_some_and(|rest| rest.chars().all(|character| character.is_ascii_digit()))
    })
}

/// Directive text with the source's micro signs written as `u`.
fn spice_text_lines(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end().replace(['\u{B5}', '\u{3BC}'], "u"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn strictly_inside(a: Point, b: Point, point: Point) -> bool {
    if point == a || point == b {
        return false;
    }
    let cross = i64::from(b.x - a.x) * i64::from(point.y - a.y)
        - i64::from(b.y - a.y) * i64::from(point.x - a.x);
    if cross != 0 {
        return false;
    }
    let within = |low: i32, high: i32, value: i32| low.min(high) <= value && value <= low.max(high);
    within(a.x, b.x, point.x) && within(a.y, b.y, point.y)
}

/// The placements of a component that put its terminals nearest the source
/// pins, cheapest first: each with its position, turn, mirror and terminal
/// offsets.
///
/// Each of the eight orientations is tried at every position that puts one
/// terminal in line with its pin. A terminal on its pin costs nothing; one
/// in line costs its distance, and more when its lead would run back across
/// the body; one out of line costs most, since its lead must bend.
fn placements(
    component: &Component,
    pins: &[Point],
    offsets: impl Fn(&Component) -> Vec<Point>,
) -> Vec<(Point, Rotation, bool, Vec<Point>)> {
    let mut found = Vec::new();
    for mirrored in [false, true] {
        for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
            let mut candidate = component.clone();
            candidate.pos = Point::default();
            candidate.rotation = rotation;
            candidate.mirror_h = mirrored;
            candidate.mirror_v = false;
            let local = offsets(&candidate);
            if local.len() != pins.len() || pins.is_empty() {
                continue;
            }
            let center = Point::new(
                local.iter().map(|point| point.x).sum::<i32>() / local.len() as i32,
                local.iter().map(|point| point.y).sum::<i32>() / local.len() as i32,
            );
            let xs = local
                .iter()
                .zip(pins)
                .map(|(offset, pin)| pin.x - offset.x)
                .collect::<BTreeSet<_>>();
            let ys = local
                .iter()
                .zip(pins)
                .map(|(offset, pin)| pin.y - offset.y)
                .collect::<BTreeSet<_>>();
            for x in &xs {
                for y in &ys {
                    let position = Point::new(*x, *y);
                    let cost = local
                        .iter()
                        .zip(pins)
                        .map(|(offset, pin)| lead_cost(position + *offset, *pin, position + center))
                        .sum::<i64>();
                    found.push((cost, position, rotation, mirrored, local.clone()));
                }
            }
        }
    }
    // A stable sort: of equal costs, the orientation tried first stays first.
    found.sort_by_key(|(cost, ..)| *cost);
    found
        .into_iter()
        .map(|(_, position, rotation, mirrored, local)| (position, rotation, mirrored, local))
        .collect()
}

/// The ways a lead runs from a terminal to its pin: straight when they are in
/// line, else bent once, across first or down first.
fn lead_routes(from: Point, to: Point) -> Vec<Vec<(Point, Point)>> {
    if from == to {
        return vec![Vec::new()];
    }
    if from.x == to.x || from.y == to.y {
        return vec![vec![(from, to)]];
    }
    [Point::new(to.x, from.y), Point::new(from.x, to.y)]
        .into_iter()
        .map(|corner| vec![(from, corner), (corner, to)])
        .collect()
}

/// Leads that bend twice: across to a jog line a few grid steps beside
/// either end, along it, and across again; shortest first.
fn detour_routes(from: Point, to: Point) -> Vec<Vec<(Point, Point)>> {
    let steps = (1..=3).flat_map(|step| [-step * GRID, step * GRID]);
    let mut routes = Vec::new();
    for step in steps {
        for x in [from.x + step, to.x + step] {
            let (a, b) = (Point::new(x, from.y), Point::new(x, to.y));
            routes.push(vec![(from, a), (a, b), (b, to)]);
        }
        for y in [from.y + step, to.y + step] {
            let (a, b) = (Point::new(from.x, y), Point::new(to.x, y));
            routes.push(vec![(from, a), (a, b), (b, to)]);
        }
    }
    let length = |route: &Vec<(Point, Point)>| {
        route
            .iter()
            .map(|(a, b)| (a.x - b.x).abs() + (a.y - b.y).abs())
            .sum::<i32>()
    };
    routes.retain(|route| route.iter().all(|(a, b)| a != b));
    routes.sort_by_key(length);
    routes
}

/// Whether `point` lies on the segment from `a` to `b`, ends included.
fn on_segment(a: Point, b: Point, point: Point) -> bool {
    point == a || point == b || strictly_inside(a, b, point)
}

/// Whether two segments run along each other for some length.
fn overlap(a: Point, b: Point, c: Point, d: Point) -> bool {
    let cross = |p: Point| {
        i64::from(b.x - a.x) * i64::from(p.y - a.y) - i64::from(b.y - a.y) * i64::from(p.x - a.x)
    };
    if a == b || cross(c) != 0 || cross(d) != 0 {
        return false;
    }
    let (low, high, other_low, other_high) = if a.x == b.x {
        (a.y.min(b.y), a.y.max(b.y), c.y.min(d.y), c.y.max(d.y))
    } else {
        (a.x.min(b.x), a.x.max(b.x), c.x.min(d.x), c.x.max(d.x))
    };
    high.min(other_high) > low.max(other_low)
}

fn lead_cost(terminal: Point, pin: Point, center: Point) -> i64 {
    let dx = i64::from(pin.x - terminal.x);
    let dy = i64::from(pin.y - terminal.y);
    if dx == 0 && dy == 0 {
        return 0;
    }
    let length = dx.abs() + dy.abs();
    if dx != 0 && dy != 0 {
        return 100_000 + length;
    }
    let outward_x = i64::from(terminal.x - center.x);
    let outward_y = i64::from(terminal.y - center.y);
    if dx * outward_x + dy * outward_y < 0 {
        10_000 + length
    } else {
        length
    }
}

/// Start angle and counter-clockwise sweep, in radians on screen, of the
/// arc LTspice draws in the box `a`–`b` from the ray through `start` to the
/// ray through `end`.
fn arc_angles(a: Point, b: Point, start: Point, end: Point) -> Option<(f64, f64)> {
    let center = (f64::from(a.x + b.x) / 2.0, f64::from(a.y + b.y) / 2.0);
    if a.x == b.x || a.y == b.y {
        return None;
    }
    // Screen y points down, so counter-clockwise on screen is a falling
    // mathematical angle once y is turned over.
    let angle = |point: Point| (center.1 - f64::from(point.y)).atan2(f64::from(point.x) - center.0);
    let from = angle(start);
    let mut sweep = angle(end) - from;
    if sweep <= 0.0 {
        sweep += std::f64::consts::TAU;
    }
    Some((from, sweep))
}

/// `segments + 1` points along the ellipse in the box `a`–`b`, from `from`
/// counter-clockwise through `sweep` (radians, y up).
fn ellipse_points(a: Point, b: Point, from: f64, sweep: f64, segments: usize) -> Vec<Point> {
    let center = (f64::from(a.x + b.x) / 2.0, f64::from(a.y + b.y) / 2.0);
    let radius = (
        f64::from((b.x - a.x).abs()) / 2.0,
        f64::from((b.y - a.y).abs()) / 2.0,
    );
    let closed = (sweep - std::f64::consts::TAU).abs() < 1e-9;
    let count = if closed { segments } else { segments + 1 };
    (0..count)
        .map(|index| {
            let angle = from + sweep * index as f64 / segments as f64;
            Point::new(
                (center.0 + radius.0 * angle.cos()).round() as i32,
                (center.1 - radius.1 * angle.sin()).round() as i32,
            )
        })
        .collect()
}
