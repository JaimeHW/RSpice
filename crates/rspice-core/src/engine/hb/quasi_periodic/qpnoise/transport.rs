//! One bounded scalar buffer for complete noise spectra, adjoints and covariances.
//!
//! Large sampled densities, complex coefficients, explicit modulation tuples,
//! residual certificates and covariance matrices live in the numeric buffer.
//! Metadata describes their exact layout; every length is checked before decode.
use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicAdjointSolution;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum QpnoiseSpectrumLayout {
    White {
        density_count: usize,
        binary_scale_exponent: i32,
    },
    PowerLaw {
        coefficient: Value,
        exponent: Value,
        mode_count: usize,
        explicit_lattices: bool,
        binary_scale_exponent: i32,
    },
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseSourceLayout {
    pub name: String,
    pub injections: Vec<(usize, Complex64)>,
    pub spectrum: QpnoiseSpectrumLayout,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseTransferMetadata {
    pub result: QpnoiseResultMetadata,
    pub sources: Vec<QpnoiseSourceLayout>,
}
impl QpnoiseTransferMetadata {
    fn value_count(&self, grid: &QuasiPeriodicGrid) -> usize {
        let m = &self.result;
        let outputs = m.request.outputs.len();
        let coordinates = m.node_names.len().saturating_add(m.branch_names.len());
        let mut count = m.request.frequencies_hz.len().saturating_mul(
            outputs
                .saturating_mul(
                    1usize.saturating_add(coordinates.saturating_mul(grid.len()).saturating_mul(2)),
                )
                .saturating_add(
                    self.sources
                        .len()
                        .saturating_mul(outputs)
                        .saturating_mul(outputs)
                        .saturating_mul(3),
                ),
        );
        for source in &self.sources {
            count = count.saturating_add(match source.spectrum {
                QpnoiseSpectrumLayout::White { density_count, .. } => density_count,
                QpnoiseSpectrumLayout::PowerLaw {
                    mode_count,
                    explicit_lattices,
                    ..
                } => mode_count.saturating_mul(2usize.saturating_add(if explicit_lattices {
                    grid.dimensions().len()
                } else {
                    0
                })),
            });
        }
        count
    }
    /// Check the complete receive allocation, including the packed buffer and
    /// decoded primary/derived evidence, before copying any numeric payload.
    pub fn validate_transfer_layout_with_abort(
        &self,
        value_count: usize,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<QuasiPeriodicGrid>, SimulationError> {
        check_abort(abort)?;
        let m = &self.result;
        m.request.validate()?;
        let points = m.request.frequencies_hz.len();
        let outputs = m.request.outputs.len();
        let coordinates = m.node_names.len().saturating_add(m.branch_names.len());
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            points
                .saturating_mul(outputs)
                .saturating_mul(self.sources.len().max(1)),
            limits.max_analysis_points,
        )?;
        if coordinates == 0
            || coordinates > 65_536
            || m.version != 1
            || !is_canonical_blake3_identity(&m.retained_identity)
            || !is_canonical_blake3_identity(&m.operating_point_identity)
        {
            return Err(qpnoise_error(
                "packed result has invalid dimensions, version or identities",
            ));
        }
        let grid = Arc::new(
            QuasiPeriodicGrid::new_with_abort(m.grid.clone(), limits, abort)
                .map_err(numerical_error)?,
        );
        let expected = self.value_count(&grid);
        // At most two copies of primary numbers are live during decode. The
        // last packed allocation is released before derived reconstruction.
        let metadata_values = self
            .sources
            .iter()
            .fold(0usize, |n, s| {
                n.saturating_add(s.injections.len().saturating_mul(3))
            })
            .saturating_add(
                m.observations
                    .iter()
                    .fold(0usize, |n, o| n.saturating_add(o.len().saturating_mul(3))),
            )
            .saturating_add(
                m.input_lattices
                    .len()
                    .saturating_mul(grid.dimensions().len()),
            );
        let resident = expected
            .saturating_mul(2)
            .saturating_add(
                points.saturating_mul(
                    outputs
                        .saturating_mul(32)
                        .saturating_add(outputs.saturating_mul(outputs).saturating_mul(6)),
                ),
            )
            .saturating_add(
                outputs
                    .saturating_mul(self.sources.len())
                    .saturating_mul(12),
            )
            .saturating_add(metadata_values);
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            resident,
            limits.max_result_values.min(32_000_000),
        )?;
        if value_count != expected {
            return Err(qpnoise_error(
                "packed buffer length differs from the complete noise layout",
            ));
        }
        if m.observations.len() != outputs
            || m.input_lattices != m.request.input_lattices.resolve(&grid)?
        {
            return Err(qpnoise_error(
                "packed observations or noise-input tuples differ from the request",
            ));
        }
        for source in &self.sources {
            check_abort(abort)?;
            let valid = match source.spectrum {
                QpnoiseSpectrumLayout::White { density_count, .. } => {
                    density_count == 1 || density_count == grid.sample_count()
                }
                QpnoiseSpectrumLayout::PowerLaw {
                    coefficient,
                    exponent,
                    mode_count,
                    explicit_lattices,
                    ..
                } => {
                    coefficient.is_finite()
                        && coefficient >= 0.0
                        && exponent.is_finite()
                        && mode_count > 0
                        && (explicit_lattices || mode_count == grid.len())
                }
            };
            if !valid {
                return Err(qpnoise_error(
                    "packed physical spectrum dimensions are invalid",
                ));
            }
        }
        Ok(grid)
    }
}

impl QpnoiseAnalysisResult {
    pub fn into_transfer_parts_with_abort(
        self,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(QpnoiseTransferMetadata, Vec<Value>), SimulationError> {
        let grid = self.validate_retained_payload_with_abort(limits, abort)?;
        let layouts = self
            .sources
            .iter()
            .map(|source| QpnoiseSourceLayout {
                name: source.name.clone(),
                injections: source.injections.clone(),
                spectrum: match &source.spectrum {
                    QuasiPeriodicNoiseSpectrum::White {
                        density,
                        binary_scale_exponent,
                    } => QpnoiseSpectrumLayout::White {
                        density_count: density.len(),
                        binary_scale_exponent: *binary_scale_exponent,
                    },
                    QuasiPeriodicNoiseSpectrum::PowerLaw {
                        coefficient,
                        exponent,
                        modulation,
                        modulation_lattices,
                        binary_scale_exponent,
                    } => QpnoiseSpectrumLayout::PowerLaw {
                        coefficient: *coefficient,
                        exponent: *exponent,
                        mode_count: modulation.len(),
                        explicit_lattices: modulation_lattices.is_some(),
                        binary_scale_exponent: *binary_scale_exponent,
                    },
                },
            })
            .collect();
        let Self {
            metadata,
            sources,
            points,
            total_covariances,
            outputs,
        } = self;
        drop((total_covariances, outputs));
        let metadata = QpnoiseTransferMetadata {
            result: metadata,
            sources: layouts,
        };
        let count = metadata.value_count(&grid);
        metadata.validate_transfer_layout_with_abort(count, limits, abort)?;
        let mut packed = Pack {
            values: Vec::new(),
            abort,
        };
        packed
            .values
            .try_reserve_exact(count)
            .map_err(|e| qpnoise_error(format!("packed allocation failed: {e}")))?;
        for source in sources {
            match source.spectrum {
                QuasiPeriodicNoiseSpectrum::White { density, .. } => {
                    for value in density {
                        packed.push(value)?;
                    }
                }
                QuasiPeriodicNoiseSpectrum::PowerLaw {
                    modulation,
                    modulation_lattices,
                    ..
                } => {
                    for value in modulation {
                        packed.complex(value)?;
                    }
                    if let Some(tuples) = modulation_lattices {
                        for tuple in tuples {
                            for coordinate in tuple {
                                packed.push(Value::from(coordinate))?;
                            }
                        }
                    }
                }
            }
        }
        for point in points {
            for adjoint in point.adjoints {
                packed.push(adjoint.normalized_residual)?;
                for row in adjoint.sensitivities {
                    for value in row {
                        packed.complex(value)?;
                    }
                }
            }
            for covariance in point.source_covariances {
                for value in covariance.values {
                    packed.complex(value)?;
                }
                for value in covariance.roundoff_bounds {
                    packed.push(value)?;
                }
            }
        }
        check_abort(abort)?;
        debug_assert_eq!(packed.values.len(), count);
        Ok((metadata, packed.values))
    }
    pub fn from_transfer_parts_with_abort(
        metadata: QpnoiseTransferMetadata,
        values: Vec<Value>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let grid = metadata.validate_transfer_layout_with_abort(values.len(), limits, abort)?;
        let mut packed = Unpack {
            values: values.into_iter(),
            read: 0,
            abort,
        };
        let mut sources = Vec::with_capacity(metadata.sources.len());
        for source in metadata.sources {
            check_abort(abort)?;
            let spectrum = match source.spectrum {
                QpnoiseSpectrumLayout::White {
                    density_count,
                    binary_scale_exponent,
                } => QuasiPeriodicNoiseSpectrum::White {
                    density: packed.reals(density_count)?,
                    binary_scale_exponent,
                },
                QpnoiseSpectrumLayout::PowerLaw {
                    coefficient,
                    exponent,
                    mode_count,
                    explicit_lattices,
                    binary_scale_exponent,
                } => {
                    let modulation = packed.complexes(mode_count)?;
                    let modulation_lattices = if explicit_lattices {
                        let mut tuples = Vec::with_capacity(mode_count);
                        for _ in 0..mode_count {
                            let mut tuple = Vec::with_capacity(grid.dimensions().len());
                            for _ in grid.dimensions() {
                                let value = packed.next()?;
                                if value.fract() != 0.0
                                    || value < Value::from(i32::MIN)
                                    || value > Value::from(i32::MAX)
                                {
                                    return Err(qpnoise_error(
                                        "packed modulation coordinates must be exact signed integers",
                                    ));
                                }
                                tuple.push(value as i32);
                            }
                            tuples.push(tuple);
                        }
                        Some(tuples)
                    } else {
                        None
                    };
                    QuasiPeriodicNoiseSpectrum::PowerLaw {
                        coefficient,
                        exponent,
                        modulation,
                        modulation_lattices,
                        binary_scale_exponent,
                    }
                }
            };
            sources.push(QuasiPeriodicNoiseSource {
                name: source.name,
                injections: source.injections,
                spectrum,
            });
        }
        let metadata = metadata.result;
        let count = metadata.request.frequencies_hz.len();
        let outputs = metadata.request.outputs.len();
        let coordinates = metadata.node_names.len() + metadata.branch_names.len();
        let anchor = metadata.request.frequency_anchor();
        let mut points = Vec::with_capacity(count);
        for &frequency_hz in &metadata.request.frequencies_hz {
            let mut adjoints = Vec::with_capacity(outputs);
            for _ in 0..outputs {
                let normalized_residual = packed.next()?;
                let sensitivities = (0..coordinates)
                    .map(|_| packed.complexes(grid.len()))
                    .collect::<Result<Vec<_>, _>>()?;
                adjoints.push(QuasiPeriodicAdjointSolution {
                    frequency_hz,
                    frequency_lattice: anchor.clone(),
                    normalized_residual,
                    sensitivities,
                });
            }
            let mut source_covariances = Vec::with_capacity(sources.len());
            for _ in &sources {
                source_covariances.push(QuasiPeriodicNoiseCovariance {
                    outputs,
                    values: packed.complexes(outputs * outputs)?,
                    roundoff_bounds: packed.reals(outputs * outputs)?,
                });
            }
            points.push(QuasiPeriodicNoisePoint {
                adjoints,
                source_covariances,
            });
        }
        debug_assert_eq!(packed.values.len(), 0);
        drop(packed);
        let mut result = Self {
            metadata,
            sources,
            points,
            total_covariances: Vec::new(),
            outputs: Vec::new(),
        };
        // Validate primary structure and hash before indexing source directions
        // or deriving any display data from an untrusted worker/saved payload.
        result.validate_primary_with_abort(limits, abort)?;
        (result.total_covariances, result.outputs) =
            derived::reconstruct(&result.metadata, &result.points, &grid, abort)?;
        Ok(result)
    }
}
struct Pack<'a> {
    values: Vec<Value>,
    abort: &'a dyn AbortSignal,
}
impl Pack<'_> {
    fn push(&mut self, value: Value) -> Result<(), SimulationError> {
        if self.values.len().is_multiple_of(256) {
            check_abort(self.abort)?;
        }
        self.values.push(value);
        Ok(())
    }
    fn complex(&mut self, value: Complex64) -> Result<(), SimulationError> {
        self.push(value.re)?;
        self.push(value.im)
    }
}
struct Unpack<'a> {
    values: std::vec::IntoIter<Value>,
    read: usize,
    abort: &'a dyn AbortSignal,
}
impl Unpack<'_> {
    fn next(&mut self) -> Result<Value, SimulationError> {
        if self.read.is_multiple_of(256) {
            check_abort(self.abort)?;
        }
        self.read += 1;
        let value = self
            .values
            .next()
            .ok_or_else(|| qpnoise_error("truncated numeric buffer"))?;
        if !value.is_finite() {
            return Err(qpnoise_error(
                "packed numeric buffer contains a nonfinite value",
            ));
        }
        Ok(value)
    }
    fn reals(&mut self, n: usize) -> Result<Vec<Value>, SimulationError> {
        (0..n).map(|_| self.next()).collect()
    }
    fn complexes(&mut self, n: usize) -> Result<Vec<Complex64>, SimulationError> {
        (0..n)
            .map(|_| Ok(Complex64::new(self.next()?, self.next()?)))
            .collect()
    }
}
