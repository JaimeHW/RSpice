use super::*;

fn parsed(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("deck parses")
}

#[test]
fn trials_are_the_cards_own_numbers_varied_as_the_deck_states() {
    let deck =
        "mc\n.param r={agauss(1k,100,1)}\nV1 a 0 1\nR1 a 0 {r}\n.mc 3 seed 9\n.tran 1n 10n\n.end\n";
    let members = monte_carlo_members(&parsed(deck), deck_states_statistics(deck)).unwrap();
    let indices: Vec<_> = members
        .iter()
        .map(|member| match member.member {
            TransientMember::MonteCarloTrial {
                index,
                variation_source: McVariationSource::DeckStatistics,
            } => index,
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(indices, [0, 1, 2]);
    assert!(members.iter().all(|member| member.coordinate.is_empty()));

    let draws =
        member_parameters(&Engine::new(Default::default()), &parsed(deck), &members).unwrap();
    assert_eq!(draws.len(), 3);
    assert_ne!(draws[0]["r"], draws[1]["r"], "each trial draws its own r");
}

#[test]
fn a_deck_with_no_statistics_and_no_named_parameter_is_refused() {
    let deck = "mc\n.param r=1k vcc=5\nV1 a 0 {vcc}\nR1 a 0 {r}\n.mc 3\n.tran 1n 10n\n.end\n";
    assert!(!deck_states_statistics(deck));
    let refusal = monte_carlo_members(&parsed(deck), false).unwrap_err();
    assert!(
        refusal.contains("refusing to perturb every .param"),
        "{refusal}"
    );
    // A statistic in a comment is not the deck's variation.
    assert!(!deck_states_statistics(
        "* r={agauss(1k,100,1)}\nR1 a 0 1k $ agauss(1,2,3)\n"
    ));
}

#[test]
fn points_carry_the_value_each_swept_axis_takes() {
    let deck = "step\n.param r=1k\nV1 a 0 1\nR1 a 0 {r}\n.step param r list 1k 2k 3k\n.tran 1n 10n\n.end\n";
    let netlist = parsed(deck);
    let members = step_members(&netlist, &ResourceLimits::default()).unwrap();
    assert_eq!(members.len(), 3);
    for (index, (member, value)) in members.iter().zip([1e3, 2e3, 3e3]).enumerate() {
        assert_eq!(member.member, TransientMember::StepPoint { index });
        assert_eq!(member.coordinate, [("r".to_owned(), value)]);
    }
    let resolved = member_parameters(&Engine::new(Default::default()), &netlist, &members).unwrap();
    assert_eq!(resolved[2]["r"], 3e3);
}
