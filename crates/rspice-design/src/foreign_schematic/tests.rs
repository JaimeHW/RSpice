use super::*;

/// Every `.asc` record type the reader knows, in one drawing.
const EVERY_RECORD: &str = "Version 4\r
SHEET 1 880 680\r
WIRE 96 80 16 80\r
WIRE 96 176 96 160\r
FLAG 16 160 0\r
FLAG 96 80 out\r
IOPIN 96 80 Out\r
FLAG 208 80 in\r
IOPIN 208 80 In\r
FLAG 208 160 io\r
IOPIN 208 160 BiDir\r
SYMBOL res 80 64 R0\r
WINDOW 0 36 40 Left 2\r
WINDOW 3 36 76 Left 2\r
SYMATTR InstName R1\r
SYMATTR Value 1k\r
SYMATTR SpiceLine tc=0.001\r
SYMBOL Digital\\and 320 96 M90\r
SYMATTR InstName A1\r
TEXT 16 232 Left 2 !.tran 0 1m 0 1u\\n.param r=1k\r
TEXT 16 264 Left 2 ;a note to the reader\r
LINE Normal 0 0 64 0\r
RECTANGLE Normal 0 0 64 32 2\r
CIRCLE Normal 0 0 32 32\r
ARC Normal 0 0 32 32 32 16 0 16\r
DATAFLAG 96 80 \"$\"\r
BUSTAP 304 80 304 96\r
";

#[test]
fn an_asc_drawing_keeps_every_record_it_holds() {
    let drawing = parse_asc(EVERY_RECORD).expect("the drawing reads");
    assert_eq!(drawing.tool, ForeignTool::Ltspice);
    assert_eq!(
        drawing.wires,
        vec![
            (ForeignPoint::new(96, 80), ForeignPoint::new(16, 80)),
            (ForeignPoint::new(96, 176), ForeignPoint::new(96, 160)),
        ]
    );
    let flags = drawing
        .flags
        .iter()
        .map(|flag| (flag.name.as_str(), flag.port))
        .collect::<Vec<_>>();
    assert_eq!(
        flags,
        vec![
            ("0", None),
            ("out", Some(ForeignPortDirection::Output)),
            ("in", Some(ForeignPortDirection::Input)),
            ("io", Some(ForeignPortDirection::Bidirectional)),
        ]
    );
    assert!(drawing.flags[0].is_ground());

    let resistor = &drawing.symbols[0];
    assert_eq!(resistor.symbol, "res");
    assert_eq!(resistor.position, ForeignPoint::new(80, 64));
    assert_eq!(resistor.orientation, ForeignOrientation::new(0, false));
    assert_eq!(resistor.attribute("InstName"), Some("R1"));
    assert_eq!(resistor.attribute("value"), Some("1k"));
    assert_eq!(resistor.attribute("SpiceLine"), Some("tc=0.001"));
    assert_eq!(resistor.windows.len(), 2);
    assert_eq!(resistor.windows[1].offset, ForeignPoint::new(36, 76));

    let gate = &drawing.symbols[1];
    assert_eq!(gate.symbol, "Digital\\and");
    assert_eq!(gate.base_name(), "and");
    assert_eq!(gate.orientation, ForeignOrientation::new(1, true));

    assert_eq!(drawing.texts.len(), 2);
    assert_eq!(drawing.texts[0].kind, ForeignTextKind::Directive);
    assert_eq!(drawing.texts[0].text, ".tran 0 1m 0 1u\n.param r=1k");
    assert_eq!(drawing.texts[1].kind, ForeignTextKind::Comment);
    assert_eq!(drawing.texts[1].text, "a note to the reader");

    assert_eq!(drawing.shapes.len(), 4);
    assert!(matches!(drawing.shapes[3], ForeignShape::Arc(_)));
    assert_eq!(drawing.data_flags[0].expression, "$");
    assert_eq!(drawing.bus_taps.len(), 1);
}

#[test]
fn every_ltspice_orientation_places_a_pin_where_ltspice_draws_it() {
    // A resistor's lower pin, at (16, 96) from its origin, under each
    // orientation, as LTspice's own netlists of its example schematics put it.
    let pin = ForeignPoint::new(16, 96);
    for (name, expected) in [
        ("R0", (16, 96)),
        ("R90", (-96, 16)),
        ("R180", (-16, -96)),
        ("R270", (96, -16)),
        ("M0", (-16, 96)),
        ("M90", (96, 16)),
        ("M180", (16, -96)),
        ("M270", (-96, -16)),
    ] {
        let orientation = ForeignOrientation::from_ltspice(name).expect("a known orientation");
        assert_eq!(orientation.ltspice_name(), name);
        assert_eq!(
            orientation.apply(pin),
            ForeignPoint::new(expected.0, expected.1),
            "{name}"
        );
    }
    assert_eq!(ForeignOrientation::from_ltspice("R45"), None);
}

#[test]
fn utf16_and_code_page_files_read_like_utf8() {
    let utf8 = parse_asc(EVERY_RECORD).expect("UTF-8 reads");
    let little = EVERY_RECORD
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    let mut marked = vec![0xFF, 0xFE];
    marked.extend_from_slice(&little);
    let big = EVERY_RECORD
        .encode_utf16()
        .flat_map(u16::to_be_bytes)
        .collect::<Vec<_>>();
    for bytes in [little, marked, big] {
        let decoded = parse_asc(&decode_foreign_text(&bytes)).expect("UTF-16 reads");
        assert_eq!(decoded, utf8);
    }
    // LTspice writes micro as the code-page byte 0xB5.
    assert_eq!(
        decode_foreign_text(b"SYMATTR Value 10\xB5"),
        "SYMATTR Value 10\u{B5}"
    );
    assert_eq!(decode_foreign_text(b"\x80"), "\u{20AC}");
}

#[test]
fn an_asc_without_a_version_or_with_a_bad_record_is_refused_by_line() {
    assert!(parse_asc("WIRE 0 0 16 0\n").is_err());
    let error = parse_asc("Version 4\nSYMBOL res 0 0 R45\n").expect_err("R45 is refused");
    assert_eq!(error.line, 2);
    let error = parse_asc("Version 4\nWIRE 0 0 x 0\n").expect_err("a word is no coordinate");
    assert_eq!(error.line, 2);
    let error = parse_asc("Version 4\nIOPIN 0 0 In\n").expect_err("an IOPIN needs its FLAG");
    assert!(error.message.contains("FLAG"));
}

#[test]
fn an_asy_symbol_lists_its_pins_in_spice_order() {
    let symbol = parse_asy(
        "Version 4\nSymbolType BLOCK\nRECTANGLE Normal -32 -32 32 32\nTEXT 0 0 Center 2 U\n\
         WINDOW 0 0 -40 Bottom 2\nSYMATTR Prefix X\nSYMATTR SpiceModel amp.sub\nSYMATTR Value amp\n\
         PIN 32 0 LEFT 8\nPINATTR PinName OUT\nPINATTR SpiceOrder 3\n\
         PIN -32 -16 NONE 0\nPINATTR PinName IN\nPINATTR SpiceOrder 1\n\
         PIN -32 16 NONE 0\nPINATTR PinName REF\nPINATTR SpiceOrder 2\n",
    )
    .expect("the symbol reads");
    assert_eq!(symbol.symbol_type, "BLOCK");
    assert_eq!(symbol.attribute("prefix"), Some("X"));
    assert_eq!(symbol.attribute("SpiceModel"), Some("amp.sub"));
    let pins = symbol
        .ordered_pins()
        .into_iter()
        .map(|pin| (pin.name.as_str(), pin.position))
        .collect::<Vec<_>>();
    assert_eq!(
        pins,
        vec![
            ("IN", ForeignPoint::new(-32, -16)),
            ("REF", ForeignPoint::new(-32, 16)),
            ("OUT", ForeignPoint::new(32, 0)),
        ]
    );
    assert_eq!(symbol.shapes.len(), 1);
}

/// A QSPICE drawing: the file mark, then the record tree with its marks and
/// device letters in code-page bytes and a UTF-8 note inside one string.
fn qsch(body: &str) -> Vec<u8> {
    let mut bytes = vec![0xFF, 0xD8, 0xFF, 0xDB];
    for character in body.chars() {
        match character {
            '«' => bytes.push(0xAB),
            '»' => bytes.push(0xBB),
            '¥' => bytes.push(0xA5),
            '¬' => bytes.push(0xAC),
            other => {
                let mut buffer = [0; 4];
                bytes.extend_from_slice(other.encode_utf8(&mut buffer).as_bytes());
            }
        }
    }
    bytes
}

const QSPICE_DRAWING: &str = "«schematic
  «component (0,0) 6 0
    «symbol V
      «type: V»
      «description: Independent Voltage Source»
      «line (0,-130) (0,-200) 0 0 0x1000000 -1 -1»
      «ellipse (-130,130) (130,-130) 0 0 0 0x1000000 0x1000000 -1 -1»
      «text (-130,0) 1 46 0 0x1000000 -1 -1 \"V1\"»
      «text (130,0) 1 45 0 0x1000000 -1 -1 \"SINE(0 1 1K)\"»
      «pin (0,200) (0,0) 1 0 0 0x1000000 -1 \"+\"»
      «pin (0,-200) (0,0) 1 0 0 0x1000000 -1 \"-\"»
    »
  »
  «component (500,0) 8 0
    «symbol SCHMITT
      «type: ¥»
      «text (150,300) 1 7 0 0x1000000 -1 -1 \"¥1\"»
      «text (50,0) 0.7 0 2 0x1000000 -1 -1 \"HMITT\"»
      «text (150,-200) 1 7 0 0x1000000 -1 -1 \"Vt=1.2\"»
      «text (100,-400) 1 7 0 0x1000000 -1 -1 \"Vh=50m\"»
      «pin (400,0) (0,0) 1 0 0 0x1000000 -1 \"Q\"»
      «pin (0,0) (0,0) 1 0 0 0x1000000 -1 \"¬Q\" \"¥\"»
    »
  »
  «net (-400,0) 1 11 0 \"GND\"»
  «net (900,0) 1 14 0 \"OUT\"»
  «junction (200,0)»
  «wire (-400,0) (-200,0) \"GND\"»
  «wire (200,0) (900,0) \"OUT\"»
  «text (0,-900) 0.8 7 0 0x1000000 -1 -1 \"\u{FEFF}.tran 3m\\n.plot V(out)\"»
  «text (0,900) 1 14 1 0x1000000 -1 -1 \"\u{FEFF}150°C \"lockout\" \\nnote\"»
»
";

#[test]
fn a_qsch_drawing_reads_its_components_nets_and_texts() {
    let drawing = parse_qsch(&qsch(QSPICE_DRAWING)).expect("the drawing reads");
    assert_eq!(drawing.tool, ForeignTool::Qspice);
    assert_eq!(drawing.symbols.len(), 2);

    let source = &drawing.symbols[0];
    assert_eq!(source.symbol, "V");
    assert_eq!(source.attribute("InstName"), Some("V1"));
    assert_eq!(source.attribute("Value"), Some("SINE(0 1 1K)"));
    // Code 6 turns the source three quarters counter-clockwise: its `+` pin,
    // above its origin, lands to the right.
    assert_eq!(source.orientation, ForeignOrientation::new(1, false));
    let definition = source.embedded.as_ref().expect("QSPICE embeds the symbol");
    assert_eq!(definition.symbol_type, "V");
    assert_eq!(
        definition.attribute("description"),
        Some("Independent Voltage Source")
    );
    let plus = &definition.pins[0];
    assert_eq!((plus.name.as_str(), plus.spice_order), ("+", 1));
    assert_eq!(
        source.orientation.apply(plus.position),
        ForeignPoint::new(200, 0)
    );
    assert_eq!(definition.shapes.len(), 2);

    let gate = &drawing.symbols[1];
    assert_eq!(gate.orientation, ForeignOrientation::new(0, true));
    assert_eq!(gate.attribute("InstName"), Some("¥1"));
    // The drawn-only text stays off the instance line.
    assert_eq!(gate.attribute("Value"), Some("Vt=1.2 Vh=50m"));
    let gate_definition = gate.embedded.as_ref().expect("embedded");
    assert_eq!(gate_definition.symbol_type, "¥");
    assert_eq!(gate_definition.pins[1].name, "¬Q");

    assert_eq!(drawing.flags[0].name, "0");
    assert_eq!(drawing.flags[1].name, "OUT");
    // The drawing is turned over: QSPICE's y points up.
    assert_eq!(drawing.flags[1].position, ForeignPoint::new(900, 0));
    assert_eq!(drawing.junctions, vec![ForeignPoint::new(200, 0)]);
    assert_eq!(drawing.wire_nets, vec!["GND".to_owned(), "OUT".to_owned()]);

    assert_eq!(drawing.texts[0].kind, ForeignTextKind::Directive);
    assert_eq!(drawing.texts[0].text, ".tran 3m\n.plot V(out)");
    assert_eq!(drawing.texts[0].position, ForeignPoint::new(0, 900));
    assert_eq!(drawing.texts[1].kind, ForeignTextKind::Comment);
    assert_eq!(drawing.texts[1].text, "150°C \"lockout\" \nnote");
}

#[test]
fn a_qsch_component_turned_by_an_eighth_is_refused() {
    let turned = QSPICE_DRAWING.replacen("«component (0,0) 6 0", "«component (0,0) 3 0", 1);
    let error = parse_qsch(&qsch(&turned)).expect_err("45 degrees is refused");
    assert_eq!(error.line, 2);
    assert!(parse_qsch(b"\xFF\xD8\xFF\xDB\xABcircuit\n\xBB\n").is_err());
    assert!(parse_qsch(b"\xFF\xD8\xFF\xDB\xABschematic\n").is_err());
}
