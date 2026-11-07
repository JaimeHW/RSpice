//! What each LTspice and QSPICE symbol becomes in RSpice.
//!
//! A symbol RSpice draws natively maps to a component type, with the RSpice
//! terminal each of its pins lands on, in instance-line order. Every other
//! symbol keeps its own drawing and pins and becomes a cell instance whose
//! card is written with the symbol's own device letter.

use crate::schematic::component_type::ComponentType;

use super::{ForeignPin, ForeignPoint, ForeignSymbolDefinition, ForeignTool};

/// A symbol RSpice has its own component for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct NativeMapping {
    pub kind: NativeKind,
    /// The RSpice terminal each source pin lands on, in instance-line order.
    /// A terminal the source symbol does not have is tied to the terminal
    /// the source simulator ties it to (`tie`).
    pub terminals: &'static [&'static str],
    pub tie: Option<(&'static str, &'static str)>,
}

/// Which component type a native symbol becomes. Sources and switches are
/// decided by their value, not by the symbol alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NativeKind {
    Fixed(ComponentType),
    VoltageSource,
    CurrentSource,
}

const TWO: &[&str] = &["+", "-"];
const DIODE: &[&str] = &["A", "K"];
const BJT: &[&str] = &["C", "B", "E"];
const BJT4: &[&str] = &["C", "B", "E", "S"];
const MOS3: &[&str] = &["D", "G", "S"];
const MOS4: &[&str] = &["D", "G", "S", "B"];
const CONTROLLED: &[&str] = &["O+", "O-", "C+", "C-"];
const SWITCH: &[&str] = &["1", "2", "c+", "c-"];
const LINE: &[&str] = &["a+", "a-", "b+", "b-"];

const fn fixed(kind: ComponentType, terminals: &'static [&'static str]) -> NativeMapping {
    NativeMapping {
        kind: NativeKind::Fixed(kind),
        terminals,
        tie: None,
    }
}

/// LTspice's built-in symbols that RSpice draws natively, by file name.
pub(super) fn ltspice_native(base_name: &str) -> Option<NativeMapping> {
    use ComponentType as T;
    let name = base_name.to_ascii_lowercase();
    Some(match name.as_str() {
        "res" | "res2" | "europeanresistor" => fixed(T::Resistor, TWO),
        "cap" | "polcap" | "europeancap" | "europeanpolcap" => fixed(T::Capacitor, TWO),
        "ind" | "ind2" => fixed(T::Inductor, TWO),
        "diode" | "zener" | "schottky" | "led" | "varactor" | "tvsdiode" => fixed(T::Diode, DIODE),
        "npn" | "npn2" | "npn3" => fixed(T::NpnBjt, BJT),
        "pnp" | "pnp2" | "pnp3" => fixed(T::PnpBjt, BJT),
        "npn4" => fixed(T::NpnBjt4, BJT4),
        "pnp4" => fixed(T::PnpBjt4, BJT4),
        // LTspice's three-pin MOSFET ties its body to its source.
        "nmos" => NativeMapping {
            kind: NativeKind::Fixed(T::Nmos),
            terminals: MOS3,
            tie: Some(("B", "S")),
        },
        "pmos" => NativeMapping {
            kind: NativeKind::Fixed(T::Pmos),
            terminals: MOS3,
            tie: Some(("B", "S")),
        },
        "nmos4" => fixed(T::Nmos, MOS4),
        "pmos4" => fixed(T::Pmos, MOS4),
        "njf" => fixed(T::Njfet, MOS3),
        "pjf" => fixed(T::Pjfet, MOS3),
        "mesfet" => fixed(T::Nmesfet, MOS3),
        "e" | "e2" => fixed(T::Vcvs, CONTROLLED),
        "g" | "g2" => fixed(T::Vccs, CONTROLLED),
        "bv" | "bi" | "bi2" => fixed(T::BehavioralSource, TWO),
        "sw" => fixed(T::VSwitch, SWITCH),
        "tline" => fixed(T::TransmissionLine, LINE),
        "voltage" | "battery" | "cell" => NativeMapping {
            kind: NativeKind::VoltageSource,
            terminals: TWO,
            tie: None,
        },
        "current" | "load" | "load2" => NativeMapping {
            kind: NativeKind::CurrentSource,
            terminals: TWO,
            tie: None,
        },
        _ => return None,
    })
}

/// QSPICE's built-in device letters that RSpice draws natively.
pub(super) fn qspice_native(device_type: &str, pin_count: usize) -> Option<NativeMapping> {
    use ComponentType as T;
    Some(match (device_type, pin_count) {
        ("R", 2) => fixed(T::Resistor, TWO),
        ("C", 2) => fixed(T::Capacitor, TWO),
        ("L", 2) => fixed(T::Inductor, TWO),
        ("D", 2) => fixed(T::Diode, DIODE),
        ("QN", 3) => fixed(T::NpnBjt, BJT),
        ("QP", 3) => fixed(T::PnpBjt, BJT),
        ("QN", 4) => fixed(T::NpnBjt4, BJT4),
        ("QP", 4) => fixed(T::PnpBjt4, BJT4),
        ("MN", 4) => fixed(T::Nmos, MOS4),
        ("MP", 4) => fixed(T::Pmos, MOS4),
        ("MN", 3) => NativeMapping {
            kind: NativeKind::Fixed(T::Nmos),
            terminals: MOS3,
            tie: Some(("B", "S")),
        },
        ("MP", 3) => NativeMapping {
            kind: NativeKind::Fixed(T::Pmos),
            terminals: MOS3,
            tie: Some(("B", "S")),
        },
        ("JN", 3) => fixed(T::Njfet, MOS3),
        ("JP", 3) => fixed(T::Pjfet, MOS3),
        ("E", 4) => fixed(T::Vcvs, CONTROLLED),
        ("G", 4) => fixed(T::Vccs, CONTROLLED),
        ("B", 2) => fixed(T::BehavioralSource, TWO),
        ("S", 4) => fixed(T::VSwitch, SWITCH),
        ("V", 2) => NativeMapping {
            kind: NativeKind::VoltageSource,
            terminals: TWO,
            tie: None,
        },
        ("I", 2) => NativeMapping {
            kind: NativeKind::CurrentSource,
            terminals: TWO,
            tie: None,
        },
        _ => return None,
    })
}

/// Why a QSPICE device whose symbol type is `device_type` (`¥`, `Ã`, `Ø`, …)
/// and whose stated type is `kind` (`D-FLOP`, `RROPAMP`, empty when it
/// states none) is left out, or `None` when RSpice imports its kind.
pub(super) fn qspice_refusal(device_type: &str, kind: &str) -> Option<String> {
    let letter = device_type.chars().next()?;
    // QSPICE's drawings write the Φ of a phase detector's type as Ø.
    let kind = kind.trim().to_uppercase().replace("Ø-DET", "Φ-DET");
    let what = match letter {
        'Ø' => {
            return Some(
                "is a QSPICE C++/Verilog DLL block; RSpice runs behavioral blocks written in Verilog-A, so rewrite it as a Verilog-A module and place that instead"
                    .to_owned(),
            );
        }
        'Ã' if kind == "RROPAMP" => "rail-to-rail op-amp",
        'Ã' => "Gm amplifier",
        '¥' => match kind.as_str() {
            "D-FLOP" => "D flip-flop",
            "JK-FLOP" => "JK flip-flop",
            "SR-FLOP" => "SR flip-flop",
            "T-FLOP" => "toggle flip-flop",
            "PS-FLOP" => "switching-regulator flip-flop",
            "CLOCKSYNC" => "clock synchronizer",
            "MONOSTABLE" => "monostable",
            "ULATOR" => "AM/FM modulator",
            "Φ-DET" => "phase/frequency detector",
            oscillator if oscillator.contains("OSC") || oscillator == "VCO" => "oscillator",
            _ => "digital device",
        },
        '€' => "digital block",
        'Ö' | '£' | '×' | '¢' | 'Æ' => "device",
        _ => return None,
    };
    let device = if kind.is_empty() {
        format!("{letter}-device")
    } else {
        format!("{letter}-device {kind}")
    };
    Some(format!(
        "is a QSPICE behavioral {what} ({device}), which has no RSpice equivalent"
    ))
}

/// The parts LTspice's power and discrete library draws that RSpice will
/// carry as its own parts; until then they come across as the symbol LTspice
/// draws, with their model read in place.
pub(super) const LTSPICE_AWAITING_NATIVE_PARTS: &[&str] = &[
    "scr", "triac", "diac", "xtal", "nigbt", "pigbt", "neonbulb", "triode", "tetrode", "pentode",
];

/// LTspice's own pins for its primitive symbols, as its symbol library draws
/// them, so a schematic imports on a machine without LTspice installed.
pub(super) fn ltspice_builtin_definition(base_name: &str) -> Option<ForeignSymbolDefinition> {
    let name = base_name.to_ascii_lowercase();
    let (prefix, pins): (&str, &[(&str, i32, i32)]) = match name.as_str() {
        "res" | "ind" | "ind2" => (
            if name == "res" { "R" } else { "L" },
            &[("A", 16, 16), ("B", 16, 96)],
        ),
        "res2" | "cap" | "polcap" => (
            if name == "res2" { "R" } else { "C" },
            &[("A", 16, 0), ("B", 16, 64)],
        ),
        "diode" | "zener" | "schottky" | "led" | "varactor" | "tvsdiode" => {
            ("D", &[("+", 16, 0), ("-", 16, 64)])
        }
        "voltage" | "h" | "bv" => (
            match name.as_str() {
                "voltage" => "V",
                "h" => "H",
                _ => "B",
            },
            &[("+", 0, 16), ("-", 0, 96)],
        ),
        "current" | "f" | "bi" | "csw" | "load2" => (
            match name.as_str() {
                "current" | "load2" => "I",
                "f" => "F",
                "csw" => "W",
                _ => "B",
            },
            &[("+", 0, 0), ("-", 0, 80)],
        ),
        "bi2" => ("B", &[("+", 0, 80), ("-", 0, 0)]),
        "load" => ("I", &[("A", 16, 0), ("B", 16, 64)]),
        "npn" | "pnp" | "npn2" | "pnp2" | "npn3" | "pnp3" => (
            if name.starts_with('n') { "QN" } else { "QP" },
            &[("C", 64, 0), ("B", 0, 48), ("E", 64, 96)],
        ),
        "npn4" | "pnp4" => (
            if name.starts_with('n') { "QN" } else { "QP" },
            &[("C", 64, 0), ("B", 0, 48), ("E", 64, 96), ("S", 64, 48)],
        ),
        "nmos" | "pmos" | "mesfet" => (
            match name.as_str() {
                "nmos" => "MN",
                "pmos" => "MP",
                _ => "Z",
            },
            &[("D", 48, 0), ("G", 0, 80), ("S", 48, 96)],
        ),
        "nmos4" | "pmos4" => (
            if name.starts_with('n') { "MN" } else { "MP" },
            &[("D", 48, 0), ("G", 0, 80), ("S", 48, 96), ("B", 48, 48)],
        ),
        "njf" | "pjf" => (
            if name.starts_with('n') { "JN" } else { "JP" },
            &[("D", 48, 0), ("G", 0, 64), ("S", 48, 96)],
        ),
        "e" => (
            "E",
            &[("+", 0, 16), ("-", 0, 96), ("P", -48, 32), ("N", -48, 80)],
        ),
        "e2" => (
            "E",
            &[("+", 0, 16), ("-", 0, 96), ("P", -48, 80), ("N", -48, 32)],
        ),
        "g" => (
            "G",
            &[
                ("+", 0, 96),
                ("-", 0, 16),
                ("NC+", -48, 32),
                ("NC-", -48, 80),
            ],
        ),
        "g2" => (
            "G",
            &[
                ("+", 0, 96),
                ("-", 0, 16),
                ("NC+", -48, 80),
                ("NC-", -48, 32),
            ],
        ),
        "sw" => (
            "S",
            &[
                ("A", 0, 16),
                ("B", 0, 96),
                ("NC+", -48, 80),
                ("NC-", -48, 32),
            ],
        ),
        "tline" | "ltline" => (
            if name == "tline" { "T" } else { "O" },
            &[
                ("I1", -48, -16),
                ("R1", -48, 16),
                ("I2", 48, -16),
                ("R2", 48, 16),
            ],
        ),
        _ => return None,
    };
    Some(ForeignSymbolDefinition {
        symbol_type: "CELL".to_owned(),
        pins: pins
            .iter()
            .enumerate()
            .map(|(index, (pin, x, y))| ForeignPin {
                name: (*pin).to_owned(),
                spice_order: index + 1,
                position: ForeignPoint::new(*x, *y),
                default_net: None,
                name_hidden: false,
                name_offset: None,
            })
            .collect(),
        shapes: Vec::new(),
        attributes: vec![super::ForeignAttribute {
            name: "Prefix".to_owned(),
            value: prefix.to_owned(),
        }],
        windows: Vec::new(),
    })
}

/// Which native mapping a symbol takes, if any.
pub(super) fn native_mapping(
    tool: ForeignTool,
    base_name: &str,
    definition: Option<&ForeignSymbolDefinition>,
) -> Option<NativeMapping> {
    let mapping = match tool {
        ForeignTool::Ltspice => ltspice_native(base_name)?,
        ForeignTool::Qspice => {
            let definition = definition?;
            qspice_native(definition.symbol_type.trim(), definition.pins.len())?
        }
    };
    // A symbol whose pins disagree with the mapping's is a different part
    // under a familiar name; it keeps its own drawing.
    match definition {
        Some(definition) if definition.pins.len() != mapping.terminals.len() => None,
        _ => Some(mapping),
    }
}

/// The source simulator's spelling of a number, in RSpice's: the micro sign
/// is `u`.
pub(super) fn spice_text(text: &str) -> String {
    text.trim().replace(['\u{B5}', '\u{3BC}'], "u")
}

/// How an independent source's value text reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceValue {
    pub kind: ComponentType,
    /// The waveform or DC value, as the card writes it.
    pub value: String,
    /// `ac`, `acphase`, and the series/parallel parasitics RSpice names.
    pub params: Vec<(String, String)>,
    /// Words RSpice has no field for, kept on the instance.
    pub extra: Vec<String>,
}

/// Reads an LTspice or QSPICE source value: an optional `DC`, a value or a
/// waveform, an optional `AC magnitude [phase]`, and `Rser=`/`Cpar=`/`Rpar=`
/// parasitics.
pub(super) fn source_value(voltage: bool, text: &str) -> SourceValue {
    use ComponentType as T;
    let text = spice_text(text);
    let tokens = spice_tokens(&text);
    let mut kind = if voltage {
        T::VoltageSource
    } else {
        T::CurrentSource
    };
    let mut value = String::new();
    let mut params = Vec::new();
    let mut extra = Vec::new();
    let mut arguments = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let token = &tokens[index];
        let upper = token.to_ascii_uppercase();
        let keyword = upper.split('(').next().unwrap_or_default();
        if upper == "DC" {
            index += 1;
            continue;
        }
        if upper == "AC" {
            let magnitude = tokens.get(index + 1).filter(|word| is_number_word(word));
            let phase = magnitude
                .and(tokens.get(index + 2))
                .filter(|word| is_number_word(word));
            params.push((
                "ac".to_owned(),
                magnitude.cloned().unwrap_or_else(|| "1".to_owned()),
            ));
            if let Some(phase) = phase {
                params.push(("acphase".to_owned(), phase.clone()));
            }
            index += 1 + usize::from(magnitude.is_some()) + usize::from(phase.is_some());
            continue;
        }
        let waveform = match keyword {
            "PULSE" => Some(if voltage {
                T::VoltageSourcePulse
            } else {
                T::CurrentSourcePulse
            }),
            "SINE" | "SIN" => Some(if voltage {
                T::VoltageSourceSin
            } else {
                T::CurrentSourceSin
            }),
            "PWL" => Some(if voltage {
                T::VoltageSourcePwl
            } else {
                T::CurrentSourcePwl
            }),
            "EXP" => Some(if voltage {
                T::VoltageSourceExp
            } else {
                T::CurrentSourceExp
            }),
            "SFFM" => Some(if voltage {
                T::VoltageSourceSffm
            } else {
                T::CurrentSourceSffm
            }),
            _ => None,
        };
        if let Some(waveform) = waveform.filter(|_| value.is_empty()) {
            kind = waveform;
            index += 1;
            // LTspice's `SINE` is SPICE's `SIN`.
            let name = if keyword == "SINE" {
                "SIN"
            } else {
                &token[..keyword.len()]
            };
            if token.contains('(') {
                let written = &token[keyword.len()..];
                value = format!("{name}{written}");
                arguments = written
                    .trim()
                    .strip_prefix('(')
                    .and_then(|inner| inner.strip_suffix(')'))
                    .map(spice_tokens)
                    .unwrap_or_default();
                continue;
            }
            // QSPICE and LTspice also write a waveform's arguments as bare
            // words after its name: `PWL 0 0 .5m 48`.
            arguments = tokens[index..]
                .iter()
                .take_while(|word| {
                    let upper = word.to_ascii_uppercase();
                    is_number_word(word) && upper != "AC" && upper != "DC"
                })
                .cloned()
                .collect::<Vec<_>>();
            index += arguments.len();
            value = format!("{name}({})", arguments.join(" "));
            continue;
        }
        if let Some((key, parameter)) = token.split_once('=') {
            let name = match key.to_ascii_lowercase().as_str() {
                "rser" => Some("rs"),
                "rpar" => Some("rp"),
                "cpar" => Some("cpar"),
                _ => None,
            };
            match name {
                Some(name) => params.push((name.to_owned(), parameter.to_owned())),
                None => extra.push(token.clone()),
            }
            index += 1;
            continue;
        }
        if value.is_empty() && is_number_word(token) {
            value = token.clone();
        } else {
            extra.push(token.clone());
        }
        index += 1;
    }
    if extra.is_empty()
        && let Some((first, named)) = named_waveform(kind, &arguments)
    {
        value = first;
        params = named.into_iter().chain(params).collect();
    }
    SourceValue {
        kind,
        value,
        params,
        extra,
    }
}

/// A positional `PULSE` or `SIN` as the named fields RSpice's own sources
/// keep: the first argument as the value, the rest by name. Only a list
/// that gives every argument the named form writes is read, so the card
/// written from the fields is the source's own: `PULSE` with seven or eight
/// arguments, `SIN` with three to six (the delay, damping and phase written
/// for absent ones are 0, SPICE's own defaults). A `PWL` keeps its time and
/// value pairs as its value, the list its card writes between `PWL(` and
/// `)`. Any other list, and one written with commas, stays as written.
fn named_waveform(
    kind: ComponentType,
    arguments: &[String],
) -> Option<(String, Vec<(String, String)>)> {
    use ComponentType as T;
    let count = arguments.len();
    let plain = arguments
        .iter()
        .all(|argument| !argument.contains(',') && is_number_word(argument));
    if matches!(kind, T::VoltageSourcePwl | T::CurrentSourcePwl) {
        return (plain && count >= 2 && count.is_multiple_of(2))
            .then(|| (arguments.join(" "), Vec::new()));
    }
    let names: &[&str] = match kind {
        T::VoltageSourcePulse if (7..=8).contains(&count) => {
            &["v1", "v2", "td", "tr", "tf", "pw", "per", "np"]
        }
        T::CurrentSourcePulse if (7..=8).contains(&count) => {
            &["i1", "i2", "td", "tr", "tf", "pw", "per", "np"]
        }
        T::VoltageSourceSin if (3..=6).contains(&count) => {
            &["vo", "va", "freq", "td", "theta", "phase"]
        }
        T::CurrentSourceSin if (3..=6).contains(&count) => {
            &["io", "ia", "freq", "td", "theta", "phase"]
        }
        _ => return None,
    };
    if !plain {
        return None;
    }
    let (first, rest) = arguments.split_first()?;
    let named = names[1..]
        .iter()
        .zip(rest)
        .map(|(name, argument)| ((*name).to_owned(), argument.clone()))
        .collect();
    Some((first.clone(), named))
}

/// A value word: a number, a braced expression, or a parameter name.
fn is_number_word(word: &str) -> bool {
    !word.contains('=') && !word.contains('(')
        || word.starts_with('{')
        || word.starts_with('(') && word.ends_with(')')
}

/// Splits SPICE text at spaces outside parentheses, braces and quotes, so a
/// waveform such as `PULSE(0 5 0 1n)` stays one token.
pub(super) fn spice_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut depth = 0_i32;
    let mut quoted = false;
    for character in text.chars() {
        match character {
            '"' => quoted = !quoted,
            '(' | '{' if !quoted => depth += 1,
            ')' | '}' if !quoted => depth -= 1,
            _ => {}
        }
        if character.is_whitespace() && depth <= 0 && !quoted {
            // `PULSE (0 5)` and `Rser = 1` are written apart; keep them whole.
            if !current.is_empty() {
                tokens.push(std::mem::take(&mut current));
            }
            continue;
        }
        if (character == '(' && depth == 1 || character == '=')
            && current.is_empty()
            && let Some(previous) = tokens.pop()
        {
            current = previous;
        }
        current.push(character);
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    // Rejoin `key =value` and `key= value` written with spaces.
    let mut joined: Vec<String> = Vec::new();
    for token in tokens {
        match joined.last_mut() {
            Some(last) if last.ends_with('=') => last.push_str(&token),
            _ => joined.push(token),
        }
    }
    joined
}
