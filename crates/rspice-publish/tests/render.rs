//! Bundle-level conformance for the publication renderer, driven by the
//! contract crate's golden fixtures so renderer and schema can never drift
//! against different corpora.

use rspice_publication_contract::{
    Disclosure, FigureManifest, FigurePayload, PublicationMetadata, PublicationSnapshot,
    Validate as _,
};
use rspice_publish::render_bundle;
use sha2::{Digest as _, Sha256};

const RC_LOWPASS: &str =
    include_str!("../../rspice-publication-contract/tests/fixtures/rc-lowpass.json");
const MULTI_ANALYSIS: &str =
    include_str!("../../rspice-publication-contract/tests/fixtures/multi-analysis.json");

fn snapshot(golden: &str) -> PublicationSnapshot {
    PublicationSnapshot::from_canonical_bytes(golden.as_bytes()).expect("fixture parses")
}

fn utf8(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).expect("bundle text assets are UTF-8")
}

#[test]
fn bundles_are_deterministic() {
    for golden in [RC_LOWPASS, MULTI_ANALYSIS] {
        let value = snapshot(golden);
        let first = render_bundle(&value, "0".repeat(64).as_str()).expect("render");
        let second = render_bundle(&value, "0".repeat(64).as_str()).expect("render again");
        assert_eq!(first, second, "identical input must yield identical bytes");
    }
}

#[test]
fn rc_bundle_carries_every_disclosed_surface() {
    let value = snapshot(RC_LOWPASS);
    let bundle = render_bundle(&value, "0".repeat(64).as_str()).expect("render");

    let page = utf8(&bundle["index.html"]);
    assert!(page.contains("RC low-pass step response"));
    assert!(page.contains("id=\"figure-1\""), "schematic figure mounts");
    assert!(page.contains("id=\"figure-2\""), "plot figure mounts");
    assert!(page.contains("<circle"), "V1 source renders as a circle");
    assert!(
        page.contains("data-instance=\"R1\""),
        "instance tags survive"
    );
    assert!(page.contains("2.20 ms"), "measurement value renders");
    assert!(page.contains("PASS"), "measurement status renders");
    assert!(
        page.contains("archive.rspice"),
        "disclosed archive is linked"
    );

    assert_eq!(
        utf8(&bundle["netlist.cir"]),
        value.netlist.as_ref().expect("netlist").deck,
        "the deck asset is the exact deck bytes"
    );

    let csv = utf8(&bundle["data/1-tran1.csv"]);
    let mut lines = csv.lines();
    assert_eq!(lines.next(), Some("time (s),V(in) (V),V(out) (V)"));
    assert_eq!(lines.next(), Some("0,0,0"));
    assert_eq!(csv.lines().count(), 12, "header plus eleven samples");
}

#[test]
fn manifest_digests_match_emitted_payload_bytes() {
    let value = snapshot(RC_LOWPASS);
    let bundle = render_bundle(&value, "0".repeat(64).as_str()).expect("render");

    let manifest = FigureManifest::from_canonical_bytes(&bundle["figure-manifest.json"])
        .expect("manifest parses");
    manifest.validate().expect("manifest validates");
    assert_eq!(manifest.figures.len(), 2);

    for entry in &manifest.figures {
        let bytes = &bundle[&entry.payload.path];
        assert_eq!(bytes.len() as u64, entry.payload.byte_len);
        let mut hex = String::new();
        for byte in Sha256::digest(bytes) {
            use std::fmt::Write as _;
            let _ = write!(hex, "{byte:02x}");
        }
        assert_eq!(hex, entry.payload.sha256_hex, "{}", entry.payload.path);

        let payload = FigurePayload::from_canonical_bytes(bytes).expect("payload parses");
        payload.validate().expect("payload validates");
        assert_eq!(payload.figure_id, entry.figure_id);
    }
}

#[test]
fn hydration_payloads_carry_exactly_the_referenced_datasets() {
    let value = snapshot(MULTI_ANALYSIS);
    let bundle = render_bundle(&value, "0".repeat(64).as_str()).expect("render");

    // Figure 3 binds datasets 2 and 3 (the two DC corners).
    let corners = FigurePayload::from_canonical_bytes(&bundle["figures/3.json"]).expect("payload");
    let mut ids: Vec<u64> = corners.datasets.iter().map(|d| d.id).collect();
    ids.sort_unstable();
    assert_eq!(ids, vec![2, 3]);

    // Figure 4 has no hydration: scene only.
    let static_only =
        FigurePayload::from_canonical_bytes(&bundle["figures/4.json"]).expect("payload");
    assert!(static_only.datasets.is_empty());
    assert!(static_only.hydration.is_none());
}

#[test]
fn disclosure_subtraction_reaches_the_rendered_page() {
    let value = snapshot(MULTI_ANALYSIS);
    let bundle = render_bundle(&value, "0".repeat(64).as_str()).expect("render");
    let page = utf8(&bundle["index.html"]);

    assert!(
        !page.contains("data-instance"),
        "no schematic content may leak when the schematic is withheld"
    );
    assert!(
        !page.contains("archive.rspice"),
        "undisclosed archive must not be linked"
    );
    assert!(page.contains("FAIL"), "failing measurement renders as FAIL");
    assert!(
        page.contains("not computed"),
        "unevaluated measurement renders its display text"
    );

    let csv = utf8(&bundle["data/1-ac1.csv"]);
    assert_eq!(
        csv.lines().next(),
        Some("frequency (Hz),V(out) re (V),V(out) im (V)"),
        "complex traces split into real and imaginary columns"
    );
}

#[test]
fn author_text_cannot_smuggle_markup() {
    let mut value = snapshot(RC_LOWPASS);
    value.metadata = PublicationMetadata {
        title: "<script>alert(1)</script>".to_string(),
        description: "a </pre><img src=x onerror=alert(2)> b".to_string(),
        author_display: "\"quoted\" & <tagged>".to_string(),
        app_version: value.metadata.app_version.clone(),
        created_utc: value.metadata.created_utc.clone(),
    };
    let bundle = render_bundle(&value, "0".repeat(64).as_str()).expect("render");
    let page = utf8(&bundle["index.html"]);
    assert!(!page.contains("<script>alert"), "title is escaped");
    assert!(!page.contains("<img"), "description is escaped");
    assert!(page.contains("&lt;script&gt;"), "escaped form is present");
}

#[test]
fn withheld_sections_yield_no_assets_at_all() {
    let mut value = snapshot(RC_LOWPASS);
    value.disclosure = Disclosure {
        schematic: false,
        netlist: false,
        results: false,
        archive: false,
    };
    value.schematic = None;
    value.netlist = None;
    value.results = None;
    value.figures.clear();
    let bundle = render_bundle(&value, "0".repeat(64).as_str()).expect("render");
    assert_eq!(
        bundle.keys().collect::<Vec<_>>(),
        vec!["figure-manifest.json", "index.html"],
        "a fully withheld publication is a bare document"
    );
}
