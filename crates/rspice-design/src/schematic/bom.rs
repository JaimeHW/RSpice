//! Bill-of-materials fields a component carries, and the grouped BOM a design
//! exports as CSV.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::component::Component;
use super::component_type::ComponentType;

/// Purchasing data for one placed part. Every field is free text as a
/// datasheet writes it; an empty field is unknown, not zero.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct BomFields {
    pub manufacturer: String,
    pub mpn: String,
    pub description: String,
    pub package: String,
    pub voltage_rating: String,
    pub current_rating: String,
    pub power_rating: String,
}

/// One BOM field, for surfaces that edit them one at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BomField {
    Manufacturer,
    Mpn,
    Description,
    Package,
    VoltageRating,
    CurrentRating,
    PowerRating,
}

impl BomField {
    pub const ALL: [Self; 7] = [
        Self::Manufacturer,
        Self::Mpn,
        Self::Description,
        Self::Package,
        Self::VoltageRating,
        Self::CurrentRating,
        Self::PowerRating,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Manufacturer => "Manufacturer",
            Self::Mpn => "MPN",
            Self::Description => "Description",
            Self::Package => "Package",
            Self::VoltageRating => "Voltage rating",
            Self::CurrentRating => "Current rating",
            Self::PowerRating => "Power rating",
        }
    }

    /// The name a property sheet and bulk edit address this field by. The
    /// `bom_` prefix keeps it apart from every SPICE instance parameter, so
    /// a sheet row can never be mistaken for one and written to the deck.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Manufacturer => "bom_manufacturer",
            Self::Mpn => "bom_mpn",
            Self::Description => "bom_description",
            Self::Package => "bom_package",
            Self::VoltageRating => "bom_voltage_rating",
            Self::CurrentRating => "bom_current_rating",
            Self::PowerRating => "bom_power_rating",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|field| field.key().eq_ignore_ascii_case(key))
    }
}

impl BomFields {
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    pub fn get(&self, field: BomField) -> &str {
        match field {
            BomField::Manufacturer => &self.manufacturer,
            BomField::Mpn => &self.mpn,
            BomField::Description => &self.description,
            BomField::Package => &self.package,
            BomField::VoltageRating => &self.voltage_rating,
            BomField::CurrentRating => &self.current_rating,
            BomField::PowerRating => &self.power_rating,
        }
    }

    pub fn get_mut(&mut self, field: BomField) -> &mut String {
        match field {
            BomField::Manufacturer => &mut self.manufacturer,
            BomField::Mpn => &mut self.mpn,
            BomField::Description => &mut self.description,
            BomField::Package => &mut self.package,
            BomField::VoltageRating => &mut self.voltage_rating,
            BomField::CurrentRating => &mut self.current_rating,
            BomField::PowerRating => &mut self.power_rating,
        }
    }
}

/// Whether a component is something bought and placed on a board. Sources,
/// ports, probes, ground, ideal behavioral blocks and built-in code or
/// Verilog-A models exist only for the simulator, unless the designer gave
/// one a part number.
pub fn is_purchased_part(component: &Component) -> bool {
    if !component.bom.mpn.trim().is_empty() {
        return true;
    }
    if component
        .library_cell
        .as_ref()
        .is_some_and(|binding| binding.is_executable_builtin())
    {
        return false;
    }
    let kind = component.kind;
    !(kind.is_source()
        || matches!(
            kind,
            ComponentType::Vcvs
                | ComponentType::Vccs
                | ComponentType::Ccvs
                | ComponentType::Cccs
                | ComponentType::BehavioralSource
                | ComponentType::TransmissionLine
                | ComponentType::LossyTransmissionLine
                | ComponentType::CoupledTransmissionLine
                | ComponentType::RfPort
                | ComponentType::LoopProbe
                | ComponentType::Ground
                | ComponentType::Port
                | ComponentType::XspiceGain
                | ComponentType::XspiceSummer
                | ComponentType::XspiceMultiplier
                | ComponentType::XspiceDivider
                | ComponentType::XspiceLimiter
                | ComponentType::XspiceIntegrator
                | ComponentType::XspiceDifferentiator
                | ComponentType::XspiceAdcBridge
                | ComponentType::XspiceDacBridge
        ))
}

/// One BOM line: the parts that share a value and every purchasing field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BomLine {
    pub references: Vec<String>,
    pub value: String,
    pub fields: BomFields,
}

impl BomLine {
    /// `R1-R4, R7`: a run of three or more consecutive designators with one
    /// prefix collapses to a range; anything else is listed.
    pub fn reference_ranges(&self) -> String {
        let mut out: Vec<String> = Vec::new();
        let mut index = 0;
        while index < self.references.len() {
            let start = &self.references[index];
            let mut end = index;
            if let Some((prefix, first)) = split_designator(start) {
                while let Some((next_prefix, next)) = self
                    .references
                    .get(end + 1)
                    .and_then(|reference| split_designator(reference))
                {
                    if next_prefix != prefix || next != first + (end + 1 - index) as u64 {
                        break;
                    }
                    end += 1;
                }
            }
            if end - index >= 2 {
                out.push(format!("{start}-{}", self.references[end]));
            } else {
                out.extend(self.references[index..=end].iter().cloned());
            }
            index = end + 1;
        }
        out.join(", ")
    }
}

/// `("R", 12)` for `R12`; `None` for a designator without a trailing number.
fn split_designator(reference: &str) -> Option<(&str, u64)> {
    let digits = reference
        .char_indices()
        .rev()
        .take_while(|(_, c)| c.is_ascii_digit())
        .last()
        .map(|(index, _)| index)?;
    let (prefix, number) = reference.split_at(digits);
    // A leading zero (`R01`) is part of the name, not a count.
    if number.len() > 1 && number.starts_with('0') {
        return None;
    }
    Some((prefix, number.parse().ok()?))
}

/// Designators in reading order: prefix, then number, then text.
fn designator_order(a: &str, b: &str) -> std::cmp::Ordering {
    match (split_designator(a), split_designator(b)) {
        (Some((pa, na)), Some((pb, nb))) => pa.cmp(pb).then(na.cmp(&nb)),
        _ => a.cmp(b),
    }
}

/// Group purchased parts into BOM lines. A line holds the parts of one
/// device type that agree on value and every purchasing field, so two parts
/// sharing an MPN but disagreeing on, say, the package are never merged into
/// a line that misstates one of them. Lines are ordered by first designator.
pub fn bom_lines<'a>(parts: impl IntoIterator<Item = (&'a str, &'a Component)>) -> Vec<BomLine> {
    let mut groups: BTreeMap<(&'static str, String, BomFields), Vec<String>> = BTreeMap::new();
    for (reference, component) in parts {
        if !is_purchased_part(component) {
            continue;
        }
        let fields = BomFields {
            manufacturer: component.bom.manufacturer.trim().to_owned(),
            mpn: component.bom.mpn.trim().to_owned(),
            description: component.bom.description.trim().to_owned(),
            package: component.bom.package.trim().to_owned(),
            voltage_rating: component.bom.voltage_rating.trim().to_owned(),
            current_rating: component.bom.current_rating.trim().to_owned(),
            power_rating: component.bom.power_rating.trim().to_owned(),
        };
        groups
            .entry((
                component.kind.display_name(),
                component.value.trim().to_owned(),
                fields,
            ))
            .or_default()
            .push(reference.to_owned());
    }
    let mut lines = groups
        .into_iter()
        .map(|((_, value, fields), mut references)| {
            references.sort_by(|a, b| designator_order(a, b));
            references.dedup();
            BomLine {
                references,
                value,
                fields,
            }
        })
        .collect::<Vec<_>>();
    lines.sort_by(|a, b| designator_order(&a.references[0], &b.references[0]));
    lines
}

/// The BOM as RFC 4180 CSV: a header row, CRLF line ends, and a field quoted
/// only when it holds a comma, a quote or a line break.
pub fn bom_csv(lines: &[BomLine]) -> String {
    let mut header = vec!["Reference", "Quantity", "Value"];
    header.extend(BomField::ALL.map(BomField::label));
    let mut out = csv_row(header.iter().map(|cell| (*cell).to_owned()));
    for line in lines {
        let mut cells = vec![
            line.reference_ranges(),
            line.references.len().to_string(),
            line.value.clone(),
        ];
        cells.extend(BomField::ALL.map(|field| line.fields.get(field).to_owned()));
        out.push_str(&csv_row(cells));
    }
    out
}

fn csv_row(cells: impl IntoIterator<Item = String>) -> String {
    let mut row = cells
        .into_iter()
        .map(|cell| {
            if cell.contains([',', '"', '\r', '\n']) {
                format!("\"{}\"", cell.replace('"', "\"\""))
            } else {
                cell
            }
        })
        .collect::<Vec<_>>()
        .join(",");
    row.push_str("\r\n");
    row
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::component::Component;
    use crate::schematic::component_type::ComponentType;
    use rspice_design_model::Point;

    fn part(kind: ComponentType, name: &str, value: &str, mpn: &str) -> Component {
        let mut component = Component::new(0, kind, Point::new(0, 0));
        component.name = name.to_owned();
        component.value = value.to_owned();
        component.bom.mpn = mpn.to_owned();
        component
    }

    #[test]
    fn parts_group_by_value_and_mpn_with_reference_ranges() {
        let mut parts = Vec::new();
        for name in ["R7", "R2", "R1", "R4", "R3"] {
            parts.push(part(
                ComponentType::Resistor,
                name,
                "10k",
                "RC0603FR-0710KL",
            ));
        }
        parts.push(part(ComponentType::Resistor, "R5", "10k", ""));
        parts.push(part(
            ComponentType::Resistor,
            "R6",
            "4.7k",
            "RC0603FR-074K7L",
        ));
        parts.push(part(
            ComponentType::Resistor,
            "R8",
            "4.7k",
            "RC0603FR-074K7L",
        ));
        parts.push(part(ComponentType::Capacitor, "C1", "1u", ""));
        parts.push(part(ComponentType::Inductor, "L1", "1u", ""));
        parts.push(part(ComponentType::VoltageSource, "V1", "5", ""));
        for component in parts.iter_mut().take(5) {
            component.bom.manufacturer = "Yageo".into();
            component.bom.description = "Resistor, 1%, \"thick film\"".into();
        }
        let lines = bom_lines(parts.iter().map(|c| (c.name.as_str(), c)));
        let csv = bom_csv(&lines);
        let rows = csv.split("\r\n").collect::<Vec<_>>();
        assert_eq!(
            rows[0],
            "Reference,Quantity,Value,Manufacturer,MPN,Description,Package,\
             Voltage rating,Current rating,Power rating"
        );
        // The source is a simulation construct, and the 1u capacitor and
        // inductor are different parts despite sharing a value.
        assert_eq!(
            &rows[1..],
            [
                "C1,1,1u,,,,,,,",
                "L1,1,1u,,,,,,,",
                "\"R1-R4, R7\",5,10k,Yageo,RC0603FR-0710KL,\"Resistor, 1%, \"\"thick film\"\"\",,,,",
                "R5,1,10k,,,,,,,",
                "\"R6, R8\",2,4.7k,,RC0603FR-074K7L,,,,,",
                "",
            ]
        );
    }

    #[test]
    fn a_source_with_a_part_number_is_purchased() {
        let source = part(ComponentType::VoltageSource, "V1", "5", "");
        assert!(!is_purchased_part(&source));
        let regulator = part(ComponentType::VoltageSource, "V2", "5", "LM7805");
        assert!(is_purchased_part(&regulator));
    }

    #[test]
    fn designators_without_a_plain_number_are_listed_not_ranged() {
        let line = BomLine {
            references: ["R01", "R02", "R03", "R9", "R10", "R11", "Rload"]
                .map(str::to_owned)
                .to_vec(),
            value: "1k".into(),
            fields: BomFields::default(),
        };
        assert_eq!(line.reference_ranges(), "R01, R02, R03, R9-R11, Rload");
    }
}
