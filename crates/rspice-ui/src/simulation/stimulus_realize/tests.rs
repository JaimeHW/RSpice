//! What the bridge has to agree with: the deck, the parser, and the evaluator.

use super::*;

use crate::state::stimulus_library::definition::{StimulusFamily, StimulusKind};
use crate::state::{PropertyRegistry, SchematicState, Wire};

fn timing(tstop: f64) -> PreviewTiming {
    PreviewTiming {
        tstep: tstop / 1000.0,
        tstop,
        from_analysis: true,
    }
}

/// A source seeded exactly as placement seeds one: nothing authored, so every
/// field resolves to the sheet's own default.
fn seeded(kind: ComponentType) -> Component {
    let mut component = Component::new(1, kind, Point::origin());
    component.name = format!("{}1", kind.spice_prefix());
    if kind.is_pwl_file_source() {
        component.value = "table.csv".to_owned();
    }
    component
}

#[test]
fn every_placeable_source_seeded_from_its_sheet_parses_as_the_engine_reads_it() {
    let registry = PropertyRegistry::new();
    for family in StimulusFamily::ALL {
        for kind in [StimulusKind::Voltage, StimulusKind::Current] {
            let component_type = family.component_type(kind);
            assert!(
                registry.get(component_type).is_some(),
                "{component_type:?} has no property sheet"
            );
            let component = seeded(component_type);
            let spec = source_spec(&component)
                .unwrap_or_else(|error| panic!("{component_type:?}: {error}"));
            assert!(
                !matches!(spec, SourceSpec::RfPort { .. }),
                "{component_type:?} is not a port"
            );
        }
    }
}

/// The card the bridge writes has to be the card the deck writes, or a preview
/// is showing a source the run will not have.
#[test]
fn the_bridge_writes_the_card_the_generated_deck_carries() {
    let mut schematic = SchematicState::default();
    let mut source = Component::new(1, ComponentType::VoltageSourcePulse, Point::new(4, 4));
    source.name = "V1".to_owned();
    source.value = "0".to_owned();
    source.params = "v2=5 tr=1n tf=1n pw=1u per=2u".to_owned();
    schematic.components.push(source.clone());

    let generator_card = source_card_text(&source, ["in", "0"]).expect("card");
    assert_eq!(generator_card, "V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)");

    // The same instance inside a real deck emits the same specification; only
    // the nets differ, because the sheet resolves those and the bridge names
    // them.
    schematic
        .wires
        .push(Wire::new(1, vec![Point::new(4, 4), Point::new(8, 4)]));
    let deck = crate::simulation::netlist_gen::NetlistGenerator::new(&schematic).generate();
    let deck_card = deck
        .lines()
        .find(|line| line.starts_with("V1 "))
        .expect("the deck emits V1");
    let specification = specification_of(deck_card).expect("a specification");
    assert_eq!(specification, "PULSE(0 5 0 1n 1n 1u 2u)");
}

#[test]
fn provenance_never_reaches_the_card() {
    let mut plain = Component::new(1, ComponentType::VoltageSourceSin, Point::origin());
    plain.name = "V1".to_owned();
    plain.params = "va=3m freq=1k".to_owned();

    let mut adopted = plain.clone();
    let definition = crate::state::stimulus_library::definition::StimulusDefinition::new(
        "sensor_diff_1k",
        ComponentType::VoltageSourceSin,
    )
    .expect("definition");
    let mut copy = definition.clone();
    copy.params = "va=3m freq=1k".to_owned();
    copy.adopt_onto(&mut adopted).expect("adopt");

    assert!(adopted.stimulus_provenance.is_some());
    assert_eq!(
        source_card_text(&adopted, ["in", "0"]),
        source_card_text(&plain, ["in", "0"])
    );
}

#[test]
fn a_definition_realizes_through_the_instance_it_would_be_adopted_onto() {
    let mut definition = crate::state::stimulus_library::definition::StimulusDefinition::new(
        "sensor_diff_1k",
        ComponentType::VoltageSourceSin,
    )
    .expect("definition");
    definition.value = "0".to_owned();
    definition.params = "va=3m freq=1k".to_owned();

    let card = definition.card_text(["p", "n"]).expect("card");
    assert_eq!(card, "sensor_diff_1k p n SIN(0 3m 1k 0 0 0)");

    let mut instance = Component::new(9, ComponentType::VoltageSourceSin, Point::origin());
    instance.name = "V9".to_owned();
    definition.adopt_onto(&mut instance).expect("adopt");
    assert_eq!(
        source_card_text(&instance, ["p", "n"]).expect("card"),
        "V9 p n SIN(0 3m 1k 0 0 0)"
    );
}

#[test]
fn a_component_that_is_not_an_independent_source_is_not_realized() {
    assert!(!is_independent_source(ComponentType::Resistor));
    assert!(!is_independent_source(ComponentType::BehavioralSource));
    assert!(is_independent_source(ComponentType::CurrentSourcePwlFile));
}

/// The ngspice-46/47 pin. `PULSE(V1 V2 TD TR TF)` with the width omitted and
/// both edges authored resolves to PW = 0 and PER = TSTOP: one triangle of zero
/// width, then V1 for the rest of the run. The old hand-rolled sampler read the
/// same card as a 50 % square wave.
///
/// The card is spelled here rather than emitted from a placed component,
/// because the netlist generator cannot write this form: `format_source_value`
/// substitutes its own `pw=1u per=2u` for an omitted width and period, so the
/// only way an RSpice project holds an edges-only pulse is a deck someone
/// wrote. The bridge reads such a deck's source exactly as the engine does,
/// which is what this pins.
#[test]
fn an_edges_only_pulse_previews_as_one_zero_width_pulse_then_v1() {
    let spec =
        rspice_core::netlist::parse_source_spec_text("PULSE(0 5 0 1n 1n)", 0, &ParamContext::new())
            .expect("spec");
    let timing = timing(1e-3);
    let peak = evaluate_waveform(
        &spec,
        PreviewWindow {
            start: 0.5e-9,
            stop: 0.5e-9,
            samples: 2,
        },
        timing.tstep,
        timing.tstop,
        PREVIEW_DIALECT,
    );
    assert!(
        (peak[0].1 - 2.5).abs() < 1e-9,
        "halfway up the rising edge is halfway to V2: {peak:?}"
    );

    // Every later sample is back at V1, for the whole stop time — the period is
    // TSTOP, so the triangle never comes round again.
    let run = evaluate_waveform(
        &spec,
        timing.window(64),
        timing.tstep,
        timing.tstop,
        PREVIEW_DIALECT,
    );
    assert!(
        run.iter().skip(1).all(|(_, value)| value.abs() < 1e-12),
        "a zero-width pulse contributes nothing after its edges: {run:?}"
    );
    let readouts = WaveformReadouts::of(&run).expect("readouts");
    assert!(readouts.span() < 1e-12);
}

/// A pulse that authors its width is the square wave it looks like, so the
/// rule above is a rule about omission and not about pulses.
#[test]
fn an_authored_pulse_width_still_previews_as_a_train() {
    let mut source = Component::new(1, ComponentType::VoltageSourcePulse, Point::origin());
    source.name = "V1".to_owned();
    source.value = "0".to_owned();
    source.params = "v2=5 tr=1n tf=1n pw=1u per=2u".to_owned();

    let spec = source_spec(&source).expect("spec");
    let samples = evaluate_waveform(
        &spec,
        PreviewWindow {
            start: 0.0,
            stop: 4e-6,
            samples: 401,
        },
        1e-9,
        1e-3,
        PREVIEW_DIALECT,
    );
    let readouts = WaveformReadouts::of(&samples).expect("readouts");
    assert!((readouts.maximum - 5.0).abs() < 1e-9, "{readouts:?}");
    assert!(readouts.minimum.abs() < 1e-9, "{readouts:?}");
}

/// The SFFM pin. An authored `FC=0` is read as omitted and the engine puts
/// `5 / TSTOP` in its place, so the same card previews differently under two
/// stop times — which is exactly why the card names the transient it used.
#[test]
fn an_sffm_carrier_of_zero_previews_with_the_engine_tstop_substitution() {
    let mut source = Component::new(1, ComponentType::VoltageSourceSffm, Point::origin());
    source.name = "V1".to_owned();
    source.value = "0".to_owned();
    source.params = "va=1 fc=0 mdi=0 fm=1k".to_owned();

    let spec = source_spec(&source).expect("spec");
    let window = PreviewWindow {
        start: 0.0,
        stop: 1e-3,
        samples: 129,
    };
    let fast = evaluate_waveform(&spec, window, 1e-6, 1e-3, PREVIEW_DIALECT);
    let slow = evaluate_waveform(&spec, window, 1e-6, 1e-1, PREVIEW_DIALECT);

    assert_ne!(
        fast, slow,
        "an omitted carrier resolves against TSTOP, so the two runs cannot agree"
    );
    let fast_span = WaveformReadouts::of(&fast).expect("readouts").span();
    let slow_span = WaveformReadouts::of(&slow).expect("readouts").span();
    assert!(
        fast_span > slow_span,
        "5 / TSTOP is a faster carrier at the shorter stop time: {fast_span} vs {slow_span}"
    );
}

#[test]
fn a_file_backed_source_says_what_its_preview_is_missing() {
    let mut source = Component::new(1, ComponentType::VoltageSourcePwlFile, Point::origin());
    source.name = "V1".to_owned();
    source.value = "bridge_step.csv".to_owned();

    let spec = source_spec(&source).expect("spec");
    let defect = preview_defect(&spec).expect("a stated defect");
    assert!(defect.contains("bridge_step.csv"), "{defect}");

    let mut sine = Component::new(2, ComponentType::VoltageSourceSin, Point::origin());
    sine.name = "V2".to_owned();
    assert!(preview_defect(&source_spec(&sine).expect("spec")).is_none());
}

/// The engine's spec evaluator returns exactly 0 for TRNOISE — the train is
/// built when the transient is, not read out of the spec — so a card that drew
/// it would be drawing a flat line and calling it noise.
#[test]
fn a_noise_source_says_it_has_no_waveform_until_a_run_builds_one() {
    let mut source = Component::new(1, ComponentType::CurrentSourceNoise, Point::origin());
    source.name = "I1".to_owned();
    source.params = "dc=2m na=1n nt=1u".to_owned();

    let spec = source_spec(&source).expect("spec");
    let defect = preview_defect(&spec).expect("a stated defect");
    assert!(defect.contains("TRNOISE"), "{defect}");

    let samples = evaluate_waveform(&spec, timing(1e-3).window(16), 1e-6, 1e-3, PREVIEW_DIALECT);
    assert!(
        samples.iter().all(|(_, value)| *value == 0.0),
        "the defect is stated because the evaluator has nothing: {samples:?}"
    );
}

/// The TRRANDOM card, byte for byte, and the spec the engine's own parser makes
/// of it.
///
/// This is the pin for the whole family: the sheet spells the distribution,
/// the generator writes the integer the card carries, and every one of the five
/// positional fields arrives in the engine holding what was authored. A `1.0`
/// stays `1.0` — the emitter passes authored text through rather than
/// re-formatting it, which is what keeps a definition's realization line and a
/// placed adopter's deck line the same string.
#[test]
fn a_random_source_writes_the_card_the_engine_parses_field_for_field() {
    let mut source = Component::new(1, ComponentType::VoltageSourceRandom, Point::origin());
    source.name = "V1".to_owned();
    source.params = "type=gaussian ts=1u td=0 param1=1.0 param2=0".to_owned();

    assert_eq!(
        source_card_text(&source, ["in", "0"]).expect("card"),
        "V1 in 0 TRRANDOM(2 1u 0 1.0 0)"
    );

    let spec = source_spec(&source).expect("spec");
    let SourceSpec::TrRandom {
        distribution,
        sample_interval,
        delay,
        parameter1,
        parameter2,
    } = spec
    else {
        panic!("a TRRANDOM card parses as a TRRANDOM spec: {spec:?}");
    };
    assert_eq!(distribution, 2);
    assert_eq!(sample_interval, 1e-6);
    assert_eq!(delay, 0.0);
    assert_eq!(parameter1, 1.0);
    assert_eq!(parameter2, 0.0);
}

/// A missing file the generator *can* check is refused before there is a spec
/// at all, in the generator's own words, which name the component and the path
/// the user typed.
#[test]
fn an_absolute_data_file_that_is_not_there_is_refused_at_the_card() {
    let mut source = Component::new(1, ComponentType::CurrentSourcePwlFile, Point::origin());
    source.name = "I1".to_owned();
    source.value = "C:/nowhere/bridge_step.csv".to_owned();

    let error = source_spec(&source).expect_err("the card cannot be written");
    assert!(error.contains("bridge_step.csv"), "{error}");
}

#[test]
fn the_timing_caption_names_the_transient_and_whether_the_plan_holds_one() {
    assert_eq!(
        PreviewTiming {
            tstep: 1e-8,
            tstop: 1e-3,
            from_analysis: true,
        }
        .caption(),
        "TRAN 1ms · TSTEP 10ns"
    );
    assert!(
        PreviewTiming {
            tstep: 1e-8,
            tstop: 1e-3,
            from_analysis: false,
        }
        .caption()
        .ends_with("· plan default")
    );
}

#[test]
fn a_window_of_fewer_than_two_samples_is_no_curve() {
    let mut source = Component::new(1, ComponentType::VoltageSource, Point::origin());
    source.name = "V1".to_owned();
    source.value = "1.8".to_owned();
    let spec = source_spec(&source).expect("spec");

    assert!(
        evaluate_waveform(
            &spec,
            PreviewWindow {
                start: 0.0,
                stop: 1e-3,
                samples: 1,
            },
            1e-9,
            1e-3,
            PREVIEW_DIALECT,
        )
        .is_empty()
    );
    assert!(WaveformReadouts::of(&[]).is_none());
}
