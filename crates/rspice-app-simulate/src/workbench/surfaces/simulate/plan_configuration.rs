//! The plan's configuration: which design the plan runs.
//!
//! A plan names its design as an ADE test names its config view. It follows
//! the configuration the project marks "Used for runs" until the reader picks
//! another, so the row under the plan's name says what it follows as well as
//! what it would run.

use egui::{Align, Layout, Sense, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::select;
use crate::workbench::app_state::Model;
use crate::workbench::app_state::design_history::SetupParts;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::SimulateFeature;
use rspice_simulation_contract::design_configuration::DesignConfiguration;

use super::page_kit::paint_text;

/// The label the row and its select carry.
pub const CONFIGURATION_LABEL: &str = "Configuration";
/// Gap between the label and the select.
const LABEL_GAP: f32 = 8.0;
/// The select's width where the heading column has room for it.
const SELECT_WIDTH: f32 = 300.0;

/// The configurations a plan can name, in the order the Configuration page
/// lists them, and the one it names now.
pub struct ConfigurationChoices {
    pub selected: String,
    pub options: Vec<String>,
    values: Vec<DesignConfiguration>,
}

impl ConfigurationChoices {
    pub fn of(model: &Model) -> Self {
        let catalog = &*model.workspace.content.configuration_sets;
        let marked = catalog
            .active()
            .map_or("Default", |configuration| configuration.name());
        let mut options = vec![
            format!("Used for runs \u{b7} {marked}"),
            "Default".to_owned(),
        ];
        let mut values = vec![
            DesignConfiguration::UsedForRuns,
            DesignConfiguration::BuiltIn,
        ];
        for configuration in catalog.configurations() {
            options.push(configuration.name().to_owned());
            values.push(DesignConfiguration::Named(configuration.id()));
        }
        let design = model.sim_setup.design_configuration;
        let selected = values
            .iter()
            .position(|value| *value == design)
            .map_or_else(
                || "Deleted configuration".to_owned(),
                |index| options[index].clone(),
            );
        Self {
            selected,
            options,
            values,
        }
    }

    /// The configuration option `index` names.
    pub fn value(&self, index: usize) -> Option<DesignConfiguration> {
        self.values.get(index).copied()
    }
}

/// The row under the plan's name: its label and the select. Returns the
/// configuration picked this frame.
pub fn configuration_row(
    ui: &mut Ui,
    choices: &ConfigurationChoices,
) -> Option<DesignConfiguration> {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let label_width = ui
        .painter()
        .layout_no_wrap(
            CONFIGURATION_LABEL.to_owned(),
            font.clone(),
            t.color.text_dim,
        )
        .size()
        .x
        .ceil();
    let mut picked = None;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 0.0),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = LABEL_GAP;
            let height = ui.spacing().interact_size.y;
            let (label_rect, _) = ui.allocate_exact_size(vec2(label_width, height), Sense::hover());
            paint_text(ui, label_rect, CONFIGURATION_LABEL, font, t.color.text_dim);
            let width = ui.available_width().min(SELECT_WIDTH).max(1.0);
            picked = select(
                ui,
                "simulation.plan.configuration",
                CONFIGURATION_LABEL,
                &choices.selected,
                &choices.options,
                width,
            )
            .and_then(|index| choices.value(index));
        },
    );
    picked
}

/// Make `design` the configuration the active plan runs, as one step of the
/// setup's Undo and a change to the plan with its receipt. Returns whether it
/// changed anything; why the plan refused it is the error.
pub fn commit_design_configuration(
    cx: &mut Cx<'_, SimulateFeature>,
    design: DesignConfiguration,
) -> Result<bool, String> {
    if cx.model.sim_setup.design_configuration == design {
        return Ok(false);
    }
    let step = cx.model.setup_snapshot(SetupParts::DESIGN_CONFIGURATION);
    let name = match design {
        DesignConfiguration::UsedForRuns => "the configuration used for runs".to_owned(),
        DesignConfiguration::BuiltIn => "Default".to_owned(),
        DesignConfiguration::Named(id) => cx
            .model
            .workspace
            .content
            .configuration_sets
            .find(id)
            .map(|configuration| configuration.name().to_owned())
            .ok_or_else(|| "That configuration no longer exists.".to_owned())?,
    };
    let mut setup = cx.model.sim_setup.clone();
    setup.design_configuration = design;
    let receipt = setup
        .commit_active_plan_configuration_change(format!("The plan runs {name}."))
        .map_err(|error| error.to_string())?;
    cx.model.sim_setup = setup;
    cx.model.record_setup_change("configuration", step);
    crate::workbench::simulate::invalidate_preflight(cx);
    cx.model.record_plan_receipt(receipt.status_line());
    Ok(true)
}
