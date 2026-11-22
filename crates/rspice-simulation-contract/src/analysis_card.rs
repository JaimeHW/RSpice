//! A SPICE analysis card as the plan draft that writes it.
//!
//! The card is read as SPICE reads it (`.tran tstep tstop [tstart [tmax]]
//! [uic]`), into the draft's own fields, so the plan validates and writes it
//! as one entered on the Analyses page. A card the plan has no draft for is
//! refused with why.

use crate::analysis_draft::AnalysisDraft;
use crate::analysis_kind::AnalysisKind;
use crate::config::NoiseSweepType;
use crate::options::format_si_value;
use crate::spice_value::parse_spice_value_checked;

/// The draft for one analysis card, and the card's words the draft has no
/// field for.
pub fn analysis_card_draft(card: &str) -> Result<(AnalysisDraft, Vec<String>), String> {
    let words = card.split_whitespace().collect::<Vec<_>>();
    let Some((keyword, words)) = words.split_first() else {
        return Err("the card is empty".to_owned());
    };
    let keyword = keyword.to_ascii_lowercase();
    if keyword == ".tran" {
        return transient(words);
    }
    let words = words
        .iter()
        .map(|word| (*word).to_owned())
        .collect::<Vec<_>>();
    analysis_words_draft(&keyword, &words).map(|draft| (draft, Vec::new()))
}

fn number(word: &str) -> Result<f64, String> {
    parse_spice_value_checked(word).map_err(|_| format!("`{word}` is not a number"))
}

/// `.tran tstep tstop [tstart [tmax]] [uic]`, or `.tran tstop` alone.
fn transient(words: &[&str]) -> Result<(AnalysisDraft, Vec<String>), String> {
    let mut uic = false;
    let mut values = Vec::new();
    let mut ignored = Vec::new();
    for word in words {
        match word.to_ascii_lowercase().as_str() {
            "uic" => uic = true,
            _ if number(word).is_ok() => values.push(*word),
            _ => ignored.push((*word).to_owned()),
        }
    }
    let (step, stop, start, max_step) = match values.as_slice() {
        [stop] => (None, *stop, None, None),
        [step, stop] => (Some(*step), *stop, None, None),
        [step, stop, start] => (Some(*step), *stop, Some(*start), None),
        [step, stop, start, max_step] => (Some(*step), *stop, Some(*start), Some(*max_step)),
        _ => return Err("a transient card names one to four times".to_owned()),
    };
    let stop_time = number(stop)?;
    let mut draft = AnalysisDraft::for_kind(AnalysisKind::Transient);
    if let AnalysisDraft::Transient(tran) = &mut draft {
        tran.stop = stop.to_owned();
        tran.step = step.map_or_else(|| format_si_value(stop_time / 1000.0), str::to_owned);
        tran.start = start.unwrap_or("0").to_owned();
        if let Some(max_step) = max_step {
            tran.max_step = max_step.to_owned();
        }
        tran.uic = uic;
    }
    Ok((draft, ignored))
}

/// The draft for an `.op`, `.ac`, `.dc`, `.noise` or `.tf` card, from the
/// words after its keyword. Their words read the same in every SPICE.
pub fn analysis_words_draft(keyword: &str, words: &[String]) -> Result<AnalysisDraft, String> {
    let draft = match keyword {
        ".op" => AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
        ".ac" => {
            let [mode, points, start, stop] = words else {
                return Err("only a dec, oct or lin frequency sweep is a plan AC analysis".into());
            };
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::Ac);
            if let AnalysisDraft::Ac(ac) = &mut draft {
                ac.sweep = sweep_index(mode)?;
                ac.points = points.clone();
                ac.fstart = start.clone();
                ac.fstop = stop.clone();
            }
            draft
        }
        ".dc" => {
            let (mode, axes) = match words.first().map(|word| word.to_ascii_lowercase()) {
                Some(word) if word == "lin" => (0, &words[1..]),
                Some(word) if word == "dec" => (2, &words[1..]),
                Some(word) if word == "oct" => (3, &words[1..]),
                _ => (0, words),
            };
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::DcSweep);
            if let AnalysisDraft::DcSweep(dc) = &mut draft {
                dc.mode = mode;
                match axes {
                    [source, start, stop, step, rest @ ..] => {
                        dc.source = source.clone();
                        dc.start = start.clone();
                        dc.stop = stop.clone();
                        dc.step = step.clone();
                        dc.points = step.clone();
                        match rest {
                            [] => dc.nested = false,
                            [source2, start2, stop2, step2] => {
                                dc.nested = true;
                                dc.source2 = source2.clone();
                                dc.start2 = start2.clone();
                                dc.stop2 = stop2.clone();
                                dc.step2 = step2.clone();
                            }
                            _ => return Err("a DC sweep card has one or two sweeps".into()),
                        }
                    }
                    _ => return Err("a DC sweep card names a source, start, stop and step".into()),
                }
            }
            draft
        }
        ".noise" => {
            let [output, input, mode, points, start, stop] = words else {
                return Err("a noise card names an output, a source and a frequency sweep".into());
            };
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::Noise);
            if let AnalysisDraft::Noise(noise) = &mut draft {
                noise.output = output.clone();
                noise.input = input.clone();
                noise.sweep = match sweep_index(mode)? {
                    0 => NoiseSweepType::Decade,
                    1 => NoiseSweepType::Octave,
                    _ => NoiseSweepType::Linear,
                };
                noise.points = points.clone();
                noise.fstart = start.clone();
                noise.fstop = stop.clone();
            }
            draft
        }
        ".tf" => {
            let [output, input] = words else {
                return Err("a transfer-function card names an output and a source".into());
            };
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::TransferFunction);
            if let AnalysisDraft::TransferFunction(xf) = &mut draft {
                xf.output_expression = output.clone();
                xf.input_source = input.clone();
            }
            draft
        }
        other => return Err(format!("`{other}` is not a plan analysis")),
    };
    Ok(draft)
}

fn sweep_index(mode: &str) -> Result<usize, String> {
    match mode.to_ascii_lowercase().as_str() {
        "dec" => Ok(0),
        "oct" => Ok(1),
        "lin" => Ok(2),
        _ => Err(format!("`{mode}` is not a dec, oct or lin sweep")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_transient_card_reads_as_spice_reads_it() {
        let (draft, ignored) = analysis_card_draft(".TRAN 1n 2u 0 5n uic").unwrap();
        assert!(ignored.is_empty());
        let AnalysisDraft::Transient(tran) = draft else {
            panic!("a transient draft");
        };
        assert_eq!(
            (
                tran.step.as_str(),
                tran.stop.as_str(),
                tran.start.as_str(),
                tran.max_step.as_str(),
                tran.uic
            ),
            ("1n", "2u", "0", "5n", true)
        );
    }

    #[test]
    fn a_card_the_plan_has_no_draft_for_is_refused() {
        assert_eq!(
            analysis_card_draft(".four 1k v(out)").unwrap_err(),
            "`.four` is not a plan analysis"
        );
        assert!(analysis_card_draft(".ac dec 10 1 1meg").is_ok());
    }
}
