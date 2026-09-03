//! Third-order Volterra distortion results (`.DISTO`).
//!
//! Products are addressed by the stable labels `2f1`, `3f1`, `f1+f2`, `f1-f2`,
//! and `2f1-f2` rather than by ordinal, so a pickled result stays readable
//! across builds and a caller's string matches the deck's own notation.

use super::*;

/// Require one distortion response set to publish the fundamental's schema.
///
/// The result addresses every response through a single node/branch namespace,
/// so a response that names different signals is a malformed result, not a
/// response to be indexed positionally and hoped for.
fn require_matching_distortion_schema(
    response: &str,
    expected: (&[String], &[String]),
    actual: (&[String], &[String]),
) -> PyResult<()> {
    if expected.0 == actual.0 && expected.1 == actual.1 {
        return Ok(());
    }
    Err(crate::errors::SimulationError::new_err(format!(
        "malformed distortion result: the '{response}' response publishes \
         {} node and {} branch signals, but the F1 fundamental publishes {} and {}",
        actual.0.len(),
        actual.1.len(),
        expected.0.len(),
        expected.1.len()
    )))
}

/// Third-order Volterra distortion sweep.
///
/// Every returned `AcResult` contains actual sinusoidal peak phasors at the
/// physical product frequency. They are not internal Volterra kernels or
/// pre-normalized distortion ratios.
#[pyclass(name = "DistortionResult", module = "rspice")]
#[derive(Debug)]
pub struct PyDistortionResult {
    f2_over_f1: Option<f64>,
    f1_frequencies: Vec<f64>,
    fundamental_f1: Vec<AcResult>,
    fundamental_f2: Option<Vec<AcResult>>,
    products: Vec<(DistortionProduct, Vec<AcResult>)>,
    node_names: Vec<String>,
    branch_names: Vec<String>,
    /// The core sweep, kept because this projection re-lays every point's
    /// products into product-major rows the shared projection cannot read back.
    evidence: Option<DocumentEvidence<DistortionAnalysisResult>>,
}

impl CarriesDocumentEvidence for PyDistortionResult {
    fn bind_execution(
        &mut self,
        analysis: rspice_core::execution::AnalysisInstanceId,
        coordinate: Option<&rspice_core::execution::ResultCoordinate>,
    ) {
        self.evidence = self
            .evidence
            .take()
            .map(|evidence| evidence.with_execution(analysis, coordinate));
    }
}

impl PyDistortionResult {
    /// The shared result document, projected from the retained sweep.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "distortion")?;
        let coordinate = evidence.coordinate.clone();
        let analysis = evidence.analysis;
        let result = &evidence.core;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_distortion(analysis, result)
        })
    }

    pub fn from_core(result: &DistortionAnalysisResult) -> PyResult<Self> {
        if result.points.is_empty() {
            return Err(crate::errors::value_error(
                "malformed distortion result: no F1 points",
            ));
        }
        let product_kinds: &[DistortionProduct] = if result.is_two_tone() {
            &[
                DistortionProduct::Sum,
                DistortionProduct::Difference,
                DistortionProduct::ThirdOrderDifference,
            ]
        } else {
            &[
                DistortionProduct::SecondHarmonic,
                DistortionProduct::ThirdHarmonic,
            ]
        };
        let f1_frequencies = result
            .points
            .iter()
            .map(|point| point.fundamental_f1.frequency)
            .collect();
        let fundamental_f1: Vec<_> = result
            .points
            .iter()
            .map(|point| point.fundamental_f1.clone())
            .collect();
        let fundamental_f2 = if result.is_two_tone() {
            Some(
                result
                    .points
                    .iter()
                    .enumerate()
                    .map(|(index, point)| {
                        point.fundamental_f2.clone().ok_or_else(|| {
                            crate::errors::value_error(format!(
                                "malformed distortion result: missing F2 response at F1 index {index}"
                            ))
                        })
                    })
                    .collect::<PyResult<Vec<_>>>()?,
            )
        } else {
            None
        };
        let products = product_kinds
            .iter()
            .map(|&kind| {
                let rows = result
                    .points
                    .iter()
                    .enumerate()
                    .map(|(index, point)| {
                        point
                            .product(kind)
                            .map(|value| value.response.clone())
                            .ok_or_else(|| {
                                crate::errors::value_error(format!(
                                    "malformed distortion result: missing '{}' response at F1 index {index}",
                                    kind.label()
                                ))
                            })
                    })
                    .collect::<PyResult<Vec<_>>>()?;
                Ok((kind, rows))
            })
            .collect::<PyResult<Vec<_>>>()?;
        // Every response this result publishes is addressed through one set of
        // node and branch names, so every response must actually carry it. The
        // fundamental establishes the schema and each F2 and product response
        // is proved against it rather than indexed hopefully.
        let (node_names, branch_names) =
            crate::results::validated_ac_schema("distortion F1 fundamental", &fundamental_f1)
                .map_err(crate::errors::SimulationError::new_err)?;
        if let Some(rows) = &fundamental_f2 {
            let (f2_nodes, f2_branches) =
                crate::results::validated_ac_schema("distortion F2 fundamental", rows)
                    .map_err(crate::errors::SimulationError::new_err)?;
            require_matching_distortion_schema(
                "F2 fundamental",
                (&node_names, &branch_names),
                (&f2_nodes, &f2_branches),
            )?;
        }
        for (kind, rows) in &products {
            let (product_nodes, product_branches) = crate::results::validated_ac_schema(
                &format!("distortion '{}' product", kind.label()),
                rows,
            )
            .map_err(crate::errors::SimulationError::new_err)?;
            require_matching_distortion_schema(
                kind.label(),
                (&node_names, &branch_names),
                (&product_nodes, &product_branches),
            )?;
        }
        Ok(Self {
            f2_over_f1: result.f2_over_f1,
            f1_frequencies,
            fundamental_f1,
            fundamental_f2,
            products,
            node_names,
            branch_names,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Distortion,
                result.clone(),
            )),
        })
    }

    fn parse_product(&self, name: &str) -> PyResult<DistortionProduct> {
        let normalized = name.trim().to_ascii_lowercase().replace([' ', '_'], "");
        let product = match normalized.as_str() {
            "2f1" | "hd2" | "secondharmonic" => DistortionProduct::SecondHarmonic,
            "3f1" | "hd3" | "thirdharmonic" => DistortionProduct::ThirdHarmonic,
            "f1+f2" | "sum" | "im2sum" => DistortionProduct::Sum,
            "f1-f2" | "difference" | "im2difference" => DistortionProduct::Difference,
            "2f1-f2" | "im3" | "thirdorderdifference" => DistortionProduct::ThirdOrderDifference,
            _ => {
                return Err(crate::errors::value_error(format!(
                    "unknown distortion product '{name}'; available products: {}",
                    self.available_product_labels().join(", ")
                )));
            }
        };
        if self.products.iter().any(|(kind, _)| *kind == product) {
            Ok(product)
        } else {
            Err(crate::errors::value_error(format!(
                "distortion product '{}' is not available in {} mode; available products: {}",
                product.label(),
                if self.f2_over_f1.is_some() {
                    "two-tone"
                } else {
                    "harmonic"
                },
                self.available_product_labels().join(", ")
            )))
        }
    }

    fn available_product_labels(&self) -> Vec<String> {
        self.products
            .iter()
            .map(|(product, _)| product.label().to_string())
            .collect()
    }

    fn product_rows(&self, product: DistortionProduct) -> PyResult<&[AcResult]> {
        self.products
            .iter()
            .find(|(kind, _)| *kind == product)
            .map(|(_, rows)| rows.as_slice())
            .ok_or_else(|| {
                crate::errors::value_error("distortion result is missing a product series")
            })
    }

    fn validate_series_length(&self, label: &str, rows: &[AcResult]) -> PyResult<()> {
        if rows.len() == self.fundamental_f1.len() {
            Ok(())
        } else {
            Err(crate::errors::value_error(format!(
                "malformed distortion result: {label} has {} rows for {} F1 points",
                rows.len(),
                self.fundamental_f1.len()
            )))
        }
    }

    fn resolve_node(&self, node: &NodeIdentifier) -> AccessResult<usize> {
        match node {
            NodeIdentifier::Index(index) => {
                let count = self
                    .fundamental_f1
                    .first()
                    .map(|row| row.voltages.len())
                    .unwrap_or(self.node_names.len());
                if *index <= count {
                    Ok(*index)
                } else {
                    Err(invalid_node_index_error(*index, count))
                }
            }
            NodeIdentifier::Name(name) => {
                if is_ground_name(name) {
                    return Ok(0);
                }
                self.node_names
                    .iter()
                    .position(|candidate| candidate.eq_ignore_ascii_case(name))
                    .map(|index| index + 1)
                    .ok_or_else(|| unknown_node_name_error(name))
            }
        }
    }

    fn resolve_branch(&self, name: &str) -> AccessResult<usize> {
        self.branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))
            .ok_or_else(|| unknown_branch_name_error(name))
    }

    fn voltage_ratio_values(
        &self,
        product: DistortionProduct,
        node: &NodeIdentifier,
    ) -> PyResult<Vec<f64>> {
        let node = self.resolve_node(node).map_err(PyErr::from)?;
        let product_rows = self.product_rows(product)?;
        self.validate_series_length(product.label(), product_rows)?;
        product_rows
            .iter()
            .zip(self.fundamental_f1.iter())
            .enumerate()
            .map(|(index, (numerator, denominator))| {
                if node == 0 {
                    return Ok(0.0);
                }
                let numerator = numerator.voltages.get(node - 1).ok_or_else(|| {
                    crate::errors::value_error(format!(
                        "malformed distortion product '{}' at F1 index {index}: missing node {node}",
                        product.label()
                    ))
                })?;
                let denominator = denominator.voltages.get(node - 1).ok_or_else(|| {
                    crate::errors::value_error(format!(
                        "malformed F1 result at index {index}: missing node {node}"
                    ))
                })?;
                Ok(magnitude_ratio(numerator.norm(), denominator.norm()))
            })
            .collect()
    }

    fn branch_ratio_values(&self, product: DistortionProduct, branch: &str) -> PyResult<Vec<f64>> {
        let branch = self.resolve_branch(branch).map_err(PyErr::from)?;
        let product_rows = self.product_rows(product)?;
        self.validate_series_length(product.label(), product_rows)?;
        product_rows
            .iter()
            .zip(self.fundamental_f1.iter())
            .enumerate()
            .map(|(index, (numerator, denominator))| {
                let numerator = numerator.currents.get(branch).ok_or_else(|| {
                    crate::errors::value_error(format!(
                        "malformed distortion product '{}' at F1 index {index}: missing branch current {branch}",
                        product.label()
                    ))
                })?;
                let denominator = denominator.currents.get(branch).ok_or_else(|| {
                    crate::errors::value_error(format!(
                        "malformed F1 result at index {index}: missing branch current {branch}"
                    ))
                })?;
                Ok(magnitude_ratio(numerator.norm(), denominator.norm()))
            })
            .collect()
    }
}

fn magnitude_ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        if numerator == 0.0 { 0.0 } else { f64::INFINITY }
    } else {
        numerator / denominator
    }
}

#[pymethods]
impl PyDistortionResult {
    /// Typed inventory of every signal in this result's shared document.
    ///
    /// The descriptors are the ones the CLI, the WASM build and the engine
    /// adapter publish, so a canonical name, unit, owner, or availability
    /// means the same thing on every surface.
    fn signals(&self, py: Python<'_>) -> PyResult<Vec<PySignalDescriptor>> {
        Ok(document::signals(&self.shared_document(py)?))
    }

    /// Every analysis-owned scalar this result publishes, with its unit.
    fn scalars(&self, py: Python<'_>) -> PyResult<Vec<PyResultScalar>> {
        Ok(document::scalars(&self.shared_document(py)?))
    }

    /// Every per-device observable history this result captured.
    fn device_observables(&self, py: Python<'_>) -> PyResult<Vec<PyDeviceObservable>> {
        Ok(document::device_observables(&self.shared_document(py)?))
    }

    /// The whole shared result document as JSON-serializable Python data.
    fn document<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        document::json_view(py, &self.shared_document(py)?)
    }

    /// Swept F1 frequencies in Hz.
    #[getter]
    fn f1_frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.f1_frequencies.to_pyarray(py)
    }

    /// Fixed F2 frequency in Hz in two-tone mode, otherwise None.
    #[getter]
    fn f2_frequency(&self) -> Option<f64> {
        self.fundamental_f2
            .as_ref()
            .and_then(|rows| rows.first())
            .map(|row| row.frequency)
    }

    #[getter]
    fn f2_over_f1(&self) -> Option<f64> {
        self.f2_over_f1
    }

    #[getter]
    fn is_two_tone(&self) -> bool {
        self.f2_over_f1.is_some()
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.f1_frequencies.len()
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.node_names.clone()
    }

    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.branch_names.clone()
    }

    /// Canonical product names accepted by `product()` for this mode.
    #[getter]
    fn available_products(&self) -> Vec<String> {
        self.available_product_labels()
    }

    /// Actual first-order F1 response, aligned with `f1_frequencies`.
    #[getter]
    fn fundamental_f1(&self) -> PyResult<PyAcResult> {
        PyAcResult::new(self.f1_frequencies.clone(), self.fundamental_f1.clone())
    }

    /// Actual first-order F2 response at each F1 sweep point.
    ///
    /// F2 is fixed by SPICE's two-tone contract, so its frequency array
    /// repeats the same value. Returns None in harmonic mode.
    #[getter]
    fn fundamental_f2(&self) -> PyResult<Option<PyAcResult>> {
        self.fundamental_f2
            .as_ref()
            .map(|rows| {
                PyAcResult::new(rows.iter().map(|row| row.frequency).collect(), rows.clone())
            })
            .transpose()
    }

    /// Actual complex response for a spectral product.
    fn product(&self, name: &str) -> PyResult<PyAcResult> {
        let product = self.parse_product(name)?;
        let rows = self.product_rows(product)?;
        self.validate_series_length(product.label(), rows)?;
        PyAcResult::new(
            rows.iter().map(|row| row.frequency).collect(),
            rows.to_vec(),
        )
    }

    /// |V(product)| / |V(F1)| across the F1 sweep.
    fn voltage_ratio<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let product = self.parse_product(name)?;
        Ok(self.voltage_ratio_values(product, &node)?.to_pyarray(py))
    }

    /// Product voltage relative to F1 in dBc (20*log10 of the ratio).
    fn voltage_db_relative<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let product = self.parse_product(name)?;
        let values = self
            .voltage_ratio_values(product, &node)?
            .into_iter()
            .map(|ratio| 20.0 * ratio.log10())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    /// |I(product)| / |I(F1)| for a named MNA branch across the sweep.
    fn branch_current_ratio<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        branch: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let product = self.parse_product(name)?;
        Ok(self.branch_ratio_values(product, branch)?.to_pyarray(py))
    }

    /// Product branch current relative to F1 in dBc.
    fn branch_current_db_relative<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        branch: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let product = self.parse_product(name)?;
        let values = self
            .branch_ratio_values(product, branch)?
            .into_iter()
            .map(|ratio| 20.0 * ratio.log10())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn __repr__(&self) -> String {
        format!(
            "DistortionResult(mode={}, points={}, products=[{}])",
            if self.is_two_tone() {
                "two-tone"
            } else {
                "harmonic"
            },
            self.num_points(),
            self.available_product_labels().join(", ")
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        f2_over_f1: Option<f64>,
        f1_frequencies: Vec<f64>,
        fundamental_f1: Vec<AcRowState>,
        fundamental_f2: Option<Vec<AcRowState>>,
        products: Vec<DistortionProductState>,
        node_names: Vec<String>,
        branch_names: Vec<String>,
    ) -> PyResult<Self> {
        let products = products
            .into_iter()
            .map(|(label, rows)| {
                let rows: Vec<AcResult> = rows.into_iter().map(rebuild_ac_row).collect();
                Ok((distortion_product_from_label(&label)?, rows))
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            f2_over_f1,
            f1_frequencies,
            fundamental_f1: fundamental_f1.into_iter().map(rebuild_ac_row).collect(),
            fundamental_f2: fundamental_f2
                .map(|rows| rows.into_iter().map(rebuild_ac_row).collect()),
            products,
            node_names,
            branch_names,
            evidence: None,
        })
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            Option<f64>,
            Vec<f64>,
            Vec<AcRowState>,
            Option<Vec<AcRowState>>,
            Vec<DistortionProductState>,
            Vec<String>,
            Vec<String>,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.f2_over_f1,
                self.f1_frequencies.clone(),
                self.fundamental_f1.iter().map(ac_row_state).collect(),
                self.fundamental_f2
                    .as_ref()
                    .map(|rows| rows.iter().map(ac_row_state).collect()),
                self.products
                    .iter()
                    .map(|(product, rows)| {
                        (
                            product.label().to_string(),
                            rows.iter().map(ac_row_state).collect(),
                        )
                    })
                    .collect(),
                self.node_names.clone(),
                self.branch_names.clone(),
            ),
        ))
    }
}
