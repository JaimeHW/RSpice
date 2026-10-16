//! Everything about the current finding that will not fit in its row.
//!
//! The grid prints one line per finding, which is what makes a release-scale
//! list readable; this is where the finding that the reader stopped on says
//! the rest. It states the message in full, the rule and producer that raised
//! it, the waiver covering it where one is recorded, where its batch stands
//! against the live design, and the other places it names.
//!
//! Each verb here is drawn only where it leads somewhere: no Follow for a
//! finding with nowhere to go, no Explain where the rule has no prose of its
//! own, and no fix where the producer offered no edits. A control that cannot
//! do what it says is worse than no control.

use std::sync::Arc;

use egui::{Ui, Vec2};

use super::super::{CONSOLE_COLUMN_GAP, CONSOLE_FONT_SIZE, CONSOLE_ROW_MIN_HEIGHT};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Button;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;
use crate::workbench::app_state::diagnostic_store::listing::DiagnosticRow;

/// The height of the verb row above the detail lines.
const ACTION_ROW_HEIGHT: f32 = 24.0;

/// The detail region's share of the page, in lines of its own face.
const MIN_DETAIL_LINES: f32 = 2.0;
const MAX_DETAIL_LINES: f32 = 6.0;
const DETAIL_SHARE: f32 = 0.38;

/// What the reader asked the page to do with the current finding.
#[derive(Debug, Clone, Default)]
pub(super) struct DetailActions {
    pub(super) follow: bool,
    pub(super) copy_row: bool,
    pub(super) copy_listed: bool,
    /// Whether the reader asked for, or asked to stop seeing, the rule's own
    /// words.
    pub(super) explain: bool,
    /// The fix the producer offered, where the reader asked to apply it.
    pub(super) quick_fix: bool,
    /// A related place the reader asked to go to.
    pub(super) related: Option<DiagnosticRoute>,
}

/// What the copy-everything verb copies.
#[derive(Debug, Clone, Copy)]
pub(super) struct Listed {
    /// The findings the filters admit.
    pub(super) count: usize,
    /// Whether some of them are in collapsed groups, and so are copied
    /// without being printed.
    pub(super) collapsed: bool,
}

/// How tall the detail region is in a page of `available` height.
pub(super) fn height(available: f32) -> f32 {
    let lines = (available * DETAIL_SHARE / CONSOLE_ROW_MIN_HEIGHT)
        .floor()
        .clamp(MIN_DETAIL_LINES, MAX_DETAIL_LINES);
    ACTION_ROW_HEIGHT + lines * CONSOLE_ROW_MIN_HEIGHT
}

/// Draw the region for the current finding.
///
/// `refusal` is why its route cannot be followed, where the page checked and
/// found one; `listed` is what the filters are listing, which is what the
/// copy-everything verb promises to copy.
pub(super) fn detail(
    ui: &mut Ui,
    row: &DiagnosticRow<'_>,
    explaining: bool,
    listed: Listed,
    refusal: Option<&str>,
) -> DetailActions {
    let tokens = Tokens::get(ui.ctx());
    let mut actions = DetailActions::default();
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.max_rect().top(),
        egui::Stroke::new(1.0, tokens.color.border),
    );
    ui.add_space(3.0);
    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), ACTION_ROW_HEIGHT),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            if row.route.is_some() && refusal.is_none() {
                actions.follow = Button::new("Follow")
                    .compact()
                    .accessible_label("Follow this finding to where it is")
                    .show(ui)
                    .clicked();
            }
            actions.copy_row = Button::new("Copy row").compact().show(ui).clicked();
            // The count is what the filters list, collapsed groups included,
            // and that is what is copied; where a collapsed group keeps some
            // of it off screen, the verb says so.
            let copies = if listed.collapsed {
                "Copy every listed finding, collapsed groups included"
            } else {
                "Copy every listed finding"
            };
            let copy = Button::new(&format!("Copy {} listed", listed.count))
                .compact()
                .accessible_label(copies)
                .show(ui);
            actions.copy_listed = if listed.collapsed {
                copy.on_hover_text(copies)
            } else {
                copy
            }
            .clicked();
            if !row.rule.is_empty() {
                actions.explain = Button::new(if explaining { "Hide rule" } else { "Explain" })
                    .compact()
                    .accessible_label(if explaining {
                        "Stop stating this rule"
                    } else {
                        "State this rule in its own words"
                    })
                    .show(ui)
                    .clicked();
            }
            // Only the producer that offered the edits can apply them, and the
            // netlist editor is the one that does. A fix offered by anything
            // else would need its own owner to apply it, so nothing is drawn
            // for it rather than a verb that fails when it is pressed.
            if let Some(fix) = applicable_fix(row) {
                actions.quick_fix = Button::new(&fix.label)
                    .compact()
                    .accessible_label(&format!("Apply the fix: {}", fix.label))
                    .show(ui)
                    .clicked();
            }
        },
    );
    egui::ScrollArea::vertical()
        .id_salt("workbench.problems.detail")
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            line(ui, row.message, tokens.color.text, FontWeight::Regular);
            if explaining && !row.rule.is_empty() {
                line(ui, row.rule, tokens.color.text_dim, FontWeight::Regular);
            }
            if !row.details.is_empty() {
                line(ui, row.details, tokens.color.text_dim, FontWeight::Regular);
            }
            line(
                ui,
                &format!("Rule {} · {}", row.code, row.producer),
                tokens.color.text_faint,
                FontWeight::Medium,
            );
            if let Some(waiver) = row.suppression {
                line(
                    ui,
                    &format!(
                        "Waived · rule {} · scope {} · {}",
                        waiver.rule,
                        waiver.scope,
                        if waiver.justification_required {
                            "a justification is required"
                        } else {
                            "no justification is required"
                        }
                    ),
                    tokens.color.text_faint,
                    FontWeight::Medium,
                );
            }
            if let Some(standing) = standing(row) {
                line(ui, &standing, tokens.color.text_faint, FontWeight::Medium);
            }
            if let Some(refusal) = refusal {
                line(ui, refusal, tokens.color.text_faint, FontWeight::Medium);
            }
            for related in row.related {
                let route = related_route(row, related);
                if related_line(ui, related, route.is_some()) {
                    actions.related = route;
                }
            }
        });
    actions
}

/// Where the current finding's batch stands, where that is not simply current,
/// and what moved. "Stale" alone left the reader to guess what to re-run; a
/// current finding is the normal case and says nothing, so the line appears
/// only when it has something to say.
fn standing(row: &DiagnosticRow<'_>) -> Option<String> {
    use crate::workbench::app_state::diagnostic_store::listing::RowStanding;
    match &row.standing {
        RowStanding::Current => None,
        RowStanding::Stale(moved) => Some(format!("Stale · {moved} moved")),
        RowStanding::Unavailable(reason) => Some(format!("Cannot be evaluated · {reason}")),
    }
}

/// The fix the page can actually apply for this finding, if any.
fn applicable_fix<'a>(
    row: &'a DiagnosticRow<'_>,
) -> Option<&'a rspice_app_types::diagnostics::CanonicalQuickFix> {
    matches!(row.route, Some(DiagnosticRoute::NetlistDiagnostic(_)))
        .then(|| row.quick_fixes.first())
        .flatten()
}

/// Where a related place is, and how to get there.
///
/// Only a finding whose own route names a source bundle can say which editor
/// holds the document a related location names; without that the location is
/// still stated, as text, rather than offered as a jump that would land
/// somewhere else.
fn related_route(
    row: &DiagnosticRow<'_>,
    related: &rspice_app_types::diagnostics::DiagnosticRelatedLocation,
) -> Option<DiagnosticRoute> {
    match &row.route {
        Some(DiagnosticRoute::CodeDocument { bundle, .. }) => Some(DiagnosticRoute::CodeDocument {
            bundle: *bundle,
            logical_path: Arc::from(related.document_id.as_str()),
            line: related.range.map(|range| range.start.line),
        }),
        Some(
            DiagnosticRoute::Anchor(_)
            | DiagnosticRoute::NetlistDiagnostic(_)
            | DiagnosticRoute::Repair(_),
        )
        | None => None,
    }
}

/// One related place. Returns true when the reader asked to go there.
fn related_line(
    ui: &mut Ui,
    related: &rspice_app_types::diagnostics::DiagnosticRelatedLocation,
    followable: bool,
) -> bool {
    let tokens = Tokens::get(ui.ctx());
    let position = related.range.map_or_else(String::new, |range| {
        format!(" · line {}", range.start.line + 1)
    });
    let text = format!("{}{position} · {}", related.document_id, related.message);
    if !followable {
        line(ui, &text, tokens.color.text_faint, FontWeight::Regular);
        return false;
    }
    let mut followed = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = CONSOLE_COLUMN_GAP;
        followed = Button::new("Go")
            .compact()
            .accessible_label(&format!("Go to {text}"))
            .show(ui)
            .clicked();
        line(ui, &text, tokens.color.text_dim, FontWeight::Regular);
    });
    followed
}

/// One line of detail, in the page's own face.
fn line(ui: &mut Ui, text: &str, color: egui::Color32, weight: FontWeight) {
    if text.is_empty() {
        return;
    }
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::mono(CONSOLE_FONT_SIZE, weight))
                .color(color),
        )
        .wrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app_state::diagnostic_store::listing::RowStanding;
    use rspice_app_types::diagnostics::{DiagnosticPosition, DiagnosticRange};

    fn related() -> rspice_app_types::diagnostics::DiagnosticRelatedLocation {
        rspice_app_types::diagnostics::DiagnosticRelatedLocation {
            document_id: "models/opamp.va".to_owned(),
            range: Some(DiagnosticRange {
                start: DiagnosticPosition {
                    line: 41,
                    column: 0,
                    byte_offset: 0,
                },
                end: DiagnosticPosition {
                    line: 41,
                    column: 4,
                    byte_offset: 4,
                },
            }),
            message: "first declared here".to_owned(),
        }
    }

    fn row<'a>(route: Option<DiagnosticRoute>) -> DiagnosticRow<'a> {
        DiagnosticRow {
            id: uuid::Uuid::nil(),
            code: "VA-REDECLARED",
            producer: "rspice.veriloga.compiler",
            level: crate::workbench::app_state::diagnostic_store::DiagnosticLevel::Error,
            standing: RowStanding::Current,
            subject: String::new(),
            location: String::new(),
            message: "",
            details: "",
            rule: "",
            related: &[],
            quick_fixes: &[],
            suppression: None,
            route,
        }
    }

    /// A related place is followed through the editor that owns the finding's
    /// own document. A finding whose route names no source bundle has nothing
    /// to resolve the related document against, so the place is stated rather
    /// than offered as a jump that would land somewhere else.
    #[test]
    fn a_related_place_is_followable_only_through_the_bundle_that_owns_it() {
        let bundle = crate::state::ProjectSourceId::new();
        let route = related_route(
            &row(Some(DiagnosticRoute::CodeDocument {
                bundle,
                logical_path: Arc::from("models/top.va"),
                line: Some(7),
            })),
            &related(),
        );
        assert_eq!(
            route,
            Some(DiagnosticRoute::CodeDocument {
                bundle,
                logical_path: Arc::from("models/opamp.va"),
                line: Some(41),
            }),
            "the related document and its own line, in the finding's bundle",
        );

        for elsewhere in [
            None,
            Some(DiagnosticRoute::NetlistDiagnostic(uuid::Uuid::nil())),
            Some(DiagnosticRoute::Repair(
                crate::workbench::state::PreflightRemediation::DesignChecks,
            )),
        ] {
            assert_eq!(
                related_route(&row(elsewhere.clone()), &related()),
                None,
                "{elsewhere:?} names no bundle to resolve a related document in",
            );
        }
    }

    /// Only the netlist editor applies the edits a finding offers, so only a
    /// netlist finding carries a fix control.
    #[test]
    fn a_fix_is_applicable_only_where_its_owner_can_apply_it() {
        let fix = rspice_app_types::diagnostics::CanonicalQuickFix {
            fix_id: "fix".to_owned(),
            label: "Replace with nch".to_owned(),
            preferred: true,
            edits: Vec::new(),
        };
        let fixes = [fix];
        let mut netlist = row(Some(DiagnosticRoute::NetlistDiagnostic(uuid::Uuid::nil())));
        netlist.quick_fixes = &fixes;
        assert_eq!(
            applicable_fix(&netlist).map(|fix| fix.label.as_str()),
            Some("Replace with nch"),
        );

        let mut source = row(Some(DiagnosticRoute::CodeDocument {
            bundle: crate::state::ProjectSourceId::new(),
            logical_path: Arc::from("models/top.va"),
            line: None,
        }));
        source.quick_fixes = &fixes;
        assert!(
            applicable_fix(&source).is_none(),
            "no owner outside the netlist editor can apply these edits yet",
        );
    }
}
