//! Existing retained-model vocabulary compatibility case.

use super::*;

/// A projection written before this build could name a family still opens.
///
/// The card's retained bytes are what the closure authenticates, and the token
/// they declare is compared exactly here — so a project whose only difference
/// is that its writer had no `Njfet` to write is a faithful projection, not a
/// tampered one. It is accepted in that one direction only: a persisted
/// classification that says something *else* is still a mismatch, and so is
/// one that claims a family the reparse did not reach.
#[test]
fn a_projection_classified_by_the_older_vocabulary_still_matches() {
    let card = |model_type, spice_type: &str| {
        let mut card = DeviceModel::new("JMOD", model_type);
        card.spice_type = Some(spice_type.to_owned());
        card
    };

    let parsed = card(ModelType::Njfet, "NJF");
    assert!(parsed_model_projection_matches_without_section(
        &card(ModelType::Other, "NJF"),
        &parsed
    ));
    assert!(parsed_model_projection_matches_without_section(
        &card(ModelType::Njfet, "NJF"),
        &parsed
    ));
    assert!(
        !parsed_model_projection_matches_without_section(&card(ModelType::Pjfet, "NJF"), &parsed),
        "the wrong polarity is a mismatch, not an older spelling"
    );
    assert!(
        !parsed_model_projection_matches_without_section(
            &card(ModelType::Njfet, "NJF"),
            &card(ModelType::Other, "NJF")
        ),
        "the widening is one-directional: a claimed family the reparse did not \
         reach is a mismatch"
    );
    assert!(
        !parsed_model_projection_matches_without_section(
            &card(ModelType::Other, "NJF"),
            &card(ModelType::Nmos, "NMOS")
        ),
        "only the families added with this vocabulary are excused"
    );
}

#[test]
fn current_schema_rejects_every_retired_singleton_analysis_field() {
    let context =
        ProjectExecutionContext::from_setup(SimulationSetup::new(), Vec::new(), Vec::new(), None)
            .expect("baseline context validates");
    let baseline = serde_json::to_value(context).expect("context serializes");

    for field in RETIRED_SINGLETON_ANALYSIS_FIELDS {
        let mut value = baseline.clone();
        value["simulation_plan"]
            .as_object_mut()
            .expect("simulation plan is an object")
            .insert((*field).to_owned(), serde_json::Value::Null);
        let error = serde_json::from_value::<ProjectExecutionContext>(value)
            .expect_err("current schema must reject retired singleton input")
            .to_string();
        assert!(
            error.contains(&format!("retired singleton field `{field}`")),
            "{error}"
        );
    }
}
