//! The Verify fixtures over the whole application, for the checks that need it: each
//! is the application's test instance with the seeding the Verify raster fixtures
//! run on the feature harness. The application is a Verify page stage too, so
//! the one page driver draws over either.

use crate::state::VerificationPage;
use crate::workbench::RSpiceApp;
use crate::workbench::state::Workspace;
use crate::workbench::surfaces::verify::raster::{
    VerifyStage, seed_corner, seed_monte_carlo, seed_optimization, seed_soa,
};
use crate::workbench::surfaces::verify::show;

/// Forty trials of the plan's Monte Carlo analysis, three under its gain floor.
pub(super) fn monte_carlo_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    seed_monte_carlo(&mut app.state.model);
    app
}

/// An SOA run of five checks on four devices: two violations, one check near
/// its limit, two clean.
pub(super) fn soa_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    seed_soa(&mut app.state.model);
    app
}

/// A seven-corner run of four limited outputs and two node voltages.
pub(super) fn corner_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    seed_corner(&mut app.state.model);
    app
}

/// Twenty-four optimizer iterations over two variables, converged.
pub(super) fn optimization_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    seed_optimization(&mut app.state.model);
    app
}

impl VerifyStage for RSpiceApp {
    fn open(&mut self, page: VerificationPage) {
        self.state.workbench.activate(Workspace::Verify);
        self.state.simulate.verification_page = page;
    }

    fn draw(&mut self, ui: &mut egui::Ui) {
        show(ui, &mut crate::workbench::features::cx(self));
    }
}
