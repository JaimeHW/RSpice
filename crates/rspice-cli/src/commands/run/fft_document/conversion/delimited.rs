//! Read the row-oriented FFT schema without flattening metadata or harmonics.
use super::*;
use crate::commands::waveform_io::{
    conversion_error, enforce_resource_limit, parse_delimited_record,
};

struct Fields(Vec<String>);

impl Fields {
    fn text(&self, name: &str) -> &str {
        &self.0[FFT_DELIMITED_HEADER
            .iter()
            .position(|field| *field == name)
            .expect("FFT schema field")]
    }
    fn string(&self, name: &str) -> String {
        self.text(name).to_owned()
    }
    fn number<T: std::str::FromStr>(&self, name: &str) -> Result<T, String> {
        self.text(name)
            .parse()
            .map_err(|_| format!("invalid FFT {name}: '{}'", self.text(name)))
    }
    fn optional<T: std::str::FromStr>(&self, name: &str) -> Result<Option<T>, String> {
        if self.text(name).is_empty() {
            Ok(None)
        } else {
            self.number(name).map(Some)
        }
    }
    fn coordinate(&self) -> Result<Option<FftRawCoordinate>, String> {
        if self.0[5..9].iter().all(String::is_empty) {
            return Ok(None);
        }
        Ok(Some(FftRawCoordinate {
            coordinate_id: self.string("coordinate_id"),
            ordinal: self.number("coordinate_ordinal")?,
            tag: self.string("coordinate_tag"),
            assignment: self.string("coordinate_assignment"),
        }))
    }
    fn metadata(&self) -> Result<FftRawMetadataResult, String> {
        let status = match self.text("status") {
            "complete" if self.0[42..44].iter().all(String::is_empty) => {
                rspice_core::engine::TransientFftStatus::Complete
            }
            "incomplete-history" => rspice_core::engine::TransientFftStatus::IncompleteHistory {
                available_start: self.number("available_start_s")?,
                available_stop: self.number("available_stop_s")?,
            },
            _ => return Err("invalid FFT status or history bounds".into()),
        };
        let unit = (!self.text("value_unit").is_empty()).then(|| self.string("value_unit"));
        let metrics = if self.0[32..41].iter().all(String::is_empty) {
            None
        } else {
            Some(FftRawMetrics {
                units: FftRawMetricUnits {
                    fundamental_magnitude: unit.clone(),
                    thd_ratio: "1".into(),
                    thd_db: "dB".into(),
                    sndr_db: "dB".into(),
                    enob_bits: "bit".into(),
                    snr_db: "dB".into(),
                    sfdr_db: "dB".into(),
                    sfdr_spur_frequency: "Hz".into(),
                },
                fundamental_magnitude: self.number("fundamental_magnitude")?,
                thd_ratio: self.number("thd_ratio")?,
                thd_db: self.number("thd_db")?,
                sndr_db: self.number("sndr_db")?,
                enob_bits: self.number("enob_bits")?,
                snr_db: self.number("snr_db")?,
                sfdr_db: self.number("sfdr_db")?,
                sfdr_spur_bin: self.optional("sfdr_spur_bin")?,
                sfdr_spur_frequency_hz: self.optional("sfdr_spur_frequency_hz")?,
                largest_harmonics: Vec::new(),
            })
        };
        Ok(FftRawMetadataResult {
            status,
            analysis_id: self.string("analysis_id"),
            parent_analysis_id: self.string("parent_analysis_id"),
            ordinal: self.number("fft_ordinal")?,
            source: FftRawSource {
                kind: self.string("source_kind"),
                text: self.string("source_text"),
                authored_output: self.string("authored_output"),
            },
            signal: FftRawSignal {
                name: self.string("output_name"),
                physical_type: self.string("physical_type"),
                unit,
            },
            sampling: FftRawSampling {
                start_time_s: self.number("start_time_s")?,
                stop_time_s: self.number("stop_time_s")?,
                sample_interval_s: self.number("sample_interval_s")?,
                point_count: self.number("point_count")?,
                accurate_sampling: self.number("accurate_sampling")?,
            },
            transform: FftRawTransform {
                format: self.string("format"),
                mode: self.string("mode"),
                window: self.string("window"),
                window_name: self.string("window_name"),
                alpha: self.number("alpha")?,
                coherent_gain: self.number("coherent_gain")?,
                frequency_resolution_hz: self.number("frequency_resolution_hz")?,
                fundamental_bin: self.number("fundamental_bin")?,
                minimum_metric_bin: self.number("minimum_metric_bin")?,
                maximum_metric_bin: self.number("maximum_metric_bin")?,
                sfdr_search_minimum_bin: self.number("sfdr_search_minimum_bin")?,
            },
            metrics,
        })
    }
}

/// Preserve embedded newlines in quoted source expressions and assignments.
fn records(content: &str) -> impl Iterator<Item = &str> {
    let mut start = 0;
    let mut quoted = false;
    let mut offsets = content.char_indices();
    std::iter::from_fn(move || {
        for (index, ch) in offsets.by_ref() {
            if ch == '"' {
                quoted = !quoted;
            }
            if ch == '\n' && !quoted {
                let record = &content[start..index];
                start = index + 1;
                return Some(record.trim_end_matches('\r'));
            }
        }
        if start < content.len() {
            let record = &content[start..];
            start = content.len();
            Some(record)
        } else {
            None
        }
    })
    .filter(|record| !record.trim().is_empty())
}

impl FftBundle {
    pub(crate) fn is_delimited(header: &[String]) -> bool {
        // A damaged typed header still takes the typed decoder and is refused.
        header
            .first()
            .is_some_and(|field| field == "schema_version")
            && header.get(1).is_some_and(|field| field == "analysis")
    }

    pub(crate) fn from_delimited(
        path: &Path,
        content: &str,
        separator: char,
        limits: rspice_core::ResourceLimits,
    ) -> Result<Self, CliError> {
        let err = |message| conversion_error(path, message);
        let mut rows = records(content);
        let header = parse_delimited_record(
            rows.next().ok_or_else(|| err("empty FFT table".into()))?,
            separator,
        )
        .map_err(err)?;
        if header.iter().map(String::as_str).ne(FFT_DELIMITED_HEADER) {
            return Err(err("invalid FFT table header".into()));
        }
        let mut results: Vec<FftRawMetadataResult> = Vec::new();
        let mut bins = Vec::new();
        let mut common: Option<Vec<String>> = None;
        let mut parent = None;
        let mut coordinate = None;
        let mut unavailable = false;
        let mut numeric_values = 0usize;
        for row in rows {
            let fields = Fields(parse_delimited_record(row, separator).map_err(err)?);
            if fields.0.len() != FFT_DELIMITED_HEADER.len() {
                return Err(err("invalid FFT table row length".into()));
            }
            numeric_values = numeric_values.saturating_add(
                fields
                    .0
                    .iter()
                    .filter(|field| field.parse::<f64>().is_ok())
                    .count(),
            );
            enforce_resource_limit(
                path,
                rspice_core::ResourceKind::ExternalDataValues,
                numeric_values,
                limits.max_external_data_values,
            )?;
            enforce_resource_limit(
                path,
                rspice_core::ResourceKind::ResultValues,
                numeric_values,
                limits.max_result_values,
            )?;
            if fields.number::<u32>("schema_version").map_err(err)? != FFT_ARTIFACT_SCHEMA_VERSION
                || fields.text("analysis") != "fft"
                || fields.text("artifact_format") != if separator == ',' { "csv" } else { "tsv" }
            {
                return Err(err("invalid FFT table envelope".into()));
            }
            let row_coordinate = fields.coordinate().map_err(err)?;
            if let Some(parent) = &parent {
                if parent != fields.text("parent_analysis_id") || coordinate != row_coordinate {
                    return Err(err("FFT parent or coordinate changes between rows".into()));
                }
            } else {
                parent = Some(fields.string("parent_analysis_id"));
                coordinate = row_coordinate;
            }
            if common
                .as_ref()
                .is_none_or(|previous| previous[3] != fields.text("analysis_id"))
            {
                results.push(fields.metadata().map_err(err)?);
                common = Some(fields.0[..44].to_vec());
                unavailable = false;
            } else if common.as_deref() != Some(&fields.0[..44]) {
                return Err(err(
                    "FFT metadata changes between records for the same result".into(),
                ));
            }
            let result = results.last_mut().expect("row established FFT result");
            match fields.text("record_kind") {
                "bin"
                    if result.status.is_complete()
                        && fields.0[51..].iter().all(String::is_empty) =>
                {
                    bins.push(DecodedFftRawBin {
                        analysis_id: result.analysis_id.clone(),
                        index: fields.number("bin_index").map_err(err)?,
                        frequency_hz: fields.number("frequency_hz").map_err(err)?,
                        real: fields.number("real").map_err(err)?,
                        imaginary: fields.number("imaginary").map_err(err)?,
                        magnitude: fields.number("magnitude").map_err(err)?,
                        phase_degrees: fields.number("phase_degrees").map_err(err)?,
                    });
                }
                "largest_harmonic"
                    if result.status.is_complete()
                        && fields.0[45..51].iter().all(String::is_empty) =>
                {
                    let metrics = result
                        .metrics
                        .as_mut()
                        .ok_or_else(|| err("FFT harmonic without metrics".into()))?;
                    metrics.largest_harmonics.push(FftRawHarmonic {
                        rank: fields.number("harmonic_rank").map_err(err)?,
                        bin: fields.number("harmonic_bin").map_err(err)?,
                        frequency_hz: fields.number("harmonic_frequency_hz").map_err(err)?,
                        magnitude: fields.number("harmonic_magnitude").map_err(err)?,
                        magnitude_db: fields.number("harmonic_magnitude_db").map_err(err)?,
                        phase_degrees: fields.number("harmonic_phase_degrees").map_err(err)?,
                    });
                }
                "unavailable"
                    if !result.status.is_complete()
                        && !unavailable
                        && fields.0[45..].iter().all(String::is_empty) =>
                {
                    unavailable = true
                }
                _ => {
                    return Err(err(
                        "invalid FFT record kind, status, or unused fields".into()
                    ));
                }
            }
        }
        Self::from_metadata(
            readback::metadata_envelope(parent.unwrap_or_default(), coordinate, results),
            bins,
        )
        .map_err(err)
    }
}
