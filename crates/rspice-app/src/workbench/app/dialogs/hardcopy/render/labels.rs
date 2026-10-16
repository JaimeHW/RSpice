//! How the hardcopy studio names its settings: the label each value is shown
//! by, the key a choice list stores it under, and the options a list offers.

use super::*;

pub(super) fn document_kind_label(kind: HardcopyDocumentKind) -> &'static str {
    match kind {
        HardcopyDocumentKind::SchematicOrSymbol => "Schematic / symbol",
        HardcopyDocumentKind::LayoutWithLayerLegend => "Layout + layer legend",
        HardcopyDocumentKind::PlotOrWorksheet => "Plot / worksheet",
        HardcopyDocumentKind::Report => "Report",
        HardcopyDocumentKind::EngineeringDocument => "Engineering document",
    }
}

pub(super) fn scope_label(scope: &HardcopyScope) -> &'static str {
    match scope {
        HardcopyScope::Selection => "Selection",
        HardcopyScope::CurrentSheet => "Current sheet",
        HardcopyScope::VisibleHierarchy => "Visible hierarchy",
        HardcopyScope::ActivePlotDocument => "Active plot document",
        HardcopyScope::CompleteReport => "Complete report",
        HardcopyScope::ActiveDocument => "Active document",
        HardcopyScope::AllSheetsOrPanes => "All sheets / panes",
        HardcopyScope::NamedPrintSet(_) => "Named print set",
    }
}

pub(super) fn detailed_scope_label(scope: &HardcopyScope) -> &str {
    match scope {
        HardcopyScope::Selection => "Selection",
        HardcopyScope::CurrentSheet => "Current sheet",
        HardcopyScope::VisibleHierarchy => "Visible hierarchy",
        HardcopyScope::ActivePlotDocument => "Active plot document",
        HardcopyScope::CompleteReport => "Complete report",
        HardcopyScope::ActiveDocument => "Active document",
        HardcopyScope::AllSheetsOrPanes => "All sheets / panes",
        HardcopyScope::NamedPrintSet(name) => name,
    }
}

pub(super) fn paper_label(paper: &PaperDraft) -> String {
    match paper {
        PaperDraft::Standard(paper) => standard_paper_label(*paper).to_owned(),
        PaperDraft::MatchAuthoredSheets => "Match each authored sheet".to_owned(),
        PaperDraft::Custom { name, .. } => format!("Custom · {name}"),
    }
}

pub(super) fn scale_key(value: ScaleMode) -> u8 {
    match value {
        ScaleMode::FitPrintableArea => 0,
        ScaleMode::EngineeringOneToOne => 1,
        ScaleMode::CustomPercent { .. } => 2,
        ScaleMode::FitWidth => 3,
    }
}

pub(super) fn scale_label(value: u8) -> &'static str {
    match value {
        0 => "Fit to printable area",
        1 => "1:1 engineering scale",
        2 => "Custom",
        _ => "Fit width",
    }
}

pub(super) fn tiling_key(value: TilingMode) -> u8 {
    match value {
        TilingMode::Automatic => 0,
        TilingMode::SinglePage => 1,
        TilingMode::Manual { .. } => 2,
    }
}

pub(super) fn format_label(format: OutputFormat) -> &'static str {
    match format {
        OutputFormat::NativePrinter => "System printer",
        OutputFormat::BrowserPrintDocument => {
            if cfg!(target_arch = "wasm32") {
                "Browser print dialog"
            } else {
                "Print-ready document handoff"
            }
        }
        OutputFormat::PdfVector => "PDF · vector",
        OutputFormat::PdfA => "PDF/A · vector",
        OutputFormat::SvgVector => "SVG · vector",
        // The resolution is its own field, so the format no longer spells one
        // out. It used to read "PNG · 600 dpi" — a number nobody could change,
        // and on a browser-sized raster budget one nothing could deliver.
        OutputFormat::Png { .. } => "PNG · raster",
        OutputFormat::Tiff { .. } => "TIFF · raster",
    }
}

pub(super) fn background_label(background: BackgroundMode) -> &'static str {
    match background {
        BackgroundMode::White => "White",
        BackgroundMode::Transparent => "Transparent · vector export",
        BackgroundMode::WorkspaceBackground => "Workspace background",
    }
}

pub(super) fn color_mapping_label(mapping: ColorMapping) -> &'static str {
    match mapping {
        ColorMapping::PrintSafeEngineeringPalette => "Print-safe engineering palette",
        ColorMapping::ScreenColors => "Screen colors",
        ColorMapping::GrayscaleWithDashMarkerRedundancy => "Grayscale with dash/marker redundancy",
        ColorMapping::Monochrome => "Monochrome",
    }
}

pub(super) fn print_color_label(value: PrintColor) -> String {
    match value {
        PrintColor::Black => "Black".to_owned(),
        PrintColor::GrayPercent(percent) => format!("{percent}% black"),
        PrintColor::Rgb { red, green, blue } => {
            if let Some(name) = named_print_color(value) {
                format!("{name} · #{red:02X}{green:02X}{blue:02X}")
            } else {
                format!("RGB · #{red:02X}{green:02X}{blue:02X}")
            }
        }
    }
}

pub(super) fn print_color_options(current: PrintColor) -> Vec<(PrintColor, String)> {
    let mut values = vec![
        PrintColor::Black,
        PrintColor::GrayPercent(40),
        PrintColor::GrayPercent(60),
        PrintColor::GrayPercent(70),
        PrintColor::Rgb {
            red: 0,
            green: 174,
            blue: 199,
        },
        PrintColor::Rgb {
            red: 0,
            green: 128,
            blue: 72,
        },
        PrintColor::Rgb {
            red: 232,
            green: 185,
            blue: 35,
        },
        PrintColor::Rgb {
            red: 210,
            green: 56,
            blue: 56,
        },
        PrintColor::Rgb {
            red: 226,
            green: 126,
            blue: 34,
        },
        PrintColor::Rgb {
            red: 0,
            green: 95,
            blue: 180,
        },
    ];
    if !values.contains(&current) {
        values.insert(0, current);
    }
    values
        .into_iter()
        .map(|value| (value, print_color_label(value)))
        .collect()
}

fn named_print_color(value: PrintColor) -> Option<&'static str> {
    match value {
        PrintColor::Rgb {
            red: 0,
            green: 174,
            blue: 199,
        } => Some("Cyan"),
        PrintColor::Rgb {
            red: 0,
            green: 128,
            blue: 72,
        } => Some("Green"),
        PrintColor::Rgb {
            red: 232,
            green: 185,
            blue: 35,
        } => Some("Yellow"),
        PrintColor::Rgb {
            red: 210,
            green: 56,
            blue: 56,
        } => Some("Red"),
        PrintColor::Rgb {
            red: 226,
            green: 126,
            blue: 34,
        } => Some("Orange"),
        PrintColor::Rgb {
            red: 0,
            green: 95,
            blue: 180,
        } => Some("Engineering blue"),
        _ => None,
    }
}

pub(super) fn redundancy_options(current: PrintRedundancy) -> Vec<PrintRedundancy> {
    let thin = Length::from_micrometres(200);
    let mut values = vec![
        PrintRedundancy::SourceStyle,
        PrintRedundancy::SolidLine {
            width: Length::from_micrometres(350),
        },
        PrintRedundancy::DashedLine {
            width: Length::from_micrometres(300),
            dash: Length::from_micrometres(2_000),
            gap: Length::from_micrometres(1_000),
        },
        PrintRedundancy::DottedLeader {
            width: thin,
            spacing: Length::from_micrometres(1_250),
        },
        PrintRedundancy::SolidFill,
        PrintRedundancy::CrossHatch {
            line_width: thin,
            spacing: Length::from_micrometres(1_500),
        },
        PrintRedundancy::TriangleWithId {
            size: Length::from_micrometres(3_000),
        },
    ];
    if !values.contains(&current) {
        values.insert(0, current);
    }
    values
}

pub(super) fn redundancy_label(value: PrintRedundancy) -> String {
    match value {
        PrintRedundancy::SourceStyle => "Source style".to_owned(),
        PrintRedundancy::SolidLine { width } => {
            format!("Solid · {} mm", format_millimetres(width))
        }
        PrintRedundancy::DashedLine { width, dash, gap } => format!(
            "Dash · {} mm · {}/{} mm",
            format_millimetres(width),
            format_millimetres(dash),
            format_millimetres(gap)
        ),
        PrintRedundancy::DottedLeader { width, spacing } => format!(
            "Dotted leader · {} mm · {} mm spacing",
            format_millimetres(width),
            format_millimetres(spacing)
        ),
        PrintRedundancy::SolidFill => "Solid fill".to_owned(),
        PrintRedundancy::CrossHatch {
            line_width,
            spacing,
        } => format!(
            "Cross hatch · {}/{} mm",
            format_millimetres(line_width),
            format_millimetres(spacing)
        ),
        PrintRedundancy::TriangleWithId { size } => {
            format!("Triangle + ID · {} mm", format_millimetres(size))
        }
    }
}

fn format_millimetres(value: Length) -> String {
    let micrometres = value.micrometres();
    let mut value = format!("{}.{:03}", micrometres / 1_000, micrometres % 1_000);
    if value.ends_with('0') {
        value.pop();
    }
    value
}

#[cfg(target_os = "windows")]
pub(super) fn duplex_label(value: DuplexMode) -> &'static str {
    match value {
        DuplexMode::Off => "Off",
        DuplexMode::LongEdge => "Long edge",
        DuplexMode::ShortEdge => "Short edge",
    }
}

#[cfg(target_os = "windows")]
pub(super) fn media_label(value: &PrinterMediaSource) -> String {
    match value {
        PrinterMediaSource::AutomaticCompatibleTray => "Automatic compatible tray".to_owned(),
        PrinterMediaSource::NamedTray(name) => name.clone(),
        PrinterMediaSource::ManualFeed => "Manual feed".to_owned(),
        PrinterMediaSource::Roll { width } => {
            format!("Roll · {} μm", width.micrometres())
        }
    }
}
