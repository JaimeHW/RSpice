//! Bundle-level conformance for the publication renderer, driven by the
//! contract crate's golden fixtures so renderer and schema can never drift
//! against different corpora.

use rspice_publication_contract::{
    ComponentPin, ComponentRecord, Disclosure, EngineeringPublication, FigureManifest,
    FigurePayload, FigurePresentation, PublicationMetadata, PublicationOverview,
    PublicationPresentation, PublicationSection, PublicationSnapshot, SignalIdentity, SignalTarget,
    SimulationProvenance, SimulationSetting, SimulationWarning, Specification, Validate as _,
    WarningSeverity,
};
use rspice_publish::{ViewerRuntime, render_bundle};
use sha2::{Digest as _, Sha256};

const RC_LOWPASS: &str =
    include_str!("../../rspice-publication-contract/tests/fixtures/rc-lowpass.json");
const MULTI_ANALYSIS: &str =
    include_str!("../../rspice-publication-contract/tests/fixtures/multi-analysis.json");

/// Deterministic stand-in for the wasm-bindgen output the component build
/// stages. The renderer treats the runtime as sealed bytes, so fakes keep
/// every bundle assertion exact without a wasm toolchain.
const FAKE_WASM: &[u8] = b"\0asm test viewer runtime";
const FAKE_GLUE: &[u8] = b"// test viewer glue\n";

fn viewer() -> ViewerRuntime {
    ViewerRuntime::new(FAKE_WASM.to_vec(), FAKE_GLUE.to_vec()).expect("test runtime")
}

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
        let first = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");
        let second =
            render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render again");
        assert_eq!(first, second, "identical input must yield identical bytes");
    }
}

#[test]
fn rc_bundle_carries_every_disclosed_surface() {
    let value = snapshot(RC_LOWPASS);
    let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");

    let page = utf8(&bundle["index.html"]);
    assert!(page.contains("<!--rspice-cloud-head-->"));
    assert!(page.contains("rel=\"rspice-publication-context\" href=\"context.json\""));
    assert!(page.contains("data-embed-copy"));
    assert!(page.contains("data-cloud-context"));
    assert!(page.contains("RC low-pass step response"));
    assert!(page.contains("id=\"figure-1\""), "schematic figure mounts");
    assert!(page.contains("id=\"figure-2\""), "plot figure mounts");
    assert!(page.contains("<circle"), "V1 source renders as a circle");
    assert!(
        page.contains("data-instance=\"R1\""),
        "instance tags survive"
    );
    assert!(
        page.contains(
            "data-instance=\"R1\" tabindex=\"0\" role=\"button\" aria-label=\"Component R1\""
        ),
        "tagged schematic objects are keyboard selectable"
    );
    assert!(!page.contains("data-schematic-search"));
    assert!(page.contains("2.20 ms"), "measurement value renders");
    assert!(page.contains("PASS"), "measurement status renders");
    assert!(
        !page.contains("archive.rspice"),
        "the page must not advertise an archive the bundle does not contain"
    );
    assert!(
        page.contains("class=\"tabbar\"")
            && !page.contains("id=\"overview\"")
            && page.contains("id=\"schematic\"")
            && page.contains("id=\"results\"")
            && page.contains("id=\"files\"")
            && page.contains("id=\"details\""),
        "the production shell opens on the circuit and exposes its supporting sections"
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
fn result_tab_count_includes_measurements_without_claiming_zero() {
    let mut value = snapshot(RC_LOWPASS);
    value.figures.retain(|figure| {
        !matches!(
            figure.content,
            rspice_publication_contract::FigureContent::Plot(_)
        )
    });

    let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");
    let page = utf8(&bundle["index.html"]);
    assert!(page.contains(
        "href=\"#results\" data-tab=\"results\">Results <span class=\"tab-count\">1</span>"
    ));
    assert!(!page.contains(">Results <span class=\"tab-count\">0</span>"));
}

#[test]
fn figure_bundles_seal_the_viewer_runtime_and_its_handshake() {
    let value = snapshot(RC_LOWPASS);
    let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");

    assert_eq!(
        bundle["assets/viewer.wasm"], FAKE_WASM,
        "the runtime is sealed byte-for-byte"
    );
    assert_eq!(bundle["assets/viewer.js"], FAKE_GLUE);
    assert!(
        utf8(&bundle["assets/loader.js"]).contains("fetchVerified"),
        "the authored loader ships verbatim"
    );

    let page = utf8(&bundle["index.html"]);
    let island_start = page
        .find("<script type=\"application/json\" id=\"rspice-hydration\">")
        .expect("hydration island present");
    let island = &page[island_start..];
    let island_json = &island
        [island.find('>').expect("open") + 1..island.find("</script>").expect("island closes")];
    assert!(
        !island_json.contains('<'),
        "the island can never terminate its own script element"
    );
    let config: serde_json::Value =
        serde_json::from_str(island_json).expect("island is valid JSON");

    let mut wasm_hex = String::new();
    for byte in Sha256::digest(FAKE_WASM) {
        use std::fmt::Write as _;
        let _ = write!(wasm_hex, "{byte:02x}");
    }
    assert_eq!(config["runtime"]["wasm"], "assets/viewer.wasm");
    assert_eq!(config["runtime"]["js"], "assets/viewer.js");
    assert_eq!(config["runtime"]["wasm_sha256"], wasm_hex.as_str());
    assert_eq!(
        config["runtime"]["wasm_byte_len"].as_u64(),
        Some(FAKE_WASM.len() as u64)
    );

    // The island's figure entries are the manifest entries, verbatim, so the
    // runtime parses exactly what the manifest sealed.
    let manifest = FigureManifest::from_canonical_bytes(&bundle["figure-manifest.json"])
        .expect("manifest parses");
    let island_figures = config["figures"].as_array().expect("figure entries");
    assert_eq!(island_figures.len(), manifest.figures.len());
    for (entry, island_entry) in manifest.figures.iter().zip(island_figures) {
        assert_eq!(island_entry["figure_id"].as_u64(), Some(entry.figure_id));
        assert_eq!(island_entry["dom_id"].as_str(), Some(entry.dom_id.as_str()));
        assert_eq!(
            island_entry["payload"]["sha256_hex"].as_str(),
            Some(entry.payload.sha256_hex.as_str())
        );
    }

    assert!(
        page.contains("<script type=\"module\" src=\"assets/loader.js\" integrity=\"sha384-"),
        "the loader tag is integrity-pinned"
    );
    assert!(
        page.contains("<canvas id=\"figure-1-canvas\" class=\"viewer\""),
        "each figure carries an inert canvas"
    );
    assert!(
        page.contains(">Open interactive schematic</button>"),
        "schematic figures get their activation control"
    );
    assert!(
        page.contains(">Open interactive plot</button>"),
        "plot figures get their activation control"
    );
    assert!(
        page.contains("<button class=\"button primary hydrate\" type=\"button\" hidden>"),
        "controls stay hidden until the loader proves it can run"
    );
    assert!(
        utf8(&bundle["assets/page.css"]).contains(
            ".figure-stage canvas.viewer[hidden], .figure-actions button.hydrate[hidden]"
        ),
        "author styles must not override the hidden state and reserve a second figure height"
    );
    assert!(
        page.contains("href=\"assets/page.css\" integrity=\"sha384-")
            && page.contains("src=\"assets/page.js\" integrity=\"sha384-"),
        "page chrome is external, sealed, and integrity-pinned"
    );
    assert!(
        utf8(&bundle["assets/page.js"]).contains("root.classList.add(\"js-ready\")"),
        "the progressive shell behavior ships in the immutable bundle"
    );
    assert!(
        page.contains("data-figure-fullscreen") && page.contains("data-figure-svg"),
        "every figure exposes progressive fullscreen and sealed SVG export actions"
    );
    assert!(
        utf8(&bundle["assets/page.js"]).contains("selectSchematicTag")
            && utf8(&bundle["assets/page.js"]).contains("exportSvg"),
        "selection and export behavior are sealed with the page"
    );
}

#[test]
fn manifest_digests_match_emitted_payload_bytes() {
    let value = snapshot(RC_LOWPASS);
    let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");

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
    let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");

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
    let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");
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
        license: value.metadata.license,
    };
    let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");
    let page = utf8(&bundle["index.html"]);
    assert!(!page.contains("<script>alert"), "title is escaped");
    assert!(!page.contains("<img"), "description is escaped");
    assert!(page.contains("&lt;script&gt;"), "escaped form is present");
}

#[test]
fn v3_engineering_metadata_reaches_the_production_page() {
    let mut value = snapshot(RC_LOWPASS);
    value.schema_version = rspice_publication_contract::PUBLICATION_SNAPSHOT_SCHEMA_VERSION;
    value.presentation = Some(PublicationPresentation {
        overview: Some(PublicationOverview {
            narrative: "A deliberately documented RC low-pass.".to_string(),
            specifications: vec![Specification {
                label: "Time constant".to_string(),
                value: "1.00".to_string(),
                unit: Some("ms".to_string()),
            }],
        }),
        section_order: vec![
            PublicationSection::Overview,
            PublicationSection::Schematic,
            PublicationSection::Results,
            PublicationSection::Components,
            PublicationSection::Files,
            PublicationSection::Details,
        ],
        default_section: PublicationSection::Results,
        featured_figure_id: Some(2),
        figure_details: vec![FigurePresentation {
            figure_id: 2,
            caption: Some("Output settles monotonically.".to_string()),
            accessible_summary: "Output voltage rises toward its final value.".to_string(),
            default_interactive: true,
        }],
    });
    value.engineering = Some(EngineeringPublication {
        components: vec![ComponentRecord {
            reference: "R1".to_string(),
            value: "1k".to_string(),
            device: "Resistor".to_string(),
            model: None,
            pins: vec![ComponentPin {
                name: "2".to_string(),
                number: None,
                net: None,
            }],
        }],
        nets: vec![],
        signals: vec![SignalIdentity {
            dataset_id: 1,
            trace_index: 0,
            target: SignalTarget::Expression {
                label: "input voltage".to_string(),
            },
        }],
        simulation: Some(SimulationProvenance {
            engine: "RSpice".to_string(),
            engine_version: "0.1.0".to_string(),
            temperature_c_bits: Some(27.0f64.to_bits()),
            corner: Some("typical".to_string()),
            settings: vec![SimulationSetting {
                name: "Relative tolerance".to_string(),
                value: "1e-3".to_string(),
            }],
            warnings: vec![SimulationWarning {
                severity: WarningSeverity::Information,
                message: "Operating point converged.".to_string(),
                analysis_id: Some(1),
            }],
        }),
    });

    let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render v3");
    let page = utf8(&bundle["index.html"]);
    assert!(page.contains("data-default-panel=\"schematic\""));
    assert!(!page.contains("href=\"#overview\""));
    assert!(page.contains("id=\"components\"") && page.contains("Resistor"));
    assert!(page.contains("A deliberately documented RC low-pass."));
    assert!(page.contains("Time constant") && page.contains("1.00"));
    assert!(page.contains("data-default-interactive"));
    assert!(page.contains("Output voltage rises toward its final value."));
    assert!(page.contains("Simulation provenance") && page.contains("Relative tolerance"));
    assert!(page.contains("Operating point converged."));
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
    let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");
    assert_eq!(
        bundle.keys().collect::<Vec<_>>(),
        vec![
            "assets/page.css",
            "assets/page.js",
            "figure-manifest.json",
            "index.html"
        ],
        "a fully withheld publication carries only the semantic shell, with no viewer runtime"
    );
    assert!(
        !utf8(&bundle["index.html"]).contains("rspice-hydration")
            && !bundle.contains_key("assets/loader.js")
            && !bundle.contains_key("assets/viewer.js")
            && !bundle.contains_key("assets/viewer.wasm"),
        "a figureless page carries no hydration handshake or viewer runtime"
    );
}
