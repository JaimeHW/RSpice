//! Small-signal analyses around an accepted transient state (P1 design S10, §5).
//!
//! A card that carries `ACTIMES=` linearizes at the accepted state of the
//! deck's own `.TRAN` run at each requested time; one that carries
//! `CHECKPOINT=` linearizes at the state a checkpoint file holds. The oracles
//! are closed forms and held-state twins: a deck whose operating point *is*
//! the transient state, solved as an ordinary operating point.

use rspice_core::Netlist;
use rspice_core::netlist::{AnalysisCommand, SmallSignalStateSource};

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn parse_error(deck: &str) -> String {
    match Netlist::parse(deck) {
        Ok(_) => panic!("deck must be refused:\n{deck}"),
        Err(error) => error.to_string(),
    }
}

/// An RC deck with one `.TRAN` card and the card under test.
fn rc_deck(card: &str) -> String {
    format!(
        "* state keywords\n\
         .param tcap=3u\n\
         v1 in 0 pulse(0 1 0 1n 1n 10u 20u) ac 1\n\
         r1 in out 1k\n\
         c1 out 0 1n\n\
         vp out x 0\n\
         rx x 0 10k\n\
         .tran 10n 5u\n\
         {card}\n\
         .end\n"
    )
}

/// The card's analysis command and the state it names.
fn card_and_state(card: &str) -> (AnalysisCommand, Option<SmallSignalStateSource>) {
    let netlist = parse(&rc_deck(card));
    let index = netlist
        .analyses
        .iter()
        .position(|analysis| !matches!(analysis, AnalysisCommand::Tran { .. }))
        .expect("the card under test is an analysis");
    (
        netlist.analyses[index].clone(),
        netlist.small_signal_state(index).cloned(),
    )
}

/// Every small-signal card reads the same card with and without its state
/// keyword, and the keyword lands in the sidecar under the card's index.
#[test]
fn every_small_signal_card_round_trips_its_state_keyword() {
    for card in [
        ".ac dec 10 1 1meg",
        ".noise v(out) v1 dec 10 1 1meg",
        ".noise v(out) v1 dec 10 1 1meg 5",
        ".tf v(out) v1",
        ".sp dec 10 1 1meg",
        ".sp dec 10 1 1meg 1 port1=(out)",
        ".stb dec 10 1 1meg probe=vp",
        ".stb dec 10 1 1meg vp",
        ".pz in 0 out 0 vol pz",
        ".sens v(out)",
        ".sens v(out) r1 c1",
        ".sens v(out) ac dec 10 1 1meg",
        ".disto dec 10 1 1meg",
        ".disto dec 10 1 1meg 0.9",
    ] {
        let (plain, none) = card_and_state(card);
        assert!(none.is_none(), "{card}: no keyword, no state");
        for (suffix, expected) in [
            (
                " ACTIMES=1u,{tcap}",
                SmallSignalStateSource::TransientTimes(vec![1.0e-6, 3.0e-6]),
            ),
            (
                " actimes=[1u 2u, 4u]",
                SmallSignalStateSource::TransientTimes(vec![1.0e-6, 2.0e-6, 4.0e-6]),
            ),
            (
                // A scaled literal is its mantissa times the suffix's scale.
                " ACTIMES=2.5u",
                SmallSignalStateSource::TransientTimes(vec![2.5 * 1.0e-6]),
            ),
            (
                " CHECKPOINT=\"state at 3u.chk\"",
                SmallSignalStateSource::CheckpointFile("state at 3u.chk".to_owned()),
            ),
            (
                " checkpoint=trim.chk",
                SmallSignalStateSource::CheckpointFile("trim.chk".to_owned()),
            ),
        ] {
            let stated = format!("{card}{suffix}");
            let (analysis, state) = card_and_state(&stated);
            assert_eq!(
                format!("{analysis:?}"),
                format!("{plain:?}"),
                "{stated}: the keyword leaves the card itself unchanged"
            );
            assert_eq!(state.as_ref(), Some(&expected), "{stated}");
        }
    }
}

/// The sidecar names its card by index, so a card after a `.STEP` axis keeps
/// its state when a run axis is stripped from a materialized coordinate.
#[test]
fn a_state_request_follows_its_card_when_cards_are_dropped() {
    let mut netlist = parse(&rc_deck(
        ".step param tcap list 3u 4u\n.op\n.ac dec 10 1 1meg actimes=2u\n.tf v(out) v1",
    ));
    let index = |netlist: &Netlist| {
        netlist
            .analyses
            .iter()
            .position(|analysis| matches!(analysis, AnalysisCommand::Ac { .. }))
            .expect("the .ac card")
    };
    let before = index(&netlist);
    assert_eq!(
        netlist.small_signal_state(before),
        Some(&SmallSignalStateSource::TransientTimes(vec![2.0e-6]))
    );
    netlist.retain_analyses(|_, analysis| !matches!(analysis, AnalysisCommand::Step(_)));
    let after = index(&netlist);
    assert_eq!(after, before - 1);
    assert_eq!(netlist.small_signal_states.len(), 1);
    assert_eq!(
        netlist.small_signal_state(after),
        Some(&SmallSignalStateSource::TransientTimes(vec![2.0e-6]))
    );
    netlist.retain_analyses(|_, analysis| !matches!(analysis, AnalysisCommand::Ac { .. }));
    assert!(
        netlist.small_signal_states.is_empty(),
        "a dropped card takes its state with it"
    );
}

#[test]
fn actimes_and_checkpoint_are_mutually_exclusive_and_said_once() {
    for (card, named) in [
        (
            ".ac dec 10 1 1meg actimes=1u checkpoint=\"s.chk\"",
            "both ACTIMES= and CHECKPOINT=",
        ),
        (
            ".noise v(out) v1 dec 10 1 1meg checkpoint=s.chk actimes=1u",
            "both ACTIMES= and CHECKPOINT=",
        ),
        (
            ".tf v(out) v1 actimes=1u actimes=2u",
            "ACTIMES= at most once",
        ),
        (
            ".pz in 0 out 0 vol pz checkpoint=a.chk checkpoint=b.chk",
            "CHECKPOINT= at most once",
        ),
    ] {
        let message = parse_error(&rc_deck(card));
        assert!(message.contains(named), "{card}: {message}");
    }
}

#[test]
fn actimes_needs_a_transient_card_that_reaches_every_time() {
    let without_tran = "* no transient\n\
                        v1 in 0 dc 1 ac 1\n\
                        r1 in out 1k\n\
                        c1 out 0 1n\n\
                        .ac dec 10 1 1meg actimes=1u\n\
                        .end\n";
    let message = parse_error(without_tran);
    assert!(message.contains("no .TRAN card"), "{message}");
    // A checkpoint names its own state and needs no transient card.
    let checkpoint = parse(&without_tran.replace("actimes=1u", "checkpoint=s.chk"));
    assert_eq!(checkpoint.small_signal_states.len(), 1);

    let message = parse_error(&rc_deck(".ac dec 10 1 1meg actimes=1u,6u"));
    assert!(message.contains("past the .TRAN stop time"), "{message}");
    // The stop time itself is reachable, and so is t = 0.
    parse(&rc_deck(".ac dec 10 1 1meg actimes=0,5u"));
    // A .TRAN card after the small-signal card still counts.
    let late = "* late transient\n\
                v1 in 0 dc 1 ac 1\n\
                r1 in out 1k\n\
                c1 out 0 1n\n\
                .ac dec 10 1 1meg actimes=1u\n\
                .tran 10n 2u\n\
                .end\n";
    assert_eq!(parse(late).small_signal_states.len(), 1);
}

#[test]
fn actimes_must_be_strictly_increasing_and_non_negative() {
    for card in [
        ".ac dec 10 1 1meg actimes=2u,1u",
        ".ac dec 10 1 1meg actimes=1u,1u",
        ".ac dec 10 1 1meg actimes=[1u 3u 2u]",
    ] {
        let message = parse_error(&rc_deck(card));
        assert!(message.contains("strictly increasing"), "{card}: {message}");
    }
    let message = parse_error(&rc_deck(".ac dec 10 1 1meg actimes={-1u}"));
    assert!(message.contains("non-negative"), "{message}");
    let message = parse_error(&rc_deck(".ac dec 10 1 1meg actimes=[1u 2u"));
    assert!(message.contains("closing ']'"), "{message}");
}

/// The keyword closes the card: only the eight small-signal cards take it,
/// a table-driven sweep does not, and nothing may follow it.
#[test]
fn only_small_signal_cards_take_a_state_and_it_closes_the_card() {
    for card in [
        ".op actimes=1u",
        ".dc v1 0 1 0.1 actimes=1u",
        ".tran 10n 5u actimes=1u",
    ] {
        parse_error(&rc_deck(card));
    }
    let message = parse_error(&rc_deck(".ac dec 10 1 1meg actimes=1u extra"));
    assert!(!message.is_empty());
    let table = "* table\n\
                 v1 in 0 dc 1 ac 1\n\
                 r1 in out 1k\n\
                 .data grid freq\n\
                 1k\n\
                 2k\n\
                 .enddata\n\
                 .tran 10n 5u\n\
                 .ac data=grid actimes=1u\n\
                 .end\n";
    let message = parse_error(table);
    assert!(message.contains("DATA="), "{message}");
}
