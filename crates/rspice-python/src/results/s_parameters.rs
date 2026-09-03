//! N-port scattering-parameter results.
//!
//! Carries the S-matrix over frequency, per-port reference impedances, and,
//! when the run requested it, the Norton current-noise correlation matrix `Cy`
//! plus the derived two-port noise parameters `Rn`, `NF`, `NFmin`, and `Sopt`.
//!
//! Stability and gain figures (K, mu, MAG, MSG) are defined for a two-port
//! only. Deriving them from a sub-matrix of a larger network would describe a
//! different device, so anything else yields `None` rather than a plausible
//! wrong number.

use super::*;

/// N-port scattering-parameter sweep with per-port reference impedances.
#[pyclass(name = "SParameterResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PySParameterResult {
    frequencies: Vec<f64>,
    port_names: Vec<String>,
    reference_impedances: Vec<f64>,
    parameters: Vec<Vec<Vec<rspice_core::Complex64>>>,
    noise: Option<SParameterNoiseData>,
}

/// Optional `.SP donoise` data attached to an S-parameter sweep.
///
/// The core assembly returns one covariance matrix per frequency; this is that
/// evidence pivoted into the port-major cube the accessors publish, which is
/// the only difference between the two. The derived two-port parameters are
/// either present and physical for every frequency or the sweep failed: there
/// is no per-point validity mask for a caller to overlook.
#[derive(Debug, Clone)]
pub(crate) struct SParameterNoiseData {
    reference_temperature_kelvin: f64,
    /// Norton port-current covariance in A²/Hz, indexed
    /// `[output_port][input_port][frequency_point]`.
    current_correlation: Vec<Vec<Vec<rspice_core::Complex64>>>,
    two_port_parameters: Option<Vec<rspice_core::analysis::s_param::TwoPortNoise>>,
}

impl SParameterNoiseData {
    fn from_core(assembly: &rspice_core::analysis::s_param::PortNoiseAssembly) -> Self {
        let ports = assembly
            .points
            .first()
            .map_or(0, |point| point.current_correlation.len());
        let mut current_correlation =
            vec![vec![Vec::with_capacity(assembly.points.len()); ports]; ports];
        for point in &assembly.points {
            for (row, correlations) in current_correlation.iter_mut().enumerate() {
                for (column, series) in correlations.iter_mut().enumerate() {
                    // Core validated every matrix as square and `ports x ports`
                    // before publishing the assembly, so a missing entry would
                    // be a core defect rather than caller input; it stays a
                    // typed absence here instead of a fabricated zero.
                    series.extend(
                        point
                            .current_correlation
                            .get(row)
                            .and_then(|values| values.get(column))
                            .copied(),
                    );
                }
            }
        }
        Self {
            reference_temperature_kelvin: assembly.reference_temperature_kelvin,
            current_correlation,
            two_port_parameters: assembly.two_port.clone(),
        }
    }
}

impl PySParameterResult {
    /// Per-frequency stability analysis, for two-port results only.
    ///
    /// K, mu, and the stability circles are defined for a two-port; deriving
    /// them from a sub-matrix of a larger network would describe a different
    /// device, so anything else yields None.
    fn two_port_stability(&self) -> Option<Vec<rspice_core::analysis::s_param::StabilityAnalysis>> {
        Some(
            self.two_port_matrices()?
                .iter()
                .map(rspice_core::analysis::s_param::StabilityAnalysis::from_s_matrix)
                .collect(),
        )
    }

    /// Per-frequency gain analysis, for two-port results only.
    fn two_port_gain(&self) -> Option<Vec<rspice_core::analysis::s_param::GainAnalysis>> {
        Some(
            self.two_port_matrices()?
                .iter()
                .map(rspice_core::analysis::s_param::GainAnalysis::from_s_matrix)
                .collect(),
        )
    }

    /// Rebuild core's one-based `SMatrix` at each frequency.
    fn two_port_matrices(&self) -> Option<Vec<rspice_core::analysis::s_param::SMatrix>> {
        use rspice_core::analysis::s_param::SMatrix;
        if self.parameters.len() != 2 {
            return None;
        }
        Some(
            self.frequencies
                .iter()
                .enumerate()
                .map(|(index, frequency)| {
                    let mut matrix = SMatrix::new(*frequency, 2);
                    for row in 0..2 {
                        for column in 0..2 {
                            let value = self.parameters[row][column]
                                .get(index)
                                .copied()
                                .unwrap_or_else(|| Complex64::new(f64::NAN, f64::NAN));
                            matrix.set(row + 1, column + 1, value);
                        }
                    }
                    matrix
                })
                .collect(),
        )
    }

    /// Project one scalar out of the per-frequency stability analyses.
    fn stability_series<'py, F>(
        &self,
        py: Python<'py>,
        select: F,
    ) -> Option<Bound<'py, PyArray1<f64>>>
    where
        F: Fn(&rspice_core::analysis::s_param::StabilityAnalysis) -> f64,
    {
        let values: Vec<f64> = self.two_port_stability()?.iter().map(select).collect();
        Some(values.to_pyarray(py))
    }

    /// Project one scalar out of the per-frequency gain analyses.
    fn gain_series<'py, F>(&self, py: Python<'py>, select: F) -> Option<Bound<'py, PyArray1<f64>>>
    where
        F: Fn(&rspice_core::analysis::s_param::GainAnalysis) -> f64,
    {
        let values: Vec<f64> = self.two_port_gain()?.iter().map(select).collect();
        Some(values.to_pyarray(py))
    }

    /// Project one core `.SP` run onto the Python result surface.
    ///
    /// Core owns the sweep, the port-noise assembly and the two-port
    /// derivation; this only re-lays the per-frequency matrices as the
    /// port-major cubes the accessors index.
    pub(crate) fn from_run(run: &rspice_core::engine::SParameterRun) -> Self {
        let ports = run.ports.len();
        let mut parameters =
            vec![vec![Vec::with_capacity(run.scattering.data.len()); ports]; ports];
        for matrix in &run.scattering.data {
            for (row, columns) in parameters.iter_mut().enumerate() {
                for (column, series) in columns.iter_mut().enumerate() {
                    series.push(matrix.get(row + 1, column + 1));
                }
            }
        }
        Self {
            frequencies: run.scattering.frequencies(),
            port_names: run
                .ports
                .iter()
                .map(|port| port.source_name.clone())
                .collect(),
            reference_impedances: run.ports.iter().map(|port| port.z0).collect(),
            parameters,
            noise: run.port_noise.as_ref().map(SParameterNoiseData::from_core),
        }
    }

    fn parameter(
        &self,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<&[rspice_core::Complex64]> {
        let num_ports = self.port_names.len();
        if !(1..=num_ports).contains(&output_port) {
            return Err(crate::errors::index_error(format!(
                "output_port must be in 1..={num_ports}, got {output_port}"
            )));
        }
        if !(1..=num_ports).contains(&input_port) {
            return Err(crate::errors::index_error(format!(
                "input_port must be in 1..={num_ports}, got {input_port}"
            )));
        }
        self.parameters
            .get(output_port - 1)
            .and_then(|row| row.get(input_port - 1))
            .map(Vec::as_slice)
            .ok_or_else(|| crate::errors::value_error("malformed S-parameter result matrix"))
    }

    /// Project one real scalar out of the derived two-port noise parameters.
    ///
    /// `None` means the sweep is not a two-port or carried no `donoise` data;
    /// it never means "the parameters were undefined here", because core
    /// refuses to assemble a sweep whose two-port derivation has no physical
    /// solution.
    fn two_port_noise_field<'py, F>(
        &self,
        py: Python<'py>,
        select: F,
    ) -> Option<Bound<'py, PyArray1<f64>>>
    where
        F: Fn(&rspice_core::analysis::s_param::TwoPortNoise) -> f64,
    {
        let points = self.noise.as_ref()?.two_port_parameters.as_ref()?;
        Some(points.iter().map(select).collect::<Vec<_>>().to_pyarray(py))
    }

    fn current_noise_correlation(
        &self,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<&[rspice_core::Complex64]> {
        let num_ports = self.port_names.len();
        if !(1..=num_ports).contains(&output_port) {
            return Err(crate::errors::index_error(format!(
                "output_port must be in 1..={num_ports}, got {output_port}"
            )));
        }
        if !(1..=num_ports).contains(&input_port) {
            return Err(crate::errors::index_error(format!(
                "input_port must be in 1..={num_ports}, got {input_port}"
            )));
        }
        let noise = self.noise.as_ref().ok_or_else(|| {
            crate::errors::value_error(
                "port-noise data was not computed; pass do_noise=True or use .SP ... donoise",
            )
        })?;
        noise
            .current_correlation
            .get(output_port - 1)
            .and_then(|row| row.get(input_port - 1))
            .map(Vec::as_slice)
            .ok_or_else(|| crate::errors::value_error("malformed port-noise correlation matrix"))
    }
}

#[pymethods]
impl PySParameterResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    #[getter]
    fn port_names(&self) -> Vec<String> {
        self.port_names.clone()
    }

    #[getter]
    fn reference_impedances<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.reference_impedances.to_pyarray(py)
    }

    #[getter]
    fn num_ports(&self) -> usize {
        self.port_names.len()
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.frequencies.len()
    }

    /// Whether this result includes `.SP donoise` data.
    #[getter]
    fn has_noise(&self) -> bool {
        self.noise.is_some()
    }

    /// Temperature in kelvin used to evaluate device noise.
    #[getter]
    fn noise_temperature(&self) -> Option<f64> {
        self.noise
            .as_ref()
            .map(|noise| noise.reference_temperature_kelvin)
    }

    /// Whether two-port `Rn`, `NF`, `NFmin`, and `Sopt` are available.
    #[getter]
    fn has_two_port_noise_parameters(&self) -> bool {
        self.noise
            .as_ref()
            .is_some_and(|noise| noise.two_port_parameters.is_some())
    }

    fn s<'py>(
        &self,
        py: Python<'py>,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        Ok(self.parameter(output_port, input_port)?.to_pyarray(py))
    }

    fn magnitude<'py>(
        &self,
        py: Python<'py>,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self
            .parameter(output_port, input_port)?
            .iter()
            .map(|value| value.norm())
            .collect::<Vec<_>>()
            .to_pyarray(py))
    }

    fn magnitude_db<'py>(
        &self,
        py: Python<'py>,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self
            .parameter(output_port, input_port)?
            .iter()
            .map(|value| 20.0 * value.norm().log10())
            .collect::<Vec<_>>()
            .to_pyarray(py))
    }

    fn phase_degrees<'py>(
        &self,
        py: Python<'py>,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self
            .parameter(output_port, input_port)?
            .iter()
            .map(|value| value.arg().to_degrees())
            .collect::<Vec<_>>()
            .to_pyarray(py))
    }

    /// Complex Norton port-current correlation `Cy(i,j)` in A²/Hz.
    fn cy<'py>(
        &self,
        py: Python<'py>,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        Ok(self
            .current_noise_correlation(output_port, input_port)?
            .to_pyarray(py))
    }

    /// Two-port equivalent noise resistance `Rn` in ohms.
    #[getter]
    fn noise_resistance<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.two_port_noise_field(py, |point| point.noise_resistance)
    }

    /// Two-port matched-source noise factor (linear power ratio).
    #[getter]
    fn noise_factor<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.two_port_noise_field(py, |point| point.noise_factor)
    }

    /// Two-port matched-source noise figure in dB (`NF`).
    #[getter]
    fn noise_figure_db<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.two_port_noise_field(py, |point| 10.0 * point.noise_factor.log10())
    }

    /// Minimum two-port noise factor (linear power ratio).
    #[getter]
    fn minimum_noise_factor<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.two_port_noise_field(py, |point| point.minimum_noise_factor)
    }

    /// Minimum two-port noise figure in dB (`NFmin`).
    #[getter]
    fn minimum_noise_figure_db<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.two_port_noise_field(py, |point| 10.0 * point.minimum_noise_factor.log10())
    }

    /// Optimum source reflection coefficient (`Sopt`) for port 1.
    #[getter]
    fn optimum_source_reflection<'py>(
        &self,
        py: Python<'py>,
    ) -> Option<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        let points = self.noise.as_ref()?.two_port_parameters.as_ref()?;
        Some(
            points
                .iter()
                .map(|point| point.optimum_source_reflection)
                .collect::<Vec<_>>()
                .to_pyarray(py),
        )
    }

    /// ngspice-compatible aliases for common RF notation.
    #[getter]
    fn rn<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.noise_resistance(py)
    }

    #[getter]
    fn nf<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.noise_figure_db(py)
    }

    #[getter]
    fn nfmin<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.minimum_noise_figure_db(py)
    }

    /// Conventional Touchstone extension for this port count (e.g. "s2p").
    #[getter]
    fn touchstone_extension(&self) -> String {
        crate::export::touchstone_extension(self.parameters.len())
    }

    /// Whether the two-port stability and gain figures are available.
    ///
    /// K, mu, MAG, MSG, and the stability circles are two-port quantities;
    /// they are None for any other port count rather than silently reported
    /// from a sub-matrix.
    #[getter]
    fn has_two_port_stability(&self) -> bool {
        self.parameters.len() == 2
    }

    /// Rollett stability factor K at each frequency.
    ///
    /// A two-port is unconditionally stable where `K > 1` and `|delta| < 1`;
    /// `unconditionally_stable` reports that combined test directly.
    #[getter]
    fn k_factor<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.stability_series(py, |analysis| analysis.k_factor)
    }

    /// Edwards-Sinsky mu factor at each frequency (`mu > 1` is stable).
    #[getter]
    fn mu_factor<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.stability_series(py, |analysis| analysis.mu_factor)
    }

    /// Load-side mu' factor at each frequency.
    #[getter]
    fn mu_prime<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.stability_series(py, |analysis| analysis.mu_prime)
    }

    /// Determinant of the scattering matrix at each frequency.
    #[getter]
    fn delta<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<Complex64>>> {
        let values: Vec<Complex64> = self
            .two_port_stability()?
            .iter()
            .map(|analysis| Complex64::new(analysis.delta.re, analysis.delta.im))
            .collect();
        Some(values.to_pyarray(py))
    }

    /// Whether the device is unconditionally stable at each frequency.
    #[getter]
    fn unconditionally_stable<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<bool>>> {
        let values: Vec<bool> = self
            .two_port_stability()?
            .iter()
            .map(|analysis| analysis.unconditionally_stable)
            .collect();
        Some(values.to_pyarray(py))
    }

    /// Maximum available gain in dB at each frequency.
    ///
    /// MAG is defined only where the device is unconditionally stable; it is
    /// negative infinity elsewhere, and `max_stable_gain_db` is the figure to
    /// use there.
    #[getter]
    fn max_available_gain_db<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.gain_series(py, |analysis| analysis.mag_db)
    }

    /// Maximum stable gain `|S21/S12|` in dB at each frequency.
    #[getter]
    fn max_stable_gain_db<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.gain_series(py, |analysis| analysis.msg_db)
    }

    /// Mason's unilateral gain U in dB at each frequency.
    #[getter]
    fn mason_unilateral_gain_db<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.gain_series(py, |analysis| analysis.mason_u_db)
    }

    /// Forward transducer gain `|S21|^2` in dB at each frequency.
    #[getter]
    fn transducer_gain_db<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.gain_series(py, |analysis| analysis.s21_gain_db)
    }

    /// Reverse isolation `|S12|^2` in dB at each frequency.
    #[getter]
    fn reverse_isolation_db<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.gain_series(py, |analysis| analysis.s12_isolation_db)
    }

    /// Source and load stability circles at one frequency.
    ///
    /// Returns a dict with `input_center`, `input_radius`,
    /// `input_stable_inside`, and the matching `output_*` entries. The
    /// `*_stable_inside` flags say which side of each circle is the stable
    /// region, which the centre and radius alone do not determine.
    ///
    /// Raises:
    ///     IndexError: If the frequency index is out of range
    ///     ValueError: If this is not a two-port result
    ///
    /// Example:
    ///     >>> circles = sparams.stability_circles(0)
    ///     >>> circles["input_center"], circles["input_radius"]
    fn stability_circles<'py>(
        &self,
        py: Python<'py>,
        frequency_index: usize,
    ) -> PyResult<Bound<'py, PyDict>> {
        let analyses = self.two_port_stability().ok_or_else(|| {
            crate::errors::value_error(format!(
                "stability circles are a two-port quantity, but this result has {} ports",
                self.parameters.len()
            ))
        })?;
        let analysis = analyses.get(frequency_index).ok_or_else(|| {
            crate::errors::index_error(format!(
                "frequency index {frequency_index} is out of range for result with {} points",
                self.frequencies.len()
            ))
        })?;

        let result = PyDict::new(py);
        result.set_item(
            "input_center",
            pyo3::types::PyComplex::from_doubles(
                py,
                analysis.input_stability_center.re,
                analysis.input_stability_center.im,
            ),
        )?;
        result.set_item("input_radius", analysis.input_stability_radius)?;
        result.set_item("input_stable_inside", analysis.input_stable_inside)?;
        result.set_item(
            "output_center",
            pyo3::types::PyComplex::from_doubles(
                py,
                analysis.output_stability_center.re,
                analysis.output_stability_center.im,
            ),
        )?;
        result.set_item("output_radius", analysis.output_stability_radius)?;
        result.set_item("output_stable_inside", analysis.output_stable_inside)?;
        Ok(result)
    }

    /// Render this sweep as a Touchstone v1 document.
    ///
    /// Args:
    ///     format: "ri" (real/imaginary, default), "ma" (magnitude/angle),
    ///             or "db" (dB magnitude/angle). Angles are in degrees.
    ///     frequency_unit: "hz", "khz", "mhz", or "ghz" (default "ghz")
    ///     comments: Optional `!` comment lines written above the option line
    ///
    /// Returns:
    ///     str: The complete Touchstone document
    ///
    /// Raises:
    ///     ValueError: For an unknown format or frequency unit, or when the
    ///                 ports do not share one reference impedance — Touchstone
    ///                 v1 has no way to express per-port normalization.
    ///
    /// Example:
    ///     >>> open("dut.s2p", "w").write(sparams.to_touchstone(format="ma"))
    #[pyo3(signature = (*, format="ri", frequency_unit="ghz", comments=None))]
    fn to_touchstone(
        &self,
        format: &str,
        frequency_unit: &str,
        comments: Option<Vec<String>>,
    ) -> PyResult<String> {
        crate::export::touchstone(
            &crate::export::TouchstoneInput {
                frequencies: &self.frequencies,
                parameters: &self.parameters,
                reference_impedances: &self.reference_impedances,
                comments: comments.as_deref().unwrap_or(&[]),
            },
            crate::export::TouchstoneFormat::parse(format).map_err(crate::errors::value_error)?,
            crate::export::TouchstoneFrequencyUnit::parse(frequency_unit)
                .map_err(crate::errors::value_error)?,
        )
        .map_err(crate::errors::value_error)
    }

    /// Write this sweep to a Touchstone file.
    ///
    /// Same arguments as `to_touchstone`, plus the destination path. The
    /// caller chooses the filename; `touchstone_extension` gives the
    /// conventional suffix for this port count.
    ///
    /// Example:
    ///     >>> sparams.write_touchstone(f"dut.{sparams.touchstone_extension}")
    #[pyo3(signature = (path, *, format="ri", frequency_unit="ghz", comments=None))]
    fn write_touchstone(
        &self,
        path: PathBuf,
        format: &str,
        frequency_unit: &str,
        comments: Option<Vec<String>>,
    ) -> PyResult<()> {
        let document = self.to_touchstone(format, frequency_unit, comments)?;
        write_export_file(&path, document.as_bytes())
    }

    #[getter]
    fn sopt<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        self.optimum_source_reflection(py)
    }

    fn __repr__(&self) -> String {
        format!(
            "SParameterResult(ports={}, points={}, noise={})",
            self.port_names.len(),
            self.frequencies.len(),
            self.noise.is_some()
        )
    }
}
