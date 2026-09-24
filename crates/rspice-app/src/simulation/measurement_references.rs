//! Immutable comparison-table inputs shared by preparation and workers.

use crate::product::ContentDigest;
use crate::state::SpecificationDefinition;
use crate::state::workspace::MeasurementReferenceSource;
use rspice_core::netlist::measure::MeasureType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedReference {
    measurement: String,
    source: MeasurementReferenceSource,
    digest: ContentDigest,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PreparedMeasurementReferences {
    entries: Vec<PreparedReference>,
}

impl PreparedMeasurementReferences {
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn capture(
        source: &str,
        definitions: &[SpecificationDefinition],
    ) -> Result<Self, String> {
        if !may_reference_file(source) {
            return Ok(Self::default());
        }
        let mut netlist = parse(source)?;
        let mut entries = Vec::new();
        for measurement in &netlist.measurements {
            let MeasureType::FileError { file, .. } = &measurement.measure_type else {
                continue;
            };
            let candidates = definitions
                .iter()
                .filter(|definition| {
                    definition
                        .measurement
                        .eq_ignore_ascii_case(&measurement.name)
                })
                .filter_map(|definition| definition.measurement_reference.as_ref())
                .collect::<Vec<_>>();
            let [reference] = candidates.as_slice() else {
                return Err(format!(
                    "Measurement '{}' requires one imported reference table for FILE={}; attach it in Specs",
                    measurement.name,
                    file.path()
                ));
            };
            if reference.logical_path != file.path() {
                return Err(format!(
                    "Measurement '{}' expects FILE={}, but its imported reference is named {}",
                    measurement.name,
                    file.path(),
                    reference.logical_path
                ));
            }
            entries.push(PreparedReference {
                measurement: measurement.name.clone(),
                source: (*reference).clone(),
                digest: reference_digest(reference),
            });
        }
        let prepared = Self { entries };
        prepared.bind(&mut netlist)?;
        Ok(prepared)
    }

    pub(crate) fn validate_source(&self, source: &str) -> Result<(), String> {
        if self.is_empty() && !may_reference_file(source) {
            return Ok(());
        }
        self.bind(&mut parse(source)?)
    }

    pub(crate) fn bind(&self, netlist: &mut rspice_core::Netlist) -> Result<(), String> {
        let mut names = HashSet::new();
        let mut bytes = 0usize;
        for entry in &self.entries {
            entry.source.validate()?;
            if entry.digest != reference_digest(&entry.source) {
                return Err("Measurement reference content digest mismatch".into());
            }
            if !names.insert(entry.measurement.to_ascii_lowercase()) {
                return Err("Duplicate prepared measurement reference".into());
            }
            bytes = bytes
                .checked_add(entry.source.contents.len())
                .ok_or("Measurement reference size overflow")?;
        }
        if bytes > rspice_core::ResourceLimits::default().max_netlist_bytes {
            return Err("Combined measurement references exceed the input size limit".into());
        }
        let mut used = HashSet::new();
        for measurement in &mut netlist.measurements {
            let MeasureType::FileError { file, .. } = &measurement.measure_type else {
                continue;
            };
            let entry = self
                .entries
                .iter()
                .find(|entry| entry.measurement.eq_ignore_ascii_case(&measurement.name))
                .ok_or_else(|| {
                    format!(
                        "Measurement '{}' has an unsealed file-backed measurement reference: {}",
                        measurement.name,
                        file.path()
                    )
                })?;
            if entry.source.logical_path != file.path() {
                return Err(format!(
                    "Measurement '{}' reference path changed after preparation",
                    measurement.name
                ));
            }
            rspice_core::analysis::bind_error_measurement_reference(
                measurement,
                entry.source.contents.as_str(),
            )
            .map_err(|error| format!("Measurement '{}': {error}", measurement.name))?;
            used.insert(entry.measurement.to_ascii_lowercase());
        }
        if used != names {
            return Err("Prepared comparison data has no matching ERROR measurement".into());
        }
        Ok(())
    }

    pub(crate) fn digest(&self) -> ContentDigest {
        crate::simulation::execution::content_digest(
            "rspice.prepared-measurement-references/v1",
            &serde_json::to_vec(&self.entries).expect("reference entries serialize"),
        )
    }
}

pub(in crate::simulation) fn reference_digest(
    reference: &MeasurementReferenceSource,
) -> ContentDigest {
    crate::simulation::execution::content_digest(
        "rspice.measurement-reference/v1",
        &serde_json::to_vec(reference).expect("reference text serializes"),
    )
}

fn parse(source: &str) -> Result<rspice_core::Netlist, String> {
    rspice_core::Netlist::parse_with_options(
        source,
        rspice_core::netlist::NetlistParseOptions {
            statistical_mode: rspice_core::netlist::StatisticalParamMode::Nominal,
            ..Default::default()
        },
    )
    .map_err(|error| format!("Cannot bind measurement reference data: {error}"))
}

fn may_reference_file(source: &str) -> bool {
    source
        .as_bytes()
        .windows(4)
        .any(|token| token.eq_ignore_ascii_case(b"file"))
}
