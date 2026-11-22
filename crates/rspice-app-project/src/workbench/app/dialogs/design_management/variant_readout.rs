//! The Variants tab's read-out: the Substitution matrix, every part a
//! variant changes against every variant. It changes nothing; the parts are
//! changed in the inspector with a variant chosen.

use super::widgets::*;
use super::*;
use crate::state::{AssemblyVariantCatalog, CellViewRef, VariantObjectOverride};

/// What a read-out was worked out from; a change to any of it works the
/// read-out out again.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ReadoutBasis {
    page: DesignManagementPage,
    /// Each variant and its revision, so an edit to any of them shows.
    variants: Vec<(AssemblyVariantId, u64)>,
    scope: VariantMatrixScope,
}

/// One part of a read-out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReadoutRow {
    pub(super) object: SchematicObjectKey,
    /// The part's name, or "deleted part #41" for a part no longer drawn.
    part: String,
    /// The cell that holds the part.
    cell: String,
    /// What each column does to the part; empty where it is fitted as the
    /// base design has it.
    cells: Vec<String>,
}

/// A read-out's table: one row per part, and a column per variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VariantReadout {
    basis: ReadoutBasis,
    /// The headings of the columns after Part and Cell.
    columns: Vec<String>,
    pub(super) rows: Vec<ReadoutRow>,
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
    let basis = ReadoutBasis {
        page,
        variants: catalog
            .variants()
            .iter()
            .map(|variant| (variant.id(), variant.revision()))
            .collect(),
        scope: dialog.inputs.matrix_scope,
    };
    if dialog
        .variant_readout
        .as_ref()
        .is_some_and(|readout| readout.basis == basis)
    {
        return;
    }
    let readout = substitution_matrix(catalog, &dialog.owner_key, model, basis);
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
    let mut rows = objects
        .into_iter()
        .map(|object| {
            let (part, cell) = part_and_cell(model, object);
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
            ReadoutRow {
                object: object.clone(),
                part,
                cell,
                cells,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.cell
            .cmp(&right.cell)
            .then_with(|| designator_order(&left.part, &right.part))
    });
    VariantReadout {
        basis,
        columns: variants
            .iter()
            .map(|variant| variant.name().to_owned())
            .collect(),
        rows,
    }
}

/// The part `object` names and the cell that holds it, as the design draws
/// them now.
fn part_and_cell(model: &Model, object: &SchematicObjectKey) -> (String, String) {
    let view_key = object.cell_view_key();
    let content = &model.workspace.content;
    let document = if view_key.eq_ignore_ascii_case(&content.active_schematic_reference().key()) {
        Some(model.schematic.document())
    } else {
        content
            .schematic_buffers
            .get(view_key)
            .map(|buffer| buffer.document())
    };
    let part = document
        .and_then(|document| {
            document
                .components
                .iter()
                .find(|component| component.id == object.object_id())
        })
        .map_or_else(
            || format!("deleted part #{}", object.object_id()),
            |component| component.name.clone(),
        );
    let cell = CellViewRef::from_key(view_key)
        .map_or_else(|| view_key.to_owned(), |reference| reference.cell);
    (part, cell)
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

/// The read-out's list under a heading with its count.
pub(super) fn variant_readout_list(ui: &mut Ui, readout: &VariantReadout) {
    section_header(
        ui,
        "Parts",
        Some(&format!(
            "{} \u{00b7} {}",
            counted(readout.rows.len(), "part"),
            counted(readout.columns.len(), "variant")
        )),
    );
    let weights = [PART_WEIGHT, CELL_WEIGHT]
        .into_iter()
        .chain(readout.columns.iter().map(|_| VARIANT_WEIGHT))
        .collect::<Vec<_>>();
    let headings = ["Part", "Cell"]
        .into_iter()
        .chain(readout.columns.iter().map(String::as_str))
        .collect::<Vec<_>>();
    paint_table_header(ui, &weights, &headings);
    if readout.rows.is_empty() {
        empty_table_row(ui, "No variant changes a part in this scope.");
        return;
    }
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
                paint_table_row(ui, &weights, &values, None);
            }
        });
}
