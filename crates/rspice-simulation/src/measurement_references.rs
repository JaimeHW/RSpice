//! Immutable comparison-table inputs shared by preparation and workers.

use rspice_core::netlist::measure::MeasureType;
use rspice_results::specification::{MeasurementReferenceSource, SpecificationDefinition};
use rspice_vocabulary::product::ContentDigest;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedReference {
    measurement: String,
    source: MeasurementReferenceSource,
    digest: ContentDigest,
}

/// Captured reference tables carried with prepared execution inputs.
///
/// Deserialized entries are checked by `validate_source` or `bind` before use.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedMeasurementReferences {
    entries: Vec<PreparedReference>,
}

impl PreparedMeasurementReferences {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The reference tables `source`'s measurements read, the deck read
    /// under `limits`, the limits its run executes under.
    pub fn capture(
        source: &str,
        definitions: &[SpecificationDefinition],
        limits: rspice_core::ResourceLimits,
    ) -> Result<Self, String> {
        if !may_reference_file(source) {
            return Ok(Self::default());
        }
        let mut netlist = parse(source, limits)?;
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

    /// [`Self::capture`] for `source` as its run's preparation parsed it
    /// (`parsed`): a deck that reads no table there is neither scanned nor
    /// parsed again.
    pub fn capture_of_parsed(
        source: &str,
        parsed: &rspice_core::Netlist,
        definitions: &[SpecificationDefinition],
        limits: rspice_core::ResourceLimits,
    ) -> Result<Self, String> {
        if reads_no_table(parsed) {
            return Ok(Self::default());
        }
        Self::capture(source, definitions, limits)
    }

    /// [`Self::validate_source`] for `source` as its run's preparation
    /// parsed it (`parsed`): no tables, and a deck that reads none there, is
    /// neither scanned nor parsed again.
    pub fn validate_parsed(
        &self,
        source: &str,
        parsed: &rspice_core::Netlist,
        limits: rspice_core::ResourceLimits,
    ) -> Result<(), String> {
        if self.is_empty() && reads_no_table(parsed) {
            return Ok(());
        }
        self.validate_source(source, limits)
    }

    /// Whether `source`, read under `limits`, binds these tables.
    pub fn validate_source(
        &self,
        source: &str,
        limits: rspice_core::ResourceLimits,
    ) -> Result<(), String> {
        if self.is_empty() && !may_reference_file(source) {
            return Ok(());
        }
        self.bind(&mut parse(source, limits)?)
    }

    pub fn bind(&self, netlist: &mut rspice_core::Netlist) -> Result<(), String> {
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
                        "Measurement '{}' reads a reference file that is not kept with the run: {}",
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

    pub fn digest(&self) -> ContentDigest {
        rspice_vocabulary::canonical::content_digest(
            "rspice.prepared-measurement-references/v1",
            &serde_json::to_vec(&self.entries).expect("reference entries serialize"),
        )
    }
}

pub fn reference_digest(reference: &MeasurementReferenceSource) -> ContentDigest {
    rspice_vocabulary::canonical::content_digest(
        "rspice.measurement-reference/v1",
        &serde_json::to_vec(reference).expect("reference text serializes"),
    )
}

fn parse(
    source: &str,
    limits: rspice_core::ResourceLimits,
) -> Result<rspice_core::Netlist, String> {
    rspice_core::Netlist::parse_with_options(
        source,
        rspice_core::netlist::NetlistParseOptions {
            statistical_mode: rspice_core::netlist::StatisticalParamMode::Nominal,
            resource_limits: limits,
            ..Default::default()
        },
    )
    .map_err(|error| format!("Cannot bind measurement reference data: {error}"))
}

/// Whether `parsed`, a deck as its run's preparation parsed it, reads no
/// reference table however its statistics are read: none of its
/// measurements reads a file, and it skips no line that a reading at
/// nominal values, as these tables are bound, could read instead.
fn reads_no_table(parsed: &rspice_core::Netlist) -> bool {
    parsed.unread_root_source().next().is_none()
        && !parsed
            .measurements
            .iter()
            .any(|measurement| matches!(measurement.measure_type, MeasureType::FileError { .. }))
}

fn may_reference_file(source: &str) -> bool {
    source
        .as_bytes()
        .windows(4)
        .any(|token| token.eq_ignore_ascii_case(b"file"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed(deck: &str) -> rspice_core::Netlist {
        rspice_core::Netlist::parse(deck).expect("the deck parses")
    }

    /// A deck its parse shows reads no table is answered from the parse, a
    /// comment that names a file among it; one whose measurement reads a
    /// file is captured and checked as the deck itself is.
    #[test]
    fn the_runs_parse_answers_for_a_deck_that_reads_no_table() {
        let limits = rspice_core::ResourceLimits::default();
        let commented = "deck\n* the file this deck was written from\nV1 out 0 1\nR1 out 0 1k\n.tran 1n 10n\n.end\n";
        let prepared = PreparedMeasurementReferences::capture_of_parsed(
            commented,
            &parsed(commented),
            &[],
            limits,
        )
        .expect("a deck that reads no table captures none");
        assert!(prepared.is_empty());
        prepared
            .validate_parsed(commented, &parsed(commented), limits)
            .expect("and binds none");

        let reads = "deck\nV1 out 0 1\nR1 out 0 1k\n.tran 1n 10n\n.measure tran fit ERROR V(out) FILE reference.prn DEPVARCOL 2\n.end\n";
        let error =
            PreparedMeasurementReferences::capture_of_parsed(reads, &parsed(reads), &[], limits)
                .expect_err("a measurement that reads a file needs its table");
        assert!(
            error.contains("requires one imported reference table"),
            "{error}"
        );
        let error = PreparedMeasurementReferences::default()
            .validate_parsed(reads, &parsed(reads), limits)
            .expect_err("a deck that reads a table not kept is refused");
        assert!(error.contains("not kept with the run"), "{error}");
    }
}
