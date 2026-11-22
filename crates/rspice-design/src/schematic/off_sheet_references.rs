//! Where an off-sheet connector's partners stand: the page and zone of every
//! connector of the same name on another sheet of the drawing, which the
//! canvas and the printed page both letter after the connector's tab, as
//! KiCad letters intersheet references beside a global label.

use std::collections::{BTreeMap, HashMap};

use rspice_design_model::Point;
use rspice_design_model::design_management::{SheetCatalog, SheetId};

use super::net_label::NetLabel;

/// What each off-sheet connector on `sheet` letters after its tab: the page
/// and zone of every connector of the same name on another sheet, such as
/// `[p2 B3, p4 A1]`, so a reader can find where the signal goes.
///
/// `zone_at` names the zone a point stands in on a sheet, such as `B3`, or
/// `None` where that sheet's format draws no zones; such a page gives its
/// number alone. A connector with no partner on another sheet letters
/// nothing, and neither does a drawing of one sheet. An object the catalog
/// assigns to no sheet stands on the active one, as the canvas draws it.
pub fn off_sheet_partner_references(
    net_labels: &[NetLabel],
    catalog: &SheetCatalog,
    sheet: SheetId,
    mut zone_at: impl FnMut(SheetId, Point) -> Option<String>,
) -> HashMap<u64, String> {
    let Some(active) = catalog.active_sheet_id() else {
        return HashMap::new();
    };
    // Two connectors are partners when the netlister would join them, and
    // it folds ASCII case.
    let mut by_name: BTreeMap<String, Vec<(SheetId, &NetLabel)>> = BTreeMap::new();
    for connector in net_labels {
        if connector.kind.off_sheet_direction().is_some() {
            let on = catalog.sheet_for_object(connector.id).unwrap_or(active);
            by_name
                .entry(connector.name.trim().to_ascii_lowercase())
                .or_default()
                .push((on, connector));
        }
    }
    let mut references = HashMap::new();
    for connectors in by_name.values() {
        for &(on, connector) in connectors {
            if on != sheet {
                continue;
            }
            let mut partners = connectors
                .iter()
                .filter(|(elsewhere, _)| *elsewhere != sheet)
                .filter_map(|&(elsewhere, partner)| {
                    let (page, _) = catalog.page_number_and_count(elsewhere)?;
                    Some((page, zone_at(elsewhere, partner.pos)))
                })
                .collect::<Vec<_>>();
            if partners.is_empty() {
                continue;
            }
            partners.sort();
            partners.dedup();
            let partners = partners
                .into_iter()
                .map(|(page, zone)| match zone {
                    Some(zone) => format!("p{page} {zone}"),
                    None => format!("p{page}"),
                })
                .collect::<Vec<_>>()
                .join(", ");
            references.insert(connector.id, format!("[{partners}]"));
        }
    }
    references
}
