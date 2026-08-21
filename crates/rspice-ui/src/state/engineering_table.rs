//! Universal engineering-table contracts.
//!
//! The view manager is deliberately data-oriented: a table view is a
//! serializable projection over a truthful logical dataset. Rendering,
//! clipboard publication, and export all consume the same projection so a
//! hidden column or typed filter cannot be silently ignored at an artifact
//! boundary.

use std::collections::{BTreeMap, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::state::{ComponentType, SchematicState};

pub const ACTIVE_SCHEMATIC_GRID_ID: &str = "active-schematic-objects";
pub const VIRTUALIZATION_THRESHOLD: usize = 250;
pub const VIRTUALIZATION_OVERSCAN: usize = 24;
pub const ENGINEERING_VIEW_EXCHANGE_SCHEMA: u32 = 1;
pub const MAX_SAVED_ENGINEERING_VIEWS: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum EngineeringFilterGrammar {
    #[default]
    EngineeringValues,
    TextMatching,
}

impl EngineeringFilterGrammar {
    pub const ALL: [Self; 2] = [Self::EngineeringValues, Self::TextMatching];

    pub const fn label(self) -> &'static str {
        match self {
            Self::EngineeringValues => "Engineering values",
            Self::TextMatching => "Text matching",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum EngineeringVirtualizationPolicy {
    #[default]
    Above250Rows,
    Always,
    RenderAll,
}

impl EngineeringVirtualizationPolicy {
    pub const ALL: [Self; 3] = [Self::Above250Rows, Self::Always, Self::RenderAll];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Above250Rows => "Virtualize above 250 rows",
            Self::Always => "Always virtualize",
            Self::RenderAll => "Render all rows",
        }
    }

    pub const fn virtualizes(self, rows: usize) -> bool {
        match self {
            Self::Above250Rows => rows > VIRTUALIZATION_THRESHOLD,
            Self::Always => true,
            Self::RenderAll => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum FrozenIdentifierPolicy {
    #[default]
    FirstVisibleIdentifier,
    TwoLeftColumns,
    None,
}

impl FrozenIdentifierPolicy {
    pub const ALL: [Self; 3] = [
        Self::FirstVisibleIdentifier,
        Self::TwoLeftColumns,
        Self::None,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FirstVisibleIdentifier => "First visible identifier column",
            Self::TwoLeftColumns => "Two left columns",
            Self::None => "None",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineeringSortRule {
    pub column_id: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineeringColumnView {
    pub column_id: String,
    pub visible: bool,
    pub pinned: bool,
    pub width: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineeringTableView {
    pub grid_id: String,
    pub columns: Vec<EngineeringColumnView>,
    pub sort: Vec<EngineeringSortRule>,
    pub filters: BTreeMap<String, String>,
    pub show_filter_row: bool,
    pub filter_grammar: EngineeringFilterGrammar,
    pub virtualization: EngineeringVirtualizationPolicy,
    pub frozen_identifiers: FrozenIdentifierPolicy,
}

impl EngineeringTableView {
    pub fn for_dataset(dataset: &EngineeringDataset) -> Self {
        Self {
            grid_id: dataset.id.clone(),
            columns: dataset
                .columns
                .iter()
                .enumerate()
                .map(|(index, column)| EngineeringColumnView {
                    column_id: column.id.clone(),
                    visible: true,
                    pinned: index == 0,
                    width: if index == 0 { 160 } else { 120 },
                })
                .collect(),
            sort: Vec::new(),
            filters: BTreeMap::new(),
            show_filter_row: false,
            filter_grammar: EngineeringFilterGrammar::EngineeringValues,
            virtualization: EngineeringVirtualizationPolicy::Above250Rows,
            frozen_identifiers: FrozenIdentifierPolicy::FirstVisibleIdentifier,
        }
    }

    pub fn normalize_for(&mut self, dataset: &EngineeringDataset) {
        self.grid_id.clone_from(&dataset.id);
        let valid = dataset
            .columns
            .iter()
            .map(|column| column.id.as_str())
            .collect::<HashSet<_>>();
        self.columns
            .retain(|column| valid.contains(column.column_id.as_str()));
        let retained = self
            .columns
            .iter()
            .map(|column| column.column_id.clone())
            .collect::<HashSet<_>>();
        for (index, column) in dataset.columns.iter().enumerate() {
            if !retained.contains(&column.id) {
                self.columns.push(EngineeringColumnView {
                    column_id: column.id.clone(),
                    visible: true,
                    pinned: index == 0,
                    width: if index == 0 { 160 } else { 120 },
                });
            }
        }
        self.sort
            .retain(|rule| valid.contains(rule.column_id.as_str()));
        self.filters
            .retain(|column, _| valid.contains(column.as_str()));
        for column in &mut self.columns {
            column.width = column.width.clamp(56, 640);
        }
        if !self.columns.iter().any(|column| column.visible)
            && let Some(first) = self.columns.first_mut()
        {
            first.visible = true;
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineeringViewScope {
    #[default]
    Personal,
    Project,
}

impl EngineeringViewScope {
    pub const ALL: [Self; 2] = [Self::Project, Self::Personal];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Personal => "Personal",
            Self::Project => "Project",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedEngineeringTableView {
    pub id: String,
    pub name: String,
    pub scope: EngineeringViewScope,
    pub definition: EngineeringTableView,
    pub is_default: bool,
    pub revision: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EngineeringTableViewStore {
    pub working: HashMap<String, EngineeringTableView>,
    pub saved: Vec<SavedEngineeringTableView>,
    next_revision: u64,
}

impl EngineeringTableViewStore {
    pub fn working_view(&mut self, dataset: &EngineeringDataset) -> EngineeringTableView {
        let view = self
            .working
            .entry(dataset.id.clone())
            .or_insert_with(|| EngineeringTableView::for_dataset(dataset));
        view.normalize_for(dataset);
        view.clone()
    }

    pub fn set_working(&mut self, mut view: EngineeringTableView, dataset: &EngineeringDataset) {
        view.normalize_for(dataset);
        self.working.insert(dataset.id.clone(), view);
    }

    pub fn save(
        &mut self,
        name: &str,
        scope: EngineeringViewScope,
        mut definition: EngineeringTableView,
        make_default: bool,
        dataset: &EngineeringDataset,
    ) -> Result<String, String> {
        if self.saved.len() >= MAX_SAVED_ENGINEERING_VIEWS {
            return Err(format!(
                "The saved-view limit of {MAX_SAVED_ENGINEERING_VIEWS} has been reached."
            ));
        }
        let name = name.trim();
        if name.is_empty() {
            return Err("A saved view name is required.".to_owned());
        }
        if name.chars().count() > 96 || name.chars().any(char::is_control) {
            return Err("Saved view names must be at most 96 printable characters.".to_owned());
        }
        if self.saved.iter().any(|view| {
            view.definition.grid_id == dataset.id && view.name.eq_ignore_ascii_case(name)
        }) {
            return Err(format!(
                "A saved view named “{name}” already exists for this grid."
            ));
        }
        definition.normalize_for(dataset);
        if make_default {
            for view in &mut self.saved {
                if view.definition.grid_id == dataset.id {
                    view.is_default = false;
                }
            }
        }
        self.next_revision = self
            .next_revision
            .max(
                self.saved
                    .iter()
                    .map(|view| view.revision)
                    .max()
                    .unwrap_or_default(),
            )
            .saturating_add(1)
            .max(1);
        let id = format!(
            "{}-{}-{}",
            match scope {
                EngineeringViewScope::Personal => "personal",
                EngineeringViewScope::Project => "project",
            },
            slug(name),
            self.next_revision
        );
        self.saved.push(SavedEngineeringTableView {
            id: id.clone(),
            name: name.to_owned(),
            scope,
            definition,
            is_default: make_default,
            revision: self.next_revision,
        });
        Ok(id)
    }

    pub fn delete(&mut self, id: &str) -> bool {
        let prior = self.saved.len();
        self.saved.retain(|view| view.id != id);
        prior != self.saved.len()
    }

    pub fn make_default(&mut self, id: &str) -> bool {
        let Some(grid_id) = self
            .saved
            .iter()
            .find(|view| view.id == id)
            .map(|view| view.definition.grid_id.clone())
        else {
            return false;
        };
        let mut found = false;
        for view in &mut self.saved {
            if view.definition.grid_id == grid_id {
                view.is_default = view.id == id;
                found |= view.is_default;
            }
        }
        found
    }

    pub fn rename(&mut self, id: &str, name: &str) -> Result<(), String> {
        let name = validated_view_name(name)?;
        let Some(index) = self.saved.iter().position(|view| view.id == id) else {
            return Err("The saved view no longer exists.".to_owned());
        };
        let grid_id = self.saved[index].definition.grid_id.clone();
        if self.saved.iter().enumerate().any(|(candidate, view)| {
            candidate != index
                && view.definition.grid_id == grid_id
                && view.name.eq_ignore_ascii_case(name)
        }) {
            return Err(format!(
                "A saved view named “{name}” already exists for this grid."
            ));
        }
        self.next_revision = self
            .next_revision
            .max(
                self.saved
                    .iter()
                    .map(|view| view.revision)
                    .max()
                    .unwrap_or(0),
            )
            .saturating_add(1);
        self.saved[index].name = name.to_owned();
        self.saved[index].revision = self.next_revision;
        Ok(())
    }

    pub fn duplicate(
        &mut self,
        id: &str,
        name: &str,
        scope: EngineeringViewScope,
        dataset: &EngineeringDataset,
    ) -> Result<String, String> {
        let source = self
            .saved
            .iter()
            .find(|view| view.id == id)
            .ok_or_else(|| "The saved view no longer exists.".to_owned())?
            .definition
            .clone();
        self.save(name, scope, source, false, dataset)
    }

    pub fn export_view(&self, id: &str) -> Result<String, String> {
        let view = self
            .saved
            .iter()
            .find(|view| view.id == id)
            .ok_or_else(|| "The saved view no longer exists.".to_owned())?;
        serde_json::to_string_pretty(&EngineeringViewExchange {
            schema: ENGINEERING_VIEW_EXCHANGE_SCHEMA,
            view: view.clone(),
        })
        .map_err(|error| error.to_string())
    }

    pub fn import_view(
        &mut self,
        source: &str,
        scope: EngineeringViewScope,
        dataset: &EngineeringDataset,
    ) -> Result<String, String> {
        if source.len() > 512 * 1024 {
            return Err("Saved-view import exceeds the 512 KiB limit.".to_owned());
        }
        let exchange: EngineeringViewExchange =
            serde_json::from_str(source).map_err(|error| format!("Invalid saved view: {error}"))?;
        if exchange.schema != ENGINEERING_VIEW_EXCHANGE_SCHEMA {
            return Err(format!(
                "Saved-view schema {} is unsupported; expected {}.",
                exchange.schema, ENGINEERING_VIEW_EXCHANGE_SCHEMA
            ));
        }
        if exchange.view.definition.grid_id != dataset.id {
            return Err(format!(
                "Saved view targets grid “{}”, not the active “{}” grid.",
                exchange.view.definition.grid_id, dataset.id
            ));
        }
        self.save(
            &exchange.view.name,
            scope,
            exchange.view.definition,
            false,
            dataset,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EngineeringViewExchange {
    schema: u32,
    view: SavedEngineeringTableView,
}

fn validated_view_name(name: &str) -> Result<&str, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("A saved view name is required.".to_owned());
    }
    if name.chars().count() > 96 || name.chars().any(char::is_control) {
        return Err("Saved view names must be at most 96 printable characters.".to_owned());
    }
    Ok(name)
}

fn slug(value: &str) -> String {
    let slug = value
        .chars()
        .flat_map(char::to_lowercase)
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();
    slug.trim_matches('-').to_owned()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineeringColumn {
    pub id: String,
    pub label: String,
    pub unit: Option<String>,
    pub identifier: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineeringCell {
    pub display: String,
    pub numeric: Option<f64>,
}

impl EngineeringCell {
    fn text(value: impl Into<String>) -> Self {
        Self {
            display: value.into(),
            numeric: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineeringRow {
    pub stable_id: String,
    pub cells: HashMap<String, EngineeringCell>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineeringDataset {
    pub id: String,
    pub title: String,
    pub source_revision: u64,
    pub columns: Vec<EngineeringColumn>,
    pub rows: Vec<EngineeringRow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineeringProjection {
    pub columns: Vec<EngineeringColumn>,
    pub rows: Vec<EngineeringRow>,
    pub logical_rows: usize,
    pub virtualized: bool,
}

impl EngineeringDataset {
    pub fn active_schematic(schematic: &SchematicState) -> Self {
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
            schematic.components.len()
                + schematic.net_labels.len()
                + schematic.wires.len()
                + schematic.buses.len()
                + schematic.bus_taps.len()
                + schematic.junctions.len(),
        );
        for component in &schematic.components {
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
        for label in &schematic.net_labels {
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
        for wire in &schematic.wires {
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
        for bus in &schematic.buses {
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
        for tap in &schematic.bus_taps {
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
        for junction in &schematic.junctions {
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
        Self {
            id: ACTIVE_SCHEMATIC_GRID_ID.to_owned(),
            title: "Active schematic objects".to_owned(),
            source_revision: schematic.topology_version(),
            columns,
            rows,
        }
    }

    pub fn project(&self, view: &EngineeringTableView) -> EngineeringProjection {
        self.project_selected(view, false, None)
    }

    pub fn project_selected(
        &self,
        view: &EngineeringTableView,
        include_hidden_columns: bool,
        selected_rows: Option<&std::collections::BTreeSet<String>>,
    ) -> EngineeringProjection {
        let mut normalized = view.clone();
        normalized.normalize_for(self);
        let columns_by_id = self
            .columns
            .iter()
            .map(|column| (column.id.as_str(), column))
            .collect::<HashMap<_, _>>();
        let columns = normalized
            .columns
            .iter()
            .filter(|view| include_hidden_columns || view.visible)
            .filter_map(|view| columns_by_id.get(view.column_id.as_str()).cloned())
            .cloned()
            .collect::<Vec<_>>();
        let mut rows = self
            .rows
            .iter()
            .filter(|row| {
                selected_rows.is_none_or(|selected| selected.contains(&row.stable_id))
                    && normalized.filters.iter().all(|(column, query)| {
                        row.cells.get(column).is_some_and(|cell| {
                            filter_matches(cell, query, normalized.filter_grammar)
                        })
                    })
            })
            .cloned()
            .collect::<Vec<_>>();
        rows.sort_by(|left, right| {
            for rule in &normalized.sort {
                let ordering = compare_cells(
                    left.cells.get(&rule.column_id),
                    right.cells.get(&rule.column_id),
                );
                if ordering != std::cmp::Ordering::Equal {
                    return match rule.direction {
                        SortDirection::Ascending => ordering,
                        SortDirection::Descending => ordering.reverse(),
                    };
                }
            }
            left.stable_id.cmp(&right.stable_id)
        });
        EngineeringProjection {
            columns,
            rows,
            logical_rows: self.rows.len(),
            virtualized: normalized.virtualization.virtualizes(self.rows.len()),
        }
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

fn compare_cells(
    left: Option<&EngineeringCell>,
    right: Option<&EngineeringCell>,
) -> std::cmp::Ordering {
    match (left, right) {
        (Some(left), Some(right)) => match (left.numeric, right.numeric) {
            (Some(left), Some(right)) => left.total_cmp(&right),
            _ => natural_key(&left.display).cmp(&natural_key(&right.display)),
        },
        (Some(_), None) => std::cmp::Ordering::Greater,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (None, None) => std::cmp::Ordering::Equal,
    }
}

fn natural_key(value: &str) -> String {
    value.to_lowercase()
}

fn filter_matches(cell: &EngineeringCell, query: &str, grammar: EngineeringFilterGrammar) -> bool {
    let query = query.trim();
    if query.is_empty() {
        return true;
    }
    match grammar {
        EngineeringFilterGrammar::EngineeringValues => {
            if let Some(number) = cell
                .numeric
                .or_else(|| parse_engineering_number(&cell.display))
            {
                if let Some((operator, target)) = parse_comparator(query) {
                    return compare_number(number, operator, target);
                }
                if let Some((lower, upper)) = parse_range(query) {
                    return number >= lower.min(upper) && number <= lower.max(upper);
                }
            }
            cell.display.to_lowercase().contains(&query.to_lowercase())
        }
        EngineeringFilterGrammar::TextMatching => {
            if let Some(exact) = query.strip_prefix('=') {
                return cell.display.eq_ignore_ascii_case(exact.trim());
            }
            if query.starts_with('/') && query.ends_with('/') && query.len() > 2 {
                return regex::RegexBuilder::new(&query[1..query.len() - 1])
                    .case_insensitive(true)
                    .build()
                    .is_ok_and(|regex| regex.is_match(&cell.display));
            }
            cell.display.to_lowercase().contains(&query.to_lowercase())
        }
    }
}

fn parse_comparator(query: &str) -> Option<(&str, f64)> {
    for operator in ["<=", ">=", "<", ">", "="] {
        if let Some(value) = query.strip_prefix(operator) {
            return parse_engineering_number(value).map(|value| (operator, value));
        }
    }
    None
}

fn parse_range(query: &str) -> Option<(f64, f64)> {
    let (lower, upper) = query.split_once("..")?;
    Some((
        parse_engineering_number(lower)?,
        parse_engineering_number(upper)?,
    ))
}

/// Parse one engineering-notation operand of a typed filter — the number in
/// `>= 10k` or either end of `1p..5p` — or the displayed text of a cell that
/// carries no already-parsed number.
///
/// The crate keeps one engineering parser, in [`crate::quantity`], and this is
/// an adapter onto it rather than a second dialect of the same notation. All
/// the adapter does is exchange the parser's message for the `Option` that
/// [`filter_matches`] reads: `None` is how an operand that is not an
/// engineering number reaches the caller, and the caller answers it by
/// matching the query as a substring instead. That fallback is the filter
/// row's whole error surface — `filter_matches` returns a `bool` and has no
/// other channel — so a rejected operand degrades to text matching rather
/// than silently comparing against a fabricated value.
///
/// An unrecognised suffix is a rejection, never a multiplier of 1.0. That old
/// default read the leading digits of anything and scaled by the first letter
/// that followed, so a part number in a value column became a quantity:
/// `1N4148` parsed as 1e-9 and was then compared against the user's
/// threshold, dropping the row on a number nobody had written.
fn parse_engineering_number(value: &str) -> Option<f64> {
    crate::quantity::parse_engineering_value(value).ok()
}

fn compare_number(value: f64, operator: &str, target: f64) -> bool {
    match operator {
        "<" => value < target,
        "<=" => value <= target,
        ">" => value > target,
        ">=" => value >= target,
        "=" => value.total_cmp(&target).is_eq(),
        _ => false,
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
    let mut writer = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(Vec::new());
    if include_headers {
        writer
            .write_record(projection.columns.iter().map(|column| {
                if include_units {
                    column.unit.as_ref().map_or_else(
                        || column.label.clone(),
                        |unit| format!("{} [{}]", column.label, unit),
                    )
                } else {
                    column.label.clone()
                }
            }))
            .map_err(|error| error.to_string())?;
    }
    for row in projection.rows {
        writer
            .write_record(projection.columns.iter().map(|column| {
                row.cells
                    .get(&column.id)
                    .map(|cell| cell.display.as_str())
                    .unwrap_or_default()
            }))
            .map_err(|error| error.to_string())?;
    }
    let bytes = writer.into_inner().map_err(|error| error.to_string())?;
    String::from_utf8(bytes).map_err(|error| error.to_string())
}

pub fn schema_json(
    dataset: &EngineeringDataset,
    view: &EngineeringTableView,
    include_hidden_columns: bool,
    include_metadata: bool,
) -> Result<String, String> {
    let projection = dataset.project_selected(view, include_hidden_columns, None);
    serde_json::to_string_pretty(&serde_json::json!({
        "schema": 1,
        "grid_id": dataset.id,
        "title": dataset.title,
        "source_revision": include_metadata.then_some(dataset.source_revision),
        "logical_rows": dataset.rows.len(),
        "columns": projection.columns.iter().map(|column| serde_json::json!({
            "id": column.id,
            "label": column.label,
            "unit": column.unit,
            "quantity_type": if column.unit.is_some() { "engineering-scalar" } else { "text" },
            "identifier": column.identifier,
        })).collect::<Vec<_>>(),
        "sort": include_metadata.then_some(&view.sort),
        "filters": include_metadata.then_some(&view.filters),
        "filter_grammar": include_metadata.then_some(view.filter_grammar),
        "virtualization": include_metadata.then_some(view.virtualization),
        "frozen_identifiers": include_metadata.then_some(view.frozen_identifiers),
    }))
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
    use rust_xlsxwriter::{Color, Format, Workbook};

    let projection = dataset.project_selected(view, include_hidden_columns, selected_rows);
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet
        .set_name("Engineering table")
        .map_err(|error| error.to_string())?;
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x20282d))
        .set_font_color(Color::RGB(0xd7dbde));
    let row_offset = u32::from(include_headers);
    if include_headers {
        for (column_index, column) in projection.columns.iter().enumerate() {
            let header = if include_units {
                column.unit.as_ref().map_or_else(
                    || column.label.clone(),
                    |unit| format!("{} [{}]", column.label, unit),
                )
            } else {
                column.label.clone()
            };
            worksheet
                .write_string_with_format(0, column_index as u16, &header, &header_format)
                .map_err(|error| error.to_string())?;
        }
        if !projection.columns.is_empty() {
            worksheet
                .autofilter(
                    0,
                    0,
                    projection.rows.len() as u32,
                    projection.columns.len() as u16 - 1,
                )
                .map_err(|error| error.to_string())?;
        }
    }
    for (row_index, row) in projection.rows.iter().enumerate() {
        for (column_index, column) in projection.columns.iter().enumerate() {
            let cell = row.cells.get(&column.id);
            if let Some(number) = cell.and_then(|cell| cell.numeric) {
                worksheet
                    .write_number(row_index as u32 + row_offset, column_index as u16, number)
                    .map_err(|error| error.to_string())?;
            } else {
                worksheet
                    .write_string(
                        row_index as u32 + row_offset,
                        column_index as u16,
                        cell.map(|cell| cell.display.as_str()).unwrap_or_default(),
                    )
                    .map_err(|error| error.to_string())?;
            }
        }
    }
    if include_headers {
        let pinned = view
            .columns
            .iter()
            .filter(|column| column.visible && column.pinned)
            .count()
            .min(projection.columns.len());
        worksheet
            .set_freeze_panes(1, pinned as u16)
            .map_err(|error| error.to_string())?;
    }
    for (index, column) in projection.columns.iter().enumerate() {
        let width = view
            .columns
            .iter()
            .find(|candidate| candidate.column_id == column.id)
            .map_or(120, |candidate| candidate.width);
        worksheet
            .set_column_width(index as u16, f64::from(width) / 7.0)
            .map_err(|error| error.to_string())?;
    }
    if include_metadata {
        let metadata = workbook.add_worksheet();
        metadata
            .set_name("RSpice provenance")
            .map_err(|error| error.to_string())?;
        for (row, (key, value)) in [
            ("Grid", dataset.id.clone()),
            ("Title", dataset.title.clone()),
            ("Source revision", dataset.source_revision.to_string()),
            ("Filter grammar", view.filter_grammar.label().to_owned()),
            ("Virtualization", view.virtualization.label().to_owned()),
        ]
        .into_iter()
        .enumerate()
        {
            metadata
                .write_string(row as u32, 0, key)
                .map_err(|error| error.to_string())?;
            metadata
                .write_string(row as u32, 1, &value)
                .map_err(|error| error.to_string())?;
        }
    }
    workbook.save_to_buffer().map_err(|error| error.to_string())
}

pub fn parquet_bytes(
    dataset: &EngineeringDataset,
    view: &EngineeringTableView,
    include_metadata: bool,
    include_hidden_columns: bool,
    selected_rows: Option<&std::collections::BTreeSet<String>>,
) -> Result<Vec<u8>, String> {
    use std::sync::Arc;

    use arrow_array::{ArrayRef, Float64Array, RecordBatch, StringArray};
    use arrow_schema::{DataType, Field, Schema};
    use parquet::arrow::ArrowWriter;
    use parquet::file::metadata::KeyValue;
    use parquet::file::properties::WriterProperties;

    let projection = dataset.project_selected(view, include_hidden_columns, selected_rows);
    let fields = projection
        .columns
        .iter()
        .map(|column| {
            Field::new(
                &column.id,
                if projection.rows.iter().any(|row| {
                    row.cells
                        .get(&column.id)
                        .is_some_and(|cell| cell.numeric.is_some())
                }) {
                    DataType::Float64
                } else {
                    DataType::Utf8
                },
                true,
            )
            .with_metadata(
                [
                    ("label".to_owned(), column.label.clone()),
                    ("unit".to_owned(), column.unit.clone().unwrap_or_default()),
                ]
                .into_iter()
                .collect(),
            )
        })
        .collect::<Vec<_>>();
    let schema = Arc::new(Schema::new(fields));
    let arrays = projection
        .columns
        .iter()
        .map(|column| {
            if schema
                .field_with_name(&column.id)
                .is_ok_and(|field| field.data_type() == &DataType::Float64)
            {
                Arc::new(Float64Array::from(
                    projection
                        .rows
                        .iter()
                        .map(|row| row.cells.get(&column.id).and_then(|cell| cell.numeric))
                        .collect::<Vec<_>>(),
                )) as ArrayRef
            } else {
                Arc::new(StringArray::from(
                    projection
                        .rows
                        .iter()
                        .map(|row| row.cells.get(&column.id).map(|cell| cell.display.as_str()))
                        .collect::<Vec<_>>(),
                )) as ArrayRef
            }
        })
        .collect::<Vec<_>>();
    let batch = RecordBatch::try_new(schema.clone(), arrays).map_err(|error| error.to_string())?;
    let metadata = include_metadata.then(|| {
        vec![
            KeyValue {
                key: "rspice.grid_id".to_owned(),
                value: Some(dataset.id.clone()),
            },
            KeyValue {
                key: "rspice.source_revision".to_owned(),
                value: Some(dataset.source_revision.to_string()),
            },
            KeyValue {
                key: "rspice.view".to_owned(),
                value: serde_json::to_string(view).ok(),
            },
        ]
    });
    let properties = WriterProperties::builder()
        .set_key_value_metadata(metadata)
        .build();
    let mut writer = ArrowWriter::try_new(Vec::new(), schema, Some(properties))
        .map_err(|error| error.to_string())?;
    writer.write(&batch).map_err(|error| error.to_string())?;
    writer.into_inner().map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, Point};

    fn dataset() -> EngineeringDataset {
        let mut schematic = SchematicState::default();
        schematic.components.push(
            Component::new(7, ComponentType::Resistor, Point::new(20, -10))
                .with_name_value("R7", "10k"),
        );
        schematic.components.push(
            Component::new(2, ComponentType::Capacitor, Point::new(0, 30))
                .with_name_value("C2", "2p"),
        );
        EngineeringDataset::active_schematic(&schematic)
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
    fn typed_grammar_understands_engineering_values_ranges_exact_and_regex() {
        let numeric = EngineeringCell::text("10k");
        assert!(filter_matches(
            &numeric,
            ">= 9k",
            EngineeringFilterGrammar::EngineeringValues
        ));
        assert!(filter_matches(
            &numeric,
            "9k..11k",
            EngineeringFilterGrammar::EngineeringValues
        ));
        assert!(filter_matches(
            &EngineeringCell::text("R10"),
            "=r10",
            EngineeringFilterGrammar::TextMatching
        ));
        assert!(filter_matches(
            &EngineeringCell::text("R10"),
            "/^r\\d+$/",
            EngineeringFilterGrammar::TextMatching
        ));
    }

    #[test]
    fn filter_operands_read_the_shared_spice_suffix_ladder() {
        for (text, expected) in [
            ("1.5k", 1.5e3),
            ("2.2u", 2.2e-6),
            ("2.2\u{00B5}", 2.2e-6),
            ("2.2\u{03BC}", 2.2e-6),
            ("3meg", 3e6),
            ("3Meg", 3e6),
            ("3m", 3e-3),
            ("10", 10.0),
        ] {
            let parsed = parse_engineering_number(text)
                .unwrap_or_else(|| panic!("{text} is an engineering number"));
            assert!(
                (parsed - expected).abs() <= expected.abs() * 1e-12,
                "{text} parsed as {parsed}, expected {expected}"
            );
        }
    }

    #[test]
    fn an_unrecognised_suffix_is_rejected_rather_than_scaled_by_one() {
        for text in ["12x", "1N4148", "5 volts", "1kohm"] {
            assert_eq!(
                parse_engineering_number(text),
                None,
                "{text} carries no engineering suffix and must not parse"
            );
        }
    }

    #[test]
    fn a_part_number_cell_falls_through_to_substring_matching() {
        // The value column holds authored text. `1N4148` once parsed as
        // 1e-9, so `>= 1` excluded the row; the rejection now sends the cell
        // to the substring arm, where the query matches what is displayed.
        let part = EngineeringCell::text("1N4148");
        assert!(!filter_matches(
            &part,
            ">= 1",
            EngineeringFilterGrammar::EngineeringValues
        ));
        assert!(filter_matches(
            &part,
            "4148",
            EngineeringFilterGrammar::EngineeringValues
        ));
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
