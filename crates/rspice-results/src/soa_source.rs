//! Full SOA observations retained independently of the requested reporting grid.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaSourceHistory<S = Vec<f64>> {
    pub time: S,
    pub waveforms: Vec<SoaSourceWaveform<S>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaSourceWaveform<S = Vec<f64>> {
    pub name: String,
    pub unit: String,
    pub values: S,
}

impl<S> SoaSourceHistory<S> {
    pub fn map_series<T, E>(
        self,
        mut map: impl FnMut(S) -> Result<T, E>,
    ) -> Result<SoaSourceHistory<T>, E> {
        Ok(SoaSourceHistory {
            time: map(self.time)?,
            waveforms: self
                .waveforms
                .into_iter()
                .map(|wave| {
                    Ok(SoaSourceWaveform {
                        name: wave.name,
                        unit: wave.unit,
                        values: map(wave.values)?,
                    })
                })
                .collect::<Result<_, E>>()?,
        })
    }
}

impl SoaSourceHistory {
    pub fn value_count(&self) -> usize {
        self.waveforms.iter().fold(self.time.len(), |count, wave| {
            count.saturating_add(wave.values.len())
        })
    }

    /// Count histories are right-continuous steps; continuous stress channels
    /// use the same linear interpolation as other transient reporting views.
    pub fn report_values(
        &self,
        wave: &SoaSourceWaveform,
        projection: &rspice_core::analysis::transient::TransientOutputProjection,
    ) -> Result<Vec<f64>, String> {
        if wave.values.len() != self.time.len() {
            return Err("SOA source trace has incomplete sample coverage".into());
        }
        if wave.name == "SOA_VIOLATION_COUNT" {
            projection
                .times()
                .iter()
                .map(|time| {
                    let index = self.time.partition_point(|sample| sample <= time);
                    index
                        .checked_sub(1)
                        .map(|index| wave.values[index])
                        .ok_or_else(|| "SOA reporting time precedes its observations".into())
                })
                .collect()
        } else {
            projection.project(&wave.values)
        }
    }

    pub fn validate_report_columns<'a>(
        &self,
        reporting_time: &[f64],
        waves: impl ExactSizeIterator<Item = (&'a str, &'a [f64], &'a [f64], Option<&'a str>, bool)>,
    ) -> Result<(), String> {
        if self.time.is_empty()
            || self.waveforms.is_empty()
            || self.time.iter().any(|t| !t.is_finite() || *t < 0.0)
            || self.time.windows(2).any(|pair| pair[1] <= pair[0])
            || self.waveforms.len() != waves.len()
        {
            return Err(
                "SOA source history has an invalid observation axis or channel count".into(),
            );
        }
        let projection =
            rspice_core::analysis::transient::TransientOutputProjection::interpolate_times(
                &self.time,
                reporting_time,
                reporting_time.len(),
            )?;
        if reporting_time.last() != self.time.last() {
            return Err("SOA reporting view must include the final observation".into());
        }
        let mut sources = BTreeMap::new();
        for source in &self.waveforms {
            if source.name.is_empty()
                || source.unit.is_empty()
                || sources.insert(source.name.as_str(), source).is_some()
                || source.values.len() != self.time.len()
                || source.values.iter().any(|v| !v.is_finite() || *v < 0.0)
                || (source.name == "SOA_VIOLATION_COUNT"
                    && (source.unit != "count"
                        || source.values.iter().any(|v| v.fract() != 0.0)
                        || source.values.windows(2).any(|pair| pair[1] < pair[0])))
            {
                return Err(
                    "SOA source history has invalid channel identities, units or samples".into(),
                );
            }
        }
        for (name, x, y, unit, complex) in waves {
            let source = sources
                .remove(name)
                .ok_or("SOA reporting view has duplicate or unknown channels")?;
            if unit != Some(source.unit.as_str())
                || complex
                || x != reporting_time
                || y != self.report_values(source, &projection)?
            {
                return Err(format!(
                    "SOA reporting trace '{name}' contradicts its complete source history"
                ));
            }
        }
        Ok(())
    }
}
