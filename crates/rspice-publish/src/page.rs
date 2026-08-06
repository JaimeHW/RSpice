//! The published document page.
//!
//! One self-contained HTML document: inline styles, inline SVG figures, the
//! netlist text, the measurement table, raw-asset links, and a provenance
//! footer. No scripts are emitted here — the viewer runtime attaches through
//! the figure manifest, and the page must remain complete without it.

use std::fmt::Write as _;

use rspice_publication_contract::{FigureContent, PublicationSnapshot};

use crate::{Bundle, dataset_csv_path, escape_html, svg::scene_svg};

/// Palette, typography, and print rules for the published page and its
/// scenes. Scene classes are the `s-`/`f-`/`t-` role tokens emitted by the
/// SVG module.
pub const PAGE_STYLES: &str = r#":root{--paper:#f7f8f7;--card:#ffffff;--ink:#181c1a;--muted:#5c6663;--line:#dce1de;--accent:#0d6b52;--fail:#a33232;--sheet-frame:#7a827e;--symbol-body:#20423a;--wire:#155e49;--bus:#0f4a63;--junction:#155e49;--pin:#8a4d18;--net-label:#0f4a63;--reference-designator:#20423a;--component-value:#5c6663;--annotation:#7a5a1e;--plot-frame:#7a827e;--plot-grid:#c9cfcc;--plot-axis-text:#5c6663;--trace-0:#0d6b52;--trace-1:#1f5f8f;--trace-2:#a3572a;--trace-3:#6a4b8f;--trace-4:#8f6a1f;--trace-5:#2a7d7d;--trace-6:#9c3f68;--trace-7:#4a6b2a}
@media (prefers-color-scheme:dark){:root{--paper:#111514;--card:#181d1b;--ink:#e4e9e6;--muted:#94a09b;--line:#2b332f;--accent:#4cc19a;--fail:#e08585;--sheet-frame:#5d6662;--symbol-body:#cdd8d3;--wire:#57bd9c;--bus:#6aa8cf;--junction:#57bd9c;--pin:#cf9257;--net-label:#6aa8cf;--reference-designator:#cdd8d3;--component-value:#94a09b;--annotation:#cfa957;--plot-frame:#5d6662;--plot-grid:#2f3733;--plot-axis-text:#94a09b;--trace-0:#4cc19a;--trace-1:#6aa8cf;--trace-2:#d99a6c;--trace-3:#a98fd0;--trace-4:#cfb46a;--trace-5:#6ac4c4;--trace-6:#d284a9;--trace-7:#96b96a}}
*{box-sizing:border-box}
body{margin:0;background:var(--paper);color:var(--ink);font:15px/1.6 "Segoe UI",system-ui,sans-serif}
.page{max-width:960px;margin:0 auto;padding:40px 24px 80px}
header .eyebrow{font-family:Consolas,monospace;font-size:11px;letter-spacing:.14em;text-transform:uppercase;color:var(--accent);margin:0 0 8px}
h1{font-size:28px;line-height:1.2;margin:0 0 8px;letter-spacing:-.01em}
.description{color:var(--muted);margin:0 0 4px;max-width:70ch}
.byline{font-family:Consolas,monospace;font-size:12px;color:var(--muted);margin:0 0 32px}
h2{font-size:17px;margin:36px 0 10px}
figure{margin:0 0 28px;background:var(--card);border:1px solid var(--line);border-radius:8px;padding:16px}
figcaption{font-weight:600;margin-bottom:10px}
figure svg{display:block;width:100%;height:auto}
table{border-collapse:collapse;width:100%;font-size:13.5px;font-variant-numeric:tabular-nums}
th{text-align:left;font-family:Consolas,monospace;font-size:11px;letter-spacing:.1em;text-transform:uppercase;color:var(--muted);padding:6px 12px 6px 0;border-bottom:1px solid var(--line)}
td{padding:7px 12px 7px 0;border-bottom:1px solid var(--line)}
td.num{font-family:Consolas,monospace}
td.status{font-family:Consolas,monospace;font-size:12px}
td.status.pass{color:var(--accent)}
td.status.fail{color:var(--fail)}
pre.deck{background:var(--card);border:1px solid var(--line);border-radius:8px;padding:14px 16px;overflow-x:auto;font:12.5px/1.55 Consolas,monospace}
ul.assets{list-style:none;margin:0;padding:0}
ul.assets li{padding:6px 0;border-bottom:1px solid var(--line);font-size:13.5px}
ul.assets a{color:var(--accent);font-family:Consolas,monospace;text-decoration:none}
ul.assets a:hover,ul.assets a:focus-visible{text-decoration:underline}
footer{margin-top:48px;padding-top:14px;border-top:1px solid var(--line);font-family:Consolas,monospace;font-size:11.5px;color:var(--muted);overflow-wrap:anywhere}
svg .s-sheet-frame{stroke:var(--sheet-frame)}svg .s-symbol-body{stroke:var(--symbol-body)}svg .s-wire{stroke:var(--wire)}svg .s-bus{stroke:var(--bus)}svg .s-junction{stroke:var(--junction)}svg .s-pin{stroke:var(--pin)}svg .s-net-label{stroke:var(--net-label)}svg .s-plot-frame{stroke:var(--plot-frame)}svg .s-plot-grid{stroke:var(--plot-grid)}svg .s-annotation{stroke:var(--annotation)}
svg .s-trace-0{stroke:var(--trace-0)}svg .s-trace-1{stroke:var(--trace-1)}svg .s-trace-2{stroke:var(--trace-2)}svg .s-trace-3{stroke:var(--trace-3)}svg .s-trace-4{stroke:var(--trace-4)}svg .s-trace-5{stroke:var(--trace-5)}svg .s-trace-6{stroke:var(--trace-6)}svg .s-trace-7{stroke:var(--trace-7)}
svg .f-junction{fill:var(--junction)}svg .f-symbol-body{fill:var(--symbol-body)}svg .f-annotation{fill:var(--annotation)}
svg .t-net-label{fill:var(--net-label)}svg .t-reference-designator{fill:var(--reference-designator)}svg .t-component-value{fill:var(--component-value)}svg .t-plot-axis-text{fill:var(--plot-axis-text)}svg .t-annotation{fill:var(--annotation)}svg .t-sheet-frame{fill:var(--sheet-frame)}
@media print{body{background:#fff;color:#000}.page{max-width:none;padding:0}figure{break-inside:avoid;border:none;padding:0}ul.assets,footer{display:none}}
"#;

/// Assemble the complete `index.html` document.
pub fn document(
    snapshot: &PublicationSnapshot,
    snapshot_sha256_hex: &str,
    bundle: &Bundle,
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
    let _ = write!(
        html,
        "<p class=\"byline\">{author} · {created}</p>\n</header>\n<main>\n"
    );

    // Figures.
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
        let _ = write!(
            html,
            "<figure id=\"figure-{}\">\n<figcaption>{}</figcaption>\n{}\n</figure>\n",
            figure.id,
            escape_html(&figure.title),
            scene_svg(scene, &figure.title),
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

    // Provenance footer.
    let _ = write!(
        html,
        "</main>\n<footer>Published with RSpice {} · rendered by rspice-publish {} · sealed snapshot sha256:{} </footer>\n</div>\n</body>\n</html>\n",
        escape_html(&snapshot.metadata.app_version),
        env!("CARGO_PKG_VERSION"),
        escape_html(snapshot_sha256_hex),
    );
    html
}
