//! The imported schematic joins exactly what the source drawing joins, puts
//! every point on RSpice's grid, and keeps what it cannot place in view.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

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
    // `Left` starts the text 3 past (40, 200) and centres its first 15-unit
    // line on it, the second running below; the anchor stands 6 left of and
    // 3 over the text.
    assert_eq!(note.pos, Point::new(37, 189));
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
    assert_on_grid(&imported);

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
        (170.0, 100.0, TextEdge::Center, TextEdge::End)
    );
    assert_eq!(
        edges(resistor.value),
        (170.0, 140.0, TextEdge::Center, TextEdge::Start)
    );
    // Mirrored, a run that grew rightward grows leftward from (512, 208).
    let amplifier = layout("U1");
    assert_eq!(
        edges(amplifier.value),
        (640.0, 260.0, TextEdge::End, TextEdge::Center)
    );
    assert_eq!(
        component("C1").display_mode,
        ComponentDisplayMode::ValueOnly
    );
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
            ("Ron=1", -6.0, 4.5),
            ("Roff=1Meg", 3.0, 4.5),
            ("Vt=1", 12.0, 4.5),
            ("Vh=-0.5", 21.0, 4.5),
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
    for (index, (text, _, _, a)) in boxes.iter().enumerate() {
        assert!(apart(a, &name), "{text} runs into the name");
        for (other, _, _, b) in &boxes[index + 1..] {
            assert!(apart(a, b), "{text} runs into {other}");
        }
    }

    switch.value = "Ron=2 Roff=1Meg Vt=1 Vh=-0.5".to_owned();
    assert!(
        value_pieces(&switch).is_none(),
        "an edited value letters as one text"
    );
}

/// A part RSpice draws with its own artwork has a body of another size than
/// the source symbol's. A name or value the drawing stood just beside the
/// source body keeps its side and its line but moves out along the way it
/// reads until it clears RSpice's body.
#[test]
fn a_name_or_value_beside_a_source_body_clears_the_body_rspice_draws() {
    use crate::schematic::label_layout::{
        INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD, LayoutPoint, TextEdge, instance_label_layout,
        text_box,
    };

    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL npn 0 0 R0\nWINDOW 0 52 32 Left 2\nWINDOW 3 52 72 Left 2\n\
         SYMATTR InstName Q1\nSYMATTR Value BC547\n\
         SYMBOL npn 320 0 M0\nWINDOW 0 52 32 Left 2\nWINDOW 3 52 72 Left 2\n\
         SYMATTR InstName Q2\nSYMATTR Value BC557\n\
         SYMBOL cap 640 0 R0\nWINDOW 0 20 12 Left 2\nWINDOW 3 20 56 Left 2\n\
         SYMATTR InstName C1\nSYMATTR Value 10n\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    for (name, value, source_y) in [
        ("Q1", "BC547", (40.0, 90.0)),
        ("Q2", "BC557", (40.0, 90.0)),
        ("C1", "10n", (15.0, 70.0)),
    ] {
        let part = imported
            .document
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("{name} is placed"));
        let (width, height) = part.symbol_dimensions();
        let (width, height) = if part.rotation.is_vertical() {
            (height as f32, width as f32)
        } else {
            (width as f32, height as f32)
        };
        let (x, y) = (part.pos.x as f32, part.pos.y as f32);
        let layout = instance_label_layout(LayoutPoint::new(x, y), 1.0, part);
        let reads = if name == "Q2" {
            TextEdge::End
        } else {
            TextEdge::Start
        };
        for (placed, text, size, line) in [
            (layout.name, name, INSTANCE_NAME_WORLD, source_y.0),
            (layout.value, value, INSTANCE_VALUE_WORLD, source_y.1),
        ] {
            assert_eq!(
                (placed.horizontal, placed.vertical, placed.anchor.y),
                (reads, TextEdge::Center, line),
                "{name}: {text:?} keeps its side and its line"
            );
            let text_box = text_box(placed, text, size);
            let clear = text_box.min.x >= x + width * 0.5
                || text_box.max.x <= x - width * 0.5
                || text_box.min.y >= y + height * 0.5
                || text_box.max.y <= y - height * 0.5;
            assert!(
                clear,
                "{name}: {text:?} at {text_box:?} stands on the {width}x{height} body at ({x}, {y})"
            );
        }
    }
}

/// A name or value moved off the body RSpice draws goes to the nearest
/// place where it lands on no body and no other text: a part's name and
/// value both centred beside a turned body do not move onto each other.
#[test]
fn a_text_moved_off_a_body_lands_on_no_other_text() {
    use crate::schematic::label_layout::{
        INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD, LayoutPoint, LayoutRect, instance_label_layout,
        text_box,
    };

    let drawing = parse_asc(
        "Version 4\n\
         SYMBOL njf 0 0 R90\nWINDOW 0 32 60 VCenter 2\nWINDOW 3 60 48 VCenter 2\n\
         SYMATTR InstName J1\nSYMATTR Value 2N4117\n",
    )
    .expect("the drawing reads");
    let imported = import_foreign_schematic(&drawing, "work", &library());
    let mut boxes = Vec::new();
    for part in &imported.document.components {
        let (width, height) = part.symbol_dimensions();
        let (width, height) = if part.rotation.is_vertical() {
            (height as f32, width as f32)
        } else {
            (width as f32, height as f32)
        };
        let (x, y) = (part.pos.x as f32, part.pos.y as f32);
        boxes.push((
            format!("{} body", part.name),
            LayoutRect {
                min: LayoutPoint::new(x - width * 0.5, y - height * 0.5),
                max: LayoutPoint::new(x + width * 0.5, y + height * 0.5),
            },
        ));
        let layout = instance_label_layout(LayoutPoint::new(x, y), 1.0, part);
        boxes.push((
            part.name.clone(),
            text_box(layout.name, &part.name, INSTANCE_NAME_WORLD),
        ));
        boxes.push((
            format!("{} value", part.name),
            text_box(layout.value, &part.value, INSTANCE_VALUE_WORLD),
        ));
    }
    for (index, (a, first)) in boxes.iter().enumerate() {
        for (b, second) in &boxes[index + 1..] {
            let apart = first.max.x <= second.min.x
                || second.max.x <= first.min.x
                || first.max.y <= second.min.y
                || second.max.y <= first.min.y;
            assert!(apart, "{a} at {first:?} lands on {b} at {second:?}");
        }
    }
}

/// An LTspice flag names its net away from the one side its wire or pin
/// lead reaches it from: below a pin whose lead comes down to it, above one
/// whose lead runs up from it, beyond either end of a wire. A flag on the
/// middle of a wire keeps the usual side.
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
            ("FOOT", NetLabelSide::Below),
            ("MIDDLE", NetLabelSide::Right),
            ("TOP", NetLabelSide::Above),
            ("WEST", NetLabelSide::Left),
        ])
    );
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
    // The flags `a` and `b` sit on the jumper's pins, (0, 0) and (0, 120) on
    // RSpice's grid; wire runs the whole way between them.
    let covered: i32 = imported
        .document
        .wires
        .iter()
        .flat_map(|wire| wire.points.windows(2).map(|ends| (ends[0], ends[1])))
        .filter(|(start, end)| start.x == 0 && end.x == 0)
        .map(|(start, end)| (end.y - start.y).abs())
        .sum();
    assert_eq!(covered, 120, "the jumper joins its two nets");
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
    assert_on_grid(&imported);
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
    assert_on_grid(&imported);
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
    assert_on_grid(&imported);
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
    assert_on_grid(&imported);
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
    assert_on_grid(&imported);
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
    assert_on_grid(&imported);
}

/// LTspice's UniversalOpAmp2 at level2 is RSpice's Op-Amp with its settings
/// on the sheet; another level, or a setting the Op-Amp has no field for,
/// keeps LTspice's part with a notice.
#[test]
fn an_ltspice_universal_opamp_at_level2_is_the_functional_op_amp() {
    const OPAMP_ASY: &str = "Version 4\nSymbolType CELL\nLINE Normal -32 -32 32 0\n\
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
    let mut library = Library::default();
    library.symbols.insert(
        "opamps\\universalopamp2".to_owned(),
        ForeignSymbolSource {
            definition: parse_asy(OPAMP_ASY).expect("the test symbol reads"),
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
    assert_on_grid(&imported);
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
    assert_on_grid(&imported);
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
    assert_eq!(input.name_offset, Some(10));
    let terminal = resolved.effective_pin_offset(input);
    assert_eq!(
        resolved.pin_label_anchor(input),
        Point::new(terminal.x + 10, terminal.y)
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
        LINE_BOX_EM, NET_LABEL_FONT_WORLD, NET_LABEL_GLYPH_WIDTH_WORLD, net_label_name_corner,
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
        let width = name.len() as f32 * NET_LABEL_GLYPH_WIDTH_WORLD;
        let top = corner.y - NET_LABEL_FONT_WORLD * LINE_BOX_EM;
        let (x, y) = (label.pos.x as f32, label.pos.y as f32);
        match side {
            NetLabelSide::Below => {
                assert!(top > y, "{name} hangs under its point");
                assert!(
                    (corner.x + width * 0.5 - x).abs() < 0.01,
                    "{name} is centred"
                );
            }
            NetLabelSide::Above => {
                assert!(corner.y < y, "{name} stands over its point");
                assert!(
                    (corner.x + width * 0.5 - x).abs() < 0.01,
                    "{name} is centred"
                );
            }
            NetLabelSide::Left => {
                assert!(corner.x + width < x, "{name} ends short of its point");
                assert!(
                    corner.y < y,
                    "{name} stands over the line through its point"
                );
            }
            NetLabelSide::Right => {
                assert!(corner.x > x, "{name} starts past its point");
                assert!(
                    corner.y < y,
                    "{name} stands over the line through its point"
                );
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
    use crate::schematic::design_note::{
        DESIGN_NOTE_TEXT_OFFSET_WORLD, DesignNote, DesignNoteKind,
    };

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
        let (width, height) = note.text_extent_world(&note.text);
        let left = note.pos.x as f32 + DESIGN_NOTE_TEXT_OFFSET_WORLD.0;
        let top = note.pos.y as f32 + DESIGN_NOTE_TEXT_OFFSET_WORLD.1;
        (
            note.text_size_percent,
            [left, top, left + width, top + height],
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
    // A box that stands on its point by an edge stands this far off it, so
    // its descenders clear the wire it labels.
    let standoff = 3.0;

    let (size, [left, top, right, bottom]) = lettered(&qspice, "centred\nnote");
    let (x, y) = point(&qspice, "centre");
    assert_eq!(size, 80);
    assert!(near((left + right) * 0.5, x) && near((top + bottom) * 0.5, y));

    let (size, [left, _, right, bottom]) = lettered(&qspice, "standing");
    let (x, y) = point(&qspice, "base");
    assert_eq!(size, 100);
    assert!(near((left + right) * 0.5, x) && near(bottom, y - standoff));

    let (size, [left, top, right, _]) = lettered(&ltspice, "hanging");
    let (x, y) = point(&ltspice, "top");
    assert_eq!(size, 67, "LTspice size 1 is 1.0x against the standard 1.5x");
    assert!(near((left + right) * 0.5, x) && near(top, y + standoff));

    let (size, [left, top, _, bottom]) = lettered(&ltspice, "starting");
    let (x, y) = point(&ltspice, "left");
    assert_eq!(size, 100);
    assert!(near(left, x + standoff) && near((top + bottom) * 0.5, y));

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
fn a_comment_draws_no_anchor_mark_where_its_drawing_drew_none() {
    let drawing =
        parse_asc("Version 4\nTEXT 32 160 Left 2 ;Bias network\nDATAFLAG 96 80 \"$\"\n").unwrap();
    let mut imported = import_foreign_schematic(&drawing, "work", &Library::default());
    keep_directive_text(
        &mut imported.document,
        Point::new(0, 0),
        ".options gmin=1p",
        ForeignLettering::default(),
    )
    .unwrap();
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
         WIRE 480 0 480 64\nFLAG 480 64 VEE\n",
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
    use crate::schematic::design_note::DESIGN_NOTE_TEXT_OFFSET_WORLD;

    let (width, height) = note.text_extent_world(&note.text);
    let left = note.pos.x as f32 + DESIGN_NOTE_TEXT_OFFSET_WORLD.0;
    let top = note.pos.y as f32 + DESIGN_NOTE_TEXT_OFFSET_WORLD.1;
    (
        [left, top, left + width, top + height],
        note.text_extent_world("").1,
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
    let standoff = 3.0;

    let ([left, top, _, _], line) = note_box(note("first"));
    let (x, y) = label_point(&imported, "left");
    assert!(
        near(left, x + standoff) && near(top + line * 0.5, y),
        "the first line stands centred on the point: {left},{top} against {x},{y}"
    );

    let ([_, _, _, bottom], _) = note_box(note("upper"));
    let (_, y) = label_point(&imported, "base");
    assert!(near(bottom, y - standoff), "{bottom} against {y}");
}

/// Directive text kept on the sheet is lettered as its block was: at the
/// block's size, its first line centred on the point the drawing stood the
/// block on.
#[test]
fn directive_text_kept_on_the_sheet_is_lettered_as_its_block_was() {
    let drawing =
        parse_asc("Version 4\nFLAG 640 0 base\nTEXT 640 0 Left 3 !.backanno\\n.options gmin=1p\n")
            .unwrap();
    let mut imported = import_foreign_schematic(&drawing, "work", &Library::default());
    let card = foreign_directive_cards(&imported.directives)
        .into_iter()
        .find(|card| card.keyword() == ".options")
        .expect("the block holds an .options card");
    keep_directive_text(
        &mut imported.document,
        card.position,
        &card.text,
        card.lettering,
    )
    .unwrap();
    let note = imported
        .document
        .design_notes
        .iter()
        .find(|note| note.text == ".options gmin=1p")
        .expect("the card is kept as a note");
    assert_eq!(
        note.text_size_percent, 133,
        "LTspice size 3 is 2.0x against 1.5x"
    );
    let near = |a: f32, b: f32| (a - b).abs() <= 0.5;
    let ([left, top, _, _], line) = note_box(note);
    let (x, y) = label_point(&imported, "base");
    assert!(
        near(left, x + 3.0) && near(top + line * 0.5, y),
        "{left},{top} against {x},{y}"
    );
}
