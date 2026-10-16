//! Built-in XSPICE vector widths over canonical port metadata and a local draft.
//!
//! One row per port the reader can size. A port whose width is fixed by the
//! model is not a decision, so it is not a row.

use rspice_design::schematic::device_catalog::CatalogXspiceVectorPort;
use rspice_ui_kit::widgets::FormRows;
use std::collections::BTreeMap;

/// Whether this port is one the reader chooses a width for.
pub fn is_sizable(port: &CatalogXspiceVectorPort) -> bool {
    port.maximum.is_none_or(|maximum| maximum != port.minimum)
}

/// The rows of the vector-width form: whether any width changed, and the
/// control that takes the caret.
pub fn show(
    rows: &mut FormRows<'_>,
    ports: &[CatalogXspiceVectorPort],
    widths: &mut BTreeMap<String, usize>,
) -> (bool, Option<egui::Id>) {
    let mut edited = false;
    let mut first = None;
    for port in ports.iter().filter(|port| is_sizable(port)) {
        let maximum = port.maximum.unwrap_or(usize::MAX);
        let width = widths
            .entry(port.name.clone())
            .or_insert(port.default_width);
        let allowed = if port.null_allowed && port.minimum == 0 {
            format!("0\u{2013}{maximum} (0 leaves it out)")
        } else {
            format!("{}\u{2013}{maximum}", port.minimum)
        };
        let response = rows.number(&port.name, width, port.minimum..=maximum, &allowed);
        first.get_or_insert(response.id);
        edited |= response.changed();
    }
    (edited, first)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn port(name: &str, minimum: usize, maximum: Option<usize>) -> CatalogXspiceVectorPort {
        CatalogXspiceVectorPort {
            name: name.to_owned(),
            minimum,
            maximum,
            default_width: minimum.max(1),
            null_allowed: minimum == 0,
        }
    }

    /// A port the model pins to one width is not a choice, so the form does
    /// not offer it as a disabled one.
    #[test]
    fn only_a_port_whose_width_can_change_is_a_row() {
        assert!(!is_sizable(&port("clk", 1, Some(1))));
        assert!(is_sizable(&port("in", 2, Some(8))));
        assert!(is_sizable(&port("out", 0, None)));
    }
}
