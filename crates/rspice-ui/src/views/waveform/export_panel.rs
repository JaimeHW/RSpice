//! Waveform Export Panel
//!
//! UI component for exporting waveform data and images.
//! Supports CSV data export and SVG image export.

use dioxus::prelude::*;

use crate::state::SimulationState;
use crate::theme::Theme;

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Csv,
    Svg,
    Png,
    Spice,
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::Csv => write!(f, "CSV Data"),
            ExportFormat::Svg => write!(f, "SVG Image"),
            ExportFormat::Png => write!(f, "PNG Image"),
            ExportFormat::Spice => write!(f, "SPICE PWL"),
        }
    }
}

/// Props for export panel
#[derive(Props, Clone, PartialEq)]
pub struct ExportPanelProps {
    /// Whether the panel is visible
    pub visible: bool,
    /// Callback to close the panel
    pub on_close: EventHandler<()>,
    /// Panel position (managed by parent for global drag)
    #[props(default = (400, 50))]
    pub position: (i32, i32),
    /// Callback when drag starts
    #[props(default)]
    pub on_drag_start: EventHandler<(i32, i32)>,
}

/// Export configuration and actions panel
#[component]
pub fn ExportPanel(props: ExportPanelProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let sim_state: Signal<SimulationState> = use_context();
    let th = theme.read();

    let mut format = use_signal(|| ExportFormat::Csv);
    let mut include_header = use_signal(|| true);
    let mut selected_traces = use_signal(Vec::<String>::new);

    if !props.visible {
        return rsx! {};
    }

    let (x, y) = props.position;
    let waveforms = &sim_state.read().waveforms;

    rsx! {
        div {
            class: "export-panel",
            style: "
                position: absolute;
                left: {x}px;
                top: {y}px;
                width: 300px;
                background: {th.bg_secondary()};
                border: 1px solid {th.border()};
                border-radius: {Theme::RADIUS_MD};
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
                z-index: 100;
                overflow: hidden;
            ",

            // Draggable Header
            div {
                style: "
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: {Theme::SPACING_SM} {Theme::SPACING_MD};
                    border-bottom: 1px solid {th.border()};
                    background: {th.surface()};
                    cursor: move;
                    user-select: none;
                ",
                onmousedown: {
                    let position = props.position;
                    move |e| {
                        let offset_x = e.client_coordinates().x as i32 - position.0;
                        let offset_y = e.client_coordinates().y as i32 - position.1;
                        props.on_drag_start.call((offset_x, offset_y));
                    }
                },

                span {
                    style: "
                        font-weight: 600;
                        font-size: {Theme::FONT_SIZE_SM};
                        color: {th.text_primary()};
                    ",
                    "Export"
                }

                button {
                    style: "
                        background: none;
                        border: none;
                        color: {th.text_muted()};
                        cursor: pointer;
                        padding: 4px;
                        font-size: 14px;
                    ",
                    onmousedown: move |e| e.stop_propagation(),
                    onclick: move |_| props.on_close.call(()),
                    "✕"
                }
            }

            // Content
            div {
                style: "padding: {Theme::SPACING_MD};",

                // Format selector
                div {
                    style: "margin-bottom: {Theme::SPACING_MD};",

                    label {
                        style: "
                            display: block;
                            font-size: 11px;
                            color: {th.text_secondary()};
                            margin-bottom: 4px;
                        ",
                        "Format"
                    }

                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",

                        FormatButton {
                            format: ExportFormat::Csv,
                            active: *format.read() == ExportFormat::Csv,
                            onclick: move |_| format.set(ExportFormat::Csv),
                        }
                        FormatButton {
                            format: ExportFormat::Svg,
                            active: *format.read() == ExportFormat::Svg,
                            onclick: move |_| format.set(ExportFormat::Svg),
                        }
                        FormatButton {
                            format: ExportFormat::Png,
                            active: *format.read() == ExportFormat::Png,
                            onclick: move |_| format.set(ExportFormat::Png),
                        }
                        FormatButton {
                            format: ExportFormat::Spice,
                            active: *format.read() == ExportFormat::Spice,
                            onclick: move |_| format.set(ExportFormat::Spice),
                        }
                    }
                }

                // CSV options
                if *format.read() == ExportFormat::Csv {
                    div {
                        style: "margin-bottom: {Theme::SPACING_MD};",

                        label {
                            style: "
                                display: flex;
                                align-items: center;
                                gap: 8px;
                                font-size: 12px;
                                color: {th.text_secondary()};
                                cursor: pointer;
                            ",
                            input {
                                r#type: "checkbox",
                                checked: *include_header.read(),
                                onchange: move |e| include_header.set(e.checked()),
                            }
                            "Include header row"
                        }
                    }
                }

                // Trace selection
                div {
                    style: "margin-bottom: {Theme::SPACING_MD};",

                    label {
                        style: "
                            display: block;
                            font-size: 11px;
                            color: {th.text_secondary()};
                            margin-bottom: 8px;
                        ",
                        "Traces to Export"
                    }

                    div {
                        style: "
                            max-height: 150px;
                            overflow-y: auto;
                            background: {th.bg_primary()};
                            border-radius: {Theme::RADIUS_SM};
                            padding: {Theme::SPACING_XS};
                        ",

                        if waveforms.is_empty() {
                            div {
                                style: "
                                    padding: {Theme::SPACING_SM};
                                    color: {th.text_muted()};
                                    font-size: 11px;
                                    text-align: center;
                                ",
                                "No waveforms available"
                            }
                        } else {
                            for wf in waveforms.iter() {
                                TraceCheckbox {
                                    name: wf.name.clone(),
                                    color: wf.color.clone(),
                                    checked: selected_traces.read().contains(&wf.name),
                                    onchange: {
                                        let name = wf.name.clone();
                                        move |checked| {
                                            let mut traces = selected_traces.write();
                                            if checked {
                                                if !traces.contains(&name) {
                                                    traces.push(name.clone());
                                                }
                                            } else {
                                                traces.retain(|t| t != &name);
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }

                    // Select all / none buttons
                    div {
                        style: "
                            display: flex;
                            gap: {Theme::SPACING_SM};
                            margin-top: {Theme::SPACING_XS};
                        ",

                        button {
                            style: "
                                flex: 1;
                                padding: 4px;
                                background: none;
                                border: 1px solid {th.border()};
                                border-radius: {Theme::RADIUS_SM};
                                color: {th.text_secondary()};
                                font-size: 10px;
                                cursor: pointer;
                            ",
                            onclick: {
                                let names: Vec<String> = waveforms.iter().map(|w| w.name.clone()).collect();
                                move |_| {
                                    selected_traces.set(names.clone());
                                }
                            },
                            "Select All"
                        }

                        button {
                            style: "
                                flex: 1;
                                padding: 4px;
                                background: none;
                                border: 1px solid {th.border()};
                                border-radius: {Theme::RADIUS_SM};
                                color: {th.text_secondary()};
                                font-size: 10px;
                                cursor: pointer;
                            ",
                            onclick: move |_| selected_traces.set(vec![]),
                            "Select None"
                        }
                    }
                }

                // Export button
                button {
                    style: "
                        width: 100%;
                        padding: {Theme::SPACING_SM} {Theme::SPACING_MD};
                        background: {th.accent_primary()};
                        border: none;
                        border-radius: {Theme::RADIUS_SM};
                        color: white;
                        font-size: {Theme::FONT_SIZE_SM};
                        font-weight: 600;
                        cursor: pointer;
                    ",
                    disabled: selected_traces.read().is_empty() && !waveforms.is_empty(),
                    onclick: move |_| {
                        let fmt = *format.read();
                        let traces = selected_traces.read().clone();
                        let include_hdr = *include_header.read();

                        spawn(async move {
                            let waveforms = &sim_state.read().waveforms;

                            // Generate export content
                            let (content, default_name, filter_name, extension) = match fmt {
                                ExportFormat::Csv => {
                                    let csv = generate_csv(waveforms, &traces, include_hdr);
                                    (csv, "waveforms.csv", "CSV Files", "csv")
                                }
                                ExportFormat::Spice => {
                                    let pwl = generate_pwl(waveforms, &traces);
                                    (pwl, "waveforms.pwl", "SPICE PWL Files", "pwl")
                                }
                                ExportFormat::Svg => {
                                    // TODO: Implement SVG export
                                    (String::from("; SVG export not yet implemented"), "waveform.svg", "SVG Files", "svg")
                                }
                                ExportFormat::Png => {
                                    // TODO: Implement PNG export
                                    return;
                                }
                            };

                            if content.is_empty() {
                                return;
                            }

                            // Show save dialog
                            if let Some(file) = rfd::AsyncFileDialog::new()
                                .add_filter(filter_name, &[extension])
                                .set_file_name(default_name)
                                .save_file()
                                .await
                            {
                                match std::fs::write(file.path(), &content) {
                                    Ok(_) => {
                                        println!("Exported to: {}", file.path().display());
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to save export: {}", e);
                                    }
                                }
                            }
                        });
                    },
                    "💾 Export"
                }
            }
        }
    }
}

/// Format selection button
#[component]
fn FormatButton(format: ExportFormat, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let bg = if active {
        th.accent_primary()
    } else {
        th.surface()
    };
    let color = if active { "#ffffff" } else { th.text_primary() };

    let icon = match format {
        ExportFormat::Csv => "📊",
        ExportFormat::Svg => "🖼️",
        ExportFormat::Png => "📷",
        ExportFormat::Spice => "⚡",
    };

    rsx! {
        button {
            style: "
                padding: 8px;
                background: {bg};
                border: 1px solid {th.border()};
                border-radius: {Theme::RADIUS_SM};
                color: {color};
                font-size: 11px;
                cursor: pointer;
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 4px;
            ",
            onclick: move |e| onclick.call(e),

            span { style: "font-size: 16px;", "{icon}" }
            span { "{format}" }
        }
    }
}

/// Trace selection checkbox
#[component]
fn TraceCheckbox(
    name: String,
    color: String,
    checked: bool,
    onchange: EventHandler<bool>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        label {
            style: "
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 4px 6px;
                cursor: pointer;
                border-radius: {Theme::RADIUS_SM};
            ",

            input {
                r#type: "checkbox",
                checked: checked,
                onchange: move |e| onchange.call(e.checked()),
            }

            div {
                style: "
                    width: 12px;
                    height: 3px;
                    background: {color};
                    border-radius: 1px;
                "
            }

            span {
                style: "
                    font-size: 11px;
                    color: {th.text_primary()};
                ",
                "{name}"
            }
        }
    }
}

/// Generate CSV content from waveforms
fn generate_csv(
    waveforms: &[crate::state::WaveformData],
    selected: &[String],
    include_header: bool,
) -> String {
    let selected_wfs: Vec<_> = waveforms
        .iter()
        .filter(|w| selected.is_empty() || selected.contains(&w.name))
        .collect();

    if selected_wfs.is_empty() {
        return String::new();
    }

    let mut csv = String::new();

    // Header
    if include_header {
        csv.push_str("Time");
        for wf in &selected_wfs {
            csv.push(',');
            csv.push_str(&wf.name);
        }
        csv.push('\n');
    }

    // Data rows (assume all waveforms have same time base)
    if let Some(first) = selected_wfs.first() {
        for i in 0..first.x.len() {
            csv.push_str(&format!("{:.9e}", first.x[i]));
            for wf in &selected_wfs {
                csv.push(',');
                if i < wf.y.len() {
                    csv.push_str(&format!("{:.9e}", wf.y[i]));
                }
            }
            csv.push('\n');
        }
    }

    csv
}

/// Generate SPICE PWL format from waveform
fn generate_pwl(waveforms: &[crate::state::WaveformData], selected: &[String]) -> String {
    let selected_wfs: Vec<_> = waveforms
        .iter()
        .filter(|w| selected.is_empty() || selected.contains(&w.name))
        .collect();

    let mut output = String::new();

    for wf in selected_wfs {
        output.push_str(&format!("* PWL data for {}\n", wf.name));
        output.push_str("PWL(\n");
        for (i, (t, v)) in wf.x.iter().zip(wf.y.iter()).enumerate() {
            if i > 0 {
                output.push_str("\n+ ");
            }
            output.push_str(&format!("{:.9e} {:.9e}", t, v));
        }
        output.push_str("\n)\n\n");
    }

    output
}
