//! Human-readable descriptions of analysis directives.
//!
//! Every `AnalysisRecord` carries one of these, so a run report names what it
//! executed in the deck's own notation rather than by an internal tag.

use super::*;

fn describe_dc_sweep_spec(
    source: &str,
    start: Value,
    stop: Value,
    step: Value,
    mode: &DcSweepMode,
) -> String {
    match mode {
        DcSweepMode::Linear => format!("{source} {start} {stop} {step}"),
        DcSweepMode::List(values) => {
            let values = values
                .iter()
                .map(Value::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            format!("{source} list {values}")
        }
        DcSweepMode::Decade { points_per_decade } => {
            format!("{source} dec {start} {stop} {points_per_decade}")
        }
        DcSweepMode::Octave { points_per_octave } => {
            format!("{source} oct {start} {stop} {points_per_octave}")
        }
    }
}

fn describe_dc_analysis(
    source: &str,
    start: Value,
    stop: Value,
    step: Value,
    mode: &DcSweepMode,
    sweep2: Option<&DcSecondSweep>,
) -> String {
    let mut out = format!(
        ".dc {}",
        describe_dc_sweep_spec(source, start, stop, step, mode)
    );
    if let Some(outer) = sweep2 {
        out.push(' ');
        out.push_str(&describe_dc_sweep_spec(
            &outer.source,
            outer.start,
            outer.stop,
            outer.step,
            &outer.mode,
        ));
    }
    out
}

/// Render an analysis command as a short human-readable summary.
pub(crate) fn describe_analysis(analysis: &AnalysisCommand) -> String {
    match analysis {
        AnalysisCommand::Op => ".op".to_string(),
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2,
        } => describe_dc_analysis(source, *start, *stop, *step, mode, sweep2.as_ref()),
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => format!(
            ".ac {} {points} {start_freq} {stop_freq}",
            format!("{variation:?}").to_lowercase()
        ),
        AnalysisCommand::Stb {
            variation,
            points,
            start_freq,
            stop_freq,
            probe,
        } => format!(
            ".stb {} {points} {start_freq} {stop_freq} probe={probe}",
            format!("{variation:?}").to_lowercase()
        ),
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic: _,
        } => {
            let mut out = format!(".tran {step} {stop}");
            if let Some(start) = start {
                out.push_str(&format!(" {start}"));
            }
            if let Some(max_step) = max_step {
                out.push_str(&format!(" {max_step}"));
            }
            out
        }
        AnalysisCommand::Noise {
            output_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        } => format!(
            ".noise v({output_node}) {input_source} {} {points} {start_freq} {stop_freq}",
            format!("{variation:?}").to_lowercase()
        ),
        AnalysisCommand::NoiseData {
            output_node,
            input_source,
            table_name,
            ..
        } => format!(".noise v({output_node}) {input_source} data={table_name}"),
        AnalysisCommand::Tf {
            output_node,
            input_source,
            output_is_current,
            ..
        } => {
            let probe = if *output_is_current { "i" } else { "v" };
            format!(".tf {probe}({output_node}) {input_source}")
        }
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            output_is_current,
            filters,
            ac_sweep,
        } => {
            let probe = if *output_is_current {
                format!("i({output_node})")
            } else if let Some(reference) = reference_node {
                format!("v({output_node},{reference})")
            } else {
                format!("v({output_node})")
            };
            let mut description = format!(".sens {probe}");
            if !filters.is_empty() {
                description.push(' ');
                description.push_str(&filters.join(" "));
            }
            if let Some(sweep) = ac_sweep {
                description.push_str(&format!(
                    " ac {} {} {} {}",
                    format!("{:?}", sweep.variation).to_lowercase(),
                    sweep.points,
                    sweep.start_freq,
                    sweep.stop_freq
                ));
            }
            description
        }
        AnalysisCommand::Four {
            fundamental,
            outputs,
            num_harmonics,
        } => format!(
            ".four {fundamental} {} ({num_harmonics} harmonics)",
            outputs.join(" ")
        ),
        AnalysisCommand::Pss(card) => {
            if card.is_autonomous() {
                format!(
                    ".pss autonomous ({} harmonics, period guess {})",
                    card.num_harmonics, card.period_guess
                )
            } else {
                format!(
                    ".pss {} ({} harmonics)",
                    card.fundamental_freq, card.num_harmonics
                )
            }
        }
        AnalysisCommand::Pac(card) => {
            let probe = match &card.output_ref {
                Some(reference) => format!("v({},{reference})", card.output_node),
                None => format!("v({})", card.output_node),
            };
            format!(
                ".pac {} {} {} {} {} {probe} (sidebands {}..{})",
                describe_variation(card.sweep.variation),
                card.sweep.points,
                card.sweep.start_freq,
                card.sweep.stop_freq,
                card.input_source,
                card.sideband_min,
                card.sideband_max
            )
        }
        AnalysisCommand::Pnoise(card) => {
            let probe = match &card.reference_node {
                Some(reference) => format!("v({},{reference})", card.output_node),
                None => format!("v({})", card.output_node),
            };
            format!(
                ".pnoise {} {} {} {} {probe} (max sideband {})",
                describe_variation(card.sweep.variation),
                card.sweep.points,
                card.sweep.start_freq,
                card.sweep.stop_freq,
                card.max_sideband
            )
        }
        AnalysisCommand::Envelope(card) => {
            let mut description = format!(".envelope {} {}", card.duration, card.max_step);
            if !card.frozen_sources.is_empty() {
                description.push_str(&format!(" frozen={}", card.frozen_sources.join(",")));
            }
            description
        }
        other => format!("{other:?}"),
    }
}

fn describe_variation(variation: rspice_core::netlist::FreqVariation) -> String {
    format!("{variation:?}").to_lowercase()
}
