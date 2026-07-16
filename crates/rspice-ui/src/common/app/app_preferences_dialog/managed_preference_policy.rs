//! Resolved preference-policy review specified by the Preferences mockup.
//!
//! This is a read-only projection of policy that is actually enforced by the
//! current runtime. It never invents organization membership or a managed
//! override that RSpice has not received.

use std::sync::Arc;

use egui::{Align, Context, Galley, Layout, Rect, Response, Sense, Stroke, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::{ChoicePreference, ScalarPreference, TogglePreference};

use super::AppState;
use super::preferences_shell::PreferenceCategory;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedPolicyRow {
    setting: String,
    resolved_value: String,
    source: String,
    override_state: String,
    reason: String,
    locked: bool,
}

impl ResolvedPolicyRow {
    fn user(setting: &str, resolved_value: impl Into<String>, source: &str, reason: &str) -> Self {
        Self {
            setting: setting.to_owned(),
            resolved_value: resolved_value.into(),
            source: source.to_owned(),
            override_state: "allowed".to_owned(),
            reason: reason.to_owned(),
            locked: false,
        }
    }
}

const POLICY_TABLE_NARROW_BREAKPOINT: f32 = 820.0;
const POLICY_TABLE_NARROW_MIN_WIDTH: f32 = 660.0;
const POLICY_NOTE_NARROW_BREAKPOINT: f32 = 760.0;
const POLICY_HEADER_HEIGHT: f32 = 27.0;
const POLICY_ROW_HEIGHT: f32 = 28.0;
const POLICY_COLUMN_COUNT: usize = 5;
const POLICY_HEADERS: [&str; POLICY_COLUMN_COUNT] = [
    "Setting",
    "Resolved value",
    "Source",
    "User override",
    "Reason",
];

pub(super) fn render(ctx: &Context, state: &mut AppState, category: PreferenceCategory) {
    if !state.dialogs.managed_preference_policy_open {
        return;
    }
    let rows = resolved_policy_rows(category, state);
    let choice = Dialog::new(
        "PREFERENCES \u{00b7} RESOLVED POLICY \u{00b7} AUDITABLE",
        "Resolved preference policy",
        "Close",
    )
    .description(
        "Review each resolved preference, its authority source, override state, and enforcement reason.",
    )
    .size(DialogSize::Transaction)
    .flush_body()
    .show(ctx, |ui| {
        if rows.is_empty() {
            render_empty_policy(ui);
        } else {
            render_policy_table(ui, &rows);
        }
        render_notes(ui);
    });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        state.dialogs.managed_preference_policy_open = false;
    }
}

fn render_empty_policy(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        [ui.available_width(), 48.0],
        egui::Label::new(
            egui::RichText::new("No runtime-backed preferences are exposed for this category.")
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .wrap(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            true,
            "No runtime-backed preferences are exposed for this category",
        )
    });
}

fn render_policy_table(ui: &mut Ui, rows: &[ResolvedPolicyRow]) {
    let narrow = ui.ctx().content_rect().width() <= POLICY_TABLE_NARROW_BREAKPOINT;
    let viewport_width = ui.available_width().max(1.0);
    let mut table_width = viewport_width;
    let table_height = POLICY_HEADER_HEIGHT + POLICY_ROW_HEIGHT * rows.len() as f32;
    let response = ui
        .allocate_ui_with_layout(
            vec2(ui.available_width(), table_height),
            Layout::top_down(Align::Min),
            |ui| {
                egui::ScrollArea::horizontal()
                    .id_salt("preferences.resolved-policy.scroll")
                    .auto_shrink([false, true])
                    .max_height(table_height)
                    .min_scrolled_height(table_height)
                    .show(ui, |ui| {
                        table_width = if narrow {
                            viewport_width.max(POLICY_TABLE_NARROW_MIN_WIDTH)
                        } else {
                            viewport_width
                        };
                        ui.set_min_width(table_width);
                        ui.spacing_mut().item_spacing.y = 0.0;
                        policy_header_row(ui, table_width);
                        for row in rows {
                            policy_data_row(ui, table_width, row);
                        }
                    });
            },
        )
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label("Resolved preference policy");
        node.set_bounds(egui::accesskit::Rect {
            x0: f64::from(response.rect.left()),
            y0: f64::from(response.rect.top()),
            x1: f64::from(response.rect.left() + table_width),
            y1: f64::from(response.rect.top() + table_height),
        });
    });
}

fn policy_header_row(ui: &mut Ui, table_width: f32) {
    let column_width = table_width / POLICY_COLUMN_COUNT as f32;
    let response = ui
        .allocate_ui_with_layout(
            vec2(table_width, POLICY_HEADER_HEIGHT),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                for header in POLICY_HEADERS {
                    policy_cell(
                        ui,
                        header,
                        column_width,
                        POLICY_HEADER_HEIGHT,
                        true,
                        PolicyCellTone::Normal,
                    );
                }
            },
        )
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label("Policy table headers");
    });
}

fn policy_data_row(ui: &mut Ui, table_width: f32, row: &ResolvedPolicyRow) {
    let column_width = table_width / POLICY_COLUMN_COUNT as f32;
    let cells = [
        (row.setting.as_str(), PolicyCellTone::Normal),
        (row.resolved_value.as_str(), PolicyCellTone::Normal),
        (row.source.as_str(), PolicyCellTone::Normal),
        (
            row.override_state.as_str(),
            if row.locked {
                PolicyCellTone::Locked
            } else {
                PolicyCellTone::Allowed
            },
        ),
        (row.reason.as_str(), PolicyCellTone::Normal),
    ];
    let response = ui
        .allocate_ui_with_layout(
            vec2(table_width, POLICY_ROW_HEIGHT),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                for (text, tone) in cells {
                    policy_cell(ui, text, column_width, POLICY_ROW_HEIGHT, false, tone);
                }
            },
        )
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(row.setting.as_str());
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PolicyCellTone {
    Normal,
    Locked,
    Allowed,
}

fn policy_cell(
    ui: &mut Ui,
    text: &str,
    width: f32,
    height: f32,
    header: bool,
    tone: PolicyCellTone,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(
            1.0,
            if header {
                t.color.border
            } else {
                t.color.border.gamma_multiply(0.75)
            },
        ),
    );
    let color = policy_cell_color(&t, header, tone);
    let font = theme::sans(
        tokens::FS_0,
        if header {
            FontWeight::Medium
        } else {
            FontWeight::Regular
        },
    );
    let clip = rect.shrink2(vec2(8.0, 0.0));
    let galley = if header {
        let mut job = egui::text::LayoutJob::default();
        job.append(
            &text.to_uppercase(),
            0.0,
            egui::TextFormat {
                font_id: font,
                color,
                extra_letter_spacing: tokens::FS_0 * 0.04,
                ..Default::default()
            },
        );
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    } else {
        ellipsized_galley(ui, text, font, color, clip.width())
    };
    ui.painter().with_clip_rect(clip).galley(
        egui::pos2(clip.left(), rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(if header {
            egui::accesskit::Role::ColumnHeader
        } else {
            egui::accesskit::Role::Cell
        });
        node.set_label(text);
        node.set_bounds(access_bounds(rect));
    });
    response
}

fn policy_cell_color(tokens: &Tokens, header: bool, tone: PolicyCellTone) -> egui::Color32 {
    match tone {
        PolicyCellTone::Normal => {
            if header {
                tokens.color.text_faint
            } else {
                tokens.color.text_dim
            }
        }
        PolicyCellTone::Locked => tokens.color.err,
        PolicyCellTone::Allowed => tokens.color.ok,
    }
}

fn ellipsized_galley(
    ui: &mut Ui,
    text: &str,
    font: egui::FontId,
    color: egui::Color32,
    max_width: f32,
) -> Arc<Galley> {
    let full = ui.fonts_mut(|fonts| fonts.layout_no_wrap(text.to_owned(), font.clone(), color));
    if full.size().x <= max_width {
        return full;
    }
    let characters = text.chars().collect::<Vec<_>>();
    let mut low = 0;
    let mut high = characters.len();
    while low < high {
        let middle = (low + high).div_ceil(2);
        let candidate = characters[..middle]
            .iter()
            .chain(['\u{2026}'].iter())
            .collect::<String>();
        let fits = ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(candidate, font.clone(), color)
                .size()
                .x
                <= max_width
        });
        if fits {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    let candidate = characters[..low]
        .iter()
        .chain(['\u{2026}'].iter())
        .collect::<String>();
    ui.fonts_mut(|fonts| fonts.layout_no_wrap(candidate, font, color))
}

fn access_bounds(rect: Rect) -> egui::accesskit::Rect {
    egui::accesskit::Rect {
        x0: f64::from(rect.left()),
        y0: f64::from(rect.top()),
        x1: f64::from(rect.right()),
        y1: f64::from(rect.bottom()),
    }
}

fn option_label(index: usize, options: &[&str]) -> String {
    options
        .get(index)
        .copied()
        .unwrap_or("Unknown value (blocked)")
        .to_owned()
}

fn choice_policy_row(
    state: &AppState,
    setting: &str,
    key: ChoicePreference,
    options: &[&str],
    source: &str,
    reason: &str,
) -> ResolvedPolicyRow {
    ResolvedPolicyRow::user(
        setting,
        option_label(state.ui.preferences.choice(key), options),
        source,
        reason,
    )
}

fn toggle_policy_row(
    state: &AppState,
    setting: &str,
    key: TogglePreference,
    source: &str,
    reason: &str,
) -> ResolvedPolicyRow {
    ResolvedPolicyRow::user(
        setting,
        if state.ui.preferences.toggle(key) {
            "Enabled"
        } else {
            "Disabled"
        },
        source,
        reason,
    )
}

fn scalar_policy_row(
    state: &AppState,
    setting: &str,
    key: ScalarPreference,
    suffix: &str,
    source: &str,
    reason: &str,
) -> ResolvedPolicyRow {
    ResolvedPolicyRow::user(
        setting,
        format!("{}{}", state.ui.preferences.scalar(key), suffix),
        source,
        reason,
    )
}

fn resolved_policy_rows(category: PreferenceCategory, state: &AppState) -> Vec<ResolvedPolicyRow> {
    match category {
        PreferenceCategory::Appearance => vec![
            ResolvedPolicyRow::user(
                "Color mode",
                state.ui.theme.mode.label(),
                "User profile",
                "interface presentation",
            ),
            ResolvedPolicyRow::user(
                "Interface density",
                state.ui.theme.density.label(),
                "User profile",
                "device presentation",
            ),
            ResolvedPolicyRow::user(
                "Color-safe traces",
                if state.ui.theme.colorblind_traces {
                    "Enabled"
                } else {
                    "Disabled"
                },
                "User profile",
                "trace accessibility",
            ),
            ResolvedPolicyRow::user(
                "Canvas contrast",
                format!("{}%", state.ui.theme.canvas_contrast),
                "User profile",
                "engineering canvas",
            ),
            ResolvedPolicyRow::user(
                "Engineering canvas theme",
                state.ui.theme.canvas_theme.label(),
                "User profile",
                "engineering canvas",
            ),
        ],
        PreferenceCategory::Workspace => {
            let mut rows = vec![ResolvedPolicyRow::user(
                "Engineering profile",
                state.workbench.engineering_profile.label(),
                "Device local",
                "workspace capability scope",
            )];
            if let Some(workspace) = state.ui.preferences.workspace() {
                let preset = ["Engineering", "Canvas", "Diagnostics"][workspace.preset().index()];
                let console = ["Collapsed", "Open"][workspace.console_on_launch().index()];
                let attention = [
                    "Badge and notify; never steal focus",
                    "Notify on failure only",
                    "Silent",
                ][workspace.background_task_attention().index()];
                let layout = state.workbench.workspace_layout(state.workbench.workspace);
                rows.extend([
                    ResolvedPolicyRow::user(
                        "Workspace preset",
                        preset,
                        "Device local",
                        "dock composition",
                    ),
                    ResolvedPolicyRow::user(
                        "Console on launch",
                        console,
                        "Device local",
                        "launch presentation",
                    ),
                    ResolvedPolicyRow::user(
                        "Dock sizes",
                        format!(
                            "navigator {:.0} px · inspector {:.0} px · console {:.0} px",
                            layout.navigator_width, layout.inspector_width, layout.console_height
                        ),
                        "Device local",
                        "workspace layout",
                    ),
                    ResolvedPolicyRow::user(
                        "Background task attention",
                        attention,
                        "Device local",
                        "activity stream",
                    ),
                ]);
            }
            rows
        }
        PreferenceCategory::Units => [
            (
                "Unit system",
                ChoicePreference::UnitSystem,
                &["Mixed", "SI", "Imperial layout"][..],
            ),
            (
                "Engineering suffixes",
                ChoicePreference::EngineeringSuffixes,
                &["Strict RSpice · 10Meg, 10m", "Classic SPICE compatibility"][..],
            ),
            (
                "Frequency display",
                ChoicePreference::FrequencyDisplay,
                &["Hz · engineering prefixes", "rad/s"][..],
            ),
            (
                "Temperature display",
                ChoicePreference::TemperatureDisplay,
                &["°C", "K", "°F"][..],
            ),
            (
                "Copied values",
                ChoicePreference::CopiedValueFormat,
                &[
                    "Engineering notation + unit",
                    "Scientific notation + SI unit",
                ][..],
            ),
            (
                "Angle display",
                ChoicePreference::AngleDisplay,
                &["Degrees", "Radians"][..],
            ),
            (
                "Layout coordinate display",
                ChoicePreference::LayoutCoordinateDisplay,
                &["µm with database-unit remainder", "nm", "Database units"][..],
            ),
            (
                "Time and frequency input",
                ChoicePreference::TimeFrequencyInput,
                &["Strict units required", "Infer from field quantity"][..],
            ),
            (
                "Decimal separator on input",
                ChoicePreference::DecimalSeparatorInput,
                &["Locale-aware UI · portable files", "Period everywhere"][..],
            ),
        ]
        .into_iter()
        .map(|(setting, key, options)| {
            choice_policy_row(
                state,
                setting,
                key,
                options,
                "User profile",
                "display/input",
            )
        })
        .collect(),
        PreferenceCategory::Schematic => {
            let mut rows = vec![
                choice_policy_row(
                    state,
                    "Grid and snap",
                    ChoicePreference::SchematicGrid,
                    &["50 mil", "25 mil", "Metric"],
                    "User default",
                    "new schematic document",
                ),
                choice_policy_row(
                    state,
                    "Operating-point annotation",
                    ChoicePreference::OperatingPointAnnotation,
                    &["Voltages + selected currents", "Voltages only", "Hidden"],
                    "User default",
                    "new schematic document",
                ),
                toggle_policy_row(
                    state,
                    "Cross-probe behavior",
                    TogglePreference::CrossProbeBehavior,
                    "User default",
                    "schematic interaction",
                ),
                toggle_policy_row(
                    state,
                    "Connectivity checks",
                    TogglePreference::IncrementalConnectivityChecks,
                    "User default",
                    "schematic evidence",
                ),
            ];
            for (setting, key, options) in [
                (
                    "Wire and junction behavior",
                    ChoicePreference::WireJunctionBehavior,
                    &[
                        "Orthogonal · automatic explicit junctions",
                        "Orthogonal · manual junctions",
                        "Any-angle routing",
                    ][..],
                ),
                (
                    "Selection crossing policy",
                    ChoicePreference::SelectionCrossingPolicy,
                    &[
                        "Directional window selection",
                        "Enclosed objects only",
                        "Intersecting objects",
                    ][..],
                ),
                (
                    "Net naming policy",
                    ChoicePreference::NetNamingPolicy,
                    &[
                        "Strict project policy · case sensitive",
                        "SPICE-compatible relaxed",
                    ][..],
                ),
                (
                    "Property commit",
                    ChoicePreference::PropertyCommitPolicy,
                    &[
                        "Atomic · reject the complete invalid edit",
                        "Apply valid fields and report failures",
                    ][..],
                ),
            ] {
                rows.push(choice_policy_row(
                    state,
                    setting,
                    key,
                    options,
                    "User default",
                    "new schematic document",
                ));
            }
            rows
        }
        PreferenceCategory::Simulation => vec![choice_policy_row(
            state,
            "Default solver preset",
            ChoicePreference::DefaultSolverPreset,
            &["Balanced", "Fast", "Accurate", "Robust"],
            "User default",
            "new-project simulation plan",
        )],
        PreferenceCategory::Results => {
            let mut rows = vec![scalar_policy_row(
                state,
                "Displayed significant digits",
                ScalarPreference::DisplayedSignificantDigits,
                "",
                "User default",
                "result presentation",
            )];
            for (setting, key, options) in [
                (
                    "Cursor interpolation",
                    ChoicePreference::CursorInterpolation,
                    &[
                        "Monotone cubic where valid",
                        "Linear",
                        "Nearest accepted point",
                    ][..],
                ),
                (
                    "Complex-number display",
                    ChoicePreference::ComplexNumberDisplay,
                    &[
                        "Magnitude / phase · degrees",
                        "Real / imaginary",
                        "Magnitude / phase · radians",
                    ][..],
                ),
                (
                    "Large-dataset display",
                    ChoicePreference::LargeDatasetDisplay,
                    &[
                        "Envelope + extrema-preserving decimation",
                        "Uniform display sampling",
                        "No display decimation",
                    ][..],
                ),
                (
                    "Default engineering export",
                    ChoicePreference::EngineeringExport,
                    &["CSV", "Touchstone where compatible"][..],
                ),
            ] {
                rows.push(choice_policy_row(
                    state,
                    setting,
                    key,
                    options,
                    "User default",
                    "result presentation",
                ));
            }
            rows
        }
        PreferenceCategory::Files => vec![ResolvedPolicyRow::user(
            "Autosave interval",
            format!("{} minutes", state.ui.autosave_minutes),
            "User + project",
            "recovery checkpoint cadence",
        )],
        PreferenceCategory::Compute => Vec::new(),
        PreferenceCategory::Security => Vec::new(),
        PreferenceCategory::Accessibility => vec![
            toggle_policy_row(
                state,
                "Reduced motion",
                TogglePreference::ReducedMotion,
                "User profile",
                "interface accessibility",
            ),
            choice_policy_row(
                state,
                "Minimum touch target",
                ChoicePreference::MinimumTouchTarget,
                &["44 px · WCAG recommended", "48 px"],
                "User profile",
                "pointer accessibility",
            ),
        ],
        PreferenceCategory::Shortcuts => {
            let policies = state.ui.preferences.shortcuts().policies();
            vec![
                ResolvedPolicyRow::user(
                    "Single-key canvas commands",
                    policies.single_key_canvas().label(),
                    "User profile",
                    "shortcut dispatch",
                ),
                ResolvedPolicyRow::user(
                    "Chord timeout",
                    policies.chord_timeout().label(),
                    "User profile",
                    "shortcut dispatch",
                ),
                ResolvedPolicyRow::user(
                    "Protected platform shortcuts",
                    policies.protected_shortcuts().label(),
                    "User profile + platform safety",
                    "shortcut conflict policy",
                ),
                ResolvedPolicyRow::user(
                    "Context precedence",
                    policies.context_precedence().label(),
                    "User profile",
                    "shortcut dispatch",
                ),
            ]
        }
        PreferenceCategory::Integrations => Vec::new(),
    }
}

fn render_notes(ui: &mut Ui) {
    let notes = [
        (
            "Inclusion rule",
            "Only preferences with an active runtime owner appear here. Persisted values without an enforcing consumer are omitted.",
        ),
        (
            "Audit",
            "Resolved values are projected from current retained state. This view does not infer an organization, quota, credential store, remote target, or integration provider.",
        ),
    ];
    let narrow = ui.ctx().content_rect().width() <= POLICY_NOTE_NARROW_BREAKPOINT;
    let t = Tokens::get(ui.ctx());
    let mut divider = None;
    let response = egui::Frame::NONE
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if narrow {
                ui.spacing_mut().item_spacing.y = 0.0;
                let width = ui.available_width();
                let first = note_cell(ui, width, notes[0].0, notes[0].1);
                divider = Some(NoteDivider::Horizontal(first.rect.bottom()));
                note_cell(ui, width, notes[1].0, notes[1].1);
            } else {
                ui.spacing_mut().item_spacing.x = 0.0;
                let width = ui.available_width() * 0.5;
                ui.horizontal(|ui| {
                    let first = note_cell(ui, width, notes[0].0, notes[0].1);
                    divider = Some(NoteDivider::Vertical(first.rect.right()));
                    note_cell(ui, width, notes[1].0, notes[1].1);
                });
            }
        })
        .response;
    match divider {
        Some(NoteDivider::Horizontal(y)) => {
            ui.painter().hline(
                response.rect.x_range(),
                y,
                Stroke::new(1.0, t.color.border_strong),
            );
        }
        Some(NoteDivider::Vertical(x)) => {
            ui.painter().vline(
                x,
                response.rect.y_range(),
                Stroke::new(1.0, t.color.border_strong),
            );
        }
        None => {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NoteDivider {
    Horizontal(f32),
    Vertical(f32),
}

fn note_cell(ui: &mut Ui, width: f32, title: &str, body: &str) -> Response {
    let mut title_id = None;
    let response = ui
        .scope(|ui| {
            ui.set_width(width);
            egui::Frame::NONE
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    title_id = Some(policy_text(ui, title, true, false).id);
                    ui.add_space(4.0);
                    policy_text(ui, body, false, true);
                });
        })
        .response;
    if let Some(title_id) = title_id {
        ui.ctx().accesskit_node_builder(title_id, |node| {
            node.set_role(egui::accesskit::Role::Label);
            node.set_label(title);
            node.set_description(body);
            node.set_bounds(access_bounds(response.rect));
        });
    }
    response
}

fn policy_text(ui: &mut Ui, text: &str, strong: bool, dim: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::sans(
                    tokens::FS_0,
                    if strong {
                        FontWeight::SemiBold
                    } else {
                        FontWeight::Regular
                    },
                ))
                .color(if dim { t.color.text_dim } else { t.color.text }),
        )
        .wrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_policy_nodes(
        viewport_width: f32,
        surface_width: f32,
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let state = AppState::default();
        let rows = resolved_policy_rows(PreferenceCategory::Appearance, &state);
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(viewport_width, 700.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        ui.scope(|ui| {
                            ui.set_width(surface_width);
                            ui.spacing_mut().item_spacing.y = 0.0;
                            render_policy_table(ui, &rows);
                            render_notes(ui);
                        });
                    });
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes
    }

    fn node_bounds(
        nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
        role: egui::accesskit::Role,
        label: &str,
    ) -> egui::accesskit::Rect {
        nodes
            .iter()
            .find(|(_, node)| node.role() == role && node.label() == Some(label))
            .and_then(|(_, node)| node.bounds())
            .unwrap_or_else(|| {
                panic!(
                    "missing {role:?} node {label}; labeled nodes: {:?}",
                    nodes
                        .iter()
                        .filter_map(|(_, node)| node.label().map(|label| (node.role(), label)))
                        .collect::<Vec<_>>()
                )
            })
    }

    #[test]
    fn policy_projection_is_truthful_without_claiming_an_organization_overlay() {
        let state = AppState::default();
        for category in [
            PreferenceCategory::Compute,
            PreferenceCategory::Security,
            PreferenceCategory::Integrations,
        ] {
            assert!(resolved_policy_rows(category, &state).is_empty());
        }
        let simulation = resolved_policy_rows(PreferenceCategory::Simulation, &state);
        assert_eq!(simulation.len(), 1);
        assert_eq!(simulation[0].setting, "Default solver preset");
        assert_eq!(simulation[0].resolved_value, "Balanced");
        assert_eq!(simulation[0].source, "User default");
        assert!(!simulation[0].locked);
        let appearance = resolved_policy_rows(PreferenceCategory::Appearance, &state);
        let density = appearance
            .iter()
            .find(|row| row.setting == "Interface density")
            .expect("density projection");
        assert_eq!(density.source, "User profile");
        assert!(!density.locked);

        let schematic = resolved_policy_rows(PreferenceCategory::Schematic, &state);
        assert!(
            schematic
                .iter()
                .all(|row| row.setting != "Edit-in-place hierarchy")
        );
        let results = resolved_policy_rows(PreferenceCategory::Results, &state);
        for unsupported in [
            "Family trace labeling",
            "Default axis policy",
            "Measurement evaluation",
            "Plot documents",
            "Cross-probe result families",
        ] {
            assert!(results.iter().all(|row| row.setting != unsupported));
        }
    }

    #[test]
    fn override_state_tones_match_locked_and_allowed_contract_colors() {
        let tokens = Tokens::default();
        assert_eq!(
            policy_cell_color(&tokens, false, PolicyCellTone::Locked),
            tokens.color.err
        );
        assert_eq!(
            policy_cell_color(&tokens, false, PolicyCellTone::Allowed),
            tokens.color.ok
        );
    }

    #[test]
    fn desktop_policy_surface_uses_fixed_full_width_columns_and_shared_notes() {
        let nodes = render_policy_nodes(1_440.0, 760.0);
        let table = node_bounds(
            &nodes,
            egui::accesskit::Role::Table,
            "Resolved preference policy",
        );
        assert_eq!(table.x1 - table.x0, 760.0);

        let headers = POLICY_HEADERS
            .map(|label| node_bounds(&nodes, egui::accesskit::Role::ColumnHeader, label));
        for header in &headers {
            assert_eq!(header.x1 - header.x0, 152.0);
        }
        for pair in headers.windows(2) {
            assert_eq!(pair[0].x1, pair[1].x0);
        }

        let resolution = node_bounds(&nodes, egui::accesskit::Role::Label, "Inclusion rule");
        let audit = node_bounds(&nodes, egui::accesskit::Role::Label, "Audit");
        assert_eq!(resolution.y0, audit.y0);
        assert_eq!(resolution.x1, audit.x0);
        assert_eq!(resolution.x1 - resolution.x0, 379.0);
        assert_eq!(audit.x1 - audit.x0, 379.0);
    }

    #[test]
    fn phone_policy_surface_keeps_scroll_track_and_stacks_shared_notes() {
        let nodes = render_policy_nodes(390.0, 390.0);
        let table = node_bounds(
            &nodes,
            egui::accesskit::Role::Table,
            "Resolved preference policy",
        );
        assert_eq!(table.x1 - table.x0, POLICY_TABLE_NARROW_MIN_WIDTH as f64);

        let resolution = node_bounds(&nodes, egui::accesskit::Role::Label, "Inclusion rule");
        let audit = node_bounds(&nodes, egui::accesskit::Role::Label, "Audit");
        assert_eq!(resolution.x0, audit.x0);
        assert_eq!(resolution.x1, audit.x1);
        assert_eq!(resolution.y1, audit.y0);
        assert_eq!(resolution.x1 - resolution.x0, 388.0);
    }

    #[test]
    fn policy_table_exposes_complete_table_semantics() {
        let state = AppState::default();
        let rows = resolved_policy_rows(PreferenceCategory::Appearance, &state);
        let nodes = render_policy_nodes(1_440.0, 760.0);
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Table)
                .count(),
            1
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Row)
                .count(),
            rows.len() + 1
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::ColumnHeader)
                .count(),
            POLICY_COLUMN_COUNT
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Cell)
                .count(),
            POLICY_COLUMN_COUNT * rows.len()
        );
        for setting in rows.iter().map(|row| row.setting.as_str()) {
            assert!(nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Row && node.label() == Some(setting)
            }));
        }
    }
}
