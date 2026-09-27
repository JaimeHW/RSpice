//! Located optimizer history over canonical retained evidence.
use crate::{
    analysis_result::AnalysisResult, analysis_type::AnalysisType,
    family_metadata::AnalysisResultFamilyMetadata, waveform::RetainedWaveform,
};

/// Borrowed history cells for a caller whose retained-data generation is current.
pub struct OptimizationView<'a> {
    pub iterations: &'a [f64],
    pub cost: &'a RetainedWaveform,
    pub variables: Vec<(&'a str, &'a RetainedWaveform)>,
    pub best_cost: f64,
    pub best_objectives: &'a [super::OptimizationObjectiveObservation],
    pub best_constraints: &'a [super::OptimizationConstraintObservation],
    pub best_index: usize,
    pub converged: bool,
}

/// Positions found by checking the retained candidate axes and exact optimum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationIndices {
    cost: usize,
    variables: Vec<(String, usize)>,
    best_index: usize,
}

impl OptimizationIndices {
    /// Locate once per immutable generation, using its owner's memoized evidence verdict.
    pub fn locate<W: AsRef<RetainedWaveform>>(
        analysis: &AnalysisResult<W>,
        evidence_is_valid: bool,
    ) -> Option<Self> {
        let Some(AnalysisResultFamilyMetadata::Optimization {
            iterations,
            best_cost,
            best_variables,
            ..
        }) = analysis.family_metadata.as_ref()
        else {
            return None;
        };
        if !analysis.success
            || analysis.analysis_type != AnalysisType::Optimization
            || iterations.is_empty()
            || !evidence_is_valid
        {
            return None;
        }
        let (cost_index, cost) = analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .enumerate()
            .find(|(_, waveform)| {
                waveform.name == "OPT_COST"
                    && waveform.x.as_slice() == iterations
                    && waveform.y.len() == iterations.len()
                    && waveform.y.iter().all(|value| value.is_finite())
            })?;
        let mut variables: Vec<_> = analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .enumerate()
            .filter_map(|(index, waveform)| {
                let name = waveform.name.strip_prefix("OPT_")?;
                (name != "COST"
                    && waveform.x.as_slice() == iterations
                    && waveform.y.len() == iterations.len()
                    && waveform.y.iter().all(|value| value.is_finite()))
                .then_some((name, index, waveform))
            })
            .collect();
        variables.sort_by(|left, right| left.0.cmp(right.0));
        if variables.is_empty()
            || variables.len() != best_variables.len()
            || variables.windows(2).any(|pair| pair[0].0 == pair[1].0)
            || best_variables.keys().any(|name| {
                !variables
                    .iter()
                    .any(|(candidate, _, _)| *candidate == name.as_str())
            })
        {
            return None;
        }
        let best_index = (0..iterations.len()).find(|&index| {
            cost.y[index].to_bits() == best_cost.to_bits()
                && variables.iter().all(|(name, _, waveform)| {
                    best_variables
                        .get(*name)
                        .is_some_and(|best| waveform.y[index].to_bits() == best.to_bits())
                })
        })?;
        Some(OptimizationIndices {
            cost: cost_index,
            variables: variables
                .into_iter()
                .map(|(name, index, _)| ((*name).to_owned(), index))
                .collect(),
            best_index,
        })
    }

    /// Rebind after the caller confirms the same retained-data generation.
    /// Checks shape and bound names without reading samples or copying buffers.
    /// Full sample qualification and cache currentness remain the owner's responsibility.
    pub fn view<'a, W: AsRef<RetainedWaveform>>(
        &self,
        analysis: &'a AnalysisResult<W>,
    ) -> Option<OptimizationView<'a>> {
        let AnalysisResultFamilyMetadata::Optimization {
            iterations,
            best_cost,
            best_objectives,
            best_constraints,
            converged,
            best_variables,
            ..
        } = analysis.family_metadata.as_ref()?
        else {
            return None;
        };
        let count = iterations.len();
        let cost = analysis.waveforms.get(self.cost)?.as_ref();
        if !analysis.success
            || analysis.analysis_type != AnalysisType::Optimization
            || count == 0
            || self.best_index >= count
            || cost.name != "OPT_COST"
            || cost.x.len() != count
            || cost.y.len() != count
            || self.variables.len() != best_variables.len()
        {
            return None;
        }
        let variables = self
            .variables
            .iter()
            .map(|(name, index)| {
                let waveform = analysis.waveforms.get(*index)?.as_ref();
                if waveform.x.len() != count || waveform.y.len() != count {
                    return None;
                }
                Some((
                    waveform
                        .name
                        .strip_prefix("OPT_")
                        .filter(|found| *found == name)?,
                    waveform,
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        Some(OptimizationView {
            iterations,
            cost,
            variables,
            best_cost: *best_cost,
            best_objectives,
            best_constraints,
            best_index: self.best_index,
            converged: *converged,
        })
    }

    pub fn best_index(&self) -> usize {
        self.best_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn located_history_borrows_samples_and_refuses_incompatible_rebinding() {
        let mut analysis: AnalysisResult =
            AnalysisResult::new(1, AnalysisType::Optimization, "OPT", 0.0);
        analysis.family_metadata = Some(AnalysisResultFamilyMetadata::Optimization {
            iterations: vec![0.0, 1.0, 2.0],
            best_cost: 1.0,
            best_variables: std::collections::BTreeMap::from([("GAIN".into(), 0.3)]),
            best_objectives: vec![],
            best_constraints: vec![],
            converged: true,
        });
        analysis.waveforms = vec![
            RetainedWaveform::new("OPT_COST", vec![0.0, 1.0, 2.0], vec![3.0, 2.0, 1.0]),
            RetainedWaveform::new("OPT_GAIN", vec![0.0, 1.0, 2.0], vec![0.1, 0.2, 0.3]),
        ];
        assert!(OptimizationIndices::locate(&analysis, false).is_none());
        let located = OptimizationIndices::locate(&analysis, true).unwrap();
        let view = located.view(&analysis).unwrap();
        assert_eq!(view.best_index, 2);
        assert!(std::ptr::eq(view.cost, &analysis.waveforms[0]));
        assert!(std::ptr::eq(view.variables[0].1, &analysis.waveforms[1]));
        for mutation in 0..7 {
            let mut changed = analysis.clone();
            match mutation {
                0 => changed.waveforms[0].y = vec![3.0, 2.0].into(),
                1 => changed.waveforms[1].y = vec![0.1].into(),
                2 => changed.waveforms[0].name = "unrelated".into(),
                3 => changed.waveforms[1].name = "OPT_OTHER".into(),
                4 => changed.family_metadata = None,
                5 => changed.success = false,
                _ => changed.analysis_type = AnalysisType::Transient,
            }
            assert!(located.view(&changed).is_none(), "mutation {mutation}");
        }
    }
}
