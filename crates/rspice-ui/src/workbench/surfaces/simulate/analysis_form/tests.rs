//! What the per-analysis forms promise, checked by rendering them.
//!
//! Split from `analysis_form.rs` so the forms and the contract they are held
//! to are separate files: the assertions here render a form off-screen and
//! read the text it painted, which is a different concern from painting it and
//! is the half that grows every time a form makes a new claim.

use super::*;
use crate::simulation::dialog::{PssConfig, PssDialogState};
use crate::simulation::plan::{AnalysisKind, NoiseDraft};

#[test]
fn operating_point_startup_choices_match_the_execution_contract() {
    for initial_guess in 0..OP_INITIAL_GUESS_CHOICES.len() {
        let disabled = op_node_initialization_disabled(initial_guess);
        for node_initialization in 0..OP_NODE_INITIALIZATION_CHOICES.len() {
            assert_eq!(
                disabled
                    .iter()
                    .any(|(disabled_idx, _)| *disabled_idx == node_initialization),
                !op_startup_indices_compatible(initial_guess, node_initialization),
            );
        }
    }
    assert!(
        op_initial_guess_disabled(1, false)
            .iter()
            .any(|(index, _)| *index == 1),
        "previous-state policy remains disabled without bound evidence"
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn analysis_form_height(draft: AnalysisDraft) -> f32 {
    analysis_form_height_in_domain(draft, NoiseDomain::default())
}

#[cfg(not(target_arch = "wasm32"))]
fn analysis_form_height_in_domain(draft: AnalysisDraft, noise_domain: NoiseDomain<'_>) -> f32 {
    render_analysis_form(draft, noise_domain).0
}

/// Render one analysis form off-screen: the height it occupied, and every
/// line of text it painted.
///
/// The painted text is what lets a test read a *rendered* label rather than
/// the constant behind it. A form that stopped asking
/// [`sweep_point_field_label`] what its point field is called would still
/// satisfy an assertion about the resolver, and would still paint the wrong
/// units beside the mode selector.
#[cfg(not(target_arch = "wasm32"))]
fn render_analysis_form(draft: AnalysisDraft, noise_domain: NoiseDomain<'_>) -> (f32, Vec<String>) {
    render_analysis_form_against(draft, noise_domain, &[])
}

/// Render one analysis form off-screen against a design that places the given
/// RF ports.
#[cfg(not(target_arch = "wasm32"))]
fn render_analysis_form_against(
    mut draft: AnalysisDraft,
    noise_domain: NoiseDomain<'_>,
    placed_rf_ports: &[crate::simulation::placed_sources::PlacedRfPort],
) -> (f32, Vec<String>) {
    render_analysis_form_into(&mut draft, noise_domain, placed_rf_ports, VIEWPORT_HEIGHT)
}

/// The viewport every geometry assertion is measured in. A form taller than
/// this still lays out, but egui does not paint a widget it can see is
/// off-screen, so a test reading painted text from the bottom of a long form
/// has to say how tall a viewport it is reading.
#[cfg(not(target_arch = "wasm32"))]
const VIEWPORT_HEIGHT: f32 = 600.0;

/// The same render, keeping the draft the form edited.
///
/// A form is a mutation as much as a painting, and what it did *not* write is
/// as much of its contract as what it painted.
#[cfg(not(target_arch = "wasm32"))]
fn render_analysis_form_into(
    draft: &mut AnalysisDraft,
    noise_domain: NoiseDomain<'_>,
    placed_rf_ports: &[crate::simulation::placed_sources::PlacedRfPort],
    viewport_height: f32,
) -> (f32, Vec<String>) {
    // The run-space forms read the plan; a height measurement supplies a
    // plan-shaped fixture rather than letting the form invent one.
    let fixture_run_set = crate::simulation::run_set::RunSetState::default();
    let run_space_fixture = RunSpaceContext {
        run_set: &fixture_run_set,
        reference: crate::simulation::run_set::ReferencePoint::default(),
        nominal_failure: crate::state::NominalFailurePolicy::Block,
        model_binding_count: 0,
        parallelism: ("Desktop background thread", 1),
    };
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut height = 0.0;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(964.0, viewport_height),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    let top = ui.cursor().top();
                    form(
                        ui,
                        draft,
                        QuantityPresentationPolicy::default(),
                        UiNumberLocale::default(),
                        &["VIN_AM".to_owned(), "VIN_IQ".to_owned()],
                        &["VLOOP1".to_owned()],
                        placed_rf_ports,
                        noise_domain,
                        OpContextAvailability::default(),
                        &run_space_fixture,
                        &mut None,
                    );
                    height = ui.cursor().top() - top;
                });
        },
    );
    (height, painted_lines(&output.shapes))
}

/// Every line of text a rendered frame painted, in paint order.
#[cfg(not(target_arch = "wasm32"))]
fn painted_lines(shapes: &[egui::epaint::ClippedShape]) -> Vec<String> {
    fn collect(shape: &egui::epaint::Shape, lines: &mut Vec<String>) {
        match shape {
            egui::epaint::Shape::Text(text) => lines.push(text.galley.job.text.clone()),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, lines);
                }
            }
            _ => {}
        }
    }
    let mut lines = Vec::new();
    for clipped in shapes {
        collect(&clipped.shape, &mut lines);
    }
    lines
}

/// The corner form's height against one declared space.
#[cfg(not(target_arch = "wasm32"))]
fn corner_form_height(run_set: &crate::simulation::run_set::RunSetState) -> f32 {
    let context = RunSpaceContext {
        run_set,
        reference: crate::simulation::run_set::ReferencePoint::default(),
        nominal_failure: crate::state::NominalFailurePolicy::Block,
        model_binding_count: 0,
        parallelism: ("Desktop background thread", 1),
    };
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut height = 0.0;
    let mut base_analysis_idx = 0;
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(964.0, 600.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    let top = ui.cursor().top();
                    run_space::corner_form(ui, &mut base_analysis_idx, &context, &mut None);
                    height = ui.cursor().top() - top;
                });
        },
    );
    height
}

/// A disabled axis is still a declared axis, so the form states the same
/// rows either way and only their values change. Toggling enablement must
/// not make the panel jump.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn a_disabled_axis_does_not_move_the_corner_form() {
    let mut disabled = crate::simulation::run_set::RunSetState::default();
    for dimension in &mut disabled.dimensions {
        dimension.enabled = false;
    }
    let mut enabled = crate::simulation::run_set::RunSetState::default();
    for dimension in &mut enabled.dimensions {
        dimension.enabled = true;
    }

    assert_eq!(
        corner_form_height(&disabled),
        corner_form_height(&enabled),
        "enabling an axis changes what a row says, never how tall the form is"
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn pss_form_height(oscillator_mode: bool) -> f32 {
    let mut setup = PssDialogState::from_config(&PssConfig::default());
    setup.osc_mode = oscillator_mode;
    analysis_form_height(AnalysisDraft::Pss(setup))
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn pss_oscillator_toggle_preserves_form_geometry() {
    let driven_height = pss_form_height(false);
    let oscillator_height = pss_form_height(true);
    assert_eq!(driven_height, oscillator_height);
}

#[test]
fn pss_field_order_and_wording_match_the_canonical_mockup() {
    assert_eq!(
        PSS_FIELD_LABELS,
        [
            "Mode",
            "Fundamental",
            "Tones",
            "Stabilization cycles",
            "Shooting points",
            "Period tolerance",
            "Autonomous oscillator",
            "Oscillator node",
            "Save harmonics",
        ]
    );
    assert_eq!(PSS_MODE_CHOICES, ["Driven shooting"]);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn pss_catalog_and_declared_tone_modes_preserve_form_geometry() {
    let mut selected = PssDialogState::from_config(&PssConfig::default());
    selected.tone_sources = "VIN_AM".to_owned();
    let mut declared = selected.clone();
    declared.tone_sources = "VIN_AM, VIN_IQ".to_owned();
    assert_eq!(
        analysis_form_height(AnalysisDraft::Pss(selected)),
        analysis_form_height(AnalysisDraft::Pss(declared))
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_noise_selection_preserves_the_eight_field_geometry() {
    let expected = analysis_form_height(AnalysisDraft::Noise(NoiseDraft::default()));
    for sweep in [
        NoiseSweepType::Decade,
        NoiseSweepType::Octave,
        NoiseSweepType::Linear,
        NoiseSweepType::ExplicitFrequencyList,
    ] {
        for contribution_detail in [
            NoiseContributionDetail::Top50,
            NoiseContributionDetail::AllContributors,
            NoiseContributionDetail::Top20,
            NoiseContributionDetail::SummaryOnly,
        ] {
            for integration_mode in [
                NoiseIntegrationMode::Enabled,
                NoiseIntegrationMode::OutputNoiseOnly,
                NoiseIntegrationMode::Disabled,
            ] {
                let draft = NoiseDraft {
                    sweep,
                    contribution_detail,
                    integration_mode,
                    ..NoiseDraft::default()
                };
                assert_eq!(expected, analysis_form_height(AnalysisDraft::Noise(draft)));
            }
        }
    }

    for (output, input) in [
        ("out_p,out_n", "I1"),
        ("V(custom_p,custom_n)", "VCUSTOM"),
        ("", ""),
    ] {
        let draft = NoiseDraft {
            output: output.to_owned(),
            input: input.to_owned(),
            ..NoiseDraft::default()
        };
        assert_eq!(expected, analysis_form_height(AnalysisDraft::Noise(draft)));
    }
}

/// Whether the elaborated design was reachable changes what the two domain
/// rows offer, and must not change where any field sits.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_elaborated_noise_domain_preserves_the_eight_field_geometry() {
    let empty = analysis_form_height(AnalysisDraft::Noise(NoiseDraft::default()));
    let nodes = (0..NOISE_DOMAIN_PRESET_LIMIT * 3)
        .map(|index| format!("net{index}"))
        .collect::<Vec<_>>();
    let sources = vec!["I1".to_owned(), "V1".to_owned()];
    for (draft, domain) in [
        (
            NoiseDraft::default(),
            NoiseDomain {
                nodes: &nodes,
                sources: &sources,
                unavailable: None,
            },
        ),
        (
            NoiseDraft {
                output: nodes[1].clone(),
                input: sources[1].clone(),
                ..NoiseDraft::default()
            },
            NoiseDomain {
                nodes: &nodes,
                sources: &sources,
                unavailable: None,
            },
        ),
        (
            NoiseDraft::default(),
            NoiseDomain {
                nodes: &[],
                sources: &[],
                unavailable: None,
            },
        ),
    ] {
        assert_eq!(
            empty,
            analysis_form_height_in_domain(AnalysisDraft::Noise(draft), domain)
        );
    }
}

/// A domain that could not be measured says why, where the reader is.
///
/// The form knew the elaboration diagnostic and painted only "design nodes
/// unavailable", which tells an engineer that something is wrong and nothing
/// about what: the reason was carried into the form and dropped there. It is
/// now stated under the two rows it is about, and the form is allowed to be
/// taller for it — a reason folded into the field's own caption would be
/// clipped to the cell or painted over the label.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_unmeasured_noise_domain_states_why_it_could_not_be_measured() {
    const REASON: &str = "the design has no top-level cell bound";
    let (available_height, available_lines) = render_analysis_form(
        AnalysisDraft::Noise(NoiseDraft::default()),
        NoiseDomain {
            nodes: &[],
            sources: &[],
            unavailable: None,
        },
    );
    assert!(
        !available_lines.iter().any(|line| line.contains(REASON)),
        "a measured domain has no reason to state"
    );

    let (height, lines) = render_analysis_form(
        AnalysisDraft::Noise(NoiseDraft::default()),
        NoiseDomain {
            nodes: &[],
            sources: &[],
            unavailable: Some(REASON),
        },
    );
    assert!(
        lines.iter().any(|line| line.contains(REASON)),
        "the form states why the domain is unavailable; it painted {lines:?}"
    );
    assert!(
        height > available_height,
        "the advisory is laid out, not painted over the rows: {height} vs \
         {available_height}"
    );
}

/// The row says what it is offering. A truncated list must not describe
/// itself as the design, and an unmeasured one must not read as an empty
/// design.
#[test]
fn a_noise_domain_row_states_which_kind_of_empty_it_has() {
    assert_eq!(
        noise_domain_hint("nodes", 0, 0, true),
        "design nodes unavailable"
    );
    assert_eq!(noise_domain_hint("nodes", 0, 0, false), "no design nodes");
    assert_eq!(
        noise_domain_hint("sources", 0, 0, false),
        "no design sources"
    );
    assert_eq!(noise_domain_hint("nodes", 7, 7, false), "design nodes");
    assert_eq!(
        noise_domain_hint("nodes", 64, 812, false),
        "64 of 812 nodes"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn conditional_analysis_controls_preserve_form_geometry() {
    let mut pairs = Vec::new();

    let disabled = AnalysisDraft::for_kind(AnalysisKind::DcSweep);
    let mut enabled = disabled.clone();
    if let AnalysisDraft::DcSweep(setup) = &mut enabled {
        setup.nested = true;
    }
    pairs.push((disabled, enabled));

    let disabled = AnalysisDraft::for_kind(AnalysisKind::Sensitivity);
    let mut enabled = disabled.clone();
    if let AnalysisDraft::Sensitivity(setup) = &mut enabled {
        setup.sens_type_idx = 1;
    }
    pairs.push((disabled, enabled));

    // The Corner pair is absent on purpose: axis enablement is no longer a
    // draft field, so there is nothing on the draft to toggle. The same
    // geometry property is pinned against the plan's declaration by
    // `a_disabled_axis_does_not_move_the_corner_form`.

    let mut disabled = AnalysisDraft::for_kind(AnalysisKind::Optimization);
    let mut enabled = disabled.clone();
    if let AnalysisDraft::Optimization(setup) = &mut disabled {
        setup.goal_mode = 0;
    }
    if let AnalysisDraft::Optimization(setup) = &mut enabled {
        setup.goal_mode = 2;
    }
    pairs.push((disabled, enabled));

    let mut disabled = AnalysisDraft::for_kind(AnalysisKind::Soa);
    let mut enabled = disabled.clone();
    if let AnalysisDraft::Soa(setup) = &mut disabled {
        setup.check_vgs_max = false;
        setup.check_vds_max = false;
        setup.check_vbe_max = false;
        setup.check_vce_max = false;
    }
    if let AnalysisDraft::Soa(setup) = &mut enabled {
        setup.check_vgs_max = true;
        setup.check_vds_max = true;
        setup.check_vbe_max = true;
        setup.check_vce_max = true;
    }
    pairs.push((disabled, enabled));

    let mut disabled = AnalysisDraft::for_kind(AnalysisKind::Qpss);
    let mut enabled = disabled.clone();
    if let AnalysisDraft::Qpss(setup) = &mut disabled {
        setup.autonomous = false;
    }
    if let AnalysisDraft::Qpss(setup) = &mut enabled {
        setup.autonomous = true;
    }
    pairs.push((disabled, enabled));

    for (disabled, enabled) in pairs {
        assert_eq!(
            analysis_form_height(disabled),
            analysis_form_height(enabled)
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn envelope_modulation_source_modes_preserve_form_geometry() {
    let mut named = AnalysisDraft::for_kind(AnalysisKind::Envelope);
    let mut declared = named.clone();
    if let AnalysisDraft::Envelope(setup) = &mut named {
        setup.modulation_sources = "VIN_AM".to_owned();
    }
    if let AnalysisDraft::Envelope(setup) = &mut declared {
        setup.modulation_sources = "VCTRL, VDATA".to_owned();
    }

    assert_eq!(analysis_form_height(named), analysis_form_height(declared));
}

#[test]
fn envelope_modulation_source_split_is_fixed_and_fills_the_control_row() {
    for available_width in [1.0, 120.0, 320.0, 640.0] {
        let (selector, editor) = envelope_modulation_control_widths(available_width);
        assert!(selector > 0.0);
        assert!(editor > 0.0);
        assert_eq!(
            selector + editor + ENVELOPE_INLINE_CONTROL_GAP,
            available_width.max(ENVELOPE_INLINE_CONTROL_GAP + 2.0)
        );
    }
}

#[test]
fn envelope_form_matches_mockup_owned_contract() {
    assert_eq!(
        ENVELOPE_FIELD_LABELS,
        [
            "Carrier tones",
            "Envelope stop",
            "Envelope step",
            "Harmonic order",
            "Modulation sources",
            "Initial periodic solve",
            "Output schedule",
            "Extraction path",
        ]
    );
    assert_eq!(
        ENVELOPE_INITIAL_SOLVE_CHOICES,
        ["HB", "PSS", "Transient spectral estimate"]
    );
    assert_eq!(
        ENVELOPE_ADAPTIVE_CHOICES,
        [
            "Adaptive solver samples",
            "Fixed envelope step",
            "Event-aligned only",
        ]
    );
    assert_eq!(ENVELOPE_DECLARED_SOURCES_CHOICE, "Declared list...");
    assert_eq!(ENVELOPE_HARMONIC_ORDER_HELPER, "positive integer");
    assert_eq!(ENVELOPE_EXTRACTION_PATH, "Least-squares projection");
}

#[test]
fn transfer_function_form_matches_mockup_owned_contract() {
    assert_eq!(
        XF_FIELD_LABELS,
        [
            "Input source",
            "Output expression",
            "Solve point",
            "Transfer gain",
            "Input resistance",
            "Output resistance",
            "Normalize",
            "Accuracy",
        ]
    );
    assert_eq!(XF_SOLVE_POINT, "DC operating point");
    assert_eq!(XF_ENABLED_CHOICES, ["Enabled", "Disabled"]);
    assert_eq!(
        XF_NORMALIZATION_CHOICES,
        ["Disabled", "Relative to nominal", "Per source unit"]
    );
    assert_eq!(
        XF_ACCURACY_CHOICES,
        ["Fast", "Balanced", "Accurate", "Robust"]
    );
}

/// Where a form pairs a point count with a graded [`SWEEP_KINDS`] mode,
/// select that mode on the form's own draft. `false` means this kind has no
/// graded sweep at all.
#[cfg(not(target_arch = "wasm32"))]
fn select_graded_sweep_mode(draft: &mut AnalysisDraft, mode: usize) -> bool {
    match draft {
        AnalysisDraft::Ac(setup) => setup.sweep = mode,
        AnalysisDraft::Disto(setup) => setup.sweep.sweep = mode,
        AnalysisDraft::Stb(setup) => setup.sweep_type_idx = mode,
        AnalysisDraft::SParameter(setup) => setup.sweep_type_idx = mode,
        AnalysisDraft::Pac(setup) => setup.sweep_type_idx = mode,
        AnalysisDraft::Pnoise(setup) => setup.sweep_type_idx = mode,
        AnalysisDraft::Pxf(setup) => setup.sweep_type_idx = mode,
        AnalysisDraft::Hbsp(setup) | AnalysisDraft::Psp(setup) => setup.sweep.sweep = mode,
        AnalysisDraft::Hbnoise(setup) => setup.sweep.sweep = mode,
        AnalysisDraft::Qpac(setup) => setup.sweep.sweep = mode,
        AnalysisDraft::Qpnoise(setup) => setup.sweep.sweep = mode,
        AnalysisDraft::Qpxf(setup) => setup.sweep.sweep = mode,
        _ => return false,
    }
    true
}

/// Every form that grades its sweep names its point field after the mode
/// selected beside it, and no form paints the ungraded spelling.
///
/// Wave 4 routed AC, loop stability and noise through
/// [`sweep_point_field_label`]; eleven further forms — S-parameter, PAC,
/// PNOISE, PXF, DISTO, HBSP, HBNOISE, PSP, QPAC, QPNOISE and QPXF — still
/// painted a bare `Points` beside the same three-mode selector. That is a
/// false unit in two modes out of three, and it fails silently: nothing
/// refuses the run, no diagnostic names it, and the sweep simply resolves
/// to the wrong density by the span of the decade or octave range. The
/// grid the engine actually builds is the shared SPICE one — see
/// `services::simulation_runner::helpers::generate_freq_points_with_abort`,
/// where `dec` counts per decade, `oct` per octave and `lin` is the whole
/// count — so `lin` really does mean the total.
///
/// The assertion reads the *painted* text rather than the resolver, so a
/// form that goes back to a literal fails here instead of passing on the
/// owner's behalf.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_graded_sweep_form_names_what_a_point_is() {
    let graded_labels = (0..SWEEP_KINDS.len())
        .map(sweep_point_field_label)
        .collect::<Vec<_>>();
    let mut graded = Vec::new();
    for kind in AnalysisKind::ALL {
        if !select_graded_sweep_mode(&mut AnalysisDraft::for_kind(kind), 0) {
            // A form with no graded sweep must not paint a point field at
            // all, in either spelling — otherwise this test would skip a
            // form that quietly grew one. The noise form is the one
            // deliberate exception: its sweep is a typed enum rather than a
            // `SWEEP_KINDS` index, resolved by `noise_point_field_label`
            // and pinned by `noise_sweep_modes_are_graded_the_way_the_shared_sweep_is`.
            if kind == AnalysisKind::Noise {
                continue;
            }
            let painted =
                render_analysis_form(AnalysisDraft::for_kind(kind), NoiseDomain::default()).1;
            for label in graded_labels.iter().chain([&SWEEP_POINT_NEUTRAL_LABEL]) {
                assert!(
                    !painted.iter().any(|line| line == label),
                    "{kind:?} paints a `{label}` row but no graded sweep reaches it"
                );
            }
            continue;
        }
        graded.push(kind);
        for (mode, expected) in graded_labels.iter().enumerate() {
            let mut draft = AnalysisDraft::for_kind(kind);
            assert!(select_graded_sweep_mode(&mut draft, mode));
            let painted = render_analysis_form(draft, NoiseDomain::default()).1;
            assert!(
                painted.iter().any(|line| line == expected),
                "{kind:?} in {} mode painted no `{expected}` row: {painted:?}",
                SWEEP_KINDS[mode]
            );
            assert!(
                !painted.iter().any(|line| line == SWEEP_POINT_NEUTRAL_LABEL),
                "{kind:?} in {} mode still paints the ungraded `{SWEEP_POINT_NEUTRAL_LABEL}`",
                SWEEP_KINDS[mode]
            );
        }
    }
    assert_eq!(
        graded,
        [
            AnalysisKind::Ac,
            AnalysisKind::Stb,
            AnalysisKind::SParameter,
            AnalysisKind::Pac,
            AnalysisKind::Pnoise,
            AnalysisKind::Pxf,
            AnalysisKind::Disto,
            AnalysisKind::Hbsp,
            AnalysisKind::Hbnoise,
            AnalysisKind::Psp,
            AnalysisKind::Qpac,
            AnalysisKind::Qpnoise,
            AnalysisKind::Qpxf,
        ],
        "a new graded sweep form has to be added to `select_graded_sweep_mode` \
             before this test can see it"
    );
}

#[test]
fn noise_form_matches_mockup_owned_contract() {
    assert_eq!(
        NOISE_FIELD_LABELS,
        [
            "Sweep",
            // The point field names its own units, so the frozen entry is
            // the ungraded spelling and the rendered one re-resolves. See
            // `the_sweep_point_label_names_what_a_point_is_in_each_mode`.
            "Points",
            "Start frequency",
            "Stop frequency",
            "Output node",
            "Input source",
            "Contribution detail",
            "Integrated noise",
        ]
    );
    assert_eq!(
        NOISE_SWEEP_CHOICES,
        ["Decade", "Octave", "Linear", "Explicit frequency list"]
    );
    // The two domain rows carry no authored presets at all: everything
    // they offer is read off the elaborated design, and the only fixed
    // entry is the exact-entry escape.
    assert_eq!(NOISE_OUTPUT_CUSTOM_CHOICE, "Exact expression\u{2026}");
    assert_eq!(NOISE_INPUT_CUSTOM_CHOICE, "Exact source name\u{2026}");
    assert_eq!(
        NOISE_CONTRIBUTION_CHOICES,
        ["Top 50", "All contributors", "Top 20", "Summary only"]
    );
    assert_eq!(
        NOISE_INTEGRATION_CHOICES,
        ["Enabled", "Output noise only", "Disabled"]
    );
    assert_eq!(NOISE_SWEEP_CONTROL_COUNT, 2);
    // Rendered, the point row names the units the selected mode gives it.
    assert_eq!(
        noise_point_field_label(NoiseSweepType::Decade),
        "Points / decade"
    );
    assert_eq!(
        noise_point_field_label(NoiseSweepType::Linear),
        "Total points"
    );
    for available_width in [1.0, 120.0, 320.0, 640.0] {
        let (selector, editor) = noise_sweep_control_widths(available_width);
        assert!(selector > 0.0);
        assert!(editor > 0.0);
        assert_eq!(
            selector + editor + ENVELOPE_INLINE_CONTROL_GAP,
            available_width.max(ENVELOPE_INLINE_CONTROL_GAP + 2.0)
        );
    }
}

/// Both loop-stability forms designate their probe the same way.
///
/// STB and PSTB break the same loop at the same element, and for a long time
/// only one of them said so: PSTB's probe was free text, so a name that
/// matched nothing on the drawing produced a run that failed in the solver
/// against a source the schematic had never held. Rendering both and reading
/// the hint is what proves the field is the picker rather than a text box that
/// happens to hold the same string — the hint is derived from the placed list,
/// and a free-text field has nothing to derive it from.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn both_loop_stability_forms_offer_the_probes_the_drawing_holds() {
    use crate::simulation::dialog::StbProbeReference;

    for kind in [AnalysisKind::Stb, AnalysisKind::Pstb] {
        let (_, painted) =
            render_analysis_form(AnalysisDraft::for_kind(kind), NoiseDomain::default());
        assert!(
            painted.iter().any(|line| line == "Probe"),
            "{kind:?} paints no probe row: {painted:?}"
        );
        // The fixture hands both forms one placed probe, and a fresh draft has
        // not been shown a drawing, so both must report the entered form and
        // both must offer the placed one to switch to.
        assert!(
            painted.iter().any(|line| line == "entered by hand"),
            "{kind:?} must say where its probe name came from: {painted:?}"
        );
        assert!(
            painted.iter().any(|line| line == "Enter name"),
            "{kind:?} must offer the hand-entered choice beside the placed ones: {painted:?}"
        );
    }

    // And the placed count is what the hint states once a probe is chosen,
    // for both forms, from the one resolver.
    let mut stb = AnalysisDraft::for_kind(AnalysisKind::Stb);
    let AnalysisDraft::Stb(setup) = &mut stb else {
        unreachable!("for_kind returns the draft of the kind it was given");
    };
    setup.probe_source = "VLOOP1".to_owned();
    setup.probe_reference = StbProbeReference::Placed;
    let mut pstb = AnalysisDraft::for_kind(AnalysisKind::Pstb);
    let AnalysisDraft::Pstb(setup) = &mut pstb else {
        unreachable!("for_kind returns the draft of the kind it was given");
    };
    setup.probe = "VLOOP1".to_owned();
    setup.probe_reference = StbProbeReference::Placed;

    for draft in [stb, pstb] {
        let kind = draft.kind();
        let (_, painted) = render_analysis_form(draft, NoiseDomain::default());
        assert!(
            painted.iter().any(|line| line == "1 placed"),
            "{kind:?} must state how many probes the drawing holds: {painted:?}"
        );
    }
}

/// A design that places the given RF ports, resolved through the one
/// derivation the navigator and the studio also read.
#[cfg(not(target_arch = "wasm32"))]
fn placed_ports(ports: &[(&str, &str)]) -> Vec<crate::simulation::placed_sources::PlacedRfPort> {
    let mut schematic = crate::state::SchematicState::default();
    schematic.components = ports
        .iter()
        .enumerate()
        .map(|(index, (name, params))| {
            let mut component = crate::state::Component::new(
                index as u64 + 1,
                crate::state::ComponentType::RfPort,
                crate::state::Point::origin(),
            )
            .with_name_value(*name, "");
            component.params = (*params).to_owned();
            component
        })
        .collect();
    crate::simulation::placed_sources::placed_rf_ports(&schematic, None)
}

/// A fresh S-parameter setup, as inserting the analysis produces one.
#[cfg(not(target_arch = "wasm32"))]
fn sp_setup() -> crate::simulation::dialog::SpDialogState {
    let mut draft = AnalysisDraft::for_kind(AnalysisKind::SParameter);
    let AnalysisDraft::SParameter(setup) = &mut draft else {
        unreachable!("for_kind returns the draft of the kind it was given");
    };
    setup.ensure_initialized();
    setup.clone()
}

/// The S-parameter form asks the design which declaration owns its ports, and
/// a form nobody has told takes the design's answer: an RF bench reads its
/// placed ports, a netlist-first sheet keeps the node table.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_port_source_defaults_to_whichever_declaration_the_design_makes() {
    let bench = placed_ports(&[("P1", "port=1 z0=50"), ("P2", "port=2 z0=75")]);
    let (_, painted) = render_analysis_form_against(
        AnalysisDraft::SParameter(sp_setup()),
        NoiseDomain::default(),
        &bench,
    );
    assert!(
        painted.iter().any(|line| line == "From placed RF ports"),
        "a design that places ports enters placed mode: {painted:?}"
    );
    assert!(
        !painted.iter().any(|line| line == "Node +"),
        "and the ad-hoc node table goes quiet: {painted:?}"
    );

    let (_, painted) = render_analysis_form(
        AnalysisDraft::SParameter(sp_setup()),
        NoiseDomain::default(),
    );
    assert!(
        painted.iter().any(|line| line == "Ad-hoc node ports"),
        "a design that places none keeps the node table: {painted:?}"
    );
    assert!(painted.iter().any(|line| line == "Node +"), "{painted:?}");
}

/// The placed rows are the derivation's, read-only: number, reference, and
/// what the port does behind which impedance.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn placed_mode_states_each_placed_port_without_offering_to_edit_it() {
    let bench = placed_ports(&[("PIN", "port=1 ac_mag=1 z0=50"), ("POUT", "port=2 z0=75")]);
    let (_, painted) = render_analysis_form_against(
        AnalysisDraft::SParameter(sp_setup()),
        NoiseDomain::default(),
        &bench,
    );

    for expected in [
        "PORT 1",
        "PORT 2",
        "PIN",
        "POUT",
        "AC drive \u{00b7} Z0 50",
        "term \u{00b7} Z0 75",
    ] {
        assert!(
            painted.iter().any(|line| line == expected),
            "the placed roster must state {expected:?}: {painted:?}"
        );
    }
    assert!(
        !painted.iter().any(|line| line == "+ Add port"),
        "the design owns the roster, so the form does not add to it: {painted:?}"
    );
}

/// Every placed-mode refusal is stated on the form, beside the ports it is
/// about, in the same words dispatch refuses with.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn a_placed_roster_that_cannot_run_says_so_on_the_form() {
    let mut setup = sp_setup();
    setup.port_source_idx = Some(crate::simulation::dialog::SpPortSource::Placed.index());

    for (ports, expected) in [
        (vec![("PA", "port=1"), ("PB", "port=1")], "port number 1"),
        (vec![("PA", "port=1"), ("PB", "port=3")], "skip port 2"),
        (Vec::new(), "the sheet places none"),
    ] {
        let placed = placed_ports(&ports);
        let reason = setup
            .port_roster_error(&placed)
            .expect("this roster cannot run");
        assert!(reason.contains(expected), "{reason}");

        let (_, painted) = render_analysis_form_against(
            AnalysisDraft::SParameter(setup.clone()),
            NoiseDomain::default(),
            &placed,
        );
        assert!(
            painted.iter().any(|line| *line == reason),
            "the form must state the refusal dispatch will give: {painted:?}"
        );
    }
}

/// Ad-hoc mode beside placed ports states the true consequence: the deck's own
/// `P` cards are what the run measures, so the table below would not be used.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn ad_hoc_mode_beside_placed_ports_states_that_the_table_is_not_what_runs() {
    let mut setup = sp_setup();
    setup.port_source_idx = Some(crate::simulation::dialog::SpPortSource::AdHoc.index());
    let bench = placed_ports(&[("P1", "port=1"), ("P2", "port=2")]);

    let reason = setup
        .port_roster_error(&bench)
        .expect("two declarations of one thing is no run");
    assert!(reason.contains("would not be used"), "{reason}");

    // The node table plus its advisory runs past a 600-point viewport, and
    // egui does not paint what it can see is off-screen.
    let (_, painted) = render_analysis_form_into(
        &mut AnalysisDraft::SParameter(setup),
        NoiseDomain::default(),
        &bench,
        1200.0,
    );
    assert!(painted.iter().any(|line| *line == reason), "{painted:?}");
    assert!(
        painted.iter().any(|line| line == "Node +"),
        "the table stays visible, because switching back to it is a real answer: {painted:?}"
    );
}

/// Drawing the form must not record a choice nobody made: an untouched row
/// leaves the analysis following the design.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn drawing_the_form_does_not_stamp_a_port_source_choice() {
    let mut draft = AnalysisDraft::SParameter(sp_setup());
    render_analysis_form_into(
        &mut draft,
        NoiseDomain::default(),
        &placed_ports(&[("P1", "port=1"), ("P2", "port=2")]),
        VIEWPORT_HEIGHT,
    );
    let AnalysisDraft::SParameter(setup) = &mut draft else {
        unreachable!("the draft is the one just built");
    };
    assert_eq!(
        setup.port_source_idx, None,
        "an untouched row leaves the choice to the design"
    );
}
