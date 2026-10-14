//! Slice, pivot, encode and filter a sweep, corner or Monte Carlo family.

use egui::{Context, Id};
use rspice_ui_kit::widgets::{CommandForm, DialogChoice, FieldLine};

/// Dimension keys and the filter, as the reader set them. An empty key is
/// "None" for the encodings that may go unused.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FamilyDraft {
    pub x: String,
    pub family: String,
    pub color: String,
    pub dash: String,
    pub marker: String,
    pub filter: String,
    /// Omit points that did not run, instead of keeping them as not-run.
    pub omit_missing: bool,
}

/// What the form offers, resolved by the host from the family's manifest.
pub struct FamilyChoices {
    /// The pane or analysis the presentation applies to.
    pub scope: String,
    /// `(key, label)` of the dimensions that can be the X axis.
    pub x_dimensions: Vec<(String, String)>,
    /// `(key, label)` of the dimensions that can group or encode traces.
    pub dimensions: Vec<(String, String)>,
    /// How many points the filter keeps, or why it is refused.
    pub points: Result<String, String>,
    /// Why Apply is refused when the filter is not the reason.
    pub refusal: Option<String>,
}

pub fn show(ctx: &Context, draft: &mut FamilyDraft, choices: &FamilyChoices) -> DialogChoice {
    let filter = Id::new("rspice.family.filter");
    let refusal = choices.refusal.as_deref();
    CommandForm::new("Family slicing and pivot", "Apply")
        .context(Some(choices.scope.as_str()))
        .describe(
            "Choose the X axis and family dimension of a swept result, how members are \
             told apart, and which points are shown.",
        )
        .primary_enabled(refusal.is_none() && choices.points.is_ok())
        .status(refusal.map_or(Ok(None), Err))
        .status_lines(2)
        .show(ctx, |rows| {
            dimension_row(
                rows,
                "X axis",
                "x",
                &mut draft.x,
                &choices.x_dimensions,
                false,
            );
            dimension_row(
                rows,
                "Family",
                "family",
                &mut draft.family,
                &choices.dimensions,
                false,
            );
            dimension_row(
                rows,
                "Color",
                "color",
                &mut draft.color,
                &choices.dimensions,
                false,
            );
            dimension_row(
                rows,
                "Dash",
                "dash",
                &mut draft.dash,
                &choices.dimensions,
                true,
            );
            dimension_row(
                rows,
                "Marker",
                "marker",
                &mut draft.marker,
                &choices.dimensions,
                true,
            );
            rows.text(
                "Filter",
                filter,
                &mut draft.filter,
                "temperature in {27,125}",
            );
            rows.field_line(
                Some(filter),
                match &choices.points {
                    Ok(points) => FieldLine::Derived(points),
                    Err(reason) => FieldLine::Error(reason),
                },
            );
            let mut missing = usize::from(draft.omit_missing);
            if rows.segmented(
                "Not run",
                "rspice.family.missing",
                &["Keep", "Omit"],
                &mut missing,
            ) {
                draft.omit_missing = missing == 1;
            }
            None
        })
}

/// One dimension select. `optional` adds a "None" entry, kept as an empty key.
fn dimension_row(
    rows: &mut rspice_ui_kit::widgets::FormRows<'_>,
    label: &str,
    id: &str,
    selected: &mut String,
    dimensions: &[(String, String)],
    optional: bool,
) {
    let mut keys = Vec::with_capacity(dimensions.len() + 1);
    let mut labels = Vec::with_capacity(dimensions.len() + 1);
    if optional {
        keys.push(String::new());
        labels.push("None".to_owned());
    }
    for (key, name) in dimensions {
        keys.push(key.clone());
        labels.push(name.clone());
    }
    let current = keys
        .iter()
        .position(|key| key == selected)
        .map_or_else(String::new, |index| labels[index].clone());
    if let Some(index) = rows.select(label, &format!("rspice.family.{id}"), &current, &labels) {
        *selected = keys[index].clone();
    }
}
