//! Typed metadata and transferable complete MNA Fourier coefficients.
use super::*;
use rspice_core::engine::{QpssOperatingPoint, QpssOperatingPointMetadata};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerQpssOperatingPointTransport {
    metadata: QpssOperatingPointMetadata,
    spectra: Vec<WorkerQpssSpectrum>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkerQpssSpectrum {
    real: WorkerF64Series,
    imaginary: WorkerF64Series,
}

impl WorkerQpssOperatingPointTransport {
    pub(super) fn from_operating_point(
        point: QpssOperatingPoint,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Self {
        let (metadata, rows) = point.into_transfer_parts();
        let spectra = rows
            .into_iter()
            .map(|row| {
                let (real, imaginary) = row.into_iter().map(|value| (value.re, value.im)).unzip();
                WorkerQpssSpectrum {
                    real: WorkerF64Series::from_vec(real, buffers),
                    imaginary: WorkerF64Series::from_vec(imaginary, buffers),
                }
            })
            .collect();
        Self { metadata, spectra }
    }

    pub(super) fn into_operating_point(
        self,
        buffers: &[Vec<f64>],
    ) -> Result<QpssOperatingPoint, String> {
        if self.spectra.len() > 65_536 {
            return Err("QPSS worker spectrum exceeds structural limits".into());
        }
        let lengths = self
            .spectra
            .iter()
            .map(|row| {
                if !matches!(row.real, WorkerF64Series::Buffer { .. })
                    || !matches!(row.imaginary, WorkerF64Series::Buffer { .. })
                    || row.real.len() != row.imaginary.len()
                {
                    return Err(
                        "QPSS spectra require equal dedicated real/imaginary buffers".to_owned(),
                    );
                }
                Ok(row.real.len())
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.metadata
            .validate_transfer_layout_with_abort(
                &lengths,
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|error| error.to_string())?;
        let spectra = self
            .spectra
            .into_iter()
            .map(|row| {
                worker_join_complex(
                    "QPSS MNA spectrum",
                    row.real.into_vec(buffers)?,
                    row.imaginary.into_vec(buffers)?,
                )
            })
            .collect::<Result<Vec<_>, String>>()?;
        QpssOperatingPoint::from_transfer_parts_with_abort(
            self.metadata,
            spectra,
            &rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .map_err(|error| error.to_string())
    }
}
