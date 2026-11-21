//! Frame cost of the waveform strip on very large results.
//!
//! Two loads stand for the results that break a waveform viewer: many
//! unrelated signals on one time grid, and a family of many members of one
//! quantity. The gates count work rather than time it: how many envelopes a
//! frame rebuilt from the retained samples. Wall clock on CI is noise; a
//! rebuild count is not.

use super::*;

use rspice_ui_kit::plot::DecimationWork;

/// The window the measured frames are drawn into, as in the idle-frame gates.
const SCREEN: egui::Vec2 = egui::vec2(1_680.0, 1_020.0);

/// One transient analysis holding `traces` unrelated waveforms of `samples`
/// points on one shared time grid.
///
/// Each is its own signal, with its own gain and phase on one shape, so the
/// curves overlap as densely as a family's members do and the pane fits one
/// Y range to all of them; but no family model relates them.
fn crowded_transient(traces: usize, samples: usize) -> ResultsTestHost {
    transient_in_units(traces, samples, &["V"])
}

/// [`crowded_transient`] with `traces` members in each of `units`: one pane
/// per unit, every curve on the same time grid.
fn transient_in_units(traces: usize, samples: usize, units: &[&str]) -> ResultsTestHost {
    let time: Arc<Vec<f64>> = Arc::new((0..samples).map(|index| index as f64 * 1.0e-9).collect());
    // About three periods across the window whatever the sample count, so
    // every column of the strip holds a real minimum and maximum.
    let step = 20.0 / samples as f64;
    let waveforms = units
        .iter()
        .enumerate()
        .flat_map(|(pane, unit)| (0..traces).map(move |member| (pane, *unit, member)))
        .map(|(pane, unit, member)| {
            let spread = member as f64 / traces.max(1) as f64 - 0.5;
            let gain = 0.1f64.mul_add(spread, 1.0);
            let phase = 0.4 * spread;
            let values: Vec<f64> = (0..samples)
                .map(|index| gain * (index as f64).mul_add(step, phase).sin())
                .collect();
            RetainedTrace::new(
                format!("p{pane}m{member}"),
                Arc::clone(&time),
                values,
                "#00aaff",
            )
            .with_unit(unit)
        })
        .collect();
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(waveforms),
    );
    state
}

/// A family of `members` curves of `samples` points: one quantity, `V(out)`,
/// solved at `members` temperatures against one swept input, presented
/// through the strip's family model as one curve per member.
///
/// Each member has its own gain and phase, as [`crowded_transient`]'s curves
/// do, so the members overlap the way a real family's do.
fn family_of(members: usize, samples: usize) -> ResultsTestHost {
    family_of_signals(members, samples, &["V(out)"])
}

/// [`family_of`] answering every one of `signals`, each a quarter period
/// behind the one before it, so any two of them trace a loop.
pub(super) fn family_of_signals(
    members: usize,
    samples: usize,
    signals: &[&str],
) -> ResultsTestHost {
    let step = 20.0 / samples as f64;
    let rows = members * samples;
    let (mut x, mut ys, mut temperatures) = (
        Vec::with_capacity(rows),
        vec![Vec::with_capacity(rows); signals.len()],
        Vec::with_capacity(rows),
    );
    for member in 0..members {
        let spread = member as f64 / members.max(1) as f64 - 0.5;
        let gain = 0.1f64.mul_add(spread, 1.0);
        let phase = 0.4 * spread;
        for index in 0..samples {
            x.push(index as f64 * 1.0e-3);
            for (lag, y) in ys.iter_mut().enumerate() {
                let behind = lag as f64 * std::f64::consts::FRAC_PI_2;
                y.push(gain * (index as f64).mul_add(step, phase - behind).sin());
            }
            temperatures.push(member as f64);
        }
    }
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Corner, "PVT")
            .with_waveforms(
                signals
                    .iter()
                    .zip(ys)
                    .map(|(name, y)| {
                        RetainedTrace::new(*name, x.clone(), y, "#00aaff").with_unit("V")
                    })
                    .collect(),
            )
            .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
                member_measurements: Vec::new(),
                x_values: x,
                x_label: "VIN".to_owned(),
                x_unit: "V".to_owned(),
                temperatures_c: temperatures,
                corner_labels: vec!["TT".to_owned(); rows],
                failed_corners: 0,
            }),
    );
    let manifest = FamilyManifest::from_metadata(
        run.analyses[0].analysis_type,
        run.analyses[0].family_metadata.as_ref(),
    )
    .expect("the corner metadata is valid")
    .expect("a corner result is a family");
    let temperature = FamilyDimension::new("temperature", ValueType::Real).expect("dimension");
    let policy = FamilyPresentationPolicy {
        x_dimension: FamilyXDimension {
            dimension: FamilyDimension::new("VIN", ValueType::Real).expect("dimension"),
            ordering: FamilyXOrdering::Source,
        },
        family_dimensions: vec![temperature.clone()],
        facet_layout: None,
        aggregation: FamilyAggregationPolicy {
            method: FamilyAggregationMethod::None,
            over_dimensions: Vec::new(),
        },
        filter: None,
        missing_points: MissingPointPolicy::ExcludeWithOmissionRecord,
        // Labelled, not coloured: every member keeps the quantity's hue, as
        // a family drawn for its spread is.
        encodings: vec![FamilyEncodingMap::Label {
            dimension: temperature,
            prefix: None,
        }],
    };
    let selection = SourceSampleSelection::new(run.dataset_id, 1, (0..rows).collect())
        .expect("every row is selected")
        .with_family_presentation(&manifest, &policy)
        .expect("the policy presents the family");
    state.results.results.set_sample_selection(Some(selection));
    state
}

/// What one frame cost.
#[derive(Debug, Clone, Copy)]
struct FrameCost {
    vertices: usize,
    work: DecimationWork,
}

fn new_context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// Run one frame of the waveform strip with `events` as its only input.
fn frame(ctx: &egui::Context, state: &mut ResultsTestHost, events: Vec<egui::Event>) -> FrameCost {
    let before = state.results.results.session.cache.work();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
            events,
            ..Default::default()
        },
        |ctx| {
            // The workspace prepares its caches once per frame before any
            // sheet draws; that is what advances the envelope cache's frame.
            crate::workbench::documents::result_document::prepare_viewer_state(
                &mut state.results_parts(),
            );
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
        },
    );
    let primitives = ctx.tessellate(output.shapes, output.pixels_per_point);
    let vertices = primitives
        .iter()
        .map(|primitive| match &primitive.primitive {
            egui::epaint::Primitive::Mesh(mesh) => mesh.vertices.len(),
            egui::epaint::Primitive::Callback(_) => 0,
        })
        .sum();
    FrameCost {
        vertices,
        work: state.results.results.session.cache.work().since(before),
    }
}

/// A pointer sweep across the plot, one position per frame.
fn hover_positions(count: usize) -> Vec<egui::Pos2> {
    (0..count)
        .map(|index| {
            let fraction = (index as f32 + 0.5) / count as f32;
            egui::pos2(fraction.mul_add(1_100.0, 400.0), 420.0)
        })
        .collect()
}

/// A family too large to stroke member by member is drawn as the band it
/// occupies. Its warm frames read nothing from the retained samples, and its
/// hovered frames build at most the one envelope of the member newly under
/// the pointer — never the family's.
#[test]
fn a_large_family_draws_its_band_and_rebuilds_nothing_warm_or_hovered() {
    let mut state = family_of(100, 1_000);
    let ctx = new_context();
    let cold = frame(&ctx, &mut state, Vec::new());
    assert_eq!(
        cold.work.rebuilds, 1,
        "the cold frame folds the family into one band, not 100 envelopes"
    );
    for _ in 0..3 {
        let warm = frame(&ctx, &mut state, Vec::new());
        assert_eq!(warm.work.rebuilds, 0, "a warm frame rebuilt an envelope");
        // A hundred members of a thousand points stroked one by one are
        // hundreds of thousands of vertices; the band and its edges are
        // thousands.
        assert!(
            warm.vertices < 100_000,
            "a 100-member family tessellated {} vertices",
            warm.vertices
        );
    }
    let sweep = hover_positions(12);
    let first: Vec<FrameCost> = sweep
        .iter()
        .map(|position| frame(&ctx, &mut state, vec![egui::Event::PointerMoved(*position)]))
        .collect();
    assert!(
        first.iter().all(|cost| cost.work.rebuilds <= 1),
        "a hover rebuilt more than the curve under the pointer: {:?}",
        first
            .iter()
            .map(|cost| cost.work.rebuilds)
            .collect::<Vec<_>>()
    );
    assert!(
        first
            .iter()
            .all(|cost| cost.work.hits + cost.work.rebuilds == 2),
        "every hovered frame draws the band and the one curve under the pointer: {:?}",
        first.iter().map(|cost| cost.work).collect::<Vec<_>>()
    );
    // The same sweep again finds every curve it highlights already reduced.
    for position in &sweep {
        let again = frame(&ctx, &mut state, vec![egui::Event::PointerMoved(*position)]);
        assert_eq!(
            again.work.rebuilds, 0,
            "a repeated hover rebuilt an envelope"
        );
    }
}

/// A hundred unrelated curves in one pane are stroked one by one, never
/// folded into a band: a band says its members are one quantity spread by
/// what varied between them, which is false of unrelated signals. Every
/// frame reads each curve's own envelope, and no frame reads a band.
#[test]
fn a_hundred_unrelated_curves_are_never_drawn_as_a_band() {
    let mut state = crowded_transient(100, 1_000);
    let ctx = new_context();
    let cold = frame(&ctx, &mut state, Vec::new());
    assert_eq!(
        (cold.work.hits, cold.work.rebuilds),
        (0, 100),
        "the cold frame reduces every curve on its own"
    );
    let warm = frame(&ctx, &mut state, Vec::new());
    assert_eq!(
        (warm.work.hits, warm.work.rebuilds),
        (100, 0),
        "a warm frame strokes every curve from its own envelope"
    );
    for position in hover_positions(6) {
        let hover = frame(&ctx, &mut state, vec![egui::Event::PointerMoved(position)]);
        assert_eq!(
            (hover.work.hits, hover.work.rebuilds),
            (100, 0),
            "a hovered frame strokes every curve, and reads no band"
        );
    }
}

/// A strip that strokes more curves than the envelope cache's stale bound —
/// five panes of sixty — reuses every envelope on its warm and hovered
/// frames. With a hard 256-entry bound it evicted and rebuilt most of them on
/// every repaint.
#[test]
fn a_strip_stroking_more_curves_than_the_cache_bound_rebuilds_nothing_warm_or_hovered() {
    let mut state = transient_in_units(60, 1_000, &["V", "A", "W", "F", "H"]);
    let ctx = new_context();
    let cold = frame(&ctx, &mut state, Vec::new());
    assert_eq!(cold.work.rebuilds, 300, "every curve is reduced once");
    for _ in 0..3 {
        let warm = frame(&ctx, &mut state, Vec::new());
        assert_eq!(
            (warm.work.rebuilds, warm.work.evictions),
            (0, 0),
            "a warm frame rebuilt or evicted an envelope"
        );
    }
    for position in hover_positions(6) {
        let hover = frame(&ctx, &mut state, vec![egui::Event::PointerMoved(position)]);
        assert_eq!(hover.work.rebuilds, 0, "a hover rebuilt an envelope");
    }
}
