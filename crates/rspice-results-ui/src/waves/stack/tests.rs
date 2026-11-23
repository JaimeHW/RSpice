use super::*;

/// A held node's samples can differ in their last bit, and a fit to that span
/// fills the pane with rounding. A real ripple on the same level keeps its
/// span, and so does a small signal around zero.
#[test]
fn a_fit_reads_rounding_on_a_held_node_as_one_level() {
    let level = 0.999_995_000_024_839_f64;
    let next = f64::from_bits(level.to_bits() + 1);
    assert_eq!(
        pane_auto_y(Some((level, next)), None, &[]),
        Some((level / 1.1, next * 1.1))
    );
    for span in [(level, level + 1.0e-6), (-1.0e-15, 1.0e-15)] {
        assert_eq!(pane_auto_y(Some(span), None, &[]), Some(span));
    }
    assert_eq!(pane_auto_y(Some((0.0, 0.0)), None, &[]), Some((-1.0, 1.0)));
    assert_eq!(
        pane_auto_y(Some((-5.0, -5.0)), None, &[]),
        Some((-6.0, -4.0))
    );
}

/// A transient whose columns, all volts, are `names`, in that order.
fn volts(names: &[&str]) -> StripModel {
    let columns: Vec<(&str, &str)> = names.iter().map(|name| (*name, "V")).collect();
    transient(&columns)
}

/// A transient whose columns are `columns`, each its name and its unit, in
/// that order.
fn transient(columns: &[(&str, &str)]) -> StripModel {
    let grid = vec![0.0, 1.0e-3];
    let mut run = rspice_results::run::SimulationRun::new(
        1,
        0.0,
        rspice_results::run::ExecutionTarget::LocalDesktop,
    );
    run.analyses.push(
        rspice_results::analysis_result::AnalysisResult::new(
            1,
            rspice_results::analysis_type::AnalysisType::Transient,
            "Tran",
            0.0,
        )
        .with_waveforms(
            columns
                .iter()
                .map(|(name, unit)| {
                    rspice_results::waveform::RetainedTrace::new(
                        *name,
                        grid.clone(),
                        vec![0.0, 1.0],
                        "#fff",
                    )
                    .with_unit(*unit)
                })
                .collect(),
        ),
    );
    let mut models = crate::waves::build_models(
        &[&run],
        &rspice_results::executed_deck::ExecutedDeckArchive::default(),
        &mut crate::derived::DerivedSeries::default(),
        &rspice_ui_kit::tokens::Tokens::default(),
        crate::waves::ProjectionOptions {
            phase_continuous: false,
            complex_display: Default::default(),
            ac: Default::default(),
            selection: None,
            hidden_family_traces: &std::collections::HashSet::new(),
        },
        |_| None,
    );
    models.remove(0)
}

/// What a control acting on a pane calls it: its curve, its two curves, or
/// its first curve and how many more it draws, each name counted once
/// however many curves draw it.
#[test]
fn a_pane_is_named_by_its_first_curve_and_how_many_more_it_draws() {
    let name = |names: &[&str]| {
        let model = volts(names);
        let pane = model.unit_panes().into_iter().next().expect("a volts pane");
        pane_name(&model, &pane)
    };
    assert_eq!(name(&["V(a)"]).as_deref(), Some("V(a)"));
    assert_eq!(name(&["V(a)", "V(b)"]).as_deref(), Some("V(a) and V(b)"));
    assert_eq!(
        name(&["V(a)", "V(b)", "V(a)", "V(c)"]).as_deref(),
        Some("V(a) +2")
    );
}

/// What a pane can be drawn against: each signal of its strip once, however
/// many curves draw it, the signals of the pane's own unit first, each in
/// the order the run holds them.
#[test]
fn a_pane_is_offered_each_signal_once_its_own_units_first() {
    let model = transient(&[
        ("V(a)", "V"),
        ("I(r1)", "A"),
        ("V(b)", "V"),
        ("V(a)", "V"),
        ("I(r2)", "A"),
    ]);
    let pane = model
        .unit_panes()
        .into_iter()
        .find(|pane| pane.quantity_unit() == "V")
        .expect("a volts pane");
    let menu = abscissa_menu(&ResultViewerState::default(), &model, &pane, None);
    assert_eq!(menu.signals, ["V(a)", "V(b)", "I(r1)", "I(r2)"]);
}
