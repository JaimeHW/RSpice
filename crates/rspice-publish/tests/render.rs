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
use rspice_publish::{PAGE_STYLES, ViewerRuntime, render_bundle};
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

/// The `usWeightClass` an sfnt face's `OS/2` table records.
fn weight_class(face: &[u8]) -> u16 {
    let read_u16 = |at: usize| u16::from_be_bytes([face[at], face[at + 1]]);
    let read_u32 =
        |at: usize| u32::from_be_bytes([face[at], face[at + 1], face[at + 2], face[at + 3]]);
    let tables = usize::from(read_u16(4));
    let os2 = (0..tables)
        .map(|index| 12 + index * 16)
        .find(|&record| &face[record..record + 4] == b"OS/2")
        .expect("an sfnt face carries an OS/2 table");
    read_u16(read_u32(os2 + 8) as usize + 4)
}

#[test]
fn a_figure_bundle_ships_the_faces_its_stylesheet_declares() {
    let bundle =
        render_bundle(&snapshot(RC_LOWPASS), "0".repeat(64).as_str(), &viewer()).expect("render");
    // A Windows checkout may carry the stylesheet with CRLF line endings.
    let styles = utf8(&bundle["assets/page.css"]).replace("\r\n", "\n");
    assert!(styles.contains("font-family: \"RSpice Plex Mono\";"));
    for (file, weight) in [
        ("ibm-plex-mono-regular.ttf", 400),
        ("ibm-plex-mono-medium.ttf", 500),
    ] {
        // The stylesheet names each cut relative to itself, at the weight
        // the face records, so a 500 line can never resolve to the 400 cut.
        assert!(
            styles.contains(&format!(
                "font-weight: {weight};\n  font-display: swap;\n  src: url(\"fonts/{file}\")"
            )),
            "page.css must declare {file} at weight {weight}"
        );
        assert_eq!(
            weight_class(&bundle[&format!("assets/fonts/{file}")]),
            weight,
            "{file} must be the {weight} cut"
        );
    }
    assert!(
        bundle.contains_key("assets/fonts/ofl-ibm-plex.txt"),
        "the faces travel with their licence"
    );
    // RSpice Symbols, for the symbols no Plex face carries, in one cut that
    // serves every weight so none is synthesized.
    assert!(
        styles.contains(
            "font-family: \"RSpice Symbols\";\n  font-style: normal;\n  font-weight: 100 900;\n  font-display: swap;\n  src: url(\"fonts/rspice-symbols-regular.ttf\")"
        ),
        "page.css must declare RSpice Symbols"
    );
    assert_eq!(
        weight_class(&bundle["assets/fonts/rspice-symbols-regular.ttf"]),
        400
    );
    assert!(
        bundle.contains_key("assets/fonts/ofl-rspice-symbols.txt"),
        "RSpice Symbols travels with its licence"
    );
    assert!(
        utf8(&bundle["index.html"]).contains("font-family=\"'RSpice Plex Mono', "),
        "the figures letter in the face the page ships"
    );
}

/// Asserts that `stack` names RSpice Symbols once, right after its Plex face,
/// or first where it names none.
fn assert_symbols_follow_plex(stack: &str) {
    let families: Vec<&str> = stack.split(',').collect();
    let symbols: Vec<usize> = families
        .iter()
        .enumerate()
        .filter(|(_, family)| family.contains("RSpice Symbols"))
        .map(|(index, _)| index)
        .collect();
    let plex = families.iter().position(|family| family.contains("Plex"));
    assert_eq!(
        symbols,
        [plex.map_or(0, |plex| plex + 1)],
        "{stack} does not name RSpice Symbols right after its Plex face, or first"
    );
}

/// Every font stack on the page, chrome and figures alike, names RSpice
/// Symbols right after its Plex face, or first where it has none, so a symbol
/// Plex lacks letters from the face the product paints and prints it in,
/// ahead of whichever installed face also carries it.
#[test]
fn every_font_stack_names_rspice_symbols_after_its_plex_face() {
    let bundle =
        render_bundle(&snapshot(RC_LOWPASS), "0".repeat(64).as_str(), &viewer()).expect("render");
    let styles = utf8(&bundle["assets/page.css"]).replace("\r\n", "\n");
    let mut rules = styles.as_str();
    let mut stacks = 0;
    while !rules.is_empty() {
        let (rule, rest) = rules.split_once('}').unwrap_or((rules, ""));
        rules = rest;
        // A face's own declaration names one family, not a stack.
        if rule.contains("@font-face") {
            continue;
        }
        for declaration in rule.split(['{', ';']).map(str::trim) {
            let Some(value) = declaration
                .strip_prefix("font:")
                .or_else(|| declaration.strip_prefix("font-family:"))
            else {
                continue;
            };
            if value.trim() == "inherit" {
                continue;
            }
            stacks += 1;
            assert_symbols_follow_plex(value);
        }
    }
    assert!(stacks >= 10, "only {stacks} font stacks were found");

    let html = utf8(&bundle["index.html"]);
    let figure_stacks: Vec<&str> = html
        .split("font-family=\"")
        .skip(1)
        .map(|rest| rest.split_once('"').expect("a closed attribute").0)
        .collect();
    for kind in ["monospace", "sans-serif"] {
        assert!(
            figure_stacks.iter().any(|stack| stack.contains(kind)),
            "no figure names a {kind} stack"
        );
    }
    for stack in figure_stacks {
        assert_symbols_follow_plex(stack);
    }

    // The face is fetched only for the characters it carries.
    let (_, range) = styles
        .split_once("unicode-range:")
        .expect("RSpice Symbols declares its unicode-range");
    let range = &range[..range.find(';').expect("the range ends")];
    let mut points = Vec::new();
    for item in range.split(',').map(str::trim) {
        let item = item.strip_prefix("U+").expect("a range item is U+hex");
        let (first, last) = item.split_once('-').unwrap_or((item, item));
        let first = u32::from_str_radix(first, 16).expect("hex");
        let last = u32::from_str_radix(last, 16).expect("hex");
        points.extend((first..=last).filter_map(char::from_u32));
    }
    assert_eq!(points.len(), 82, "the range lists every guaranteed symbol");
    for point in &points {
        assert!(
            rspice_fonts::Face::Symbols.covers(*point),
            "the range lists U+{:04X}, which RSpice Symbols lacks",
            u32::from(*point)
        );
    }
    for symbol in ['\u{2300}', '\u{23da}', '\u{25fb}'] {
        assert!(points.contains(&symbol), "the range omits {symbol}");
    }
}

/// The file types RSpice Cloud's render worker serves from a bundle (its
/// `BUNDLE_CONTENT_TYPES`). Any other file dead-letters the whole
/// publication, so a new kind of bundle file is admitted there first.
const CLOUD_SERVED_EXTENSIONS: [&str; 11] = [
    "html", "cir", "css", "csv", "json", "js", "png", "svg", "wasm", "ttf", "txt",
];

/// The RC low-pass publication, titled in Arabic, Hebrew and Thai, so its
/// bundle carries a face for each.
fn scripts_titled() -> PublicationSnapshot {
    let mut value = snapshot(RC_LOWPASS);
    value.metadata.title = "مرشح RC מסנן ตัวกรอง".to_owned();
    value
}

/// Every file of every bundle, including the faces for the scripts Plex Sans
/// lacks, has a path and a type the cloud serves. The cloud has served
/// lowercase `.ttf` paths under `assets/fonts/`, and `font-src 'self'`, since
/// RSpice-Cloud 149f04b, so a new face there needs no cloud change.
#[test]
fn every_bundle_path_is_one_the_cloud_render_worker_accepts() {
    let goldens = [
        snapshot(RC_LOWPASS),
        snapshot(MULTI_ANALYSIS),
        scripts_titled(),
    ];
    for value in goldens {
        let bundle = render_bundle(&value, "0".repeat(64).as_str(), &viewer()).expect("render");
        for path in bundle.keys() {
            // The worker's canonical path grammar: lowercase ASCII letters,
            // digits, `.`, `_`, `-` and `/`, opening on a letter or digit,
            // with no empty or dot segment and at most four directories deep.
            let canonical = path.len() <= 255
                && path.starts_with(|first: char| {
                    first.is_ascii_lowercase() || first.is_ascii_digit()
                })
                && path.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || matches!(byte, b'.' | b'_' | b'/' | b'-')
                })
                && path
                    .split('/')
                    .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
                && path.matches('/').count() <= 4;
            assert!(canonical, "{path} breaks the cloud bundle path grammar");
            let extension = path.rsplit_once('.').map(|(_, extension)| extension);
            assert!(
                extension.is_some_and(|extension| CLOUD_SERVED_EXTENSIONS.contains(&extension)),
                "{path} is a file type the cloud does not serve"
            );
        }
    }
}

#[test]
fn the_stylesheet_colours_every_paint_class_a_figure_can_carry() {
    let bundle =
        render_bundle(&snapshot(RC_LOWPASS), "0".repeat(64).as_str(), &viewer()).expect("render");
    let styles = utf8(&bundle["assets/page.css"]);
    let traces = (0..8).map(|index| format!("trace-{index}"));
    let named = [
        "foreground",
        "secondary",
        "grid",
        "accent",
        "warning",
        "success",
    ];
    let roles = named.map(str::to_owned).into_iter().chain(traces);
    for role in roles {
        // The root draws with `fill="none"`, so a class the page leaves
        // unstyled is invisible rather than black.
        for (prefix, property) in [("s", "stroke"), ("f", "fill"), ("t", "fill")] {
            let rule = format!("svg .{prefix}-{role} {{ {property}: var(--{role}); }}");
            assert!(styles.contains(&rule), "page.css must carry `{rule}`");
        }
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
            "assets/fonts/ibm-plex-sans-medium.ttf",
            "assets/fonts/ibm-plex-sans-regular.ttf",
            "assets/fonts/ibm-plex-sans-semibold.ttf",
            "assets/fonts/ofl-ibm-plex.txt",
            "assets/page.css",
            "assets/page.js",
            "figure-manifest.json",
            "index.html"
        ],
        "a fully withheld publication carries only the semantic shell and the faces it is          lettered in, with no viewer runtime"
    );
    assert!(
        !utf8(&bundle["index.html"]).contains("rspice-hydration")
            && !bundle.contains_key("assets/loader.js")
            && !bundle.contains_key("assets/viewer.js")
            && !bundle.contains_key("assets/viewer.wasm"),
        "a figureless page carries no hydration handshake or viewer runtime"
    );
}

const SCRIPT_FACES: [(&str, rspice_fonts::Role, rspice_fonts::Face, u16); 6] = [
    (
        "ibm-plex-sans-arabic-regular.ttf",
        rspice_fonts::Role::Sans,
        rspice_fonts::Face::SansArabicRegular,
        400,
    ),
    (
        "ibm-plex-sans-arabic-semibold.ttf",
        rspice_fonts::Role::SansSemibold,
        rspice_fonts::Face::SansArabicSemibold,
        600,
    ),
    (
        "ibm-plex-sans-hebrew-regular.ttf",
        rspice_fonts::Role::Sans,
        rspice_fonts::Face::SansHebrewRegular,
        400,
    ),
    (
        "ibm-plex-sans-hebrew-semibold.ttf",
        rspice_fonts::Role::SansSemibold,
        rspice_fonts::Face::SansHebrewSemibold,
        600,
    ),
    (
        "ibm-plex-sans-thai-regular.ttf",
        rspice_fonts::Role::Sans,
        rspice_fonts::Face::SansThaiRegular,
        400,
    ),
    (
        "ibm-plex-sans-thai-semibold.ttf",
        rspice_fonts::Role::SansSemibold,
        rspice_fonts::Face::SansThaiSemibold,
        600,
    ),
];

/// Every bundle letters its page in IBM Plex Sans, at the three weights
/// the stylesheet declares, and carries a face for a script Plex Sans lacks
/// only when its text is written in that script.
#[test]
fn a_bundle_carries_the_faces_its_text_is_lettered_in_and_no_others() {
    let latin =
        render_bundle(&snapshot(RC_LOWPASS), "0".repeat(64).as_str(), &viewer()).expect("render");
    let styles = utf8(&latin["assets/page.css"]).replace("\r\n", "\n");
    for (file, weight) in [
        ("ibm-plex-sans-regular.ttf", 400),
        ("ibm-plex-sans-medium.ttf", 500),
        ("ibm-plex-sans-semibold.ttf", 600),
    ] {
        assert!(
            styles.contains(&format!(
                "font-family: \"RSpice Plex Sans\";\n  font-style: normal;\n  font-weight: {weight};\n  font-display: swap;\n  src: url(\"fonts/{file}\")"
            )),
            "page.css must declare {file} at weight {weight}"
        );
        assert_eq!(
            weight_class(&latin[&format!("assets/fonts/{file}")]),
            weight
        );
    }
    assert!(latin.contains_key("assets/fonts/ofl-ibm-plex.txt"));
    assert!(
        !latin.keys().any(|path| path.contains("arabic")
            || path.contains("hebrew")
            || path.contains("thai")),
        "a Latin page carries no script face"
    );
    assert!(
        utf8(&latin["index.html"]).contains("font-family=\"'RSpice Plex Sans', 'RSpice Symbols', "),
        "the figures letter in the face the page ships"
    );

    let mut arabic = snapshot(RC_LOWPASS);
    arabic.metadata.title = "مرشح RC".to_owned();
    let arabic = render_bundle(&arabic, "0".repeat(64).as_str(), &viewer()).expect("render");
    let scripts =
        render_bundle(&scripts_titled(), "0".repeat(64).as_str(), &viewer()).expect("render");
    for (file, _, face, weight) in SCRIPT_FACES {
        let path = format!("assets/fonts/{file}");
        assert_eq!(
            arabic.contains_key(&path),
            file.contains("arabic"),
            "{file}"
        );
        assert_eq!(
            scripts.get(&path).map(Vec::as_slice),
            Some(face.bytes()),
            "{file}"
        );
        assert_eq!(weight_class(face.bytes()), weight, "{file}");
    }
}

/// The characters a script face letters in the product's chain, as a CSS
/// `unicode-range`: those it has and no face before it in `role`'s chain
/// has.
fn chain_range(role: rspice_fonts::Role, face: rspice_fonts::Face) -> Vec<String> {
    let mut ranges: Vec<(u32, u32)> = Vec::new();
    for point in (0..=0x10_FFFF_u32).filter(|&point| {
        char::from_u32(point).is_some_and(|character| role.face_for(character) == Some(face))
    }) {
        match ranges.last_mut() {
            Some((_, last)) if *last + 1 == point => *last = point,
            _ => ranges.push((point, point)),
        }
    }
    ranges
        .into_iter()
        .map(|(first, last)| {
            if first == last {
                format!("U+{first:04X}")
            } else {
                format!("U+{first:04X}-{last:04X}")
            }
        })
        .collect()
}

/// Each script face is filed under "RSpice Plex Sans" for exactly the
/// characters it letters in the product's chain: a page draws each from
/// the face the canvas and print draw it from, and fetches the face only
/// for text in its script.
#[test]
fn each_script_face_serves_the_characters_it_letters_in_the_product() {
    let styles = PAGE_STYLES.replace("\r\n", "\n");
    let mut wanted = Vec::new();
    for (file, role, face, weight) in SCRIPT_FACES {
        let at = styles
            .find(&format!("src: url(\"fonts/{file}\")"))
            .unwrap_or_else(|| panic!("page.css declares {file}"));
        let rule = &styles[styles[..at].rfind("@font-face").unwrap()..];
        let rule = &rule[..rule.find('}').unwrap()];
        assert!(
            rule.contains("font-family: \"RSpice Plex Sans\";"),
            "{rule}"
        );
        assert!(rule.contains(&format!("font-weight: {weight};")), "{rule}");
        let declared: Vec<String> = rule
            .split("unicode-range:")
            .nth(1)
            .and_then(|range| range.split(';').next())
            .unwrap_or_default()
            .split(',')
            .map(|item| item.trim().to_owned())
            .collect();
        let expected = chain_range(role, face);
        if declared != expected {
            wanted.push(format!("{file}={}", expected.join(", ")));
        }
    }
    assert!(wanted.is_empty(), "@@{}@@", wanted.join("@@"));
}
