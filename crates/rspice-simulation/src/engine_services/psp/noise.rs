//! Periodic noise-wave spectra and matched-channel noise parameters.
use super::*;
use rspice_core::analysis::s_param::{
    PeriodicPortNoiseParameters, PeriodicPortNoiseReference, SMatrix,
};

#[derive(Debug, Clone)]
pub(crate) struct PspNoiseData {
    pub paths: Vec<PspPath>,
    pub parameters: Option<Vec<PeriodicPortNoiseParameters>>,
    pub reference: Option<PeriodicPortNoiseReference>,
    pub fundamental_hz: Value,
}

/// Orthogonal power-wave transform: adjacent equal-Z ports become d,c.
fn mixed_entry(
    row: usize,
    column: usize,
    bands: usize,
    get: impl Fn(usize, usize) -> Complex64,
) -> Complex64 {
    let row_port = row / bands;
    let column_port = column / bands;
    let mut sum = Complex64::ZERO;
    for r in 0..2 {
        for c in 0..2 {
            let rsign = if row_port.is_multiple_of(2) && r == 1 {
                -1.0
            } else {
                1.0
            };
            let csign = if column_port.is_multiple_of(2) && c == 1 {
                -1.0
            } else {
                1.0
            };
            sum += get(
                (row_port / 2 * 2 + r) * bands + row % bands,
                (column_port / 2 * 2 + c) * bands + column % bands,
            ) * (0.5 * rsign * csign);
        }
    }
    sum
}

pub(super) fn collect_noise(
    result: &rspice_core::engine::PspAnalysisResult,
    config: &PspRunConfig,
    dialect: rspice_core::engine::SpiceDialect,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<PspNoiseData>> {
    let Some(points) = &result.noise else {
        return Ok(None);
    };
    if points.len() != result.data.len() {
        return Err(ServiceRunError::Failure(
            "periodic noise and scattering grids differ".into(),
        ));
    }
    use rspice_core::analysis::noise::NoisePhysicalConstants as Constants;
    let boltzmann = match dialect {
        rspice_core::engine::SpiceDialect::BestAvailable => Constants::MODERN.boltzmann,
        rspice_core::engine::SpiceDialect::Ngspice => Constants::NGSPICE_46.boltzmann,
        rspice_core::engine::SpiceDialect::Xyce => Constants::XYCE_7_10.boltzmann,
    };
    let bands =
        usize::try_from(i64::from(result.sideband_max) - i64::from(result.sideband_min) + 1)
            .map_err(|_| {
                ServiceRunError::Failure(
                    "periodic noise sideband dimension exceeds this platform".into(),
                )
            })?;
    let count = result.ports.len() * bands;
    let references = if config.mixed_mode {
        result
            .ports
            .chunks_exact(2)
            .flat_map(|pair| [2.0 * pair[0].z0, 0.5 * pair[0].z0])
            .collect::<Vec<_>>()
    } else {
        result.ports.iter().map(|p| p.z0).collect()
    };
    let mut covariances = Vec::with_capacity(points.len());
    let mut parameters = config
        .noise_reference
        .as_ref()
        .map(|_| Vec::with_capacity(points.len()));
    for (point, scattering) in points.iter().zip(&result.data) {
        ensure_not_aborted(abort)?;
        let covariance = if config.mixed_mode {
            let mut converted = vec![vec![Complex64::ZERO; count]; count];
            for (row, values) in converted.iter_mut().enumerate() {
                ensure_not_aborted(abort)?;
                for (column, value) in values.iter_mut().enumerate() {
                    *value = mixed_entry(row, column, bands, |r, c| point.wave_correlation[r][c]);
                }
            }
            converted
        } else {
            point.wave_correlation.clone()
        };
        if let (Some(reference), Some(parameters)) = (&config.noise_reference, &mut parameters) {
            let mixed;
            let scattering = if config.mixed_mode {
                let mut converted = SMatrix::new(scattering.frequency, count);
                for row in 0..count {
                    ensure_not_aborted(abort)?;
                    for column in 0..count {
                        converted.set(
                            row + 1,
                            column + 1,
                            mixed_entry(row, column, bands, |r, c| scattering.get(r + 1, c + 1)),
                        );
                    }
                }
                mixed = converted;
                &mixed
            } else {
                scattering
            };
            parameters.push(
                s_param::derive_periodic_port_noise_with_abort(
                    scattering,
                    &covariance,
                    &references,
                    result.sideband_min,
                    result.sideband_max,
                    reference,
                    boltzmann,
                    abort,
                )
                .map_err(|error| match error {
                    s_param::NetworkError::Aborted => ServiceRunError::Aborted,
                    error => {
                        ServiceRunError::Failure(format!("Periodic port noise parameters: {error}"))
                    }
                })?,
            );
        }
        covariances.push(covariance);
    }
    let mut paths = Vec::with_capacity(count * count);
    for row in 0..count {
        for column in 0..count {
            ensure_not_aborted(abort)?;
            let output = row / bands;
            let input = column / bands;
            let name = if config.mixed_mode {
                let mode = |port: usize| if port.is_multiple_of(2) { 'd' } else { 'c' };
                format!(
                    "Cw{}{}{}_{}",
                    mode(output),
                    mode(input),
                    output / 2 + 1,
                    input / 2 + 1
                )
            } else {
                format!("Cw{}_{}", output + 1, input + 1)
            };
            paths.push(PspPath {
                output_port: output + 1,
                input_port: input + 1,
                base_name: name,
                output_sideband: result.sideband_min + (row % bands) as i32,
                input_sideband: result.sideband_min + (column % bands) as i32,
                values: covariances
                    .iter()
                    .map(|matrix| matrix[row][column])
                    .collect(),
            });
        }
    }
    Ok(Some(PspNoiseData {
        paths,
        parameters,
        reference: config.noise_reference.clone(),
        fundamental_hz: result.fundamental_freq,
    }))
}
