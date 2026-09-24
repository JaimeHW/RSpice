//! The display spectrum and signed retained state must describe one solution.
use super::*;

pub(super) fn validate_worker_qpss_result(result: &WorkerSimulationResult) -> Result<(), String> {
    let WorkerSimulationResult::Qpss {
        frequencies,
        tuples,
        waveforms,
        operating_point,
    } = result
    else {
        return Ok(());
    };
    operating_point
        .validate_retained_payload_with_abort(
            &rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .map_err(|error| error.to_string())?;
    let SimulationResult::Qpss {
        frequencies: expected_frequencies,
        tuples: expected_tuples,
        waveforms: expected_waveforms,
        ..
    } = SimulationResult::from_qpss_operating_point(Arc::new(operating_point.clone()))?
    else {
        unreachable!("QPSS projection has one result type");
    };
    if *frequencies != expected_frequencies
        || *tuples != expected_tuples
        || *waveforms != worker_waveforms(expected_waveforms)
    {
        return Err("QPSS worker display does not match the retained signed-tuple spectrum".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result() -> SimulationResult {
        let data = crate::services::simulation_runner::run_qpss_analysis_with_source_path_and_abort(
            "QPSS transfer\nV1 in 0 SIN(.1 .2 1k)\nR1 in out 1k\nC1 out 0 1u\nI1 0 out SIN(0 .001 1414.213562373095)\nBmemory memory 0 V=1k*sdt(v(out)-v(memory))\nRmemory memory 0 1k\n.end\n",
            rspice_core::engine::QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]),
            None, &rspice_core::NoAbort,
        ).unwrap();
        SimulationResult::from_qpss_operating_point(data.operating_point).unwrap()
    }

    fn packet() -> WorkerResponseTransport {
        WorkerResponseTransport::from_response(WorkerResponse::from_result_for_transfer(
            71,
            Ok(result()),
        ))
        .unwrap()
    }

    #[test]
    fn qpss_result_worker_and_saved_result_preserve_complete_state() {
        let packet = packet();
        assert!(!packet.buffers.is_empty());
        let metadata = serde_json::to_string(&packet.response).unwrap();
        let restored = WorkerResponseTransport {
            response: serde_json::from_str(&metadata).unwrap(),
            ..packet
        }
        .into_response()
        .unwrap()
        .into_result()
        .unwrap();
        let SimulationResult::Qpss {
            tuples,
            waveforms,
            operating_point,
            ..
        } = &restored
        else {
            panic!("wrong result family")
        };
        assert!(tuples.iter().any(|tuple| tuple == &[-1, 1]));
        assert_eq!(operating_point.integral_names(), ["B:BMEMORY:sdt:0"]);
        assert_eq!(
            operating_point.complete_spectra().len(),
            waveforms.len() + 1
        );
        assert!(!waveforms.iter().any(|(name, _)| name.contains("sdt:")));
        assert!(
            waveforms
                .iter()
                .any(|(name, waveform)| name.starts_with("I(") && waveform.y_unit == "A")
        );
        let point = Arc::clone(operating_point);
        let controller = crate::simulation::controller::SimulationController::new();
        let retained = controller.convert_to_analysis_result_with_metadata_owned(
            restored,
            crate::state::AnalysisType::Qpss,
            "QPSS",
        );
        retained.validate_retained_evidence().unwrap();
        let mut display_only = retained.clone();
        display_only.result_payload = None;
        let state_bytes = point.complete_spectra().iter().map(Vec::len).sum::<usize>() * 16;
        assert!(
            retained.retained_storage_bytes() - display_only.retained_storage_bytes()
                >= state_bytes as u64
        );
        let mut altered = retained.clone();
        let trace = altered
            .waveforms
            .iter_mut()
            .find(|waveform| waveform.complex.is_some())
            .unwrap();
        Arc::make_mut(&mut trace.complex.as_mut().unwrap().real)[1] += 0.1;
        assert!(altered.validate_retained_evidence().is_err());
        let persisted = crate::io::project_io::ProjectAnalysisResult::from(&retained);
        let json = serde_json::to_string(&persisted).unwrap();
        let decoded: crate::io::project_io::ProjectAnalysisResult =
            serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, persisted);
        let crate::io::project_io::PersistedField::Value(
            crate::state::AnalysisResultPayload::Qpss { operating_point },
        ) = decoded.result_payload
        else {
            panic!("missing retained point")
        };
        operating_point
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .unwrap();
        assert_eq!(operating_point, point);
    }

    #[test]
    fn qpss_result_rejects_altered_worker_spectrum_tuple_and_configuration() {
        let original = packet();
        let mut layout = serde_json::to_value(&original.response).unwrap();
        layout["outcome"]["Success"]["Qpss"]["operating_point"]["spectra"][0]["real"]["Buffer"]["len"] =
            serde_json::json!(100000000);
        let changed = WorkerResponseTransport {
            response: serde_json::from_value(layout).unwrap(),
            ..original.clone()
        };
        assert!(changed.into_response().is_err());
        let mut numeric = original.clone();
        numeric.buffers.last_mut().unwrap()[0] += 0.125;
        assert!(numeric.into_response().is_err());
        let mut metadata = serde_json::to_value(&original.response).unwrap();
        metadata["outcome"]["Success"]["Qpss"]["tuples"][1][0] = serde_json::json!(77);
        let changed = WorkerResponseTransport {
            response: serde_json::from_value(metadata).unwrap(),
            ..original.clone()
        };
        assert!(changed.into_response().is_err());
        let mut metadata = serde_json::to_value(&original.response).unwrap();
        metadata["outcome"]["Success"]["Qpss"]["operating_point"]["metadata"]["config"]["solver"]
            ["relative_tolerance"] = serde_json::json!(0.01);
        let changed = WorkerResponseTransport {
            response: serde_json::from_value(metadata).unwrap(),
            ..original
        };
        assert!(changed.into_response().is_err());
    }
}
