//! The source a new Verilog-A view starts from: the module it declares,
//! named after its cell, and a declaration per interface port.

/// The module a new Verilog-A view of `cell_name` starts out declaring: the
/// cell's own name, as a Virtuoso `veriloga` view's module is named after its
/// cell. A cell name no module can carry is made into one: each character an
/// identifier cannot hold becomes `_`, a leading digit gains a `_` in front,
/// and a Verilog-AMS keyword, by the frontend's own table, gains a `_` behind.
///
/// This names only what a starter declares. Once the source exists it is the
/// authority, and a view keeps whatever module its source declares.
pub fn verilog_module_name(cell_name: &str) -> String {
    let mut name = cell_name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if name.is_empty() || name.starts_with(|character: char| character.is_ascii_digit()) {
        name.insert(0, '_');
    }
    if rspice_veriloga::TokenKind::keyword(&name).is_some() {
        name.push('_');
    }
    name
}

fn is_verilog_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

/// How a module declares the port an interface names: a scalar under its
/// own name, or a vector `d[7:0]` under `d` with its range. `None` for a name
/// a Verilog-A module cannot declare.
pub fn verilog_port_declaration(name: &str) -> Option<(&str, Option<(u32, u32)>)> {
    if is_verilog_identifier(name) {
        return Some((name, None));
    }
    let vector = rspice_design_model::bus::declared_vector(name)?;
    let base = name.split_once('[')?.0;
    (vector.notation == rspice_design_model::bus::BusNotation::Square
        && vector.width() > 1
        && is_verilog_identifier(base))
    .then_some((base, Some((vector.msb, vector.lsb))))
}

/// The module a new Verilog-A view starts from: its header, a direction
/// declaration per port and its discipline, and no behaviour. A vector port
/// is declared under its base name with its range, as `inout [7:0] d;`. A
/// supply pin is an `inout` port, the only way a module can declare one.
pub fn starter_veriloga_source(module_name: &str, ports: &[crate::state::PortSpec]) -> String {
    use crate::state::PortDirection;

    let declared = ports
        .iter()
        .filter_map(|port| {
            verilog_port_declaration(&port.name).map(|(base, range)| (port.direction, base, range))
        })
        .collect::<Vec<_>>();
    let mut source = String::from("`include \"constants.vams\"\n`include \"disciplines.vams\"\n\n");
    if declared.is_empty() {
        source.push_str(&format!("module {module_name};\nendmodule\n"));
        return source;
    }
    let names = declared
        .iter()
        .map(|(_, base, _)| *base)
        .collect::<Vec<_>>();
    source.push_str(&format!("module {module_name}({});\n", names.join(", ")));
    // Scalars that share a keyword share a line; each vector has its own,
    // because a declaration states one range for everything it names.
    let declare = |source: &mut String, keyword: &str, group: Vec<(&str, Option<(u32, u32)>)>| {
        let scalars = group
            .iter()
            .filter(|(_, range)| range.is_none())
            .map(|(base, _)| *base)
            .collect::<Vec<_>>();
        if !scalars.is_empty() {
            source.push_str(&format!("    {keyword} {};\n", scalars.join(", ")));
        }
        for (base, range) in group {
            if let Some((msb, lsb)) = range {
                source.push_str(&format!("    {keyword} [{msb}:{lsb}] {base};\n"));
            }
        }
    };
    for (keyword, direction) in [
        ("input", PortDirection::In),
        ("output", PortDirection::Out),
        ("inout", PortDirection::InOut),
    ] {
        let group = declared
            .iter()
            .filter(|(port_direction, _, _)| {
                *port_direction == direction
                    || (direction == PortDirection::InOut
                        && *port_direction == PortDirection::Supply)
            })
            .map(|(_, base, range)| (*base, *range))
            .collect::<Vec<_>>();
        declare(&mut source, keyword, group);
    }
    declare(
        &mut source,
        "electrical",
        declared
            .iter()
            .map(|(_, base, range)| (*base, *range))
            .collect(),
    );
    source.push_str("endmodule\n");
    source
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{PortDirection, PortSpec};

    /// A vector port is declared under its base name with its range, the
    /// only spelling a module header can carry.
    #[test]
    fn a_starter_declares_a_bus_port_under_its_base_name_with_its_range() {
        let source = starter_veriloga_source(
            "dac",
            &[
                PortSpec {
                    name: "d[7:0]".to_owned(),
                    direction: PortDirection::In,
                },
                PortSpec {
                    name: "clk".to_owned(),
                    direction: PortDirection::In,
                },
                PortSpec {
                    name: "out".to_owned(),
                    direction: PortDirection::Out,
                },
            ],
        );
        assert!(
            source.contains(
                "module dac(d, clk, out);\n    input clk;\n    input [7:0] d;\n    output out;\n    electrical clk, out;\n    electrical [7:0] d;\nendmodule\n"
            ),
            "{source}"
        );
        assert_eq!(
            verilog_port_declaration("d[7:0]"),
            Some(("d", Some((7, 0))))
        );
        assert_eq!(verilog_port_declaration("A<3:0>"), None);
        assert_eq!(verilog_port_declaration("d[3:3]"), None);
        assert_eq!(verilog_port_declaration("9d"), None);
    }

    /// A `veriloga` view's module is named after its cell, as in Virtuoso.
    /// A cell name a module cannot carry is made into one, and nothing is
    /// added to a name that already is one.
    #[test]
    fn a_new_module_is_named_after_its_cell() {
        for (cell, module) in [
            ("opamp_ideal", "opamp_ideal"),
            ("Gain2", "Gain2"),
            ("2stage", "_2stage"),
            ("\u{c9}tage", "_tage"),
            ("amp-v2", "amp_v2"),
            // Verilog-AMS keywords, by the frontend's table: a module named
            // `module` or `analog` would not parse.
            ("module", "module_"),
            ("analog", "analog_"),
            ("wire", "wire_"),
            // Keywords are case-sensitive, so this one is a name.
            ("Module", "Module"),
        ] {
            assert_eq!(verilog_module_name(cell), module, "{cell}");
        }
    }
}
