//! LTspice schematics (`.asc`) and symbols (`.asy`).
//!
//! Both are line records: a keyword, then space-separated fields. A text
//! record keeps everything after its fixed fields, and writes its line breaks
//! as the two characters `\n`.

use super::{
    ForeignAttribute, ForeignDataFlag, ForeignFlag, ForeignLettering, ForeignOrientation,
    ForeignParseError, ForeignPin, ForeignPoint, ForeignPortDirection, ForeignSchematic,
    ForeignShape, ForeignSymbol, ForeignSymbolDefinition, ForeignText, ForeignTextKind,
    ForeignTool, ForeignWindow,
};
use crate::schematic::label_layout::TextEdge;

/// Reads an `.asc` schematic.
pub fn parse_asc(source: &str) -> Result<ForeignSchematic, ForeignParseError> {
    let mut schematic = ForeignSchematic::new(ForeignTool::Ltspice);
    let mut version_seen = false;
    for (index, raw) in source.lines().enumerate() {
        let line = index + 1;
        let record = raw.trim_end_matches(['\r', '\n']);
        let trimmed = record.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let words = trimmed.split_ascii_whitespace().collect::<Vec<_>>();
        let keyword = words[0].to_ascii_uppercase();
        match keyword.as_str() {
            "VERSION" => version_seen = true,
            "SHEET" => {}
            "WIRE" => {
                let [a, b] = two_points(&words, 1, line)?;
                schematic.wires.push((a, b));
            }
            "FLAG" => {
                let position = point(&words, 1, line)?;
                let name = rest_after(trimmed, 3).trim().to_owned();
                if name.is_empty() {
                    return Err(ForeignParseError::new(line, "FLAG names no net"));
                }
                schematic.flags.push(ForeignFlag {
                    position,
                    name,
                    port: None,
                    facing: None,
                });
            }
            "IOPIN" => {
                let position = point(&words, 1, line)?;
                let direction = match words.get(3).map(|word| word.to_ascii_lowercase()) {
                    Some(word) if word == "in" => ForeignPortDirection::Input,
                    Some(word) if word == "out" => ForeignPortDirection::Output,
                    Some(word) if word == "bidir" => ForeignPortDirection::Bidirectional,
                    _ => {
                        return Err(ForeignParseError::new(
                            line,
                            "IOPIN direction must be In, Out or BiDir",
                        ));
                    }
                };
                let flag = schematic
                    .flags
                    .iter_mut()
                    .rev()
                    .find(|flag| flag.position == position)
                    .ok_or_else(|| {
                        ForeignParseError::new(line, "IOPIN has no FLAG at its point")
                    })?;
                flag.port = Some(direction);
            }
            "SYMBOL" => {
                if words.len() < 5 {
                    return Err(ForeignParseError::new(
                        line,
                        "SYMBOL needs a name, a point and an orientation",
                    ));
                }
                let orientation_word = words[words.len() - 1];
                let orientation =
                    ForeignOrientation::from_ltspice(orientation_word).ok_or_else(|| {
                        ForeignParseError::new(
                            line,
                            format!("unknown symbol orientation `{orientation_word}`"),
                        )
                    })?;
                let position = point(&words, words.len() - 3, line)?;
                let symbol = words[1..words.len() - 3].join(" ");
                schematic.symbols.push(ForeignSymbol {
                    symbol,
                    position,
                    orientation,
                    attributes: Vec::new(),
                    windows: Vec::new(),
                    value_pieces: Vec::new(),
                    embedded: None,
                    line,
                });
            }
            "SYMATTR" => {
                let symbol = schematic
                    .symbols
                    .last_mut()
                    .ok_or_else(|| ForeignParseError::new(line, "SYMATTR before any SYMBOL"))?;
                let name = words
                    .get(1)
                    .ok_or_else(|| ForeignParseError::new(line, "SYMATTR names no attribute"))?;
                set_attribute(&mut symbol.attributes, name, rest_after(trimmed, 2));
            }
            "WINDOW" => {
                let symbol = schematic
                    .symbols
                    .last_mut()
                    .ok_or_else(|| ForeignParseError::new(line, "WINDOW before any SYMBOL"))?;
                set_window(&mut symbol.windows, window(&words, line)?);
            }
            "TEXT" => {
                let position = point(&words, 1, line)?;
                let body = rest_after(trimmed, 5);
                let (kind, text) = match body.chars().next() {
                    Some('!') => (ForeignTextKind::Directive, &body[1..]),
                    Some(';') => (ForeignTextKind::Comment, &body[1..]),
                    _ => (ForeignTextKind::Comment, body),
                };
                schematic.texts.push(ForeignText {
                    position,
                    kind,
                    text: unescape_text(text),
                    line,
                    lettering: ForeignLettering {
                        size_percent: text_size_percent(words.get(4).copied()),
                        justification: text_justification(words.get(3).copied()),
                        by_first_line: true,
                    },
                });
            }
            "LINE" | "RECTANGLE" | "CIRCLE" | "ARC" => {
                schematic.shapes.push(shape(&keyword, &words, line)?);
            }
            "DATAFLAG" => {
                let position = point(&words, 1, line)?;
                schematic.data_flags.push(ForeignDataFlag {
                    position,
                    expression: rest_after(trimmed, 3).trim().trim_matches('"').to_owned(),
                });
            }
            "BUSTAP" => {
                let [a, b] = two_points(&words, 1, line)?;
                schematic.bus_taps.push((a, b));
            }
            _ => {
                return Err(ForeignParseError::new(
                    line,
                    format!("unknown record `{}`", words[0]),
                ));
            }
        }
    }
    if !version_seen {
        return Err(ForeignParseError::new(
            0,
            "not an LTspice schematic: it has no Version record",
        ));
    }
    Ok(schematic)
}

/// Reads an `.asy` symbol, keeping what an instance needs: pins in
/// `SpiceOrder`, the drawing, and the default attributes.
pub fn parse_asy(source: &str) -> Result<ForeignSymbolDefinition, ForeignParseError> {
    let mut definition = ForeignSymbolDefinition::default();
    let mut version_seen = false;
    for (index, raw) in source.lines().enumerate() {
        let line = index + 1;
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            continue;
        }
        let words = trimmed.split_ascii_whitespace().collect::<Vec<_>>();
        let keyword = words[0].to_ascii_uppercase();
        match keyword.as_str() {
            "VERSION" => version_seen = true,
            "SYMBOLTYPE" => {
                definition.symbol_type = words.get(1).copied().unwrap_or_default().to_owned();
            }
            "SYMATTR" => {
                let name = words
                    .get(1)
                    .ok_or_else(|| ForeignParseError::new(line, "SYMATTR names no attribute"))?;
                set_attribute(&mut definition.attributes, name, rest_after(trimmed, 2));
            }
            "WINDOW" => set_window(&mut definition.windows, window(&words, line)?),
            "PIN" => {
                let position = point(&words, 1, line)?;
                definition.pins.push(ForeignPin {
                    name: String::new(),
                    spice_order: 0,
                    position,
                    default_net: None,
                    // The label's justification; `NONE` letters no name.
                    name_hidden: words
                        .get(3)
                        .is_some_and(|justification| justification.eq_ignore_ascii_case("NONE")),
                    // How far in from the pin the name starts.
                    name_offset: words.get(4).and_then(|offset| offset.parse().ok()),
                    name_at: pin_name_at(words.get(3).copied(), words.get(4).copied()),
                });
            }
            "PINATTR" => {
                let pin = definition
                    .pins
                    .last_mut()
                    .ok_or_else(|| ForeignParseError::new(line, "PINATTR before any PIN"))?;
                match words
                    .get(1)
                    .map(|word| word.to_ascii_lowercase())
                    .as_deref()
                {
                    Some("pinname") => pin.name = rest_after(trimmed, 2).to_owned(),
                    Some("spiceorder") => {
                        pin.spice_order = words
                            .get(2)
                            .and_then(|word| word.parse().ok())
                            .filter(|order| *order > 0)
                            .ok_or_else(|| {
                                ForeignParseError::new(line, "SpiceOrder must be a positive number")
                            })?;
                    }
                    _ => {}
                }
            }
            "LINE" | "RECTANGLE" | "CIRCLE" | "ARC" => {
                definition.shapes.push(shape(&keyword, &words, line)?);
            }
            // Free text drawn on the symbol carries no electrical meaning.
            "TEXT" => {}
            _ => {
                return Err(ForeignParseError::new(
                    line,
                    format!("unknown symbol record `{}`", words[0]),
                ));
            }
        }
    }
    if !version_seen {
        return Err(ForeignParseError::new(
            0,
            "not an LTspice symbol: it has no Version record",
        ));
    }
    // A pin with no order of its own follows the pins before it, the way
    // LTspice numbers them when a symbol leaves SpiceOrder out.
    let mut next = definition
        .pins
        .iter()
        .map(|pin| pin.spice_order)
        .max()
        .unwrap_or(0);
    for pin in &mut definition.pins {
        if pin.spice_order == 0 {
            next += 1;
            pin.spice_order = next;
        }
    }
    Ok(definition)
}

fn set_attribute(attributes: &mut Vec<ForeignAttribute>, name: &str, value: &str) {
    let value = value.to_owned();
    match attributes
        .iter_mut()
        .find(|attribute| attribute.name.eq_ignore_ascii_case(name))
    {
        Some(attribute) => attribute.value = value,
        None => attributes.push(ForeignAttribute {
            name: name.to_owned(),
            value,
        }),
    }
}

fn set_window(windows: &mut Vec<ForeignWindow>, window: ForeignWindow) {
    windows.retain(|existing| existing.attribute != window.attribute);
    windows.push(window);
}

fn window(words: &[&str], line: usize) -> Result<ForeignWindow, ForeignParseError> {
    let attribute = words
        .get(1)
        .and_then(|word| word.parse().ok())
        .ok_or_else(|| ForeignParseError::new(line, "WINDOW needs an attribute number"))?;
    let offset = point(words, 2, line)?;
    let alignment = words.get(4).copied().unwrap_or("Left").to_ascii_lowercase();
    Ok(ForeignWindow {
        attribute,
        offset,
        reach: window_reach(&alignment),
        hidden: alignment == "invisible",
    })
}

/// Which way a `WINDOW`'s text reaches from its anchor, in the symbol's own
/// frame: `Left` text starts at the anchor, `Top` text hangs below it, and
/// the `V` forms are the same runs turned a quarter counter-clockwise. Each
/// word names one axis; the text centres on the other.
fn window_reach(alignment: &str) -> ForeignPoint {
    match alignment {
        "left" | "vtop" => ForeignPoint::new(1, 0),
        "right" | "vbottom" => ForeignPoint::new(-1, 0),
        "top" | "vright" => ForeignPoint::new(0, 1),
        "bottom" | "vleft" => ForeignPoint::new(0, -1),
        _ => ForeignPoint::new(0, 0),
    }
}

fn shape(keyword: &str, words: &[&str], line: usize) -> Result<ForeignShape, ForeignParseError> {
    // The second field is the line weight (`Normal`, `Wide`); points follow.
    match keyword {
        "ARC" => {
            let [a, b] = two_points(words, 2, line)?;
            let [c, d] = two_points(words, 6, line)?;
            Ok(ForeignShape::Arc([a, b, c, d]))
        }
        _ => {
            let [a, b] = two_points(words, 2, line)?;
            Ok(match keyword {
                "LINE" => ForeignShape::Line(a, b),
                "RECTANGLE" => ForeignShape::Rectangle(a, b),
                _ => ForeignShape::Ellipse(a, b),
            })
        }
    }
}

fn number(words: &[&str], index: usize, line: usize) -> Result<i32, ForeignParseError> {
    let word = words.get(index).ok_or_else(|| {
        ForeignParseError::new(line, format!("`{}` is missing a coordinate", words[0]))
    })?;
    word.parse::<i32>()
        .ok()
        .filter(|value| value.abs() <= 10_000_000)
        .ok_or_else(|| ForeignParseError::new(line, format!("`{word}` is not a coordinate")))
}

fn point(words: &[&str], index: usize, line: usize) -> Result<ForeignPoint, ForeignParseError> {
    Ok(ForeignPoint::new(
        number(words, index, line)?,
        number(words, index + 1, line)?,
    ))
}

fn two_points(
    words: &[&str],
    index: usize,
    line: usize,
) -> Result<[ForeignPoint; 2], ForeignParseError> {
    Ok([point(words, index, line)?, point(words, index + 2, line)?])
}

/// What follows the first `fields` space-separated fields of a record, with
/// its own spacing kept.
/// LTspice's text size code: an index into the sizes its text dialog
/// offers, 0.625x to 7x, of which 1.5x (code 2) is the standard.
fn text_size_percent(code: Option<&str>) -> u16 {
    const PERCENT: [u16; 8] = [42, 67, 100, 133, 167, 233, 333, 467];
    code.and_then(|code| code.parse::<usize>().ok())
        .and_then(|index| PERCENT.get(index).copied())
        .unwrap_or(100)
}

/// Which edge of a text, across and down, LTspice stands on its point. A
/// `V` before the word turns the text upright; its box is placed the same.
/// Where LTspice letters a pin's name: `offset` from the pin, away from the
/// edge the justification puts the pin on, the name reaching on from there
/// and centred across. A name lettered upright on its side (`VTOP`) has no
/// place here.
fn pin_name_at(
    justification: Option<&str>,
    offset: Option<&str>,
) -> Option<(ForeignPoint, ForeignPoint)> {
    let reach = match justification?.to_ascii_uppercase().as_str() {
        "LEFT" => ForeignPoint::new(1, 0),
        "RIGHT" => ForeignPoint::new(-1, 0),
        "TOP" => ForeignPoint::new(0, 1),
        "BOTTOM" => ForeignPoint::new(0, -1),
        _ => return None,
    };
    let offset = offset
        .and_then(|offset| offset.parse::<i32>().ok())
        .unwrap_or(0);
    Some((ForeignPoint::new(reach.x * offset, reach.y * offset), reach))
}

fn text_justification(word: Option<&str>) -> (TextEdge, TextEdge) {
    let word = word.unwrap_or("Left").to_ascii_lowercase();
    match word.strip_prefix('v').unwrap_or(&word) {
        "right" => (TextEdge::End, TextEdge::Center),
        "center" => (TextEdge::Center, TextEdge::Center),
        "top" => (TextEdge::Center, TextEdge::Start),
        "bottom" => (TextEdge::Center, TextEdge::End),
        _ => (TextEdge::Start, TextEdge::Center),
    }
}

fn rest_after(record: &str, fields: usize) -> &str {
    let mut rest = record.trim_start();
    for _ in 0..fields {
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        rest = rest[end..].trim_start();
    }
    rest
}

/// LTspice writes a line break inside a text record as `\n`.
fn unescape_text(text: &str) -> String {
    text.replace("\\n", "\n")
}
