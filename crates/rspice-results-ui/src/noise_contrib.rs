//! Band-integrated noise contributors, conversion provenance and noise-figure presentation.

use egui::Ui;
use rspice_results::noise::NoiseSummary;
pub use rspice_results::visualization_document::PinnedContributor;
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::fmt_column_reading;
use rspice_ui_kit::widgets::measurement_table;
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

mod figure;
mod table;
pub(crate) use table::contributor_table;
pub use table::{ContributorLine, ContributorResponse};

/// Contributor evidence bound by the host to the displayed noise spectrum.
pub enum ContributorEvidence<'a> {
    /// The selected, qualified analysis retains a contributor summary.
    Retained {
        summary: &'a NoiseSummary,
        label: &'a str,
    },
    /// A spectrum is displayed, but its analysis retained no contributor summary.
    NotRetained,
    /// No usable noise analysis is bound to the spectrum.
    Unavailable,
}

/// Show contributors for the host-selected analysis and its validated
/// noise-figure evidence. The reader's `pinned` contributors lead the
/// table; returns whether the reader changed them and the device they asked
/// to see on the schematic.
pub fn right_panel(
    ui: &mut Ui,
    evidence: ContributorEvidence<'_>,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
    pinned: &mut Vec<PinnedContributor>,
) -> ContributorResponse {
    let frequency = |hertz| crate::presentation::frequency_readout(quantity_policy, hertz, digits);
    let summary = match evidence {
        ContributorEvidence::Retained { summary, .. } => summary,
        _ => {
            section_header(ui, "Contributors", None);
            crate::presentation::panel_note(
                ui,
                "No contributor data. Turn on contributor ranking in the noise analysis.",
            );
            return ContributorResponse::default();
        }
    };

    // The band and both integrated totals are the "Noise spectrum" section
    // directly above this one, and the contributor count is the length of the
    // table directly below it.
    if let Some(conversion) = &summary.conversion {
        // Jitter is noise divided by slew, and the reference signal says what
        // the phase was measured against; the sampled phase and voltage are
        // bookkeeping.
        if let Some(sampling) = &conversion.sampling {
            section_header(ui, "Sampling", None);
            let mut rows = vec![(
                "Output slew",
                fmt_column_reading(sampling.output.slew_volts_per_second, "V/s", digits),
            )];
            if let Some(reference) = &sampling.reference {
                rows.push((
                    "Reference signal",
                    format!(
                        "V({},{})",
                        reference.node,
                        reference.reference.as_deref().unwrap_or("0")
                    ),
                ));
            }
            measurement_table(
                ui,
                &rows
                    .iter()
                    .map(|(label, value)| (*label, value.as_str()))
                    .collect::<Vec<_>>(),
            );
        }
        section_header(ui, "Sidebands", None);
        let requested = !conversion.input_source.is_empty();
        let rows = [
            (
                "Input source",
                if requested {
                    conversion.input_source.clone()
                } else {
                    "Not requested".into()
                },
            ),
            ("Carrier fundamental", frequency(conversion.carrier_hz)),
            (
                "Sideband in → out",
                if requested {
                    format!(
                        "{} → {}",
                        conversion.input_sideband, conversion.output_sideband
                    )
                } else {
                    conversion.output_sideband.to_string()
                },
            ),
            (
                "Input frequency band",
                if requested {
                    format!(
                        "{} … {}",
                        frequency(conversion.input_frequency(summary.band.0)),
                        frequency(conversion.input_frequency(summary.band.1))
                    )
                } else {
                    "Not requested".into()
                },
            ),
            (
                "Output frequency band",
                format!(
                    "{} … {}",
                    frequency(conversion.output_frequency(summary.band.0)),
                    frequency(conversion.output_frequency(summary.band.1))
                ),
            ),
        ];
        measurement_table(
            ui,
            &rows
                .iter()
                .map(|(label, value)| (*label, value.as_str()))
                .collect::<Vec<_>>(),
        );
    }
    if let Some(figure) = &summary.noise_figure {
        figure::show(ui, figure, digits);
    }

    if summary.rows.is_empty() {
        section_header(ui, "Contributors", None);
        crate::presentation::panel_note(
            ui,
            "No contributor data. Turn on contributor ranking in the noise analysis.",
        );
        return ContributorResponse::default();
    }
    section_header(ui, "Contributors", Some("band-integrated"));
    let lines: Vec<ContributorLine<'_>> = summary
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| ContributorLine {
            rank: index + 1,
            device: &row.device,
            mechanism: &row.mechanism,
            figure: Some(row.power),
            figure_absent: None,
            share_pct: Some(row.share_pct),
        })
        .collect();
    contributor_table(
        ui,
        "noise-contributors",
        &lines,
        ("Power", summary.power_unit()),
        digits,
        Some(pinned),
    )
}
