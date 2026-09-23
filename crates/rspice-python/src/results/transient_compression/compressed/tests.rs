use super::*;

const EVENT_DECK: &str = "\
* an analog clock crossing into the digital world
vclk clk 0 pulse(0 3.3 1n 0.2n 0.2n 4n 8n)
rclk clk 0 1k
aadc [clk] [d] adc
.model adc adc_bridge (in_low=1.0 in_high=2.0)
ainv [d] [q] inv
.model inv d_inverter (rise_delay=0.3n fall_delay=0.3n)
.end
";

fn compressed_event_run() -> PyCompressedTransientResult {
    let netlist = rspice_core::Netlist::parse(EVENT_DECK).expect("the deck parses");
    let engine = rspice_core::engine::Engine::default();
    let inner = engine
        .run_tran_compressed_with_abort(
            &netlist,
            2.0e-8,
            2.0e-10,
            rspice_core::engine::CompressionConfig {
                abs_tol: 1.0e-6,
                rel_tol: 1.0e-4,
                enabled: true,
                maximum_retained_interval: 0.0,
            },
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("the compressed transient solves");
    PyCompressedTransientResult::new(inner)
}

/// The compressed listing agrees with the document it is a listing of.
///
/// The container keeps an event-only net's channel because that inventory
/// *is* the node namespace and dropping one would renumber it — but the
/// document publishes no descriptor for it, so a listing that still names
/// it advertises a channel nothing can describe, and `channel_availability`
/// answered "not-projected", which claims the run chose not to keep a
/// voltage this net never had.
#[test]
fn the_channel_listing_omits_an_event_only_net_the_document_drops() {
    let result = compressed_event_run();

    let nodes = result.node_names();
    for event_only in ["d", "q"] {
        assert!(
            nodes
                .iter()
                .any(|name| name.eq_ignore_ascii_case(event_only)),
            "{event_only} must keep its place in the node namespace, got {nodes:?}"
        );
    }

    let channels = result.channel_names();
    for event_only in ["v(d)", "v(q)"] {
        assert!(
            !channels
                .iter()
                .any(|name| name.eq_ignore_ascii_case(event_only)),
            "{event_only} must not be listed as a channel, got {channels:?}"
        );
    }
    assert!(
        channels
            .iter()
            .any(|name| name.eq_ignore_ascii_case("v(clk)")),
        "the loaded analog node is still listed, got {channels:?}"
    );

    // And the predicate the listing filters on is the one the accessors
    // refuse through, in the domain the net actually carries.
    assert_eq!(
        result
            .event_only_node(
                nodes
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case("d"))
                    .expect("d is a node")
            )
            .map(|(name, kind)| (name.to_ascii_lowercase(), kind)),
        Some((
            "d".to_string(),
            rspice_core::analysis::transient::EventOnlyNetKind::Digital
        ))
    );
}
/// The accessor this class's refusal recommends is one THIS class has.
///
/// `TransientResult` spells the digital history `digital_events`; this
/// class spells it `digital_trace`. A refusal rendered for a compressed
/// container that named `digital_events` would send the caller to a method
/// the object does not have — the same defect as the `D(clk)` hint the
/// sentence replaced, one class over. Both halves are checked against the
/// published stub, scoped to this class's own block so a method that only
/// exists on the sibling cannot satisfy it.
#[test]
fn the_compressed_refusals_python_accessor_is_one_this_class_publishes() {
    use rspice_core::analysis::transient::{
        EventOnlyNetKind, EventTraceSurface, event_only_voltage_refusal,
    };

    let stub = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/rspice.pyi"))
        .expect("the published type stub is beside the crate manifest");
    let block = stub
        .split_once("class CompressedTransientResult:")
        .expect("the stub declares this class")
        .1;
    let block = block.split_once("\nclass ").map_or(block, |(head, _)| head);

    let digital = event_only_voltage_refusal(
        "q",
        EventOnlyNetKind::Digital,
        EventTraceSurface::Compressed,
    );
    assert!(digital.contains("digital_trace('q')"), "{digital}");
    assert!(
        !digital.contains("digital_events("),
        "the sibling class's spelling must not appear here: {digital}"
    );
    assert!(block.contains("def digital_trace("), "{block}");

    let real = event_only_voltage_refusal(
        "watched",
        EventOnlyNetKind::Real,
        EventTraceSurface::Compressed,
    );
    assert!(real.contains("real_trace('watched')"), "{real}");
    assert!(block.contains("def real_trace("), "{block}");

    // And the run's own container really answers to the digital spelling
    // for the net whose voltage it refuses, so the sentence is a hint a
    // caller can act on rather than a claim about the stub alone.
    let result = compressed_event_run();
    assert!(
        result.digital_trace("d").is_ok(),
        "the digital accessor the refusal names answers for that net"
    );
}
