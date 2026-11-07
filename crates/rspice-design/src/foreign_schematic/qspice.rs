//! QSPICE schematics (`.qsch`).
//!
//! A `.qsch` file is a tree of `«keyword fields … children »` blocks after a
//! four-byte file mark. Every component carries its own symbol: its pins in
//! instance-line order, its drawing, and the texts that make up its instance
//! line. QSPICE draws with y pointing up; this reader turns the drawing over
//! so it reads like every other drawing here.

use super::{
    ForeignAttribute, ForeignOrientation, ForeignParseError, ForeignPin, ForeignPoint,
    ForeignSchematic, ForeignShape, ForeignSymbol, ForeignSymbolDefinition, ForeignText,
    ForeignTextKind, ForeignTool, ForeignWindow, decode_foreign_text,
};

/// The bytes QSPICE writes before the schematic tree.
const FILE_MARK: [u8; 4] = [0xFF, 0xD8, 0xFF, 0xDB];
const OPEN: u8 = 0xAB;
const CLOSE: u8 = 0xBB;

/// A note QSPICE writes on a symbol (a vendor's name, a pin legend), never
/// on the instance line.
const COMMENT_TEXT_FLAG: &str = "1";
/// A text QSPICE draws on a symbol without putting it on the instance line,
/// unless it is the symbol's first text, its instance name.
const GRAPHIC_TEXT_FLAG: &str = "2";
/// The attribute holding a symbol's first drawn text after its name: a
/// QSPICE behavioural device's type (`AND`, `MultGmAmp`).
pub(super) const DEVICE_TYPE: &str = "DeviceType";

/// Reads a `.qsch` schematic from its bytes.
pub fn parse_qsch(bytes: &[u8]) -> Result<ForeignSchematic, ForeignParseError> {
    let root = parse_tree(bytes.strip_prefix(&FILE_MARK).unwrap_or(bytes))?;
    if !root.head.trim().eq_ignore_ascii_case("schematic") {
        return Err(ForeignParseError::new(
            root.line,
            "not a QSPICE schematic: the outer block is not «schematic»",
        ));
    }
    let mut schematic = ForeignSchematic::new(ForeignTool::Qspice);
    for node in &root.children {
        let (keyword, fields) = node.keyword();
        match keyword {
            "component" => schematic.symbols.push(component(node)?),
            "wire" => {
                let points = points(fields, node.line)?;
                let [a, b] = first_two(&points, node.line)?;
                schematic.wires.push((a, b));
                schematic
                    .wire_nets
                    .push(quoted(fields).into_iter().next().unwrap_or_default());
            }
            "net" => {
                let points = points(fields, node.line)?;
                let name = quoted(fields)
                    .into_iter()
                    .next()
                    .ok_or_else(|| ForeignParseError::new(node.line, "net label names no net"))?;
                schematic.flags.push(super::ForeignFlag {
                    position: points.first().copied().ok_or_else(|| {
                        ForeignParseError::new(node.line, "net label has no point")
                    })?,
                    name: if name.eq_ignore_ascii_case("GND") {
                        "0".to_owned()
                    } else {
                        name
                    },
                    port: None,
                });
            }
            "junction" => {
                let points = points(fields, node.line)?;
                schematic.junctions.extend(points.first().copied());
            }
            "text" => {
                let points = points(fields, node.line)?;
                let numbers = numbers_after_points(fields);
                let kind = match numbers.get(2).map(String::as_str) {
                    Some("0") => ForeignTextKind::Directive,
                    _ => ForeignTextKind::Comment,
                };
                schematic.texts.push(ForeignText {
                    position: points.first().copied().unwrap_or_default(),
                    kind,
                    text: quoted(fields).join("\n"),
                    line: node.line,
                });
            }
            "line" | "rect" | "ellipse" | "arc" | "arc3p" | "zigzag" | "triangle" => {
                schematic.shapes.extend(shape(keyword, fields, node.line)?);
            }
            // Page furniture, bitmaps and plot settings carry no circuit.
            _ => {}
        }
    }
    Ok(schematic)
}

/// One `«…»` block: the text before its first child, and its children.
#[derive(Debug, Default)]
struct Node {
    head: String,
    children: Vec<Node>,
    line: usize,
}

impl Node {
    fn keyword(&self) -> (&str, &str) {
        let head = self.head.trim();
        match head.find(|c: char| c.is_whitespace() || c == ':') {
            Some(end) => (&head[..end], head[end..].trim_start_matches(':').trim()),
            None => (head, ""),
        }
    }

    fn child(&self, keyword: &str) -> Option<&Node> {
        self.children
            .iter()
            .find(|child| child.keyword().0 == keyword)
    }
}

fn parse_tree(bytes: &[u8]) -> Result<Node, ForeignParseError> {
    // QSPICE writes its structure in single bytes (« is 0xAB, » is 0xBB) and
    // one record per line, while a text inside a record may be UTF-8 whose
    // bytes include 0xBB. So a line opens a block when it starts with «,
    // holds a whole record when it also ends with », and closes a block when
    // it is a lone ».
    let mut stack: Vec<Node> = Vec::new();
    let mut root = None;
    for (index, raw) in bytes.split(|byte| *byte == b'\n').enumerate() {
        let line = index + 1;
        let record = raw.trim_ascii();
        if record.is_empty() {
            continue;
        }
        if record == [CLOSE] {
            let node = stack
                .pop()
                .ok_or_else(|| ForeignParseError::new(line, "» closes no block"))?;
            match stack.last_mut() {
                Some(parent) => parent.children.push(node),
                None if root.is_none() => root = Some(node),
                None => return Err(ForeignParseError::new(line, "a second top-level block")),
            }
            continue;
        }
        let Some(content) = record.strip_prefix(&[OPEN]) else {
            return Err(ForeignParseError::new(line, "a line outside any «» record"));
        };
        match content.strip_suffix(&[CLOSE]) {
            Some(leaf) => {
                let parent = stack
                    .last_mut()
                    .ok_or_else(|| ForeignParseError::new(line, "a record outside any block"))?;
                parent.children.push(Node {
                    head: decode_record(leaf),
                    children: Vec::new(),
                    line,
                });
            }
            None => stack.push(Node {
                head: decode_record(content),
                children: Vec::new(),
                line,
            }),
        }
    }
    if let Some(open) = stack.last() {
        return Err(ForeignParseError::new(open.line, "a block is never closed"));
    }
    root.ok_or_else(|| ForeignParseError::new(0, "not a QSPICE schematic: no «» block"))
}

/// A record's text: UTF-8 when it is valid UTF-8, the Windows code page
/// otherwise (QSPICE's device letters such as `Ø` and `¥` are single bytes).
fn decode_record(bytes: &[u8]) -> String {
    decode_foreign_text(bytes)
}

fn component(node: &Node) -> Result<ForeignSymbol, ForeignParseError> {
    let (_, fields) = node.keyword();
    let position = points(fields, node.line)?
        .first()
        .copied()
        .ok_or_else(|| ForeignParseError::new(node.line, "component has no position"))?;
    let numbers = numbers_after_points(fields);
    let code = numbers
        .first()
        .and_then(|number| number.parse::<u8>().ok())
        .unwrap_or(0);
    let orientation = orientation(code).ok_or_else(|| {
        ForeignParseError::new(
            node.line,
            "the component is turned by 45 degrees, which a schematic grid cannot hold",
        )
    })?;
    let symbol_node = node
        .child("symbol")
        .ok_or_else(|| ForeignParseError::new(node.line, "component has no symbol"))?;
    let (_, symbol_name) = symbol_node.keyword();
    let mut definition = ForeignSymbolDefinition::default();
    let mut texts = Vec::new();
    let mut device_type = None;
    for child in &symbol_node.children {
        let (keyword, fields) = child.keyword();
        match keyword {
            "type" => definition.symbol_type = fields.to_owned(),
            "description" | "library" | "shorted" => {
                let (name, value) = child
                    .head
                    .trim()
                    .split_once(':')
                    .unwrap_or((child.head.trim(), ""));
                definition.attributes.push(ForeignAttribute {
                    name: name.trim().to_owned(),
                    value: value.trim().to_owned(),
                });
            }
            "pin" => {
                let pin_points = points(fields, child.line)?;
                // A second string names the net the pin joins when nothing
                // is wired to it; `¥` marks a pin that may stay open.
                let mut strings = quoted(fields).into_iter();
                let name = strings.next().unwrap_or_default();
                let default_net = strings
                    .next()
                    .map(|net| net.trim().to_owned())
                    .filter(|net| !net.is_empty() && net != "¥");
                definition.pins.push(ForeignPin {
                    name,
                    spice_order: definition.pins.len() + 1,
                    default_net,
                    position: pin_points
                        .first()
                        .copied()
                        .ok_or_else(|| ForeignParseError::new(child.line, "pin has no point"))?,
                });
            }
            "text" => {
                let text_points = points(fields, child.line)?;
                let flags = numbers_after_points(fields);
                match flags.get(2).map(String::as_str) {
                    Some(COMMENT_TEXT_FLAG) => continue,
                    Some(GRAPHIC_TEXT_FLAG) if !texts.is_empty() => {
                        // A behavioural device draws its type this way.
                        if device_type.is_none() {
                            device_type = Some(quoted(fields).join(" "));
                        }
                        continue;
                    }
                    _ => {}
                }
                texts.push((
                    text_points.first().copied().unwrap_or_default(),
                    quoted(fields).join(" "),
                ));
            }
            _ => definition
                .shapes
                .extend(shape(keyword, fields, child.line)?),
        }
    }
    let mut attributes = Vec::new();
    if let Some(device_type) = device_type {
        attributes.push(ForeignAttribute {
            name: DEVICE_TYPE.to_owned(),
            value: device_type,
        });
    }
    let mut windows = Vec::new();
    let mut texts = texts.into_iter();
    if let Some((offset, name)) = texts.next() {
        attributes.push(ForeignAttribute {
            name: "InstName".to_owned(),
            value: name,
        });
        windows.push(ForeignWindow {
            attribute: 0,
            offset,
            alignment: "Left".to_owned(),
        });
    }
    let rest = texts.collect::<Vec<_>>();
    if let Some((offset, _)) = rest.first() {
        windows.push(ForeignWindow {
            attribute: 3,
            offset: *offset,
            alignment: "Left".to_owned(),
        });
        attributes.push(ForeignAttribute {
            name: "Value".to_owned(),
            value: rest
                .iter()
                .map(|(_, text)| text.as_str())
                .collect::<Vec<_>>()
                .join(" "),
        });
    }
    Ok(ForeignSymbol {
        symbol: symbol_name.to_owned(),
        position,
        orientation,
        attributes,
        windows,
        embedded: Some(definition),
        line: node.line,
    })
}

/// QSPICE's orientation code: the low three bits turn the symbol
/// counter-clockwise in eighths of a turn, and bit 3 mirrors it left to right
/// before it turns. With the drawing turned over, a counter-clockwise turn by
/// `k` quarters is LTspice's clockwise turn by `4 - k`, and a mirrored one is
/// LTspice's `M` by `k`.
fn orientation(code: u8) -> Option<ForeignOrientation> {
    let eighths = code & 7;
    if !eighths.is_multiple_of(2) {
        return None;
    }
    let quarters = eighths / 2;
    Some(if code & 8 != 0 {
        ForeignOrientation::new(quarters, true)
    } else {
        ForeignOrientation::new((4 - quarters) % 4, false)
    })
}

fn shape(keyword: &str, fields: &str, line: usize) -> Result<Vec<ForeignShape>, ForeignParseError> {
    let points = points(fields, line)?;
    Ok(match (keyword, points.as_slice()) {
        ("line", [a, b, ..]) => vec![ForeignShape::Line(*a, *b)],
        ("rect" | "zigzag", [a, b, ..]) => vec![ForeignShape::Rectangle(*a, *b)],
        ("ellipse", [a, b, ..]) => vec![ForeignShape::Ellipse(*a, *b)],
        ("triangle", [a, b, c, ..]) => vec![
            ForeignShape::Line(*a, *b),
            ForeignShape::Line(*b, *c),
            ForeignShape::Line(*c, *a),
        ],
        // Turned over, a counter-clockwise sweep runs the other way.
        ("arc" | "arc3p", [a, b, c, d, ..]) => vec![ForeignShape::Arc([*a, *b, *d, *c])],
        _ => Vec::new(),
    })
}

/// Every `(x,y)` in a record, turned over so y points down.
fn points(fields: &str, line: usize) -> Result<Vec<ForeignPoint>, ForeignParseError> {
    let mut points = Vec::new();
    let mut rest = unquoted(fields);
    while let Some(open) = rest.find('(') {
        let close = rest[open..]
            .find(')')
            .map(|offset| open + offset)
            .ok_or_else(|| ForeignParseError::new(line, "a point is never closed"))?;
        let (x, y) = rest[open + 1..close]
            .split_once(',')
            .ok_or_else(|| ForeignParseError::new(line, "a point needs two coordinates"))?;
        let parse = |text: &str| {
            text.trim()
                .parse::<i32>()
                .ok()
                .filter(|value| value.abs() <= 100_000_000)
                .ok_or_else(|| {
                    ForeignParseError::new(line, format!("`{text}` is not a coordinate"))
                })
        };
        points.push(ForeignPoint::new(parse(x)?, -parse(y)?));
        rest = &rest[close + 1..];
    }
    Ok(points)
}

fn first_two(points: &[ForeignPoint], line: usize) -> Result<[ForeignPoint; 2], ForeignParseError> {
    match points {
        [a, b, ..] => Ok([*a, *b]),
        _ => Err(ForeignParseError::new(line, "a wire needs two points")),
    }
}

/// The bare words after the last point of a record, before its strings.
fn numbers_after_points(fields: &str) -> Vec<String> {
    let text = unquoted(fields);
    let start = text.rfind(')').map_or(0, |index| index + 1);
    text[start..]
        .split_ascii_whitespace()
        .map(str::to_owned)
        .collect()
}

/// The record with its quoted strings cut off.
fn unquoted(fields: &str) -> &str {
    fields.find('"').map_or(fields, |index| &fields[..index])
}

/// The quoted strings of a record, with `\n` as a line break. QSPICE writes a
/// string's own quotes unescaped, so the strings run from the record's first
/// quote to its last, and only a quote, spaces and a quote divide two.
fn quoted(fields: &str) -> Vec<String> {
    let (Some(first), Some(last)) = (fields.find('"'), fields.rfind('"')) else {
        return Vec::new();
    };
    if last <= first {
        return Vec::new();
    }
    let mut strings = Vec::new();
    let mut rest = &fields[first + 1..last];
    loop {
        let divider = rest.char_indices().find_map(|(index, character)| {
            if character != '"' {
                return None;
            }
            let after = &rest[index + 1..];
            let spaces = after.len() - after.trim_start().len();
            (spaces > 0 && after[spaces..].starts_with('"'))
                .then_some((index, index + 1 + spaces + 1))
        });
        match divider {
            Some((end, next)) => {
                strings.push(clean_text(&rest[..end]));
                rest = &rest[next..];
            }
            None => {
                strings.push(clean_text(rest));
                return strings;
            }
        }
    }
}

/// A string's text: `\n` is a line break, and the byte-order mark QSPICE
/// sometimes writes at the start is dropped.
fn clean_text(text: &str) -> String {
    text.trim_start_matches('\u{FEFF}')
        .trim_start_matches("\u{EF}\u{BB}\u{BF}")
        .replace("\\n", "\n")
}
