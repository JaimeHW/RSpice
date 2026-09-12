//! One solved DC operating point and its per-device projection.
//!
//! `SimulationResult` is also the point type a DC sweep, a `.STEP` run and a
//! `.TEMP` run are made of, so it is deliberately free of any notion of a
//! sweep axis.

use super::*;

/// DC operating point simulation result
///
/// Contains node voltages and branch currents from a DC operating point
/// analysis. Access voltages by node index or name.
///
/// Example:
///     >>> result = engine.run_dc_op(netlist)
///     >>> v1 = result.voltage(1)
///     >>> v_out = result.voltage("out")
#[pyclass(name = "SimulationResult", module = "rspice")]
pub struct PySimulationResult {
    pub(crate) inner: SimulationResult,
    /// `None` when the producing analysis captured no device operating-point
    /// report at all, which is a different fact from "the circuit has no
    /// devices with operating points". An empty list means the latter.
    device_operating_points: Option<Vec<PyDeviceOperatingPoint>>,
    /// The core device report the shared document's device observables come
    /// from, kept beside its Python projection because the projection maps
    /// the report's static parameter names to owned strings and cannot be
    /// turned back into one.
    evidence: Option<DocumentEvidence<Option<rspice_core::circuit::DeviceOpReport>>>,
}

impl CarriesDocumentEvidence for PySimulationResult {
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

impl PySimulationResult {
    /// A solution whose producer captured no device operating-point report.
    pub fn new(inner: SimulationResult) -> Self {
        Self {
            inner,
            device_operating_points: None,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Op,
                None,
            )),
        }
    }

    pub fn new_with_report(
        inner: SimulationResult,
        report: rspice_core::circuit::DeviceOpReport,
    ) -> Self {
        Self {
            inner,
            device_operating_points: Some(
                report
                    .entries
                    .iter()
                    .cloned()
                    .map(PyDeviceOperatingPoint::from_core)
                    .collect(),
            ),
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Op,
                Some(report),
            )),
        }
    }

    pub(super) fn new_with_device_operating_points(
        inner: SimulationResult,
        device_operating_points: Option<Vec<PyDeviceOperatingPoint>>,
    ) -> Self {
        Self {
            inner,
            device_operating_points,
            // Only a route that ran the analysis holds the core device report
            // the shared document needs, so a result rebuilt from a Python
            // projection carries no document evidence.
            evidence: None,
        }
    }

    /// The shared result document, projected from the retained solution.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "operating-point")?;
        let coordinate = evidence.coordinate.clone();
        let analysis = evidence.analysis;
        let report = evidence.core.as_ref();
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_operating_point(analysis, &self.inner, report)
        })
    }

    fn checked_voltage(&self, node: usize) -> AccessResult<f64> {
        checked_simulation_voltage(&self.inner, node)
    }

    fn checked_voltage_named(&self, name: &str) -> AccessResult<f64> {
        checked_simulation_voltage_named(&self.inner, name)
    }
}

#[pymethods]
impl PySimulationResult {
    /// Get voltage at a node by index or name
    ///
    /// Args:
    ///     node: Node index (int) or node name (str)
    ///
    /// Returns:
    ///     float: Voltage at the specified node
    ///
    /// Raises:
    ///     IndexError: If the node index is out of range
    ///     KeyError: If the node name does not exist
    ///
    /// Example:
    ///     >>> v = result.voltage(1)      # By index
    ///     >>> v = result.voltage("out")  # By name
    fn voltage(&self, node: NodeIdentifier) -> PyResult<f64> {
        match node {
            NodeIdentifier::Index(idx) => self.checked_voltage(idx),
            NodeIdentifier::Name(name) => self.checked_voltage_named(&name),
        }
        .map_err(PyErr::from)
    }

    /// Get all node voltages as a NumPy array
    ///
    /// The array stays aligned with `node_names`, so a net only the event
    /// domain resolves keeps its position and reads back as `nan`: it owns an
    /// MNA placeholder row rather than a solved level, and publishing that
    /// row would publish 0 V for a net that carries events. Read such a net
    /// through a transient run's event trace; asking `voltage()` for it
    /// raises `KeyError` with the same explanation.
    ///
    /// Returns:
    ///     numpy.ndarray: Array of all node voltages (index 0 = ground = 0V)
    #[getter]
    fn node_voltages<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        let voltages: Vec<f64> = self
            .inner
            .node_voltages
            .iter()
            .enumerate()
            .map(|(node, value)| {
                if self.inner.event_only_node_kind(node).is_some() {
                    f64::NAN
                } else {
                    *value
                }
            })
            .collect();
        voltages.to_pyarray(py)
    }

    /// Get all node names
    ///
    /// Returns:
    ///     list[str]: List of node names indexed by node ID
    #[getter]
    pub fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    /// Get branch current by element name
    ///
    /// Args:
    ///     name: Element name (e.g., "V1", "L1")
    ///
    /// Returns:
    ///     float: Current through the element
    ///
    /// Raises:
    ///     KeyError: If no branch carries that name
    fn branch_current(&self, name: &str) -> PyResult<f64> {
        self.inner
            .branch_current_named(name)
            .ok_or_else(|| unknown_branch_name_error(name))
            .map_err(PyErr::from)
    }

    /// Per-device operating-point summaries captured by DC OP analysis.
    ///
    /// `None` when the analysis that produced this solution captured no
    /// report; an empty list when it captured one and the circuit has no
    /// device with an operating point. The two are not the same fact.
    #[getter]
    fn device_operating_points(&self) -> Option<Vec<PyDeviceOperatingPoint>> {
        self.device_operating_points.clone()
    }

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

    /// Whether this solution carries a device operating-point report.
    #[getter]
    fn has_device_operating_points(&self) -> bool {
        self.device_operating_points.is_some()
    }

    /// Look up one device's operating-point summary (case-insensitive).
    ///
    /// Raises `KeyError` naming the absent report rather than the device when
    /// the producing analysis captured none, so "this run did not record them"
    /// never reads as "your circuit does not contain that device".
    fn device_operating_point(&self, name: &str) -> PyResult<PyDeviceOperatingPoint> {
        let entries = self.device_operating_points.as_ref().ok_or_else(|| {
            crate::errors::key_error(
                "this result carries no device operating-point report; it was produced by an \
                 analysis that does not capture one",
            )
        })?;
        entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
            .cloned()
            .ok_or_else(|| crate::errors::key_error(format!("unknown device '{name}'")))
    }

    /// Get all branch currents as a NumPy array
    ///
    /// Returns:
    ///     numpy.ndarray: Array of all branch currents
    #[getter]
    fn branch_currents<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.branch_currents.to_pyarray(py)
    }

    /// Get canonical branch names aligned with `branch_currents`.
    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.inner.branch_names.clone()
    }

    /// Number of nodes in the circuit (excluding ground)
    #[getter]
    pub fn num_nodes(&self) -> usize {
        self.inner.node_voltages.len().saturating_sub(1)
    }

    fn __repr__(&self) -> String {
        format!(
            "SimulationResult(nodes={}, branches={})",
            self.num_nodes(),
            self.inner.branch_currents.len()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    ///
    /// `event_only_nodes` is optional so a pickle written before a solved
    /// point carried that mask still loads.
    #[staticmethod]
    #[pyo3(signature = (state, device_operating_points, event_only_nodes=None))]
    fn _unpickle(
        state: SimulationResultState,
        device_operating_points: Option<Vec<PyDeviceOperatingPoint>>,
        event_only_nodes: Option<Vec<u8>>,
    ) -> Self {
        let mut inner = rebuild_simulation_result(state);
        restore_event_only_nodes(&mut inner, event_only_nodes);
        Self::new_with_device_operating_points(inner, device_operating_points)
    }

    /// Project this operating point onto a deck's authored output contract
    ///
    /// Returns the columns the deck's `.SAVE`, `.PROBE`, `.PRINT OP` and
    /// `.PLOT OP` cards select, each with its per-sample validity. Device
    /// observables resolve through the same `@device[param]` grammar the CLI
    /// uses. Whole-result access is unaffected.
    ///
    /// Args:
    ///     netlist: The parsed deck whose output cards to apply
    ///
    /// Returns:
    ///     list[ProjectedSignal]: Selected columns in authored order
    ///
    /// Raises:
    ///     RequestedSignalUnavailableError: If an authored symbol is absent
    ///
    /// Example:
    ///     >>> [s.name for s in op.saved_signals(netlist)]
    fn saved_signals(
        &self,
        netlist: &crate::netlist::PyNetlist,
    ) -> PyResult<Vec<crate::results::PyProjectedSignal>> {
        let inventory = rspice_core::execution::operating_point_projection_signals(&self.inner)
            .map_err(|error| crate::errors::value_error(error.to_string()))?;
        let observables = rspice_core::execution::operating_point_observable_series(&self.inner);
        crate::results::projection::project_real(
            &netlist.inner,
            rspice_core::execution::AnalysisResultKind::OperatingPoint,
            "DC OP",
            &[0.0],
            inventory,
            rspice_core::execution::observable_lookup(&observables),
            None,
        )
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            SimulationResultState,
            Option<Vec<PyDeviceOperatingPoint>>,
            Option<Vec<u8>>,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                simulation_result_state(&self.inner),
                self.device_operating_points.clone(),
                event_only_node_state(&self.inner),
            ),
        ))
    }
}

/// Spectre-style operating-point information for one device instance.
#[pyclass(name = "DeviceOperatingPoint", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyDeviceOperatingPoint {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub device_kind: String,
    #[pyo3(get)]
    pub region: Option<String>,
    params: Vec<(String, f64)>,
}

impl PyDeviceOperatingPoint {
    pub(crate) fn from_core(entry: rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: entry.name,
            device_kind: entry.device_kind.to_string(),
            region: entry.region.map(str::to_string),
            params: entry
                .params
                .into_iter()
                .map(|(name, value)| (name.to_string(), value))
                .collect(),
        }
    }
}

#[pymethods]
impl PyDeviceOperatingPoint {
    /// Named operating-point quantities in stable display order.
    #[getter]
    fn params<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let result = PyDict::new(py);
        for (name, value) in &self.params {
            result.set_item(name, value)?;
        }
        Ok(result)
    }

    #[getter]
    fn param_names(&self) -> Vec<String> {
        self.params.iter().map(|(name, _)| name.clone()).collect()
    }

    /// Read one operating-point quantity (case-insensitive).
    fn param(&self, name: &str) -> PyResult<f64> {
        self.params
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
            .ok_or_else(|| {
                crate::errors::key_error(format!(
                    "device '{}' has no operating-point parameter '{name}'",
                    self.name
                ))
            })
    }

    fn __getitem__(&self, name: &str) -> PyResult<f64> {
        self.param(name)
    }

    fn __repr__(&self) -> String {
        format!(
            "DeviceOperatingPoint(name='{}', kind='{}', region={:?}, parameters={})",
            self.name,
            self.device_kind,
            self.region,
            self.params.len()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        name: String,
        device_kind: String,
        region: Option<String>,
        params: Vec<(String, f64)>,
    ) -> Self {
        Self {
            name,
            device_kind,
            region,
            params,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (String, String, Option<String>, Vec<(String, f64)>),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.name.clone(),
                self.device_kind.clone(),
                self.region.clone(),
                self.params.clone(),
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A solved point whose second node carries logic rather than a level.
    fn masked_point() -> SimulationResult {
        let mut result = SimulationResult::new(2, 0);
        result.node_names = vec!["0".to_owned(), "clk".to_owned(), "out".to_owned()];
        result.node_voltages = vec![0.0, 0.0, 1.25];
        result.set_event_only_nodes(vec![None, Some(EventOnlyNetKind::Digital), None]);
        result
    }

    /// Asking a solved DC point for the voltage of an event-only net raises
    /// `KeyError` with the engine's one sentence, not "unknown node" and not
    /// the placeholder row's zero.
    ///
    /// "Unknown node" would send the caller looking for a typo; `0.0` would be
    /// read as a level. The net exists, it was recorded, and it was recorded
    /// in the only domain that resolves it — which is what the sentence says.
    #[test]
    fn an_event_only_net_refuses_the_operating_points_voltage_accessors() {
        let result = masked_point();
        let by_name = checked_simulation_voltage_named(&result, "clk")
            .expect_err("an event-only net has no voltage");
        let by_index =
            checked_simulation_voltage(&result, 1).expect_err("addressing it by index is the same");
        assert_eq!(by_name, by_index);
        assert_eq!(
            by_name,
            ResultAccessError::EventOnlyNode {
                name: "clk".to_owned(),
                kind: EventOnlyNetKind::Digital,
                surface: EventTraceSurface::SolvedPoint,
            }
        );

        // The analog node beside it is unaffected, and a genuinely unknown
        // name still says so.
        assert_eq!(
            checked_simulation_voltage_named(&result, "out"),
            Ok(1.25_f64)
        );
        assert_eq!(
            checked_simulation_voltage_named(&result, "nope"),
            Err(ResultAccessError::UnknownNodeName {
                name: "nope".to_owned()
            })
        );
    }

    /// The carrier the refusal recommends is a spelling a reader can reach.
    ///
    /// A solved DC point publishes no event trace of its own, so the sentence
    /// names a transient run's accessor and says whose it is. Both halves are
    /// checked against the published type stub, which `stubtest` holds to the
    /// real classes.
    #[test]
    fn the_solved_points_refusal_names_an_accessor_the_stub_declares() {
        let stub = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/rspice.pyi"))
            .expect("the published type stub is beside the crate manifest");

        let digital = rspice_core::analysis::transient::event_only_voltage_refusal(
            "clk",
            EventOnlyNetKind::Digital,
            EventTraceSurface::SolvedPoint,
        );
        assert!(
            digital.contains("a transient run's digital_events('clk')"),
            "{digital}"
        );
        assert!(stub.contains("def digital_events("), "the stub declares it");

        let real = rspice_core::analysis::transient::event_only_voltage_refusal(
            "ctrl",
            EventOnlyNetKind::Real,
            EventTraceSurface::SolvedPoint,
        );
        assert!(
            real.contains("a transient run's real_trace('ctrl')"),
            "{real}"
        );
        assert!(stub.contains("def real_trace("), "the stub declares it");
    }

    /// The mask survives a pickle, and its absence is how an older pickle
    /// reads.
    ///
    /// Without the round trip an unpickled operating point would publish the
    /// placeholder row again, which is the whole defect coming back through
    /// the serialization path.
    #[test]
    fn the_event_only_mask_round_trips_through_pickled_state() {
        let result = masked_point();
        let codes = event_only_node_state(&result).expect("the point has an event domain");
        assert_eq!(codes, vec![0, 1, 0]);

        let mut restored = rebuild_simulation_result(simulation_result_state(&result));
        assert_eq!(
            restored.event_only_node_kind(1),
            None,
            "state alone carries no mask"
        );
        restore_event_only_nodes(&mut restored, Some(codes));
        assert_eq!(
            restored.event_only_node_kind(1),
            Some(EventOnlyNetKind::Digital)
        );
        assert_eq!(restored.try_voltage(1), None);
        assert_eq!(restored.try_voltage(2), Some(1.25));

        // A pickle written before the mask existed carries none, and a result
        // with no event domain writes none.
        restore_event_only_nodes(&mut restored, None);
        assert_eq!(
            restored.event_only_node_kind(1),
            Some(EventOnlyNetKind::Digital),
            "an absent mask leaves what is already there alone"
        );
        let analog = SimulationResult::new(2, 0);
        assert_eq!(event_only_node_state(&analog), None);
    }
}
