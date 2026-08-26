//! What counts as a contested definition name.

/// A duplicate the manager settled is not contested.
///
/// `contested()` was `providers.len() > 1`, which marked every multi-provider
/// name — including the ones whose RESOLUTION column, three cells to the right,
/// already said "resolved · <library>". That inflated the card's own count of
/// what has to be repaired and offered a conflict dialog for a name that binds.
#[test]
fn a_definition_with_a_recorded_provider_is_not_contested() {
    use crate::state::model_library::ModelConsumerScope;

    let settled = super::DefinitionRow {
        definition: "nch".to_owned(),
        scope: ModelConsumerScope::PrimitiveModel,
        providers: vec!["foundry".to_owned(), "vendor".to_owned()],
        provider_list: "foundry, vendor".to_owned(),
        resolution: "resolved · foundry".to_owned(),
        resolved_provider: Some("foundry".to_owned()),
    };
    assert!(
        !settled.contested(),
        "a name with a recorded provider binds, so it is not what has to be repaired"
    );

    let unsettled = super::DefinitionRow {
        resolved_provider: None,
        resolution: "contested · fails closed".to_owned(),
        ..settled
    };
    assert!(
        unsettled.contested(),
        "two providers and no decision is the case that fails closed"
    );

    let unique = super::DefinitionRow {
        providers: vec!["foundry".to_owned()],
        provider_list: "foundry".to_owned(),
        resolution: "unique".to_owned(),
        resolved_provider: None,
        ..unsettled
    };
    assert!(!unique.contested(), "one provider is never contested");
}
