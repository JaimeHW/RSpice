//! Importing a symbol definition.
//!
//! The formats a definition can arrive in, and the parse into the internal
//! model.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolImportFormat {
    RSpiceJson,
    Svg,
    Edif,
    LtspiceAsy,
}

impl SymbolImportFormat {
    pub const fn label(self) -> &'static str {
        match self {
            Self::RSpiceJson => "RSpice symbol JSON",
            Self::Svg => "SVG",
            Self::Edif => "EDIF 2 0 0",
            Self::LtspiceAsy => "LTspice ASY",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolImportReport {
    pub format: SymbolImportFormat,
    pub primitive_count: usize,
    pub explicit_pin_anchor_count: usize,
    pub pin_order_valid: bool,
    pub binding_valid: bool,
    pub warnings: Vec<String>,
}

impl SymbolImportReport {
    pub fn format_label(&self) -> &'static str {
        self.format.label()
    }
}

#[derive(Debug, Clone)]
pub struct SymbolDefinitionImport {
    pub definition: ModelBoundSymbolDefinition,
    pub report: SymbolImportReport,
}

impl SymbolDefinitionImport {
    /// Import a canonical definition or bounded graphic source. SVG/EDIF
    /// callers must supply an explicit definition; geometry never supplies
    /// or changes electrical semantics.
    pub fn from_bytes(
        bytes: &[u8],
        source_name: &str,
        explicit_contract: Option<ModelBoundSymbolDefinition>,
    ) -> Result<Self, SymbolDefinitionError> {
        let format = sniff_import_format(bytes, source_name)?;
        match format {
            SymbolImportFormat::RSpiceJson => {
                if explicit_contract.is_some() {
                    return Err(SymbolDefinitionError::Import(
                        "an explicit contract cannot override canonical symbol JSON".to_owned(),
                    ));
                }
                let definition = ModelBoundSymbolDefinition::from_json_bytes(bytes, source_name)?;
                let report = report_for_definition(format, &definition, Vec::new());
                Ok(Self { definition, report })
            }
            SymbolImportFormat::Svg | SymbolImportFormat::Edif | SymbolImportFormat::LtspiceAsy => {
                if bytes.len() > MAX_IMPORTED_GRAPHIC_BYTES {
                    return Err(SymbolDefinitionError::Import(format!(
                        "{source_name}: graphic source exceeds the {MAX_IMPORTED_GRAPHIC_BYTES}-byte limit"
                    )));
                }
                let source = std::str::from_utf8(bytes).map_err(|_| {
                    SymbolDefinitionError::Import(format!("{source_name}: source is not UTF-8"))
                })?;
                let parsed = parse_graphic_source(format, source_name, source)?;
                let primitive_count = parsed.shapes.len();
                let mut definition = explicit_contract.ok_or_else(|| {
                    SymbolDefinitionError::Import(format!(
                        "{source_name}: {} geometry has no electrical semantics; choose a model/pin contract or Blank explicit contract",
                        format.label()
                    ))
                })?;
                definition.imported_graphic = Some(ImportedGraphicSource {
                    format: match format {
                        SymbolImportFormat::Svg => ImportedGraphicFormat::Svg,
                        SymbolImportFormat::Edif => ImportedGraphicFormat::Edif,
                        SymbolImportFormat::LtspiceAsy => ImportedGraphicFormat::LtspiceAsy,
                        SymbolImportFormat::RSpiceJson => unreachable!(),
                    },
                    source_name: source_name.to_owned(),
                    source: source.to_owned(),
                    primitive_count,
                    shapes: parsed.shapes,
                    pin_anchors: parsed.pin_anchors,
                    attributes: parsed.attributes,
                });
                validate_import_pin_anchors(&definition)?;
                definition.validate()?;
                let warnings = definition
                    .source
                    .is_explicitly_unbound_for_review()
                    .then(|| "graphic is explicitly unbound and remains review-only".to_owned())
                    .into_iter()
                    .collect();

                let report = report_for_definition(format, &definition, warnings);
                Ok(Self { definition, report })
            }
        }
    }
}

pub(super) fn validate_imported_graphic(
    imported: &ImportedGraphicSource,
) -> Result<(), SymbolDefinitionError> {
    if imported.source.len() > MAX_IMPORTED_GRAPHIC_BYTES {
        return Err(SymbolDefinitionError::Import(
            "retained graphic source exceeds the supported limit".to_owned(),
        ));
    }
    let format = match imported.format {
        ImportedGraphicFormat::Svg => SymbolImportFormat::Svg,
        ImportedGraphicFormat::Edif => SymbolImportFormat::Edif,
        ImportedGraphicFormat::LtspiceAsy => SymbolImportFormat::LtspiceAsy,
    };
    let observed = parse_graphic_source(format, &imported.source_name, &imported.source)?;
    if observed.shapes != imported.shapes
        || observed.pin_anchors != imported.pin_anchors
        || observed.attributes != imported.attributes
        || observed.shapes.len() != imported.primitive_count
    {
        return Err(SymbolDefinitionError::Import(
            "retained typed graphic does not match its source".to_owned(),
        ));
    }
    Ok(())
}

fn sniff_import_format(
    bytes: &[u8],
    source_name: &str,
) -> Result<SymbolImportFormat, SymbolDefinitionError> {
    let prefix = std::str::from_utf8(bytes)
        .map_err(|_| SymbolDefinitionError::Import(format!("{source_name}: source is not UTF-8")))?
        .trim_start();
    let extension = source_name
        .rsplit_once('.')
        .map(|(_, extension)| extension.to_ascii_lowercase());
    let sniffed = if prefix.starts_with('{') {
        SymbolImportFormat::RSpiceJson
    } else if prefix.starts_with("<svg") || prefix.starts_with("<?xml") && prefix.contains("<svg") {
        SymbolImportFormat::Svg
    } else if prefix.to_ascii_lowercase().starts_with("(edif") {
        SymbolImportFormat::Edif
    } else if extension.as_deref() == Some("asy")
        && prefix.lines().any(|line| line.starts_with("SymbolType "))
    {
        SymbolImportFormat::LtspiceAsy
    } else {
        return Err(SymbolDefinitionError::Import(format!(
            "{source_name}: unsupported symbol source; expected JSON, SVG, or EDIF 2 0 0"
        )));
    };
    if let Some(extension) = extension {
        let extension_format = match extension.as_str() {
            "json" | "rspicesym" | "rspicesymbol" => Some(SymbolImportFormat::RSpiceJson),
            "svg" => Some(SymbolImportFormat::Svg),
            "edif" | "edf" => Some(SymbolImportFormat::Edif),
            "asy" => Some(SymbolImportFormat::LtspiceAsy),
            _ => None,
        };
        if let Some(extension_format) = extension_format
            && extension_format != sniffed
        {
            return Err(SymbolDefinitionError::Import(format!(
                "{source_name}: file extension does not match detected {} content",
                sniffed.label()
            )));
        }
    }
    Ok(sniffed)
}

fn parse_graphic_source(
    format: SymbolImportFormat,
    source_name: &str,
    source: &str,
) -> Result<ParsedImportedGraphic, SymbolDefinitionError> {
    match format {
        SymbolImportFormat::Svg => {
            let lowered = source.to_ascii_lowercase();
            if !lowered.contains("<svg") || !lowered.contains("</svg>") {
                return Err(SymbolDefinitionError::Import(format!(
                    "{source_name}: SVG root is incomplete"
                )));
            }
            for forbidden in [
                "<script",
                "javascript:",
                "<foreignobject",
                "xlink:href",
                " href=",
            ] {
                if lowered.contains(forbidden) {
                    return Err(SymbolDefinitionError::Import(format!(
                        "{source_name}: SVG contains forbidden active or external content `{forbidden}`"
                    )));
                }
            }
            parse_svg_shapes(source_name, source)
        }
        SymbolImportFormat::Edif => {
            let lowered = source.to_ascii_lowercase();
            if !lowered.trim_start().starts_with("(edif") {
                return Err(SymbolDefinitionError::Import(format!(
                    "{source_name}: EDIF root is missing"
                )));
            }
            if !lowered.contains("(edifversion 2 0 0)") {
                return Err(SymbolDefinitionError::Import(format!(
                    "{source_name}: only EDIF 2 0 0 is supported"
                )));
            }
            let opens = source.matches('(').count();
            let closes = source.matches(')').count();
            if opens != closes {
                return Err(SymbolDefinitionError::Import(format!(
                    "{source_name}: EDIF parentheses are unbalanced"
                )));
            }
            parse_edif_shapes(source_name, source)
        }
        SymbolImportFormat::LtspiceAsy => parse_ltspice_asy(source_name, source),
        SymbolImportFormat::RSpiceJson => unreachable!("JSON is validated by serde"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedImportedGraphic {
    shapes: Vec<SymbolShape>,
    pin_anchors: Vec<ImportedPinAnchor>,
    attributes: BTreeMap<String, String>,
}

fn parse_svg_shapes(
    source_name: &str,
    source: &str,
) -> Result<ParsedImportedGraphic, SymbolDefinitionError> {
    let lowered = source.to_ascii_lowercase();
    for unsupported in ["<path", "<ellipse", "<text", "<use", "<image"] {
        if lowered.contains(unsupported) {
            return Err(SymbolDefinitionError::Import(format!(
                "{source_name}: unsupported SVG primitive `{unsupported}`"
            )));
        }
    }
    let mut shapes = Vec::new();
    for tag in svg_tags(source, "line") {
        shapes.push(SymbolShape::Polyline {
            points: vec![
                Point::new(
                    svg_i32(source_name, tag, "x1")?,
                    svg_i32(source_name, tag, "y1")?,
                ),
                Point::new(
                    svg_i32(source_name, tag, "x2")?,
                    svg_i32(source_name, tag, "y2")?,
                ),
            ],
            closed: false,
        });
    }
    for (name, closed) in [("polyline", false), ("polygon", true)] {
        for tag in svg_tags(source, name) {
            let encoded = svg_attr(tag, "points").ok_or_else(|| {
                SymbolDefinitionError::Import(format!("{source_name}: {name} has no points"))
            })?;
            let values = encoded
                .split(|character: char| character == ',' || character.is_ascii_whitespace())
                .filter(|value| !value.is_empty())
                .map(|value| parse_coord(source_name, value))
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() < 4 || values.len() % 2 != 0 {
                return Err(SymbolDefinitionError::Import(format!(
                    "{source_name}: {name} point list is malformed"
                )));
            }
            shapes.push(SymbolShape::Polyline {
                points: values
                    .chunks_exact(2)
                    .map(|point| Point::new(point[0], point[1]))
                    .collect(),
                closed,
            });
        }
    }
    for tag in svg_tags(source, "rect") {
        let x = svg_i32(source_name, tag, "x")?;
        let y = svg_i32(source_name, tag, "y")?;
        let width = svg_i32(source_name, tag, "width")?;
        let height = svg_i32(source_name, tag, "height")?;
        if width <= 0 || height <= 0 {
            return Err(SymbolDefinitionError::Import(format!(
                "{source_name}: rectangle dimensions must be positive"
            )));
        }
        shapes.push(SymbolShape::Polyline {
            points: vec![
                Point::new(x, y),
                Point::new(x + width, y),
                Point::new(x + width, y + height),
                Point::new(x, y + height),
            ],
            closed: true,
        });
    }
    for tag in svg_tags(source, "circle") {
        let radius = svg_i32(source_name, tag, "r")?;
        if radius <= 0 {
            return Err(SymbolDefinitionError::Import(format!(
                "{source_name}: circle radius must be positive"
            )));
        }
        shapes.push(SymbolShape::Circle {
            center: Point::new(
                svg_i32(source_name, tag, "cx")?,
                svg_i32(source_name, tag, "cy")?,
            ),
            radius,
        });
    }
    if shapes.is_empty() {
        return Err(SymbolDefinitionError::Import(format!(
            "{source_name}: SVG contains no supported primitives"
        )));
    }
    Ok(ParsedImportedGraphic {
        shapes,
        pin_anchors: Vec::new(),
        attributes: BTreeMap::new(),
    })
}

fn svg_tags<'a>(source: &'a str, name: &str) -> Vec<&'a str> {
    let lowered = source.to_ascii_lowercase();
    let needle = format!("<{name}");
    let mut tags = Vec::new();
    let mut cursor = 0;
    while let Some(relative) = lowered[cursor..].find(&needle) {
        let start = cursor + relative;
        let Some(end) = lowered[start..].find('>') else {
            break;
        };
        tags.push(&source[start..=start + end]);
        cursor = start + end + 1;
    }
    tags
}

fn svg_attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let bytes = tag.as_bytes();
    let needle = name.as_bytes();
    let mut index = 0;
    while index + needle.len() < bytes.len() {
        if bytes[index..].starts_with(needle)
            && (index == 0 || bytes[index - 1].is_ascii_whitespace())
        {
            let mut cursor = index + needle.len();
            while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
                cursor += 1;
            }
            if bytes.get(cursor) != Some(&b'=') {
                index += 1;
                continue;
            }
            cursor += 1;
            while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
                cursor += 1;
            }
            let quote = *bytes.get(cursor)?;
            if quote != b'\'' && quote != b'"' {
                return None;
            }
            let start = cursor + 1;
            let end = bytes[start..].iter().position(|byte| *byte == quote)? + start;
            return tag.get(start..end);
        }
        index += 1;
    }
    None
}

fn svg_i32(source_name: &str, tag: &str, name: &str) -> Result<i32, SymbolDefinitionError> {
    parse_coord(
        source_name,
        svg_attr(tag, name).ok_or_else(|| {
            SymbolDefinitionError::Import(format!(
                "{source_name}: SVG attribute `{name}` is required"
            ))
        })?,
    )
}

fn parse_coord(source_name: &str, value: &str) -> Result<i32, SymbolDefinitionError> {
    let value = value.parse::<f64>().map_err(|_| {
        SymbolDefinitionError::Import(format!("{source_name}: invalid coordinate `{value}`"))
    })?;
    if !value.is_finite() || value.abs() > 1_000_000.0 {
        return Err(SymbolDefinitionError::Import(format!(
            "{source_name}: coordinate is non-finite or out of bounds"
        )));
    }

    Ok(value.round() as i32)
}

fn parse_edif_shapes(
    source_name: &str,
    source: &str,
) -> Result<ParsedImportedGraphic, SymbolDefinitionError> {
    let lowered = source.to_ascii_lowercase();
    for unsupported in ["(arc", "(curve", "(text"] {
        if lowered.contains(unsupported) {
            return Err(SymbolDefinitionError::Import(format!(
                "{source_name}: unsupported EDIF primitive `{unsupported}`"
            )));
        }
    }
    let mut shapes = Vec::new();
    for keyword in ["rectangle", "path", "polygon", "circle"] {
        for form in edif_forms(source, keyword)? {
            let points = edif_points(source_name, form)?;
            let shape = match keyword {
                "rectangle" if points.len() == 2 => SymbolShape::Polyline {
                    points: vec![
                        points[0],
                        Point::new(points[1].x, points[0].y),
                        points[1],
                        Point::new(points[0].x, points[1].y),
                    ],
                    closed: true,
                },
                "path" if points.len() >= 2 => SymbolShape::Polyline {
                    points,
                    closed: false,
                },
                "polygon" if points.len() >= 3 => SymbolShape::Polyline {
                    points,
                    closed: true,
                },
                "circle" if points.len() == 2 => {
                    let dx = f64::from(points[1].x - points[0].x);
                    let dy = f64::from(points[1].y - points[0].y);
                    SymbolShape::Circle {
                        center: points[0],
                        radius: (dx.hypot(dy).round() as i32).max(1),
                    }
                }
                _ => {
                    return Err(SymbolDefinitionError::Import(format!(
                        "{source_name}: malformed EDIF {keyword} primitive"
                    )));
                }
            };
            shapes.push(shape);
        }
    }
    if shapes.is_empty() {
        return Err(SymbolDefinitionError::Import(format!(
            "{source_name}: EDIF contains no supported primitives"
        )));
    }
    Ok(ParsedImportedGraphic {
        shapes,
        pin_anchors: Vec::new(),
        attributes: BTreeMap::new(),
    })
}

fn edif_forms<'a>(source: &'a str, keyword: &str) -> Result<Vec<&'a str>, SymbolDefinitionError> {
    let lowered = source.to_ascii_lowercase();
    let needle = format!("({keyword}");
    let mut forms = Vec::new();
    let mut cursor = 0;
    while let Some(relative) = lowered[cursor..].find(&needle) {
        let start = cursor + relative;
        let mut depth = 0i32;
        let mut end = None;
        for (offset, byte) in source.as_bytes()[start..].iter().enumerate() {
            match byte {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(start + offset + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        let end = end.ok_or_else(|| {
            SymbolDefinitionError::Import("unterminated EDIF primitive".to_owned())
        })?;
        forms.push(&source[start..end]);
        cursor = end;
    }
    Ok(forms)
}

fn edif_points(source_name: &str, form: &str) -> Result<Vec<Point>, SymbolDefinitionError> {
    let lowered = form.to_ascii_lowercase();
    let mut points = Vec::new();

    let mut cursor = 0;
    while let Some(relative) = lowered[cursor..].find("(pt") {
        let start = cursor + relative + 3;
        let end = lowered[start..].find(')').ok_or_else(|| {
            SymbolDefinitionError::Import(format!("{source_name}: malformed EDIF point"))
        })? + start;
        let numbers = form[start..end]
            .split_ascii_whitespace()
            .collect::<Vec<_>>();
        if numbers.len() != 2 {
            return Err(SymbolDefinitionError::Import(format!(
                "{source_name}: malformed EDIF point"
            )));
        }
        points.push(Point::new(
            parse_coord(source_name, numbers[0])?,
            parse_coord(source_name, numbers[1])?,
        ));
        cursor = end + 1;
    }
    Ok(points)
}

fn parse_ltspice_asy(
    source_name: &str,
    source: &str,
) -> Result<ParsedImportedGraphic, SymbolDefinitionError> {
    let mut shapes = Vec::new();
    let mut anchors = Vec::new();
    let mut attributes = BTreeMap::new();
    let mut pending: Option<(Point, Option<String>, Option<usize>)> = None;
    let mut version_seen = false;
    let mut symbol_type_seen = false;
    let flush_pin = |pending: &mut Option<(Point, Option<String>, Option<usize>)>,
                     anchors: &mut Vec<ImportedPinAnchor>|
     -> Result<(), SymbolDefinitionError> {
        if let Some((position, name, order)) = pending.take() {
            anchors.push(ImportedPinAnchor {
                position,
                name: name.ok_or_else(|| {
                    SymbolDefinitionError::Import(format!(
                        "{source_name}: PIN is missing PINATTR PinName"
                    ))
                })?,
                spice_order: order.ok_or_else(|| {
                    SymbolDefinitionError::Import(format!(
                        "{source_name}: PIN is missing PINATTR SpiceOrder"
                    ))
                })?,
            });
        }
        Ok(())
    };
    for (line_number, raw) in source.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        match words[0] {
            "Version" if words == ["Version", "4"] => {
                if version_seen {
                    return Err(SymbolDefinitionError::Import(format!(
                        "{source_name}: duplicate Version"
                    )));
                }
                version_seen = true;
            }
            "SymbolType" if words.len() == 2 && matches!(words[1], "CELL" | "BLOCK") => {
                if symbol_type_seen {
                    return Err(SymbolDefinitionError::Import(format!(
                        "{source_name}: duplicate SymbolType"
                    )));
                }
                symbol_type_seen = true;
            }
            "WINDOW" => {}
            "SYMATTR" if words.len() >= 3 => {
                if attributes
                    .insert(words[1].to_owned(), words[2..].join(" "))
                    .is_some()
                {
                    return Err(SymbolDefinitionError::Import(format!(
                        "{source_name}:{} duplicate SYMATTR {}",
                        line_number + 1,
                        words[1]
                    )));
                }
            }
            "LINE" if words.len() == 6 => shapes.push(SymbolShape::Polyline {
                points: vec![
                    Point::new(
                        parse_asy_i32(source_name, line_number, words[2])?,
                        parse_asy_i32(source_name, line_number, words[3])?,
                    ),
                    Point::new(
                        parse_asy_i32(source_name, line_number, words[4])?,
                        parse_asy_i32(source_name, line_number, words[5])?,
                    ),
                ],

                closed: false,
            }),
            "RECTANGLE" | "CIRCLE" if words.len() == 6 => {
                let a = Point::new(
                    parse_asy_i32(source_name, line_number, words[2])?,
                    parse_asy_i32(source_name, line_number, words[3])?,
                );
                let b = Point::new(
                    parse_asy_i32(source_name, line_number, words[4])?,
                    parse_asy_i32(source_name, line_number, words[5])?,
                );
                let width = b.x - a.x;
                let height = b.y - a.y;
                if width <= 0 || height <= 0 {
                    return Err(SymbolDefinitionError::Import(format!(
                        "{source_name}:{} primitive bounds must have positive width and height",
                        line_number + 1
                    )));
                }
                if words[0] == "RECTANGLE" {
                    shapes.push(SymbolShape::Polyline {
                        points: vec![a, Point::new(b.x, a.y), b, Point::new(a.x, b.y)],
                        closed: true,
                    });
                } else {
                    if width != height {
                        return Err(SymbolDefinitionError::Import(format!(
                            "{source_name}:{} elliptical CIRCLE cannot be represented losslessly",
                            line_number + 1
                        )));
                    }
                    shapes.push(SymbolShape::Circle {
                        center: Point::new((a.x + b.x) / 2, (a.y + b.y) / 2),
                        radius: ((b.x - a.x).abs().min((b.y - a.y).abs()) / 2).max(1),
                    });
                }
            }
            "ARC" if words.len() == 10 => {
                let a = Point::new(
                    parse_asy_i32(source_name, line_number, words[2])?,
                    parse_asy_i32(source_name, line_number, words[3])?,
                );
                let b = Point::new(
                    parse_asy_i32(source_name, line_number, words[4])?,
                    parse_asy_i32(source_name, line_number, words[5])?,
                );
                let start = Point::new(
                    parse_asy_i32(source_name, line_number, words[6])?,
                    parse_asy_i32(source_name, line_number, words[7])?,
                );
                let end = Point::new(
                    parse_asy_i32(source_name, line_number, words[8])?,
                    parse_asy_i32(source_name, line_number, words[9])?,
                );
                let center = Point::new((a.x + b.x) / 2, (a.y + b.y) / 2);
                let width = b.x - a.x;
                let height = b.y - a.y;
                if width <= 0 || height <= 0 || width != height {
                    return Err(SymbolDefinitionError::Import(format!(
                        "{source_name}:{} ARC bounds must be a positive square",
                        line_number + 1
                    )));
                }
                let angle = |point: Point| {
                    ((f64::from(point.y - center.y))
                        .atan2(f64::from(point.x - center.x))
                        .to_degrees()
                        .round() as i32)
                        .rem_euclid(360)
                };
                let start_degrees = angle(start);
                let sweep_degrees = (angle(end) - start_degrees).rem_euclid(360);
                shapes.push(SymbolShape::Arc {
                    center,
                    radius: ((b.x - a.x).abs().min((b.y - a.y).abs()) / 2).max(1),
                    start_degrees,
                    sweep_degrees,
                });
            }
            "PIN" if words.len() >= 3 => {
                flush_pin(&mut pending, &mut anchors)?;
                pending = Some((
                    Point::new(
                        parse_asy_i32(source_name, line_number, words[1])?,
                        parse_asy_i32(source_name, line_number, words[2])?,
                    ),
                    None,
                    None,
                ));
            }
            "PINATTR" if words.len() >= 3 => {
                let pin = pending.as_mut().ok_or_else(|| {
                    SymbolDefinitionError::Import(format!(
                        "{source_name}:{} PINATTR has no PIN",
                        line_number + 1
                    ))
                })?;
                match words[1] {
                    "PinName" if pin.1.is_none() => pin.1 = Some(words[2..].join(" ")),
                    "PinName" => {
                        return Err(SymbolDefinitionError::Import(format!(
                            "{source_name}:{} duplicate PINATTR PinName",
                            line_number + 1
                        )));
                    }
                    "SpiceOrder" => {
                        if pin.2.is_some() {
                            return Err(SymbolDefinitionError::Import(format!(
                                "{source_name}:{} duplicate PINATTR SpiceOrder",
                                line_number + 1
                            )));
                        }
                        pin.2 = Some(words[2].parse().map_err(|_| {
                            SymbolDefinitionError::Import(format!(
                                "{source_name}:{} invalid SpiceOrder",
                                line_number + 1
                            ))
                        })?)
                    }
                    _ => {
                        return Err(SymbolDefinitionError::Import(format!(
                            "{source_name}:{} unsupported PINATTR {}",
                            line_number + 1,
                            words[1]
                        )));
                    }
                }
            }
            _ => {
                return Err(SymbolDefinitionError::Import(format!(
                    "{source_name}:{} unsupported or malformed ASY statement `{}`",
                    line_number + 1,
                    words[0]
                )));
            }
        }
    }
    flush_pin(&mut pending, &mut anchors)?;
    if !version_seen || !symbol_type_seen {
        return Err(SymbolDefinitionError::Import(format!(
            "{source_name}: ASY requires `Version 4` and a supported SymbolType"
        )));
    }
    if shapes.is_empty() {
        return Err(SymbolDefinitionError::Import(format!(
            "{source_name}: ASY contains no supported geometry"
        )));
    }
    anchors.sort_by_key(|anchor| anchor.spice_order);
    Ok(ParsedImportedGraphic {
        shapes,
        pin_anchors: anchors,
        attributes,
    })
}

fn parse_asy_i32(
    source_name: &str,
    line: usize,
    value: &str,
) -> Result<i32, SymbolDefinitionError> {
    let value = value.parse::<i32>().map_err(|_| {
        SymbolDefinitionError::Import(format!(
            "{source_name}:{} invalid ASY coordinate `{value}`",
            line + 1
        ))
    })?;
    if value.abs() > 1_000_000 {
        return Err(SymbolDefinitionError::Import(format!(
            "{source_name}:{} ASY coordinate is out of bounds",
            line + 1
        )));
    }
    Ok(value)
}

pub(super) fn validate_import_pin_anchors(
    definition: &ModelBoundSymbolDefinition,
) -> Result<(), SymbolDefinitionError> {
    let Some(imported) = &definition.imported_graphic else {
        return Ok(());
    };
    if imported.pin_anchors.is_empty() {
        return Ok(());
    }
    let mut names = HashSet::new();
    for (index, anchor) in imported.pin_anchors.iter().enumerate() {
        if anchor.spice_order != index + 1 || !names.insert(anchor.name.to_ascii_lowercase()) {
            return Err(SymbolDefinitionError::Import(
                "imported pin names/orders must be unique and contiguous".to_owned(),
            ));
        }
    }
    if definition.source.is_explicitly_unbound_for_review() && definition.pins.is_empty() {
        return Ok(());
    }
    let mut pins = definition.pins.iter().collect::<Vec<_>>();
    pins.sort_by_key(|pin| pin.order);
    if pins.len() != imported.pin_anchors.len()
        || pins.iter().zip(&imported.pin_anchors).any(|(pin, anchor)| {
            pin.order != anchor.spice_order || !pin.name.eq_ignore_ascii_case(&anchor.name)
        })
    {
        return Err(SymbolDefinitionError::Import(
            "imported pin anchors do not match the selected explicit pin contract".to_owned(),
        ));
    }
    Ok(())
}

fn report_for_definition(
    format: SymbolImportFormat,

    definition: &ModelBoundSymbolDefinition,
    warnings: Vec<String>,
) -> SymbolImportReport {
    SymbolImportReport {
        format,
        primitive_count: definition.imported_graphic.as_ref().map_or_else(
            || definition.symbol_document().body.len(),
            |source| source.primitive_count,
        ),
        explicit_pin_anchor_count: definition.imported_graphic.as_ref().map_or_else(
            || definition.symbol_document().pins.len(),
            |source| source.pin_anchors.len(),
        ),
        pin_order_valid: validate_import_pin_anchors(definition).is_ok()
            && (definition.source.is_explicitly_unbound_for_review()
                || validate_pins(&definition.pins).is_ok()),
        binding_valid: definition.netlist.is_executable()
            && validate_source(&definition.source, &definition.pins).is_ok()
            && validate_netlist(
                &definition.netlist,
                &definition.source,
                &definition.pins,
                &definition.parameter_form,
            )
            .is_ok(),
        warnings,
    }
}
