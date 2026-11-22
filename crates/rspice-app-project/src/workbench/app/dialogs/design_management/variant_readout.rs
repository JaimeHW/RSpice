//! The Variants tab's read-outs: the Substitution matrix, every part a
//! variant changes against every variant, and Compare, every part two
//! choices treat differently. Neither changes anything; parts are changed in
//! the inspector with a variant chosen.

use super::widgets::*;
use super::*;
use crate::state::{AssemblyVariantCatalog, CellViewRef, VariantObjectOverride};
use crate::workbench::app_state::model::BASE_DESIGN;

/// What a read-out was worked out from; a change to any of it works the
/// read-out out again.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ReadoutBasis {
    page: DesignManagementPage,
    /// Each variant and its revision, so an edit to any of them shows.
    variants: Vec<(AssemblyVariantId, u64)>,
    scope: VariantMatrixScope,
    reference: Option<AssemblyVariantId>,
    comparison: Option<AssemblyVariantId>,
    classes: VariantDifferenceClasses,
}

/// One part of a read-out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReadoutRow {
    pub(super) object: SchematicObjectKey,
    /// The part's name, or "deleted part #41" for a part no longer drawn.
    part: String,
    /// The cell that holds the part.
    cell: String,
    /// What each column does to the part; empty in the matrix where it is
    /// fitted as the base design has it.
    cells: Vec<String>,
}

/// A read-out's table: one row per part, and a column per variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VariantReadout {
    basis: ReadoutBasis,
    /// The list's heading and the count beside it.
    heading: &'static str,
    count: String,
    /// The headings of the columns after Part and Cell; none where the
    /// read-out is a count alone.
    columns: Vec<String>,
    pub(super) rows: Vec<ReadoutRow>,
    /// What the list says while it has no row.
    empty: String,
    /// Whether a double-click on a row shows the part in Design.
    navigable: bool,
}

/// Work the open read-out page's table out again when what it reads has
/// changed since it was last worked out.
pub(super) fn refresh_variant_readout(dialog: &mut DesignManagementDialogState, model: &Model) {
    let page = dialog.page;
    let Some(draft) = dialog.draft.as_ref().filter(|_| page.read_out()) else {
        dialog.variant_readout = None;
        return;
    };
    let catalog = draft.variants();
    let inputs = &dialog.inputs;
    let basis = ReadoutBasis {
        page,
        variants: catalog
            .variants()
            .iter()
            .map(|variant| (variant.id(), variant.revision()))
            .collect(),
        scope: inputs.matrix_scope,
        reference: inputs.compare_reference,
        comparison: inputs.compare_target,
        classes: inputs.difference_classes,
    };
    if dialog
        .variant_readout
        .as_ref()
        .is_some_and(|readout| readout.basis == basis)
    {
        return;
    }
    let readout = if page == DesignManagementPage::CompareVariants {
        comparison(catalog, model, basis)
    } else {
        substitution_matrix(catalog, &dialog.owner_key, model, basis)
    };
    dialog.variant_readout = Some(readout);
}

/// Every part any variant changes, in scope, against every variant: what
/// the variant does to it, marked where the variant takes it from its
/// parent.
fn substitution_matrix(
    catalog: &AssemblyVariantCatalog,
    owner_key: &str,
    model: &Model,
    basis: ReadoutBasis,
) -> VariantReadout {
    let variants = catalog.variants();
    let resolved = variants
        .iter()
        .map(|variant| {
            catalog
                .resolve(variant.id())
                .map(|resolved| resolved.overrides)
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();
    let in_scope = |object: &SchematicObjectKey| {
        basis.scope == VariantMatrixScope::AllControlledInstances
            || object.cell_view_key() == owner_key
    };
    let objects = resolved
        .iter()
        .flat_map(BTreeMap::keys)
        .filter(|object| in_scope(object))
        .collect::<BTreeSet<_>>();
    let rows = sorted(
        objects
            .into_iter()
            .map(|object| {
                let cells = variants
                    .iter()
                    .zip(&resolved)
                    .map(|(variant, overrides)| {
                        overrides.get(object).map_or_else(String::new, |change| {
                            let label = change_label(change, true);
                            if variant.definition().overrides.contains_key(object) {
                                label
                            } else {
                                format!("{label} (inherited)")
                            }
                        })
                    })
                    .collect();
                row(model, object, cells)
            })
            .collect(),
    );
    VariantReadout {
        basis,
        heading: "Parts",
        count: format!(
            "{} \u{00b7} {}",
            counted(rows.len(), "part"),
            counted(variants.len(), "variant")
        ),
        columns: variants
            .iter()
            .map(|variant| variant.name().to_owned())
            .collect(),
        rows,
        empty: "No variant changes a part in this scope.".to_owned(),
        navigable: false,
    }
}

/// Every part the Reference and the Comparison treat differently, and what
/// each does to it; or, for connectivity, how many nets join different pins.
fn comparison(
    catalog: &AssemblyVariantCatalog,
    model: &Model,
    basis: ReadoutBasis,
) -> VariantReadout {
    let name = |id: Option<AssemblyVariantId>| {
        id.and_then(|id| catalog.find(id))
            .map_or(BASE_DESIGN, |variant| variant.name())
            .to_owned()
    };
    let columns = vec![name(basis.reference), name(basis.comparison)];
    let mut readout = VariantReadout {
        heading: "Differences",
        count: String::new(),
        columns,
        rows: Vec::new(),
        empty: String::new(),
        navigable: true,
        basis,
    };
    let (reference, comparison) = (readout.basis.reference, readout.basis.comparison);
    if reference == comparison {
        readout.empty = "Choose two different designs to compare.".to_owned();
        return readout;
    }
    if readout.basis.classes == VariantDifferenceClasses::ConnectivityOnly {
        readout.heading = "Connectivity";
        readout.columns.clear();
        readout.navigable = false;
        match super::operations::connectivity_difference_count(
            model, catalog, reference, comparison,
        ) {
            Ok(count) => {
                readout.count = counted(count, "difference");
                readout.empty = if count == 0 {
                    "Every pin joins the same net in both designs."
                } else {
                    "Some pins join different nets in the two designs."
                }
                .to_owned();
            }
            Err(error) => readout.empty = error,
        }
        return readout;
    }
    match catalog.compare(reference, comparison) {
        Ok(found) => {
            let fitted = "Fitted".to_owned();
            let label = |change: Option<&VariantObjectOverride>| {
                change.map_or_else(|| fitted.clone(), |change| change_label(change, false))
            };
            readout.rows = sorted(
                found
                    .differences
                    .iter()
                    .map(|difference| {
                        let cells = vec![
                            label(difference.reference.as_ref()),
                            label(difference.comparison.as_ref()),
                        ];
                        row(model, &difference.object, cells)
                    })
                    .collect(),
            );
            readout.count = counted(readout.rows.len(), "difference");
            readout.empty = "The two designs fit every part alike.".to_owned();
        }
        Err(error) => readout.empty = error.to_string(),
    }
    readout
}

/// A read-out's row for `object`, named as the design draws it now.
fn row(model: &Model, object: &SchematicObjectKey, cells: Vec<String>) -> ReadoutRow {
    let (part, cell) = part_and_cell(model, object);
    ReadoutRow {
        object: object.clone(),
        part,
        cell,
        cells,
    }
}

/// `rows` by cell, then by name in the order a reader counts them.
fn sorted(mut rows: Vec<ReadoutRow>) -> Vec<ReadoutRow> {
    rows.sort_by(|left, right| {
        left.cell
            .cmp(&right.cell)
            .then_with(|| designator_order(&left.part, &right.part))
    });
    rows
}

/// The part `object` names and the cell that holds it, as the design draws
/// them now.
fn part_and_cell(model: &Model, object: &SchematicObjectKey) -> (String, String) {
    let view_key = object.cell_view_key();
    let part = drawn_part(model, object).map_or_else(
        || format!("deleted part #{}", object.object_id()),
        |component| component.name.clone(),
    );
    let cell = CellViewRef::from_key(view_key)
        .map_or_else(|| view_key.to_owned(), |reference| reference.cell);
    (part, cell)
}

/// The part `object` names, as the design draws it now.
fn drawn_part<'a>(
    model: &'a Model,
    object: &SchematicObjectKey,
) -> Option<&'a crate::state::Component> {
    let view_key = object.cell_view_key();
    let content = &model.workspace.content;
    let document = if view_key.eq_ignore_ascii_case(&content.active_schematic_reference().key()) {
        Some(model.schematic.document())
    } else {
        content
            .schematic_buffers
            .get(view_key)
            .map(|buffer| buffer.document())
    }?;
    document
        .components
        .iter()
        .find(|component| component.id == object.object_id())
}

/// Where Design shows the part `object` names: its cell view, the sheet
/// that draws it, centred on it and selected. `None` once the part is gone.
pub(super) fn part_place(
    model: &Model,
    object: &SchematicObjectKey,
) -> Option<crate::workbench::framework::requests::ShowOnSchematic> {
    let view_key = object.cell_view_key();
    let point = drawn_part(model, object)?.pos;
    let cell_view = CellViewRef::from_key(view_key)?;
    let sheet = model
        .workspace
        .content
        .design_management
        .sheet_catalog(view_key)
        .filter(|catalog| catalog.sheets().len() > 1)
        .and_then(|catalog| catalog.sheet_for_object(object.object_id()));
    let mut selection = rspice_design::schematic::selection::Selection::default();
    selection.select_component(object.object_id());
    Some(crate::workbench::framework::requests::ShowOnSchematic {
        cell_view,
        sheet,
        point,
        selection,
    })
}

/// What `change` does to a part, in a cell's few words. `short` names a
/// part left off "DNP", as a matrix cell has room for.
pub(super) fn change_label(change: &VariantObjectOverride, short: bool) -> String {
    match change {
        VariantObjectOverride::DoNotPopulate { .. } if short => "DNP".to_owned(),
        VariantObjectOverride::DoNotPopulate { .. } => "Do not populate".to_owned(),
        VariantObjectOverride::Fitted {
            value_override: None,
        } => "Fitted".to_owned(),
        VariantObjectOverride::Fitted {
            value_override: Some(value),
        } => format!("Value {value}"),
        VariantObjectOverride::Substitute { replacement } => {
            let mut label = format!("{}/{}", replacement.library, replacement.cell);
            for detail in [&replacement.value_override, &replacement.model_section]
                .into_iter()
                .flatten()
            {
                label.push_str(" \u{00b7} ");
                label.push_str(detail);
            }
            label
        }
    }
}

/// Reference designators in the order a reader counts them: `R2` before
/// `R10`.
fn designator_order(left: &str, right: &str) -> std::cmp::Ordering {
    let split = |name: &str| {
        let digits = name.len() - name.chars().rev().take_while(char::is_ascii_digit).count();
        let (prefix, number) = name.split_at(digits);
        (prefix.to_owned(), number.parse::<u64>().ok())
    };
    split(left).cmp(&split(right)).then_with(|| left.cmp(right))
}

/// The share of the list Part and Cell take, and each variant's share.
const PART_WEIGHT: f32 = 1.0;
const CELL_WEIGHT: f32 = 0.9;
const VARIANT_WEIGHT: f32 = 1.5;
const READOUT_ROW_HEIGHT: f32 = 29.0;
/// How tall the list stands before it scrolls.
const READOUT_LIST_HEIGHT: f32 = 12.0 * READOUT_ROW_HEIGHT;

/// The read-out's list under a heading with its count. Returns the part
/// whose row was double-clicked, where a row can be.
pub(super) fn variant_readout_list(
    ui: &mut Ui,
    readout: &VariantReadout,
) -> Option<SchematicObjectKey> {
    section_header(
        ui,
        readout.heading,
        (!readout.count.is_empty()).then_some(readout.count.as_str()),
    );
    let weights = [PART_WEIGHT, CELL_WEIGHT]
        .into_iter()
        .chain(readout.columns.iter().map(|_| VARIANT_WEIGHT))
        .collect::<Vec<_>>();
    if !readout.columns.is_empty() {
        let headings = ["Part", "Cell"]
            .into_iter()
            .chain(readout.columns.iter().map(String::as_str))
            .collect::<Vec<_>>();
        paint_table_header(ui, &weights, &headings);
    }
    if readout.rows.is_empty() {
        empty_table_row(ui, &readout.empty);
        return None;
    }
    let sense = if readout.navigable {
        Sense::click()
    } else {
        Sense::hover()
    };
    let shown_in = readout.columns.last().cloned().unwrap_or_default();
    let mut opened = None;
    egui::ScrollArea::vertical()
        .id_salt("variant-readout")
        .max_height(READOUT_LIST_HEIGHT)
        .auto_shrink([false, true])
        .show_rows(ui, READOUT_ROW_HEIGHT, readout.rows.len(), |ui, rows| {
            for index in rows {
                let row = &readout.rows[index];
                let values = [row.part.clone(), row.cell.clone()]
                    .into_iter()
                    .chain(row.cells.iter().cloned())
                    .collect::<Vec<_>>();
                let response = paint_table_row_sensed(ui, &weights, &values, None, sense);
                if readout.navigable {
                    // A row that opens its part is a control: it names itself
                    // to assistive technology, shows focus, and opens on Enter
                    // as on a double-click.
                    let name = format!("Show {} in {shown_in}", row.part);
                    response.widget_info(|| {
                        egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &name)
                    });
                    rspice_ui_kit::theme::paint_focus_ring(ui, &response, response.rect);
                    let entered = response.has_focus()
                        && ui.input(|input| input.key_pressed(egui::Key::Enter));
                    let response = response
                        .on_hover_text(format!("Double-click to show {} in {shown_in}", row.part));
                    if response.double_clicked() || entered {
                        opened = Some(row.object.clone());
                    }
                }
            }
        });
    opened
}
