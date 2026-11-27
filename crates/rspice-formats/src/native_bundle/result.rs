//! Borrowed native-bundle projections from selected retained waveforms.

use std::cell::Cell;
use std::io::Write;

use serde::ser::{Error as _, SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use sha2::Digest as _;

use super::{
    NativeBundleDataset, NativeBundleError, NativeBundleKind, NativeBundleMember,
    NativeBundleSignal, NativeBundleSignalValues, NativeBundleStepValue,
};
use crate::WaveformDomain;
use crate::waveform_io::result::{
    ExportMember, WaveformExportError, WaveformProjectionError, axis_signal_for_analysis,
    export_samples, validate_shared_x_axis,
};
use crate::zip::StreamedZip;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::provenance::{AnalysisResultFamilyMember, AnalysisResultStepCoordinate};
use rspice_results::waveform::{RetainedWaveform, WaveformValues};

impl NativeBundleMember {
    /// The bundle's record of a result's family member.
    #[must_use]
    pub fn of(member: &AnalysisResultFamilyMember) -> Self {
        match member {
            AnalysisResultFamilyMember::MonteCarloTrial { index } => {
                Self::MonteCarloTrial { index: *index }
            }
            AnalysisResultFamilyMember::StepPoint { index, coordinate } => Self::StepPoint {
                index: *index,
                coordinate: coordinate
                    .iter()
                    .map(|axis| NativeBundleStepValue {
                        name: axis.name.clone(),
                        value: axis.value,
                        unit: axis.unit.clone(),
                    })
                    .collect(),
            },
        }
    }

    /// The family member a result read from the bundle is.
    #[must_use]
    pub fn into_family_member(self) -> AnalysisResultFamilyMember {
        match self {
            Self::MonteCarloTrial { index } => {
                AnalysisResultFamilyMember::MonteCarloTrial { index }
            }
            Self::StepPoint { index, coordinate } => AnalysisResultFamilyMember::StepPoint {
                index,
                coordinate: coordinate
                    .into_iter()
                    .map(|axis| AnalysisResultStepCoordinate {
                        name: axis.name,
                        value: axis.value,
                        unit: axis.unit,
                    })
                    .collect(),
            },
        }
    }
}

/// Project selected traces for a domain already admitted by the caller.
pub fn project_native_bundle<'a, W: AsRef<RetainedWaveform>>(
    analysis: &'a AnalysisResult<W>,
    waveforms: &[&'a W],
    native_analysis: WaveformDomain,
) -> Result<NativeBundleDataset<'a>, WaveformProjectionError> {
    let reference = waveforms
        .iter()
        .map(|waveform| (*waveform).as_ref())
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or(WaveformProjectionError::NoSamples)?;
    validate_shared_x_axis(waveforms, &reference.x)?;
    let whole = |signal: &str, values: &'a rspice_results::waveform::WaveformValues| {
        export_samples(signal, values).map_err(|error| match error {
            WaveformExportError::Unreadable { signal, reason } => {
                WaveformProjectionError::Unreadable { signal, reason }
            }
            _ => WaveformProjectionError::NoSamples,
        })
    };
    let coordinate = whole(&reference.name, &reference.x)?;
    let (coordinate_name, _) = axis_signal_for_analysis(analysis);
    let signals = waveforms
        .iter()
        .map(|waveform| {
            let waveform = (*waveform).as_ref();
            Ok(if let Some(complex) = &waveform.complex {
                NativeBundleSignal {
                    name: &complex.source_name,
                    unit: waveform.unit.as_deref(),
                    values: NativeBundleSignalValues::Complex {
                        real: whole(&complex.source_name, &complex.real)?,
                        imag: whole(&complex.source_name, &complex.imag)?,
                    },
                }
            } else {
                NativeBundleSignal {
                    name: &waveform.name,
                    unit: waveform.unit.as_deref(),
                    values: NativeBundleSignalValues::Real(whole(&waveform.name, &waveform.y)?),
                }
            })
        })
        .collect::<Result<_, WaveformProjectionError>>()?;
    Ok(NativeBundleDataset {
        analysis: native_analysis,
        coordinate_name,
        coordinate_unit: analysis.waveform_coordinate_unit(),
        coordinate,
        signals,
        member: analysis.family_member().map(NativeBundleMember::of),
    })
}

/// The section a retained analysis publishes under in a bundle, when it has one.
#[must_use]
pub const fn native_domain(analysis: AnalysisType) -> Option<WaveformDomain> {
    match analysis {
        AnalysisType::Transient => Some(WaveformDomain::Transient),
        AnalysisType::Ac => Some(WaveformDomain::Ac),
        AnalysisType::DcSweep => Some(WaveformDomain::DcSweep),
        _ => None,
    }
}

/// What a written bundle holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeBundleFacts {
    /// Signals of the first member.
    pub signals: usize,
    /// Points of the first member.
    pub points: usize,
    /// The datasets the bundle holds: one per member of a family.
    pub members: usize,
}

/// The archive member the `position`th dataset of a bundle is written as: the first is the
/// `dataset.json` every reader opens, each further member of a family one of its own.
fn dataset_member(position: usize) -> String {
    if position == 0 {
        "dataset.json".to_owned()
    } else {
        format!("members/{:04}.json", position + 1)
    }
}

/// The rows, signal names and reference coordinate of one member, checked against what a
/// reader of the bundle accepts without reading a sample.
fn checked_member<'a, W: AsRef<RetainedWaveform>>(
    member: &ExportMember<'a, W>,
) -> Result<(WaveformDomain, &'a RetainedWaveform), NativeBundleError> {
    let analysis = member.analysis;
    let domain = native_domain(analysis.analysis_type).ok_or_else(|| {
        NativeBundleError::InvalidData(format!(
            "a bundle carries transient, AC and DC sweep results; '{}' is none of those",
            analysis.label
        ))
    })?;
    let reference = member
        .waveforms
        .iter()
        .map(|waveform| (*waveform).as_ref())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or_else(|| NativeBundleError::InvalidData("no waveform samples to export".into()))?;
    let rows = reference.x.len();
    if !(2..=MAX_BUNDLE_ROWS).contains(&rows) {
        return Err(NativeBundleError::InvalidData(format!(
            "native bundle coordinate has {rows} samples; supported range is 2..={MAX_BUNDLE_ROWS}"
        )));
    }
    if member.waveforms.len() + 1 > MAX_BUNDLE_COLUMNS {
        return Err(NativeBundleError::InvalidData(format!(
            "native bundle has {} signals; supported range is 1..={}",
            member.waveforms.len(),
            MAX_BUNDLE_COLUMNS - 1
        )));
    }
    let mut identities = std::collections::HashSet::new();
    for waveform in &member.waveforms {
        let waveform = (*waveform).as_ref();
        let name = waveform
            .complex
            .as_ref()
            .map_or(waveform.name.as_str(), |complex| &complex.source_name);
        if name.trim().is_empty()
            || name.len() > MAX_BUNDLE_NAME_BYTES
            || name.chars().any(char::is_control)
        {
            return Err(NativeBundleError::InvalidData(
                "native bundle signal identity is empty or invalid".into(),
            ));
        }
        if !identities.insert(name.to_lowercase()) {
            return Err(NativeBundleError::InvalidData(format!(
                "native bundle repeats signal identity '{name}'"
            )));
        }
        if waveform.unit.as_deref().is_some_and(|unit| {
            unit.len() > MAX_BUNDLE_NAME_BYTES || unit.chars().any(char::is_control)
        }) {
            return Err(NativeBundleError::InvalidData(format!(
                "native bundle signal '{name}' has an invalid unit"
            )));
        }
        if waveform.x != reference.x {
            return Err(NativeBundleError::InvalidData(format!(
                "every signal of a bundle shares one coordinate; '{}' carries its own",
                waveform.name
            )));
        }
        let lengths = std::iter::once(waveform.y.len()).chain(
            waveform
                .complex
                .iter()
                .flat_map(|complex| [complex.real.len(), complex.imag.len()]),
        );
        for length in lengths {
            if length != rows {
                return Err(NativeBundleError::InvalidData(format!(
                    "native bundle signal '{name}' has {length} samples; expected {rows}"
                )));
            }
        }
    }
    if let Some(member) = analysis.family_member() {
        NativeBundleMember::of(member)
            .validate()
            .map_err(NativeBundleError::InvalidData)?;
    }
    Ok((domain, reference))
}

/// Whether `members` make a bundle, checked without reading a sample: a refusal here comes
/// before the reader is asked where the file goes. A sample a reader would refuse (a coordinate
/// that repeats or turns back, an infinite value) refuses the write when it is read.
pub fn check_native_bundle<W: AsRef<RetainedWaveform>>(
    members: &[ExportMember<'_, W>],
) -> Result<NativeBundleFacts, NativeBundleError> {
    let mut facts = None;
    for member in members {
        let (_, reference) = checked_member(member)?;
        facts.get_or_insert(NativeBundleFacts {
            signals: member.waveforms.len(),
            points: reference.x.len(),
            members: members.len(),
        });
    }
    facts.ok_or_else(|| NativeBundleError::InvalidData("no waveform samples to export".into()))
}

/// `members` as a bundle of `kind` written to `out` as their samples are read: each member's
/// dataset is serialised a block of each column at a time and its digest taken as it is
/// written, and the manifest that binds every digest comes last. One analysis is the bundle's
/// `dataset.json`; every member of a family is a dataset of its own, naming the member it is,
/// in family order. A sample that cannot be read, or that a reader would refuse, stops the
/// write; what reached `out` is then a partial file its caller discards.
pub fn write_native_bundle<W: AsRef<RetainedWaveform>>(
    kind: NativeBundleKind,
    members: &[ExportMember<'_, W>],
    out: &mut dyn Write,
) -> Result<NativeBundleFacts, NativeBundleError> {
    let facts = check_native_bundle(members)?;
    let io = |context: &str| {
        let context = context.to_owned();
        move |source| NativeBundleError::Io { context, source }
    };
    let mut archive = StreamedZip::new(out);
    let mut digests = Vec::with_capacity(members.len());
    for (position, member) in members.iter().enumerate() {
        let (domain, reference) = checked_member(member)?;
        let name = dataset_member(position);
        let values = member.waveforms.iter().fold(1_u64, |values, waveform| {
            values
                + if (*waveform).as_ref().complex.is_some() {
                    2
                } else {
                    1
                }
        });
        // A sample is at most 24 characters and its separator.
        let bytes = (reference.x.len() as u64)
            .saturating_mul(values)
            .saturating_mul(26)
            .saturating_add(1 << 16);
        archive
            .start(&name, bytes)
            .map_err(io("could not begin a bundle dataset"))?;
        let mut hashed = Hashed {
            out: &mut archive,
            digest: sha2::Sha256::new(),
        };
        let mut buffered = std::io::BufWriter::with_capacity(1 << 16, &mut hashed);
        let unavailable = Cell::new(false);
        let dataset = StreamedDataset {
            member,
            domain,
            reference,
            unavailable: &unavailable,
        };
        serde_json::to_writer(&mut buffered, &dataset).map_err(|error| {
            if error.is_io() {
                NativeBundleError::Io {
                    context: format!("could not write {name}"),
                    source: error.into(),
                }
            } else {
                NativeBundleError::InvalidData(error.to_string())
            }
        })?;
        buffered
            .flush()
            .map_err(io("could not write a bundle dataset"))?;
        drop(buffered);
        digests.push((name, format!("{:x}", hashed.digest.finalize())));
    }
    let mut digests = digests.into_iter();
    let (_, first) = digests.next().expect("a bundle of one member or more");
    let manifest = serde_json::json!({
        "schema": kind.manifest_schema(),
        "dataset_member": "dataset.json",
        "dataset_sha256": first,
    });
    let mut manifest = manifest.as_object().cloned().unwrap_or_default();
    let family = digests
        .map(|(name, digest)| serde_json::json!({ "dataset_member": name, "dataset_sha256": digest }))
        .collect::<Vec<_>>();
    if !family.is_empty() {
        manifest.insert("family".to_owned(), serde_json::Value::Array(family));
    }
    let manifest = serde_json::to_vec(&manifest).map_err(|source| NativeBundleError::Json {
        context: "native manifest serialization failed",
        source,
    })?;
    archive
        .start("manifest.json", manifest.len() as u64)
        .and_then(|()| archive.write_all(&manifest))
        .map_err(io("could not write the bundle manifest"))?;
    archive
        .finish()
        .map_err(io("could not finish the bundle"))?;
    Ok(facts)
}

/// The most rows, columns (the coordinate one of them) and bytes in a name a bundle's reader
/// takes.
const MAX_BUNDLE_ROWS: usize = 1_000_000;
const MAX_BUNDLE_COLUMNS: usize = 1_024;
const MAX_BUNDLE_NAME_BYTES: usize = 1_024;

/// A writer that digests what it writes.
struct Hashed<'a, 'w> {
    out: &'a mut StreamedZip<'w>,
    digest: sha2::Sha256,
}

impl Write for Hashed<'_, '_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let written = self.out.write(bytes)?;
        self.digest.update(&bytes[..written]);
        Ok(written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}

/// One member's `dataset.json`, serialised as its samples are read. Its schema comes last: a
/// sample that is unavailable needs the version that says so, which is known once the samples
/// are written.
struct StreamedDataset<'a, 'm, W> {
    member: &'a ExportMember<'m, W>,
    domain: WaveformDomain,
    reference: &'a RetainedWaveform,
    unavailable: &'a Cell<bool>,
}

impl<W: AsRef<RetainedWaveform>> Serialize for StreamedDataset<'_, '_, W> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let analysis = self.member.analysis;
        let (coordinate_name, _) = axis_signal_for_analysis(analysis);
        let coordinate_unit = analysis.waveform_coordinate_unit();
        let member = analysis.family_member().map(NativeBundleMember::of);
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("analysis", self.domain.schema_name())?;
        map.serialize_entry(
            "coordinate",
            &StreamedCoordinate {
                name: coordinate_name,
                unit: coordinate_unit,
                domain: self.domain,
                values: &self.reference.x,
            },
        )?;
        map.serialize_entry(
            "signals",
            &StreamedSignals {
                waveforms: &self.member.waveforms,
                unavailable: self.unavailable,
            },
        )?;
        if let Some(member) = &member {
            map.serialize_entry("member", member)?;
        }
        // Each schema adds one thing a reader of the one before refuses: a member, unavailable
        // samples, a coordinate unit.
        let schema = if member.is_some() {
            "rspice-waveform-dataset/4"
        } else if self.unavailable.get() {
            "rspice-waveform-dataset/3"
        } else if coordinate_unit.is_some() {
            "rspice-waveform-dataset/2"
        } else {
            "rspice-waveform-dataset/1"
        };
        map.serialize_entry("schema", schema)?;
        map.end()
    }
}

struct StreamedCoordinate<'a> {
    name: &'a str,
    unit: Option<&'a str>,
    domain: WaveformDomain,
    values: &'a WaveformValues,
}

impl Serialize for StreamedCoordinate<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", self.name)?;
        if let Some(unit) = self.unit {
            map.serialize_entry("unit", unit)?;
        }
        map.serialize_entry("values", &CoordinateValues(self))?;
        map.end()
    }
}

/// The coordinate's samples, checked as a reader checks them while they are written: finite,
/// never repeating, in one direction, and positive for a frequency.
struct CoordinateValues<'a, 'b>(&'a StreamedCoordinate<'b>);

impl Serialize for CoordinateValues<'_, '_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let coordinate = self.0;
        let mut sequence = serializer.serialize_seq(Some(coordinate.values.len()))?;
        let mut previous: Option<f64> = None;
        let mut direction = None;
        let mut index = 0_usize;
        for block in coordinate.values.blocks(..) {
            let block = block.map_err(S::Error::custom)?;
            if coordinate.unit.is_some() {
                // A reader converts the coordinate to its unit's base; publication must survive
                // that, as the in-memory writer checks.
                let mut converted = crate::numeric::DecodedNumericDataset {
                    domain: coordinate.domain,
                    coordinate_name: coordinate.name.to_owned(),
                    coordinate_unit: coordinate.unit.map(str::to_owned),
                    coordinate: block.to_vec(),
                    signals: Vec::new(),
                };
                converted
                    .normalize_coordinate_unit()
                    .map_err(S::Error::custom)?;
            }
            for &value in block.iter() {
                if !value.is_finite() {
                    return Err(S::Error::custom(format!(
                        "native bundle coordinate has a non-finite sample at index {index}"
                    )));
                }
                if coordinate.domain == WaveformDomain::Ac && value <= 0.0 {
                    return Err(S::Error::custom(
                        "native AC bundle requires positive frequency coordinates",
                    ));
                }
                if let Some(previous) = previous {
                    if value == previous {
                        return Err(S::Error::custom(format!(
                            "native bundle coordinate repeats at sample {index}"
                        )));
                    }
                    let step = value.total_cmp(&previous);
                    if *direction.get_or_insert(step) != step {
                        return Err(S::Error::custom(format!(
                            "native bundle coordinate reverses at sample {index}"
                        )));
                    }
                }
                previous = Some(value);
                sequence.serialize_element(&value)?;
                index += 1;
            }
        }
        sequence.end()
    }
}

struct StreamedSignals<'a, 'm, W> {
    waveforms: &'a [&'m W],
    unavailable: &'a Cell<bool>,
}

impl<W: AsRef<RetainedWaveform>> Serialize for StreamedSignals<'_, '_, W> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.waveforms.len()))?;
        for waveform in self.waveforms {
            sequence.serialize_element(&StreamedSignal {
                waveform: (*waveform).as_ref(),
                unavailable: self.unavailable,
            })?;
        }
        sequence.end()
    }
}

struct StreamedSignal<'a> {
    waveform: &'a RetainedWaveform,
    unavailable: &'a Cell<bool>,
}

impl Serialize for StreamedSignal<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let waveform = self.waveform;
        let mut map = serializer.serialize_map(None)?;
        match &waveform.complex {
            Some(complex) => {
                map.serialize_entry("name", &complex.source_name)?;
                if let Some(unit) = &waveform.unit {
                    map.serialize_entry("unit", unit)?;
                }
                map.serialize_entry(
                    "real",
                    &Samples {
                        signal: &complex.source_name,
                        component: "real",
                        values: &complex.real,
                        paired: None,
                        unavailable: Some(self.unavailable),
                    },
                )?;
                map.serialize_entry(
                    "imag",
                    &Samples {
                        signal: &complex.source_name,
                        component: "imag",
                        values: &complex.imag,
                        paired: Some(&complex.real),
                        unavailable: None,
                    },
                )?;
            }
            None => {
                map.serialize_entry("name", &waveform.name)?;
                if let Some(unit) = &waveform.unit {
                    map.serialize_entry("unit", unit)?;
                }
                map.serialize_entry(
                    "values",
                    &Samples {
                        signal: &waveform.name,
                        component: "values",
                        values: &waveform.y,
                        paired: None,
                        unavailable: Some(self.unavailable),
                    },
                )?;
            }
        }
        map.end()
    }
}

/// One component's samples: an unavailable one is `null`, an infinite one refuses the write,
/// and an imaginary part is unavailable exactly where its real part is, which `paired` reads
/// beside it a block at a time.
struct Samples<'a> {
    signal: &'a str,
    component: &'static str,
    values: &'a WaveformValues,
    paired: Option<&'a WaveformValues>,
    unavailable: Option<&'a Cell<bool>>,
}

impl Serialize for Samples<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.values.len()))?;
        let mut index = 0_usize;
        for block in self.values.blocks(..) {
            let block = block.map_err(S::Error::custom)?;
            let paired = match self.paired {
                Some(paired) => Some(
                    paired
                        .blocks(index..index + block.len())
                        .next()
                        .transpose()
                        .map_err(S::Error::custom)?
                        .unwrap_or_default(),
                ),
                None => None,
            };
            for (offset, &value) in block.iter().enumerate() {
                if value.is_infinite() {
                    return Err(S::Error::custom(format!(
                        "native bundle signal '{}' has an infinite {} sample at index {}",
                        self.signal,
                        self.component,
                        index + offset
                    )));
                }
                if let Some(paired) = &paired
                    && paired
                        .get(offset)
                        .is_some_and(|real| real.is_nan() != value.is_nan())
                {
                    return Err(S::Error::custom(format!(
                        "native bundle signal '{}' has inconsistent complex sample availability",
                        self.signal
                    )));
                }
                if value.is_nan()
                    && let Some(unavailable) = self.unavailable
                {
                    unavailable.set(true);
                }
                sequence.serialize_element(&(!value.is_nan()).then_some(value))?;
            }
            index += block.len();
        }
        sequence.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_bundle::{
        NativeBundleKind, NativeBundleReadLimits, decode_native_bundle, encode_native_bundle,
    };
    use rspice_results::analysis_type::AnalysisType;

    #[test]
    fn retained_bundle_round_trip_preserves_declared_and_unstated_units() {
        for unit in [None, Some("mA")] {
            let mut real = RetainedWaveform::new("V(out)", vec![0.0, 1.0], vec![-0.0, 2.0]);
            let mut complex = RetainedWaveform::new("|I(V1)|", vec![0.0, 1.0], vec![1.0, 2.0])
                .with_complex_components("I(V1)", vec![-0.0, 2.0], vec![1.0, -1.0]);
            real.unit = unit.map(str::to_owned);
            complex.unit = unit.map(str::to_owned);
            let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Retained samples", 0.0)
                .with_waveforms(vec![real, complex]);
            let waveforms = analysis.waveforms.iter().collect::<Vec<_>>();
            let projected =
                project_native_bundle(&analysis, &waveforms, WaveformDomain::Transient).unwrap();
            for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
                let bytes = encode_native_bundle(kind, &projected, 1_000_000).unwrap();
                let decoded = decode_native_bundle(
                    &bytes,
                    kind,
                    NativeBundleReadLimits {
                        max_members: 10,
                        max_member_bytes: 1_000_000,
                        max_expanded_bytes: 1_000_000,
                        max_rows: 1_000_000,
                        max_columns: 1_024,
                        max_numeric_values: 1_000_000 / size_of::<f64>(),
                    },
                )
                .unwrap();
                assert_eq!(decoded.signals[0].unit.as_deref(), unit);
                assert_eq!(decoded.signals[1].unit.as_deref(), unit);
                assert_eq!(decoded.signals[0].real[0].to_bits(), (-0.0_f64).to_bits());
                assert_eq!(
                    decoded.signals[1].imag.as_deref(),
                    Some([1.0, -1.0].as_slice())
                );
            }
        }
    }

    /// One member of a `.step` sweep or Monte Carlo study keeps which member it is, with its
    /// parameter values, through either bundle; a result that is no member writes none.
    #[test]
    fn a_family_member_round_trips_through_both_bundles() {
        use crate::native_bundle::decode_native_bundle_with_member;
        let limits = NativeBundleReadLimits {
            max_members: 10,
            max_member_bytes: 1_000_000,
            max_expanded_bytes: 1_000_000,
            max_rows: 1_000_000,
            max_columns: 1_024,
            max_numeric_values: 1_000_000 / size_of::<f64>(),
        };
        let step = AnalysisResultFamilyMember::StepPoint {
            index: 2,
            coordinate: vec![
                AnalysisResultStepCoordinate {
                    name: "r".to_owned(),
                    value: 2000.0,
                    unit: String::new(),
                },
                AnalysisResultStepCoordinate {
                    name: "temp".to_owned(),
                    value: -40.0,
                    unit: "°C".to_owned(),
                },
            ],
        };
        let trial = AnalysisResultFamilyMember::MonteCarloTrial { index: 7 };
        for member in [Some(step), Some(trial), None] {
            let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient", 0.0)
                .with_waveforms(vec![RetainedWaveform::new(
                    "V(out)",
                    vec![0.0, 1.0],
                    vec![0.5, 1.5],
                )]);
            if let Some(member) = &member {
                analysis.import_source = Some(rspice_results::result_import::ResultImportSource {
                    source_name: "family.raw".to_owned(),
                    format: rspice_results::result_import::ResultImportFormat::SpiceRaw,
                    coordinate: None,
                    family_member: Some(member.clone()),
                    event_dump: None,
                });
            }
            let waveforms = analysis.waveforms.iter().collect::<Vec<_>>();
            let projected =
                project_native_bundle(&analysis, &waveforms, WaveformDomain::Transient).unwrap();
            for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
                let bytes = encode_native_bundle(kind, &projected, 1_000_000).unwrap();
                let (decoded, read) =
                    decode_native_bundle_with_member(&bytes, kind, limits).unwrap();
                assert_eq!(
                    read.map(NativeBundleMember::into_family_member),
                    member,
                    "{kind:?}"
                );
                assert_eq!(decoded.signals[0].real, vec![0.5, 1.5]);
            }
        }
    }

    fn read_limits() -> NativeBundleReadLimits {
        NativeBundleReadLimits {
            max_members: 10,
            max_member_bytes: 1_000_000,
            max_expanded_bytes: 10_000_000,
            max_rows: 1_000_000,
            max_columns: 1_024,
            max_numeric_values: 1_000_000,
        }
    }

    /// A bundle written as it is read reads back sample for sample, unavailable samples and
    /// units included, through either kind.
    #[test]
    fn a_streamed_bundle_reads_back_as_written() {
        let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("|V(out)|", vec![1.0, 10.0, 100.0], vec![1.0, 2.0, 3.0])
                .with_complex_components(
                    "V(out)",
                    vec![-0.0, f64::NAN, 3.0],
                    vec![1.0, f64::NAN, -1.0],
                )
                .with_unit("V"),
            RetainedWaveform::new("Re(in)", vec![1.0, 10.0, 100.0], vec![4.0, 5.0, 6.0]),
        ]);
        let members = [ExportMember::new(
            &analysis,
            analysis.waveforms.iter().collect(),
        )];
        for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
            let mut bytes = Vec::new();
            let facts = write_native_bundle(kind, &members, &mut bytes).unwrap();
            assert_eq!(
                facts,
                NativeBundleFacts {
                    signals: 2,
                    points: 3,
                    members: 1
                }
            );
            let read =
                crate::native_bundle::read_native_bundle(&bytes, kind, read_limits()).unwrap();
            assert!(read.family.is_empty() && read.unread_members.is_empty());
            assert_eq!(read.dataset.coordinate, [1.0, 10.0, 100.0]);
            assert_eq!(read.dataset.signals[0].name, "V(out)");
            assert_eq!(read.dataset.signals[0].unit.as_deref(), Some("V"));
            assert_eq!(
                read.dataset.signals[0].real[0].to_bits(),
                (-0.0_f64).to_bits()
            );
            assert!(read.dataset.signals[0].real[1].is_nan());
            assert_eq!(read.dataset.signals[0].imag.as_ref().unwrap()[2], -1.0);
            assert_eq!(read.dataset.signals[1].real, [4.0, 5.0, 6.0]);
        }
    }

    /// Every member of a `.step` is a dataset of its own naming its parameter values; the
    /// reader reads them all, in family order.
    #[test]
    fn a_bundle_of_every_member_reads_back_each_member() {
        let members = [1000.0, 2000.0, 4000.0]
            .into_iter()
            .enumerate()
            .map(|(index, r)| {
                let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
                    .with_waveforms(vec![RetainedWaveform::new(
                        "V(out)",
                        vec![0.0, 1.0 + index as f64],
                        vec![r, -r],
                    )]);
                crate::waveform_io::result::step_member(&mut analysis, index, &[("r", r)]);
                analysis
            })
            .collect::<Vec<_>>();
        let export = members
            .iter()
            .map(|analysis| ExportMember::new(analysis, analysis.waveforms.iter().collect()))
            .collect::<Vec<_>>();
        let mut bytes = Vec::new();
        let facts = write_native_bundle(NativeBundleKind::Result, &export, &mut bytes).unwrap();
        assert_eq!(facts.members, 3);
        let read = crate::native_bundle::read_native_bundle(
            &bytes,
            NativeBundleKind::Result,
            read_limits(),
        )
        .unwrap();
        assert!(read.unread_members.is_empty());
        let datasets = std::iter::once((read.dataset, read.member))
            .chain(
                read.family
                    .into_iter()
                    .map(|member| (member.dataset, member.member)),
            )
            .collect::<Vec<_>>();
        assert_eq!(datasets.len(), 3);
        for (position, ((dataset, member), r)) in datasets
            .into_iter()
            .zip([1000.0, 2000.0, 4000.0])
            .enumerate()
        {
            assert_eq!(dataset.coordinate, [0.0, 1.0 + position as f64]);
            assert_eq!(dataset.signals[0].real, [r, -r]);
            assert_eq!(
                member
                    .map(NativeBundleMember::into_family_member)
                    .map(|member| member.label()),
                Some(format!("r={}k", r / 1000.0))
            );
        }
    }

    /// A coordinate a reader would refuse refuses the write when it is read.
    #[test]
    fn a_coordinate_that_repeats_refuses_the_write() {
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(vec![
                RetainedWaveform::new("V(out)", vec![0.0, 1.0, 1.0], vec![1.0, 2.0, 3.0]),
            ]);
        let members = [ExportMember::new(
            &analysis,
            analysis.waveforms.iter().collect(),
        )];
        check_native_bundle(&members).unwrap();
        let error =
            write_native_bundle(NativeBundleKind::Result, &members, &mut Vec::new()).unwrap_err();
        assert!(error.to_string().contains("repeats at sample 2"), "{error}");
    }
}
