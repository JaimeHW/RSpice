//! A run's net geometry, sheet by sheet, in the coordinates each sheet draws.
//!
//! The netlister reads the projected document, which lays the sheet at
//! catalog index `k` a whole stride along x so that no two sheets'
//! coordinates meet. The canvas draws every sheet at its own origin. A point
//! read off the projection is therefore taken home to the sheet that owns it
//! before anything compares it with a drawn coordinate; compared as it
//! stands, the second sheet's nets land a million units off the canvas and
//! the first sheet's land on whatever the second sheet draws there.

use std::collections::HashMap;

use rspice_design::schematic::owned::projected_point_sheet;
use rspice_design_model::Point;
use rspice_design_model::design_management::{SheetCatalog, SheetId};

/// One sheet's share of a run's net geometry, in that sheet's drawn
/// coordinates.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SheetNetGeometry {
    /// Net node points (vertices, terminals, junctions, labels) to net name.
    /// Points between nodes resolve through `net_segments`.
    pub point_to_net: HashMap<Point, String>,
    /// Each net's node points on this sheet.
    pub net_to_points: HashMap<String, Vec<Point>>,
    /// Each net's wire segments on this sheet.
    pub net_segments: HashMap<String, Vec<(Point, Point)>>,
}

/// Every sheet's geometry, keyed by the sheet that draws it. A cell view with
/// no sheet catalog has a single entry, under `None`.
pub type SheetNetGeometries = HashMap<Option<SheetId>, SheetNetGeometry>;

/// Split geometry read off the projected document into the sheets that draw
/// it. A point that lies on no sheet the catalog has is dropped: there is
/// nowhere on the canvas it could be shown.
pub fn net_geometry_by_sheet(
    catalog: Option<&SheetCatalog>,
    point_to_net: HashMap<Point, String>,
    net_to_points: HashMap<String, Vec<Point>>,
    net_segments: HashMap<String, Vec<(Point, Point)>>,
) -> SheetNetGeometries {
    let Some(catalog) = catalog else {
        return HashMap::from([(
            None,
            SheetNetGeometry {
                point_to_net,
                net_to_points,
                net_segments,
            },
        )]);
    };
    let mut sheets = SheetNetGeometries::new();
    for (point, net) in point_to_net {
        if let Some((sheet, drawn)) = projected_point_sheet(catalog, point) {
            sheets
                .entry(Some(sheet))
                .or_default()
                .point_to_net
                .insert(drawn, net);
        }
    }
    for (net, points) in net_to_points {
        for point in points {
            if let Some((sheet, drawn)) = projected_point_sheet(catalog, point) {
                sheets
                    .entry(Some(sheet))
                    .or_default()
                    .net_to_points
                    .entry(net.clone())
                    .or_default()
                    .push(drawn);
            }
        }
    }
    for (net, segments) in net_segments {
        for (start, end) in segments {
            let Some((sheet, drawn_start)) = projected_point_sheet(catalog, start) else {
                continue;
            };
            // One wire lies on one sheet, so the shift that takes the start
            // home takes the end with it.
            let drawn_end = Point::new(
                end.x.saturating_sub(start.x.saturating_sub(drawn_start.x)),
                end.y.saturating_sub(start.y.saturating_sub(drawn_start.y)),
            );
            sheets
                .entry(Some(sheet))
                .or_default()
                .net_segments
                .entry(net.clone())
                .or_default()
                .push((drawn_start, drawn_end));
        }
    }
    sheets
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design_model::design_management::{SheetDefinition, SheetPortPolicy, SheetTemplate};

    fn two_sheets() -> (SheetCatalog, SheetId, SheetId) {
        let mut catalog = SheetCatalog::default();
        let mut ids = Vec::new();
        for page in [1, 2] {
            ids.push(
                catalog
                    .create_sheet(
                        SheetDefinition {
                            name: format!("Sheet {page}"),
                            template: SheetTemplate::AnalogSchematic,
                            port_policy: SheetPortPolicy::TypedOffSheetPorts,
                            explicit_page_number: Some(page),
                        },
                        ids.last().copied(),
                    )
                    .expect("sheet"),
            );
        }
        (catalog, ids[0], ids[1])
    }

    #[test]
    fn each_point_goes_home_to_the_sheet_that_draws_it() {
        let (catalog, first, second) = two_sheets();
        let drawn = Point::new(100, 0);
        let projected = Point::new(1_000_100, 0);
        let sheets = net_geometry_by_sheet(
            Some(&catalog),
            HashMap::from([(drawn, "VOUT".to_owned()), (projected, "BIAS".to_owned())]),
            HashMap::from([
                ("VOUT".to_owned(), vec![drawn]),
                ("BIAS".to_owned(), vec![projected]),
            ]),
            HashMap::from([(
                "BIAS".to_owned(),
                vec![(projected, Point::new(1_000_200, 40))],
            )]),
        );

        assert_eq!(sheets.len(), 2);
        let on_first = &sheets[&Some(first)];
        assert_eq!(
            on_first.point_to_net.get(&drawn).map(String::as_str),
            Some("VOUT")
        );
        assert_eq!(on_first.net_to_points.get("BIAS"), None);
        let on_second = &sheets[&Some(second)];
        assert_eq!(
            on_second.point_to_net.get(&drawn).map(String::as_str),
            Some("BIAS")
        );
        assert_eq!(on_second.net_to_points["BIAS"], vec![drawn]);
        assert_eq!(
            on_second.net_segments["BIAS"],
            vec![(drawn, Point::new(200, 40))]
        );
    }

    #[test]
    fn a_cell_view_without_sheets_keeps_its_geometry_as_drawn() {
        let point = Point::new(1_000_100, 0);
        let sheets = net_geometry_by_sheet(
            None,
            HashMap::from([(point, "N1".to_owned())]),
            HashMap::from([("N1".to_owned(), vec![point])]),
            HashMap::new(),
        );
        assert_eq!(sheets.len(), 1);
        assert_eq!(sheets[&None].net_to_points["N1"], vec![point]);
    }
}
