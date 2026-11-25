//! Print mappings derived from frozen semantic content.

use super::*;
use rspice_design_model::design_management::{
    DrawingSheetBorderTemplate, DrawingSheetTitleBlockTemplate,
};

/// The light screen tokens the SVG export draws a sheet in: wires, buses
/// and taps, and symbols.
pub(crate) const WIRE_PRINT_INK: [u8; 3] = [0x4E, 0x34, 0x00];
pub(crate) const SYMBOL_PRINT_INK: [u8; 3] = [0x2C, 0x31, 0x35];

/// The sheet's eight trace colours as it draws them on a light ground, in
/// the order it gives them out: a printed trace is the colour it is on the
/// sheet, solid, as ADS and ViVA print. The kit's light palette pins the
/// same eight. A page printed in grey or black draws each series in its
/// own line instead.
pub(crate) const TRACE_PRINT_INKS: [[u8; 3]; 8] = [
    [30, 105, 190],
    [190, 30, 80],
    [0, 128, 60],
    [110, 70, 190],
    [0, 122, 128],
    [160, 50, 150],
    [195, 80, 0],
    [10, 60, 150],
];

/// The ink the `index`th series of a sheet prints in.
pub(crate) fn trace_print_ink(index: usize) -> PrintColor {
    let [red, green, blue] = TRACE_PRINT_INKS[index % TRACE_PRINT_INKS.len()];
    PrintColor::Rgb { red, green, blue }
}

pub(super) fn default_print_mapping(
    document: &HardcopySemanticDocument,
) -> Result<PrintMappingTable, HardcopySourceError> {
    let mut entries = Vec::new();
    match document {
        HardcopySemanticDocument::Schematic(schematic) => {
            if let Some(format) = &schematic.drawing_sheet {
                entries.push(layer_mapping(
                    "layer:drawing-sheet-paper",
                    "Drawing sheet paper",
                    "authored paper edge",
                )?);
                if format.border != DrawingSheetBorderTemplate::None {
                    entries.push(form_layer_mapping(
                        "layer:drawing-sheet-frame",
                        "Drawing sheet frame",
                        "authored border, zones, and marks",
                    )?);
                }
                if format.title_block.template != DrawingSheetTitleBlockTemplate::None {
                    entries.push(form_layer_mapping(
                        "layer:drawing-sheet-title-block",
                        "Drawing sheet title block",
                        "authored title block fields",
                    )?);
                }
                entries.push(layer_mapping(
                    "layer:schematic-grid",
                    "Schematic grid",
                    "authored snap-grid pitch · output inclusion optional",
                )?);
            }
            // The electrical layers print in the SVG export's inks (D1
            // section 11), so a printed sheet and an exported one agree.
            let schematic_layer = |stable_id: &str,
                                   display_name: &str,
                                   screen_style: &str,
                                   [red, green, blue]: [u8; 3]| {
                mapping_entry(
                    PrintObjectKind::Layer,
                    stable_id,
                    display_name,
                    screen_style,
                    PrintColor::Rgb { red, green, blue },
                    PrintRedundancy::SourceStyle,
                    true,
                )
            };
            if !schematic.components.is_empty() {
                entries.push(schematic_layer(
                    "layer:schematic-components",
                    "Components and symbols",
                    "schematic component color · solid",
                    SYMBOL_PRINT_INK,
                )?);
            }
            if !schematic.wires.is_empty()
                || !schematic.junctions.is_empty()
                || !schematic.net_labels.is_empty()
            {
                entries.push(schematic_layer(
                    "layer:schematic-wiring",
                    "Wires and junctions",
                    "schematic wire color · solid",
                    WIRE_PRINT_INK,
                )?);
            }
            if !schematic.buses.is_empty() || !schematic.bus_taps.is_empty() {
                entries.push(schematic_layer(
                    "layer:schematic-buses",
                    "Buses and taps",
                    "schematic bus color · heavy solid",
                    WIRE_PRINT_INK,
                )?);
            }
            if !schematic.design_notes.is_empty() {
                entries.push(layer_mapping(
                    "layer:drawing-annotation",
                    "Drawing annotations",
                    "drawing / annotation text",
                )?);
            }
            if !schematic.documentation_shapes.is_empty() {
                entries.push(layer_mapping(
                    "layer:drawing-documentation",
                    "Documentation geometry",
                    "drawing / documentation stroke",
                )?);
            }

            let mut nets = std::collections::BTreeMap::new();
            for label in &schematic.net_labels {
                nets.entry(label.name.as_str())
                    .and_modify(|id: &mut u64| *id = (*id).min(label.id))
                    .or_insert(label.id);
            }
            for (net, stable_id) in nets {
                entries.push(mapping_entry(
                    PrintObjectKind::Net,
                    format!("net:{stable_id}"),
                    net,
                    "schematic wire color · solid",
                    PrintColor::Black,
                    PrintRedundancy::SolidLine {
                        width: Length::from_micrometres(250),
                    },
                    true,
                )?);
            }
            let mut notes = schematic.design_notes.iter().collect::<Vec<_>>();
            notes.sort_by_key(|note| note.id);
            for note in notes {
                entries.push(mapping_entry(
                    PrintObjectKind::ReviewAnnotation,
                    format!("schematic-note:{}", note.id),
                    format!("{} {}", note.kind.label(), note.id),
                    note.layer.label(),
                    PrintColor::Black,
                    PrintRedundancy::DottedLeader {
                        width: Length::from_micrometres(200),
                        spacing: Length::from_micrometres(1_250),
                    },
                    false,
                )?);
            }
        }
        HardcopySemanticDocument::Symbol(symbol) => {
            if !symbol.body.is_empty() {
                entries.push(layer_mapping(
                    "layer:symbol-body",
                    "Symbol body",
                    "symbol graphic color · solid",
                )?);
            }
            if !symbol.pins.is_empty() {
                entries.push(layer_mapping(
                    "layer:symbol-pins",
                    "Symbol pins",
                    "terminal color · solid",
                )?);
            }
        }
        HardcopySemanticDocument::Plot(plot) => {
            const SCREEN_STYLES: [&str; 8] = [
                "blue · solid",
                "crimson · solid",
                "green · solid",
                "violet · solid",
                "teal · solid",
                "magenta · solid",
                "orange · solid",
                "navy · solid",
            ];
            for (trace, slot) in plot.traces.iter().zip(plot.trace_slots()) {
                let slot = usize::from(slot);
                // A companion branch closes the series before it: the same
                // ink, thin and dashed, as the sheet draws it.
                let (redundancy, screen_style) = if trace.companion {
                    (
                        PrintRedundancy::DashedLine {
                            width: Length::from_micrometres(200),
                            dash: Length::from_micrometres(1_500),
                            gap: Length::from_micrometres(1_000),
                        },
                        "series color · thin dashed",
                    )
                } else {
                    (
                        PrintRedundancy::SolidLine {
                            width: Length::from_micrometres(300),
                        },
                        SCREEN_STYLES[slot % SCREEN_STYLES.len()],
                    )
                };
                entries.push(mapping_entry(
                    PrintObjectKind::Trace,
                    format!("trace:{}", trace.trace_id),
                    trace.label.clone(),
                    screen_style,
                    trace_print_ink(slot),
                    redundancy,
                    true,
                )?);
            }
            for cursor in &plot.cursors {
                entries.push(mapping_entry(
                    PrintObjectKind::Marker,
                    format!("cursor:{}", cursor.cursor_id),
                    format!("Cursor {}", cursor.label),
                    "viewer cursor color · dashed line",
                    PrintColor::Black,
                    PrintRedundancy::DashedLine {
                        width: Length::from_micrometres(200),
                        dash: Length::from_micrometres(1_250),
                        gap: Length::from_micrometres(750),
                    },
                    true,
                )?);
            }
            for marker in &plot.markers {
                entries.push(mapping_entry(
                    PrintObjectKind::Marker,
                    format!("marker:{}", marker.marker_id),
                    marker.label.clone(),
                    "viewer marker color · triangle",
                    PrintColor::Black,
                    PrintRedundancy::TriangleWithId {
                        size: Length::from_micrometres(2_500),
                    },
                    true,
                )?);
            }
            for annotation in &plot.annotations {
                entries.push(mapping_entry(
                    PrintObjectKind::ReviewAnnotation,
                    format!("annotation:{}", annotation.annotation_id),
                    compact_display(&annotation.text, "Plot annotation"),
                    "viewer annotation color · leader",
                    PrintColor::Black,
                    PrintRedundancy::DottedLeader {
                        width: Length::from_micrometres(200),
                        spacing: Length::from_micrometres(1_250),
                    },
                    false,
                )?);
            }
        }
        HardcopySemanticDocument::ResultSummary(summary) => {
            entries.push(layer_mapping(
                format!(
                    "layer:result-summary:{}",
                    summary.viewer.label().to_ascii_lowercase()
                ),
                format!("{} result summary", summary.viewer.label()),
                "result table and semantic diagram styles",
            )?);
        }
        HardcopySemanticDocument::Report(report) => {
            entries.push(layer_mapping(
                "layer:report-content",
                "Report content",
                "report template styles",
            )?);
            for page in &report.pages {
                for section in page.sections() {
                    for block in section.blocks() {
                        if let ReportBlockKind::ReviewNote(note) = block.kind() {
                            entries.push(mapping_entry(
                                PrintObjectKind::ReviewAnnotation,
                                format!("report-review:{}", block.id()),
                                compact_display(&note.message, "Report review note"),
                                "report review note · leader",
                                PrintColor::Black,
                                PrintRedundancy::DottedLeader {
                                    width: Length::from_micrometres(200),
                                    spacing: Length::from_micrometres(1_250),
                                },
                                false,
                            )?);
                        }
                    }
                }
            }
        }
        HardcopySemanticDocument::Aggregate(aggregate) => {
            for child in &aggregate.children {
                let child_mapping = default_print_mapping(&child.document)?;
                for entry in child_mapping.entries() {
                    let object = entry.object();
                    let stable_digest = Sha256::digest(
                        [
                            b"rspice-aggregate-print-object-v1:".as_slice(),
                            &child.ordinal.to_be_bytes(),
                            object.stable_id().as_bytes(),
                        ]
                        .concat(),
                    );
                    let stable_suffix = stable_digest[..12]
                        .iter()
                        .map(|byte| format!("{byte:02x}"))
                        .collect::<String>();
                    entries.push(mapping_entry(
                        object.kind(),
                        format!("aggregate:{}:{stable_suffix}", child.ordinal),
                        compact_display(
                            &format!("{} · {}", child.display_name, object.display_name()),
                            "Aggregate object",
                        ),
                        object.screen_style(),
                        entry.print_color(),
                        entry.redundancy(),
                        entry.include_in_legend(),
                    )?);
                }
            }
        }
    }
    PrintMappingTable::try_new(PrintMappingSaveScope::Document, entries)
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))
}
