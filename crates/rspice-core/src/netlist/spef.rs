//! SPEF (IEEE 1481) parasitic ingestion for post-layout simulation.
//!
//! A SPEF file describes each net's extracted RC network: internal
//! subnodes, grounded and coupling capacitances, resistances, and which
//! instance pin or top-level port sits on which subnode. Ingestion is a
//! *back-annotation* transform on a parsed [`Netlist`]:
//!
//! * every `*I inst:pin` connection rewires that instance's terminal from
//!   the ideal net onto its SPEF subnode (`inst__pin`), so the parasitic
//!   resistance genuinely sits between driver and load;
//! * `*P` port connections keep the original net name, preserving the
//!   deck's external connectivity;
//! * the net's R/C elements are appended as ordinary resistors and
//!   capacitors (coupling caps included), with SPEF units honored.
//!
//! Decks reference a SPEF file with `.spef_include "file.spef"`
//! (Spectre's spelling) — `.include file.spef` routes there too. DSPF
//! files need none of this: DSPF is SPICE syntax and parses directly.
//!
//! Unsupported sections (`*INDUC`, reduced `*R_NET`/`*C_NET`) are skipped
//! with a warning rather than failing the deck.

use std::collections::HashMap;

use super::ast::{Element, ElementKind};
use super::{Netlist, ParseError};
use crate::Value;

/// One parsed SPEF node reference.
#[derive(Debug, Clone, PartialEq)]
enum NodeRef {
    /// Bare net or port name.
    Net(String),
    /// `net:idx` internal subnode.
    SubNode(String, String),
    /// `inst:pin` instance-pin node.
    Pin(String, String),
}

/// One `*CONN` entry of a `*D_NET`.
#[derive(Debug, Clone)]
struct Conn {
    node: NodeRef,
    /// `*P` (port) vs `*I` (instance pin).
    is_port: bool,
}

#[derive(Debug, Clone)]
struct Cap {
    a: NodeRef,
    /// `None` for a grounded capacitance.
    b: Option<NodeRef>,
    farads: Value,
}

#[derive(Debug, Clone)]
struct Res {
    a: NodeRef,
    b: NodeRef,
    ohms: Value,
}

#[derive(Debug, Clone)]
struct DNet {
    name: String,
    conns: Vec<Conn>,
    caps: Vec<Cap>,
    ress: Vec<Res>,
}

/// Parsed SPEF document.
#[derive(Debug, Default)]
pub struct SpefFile {
    nets: Vec<DNet>,
}

/// Summary of one back-annotation pass.
#[derive(Debug, Default, Clone)]
pub struct SpefReport {
    /// `*D_NET` sections applied.
    pub nets: usize,
    /// Instance terminals rewired onto SPEF subnodes.
    pub rewired_pins: usize,
    /// Pin connections that could not be matched to a deck element.
    pub skipped_pins: usize,
    /// Parasitic resistors added.
    pub resistors: usize,
    /// Parasitic capacitors added.
    pub capacitors: usize,
}

impl SpefFile {
    /// Parse SPEF text.
    pub fn parse(content: &str) -> Result<Self, ParseError> {
        Parser::new(content).parse()
    }

    /// Back-annotate `netlist` with this file's parasitics.
    pub fn apply(&self, netlist: &mut Netlist) -> SpefReport {
        let mut report = SpefReport::default();
        let mut element_index: HashMap<String, usize> = HashMap::new();
        for (idx, element) in netlist.elements.iter().enumerate() {
            element_index.insert(element.name.to_ascii_uppercase(), idx);
        }
        let subckt_ports: HashMap<String, Vec<String>> = netlist
            .subcircuits
            .iter()
            .map(|def| {
                (
                    def.name.to_ascii_uppercase(),
                    def.ports.iter().map(|p| p.to_ascii_uppercase()).collect(),
                )
            })
            .collect();

        let mut new_elements: Vec<Element> = Vec::new();
        let mut parasitic_seq = 0_usize;

        for net in &self.nets {
            report.nets += 1;

            // Bare references (ports are referenced by name, which already
            // is a deck node) pass through; subnodes and pins get generated
            // names. Everything is uppercased to match the parser's node
            // normalization, so parasitics land on the deck's actual nodes.
            let node_name = |reference: &NodeRef| -> String {
                match reference {
                    NodeRef::Net(name) => name.to_ascii_uppercase(),
                    NodeRef::SubNode(net, idx) => sanitize(&format!("{net}__{idx}")),
                    NodeRef::Pin(inst, pin) => sanitize(&format!("{inst}__{pin}")),
                }
            };

            // Rewire instance pins onto their SPEF subnodes.
            for conn in net.conns.iter().filter(|conn| !conn.is_port) {
                let NodeRef::Pin(inst, pin) = &conn.node else {
                    continue;
                };
                match rewire_pin(
                    netlist,
                    &element_index,
                    &subckt_ports,
                    inst,
                    pin,
                    &net.name,
                    &node_name(&conn.node),
                ) {
                    Ok(()) => report.rewired_pins += 1,
                    Err(reason) => {
                        report.skipped_pins += 1;
                        log::warn!(
                            "SPEF: pin {inst}:{pin} on net {} skipped: {reason}",
                            net.name
                        );
                    }
                }
            }

            for cap in &net.caps {
                if !(cap.farads.is_finite() && cap.farads > 0.0) {
                    continue;
                }
                let n1 = node_name(&cap.a);
                let n2 = cap
                    .b
                    .as_ref()
                    .map(&node_name)
                    .unwrap_or_else(|| "0".to_owned());
                parasitic_seq += 1;
                new_elements.push(Element {
                    name: format!("CSPEF{parasitic_seq}"),
                    kind: ElementKind::Capacitor {
                        value: cap.farads,
                        value_expr: None,
                        initial_voltage: None,
                        model: None,
                        instance_params: Vec::new(),
                        deferred_params: Vec::new(),
                    },
                    nodes: vec![n1, n2],
                });
                report.capacitors += 1;
            }

            for res in &net.ress {
                if !(res.ohms.is_finite() && res.ohms > 0.0) {
                    continue;
                }
                parasitic_seq += 1;
                new_elements.push(Element {
                    name: format!("RSPEF{parasitic_seq}"),
                    kind: ElementKind::Resistor {
                        value: res.ohms,
                        value_expr: None,
                        model: None,
                        instance_params: Vec::new(),
                        deferred_params: Vec::new(),
                    },
                    nodes: vec![node_name(&res.a), node_name(&res.b)],
                });
                report.resistors += 1;
            }
        }

        netlist.elements.extend(new_elements);
        report
    }
}

/// Move one instance terminal from the ideal net to its SPEF subnode.
fn rewire_pin(
    netlist: &mut Netlist,
    element_index: &HashMap<String, usize>,
    subckt_ports: &HashMap<String, Vec<String>>,
    inst: &str,
    pin: &str,
    net: &str,
    new_node: &str,
) -> Result<(), String> {
    let upper = inst.to_ascii_uppercase();
    let element_idx = element_index
        .get(&upper)
        .or_else(|| element_index.get(&format!("X{upper}")))
        .copied()
        .ok_or_else(|| format!("no element named `{inst}` in the deck"))?;

    let element = &netlist.elements[element_idx];
    let terminal = terminal_index(element, subckt_ports, pin)
        .ok_or_else(|| format!("pin `{pin}` not resolvable on `{}`", element.name))?;

    let element = &mut netlist.elements[element_idx];
    let current = element
        .nodes
        .get(terminal)
        .ok_or_else(|| format!("terminal {terminal} out of range on `{}`", element.name))?;
    if !current.eq_ignore_ascii_case(net) {
        return Err(format!(
            "terminal `{pin}` of `{}` connects to `{current}`, not net `{net}`",
            element.name
        ));
    }
    element.nodes[terminal] = new_node.to_owned();
    Ok(())
}

/// Resolve a SPEF pin name to a terminal index on a deck element.
fn terminal_index(
    element: &Element,
    subckt_ports: &HashMap<String, Vec<String>>,
    pin: &str,
) -> Option<usize> {
    let pin_upper = pin.to_ascii_uppercase();
    match &element.kind {
        ElementKind::Subcircuit { subckt_name, .. } => subckt_ports
            .get(&subckt_name.to_ascii_uppercase())?
            .iter()
            .position(|port| *port == pin_upper),
        ElementKind::Mosfet { .. } => position_in(&["D", "G", "S", "B"], &pin_upper),
        ElementKind::Bjt { .. } => position_in(&["C", "B", "E", "S"], &pin_upper),
        ElementKind::Diode { .. } => {
            position_in(&["A", "K"], &pin_upper).or_else(|| position_in(&["P", "N"], &pin_upper))
        }
        ElementKind::Resistor { .. }
        | ElementKind::Capacitor { .. }
        | ElementKind::Inductor { .. } => position_in(&["1", "2"], &pin_upper)
            .or_else(|| position_in(&["P", "N"], &pin_upper))
            .or_else(|| position_in(&["A", "B"], &pin_upper)),
        _ => None,
    }
    .filter(|idx| *idx < element.nodes.len())
}

fn position_in(table: &[&str], pin: &str) -> Option<usize> {
    table.iter().position(|candidate| *candidate == pin)
}

/// SPICE node names may not carry SPEF's `:`/`/`/`[`/`]` characters; the
/// result is uppercased to match the parser's node normalization.
fn sanitize(raw: &str) -> String {
    raw.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct Parser<'a> {
    lines: std::str::Lines<'a>,
    line_num: usize,
    name_map: HashMap<u64, String>,
    delimiter: char,
    cap_scale: Value,
    res_scale: Value,
}

/// Sections within a `*D_NET`.
#[derive(Clone, Copy, PartialEq)]
enum NetSection {
    None,
    Conn,
    Cap,
    Res,
    Skip,
}

impl<'a> Parser<'a> {
    fn new(content: &'a str) -> Self {
        Self {
            lines: content.lines(),
            line_num: 0,
            name_map: HashMap::new(),
            delimiter: ':',
            cap_scale: 1e-12, // SPEF default exchange unit is PF / OHM
            res_scale: 1.0,
        }
    }

    fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError::Syntax {
            line: self.line_num,
            message: format!("SPEF: {}", message.into()),
        }
    }

    fn parse(mut self) -> Result<SpefFile, ParseError> {
        let mut nets = Vec::new();
        let mut in_name_map = false;
        let mut current: Option<DNet> = None;
        let mut section = NetSection::None;

        while let Some(raw) = self.lines.next() {
            self.line_num += 1;
            let line = strip_comment(raw).trim();
            if line.is_empty() {
                continue;
            }
            let fields: Vec<&str> = line.split_whitespace().collect();
            let keyword = fields[0].to_ascii_uppercase();

            // Name-map entries: `*<index> <name>`.
            if in_name_map && let Some(index) = parse_map_index(fields[0]) {
                if let Some(name) = fields.get(1) {
                    self.name_map.insert(index, (*name).to_owned());
                }
                continue;
            }

            match keyword.as_str() {
                "*SPEF" | "*DESIGN" | "*DATE" | "*VENDOR" | "*PROGRAM" | "*VERSION"
                | "*DESIGN_FLOW" | "*DIVIDER" | "*BUS_DELIMITER" | "*T_UNIT" | "*L_UNIT" => {
                    in_name_map = false;
                }
                "*DELIMITER" => {
                    in_name_map = false;
                    if let Some(d) = fields.get(1).and_then(|f| f.chars().next()) {
                        self.delimiter = d;
                    }
                }
                "*C_UNIT" => {
                    in_name_map = false;
                    self.cap_scale = self.parse_unit(
                        &fields,
                        &[
                            ("FF", 1e-15),
                            ("PF", 1e-12),
                            ("NF", 1e-9),
                            ("UF", 1e-6),
                            ("F", 1.0),
                        ],
                    )?;
                }
                "*R_UNIT" => {
                    in_name_map = false;
                    self.res_scale =
                        self.parse_unit(&fields, &[("OHM", 1.0), ("KOHM", 1e3), ("MOHM", 1e6)])?;
                }
                "*NAME_MAP" => in_name_map = true,
                "*PORTS" | "*PHYSICAL_PORTS" => in_name_map = false,
                "*D_NET" => {
                    in_name_map = false;
                    if let Some(net) = current.take() {
                        nets.push(net);
                    }
                    let name_field = fields
                        .get(1)
                        .ok_or_else(|| self.error("*D_NET without a net name"))?;
                    current = Some(DNet {
                        name: self.resolve_name(name_field),
                        conns: Vec::new(),
                        caps: Vec::new(),
                        ress: Vec::new(),
                    });
                    section = NetSection::None;
                }
                "*R_NET" | "*C_NET" => {
                    in_name_map = false;
                    log::warn!(
                        "SPEF line {}: reduced {keyword} sections are not supported; skipped",
                        self.line_num
                    );
                    if let Some(net) = current.take() {
                        nets.push(net);
                    }
                    section = NetSection::Skip;
                }
                "*CONN" => section = NetSection::Conn,
                "*CAP" => section = NetSection::Cap,
                "*RES" => section = NetSection::Res,
                "*INDUC" => {
                    log::warn!(
                        "SPEF line {}: *INDUC section is not supported; skipped",
                        self.line_num
                    );
                    section = NetSection::Skip;
                }
                "*END" => {
                    if let Some(net) = current.take() {
                        nets.push(net);
                    }
                    section = NetSection::None;
                }
                "*P" | "*I" if section == NetSection::Conn => {
                    let Some(net) = current.as_mut() else {
                        continue;
                    };
                    let node_field = fields
                        .get(1)
                        .ok_or_else(|| self.error("connection entry without a node"))?;
                    let node = self.parse_node_ref(node_field);
                    net.conns.push(Conn {
                        node,
                        is_port: keyword == "*P",
                    });
                }
                "*N" if section == NetSection::Conn => {
                    // Internal-node coordinates: topology only, no action.
                }
                _ if section == NetSection::Cap => {
                    let Some(net) = current.as_mut() else {
                        continue;
                    };
                    // `id node value` (ground) or `id node node value`.
                    match fields.len() {
                        3 => {
                            let farads = self.parse_value(fields[2])? * self.cap_scale;
                            let a = self.parse_node_ref(fields[1]);
                            net.caps.push(Cap { a, b: None, farads });
                        }
                        4 => {
                            let farads = self.parse_value(fields[3])? * self.cap_scale;
                            let a = self.parse_node_ref(fields[1]);
                            let b = self.parse_node_ref(fields[2]);
                            net.caps.push(Cap {
                                a,
                                b: Some(b),
                                farads,
                            });
                        }
                        _ => {
                            return Err(self.error(format!(
                                "malformed *CAP entry `{line}` (expected 3 or 4 fields)"
                            )));
                        }
                    }
                }
                _ if section == NetSection::Res => {
                    let Some(net) = current.as_mut() else {
                        continue;
                    };
                    if fields.len() != 4 {
                        return Err(self
                            .error(format!("malformed *RES entry `{line}` (expected 4 fields)")));
                    }
                    let ohms = self.parse_value(fields[3])? * self.res_scale;
                    let a = self.parse_node_ref(fields[1]);
                    let b = self.parse_node_ref(fields[2]);
                    net.ress.push(Res { a, b, ohms });
                }
                _ => {}
            }
        }

        if let Some(net) = current.take() {
            nets.push(net);
        }
        Ok(SpefFile { nets })
    }

    fn parse_unit(&self, fields: &[&str], table: &[(&str, Value)]) -> Result<Value, ParseError> {
        let multiplier: Value = fields
            .get(1)
            .and_then(|f| f.parse().ok())
            .ok_or_else(|| self.error("unit statement without a multiplier"))?;
        let unit = fields
            .get(2)
            .map(|f| f.to_ascii_uppercase())
            .ok_or_else(|| self.error("unit statement without a unit"))?;
        let scale = table
            .iter()
            .find(|(name, _)| *name == unit)
            .map(|(_, scale)| *scale)
            .ok_or_else(|| self.error(format!("unsupported unit `{unit}`")))?;
        Ok(multiplier * scale)
    }

    fn parse_value(&self, field: &str) -> Result<Value, ParseError> {
        field
            .parse()
            .map_err(|_| self.error(format!("`{field}` is not a number")))
    }

    /// Resolve `*<index>` through the name map; pass names through.
    fn resolve_name(&self, field: &str) -> String {
        match parse_map_index(field) {
            Some(index) => self
                .name_map
                .get(&index)
                .cloned()
                .unwrap_or_else(|| field.to_owned()),
            None => field.to_owned(),
        }
    }

    /// Parse a node reference: `name`, `*3`, `name:4`, `*3:A`, `inst:pin`.
    fn parse_node_ref(&self, field: &str) -> NodeRef {
        match field.split_once(self.delimiter) {
            None => NodeRef::Net(self.resolve_name(field)),
            Some((base, sub)) => {
                let base = self.resolve_name(base);
                // A purely numeric suffix is an internal subnode; anything
                // else is an instance pin.
                if sub.chars().all(|ch| ch.is_ascii_digit()) {
                    NodeRef::SubNode(base, sub.to_owned())
                } else {
                    NodeRef::Pin(base, sub.to_owned())
                }
            }
        }
    }
}

/// `*<digits>` name-map reference.
fn parse_map_index(field: &str) -> Option<u64> {
    field.strip_prefix('*')?.parse().ok()
}

/// SPEF `//` end-of-line comments.
fn strip_comment(line: &str) -> &str {
    match line.find("//") {
        Some(pos) => &line[..pos],
        None => line,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEF: &str = r#"
*SPEF "IEEE 1481-2009"
*DESIGN "demo"
*DIVIDER /
*DELIMITER :
*BUS_DELIMITER [ ]
*T_UNIT 1 NS
*C_UNIT 1 FF
*R_UNIT 1 KOHM
*L_UNIT 1 HENRY

*NAME_MAP
*1 in
*2 XBUF

*PORTS
*1 I

*D_NET *1 2.4
*CONN
*P *1 I
*I *2:A I *C 1.0 2.0
*CAP
1 *1:1 1.5 // grounded
2 *1:2 *2:A 0.9
*RES
1 *1 *1:1 0.5
2 *1:1 *2:A 1.0
*END
"#;

    fn deck() -> Netlist {
        Netlist::parse(
            "spef demo\n\
             .subckt buf A Y\n\
             R1 A Y 100\n\
             .ends\n\
             V1 in 0 DC 1\n\
             XBUF in out buf\n\
             R2 out 0 1k\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses")
    }

    #[test]
    fn parses_units_names_and_sections() {
        let spef = SpefFile::parse(SPEF).expect("spef parses");
        assert_eq!(spef.nets.len(), 1);
        let net = &spef.nets[0];
        assert_eq!(net.name, "in");
        assert_eq!(net.conns.len(), 2);
        assert_eq!(net.caps.len(), 2);
        assert_eq!(net.ress.len(), 2);
        // 1.5 in 1-FF units.
        assert!((net.caps[0].farads - 1.5e-15).abs() < 1e-22);
        // 0.5 in 1-KOHM units.
        assert!((net.ress[0].ohms - 500.0).abs() < 1e-9);
    }

    #[test]
    fn apply_rewires_pins_and_adds_parasitics() {
        let spef = SpefFile::parse(SPEF).expect("spef parses");
        let mut netlist = deck();
        let report = spef.apply(&mut netlist);

        assert_eq!(report.nets, 1);
        assert_eq!(report.rewired_pins, 1);
        assert_eq!(report.skipped_pins, 0);
        assert_eq!(report.resistors, 2);
        assert_eq!(report.capacitors, 2);

        // The XBUF A-terminal moved off the ideal net onto its pin node.
        let xbuf = netlist
            .elements
            .iter()
            .find(|e| e.name.eq_ignore_ascii_case("XBUF"))
            .expect("XBUF present");
        assert_eq!(xbuf.nodes[0], "XBUF__A");

        // The parasitic chain reaches from the port node (deck names
        // preserved) through the subnode to the rewired pin.
        let res_nodes: Vec<&[String]> = netlist
            .elements
            .iter()
            .filter(|e| e.name.starts_with("RSPEF"))
            .map(|e| e.nodes.as_slice())
            .collect();
        assert!(res_nodes.iter().any(|n| n[0] == "IN" && n[1] == "IN__1"));
        assert!(
            res_nodes
                .iter()
                .any(|n| n[0] == "IN__1" && n[1] == "XBUF__A")
        );
    }

    #[test]
    fn mismatched_pin_net_is_skipped_not_rewired() {
        let bad = SPEF.replace("*1 in", "*1 othernet");
        let spef = SpefFile::parse(&bad).expect("spef parses");
        let mut netlist = deck();
        let report = spef.apply(&mut netlist);
        // XBUF:A sits on net1 in the deck, not `othernet` — skip + warn.
        assert_eq!(report.rewired_pins, 0);
        assert_eq!(report.skipped_pins, 1);
    }
}
