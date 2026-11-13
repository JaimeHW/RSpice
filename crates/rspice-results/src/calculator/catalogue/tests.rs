use super::*;
use crate::calculator::evaluator::{EvaluationError, evaluate};
use crate::calculator::parser::try_parse;
use crate::calculator::retained::WaveformsContext;

fn plain_number(text: &str) -> Result<f64, String> {
    text.parse().map_err(|error| format!("{error}"))
}

fn fields(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

/// The catalogue may only offer what the evaluator runs, with the argument
/// count the evaluator accepts: one argument too many must be refused as
/// exactly one too many, by every spelling.
#[test]
fn every_entry_is_a_function_the_evaluator_runs_with_the_catalogued_arity() {
    let waveforms: [crate::waveform::RetainedWaveform; 0] = [];
    for spec in MEASUREMENTS {
        for name in std::iter::once(&spec.name).chain(spec.aliases) {
            let too_many = vec!["1"; spec.arguments.len() + 1].join(", ");
            let parsed = try_parse(&format!("{name}({too_many})"), plain_number).unwrap();
            match evaluate(&parsed, &WaveformsContext::new(&waveforms)) {
                Err(EvaluationError::ArgCountMismatch {
                    expected, actual, ..
                }) => assert_eq!(
                    (expected, actual),
                    (spec.arguments.len(), spec.arguments.len() + 1),
                    "{name} takes a different number of arguments than the catalogue says"
                ),
                other => panic!("{name} with too many arguments returned {other:?}"),
            }
        }
    }
}

/// A level is read in the unit of the signal it is a level of, so it must
/// name a signal argument, and the first signal's level comes before the
/// second's.
#[test]
fn every_level_names_the_signal_whose_unit_it_takes() {
    for spec in MEASUREMENTS {
        for argument in spec.arguments {
            if let ArgumentKind::Level(signal) = argument.kind {
                assert_eq!(
                    spec.arguments.get(signal).map(|signal| signal.kind),
                    Some(ArgumentKind::Signal),
                    "{}'s {} is a level of no signal",
                    spec.name,
                    argument.label
                );
            }
        }
    }
    let delay = find("delay").unwrap();
    let levels: Vec<_> = delay
        .arguments
        .iter()
        .filter_map(|argument| match argument.kind {
            ArgumentKind::Level(signal) => Some((argument.short, signal)),
            _ => None,
        })
        .collect();
    assert_eq!(levels, [("level1", 0), ("level2", 1)]);
}

#[test]
fn signatures_name_every_argument_and_fold_the_optional_ones() {
    let rise = find("riseTime").unwrap();
    assert_eq!(rise.name, "rise");
    assert_eq!(rise.signature(), "rise(x, from, to, n, initial, final)");
    assert_eq!(rise.short_signature(), "rise(x, …)");
    assert_eq!(
        find("CROSS").unwrap().short_signature(),
        "cross(x, level, …)"
    );
    assert_eq!(
        find("gainmargin").unwrap().short_signature(),
        "gainMargin(loop)"
    );
    assert!(
        find("deriv").is_none(),
        "a series transform is not a measurement"
    );
}

#[test]
fn a_call_spells_skipped_defaults_and_drops_trailing_ones() {
    let cross = find("cross").unwrap();
    assert_eq!(
        cross.call(&fields(&["V(out)", "2.5"])).unwrap(),
        "cross(V(out), 2.5)"
    );
    assert_eq!(
        cross
            .call(&fields(&["V(out)", "2.5", "", "falling"]))
            .unwrap(),
        "cross(V(out), 2.5, 1, \"falling\")"
    );
    let delay = find("delay").unwrap();
    assert_eq!(
        delay
            .call(&fields(&[
                "V(in)", "V(out)", "1.65", "rising", "", "", "falling"
            ]))
            .unwrap(),
        "delay(V(in), V(out), 1.65, \"rising\", 1, 1.65, \"falling\")"
    );
    assert_eq!(
        delay
            .call(&fields(&["V(in)", "V(out)", "", "rising"]))
            .unwrap_err(),
        "Give level 1 to set Edge type 1."
    );
    assert_eq!(
        cross.call(&fields(&["", "2.5"])).unwrap_err(),
        "Signal is required."
    );
    assert_eq!(
        cross.call(&fields(&["V(out)"])).unwrap_err(),
        "Level is required."
    );
}

#[test]
fn a_catalogued_call_reads_back_into_its_fields() {
    let (spec, values) = split_call("cross(V(out), 2.5, 1, \"rising\")").unwrap();
    assert_eq!(spec.name, "cross");
    assert_eq!(values, fields(&["V(out)", "2.5", "1", "rising"]));
    let (spec, values) = split_call(" duty( clip(V(a), 1u, 2u) ) ").unwrap();
    assert_eq!(spec.name, "dutyCycle");
    assert_eq!(values, fields(&["clip(V(a), 1u, 2u)"]));
    let (_, values) = split_call("rms()").unwrap();
    assert!(values.is_empty());
    for not_one_call in [
        "rms(V(a)) * 2",
        "max(V(a)) - min(V(b))",
        "deriv(V(a))",
        "rms(V(a), 1)",
        "cross(V(a), 1, 1, \"rising)",
    ] {
        assert!(
            split_call(not_one_call).is_none(),
            "{not_one_call} read as one catalogued call"
        );
    }
    // Whatever the fields spell reads back as the same fields.
    let delay = find("delay").unwrap();
    let typed = fields(&["V(in)", "V(out)", "1.65", "rising", "2"]);
    let (_, read) = split_call(&delay.call(&typed).unwrap()).unwrap();
    assert_eq!(read, typed);
}
