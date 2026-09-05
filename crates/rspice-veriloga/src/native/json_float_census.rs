//! Whether every shipped model survives the JSON the browser worker is handed.
//!
//! A project or PDK Verilog-A runtime reaches the browser as
//! `PreparedVerilogARuntime`, which is two `serde_json` strings: the
//! [`CompiledModel`](crate::CompiledModel) and the
//! [`CanonicalIrArtifact`](crate::canonical_ir::CanonicalIrArtifact). JSON has
//! no spelling for an infinity or a NaN, so before
//! [`crate::json_float`] existed a model carrying one either refused to decode
//! or came back with a bound silently deleted — see that module for the two
//! failure modes and why they are not hypothetical.
//!
//! This census is the corpus-wide statement about the shipped estate. For every
//! shipped module it asks two questions:
//!
//! 1. what bare non-finite float would a JSON serializer still lose, and where
//!    (the census table, printed whether or not the answer is empty); and
//! 2. does the exact seal the runtime performs — `to_string`, `from_str` —
//!    reproduce the payload byte for byte?
//!
//! The second is the strong one: re-serializing what came back and comparing
//! bytes catches every value that did not survive, not only the non-finite
//! ones, and it is the same encode/decode pair `PreparedVerilogARuntime` runs.
//!
//! # What it found, and why it still runs
//!
//! Run first against the compiler *before* any field was annotated, all 43
//! modules answered `bare=0` and every payload was already a fixed point of its
//! own seal. The shipped corpus carries no non-finite float at all, so no
//! shipped model's sealed bytes move and no generated digest moves with them.
//!
//! `from (0:inf)` is why that is not the contradiction it looks like: the
//! parser reads `inf` in a range bound as *the absence of a bound*
//! (`RangeBound.upper: Option<Expression>`, `None` = `+inf`), so an open range
//! never becomes a float at all. The class is reachable from a project or PDK
//! source — `$bound_step`'s `+inf` reset and `inf` named in an ordinary
//! expression — which is exactly the browser path, and those are pinned on the
//! production seal in `rspice-ui`.
//!
//! So this census is a watch, not a repair: it fails the day a shipped model
//! acquires a non-finite float in a field nobody annotated, which is the day
//! the corpus stops sealing byte-identically.
//!
//! Ignored by default and run in release. It front-end compiles all 43 shipped
//! modules through [`shipped_census_models`], which is minutes of work, and it
//! is a release-qualification statement rather than a per-commit one.

use serde::Serialize;
use serde::de::DeserializeOwned;

use super::census_models::shipped_census_models;
use crate::json_float::non_finite_floats;

/// How many field paths one model prints before the row is truncated.
const PRINTED_PATH_LIMIT: usize = 12;

/// One payload's verdict.
struct Verdict {
    /// Bare non-finite floats a JSON serializer would still lose.
    bare: Vec<String>,
    /// Bytes the sealed payload occupies.
    bytes: usize,
    /// Whether the payload carries a value that only the string encoding can
    /// express. This is the positive control: a census that never sees one is
    /// not exercising the encoding at all.
    carries_encoded_non_finite: bool,
    /// Why the round trip failed, if it did.
    failure: Option<String>,
}

fn inspect<T>(label: &str, value: &T) -> Verdict
where
    T: Serialize + DeserializeOwned,
{
    let bare = match non_finite_floats(label, value) {
        Ok(found) => found.iter().map(ToString::to_string).collect(),
        Err(error) => {
            return Verdict {
                bare: Vec::new(),
                bytes: 0,
                carries_encoded_non_finite: false,
                failure: Some(format!("{label} could not be scanned: {error}")),
            };
        }
    };

    let sealed = match serde_json::to_string(value) {
        Ok(sealed) => sealed,
        Err(error) => {
            return Verdict {
                bare,
                bytes: 0,
                carries_encoded_non_finite: false,
                failure: Some(format!("{label} could not be sealed: {error}")),
            };
        }
    };
    let carries_encoded_non_finite = sealed.contains(r#":"inf""#)
        || sealed.contains(r#":"-inf""#)
        || sealed.contains(r#":"nan""#)
        || sealed.contains(r#""inf","#)
        || sealed.contains(r#""-inf","#)
        || sealed.contains(r#""nan","#);
    let bytes = sealed.len();

    let failure = match serde_json::from_str::<T>(&sealed) {
        Err(error) => Some(format!("{label} did not decode: {error}")),
        Ok(decoded) => match serde_json::to_string(&decoded) {
            Err(error) => Some(format!("{label} did not re-seal: {error}")),
            Ok(resealed) if resealed == sealed => None,
            Ok(resealed) => Some(format!(
                "{label} changed across the seal at byte {}: sealed {}, decoded {}",
                first_difference(&sealed, &resealed),
                window(&sealed, first_difference(&sealed, &resealed)),
                window(&resealed, first_difference(&sealed, &resealed)),
            )),
        },
    };

    Verdict {
        bare,
        bytes,
        carries_encoded_non_finite,
        failure,
    }
}

fn first_difference(left: &str, right: &str) -> usize {
    left.bytes()
        .zip(right.bytes())
        .position(|(left, right)| left != right)
        .unwrap_or_else(|| left.len().min(right.len()))
}

fn window(text: &str, at: usize) -> String {
    let start = text
        .char_indices()
        .map(|(index, _)| index)
        .take_while(|index| *index <= at.saturating_sub(40))
        .last()
        .unwrap_or(0);
    let end = text
        .char_indices()
        .map(|(index, _)| index)
        .find(|index| *index >= at + 40)
        .unwrap_or(text.len());
    format!("…{}…", &text[start..end])
}

/// The census. Prints one row per module, then refuses if any payload carries
/// a bare non-finite float or fails to survive its own seal.
#[test]
#[ignore = "front-end compiles all 43 shipped models; run in release"]
fn every_shipped_model_survives_the_json_the_browser_worker_is_handed() {
    let mut models = 0_usize;
    let mut carrying_non_finite = 0_usize;
    let mut bare_total = 0_usize;
    let mut sealed_bytes = 0_usize;
    let mut failures = Vec::new();
    let mut compile_seconds = 0.0;

    for shipped in shipped_census_models() {
        models += 1;
        compile_seconds += shipped.compile_seconds;

        let model = inspect("model", &shipped.model);
        let canonical_ir = inspect("canonical_ir", &shipped.canonical_ir);

        let bare: Vec<&String> = model.bare.iter().chain(canonical_ir.bare.iter()).collect();
        bare_total += bare.len();
        sealed_bytes += model.bytes + canonical_ir.bytes;
        if model.carries_encoded_non_finite || canonical_ir.carries_encoded_non_finite {
            carrying_non_finite += 1;
        }

        eprintln!(
            "json-float-census model={} bare={} encoded_non_finite={} model_bytes={} ir_bytes={} \
             compile_seconds={:.2} from_cache={}",
            shipped.name,
            bare.len(),
            model.carries_encoded_non_finite || canonical_ir.carries_encoded_non_finite,
            model.bytes,
            canonical_ir.bytes,
            shipped.compile_seconds,
            shipped.from_cache,
        );
        for path in bare.iter().take(PRINTED_PATH_LIMIT) {
            eprintln!("json-float-census   bare model={} {}", shipped.name, path);
        }
        if let Some(remaining) = bare.len().checked_sub(PRINTED_PATH_LIMIT)
            && remaining > 0
        {
            eprintln!(
                "json-float-census   bare model={} … and {remaining} more",
                shipped.name
            );
        }

        if !bare.is_empty() {
            failures.push(format!(
                "{} carries {} bare non-finite float(s): {}",
                shipped.name,
                bare.len(),
                bare.iter()
                    .take(PRINTED_PATH_LIMIT)
                    .map(|path| path.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
        for failure in [model.failure, canonical_ir.failure].into_iter().flatten() {
            failures.push(format!("{}: {failure}", shipped.name));
        }
    }

    eprintln!(
        "json-float-census models={models} carrying_non_finite={carrying_non_finite} \
         bare_total={bare_total} sealed_bytes={sealed_bytes} failures={} \
         compile_seconds={compile_seconds:.2}",
        failures.len()
    );

    assert_eq!(models, 43, "the shipped census is 43 modules");
    assert!(
        sealed_bytes > 1_000_000_000,
        "the corpus sealed only {sealed_bytes} bytes, so the payloads being compared are not \
         the shipped ones"
    );
    assert!(
        failures.is_empty(),
        "{} shipped payload(s) do not survive the seal:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
