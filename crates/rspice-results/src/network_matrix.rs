//! Retained network topology, channel references and exact matrix report values.
use crate::{
    analysis_result::AnalysisResult, analysis_type::AnalysisType,
    family_metadata::AnalysisResultFamilyMetadata, waveform::RetainedWaveform,
};
use std::collections::BTreeMap;
/// The network term one retained trace name spells.
///
/// Shared with the polar sheet: both read the same S-parameter naming
/// contract, and a second parser would be a second answer to what `Sdd21`
/// means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SParameterTraceIdentity {
    pub output_port: usize,
    pub input_port: usize,
    pub physical_ports: bool,
}

pub fn trace_identity(name: &str) -> Option<SParameterTraceIdentity> {
    let core = name
        .trim()
        .trim_matches('|')
        .split_once('[')
        .map_or_else(|| name.trim().trim_matches('|'), |(core, _)| core);
    let suffix = core.strip_prefix('S').or_else(|| core.strip_prefix('s'))?;
    let (physical_ports, indices) = match suffix.get(..2) {
        Some(prefix)
            if matches!(
                prefix.to_ascii_lowercase().as_str(),
                "dd" | "dc" | "cd" | "cc"
            ) =>
        {
            (false, &suffix[2..])
        }
        _ => (true, suffix),
    };
    let (output_port, input_port) = if let Some((output, input)) = indices.split_once('_') {
        (output.parse().ok()?, input.parse().ok()?)
    } else if indices.len() == 2 && indices.bytes().all(|byte| byte.is_ascii_digit()) {
        let bytes = indices.as_bytes();
        ((bytes[0] - b'0') as usize, (bytes[1] - b'0') as usize)
    } else {
        return None;
    };
    (output_port > 0 && input_port > 0).then_some(SParameterTraceIdentity {
        output_port,
        input_port,
        physical_ports,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BlockKey {
    mixed: bool,
    sidebands: Option<(i32, i32)>,
}

impl BlockKey {
    fn label(&self) -> String {
        let basis = if self.mixed {
            "Differential / common"
        } else {
            "Single-ended"
        };
        match self.sidebands {
            Some((output, input)) => format!("{basis} · k={output:+}, m={input:+}"),
            None => basis.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatrixBlock {
    key: BlockKey,
    /// Row-major indexes into the immutable analysis waveform inventory.
    cells: Vec<usize>,
}

impl MatrixBlock {
    pub fn label(&self) -> String {
        self.key.label()
    }
    pub fn is_mixed(&self) -> bool {
        self.key.mixed
    }
    /// Row-major positions in the analysis used to resolve this layout.
    pub fn cells(&self) -> &[usize] {
        &self.cells
    }
}

/// Resolved shape only. The caller owns source-generation tracking and sample qualification.
#[derive(Debug, Clone)]
pub struct NetworkLayout {
    references: Vec<f64>,
    blocks: Vec<MatrixBlock>,
}

/// Exact textual report facts; these do not confer result qualification.
pub struct NetworkMatrixTable {
    pub title: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

fn coefficient_name(waveform: &RetainedWaveform) -> &str {
    waveform.complex.as_ref().map_or(&waveform.name, |complex| {
        if complex.source_name.trim().is_empty() {
            &waveform.name
        } else {
            &complex.source_name
        }
    })
}
fn term(name: &str, ports: usize) -> Option<(BlockKey, usize, usize)> {
    let name = name.trim().trim_matches('|');
    let (base, sidebands) = if let Some((base, tail)) = name.split_once('[') {
        let (output, input) = tail.strip_suffix(']')?.split_once(',')?;
        (
            base,
            Some((
                output.strip_prefix("k=")?.parse().ok()?,
                input.strip_prefix("m=")?.parse().ok()?,
            )),
        )
    } else {
        (name, None)
    };
    let identity = trace_identity(base)?;
    let mixed = !identity.physical_ports;
    let (row, column) = if mixed {
        if !ports.is_multiple_of(2) {
            return None;
        }
        let pairs = ports / 2;
        if identity.output_port > pairs || identity.input_port > pairs {
            return None;
        }
        let prefix = base.get(1..3)?.to_ascii_lowercase();
        let modes = prefix.as_bytes();
        (
            identity.output_port - 1 + usize::from(modes[0] == b'c') * pairs,
            identity.input_port - 1 + usize::from(modes[1] == b'c') * pairs,
        )
    } else {
        if identity.output_port > ports || identity.input_port > ports {
            return None;
        }
        (identity.output_port - 1, identity.input_port - 1)
    };
    Some((BlockKey { mixed, sidebands }, row, column))
}

pub fn resolve<W: AsRef<RetainedWaveform>>(analysis: &AnalysisResult<W>) -> Option<NetworkLayout> {
    if !analysis.success
        || !matches!(
            analysis.analysis_type,
            AnalysisType::SParameter | AnalysisType::Psp | AnalysisType::Hbsp
        )
    {
        return None;
    }
    let Some(AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm: references,
        ..
    }) = &analysis.family_metadata
    else {
        return None;
    };
    let ports = references.len();
    let cells = ports.checked_mul(ports)?;
    if ports == 0 || references.iter().any(|z| !z.is_finite() || *z <= 0.0) {
        return None;
    }
    let mut blocks: BTreeMap<BlockKey, BTreeMap<usize, usize>> = BTreeMap::new();
    for (index, waveform) in analysis.waveforms.iter().map(AsRef::as_ref).enumerate() {
        let Some(complex) = &waveform.complex else {
            continue;
        };
        let name = if complex.source_name.trim().is_empty() {
            &waveform.name
        } else {
            &complex.source_name
        };
        let Some((key, row, column)) = term(name, ports) else {
            continue;
        };
        if waveform.x.is_empty()
            || waveform.x.len() != complex.real.len()
            || waveform.x.len() != complex.imag.len()
        {
            return None;
        }
        if key.mixed
            && references.chunks_exact(2).any(|pair| {
                pair[0] != pair[1] || !(pair[0] * 2.0).is_finite() || pair[0] / 2.0 <= 0.0
            })
        {
            return None;
        }
        if blocks
            .entry(key)
            .or_default()
            .insert(row * ports + column, index)
            .is_some()
        {
            return None;
        }
    }
    // Every advertised block must be complete. A partial matrix cannot prove
    // mode conversion, reciprocity, or the absence of a coupling term.
    if blocks.is_empty() || blocks.values().any(|block| block.len() != cells) {
        return None;
    }
    let blocks = blocks
        .into_iter()
        .map(|(key, cells)| MatrixBlock {
            key,
            cells: cells.into_values().collect(),
        })
        .collect();
    Some(NetworkLayout {
        references: references.to_vec(),
        blocks,
    })
}

pub fn channel_label(index: usize, ports: usize, mixed: bool) -> String {
    if mixed {
        let pairs = ports / 2;
        format!(
            "{}{}",
            if index < pairs { "D" } else { "C" },
            index % pairs + 1
        )
    } else {
        format!("P{}", index + 1)
    }
}

pub fn channel_reference(index: usize, references: &[f64], mixed: bool) -> f64 {
    if mixed {
        let pairs = references.len() / 2;
        references[2 * (index % pairs)] * if index < pairs { 2.0 } else { 0.5 }
    } else {
        references[index]
    }
}

impl NetworkLayout {
    pub fn references(&self) -> &[f64] {
        &self.references
    }
    pub fn blocks(&self) -> &[MatrixBlock] {
        &self.blocks
    }
    fn matches_analysis<W: AsRef<RetainedWaveform>>(&self, analysis: &AnalysisResult<W>) -> bool {
        analysis.success
            && matches!(
                analysis.analysis_type,
                AnalysisType::SParameter | AnalysisType::Psp | AnalysisType::Hbsp
            )
            && matches!(&analysis.family_metadata, Some(AnalysisResultFamilyMetadata::SParameter { reference_impedances_ohm, .. }) if reference_impedances_ohm == &self.references)
    }
    /// Check full grids and components once per retained generation, outside rendering.
    pub fn samples_are_finite_and_aligned<W: AsRef<RetainedWaveform>>(
        &self,
        analysis: &AnalysisResult<W>,
    ) -> bool {
        if !self.matches_analysis(analysis) {
            return false;
        }
        let ports = self.references.len();
        self.blocks.iter().all(|block| {
            let Some(first) = analysis.waveforms.get(block.cells[0]) else {
                return false;
            };
            let grid = &first.as_ref().x;
            !grid.is_empty()
                && grid.iter().all(|x| x.is_finite() && *x >= 0.0)
                && grid.windows(2).all(|pair| pair[0] < pair[1])
                && block.cells.iter().enumerate().all(|(cell, &index)| {
                    let Some(waveform) = analysis.waveforms.get(index).map(AsRef::as_ref) else {
                        return false;
                    };
                    term(coefficient_name(waveform), ports)
                        == Some((block.key.clone(), cell / ports, cell % ports))
                        && waveform.x.as_ref() == grid.as_ref()
                        && waveform.complex.as_ref().is_some_and(|c| {
                            c.real.len() == grid.len()
                                && c.imag.len() == grid.len()
                                && c.real.iter().chain(c.imag.iter()).all(|v| v.is_finite())
                        })
                })
        })
    }
    /// Project one sample after source-generation and structural admission.
    /// Reject incompatible rebinding and out-of-range indexes without a whole-grid scan.
    pub fn exact_table<W: AsRef<RetainedWaveform>>(
        &self,
        analysis: &AnalysisResult<W>,
        block_index: usize,
        sample: usize,
    ) -> Option<NetworkMatrixTable> {
        if !self.matches_analysis(analysis) {
            return None;
        }
        let block = self.blocks.get(block_index)?;
        let ports = self.references.len();
        let rows = block
            .cells
            .iter()
            .enumerate()
            .map(|(cell, &index)| {
                let waveform = analysis.waveforms.get(index)?.as_ref();
                let complex = waveform.complex.as_ref()?;
                if term(coefficient_name(waveform), ports)
                    != Some((block.key.clone(), cell / ports, cell % ports))
                {
                    return None;
                }
                let frequency = waveform.x.get(sample)?;
                let real = complex.real.get(sample)?;
                let imaginary = complex.imag.get(sample)?;
                Some(vec![
                    complex.source_name.clone(),
                    format!("{:.17e}", frequency),
                    channel_label(cell / ports, ports, block.key.mixed),
                    channel_label(cell % ports, ports, block.key.mixed),
                    format!(
                        "{:.17e}",
                        channel_reference(cell / ports, &self.references, block.key.mixed)
                    ),
                    format!(
                        "{:.17e}",
                        channel_reference(cell % ports, &self.references, block.key.mixed)
                    ),
                    format!("{:.17e}", real),
                    format!("{:.17e}", imaginary),
                ])
            })
            .collect::<Option<Vec<_>>>()?;
        Some(NetworkMatrixTable {
            title: format!("Network matrix · {}", block.key.label()),
            columns: [
                "Coefficient",
                "Frequency (Hz)",
                "Output",
                "Input",
                "Output reference (ohm)",
                "Input reference (ohm)",
                "Real",
                "Imaginary",
            ]
            .map(str::to_owned)
            .to_vec(),
            rows,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trace_identity_distinguishes_reflection_transmission_and_mixed_mode() {
        assert_eq!(
            trace_identity("S22"),
            Some(SParameterTraceIdentity {
                output_port: 2,
                input_port: 2,
                physical_ports: true,
            })
        );
        assert_eq!(
            trace_identity("S2_10[k=+0,m=+0]"),
            Some(SParameterTraceIdentity {
                output_port: 2,
                input_port: 10,
                physical_ports: true,
            })
        );
        assert_eq!(
            trace_identity("Sdd11"),
            Some(SParameterTraceIdentity {
                output_port: 1,
                input_port: 1,
                physical_ports: false,
            })
        );
    }

    #[test]
    fn translated_terms_preserve_large_port_and_sideband_identity() {
        assert_eq!(
            term("S12_10[k=-2,m=+3]", 12),
            Some((
                BlockKey {
                    mixed: false,
                    sidebands: Some((-2, 3))
                },
                11,
                9
            ))
        );
        assert!(term("S12[k=0]", 2).is_none());
        assert!(term("S00", 2).is_none());
    }
    #[test]
    fn resolved_layout_refuses_incompatible_rebinding_and_out_of_range_samples() {
        let mut analysis: AnalysisResult =
            AnalysisResult::new(1, AnalysisType::SParameter, "network", 0.0)
                .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                    reference_impedances_ohm: vec![50.0],
                    noise_reference_temperature_kelvin: None,
                })
                .with_waveforms(vec![
                    RetainedWaveform::new("|S11|", vec![1.0], vec![0.5]).with_complex_components(
                        "S11",
                        vec![0.5],
                        vec![-0.0],
                    ),
                ]);
        let layout = resolve(&analysis).unwrap();
        assert!(layout.samples_are_finite_and_aligned(&analysis));
        let table = layout.exact_table(&analysis, 0, 0).unwrap();
        assert_eq!(
            table.rows[0][7].parse::<f64>().unwrap().to_bits(),
            (-0.0_f64).to_bits()
        );
        assert!(layout.exact_table(&analysis, 1, 0).is_none());
        assert!(layout.exact_table(&analysis, 0, 1).is_none());
        analysis.waveforms[0].complex.as_mut().unwrap().source_name = "S22".into();
        assert!(!layout.samples_are_finite_and_aligned(&analysis));
        assert!(layout.exact_table(&analysis, 0, 0).is_none());
        analysis.waveforms.clear();
        assert!(!layout.samples_are_finite_and_aligned(&analysis));
        assert!(layout.exact_table(&analysis, 0, 0).is_none());
    }
}
