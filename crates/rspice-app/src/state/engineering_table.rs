//! Schematic dataset capture and engineering-table export adapters.

use std::collections::HashMap;

pub use rspice_results::engineering_table::*;

use crate::state::{ComponentType, SchematicState};

pub const ACTIVE_SCHEMATIC_GRID_ID: &str = "active-schematic-objects";
pub const VIRTUALIZATION_OVERSCAN: usize = 24;

pub fn active_schematic_dataset(schematic: &SchematicState) -> EngineeringDataset {
    let columns = [
        ("identifier", "Identifier", None, true),
        ("object", "Object", None, false),
        ("value", "Value", None, false),
        ("parameters", "Parameters", None, false),
        ("x", "X", Some("grid"), false),
        ("y", "Y", Some("grid"), false),
        ("status", "Status", None, false),
        ("owner", "Owner", None, false),
    ]
    .into_iter()
    .map(|(id, label, unit, identifier)| EngineeringColumn {
        id: id.to_owned(),
        label: label.to_owned(),
        unit: unit.map(str::to_owned),
        identifier,
    })
    .collect::<Vec<_>>();
    let mut rows = Vec::with_capacity(
        schematic.document().components.len()
            + schematic.document().net_labels.len()
            + schematic.document().wires.len()
            + schematic.document().buses.len()
            + schematic.document().bus_taps.len()
            + schematic.document().junctions.len(),
    );
    for component in &schematic.document().components {
        let mut cells = HashMap::new();
        cells.insert(
            "identifier".to_owned(),
            EngineeringCell::text(&component.name),
        );
        cells.insert(
            "object".to_owned(),
            EngineeringCell::text(component_kind_label(component.kind)),
        );
        cells.insert("value".to_owned(), EngineeringCell::text(&component.value));
        cells.insert(
            "parameters".to_owned(),
            EngineeringCell::text(&component.params),
        );
        cells.insert(
            "x".to_owned(),
            EngineeringCell {
                display: component.pos.x.to_string(),
                numeric: Some(f64::from(component.pos.x)),
            },
        );
        cells.insert(
            "y".to_owned(),
            EngineeringCell {
                display: component.pos.y.to_string(),
                numeric: Some(f64::from(component.pos.y)),
            },
        );
        cells.insert(
            "status".to_owned(),
            EngineeringCell::text(if component.name.trim().is_empty() {
                "unnamed"
            } else {
                "authored"
            }),
        );
        cells.insert(
            "owner".to_owned(),
            EngineeringCell::text("Active schematic"),
        );
        rows.push(EngineeringRow {
            stable_id: format!("component-{}", component.id),
            cells,
        });
    }
    for label in &schematic.document().net_labels {
        let mut cells = HashMap::new();
        cells.insert("identifier".to_owned(), EngineeringCell::text(&label.name));
        cells.insert("object".to_owned(), EngineeringCell::text("Net label"));
        cells.insert("value".to_owned(), EngineeringCell::text(""));
        cells.insert("parameters".to_owned(), EngineeringCell::text(""));
        cells.insert(
            "x".to_owned(),
            EngineeringCell {
                display: label.pos.x.to_string(),
                numeric: Some(f64::from(label.pos.x)),
            },
        );
        cells.insert(
            "y".to_owned(),
            EngineeringCell {
                display: label.pos.y.to_string(),
                numeric: Some(f64::from(label.pos.y)),
            },
        );
        cells.insert("status".to_owned(), EngineeringCell::text("authored"));
        cells.insert(
            "owner".to_owned(),
            EngineeringCell::text("Active schematic"),
        );
        rows.push(EngineeringRow {
            stable_id: format!("net-label-{}", label.id),
            cells,
        });
    }
    for wire in &schematic.document().wires {
        let point = wire.start().unwrap_or_default();
        rows.push(object_row(
            format!("wire-{}", wire.id),
            format!("W{}", wire.id),
            "Wire",
            format!("{} segments", wire.segment_count()),
            format!("length={}", wire.length()),
            point.x,
            point.y,
        ));
    }
    for bus in &schematic.document().buses {
        let point = bus.points.first().copied().unwrap_or_default();
        rows.push(object_row(
            format!("bus-{}", bus.id),
            bus.declaration
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| format!("BUS{}", bus.id)),
            "Bus",
            format!("{} segments", bus.points.len().saturating_sub(1)),
            String::new(),
            point.x,
            point.y,
        ));
    }
    for tap in &schematic.document().bus_taps {
        rows.push(object_row(
            format!("bus-tap-{}", tap.id),
            format!("TAP{}", tap.id),
            "Bus tap",
            tap.slice.to_string(),
            format!("bus={}", tap.bus_id),
            tap.bus_point.x,
            tap.bus_point.y,
        ));
    }
    for junction in &schematic.document().junctions {
        rows.push(object_row(
            format!("junction-{}", junction.id),
            format!("J{}", junction.id),
            "Junction",
            String::new(),
            String::new(),
            junction.pos.x,
            junction.pos.y,
        ));
    }
    EngineeringDataset {
        id: ACTIVE_SCHEMATIC_GRID_ID.to_owned(),
        title: "Active schematic objects".to_owned(),
        source_revision: schematic.topology_version(),
        columns,
        rows,
    }
}

fn object_row(
    stable_id: String,
    identifier: String,
    object: &str,
    value: String,
    parameters: String,
    x: i32,
    y: i32,
) -> EngineeringRow {
    let mut cells = HashMap::new();
    cells.insert("identifier".to_owned(), EngineeringCell::text(identifier));
    cells.insert("object".to_owned(), EngineeringCell::text(object));
    cells.insert("value".to_owned(), EngineeringCell::text(value));
    cells.insert("parameters".to_owned(), EngineeringCell::text(parameters));
    cells.insert(
        "x".to_owned(),
        EngineeringCell {
            display: x.to_string(),
            numeric: Some(f64::from(x)),
        },
    );
    cells.insert(
        "y".to_owned(),
        EngineeringCell {
            display: y.to_string(),
            numeric: Some(f64::from(y)),
        },
    );
    cells.insert("status".to_owned(), EngineeringCell::text("authored"));
    cells.insert(
        "owner".to_owned(),
        EngineeringCell::text("Active schematic"),
    );
    EngineeringRow { stable_id, cells }
}

fn component_kind_label(kind: ComponentType) -> &'static str {
    match kind {
        ComponentType::Port => "Port",
        ComponentType::Ground => "Ground",
        ComponentType::CellInstance => "Cell instance",
        _ => kind.display_name(),
    }
}

#[cfg(test)]
pub fn delimited_text(
    dataset: &EngineeringDataset,
    view: &EngineeringTableView,
    delimiter: u8,
    include_headers: bool,
    include_units: bool,
) -> Result<String, String> {
    delimited_text_selected(
        dataset,
        view,
        delimiter,
        include_headers,
        include_units,
        false,
        None,
    )
}

pub fn delimited_text_selected(
    dataset: &EngineeringDataset,
    view: &EngineeringTableView,
    delimiter: u8,
    include_headers: bool,
    include_units: bool,
    include_hidden_columns: bool,
    selected_rows: Option<&std::collections::BTreeSet<String>>,
) -> Result<String, String> {
    let projection = dataset.project_selected(view, include_hidden_columns, selected_rows);
    rspice_formats::table::encode_delimited_table(
        &ProjectionSource(&projection),
        delimiter,
        include_headers,
        include_units,
    )
    .map_err(|error| error.to_string())
}

pub fn schema_json(
    dataset: &EngineeringDataset,
    view: &EngineeringTableView,
    include_hidden_columns: bool,
    include_metadata: bool,
) -> Result<String, String> {
    use rspice_formats::table_schema::{SchemaColumn, SchemaView, encode_table_schema};
    let projection = dataset.project_selected(view, include_hidden_columns, None);
    encode_table_schema(
        &dataset.id,
        &dataset.title,
        dataset.source_revision,
        dataset.rows.len(),
        projection.columns.iter().map(|column| SchemaColumn {
            id: &column.id,
            label: &column.label,
            unit: column.unit.as_deref(),
            identifier: column.identifier,
        }),
        SchemaView {
            sort: &view.sort,
            filters: &view.filters,
            filter_grammar: view.filter_grammar,
            virtualization: view.virtualization,
            frozen_identifiers: view.frozen_identifiers,
        },
        include_metadata,
    )
    .map_err(|error| error.to_string())
}

pub fn xlsx_bytes(
    dataset: &EngineeringDataset,
    view: &EngineeringTableView,
    include_headers: bool,
    include_units: bool,
    include_metadata: bool,
    include_hidden_columns: bool,
    selected_rows: Option<&std::collections::BTreeSet<String>>,
) -> Result<Vec<u8>, String> {
    let projection = dataset.project_selected(view, include_hidden_columns, selected_rows);
    let pinned_columns = view
        .columns
        .iter()
        .filter(|column| column.visible && column.pinned)
        .count()
        .min(projection.columns.len());
    let column_widths = projection
        .columns
        .iter()
        .map(|column| {
            view.columns
                .iter()
                .find(|candidate| candidate.column_id == column.id)
                .map_or(120, |candidate| candidate.width)
        })
        .collect();
    let metadata = include_metadata.then(|| {
        vec![
            ("Grid".to_owned(), dataset.id.clone()),
            ("Title".to_owned(), dataset.title.clone()),
            (
                "Source revision".to_owned(),
                dataset.source_revision.to_string(),
            ),
            (
                "Filter grammar".to_owned(),
                view.filter_grammar.label().to_owned(),
            ),
            (
                "Virtualization".to_owned(),
                view.virtualization.label().to_owned(),
            ),
        ]
    });
    rspice_formats::xlsx::encode_xlsx_table(
        &ProjectionSource(&projection),
        rspice_formats::xlsx::XlsxTableOptions {
            include_headers,
            include_units,
            pinned_columns,
            column_widths,
            metadata,
        },
    )
    .map_err(|error| error.to_string())
}

struct ProjectionSource<'a>(&'a EngineeringProjection);

impl rspice_formats::table::EngineeringTableSource for ProjectionSource<'_> {
    fn column_count(&self) -> usize {
        self.0.columns.len()
    }

    fn row_count(&self) -> usize {
        self.0.rows.len()
    }

    fn column_id(&self, column: usize) -> &str {
        &self.0.columns[column].id
    }

    fn column_label(&self, column: usize) -> &str {
        &self.0.columns[column].label
    }

    fn column_unit(&self, column: usize) -> Option<&str> {
        self.0.columns[column].unit.as_deref()
    }

    fn numeric_value(&self, row: usize, column: usize) -> Option<f64> {
        self.0.rows[row]
            .cells
            .get(&self.0.columns[column].id)
            .and_then(|cell| cell.numeric)
    }

    fn display_value(&self, row: usize, column: usize) -> Option<&str> {
        self.0.rows[row]
            .cells
            .get(&self.0.columns[column].id)
            .map(|cell| cell.display.as_str())
    }
}

pub fn parquet_bytes(
    dataset: &EngineeringDataset,
    view: &EngineeringTableView,
    include_metadata: bool,
    include_hidden_columns: bool,
    selected_rows: Option<&std::collections::BTreeSet<String>>,
) -> Result<Vec<u8>, String> {
    let projection = dataset.project_selected(view, include_hidden_columns, selected_rows);
    let metadata = include_metadata.then(|| {
        vec![
            ("rspice.grid_id".to_owned(), Some(dataset.id.clone())),
            (
                "rspice.source_revision".to_owned(),
                Some(dataset.source_revision.to_string()),
            ),
            ("rspice.view".to_owned(), serde_json::to_string(view).ok()),
        ]
    });
    rspice_formats::columnar::encode_parquet_table(&ProjectionSource(&projection), metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, Point};

    fn dataset() -> EngineeringDataset {
        let mut schematic = SchematicState::default();
        schematic.document_mut_for_test().components.push(
            Component::new(7, ComponentType::Resistor, Point::new(20, -10))
                .with_name_value("R7", "10k"),
        );
        schematic.document_mut_for_test().components.push(
            Component::new(2, ComponentType::Capacitor, Point::new(0, 30))
                .with_name_value("C2", "2p"),
        );
        active_schematic_dataset(&schematic)
    }

    #[test]
    fn active_schematic_dataset_contains_only_authored_objects() {
        let dataset = dataset();
        assert_eq!(dataset.rows.len(), 2);
        assert!(
            dataset
                .rows
                .iter()
                .any(|row| row.stable_id == "component-7")
        );
        assert!(
            !dataset
                .rows
                .iter()
                .any(|row| row.stable_id.contains("demo"))
        );
    }

    #[test]
    fn view_controls_filter_sort_and_export_projection() {
        let dataset = dataset();
        let mut view = EngineeringTableView::for_dataset(&dataset);
        view.filters.insert("x".to_owned(), ">= 10".to_owned());
        view.columns
            .iter_mut()
            .find(|column| column.column_id == "parameters")
            .unwrap()
            .visible = false;
        view.sort.push(EngineeringSortRule {
            column_id: "identifier".to_owned(),
            direction: SortDirection::Descending,
        });

        let export = delimited_text(&dataset, &view, b'\t', true, true).unwrap();

        assert!(export.contains("R7"));
        assert!(!export.contains("C2"));
        assert!(!export.contains("Parameters"));
        assert!(export.contains("X [grid]"));
    }

    #[test]
    fn stores_reject_duplicate_names_and_preserve_defaults() {
        let dataset = dataset();
        let view = EngineeringTableView::for_dataset(&dataset);
        let mut store = EngineeringTableViewStore::default();
        let first = store
            .save(
                "Review",
                EngineeringViewScope::Personal,
                view.clone(),
                true,
                &dataset,
            )
            .unwrap();
        assert!(
            store
                .save(
                    "review",
                    EngineeringViewScope::Personal,
                    view,
                    false,
                    &dataset
                )
                .is_err()
        );
        assert!(store.make_default(&first));
        assert!(store.saved[0].is_default);
    }

    #[test]
    fn selected_rows_and_hidden_columns_are_honored_by_artifacts() {
        let dataset = dataset();
        let mut view = EngineeringTableView::for_dataset(&dataset);
        view.columns
            .iter_mut()
            .find(|column| column.column_id == "parameters")
            .unwrap()
            .visible = false;
        let selected = ["component-7".to_owned()].into_iter().collect();

        let visible =
            delimited_text_selected(&dataset, &view, b',', true, true, false, Some(&selected))
                .unwrap();
        let complete =
            delimited_text_selected(&dataset, &view, b',', true, true, true, Some(&selected))
                .unwrap();

        assert!(visible.contains("R7"));
        assert!(!visible.contains("C2"));
        assert!(!visible.contains("Parameters"));
        assert!(complete.contains("Parameters"));
    }

    #[test]
    fn saved_view_exchange_round_trips_through_validation() {
        let dataset = dataset();
        let view = EngineeringTableView::for_dataset(&dataset);
        let mut source = EngineeringTableViewStore::default();
        let id = source
            .save(
                "Review",
                EngineeringViewScope::Personal,
                view,
                false,
                &dataset,
            )
            .unwrap();
        let json = source.export_view(&id).unwrap();
        let mut target = EngineeringTableViewStore::default();

        target
            .import_view(&json, EngineeringViewScope::Project, &dataset)
            .unwrap();

        assert_eq!(target.saved.len(), 1);
        assert_eq!(target.saved[0].scope, EngineeringViewScope::Project);
    }

    #[test]
    fn binary_formats_have_real_container_signatures() {
        let dataset = dataset();
        let view = EngineeringTableView::for_dataset(&dataset);

        let xlsx = xlsx_bytes(&dataset, &view, true, true, true, false, None).unwrap();
        let parquet = parquet_bytes(&dataset, &view, true, false, None).unwrap();

        assert!(xlsx.starts_with(b"PK"));
        assert!(parquet.starts_with(b"PAR1"));
        assert!(parquet.ends_with(b"PAR1"));
    }
}
