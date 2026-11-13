//! How an analysis form's quantity field normalizes what was typed: a
//! spelling the deck already reads survives the blur, an unparseable one is
//! left for its reader, and a temperature with a unit becomes the draft the
//! deck reads.
//!
//! The edits driven through a real route are among the simulate feature's
//! whole-application tests.

/// The two input policies a reader can be working under.
///
/// `StrictUnitsRequired` is the default and makes a bare number invalid in a
/// time or frequency field; `InferFromFieldQuantity` accepts one. Both reach
/// the same normalization, and each makes a different half of it reachable, so
/// both are asked.
fn policies() -> [crate::quantity::QuantityPresentationPolicy; 2] {
    use crate::quantity::QuantityPresentationPolicy;
    use rspice_vocabulary::quantity::TimeFrequencyInput;
    let strict = QuantityPresentationPolicy::default();
    let inferring = QuantityPresentationPolicy {
        time_frequency_input: TimeFrequencyInput::InferFromFieldQuantity,
        ..strict
    };
    [strict, inferring]
}

/// A spelling the deck already reads is kept exactly as it was typed.
///
/// Blurring rewrote every quantity field that parsed to `{value:.17e}`, so a
/// reader working under `InferFromFieldQuantity` — where a bare number is a
/// valid time — typed `1e-3` into Stop time and got `1.00000000000000005e-3`
/// back the moment focus moved. The row's own helper text says "engineering
/// notation"; a field that will not hold it is the one thing that cannot be
/// true there.
#[test]
fn a_spelling_the_deck_already_reads_survives_the_blur() {
    use crate::quantity::{QuantityInputKind, UiNumberLocale};

    for policy in policies() {
        for spelling in ["1e-3", "0.25", "2500u", "10n"] {
            let mut value = spelling.to_owned();
            let rewritten = super::super::analysis_form::normalize_quantity(
                &mut value,
                QuantityInputKind::Time,
                policy,
                UiNumberLocale::default(),
            );
            assert!(
                !rewritten && value == spelling,
                "{spelling:?} became {value:?}: the deck reads it as typed, so nothing here \
                 may restate it"
            );
        }
    }
}

/// A field the form cannot parse at all keeps what was typed, so the reader can
/// see the thing they have to correct.
#[test]
fn an_unparseable_field_is_left_for_its_reader_to_correct() {
    use crate::quantity::{QuantityInputKind, UiNumberLocale};

    let mut value = "banana".to_owned();
    assert!(!super::super::analysis_form::normalize_quantity(
        &mut value,
        QuantityInputKind::Time,
        crate::quantity::QuantityPresentationPolicy::default(),
        UiNumberLocale::default(),
    ));
    assert_eq!(value, "banana");
}

/// The temperature field's draft is bare Celsius, so a unit typed into it is
/// converted rather than kept.
#[test]
fn a_temperature_with_a_unit_is_converted_to_the_draft_the_deck_reads() {
    use crate::quantity::{QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale};

    let mut value = "25 \u{b0}C".to_owned();
    assert!(
        super::super::analysis_form::normalize_quantity(
            &mut value,
            QuantityInputKind::Temperature,
            QuantityPresentationPolicy::default(),
            UiNumberLocale::default(),
        ),
        "a unit the deck cannot read has to be rewritten"
    );
    assert_eq!(value, crate::state::format_engineering(25.0));
}
