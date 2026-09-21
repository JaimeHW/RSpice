//! Editable sampling controls; inactive text stays in the saved draft.

use super::parse_si_value;
use rspice_core::analysis::pnoise::{
    PeriodicNoiseEdge, PeriodicNoiseEdgeDirection, PeriodicNoiseSampling,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EdgeDraft {
    pub threshold: String,
    pub direction_idx: usize,
    pub occurrence: String,
    pub phase_tolerance: String,
    pub minimum_slew: String,
}

impl Default for EdgeDraft {
    fn default() -> Self {
        Self::from(&PeriodicNoiseEdge::default())
    }
}

impl From<&PeriodicNoiseEdge> for EdgeDraft {
    fn from(edge: &PeriodicNoiseEdge) -> Self {
        Self {
            threshold: edge.threshold_volts.to_string(),
            direction_idx: match edge.direction {
                PeriodicNoiseEdgeDirection::Rising => 0,
                PeriodicNoiseEdgeDirection::Falling => 1,
                PeriodicNoiseEdgeDirection::Either => 2,
            },
            occurrence: edge.occurrence.to_string(),
            phase_tolerance: edge.phase_tolerance_degrees.to_string(),
            minimum_slew: edge.minimum_slew_volts_per_second.to_string(),
        }
    }
}

impl EdgeDraft {
    fn to_config(&self) -> Result<PeriodicNoiseEdge, String> {
        let edge = PeriodicNoiseEdge {
            threshold_volts: parse_si_value(&self.threshold)
                .map_err(|e| format!("Invalid crossing threshold: {e}"))?,
            direction: match self.direction_idx {
                0 => PeriodicNoiseEdgeDirection::Rising,
                1 => PeriodicNoiseEdgeDirection::Falling,
                2 => PeriodicNoiseEdgeDirection::Either,
                _ => return Err("Invalid crossing direction".into()),
            },
            occurrence: self
                .occurrence
                .trim()
                .parse()
                .map_err(|_| "Crossing occurrence must be a positive integer")?,
            phase_tolerance_degrees: parse_si_value(&self.phase_tolerance)
                .map_err(|e| format!("Invalid crossing phase tolerance: {e}"))?,
            minimum_slew_volts_per_second: parse_si_value(&self.minimum_slew)
                .map_err(|e| format!("Invalid minimum crossing slew: {e}"))?,
        };
        edge.validate()?;
        Ok(edge)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SamplingDraft {
    pub mode_idx: usize,
    pub phase: String,
    pub edge: EdgeDraft,
    pub reference_node: String,
    pub reference_ref: String,
    pub reference_edge: EdgeDraft,
    pub periods: String,
}

impl Default for SamplingDraft {
    fn default() -> Self {
        Self {
            mode_idx: 0,
            phase: "0".into(),
            edge: EdgeDraft::default(),
            reference_node: String::new(),
            reference_ref: String::new(),
            reference_edge: EdgeDraft::default(),
            periods: "0".into(),
        }
    }
}

impl SamplingDraft {
    pub fn from_config(config: Option<&PeriodicNoiseSampling>) -> Self {
        let mut draft = Self::default();
        match config {
            None => (),
            Some(PeriodicNoiseSampling::Phase { phase_degrees }) => {
                draft.mode_idx = 1;
                draft.phase = phase_degrees.to_string();
            }
            Some(PeriodicNoiseSampling::Edge { edge }) => {
                draft.mode_idx = 2;
                draft.edge = EdgeDraft::from(edge);
            }
            Some(PeriodicNoiseSampling::Delay {
                edge,
                reference_node,
                reference_ref,
                reference_edge,
                periods,
            }) => {
                draft.mode_idx = 3;
                draft.edge = EdgeDraft::from(edge);
                draft.reference_node = reference_node.clone();
                draft.reference_ref = reference_ref.clone().unwrap_or_default();
                draft.reference_edge = EdgeDraft::from(reference_edge);
                draft.periods = periods.to_string();
            }
        }
        draft
    }

    pub fn to_config(&self) -> Result<Option<PeriodicNoiseSampling>, String> {
        let sampling = match self.mode_idx {
            0 => return Ok(None),
            1 => PeriodicNoiseSampling::Phase {
                phase_degrees: parse_si_value(&self.phase)
                    .map_err(|e| format!("Invalid sampling phase: {e}"))?,
            },
            2 => PeriodicNoiseSampling::Edge {
                edge: self.edge.to_config()?,
            },
            3 => PeriodicNoiseSampling::Delay {
                edge: self.edge.to_config()?,
                reference_node: self.reference_node.trim().into(),
                reference_ref: (!self.reference_ref.trim().is_empty())
                    .then(|| self.reference_ref.trim().into()),
                reference_edge: self.reference_edge.to_config()?,
                periods: self
                    .periods
                    .trim()
                    .parse()
                    .map_err(|_| "Delay periods must be a nonnegative 32-bit integer")?,
            },
            _ => return Err("Invalid periodic-noise sampling mode".into()),
        };
        sampling.validate()?;
        Ok(Some(sampling))
    }
}

pub(super) fn spice_options(sampling: &PeriodicNoiseSampling) -> String {
    fn edge_options(edge: &PeriodicNoiseEdge, prefix: &str) -> String {
        let direction = match edge.direction {
            PeriodicNoiseEdgeDirection::Rising => "rising",
            PeriodicNoiseEdgeDirection::Falling => "falling",
            PeriodicNoiseEdgeDirection::Either => "either",
        };
        format!(
            " {prefix}threshold={} {prefix}direction={direction} {prefix}occurrence={} {prefix}phasetol={} {prefix}minslew={}",
            edge.threshold_volts,
            edge.occurrence,
            edge.phase_tolerance_degrees,
            edge.minimum_slew_volts_per_second
        )
    }
    match sampling {
        PeriodicNoiseSampling::Phase { phase_degrees } => {
            format!(" sampling=phase samplephase={phase_degrees}")
        }
        PeriodicNoiseSampling::Edge { edge } => format!(" sampling=edge{}", edge_options(edge, "")),
        PeriodicNoiseSampling::Delay {
            edge,
            reference_node,
            reference_ref,
            reference_edge,
            periods,
        } => format!(
            " sampling=delay{} refout=V({}){} periods={periods}",
            edge_options(edge, ""),
            reference_ref.as_ref().map_or_else(
                || reference_node.clone(),
                |reference| format!("{reference_node},{reference}")
            ),
            edge_options(reference_edge, "ref")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::dialog::pnoise::{PnoiseConfig, PnoiseDialogState};

    #[test]
    fn sampled_pnoise_studio_draft_and_deck_preserve_all_active_settings() {
        let edge = PeriodicNoiseEdge {
            threshold_volts: 0.4,
            direction: PeriodicNoiseEdgeDirection::Falling,
            occurrence: 2,
            phase_tolerance_degrees: 1e-6,
            minimum_slew_volts_per_second: 2000.0,
        };
        for sampling in [
            PeriodicNoiseSampling::Phase {
                phase_degrees: -37.0,
            },
            PeriodicNoiseSampling::Edge { edge: edge.clone() },
            PeriodicNoiseSampling::Delay {
                edge,
                reference_node: "CLK".into(),
                reference_ref: None,
                reference_edge: PeriodicNoiseEdge::default(),
                periods: 2,
            },
        ] {
            let config = PnoiseConfig {
                sampling: Some(sampling.clone()),
                ..Default::default()
            };
            let draft = PnoiseDialogState::from_config(&config);
            let restored: PnoiseDialogState =
                serde_json::from_str(&serde_json::to_string(&draft).unwrap()).unwrap();
            assert_eq!(
                restored.to_config().unwrap().sampling,
                Some(sampling.clone())
            );
            let deck =
                rspice_core::Netlist::parse(&format!("sampling\n{}\n.end\n", config.to_spice()))
                    .unwrap();
            let rspice_core::netlist::AnalysisCommand::Pnoise(card) = &deck.analyses[0] else {
                panic!()
            };
            assert_eq!(card.sampling, Some(sampling));
        }
        let mut draft = PnoiseDialogState::from_config(&PnoiseConfig::default());
        draft.sampling.edge.threshold = "unfinished".into();
        assert!(draft.to_config().unwrap().sampling.is_none());
        draft.sampling.mode_idx = 2;
        assert!(draft.to_config().is_err());
        draft.sampling.edge.threshold = "400m".into();
        draft.output_sideband = "unfinished".into();
        assert_eq!(draft.to_config().unwrap().output_sideband, 0);
    }
}
