//! Production document shell for published circuits.
//!
//! The semantic document is complete without JavaScript: every disclosed
//! figure, result, measurement, deck, download, and provenance fact is in
//! the HTML. Sealed page chrome progressively adds tabs, theme selection,
//! and sharing, while the independent hydration loader can replace a static
//! figure with its digest-verified interactive canvas on demand.

use std::fmt::Write as _;

use rspice_publication_contract::{
    Figure, FigureContent, FigurePresentation, PublicationSection, PublicationSnapshot, Scene,
    WarningSeverity,
};

use crate::{
    Bundle, dataset_csv_path, escape_html,
    hydration::{HydrationEmission, LOADER_PATH, sri_sha384},
    svg::scene_svg,
};

pub(crate) const PAGE_CSS_PATH: &str = "assets/page.css";
pub(crate) const PAGE_JS_PATH: &str = "assets/page.js";

/// Authored production stylesheet, emitted byte-for-byte into every bundle.
pub const PAGE_STYLES: &str = include_str!("assets/page.css");
/// Progressive page chrome, separate from the figure hydration loader.
pub(crate) const PAGE_SCRIPT: &str = include_str!("assets/page.js");

struct AssetLink {
    path: String,
    label: String,
    detail: String,
    bytes: usize,
}

fn figure_scene<'a>(snapshot: &'a PublicationSnapshot, figure: &'a Figure) -> &'a Scene {
    match &figure.content {
        FigureContent::SchematicSheet { sheet_index } => {
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

fn asset_links(snapshot: &PublicationSnapshot, bundle: &Bundle) -> Vec<AssetLink> {
    let mut assets = Vec::new();
    if snapshot.netlist.is_some()
        && let Some(bytes) = bundle.get("netlist.cir")
    {
        assets.push(AssetLink {
            path: "netlist.cir".to_string(),
            label: "SPICE netlist".to_string(),
            detail: "The exact deck included in this publication".to_string(),
            bytes: bytes.len(),
        });
    }
    if let Some(results) = &snapshot.results {
        for dataset in &results.datasets {
            let path = dataset_csv_path(dataset);
            let Some(bytes) = bundle.get(&path) else {
                continue;
            };
            let detail = match &dataset.variant {
                Some(variant) => format!("{} · {variant} · CSV data", dataset.name),
                None => format!("{} · CSV data", dataset.name),
            };
            assets.push(AssetLink {
                label: format!("{}.csv", dataset.name),
                path,
                detail,
                bytes: bytes.len(),
            });
        }
    }
    assets
}

fn format_bytes(bytes: usize) -> String {
    const KIB: usize = 1024;
    const MIB: usize = KIB * KIB;
    if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{bytes} B")
    }
}

fn section_id(section: PublicationSection) -> &'static str {
    match section {
        PublicationSection::Overview => "overview",
        PublicationSection::Schematic => "schematic",
        PublicationSection::Results => "results",
        PublicationSection::Components => "components",
        PublicationSection::Files => "files",
        PublicationSection::Details => "details",
    }
}

fn render_tab(html: &mut String, id: &str, label: &str, count: Option<usize>) {
    let _ = write!(html, "<a href=\"#{id}\" data-tab=\"{id}\">{label}");
    if let Some(count) = count {
        let _ = write!(html, " <span class=\"tab-count\">{count}</span>");
    }
    html.push_str("</a>\n");
}

fn render_panel_header(html: &mut String, title: &str, description: &str) {
    let _ = writeln!(
        html,
        "<div class=\"panel-header\"><div><h2>{}</h2><p>{}</p></div></div>",
        escape_html(title),
        escape_html(description)
    );
}

fn figure_presentation<'a>(
    snapshot: &'a PublicationSnapshot,
    figure: &Figure,
) -> Option<&'a FigurePresentation> {
    snapshot.presentation.as_ref().and_then(|presentation| {
        presentation
            .figure_details
            .iter()
            .find(|detail| detail.figure_id == figure.id)
    })
}

fn render_figure(html: &mut String, snapshot: &PublicationSnapshot, figure: &Figure) {
    let scene = figure_scene(snapshot, figure);
    let presentation = figure_presentation(snapshot, figure);
    let accessible_label = presentation.map_or(figure.title.as_str(), |detail| {
        detail.accessible_summary.as_str()
    });
    let default_interactive = presentation.is_some_and(|detail| detail.default_interactive);
    let (kind, control, hint) = match &figure.content {
        FigureContent::SchematicSheet { .. } => (
            "Schematic",
            "Open interactive schematic",
            "Pan, zoom, and inspect the published drawing",
        ),
        FigureContent::Plot(_) => (
            "Result plot",
            "Open interactive plot",
            "Inspect the sealed simulation result",
        ),
    };
    let _ = write!(
        html,
        "<figure class=\"figure-card\" id=\"figure-{id}\"{default_interactive}>\n\
         <div class=\"figure-heading\"><figcaption>{title}</figcaption><span class=\"figure-kind\">{kind}</span></div>\n\
         <div class=\"figure-stage\">{svg}<canvas id=\"figure-{id}-canvas\" class=\"viewer\" style=\"aspect-ratio:{width} / {height}\" hidden></canvas></div>\n",
        id = figure.id,
        title = escape_html(&figure.title),
        svg = scene_svg(scene, accessible_label),
        width = scene.width_um,
        height = scene.height_um,
        default_interactive = if default_interactive {
            " data-default-interactive"
        } else {
            ""
        },
    );
    if let Some(detail) = presentation {
        let _ = write!(
            html,
            "<div class=\"figure-description\"><p>{}</p>",
            escape_html(&detail.accessible_summary)
        );
        if let Some(caption) = &detail.caption {
            let _ = write!(
                html,
                "<p class=\"figure-caption\">{}</p>",
                escape_html(caption)
            );
        }
        html.push_str("</div>\n");
    }
    let _ = write!(
        html,
        "<div class=\"figure-actions\"><button class=\"button primary hydrate\" type=\"button\" hidden>{control}</button><button class=\"button\" type=\"button\" data-figure-fullscreen data-js-only hidden>Fullscreen</button><button class=\"button\" type=\"button\" data-figure-svg data-js-only hidden>Download SVG</button><span class=\"hint\">{hint}</span></div>\n</figure>\n"
    );
}

fn render_measurements(html: &mut String, snapshot: &PublicationSnapshot) {
    let Some(results) = &snapshot.results else {
        return;
    };
    if results.measurements.is_empty() {
        return;
    }
    html.push_str(
        "<section class=\"subsection\" aria-labelledby=\"measurements-heading\">\n\
         <h3 id=\"measurements-heading\">Measurements</h3>\n\
         <div class=\"table-wrap\"><table><caption>Published scalar measurements and declared limits</caption>\
         <thead><tr><th scope=\"col\">Measurement</th><th scope=\"col\">Value</th><th scope=\"col\">Specification</th><th scope=\"col\">Status</th></tr></thead><tbody>\n",
    );
    for measurement in &results.measurements {
        html.push_str(&crate::measurement_row(measurement));
        html.push('\n');
    }
    html.push_str("</tbody></table></div></section>\n");
}

fn render_analyses(html: &mut String, snapshot: &PublicationSnapshot) {
    let Some(results) = &snapshot.results else {
        return;
    };
    if results.analyses.is_empty() {
        return;
    }
    html.push_str(
        "<section class=\"subsection\" aria-labelledby=\"analyses-heading\">\n\
         <h3 id=\"analyses-heading\">Published analyses</h3>\n\
         <div class=\"table-wrap\"><table><thead><tr><th scope=\"col\">Analysis</th><th scope=\"col\">Control card</th></tr></thead><tbody>\n",
    );
    for analysis in &results.analyses {
        let _ = writeln!(
            html,
            "<tr><td>{}</td><td class=\"num\">{}</td></tr>",
            escape_html(&analysis.label),
            escape_html(&analysis.card),
        );
    }
    html.push_str("</tbody></table></div></section>\n");
}

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
    let license = escape_html(snapshot.metadata.license.display_name());
    let description_plain = snapshot
        .metadata
        .description
        .split('\n')
        .next()
        .unwrap_or_default();
    let schematic_figures: Vec<&Figure> = snapshot
        .figures
        .iter()
        .filter(|figure| matches!(figure.content, FigureContent::SchematicSheet { .. }))
        .collect();
    let plot_figures: Vec<&Figure> = snapshot
        .figures
        .iter()
        .filter(|figure| matches!(figure.content, FigureContent::Plot(_)))
        .collect();
    let schematic_count = snapshot
        .schematic
        .as_ref()
        .map_or(0, |schematic| schematic.sheets.len());
    let (analysis_count, dataset_count, measurement_count) =
        snapshot.results.as_ref().map_or((0, 0, 0), |results| {
            (
                results.analyses.len(),
                results.datasets.len(),
                results.measurements.len(),
            )
        });
    let assets = asset_links(snapshot, bundle);
    let component_count = snapshot
        .engineering
        .as_ref()
        .map_or(0, |engineering| engineering.components.len());
    let mut fallback_order = vec![PublicationSection::Overview];
    if snapshot.schematic.is_some() {
        fallback_order.push(PublicationSection::Schematic);
    }
    if !plot_figures.is_empty() || snapshot.results.is_some() {
        fallback_order.push(PublicationSection::Results);
    }
    if component_count > 0 {
        fallback_order.push(PublicationSection::Components);
    }
    if !assets.is_empty() {
        fallback_order.push(PublicationSection::Files);
    }
    fallback_order.push(PublicationSection::Details);
    let section_order = snapshot
        .presentation
        .as_ref()
        .map_or(fallback_order.as_slice(), |presentation| {
            presentation.section_order.as_slice()
        });
    let default_section = snapshot
        .presentation
        .as_ref()
        .map_or(PublicationSection::Overview, |presentation| {
            presentation.default_section
        });
    let authored_overview = snapshot
        .presentation
        .as_ref()
        .and_then(|presentation| presentation.overview.as_ref());
    let page_css_integrity = sri_sha384(PAGE_STYLES.as_bytes());
    let page_js_integrity = sri_sha384(PAGE_SCRIPT.as_bytes());

    let mut html = String::new();
    let _ = write!(
        html,
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <meta name=\"color-scheme\" content=\"light dark\">\n\
         <meta name=\"theme-color\" content=\"#0d1210\">\n<title>{title} · RSpice</title>\n"
    );
    if !description_plain.is_empty() {
        let description = escape_html(description_plain);
        let _ = writeln!(
            html,
            "<meta name=\"description\" content=\"{description}\">"
        );
        let _ = writeln!(
            html,
            "<meta property=\"og:description\" content=\"{description}\">"
        );
        let _ = writeln!(
            html,
            "<meta name=\"twitter:description\" content=\"{description}\">"
        );
    }
    let _ = write!(
        html,
        "<meta property=\"og:title\" content=\"{title}\">\n\
         <meta property=\"og:type\" content=\"article\">\n\
         <meta property=\"og:site_name\" content=\"RSpice\">\n\
         <meta name=\"twitter:card\" content=\"summary\">\n\
         <meta name=\"twitter:title\" content=\"{title}\">\n\
         <link rel=\"rspice-publication-context\" href=\"context.json\">\n\
         <!--rspice-cloud-head-->\n\
         <link rel=\"stylesheet\" href=\"{PAGE_CSS_PATH}\" integrity=\"{page_css_integrity}\">\n\
         <script type=\"module\" src=\"{PAGE_JS_PATH}\" integrity=\"{page_js_integrity}\"></script>\n\
         </head>\n<body>\n<a class=\"skip-link\" href=\"#publication-content\">Skip to publication content</a>\n\
         <div class=\"shell\">\n<nav class=\"publication-nav\" aria-label=\"Publication\">\n\
         <div class=\"brand\"><span class=\"brand-mark\" aria-hidden=\"true\">R</span><span>RSpice</span><span class=\"brand-context\">Published circuit</span></div>\n\
         <div class=\"nav-actions\">\n\
         <button class=\"button\" type=\"button\" data-theme-toggle data-js-only hidden><span aria-hidden=\"true\">◐</span><span class=\"button-label\" data-theme-label>System</span></button>\n\
         <button class=\"button\" type=\"button\" data-embed-copy data-js-only hidden><span aria-hidden=\"true\">&lt;/&gt;</span><span class=\"button-label\">Embed</span></button>\n\
         <button class=\"button primary\" type=\"button\" data-share data-js-only hidden><span aria-hidden=\"true\">↗</span><span class=\"button-label\">Share</span></button>\n\
         </div></nav>\n<header class=\"publication-header\">\n\
         <p class=\"eyebrow\">Immutable engineering publication</p>\n<h1>{title}</h1>\n"
    );
    if !snapshot.metadata.description.trim().is_empty() {
        let _ = writeln!(
            html,
            "<p class=\"description\">{}</p>",
            escape_html(snapshot.metadata.description.trim())
        );
    }
    let _ = write!(
        html,
        "<p class=\"byline\"><span>Published by {author}</span><span>{created}</span><span>{license}</span></p>\n\
         </header>\n<section class=\"summary-grid\" aria-label=\"Publication summary\">\n\
         <div class=\"summary-card\"><span class=\"summary-label\">Schematic</span><strong class=\"summary-value\">{}</strong><span class=\"summary-detail\">published sheet{}</span></div>\n\
         <div class=\"summary-card\"><span class=\"summary-label\">Analyses</span><strong class=\"summary-value\">{analysis_count}</strong><span class=\"summary-detail\">simulation configuration{}</span></div>\n\
         <div class=\"summary-card\"><span class=\"summary-label\">Measurements</span><strong class=\"summary-value\">{measurement_count}</strong><span class=\"summary-detail\">published result{}</span></div>\n\
         <div class=\"summary-card\"><span class=\"summary-label\">Data</span><strong class=\"summary-value\">{dataset_count}</strong><span class=\"summary-detail\">downloadable dataset{}</span></div>\n\
         </section>\n",
        schematic_count,
        if schematic_count == 1 { "" } else { "s" },
        if analysis_count == 1 { "" } else { "s" },
        if measurement_count == 1 { "" } else { "s" },
        if dataset_count == 1 { "" } else { "s" },
    );

    html.push_str("<div class=\"tabbar-wrap\"><nav class=\"tabbar\" aria-label=\"Circuit publication sections\">\n");
    for section in section_order {
        match section {
            PublicationSection::Overview => render_tab(&mut html, "overview", "Overview", None),
            PublicationSection::Schematic => {
                render_tab(&mut html, "schematic", "Schematic", Some(schematic_count));
            }
            PublicationSection::Results => {
                render_tab(&mut html, "results", "Results", Some(plot_figures.len()));
            }
            PublicationSection::Components => {
                render_tab(&mut html, "components", "Components", Some(component_count));
            }
            PublicationSection::Files => {
                render_tab(&mut html, "files", "Files", Some(assets.len()));
            }
            PublicationSection::Details => render_tab(&mut html, "details", "Details", None),
        }
    }
    let _ = write!(
        html,
        "</nav></div>\n<noscript><p class=\"noscript-note\">All published content is available below. Interactive views, tabs, sharing, and theme controls require JavaScript.</p></noscript>\n<main class=\"content\" id=\"publication-content\" data-default-panel=\"{}\">\n",
        section_id(default_section)
    );

    html.push_str("<section class=\"panel\" id=\"overview\" data-panel tabindex=\"-1\">\n");
    render_panel_header(
        &mut html,
        "Circuit overview",
        "The published design, simulation evidence, and disclosure summary.",
    );
    html.push_str("<div class=\"overview-grid\">\n<div class=\"surface overview-copy\"><h3>About this circuit</h3>");
    if let Some(overview) = authored_overview {
        let _ = write!(html, "<p>{}</p>", escape_html(overview.narrative.trim()));
        if !overview.specifications.is_empty() {
            html.push_str("<dl class=\"spec-grid\">\n");
            for specification in &overview.specifications {
                let unit = specification.unit.as_deref().unwrap_or_default();
                let _ = writeln!(
                    html,
                    "<div><dt>{}</dt><dd>{}<span>{}</span></dd></div>",
                    escape_html(&specification.label),
                    escape_html(&specification.value),
                    escape_html(unit),
                );
            }
            html.push_str("</dl>\n");
        }
    } else if snapshot.metadata.description.trim().is_empty() {
        html.push_str("<p>No additional design description was supplied by the publisher.</p>");
    } else {
        let _ = write!(
            html,
            "<p>{}</p>",
            escape_html(snapshot.metadata.description.trim())
        );
    }
    let _ = write!(
        html,
        "</div><aside class=\"surface side-card\"><h3>Publication facts</h3><dl class=\"facts\">\
         <div><dt>Publisher</dt><dd>{author}</dd></div>\
         <div><dt>Published</dt><dd>{created}</dd></div>\
         <div><dt>License</dt><dd>{license}</dd></div>\
         <div><dt>RSpice version</dt><dd>{}</dd></div>\
         </dl></aside></div>\n</section>\n",
        escape_html(&snapshot.metadata.app_version),
    );

    if snapshot.schematic.is_some() {
        html.push_str("<section class=\"panel\" id=\"schematic\" data-panel tabindex=\"-1\">\n");
        render_panel_header(
            &mut html,
            "Schematic",
            "Static by default and interactive on demand. The drawing is sealed with this publication.",
        );
        html.push_str(
            "<div class=\"schematic-tools surface\" data-js-only hidden>\
             <label class=\"search-field\"><span>Find a component or net</span><input type=\"search\" inputmode=\"search\" placeholder=\"R1, VOUT, ground…\" data-schematic-search></label>\
             <p class=\"schematic-status\" data-schematic-status role=\"status\" aria-live=\"polite\">Select a tagged component or net to inspect it.</p>\
             </div>\n",
        );
        html.push_str("<div class=\"figure-stack\">\n");
        for figure in &schematic_figures {
            render_figure(&mut html, snapshot, figure);
        }
        if schematic_figures.is_empty() {
            html.push_str("<div class=\"empty-state\"><h3>No schematic figure was selected</h3><p>The snapshot contains disclosed sheets, but the publisher did not add one to the page figure set.</p></div>\n");
        }
        html.push_str("</div></section>\n");
    }

    if !plot_figures.is_empty() || snapshot.results.is_some() {
        html.push_str("<section class=\"panel\" id=\"results\" data-panel tabindex=\"-1\">\n");
        render_panel_header(
            &mut html,
            "Simulation results",
            "Published plots, measurements, and analysis controls from the sealed simulation snapshot.",
        );
        html.push_str("<div class=\"section-stack\">\n");
        render_measurements(&mut html, snapshot);
        if !plot_figures.is_empty() {
            html.push_str("<section class=\"subsection\" aria-labelledby=\"plots-heading\"><h3 id=\"plots-heading\">Plots</h3><div class=\"figure-stack\">\n");
            for figure in plot_figures {
                render_figure(&mut html, snapshot, figure);
            }
            html.push_str("</div></section>\n");
        }
        if measurement_count == 0
            && snapshot
                .figures
                .iter()
                .all(|figure| !matches!(figure.content, FigureContent::Plot(_)))
        {
            html.push_str("<div class=\"empty-state\"><h3>No plotted results were included</h3><p>The publisher disclosed result metadata without adding a result figure or scalar measurement.</p></div>\n");
        }
        html.push_str("</div></section>\n");
    }

    if let Some(engineering) = &snapshot.engineering
        && !engineering.components.is_empty()
    {
        html.push_str("<section class=\"panel\" id=\"components\" data-panel tabindex=\"-1\">\n");
        render_panel_header(
            &mut html,
            "Components",
            "Published component identity, model labels, pins, and connected nets.",
        );
        html.push_str("<div class=\"table-wrap\"><table><caption>Components disclosed in this publication</caption><thead><tr><th scope=\"col\">Reference</th><th scope=\"col\">Value</th><th scope=\"col\">Device</th><th scope=\"col\">Model</th><th scope=\"col\">Pins and nets</th></tr></thead><tbody>\n");
        for component in &engineering.components {
            let model = component
                .model
                .as_ref()
                .map_or("—", |model| model.name.as_str());
            let pins = component
                .pins
                .iter()
                .map(|pin| match &pin.net {
                    Some(net) => format!("{} → {net}", pin.name),
                    None => pin.name.clone(),
                })
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(
                html,
                "<tr id=\"component-{}\"><td class=\"num\"><strong>{}</strong></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                escape_html(&component.reference),
                escape_html(&component.reference),
                escape_html(&component.value),
                escape_html(&component.device),
                escape_html(model),
                escape_html(&pins),
            );
        }
        html.push_str("</tbody></table></div></section>\n");
    }

    if !assets.is_empty() {
        html.push_str("<section class=\"panel\" id=\"files\" data-panel tabindex=\"-1\">\n");
        render_panel_header(
            &mut html,
            "Published files",
            "Download only the engineering artifacts explicitly disclosed with this publication.",
        );
        html.push_str("<ul class=\"asset-list\">\n");
        for asset in &assets {
            let _ = writeln!(
                html,
                "<li class=\"asset-item\"><div class=\"asset-main\"><strong>{}</strong><span>{} · {}</span></div><a class=\"button asset-download\" href=\"{}\" download>Download</a></li>",
                escape_html(&asset.label),
                escape_html(&asset.detail),
                format_bytes(asset.bytes),
                escape_html(&asset.path),
            );
        }
        html.push_str("</ul></section>\n");
    }

    html.push_str("<section class=\"panel\" id=\"details\" data-panel tabindex=\"-1\">\n");
    render_panel_header(
        &mut html,
        "Engineering details",
        "Analysis controls, disclosed source, and immutable publication provenance.",
    );
    html.push_str("<div class=\"section-stack\">\n");
    render_analyses(&mut html, snapshot);
    if let Some(simulation) = snapshot
        .engineering
        .as_ref()
        .and_then(|engineering| engineering.simulation.as_ref())
    {
        let temperature = simulation.temperature_c_bits.map_or_else(
            || "—".to_string(),
            |bits| format!("{} °C", f64::from_bits(bits)),
        );
        let corner = simulation.corner.as_deref().unwrap_or("—");
        let _ = write!(
            html,
            "<section class=\"subsection\" aria-labelledby=\"provenance-heading\"><h3 id=\"provenance-heading\">Simulation provenance</h3><div class=\"surface side-card\"><dl class=\"facts\"><div><dt>Engine</dt><dd>{} {}</dd></div><div><dt>Temperature</dt><dd>{}</dd></div><div><dt>Corner</dt><dd>{}</dd></div>",
            escape_html(&simulation.engine),
            escape_html(&simulation.engine_version),
            escape_html(&temperature),
            escape_html(corner),
        );
        for setting in &simulation.settings {
            let _ = write!(
                html,
                "<div><dt>{}</dt><dd>{}</dd></div>",
                escape_html(&setting.name),
                escape_html(&setting.value),
            );
        }
        html.push_str("</dl></div>");
        if !simulation.warnings.is_empty() {
            html.push_str("<ul class=\"warning-list\">");
            for warning in &simulation.warnings {
                let severity = match warning.severity {
                    WarningSeverity::Information => "Information",
                    WarningSeverity::Warning => "Warning",
                    WarningSeverity::Error => "Error",
                };
                let _ = write!(
                    html,
                    "<li><strong>{severity}</strong><span>{}</span></li>",
                    escape_html(&warning.message)
                );
            }
            html.push_str("</ul>");
        }
        html.push_str("</section>\n");
    }
    if let Some(netlist) = &snapshot.netlist {
        let _ = writeln!(
            html,
            "<section class=\"subsection\" aria-labelledby=\"netlist-heading\"><h3 id=\"netlist-heading\">Netlist</h3><details class=\"deck\"><summary>View the published SPICE deck</summary><pre class=\"deck\">{}</pre></details></section>",
            escape_html(&netlist.deck)
        );
    }
    if snapshot.netlist.is_none() && analysis_count == 0 {
        html.push_str("<div class=\"empty-state\"><h3>No additional engineering details were disclosed</h3><p>The publication still carries immutable authorship and renderer provenance below.</p></div>\n");
    }
    html.push_str(
        "<section class=\"subsection cloud-context\" data-cloud-context hidden aria-labelledby=\"cloud-context-heading\">\n\
         <h3 id=\"cloud-context-heading\">Publication history and original artifacts</h3>\n\
         <div class=\"cloud-context-grid\">\n\
         <div class=\"surface side-card\"><span class=\"summary-label\">Version</span><strong class=\"cloud-version\" data-cloud-version></strong><nav class=\"version-actions\" data-version-actions aria-label=\"Publication versions\"></nav></div>\n\
         <div class=\"surface side-card\" data-cloud-artifacts-wrap hidden><span class=\"summary-label\">Original cloud artifacts</span><ul class=\"cloud-artifact-list\" data-cloud-artifacts></ul></div>\n\
         </div></section>\n",
    );
    html.push_str("</div></section>\n</main>\n");

    if let Some(hydration) = hydration {
        let _ = write!(
            html,
            "<script type=\"application/json\" id=\"rspice-hydration\">{}</script>\n\
             <script type=\"module\" src=\"{LOADER_PATH}\" integrity=\"{}\"></script>\n",
            hydration.island_json, hydration.loader_integrity,
        );
    }

    let _ = write!(
        html,
        "<footer class=\"publication-footer\"><div>Published with RSpice {} · rendered by rspice-publish {}</div><div>Snapshot <code>sha256:{}</code></div></footer>\n\
         <div class=\"toast\" role=\"status\" aria-live=\"polite\" data-toast hidden></div>\n\
         </div>\n</body>\n</html>\n",
        escape_html(&snapshot.metadata.app_version),
        env!("CARGO_PKG_VERSION"),
        escape_html(snapshot_sha256_hex),
    );
    html
}
