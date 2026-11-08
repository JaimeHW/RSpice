//! Turning a foreign drawing into an RSpice schematic.
//!
//! The drawing is scaled onto RSpice's grid: one source grid step (16 LTspice
//! units, 100 QSPICE units) becomes one RSpice step. Every place a source pin,
//! wire end or flag sits keeps its scaled position, so the source's wiring
//! stands as it was drawn. A part draws its source symbol's own artwork,
//! whether it keeps a native electrical identity or comes across as a cell of
//! the project, so its pins land exactly. A native part whose symbol cannot be
//! read is turned and placed where its terminals best meet the source pins, and
//! a straight lead joins any terminal that does not land on its pin.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

use rspice_design_model::Point;
use rspice_design_model::port::{PortDirection, PortSpec};
use rspice_design_model::symbol_pin::SymbolPinSide;

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
use crate::schematic::component_display::ComponentDisplayMode;
use crate::schematic::component_type::ComponentType;
use crate::schematic::design_note::{DESIGN_NOTE_TEXT_OFFSET_WORLD, DesignNote, DesignNoteKind};
use crate::schematic::document::SchematicDocument;
use crate::schematic::document_policy::{NetNamingPolicy, SchematicDocumentPolicy};
use crate::schematic::documentation_shape::{
    DocumentationShape, DocumentationShapeGeometry, MAX_DOCUMENTATION_POLYGON_POINTS,
};
use crate::schematic::functional_catalog::{
    conform_functional_inputs, functional_library_binding, functional_part, placement_value,
};
use crate::schematic::ground_glyph::GroundGlyph;
use crate::schematic::label_layout::{
    IMPORTED_TEXT_WORLD, INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD, TextEdge,
};
use crate::schematic::label_position::{LabelPosition, ValuePiece};
use crate::schematic::net_label::{
    Junction, NetLabel, NetLabelDrawing, NetLabelKind, NetLabelSide, SupplyGlyph,
};
use crate::schematic::rotation::Rotation;
use crate::schematic::terminal_connection::rebuild_connections_from_terminals;
use crate::schematic::value_label::{value_and_parameters, value_pieces};
use crate::schematic::wire::{Wire, WireBridge};
use crate::symbol::{PinNamePlacement, SymbolCurve, SymbolDocument, SymbolPin, SymbolShape};

mod clearance;

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
        bridges: BTreeMap::new(),
        source_points: HashSet::new(),
        source_segments: Vec::new(),
        source_nets: SourceNets::default(),
        anchors: HashSet::new(),
        terminals: Vec::new(),
        names: BTreeSet::new(),
        models: BTreeSet::new(),
        pin_points: HashMap::new(),
        pin_leads: HashMap::new(),
        markers: Vec::new(),
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
    builder.place_texts();
    builder.place_shapes();
    builder.place_wires();
    builder.clear_markers();
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
    /// What each jumper draws, by the ends of the wire it is.
    bridges: BTreeMap<WireEnds, WireBridge>,
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
    /// The notes marking refused parts, by index, each with the bottom
    /// right of the gap its part left.
    markers: Vec<(usize, Point)>,
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
    /// The net the terminal joins, ground when there is none.
    net: Option<String>,
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

    /// A drawing point on the sheet. A pure function of the coordinate, so
    /// points the source joins stay joined and points it keeps apart stay
    /// apart, whatever grid they sit on.
    fn scale_ratio(&self) -> (i64, i64) {
        match self.drawing.tool {
            ForeignTool::Ltspice => (5, 8),
            ForeignTool::Qspice => (1, 10),
        }
    }

    /// Where `point` lies in world units before [`Self::scale`] rounds it: a
    /// curve is drawn through these.
    fn exact(&self, point: ForeignPoint) -> [f64; 2] {
        let (numerator, denominator) = self.scale_ratio();
        let ratio = numerator as f64 / denominator as f64;
        [f64::from(point.x) * ratio, f64::from(point.y) * ratio]
    }

    fn scale(&self, point: ForeignPoint) -> Point {
        let (numerator, denominator) = self.scale_ratio();
        let scaled = |value: i32| {
            let scaled = i64::from(value) * numerator;
            i32::try_from((scaled + scaled.signum() * denominator / 2) / denominator).unwrap_or(0)
        };
        Point::new(scaled(point.x), scaled(point.y))
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
        // LTspice's jumper joins its two nets and is no device: a wire, which
        // draws what the jumper's symbol draws.
        if tool == ForeignTool::Ltspice
            && symbol.base_name().eq_ignore_ascii_case("jumper")
            && let [from, to] = pins.as_slice()
        {
            self.lead(*from, *to);
            self.bridge_jumper(symbol, &definition, *from, *to);
            return;
        }
        let placed = self.document.components.len();
        match native_mapping(tool, symbol.base_name(), Some(&definition)) {
            Some(mapping) if schematic.is_none() => {
                self.place_native(symbol, &definition, &attributes, mapping, &pins);
            }
            _ => {
                if schematic.is_none()
                    && let Some(plan) =
                        self.functional_plan(symbol, &definition, &attributes, &pins)
                {
                    self.place_functional(symbol, &definition, &attributes, plan, &pins);
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
        let place = |window: Option<&ForeignWindow>, ordinary: f32| {
            window
                .filter(|window| !window.hidden)
                .map(|window| self.label_position(symbol, window, origin, ordinary))
        };
        let (name_position, value_position) = (
            place(name, INSTANCE_NAME_WORLD),
            place(value, INSTANCE_VALUE_WORLD),
        );
        let mut pieces = symbol
            .value_pieces
            .iter()
            .map(|piece| {
                (
                    piece.text.clone(),
                    self.label_position(symbol, &piece.window, origin, INSTANCE_VALUE_WORLD),
                    lettered_percent(piece.size_percent, INSTANCE_VALUE_WORLD),
                )
            })
            .collect::<Vec<_>>();
        // A source's drawing letters its waveform as written, on a line,
        // where RSpice letters a line for each parameter: each text the
        // drawing shows keeps its words and its place.
        let attributes = merged_attributes(definition, symbol);
        let source = component.kind.is_source();
        if source && pieces.is_empty() {
            pieces = SOURCE_TEXTS
                .into_iter()
                .filter_map(|(number, key)| {
                    let window = window(number).filter(|window| !window.hidden)?;
                    let text =
                        attribute(&attributes, key).filter(|text| !text.trim().is_empty())?;
                    Some((
                        text.trim().to_owned(),
                        self.label_position(symbol, window, origin, INSTANCE_VALUE_WORLD),
                        lettered_percent(window.size_percent, INSTANCE_VALUE_WORLD),
                    ))
                })
                .collect();
        }
        let spelled = pieces
            .iter()
            .flat_map(|(text, ..)| text.split_whitespace())
            .eq(component.value.split_whitespace());
        // A lone text that is the part's value letters as the value.
        if spelled && pieces.len() == 1 && !source {
            pieces.clear();
        }
        // Texts that say other than the part's value (a source's waveform,
        // or the settings QSPICE letters on a gate whose model RSpice names
        // itself) stand while the part keeps what it came with.
        let stands_for = (source || !spelled).then(|| value_and_parameters(component));
        let unshown =
            |window: Option<&ForeignWindow>| window.map_or(windowed, |window| window.hidden);
        // A value window letters the `Value` attribute and nothing else: the
        // model a symbol's `SpiceModel` names stays on the part, unlettered.
        let valueless = windowed
            && pieces.is_empty()
            && attribute(&attributes, "Value").is_none_or(|text| text.trim().is_empty());
        let hidden = (unshown(name), unshown(value) || valueless);
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

    fn label_position(
        &self,
        symbol: &ForeignSymbol,
        window: &ForeignWindow,
        origin: Point,
        ordinary: f32,
    ) -> LabelPosition {
        let anchor = self.scale(
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
        // LTspice turns a window's text with its symbol and letters a
        // quarter-turned one along its turned line: a vertical window on an
        // upright symbol, or an upright one on a turned symbol, reads upward.
        // Read upward, a run starts at its bottom and its top faces left.
        let upward = self.drawing.tool == ForeignTool::Ltspice
            && window.vertical != (symbol.orientation.quarter_turns % 2 == 1);
        let (horizontal, vertical) = if upward {
            (edge(-reach.y), edge(reach.x))
        } else {
            (edge(reach.x), edge(reach.y))
        };
        LabelPosition::Placed {
            offset: anchor - origin,
            horizontal,
            vertical,
            upward,
            size_percent: lettered_percent(window.size_percent, ordinary),
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
        definition: &ForeignSymbolDefinition,
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
        let terminals = mapping
            .terminals
            .iter()
            .copied()
            .zip(pins.iter().copied())
            .collect::<Vec<_>>();
        if !self.draw_as_the_source_draws(
            &mut component,
            symbol,
            definition,
            &terminals,
            mapping.tie,
        ) && !draw_rspice_glyph_at_the_source_pin_span(&mut component, &terminals, mapping.tie)
        {
            self.place_fitted(&mut component, pins, offsets);
        }
        let placed = component.terminal_positions();
        if let Some((from, to)) = mapping.tie {
            let point = |terminal: &str| {
                placed
                    .iter()
                    .find(|(name, _)| *name == terminal)
                    .map(|(_, point)| *point)
            };
            if let (Some(from), Some(to)) = (point(from), point(to))
                && from != to
            {
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
        definition: &ForeignSymbolDefinition,
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
        let terminals = binding
            .terminal_order
            .iter()
            .map(String::as_str)
            .zip(targets.iter().copied())
            .collect::<Vec<_>>();
        let points = if self.draw_as_the_source_draws(
            &mut component,
            symbol,
            definition,
            &terminals,
            None,
        ) {
            targets.clone()
        } else {
            self.place_fitted(&mut component, &targets, offsets)
        };
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
            self.tie_default_nets(symbol, definition, pins, &points);
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

    /// Gives a native part the artwork its own symbol draws, standing where
    /// and as the drawing stands it, with each terminal on its source pin.
    /// False when the definition carries no artwork, as a stand-in for a
    /// missing symbol file does.
    fn draw_as_the_source_draws(
        &self,
        component: &mut Component,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
        terminals: &[(&str, Point)],
        tie: Option<(&str, &str)>,
    ) -> bool {
        let body = definition
            .shapes
            .iter()
            .filter_map(|shape| self.symbol_shape(shape))
            .collect::<Vec<_>>();
        if body.is_empty() {
            return false;
        }
        component.pos = self.scale(symbol.position);
        // A drawing turns and then mirrors; a component mirrors and then
        // turns, so a mirrored part turns the other way.
        component.mirror_h = symbol.orientation.mirrored;
        component.rotation = match (
            symbol.orientation.quarter_turns,
            symbol.orientation.mirrored,
        ) {
            (0, _) => Rotation::R0,
            (2, _) => Rotation::R180,
            (1, false) | (3, true) => Rotation::R90,
            _ => Rotation::R270,
        };
        let pins = terminals
            .iter()
            .map(|(name, pin)| (*name, component.untransform_point(*pin - component.pos)))
            .collect::<Vec<_>>();
        component.set_source_symbol(terminal_symbol(&pins, tie, body));
        true
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
        for (source, terminal) in sources.iter().zip(terminals) {
            self.device_ties.push(DeviceTie {
                source: Some(*source),
                terminal: *terminal,
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

    /// Ties each recorded terminal whose source pin nothing reaches to the
    /// net its source simulator writes there. The source draws such a pin as
    /// open, so the tie draws only the open-pin square.
    fn tie_device_pins(&mut self) {
        for tie in std::mem::take(&mut self.device_ties) {
            if tie.source.is_some_and(|source| self.reached(source)) {
                continue;
            }
            if let Some(net) = tie.net {
                let id = self.id();
                let mut label = NetLabel::new(id, tie.terminal, net.clone());
                label.drawing = NetLabelDrawing::OpenPin;
                if is_supply_rail(&net) {
                    label.kind = NetLabelKind::Supply {
                        glyph: SupplyGlyph::for_name(&net),
                    };
                }
                self.document.net_labels.push(label);
                continue;
            }
            let id = self.id();
            let mut ground = Component::new(id, ComponentType::Ground, Point::default());
            ground.ground_glyph = GroundGlyph::OpenPin;
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
        let lettering = self.symbol_lettering(symbol, definition);
        let mut spare = clearance::unconnected_pin_points(&drawn, &lettering);
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
                .map(|offset| self.scale(ForeignPoint::new(offset, 0)).x);
            pins.push(
                SymbolPin::new(port.name.clone(), port.direction, Some(position))
                    .with_name_hidden(
                        slot.is_none_or(|pin| pin.name_hidden || pin.name.trim().is_empty()),
                    )
                    .with_name_offset(name_offset)
                    .with_name_at(slot.and_then(|pin| {
                        pin.name_at.map(|(offset, reach)| PinNamePlacement {
                            offset: self.scale(offset),
                            reach: Point::new(reach.x, reach.y),
                            upward: pin.name_upward,
                        })
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
            pin_name_size: Some(IMPORTED_TEXT_WORLD),
            body: definition
                .shapes
                .iter()
                .filter_map(|shape| self.symbol_shape(shape))
                .collect(),
            ..SymbolDocument::default()
        };
        // An LTspice pin names itself as RSpice letters a pin on the edge
        // its justification puts it on. A pin RSpice reads onto another
        // edge, one inside the box around a slanted or stepped body, keeps
        // the drawing's place for its name instead, and so does a pin it
        // letters on its side.
        if self.drawing.tool == ForeignTool::Ltspice {
            let agreed = document
                .pins
                .iter()
                .map(|pin| {
                    pin.name_at.filter(|at| !at.upward).is_some_and(|at| {
                        let side = match (at.reach.x, at.reach.y) {
                            (1, _) => SymbolPinSide::Left,
                            (-1, _) => SymbolPinSide::Right,
                            (_, 1) => SymbolPinSide::Top,
                            _ => SymbolPinSide::Bottom,
                        };
                        document.pin_side(pin) == side
                    })
                })
                .collect::<Vec<_>>();
            for (pin, agreed) in document.pins.iter_mut().zip(agreed) {
                if agreed {
                    pin.name_at = None;
                }
            }
        }
        let anchor = |attribute: u32| {
            definition
                .windows
                .iter()
                .find(|window| window.attribute == attribute)
                .map(|window| self.scale(window.offset))
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

    /// Keeps what a jumper between `from` and `to` draws, for the wire that
    /// joins them to draw instead of a line.
    fn bridge_jumper(
        &mut self,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
        from: Point,
        to: Point,
    ) {
        if from == to || (from.x != to.x && from.y != to.y) {
            return;
        }
        let key = ends_key(from, to);
        let first = Point::new(key.0.0, key.0.1);
        let run = Point::new(key.1.0, key.1.1) - first;
        let origin = self.scale(symbol.position) - first;
        let unit = |x, y| {
            let step = symbol.orientation.apply(ForeignPoint::new(x, y));
            Point::new(step.x, step.y)
        };
        let (across, down) = (unit(1, 0), unit(0, 1));
        let place = |point: Point| {
            origin
                + Point::new(
                    point.x * across.x + point.y * down.x,
                    point.x * across.y + point.y * down.y,
                )
        };
        let (mut lines, mut curves) = (Vec::new(), Vec::new());
        for shape in definition
            .shapes
            .iter()
            .filter_map(|s| self.symbol_shape(s))
        {
            match shape {
                SymbolShape::Polyline { points, closed } => {
                    lines.extend(
                        points
                            .windows(2)
                            .map(|ends| [place(ends[0]), place(ends[1])]),
                    );
                    if closed && let [start, .., end] = points.as_slice() {
                        lines.push([place(*end), place(*start)]);
                    }
                }
                SymbolShape::Curve(curve) => curves.push(curve.placed(origin, across, down)),
                _ => {}
            }
        }
        let window = symbol
            .windows
            .iter()
            .chain(&definition.windows)
            .find(|window| window.attribute == 0)
            .filter(|window| !window.hidden);
        let name_position = window.map_or(LabelPosition::Auto, |window| {
            self.label_position(symbol, window, first, INSTANCE_NAME_WORLD)
        });
        let name = window
            .and_then(|_| symbol.attribute("InstName"))
            .unwrap_or_default()
            .to_owned();
        self.bridges.insert(
            key,
            WireBridge {
                run,
                lines,
                curves,
                name,
                name_position,
            },
        );
    }

    fn symbol_shape(&self, shape: &ForeignShape) -> Option<SymbolShape> {
        Some(match shape {
            ForeignShape::Line(a, b) => SymbolShape::Polyline {
                points: vec![self.scale(*a), self.scale(*b)],
                closed: false,
            },
            ForeignShape::Rectangle(a, b) => {
                let (a, b) = (self.scale(*a), self.scale(*b));
                SymbolShape::Polyline {
                    points: vec![a, Point::new(b.x, a.y), b, Point::new(a.x, b.y)],
                    closed: true,
                }
            }
            ForeignShape::Ellipse(a, b) => {
                SymbolShape::Curve(ellipse_in_box(self.exact(*a), self.exact(*b)))
            }
            ForeignShape::Arc([a, b, start, end]) => SymbolShape::Curve(arc_in_box(
                self.exact(*a),
                self.exact(*b),
                self.exact(*start),
                self.exact(*end),
            )?),
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
                // The source's own glyph, turned as the source turns it: the
                // way the drawing says, else away from the one side a wire
                // or lead reaches the flag from, else hanging below.
                ground.ground_glyph = match self.drawing.tool {
                    ForeignTool::Ltspice => GroundGlyph::Ltspice,
                    ForeignTool::Qspice => GroundGlyph::Qspice,
                };
                let run = flag.facing.or_else(|| self.away_from_attachment(at));
                ground.rotation = match run.map(|run| (run.x.signum(), run.y.signum())) {
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
                label.drawing = NetLabelDrawing::Lettered;
                let run = flag.facing.or_else(|| self.away_from_attachment(at));
                // A flag the wires reach from more than one side letters its
                // name over its point.
                label.side = match run.map(|run| (run.x.signum(), run.y.signum())) {
                    Some((1, _)) => NetLabelSide::Right,
                    Some((-1, _)) => NetLabelSide::Left,
                    Some((0, 1)) => NetLabelSide::Below,
                    _ => NetLabelSide::Above,
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
                self.document.net_labels.push(label);
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
    /// A coupling statement over drawn inductors becomes a coupling part. The
    /// drawing shows the statement and no part, so the part draws no glyph
    /// and letters the statement where the drawing did, while it still
    /// couples what the statement says.
    fn place_coupling(
        &mut self,
        position: Point,
        line: &str,
        lettering: &ForeignLettering,
    ) -> bool {
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
        component.set_source_symbol(SymbolDocument::default());
        component.display_mode = ComponentDisplayMode::ValueOnly;
        let (horizontal, vertical) = lettering.justification;
        component.value_pieces = vec![ValuePiece {
            text: line.trim().to_owned(),
            position: LabelPosition::Placed {
                offset: Point::default(),
                horizontal,
                vertical,
                upward: false,
                size_percent: 100,
            },
            size_percent: lettered_percent(lettering.size_percent, INSTANCE_VALUE_WORLD),
            stands_for: Some(value_and_parameters(&component)),
        }];
        self.document.components.push(component);
        true
    }

    fn place_texts(&mut self) {
        for text in &self.drawing.texts {
            let position = self.scale(text.position);
            match text.kind {
                ForeignTextKind::Directive => {
                    let (lettering, line) = (text.lettering, text.line);
                    let advance = i32::from(lettering.size_percent) * 15 / 100;
                    // A coupling letters its own line of the block, so the
                    // block keeps that line empty.
                    let (mut drawn, mut read) = (Vec::new(), Vec::new());
                    let lines = spice_text_lines(&text.text);
                    for (index, line) in lines.lines().enumerate() {
                        let at = position + Point::new(0, advance * index as i32);
                        if self.place_coupling(at, line, &lettering) {
                            drawn.push("");
                        } else {
                            drawn.push(line);
                            read.push(line);
                        }
                    }
                    let text = read.join("\n");
                    if !text.trim().is_empty() {
                        // The block stays on the sheet where the source drew
                        // it, whatever a project place takes from it.
                        let id = self.id();
                        let empty = drawn.iter().take_while(|line| line.trim().is_empty());
                        let first = position + Point::new(0, advance * empty.count() as i32);
                        match DesignNote::new(
                            id,
                            first,
                            DesignNoteKind::PlainText,
                            drawn.join("\n").trim(),
                        ) {
                            Ok(note) => {
                                let note = lettered_note(note, first, &lettering);
                                self.document.design_notes.push(note);
                            }
                            Err(error) => self.notice(format!(
                                "The directive block at line {line} was not drawn: {error}."
                            )),
                        }
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
                    start: self.scale(*a),
                    end: self.scale(*b),
                },
                ForeignShape::Rectangle(a, b) => DocumentationShapeGeometry::Rectangle {
                    first: self.scale(*a),
                    opposite: self.scale(*b),
                },
                ForeignShape::Ellipse(a, b) => DocumentationShapeGeometry::Polygon {
                    points: ellipse_outline(ellipse_in_box(self.exact(*a), self.exact(*b))),
                },
                ForeignShape::Arc([a, b, start, end]) => {
                    let arc = arc_in_box(
                        self.exact(*a),
                        self.exact(*b),
                        self.exact(*start),
                        self.exact(*end),
                    );
                    match arc {
                        Some(arc) => {
                            let at = |degrees: f64| whole_point(arc.point_at(degrees));
                            let from = arc.start_degrees;
                            DocumentationShapeGeometry::Arc {
                                start: at(from + arc.sweep_degrees),
                                through: at(from + arc.sweep_degrees / 2.0),
                                end: at(from),
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
            // A jumper joins its two pins and nothing between them.
            let bridged = self.bridges.contains_key(&ends_key(a, b));
            let mut cuts = self
                .anchors
                .iter()
                .copied()
                .filter(|point| !bridged && strictly_inside(a, b, *point))
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
            let mut wire = Wire::segment(id, Point::new(a.0, a.1), Point::new(b.0, b.1));
            wire.bridge = self.bridges.remove(&(a, b)).map(Box::new);
            self.document.wires.push(wire);
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

/// Where a note's anchor goes so its text stands on `point` the way the
/// source drawing stood it: by its start, end or centre across, and by its
/// top, bottom or middle down. A text of several lines LTspice centres down
/// stands on its point by its first line, the rest running below; its
/// drawings lay their notes out by that.
fn note_anchor(note: &DesignNote, point: Point, lettering: &ForeignLettering) -> Point {
    let (width, height) = note.text_extent_world(&note.text);
    let line_height = note.text_extent_world("").1;
    let middle = if lettering.by_first_line {
        line_height
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

/// The symbol document of a part that draws `body`, with a terminal at each
/// of `pins` under its native name. A terminal the symbol has no pin for
/// stands on the pin the source simulator ties it to.
fn terminal_symbol(
    pins: &[(&str, Point)],
    tie: Option<(&str, &str)>,
    body: Vec<SymbolShape>,
) -> SymbolDocument {
    let terminal = |name: &str, position: Point| {
        SymbolPin::new(name, PortDirection::InOut, Some(position)).with_name_hidden(true)
    };
    let mut terminals = pins
        .iter()
        .map(|(name, position)| terminal(name, *position))
        .collect::<Vec<_>>();
    if let Some((tied, to)) = tie
        && let Some((_, position)) = pins.iter().find(|(name, _)| *name == to)
    {
        terminals.push(terminal(tied, *position));
    }
    SymbolDocument {
        pins: terminals,
        body,
        ..SymbolDocument::default()
    }
}

/// Draws a native part whose source symbol could not be read: RSpice's glyph
/// turned to meet the source pins and scaled so its terminals span what the
/// pins span, each terminal on its pin. False when RSpice's artwork is not
/// built in, or its glyph has no terminals to scale by.
#[cfg(feature = "symbol-artwork")]
fn draw_rspice_glyph_at_the_source_pin_span(
    component: &mut Component,
    terminals: &[(&str, Point)],
    tie: Option<(&str, &str)>,
) -> bool {
    use crate::symbol_artwork::SymbolLibrary;
    static ARTWORK: std::sync::LazyLock<Option<SymbolLibrary>> =
        std::sync::LazyLock::new(|| SymbolLibrary::load_embedded().ok());
    let pins = terminals.iter().map(|(_, pin)| *pin).collect::<Vec<_>>();
    let offsets = |candidate: &Component| -> Vec<Point> {
        let placed = candidate.terminal_positions();
        terminals
            .iter()
            .map(|(name, _)| {
                placed
                    .iter()
                    .find(|(terminal, _)| terminal == name)
                    .map_or(Point::default(), |(_, point)| *point)
            })
            .collect()
    };
    let Some((_, rotation, mirrored, local)) =
        placements(component, &pins, offsets).into_iter().next()
    else {
        return false;
    };
    let span = |points: &[Point]| {
        let reach = |axis: fn(&Point) -> i32| {
            let values = points.iter().map(axis);
            values.clone().max().unwrap_or_default() - values.min().unwrap_or_default()
        };
        reach(|point| point.x).max(reach(|point| point.y))
    };
    let (glyph_span, pin_span) = (span(&local), span(&pins));
    let Some(first) = pins.first().copied().filter(|_| glyph_span > 0) else {
        return false;
    };
    let scale = pin_span as f32 / glyph_span as f32;
    let scaled =
        |x: f32, y: f32| Point::new((x * scale).round() as i32, (y * scale).round() as i32);
    let mut turned = component.clone();
    turned.pos = Point::default();
    turned.rotation = rotation;
    turned.mirror_h = mirrored;
    let Some(runs) = ARTWORK
        .as_ref()
        .and_then(|library| library.placed_artwork(&turned))
    else {
        return false;
    };
    let mut body = runs
        .iter()
        .map(|run| SymbolShape::Polyline {
            points: run.points.iter().map(|&(x, y)| scaled(x, y)).collect(),
            closed: false,
        })
        .collect::<Vec<_>>();
    component.pos = first - scaled(local[0].x as f32, local[0].y as f32);
    // Where the glyph's own terminal falls short of its pin, a lead draws the
    // rest.
    let symbol_pins = terminals
        .iter()
        .zip(&local)
        .map(|((name, pin), terminal)| {
            let (drawn, pin) = (
                scaled(terminal.x as f32, terminal.y as f32),
                *pin - component.pos,
            );
            if drawn != pin {
                body.push(SymbolShape::Polyline {
                    points: vec![drawn, pin],
                    closed: false,
                });
            }
            (*name, pin)
        })
        .collect::<Vec<_>>();
    component.set_glyph_at_source_pins(terminal_symbol(&symbol_pins, tie, body));
    true
}

/// `component` drawn with RSpice's own glyph at the pins its carried symbol
/// stands on: the same terminals, lettering and electrical part, with the
/// drawing's artwork dropped. `None` when it carries no artwork to drop or
/// RSpice has no glyph for it.
pub fn with_rspice_symbol(component: &Component) -> Option<Component> {
    if !component.can_swap_to_rspice_symbol() {
        return None;
    }
    let placed = component.terminal_positions();
    // A pin that stands on another is a tie the glyph does not draw.
    let mut terminals: Vec<(&str, Point)> = Vec::new();
    let mut tie = None;
    for (name, point) in &placed {
        match terminals.iter().find(|(_, at)| at == point) {
            Some((to, _)) => tie = Some((*name, *to)),
            None => terminals.push((*name, *point)),
        }
    }
    let mut swapped = component.clone();
    swapped.clear_source_symbol();
    swapped.rotation = Rotation::R0;
    swapped.mirror_h = false;
    swapped.mirror_v = false;
    if !draw_rspice_glyph_at_the_source_pin_span(&mut swapped, &terminals, tie) {
        return None;
    }
    // The glyph took its own anchor; the part keeps the one its lettering is
    // measured from.
    let shift = swapped.pos - component.pos;
    let mut document = swapped.source_symbol()?.document().clone();
    for shape in &mut document.body {
        shape.translate(shift);
    }
    for pin in &mut document.pins {
        pin.position = pin.position.map(|position| position + shift);
    }
    swapped.pos = component.pos;
    swapped.set_glyph_at_source_pins(document);
    (swapped.terminal_positions() == placed).then_some(swapped)
}

/// Without RSpice's artwork there is no glyph to scale.
#[cfg(not(feature = "symbol-artwork"))]
fn draw_rspice_glyph_at_the_source_pin_span(
    _: &mut Component,
    _: &[(&str, Point)],
    _: Option<(&str, &str)>,
) -> bool {
    false
}

/// A text's size in hundredths of the ordinary lettering of its kind, when
/// its drawing letters it at `source_percent` of that tool's standard size.
fn lettered_percent(source_percent: u16, ordinary: f32) -> u16 {
    (f32::from(IMPORTED_TEXT_WORLD) * f32::from(source_percent) / ordinary).round() as u16
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

/// A wire's ends as the sheet keeps them: the lesser first.
type WireEnds = ((i32, i32), (i32, i32));

fn ends_key(a: Point, b: Point) -> WireEnds {
    let (a, b) = ((a.x, a.y), (b.x, b.y));
    if a <= b { (a, b) } else { (b, a) }
}

/// The arc a drawing makes in the box `a`–`b`: counter-clockwise on screen
/// from where the ray through `start` leaves the ellipse to where the ray
/// through `end` does, the rule both tools draw by. `None` for a box with no
/// area.
fn arc_in_box(a: [f64; 2], b: [f64; 2], start: [f64; 2], end: [f64; 2]) -> Option<SymbolCurve> {
    if a[0] == b[0] || a[1] == b[1] {
        return None;
    }
    let mut arc = ellipse_in_box(a, b);
    let [x, y] = arc.center;
    let [rx, ry] = arc.radii;
    let angle = |point: [f64; 2]| {
        ((point[1] - y) / ry)
            .atan2((point[0] - x) / rx)
            .to_degrees()
    };
    // A curve runs clockwise, so this one starts from the far end.
    let sweep = (angle(start) - angle(end)).rem_euclid(360.0);
    arc.start_degrees = angle(end).rem_euclid(360.0);
    arc.sweep_degrees = if sweep == 0.0 { 360.0 } else { sweep };
    Some(arc)
}

/// The whole ellipse in the box `a`–`b`. A box with no height or width still
/// draws, as the sliver its source does.
fn ellipse_in_box(a: [f64; 2], b: [f64; 2]) -> SymbolCurve {
    SymbolCurve {
        center: [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0],
        radii: [
            ((b[0] - a[0]).abs() / 2.0).max(0.5),
            ((b[1] - a[1]).abs() / 2.0).max(0.5),
        ],
        start_degrees: 0.0,
        sweep_degrees: 360.0,
    }
}

/// A closed ring of whole-unit points around a sheet's drawn ellipse, as fine
/// as a sheet's polygon may be.
fn ellipse_outline(ellipse: SymbolCurve) -> Vec<Point> {
    let mut pixels_per_unit = 1.0;
    loop {
        let mut ring = ellipse
            .tessellated(pixels_per_unit)
            .into_iter()
            .map(whole_point)
            .collect::<Vec<_>>();
        ring.pop();
        ring.dedup();
        if ring.len() <= MAX_DOCUMENTATION_POLYGON_POINTS {
            return ring;
        }
        pixels_per_unit /= 2.0;
    }
}

fn whole_point([x, y]: [f64; 2]) -> Point {
    Point::new(x.round() as i32, y.round() as i32)
}
