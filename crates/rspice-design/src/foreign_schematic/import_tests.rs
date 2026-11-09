//! The imported schematic joins exactly what the source drawing joins, puts
//! every point on RSpice's grid, and keeps what it cannot place in view.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use rspice_design_model::Point;

use super::*;
use crate::connectivity::extract;
use crate::resolved_symbol::ResolvedCellSymbol;
use crate::schematic::component_type::ComponentType;
use crate::schematic::rotation::Rotation;

/// A library that knows a few symbols and model files, as a user's LTspice
/// installation would.
#[derive(Default)]
struct Library {
    symbols: HashMap<String, ForeignSymbolSource>,
    models: HashMap<String, ForeignModelFile>,
    cards: HashMap<String, String>,
    blocks: HashMap<String, PathBuf>,
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

    fn block_schematic(&self, name: &str) -> Option<PathBuf> {
        self.blocks.get(&name.to_ascii_lowercase()).cloned()
    }
}

const UNIVERSAL_OPAMP_ASY: &str = "Version 4\nSymbolType CELL\nLINE Normal -32 -32 32 0\n\
        LINE Normal -32 32 32 0\nLINE Normal -32 -32 -32 32\nSYMATTR Prefix X\n\
        SYMATTR SpiceModel level2\nSYMATTR Value2 Avol=1Meg GBW=10Meg Slew=10Meg\n\
        SYMATTR SpiceLine Ilimit=25m Rail=0 Vos=0\n\
        SYMATTR SpiceLine2 En=0 Enk=0 In=0 Ink=0 Rin=500Meg\n\
        SYMATTR ModelFile UniversalOpAmp2.lib\n\
        PIN -32 16 NONE 0\nPINATTR PinName In+\nPINATTR SpiceOrder 1\n\
        PIN -32 -16 NONE 0\nPINATTR PinName In-\nPINATTR SpiceOrder 2\n\
        PIN 0 -32 NONE 0\nPINATTR PinName V+\nPINATTR SpiceOrder 3\n\
        PIN 0 32 NONE 0\nPINATTR PinName V-\nPINATTR SpiceOrder 4\n\
        PIN 32 0 NONE 0\nPINATTR PinName OUT\nPINATTR SpiceOrder 5\n";

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

/// A symbol whose pins skip place 3 of the instance line, as vendor symbols
/// over a model with a pin left out do.
const GAPPED_ASY: &str = "Version 4
SymbolType CELL
RECTANGLE Normal -32 -32 32 32
    SYMATTR Prefix X
SYMATTR SpiceModel gapped.sub
SYMATTR Value gapped
    PIN -32 0 NONE 0
PINATTR PinName IN
PINATTR SpiceOrder 1
    PIN 32 0 NONE 0
PINATTR PinName OUT
PINATTR SpiceOrder 2
    PIN 0 32 NONE 0
PINATTR PinName REF
PINATTR SpiceOrder 4
";

fn library() -> Library {
    let mut library = Library::default();
    for (name, source, schematic) in [
        ("amp", AMPLIFIER_ASY, None),
        ("digital\\and", GATE_ASY, None),
        ("block", AMPLIFIER_ASY, Some(PathBuf::from("block.asc"))),
        ("gapped", GAPPED_ASY, None),
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
/// flags join where they meet, a point on the middle of a wire joins it,
/// flags of one name are one net, and an LTspice A-device's pin that meets
/// nothing is ground. Returns `(owner, pin index) -> net`.
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
    let mut devices = BTreeSet::new();
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
        if drawing.tool == ForeignTool::Ltspice
            && definition
                .attribute("Prefix")
                .is_some_and(|prefix| prefix.trim().eq_ignore_ascii_case("A"))
        {
            devices.insert(name.clone());
        }
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
    let wired = drawing
        .wires
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .map(|point| nodes.node(point))
        .collect::<Vec<_>>();
    let wired = wired
        .into_iter()
        .map(|item| nodes.find(item))
        .collect::<BTreeSet<_>>();
    let mut sharing = HashMap::<usize, usize>::new();
    for (_, item) in &owners {
        *sharing.entry(nodes.find(*item)).or_default() += 1;
    }
    let alone = owners
        .iter()
        .filter(|((owner, _), item)| {
            let root = nodes.find(*item);
            devices.contains(owner) && sharing[&root] == 1 && !wired.contains(&root)
        })
        .map(|(_, item)| *item)
        .collect::<Vec<_>>();
    if let Some((first, rest)) = alone.split_first() {
        let ground = *by_name.entry("0".to_owned()).or_insert(*first);
        for item in std::iter::once(first).chain(rest) {
            nodes.join(*item, ground);
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
            ComponentType::VSwitch => vec!["1".into(), "2".into(), "c+".into(), "c-".into()],
            ComponentType::Vcvs | ComponentType::Vccs => {
                vec!["O+".into(), "O-".into(), "C+".into(), "C-".into()]
            }
            ComponentType::CellInstance => {
                let cell = cells[&component.library_cell.as_ref().unwrap().cell];
                cell.ports
                    .iter()
                    .filter(|port| !cell.unconnected.contains(&port.name))
                    .map(|port| port.name.clone())
                    .collect()
            }
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
        ForeignTool::Ltspice => value * 5 / 8,
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

fn assert_wires_on_grid(imported: &ForeignImport) {
    let on_grid = |point: Point| point.x % 10 == 0 && point.y % 10 == 0;
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
    assert_wires_on_grid(&imported);
    // Each resistor lies along its pins: vertical for R0/R180/M0/M180.
    for component in &imported.document.components {
        let index = component.name[1..].parse::<usize>().unwrap();
        let along_y = matches!(ORIENTATIONS[index], "R0" | "M0" | "R180" | "M180");
        let [(_, first), (_, second)] = component.terminal_positions()[..] else {
            panic!("{} has two terminals", component.name);
        };
        assert_eq!(first.x == second.x, along_y, "{}", component.name);
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
SYMATTR Value2 tol=5
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
    assert_wires_on_grid(&imported);

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
    assert_eq!(source.value, "0");
    assert_eq!(
        source.params,
        "v2=5u td=0 tr=1n tf=1n pw=5u per=10u ac=1 rs=0.1"
    );
    let resistor = component("R1");
    // `Value2` and `SpiceLine` both set `tol`; the later one holds.
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
    assert_wires_on_grid(&imported);
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
    assert_eq!(imported.directives[1].position, Point::new(20, 60));
    let note = imported
        .document
        .design_notes
        .iter()
        .find(|note| note.text == "Bias network\nkeep R1 close")
        .expect("the comment is lettered");
    // The note stands on its drawing's point, and how it hangs from there
    // is its lettering's to say.
    assert_eq!(note.pos, Point::new(20, 100));
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
      «text (100,-150) 1 7 0 0x1000000 -1 -1 \"1K tc1=1m\"»
      «pin (0,200) (0,0) 1 0 0 0x1000000 -1 \"1\"»
      «pin (0,-200) (0,0) 1 0 0 0x1000000 -1 \"2\"»
    »
  »
  «component (600,-600) 0 0
    «symbol
      «type: Ø(.DLL)»
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"Ø1\"»
      «pin (-200,0) (0,0) 1 0 0 0x1000000 -1 \"clk\"»
    »
  »
  «net (0,-400) 1 13 0 \"GND\"»
  «net (1000,400) 1 7 0 \"GND\"»
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
            '¥' => bytes.push(0xA5),
            'Ã' => bytes.push(0xC3),
            'µ' => bytes.push(0xB5),
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
    // A ground QSPICE draws running right faces right, its terminal on the
    // net label's point.
    let grounds = imported
        .document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::Ground)
        .map(|ground| (ground.rotation, ground.terminal_positions()[0].1))
        .collect::<Vec<_>>();
    assert_eq!(
        grounds,
        [
            (Rotation::R0, Point::new(0, 40)),
            (Rotation::R270, Point::new(100, -40))
        ]
    );
    // The block draws no symbol name: its note names it alone.
    assert!(
        imported
            .document
            .design_notes
            .iter()
            .any(|note| note.text == "Not imported:\nØ1")
    );
    assert_wires_on_grid(&imported);

    // QSPICE wrote each wire's net: the import joins the same wires.
    let nets = imported_nets(&drawing, &imported);
    assert_eq!(nets[&("V1".to_owned(), 1)], nets[&("flag:0".to_owned(), 0)]);
    assert_eq!(nets[&("R1".to_owned(), 1)], nets[&("flag:0".to_owned(), 0)]);
    assert_eq!(nets[&("V1".to_owned(), 0)], nets[&("R1".to_owned(), 0)]);
    // QSPICE writes a part's settings after its value in one text.
    let resistor = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "R1")
        .expect("R1 was placed");
    assert_eq!(
        (resistor.value.as_str(), resistor.params.as_str()),
        ("1K", "tc1=1m")
    );
    assert_ne!(nets[&("V1".to_owned(), 0)], nets[&("V1".to_owned(), 1)]);
    assert_eq!(imported.directives[0].text, ".tran 3m");
}

/// A drawn current source that LTspice flags as a load keeps its one
/// waveform and the flag on its card.
#[test]
fn an_imported_load_keeps_its_waveform_and_flag_on_its_card() {
    let drawing = parse_asc(
        "Version 4\nSYMBOL current 0 0 R0\nSYMATTR InstName I1\n\
         SYMATTR Value PULSE(.2 5 300u 100n 100n 300u 600u)\nSYMATTR SpiceLine load\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let load = imported
        .document
        .components
        .iter()
        .find(|part| part.name == "I1")
        .expect("I1 is placed");
    assert_eq!(load.kind, ComponentType::CurrentSourcePulse);
    assert_eq!(load.value, "PULSE(.2 5 300u 100n 100n 300u 600u) load");
    assert!(load.params.is_empty(), "{}", load.params);
}

/// LTspice netlists a block's line written `..param` as `.param`, whatever
/// else shares the block.
#[test]
fn a_doubled_dot_directive_line_is_its_single_dot_card() {
    let drawing = parse_asc(
        "Version 4\nTEXT 0 0 Left 2 !..param Vdd = 15\\n..param Vss = -15\\n.tran 100u\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let cards = foreign_directive_cards(&imported.directives);
    assert_eq!(
        cards
            .iter()
            .map(|card| (card.keyword(), card.role))
            .collect::<Vec<_>>(),
        [
            (".param".to_owned(), ForeignCardRole::Parameter),
            (".param".to_owned(), ForeignCardRole::Parameter),
            (".tran".to_owned(), ForeignCardRole::Analysis),
        ]
    );
    assert_eq!(cards[1].text, ".param Vss = -15");
}

/// A value or directive drawn with the micro sign keeps it on the sheet,
/// while the part's value and the deck's card read `u`.
#[test]
fn a_micro_sign_stays_drawn_and_reads_as_micro_on_the_card() {
    use crate::schematic::value_label::value_pieces;

    let drawing = parse_asc(
        "Version 4\nSYMBOL res 0 0 R0\nWINDOW 3 36 76 Left 2\nSYMATTR InstName R1\nSYMATTR Value 100\u{B5}\n\
         TEXT 0 0 Left 2 !.tran 250\u{B5} startup\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let part = imported
        .document
        .components
        .iter()
        .find(|part| part.name == "R1")
        .expect("R1 is placed");
    assert_eq!(part.value, "100u");
    let pieces = value_pieces(part).expect("the micro sign is lettered");
    assert_eq!(
        pieces
            .iter()
            .map(|piece| piece.text.as_str())
            .collect::<Vec<_>>(),
        ["100\u{B5}"]
    );
    assert_eq!(
        imported
            .document
            .design_notes
            .iter()
            .map(|note| note.text.as_str())
            .collect::<Vec<_>>(),
        [".tran 250\u{B5} startup"]
    );
    let cards = foreign_directive_cards(&imported.directives);
    assert_eq!(cards[0].text, ".tran 250u startup");
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
    assert_eq!(cards[3].position, Point::new(0, 40));
    assert_eq!(defined_model_names(&cards), ["d1n", "half"]);
}

#[test]
fn a_prefix_set_on_the_placed_symbol_decides_the_device_it_is() {
    // LTspice: `SYMATTR Prefix X` on an `npn` netlists XQ1 over the model.
    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL npn 0 0 R0\nSYMATTR InstName Q1\nSYMATTR Value MYSUB\nSYMATTR Prefix X\n\
         SYMBOL npn 256 0 R0\nSYMATTR InstName Q2\nSYMATTR Value 2N3904\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let part = |name: &str| {
        imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("{name} in {:?}", imported.document.components))
    };
    let binding = part("Q1")
        .library_cell
        .as_ref()
        .expect("a cell over the model");
    assert_eq!(binding.reference_prefix.as_deref(), Some("X"));
    assert_eq!(part("Q1").value, "MYSUB");
    assert_eq!(part("Q2").kind, ComponentType::NpnBjt);
}

#[test]
fn a_two_letter_device_prefix_reads_its_model_from_the_source_library() {
    const TO92: &str = "Version 4\nSymbolType CELL\nRECTANGLE Normal -32 -32 32 32\n\
        SYMATTR Prefix QP\nSYMATTR Value MYPNP\n\
        PIN -32 0 NONE 0\nPINATTR PinName C\nPINATTR SpiceOrder 1\n\
        PIN 32 0 NONE 0\nPINATTR PinName B\nPINATTR SpiceOrder 2\n\
        PIN 0 32 NONE 0\nPINATTR PinName E\nPINATTR SpiceOrder 3\n";
    let mut source = library();
    source.symbols.insert(
        "to92".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(TO92).expect("the test symbol reads"),
            location: "to92.asy".to_owned(),
            schematic: None,
        },
    );
    source
        .cards
        .insert("mypnp".to_owned(), ".model MYPNP PNP".to_owned());
    let drawing = parse_asc("Version 4\nSYMBOL to92 0 0 R0\nSYMATTR InstName Q1\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &source);
    assert_eq!(imported.model_cards, [".model MYPNP PNP"]);
}

/// A QSPICE drawing placing one part whose symbol holds the `library` record.
fn qspice_part_with_library(library_record: &str, value: &str) -> ForeignSchematic {
    let text = format!(
        "«schematic
  «component (0,0) 0 0
    «symbol QN
      «type: QN»
      {library_record}
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"Q1\"»
      «text (0,-300) 1 7 0 0x1000000 -1 -1 \"{value}\"»
      «pin (100,200) (0,0) 1 0 0 0x0 -1 \"C\"»
      «pin (-200,0) (0,0) 1 0 0 0x0 -1 \"B\"»
      «pin (100,-200) (0,0) 1 0 0 0x0 -1 \"E\"»
    »
  »
»
"
    );
    parse_qsch(&qsch_bytes(&text)).unwrap()
}

#[test]
fn a_qspice_symbol_carries_its_own_model() {
    let drawing = qspice_part_with_library("«library file: |.model MYNPN NPN(BF=120)»", "MYNPN");
    let mut source = library();
    source
        .cards
        .insert("mynpn".to_owned(), ".model MYNPN NPN(BF=9)".to_owned());
    let imported = import_foreign_schematic(&drawing, "work", &source);
    // The text the symbol carries, not the installed library's card.
    assert_eq!(imported.model_cards, [".model MYNPN NPN(BF=120)"]);
}

#[test]
fn a_qspice_symbol_carries_its_own_subcircuit_on_one_line() {
    let drawing = qspice_part_with_library(
        "«library file: |.subckt MYSUB a b\\nR1 a b 1k\\n.ends MYSUB»",
        "MYSUB",
    );
    let imported = import_foreign_schematic(&drawing, "work", &library());
    assert_eq!(
        imported.model_cards,
        [".subckt MYSUB a b\nR1 a b 1k\n.ends MYSUB"]
    );
}

#[test]
fn a_qspice_library_named_by_file_stays_for_the_installed_library() {
    let drawing = qspice_part_with_library("«library file: NPN.txt»", "MYNPN");
    let mut source = library();
    source
        .cards
        .insert("mynpn".to_owned(), ".model MYNPN NPN(BF=9)".to_owned());
    let imported = import_foreign_schematic(&drawing, "work", &source);
    assert_eq!(imported.model_cards, [".model MYNPN NPN(BF=9)"]);
}

#[test]
fn a_qspice_block_names_the_schematic_its_value_names() {
    const DRAWING: &str = "«schematic
  «component (0,0) 0 0
    «symbol
      «shorted pins: false»
      «rect (-500,500) (500,-500) 0 0 0 0x4000000 0xffffff -1 1 -1»
      «text (0,750) 1 12 0 0x1000000 -1 -1 \"X1\"»
      «text (0,-300) 1.36 13 0 0x1000000 -1 -1 \"Buck\"»
      «pin (-500,0) (0,0) 1 11 0 0x0 -1 \"IN\"»
      «pin (500,0) (0,0) 1 7 0 0x0 -1 \"OUT\"»
    »
  »
  «net (-500,0) 1 11 0 \"vin\"»
  «net (500,0) 1 7 0 \"vout\"»
»
";
    let drawing = parse_qsch(&qsch_bytes(DRAWING)).unwrap();
    let mut source = library();
    source
        .blocks
        .insert("buck".to_owned(), PathBuf::from("Buck.qsch"));
    let imported = import_foreign_schematic(&drawing, "work", &source);
    let [cell] = imported.cells.as_slice() else {
        panic!("{:?}", imported.cells);
    };
    assert_eq!(cell.cell, "Buck");
    assert_eq!(cell.schematic.as_deref(), Some(Path::new("Buck.qsch")));
    // Without such a schematic the value is the subcircuit it names.
    let imported = import_foreign_schematic(&drawing, "work", &library());
    assert!(imported.cells.iter().all(|cell| cell.schematic.is_none()));
}

#[test]
fn a_qspice_net_flagged_as_a_port_is_a_port_of_its_drawing() {
    const DRAWING: &str = "«schematic
  «net (0,0) 1 7 1 \"LX\"»
  «net (500,0) 1 7 0 \"VCC\"»
»
";
    let drawing = parse_qsch(&qsch_bytes(DRAWING)).unwrap();
    let flag = |name: &str| drawing.flags.iter().find(|flag| flag.name == name).unwrap();
    assert_eq!(flag("LX").port, Some(ForeignPortDirection::Bidirectional));
    assert_eq!(flag("VCC").port, None);
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
fn a_coupling_line_of_a_block_keeps_its_place_among_the_blocks_lines() {
    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL ind 0 0 R0\nSYMATTR InstName L1\nSYMATTR Value 10u\n\
         SYMBOL ind 128 0 R0\nSYMATTR InstName L2\nSYMATTR Value 40u\n\
         TEXT 0 192 Left 2 !K1 L1 L2 1\\n.tran 1\\nK2 L1 L2 1\\n.op\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let couplings = imported
        .document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::CoupledInductor)
        .map(|component| component.pos)
        .collect::<Vec<_>>();
    let advance = 17.5;
    assert_eq!(
        couplings,
        [
            Point::new(0, 120),
            Point::new(0, 120 + (2.0 * advance) as i32)
        ]
    );
    let [note] = &imported.document.design_notes[..] else {
        panic!("one note");
    };
    assert_eq!(note.text, ".tran 1\n\n.op");
    // The note begins on its first drawn line, as if the source had put it there.
    let plain = parse_asc("Version 4\nTEXT 0 220 Left 2 !.tran 1\\n\\n.op\n").unwrap();
    let plain = import_foreign_schematic(&plain, "work", &Library::default());
    assert_eq!(note.pos, plain.document.design_notes[0].pos);
    let [directive] = &imported.directives[..] else {
        panic!("one directive");
    };
    assert_eq!(directive.text, ".tran 1\n.op");
}

#[test]
fn a_coupling_card_over_drawn_inductors_is_a_coupling_part() {
    use crate::schematic::component_display::ComponentDisplayMode;
    use crate::schematic::value_label::value_pieces;

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
    assert_eq!(coupling.pos, Point::new(0, 125));
    // The drawing shows the statement and no part, and so does the import.
    assert!(coupling.source_symbol().is_some_and(|symbol| {
        symbol.document().body.is_empty() && symbol.document().pins.is_empty()
    }));
    assert_eq!(coupling.display_mode, ComponentDisplayMode::ValueOnly);
    let pieces = value_pieces(coupling).expect("the part letters its statement");
    assert_eq!(pieces[0].text, "K1 L1 L2 0.98");
    // A card naming a winding the drawing lacks stays directive text.
    let texts = imported
        .directives
        .iter()
        .map(|directive| directive.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(texts, ["K2 L1 L9 1"]);
    let notes = imported
        .document
        .design_notes
        .iter()
        .map(|note| note.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(notes, ["K2 L1 L9 1"], "no note repeats the statement");
    let capacitor = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "C1")
        .expect("the capacitor");
    assert_eq!(capacitor.params, "m=2");
}

#[test]
fn names_and_values_stand_where_the_drawing_lettered_them() {
    use crate::schematic::component_display::ComponentDisplayMode;
    use crate::schematic::label_layout::{LayoutPoint, TextEdge, instance_label_layout};

    // A resistor turned a quarter with LTspice's own windows for that turn,
    // an amplifier mirrored with its value window moved, and a capacitor
    // whose name window is hidden.
    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL res 192 80 R90\nWINDOW 0 0 56 VBottom 2\nWINDOW 3 32 56 VTop 2\n\
         SYMATTR InstName R1\nSYMATTR Value 1k\n\
         SYMBOL amp 480 160 M0\nWINDOW 3 -32 48 Left 2\nSYMATTR InstName U1\n\
         SYMBOL cap 640 64 R0\nWINDOW 0 24 8 Invisible 2\nSYMATTR InstName C1\nSYMATTR Value 1u\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let component = |name: &str| {
        imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("{name} is placed"))
    };
    let layout = |name: &str| {
        let component = component(name);
        instance_label_layout(
            LayoutPoint::new(component.pos.x as f32, component.pos.y as f32),
            1.0,
            component,
        )
    };
    let edges = |text: crate::schematic::label_layout::PlacedText| {
        (text.anchor.x, text.anchor.y, text.horizontal, text.vertical)
    };

    // LTspice letters a turned resistor's name above it and its value below,
    // each centred on the body: (136, 80) and (136, 112) on its grid.
    let resistor = layout("R1");
    assert_eq!(
        edges(resistor.name),
        (85.0, 50.0, TextEdge::Center, TextEdge::End)
    );
    assert_eq!(
        edges(resistor.value),
        (85.0, 70.0, TextEdge::Center, TextEdge::Start)
    );
    // Mirrored, a run that grew rightward grows leftward from (512, 208).
    let amplifier = layout("U1");
    assert_eq!(
        edges(amplifier.value),
        (320.0, 130.0, TextEdge::End, TextEdge::Center)
    );
    assert_eq!(
        component("C1").display_mode,
        ComponentDisplayMode::ValueOnly
    );
}

/// LTspice turns a window's text with its symbol and never letters it
/// upside down: a vertical window on a symbol turned a half reads upward, as
/// does an upright window on a symbol turned a quarter, while a vertical
/// window on a symbol turned a quarter reads across. Two vertical windows
/// side by side, lettered upright, ran over each other; read upward, each
/// starts where the drawing anchored it and they stand apart.
#[test]
fn an_ltspice_vertical_window_reads_upward_as_its_symbol_turns_it() {
    use crate::schematic::label_layout::{
        INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD, LayoutPoint, TextEdge, instance_label_layout,
        text_box,
    };

    let drawing = parse_asc(
        "Version 4
         SYMBOL diode 0 0 R180
WINDOW 0 72 -48 VRight 2
WINDOW 3 48 -48 VRight 2
         SYMATTR InstName D7
SYMATTR Value SCHOTTKY_40V
         SYMBOL res 320 0 R90
WINDOW 0 0 56 VBottom 2
WINDOW 3 32 56 Left 2
         SYMATTR InstName R4
SYMATTR Value 4k7
",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let part = |name: &str| {
        imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("{name} is placed"))
    };
    let layout = |name: &str| {
        let part = part(name);
        instance_label_layout(
            LayoutPoint::new(part.pos.x as f32, part.pos.y as f32),
            1.0,
            part,
        )
    };

    // The diode's name and value read upward from where the drawing anchored
    // them, (-72, 48) and (-48, 48), centred across their line.
    let diode = layout("D7");
    for (placed, x) in [(diode.name, -45.0), (diode.value, -30.0)] {
        assert!(placed.upward, "{placed:?} reads upward");
        assert_eq!(
            (
                placed.anchor.x,
                placed.anchor.y,
                placed.horizontal,
                placed.vertical
            ),
            (x, 30.0, TextEdge::Start, TextEdge::Center)
        );
    }
    let name = text_box(diode.name, "D7", INSTANCE_NAME_WORLD);
    let value = text_box(diode.value, "SCHOTTKY_40V", INSTANCE_VALUE_WORLD);
    assert!(
        value.max.y - value.min.y > value.max.x - value.min.x,
        "the value stands taller than it is wide: {value:?}"
    );
    assert!(
        name.max.x <= value.min.x,
        "the name at {name:?} stands clear of the value at {value:?}"
    );

    // The turned resistor's vertical name window reads across; its upright
    // value window reads upward.
    let resistor = layout("R4");
    assert!(!resistor.name.upward, "{:?} reads across", resistor.name);
    assert!(resistor.value.upward, "{:?} reads upward", resistor.value);
}

/// QSPICE justifies a symbol's text on both axes. A value that starts at a
/// point on its body's lower edge and hangs below it stands outside the
/// body; read on one axis only, it was centred on the edge and ran across
/// the outline.
#[test]
fn a_qspice_value_hung_below_its_anchor_stays_off_the_edge_it_starts_on() {
    use crate::schematic::label_layout::{
        INSTANCE_VALUE_WORLD, LayoutPoint, TextEdge, instance_label_layout, text_box,
    };

    let drawing = parse_qsch(&qsch_bytes(
        "«schematic
  «component (0,0) 0 0
    «symbol AMP
      «type: X»
      «line (-300,300) (300,0) 0 0 0x1000000 -1 -1»
      «line (-300,-300) (300,0) 0 0 0x1000000 -1 -1»
      «line (-300,300) (-300,-300) 0 0 0x1000000 -1 -1»
      «text (-100,0) 1 15 0 0x1000000 -1 -1 \"X1\"»
      «text (100,-100) 1 5 0 0x1000000 -1 -1 \"amp\"»
      «pin (300,0) (0,0) 1 0 0 0x1000000 -1 \"OUT\"»
      «pin (-300,100) (0,0) 1 0 0 0x1000000 -1 \"IN\"»
    »
  »
»
",
    ))
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let amplifier = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "X1")
        .expect("the amplifier is placed");
    let value = instance_label_layout(
        LayoutPoint::new(amplifier.pos.x as f32, amplifier.pos.y as f32),
        1.0,
        amplifier,
    )
    .value;
    assert_eq!(
        (
            value.anchor.x,
            value.anchor.y,
            value.horizontal,
            value.vertical
        ),
        (10.0, 10.0, TextEdge::Start, TextEdge::Start)
    );
    // The lower edge runs from (-30, 30) up to (30, 0): over the box's whole
    // width it stays at or above the box's top.
    let edge_y = |x: f32| 30.0 - (x + 30.0) * 0.5;
    let text = text_box(value, "amp", INSTANCE_VALUE_WORLD);
    assert!(
        edge_y(text.min.x) <= text.min.y && edge_y(text.max.x) <= text.min.y,
        "the value's box {text:?} crosses the lower edge"
    );
}

/// A source's drawing letters its waveform as written, a line at each of
/// its value's windows, where RSpice letters a line for each parameter and
/// runs down onto its neighbours. Each text stands where the drawing set it
/// while the source keeps the waveform it came with; edited, the source
/// letters RSpice's own label again.
#[test]
fn an_imported_source_letters_its_waveform_as_its_drawing_wrote_it() {
    use crate::schematic::label_layout::TextEdge;
    use crate::schematic::label_position::{LabelPosition, ValuePiece};
    use crate::schematic::value_label::value_pieces;

    let drawing = parse_asc(
        "Version 4\nSYMBOL voltage 0 0 R0\nWINDOW 3 24 96 Left 2\nWINDOW 123 24 124 Left 2\n\
         SYMATTR InstName V1\nSYMATTR Value PULSE(0 1 0 1n 1n 5u 10u)\nSYMATTR Value2 AC 1\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let mut source = imported
        .document
        .components
        .iter()
        .find(|part| part.name == "V1")
        .cloned()
        .expect("V1 is placed");
    assert_eq!(source.kind, ComponentType::VoltageSourcePulse);
    let pieces = value_pieces(&source).expect("the source letters its drawing's texts");
    let texts = pieces
        .iter()
        .map(|piece| piece.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(texts, ["PULSE(0 1 0 1n 1n 5u 10u)", "AC 1"]);
    // Each starts at its window, scaled from the symbol's origin.
    let anchor = |piece: &ValuePiece| match piece.position {
        LabelPosition::Placed {
            offset, horizontal, ..
        } => (source.pos + offset, horizontal),
        LabelPosition::Auto => panic!("{piece:?} has no place"),
    };
    assert_eq!(anchor(&pieces[0]), (Point::new(15, 60), TextEdge::Start));
    assert_eq!(anchor(&pieces[1]), (Point::new(15, 78), TextEdge::Start));

    source.params.push_str(" td=1u");
    assert!(
        value_pieces(&source).is_none(),
        "an edited source letters RSpice's own label"
    );
}

/// QSPICE letters each of a part's attributes as a text of its own, where
/// and as large as the drawing set it; the part's value is those texts
/// joined. Lettering them as one line runs it across whatever stands beside
/// the part, so each keeps its own place and size, until the value is
/// edited and letters as one text again.
#[test]
fn a_qspice_value_lettered_in_several_texts_keeps_each_where_the_drawing_set_it() {
    use crate::schematic::label_layout::{
        INSTANCE_NAME_WORLD, LayoutPoint, LayoutRect, TextEdge, instance_label_layout, text_box,
    };
    use crate::schematic::value_label::value_pieces;

    let drawing = parse_qsch(&qsch_bytes(
        "«schematic
  «component (0,0) 0 0
    «symbol SW
      «type: S»
      «ellipse (-150,-150) (150,150) 0 0 0 0x1000000 0x1000000 -1 -1»
      «text (-100,250) 1 7 0 0x1000000 -1 -1 \"S1\"»
      «text (250,60) 0.5 7 0 0x1000000 -1 -1 \"Ron=1\"»
      «text (250,-30) 0.5 7 0 0x1000000 -1 -1 \"Roff=1Meg\"»
      «text (250,-120) 0.5 7 0 0x1000000 -1 -1 \"Vt=1\"»
      «text (250,-210) 0.5 7 0 0x1000000 -1 -1 \"Vh=-0.5\"»
      «pin (0,200) (0,0) 1 0 0 0x1000000 -1 \"1\"»
      «pin (0,-200) (0,0) 1 0 0 0x1000000 -1 \"2\"»
      «pin (-200,100) (0,0) 1 0 0 0x1000000 -1 \"+\"»
      «pin (-200,-100) (0,0) 1 0 0 0x1000000 -1 \"-\"»
    »
  »
»
",
    ))
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let mut switch = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "S1")
        .expect("the switch is placed")
        .clone();
    assert_eq!(switch.kind, ComponentType::VSwitch);
    assert_eq!(switch.value, "Ron=1 Roff=1Meg Vt=1 Vh=-0.5");
    let origin = LayoutPoint::new(switch.pos.x as f32, switch.pos.y as f32);
    let pieces = value_pieces(&switch).expect("the value keeps its texts apart");
    let boxes = pieces
        .iter()
        .map(|piece| {
            let placed = piece
                .position
                .placed_text(origin, 1.0)
                .expect("each text stands where the drawing set it");
            assert_eq!(
                (placed.anchor.x, placed.horizontal, placed.vertical),
                (25.0, TextEdge::Start, TextEdge::Center),
                "{}",
                piece.text
            );
            (
                piece.text.as_str(),
                placed.anchor.y,
                piece.size_world(),
                text_box(placed, &piece.text, piece.size_world()),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        boxes
            .iter()
            .map(|(text, line, size, _)| (*text, *line, *size))
            .collect::<Vec<_>>(),
        [
            ("Ron=1", -6.0, 7.47),
            ("Roff=1Meg", 3.0, 7.47),
            ("Vt=1", 12.0, 7.47),
            ("Vh=-0.5", 21.0, 7.47),
        ]
    );
    let name = text_box(
        instance_label_layout(origin, 1.0, &switch).name,
        &switch.name,
        INSTANCE_NAME_WORLD,
    );
    let apart = |a: &LayoutRect, b: &LayoutRect| {
        a.max.x <= b.min.x || b.max.x <= a.min.x || a.max.y <= b.min.y || b.max.y <= a.min.y
    };
    for (text, _, _, a) in &boxes {
        assert!(apart(a, &name), "{text} runs into the name");
    }
    // The drawing's lines stand closer than a line box and clear of each
    // other's capitals.
    for pair in boxes.windows(2) {
        assert!(pair[1].1 - pair[0].1 > 7.47 * 0.7, "{}", pair[0].0);
    }

    switch.value = "Ron=2 Roff=1Meg Vt=1 Vh=-0.5".to_owned();
    assert!(
        value_pieces(&switch).is_none(),
        "an edited value letters as one text"
    );
}

/// A text that could stand clear only past other parts stays by its own
/// part: the clearing moves a name or value at most across its own part's
/// body, never along a row of other parts to stand beyond the last of them.
#[test]
fn a_text_with_no_clear_place_beside_its_part_stays_by_it() {
    use crate::schematic::label_layout::{
        INSTANCE_VALUE_WORLD, LayoutPoint, instance_label_layout, text_box,
    };

    // Upright resistors side by side, their bodies touching; the first one's
    // value starts on its body and reads to the right, across the rest.
    let mut drawing = String::from(
        "Version 4\nSYMBOL res 0 0 R0\nWINDOW 3 16 32 Left 2\n\
         SYMATTR InstName R1\nSYMATTR Value 1k\n",
    );
    for column in 1..10 {
        drawing.push_str(&format!(
            "SYMBOL res {} 0 R0\nSYMATTR InstName R{}\nSYMATTR Value 1k\n",
            16 * column,
            column + 1
        ));
    }
    let imported = import_foreign_schematic(
        &parse_asc(&drawing).expect("the drawing reads"),
        "work",
        &library(),
    );
    let part = imported
        .document
        .components
        .iter()
        .find(|part| part.name == "R1")
        .expect("R1 is placed");
    let (width, height) = part.symbol_dimensions();
    let width = if part.rotation.is_vertical() {
        height
    } else {
        width
    } as f32;
    let origin = LayoutPoint::new(part.pos.x as f32, part.pos.y as f32);
    let layout = instance_label_layout(origin, 1.0, part);
    let value = text_box(layout.value, &part.value, INSTANCE_VALUE_WORLD);
    let middle = (value.min.x + value.max.x) * 0.5;
    assert!(
        (middle - origin.x).abs() <= width * 2.0,
        "the value stands at {middle}, beside a {width} wide part at {}",
        origin.x
    );
}

/// An LTspice flag names its net away from the one side its wire or pin
/// lead reaches it from: below a pin whose lead comes down to it, above one
/// whose lead runs up from it, beyond either end of a wire. A flag on the
/// middle of a wire keeps the usual side.
#[test]
fn an_ltspice_flag_reached_only_along_a_vertical_line_reads_up_it() {
    use crate::schematic::label_layout::{
        NET_LABEL_FONT_WORLD, NET_LABEL_GLYPH_WIDTH_WORLD, net_label_layout, net_label_name_corner,
    };
    use crate::schematic::net_label::NetLabelSide;

    let drawing = parse_asc(
        "Version 4\n\
         WIRE 0 0 0 160\nFLAG 0 0 ONTOP\nFLAG 0 160 ATFOOT\n\
         WIRE 160 0 160 160\nFLAG 160 80 PAST\n\
         WIRE 320 0 320 160\nWIRE 320 0 480 0\nFLAG 320 0 CORNER\n\
         WIRE 480 160 640 160\nFLAG 640 160 LEVEL\n\
         WIRE 800 0 800 160\nWIRE 800 160 960 160\nFLAG 800 160 UNDER\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let label = |name: &str| {
        imported
            .document
            .net_labels
            .iter()
            .find(|label| label.name == name)
            .unwrap_or_else(|| panic!("no label {name}"))
    };
    for (name, side, upward) in [
        ("ATFOOT", NetLabelSide::RisingBelow, true),
        ("PAST", NetLabelSide::RisingBeside, true),
        ("CORNER", NetLabelSide::Above, false),
        ("UNDER", NetLabelSide::Below, false),
        ("LEVEL", NetLabelSide::Right, false),
    ] {
        assert_eq!(label(name).side, side, "{name}");
        assert_eq!(net_label_layout(label(name)).upward, upward, "{name}");
    }
    assert_eq!(label("ONTOP").side, NetLabelSide::RisingAbove);

    // On the top end the name starts past the wire end, centred on the wire;
    // on the bottom end it ends short of it.
    let (top, foot) = (label("ONTOP"), label("ATFOOT"));
    let level = label("LEVEL");
    let gap = net_label_name_corner(level).x - level.pos.x as f32;
    let corner = net_label_name_corner(top);
    assert_eq!(corner.y, top.pos.y as f32 - gap);
    assert!(corner.x > top.pos.x as f32, "{corner:?}");
    let layout = net_label_layout(foot);
    let width = foot.name.len() as f32 * NET_LABEL_GLYPH_WIDTH_WORLD * layout.font_size
        / NET_LABEL_FONT_WORLD;
    assert!(
        (layout.corner.y - width - foot.pos.y as f32 - gap).abs() < 0.5,
        "{:?}",
        layout.corner
    );
}

#[test]
fn an_ltspice_flag_names_its_net_away_from_what_reaches_it() {
    use crate::schematic::net_label::NetLabelSide;

    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL res 0 0 R0\nSYMATTR InstName R1\nSYMATTR Value 1k\n\
         FLAG 16 16 TOP\nFLAG 16 96 FOOT\n\
         WIRE 160 160 320 160\n\
         FLAG 160 160 WEST\nFLAG 320 160 EAST\nFLAG 240 160 MIDDLE\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let sides = imported
        .document
        .net_labels
        .iter()
        .map(|label| (label.name.as_str(), label.side))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        sides,
        BTreeMap::from([
            ("EAST", NetLabelSide::Right),
            ("FOOT", NetLabelSide::RisingBelow),
            ("MIDDLE", NetLabelSide::Above),
            ("TOP", NetLabelSide::RisingAbove),
            ("WEST", NetLabelSide::Left),
        ])
    );
}

#[test]
fn a_jumper_draws_its_bridge_and_name_on_the_wire_it_is() {
    use crate::schematic::label_layout::{LayoutPoint, TextEdge};
    use crate::schematic::wire::Wire;

    let drawing = parse_asc(
        "Version 4\n\
         WIRE 80 80 80 240\n\
         SYMBOL Misc\\jumper 80 80 R0\nSYMATTR InstName J3\n\
         SYMBOL Misc\\jumper 320 80 R90\nSYMATTR InstName J4\n",
    )
    .expect("the drawing reads");
    let mut source = library();
    source.symbols.insert(
        "misc\\jumper".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(
                "Version 4\nSymbolType CELL\n\
                 LINE Normal -48 0 -32 0\nLINE Normal 32 0 48 0\n\
                 CIRCLE Normal -36 -4 -28 4\nCIRCLE Normal 28 -4 36 4\n\
                 ARC Normal -32 -32 32 32 32 0 -32 0\n\
                 WINDOW 0 0 -40 Bottom 2\nSYMATTR Prefix J\n\
                 PIN -48 0 NONE 0\nPINATTR SpiceOrder 1\n\
                 PIN 48 0 NONE 0\nPINATTR SpiceOrder 2\n",
            )
            .expect("the jumper reads"),
            location: "jumper.asy".to_owned(),
            schematic: None,
        },
    );
    let imported = import_foreign_schematic(&drawing, "work", &source);
    assert!(imported.document.components.is_empty(), "no part");
    let bridged = |name: &str| -> &Wire {
        imported
            .document
            .wires
            .iter()
            .find(|wire| {
                wire.drawn_bridge()
                    .is_some_and(|bridge| bridge.name == name)
            })
            .expect("the jumper's wire draws its bridge")
    };

    let flat = bridged("J3");
    // The wire another wire ends on the middle of is still one run.
    assert_eq!(flat.points, [Point::new(20, 50), Point::new(80, 50)]);
    let bridge = flat.drawn_bridge().expect("drawn");
    assert_eq!(
        bridge.lines,
        [
            [Point::new(0, 0), Point::new(10, 0)],
            [Point::new(50, 0), Point::new(60, 0)],
        ]
    );
    assert_eq!(bridge.curves.len(), 3);
    let placed = bridge
        .name_position
        .placed_text(LayoutPoint::new(20.0, 50.0), 1.0)
        .expect("placed");
    assert_eq!(
        (
            placed.anchor,
            placed.horizontal,
            placed.vertical,
            placed.upward
        ),
        (
            LayoutPoint::new(50.0, 25.0),
            TextEdge::Center,
            TextEdge::End,
            false
        )
    );
    let ((low_x, low_y), (high_x, high_y)) = {
        let (low, high) = bridge.stroke_bounds();
        ((low[0], low[1]), (high[0], high[1]))
    };
    assert_eq!((low_x, high_x), (0.0, 60.0));
    assert!(low_y <= -19.0 && high_y >= 0.0, "{low_y} {high_y}");

    let upright = bridged("J4");
    assert_eq!(upright.points, [Point::new(200, 20), Point::new(200, 80)]);
    assert_eq!(
        upright.drawn_bridge().expect("drawn").run,
        Point::new(0, 60)
    );

    assert!(flat.translated(10, 40).drawn_bridge().is_some(), "it moves");
    let mut stretched = flat.clone();
    stretched.points[1].x += 10;
    assert!(stretched.drawn_bridge().is_none(), "another run is a line");
}

#[test]
fn a_jumper_is_a_wire_and_a_named_subcircuit_reads_from_the_deck() {
    let drawing = parse_asc(
        "Version 4\n\
         FLAG 0 0 a\nFLAG 0 96 b\n\
         SYMBOL Misc\\jumper -16 -16 R0\nSYMATTR InstName J1\n\
         SYMBOL Misc\\TowTom2 320 0 R0\nSYMATTR InstName U1\n",
    )
    .expect("the drawing reads");
    let mut source = library();
    source.symbols.insert(
        "misc\\jumper".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(
                "Version 4\nSymbolType CELL\nSYMATTR Prefix J\n\
                 PIN 16 16 NONE 0\nPINATTR SpiceOrder 1\nPIN 16 112 NONE 0\nPINATTR SpiceOrder 2\n",
            )
            .expect("the jumper reads"),
            location: "jumper.asy".to_owned(),
            schematic: None,
        },
    );
    source.symbols.insert(
        "misc\\towtom2".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(
                "Version 4\nSymbolType CELL\nSYMATTR Value TowTom2\nSYMATTR Prefix X\n\
                 PIN 0 0 NONE 0\nPINATTR SpiceOrder 1\nPIN 64 0 NONE 0\nPINATTR SpiceOrder 2\n",
            )
            .expect("the block reads"),
            location: "TowTom2.asy".to_owned(),
            schematic: None,
        },
    );
    let imported = import_foreign_schematic(&drawing, "work", &source);
    assert!(
        imported
            .document
            .components
            .iter()
            .all(|component| component.name != "J1"),
        "a jumper places no part"
    );
    // The flags `a` and `b` sit on the jumper's pins, (0, 0) and (0, 60) on
    // RSpice's grid; wire runs the whole way between them.
    let covered: i32 = imported
        .document
        .wires
        .iter()
        .flat_map(|wire| wire.points.windows(2).map(|ends| (ends[0], ends[1])))
        .filter(|(start, end)| start.x == 0 && end.x == 0)
        .map(|(start, end)| (end.y - start.y).abs())
        .sum();
    assert_eq!(covered, 60, "the jumper joins its two nets");
    let block = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "U1")
        .and_then(|component| component.library_cell.as_ref())
        .expect("the block is a cell instance");
    assert_eq!(block.source_path, None);
    assert_eq!(
        block.netlist_template.as_deref(),
        Some("X{name} {nodes} {model} {params}")
    );
}

#[test]
fn a_value_that_is_not_a_number_reads_as_an_expression() {
    use super::import::is_spice_number;
    for number in ["10k", "4.7uF", "1e-3", ".5", "-2.2Meg", "100"] {
        assert!(is_spice_number(number), "{number}");
    }
    for expression in [
        "2.21/3",
        "Rload",
        "(1T-Connection)+1",
        "Lbus+Lcable",
        "5K*(1-wiper)",
    ] {
        assert!(!is_spice_number(expression), "{expression}");
    }
}

#[test]
fn a_subcircuit_named_by_its_spice_model_writes_that_model_and_its_values() {
    const MODELLED: &str = "Version 4\nSymbolType CELL\nRECTANGLE Normal -32 -32 32 32\n\
        SYMATTR Prefix X\nSYMATTR SpiceModel level2\nSYMATTR Value2 Avol=1Meg GBW=10Meg\n\
        SYMATTR ModelFile opamps.lib\n\
        PIN -32 0 NONE 0\nPINATTR PinName IN\nPINATTR SpiceOrder 1\n\
        PIN 32 0 NONE 0\nPINATTR PinName OUT\nPINATTR SpiceOrder 2\n";
    let mut library = library();
    library.symbols.insert(
        "modelled".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(MODELLED).expect("the test symbol reads"),
            location: "modelled.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc(
        "Version 4\nSHEET 1 880 680\nFLAG 64 96 in\nFLAG 128 96 out\n\
         SYMBOL modelled 96 96 R0\nSYMATTR InstName U1\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let part = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "U1")
        .expect("U1");
    assert_eq!(part.value, "level2");
    assert_eq!(part.params, "Avol=1Meg GBW=10Meg");
}

#[test]
fn a_lead_to_one_pin_never_runs_through_another() {
    // The switch's two control pins stand one above the other, where its
    // RSpice symbol puts them side by side: a lead bent the wrong way from
    // one terminal ran over the other pin and shorted the control input.
    let source = "Version 4\nSHEET 1 880 680\n\
        WIRE 144 192 -208 192\nWIRE 144 240 128 240\nWIRE -208 208 -208 192\n\
        WIRE -208 320 -208 288\nWIRE 192 176 192 144\nWIRE 192 272 192 256\n\
        FLAG 128 240 0\nFLAG -208 320 0\nFLAG -208 192 IN\nFLAG 192 144 OUT\nFLAG 192 272 0\n\
        SYMBOL sw 192 272 M180\nSYMATTR InstName S1\nSYMATTR Value MYSW\n\
        SYMBOL voltage -208 192 R0\nSYMATTR InstName V1\nSYMATTR Value 1\n";
    let drawing = parse_asc(source).unwrap();
    let library = Library::default();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_same_connectivity(&drawing, &imported, &library);
    assert_wires_on_grid(&imported);
}

#[test]
fn leads_that_must_cross_jog_around_each_other() {
    // LTspice's transconductance source puts its output's plus pin below its
    // minus and its control's plus above its minus; RSpice's draws each
    // pair's plus on the same side, so one pair of leads must cross.
    let source = "Version 4\nSHEET 1 880 680\n\
        WIRE 32 32 32 -32\nWIRE 32 -32 -160 -32\nWIRE 32 128 32 80\n\
        WIRE 80 16 80 -64\nWIRE 80 128 80 96\nWIRE -160 -32 -160 16\n\
        FLAG 32 128 0\nFLAG 80 128 0\nFLAG 80 -64 out\nWIRE -160 112 -160 96\nFLAG -160 112 0\n\
        SYMBOL g 80 0 R0\nSYMATTR InstName G1\nSYMATTR Value 1m\n\
        SYMBOL voltage -160 0 R0\nSYMATTR InstName V1\nSYMATTR Value 1\n\
        WIRE 336 32 336 -32\nWIRE 336 -32 224 -32\nWIRE 336 128 336 80\n\
        WIRE 384 16 384 -64\nWIRE 384 128 384 96\n\
        FLAG 336 128 0\nFLAG 384 128 0\nFLAG 384 -64 out2\nFLAG 224 -32 ctl\n\
        SYMBOL e 384 0 R0\nSYMATTR InstName E1\nSYMATTR Value 10\n";
    let drawing = parse_asc(source).unwrap();
    let library = Library::default();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_same_connectivity(&drawing, &imported, &library);
    assert_wires_on_grid(&imported);
}

#[test]
fn a_place_its_symbol_gives_no_pin_is_an_unconnected_terminal() {
    // LTspice writes a node of its own (`MP_nn`) for each instance-line
    // place no pin takes, so the model's later nodes keep their places.
    let drawing = parse_asc(
        "Version 4
SHEET 1 800 600
         WIRE 32 64 -64 64
WIRE 192 64 96 64
WIRE 64 192 64 96
         FLAG -64 64 in
FLAG 192 64 out
FLAG 64 192 0
         SYMBOL gapped 64 64 R0
SYMATTR InstName U1
         WIRE 448 0 448 -64
WIRE 448 160 448 64
WIRE 320 32 416 32
         FLAG 448 -64 in
FLAG 448 160 out
FLAG 320 32 0
         SYMBOL gapped 448 32 R90
SYMATTR InstName U2
",
    )
    .unwrap();
    let library = library();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_same_connectivity(&drawing, &imported, &library);
    assert_wires_on_grid(&imported);
    let cell = &imported.cells[0];
    let ports = cell
        .ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ports, ["IN", "OUT", "NC3", "REF"]);
    assert_eq!(cell.unconnected, ["NC3"]);
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
    let parts = imported
        .document
        .components
        .iter()
        .filter_map(|component| Some((component, component.library_cell.as_ref()?)));
    for (component, binding) in parts {
        assert_eq!(binding.terminal_order, ["IN", "OUT", "NC3", "REF"]);
        let placeholder = connectivity
            .terminals
            .iter()
            .find(|terminal| terminal.component_id == component.id && terminal.pin == "NC3")
            .unwrap_or_else(|| panic!("{} has no NC3 terminal", component.name));
        let others = connectivity
            .terminals
            .iter()
            .filter(|terminal| terminal.net_name == placeholder.net_name)
            .count();
        assert_eq!(others, 1, "{}'s NC3 touches nothing", component.name);
    }
    let notices = imported
        .notices
        .iter()
        .filter(|notice| notice.message.contains("no pin for place 3"))
        .count();
    assert_eq!(notices, 1, "one notice per symbol: {:?}", imported.notices);
}

/// RSpice marks the terminal it leaves for an instance-line place the
/// symbol draws no pin for, and LTspice draws nothing there. The mark keeps
/// off the name and value the symbol letters across its middle: it stands
/// at the nearest place clear of them, still touching nothing.
#[test]
fn an_unconnected_terminal_keeps_its_mark_off_the_symbols_lettering() {
    use crate::schematic::label_layout::{
        INSTANCE_VALUE_WORLD, LayoutPoint, instance_label_layout, text_box,
    };

    let drawing = parse_asc(
        "Version 4
SHEET 1 800 600
         WIRE 32 64 -64 64
WIRE 192 64 96 64
WIRE 64 192 64 96
         FLAG -64 64 in
FLAG 192 64 out
FLAG 64 192 0
         SYMBOL gapped 64 64 R0
WINDOW 3 0 24 Top 2
SYMATTR InstName U1
SYMATTR Value LDO_33
",
    )
    .unwrap();
    let library = library();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_same_connectivity(&drawing, &imported, &library);
    let cell = &imported.cells[0];
    let mark = cell
        .symbol
        .pins
        .iter()
        .find(|pin| pin.name == "NC3")
        .and_then(|pin| pin.position)
        .expect("place 3 has a terminal");
    // Half a grid step off the grid, where no wire can reach it.
    assert_eq!((mark.x, mark.y), (25, 15));
    let part = &imported.document.components[0];
    let layout = instance_label_layout(
        LayoutPoint::new(part.pos.x as f32, part.pos.y as f32),
        1.0,
        part,
    );
    let value = text_box(layout.value, &part.value, INSTANCE_VALUE_WORLD);
    let at = part.pos + part.transform_point(mark - cell.symbol.origin);
    let (x, y) = (at.x as f32, at.y as f32);
    assert!(
        x >= value.max.x || x <= value.min.x || y >= value.max.y || y <= value.min.y,
        "the mark at ({x}, {y}) stands on the value at {value:?}"
    );
}

/// LTspice's varistor and D flip-flop: places without a pin, drawn pins a
/// circuit may leave open, and the common every A-device draws.
const LTSPICE_VARISTOR_ASY: &str = "Version 4
SymbolType CELL
RECTANGLE Normal -48 32 16 96
    SYMATTR Prefix A
SYMATTR SpiceModel VARISTOR
    PIN -32 48 NONE 0
PINATTR PinName invin
PINATTR SpiceOrder 1
    PIN -32 80 NONE 0
PINATTR PinName noninvin
PINATTR SpiceOrder 2
    PIN -16 32 NONE 0
PINATTR PinName out
PINATTR SpiceOrder 7
    PIN -16 96 NONE 0
PINATTR PinName com
PINATTR SpiceOrder 8
";
const LTSPICE_DFLOP_ASY: &str = "Version 4
SymbolType CELL
RECTANGLE Normal -80 0 80 144
    SYMATTR Prefix A
SYMATTR SpiceModel DFLOP
    PIN -80 48 LEFT 8
PINATTR PinName D
PINATTR SpiceOrder 1
    PIN -80 96 LEFT 8
PINATTR PinName CLK
PINATTR SpiceOrder 3
    PIN 0 0 TOP 4
PINATTR PinName PRE
PINATTR SpiceOrder 4
    PIN 0 144 BOTTOM 4
PINATTR PinName CLR
PINATTR SpiceOrder 5
    PIN 96 96 RIGHT 20
PINATTR PinName _Q
PINATTR SpiceOrder 6
    PIN 80 48 RIGHT 4
PINATTR PinName Q
PINATTR SpiceOrder 7
    PIN -80 144 NONE 0
PINATTR PinName com
PINATTR SpiceOrder 8
";

#[test]
fn an_ltspice_device_writes_eight_places_and_grounds_what_nothing_reaches() {
    let mut library = Library::default();
    for (name, source) in [
        ("special\\varistor", LTSPICE_VARISTOR_ASY),
        ("logic\\dflop", LTSPICE_DFLOP_ASY),
    ] {
        library.symbols.insert(
            name.to_owned(),
            ForeignSymbolSource {
                definition: parse_asy(source).expect("the test symbol reads"),
                location: format!("{name}.asy"),
                schematic: None,
            },
        );
    }
    let drawing = parse_asc(
        "Version 4
SHEET 1 1200 400
         FLAG -32 48 in
FLAG -32 80 0
FLAG -16 32 out
         SYMBOL special\\varistor 0 0 R0
SYMATTR InstName A1
SYMATTR SpiceLine Rclamp=1
         FLAG 320 48 d
FLAG 320 96 clk
FLAG 480 48 q
         SYMBOL logic\\dflop 400 0 R0
SYMATTR InstName A2
",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_wires_on_grid(&imported);
    assert_same_connectivity(&drawing, &imported, &library);
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
    let nets = |name: &str| {
        let component = imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("{name} was placed"));
        let binding = component.library_cell.as_ref().expect("a cell instance");
        binding
            .terminal_order
            .iter()
            .map(|pin| {
                let net = &connectivity
                    .terminals
                    .iter()
                    .find(|terminal| terminal.component_id == component.id && &terminal.pin == pin)
                    .unwrap_or_else(|| panic!("{name} has no terminal {pin}"))
                    .net_name;
                if crate::schematic::ground_names::is_ground_reference(net) {
                    "0".to_owned()
                } else {
                    net.clone()
                }
            })
            .collect::<Vec<_>>()
    };
    // Every place of the line is written; a place without a pin and a pin
    // nothing reaches are node 0, as LTspice writes them.
    assert_eq!(nets("A1"), ["in", "0", "0", "0", "0", "0", "out", "0"]);
    assert_eq!(nets("A2"), ["d", "0", "clk", "0", "0", "0", "q", "0"]);
    // One ground per open pin: the varistor's common, the flip-flop's
    // preset, clear, inverted output and common; the flag's own.
    let grounds = imported
        .document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::Ground)
        .count();
    assert_eq!(grounds, 6);
    assert!(
        !imported
            .notices
            .iter()
            .any(|notice| notice.message.contains("no pin for place")),
        "LTspice's own places need no notice: {:?}",
        imported.notices
    );
}

/// LTspice's AND gate: five inputs, the inverted and the true output, and
/// its common, at the places its library draws them.
const LTSPICE_AND_ASY: &str = "Version 4\nSymbolType CELL\nLINE Normal -32 16 32 16\n\
    SYMATTR Prefix A\nSYMATTR SpiceModel AND\n\
    PIN -32 32 NONE 0\nPINATTR PinName a\nPINATTR SpiceOrder 1\n\
    PIN -32 48 NONE 0\nPINATTR PinName b\nPINATTR SpiceOrder 2\n\
    PIN -32 64 NONE 0\nPINATTR PinName c\nPINATTR SpiceOrder 3\n\
    PIN -32 80 NONE 0\nPINATTR PinName d\nPINATTR SpiceOrder 4\n\
    PIN -32 96 NONE 0\nPINATTR PinName e\nPINATTR SpiceOrder 5\n\
    PIN 32 80 NONE 0\nPINATTR PinName _Q\nPINATTR SpiceOrder 6\n\
    PIN 32 48 NONE 0\nPINATTR PinName Q\nPINATTR SpiceOrder 7\n\
    PIN -16 96 NONE 0\nPINATTR PinName com\nPINATTR SpiceOrder 8\n";

#[test]
fn an_ltspice_gate_with_a_ground_common_and_one_output_is_a_functional_gate() {
    let mut library = Library::default();
    library.symbols.insert(
        "logic\\and".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(LTSPICE_AND_ASY).expect("the test symbol reads"),
            location: "logic/and.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc(
        "Version 4\nSHEET 1 1600 400\n\
         FLAG -32 32 in1\nFLAG -32 48 in2\nFLAG 32 48 y1\n\
         SYMBOL logic\\and 0 0 R0\nSYMATTR InstName A1\nSYMATTR SpiceLine Vhigh=5 Td=10n\n\
         FLAG 288 32 in1\nFLAG 288 48 in2\nFLAG 288 64 in3\nFLAG 352 80 y2\n\
         SYMBOL logic\\and 320 0 R0\nSYMATTR InstName A2\n\
         FLAG 608 32 in1\nFLAG 672 48 y3\nFLAG 672 80 y4\n\
         SYMBOL logic\\and 640 0 R0\nSYMATTR InstName A3\n\
         FLAG 928 32 in1\nFLAG 928 48 in2\nFLAG 992 48 y5\nFLAG 944 96 vref\n\
         SYMBOL logic\\and 960 0 R0\nSYMATTR InstName A4\n\
         FLAG 1248 32 in1\nFLAG 1248 48 in2\nFLAG 1312 48 y6\n\
         WIRE 1264 96 1264 160\nFLAG 1264 160 0\n\
         SYMBOL logic\\and 1280 0 R0\nSYMATTR InstName A5\nSYMATTR SpiceLine Tau=1n\n\
         FLAG 1568 32 in1\nFLAG 1568 48 in2\nFLAG 1632 48 y7\n\
         SYMBOL logic\\and 1600 0 R0\nSYMATTR InstName A6\nSYMATTR Value Trise=10n Vhigh=3.3\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_wires_on_grid(&imported);
    let part = |name: &str| {
        imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("{name} was placed"))
    };
    let functional = |name: &str| {
        part(name)
            .library_cell
            .as_ref()
            .and_then(|binding| binding.builtin_functional.as_ref())
            .map(|contract| contract.stable_id.as_str())
    };
    assert_eq!(functional("A1"), Some("rspice.functional.and"));
    assert_eq!(
        part("A1").params,
        "INPUTS=2 VHIGH=5 VLOW=0 TD=10n TRISE=0 ROUT=1 TFALL=0"
    );
    assert_eq!(
        part("A1").library_cell.as_ref().unwrap().terminal_order,
        ["A", "B", "Y"]
    );
    // A value of settings alone keeps the symbol's model.
    assert_eq!(functional("A6"), Some("rspice.functional.and"));
    assert_eq!(
        part("A6").params,
        "INPUTS=2 VHIGH=3.3 VLOW=0 TD=0 TRISE=10n ROUT=1 TFALL=10n"
    );
    assert_eq!(functional("A2"), Some("rspice.functional.nand"));
    assert_eq!(
        part("A2").library_cell.as_ref().unwrap().terminal_order,
        ["A", "B", "C", "Y"]
    );
    // Both outputs, a common off ground, or a setting the part lacks keep
    // LTspice's own device, and the import says why.
    for name in ["A3", "A4", "A5"] {
        assert_eq!(functional(name), None, "{name} keeps LTspice's device");
    }
    for (name, why) in [
        ("A3", "both its true and its inverted output"),
        ("A4", "common on a net other than ground"),
        ("A5", "`Tau=1n`"),
    ] {
        assert!(
            imported
                .notices
                .iter()
                .any(|notice| notice.message.starts_with(name) && notice.message.contains(why)),
            "{name}: {:?}",
            imported.notices
        );
    }
    // Each drawn pin of a functional gate lands on its source pin's net.
    let connectivity = extract(&imported.document, |_| None, |_| None);
    let nets = |name: &str| {
        let component = part(name);
        component
            .terminal_positions()
            .into_iter()
            .map(|(_, point)| {
                connectivity
                    .terminals
                    .iter()
                    .find(|terminal| {
                        terminal.component_id == component.id && terminal.point == point
                    })
                    .map(|terminal| terminal.net_name.clone())
                    .unwrap_or_else(|| panic!("{name} has no terminal at {point:?}"))
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(nets("A1"), ["in1", "in2", "y1"]);
    assert_eq!(nets("A2"), ["in1", "in2", "in3", "y2"]);
}

#[test]
fn a_gate_input_a_jumper_grounds_is_not_read() {
    let mut library = Library::default();
    for (name, source) in [
        ("logic\\and", LTSPICE_AND_ASY),
        (
            "misc\\jumper",
            "Version 4\nSymbolType CELL\nSYMATTR Prefix J\n\
             PIN -32 64 NONE 0\nPINATTR SpiceOrder 1\nPIN 32 64 NONE 0\nPINATTR SpiceOrder 2\n",
        ),
    ] {
        library.symbols.insert(
            name.to_owned(),
            ForeignSymbolSource {
                definition: parse_asy(source).expect("the test symbol reads"),
                location: format!("{name}.asy"),
                schematic: None,
            },
        );
    }
    let drawing = parse_asc(
        "Version 4\nSHEET 1 800 400\n\
         FLAG -32 32 in1\nFLAG 32 48 y\nWIRE -32 48 -96 48\n\
         WIRE -160 48 -160 96\nFLAG -160 96 0\n\
         SYMBOL Misc\\jumper -128 -16 R0\nSYMATTR InstName J1\n\
         SYMBOL logic\\and 0 0 R0\nSYMATTR InstName A1\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let gate = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "A1")
        .and_then(|component| component.library_cell.as_ref())
        .and_then(|binding| binding.builtin_functional.as_ref())
        .map(|contract| contract.stable_id.as_str());
    // The jumper puts its second input on ground, which LTspice's gate
    // does not read: one input left, a buffer.
    assert_eq!(gate, Some("rspice.functional.buffer"));
}

#[test]
fn every_attribute_a_drawing_shows_is_lettered_where_it_shows_it() {
    use crate::schematic::label_position::LabelPosition;
    use crate::schematic::value_label::value_pieces;

    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL ind 0 0 R0\nWINDOW 3 40 40 Left 2\nWINDOW 123 40 80 Left 2\nWINDOW 39 0 0 Left 0\n\
         SYMATTR InstName L7\nSYMATTR Value 3m\nSYMATTR Value2 Rser={Rs/2}\nSYMATTR SpiceLine Ipk=2\n\
         SYMBOL ind 320 0 R0\nWINDOW 3 40 40 Left 2\nWINDOW 39 40 80 Left 2\nWINDOW 40 40 120 Left 2\n\
         SYMATTR InstName L8\nSYMATTR Value 5m\nSYMATTR SpiceLine Rser=9\nSYMATTR SpiceLine2 Cpar=3p\n\
         SYMBOL ind 640 0 R0\nWINDOW 3 40 40 Left 0\nWINDOW 123 40 80 Left 2\n\
         SYMATTR InstName L9\nSYMATTR Value 9m\nSYMATTR Value2 Rser=4\n\
         SYMBOL ind 960 0 R0\nWINDOW 3 40 40 Left 2\nWINDOW 123 40 80 Left 0\n\
         SYMATTR InstName L10\nSYMATTR Value 2m\nSYMATTR Value2 Rser=4\n\
         SYMBOL shown 1280 0 R0\nSYMATTR InstName U1\n",
    )
    .expect("the drawing reads");
    let mut source = library();
    source.symbols.insert(
        "shown".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(
                "Version 4\nSymbolType CELL\nRECTANGLE Normal 0 0 64 64\n\
                 WINDOW 0 0 -16 Left 2\nWINDOW 3 0 80 Left 2\nWINDOW 38 0 112 Left 2\n\
                 SYMATTR Prefix X\nSYMATTR Value core\nSYMATTR SpiceModel plan.sub\n\
                 PIN 0 32 NONE 0\nPINATTR SpiceOrder 1\n",
            )
            .expect("the symbol reads"),
            location: "shown.asy".to_owned(),
            schematic: None,
        },
    );
    let imported = import_foreign_schematic(&drawing, "work", &source);
    let texts = |name: &str| {
        let part = imported
            .document
            .components
            .iter()
            .find(|part| part.name == name)
            .unwrap_or_else(|| panic!("{name} is placed"));
        value_pieces(part).map(|pieces| {
            pieces
                .iter()
                .map(|piece| piece.text.clone())
                .collect::<Vec<_>>()
        })
    };
    assert_eq!(texts("L7").unwrap(), ["3m", "Rser={Rs/2}"]);
    assert_eq!(texts("L8").unwrap(), ["5m", "Rser=9", "Cpar=3p"]);
    assert_eq!(
        texts("L9").unwrap(),
        ["Rser=4"],
        "its value's window is size 0"
    );
    assert_eq!(
        texts("L10"),
        None,
        "a size 0 window shows nothing, so the value letters alone"
    );
    assert_eq!(
        texts("U1").unwrap(),
        ["core", "plan.sub"],
        "the symbol's own window"
    );

    // The reading stays what the attributes say.
    let l7 = imported
        .document
        .components
        .iter()
        .find(|part| part.name == "L7")
        .expect("L7 is placed");
    assert_eq!(l7.value, "3m");
    let spots = value_pieces(l7).expect("pieces");
    let LabelPosition::Placed { offset, .. } = spots[1].position else {
        panic!("{:?} has no place", spots[1]);
    };
    assert_eq!(l7.pos + offset, Point::new(25, 50));
}

#[test]
fn an_attribute_ltspice_emptied_is_no_value() {
    // LTspice writes `""` for a value the user cleared.
    let drawing = parse_asc(
        "Version 4\nSHEET 1 400 400\nSYMBOL voltage 0 0 R0\nSYMATTR InstName V1\n\
         SYMATTR Value2 AC 1\nSYMATTR Value \"\"\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let source = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "V1")
        .expect("V1 was placed");
    assert!(!source.value.contains('"'), "{:?}", source.value);
    assert!(source.params.contains("ac=1"), "{}", source.params);
}

#[test]
fn a_part_with_no_value_writes_its_symbols_model() {
    let mut library = Library::default();
    library.symbols.insert(
        "hold".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(
                "Version 4\nSymbolType CELL\nSYMATTR Prefix A\nSYMATTR SpiceModel SAMPLEHOLD\n\
                 PIN -32 0 NONE 0\nPINATTR SpiceOrder 1\nPIN 32 0 NONE 0\nPINATTR SpiceOrder 2\n",
            )
            .expect("the test symbol reads"),
            location: "hold.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing =
        parse_asc("Version 4\nSHEET 1 400 400\nSYMBOL hold 0 0 R0\nSYMATTR InstName A1\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let binding = imported.document.components[0]
        .library_cell
        .as_ref()
        .expect("a cell instance");
    assert_eq!(binding.module_name.as_deref(), Some("SAMPLEHOLD"));
}

#[test]
fn an_ltspice_frequency_response_probe_is_refused_by_name() {
    let drawing =
        parse_asc("Version 4\nSHEET 1 400 400\nSYMBOL FRA 0 0 R0\nSYMATTR InstName @1\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let refusals = imported
        .notices
        .iter()
        .filter(|notice| notice.severity == ForeignNoticeSeverity::Refusal)
        .collect::<Vec<_>>();
    assert_eq!(refusals.len(), 1, "{refusals:?}");
    assert!(
        refusals[0].message.contains("@1")
            && refusals[0].message.contains("frequency-response probe"),
        "{}",
        refusals[0].message
    );
    assert!(imported.document.components.is_empty());
}

/// A QSPICE gate letters the settings its drawing shows, where and as large
/// as the drawing set them, rather than the name RSpice gives the model it
/// writes for the gate; edited, the gate letters RSpice's own value again.
#[test]
fn a_qspice_gate_letters_the_settings_its_drawing_shows() {
    use crate::schematic::label_position::LabelPosition;
    use crate::schematic::value_label::value_pieces;

    const DRAWING: &str = "«schematic
  «component (0,0) 0 0
    «symbol AND2Q
      «type: ¥»
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"¥1\"»
      «text (0,-200) 0.5 0 2 0x1000000 -1 -1 \"AND\"»
      «text (0,-100) 0.6 15 0 0x1000000 -1 -1 \"Ref=2\"»
      «pin (0,400) (0,0) 1 0 0 0x1000000 -1 \"Vdd\"»
      «pin (0,-400) (0,0) 1 0 0 0x1000000 -1 \"Vss\"»
      «pin (400,0) (0,0) 1 0 0 0x1000000 -1 \"Q\"»
      «pin (-400,200) (0,0) 1 0 0 0x1000000 -1 \"b0\"»
      «pin (-400,-200) (0,0) 1 0 0 0x1000000 -1 \"b1\"»
    »
  »
  «net (0,400) 1 13 0 \"VDD\"»
  «net (0,-400) 1 13 0 \"GND\"»
  «net (400,0) 1 13 0 \"y\"»
  «net (-400,200) 1 13 0 \"a\"»
  «net (-400,-200) 1 13 0 \"b\"»
»
";
    let drawing = parse_qsch(&qsch_bytes(DRAWING)).unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let mut gate = imported
        .document
        .components
        .first()
        .cloned()
        .expect("the gate is placed");
    assert_eq!(gate.value, "AND2Q_1");
    let pieces = value_pieces(&gate).expect("the gate letters its drawing's text");
    let [piece] = pieces else {
        panic!("{pieces:?}");
    };
    assert_eq!((piece.text.as_str(), piece.size_percent), ("Ref=2", 100));
    assert!(matches!(piece.position, LabelPosition::Placed { .. }));

    gate.params.push_str(" Ref=3");
    assert!(
        value_pieces(&gate).is_none(),
        "an edited gate letters RSpice's own value"
    );
}

/// QSPICE's ¥-device gate and Ã-device Gm amplifier: each comes across as a
/// behavioural subcircuit written for it, its pins in the symbol's order,
/// with the settings it states, and a notice naming it.
#[test]
fn qspice_gates_and_gm_amplifiers_come_across_as_behavioural_subcircuits() {
    const DRAWING: &str = "«schematic
  «component (0,0) 0 0
    «symbol AND2Q
      «type: ¥»
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"¥1\"»
      «text (0,-200) 0.5 0 2 0x1000000 -1 -1 \"AND\"»
      «text (0,-300) 1 7 0 0x1000000 -1 -1 \"Trise=10n Zmult=2 Td2=5n\"»
      «pin (0,400) (0,0) 1 0 0 0x1000000 -1 \"Vdd\"»
      «pin (0,-400) (0,0) 1 0 0 0x1000000 -1 \"Vss\"»
      «pin (400,0) (0,0) 1 0 0 0x1000000 -1 \"Q\"»
      «pin (400,-200) (0,0) 1 0 0 0x1000000 -1 \"Qb\" \"¥\"»
      «pin (-400,200) (0,0) 1 0 0 0x1000000 -1 \"b0\"»
      «pin (-400,-200) (0,0) 1 0 0 0x1000000 -1 \"b1\"»
    »
  »
  «component (2000,0) 0 0
    «symbol Gm
      «type: \u{c3}»
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"Ã1\"»
      «text (0,-200) 0.5 0 2 0x1000000 -1 -1 \"MultGmAmp\"»
      «text (0,-300) 1 7 0 0x1000000 -1 -1 \"Gm=<50µ> Isrc=10µ VoutMax=2.5 Ft=1Meg\"»
      «pin (0,400) (0,0) 1 0 0 0x1000000 -1 \"Vdd\"»
      «pin (0,-400) (0,0) 1 0 0 0x1000000 -1 \"Vss\"»
      «pin (400,0) (0,0) 1 0 0 0x1000000 -1 \"OUT\"»
      «pin (-400,200) (0,0) 1 0 0 0x1000000 -1 \"IN-\"»
      «pin (-400,-200) (0,0) 1 0 0 0x1000000 -1 \"IN+\"»
      «pin (-400,100) (0,0) 1 0 0 0x1000000 -1 \"MULT+\" \"¥\"»
      «pin (-400,-100) (0,0) 1 0 0 0x1000000 -1 \"MULT-\" \"¥\"»
    »
  »
  «net (0,400) 1 13 0 \"VDD\"»
  «net (0,-400) 1 13 0 \"GND\"»
  «net (400,0) 1 13 0 \"y\"»
  «net (-400,200) 1 13 0 \"a\"»
  «net (-400,-200) 1 13 0 \"b\"»
  «net (2000,400) 1 13 0 \"VDD\"»
  «net (2000,-400) 1 13 0 \"GND\"»
  «net (2400,0) 1 13 0 \"out\"»
  «net (1600,200) 1 13 0 \"out\"»
  «net (1600,-200) 1 13 0 \"a\"»
»
";
    let drawing = parse_qsch(&qsch_bytes(DRAWING)).unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    assert!(
        imported
            .notices
            .iter()
            .all(|notice| notice.severity == ForeignNoticeSeverity::Notice),
        "{:#?}",
        imported.notices
    );
    let [gate, amplifier] = imported.model_cards.as_slice() else {
        panic!("{:#?}", imported.model_cards);
    };
    assert!(
        gate.contains(
            ".subckt AND2Q_1 p1 p2 p3 p4 p5 p6
"
        ),
        "{gate}"
    );
    // Both inputs are used; the threshold is the supply midpoint above Vss.
    assert!(
        gate.contains("Bs s 0 V=u(V(p5)-V(p2)-(V(p1)-V(p2))/2)*u(V(p6)-V(p2)-(V(p1)-V(p2))/2)"),
        "{gate}"
    );
    // The native special-function buffer times the edges; Tfall is Trise.
    assert!(
        gate.contains(
            "A1 s 0 0 0 0 0 r 0 BUF Vhigh=1 Vlow=0 Trise=10n
"
        ),
        "{gate}"
    );
    // Zmult doubles the 100 Ω output impedance halfway through an edge.
    assert!(
        gate.contains(
            "Bq p2 p3 I=(V(p2)+V(r)*(V(p1)-V(p2))-V(p3))/((100)*(1+((2)-1)*4*V(r)*(1-V(r))))"
        ),
        "{gate}"
    );
    assert!(
        amplifier.contains(
            ".subckt Gm_2 p1 p2 p3 p4 p5 p6 p7
"
        ),
        "{amplifier}"
    );
    // The unused multiplier pins leave the gain as it is.
    assert!(
        amplifier.contains(
            "Bx x 0 V=(50u)*(V(p5)-V(p4))
"
        ),
        "{amplifier}"
    );
    // VoutMax holds the output with the amplifier's own transconductance.
    assert!(
        amplifier.contains("Bc c 0 V=min(V(xf),abs((50u))*((2.5)-V(p3)+V(p2)))"),
        "{amplifier}"
    );
    // Isrc bends the current both ways, the sinking limit defaulting to it.
    assert!(
        amplifier.contains(
            "Bi i 0 V=uramp(V(c))*(10u)/(uramp(V(c))+(10u))-uramp(-V(c))*(10u)/(uramp(-V(c))+(10u))"
        ),
        "{amplifier}"
    );
    assert!(
        amplifier.contains(
            "Bsnk p3 p2 I=uramp(-V(i))*min((V(p3)-V(p2))/(0.5),1)*(2-max(min((V(p3)-V(p2))/(0.5),1),0))"
        ),
        "{amplifier}"
    );
    let unmodelled = imported
        .notices
        .iter()
        .find(|notice| notice.message.contains("¥1"))
        .expect("the gate's notice");
    assert!(unmodelled.message.contains("td2"), "{}", unmodelled.message);
    let parts = imported
        .document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::CellInstance)
        .map(|component| (component.name.as_str(), component.value.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(parts, [("X1", "AND2Q_1"), ("X2", "Gm_2")]);
    assert_wires_on_grid(&imported);
}

/// LTspice's UniversalOpAmp2 at level2 is RSpice's Op-Amp with its settings
/// on the sheet; another level, or a setting the Op-Amp has no field for,
/// keeps LTspice's part with a notice.
#[test]
fn an_ltspice_universal_opamp_at_level2_is_the_functional_op_amp() {
    let mut library = Library::default();
    library.symbols.insert(
        "opamps\\universalopamp2".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(UNIVERSAL_OPAMP_ASY).expect("the test symbol reads"),
            location: "opamps/UniversalOpAmp2.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc(
        "Version 4\nSHEET 1 1600 400\n\
         FLAG -32 16 in\nFLAG -32 -16 out\nFLAG 0 -32 vcc\nFLAG 0 32 vee\nFLAG 32 0 out\n\
         SYMBOL Opamps\\UniversalOpAmp2 0 0 R0\nSYMATTR InstName U1\nSYMATTR SpiceLine Ilimit=10m Rail=0.1\n\
         FLAG 288 16 in\nFLAG 288 -16 out2\nFLAG 320 -32 vcc\nFLAG 320 32 vee\nFLAG 352 0 out2\n\
         SYMBOL Opamps\\UniversalOpAmp2 320 0 R0\nSYMATTR InstName U2\nSYMATTR SpiceModel level3a\n\
         FLAG 608 16 in\nFLAG 608 -16 out3\nFLAG 640 -32 vcc\nFLAG 640 32 vee\nFLAG 672 0 out3\n\
         SYMBOL Opamps\\UniversalOpAmp2 640 0 R0\nSYMATTR InstName U3\nSYMATTR SpiceLine2 Phimargin=45\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_wires_on_grid(&imported);
    let part = |name: &str| {
        imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("{name} was placed"))
    };
    let functional = |name: &str| {
        part(name)
            .library_cell
            .as_ref()
            .and_then(|binding| binding.builtin_functional.as_ref())
            .map(|contract| contract.stable_id.as_str())
    };
    assert_eq!(functional("U1"), Some("rspice.functional.opamp"));
    let params = part("U1").params.split_whitespace().collect::<Vec<_>>();
    for setting in [
        "AVOL=1Meg",
        "GBW=10Meg",
        "SLEW=10Meg",
        "ISRC=10m",
        "ISNK=10m",
        "HEADROOM=0.1",
        "RIN=500Meg",
    ] {
        assert!(params.contains(&setting), "{setting} in {params:?}");
    }
    for kept in ["U2", "U3"] {
        assert_eq!(functional(kept), None);
        assert!(
            imported
                .notices
                .iter()
                .any(|notice| notice.message.starts_with(kept)
                    && notice.message.contains("keeps LTspice's own part")),
            "{kept}: {:#?}",
            imported.notices
        );
    }
}

/// A QSPICE pin that names a net of its own joins it when nothing is wired
/// to it: the Gm amplifier's Vdd pin the `Vdd` rail, its Vss pin ground. A
/// behavioural device RSpice has no part for is refused by what it is.
#[test]
fn a_qspice_pin_nothing_reaches_joins_its_own_net() {
    const DRAWING: &str = "«schematic
  «component (0,0) 0 0
    «symbol GmAmp
      «type: \u{c3}»
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"Ã1\"»
      «text (0,-200) 0.5 0 2 0x1000000 -1 -1 \"MultGmAmp\"»
      «text (0,-300) 1 7 0 0x1000000 -1 -1 \"Gm=1m\"»
      «pin (0,400) (0,0) 1 0 0 0x1000000 -1 \"Vdd\" \"Vdd\"»
      «pin (0,-400) (0,0) 1 0 0 0x1000000 -1 \"Vss\" \"GND\"»
      «pin (400,0) (0,0) 1 0 0 0x1000000 -1 \"OUT\"»
      «pin (-400,200) (0,0) 1 0 0 0x1000000 -1 \"IN-\"»
      «pin (-400,-200) (0,0) 1 0 0 0x1000000 -1 \"IN+\"»
    »
  »
  «component (3000,0) 0 0
    «symbol DFLOP
      «type: ¥»
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"¥2\"»
      «text (0,-200) 0.5 0 2 0x1000000 -1 -1 \"D-FLOP\"»
      «pin (0,400) (0,0) 1 0 0 0x1000000 -1 \"Vdd\" \"Vdd\"»
      «pin (0,-400) (0,0) 1 0 0 0x1000000 -1 \"Vss\" \"GND\"»
      «pin (400,0) (0,0) 1 0 0 0x1000000 -1 \"Q\"»
      «pin (-400,0) (0,0) 1 0 0 0x1000000 -1 \"D\"»
    »
  »
  «net (400,0) 1 13 0 \"out\"»
  «net (-400,200) 1 13 0 \"out\"»
  «net (-400,-200) 1 13 0 \"in\"»
»
";
    let drawing = parse_qsch(&qsch_bytes(DRAWING)).unwrap();
    let pins = &drawing.symbols[0]
        .embedded
        .as_ref()
        .expect("QSPICE embeds the symbol")
        .pins;
    assert_eq!(pins[0].default_net.as_deref(), Some("Vdd"));
    assert_eq!(pins[2].default_net, None);
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let refusal = imported
        .notices
        .iter()
        .find(|notice| notice.severity == ForeignNoticeSeverity::Refusal)
        .expect("the flip-flop is refused");
    assert!(
        refusal.message.contains("D flip-flop (¥-device D-FLOP)"),
        "{}",
        refusal.message
    );
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
    let amplifier = imported
        .document
        .components
        .iter()
        .find(|component| component.kind == ComponentType::CellInstance)
        .expect("the amplifier is placed");
    let binding = amplifier.library_cell.as_ref().expect("a cell instance");
    let nets = binding
        .terminal_order
        .iter()
        .map(|pin| {
            let net = &connectivity
                .terminals
                .iter()
                .find(|terminal| terminal.component_id == amplifier.id && &terminal.pin == pin)
                .unwrap_or_else(|| panic!("no terminal {pin}"))
                .net_name;
            if crate::schematic::ground_names::is_ground_reference(net) {
                "0".to_owned()
            } else {
                net.to_ascii_lowercase()
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(nets, ["vdd", "0", "out", "out", "in"]);
    assert_wires_on_grid(&imported);
    // The refused flip-flop's note hangs from its body's top left corner,
    // its label under the words, not across the parts beside it.
    let note = imported
        .document
        .design_notes
        .iter()
        .find(|note| note.text.starts_with("Not imported"))
        .expect("the flip-flop leaves a note");
    assert_eq!(note.text, "Not imported:\n¥2 (DFLOP)");
    assert_eq!(note.pos, Point::new(260, -40));
    // The note marks no point: no anchor mark sits at its corner, where a
    // flag on the refused part's pin would touch it.
    assert!(note.anchor_mark_hidden);
}

/// The note RSpice leaves where it refused a part hangs from the top left
/// of the part's gap. Where its words would run onto a part beside the gap,
/// it moves to the first other place along the gap's top or bottom where
/// they land on nothing: here along the top, past the resistor whose body
/// stands in the corner.
#[test]
fn a_refused_parts_note_hangs_where_it_lands_on_nothing() {
    let mut library = library();
    library.symbols.insert(
        "wide".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(
                "Version 4\nSymbolType CELL\nRECTANGLE Normal -320 -160 320 160\n\
                 PIN -320 0 NONE 0\nPINATTR PinName IN\nPINATTR SpiceOrder 1\n",
            )
            .expect("the test symbol reads"),
            location: "wide.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL wide 0 0 R0\nSYMATTR InstName @1\n\
         SYMBOL res -256 -176 R0\nSYMATTR InstName R1\nSYMATTR Value 1k\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let note = imported
        .document
        .design_notes
        .iter()
        .find(|note| note.text.starts_with("Not imported"))
        .expect("the probe leaves a note");
    // From the top left, (-200, -100), the words would run over the
    // resistor at x = -150.
    assert_eq!(note.pos.y, -100);
    assert!(note.pos.x > -150, "{:?}", note.pos);
}

/// An LTspice pin with no `PinName` letters nothing in LTspice. RSpice names
/// its port for the pin's place on the instance line, but does not letter
/// that made-up name on the drawing.
#[test]
fn an_ltspice_pin_with_no_name_letters_none() {
    const UNNAMED_ASY: &str = "Version 4\nSymbolType BLOCK\nRECTANGLE Normal -32 -32 32 32\n\
        SYMATTR Prefix X\nSYMATTR SpiceModel unnamed.sub\nSYMATTR Value unnamed\n\
        PIN -32 0 LEFT 8\nPINATTR PinName IN\nPINATTR SpiceOrder 1\n\
        PIN 32 0 RIGHT 8\nPINATTR SpiceOrder 2\n";
    let mut library = library();
    library.symbols.insert(
        "unnamed".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(UNNAMED_ASY).expect("the test symbol reads"),
            location: "unnamed.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc("Version 4\nSYMBOL unnamed 64 64 R0\nSYMATTR InstName X1\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let cell = imported
        .cells
        .iter()
        .find(|cell| cell.source_symbol == "unnamed")
        .expect("the unnamed cell");
    let resolved = ResolvedCellSymbol::from_authored_document(cell.symbol.clone(), &cell.ports);
    assert_eq!(
        resolved
            .connectable_pins()
            .map(|pin| (pin.name.clone(), pin.name_hidden))
            .collect::<BTreeMap<_, _>>(),
        BTreeMap::from([("IN".to_owned(), false), ("P2".to_owned(), true)])
    );
}

/// A QSPICE pin sets where its name stands: its second point is the name's
/// anchor from the pin, and its text code says which way the name reaches
/// from there. A supply pin at the end of a lead drawn through an
/// amplifier's body letters its name beside the lead's end, outside the
/// body, rather than down the lead across the body's edge.
#[test]
fn a_qspice_pin_name_stands_where_the_drawing_sets_it() {
    let drawing = parse_qsch(&qsch_bytes(
        "«schematic
  «component (0,0) 0 0
    «symbol AMP
      «type: X»
      «line (-700,400) (-700,-400) 0 0 0x1000000 -1 -1»
      «triangle (-1400,600) (-1400,-600) (0,0) 0 0 0x1000000 0x1000000 -1 -1»
      «text (-700,150) 1 15 0 0x1000000 -1 -1 \"X1\"»
      «text (-700,-100) 1 15 0 0x1000000 -1 -1 \"amp\"»
      «pin (-1400,300) (20,0) 1 7 0 0x0 -1 \"IN\"»
      «pin (-700,400) (100,20) 1 7 0 0x0 -1 \"VP\"»
      «pin (-700,-400) (300,10) 1 11 0 0x0 -1 \"VN\"»
      «pin (0,0) (-20,0) 1 11 0 0x0 -1 \"OUT\"»
    »
  »
»
",
    ))
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let cell = imported
        .cells
        .iter()
        .find(|cell| cell.source_symbol == "AMP")
        .expect("the amplifier's cell");
    let resolved = ResolvedCellSymbol::from_authored_document(cell.symbol.clone(), &cell.ports);
    let placed = |name: &str| {
        let pin = resolved
            .connectable_pins()
            .find(|pin| pin.name == name)
            .unwrap_or_else(|| panic!("no pin {name}"));
        let terminal = resolved.effective_pin_offset(pin);
        let anchor = resolved.pin_label_anchor(pin);
        (
            Point::new(anchor.x - terminal.x, anchor.y - terminal.y),
            pin.name_at.map(|at| at.reach),
        )
    };
    assert_eq!(placed("VP"), (Point::new(10, -2), Some(Point::new(1, 0))));
    assert_eq!(placed("VN"), (Point::new(30, -1), Some(Point::new(-1, 0))));
}

/// An LTspice pin on a slanted edge names itself where LTspice letters it,
/// beside the pin, though RSpice reads the pin onto the top of the box
/// around the body and would letter the name below it, across the edge. A
/// pin on an edge of that box keeps RSpice's own placement.
#[test]
fn an_ltspice_pin_on_a_slanted_edge_names_itself_where_ltspice_letters_it() {
    const SLANTED_ASY: &str = "Version 4\nSymbolType CELL\n\
        LINE Normal -32 -64 32 -64\nLINE Normal 32 -64 96 64\n\
        LINE Normal 96 64 -32 64\nLINE Normal -32 64 -32 -64\n\
        SYMATTR Prefix X\nSYMATTR SpiceModel slanted.sub\nSYMATTR Value slanted\n\
        PIN 40 -48 RIGHT 8\nPINATTR PinName FB\nPINATTR SpiceOrder 1\n\
        PIN -32 0 LEFT 8\nPINATTR PinName IN\nPINATTR SpiceOrder 2\n";
    let mut library = Library::default();
    library.symbols.insert(
        "slanted".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(SLANTED_ASY).expect("the test symbol reads"),
            location: "slanted.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc("Version 4\nSYMBOL slanted 0 0 R0\nSYMATTR InstName X1\n")
        .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let cell = imported
        .cells
        .iter()
        .find(|cell| cell.source_symbol == "slanted")
        .expect("the symbol's cell");
    let resolved = ResolvedCellSymbol::from_authored_document(cell.symbol.clone(), &cell.ports);
    let placed = |name: &str| {
        let pin = resolved
            .connectable_pins()
            .find(|pin| pin.name == name)
            .unwrap_or_else(|| panic!("no pin {name}"));
        let terminal = resolved.effective_pin_offset(pin);
        let anchor = resolved.pin_label_anchor(pin);
        (
            Point::new(anchor.x - terminal.x, anchor.y - terminal.y),
            pin.name_at.map(|at| at.reach),
        )
    };
    // Eight units in from the pin, reaching left, centred on its line.
    assert_eq!(placed("FB"), (Point::new(-5, 0), Some(Point::new(-1, 0))));
    assert_eq!(placed("IN").1, None);
}

/// A drawn ellipse as large as a sheet stays on the sheet, as fine a ring as
/// a polygon may be.
#[test]
fn a_sheet_sized_drawn_ellipse_is_kept() {
    use crate::schematic::documentation_shape::{
        DocumentationShapeGeometry, MAX_DOCUMENTATION_POLYGON_POINTS,
    };

    let drawing = parse_asc("Version 4\nCIRCLE Normal 0 0 1600 1200\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let notices = format!("{:?}", imported.notices);
    assert!(!notices.contains("zero size"), "{notices}");
    let [shape] = imported.document.documentation_shapes.as_slice() else {
        panic!("one shape: {:?}", imported.document.documentation_shapes);
    };
    let DocumentationShapeGeometry::Polygon { points } = &shape.geometry else {
        panic!("a ring: {shape:?}");
    };
    assert!(points.len() > 256 && points.len() <= MAX_DOCUMENTATION_POLYGON_POINTS);
}

/// LTspice letters a name on its side where the pin's justification starts
/// with `V`: the name reads upward and runs from the pin into the body.
#[test]
fn a_pin_name_lettered_on_its_side_reads_upward_into_the_body() {
    use crate::schematic::label_layout::TextEdge;

    const CHIP_ASY: &str = "Version 4\nSymbolType CELL\n\
        RECTANGLE Normal -160 -160 160 160\n\
        SYMATTR Prefix X\nSYMATTR SpiceModel chip.sub\nSYMATTR Value chip\n\
        PIN -80 -160 VRIGHT 8\nPINATTR PinName REF\nPINATTR SpiceOrder 1\n\
        PIN 80 160 VLEFT 8\nPINATTR PinName AGND\nPINATTR SpiceOrder 2\n\
        PIN -160 0 LEFT 8\nPINATTR PinName IN\nPINATTR SpiceOrder 3\n";
    let mut library = Library::default();
    library.symbols.insert(
        "chip".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(CHIP_ASY).expect("the test symbol reads"),
            location: "chip.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc("Version 4\nSYMBOL chip 0 0 R0\nSYMATTR InstName X1\n")
        .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let cell = imported
        .cells
        .iter()
        .find(|cell| cell.source_symbol == "chip")
        .expect("the symbol's cell");
    let resolved = ResolvedCellSymbol::from_authored_document(cell.symbol.clone(), &cell.ports);
    let upright = imported.document.components[0].clone();
    let mut turned = upright.clone();
    turned.rotation = crate::schematic::rotation::Rotation::R90;
    let placed = |name: &str| {
        let pin = resolved
            .connectable_pins()
            .find(|pin| pin.name == name)
            .unwrap_or_else(|| panic!("no pin {name}"));
        let terminal = resolved.effective_pin_offset(pin);
        let anchor = resolved.pin_label_anchor(pin);
        (
            Point::new(anchor.x - terminal.x, anchor.y - terminal.y),
            upright.pin_name_hang(pin),
            turned.pin_name_hang(pin),
        )
    };
    let (edge_in, hang, on_its_side) = placed("REF");
    assert_eq!(edge_in, Point::new(0, 5), "five units below the top edge");
    assert_eq!(
        (hang.horizontal, hang.vertical, hang.upward),
        (TextEdge::End, TextEdge::Center, true),
        "its last letter nearest the pin"
    );
    assert!(!on_its_side.upward, "a quarter turn stands it upright");
    let (edge_in, hang, _) = placed("AGND");
    assert_eq!(
        edge_in,
        Point::new(0, -5),
        "five units above the bottom edge"
    );
    assert_eq!(
        (hang.horizontal, hang.vertical, hang.upward),
        (TextEdge::Start, TextEdge::Center, true),
        "its first letter nearest the pin"
    );
    assert!(!placed("IN").1.upward, "a left pin's name stays upright");
}

/// LTspice letters a symbol's attributes only where a WINDOW shows them. A
/// layout symbol that shows its name and no value, or neither, letters as
/// much here and no more: the value RSpice would otherwise place itself
/// lands on the part's own artwork.
#[test]
fn an_ltspice_attribute_with_no_window_is_not_lettered() {
    use crate::schematic::component_display::ComponentDisplayMode;

    const NAMED_ASY: &str = "Version 4\nSymbolType CELL\nRECTANGLE Normal -16 -24 16 24\n\
        WINDOW 0 0 0 VCenter 2\nSYMATTR Prefix D\nSYMATTR Value D\n\
        PIN 0 -32 NONE 0\nPINATTR PinName A\nPINATTR SpiceOrder 1\n\
        PIN 0 32 NONE 0\nPINATTR PinName K\nPINATTR SpiceOrder 2\n";
    const BARE_ASY: &str = "Version 4\nSymbolType CELL\nRECTANGLE Normal -16 -24 16 24\n\
        SYMATTR Prefix C\nSYMATTR Value C\n\
        PIN 0 -32 NONE 0\nPINATTR PinName 1\nPINATTR SpiceOrder 1\n\
        PIN 0 32 NONE 0\nPINATTR PinName 2\nPINATTR SpiceOrder 2\n";
    let mut library = library();
    for (name, text) in [("padded", NAMED_ASY), ("bare", BARE_ASY)] {
        library.symbols.insert(
            name.to_owned(),
            ForeignSymbolSource {
                definition: parse_asy(text).expect("the test symbol reads"),
                location: format!("{name}.asy"),
                schematic: None,
            },
        );
    }
    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL padded 0 0 R0\nSYMATTR InstName D7\nSYMATTR Value 1N914\n\
         SYMBOL bare 160 0 R0\nSYMATTR InstName C7\nSYMATTR Value 47n\n\
         SYMBOL padded 320 0 R0\nWINDOW 3 24 0 Left 2\nSYMATTR InstName D8\nSYMATTR Value 1N914\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let shown = |name: &str| {
        imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("{name} is placed"))
            .display_mode
    };
    assert_eq!(shown("D7"), ComponentDisplayMode::NameOnly);
    assert_eq!(shown("C7"), ComponentDisplayMode::Hidden);
    // The drawing's own WINDOW shows the value of this one.
    assert_eq!(shown("D8"), ComponentDisplayMode::Inherit);
}

/// A pin its source drawing letters no name for draws none here: LTspice's
/// `NONE`, a place the symbol draws no pin for, and a QSPICE pin with no
/// justification or one centred on its own point. A lettered pin keeps its
/// name.
#[test]
fn a_pin_its_drawing_letters_no_name_for_draws_none() {
    const LABELLED_ASY: &str = "Version 4\nSymbolType BLOCK\nRECTANGLE Normal -32 -32 32 32\n\
        SYMATTR Prefix X\nSYMATTR SpiceModel labelled.sub\nSYMATTR Value labelled\n\
        LINE Normal -32 8 -24 0\nLINE Normal -32 -8 -24 0\n\
        PIN -32 0 LEFT 8\nPINATTR PinName IN\nPINATTR SpiceOrder 1\n\
        PIN 32 0 NONE 0\nPINATTR PinName OUT\nPINATTR SpiceOrder 2\n";
    let mut library = library();
    library.symbols.insert(
        "labelled".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(LABELLED_ASY).expect("the test symbol reads"),
            location: "labelled.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc(
        "Version 4\nSHEET 1 800 600\nSYMBOL labelled 64 64 R0\nSYMATTR InstName X1\n\
         SYMBOL Digital\\and 448 64 R0\nSYMATTR InstName A1\n",
    )
    .unwrap();
    let ltspice = import_foreign_schematic(&drawing, "work", &library);

    const DRAWING: &str = "«schematic
  «component (0,0) 0 0
    «symbol GmAmp
      «type: \u{c3}»
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"Ã1\"»
      «text (0,-200) 0.5 0 2 0x1000000 -1 -1 \"MultGmAmp\"»
      «text (0,-300) 1 7 0 0x1000000 -1 -1 \"Gm=1m\"»
      «pin (0,400) (0,0) 1 0 0 0x1000000 -1 \"Vdd\"»
      «pin (0,-400) (0,0) 1 0 0 0x1000000 -1 \"Vss\"»
      «pin (400,0) (-50,0) 1 11 0 0x1000000 -1 \"OUT\"»
      «pin (-400,200) (50,0) 1 15 0 0x1000000 -1 \"IN-\"»
      «pin (-400,-200) (50,0) 1 7 0 0x1000000 -1 \"IN+\"»
    »
  »
»
";
    let qspice = import_foreign_schematic(
        &parse_qsch(&qsch_bytes(DRAWING)).unwrap(),
        "work",
        &Library::default(),
    );

    let hidden = |imported: &ForeignImport, source: &str| {
        let cell = imported
            .cells
            .iter()
            .find(|cell| cell.source_symbol.eq_ignore_ascii_case(source))
            .unwrap_or_else(|| panic!("no cell for {source}"));
        let resolved = ResolvedCellSymbol::from_authored_document(cell.symbol.clone(), &cell.ports);
        resolved
            .connectable_pins()
            .map(|pin| (pin.name.clone(), pin.name_hidden))
            .collect::<BTreeMap<_, _>>()
    };
    assert_eq!(
        hidden(&ltspice, "labelled"),
        BTreeMap::from([("IN".to_owned(), false), ("OUT".to_owned(), true)])
    );
    // A lettered pin's name starts where the symbol starts it: eight drawing
    // units in from the pin, past a clock wedge drawn there.
    let labelled = ltspice
        .cells
        .iter()
        .find(|cell| cell.source_symbol == "labelled")
        .expect("the labelled cell");
    let resolved =
        ResolvedCellSymbol::from_authored_document(labelled.symbol.clone(), &labelled.ports);
    let input = resolved
        .connectable_pins()
        .find(|pin| pin.name == "IN")
        .expect("the IN pin");
    assert_eq!(input.name_offset, Some(5));
    let terminal = resolved.effective_pin_offset(input);
    assert_eq!(
        resolved.pin_label_anchor(input),
        Point::new(terminal.x + 5, terminal.y)
    );
    // An A-device draws no pin names, and its places without a pin share one
    // point: none of them letters a name over the others.
    let gate = hidden(&ltspice, "Digital\\and");
    assert_eq!(gate.len(), 8, "{gate:?}");
    assert!(gate.values().all(|hidden| *hidden), "{gate:?}");
    assert_eq!(
        hidden(&qspice, "GmAmp"),
        BTreeMap::from([
            ("IN+".to_owned(), false),
            ("IN-".to_owned(), true),
            ("OUT".to_owned(), false),
            ("Vdd".to_owned(), true),
            ("Vss".to_owned(), true),
        ])
    );
}

/// A QSPICE net label's name stands on the side its drawing runs it to:
/// under, left of, over or right of its point. A label placed here reads to
/// the right, and a project written without a side loads that way.
#[test]
fn a_qspice_net_label_name_stands_on_the_side_its_drawing_runs_it_to() {
    use crate::schematic::label_layout::{
        LINE_BOX_EM, NET_LABEL_FONT_WORLD, NET_LABEL_GLYPH_WIDTH_WORLD, net_label_font_world,
        net_label_name_corner,
    };
    use crate::schematic::net_label::{NetLabel, NetLabelSide};

    const DRAWING: &str = "«schematic
  «net (0,0) 1 13 0 \"under\"»
  «net (2000,0) 1 11 0 \"left\"»
  «net (4000,0) 1 14 0 \"over\"»
  «net (6000,0) 1 7 0 \"right\"»
»
";
    let imported = import_foreign_schematic(
        &parse_qsch(&qsch_bytes(DRAWING)).unwrap(),
        "work",
        &Library::default(),
    );
    let label = |name: &str| {
        imported
            .document
            .net_labels
            .iter()
            .find(|label| label.name == name)
            .unwrap_or_else(|| panic!("no label {name}"))
    };
    for (name, side) in [
        ("under", NetLabelSide::Below),
        ("left", NetLabelSide::Left),
        ("over", NetLabelSide::Above),
        ("right", NetLabelSide::Right),
    ] {
        let label = label(name);
        assert_eq!(label.side, side, "{name}");
        // The name's box, from its bottom-left corner.
        let corner = net_label_name_corner(label);
        let font = net_label_font_world(label);
        let width = name.len() as f32 * NET_LABEL_GLYPH_WIDTH_WORLD * font / NET_LABEL_FONT_WORLD;
        let (x, y) = (label.pos.x as f32, label.pos.y as f32);
        let centred_on_its_line = (corner.y - font * LINE_BOX_EM * 0.5 - y).abs() < 0.01;
        match side {
            NetLabelSide::RisingAbove | NetLabelSide::RisingBelow | NetLabelSide::RisingBeside => {
                unreachable!("{name} reads level")
            }
            NetLabelSide::Below => {
                assert!(corner.y > y, "{name} hangs under its point");
                assert!(
                    (corner.x + width * 0.5 - x).abs() < 0.01,
                    "{name} is centred"
                );
            }
            NetLabelSide::Above => {
                assert!(corner.y - font * 0.275 < y, "{name} stands over its point");
                assert!(
                    (corner.x + width * 0.5 - x).abs() < 0.01,
                    "{name} is centred"
                );
            }
            NetLabelSide::Left => {
                assert!(corner.x + width < x, "{name} ends short of its point");
                assert!(centred_on_its_line, "{name} is centred on its line");
            }
            NetLabelSide::Right => {
                assert!(corner.x > x, "{name} starts past its point");
                assert!(centred_on_its_line, "{name} is centred on its line");
            }
        }
    }

    let native = NetLabel::new(1, Point::new(0, 0), "n");
    assert_eq!(native.side, NetLabelSide::Right);
    let written = serde_json::to_value(&native).unwrap();
    assert!(written.get("side").is_none(), "{written}");
    let restored: NetLabel = serde_json::from_value(written).unwrap();
    assert_eq!(restored, native);
}

/// A comment keeps the size its drawing letters it at, and its text's box
/// stands on its point the way the drawing stood it there: centred on it,
/// standing on it, hanging from it or starting at it. A note placed here
/// keeps the standard size, and a project written without one loads at it.
#[test]
fn a_comment_keeps_its_size_and_stands_on_its_point_as_its_drawing_stood_it() {
    use crate::schematic::design_note::{DesignNote, DesignNoteKind};

    const DRAWING: &str = "«schematic
  «net (0,0) 1 7 0 \"centre\"»
  «net (4000,0) 1 7 0 \"base\"»
  «text (0,0) 0.8 15 1 0x1000000 -1 -1 \"centred\\nnote\"»
  «text (4000,0) 1 14 1 0x1000000 -1 -1 \"standing\"»
»
";
    let qspice = import_foreign_schematic(
        &parse_qsch(&qsch_bytes(DRAWING)).unwrap(),
        "work",
        &Library::default(),
    );
    let ltspice = import_foreign_schematic(
        &parse_asc(
            "Version 4\nSHEET 1 800 600\nFLAG 0 0 top\nFLAG 640 0 left\n\
             TEXT 0 0 Top 1 ;hanging\nTEXT 640 0 Left 2 ;starting\n",
        )
        .unwrap(),
        "work",
        &Library::default(),
    );
    // The note's size, and its text's box: left, top, right, bottom.
    let lettered = |imported: &ForeignImport, text: &str| {
        let note = imported
            .document
            .design_notes
            .iter()
            .find(|note| note.text == text)
            .unwrap_or_else(|| panic!("no note {text:?}"));
        let drawn = note.lettered_box(note.lettering.expect("lettered"), &note.text);
        (
            note.text_size_percent,
            [drawn.min.x, drawn.min.y, drawn.max.x, drawn.max.y],
        )
    };
    let point = |imported: &ForeignImport, name: &str| {
        let label = imported
            .document
            .net_labels
            .iter()
            .find(|label| label.name == name)
            .unwrap_or_else(|| panic!("no label {name}"));
        (label.pos.x as f32, label.pos.y as f32)
    };
    let near = |a: f32, b: f32| (a - b).abs() <= 0.5;

    let (size, [left, top, right, bottom]) = lettered(&qspice, "centred\nnote");
    let (x, y) = point(&qspice, "centre");
    assert_eq!(size, 80);
    assert!(near((left + right) * 0.5, x) && near((top + bottom) * 0.5, y));

    let (size, [left, _, right, bottom]) = lettered(&qspice, "standing");
    let (x, y) = point(&qspice, "base");
    assert_eq!(size, 100);
    assert!(near((left + right) * 0.5, x) && near(bottom, y));

    let (size, [left, top, right, _]) = lettered(&ltspice, "hanging");
    let (x, y) = point(&ltspice, "top");
    assert_eq!(size, 67, "LTspice size 1 is 1.0x against the standard 1.5x");
    assert!(near((left + right) * 0.5, x) && near(top, y));

    let (size, [left, top, _, bottom]) = lettered(&ltspice, "starting");
    let (x, y) = point(&ltspice, "left");
    assert_eq!(size, 100);
    assert!(near(left, x) && near((top + bottom) * 0.5, y));

    let native = DesignNote::new(1, Point::new(0, 0), DesignNoteKind::PlainText, "n").unwrap();
    assert!(native.has_standard_text_size());
    let written = serde_json::to_value(&native).unwrap();
    assert!(written.get("text_size_percent").is_none(), "{written}");
    let restored: DesignNote = serde_json::from_value(written).unwrap();
    assert_eq!(restored, native);
}

/// A comment comes across as lettering alone: its drawing marks no anchor,
/// so the note draws none either, and nor does directive text kept on the
/// sheet. A note RSpice writes itself, here for a readout the drawing pinned
/// to a wire, keeps its mark.
#[test]
fn every_line_of_an_imported_text_keeps_its_justification() {
    use crate::schematic::label_layout::{PlacedText, TextEdge};

    let drawing = parse_asc(
        "Version 4\n\
         TEXT 160 160 Bottom 2 ;wide first line\\nnext\\nlast\n\
         TEXT 800 160 Right 2 ;ends here\\nand here\n\
         TEXT 1600 160 VLeft 2 ;reads up\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let lines = |start: &str| -> Vec<(PlacedText, String)> {
        let note = imported
            .document
            .design_notes
            .iter()
            .find(|note| note.text.starts_with(start))
            .unwrap_or_else(|| panic!("no note {start}"));
        assert!(note.anchor_mark_hidden, "{start}");
        note.lettered_lines(note.lettering.expect("lettered"), &note.text)
            .into_iter()
            .map(|(placed, line)| (placed, line.to_owned()))
            .collect()
    };

    let centred = lines("wide");
    assert_eq!(centred.len(), 3);
    for (placed, _) in &centred {
        assert_eq!(placed.anchor.x, 100.0, "every line is centred on the point");
        assert_eq!(placed.horizontal, TextEdge::Center);
    }
    assert!(
        centred[2].0.anchor.y < 100.0,
        "the last line ends over the point"
    );
    assert_eq!(
        centred[1].0.anchor.y - centred[0].0.anchor.y,
        centred[2].0.anchor.y - centred[1].0.anchor.y
    );

    let right = lines("ends");
    assert_eq!(right[0].0.horizontal, TextEdge::End);
    assert_eq!(right[0].0.anchor.x, right[1].0.anchor.x);
    assert_eq!(right[0].0.anchor.x, 500.0, "each line ends on the point");
    assert!(
        right[0].0.anchor.y == 100.0,
        "standing on the point by its first line"
    );

    let upward = lines("reads");
    assert!(upward[0].0.upward);
    assert_eq!(upward[0].0.horizontal, TextEdge::Start);
    assert_eq!(
        upward[0].0.anchor.y, 100.0,
        "it starts on the point, rising"
    );
}

#[test]
fn a_comment_draws_no_anchor_mark_where_its_drawing_drew_none() {
    let drawing = parse_asc(
        "Version 4\nTEXT 32 160 Left 2 ;Bias network\nTEXT 32 320 Left 2 !.options gmin=1p\n\
         DATAFLAG 96 80 \"$\"\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let note = |start: &str| {
        imported
            .document
            .design_notes
            .iter()
            .find(|note| note.text.starts_with(start))
            .unwrap_or_else(|| panic!("no note {start:?}"))
    };
    assert!(note("Bias network").anchor_mark_hidden);
    assert!(note(".options").anchor_mark_hidden);
    assert!(!note("LTspice readout").anchor_mark_hidden);
}

/// A rail flag draws the glyph its name calls for, turned away from the wire
/// that reaches it: a VSS flag on top of a source's lead stands its bar over
/// the point, and a VCC flag under an IC's pin hangs its bar below, where
/// the conventional glyph would stand on the lead and letter the name over
/// the part.
#[test]
fn an_imported_rail_turns_its_glyph_away_from_what_reaches_it() {
    use crate::schematic::net_label::{NetLabelKind, SupplyGlyph};

    let drawing = parse_asc(
        "Version 4\n\
         WIRE 0 0 0 64\nFLAG 0 0 Vss\n\
         WIRE 160 0 160 64\nFLAG 160 64 Vcc\n\
         WIRE 320 0 320 64\nFLAG 320 0 VDD\n\
         WIRE 480 0 480 64\nFLAG 480 64 VEE\n\
         WIRE 640 0 704 0\nWIRE 640 0 640 64\nFLAG 640 0 VSS2\n\
         WIRE 800 0 800 64\nWIRE 800 64 800 128\nFLAG 800 64 VEE2\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let glyph = |name: &str| {
        let label = imported
            .document
            .net_labels
            .iter()
            .find(|label| label.name == name)
            .unwrap_or_else(|| panic!("no label {name}"));
        match label.kind {
            NetLabelKind::Supply { glyph } => glyph,
            kind => panic!("{name} is a {kind:?}, not a rail"),
        }
    };
    assert_eq!(glyph("Vss"), SupplyGlyph::Bar, "reached from below");
    assert_eq!(glyph("Vcc"), SupplyGlyph::Down, "reached from above");
    assert_eq!(glyph("VDD"), SupplyGlyph::Bar);
    assert_eq!(glyph("VEE"), SupplyGlyph::Down);
    assert_eq!(
        glyph("VSS2"),
        SupplyGlyph::Bar,
        "on a corner, reached from below"
    );
    assert_eq!(glyph("VEE2"), SupplyGlyph::Down, "on the middle of a run");
}

/// Where a net label of `imported` named `name` stands, in world units.
fn label_point(imported: &ForeignImport, name: &str) -> (f32, f32) {
    let label = imported
        .document
        .net_labels
        .iter()
        .find(|label| label.name == name)
        .unwrap_or_else(|| panic!("no label {name}"));
    (label.pos.x as f32, label.pos.y as f32)
}

/// A note's text box: left, top, right, bottom, and the height of one line.
fn note_box(note: &crate::schematic::design_note::DesignNote) -> ([f32; 4], f32) {
    use crate::schematic::label_layout::LINE_BOX_EM;

    let drawn = note.lettered_box(note.lettering.expect("lettered"), &note.text);
    (
        [drawn.min.x, drawn.min.y, drawn.max.x, drawn.max.y],
        note.lettered_size_world() * LINE_BOX_EM,
    )
}

/// LTspice letters a comment of several lines that it centres down by its
/// first line, centred on its point, and runs the rest below: its drawings
/// lay their notes out by that, a table's rows starting under the note
/// above them. A comment standing on its point by its bottom edge stands
/// there whole.
#[test]
fn an_ltspice_comment_of_several_lines_stands_on_its_point_by_its_first_line() {
    let imported = import_foreign_schematic(
        &parse_asc(
            "Version 4\nSHEET 1 800 600\nFLAG 0 0 left\nFLAG 640 0 base\n\
             TEXT 0 0 Left 2 ;first\\nsecond\\nthird\n\
             TEXT 640 0 Bottom 2 ;upper\\nlower\n",
        )
        .unwrap(),
        "work",
        &Library::default(),
    );
    let note = |start: &str| {
        imported
            .document
            .design_notes
            .iter()
            .find(|note| note.text.starts_with(start))
            .unwrap_or_else(|| panic!("no note {start:?}"))
    };
    let near = |a: f32, b: f32| (a - b).abs() <= 0.5;

    let ([left, top, _, _], line) = note_box(note("first"));
    let (x, y) = label_point(&imported, "left");
    assert!(
        near(left, x) && near(top + line * 0.5, y),
        "the first line stands centred on the point: {left},{top} against {x},{y}"
    );

    let ([_, _, _, bottom], _) = note_box(note("upper"));
    let (_, y) = label_point(&imported, "base");
    assert!(near(bottom, y), "{bottom} against {y}");
}

/// A directive block is lettered as the drawing lettered it: whole, at the
/// block's size, its first line centred on the drawing's point.
#[test]
fn a_directive_block_is_lettered_where_and_as_the_drawing_lettered_it() {
    let drawing =
        parse_asc("Version 4\nFLAG 640 0 base\nTEXT 640 0 Left 3 !.backanno\\n.options gmin=1p\n")
            .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let note = imported
        .document
        .design_notes
        .iter()
        .find(|note| note.text == ".backanno\n.options gmin=1p")
        .expect("the block is lettered whole");
    assert_eq!(
        note.text_size_percent, 133,
        "LTspice size 3 is 2.0x against 1.5x"
    );
    let near = |a: f32, b: f32| (a - b).abs() <= 0.5;
    let ([left, top, _, _], line) = note_box(note);
    let (x, y) = label_point(&imported, "base");
    assert!(
        near(left, x) && near(top + line * 0.5, y),
        "{left},{top} against {x},{y}"
    );
}

/// Whether `text` meets the body RSpice draws for `part`: the strokes of its
/// artwork, each walked in quarter-unit steps at its drawn width.
#[cfg(feature = "symbol-artwork")]
fn meets_drawn_body(
    part: &crate::schematic::component::Component,
    text: crate::schematic::label_layout::LayoutRect,
) -> bool {
    use crate::schematic::drawing_metrics::SYMBOL_WIDTH_WORLD;

    let artwork = crate::symbol_artwork::SymbolLibrary::load_embedded().expect("the artwork loads");
    let runs = artwork
        .placed_artwork(part)
        .expect("a part RSpice draws has artwork");
    let half = SYMBOL_WIDTH_WORLD / 2.0;
    let at = |(x, y): (f32, f32)| (part.pos.x as f32 + x, part.pos.y as f32 + y);
    runs.iter().any(|run| {
        run.points.windows(2).any(|piece| {
            let (start, end) = (at(piece[0]), at(piece[1]));
            let steps = ((end.0 - start.0).hypot(end.1 - start.1) * 4.0)
                .ceil()
                .max(1.0) as usize;
            (0..=steps).any(|step| {
                let t = step as f32 / steps as f32;
                let (x, y) = (
                    start.0 + (end.0 - start.0) * t,
                    start.1 + (end.1 - start.1) * t,
                );
                x > text.min.x - half
                    && x < text.max.x + half
                    && y > text.min.y - half
                    && y < text.max.y + half
            })
        })
    })
}

/// A QSPICE source's drawing letters its waveform in one text, on one
/// line, where RSpice, keeping the waveform as parameters, letters a line
/// for each. The imported source letters it as its drawing wrote it.
#[test]
fn an_imported_qspice_source_letters_its_waveform_on_the_line_its_drawing_wrote() {
    use crate::schematic::value_label::{component_value_label, value_pieces};

    const DRAWING: &str = "«schematic
  «component (0,0) 0 0
    «symbol V
      «type: V»
      «text (130,0) 1 110 0 0x1000000 -1 -1 \"V1\"»
      «text (-130,0) 1 109 0 0x1000000 -1 -1 \"PULSE(0 3 0 1n 1n 5µ 10µ)\"»
      «pin (0,200) (0,0) 1 0 0 0x1000000 -1 \"+\"»
      «pin (0,-200) (0,0) 1 0 0 0x1000000 -1 \"-\"»
    »
  »
»
";
    let drawing = parse_qsch(&qsch_bytes(DRAWING)).unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let source = imported
        .document
        .components
        .iter()
        .find(|component| component.kind.is_source())
        .expect("the source is placed");
    assert!(
        component_value_label(source).lines().count() > 1,
        "RSpice letters a pulse a line per parameter"
    );
    let pieces = value_pieces(source).expect("the source letters its waveform as written");
    let texts = pieces
        .iter()
        .map(|piece| piece.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(texts, ["PULSE(0 3 0 1n 1n 5µ 10µ)"]);
}

/// A ground QSPICE draws is its stemless triangle hung from the net point, and
/// the imported ground draws the same: a note the drawing sets a grid step
/// below the ground clears it, where RSpice's own ground, reaching further
/// down, would run over the note's words.
#[cfg(feature = "symbol-artwork")]
#[test]
fn a_qspice_ground_draws_its_triangle_clear_of_a_note_set_below_it() {
    use crate::schematic::ground_glyph::GroundGlyph;

    const DRAWING: &str = "«schematic
  «net (0,0) 1 13 0 \"GND\"»
  «wire (0,200) (0,0) \"GND\"»
  «text (0,-100) 1 13 1 0x1000000 -1 -1 \"a note below the ground\"»
»
";
    let drawing = parse_qsch(&qsch_bytes(DRAWING)).unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let ground = imported
        .document
        .components
        .iter()
        .find(|component| component.kind == ComponentType::Ground)
        .expect("the ground is placed");
    assert_eq!(ground.ground_glyph, GroundGlyph::Qspice);
    let artwork = crate::symbol_artwork::SymbolLibrary::load_embedded().expect("the artwork loads");
    let corners = artwork
        .placed_artwork(ground)
        .expect("the ground draws")
        .iter()
        .flat_map(|run| run.points.clone())
        .map(|(x, y)| (ground.pos.x as f32 + x, ground.pos.y as f32 + y))
        .collect::<Vec<_>>();
    // No stem: the top edge runs through the net point, 16.5 across, and the
    // sides fall at 45 degrees to an apex 8.25 below it.
    let net_point = ground.terminal_positions()[0].1;
    let (net_x, net_y) = (net_point.x as f32, net_point.y as f32);
    let near = |a: f32, b: f32| (a - b).abs() < 1e-3;
    for (x, y) in [
        (net_x - 8.25, net_y),
        (net_x + 8.25, net_y),
        (net_x, net_y + 8.25),
    ] {
        assert!(
            corners.iter().any(|&(cx, cy)| near(cx, x) && near(cy, y)),
            "no corner at {x},{y} in {corners:?}"
        );
    }
    assert!(
        corners
            .iter()
            .all(|&(_, y)| y >= net_y - 1e-3 && y <= net_y + 8.25 + 1e-3)
    );

    let note = imported
        .document
        .design_notes
        .iter()
        .find(|note| note.text == "a note below the ground")
        .expect("the note is placed");
    let words = note.lettered_box(note.lettering.expect("lettered"), &note.text);
    assert!(
        !meets_drawn_body(ground, words),
        "the ground runs over {words:?}"
    );
    let mut native = ground.clone();
    native.ground_glyph = GroundGlyph::Native;
    assert!(
        meets_drawn_body(&native, words),
        "RSpice's own ground would clear the note too: the case proves nothing"
    );
}

/// LTspice draws its ground flag as an open triangle with no stem, turned
/// away from the one side a wire reaches it from. The imported ground draws
/// LTspice's triangle and turns the same way; with a wire on both sides, or
/// none, it hangs below its point.
#[test]
fn an_ltspice_ground_turns_away_from_its_wire_and_draws_ltspices_triangle() {
    use crate::schematic::ground_glyph::GroundGlyph;

    let drawing = parse_asc(
        "Version 4
         WIRE 0 0 64 0
FLAG 0 0 0
         WIRE 160 -64 160 0
FLAG 160 0 0
         WIRE 256 64 256 0
FLAG 256 0 0
         WIRE 320 0 448 0
FLAG 384 0 0
",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let grounds = imported
        .document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::Ground)
        .map(|ground| {
            assert_eq!(ground.ground_glyph, GroundGlyph::Ltspice);
            let terminal = ground.terminal_positions()[0].1;
            ((terminal.x, terminal.y), ground.rotation)
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        grounds,
        BTreeMap::from([
            ((0, 0), Rotation::R90),
            ((100, 0), Rotation::R0),
            ((160, 0), Rotation::R180),
            ((240, 0), Rotation::R0),
        ])
    );
    // LTspice's triangle is 36 of its units across and 18 deep, drawn at
    // five eighths: its top edge on the terminal, its sides at 45 degrees.
    let [left, right, apex]: [(f32, f32); 3] = GroundGlyph::Ltspice
        .outline()
        .expect("an imported glyph")
        .try_into()
        .expect("a triangle");
    let (_, terminal) = ComponentType::Ground.terminal_offsets()[0];
    assert_eq!((left.1, right.1), (terminal.y as f32, terminal.y as f32));
    assert_eq!((right.0 - left.0, apex.1 - left.1), (22.5, 11.25));
    assert_eq!(apex.0, terminal.x as f32);
}

/// A resistor's symbol as a user's installation might draw it: pins at 16, 16
/// and 16, 96, a box between two leads, its name and value windowed.
const BOXED_RESISTOR_ASY: &str = "Version 4\nSymbolType CELL\n\
    LINE Normal 16 16 16 32\nLINE Normal 16 80 16 96\nRECTANGLE Normal 4 32 28 80\n\
    WINDOW 0 40 40 Left 2\nWINDOW 3 40 72 Left 1\n\
    SYMATTR Value R\nSYMATTR Prefix R\n\
    PIN 16 16 NONE 0\nPINATTR PinName A\nPINATTR SpiceOrder 1\n\
    PIN 16 96 NONE 0\nPINATTR PinName B\nPINATTR SpiceOrder 2\n";

fn library_of(name: &str, source: &str) -> Library {
    let mut library = Library::default();
    library.symbols.insert(
        name.to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(source).expect("the test symbol reads"),
            location: format!("{name}.asy"),
            schematic: None,
        },
    );
    library
}

fn wire_points(imported: &ForeignImport) -> BTreeSet<(i32, i32)> {
    imported
        .document
        .wires
        .iter()
        .flat_map(|wire| wire.points.iter().map(|point| (point.x, point.y)))
        .collect()
}

/// Sixteen LTspice units are one of RSpice's ten-unit steps, and a point that
/// falls between steps stays between them: scaling is a function of the
/// coordinate alone, so nothing merges or splits.
#[test]
fn an_ltspice_coordinate_scales_by_five_eighths_and_nothing_snaps() {
    let drawing =
        parse_asc("Version 4\nWIRE 0 0 16 0\nWIRE 16 0 16 8\nWIRE 16 8 -20 8\nFLAG -20 8 out\n")
            .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    assert_eq!(
        wire_points(&imported),
        BTreeSet::from([(0, 0), (10, 0), (10, 5), (-13, 5)])
    );
    assert_eq!(imported.document.net_labels[0].pos, Point::new(-13, 5));
}

/// A native part stays that part and draws the symbol its source tool draws,
/// its pins on the source's pins whichever way the drawing turns it.
#[test]
fn a_native_part_draws_its_sources_own_symbol_in_every_orientation() {
    let library = library_of("res", BOXED_RESISTOR_ASY);
    for orientation in ORIENTATIONS {
        let drawing = parse_asc(&format!(
            "Version 4\nSYMBOL res 160 160 {orientation}\nSYMATTR InstName R1\nSYMATTR Value 1k\n"
        ))
        .unwrap();
        let imported = import_foreign_schematic(&drawing, "work", &library);
        let resistor = &imported.document.components[0];
        assert_eq!(resistor.kind, ComponentType::Resistor, "{orientation}");
        let symbol = resistor.source_symbol().expect("the source's symbol");
        assert_eq!(symbol.document().body.len(), 3, "{orientation}");
        let placed = &drawing.symbols[0];
        let expected = [ForeignPoint::new(16, 16), ForeignPoint::new(16, 96)].map(|pin| {
            let point = placed.position.offset(placed.orientation.apply(pin));
            (point.x * 5 / 8, point.y * 5 / 8)
        });
        let terminals = resistor
            .terminal_positions()
            .into_iter()
            .map(|(_, point)| (point.x, point.y))
            .collect::<Vec<_>>();
        assert_eq!(terminals, expected, "{orientation}");
    }
}

/// An arc reaches its ends exactly where the source's rays leave the circle,
/// with nothing rounded to whole units on the way.
#[test]
fn an_imported_arc_is_the_sources_own_curve_and_ends_on_its_rays() {
    use crate::symbol::SymbolShape;

    const LOOP_ASY: &str = "Version 4\nSymbolType CELL\n\
        ARC Normal 0 16 32 48 4 44 20 16\n\
        CIRCLE Normal 6 60 26 76\n\
        SYMATTR Value L\nSYMATTR Prefix L\n\
        PIN 16 0 NONE 0\nPINATTR PinName A\nPINATTR SpiceOrder 1\n\
        PIN 16 96 NONE 0\nPINATTR PinName B\nPINATTR SpiceOrder 2\n";
    let library = library_of("ind", LOOP_ASY);
    let drawing =
        parse_asc("Version 4\nSYMBOL ind 0 0 R0\nSYMATTR InstName L1\nSYMATTR Value 1u\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let inductor = &imported.document.components[0];
    assert_eq!(inductor.kind, ComponentType::Inductor);
    let body = &inductor
        .source_symbol()
        .expect("the source's symbol")
        .document()
        .body;
    let [SymbolShape::Curve(arc), SymbolShape::Curve(circle)] = &body[..] else {
        panic!("two curves: {body:?}");
    };
    assert_eq!(arc.center, [10.0, 20.0]);
    assert_eq!(arc.radii, [10.0, 10.0]);
    let on_ray = |point: [f64; 2], (x, y): (f64, f64)| {
        let (dx, dy) = (point[0] - arc.center[0], point[1] - arc.center[1]);
        let (rx, ry) = (x - arc.center[0], y - arc.center[1]);
        (dx * ry - dy * rx).abs() < 1e-6 && dx * rx + dy * ry > 0.0
    };
    // LTspice runs counter-clockwise from the first ray to the second, and a
    // curve runs clockwise, so the first ray is where the curve ends.
    assert!(on_ray(arc.end_point(), (2.5, 27.5)));
    assert!(on_ray(arc.start_point(), (12.5, 10.0)));
    assert!((arc.sweep_degrees - 210.963_756_532).abs() < 1e-6);
    // A source circle whose box is not on whole units stays exactly there.
    assert_eq!(circle.center, [10.0, 42.5]);
    assert_eq!(circle.radii, [6.25, 5.0]);
    assert!(circle.is_whole());
}

/// The swap to RSpice's symbol changes what is drawn and nothing else, in
/// every orientation the drawing turned the part.
#[cfg(feature = "symbol-artwork")]
#[test]
fn a_part_swapped_to_rspices_symbol_keeps_its_pins_lettering_and_identity() {
    let library = library_of("res", BOXED_RESISTOR_ASY);
    for orientation in ORIENTATIONS {
        let drawing = parse_asc(&format!(
            "Version 4\nSYMBOL res 160 160 {orientation}\nSYMATTR InstName R1\nSYMATTR Value 1k\n"
        ))
        .unwrap();
        let imported = import_foreign_schematic(&drawing, "work", &library);
        let carried = &imported.document.components[0];
        let swapped = with_rspice_symbol(carried).expect("a glyph to swap to");
        assert_ne!(
            swapped.source_symbol().map(|symbol| symbol.document()),
            carried.source_symbol().map(|symbol| symbol.document()),
            "{orientation}: the artwork is dropped"
        );
        assert_eq!(swapped.terminal_positions(), carried.terminal_positions());
        assert_eq!(swapped.pos, carried.pos, "{orientation}");
        assert_eq!(swapped.kind, carried.kind, "{orientation}");
        assert_eq!(
            (swapped.name.as_str(), swapped.value.as_str()),
            (carried.name.as_str(), carried.value.as_str()),
            "{orientation}"
        );
        assert_eq!(
            swapped.name_label_pos, carried.name_label_pos,
            "{orientation}"
        );
        assert_eq!(
            swapped.value_label_pos, carried.value_label_pos,
            "{orientation}"
        );
        assert!(
            with_rspice_symbol(&swapped).is_none(),
            "{orientation}: already swapped"
        );
    }
}

/// With no symbol to read, a native part draws RSpice's glyph, scaled so its
/// terminals land on the pins the drawing puts them on.
#[cfg(feature = "symbol-artwork")]
#[test]
fn a_native_part_with_no_readable_symbol_draws_rspices_glyph_at_the_pin_span() {
    let drawing =
        parse_asc("Version 4\nSYMBOL res 0 0 R0\nSYMATTR InstName R1\nSYMATTR Value 1k\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let resistor = &imported.document.components[0];
    assert!(resistor.source_symbol().is_some(), "a scaled glyph");
    let terminals = resistor
        .terminal_positions()
        .into_iter()
        .map(|(_, point)| (point.x, point.y))
        .collect::<Vec<_>>();
    assert_eq!(terminals, [(10, 10), (10, 60)]);
}

/// A switch keeps its own behaviour and draws the source's symbol too.
#[test]
fn a_functional_part_draws_its_sources_own_symbol() {
    const SWITCH_ASY: &str = "Version 4\nSymbolType CELL\nCIRCLE Normal -32 -32 32 32\n\
        PIN 0 -48 NONE 0\nPINATTR PinName 1\nPINATTR SpiceOrder 1\n\
        PIN 0 48 NONE 0\nPINATTR PinName 2\nPINATTR SpiceOrder 2\n\
        PIN -48 -16 NONE 0\nPINATTR PinName 3\nPINATTR SpiceOrder 3\n\
        PIN -48 16 NONE 0\nPINATTR PinName 4\nPINATTR SpiceOrder 4\n";
    let library = library_of("sw", SWITCH_ASY);
    let drawing =
        parse_asc("Version 4\nSYMBOL sw 160 160 R0\nSYMATTR InstName S1\nSYMATTR Value MYSW\n")
            .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let switch = &imported.document.components[0];
    assert_eq!(switch.kind, ComponentType::VSwitch);
    assert!(switch.source_symbol().is_some(), "the source's symbol");
    let terminals = switch
        .terminal_positions()
        .into_iter()
        .map(|(_, point)| (point.x, point.y))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        terminals,
        BTreeSet::from([(100, 70), (100, 130), (70, 90), (70, 110)])
    );
}

/// A pin the source leaves open but ties to ground itself keeps the tie and
/// draws the small open square, not a ground symbol the drawing does not have.
#[test]
fn a_pin_the_source_ties_by_itself_draws_an_open_square_and_keeps_its_net() {
    use crate::schematic::ground_glyph::GroundGlyph;

    let drawing =
        parse_asc("Version 4\nSYMBOL Digital\\and 64 64 R0\nSYMATTR InstName A7\n").unwrap();
    let library = library();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_same_connectivity(&drawing, &imported, &library);
    let grounds = imported
        .document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::Ground)
        .collect::<Vec<_>>();
    assert!(!grounds.is_empty(), "the pins are tied");
    for ground in grounds {
        assert_eq!(ground.ground_glyph, GroundGlyph::OpenPin, "{}", ground.name);
    }
}

/// QSPICE's default net on a pin nothing reaches is the same: the net is kept
/// and the pin draws open, lettering no name.
#[test]
fn a_qspice_default_net_draws_an_open_pin_and_keeps_its_net() {
    use crate::schematic::label_layout::net_label_display_name;
    use crate::schematic::net_label::NetLabelDrawing;

    const DRAWING: &str = "«schematic
  «component (0,0) 0 0
    «symbol AND2Q
      «type: ¥»
      «text (0,300) 1 7 0 0x1000000 -1 -1 \"¥1\"»
      «text (0,-200) 0.5 0 2 0x1000000 -1 -1 \"AND\"»
      «pin (0,400) (0,0) 1 0 0 0x1000000 -1 \"Vdd\" \"Vdd\"»
      «pin (0,-400) (0,0) 1 0 0 0x1000000 -1 \"Vss\" \"GND\"»
      «pin (400,0) (0,0) 1 0 0 0x1000000 -1 \"Q\"»
      «pin (-400,200) (0,0) 1 0 0 0x1000000 -1 \"b0\"»
      «pin (-400,-200) (0,0) 1 0 0 0x1000000 -1 \"b1\"»
    »
  »
  «net (400,0) 1 13 0 \"y\"»
  «net (-400,200) 1 13 0 \"a\"»
  «net (-400,-200) 1 13 0 \"b\"»
»
";
    let imported = import_foreign_schematic(
        &parse_qsch(&qsch_bytes(DRAWING)).unwrap(),
        "work",
        &Library::default(),
    );
    let label = imported
        .document
        .net_labels
        .iter()
        .find(|label| label.name == "Vdd")
        .expect("the net is kept");
    assert_eq!(label.drawing, NetLabelDrawing::OpenPin);
    assert_eq!(net_label_display_name(label), "");
}

/// A value window letters the `Value` attribute: with none, the drawing shows
/// nothing there, while the model stays on the part for the run.
#[test]
fn a_value_window_with_no_value_letters_nothing_and_keeps_the_model() {
    use crate::schematic::component_display::ComponentDisplayMode;

    const MODELLED_ASY: &str = "Version 4\nSymbolType CELL\nRECTANGLE Normal -16 -16 16 16\n\
        WINDOW 0 0 -32 Center 2\nWINDOW 3 0 32 Center 2\n\
        SYMATTR Prefix X\nSYMATTR SpiceModel amp.sub\n\
        PIN -16 0 NONE 0\nPINATTR PinName A\nPINATTR SpiceOrder 1\n";
    let mut library = library();
    library.symbols.insert(
        "modelled".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(MODELLED_ASY).expect("the test symbol reads"),
            location: "modelled.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc("Version 4\nSYMBOL modelled 64 64 R0\nSYMATTR InstName X1\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let part = &imported.document.components[0];
    assert_eq!(part.display_mode, ComponentDisplayMode::NameOnly);
    let binding = part.library_cell.as_ref().expect("a cell");
    assert_eq!(binding.source_path, Some(PathBuf::from("lib/amp.sub")));
}

/// An imported net flag letters its name at the wire's end and nothing else:
/// no attachment diamond, no rail glyph.
#[test]
fn an_imported_net_flag_letters_its_name_at_the_wire_end_with_no_diamond() {
    use crate::schematic::label_layout::{
        IMPORTED_TEXT_WORLD, net_label_font_world, net_label_layout, supply_glyph_marks,
    };
    use crate::schematic::net_label::NetLabelDrawing;

    let drawing = parse_asc("Version 4\nWIRE 0 0 160 0\nFLAG 160 0 OUT\nFLAG 0 0 VCC\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &Library::default());
    for label in &imported.document.net_labels {
        assert_eq!(label.drawing, NetLabelDrawing::Lettered, "{}", label.name);
        assert_eq!(net_label_layout(label).diamond_half, 0.0, "{}", label.name);
        assert!(supply_glyph_marks(label).is_empty(), "{}", label.name);
        assert_eq!(net_label_font_world(label), f32::from(IMPORTED_TEXT_WORLD));
    }
}

/// Names, values and pin names are lettered at the size their source gives
/// them, scaled like the geometry: size 2 is the ordinary fifteen units here,
/// and size 1 two thirds of it.
#[test]
fn imported_text_keeps_the_proportions_of_its_source() {
    use crate::schematic::label_layout::{
        IMPORTED_TEXT_WORLD, INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD,
    };

    let library = library_of("res", BOXED_RESISTOR_ASY);
    let drawing =
        parse_asc("Version 4\nSYMBOL res 0 0 R0\nSYMATTR InstName R1\nSYMATTR Value 1k\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let resistor = &imported.document.components[0];
    let ordinary = f32::from(IMPORTED_TEXT_WORLD);
    let name = resistor.name_label_pos.size_world(INSTANCE_NAME_WORLD);
    let value = resistor.value_label_pos.size_world(INSTANCE_VALUE_WORLD);
    assert!((name - ordinary).abs() < 0.1, "name at {name}");
    assert!(
        (value - ordinary * 2.0 / 3.0).abs() < 0.2,
        "value at {value}"
    );

    let library = self::library();
    let drawing = parse_asc("Version 4\nSYMBOL amp 64 64 R0\nSYMATTR InstName U1\n").unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    assert_eq!(
        imported.cells[0].symbol.pin_name_size,
        Some(IMPORTED_TEXT_WORLD)
    );
}

#[test]
fn a_library_symbols_value2_names_its_model_while_its_value_stays_drawn() {
    use crate::schematic::value_label::value_pieces;

    // LTspice 26.1.1 netlists `Value2` for each of the 2345 cells in its
    // examples whose `SpiceModel` is a file, whatever `Value` says.
    const PART: &str = "Version 4\nSymbolType CELL\nRECTANGLE Normal -32 -32 32 32\n\
        WINDOW 3 0 48 Center 2\n\
        SYMATTR Prefix X\nSYMATTR Value shown-3\nSYMATTR SpiceModel amp.sub\n\
        SYMATTR Value2 real-3 top=1K\n\
        PIN -32 0 NONE 0\nPINATTR PinName IN\nPINATTR SpiceOrder 1\n\
        PIN 32 0 NONE 0\nPINATTR PinName OUT\nPINATTR SpiceOrder 2\n";
    let mut library = library();
    library.symbols.insert(
        "shown".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(PART).expect("the test symbol reads"),
            location: "shown.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc(
        "Version 4\nSHEET 1 880 680\nFLAG 64 96 in\nFLAG 128 96 out\n\
         SYMBOL shown 96 96 R0\nSYMATTR InstName U1\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let part = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "U1")
        .expect("U1");
    assert_eq!(part.value, "real-3");
    assert_eq!(part.params, "top=1K");
    let drawn = value_pieces(part).expect("the drawing's value text stands");
    assert_eq!(drawn.len(), 1);
    assert_eq!(drawn[0].text, "shown-3");
}

#[test]
fn an_op_amp_with_both_supply_pins_open_drives_through_its_own_output_resistance() {
    // LTspice 26.1.1 batch: with both rails open a universal op-amp behaves as
    // a 100 MOhm output with no swing limit; a multiple-feedback stage drawn
    // that way gives V(EQ) 0.1139 at 2 kHz, and these settings give 0.1139065.
    let mut library = Library::default();
    library.symbols.insert(
        "opamps\\universalopamp2".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(UNIVERSAL_OPAMP_ASY).expect("the test symbol reads"),
            location: "opamps/UniversalOpAmp2.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc(
        "Version 4\nSHEET 1 1600 400\n\
         FLAG -32 16 in\nFLAG -32 -16 out\nFLAG 32 0 out\n\
         SYMBOL Opamps\\UniversalOpAmp2 0 0 R0\nSYMATTR InstName U1\n\
         FLAG 288 16 in\nFLAG 288 -16 out2\nFLAG 320 -32 vcc\nFLAG 352 0 out2\n\
         SYMBOL Opamps\\UniversalOpAmp2 320 0 R0\nSYMATTR InstName U2\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let settings = |name: &str| {
        imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .expect("the part")
            .params
            .clone()
    };
    let open = settings("U1");
    assert!(open.contains("ROUT=100Meg"), "{open}");
    assert!(open.contains("HEADROOM=-1k"), "{open}");
    assert!(!open.contains("HEADROOM=0"), "{open}");
    let grounds = imported
        .document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::Ground)
        .count();
    assert_eq!(grounds, 2, "the two open supply pins are tied");
    // One supply pin wired is not the open-rail case.
    let driven = settings("U2");
    assert!(!driven.contains("ROUT"), "{driven}");
}

#[test]
fn an_instance_line_multiplier_on_a_library_cell_becomes_its_multiplicity() {
    const PART: &str = "Version 4\nSymbolType CELL\nRECTANGLE Normal -32 -32 32 32\n\
        SYMATTR Prefix X\nSYMATTR Value shown\nSYMATTR SpiceModel amp.sub\n\
        SYMATTR Value2 real top=1K\nSYMATTR SpiceLine m=1.5\n\
        PIN -32 0 NONE 0\nPINATTR PinName IN\nPINATTR SpiceOrder 1\n\
        PIN 32 0 NONE 0\nPINATTR PinName OUT\nPINATTR SpiceOrder 2\n";
    let mut library = library();
    library.symbols.insert(
        "shown".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(PART).expect("the test symbol reads"),
            location: "shown.asy".to_owned(),
            schematic: None,
        },
    );
    let drawing = parse_asc(
        "Version 4\nSHEET 1 880 680\nFLAG 64 96 in\nFLAG 128 96 out\n\
         SYMBOL shown 96 96 R0\nSYMATTR InstName U1\n",
    )
    .unwrap();
    let imported = import_foreign_schematic(&drawing, "work", &library);
    let part = imported
        .document
        .components
        .iter()
        .find(|component| component.name == "U1")
        .expect("U1");
    assert_eq!(part.params, "top=1K");
    assert_eq!(part.multiplicity.map(|m| m.value()), Some(1.5));
}
