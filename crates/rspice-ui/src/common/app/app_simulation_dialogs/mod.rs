use egui::{Color32, Context};

use super::{app_simulation_analysis_options, ConsoleMessage, RSpiceApp};
use crate::common::simulation_analysis_tabs::SIMULATION_ANALYSIS_CATEGORIES;

mod defaults;
mod options_dialog;
mod setup_dialog;

#[cfg(test)]
mod tests;
