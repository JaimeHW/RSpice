//! Deterministic renderer from sealed publication snapshots to immutable
//! page bundles.
//!
//! The input is a validated [`PublicationSnapshot`]; the output is a
//! [`BTreeMap`] from bundle-relative path to exact bytes: the document page
//! (`index.html` with inline styles and inline SVG figures), raw data assets
//! (`netlist.cir`, per-dataset CSV), and the figure-hydration handshake
//! (`figure-manifest.json` plus one payload per figure) for the viewer
//! runtime. Identical snapshots always produce byte-identical bundles — the
//! cloud pipeline digests and seals what this crate emits.
//!
//! Nothing here consults the network, the clock, the environment, or any
//! state outside the snapshot. Every author-controlled string is escaped at
//! this boundary; the contract already rejects control characters, so
//! escaping is the only transformation text needs.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use rspice_publication_contract::{
    Dataset, FIGURE_MANIFEST_SCHEMA_VERSION, Figure, FigureContent, FigureManifest, FigurePayload,
    ManifestEntry, ManifestFigureKind, Measurement, PayloadRef, PublicationSnapshot, TraceValues,
    Validate as _,
};
use sha2::{Digest as _, Sha256};
use thiserror::Error;

mod hydration;
mod page;
mod svg;

pub use hydration::ViewerRuntime;
pub use page::PAGE_STYLES;

/// Everything that can stop a bundle from being produced. Contract
/// violations surface verbatim so the pipeline can log the producing
/// client's exact defect.
#[derive(Debug, Error)]
pub enum RenderError {
    #[error("snapshot rejected: {0}")]
    Contract(#[from] rspice_publication_contract::ContractError),
    #[error("figure payload serialization failed: {0}")]
    Payload(String),
    #[error("viewer runtime rejected: {0}")]
    ViewerRuntime(String),
}

/// A rendered bundle: bundle-relative path → exact bytes. `BTreeMap` keeps
/// emission order deterministic for digesting and archiving.
pub type Bundle = BTreeMap<String, Vec<u8>>;

/// HTML-escape one author-controlled string.
#[must_use]
pub fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            other => escaped.push(other),
        }
    }
    escaped
}

/// Lowercase filesystem-safe slug for asset filenames.
fn slug(text: &str) -> String {
    let mut out = String::new();
    let mut pending_dash = false;
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            if pending_dash && !out.is_empty() {
                out.push('-');
            }
            pending_dash = false;
            out.push(c.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }
    if out.is_empty() {
        "dataset".to_string()
    } else {
        out
    }
}

fn fmt_f64(value: f64) -> String {
    format!("{value}")
}

/// Deterministic CSV for one dataset. Complex traces split into real and
/// imaginary columns; every value prints as Rust's shortest round-trip form.
fn dataset_csv(dataset: &Dataset) -> Vec<u8> {
    let mut header = String::new();
    let unit_suffix = |unit: &str| {
        if unit.is_empty() {
            String::new()
        } else {
            format!(" ({unit})")
        }
    };
    let _ = write!(
        header,
        "{}{}",
        dataset.sweep.label,
        unit_suffix(&dataset.sweep.unit)
    );
    for trace in &dataset.traces {
        match &trace.values {
            TraceValues::Real { .. } => {
                let _ = write!(header, ",{}{}", trace.label, unit_suffix(&trace.unit));
            }
            TraceValues::Complex { .. } => {
                let _ = write!(
                    header,
                    ",{} re{suffix},{} im{suffix}",
                    trace.label,
                    trace.label,
                    suffix = unit_suffix(&trace.unit)
                );
            }
        }
    }
    let mut csv = header;
    csv.push('\n');
    for (row, &x_bits) in dataset.sweep.values_bits.iter().enumerate() {
        let mut line = fmt_f64(f64::from_bits(x_bits));
        for trace in &dataset.traces {
            match &trace.values {
                TraceValues::Real { bits } => {
                    let _ = write!(line, ",{}", fmt_f64(f64::from_bits(bits[row])));
                }
                TraceValues::Complex {
                    real_bits,
                    imaginary_bits,
                } => {
                    let _ = write!(
                        line,
                        ",{},{}",
                        fmt_f64(f64::from_bits(real_bits[row])),
                        fmt_f64(f64::from_bits(imaginary_bits[row]))
                    );
                }
            }
        }
        csv.push_str(&line);
        csv.push('\n');
    }
    csv.into_bytes()
}

/// `data/<id>-<name>[-<variant>].csv`
fn dataset_csv_path(dataset: &Dataset) -> String {
    match &dataset.variant {
        Some(variant) => format!(
            "data/{}-{}-{}.csv",
            dataset.id,
            slug(&dataset.name),
            slug(variant)
        ),
        None => format!("data/{}-{}.csv", dataset.id, slug(&dataset.name)),
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

fn figure_scene<'a>(
    snapshot: &'a PublicationSnapshot,
    figure: &'a Figure,
) -> &'a rspice_publication_contract::Scene {
    match &figure.content {
        FigureContent::SchematicSheet { sheet_index } => {
            // Validation guarantees the reference resolves.
            &snapshot
                .schematic
                .as_ref()
                .expect("validated snapshot has a schematic for sheet figures")
                .sheets[*sheet_index as usize]
                .scene
        }
        FigureContent::Plot(plot) => &plot.scene,
    }
}

/// The self-contained hydration payload for one figure: its scene plus
/// exactly the datasets its bindings reference.
fn figure_payload(
    snapshot: &PublicationSnapshot,
    figure: &Figure,
) -> Result<FigurePayload, RenderError> {
    let (datasets, hydration) = match &figure.content {
        FigureContent::SchematicSheet { .. } => (Vec::new(), None),
        FigureContent::Plot(plot) => match &plot.hydration {
            None => (Vec::new(), None),
            Some(hydration) => {
                let all = snapshot
                    .results
                    .as_ref()
                    .map_or(&[][..], |results| &results.datasets[..]);
                let mut referenced: Vec<Dataset> = Vec::new();
                for binding in &hydration.bindings {
                    if referenced.iter().any(|d| d.id == binding.dataset_id) {
                        continue;
                    }
                    if let Some(dataset) = all.iter().find(|d| d.id == binding.dataset_id) {
                        referenced.push(dataset.clone());
                    }
                }
                (referenced, Some(hydration.clone()))
            }
        },
    };
    Ok(FigurePayload {
        schema_version: FIGURE_MANIFEST_SCHEMA_VERSION,
        figure_id: figure.id,
        scene: figure_scene(snapshot, figure).clone(),
        datasets,
        hydration,
    })
}

fn measurement_row(measurement: &Measurement) -> String {
    let status = match measurement.passed {
        Some(true) => "<td><span class=\"status pass\">PASS</span></td>",
        Some(false) => "<td><span class=\"status fail\">FAIL</span></td>",
        None => "<td><span class=\"status neutral\">—</span></td>",
    };
    format!(
        "<tr><td>{}</td><td class=\"num\">{}</td><td class=\"num\">{}</td>{}</tr>",
        escape_html(&measurement.name),
        escape_html(&measurement.display),
        measurement
            .spec_display
            .as_deref()
            .map_or_else(|| "—".to_string(), escape_html),
        status,
    )
}

/// Render the complete page bundle from a snapshot that has already passed
/// [`rspice_publication_contract::Validate`]. `snapshot_sha256_hex` is the
/// digest of the exact canonical snapshot bytes, printed in the provenance
/// footer so a published page names its sealed source. `viewer` is the built
/// viewer runtime sealed into every figure-bearing bundle; a snapshot with
/// no figures emits no runtime bytes at all.
pub fn render_bundle(
    snapshot: &PublicationSnapshot,
    snapshot_sha256_hex: &str,
    viewer: &ViewerRuntime,
) -> Result<Bundle, RenderError> {
    snapshot.validate()?;
    let mut bundle = Bundle::new();

    // Page chrome is sealed like every other publication resource. Keeping
    // it external lets the serving policy admit authored styles and behavior
    // from same-origin manifest entries instead of inline executable text.
    bundle.insert(
        page::PAGE_CSS_PATH.to_string(),
        page::PAGE_STYLES.as_bytes().to_vec(),
    );
    bundle.insert(
        page::PAGE_JS_PATH.to_string(),
        page::PAGE_SCRIPT.as_bytes().to_vec(),
    );

    if let Some(netlist) = &snapshot.netlist {
        bundle.insert("netlist.cir".to_string(), netlist.deck.clone().into_bytes());
    }
    if let Some(results) = &snapshot.results {
        for dataset in &results.datasets {
            bundle.insert(dataset_csv_path(dataset), dataset_csv(dataset));
        }
    }

    let mut manifest_entries = Vec::new();
    for figure in &snapshot.figures {
        let payload = figure_payload(snapshot, figure)?;
        let bytes = payload
            .canonical_bytes()
            .map_err(|error| RenderError::Payload(error.to_string()))?;
        let path = format!("figures/{}.json", figure.id);
        manifest_entries.push(ManifestEntry {
            figure_id: figure.id,
            dom_id: format!("figure-{}", figure.id),
            kind: match &figure.content {
                FigureContent::SchematicSheet { .. } => ManifestFigureKind::SchematicSheet,
                FigureContent::Plot(_) => ManifestFigureKind::Plot,
            },
            payload: PayloadRef {
                sha256_hex: sha256_hex(&bytes),
                byte_len: bytes.len() as u64,
                path: path.clone(),
            },
        });
        bundle.insert(path, bytes);
    }
    let manifest = FigureManifest {
        schema_version: FIGURE_MANIFEST_SCHEMA_VERSION,
        figures: manifest_entries,
    };
    bundle.insert(
        "figure-manifest.json".to_string(),
        manifest
            .canonical_bytes()
            .map_err(|error| RenderError::Payload(error.to_string()))?,
    );

    let emission = if manifest.figures.is_empty() {
        None
    } else {
        bundle.insert(
            hydration::LOADER_PATH.to_string(),
            hydration::LOADER_JS.as_bytes().to_vec(),
        );
        bundle.insert(
            hydration::VIEWER_JS_PATH.to_string(),
            viewer.js_glue.clone(),
        );
        bundle.insert(hydration::VIEWER_WASM_PATH.to_string(), viewer.wasm.clone());
        Some(hydration::HydrationEmission::new(
            &manifest.figures,
            viewer,
        )?)
    };

    bundle.insert(
        "index.html".to_string(),
        page::document(snapshot, snapshot_sha256_hex, &bundle, emission.as_ref()).into_bytes(),
    );
    Ok(bundle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escaping_neutralizes_markup() {
        assert_eq!(
            escape_html("<script>alert(\"x&y\")</script>"),
            "&lt;script&gt;alert(&quot;x&amp;y&quot;)&lt;/script&gt;"
        );
    }

    #[test]
    fn slugs_are_filesystem_safe() {
        assert_eq!(slug("27 °C"), "27-c");
        assert_eq!(slug("tran1"), "tran1");
        assert_eq!(slug("···"), "dataset");
    }

    #[test]
    fn shortest_round_trip_formatting_is_exact() {
        for value in [0.0, 0.5e-3, 3.2e6, 0.1, -61.4] {
            let printed = fmt_f64(value);
            assert_eq!(printed.parse::<f64>().unwrap(), value, "{printed}");
        }
    }
}
