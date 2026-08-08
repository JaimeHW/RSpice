//! The published document page.
//!
//! One self-contained HTML document: inline styles, inline SVG figures, the
//! netlist text, the measurement table, raw-asset links, and a provenance
//! footer. The document is complete without any of the hydration pieces it
//! carries: an inert canvas and a hidden activation control per figure, a
//! JSON island of sealed digests, and one integrity-pinned loader script
//! that only ever adds to the static page.

use std::fmt::Write as _;

use rspice_publication_contract::{FigureContent, PublicationSnapshot};

use crate::{
    Bundle, dataset_csv_path, escape_html,
    hydration::{HydrationEmission, LOADER_PATH},
    svg::scene_svg,
};

/// Palette, typography, and print rules for the published page and its
/// scenes. Scene classes are the `s-`/`f-`/`t-` role tokens emitted by the
/// SVG module.
pub const PAGE_STYLES: &str = r#":root{--paper:#f7f8f7;--card:#ffffff;--ink:#181c1a;--muted:#5c6663;--line:#dce1de;--link:#0d6b52;--fail:#a33232;--foreground:#192026;--secondary:#5b6670;--grid:#b5bcc2;--accent:#c48b00;--warning:#be4336;--success:#007d52;--trace-0:#0d6b52;--trace-1:#1f5f8f;--trace-2:#a3572a;--trace-3:#6a4b8f;--trace-4:#8f6a1f;--trace-5:#2a7d7d;--trace-6:#9c3f68;--trace-7:#4a6b2a}
@media (prefers-color-scheme:dark){:root{--paper:#111514;--card:#181d1b;--ink:#e4e9e6;--muted:#94a09b;--line:#2b332f;--link:#4cc19a;--fail:#e08585;--foreground:#d9e0dc;--secondary:#94a09b;--grid:#333b37;--accent:#d9a75a;--warning:#e08585;--success:#4cc19a;--trace-0:#4cc19a;--trace-1:#6aa8cf;--trace-2:#d99a6c;--trace-3:#a98fd0;--trace-4:#cfb46a;--trace-5:#6ac4c4;--trace-6:#d284a9;--trace-7:#96b96a}}
*{box-sizing:border-box}
body{margin:0;background:var(--paper);color:var(--ink);font:15px/1.6 "Segoe UI",system-ui,sans-serif}
.page{max-width:960px;margin:0 auto;padding:40px 24px 80px}
header .eyebrow{font-family:Consolas,monospace;font-size:11px;letter-spacing:.14em;text-transform:uppercase;color:var(--link);margin:0 0 8px}
h1{font-size:28px;line-height:1.2;margin:0 0 8px;letter-spacing:-.01em}
.description{color:var(--muted);margin:0 0 4px;max-width:70ch}
.byline{font-family:Consolas,monospace;font-size:12px;color:var(--muted);margin:0 0 32px}
h2{font-size:17px;margin:36px 0 10px}
figure{margin:0 0 28px;background:var(--card);border:1px solid var(--line);border-radius:8px;padding:16px}
figcaption{font-weight:600;margin-bottom:10px}
figure svg{display:block;width:100%;height:auto}
figure canvas.viewer{display:block;width:100%;border-radius:4px}
button.hydrate{margin-top:10px;padding:5px 12px;background:none;border:1px solid var(--line);border-radius:4px;color:var(--link);font:12px Consolas,monospace;letter-spacing:.02em;cursor:pointer}
button.hydrate:hover:enabled,button.hydrate:focus-visible{border-color:var(--link)}
button.hydrate:disabled{color:var(--muted);cursor:default}
table{border-collapse:collapse;width:100%;font-size:13.5px;font-variant-numeric:tabular-nums}
th{text-align:left;font-family:Consolas,monospace;font-size:11px;letter-spacing:.1em;text-transform:uppercase;color:var(--muted);padding:6px 12px 6px 0;border-bottom:1px solid var(--line)}
td{padding:7px 12px 7px 0;border-bottom:1px solid var(--line)}
td.num{font-family:Consolas,monospace}
td.status{font-family:Consolas,monospace;font-size:12px}
td.status.pass{color:var(--link)}
td.status.fail{color:var(--fail)}
pre.deck{background:var(--card);border:1px solid var(--line);border-radius:8px;padding:14px 16px;overflow-x:auto;font:12.5px/1.55 Consolas,monospace}
ul.assets{list-style:none;margin:0;padding:0}
ul.assets li{padding:6px 0;border-bottom:1px solid var(--line);font-size:13.5px}
ul.assets a{color:var(--link);font-family:Consolas,monospace;text-decoration:none}
ul.assets a:hover,ul.assets a:focus-visible{text-decoration:underline}
footer{margin-top:48px;padding-top:14px;border-top:1px solid var(--line);font-family:Consolas,monospace;font-size:11.5px;color:var(--muted);overflow-wrap:anywhere}
svg .s-foreground{stroke:var(--foreground)}svg .s-secondary{stroke:var(--secondary)}svg .s-grid{stroke:var(--grid)}svg .s-accent{stroke:var(--accent)}svg .s-warning{stroke:var(--warning)}svg .s-success{stroke:var(--success)}
svg .s-trace-0{stroke:var(--trace-0)}svg .s-trace-1{stroke:var(--trace-1)}svg .s-trace-2{stroke:var(--trace-2)}svg .s-trace-3{stroke:var(--trace-3)}svg .s-trace-4{stroke:var(--trace-4)}svg .s-trace-5{stroke:var(--trace-5)}svg .s-trace-6{stroke:var(--trace-6)}svg .s-trace-7{stroke:var(--trace-7)}
svg .f-foreground{fill:var(--foreground)}svg .f-secondary{fill:var(--secondary)}svg .f-grid{fill:var(--grid)}svg .f-accent{fill:var(--accent)}svg .f-warning{fill:var(--warning)}svg .f-success{fill:var(--success)}
svg .f-trace-0{fill:var(--trace-0)}svg .f-trace-1{fill:var(--trace-1)}svg .f-trace-2{fill:var(--trace-2)}svg .f-trace-3{fill:var(--trace-3)}svg .f-trace-4{fill:var(--trace-4)}svg .f-trace-5{fill:var(--trace-5)}svg .f-trace-6{fill:var(--trace-6)}svg .f-trace-7{fill:var(--trace-7)}
svg .t-foreground{fill:var(--foreground)}svg .t-secondary{fill:var(--secondary)}svg .t-grid{fill:var(--grid)}svg .t-accent{fill:var(--accent)}svg .t-warning{fill:var(--warning)}svg .t-success{fill:var(--success)}
@media print{body{background:#fff;color:#000}.page{max-width:none;padding:0}figure{break-inside:avoid;border:none;padding:0}ul.assets,footer{display:none}figure svg{display:block!important}canvas.viewer,button.hydrate{display:none!important}}
"#;

/// Assemble the complete `index.html` document.
pub fn document(
    snapshot: &PublicationSnapshot,
    snapshot_sha256_hex: &str,
    bundle: &Bundle,
    hydration: Option<&HydrationEmission>,
) -> String {
    let title = escape_html(&snapshot.metadata.title);
    let author = escape_html(&snapshot.metadata.author_display);
    let created = escape_html(&snapshot.metadata.created_utc);
    let description_plain = snapshot
        .metadata
        .description
        .split('\n')
        .next()
        .unwrap_or_default();

    let mut html = String::new();
    let _ = write!(
        html,
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{title}</title>\n"
    );
    if !description_plain.is_empty() {
        let _ = writeln!(
            html,
            "<meta name=\"description\" content=\"{}\">",
            escape_html(description_plain)
        );
    }
    let _ = write!(
        html,
        "<meta property=\"og:title\" content=\"{title}\">\n<meta property=\"og:type\" content=\"article\">\n<meta property=\"og:site_name\" content=\"RSpice\">\n"
    );
    if !description_plain.is_empty() {
        let _ = writeln!(
            html,
            "<meta property=\"og:description\" content=\"{}\">",
            escape_html(description_plain)
        );
    }
    let _ = write!(
        html,
        "<style>{PAGE_STYLES}</style>\n</head>\n<body>\n<div class=\"page\">\n"
    );

    // Header.
    let _ = write!(
        html,
        "<header>\n<p class=\"eyebrow\">RSpice publication</p>\n<h1>{title}</h1>\n"
    );
    for line in snapshot
        .metadata
        .description
        .split('\n')
        .filter(|line| !line.trim().is_empty())
    {
        let _ = writeln!(html, "<p class=\"description\">{}</p>", escape_html(line));
    }
    let license = escape_html(snapshot.metadata.license.display_name());
    let _ = write!(
        html,
        "<p class=\"byline\">{author} · {created} · {license}</p>\n</header>\n<main>\n"
    );

    // Figures. The canvas and activation control are inert until the loader
    // reveals them; the static SVG is the figure until then and again on
    // print or any hydration rejection.
    for figure in &snapshot.figures {
        let scene = match &figure.content {
            FigureContent::SchematicSheet { sheet_index } => {
                &snapshot
                    .schematic
                    .as_ref()
                    .expect("validated snapshot has a schematic for sheet figures")
                    .sheets[*sheet_index as usize]
                    .scene
            }
            FigureContent::Plot(plot) => &plot.scene,
        };
        let control = match &figure.content {
            FigureContent::SchematicSheet { .. } => "Interactive schematic",
            FigureContent::Plot(_) => "Interactive plot",
        };
        let _ = write!(
            html,
            "<figure id=\"figure-{id}\">\n<figcaption>{}</figcaption>\n{}\n<canvas id=\"figure-{id}-canvas\" class=\"viewer\" style=\"aspect-ratio:{} / {}\" hidden></canvas>\n<button class=\"hydrate\" type=\"button\" hidden>{control}</button>\n</figure>\n",
            escape_html(&figure.title),
            scene_svg(scene, &figure.title),
            scene.width_um,
            scene.height_um,
            id = figure.id,
        );
    }

    // Measurements.
    if let Some(results) = &snapshot.results
        && !results.measurements.is_empty()
    {
        html.push_str(
            "<section>\n<h2>Measurements</h2>\n<table>\n<tr><th>Measurement</th><th>Value</th><th>Spec</th><th>Status</th></tr>\n",
        );
        for measurement in &results.measurements {
            html.push_str(&crate::measurement_row(measurement));
            html.push('\n');
        }
        html.push_str("</table>\n</section>\n");
    }

    // Netlist.
    if let Some(netlist) = &snapshot.netlist {
        let _ = write!(
            html,
            "<section>\n<h2>Netlist</h2>\n<pre class=\"deck\">{}</pre>\n</section>\n",
            escape_html(&netlist.deck)
        );
    }

    // Raw assets.
    let mut assets: Vec<(String, String)> = Vec::new();
    if snapshot.netlist.is_some() {
        assets.push(("netlist.cir".to_string(), "Netlist deck".to_string()));
    }
    if let Some(results) = &snapshot.results {
        for dataset in &results.datasets {
            let label = match &dataset.variant {
                Some(variant) => format!("{} · {variant} (CSV)", dataset.name),
                None => format!("{} (CSV)", dataset.name),
            };
            assets.push((dataset_csv_path(dataset), label));
        }
    }
    if snapshot.disclosure.archive {
        assets.push((
            "archive.rspice".to_string(),
            "Project archive · open in RSpice".to_string(),
        ));
    }
    if !assets.is_empty() {
        html.push_str("<section>\n<h2>Data</h2>\n<ul class=\"assets\">\n");
        for (path, label) in &assets {
            debug_assert!(
                path == "archive.rspice" || bundle.contains_key(path),
                "asset links must resolve inside the bundle"
            );
            let _ = writeln!(
                html,
                "<li><a href=\"{}\" download>{}</a> — {}</li>",
                escape_html(path),
                escape_html(path),
                escape_html(label),
            );
        }
        html.push_str("</ul>\n</section>\n");
    }

    // Hydration handshake: the digests the loader verifies, then the
    // integrity-pinned loader itself. Both are inert additions — a reader
    // who never activates a figure downloads neither runtime nor payloads.
    if let Some(hydration) = hydration {
        let _ = write!(
            html,
            "</main>\n<script type=\"application/json\" id=\"rspice-hydration\">{}</script>\n<script type=\"module\" src=\"{LOADER_PATH}\" integrity=\"{}\"></script>\n",
            hydration.island_json, hydration.loader_integrity,
        );
    } else {
        html.push_str("</main>\n");
    }

    // Provenance footer.
    let _ = write!(
        html,
        "<footer>Published with RSpice {} · rendered by rspice-publish {} · sealed snapshot sha256:{} </footer>\n</div>\n</body>\n</html>\n",
        escape_html(&snapshot.metadata.app_version),
        env!("CARGO_PKG_VERSION"),
        escape_html(snapshot_sha256_hex),
    );
    html
}
