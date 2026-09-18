//! Descriptor-only control output plus bounded complex trace windows.

use super::*;
use crate::document::SignalUnitView;
use rspice_core::engine::{
    ControlPresentation, ControlPresentationKind, ControlTrace, ControlVector,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlPresentationDescriptor<'a> {
    pub kind: &'static str,
    pub line: usize,
    pub title: Option<&'a str>,
    pub x_limits: Option<[f64; 2]>,
    pub y_limits: Option<[f64; 2]>,
    pub x_logarithmic: bool,
    pub y_logarithmic: bool,
    pub traces: Vec<ControlTraceDescriptor<'a>>,
    pub changed_vectors: Vec<ControlChangedVector<'a>>,
    pub changed_unit: Option<SignalUnitView>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ControlTraceDescriptor<'a> {
    pub x: ControlVectorDescriptor<'a>,
    pub y: ControlVectorDescriptor<'a>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlVectorDescriptor<'a> {
    pub expression: &'a str,
    pub dataset: &'a str,
    pub result_index: usize,
    pub unit: SignalUnitView,
    pub point_count: usize,
    /// These result/owner pairs identify separate charge events and coverage
    /// in the result document. They do not define a nonlinear impulse transform.
    pub current_sources: Vec<ControlCurrentDescriptor<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlCurrentDescriptor<'a> {
    pub result_index: usize,
    pub owner: &'a rspice_core::CurrentImpulseOwner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlChangedVector<'a> {
    pub result_index: usize,
    pub signal: &'a str,
}

impl WasmResultHandle {
    pub(crate) fn with_control(
        mut self,
        datasets: Vec<String>,
        presentations: Vec<ControlPresentation>,
    ) -> DetailedWasmResult<Self> {
        if (!datasets.is_empty() && datasets.len() != self.results.len())
            || (datasets.is_empty() && !presentations.is_empty())
        {
            return Err(window_error(
                "control datasets do not match retained results".into(),
            ));
        }
        self.control_datasets = datasets;
        self.control_presentations = presentations;
        Ok(self)
    }

    fn control_result_index(&self, dataset: &str) -> DetailedWasmResult<usize> {
        self.control_datasets
            .iter()
            .position(|name| name == dataset)
            .ok_or_else(|| {
                window_error(format!(
                    "control dataset '{dataset}' has no retained result"
                ))
            })
    }

    fn control_vector_metadata<'a>(
        &'a self,
        vector: &'a ControlVector,
    ) -> DetailedWasmResult<ControlVectorDescriptor<'a>> {
        Ok(ControlVectorDescriptor {
            expression: &vector.expression,
            dataset: &vector.dataset,
            result_index: self.control_result_index(&vector.dataset)?,
            unit: SignalUnitView::project(&vector.unit)?,
            point_count: vector.samples.len(),
            current_sources: vector
                .current_sources
                .iter()
                .map(|source| {
                    Ok(ControlCurrentDescriptor {
                        result_index: self.control_result_index(&source.dataset)?,
                        owner: &source.owner,
                    })
                })
                .collect::<DetailedWasmResult<_>>()?,
        })
    }

    pub(super) fn control_metadata(
        &self,
    ) -> DetailedWasmResult<Vec<ControlPresentationDescriptor<'_>>> {
        self.control_presentations
            .iter()
            .map(|presentation| {
                let mut descriptor = ControlPresentationDescriptor {
                    kind: "print",
                    line: presentation.command.line,
                    title: None,
                    x_limits: None,
                    y_limits: None,
                    x_logarithmic: false,
                    y_logarithmic: false,
                    traces: Vec::new(),
                    changed_vectors: Vec::new(),
                    changed_unit: None,
                };
                let traces = match &presentation.kind {
                    ControlPresentationKind::Plot { traces, options } => {
                        descriptor.kind = "plot";
                        descriptor.title = options.title.as_deref();
                        descriptor.x_limits = options.x_limits;
                        descriptor.y_limits = options.y_limits;
                        descriptor.x_logarithmic = options.x_logarithmic;
                        descriptor.y_logarithmic = options.y_logarithmic;
                        traces.as_slice()
                    }
                    ControlPresentationKind::Print(traces) => traces.as_slice(),
                    ControlPresentationKind::UnitsChanged { vectors, unit } => {
                        descriptor.kind = "settype";
                        descriptor.changed_unit = Some(SignalUnitView::project(unit)?);
                        descriptor.changed_vectors = vectors
                            .iter()
                            .map(|vector| {
                                Ok(ControlChangedVector {
                                    result_index: self.control_result_index(&vector.dataset)?,
                                    signal: &vector.signal,
                                })
                            })
                            .collect::<DetailedWasmResult<_>>()?;
                        &[]
                    }
                };
                descriptor.traces = traces
                    .iter()
                    .map(|trace| {
                        Ok(ControlTraceDescriptor {
                            x: self.control_vector_metadata(&trace.x)?,
                            y: self.control_vector_metadata(&trace.y)?,
                        })
                    })
                    .collect::<DetailedWasmResult<_>>()?;
                Ok(descriptor)
            })
            .collect()
    }

    fn control_trace(
        &self,
        presentation: usize,
        trace: usize,
    ) -> DetailedWasmResult<&ControlTrace> {
        let traces = self
            .control_presentations
            .get(presentation)
            .and_then(|presentation| match &presentation.kind {
                ControlPresentationKind::Plot { traces, .. }
                | ControlPresentationKind::Print(traces) => Some(traces),
                ControlPresentationKind::UnitsChanged { .. } => None,
            });
        traces.and_then(|traces| traces.get(trace)).ok_or_else(|| {
            window_error(format!(
                "no control trace {trace} in presentation {presentation}"
            ))
        })
    }

    /// Four finite columns: x real/imaginary, then y real/imaginary.
    fn control_window(
        &self,
        presentation: usize,
        trace: usize,
        start: usize,
        count: usize,
    ) -> DetailedWasmResult<[Vec<f64>; 4]> {
        if count == 0 || count.saturating_mul(4) > self.maximum_window_values {
            return Err(window_error(
                "control window exceeds the numeric transfer budget or is empty".into(),
            ));
        }
        let trace = self.control_trace(presentation, trace)?;
        let end = start
            .checked_add(count)
            .ok_or_else(|| window_error("control window index overflow".into()))?;
        let x = trace
            .x
            .samples
            .get(start..end)
            .ok_or_else(|| window_error("control x window is out of range".into()))?;
        let y = trace
            .y
            .samples
            .get(start..end)
            .ok_or_else(|| window_error("control y window is out of range".into()))?;
        Ok([
            x.iter().map(|value| value.re).collect(),
            x.iter().map(|value| value.im).collect(),
            y.iter().map(|value| value.re).collect(),
            y.iter().map(|value| value.im).collect(),
        ])
    }
}

#[wasm_bindgen]
impl WasmResultHandle {
    /// Resolve one plot/print trace window without copying its entire dataset.
    /// Returns {xReal, xImaginary, yReal, yImaginary}, each a Float64Array.
    #[wasm_bindgen(js_name = controlWindow)]
    pub fn control_window_js(
        &self,
        presentation_index: usize,
        trace_index: usize,
        start: usize,
        count: usize,
    ) -> Result<JsValue, JsValue> {
        let columns = self
            .control_window(presentation_index, trace_index, start, count)
            .map_err(|error| wasm_error_to_js(*error))?;
        let output = js_sys::Object::new();
        for (name, values) in ["xReal", "xImaginary", "yReal", "yImaginary"]
            .into_iter()
            .zip(columns)
        {
            js_sys::Reflect::set(
                &output,
                &JsValue::from_str(name),
                &js_sys::Float64Array::from(values.as_slice()),
            )?;
        }
        Ok(output.into())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic)]
mod tests {
    use super::*;
    use crate::{
        WasmExecutionOptions, run_authored_deck_document_detailed,
        run_authored_deck_document_with_options_and_abort_detailed,
    };

    fn retain(execution: crate::DeckExecution) -> WasmResultHandle {
        WasmResultHandle::new(
            &execution.plan,
            execution.coordinates,
            execution.results,
            crate::options::browser_resource_limits(),
        )
        .unwrap()
        .with_control(execution.control_datasets, execution.control_presentations)
        .unwrap()
    }

    #[test]
    fn control_documents_preserve_order_complex_windows_and_transfer_bounds() {
        let source = "ordered browser script\nV1 a 0 1 ac 1\nR1 a out 1k\nC1 out 0 1u\n.control\nforeach bias 1 2\nalter v1 $bias\nop\nprint v(out)\nend\nac lin 3 100 300\nplot v(out)\n.endc\n.end\n";
        let handle = retain(run_authored_deck_document_detailed(source).unwrap());
        let metadata = handle.metadata_snapshot().unwrap();
        assert_eq!(metadata.control_datasets, ["op1", "op2", "ac1"]);
        assert_eq!(
            handle
                .documents()
                .iter()
                .map(|document| document.analysis().tag())
                .collect::<Vec<_>>(),
            ["op-001", "op-002", "ac-001"]
        );
        assert_eq!(metadata.planned_analyses.len(), 3);
        assert_eq!(metadata.control_presentations.len(), 3);
        for (index, bias) in [1.0, 2.0].into_iter().enumerate() {
            let window = handle.control_window(index, 0, 0, 1).unwrap();
            assert!((window[2][0] - bias).abs() < 1e-8);
            assert_eq!(
                metadata.control_presentations[index].traces[0]
                    .y
                    .result_index,
                index
            );
        }
        let window = handle.control_window(2, 0, 0, 2).unwrap();
        assert_eq!(window[0], [100.0, 200.0]);
        for (index, frequency) in window[0].iter().enumerate() {
            let omega_rc = std::f64::consts::TAU * frequency * 1e-3;
            assert!((window[2][index] - 1.0 / (1.0 + omega_rc * omega_rc)).abs() < 1e-8);
            assert!((window[3][index] + omega_rc / (1.0 + omega_rc * omega_rc)).abs() < 1e-8);
        }
        assert!(handle.control_window(2, 0, 0, 0).is_err());
        assert!(handle.control_window(2, 0, 2, 2).is_err());
        assert!(handle.control_window(3, 0, 0, 1).is_err());
        assert!(
            handle
                .control_window(2, 0, 0, handle.maximum_window_values / 4 + 1)
                .is_err()
        );
        let serialized = serde_json::to_string(&metadata).unwrap();
        assert!(!serialized.contains("samples"));
    }

    #[test]
    fn control_memristor_documents_retain_each_run_and_current_dependencies() {
        let source = include_str!("../../../../tests/paranoia/memristor/memristor.sp");
        let handle = retain(run_authored_deck_document_detailed(source).unwrap());
        let metadata = handle.metadata_snapshot().unwrap();
        assert_eq!(metadata.control_datasets, ["tran1", "tran2", "tran3"]);
        assert_eq!(metadata.control_presentations.len(), 5);
        assert_eq!(metadata.control_presentations[1].kind, "settype");
        assert_eq!(
            metadata.control_presentations[2].traces[0].y.unit,
            SignalUnitView::Ohm
        );
        for (index, frequency) in [1e8, 1.1e8, 1.4e8].into_iter().enumerate() {
            let result = &handle.documents()[index];
            assert_eq!(result.analysis().tag(), format!("tran-{:03}", index + 1));
            // The authored plot lists the current run, then tran1 and tran2.
            let trace_index = [1, 2, 0][index];
            let descriptor = &metadata.control_presentations[4].traces[trace_index];
            assert_eq!(descriptor.y.current_sources[0].result_index, index);
            assert_eq!(descriptor.y.unit, SignalUnitView::Ampere);
            assert_eq!(descriptor.x.point_count, result.point_count());
            let trace = handle.control_trace(4, trace_index).unwrap();
            let window = result.window(0, result.point_count()).unwrap();
            let rspice_core::execution::result_document::AxisValues::Real { values: times } =
                &window.axes[0].values
            else {
                panic!("time axis");
            };
            assert!((times.last().unwrap() * frequency - 1.0).abs() < 1e-8);
            for (time, voltage) in times.iter().zip(&trace.x.samples) {
                assert!(
                    (voltage.re + 3.0 * (std::f64::consts::TAU * frequency * time).sin()).abs()
                        < 1e-8
                );
            }
        }
    }

    #[test]
    fn control_failure_keeps_source_line_and_limits_retained_presentations() {
        let source =
            "late browser failure\nV1 a 0 1\nR1 a 0 1k\n.control\nop\nplot missing\n.endc\n.end\n";
        let error = run_authored_deck_document_detailed(source).unwrap_err();
        assert_eq!(error.primary_line, Some(6));
        assert_eq!(error.code, "control.command");
        let source = source.replace("plot missing", "repeat 100\nplot v(a)\nend");
        let mut options = WasmExecutionOptions::default();
        options.resource_limits.max_result_values = 64;
        let error = run_authored_deck_document_with_options_and_abort_detailed(
            &source,
            &options,
            &rspice_core::NoAbort,
        )
        .unwrap_err();
        assert_eq!(error.category, "resource_limit");
    }
}
