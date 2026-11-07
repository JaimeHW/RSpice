//! The imported schematic joins exactly what the source drawing joins, puts
//! every point on RSpice's grid, and keeps what it cannot place in view.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

use rspice_design_model::Point;

use super::*;
use crate::connectivity::extract;
use crate::resolved_symbol::ResolvedCellSymbol;
use crate::schematic::component_type::ComponentType;

/// A library that knows a few symbols and model files, as a user's LTspice
/// installation would.
#[derive(Default)]
struct Library {
    symbols: HashMap<String, ForeignSymbolSource>,
    models: HashMap<String, ForeignModelFile>,
    cards: HashMap<String, String>,
}

impl ForeignLibrary for Library {
    fn symbol(&self, name: &str) -> Option<ForeignSymbolSource> {
        self.symbols.get(&name.to_ascii_lowercase()).cloned()
    }

    fn model_file(&self, name: &str) -> Option<ForeignModelFile> {
        self.models.get(&name.to_ascii_lowercase()).cloned()
    }

    fn model_card(&self, name: &str) -> Option<String> {
        self.cards.get(&name.to_ascii_lowercase()).cloned()
    }
}

const AMPLIFIER_ASY: &str = "Version 4\nSymbolType CELL\nLINE Normal -32 -32 32 0\n\
    LINE Normal -32 32 32 0\nLINE Normal -32 -32 -32 32\nSYMATTR Prefix X\n\
    SYMATTR SpiceModel amp.sub\nSYMATTR Value amp\n\
    PIN 32 0 NONE 0\nPINATTR PinName OUT\nPINATTR SpiceOrder 3\n\
    PIN -32 -16 NONE 0\nPINATTR PinName IN+\nPINATTR SpiceOrder 1\n\
    PIN -32 16 NONE 0\nPINATTR PinName IN-\nPINATTR SpiceOrder 2\n";

const GATE_ASY: &str = "Version 4\nSymbolType CELL\nLINE Normal -16 -32 16 0\n\
    LINE Normal -16 32 16 0\nSYMATTR Prefix A\nSYMATTR Value AND\nSYMATTR SpiceLine Vhigh=1\n\
    PIN -16 -16 NONE 0\nPINATTR PinName A\nPINATTR SpiceOrder 1\n\
    PIN -16 16 NONE 0\nPINATTR PinName B\nPINATTR SpiceOrder 2\n\
    PIN 16 0 NONE 0\nPINATTR PinName Y\nPINATTR SpiceOrder 3\n";

fn library() -> Library {
    let mut library = Library::default();
    for (name, source, schematic) in [
        ("amp", AMPLIFIER_ASY, None),
        ("digital\\and", GATE_ASY, None),
        ("block", AMPLIFIER_ASY, Some(PathBuf::from("block.asc"))),
    ] {
        library.symbols.insert(
            name.to_owned(),
            ForeignSymbolSource {
                definition: parse_asy(source).expect("the test symbol reads"),
                location: format!("{name}.asy"),
                schematic,
            },
        );
    }
    library.models.insert(
        "amp.sub".to_owned(),
        ForeignModelFile {
            path: PathBuf::from("lib/amp.sub"),
            encrypted: false,
        },
    );
    library.models.insert(
        "vendor.lib".to_owned(),
        ForeignModelFile {
            path: PathBuf::from("lib/vendor.lib"),
            encrypted: true,
        },
    );
    library
}

/// The nets the source simulator reads from a drawing: wire ends, pins and
/// flags join where they meet, a point on the middle of a wire joins it, and
/// flags of one name are one net. Returns `(owner, pin index) -> net`.
fn reference_nets(
    drawing: &ForeignSchematic,
    library: &Library,
) -> BTreeMap<(String, usize), usize> {
    struct Nodes {
        points: Vec<ForeignPoint>,
        index: HashMap<ForeignPoint, usize>,
        parent: Vec<usize>,
    }
    impl Nodes {
        fn node(&mut self, point: ForeignPoint) -> usize {
            if let Some(item) = self.index.get(&point) {
                return *item;
            }
            self.points.push(point);
            self.parent.push(self.parent.len());
            self.index.insert(point, self.parent.len() - 1);
            self.parent.len() - 1
        }
        fn find(&mut self, item: usize) -> usize {
            let mut root = item;
            while self.parent[root] != root {
                root = self.parent[root];
            }
            self.parent[item] = root;
            root
        }
        fn join(&mut self, a: usize, b: usize) {
            let (a, b) = (self.find(a), self.find(b));
            self.parent[a] = b;
        }
    }
    let mut nodes = Nodes {
        points: Vec::new(),
        index: HashMap::new(),
        parent: Vec::new(),
    };
    let mut owners = Vec::new();
    for symbol in &drawing.symbols {
        let definition = symbol
            .embedded
            .clone()
            .or_else(|| {
                library
                    .symbol(&symbol.symbol)
                    .map(|source| source.definition)
            })
            .or_else(|| super::mapping::ltspice_builtin_definition(symbol.base_name()));
        let Some(definition) = definition else {
            continue;
        };
        let name = symbol.attribute("InstName").unwrap_or_default().to_owned();
        for (order, pin) in definition.ordered_pins().into_iter().enumerate() {
            let at = symbol
                .position
                .offset(symbol.orientation.apply(pin.position));
            owners.push(((name.clone(), order), nodes.node(at)));
        }
    }
    for flag in &drawing.flags {
        owners.push((
            (format!("flag:{}", flag.name), 0),
            nodes.node(flag.position),
        ));
    }
    for (a, b) in &drawing.wires {
        let (a, b) = (nodes.node(*a), nodes.node(*b));
        nodes.join(a, b);
    }
    for (a, b) in &drawing.wires {
        let end = nodes.node(*a);
        for (item, point) in nodes.points.clone().into_iter().enumerate() {
            let cross = (b.x - a.x) * (point.y - a.y) - (b.y - a.y) * (point.x - a.x);
            let inside = cross == 0
                && a.x.min(b.x) <= point.x
                && point.x <= a.x.max(b.x)
                && a.y.min(b.y) <= point.y
                && point.y <= a.y.max(b.y);
            if inside {
                nodes.join(item, end);
            }
        }
    }
    let mut by_name = HashMap::<String, usize>::new();
    for ((owner, _), item) in &owners {
        if let Some(name) = owner.strip_prefix("flag:") {
            match by_name.get(name) {
                Some(other) => nodes.join(*item, *other),
                None => {
                    by_name.insert(name.to_owned(), *item);
                }
            }
        }
    }
    owners
        .into_iter()
        .map(|(owner, item)| (owner, nodes.find(item)))
        .collect()
}

/// The same `(owner, pin index) -> net` reading of the imported schematic.
fn imported_nets(
    drawing: &ForeignSchematic,
    imported: &ForeignImport,
) -> BTreeMap<(String, usize), String> {
    let cells = imported
        .cells
        .iter()
        .map(|cell| (cell.cell.clone(), cell))
        .collect::<HashMap<_, _>>();
    let connectivity = extract(
        &imported.document,
        |binding| {
            cells.get(&binding.cell).map(|cell| {
                ResolvedCellSymbol::from_authored_document(cell.symbol.clone(), &cell.ports)
            })
        },
        |_| None,
    );
    let mut nets = BTreeMap::new();
    for component in &imported.document.components {
        let order: Vec<String> = match component.kind {
            ComponentType::Ground | ComponentType::Port => continue,
            ComponentType::Resistor
            | ComponentType::Capacitor
            | ComponentType::Inductor
            | ComponentType::VoltageSource
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::CurrentSource => {
                vec!["+".into(), "-".into()]
            }
            ComponentType::Diode => vec!["A".into(), "K".into()],
            ComponentType::NpnBjt | ComponentType::PnpBjt => {
                vec!["C".into(), "B".into(), "E".into()]
            }
            ComponentType::Nmos | ComponentType::Pmos => vec!["D".into(), "G".into(), "S".into()],
            ComponentType::Vcvs | ComponentType::Vccs => {
                vec!["O+".into(), "O-".into(), "C+".into(), "C-".into()]
            }
            ComponentType::CellInstance => cells[&component.library_cell.as_ref().unwrap().cell]
                .ports
                .iter()
                .map(|port| port.name.clone())
                .collect(),
            other => panic!("the test does not read {other:?}"),
        };
        let name = drawing
            .symbols
            .iter()
            .filter_map(|symbol| symbol.attribute("InstName"))
            .find(|name| component.name.ends_with(name))
            .unwrap_or(&component.name)
            .to_owned();
        for (index, pin) in order.iter().enumerate() {
            let terminal = connectivity
                .terminals
                .iter()
                .find(|terminal| terminal.component_id == component.id && &terminal.pin == pin)
                .unwrap_or_else(|| panic!("{} has no terminal {pin}", component.name));
            nets.insert((name.clone(), index), terminal.net_name.clone());
        }
    }
    let scale = |value: i32| match drawing.tool {
        ForeignTool::Ltspice => value * 5 / 4,
        ForeignTool::Qspice => value / 10,
    };
    for flag in &drawing.flags {
        let point = Point::new(scale(flag.position.x), scale(flag.position.y));
        // A flag that touches nothing is a net of its own.
        let name = match connectivity.net_at(point) {
            Some(net) => connectivity
                .terminals
                .iter()
                .find(|terminal| net.points.contains(&terminal.point))
                .map_or_else(
                    || net.label.clone().unwrap_or_default(),
                    |terminal| terminal.net_name.clone(),
                ),
            None => format!("alone:{}", flag.name),
        };
        nets.insert((format!("flag:{}", flag.name), 0), name);
    }
    nets
}

/// Groups owners by net, so two readings compare up to net names.
fn partition<T: Ord + Clone>(
    nets: &BTreeMap<(String, usize), T>,
) -> BTreeSet<BTreeSet<(String, usize)>> {
    let mut groups = BTreeMap::<T, BTreeSet<(String, usize)>>::new();
    for (owner, net) in nets {
        groups.entry(net.clone()).or_default().insert(owner.clone());
    }
    groups.into_values().collect()
}

fn assert_same_connectivity(
    drawing: &ForeignSchematic,
    imported: &ForeignImport,
    library: &Library,
) {
    let reference = partition(&reference_nets(drawing, library));
    let ours = partition(&imported_nets(drawing, imported));
    assert_eq!(ours, reference);
}

fn assert_on_grid(imported: &ForeignImport) {
    let on_grid = |point: Point| point.x % 10 == 0 && point.y % 10 == 0;
    for component in &imported.document.components {
        assert!(on_grid(component.pos), "{} is off the grid", component.name);
    }
    for wire in &imported.document.wires {
        for point in &wire.points {
            assert!(on_grid(*point), "a wire point {point:?} is off the grid");
        }
        let [a, b] = [wire.points[0], wire.points[1]];
        assert!(
            a.x == b.x || a.y == b.y,
            "a wire runs diagonally: {a:?} {b:?}"
        );
    }
}

const ORIENTATIONS: [&str; 8] = ["R0", "R90", "R180", "R270", "M0", "M90", "M180", "M270"];

#[test]
fn every_orientation_of_a_part_lands_its_pins_on_their_own_wires() {
    // One resistor per orientation, each pin led out by a wire to a flag.
    let mut source = String::from("Version 4\nSHEET 1 2000 2000\n");
    for (index, orientation) in ORIENTATIONS.iter().enumerate() {
        let origin = ForeignPoint::new(320 * (index as i32 % 4), 320 * (index as i32 / 4));
        let orientation_value = ForeignOrientation::from_ltspice(orientation).unwrap();
        for (pin, (x, y)) in [("a", (16, 16)), ("b", (16, 96))] {
            let at = origin.offset(orientation_value.apply(ForeignPoint::new(x, y)));
            let out = origin.offset(
                orientation_value.apply(ForeignPoint::new(x, if pin == "a" { -32 } else { 144 })),
            );
            source.push_str(&format!("WIRE {} {} {} {}\n", at.x, at.y, out.x, out.y));
            source.push_str(&format!("FLAG {} {} n{index}{pin}\n", out.x, out.y));
        }
        source.push_str(&format!(
            "SYMBOL res {} {} {orientation}\nSYMATTR InstName R{index}\nSYMATTR Value {}k\n",
            origin.x,
            origin.y,
            index + 1
        ));
    }
    let drawing = parse_asc(&source).unwrap();
    let library = Library::default();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert!(imported.notices.is_empty(), "{:?}", imported.notices);
    assert_same_connectivity(&drawing, &imported, &library);
    assert_on_grid(&imported);
    // Each resistor lies along its pins: vertical for R0/R180/M0/M180.
    for component in &imported.document.components {
        let index = component.name[1..].parse::<usize>().unwrap();
        let along_y = matches!(ORIENTATIONS[index], "R0" | "M0" | "R180" | "M180");
        assert_eq!(
            component.rotation.is_vertical(),
            along_y,
            "{}",
            component.name
        );
        assert_eq!(component.value, format!("{}k", index + 1));
    }
}

/// A transistor stage with a T-junction, ground, a supply rail, an I/O pin,
/// a three-pin MOSFET and a controlled source, every part turned.
const MIXED: &str = "Version 4
SHEET 1 1200 900
WIRE 160 64 64 64
WIRE 320 64 160 64
WIRE 160 160 160 64
WIRE 160 256 160 240
WIRE 64 176 64 64
WIRE 64 352 64 256
WIRE 96 304 64 304
WIRE 160 352 160 352
WIRE 480 160 320 160
WIRE 320 160 320 64
WIRE 544 208 480 208
WIRE 480 208 480 160
FLAG 64 352 0
FLAG 320 64 VCC
FLAG 544 208 out
IOPIN 544 208 Out
FLAG 160 352 0
FLAG 704 320 0
FLAG 704 224 ctl
SYMBOL voltage 64 160 R0
SYMATTR InstName V1
SYMATTR Value PULSE(0 5µ 0 1n 1n 5u 10u) AC 1 Rser=0.1
SYMBOL res 144 144 R0
SYMATTR InstName R1
SYMATTR Value 10k
SYMATTR SpiceLine tol=1 pwr=0.1
SYMBOL npn 96 256 R0
SYMATTR InstName Q1
SYMATTR Value 2N3904
SYMBOL nmos 112 256 M0
SYMATTR InstName M1
SYMATTR Value BSS123
SYMBOL e 704 224 R180
SYMATTR InstName E1
SYMATTR Value 10
SYMBOL diode 304 160 R270
SYMATTR InstName D1
SYMATTR Value 1N4148
";

#[test]
fn a_mixed_drawing_imports_with_its_connectivity_and_parameters() {
    let drawing = parse_asc(MIXED).unwrap();
    let library = Library::default();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_same_connectivity(&drawing, &imported, &library);
    assert_on_grid(&imported);

    let component = |name: &str| {
        imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("{name} was not placed"))
    };
    let source = component("V1");
    assert_eq!(source.kind, ComponentType::VoltageSourcePulse);
    assert_eq!(source.value, "PULSE(0 5u 0 1n 1n 5u 10u)");
    assert_eq!(source.params, "ac=1 rs=0.1");
    let resistor = component("R1");
    assert_eq!(resistor.params, "tol=1 pwr=0.1");
    assert_eq!(component("Q1").kind, ComponentType::NpnBjt);
    assert_eq!(component("Q1").value, "2N3904");
    assert_eq!(component("E1").kind, ComponentType::Vcvs);
    let mosfet = component("M1");
    assert_eq!(mosfet.kind, ComponentType::Nmos);
    // LTspice's three-pin MOSFET ties body to source: a lead joins them.
    let connectivity = extract(&imported.document, |_| None, |_| None);
    let net_of = |pin: &str| {
        connectivity
            .terminals
            .iter()
            .find(|terminal| terminal.component_id == mosfet.id && terminal.pin == pin)
            .map(|terminal| terminal.net_name.clone())
    };
    assert_eq!(net_of("B"), net_of("S"));
    assert!(net_of("B").is_some());

    let grounds = imported
        .document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::Ground)
        .count();
    assert_eq!(grounds, 3);
    let port = imported
        .document
        .components
        .iter()
        .find(|component| component.kind == ComponentType::Port)
        .expect("the I/O pin is a port");
    assert_eq!(
        (port.value.as_str(), port.params.as_str()),
        ("out", "dir=out")
    );
    let rail = imported
        .document
        .net_labels
        .iter()
        .find(|label| label.name == "VCC")
        .expect("the rail is labelled");
    assert!(rail.kind.supply_glyph().is_some());
    assert!(
        imported
            .document
            .net_labels
            .iter()
            .all(|label| label.name != "ctl" || label.kind.supply_glyph().is_none())
    );
    // Three wire ends meet at the T on the top rail: one dot.
    assert!(!imported.document.junctions.is_empty());
}

#[test]
fn a_symbol_from_the_library_keeps_its_pins_and_model_file() {
    let drawing = parse_asc(
        "Version 4\nSHEET 1 800 600\n\
         WIRE 32 48 -64 48\nWIRE 32 80 -64 80\nWIRE 192 64 96 64\n\
         FLAG -64 48 inp\nFLAG -64 80 inn\nFLAG 192 64 out\n\
         SYMBOL amp 64 64 R0\nSYMATTR InstName U1\nSYMATTR SpiceLine gain=2\n\
         SYMBOL amp 64 320 M180\nSYMATTR InstName U2\n\
         SYMBOL Digital\\and 448 64 R90\nSYMATTR InstName A7\n\
         TEXT 0 400 Left 2 !.tran 1m\n",
    )
    .unwrap();
    let library = library();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_same_connectivity(&drawing, &imported, &library);
    assert_on_grid(&imported);
    assert_eq!(imported.cells.len(), 2, "one cell per symbol");
    let amplifier = &imported.cells[0];
    assert_eq!(amplifier.cell, "amp");
    let ports = amplifier
        .ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ports, ["IN+", "IN-", "OUT"]);
    let instance = |name: &str| {
        imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap()
    };
    let u1 = instance("U1");
    let binding = u1.library_cell.as_ref().unwrap();
    assert_eq!(binding.library, "work");
    assert_eq!(binding.module_name.as_deref(), Some("amp"));
    assert_eq!(binding.source_path, Some(PathBuf::from("lib/amp.sub")));
    assert_eq!(binding.netlist_template, None);
    assert_eq!(u1.params, "gain=2");
    // An LTspice special function keeps its own device letter and model.
    let gate = instance("A7");
    let binding = gate.library_cell.as_ref().unwrap();
    assert_eq!(
        binding.netlist_template.as_deref(),
        Some("A{name} {nodes} {model} {params}")
    );
    assert_eq!(binding.module_name.as_deref(), Some("AND"));
    assert_eq!(binding.parameter_order, ["Vhigh"]);
    assert_eq!(gate.params, "Vhigh=1");
    assert_eq!(imported.directives.len(), 1);
    assert_eq!(imported.directives[0].text, ".tran 1m");
}

#[test]
fn a_symbol_with_no_definition_is_a_box_and_a_refusal() {
    let drawing =
        parse_asc("Version 4\nSYMBOL Mystery\\part 64 64 R0\nSYMATTR InstName U9\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    assert_eq!(imported.notices.len(), 1);
    assert_eq!(imported.notices[0].severity, ForeignNoticeSeverity::Refusal);
    assert!(imported.notices[0].message.contains("Mystery\\part.asy"));
    assert_eq!(imported.cells.len(), 1);
    assert!(imported.cells[0].ports.is_empty());
    assert!(!imported.cells[0].symbol.body.is_empty());
    assert_eq!(imported.document.components.len(), 1);
}

#[test]
fn a_hierarchical_block_names_its_schematic_and_its_own_cell() {
    let drawing = parse_asc("Version 4\nSYMBOL block 64 64 R0\nSYMATTR InstName X1\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let cell = &imported.cells[0];
    assert_eq!(cell.schematic, Some(PathBuf::from("block.asc")));
    let binding = imported.document.components[0]
        .library_cell
        .as_ref()
        .unwrap();
    assert_eq!(binding.module_name.as_deref(), Some("block"));
    assert_eq!(binding.source_path, None);
}

#[test]
fn an_encrypted_model_file_is_named_and_left_unresolved() {
    let mut library = library();
    let mut definition = parse_asy(AMPLIFIER_ASY).unwrap();
    definition
        .attributes
        .iter_mut()
        .find(|attribute| attribute.name == "SpiceModel")
        .unwrap()
        .value = "vendor.lib".to_owned();
    library.symbols.insert(
        "vendoramp".to_owned(),
        ForeignSymbolSource {
            definition,
            location: "vendoramp.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc(
        "Version 4\nSYMBOL VendorAmp 64 64 R0\nSYMATTR InstName U3\nSYMATTR Value LT9999\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let refusal = &imported.notices[0];
    assert_eq!(refusal.severity, ForeignNoticeSeverity::Refusal);
    assert!(refusal.message.contains("vendor.lib") && refusal.message.contains("LT9999"));
    let binding = imported.document.components[0]
        .library_cell
        .as_ref()
        .unwrap();
    assert_eq!(binding.source_path, None);
}

#[test]
fn directives_and_comments_keep_their_text_and_place() {
    let drawing = parse_asc(
        "Version 4\n\
         TEXT 32 32 Left 2 !.tran 0 10m 0 1µ\\n.step param R list 1k 2k\\n.meas tran vmax MAX V(out)\n\
         TEXT 32 96 Left 2 !.param R=1k\\n.include models.lib\\n.machine\\n.endmachine\n\
         TEXT 32 160 Left 2 ;Bias network\\nkeep R1 close\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let texts = imported
        .directives
        .iter()
        .map(|directive| directive.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        texts,
        [
            ".tran 0 10m 0 1u\n.step param R list 1k 2k\n.meas tran vmax MAX V(out)",
            ".param R=1k\n.include models.lib\n.machine\n.endmachine",
        ]
    );
    assert_eq!(imported.directives[1].position, Point::new(40, 120));
    let note = &imported.document.design_notes[0];
    assert_eq!(note.text, "Bias network\nkeep R1 close");
    assert_eq!(note.pos, Point::new(40, 200));
}

/// A QSPICE drawing: a source, a resistor, a DLL block and a comment, with
/// QSPICE's own net name on every wire.
const QSPICE: &str = "«schematic
  «component (0,0) 0 0
    «symbol V
      «type: V»
      «text (100,150) 1 7 0 0x1000000 -1 -1 \"V1\"»
      «text (100,-150) 1 7 0 0x1000000 -1 -1 \"SINE(0 1 1K)\"»
      «pin (0,200) (0,0) 1 0 0 0x1000000 -1 \"+\"»
      «pin (0,-200) (0,0) 1 0 0 0x1000000 -1 \"-\"»
    »
  »
  «component (600,400) 2 0
    «symbol R
      «type: R»
      «text (100,150) 1 7 0 0x1000000 -1 -1 \"R1\"»
      «text (100,-150) 1 7 0 0x1000000 -1 -1 \"1K\"»
      «pin (0,200) (0,0) 1 0 0 0x1000000 -1 \"1\"»
      «pin (0,-200) (0,0) 1 0 0 0x1000000 -1 \"2\"»
    »
  »
  «component (600,-600) 0 0
    «symbol Counter
      «type: Ø(.DLL)»
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"Ø1\"»
      «pin (-200,0) (0,0) 1 0 0 0x1000000 -1 \"clk\"»
    »
  »
  «net (0,-400) 1 13 0 \"GND\"»
  «net (1000,400) 1 13 0 \"GND\"»
  «wire (0,-200) (0,-400) \"GND\"»
  «wire (0,200) (0,400) \"in\"»
  «wire (0,400) (400,400) \"in\"»
  «wire (800,400) (1000,400) \"GND\"»
  «text (0,-900) 0.8 7 0 0x1000000 -1 -1 \".tran 3m\"»
»
";

fn qsch_bytes(text: &str) -> Vec<u8> {
    let mut bytes = vec![0xFF, 0xD8, 0xFF, 0xDB];
    for character in text.chars() {
        match character {
            '«' => bytes.push(0xAB),
            '»' => bytes.push(0xBB),
            'Ø' => bytes.push(0xD8),
            other => bytes.extend_from_slice(other.to_string().as_bytes()),
        }
    }
    bytes
}

#[test]
fn a_qspice_drawing_joins_what_its_wire_names_join_and_refuses_a_dll() {
    let drawing = parse_qsch(&qsch_bytes(QSPICE)).unwrap();
    let library = Library::default();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let refusals = imported
        .notices
        .iter()
        .filter(|notice| notice.severity == ForeignNoticeSeverity::Refusal)
        .collect::<Vec<_>>();
    assert_eq!(refusals.len(), 1);
    assert!(
        refusals[0].message.contains("Verilog-A"),
        "{}",
        refusals[0].message
    );
    let kinds = imported
        .document
        .components
        .iter()
        .map(|component| component.kind)
        .collect::<Vec<_>>();
    assert!(kinds.contains(&ComponentType::VoltageSourceSin));
    assert!(kinds.contains(&ComponentType::Resistor));
    assert_eq!(
        kinds
            .iter()
            .filter(|kind| **kind == ComponentType::Ground)
            .count(),
        2
    );
    assert!(
        imported
            .document
            .design_notes
            .iter()
            .any(|note| note.text.contains("Ø1"))
    );
    assert_on_grid(&imported);

    // QSPICE wrote each wire's net: the import joins the same wires.
    let nets = imported_nets(&drawing, &imported);
    assert_eq!(nets[&("V1".to_owned(), 1)], nets[&("flag:0".to_owned(), 0)]);
    assert_eq!(nets[&("R1".to_owned(), 1)], nets[&("flag:0".to_owned(), 0)]);
    assert_eq!(nets[&("V1".to_owned(), 0)], nets[&("R1".to_owned(), 0)]);
    assert_ne!(nets[&("V1".to_owned(), 0)], nets[&("V1".to_owned(), 1)]);
    assert_eq!(imported.directives[0].text, ".tran 3m");
}

#[test]
fn directive_cards_sort_by_the_project_place_they_belong_in() {
    let drawing = parse_asc(
        "Version 4\n\
         TEXT 0 0 Left 2 !.tran 0 1m\\n.param R=1k\\n+ C=1n\\n.model D1N D(Is=1n\\n+ Rs=1)\n\
         TEXT 0 64 Left 2 !.subckt half a b\\nR1 a b {R}\\n.ends half\\n.lib my.lib\\n.backanno\\nK1 L1 L2 1\\n.step param R 1k 2k 1k\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let cards = foreign_directive_cards(&imported.directives);
    let roles = cards
        .iter()
        .map(|card| (card.keyword(), card.role))
        .collect::<Vec<_>>();
    assert_eq!(
        roles,
        [
            (".tran".to_owned(), ForeignCardRole::Analysis),
            (".param".to_owned(), ForeignCardRole::Parameter),
            (".model".to_owned(), ForeignCardRole::ModelText),
            (".subckt".to_owned(), ForeignCardRole::ModelText),
            (".lib".to_owned(), ForeignCardRole::Include),
            (".backanno".to_owned(), ForeignCardRole::SourceOnly),
            ("k1".to_owned(), ForeignCardRole::Other),
            (".step".to_owned(), ForeignCardRole::Step),
        ]
    );
    assert_eq!(cards[1].tokens, [".param", "R=1k", "C=1n"]);
    assert_eq!(cards[2].text, ".model D1N D(Is=1n\n+ Rs=1)");
    assert_eq!(cards[3].text, ".subckt half a b\nR1 a b {R}\n.ends half");
    assert_eq!(cards[3].position, Point::new(0, 80));
    assert_eq!(defined_model_names(&cards), ["d1n", "half"]);
}

#[test]
fn a_model_the_drawing_does_not_define_comes_from_the_source_library() {
    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL diode 0 0 R0\nSYMATTR InstName D1\nSYMATTR Value 1N4148\n\
         SYMBOL diode 128 0 R0\nSYMATTR InstName D2\nSYMATTR Value MYD\n\
         SYMBOL res 256 0 R0\nSYMATTR InstName R1\nSYMATTR Value 1k\n\
         TEXT 0 200 Left 2 !.model MYD D(Is=1n)\n",
    )
    .unwrap();
    let mut library = Library::default();
    for name in ["1n4148", "myd"] {
        library
            .cards
            .insert(name.to_owned(), format!(".model {name} D(Is=2n)"));
    }
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_eq!(imported.model_cards, [".model 1n4148 D(Is=2n)"]);
}

#[test]
fn the_installed_library_reads_symbols_blocks_and_cards_in_place() {
    let root = std::env::temp_dir().join(format!(
        "rspice-foreign-library-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    ));
    let beside = root.join("drawing");
    let install = root.join("lib");
    for folder in [
        &beside,
        &install.join("sym/OpAmps"),
        &install.join("cmp"),
        &install.join("sub"),
    ] {
        std::fs::create_dir_all(folder).expect("fixture folder");
    }
    std::fs::write(beside.join("block.asy"), AMPLIFIER_ASY).unwrap();
    std::fs::write(beside.join("block.asc"), "Version 4\n").unwrap();
    std::fs::write(install.join("sym/OpAmps/amp.asy"), AMPLIFIER_ASY).unwrap();
    std::fs::write(
        install.join("cmp/standard.dio"),
        "* diodes\n.model 1N4148 D(Is=2.52n Rs=.568\n+ N=1.752)\n.model OTHER D\n",
    )
    .unwrap();
    std::fs::write(
        install.join("sub/locked.lib"),
        "* LTspice Encrypted File\n0123456789\n",
    )
    .unwrap();
    std::fs::write(install.join("sub/open.lib"), ".subckt amp 1 2 3\n.ends\n").unwrap();

    let library = InstalledForeignLibrary::new(ForeignTool::Ltspice, &beside, Some(&install));
    let block = library
        .symbol("block")
        .expect("the block symbol beside the drawing");
    assert_eq!(block.schematic, Some(beside.join("block.asc")));
    let amp = library.symbol("OpAmps\\amp").expect("an installed symbol");
    assert_eq!(amp.schematic, None);
    assert_eq!(amp.definition.pins.len(), 3);
    assert!(library.symbol("missing").is_none());
    assert_eq!(
        library.model_card("1n4148").as_deref(),
        Some(".model 1N4148 D(Is=2.52n Rs=.568\n+ N=1.752)")
    );
    assert!(library.model_card("absent").is_none());
    assert!(library.model_file("locked.lib").expect("found").encrypted);
    let open = library.model_file("open.lib").expect("found");
    assert!(!open.encrypted);
    assert_eq!(open.path, install.join("sub").join("open.lib"));
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_coupling_card_over_drawn_inductors_is_a_coupling_part() {
    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL ind 0 0 R0\nSYMATTR InstName L1\nSYMATTR Value 10u\n\
         SYMBOL ind 128 0 R0\nSYMATTR InstName L2\nSYMATTR Value 40u\n\
         SYMBOL cap 256 0 R0\nSYMATTR InstName C1\nSYMATTR Value 1u\nSYMATTR Value2 x2\n\
         TEXT 0 200 Left 2 !K1 L1 L2 0.98\n\
         TEXT 0 260 Left 2 !K2 L1 L9 1\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let coupling = imported
        .document
        .components
        .iter()
        .find(|component| component.kind == ComponentType::CoupledInductor)
        .expect("a coupling part");
    assert_eq!(
        (
            coupling.name.as_str(),
            coupling.value.as_str(),
            coupling.params.as_str()
        ),
        ("K1", "0.98", "inductors=\"L1 L2\"")
    );
    assert_eq!(coupling.pos, Point::new(0, 250));
    // A card naming a winding the drawing lacks stays directive text.
    let texts = imported
        .directives
        .iter()
        .map(|directive| directive.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(texts, ["K2 L1 L9 1"]);
    let capacitor = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "C1")
        .expect("the capacitor");
    assert_eq!(capacitor.params, "m=2");
}
