//! The Simulate workspace's navigator: the setup routes, the plan's creating
//! action, and the run set's swept axes.

use egui::{ScrollArea, Ui, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;

use super::super::super::commands::vocabulary::Command;
use super::super::super::design_system::{WorkbenchIcon, section_header};
use super::super::super::state::SimulationPage;
use super::{nav_matches, nav_row};

// Mirrors the mockup's `.section-body { padding-inline: 10px; }` contract so
// run-set values remain visually contained beside the analysis-stack divider.
pub(super) const NAV_PROPERTY_PADDING_X: f32 = 10.0;
/// Gutter between a property row's label column and its value column, from the
/// mockup's `.form-grid { gap: 6px 11px; }` column gap.
pub(super) const NAV_PROPERTY_COLUMN_GAP: f32 = 11.0;
/// The share of a property row its label column may claim before it elides.
///
/// The mockup lays this card out as a `.form-grid` — `minmax(110px, 0.8fr)`
/// for the label, `minmax(130px, 1.2fr)` for the value — so neither column can
/// grow into the other. At the 208 px of content the dock leaves below 1261 px
/// the 110 px floor is what binds, which is just over half the row. Laying the
/// two out as one row with the value floated right instead let a long value
/// grow leftwards until it reached the label: at that width "Supply voltage"
/// and "3 values · axis off" were painted with no gap between them at all.
const NAV_PROPERTY_LABEL_FRACTION: f32 = 0.5;
/// Axis values a Run set row shows before it counts the rest. Five, the same
/// as the mockup rail: enough to recognize a corner list, short enough that
/// the row stays one line.
const NAV_AXIS_VALUE_LIMIT: usize = 5;
// The creator needs clearance from the last tree row above it while staying
// flush to the panel's sides and to the block's bottom.
const SIMULATION_CREATOR_GAP: f32 = 6.0;
const SIMULATION_CREATOR_EXTRA_HEIGHT: f32 = 8.0;

pub(in crate::workbench) fn simulate(ui: &mut Ui, app: &mut RSpiceApp) {
    let enabled = app.state.model.sim_setup.enabled_analysis_instance_count();
    let total = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .map_or(0, |plan| plan.instances().len());
    let query = app.state.workbench.navigator_filter().trim().to_lowercase();
    // One validation for the whole rail. The Run set row states the point
    // count and the Run set card below states it again, and each asked the run
    // set for it — which validates the declaration and expands the space, so
    // the dock paid for the expansion twice a frame to print one number in two
    // places. The forecast's point count does not depend on how many analyses
    // are enabled; only its task count does, so one validation answers both.
    let run_set_validation =
        rspice_simulation_contract::run_set::validate(&app.state.model.sim_setup.run_set, enabled);
    let declared_points = run_set_validation
        .errors
        .is_empty()
        .then_some(run_set_validation.forecast.point_count);
    // One reading of the design for the whole rail, for the same reason. The
    // Excitations row states what the whole design places, and resolving that
    // digests every cell view and walks the configured hierarchy; nine rows
    // each asking would pay for it nine times to print one number once.
    let placed_sources = crate::simulation::placed_sources::whole_design_source_count(
        &app.state.model.library_manager,
        &app.state.model.workspace,
        &app.state.model.schematic,
    );
    // The nine routes are resolved before the head is drawn, because the head
    // is that section's own and a header over nothing is a claim the rail
    // cannot support.
    let routes = SimulationPage::NAVIGATION
        .into_iter()
        .filter(|page| nav_matches(&query, page.label()))
        .collect::<Vec<_>>();
    // The Analyses row already counts what is enabled; the header only names
    // the section.
    if !routes.is_empty() {
        section_header(ui, "Setup", None);
    }
    ScrollArea::vertical()
        .id_salt("workbench.simulation.navigator")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let active = app.state.simulate.simulation_page;
            // Enabled over total. The previous wording called the total
            // "active", which is what `enabled` means, so a one-analysis plan
            // read "1 active · 1 enabled".
            let analyses_meta = format!("{enabled} / {total}");
            let mut requested = None;
            for page in routes {
                let label = page.label();
                let meta =
                    simulate_nav_meta(app, page, &analyses_meta, declared_points, placed_sources);
                if nav_row(
                    ui,
                    simulate_nav_icon(page),
                    label,
                    active == page,
                    meta.as_deref(),
                ) {
                    requested = Some(page);
                }
            }
            if let Some(page) = requested {
                app.state.simulate.simulation_page = page;
                app.state.workbench.close_drawer();
            }
            simulation_plan_creator(ui, app);
            // One row per declared axis, because the card is a read of the run
            // set and the run set is its axes. The reference point it named
            // instead is one point of that space: with three axes enabled the
            // rail reported a single process corner and a single temperature
            // while the plan was going to run twenty-seven points over four
            // dimensions, and nothing on the rail said so.
            let run_set = &app.state.model.sim_setup.run_set;
            // An axis answers for its own name, which is what the row states.
            // Matching the values instead would leave a row on screen for a
            // query the reader cannot see the reason for. Only the axes the
            // run sweeps are listed: a disabled axis is not part of the run,
            // and no axes at all is the default, which needs no row. The point
            // count is the Run set route row's.
            let axes = run_set
                .dimensions
                .iter()
                .filter(|dimension| dimension.enabled && nav_matches(&query, &dimension.name))
                .collect::<Vec<_>>();
            if !axes.is_empty() {
                section_header(ui, "Run set", None);
            }
            for dimension in axes {
                let values = dimension
                    .values
                    .iter()
                    .map(|value| value.lexical.as_str())
                    .collect::<Vec<_>>();
                // The unit is the axis's, not the value's: the run set stores
                // temperature values as bare lexical numbers, so a rail that
                // printed them as authored said "-40 25 125" and left the
                // reader to guess °C from the row's name. The mockup card
                // carries the unit on every value; one unit for the axis says
                // the same thing and is what fits the dock's 228 px.
                let unit = dimension.unit();
                let value = if values.len() > NAV_AXIS_VALUE_LIMIT {
                    format!(
                        "{}{} +{}",
                        values[..NAV_AXIS_VALUE_LIMIT].join(" "),
                        unit.map(|unit| format!(" {unit}")).unwrap_or_default(),
                        values.len() - NAV_AXIS_VALUE_LIMIT
                    )
                } else {
                    format!(
                        "{}{}",
                        values.join(" "),
                        unit.map(|unit| format!(" {unit}")).unwrap_or_default()
                    )
                };
                nav_property(ui, &dimension.name, &value);
            }
        });
}

/// The panel's one creating action, at the foot of the tree it adds to.
///
/// Full-bleed and square rather than an inset pill: it reads as the last row
/// of the tree rather than a control floating inside the panel, and it carries
/// the filled accent so it is the obvious way into an empty plan.
///
/// The row is the command's, not a look-alike: the panel draws it on all nine
/// setup routes, and it was writing the catalogue's four palette fields itself
/// while only the Analyses route drew the catalogue. Routing through
/// `Command::AddAnalysis` puts the arming and the drawing under one owner.
fn simulation_plan_creator(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.add_space(SIMULATION_CREATOR_GAP);
    let width = ui.available_width();
    if Button::new(Command::AddAnalysis.spec().label)
        .accent()
        .min_width(width)
        .min_height(Tokens::get(ui.ctx()).metrics.row_h + SIMULATION_CREATOR_EXTRA_HEIGHT)
        .square()
        .show(ui)
        .clicked()
    {
        Command::AddAnalysis.execute(app);
    }
}

const fn simulate_nav_icon(page: SimulationPage) -> WorkbenchIcon {
    match page {
        SimulationPage::Analyses => WorkbenchIcon::Results,
        SimulationPage::Excitations => WorkbenchIcon::ArrowRight,
        SimulationPage::Variables => WorkbenchIcon::Sliders,
        SimulationPage::Outputs => WorkbenchIcon::Probe,
        SimulationPage::Specifications => WorkbenchIcon::Target,
        SimulationPage::RunSet => WorkbenchIcon::Grid,
        SimulationPage::Models => WorkbenchIcon::Models,
        SimulationPage::Solver => WorkbenchIcon::Settings,
        SimulationPage::Save => WorkbenchIcon::Save,
    }
}

/// Row meta is a count of what the page owns, read from the same state the
/// page edits. A page whose owner is empty shows nothing rather than a zero,
/// so the column does not fill with noise before a plan has been authored.
pub(super) fn simulate_nav_meta(
    app: &RSpiceApp,
    page: SimulationPage,
    analyses: &str,
    points: Option<usize>,
    placed_sources: usize,
) -> Option<String> {
    let payload = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| app.state.model.workspace.content.plan_data(plan_id));
    let count = |value: usize| (value > 0).then(|| value.to_string());
    match page {
        SimulationPage::Analyses => Some(analyses.to_owned()),
        // Counted from the drawing rather than from the plan payload: a source
        // is a placed instance, and the plan holds no list of them. The whole
        // design rather than the open sheet, and the same number the page this
        // row opens states in its heading — a rail promising one on the way
        // into a page headed `DESIGN · 3 sources` was counting a different
        // design from the one the run drives. Read once for the whole rail by
        // the caller, like the analysis and run-set numbers beside it.
        SimulationPage::Excitations => count(placed_sources),
        SimulationPage::Variables => count(payload.map_or(0, |data| data.design_variables.len())),
        SimulationPage::Outputs => count(payload.map_or(0, |data| data.saved_outputs.len())),
        // The same saved-output count the Outputs row states; one number is
        // printed once.
        SimulationPage::Save => None,
        SimulationPage::Specifications => count(payload.map_or(0, |data| data.specs.len())),
        SimulationPage::RunSet => points.map(|points| format!("{points} pt")),
        SimulationPage::Models => count(app.state.model.sim_setup.model_bindings.len()),
        // The active numerical policy, not a count — the tree's job is to say
        // what each route currently holds, and for the solver that is which
        // preset the effective options match.
        SimulationPage::Solver => Some(app.state.model.sim_setup.options.preset_label()),
    }
}

/// One `label`/`value` row of a navigator card, laid out as two columns.
///
/// The label column is measured — it takes what the label needs, up to
/// [`NAV_PROPERTY_LABEL_FRACTION`] of the row, and elides past that. The value
/// column is what is left, and the value wraps inside it rather than growing
/// out of it. Neither column can reach the other, which is the whole point:
/// the previous single-row layout floated the value right and let it run
/// leftwards until it hit the label, so at the dock's 228 px "Supply voltage"
/// and "3 values · axis off" were painted over each other with no space
/// between them and nothing in the row said which glyphs belonged to which.
fn nav_property(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let value_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let width = ui.available_width();
    let inner = (width - NAV_PROPERTY_PADDING_X * 2.0).max(0.0);
    let label_natural = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(label.to_owned(), label_font.clone(), t.color.text_dim)
            .size()
            .x
    });
    let label_column = label_natural.min(inner * NAV_PROPERTY_LABEL_FRACTION);
    let value_column = (inner - label_column - NAV_PROPERTY_COLUMN_GAP).max(0.0);
    // Both galleys are laid out against their own column before either is
    // placed. Nesting two child regions inside one row does not hold here:
    // egui grows a region to include whatever overflowed it, so the first
    // value that did not fit widened the card and every row below it started
    // further right than the one above.
    let label_galley = ui.fonts_mut(|fonts| {
        let mut job = egui::text::LayoutJob::simple_singleline(
            label.to_owned(),
            label_font,
            t.color.text_dim,
        );
        job.wrap = egui::text::TextWrapping::truncate_at_width(label_column);
        fonts.layout_job(job)
    });
    let value_galley = ui
        .fonts_mut(|fonts| fonts.layout(value.to_owned(), value_font, t.color.text, value_column));
    let height = label_galley.size().y.max(value_galley.size().y);
    let (_, rect) = ui.allocate_space(Vec2::new(width, height));
    let painter = ui.painter();
    painter.galley(
        egui::pos2(rect.left() + NAV_PROPERTY_PADDING_X, rect.top()),
        label_galley,
        t.color.text_dim,
    );
    // The value block is flush to the card's trailing inset. A value that had
    // to wrap keeps its lines left-aligned inside that block, so a two-line
    // value still reads as one value rather than two ragged fragments.
    painter.galley(
        egui::pos2(
            rect.right() - NAV_PROPERTY_PADDING_X - value_galley.size().x,
            rect.top(),
        ),
        value_galley,
        t.color.text,
    );
}
