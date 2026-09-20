//! Measurements reconstructed from the complete adjoints and source covariances.
use super::*;
use crate::analysis::noise::ScaledPositiveSum;
use crate::analysis::quasi_periodic::QuasiPeriodicAdjointSolution;
use crate::numerics::scaled_noise::{ScaledComplexAccumulator, scaled_complex_product3};

pub(super) fn reconstruct(
    metadata: &QpnoiseResultMetadata,
    points: &[QuasiPeriodicNoisePoint],
    grid: &QuasiPeriodicGrid,
    abort: &dyn AbortSignal,
) -> Result<
    (
        Vec<QuasiPeriodicNoiseCovariance>,
        Vec<QpnoiseOutputSpectrum>,
    ),
    SimulationError,
> {
    let request = &metadata.request;
    let n = request.outputs.len();
    let totals = points
        .iter()
        .map(|point| total_covariance(&point.source_covariances, n, abort))
        .collect::<Result<Vec<_>, _>>()?;
    let anchor = request.frequency_anchor();
    let input_index = request
        .input
        .as_ref()
        .map(|i| grid.index_of(&i.lattice).expect("validated input tuple"));
    let mut outputs = Vec::with_capacity(n);
    for (output_index, output) in request.outputs.iter().enumerate() {
        check_abort(abort)?;
        let frequencies_hz = request
            .frequencies_hz
            .iter()
            .enumerate()
            .map(|(i, f)| {
                if i.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                grid.frequency_relative_to(*f, &anchor, &output.lattice)
                    .map_err(numerical_error)
            })
            .collect::<Result<Vec<_>, _>>()?;
        if frequencies_hz.windows(2).any(|p| p[0] >= p[1]) {
            return Err(qpnoise_error(
                "output frequency grid collapses at a translated tuple",
            ));
        }
        let diagonal = output_index * n + output_index;
        let output_density = totals
            .iter()
            .map(|c| QpnoiseValue::Finite(c.values[diagonal].re))
            .collect::<Vec<_>>();
        let input_transfer = metadata
            .input_source
            .as_ref()
            .map(|source| {
                points
                    .iter()
                    .map(|p| {
                        transfer(
                            &p.adjoints[output_index],
                            source,
                            input_index.expect("validated input tuple"),
                            abort,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let input_noise = input_transfer.as_ref().map(|gains| {
            gains
                .iter()
                .zip(&totals)
                .map(|(gain, c)| refer(c.values[diagonal].re, *gain))
                .collect::<Vec<_>>()
        });
        let noise_figure_db = metadata
            .reference
            .as_ref()
            .map(|reference| {
                points
                    .iter()
                    .map(|p| figure(metadata, p, output_index, reference, grid, abort))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let ranking_config = QpnoiseIntegration {
            band_hz: None,
            method: QpnoiseIntegrationMethod::Linear,
        };
        let integration_config = request.integration.as_ref().unwrap_or(&ranking_config);
        let needs_integral = request.integration.is_some() || request.contributor_ranking;
        let mut contributor_powers = Vec::new();
        let count = points.first().map_or(0, |p| p.source_covariances.len());
        if needs_integral {
            for source in 0..count {
                check_abort(abort)?;
                let density = points
                    .iter()
                    .map(|p| QpnoiseValue::Finite(p.source_covariances[source].values[diagonal].re))
                    .collect::<Vec<_>>();
                let power = if request.contributor_ranking
                    && request.integration.is_none()
                    && points.len() == 1
                {
                    let QpnoiseValue::Finite(value) = density[0] else {
                        unreachable!()
                    };
                    let mut sum = ScaledPositiveSum::default();
                    sum.add_product(value, 1.0, 1.0);
                    Ok(sum)
                } else {
                    integration::integrate(&frequencies_hz, &density, integration_config, abort)?
                };
                contributor_powers.push(power);
            }
        }
        let integrated = request
            .integration
            .as_ref()
            .map(|config| -> Result<_, SimulationError> {
                Ok(QpnoiseIntegrated {
                    output_rms: integration::rms(integration::integrate(
                        &frequencies_hz,
                        &output_density,
                        config,
                        abort,
                    )?),
                    input_rms: input_noise
                        .as_ref()
                        .map(|density| {
                            integration::integrate(&frequencies_hz, density, config, abort)
                                .map(integration::rms)
                        })
                        .transpose()?,
                    contributor_rms: contributor_powers
                        .iter()
                        .copied()
                        .map(integration::rms)
                        .collect(),
                })
            })
            .transpose()?;
        let ranking = if request.contributor_ranking && contributor_powers.iter().all(Result::is_ok)
        {
            let mut total = ScaledPositiveSum::default();
            for power in &contributor_powers {
                if let Some((mantissa, exponent)) = power.unwrap().parts() {
                    total.add_binary(mantissa, exponent);
                }
            }
            let mut ranking = contributor_powers
                .iter()
                .enumerate()
                .map(|(source_index, power)| {
                    let percentage = match (power.unwrap().parts(), total.parts()) {
                        (Some((a, e)), Some((b, f))) => libm::scalbn(100.0 * a / b, e - f),
                        _ => 0.0,
                    };
                    QpnoiseContributorRank {
                        source_index,
                        percentage,
                    }
                })
                .collect::<Vec<_>>();
            ranking.sort_by(|a, b| {
                contributor_powers[b.source_index]
                    .unwrap()
                    .compare_power(contributor_powers[a.source_index].unwrap())
                    .then_with(|| a.source_index.cmp(&b.source_index))
            });
            Some(ranking)
        } else {
            None
        };
        outputs.push(QpnoiseOutputSpectrum {
            frequencies_hz,
            input_transfer,
            input_noise,
            noise_figure_db,
            integrated,
            ranking,
        });
    }
    Ok((totals, outputs))
}

pub(super) fn transfer(
    adjoint: &QuasiPeriodicAdjointSolution,
    source: &super::super::qpxf::QpxfInputSource,
    index: usize,
    abort: &dyn AbortSignal,
) -> Result<Complex64, SimulationError> {
    check_abort(abort)?;
    let terms = source
        .injections
        .iter()
        .map(|(row, b)| {
            scaled_complex_product3(
                adjoint.sensitivities[*row][index].conj(),
                *b,
                Complex64::ONE,
                0,
            )
            .map_err(qpnoise_error)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let exponent = terms
        .iter()
        .filter(|t| !t.is_zero())
        .map(|t| t.exponent)
        .max();
    let Some(exponent) = exponent else {
        return Ok(Complex64::ZERO);
    };
    let mut sum = ScaledComplexAccumulator::new(exponent);
    for term in terms {
        sum.add(term).map_err(qpnoise_error)?;
    }
    Ok(sum.finish().map_err(qpnoise_error)?.0)
}

fn log_gain_power(gain: Complex64) -> Value {
    let scale = gain.re.abs().max(gain.im.abs());
    if scale == 0.0 {
        return Value::NEG_INFINITY;
    }
    2.0 * scale.ln() + ((gain.re / scale).powi(2) + (gain.im / scale).powi(2)).ln()
}
fn refer(density: Value, gain: Complex64) -> QpnoiseValue {
    if gain == Complex64::ZERO {
        return QpnoiseValue::Unavailable(QpnoiseUnavailable::ZeroInputTransfer);
    }
    if density == 0.0 {
        return QpnoiseValue::Finite(0.0);
    }
    let value = (density.ln() - log_gain_power(gain)).exp();
    if value == 0.0 {
        QpnoiseValue::Unavailable(QpnoiseUnavailable::OutsideNumericRange)
    } else {
        QpnoiseValue::from_nonnegative(value)
    }
}
fn log_add(a: Value, b: Value) -> Value {
    if a == Value::NEG_INFINITY {
        return b;
    }
    if b == Value::NEG_INFINITY {
        return a;
    }
    a.max(b) + (a.min(b) - a.max(b)).exp().ln_1p()
}
fn figure(
    m: &QpnoiseResultMetadata,
    p: &QuasiPeriodicNoisePoint,
    output: usize,
    r: &QpnoiseReference,
    grid: &QuasiPeriodicGrid,
    abort: &dyn AbortSignal,
) -> Result<QpnoiseValue, SimulationError> {
    let request = m
        .request
        .noise_figure
        .as_ref()
        .expect("validated reference");
    let source = m.input_source.as_ref().expect("validated source");
    let mut gain_power = Value::NEG_INFINITY;
    for tuple in &r.lattices {
        let gain = transfer(
            &p.adjoints[output],
            source,
            grid.index_of(tuple).expect("validated reference tuple"),
            abort,
        )?;
        gain_power = log_add(gain_power, log_gain_power(gain));
    }
    if gain_power == Value::NEG_INFINITY {
        return Ok(QpnoiseValue::Unavailable(
            QpnoiseUnavailable::ZeroInputTransfer,
        ));
    }
    let diagonal = output * m.request.outputs.len() + output;
    let mut noise = Value::NEG_INFINITY;
    for (i, covariance) in p.source_covariances.iter().enumerate() {
        if i.is_multiple_of(256) {
            check_abort(abort)?;
        }
        let q = covariance.values[diagonal].re;
        if q > 0.0 {
            noise = log_add(
                noise,
                q.ln()
                    + if i == r.source_index {
                        request.reference_temperature.ln() - r.temperature.ln()
                    } else {
                        0.0
                    },
            );
        }
    }
    let db = (10.0 / std::f64::consts::LN_10)
        * (noise
            - gain_power
            - 4.0_f64.ln()
            - r.boltzmann.ln()
            - request.reference_temperature.ln()
            - r.resistance.ln());
    Ok(if db.is_finite() {
        QpnoiseValue::Finite(db)
    } else {
        QpnoiseValue::Unavailable(QpnoiseUnavailable::OutsideNumericRange)
    })
}
fn total_covariance(
    sources: &[QuasiPeriodicNoiseCovariance],
    outputs: usize,
    abort: &dyn AbortSignal,
) -> Result<QuasiPeriodicNoiseCovariance, SimulationError> {
    let mut values = vec![Complex64::ZERO; outputs * outputs];
    let mut roundoff_bounds = vec![0.0; outputs * outputs];
    for row in 0..outputs {
        for column in row..outputs {
            check_abort(abort)?;
            let index = row * outputs + column;
            let mut bound = ScaledPositiveSum::default();
            let mut exponent = None;
            for (i, source) in sources.iter().enumerate() {
                if i.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                let term = scaled_complex_product3(
                    source.values[index],
                    Complex64::ONE,
                    Complex64::ONE,
                    0,
                )
                .map_err(qpnoise_error)?;
                if !term.is_zero() {
                    exponent = Some(exponent.map_or(term.exponent, |e: i32| e.max(term.exponent)));
                }
                bound.add_product(source.roundoff_bounds[index], 1.0, 1.0);
            }
            let mut sum = ScaledComplexAccumulator::new(exponent.unwrap_or(0));
            for (i, source) in sources.iter().enumerate() {
                if i.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                sum.add(
                    scaled_complex_product3(
                        source.values[index],
                        Complex64::ONE,
                        Complex64::ONE,
                        0,
                    )
                    .map_err(qpnoise_error)?,
                )
                .map_err(qpnoise_error)?;
            }
            let (value, absolute) = if exponent.is_some() {
                sum.finish().map_err(qpnoise_error)?
            } else {
                (Complex64::ZERO, 0.0)
            };
            bound.add_product(
                absolute,
                Value::EPSILON * 32.0 * sources.len().max(1) as Value,
                1.0,
            );
            let bound = bound.value();
            let bound = if bound.is_nan() {
                Value::from_bits(1)
            } else {
                bound
            };
            if !bound.is_finite() {
                return Err(qpnoise_error(
                    "total covariance roundoff bound is outside numeric range",
                ));
            }
            values[index] = value;
            values[column * outputs + row] = value.conj();
            roundoff_bounds[index] = bound;
            roundoff_bounds[column * outputs + row] = bound;
        }
    }
    Ok(QuasiPeriodicNoiseCovariance {
        outputs,
        values,
        roundoff_bounds,
    })
}
