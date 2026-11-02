//! The small forms the result viewer opens over a project result document.
//!
//! Each form is a [`rspice_ui_kit::widgets::CommandForm`]: a few labelled
//! rows, Cancel, and one primary action. The draft a form edits lives here, in
//! the host's runtime state. The choices a form offers and the transaction it
//! commits stay with the host, which resolves both against the retained
//! datasets every frame.

pub mod add_pane;
pub mod compare;
pub mod family;
pub mod measurement;
pub mod properties;

/// The one result-document form open at a time.
#[derive(Debug, Clone, PartialEq)]
pub enum ResultDocumentDialog {
    AddPane(add_pane::AddPaneDraft),
    /// Compare datasets also opens over a quick view, where its outcome is
    /// shown and not kept.
    Compare(compare::CompareDraft),
    /// Add measurement also opens over a quick view, where it evaluates the
    /// expression and refuses to keep it.
    Measurement(measurement::MeasurementDraft),
    /// Family slicing keeps its policy with the selected pane over a
    /// document, and with the analysis for the session over a quick view.
    Family(family::FamilyDraft),
    /// Plot properties set the document's digits and phase over a document,
    /// and the reader's own over a quick view.
    Properties(properties::PropertiesDraft),
}

/// Option labels for a select row, and the label of the selected entry.
fn labels_and_selection<K: PartialEq + Copy>(
    entries: &[(K, String)],
    selected: Option<K>,
) -> (Vec<String>, String) {
    let labels = entries.iter().map(|(_, label)| label.clone()).collect();
    let current = selected
        .and_then(|key| entries.iter().find(|(candidate, _)| *candidate == key))
        .map_or_else(String::new, |(_, label)| label.clone());
    (labels, current)
}

#[cfg(test)]
mod tests {
    use egui::accesskit::{Rect, Role};

    /// Every control `show` publishes, by name, once its form has settled.
    fn controls(mut show: impl FnMut(&egui::Context)) -> Vec<(String, Role, Rect)> {
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| show(ui.ctx()),
            );
            nodes.extend(
                output
                    .platform_output
                    .accesskit_update
                    .expect("AccessKit tree update")
                    .nodes,
            );
        }
        nodes
            .into_values()
            .filter(|node| node.role() != Role::Label)
            .filter_map(|node| Some((node.label()?.to_owned(), node.role(), node.bounds()?)))
            .collect()
    }

    /// `segments` split the control column `reference` spans into equal
    /// parts.
    fn assert_equal_segments(
        controls: &[(String, Role, Rect)],
        reference: &str,
        segments: &[&str],
    ) {
        let find = |name: &str| {
            controls
                .iter()
                .find(|(label, _, _)| label == name)
                .map(|(_, _, bounds)| *bounds)
                .unwrap_or_else(|| panic!("{name} is published"))
        };
        let column = find(reference);
        let parts: Vec<Rect> = segments.iter().map(|segment| find(segment)).collect();
        let widths: Vec<f64> = parts.iter().map(|part| part.x1 - part.x0).collect();
        let (first, last) = (parts[0], parts[parts.len() - 1]);
        assert!(
            (first.x0 - column.x0).abs() < 1.0 && (last.x1 - column.x1).abs() < 1.0,
            "{segments:?} run {}..{} against {reference} {}..{}",
            first.x0,
            last.x1,
            column.x0,
            column.x1
        );
        assert!(
            widths.iter().all(|width| (width - widths[0]).abs() < 1.0),
            "{segments:?} are {widths:?} wide"
        );
    }

    /// Compare's alignment and Family's not-run choice split the control
    /// column into equal segments, as every other segmented row in a form
    /// does.
    #[test]
    fn segmented_rows_split_the_control_column_equally() {
        let choices = super::compare::CompareChoices {
            candidate: "tran".to_owned(),
            baselines: Vec::new(),
            signals: vec!["V(out)".to_owned()],
            refusal: None,
        };
        let mut draft = super::compare::CompareDraft::default();
        let compare = controls(|ctx| {
            super::compare::show(ctx, &mut draft, &choices);
        });
        assert_equal_segments(
            &compare,
            "Baseline",
            &["X value", "Threshold", "Correlation"],
        );

        let choices = super::family::FamilyChoices {
            scope: "Pane 1".to_owned(),
            x_dimensions: vec![("temperature".to_owned(), "Temperature".to_owned())],
            dimensions: vec![("corner".to_owned(), "Corner".to_owned())],
            points: Ok("4 points".to_owned()),
            refusal: None,
        };
        let mut draft = super::family::FamilyDraft::default();
        let family = controls(|ctx| {
            super::family::show(ctx, &mut draft, &choices);
        });
        assert_equal_segments(&family, "Filter", &["Keep", "Omit"]);
    }
}
