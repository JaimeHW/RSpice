//! The Verify workspace's inspector: the selected specification, or the
//! plan's acceptance settings and the run it judges; the selected Corners
//! cell's point; the selected failing trial, or else the Monte Carlo campaign;
//! the selected check; the selected comparison with the reference run; and a
//! failed corner run.

mod acceptance;
mod check;
mod monte_carlo;
mod point;
mod reference;
mod specification;

pub(crate) use specification::SpecDraft;

use egui::Ui;

use crate::state::VerificationPage;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::SimulateFeature;

/// The Verify inspector states only what its view does not: the selected
/// specification's whole definition, or with none selected the plan's
/// acceptance settings and the run being judged; the conditions behind a
/// selected Corners cell; a selected trial, or else the Monte Carlo
/// campaign's definition; the selected check's rule and evidence; the
/// selected comparison's tolerance and values; and a corner run that failed.
/// The other views already show everything there is to say about them.
pub(in crate::workbench) fn verify(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    match cx.state.verification_page {
        VerificationPage::Specifications => specification::specifications(ui, cx),
        VerificationPage::MonteCarlo => monte_carlo::show(ui, cx.model, &cx.state.verification),
        VerificationPage::Corners => point::show(ui, cx.model, &cx.state.verification),
        VerificationPage::Checks => check::check(ui, cx.model, &cx.state.verification),
        VerificationPage::Reference => reference::reference(ui, &cx.state.verification),
    }
}
