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

use super::directives::{defined_model_names, foreign_directive_cards};
use super::mapping::{
    LTSPICE_AWAITING_NATIVE_PARTS, NativeKind, NativeMapping, ltspice_builtin_definition,
    native_mapping, qspice_refusal, source_value, spice_text, spice_tokens,
};
use super::{
    ForeignAttribute, ForeignPoint, ForeignPortDirection, ForeignSchematic, ForeignShape,
    ForeignSymbol, ForeignSymbolDefinition, ForeignTextKind, ForeignTool,
};
use crate::schematic::component::{Component, LibraryCellInstance};
use crate::schematic::component_type::ComponentType;
use crate::schematic::design_note::{DesignNote, DesignNoteKind};
use crate::schematic::document::SchematicDocument;
use crate::schematic::documentation_shape::{DocumentationShape, DocumentationShapeGeometry};
use crate::schematic::net_label::{Junction, NetLabel, NetLabelKind, SupplyGlyph};
use crate::schematic::rotation::Rotation;
use crate::schematic::terminal_connection::rebuild_connections_from_terminals;
use crate::schematic::wire::Wire;
use crate::symbol::{SymbolDocument, SymbolPin, SymbolShape};

/// RSpice's schematic grid step.
const GRID: i32 = 10;

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

/// A directive block of the drawing, where it was drawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignDirective {
    pub position: Point,
    pub text: String,
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
        document: SchematicDocument::default(),
        next_id: 1,
        cells: Vec::new(),
        cell_index: HashMap::new(),
        notices: Vec::new(),
        directives: Vec::new(),
        leads: Vec::new(),
        anchors: HashSet::new(),
        terminals: Vec::new(),
        names: BTreeSet::new(),
        models: BTreeSet::new(),
    };
    for symbol in &drawing.symbols {
        builder.place_symbol(symbol);
    }
    builder.place_flags();
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
    /// Every point the source drawing joins things at.
    anchors: HashSet<Point>,
    /// Every placed terminal, for the wire-connection cache.
    terminals: Vec<(u64, String, Point)>,
    names: BTreeSet<String>,
    /// The models native parts name, for the source program's cards.
    models: BTreeSet<String>,
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

    fn place_symbol(&mut self, symbol: &ForeignSymbol) {
        let tool = self.drawing.tool;
        let label = self.symbol_label(symbol);
        if tool == ForeignTool::Qspice
            && let Some(reason) = symbol
                .embedded
                .as_ref()
                .and_then(|definition| qspice_refusal(definition.symbol_type.trim()))
        {
            self.refuse(format!("{label} {reason}; it was left out."));
            self.place_marker(symbol, &label);
            return;
        }
        let (definition, location, schematic) = match &symbol.embedded {
            Some(definition) => (Some(definition.clone()), None, None),
            None => match self.access.symbol(&symbol.symbol) {
                Some(source) => (
                    Some(source.definition),
                    Some(source.location),
                    source.schematic,
                ),
                None => (ltspice_builtin_definition(symbol.base_name()), None, None),
            },
        };
        let Some(definition) = definition else {
            self.refuse(format!(
                "{label}: no symbol file `{}.asy` was found beside the schematic or in LTspice's symbol library, so its pins are unknown; it is drawn as a box with no pins.",
                symbol.symbol
            ));
            self.place_unknown_box(symbol);
            return;
        };
        let attributes = merged_attributes(&definition, symbol);
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
        match native_mapping(tool, symbol.base_name(), Some(&definition)) {
            Some(mapping) if schematic.is_none() => {
                self.place_native(symbol, &attributes, mapping, &pins);
            }
            _ => {
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

    fn symbol_label(&self, symbol: &ForeignSymbol) -> String {
        match symbol.attribute("InstName") {
            Some(name) => format!("{name} ({})", symbol.symbol),
            None => format!("Symbol {} on line {}", symbol.symbol, symbol.line),
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
        let value = attribute(attributes, "Value").unwrap_or_default();
        let mut words = Vec::new();
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
                        Some((key, value)) if !key.is_empty() && !value.is_empty() => {
                            params.push((key.to_owned(), value.to_owned()));
                        }
                        _ => extra.push(word),
                    }
                }
                (kind, spice_text(value), params, extra)
            }
            NativeKind::VoltageSource | NativeKind::CurrentSource => {
                let voltage = mapping.kind == NativeKind::VoltageSource;
                let mut text = value.to_owned();
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
        let (position, rotation, mirrored) = fit(&component, pins, offsets);
        component.pos = position;
        component.rotation = rotation;
        component.mirror_h = mirrored;
        let placed = component.terminal_positions();
        for (terminal, pin) in mapping.terminals.iter().zip(pins) {
            if let Some((_, point)) = placed.iter().find(|(name, _)| name == terminal) {
                self.lead(*point, *pin);
            }
        }
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
        let cell_index = self.symbol_cell(symbol, definition, location, schematic.clone());
        let cell = self.cells[cell_index].clone();
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
        let model = if subcircuit {
            cell.cell.clone()
        } else {
            attribute(attributes, "Value")
                .filter(|value| !value.trim().is_empty())
                .or_else(|| attribute(attributes, "Value2"))
                .map(spice_text)
                .unwrap_or_else(|| symbol.base_name().to_owned())
        };
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
        let (model, model_words) = split_model(&model);
        words.splice(0..0, model_words);
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
                    Some(found) => binding.source_path = Some(found.path),
                    None => self.refuse(format!(
                        "{label}: its model file `{file}` was not found, so model `{model}` is unresolved."
                    )),
                }
                break;
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
        let local = cell
            .symbol
            .pins
            .iter()
            .map(|pin| pin.position.unwrap_or_default())
            .collect::<Vec<_>>();
        let offsets = |candidate: &Component| -> Vec<Point> {
            local
                .iter()
                .map(|point| candidate.transform_point(*point))
                .collect()
        };
        let (position, rotation, mirrored) = fit(&component, pins, offsets);
        component.pos = position;
        component.rotation = rotation;
        component.mirror_h = mirrored;
        for ((port, offset), pin) in cell.ports.iter().zip(&local).zip(pins) {
            let point = position + component.transform_point(*offset);
            self.lead(point, *pin);
            self.terminals.push((id, port.name.clone(), point));
        }
        self.document.components.push(component);
    }

    /// The project cell for a symbol, made the first time the symbol is
    /// placed.
    fn symbol_cell(
        &mut self,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
        location: Option<String>,
        schematic: Option<PathBuf>,
    ) -> usize {
        // QSPICE embeds a symbol per component, so two components under one
        // name may differ; their drawings decide.
        let key = format!(
            "{}|{:?}",
            symbol.symbol.to_ascii_lowercase(),
            definition.pins
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
        let ports = unique_pin_names(definition);
        let mut document = SymbolDocument {
            pins: definition
                .ordered_pins()
                .iter()
                .zip(&ports)
                .map(|(pin, port)| {
                    SymbolPin::new(
                        port.name.clone(),
                        port.direction,
                        Some(self.scale(pin.position)),
                    )
                })
                .collect(),
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

    /// A note where a refused component stood, so the gap reads as one.
    fn place_marker(&mut self, symbol: &ForeignSymbol, label: &str) {
        let id = self.id();
        let text = format!("Not imported: {label}");
        if let Ok(note) = DesignNote::new(
            id,
            self.scale(symbol.position),
            DesignNoteKind::PlainText,
            text,
        ) {
            self.document.design_notes.push(note);
        }
    }

    /// A straight or once-bent lead from a placed terminal to the source pin
    /// it stands for.
    fn lead(&mut self, from: Point, to: Point) {
        if from == to {
            return;
        }
        if from.x == to.x || from.y == to.y {
            self.leads.push((from, to));
        } else {
            let corner = Point::new(to.x, from.y);
            self.leads.push((from, corner));
            self.leads.push((corner, to));
            self.anchors.insert(corner);
        }
    }

    fn place_flags(&mut self) {
        let mut ports = 0;
        for flag in &self.drawing.flags {
            let at = self.scale(flag.position);
            self.anchors.insert(at);
            let id = self.id();
            if flag.is_ground() {
                let mut ground = Component::new(id, ComponentType::Ground, at + Point::new(0, 10));
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
                if is_supply_rail(&flag.name) {
                    label.kind = NetLabelKind::Supply {
                        glyph: SupplyGlyph::for_name(&flag.name),
                    };
                }
                self.document.net_labels.push(label);
            }
        }
    }

    fn place_texts(&mut self) {
        for text in &self.drawing.texts {
            let position = self.scale(text.position);
            match text.kind {
                ForeignTextKind::Directive => self.directives.push(ForeignDirective {
                    position,
                    text: spice_text_lines(&text.text),
                }),
                ForeignTextKind::Comment => {
                    let id = self.id();
                    match DesignNote::new(id, position, DesignNoteKind::PlainText, text.text.trim())
                    {
                        Ok(note) => self.document.design_notes.push(note),
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
        let defined = defined_model_names(&foreign_directive_cards(&self.directives));
        let model_cards = self
            .models
            .iter()
            .filter(|model| !defined.contains(&model.to_ascii_lowercase()))
            .filter_map(|model| self.access.model_card(model))
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

fn attribute<'a>(attributes: &'a [ForeignAttribute], name: &str) -> Option<&'a str> {
    attributes
        .iter()
        .find(|attribute| attribute.name.eq_ignore_ascii_case(name))
        .map(|attribute| attribute.value.as_str())
}

/// A subcircuit name followed by words of its own instance line splits into
/// the name and those words.
fn split_model(model: &str) -> (String, Vec<String>) {
    let mut tokens = spice_tokens(model).into_iter();
    let name = tokens.next().unwrap_or_default();
    (name, tokens.collect())
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

/// The definition's pins as ports in instance-line order, each with a name
/// of its own.
fn unique_pin_names(definition: &ForeignSymbolDefinition) -> Vec<PortSpec> {
    let mut seen = BTreeSet::new();
    definition
        .ordered_pins()
        .iter()
        .map(|pin| {
            let base = if pin.name.trim().is_empty() {
                format!("P{}", pin.spice_order)
            } else {
                pin.name.trim().to_owned()
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

fn strictly_inside(a: Point, b: Point, point: Point) -> bool {
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

/// Turns and places a component so its terminals best meet the source pins.
///
/// Each of the eight orientations is tried at every position that puts one
/// terminal in line with its pin. A terminal on its pin costs nothing; one
/// in line costs its distance, and more when its lead would run back across
/// the body; one out of line costs most, since its lead must bend.
fn fit(
    component: &Component,
    pins: &[Point],
    offsets: impl Fn(&Component) -> Vec<Point>,
) -> (Point, Rotation, bool) {
    let mut best: Option<(i64, Point, Rotation, bool)> = None;
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
                    if best.as_ref().is_none_or(|(least, ..)| cost < *least) {
                        best = Some((cost, position, rotation, mirrored));
                    }
                }
            }
        }
    }
    match best {
        Some((_, position, rotation, mirrored)) => (position, rotation, mirrored),
        None => {
            let anchor = pins.first().copied().unwrap_or_default();
            (anchor, Rotation::R0, false)
        }
    }
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
