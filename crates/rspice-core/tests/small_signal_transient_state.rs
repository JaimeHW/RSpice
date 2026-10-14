//! Small-signal analyses around an accepted transient state (design S10).
//!
//! A card that carries `ACTIMES=` linearizes at the accepted state of the
//! deck's own `.TRAN` run at each requested time; one that carries
//! `CHECKPOINT=` linearizes at the state a checkpoint file holds. The oracles
//! are closed forms and held-state twins: a deck whose operating point *is*
//! the transient state, solved as an ordinary operating point.

use rspice_core::engine::{SmallSignalCardResult, StateLinearizedResult};
use rspice_core::netlist::{AnalysisCommand, SmallSignalStateSource};
use rspice_core::{Complex64, Engine, Netlist, NoAbort};

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

/// The analysis card's index in its deck.
fn card_index(netlist: &Netlist, matches: impl Fn(&AnalysisCommand) -> bool) -> usize {
    netlist
        .analyses
        .iter()
        .position(matches)
        .expect("the deck carries the card")
}

/// The card's `ACTIMES=` times, as the deck spelled them.
fn state_times(netlist: &Netlist, index: usize) -> Vec<f64> {
    match netlist.small_signal_state(index) {
        Some(SmallSignalStateSource::TransientTimes(times)) => times.clone(),
        other => panic!("the card carries ACTIMES=, not {other:?}"),
    }
}

/// Run the card at `index` at its own `ACTIMES=` times, checking that every
/// result names the accepted time it linearized at.
fn run_at_state_times(
    engine: &Engine,
    netlist: &Netlist,
    index: usize,
) -> Vec<StateLinearizedResult> {
    let times = state_times(netlist, index);
    let results = engine
        .run_small_signal_at_transient_times(netlist, index, &times, &NoAbort)
        .unwrap_or_else(|error| panic!("the card runs at its transient states: {error}"));
    assert_eq!(
        results
            .iter()
            .map(|result| result.state_time)
            .collect::<Vec<_>>(),
        times,
        "one result per requested time, at that time"
    );
    results
}

fn ac_points(result: &StateLinearizedResult) -> &[rspice_core::analysis::AcResult] {
    match &result.result {
        SmallSignalCardResult::Ac(points) => points,
        other => panic!("an .ac card gives AC points, not {other:?}"),
    }
}

fn ac_voltage(point: &rspice_core::analysis::AcResult, node: &str) -> Complex64 {
    let index = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("the AC result names {node}: {:?}", point.node_names));
    point.voltages[index]
}

fn assert_phasor_close(what: &str, actual: Complex64, expected: Complex64, rel: f64) {
    let bound = rel * expected.norm() + 1.0e-12;
    assert!(
        (actual - expected).norm() <= bound,
        "{what}: {actual} differs from the held twin's {expected} by {:e} (allowed {bound:e})",
        (actual - expected).norm()
    );
}

/// A diode charging a capacitor: the state at `T` is mid-charge, so the
/// diode's conductance and junction capacitance at `T` are not the operating
/// point's. Linearized around that state, the deck is the same small-signal
/// system as its frozen-time twin — the same circuit at DC, with a DC source
/// holding `v(out)` at its value at `T` through an inductor (a short at DC,
/// open at every analysis frequency).
const DIODE_RC: &str = "* diode-RC charging\n\
                        v1 in 0 pulse(0 1 0 100n 100n 1 2) ac 1\n\
                        d1 in out dmod\n\
                        r1 out 0 10k\n\
                        c1 out 0 100p\n\
                        .model dmod d is=1e-14 n=1.2 cjo=5p\n\
                        .tran 0.1u 20u 0 0.1u\n\
                        .ac dec 4 1k 100meg actimes=20u\n\
                        .end\n";

#[test]
fn a_nonlinear_state_linearizes_like_its_frozen_time_twin() {
    let netlist = parse(DIODE_RC);
    let index = card_index(&netlist, |card| matches!(card, AnalysisCommand::Ac { .. }));
    let engine = Engine::default();
    let results = run_at_state_times(&engine, &netlist, index);
    assert_eq!(results.len(), 1);

    // The state the card linearized at is the .TRAN card's state at T: the
    // same window, run to T, ends on it.
    let Some(AnalysisCommand::Tran {
        step,
        stop,
        start,
        max_step,
        ..
    }) = netlist.analyses.first().cloned()
    else {
        panic!("the deck's first card is its .TRAN card")
    };
    let max_step =
        rspice_core::execution::resolve_transient_maximum_step(step, stop, start, max_step)
            .expect("the window resolves");
    let transient = engine
        .run_tran(&netlist, stop, max_step)
        .expect("the transient runs");
    assert_eq!(
        *transient.time.last().expect("accepted points"),
        results[0].state_time
    );
    let held = *transient
        .try_voltage_waveform_named("out")
        .expect("out is retained")
        .last()
        .expect("a final point");
    assert!(
        held > 0.05 && held < 0.95,
        "the capacitor is mid-charge at T: v(out) = {held}"
    );
    let twin = parse(&format!(
        "* frozen-time twin\n\
         v1 in 0 dc 1 ac 1\n\
         d1 in out dmod\n\
         r1 out 0 10k\n\
         c1 out 0 100p\n\
         vh h 0 dc {held:.17e}\n\
         lh h out 1e15\n\
         .model dmod d is=1e-14 n=1.2 cjo=5p\n\
         .end\n"
    ));
    let frequencies = ac_points(&results[0])
        .iter()
        .map(|point| point.frequency)
        .collect::<Vec<_>>();
    let expected = engine.run_ac(&twin, &frequencies).expect("the twin's .ac");
    // The same engine, after the state run, linearizes at the operating point
    // again: the state belonged to that one run, not to the engine.
    let op = engine
        .run_ac(&netlist, &frequencies)
        .expect("the operating-point .ac");
    let mut moved = false;
    for ((point, reference), at_op) in ac_points(&results[0]).iter().zip(&expected).zip(&op) {
        let actual = ac_voltage(point, "out");
        assert_phasor_close(
            &format!("v(out) at {} Hz", point.frequency),
            actual,
            ac_voltage(reference, "out"),
            1.0e-6,
        );
        moved |= (actual - ac_voltage(at_op, "out")).norm() > 1.0e-3 * actual.norm();
    }
    assert!(
        moved,
        "the state at T linearizes differently from the operating point"
    );
}

/// B3SOI keeps accepted integration history no checkpoint image carries.
/// A live capture is a clone of the circuit, so the card still runs around
/// the state.
const B3SOI_GAIN_STAGE: &str = "* B3SOI gain stage after an input step\n\
                                m1 3 2 0 0 nmos w=4u l=1u\n\
                                rsource 1 2 100k\n\
                                rload 3 vdd 25k\n\
                                vdd1 vdd 0 5\n\
                                vin 1 0 pulse(1.44 1.5 10n 1n 1n 1 2) ac .1\n\
                                .model nmos nmos level=10\n\
                                .tran 1n 100n\n\
                                .ac dec 5 1k 1g actimes=50n,100n\n\
                                .end\n";

#[test]
fn a_device_no_checkpoint_carries_still_linearizes_around_a_live_state() {
    let netlist = parse(B3SOI_GAIN_STAGE);
    let index = card_index(&netlist, |card| matches!(card, AnalysisCommand::Ac { .. }));
    let results = run_at_state_times(&Engine::default(), &netlist, index);
    assert_eq!(results.len(), 2);
    for result in &results {
        for point in ac_points(result) {
            let gain = ac_voltage(point, "3");
            assert!(
                gain.re.is_finite() && gain.im.is_finite() && gain.norm() > 1.0e-6,
                "a finite, nonzero response at {} Hz: {gain}",
                point.frequency
            );
        }
    }
}

/// A trim code that only the transient reaches: the corpus trim register
/// starts at code 1010 and an SPI frame shifts in 1111, latched when chip
/// select rises at 400 µs. Around the state at 550 µs the amplifier is the
/// code-15 amplifier, which is its O5 twin: every converter replaced by its
/// held level behind 20 ohm.
#[cfg(feature = "veriloga")]
mod trim {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQUENCE: AtomicU64 = AtomicU64::new(0);

    /// A source file the deck includes, removed when the test ends.
    pub(super) struct SourceFile(pub(super) std::path::PathBuf);

    impl SourceFile {
        pub(super) fn new(extension: &str, source: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "rspice_state_trim_{}_{}.{extension}",
                std::process::id(),
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::write(&path, source).expect("write source file");
            Self(path)
        }

        pub(super) fn path(&self) -> String {
            self.0.display().to_string().replace('\\', "/")
        }
    }

    impl Drop for SourceFile {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    /// The corpus C5 trim register: a four-bit shift register clocked while
    /// chip select is low, latched into the outputs when it rises. Its
    /// initial block loads 1010.
    pub(super) const TRIM_REGISTER: &str = "`timescale 1ns/1ps\n\
        module trim_reg(sclk, sdi, csn, tr3, tr2, tr1, tr0);\n\
          input sclk, sdi, csn;\n\
          output tr3, tr2, tr1, tr0;\n\
          reg tr3, tr2, tr1, tr0;\n\
          reg [3:0] sh;\n\
          initial begin sh = 4'b1010; tr3 = 1'b1; tr2 = 1'b0; tr1 = 1'b1; tr0 = 1'b0; end\n\
          always @(posedge sclk) if (!csn) sh <= {sh[2:0], sdi};\n\
          always @(posedge csn) {tr3, tr2, tr1, tr0} <= sh;\n\
        endmodule\n";

    /// A runtime Verilog-A load with a charge and a noise source.
    pub(super) const NOISY_LOAD: &str = "`include \"disciplines.vams\"\n\
        module noisy_load(p, n);\n\
          inout p, n; electrical p, n;\n\
          parameter real r = 5k;\n\
          parameter real c = 10p;\n\
          analog I(p, n) <+ V(p, n) / r + ddt(c * V(p, n)) + white_noise(1.0e-22, \"load\");\n\
        endmodule\n";

    /// The amplifier the trim drives: the held-state suite's fixture with its
    /// load diode turned to face the inverting stage's negative output. There
    /// it conducts the current the trim code sets, so the small-signal
    /// divider it forms with `rd` depends on the code; the other way round it
    /// is reverse biased at every code and nothing in the response tells one
    /// code from another.
    const AMPLIFIER: &str = "vin in 0 dc 0.2 ac 1\n\
        r1 in sum 10k\n\
        rt3 tr3 sum 10k\n\
        rt2 tr2 sum 20k\n\
        rt1 tr1 sum 40k\n\
        rt0 tr0 sum 80k\n\
        rf sum fb 20k\n\
        vprobe out fb 0\n\
        eamp out 0 0 sum 1e3\n\
        d1 dn out dmod\n\
        rd dn 0 5k\n\
        .model dmod d is=1e-14 n=1.5\n\
        rl out 0 2k\n";

    /// The SPI frame: chip select falls at 50 µs, four clock edges shift in
    /// four ones, and chip select rises at 400 µs.
    const SPI_FRAME: &str = "vcsn csn 0 pwl(0 1.8 50u 1.8 51u 0 399u 0 400u 1.8)\n\
        vsdi sdi 0 1.8\n\
        vsclk sclk 0 pwl(0 0 100u 0 101u 1.8 120u 1.8 121u 0 150u 0 151u 1.8 170u 1.8 \
        171u 0 200u 0 201u 1.8 220u 1.8 221u 0 250u 0 251u 1.8 270u 1.8 271u 0)\n";

    /// The SPI-loaded deck, with `cards` after its `.TRAN` card.
    pub(super) fn trim_deck(register: &SourceFile, load: &SourceFile, cards: &str) -> String {
        format!(
            "* SPI-loaded trim register\n\
             .param vcc=1.8\n\
             {SPI_FRAME}\
             xtrim sclk sdi csn tr3 tr2 tr1 tr0 trim_reg\n\
             {AMPLIFIER}\
             xload out 0 noisy_load\n\
             .hdl \"{}\"\n\
             .va \"{}\" noisy_load\n\
             .tran 1u 550u\n\
             {cards}\
             .end\n",
            register.path(),
            load.path()
        )
    }

    /// The O5 twin of a held code: every converter its held level behind
    /// 20 ohm, solved analog-only.
    pub(super) fn held_code_twin(load: &SourceFile, code: [bool; 4]) -> Netlist {
        let held = ["tr3", "tr2", "tr1", "tr0"]
            .iter()
            .zip(code)
            .map(|(net, bit)| {
                let level = if bit { 1.8 } else { 0.0 };
                format!("v{net} {net}_h 0 {level}\nr{net}_h {net}_h {net} 20\n")
            })
            .collect::<String>();
        parse(&format!(
            "* held trim code, O5 twin\n\
             {held}\
             {AMPLIFIER}\
             xload out 0 noisy_load\n\
             .va \"{}\" noisy_load\n\
             .end\n",
            load.path()
        ))
    }

    pub(super) const NODES: [&str; 7] = ["out", "dn", "sum", "tr3", "tr2", "tr1", "tr0"];

    #[test]
    fn the_code_loaded_over_spi_linearizes_like_its_held_code_twin() {
        let register = SourceFile::new("v", TRIM_REGISTER);
        let load = SourceFile::new("va", NOISY_LOAD);
        let netlist = parse(&trim_deck(
            &register,
            &load,
            ".ac dec 2 1k 10meg actimes=550u\n",
        ));
        let index = card_index(&netlist, |card| matches!(card, AnalysisCommand::Ac { .. }));
        let engine = Engine::default();
        let results = run_at_state_times(&engine, &netlist, index);
        let points = ac_points(&results[0]);
        let frequencies = points
            .iter()
            .map(|point| point.frequency)
            .collect::<Vec<_>>();
        let twin = engine
            .run_ac(&held_code_twin(&load, [true; 4]), &frequencies)
            .expect("the code-15 twin's .ac");
        let initial = engine
            .run_ac(&netlist, &frequencies)
            .expect("the operating-point .ac holds the initial code");
        let mut moved = false;
        for ((point, reference), at_op) in points.iter().zip(&twin).zip(&initial) {
            for node in NODES {
                let actual = ac_voltage(point, node);
                assert_phasor_close(
                    &format!("v({node}) at {} Hz", point.frequency),
                    actual,
                    ac_voltage(reference, node),
                    1.0e-4,
                );
                moved |= (actual - ac_voltage(at_op, node)).norm() > 1.0e-3 * actual.norm();
            }
        }
        assert!(
            moved,
            "code 15 linearizes differently from the initial code 10"
        );
    }
}

/// Verilog-A's `$abstime` reads the state's time while the circuit is
/// linearized around it, as Spectre's `actimes` linearizes the circuit as it
/// is at that time, and zero at the operating point.
#[cfg(feature = "veriloga")]
mod abstime {
    use super::trim::SourceFile;
    use super::*;

    /// A conductance that grows with time, `g0 (1 + $abstime / tau)`; as a
    /// mixed module when it also holds a register.
    fn aging_conductance(mixed: bool) -> String {
        format!(
            "`include \"disciplines.vams\"\n\
             module aging(p, n);\n\
               inout p, n; electrical p, n;\n\
               parameter real g0 = 1m;\n\
               parameter real tau = 5u;\n\
               {}\
               analog I(p, n) <+ V(p, n) * g0 * (1.0 + $abstime / tau);\n\
             endmodule\n",
            if mixed { "reg q; initial q = 0;\n" } else { "" }
        )
    }

    fn linearizes_at_the_state_time(mixed: bool) {
        let module = SourceFile::new("va", &aging_conductance(mixed));
        let netlist = parse(&format!(
            "* a conductance that ages\n\
             vin in 0 dc 1 ac 1\n\
             r1 in out 1k\n\
             xg out 0 aging\n\
             .va \"{}\" aging\n\
             .tran 0.1u 10u\n\
             .ac lin 2 1k 2k actimes=5u,10u\n\
             .end\n",
            module.path()
        ));
        let index = card_index(&netlist, |card| matches!(card, AnalysisCommand::Ac { .. }));
        let engine = Engine::default();
        // v(out) = 1 / (1 + R g(T)) with R g0 = 1: 1 / (2 + T / tau).
        let divider = |time: f64| Complex64::new(1.0 / (2.0 + time / 5.0e-6), 0.0);
        for result in run_at_state_times(&engine, &netlist, index) {
            for point in ac_points(&result) {
                assert_phasor_close(
                    &format!(
                        "v(out) around t={:e} s at {} Hz",
                        result.state_time, point.frequency
                    ),
                    ac_voltage(point, "out"),
                    divider(result.state_time),
                    1.0e-9,
                );
            }
        }
        for point in engine
            .run_ac(&netlist, &[1.0e3, 2.0e3])
            .expect("the operating-point .ac")
        {
            assert_phasor_close(
                &format!("v(out) at the operating point at {} Hz", point.frequency),
                ac_voltage(&point, "out"),
                divider(0.0),
                1.0e-9,
            );
        }
    }

    #[test]
    fn abstime_reads_the_state_time_in_a_runtime_device() {
        linearizes_at_the_state_time(false);
    }

    #[test]
    fn abstime_reads_the_state_time_in_a_mixed_host() {
        linearizes_at_the_state_time(true);
    }
}
