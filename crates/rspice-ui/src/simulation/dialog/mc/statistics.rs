//! Authored per-parameter statistics backed by the native statistical sampler.

use rspice_core::netlist::{
    SpectreCorrelation, SpectreDistribution, SpectreSpread, SpectreStatisticsPlan,
    SpectreVariation, SpectreVariationScope,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McScope {
    #[default]
    Process,
    Mismatch,
}
impl McScope {
    fn native(self) -> SpectreVariationScope {
        match self {
            Self::Process => SpectreVariationScope::Process,
            Self::Mismatch => SpectreVariationScope::Mismatch,
        }
    }
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McShape {
    #[default]
    Gaussian,
    Uniform,
    Lognormal,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McParameterVariation {
    pub parameter: String,
    pub scope: McScope,
    pub distribution: McShape,
    pub spread: f64,
    pub percent: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McParameterCorrelation {
    pub scope: McScope,
    pub parameters: Vec<String>,
    pub coefficient: f64,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McStatisticsConfig {
    pub variations: Vec<McParameterVariation>,
    pub correlations: Vec<McParameterCorrelation>,
}
impl McStatisticsConfig {
    pub fn parser_directive(&self) -> Result<String, String> {
        if self.variations.is_empty() {
            return Err("Add at least one parameter variation".into());
        }
        let mut plan = SpectreStatisticsPlan::default();
        for (index, row) in self.variations.iter().enumerate() {
            if !super::is_parameter_name(&row.parameter)
                || !row.spread.is_finite()
                || row.spread < 0.0
            {
                return Err(
                    "Variations require valid parameter names and finite nonnegative spreads"
                        .into(),
                );
            }
            plan.variations.push(SpectreVariation {
                bounds: None,
                line: index + 1,
                scope: row.scope.native(),
                parameter: row.parameter.clone(),
                distribution: match row.distribution {
                    McShape::Gaussian => SpectreDistribution::Gaussian,
                    McShape::Uniform => SpectreDistribution::Uniform,
                    McShape::Lognormal => SpectreDistribution::Lognormal,
                },
                spread: match row.distribution {
                    McShape::Uniform => SpectreSpread::HalfRange(row.spread.to_string()),
                    _ => SpectreSpread::StandardDeviation(row.spread.to_string()),
                },
                percent: row.percent,
            });
        }
        for (index, row) in self.correlations.iter().enumerate() {
            if !row.coefficient.is_finite() || !(-1.0..=1.0).contains(&row.coefficient) {
                return Err("Correlation coefficients must be finite and in [-1, 1]".into());
            }
            plan.correlations.push(SpectreCorrelation {
                line: index + 1,
                scope: row.scope.native(),
                parameters: row.parameters.clone(),
                coefficient: row.coefficient.to_string(),
            });
        }
        let directive = plan
            .to_parser_directive()
            .map_err(|error| error.to_string())?;
        // Validate authored target matrices before dispatch. The sampler also
        // validates distribution-dependent attainable correlations at solve time.
        for scope in [McScope::Process, McScope::Mismatch] {
            if !self.correlations.iter().any(|row| row.scope == scope) {
                continue;
            }
            let names = self
                .variations
                .iter()
                .filter(|row| row.scope == scope)
                .map(|row| row.parameter.to_ascii_uppercase())
                .collect::<Vec<_>>();
            if names.len().saturating_mul(names.len())
                > rspice_core::ResourceLimits::default().max_result_values
            {
                return Err(
                    "Statistical correlation matrix exceeds the configuration resource limit"
                        .into(),
                );
            }
            let mut matrix = vec![vec![0.0; names.len()]; names.len()];
            for (index, row) in matrix.iter_mut().enumerate() {
                row[index] = 1.0;
            }
            let mut assigned = std::collections::BTreeMap::new();
            for row in self.correlations.iter().filter(|row| row.scope == scope) {
                let indices = row
                    .parameters
                    .iter()
                    .map(|name| {
                        names
                            .iter()
                            .position(|value| value.eq_ignore_ascii_case(name))
                            .expect("validated variation reference")
                    })
                    .collect::<Vec<_>>();
                for (offset, &left) in indices.iter().enumerate() {
                    for &right in &indices[offset + 1..] {
                        let key = (left.min(right), left.max(right));
                        if assigned
                            .insert(key, row.coefficient)
                            .is_some_and(|previous| previous != row.coefficient)
                        {
                            return Err(
                                "Conflicting correlation coefficients for one parameter pair"
                                    .into(),
                            );
                        }
                        matrix[left][right] = row.coefficient;
                        matrix[right][left] = row.coefficient;
                    }
                }
            }
            rspice_core::netlist::SpectreCorrelationMatrix::new(matrix)
                .map_err(|error| error.to_string())?;
        }
        Ok(directive)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McVariationDraft {
    pub parameter: String,
    pub scope: usize,
    pub distribution: usize,
    pub spread: String,
    pub percent: bool,
}
impl Default for McVariationDraft {
    fn default() -> Self {
        Self {
            parameter: String::new(),
            scope: 0,
            distribution: 0,
            spread: "1".into(),
            percent: true,
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McCorrelationDraft {
    pub scope: usize,
    pub parameters: String,
    pub coefficient: String,
}
fn scope(index: usize) -> Result<McScope, String> {
    match index {
        0 => Ok(McScope::Process),
        1 => Ok(McScope::Mismatch),
        _ => Err("Invalid statistical scope".into()),
    }
}
impl McVariationDraft {
    pub fn from_config(row: &McParameterVariation) -> Self {
        Self {
            parameter: row.parameter.clone(),
            scope: usize::from(row.scope == McScope::Mismatch),
            distribution: match row.distribution {
                McShape::Gaussian => 0,
                McShape::Uniform => 1,
                McShape::Lognormal => 2,
            },
            spread: row.spread.to_string(),
            percent: row.percent,
        }
    }
    pub fn to_config(&self) -> Result<McParameterVariation, String> {
        Ok(McParameterVariation {
            parameter: self.parameter.trim().into(),
            scope: scope(self.scope)?,
            distribution: match self.distribution {
                0 => McShape::Gaussian,
                1 => McShape::Uniform,
                2 => McShape::Lognormal,
                _ => return Err("Invalid statistical distribution".into()),
            },
            spread: crate::simulation::dialog::options::parse_si_value(&self.spread)
                .map_err(|error| format!("Invalid parameter spread: {error}"))?,
            percent: self.percent,
        })
    }
}
impl McCorrelationDraft {
    pub fn from_config(row: &McParameterCorrelation) -> Self {
        Self {
            scope: usize::from(row.scope == McScope::Mismatch),
            parameters: row.parameters.join(", "),
            coefficient: row.coefficient.to_string(),
        }
    }
    pub fn to_config(&self) -> Result<McParameterCorrelation, String> {
        Ok(McParameterCorrelation {
            scope: scope(self.scope)?,
            parameters: self
                .parameters
                .split(|ch: char| ch == ',' || ch.is_whitespace())
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
                .collect(),
            coefficient: self
                .coefficient
                .trim()
                .parse()
                .map_err(|_| "Invalid correlation coefficient")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn custom_statistics_preserve_editor_buffers_and_validate_correlations() {
        let mut draft = crate::simulation::dialog::McDialogState::from_config(
            &super::super::McConfig::default(),
        );
        draft.variation_source_idx = 2;
        draft.variation_pct = "unfinished(".into();
        draft.vary_only = "inactive unfinished".into();
        draft.variations = ["X", "Y", "Z"]
            .iter()
            .map(|name| McVariationDraft {
                parameter: (*name).into(),
                spread: "10m".into(),
                percent: false,
                ..Default::default()
            })
            .collect();
        draft.correlations.push(McCorrelationDraft {
            parameters: "X, Y, Z".into(),
            coefficient: "0.5".into(),
            ..Default::default()
        });
        for restored in [
            serde_json::from_str::<crate::simulation::dialog::McDialogState>(
                &serde_json::to_string(&draft).unwrap(),
            )
            .unwrap(),
            ron::from_str::<crate::simulation::dialog::McDialogState>(
                &ron::to_string(&draft).unwrap(),
            )
            .unwrap(),
        ] {
            assert_eq!(
                restored.to_config().unwrap().statistics.unwrap().variations[0].spread,
                0.01
            );
            assert_eq!(restored.variation_pct, "unfinished(");
        }
        draft.correlations[0].coefficient = "-0.9".into();
        assert!(
            draft
                .to_config()
                .unwrap_err()
                .contains("positive semidefinite")
        );
        draft.correlations[0].parameters = "X, missing".into();
        assert!(draft.to_config().is_err());
        draft.variation_source_idx = 1;
        assert!(draft.to_config().unwrap().statistics.is_none());
    }
}
