//! Application source selection and cache coordination for the Nyquist viewer.

#[cfg(test)]
mod agreement;

use crate::ui::plot;
use crate::workbench::AppState;
use egui::Ui;
use rspice_results_ui::nyquist::NyquistData;
use rspice_results_ui::nyquist::view::{
    self, IMAGINARY_SERIES, NYQUIST_CACHE_BASE, NyquistPlot, NyquistStability, REAL_SERIES,
};

/// The retained loop-gain locus.
fn active_curve(state: &AppState) -> Option<&NyquistData> {
    state
        .analysis
        .nyquist_state
        .curve()
        .filter(|curve| !curve.is_empty())
}

/// Stability numbers cached on the application data version.
#[derive(Debug, Clone, Copy)]
pub struct NyquistDerived {
    version: u64,
    stability: NyquistStability,
}

fn derived(state: &mut AppState) -> Option<NyquistDerived> {
    let version = state.simulation.data_version;
    if let Some(cached) = state.ui.results.nyquist
        && cached.version == version
    {
        return Some(cached);
    }
    let curve = active_curve(state)?;
    let computed = NyquistDerived {
        version,
        stability: NyquistStability {
            encirclements: curve.count_encirclements(),
            min_distance: curve.min_distance_from_critical(),
            gain_margin: curve.gain_margin(),
            phase_margin: curve.phase_margin(),
        },
    };
    state.ui.results.nyquist = Some(computed);
    Some(computed)
}

/// Present the active retained loop gain and apply local viewer outcomes.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    // Component arrays for the plot engine, cached per data version.
    //
    // The cache and the locus live in disjoint halves of the session, so the
    // arrays are built by reference. Copying the whole contour out first —
    // purely to release the borrow — meant every frame paid for a complete
    // copy of the locus before finding out the cache already held it.
    let prepared = if active_curve(state).is_some() {
        let nyquist = &state.analysis.nyquist_state;
        let points = nyquist
            .curve()
            .map(|curve| curve.points.as_slice())
            .unwrap_or_default();
        let series = &mut state.ui.results.session.derived;
        let re = series.get_or(
            plot::trace_cache_key(NYQUIST_CACHE_BASE, REAL_SERIES),
            || std::sync::Arc::new(points.iter().map(|p| p.real).collect::<Vec<_>>()),
        );
        let im = series.get_or(
            plot::trace_cache_key(NYQUIST_CACHE_BASE, IMAGINARY_SERIES),
            || std::sync::Arc::new(points.iter().map(|p| p.imag).collect::<Vec<_>>()),
        );
        let stats = derived(state).map(|cached| cached.stability);
        Some((re, im, stats))
    } else {
        None
    };
    let source = prepared.as_ref().and_then(|(re, im, stats)| {
        state
            .analysis
            .nyquist_state
            .curve()
            .map(|curve| NyquistPlot {
                curve,
                real: re,
                imaginary: im,
                stability: *stats,
            })
    });
    let results = &mut state.ui.results;
    let viewer = super::ResultViewer::Nyquist;
    let mut view = results.session.plot_view(viewer, 0);
    let mut pin = results.session.rf_pin.get(&viewer).copied();
    if let Some(response) = view::show(
        ui,
        source,
        &mut view,
        &mut pin,
        &mut results.session.cache,
        &quantity_policy,
    ) {
        super::record_drawn_axes(results, viewer, &response);
        if response.view.any() {
            *results.session.plot_view_mut(viewer, 0) = view;
        }
        if response.response.clicked() {
            match pin {
                Some(hit) => {
                    results.session.rf_pin.insert(viewer, hit);
                }
                None => {
                    results.session.rf_pin.remove(&viewer);
                }
            }
        }
    }
}

/// Read the cached stability of the selected source into the inspector.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    view::right_panel(
        ui,
        derived(state).map(|cached| cached.stability),
        &quantity_policy,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The derived numbers are cached on the retained data version, so a new
    /// result replaces them instead of serving the previous locus's stability.
    #[test]
    fn derived_stability_follows_the_retained_data_version() {
        let mut state = AppState::default();
        state
            .analysis
            .nyquist_state
            .load_data(NyquistData::from_arrays("L(jω)", &[1.0], &[0.0], &[0.0]));

        let first = derived(&mut state).expect("a loaded locus has stability numbers");
        assert_eq!(first.stability.min_distance, Some(1.0));

        state
            .analysis
            .nyquist_state
            .load_data(NyquistData::from_arrays("L(jω)", &[1.0], &[-1.0], &[2.0]));
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);
        let second = derived(&mut state).expect("the replacement locus is derived afresh");
        assert_eq!(second.stability.min_distance, Some(2.0));
    }
}
