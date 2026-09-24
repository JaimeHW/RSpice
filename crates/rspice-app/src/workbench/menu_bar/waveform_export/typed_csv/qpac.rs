//! Lossless QPAC rows distinguish probe offsets from physical sideband frequencies.
use super::*;
use rspice_core::engine::{QpacAnalysisResult, QpacInputQuantity};

pub(super) fn prepare(response: &QpacAnalysisResult) -> Option<PreparedTypedResultCsv> {
    let grid = response
        .validate_retained_payload_with_abort(
            &rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .ok()?;
    let metadata = &response.metadata;
    let request = &metadata.request;
    let tuple = |tuple: &[i32]| {
        csv_text(
            &tuple
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join(";"),
        )
    };
    let input_tuple = tuple(&request.input_lattice);
    let input_unit = match metadata.input_quantity {
        QpacInputQuantity::Voltage => "V",
        QpacInputQuantity::Current => "A",
    };
    let source = csv_text(&request.input_source);
    let drive = metadata.drive();
    let mut contents = String::from(
        "kind,signal,response_unit,gain_unit,input_source,input_unit,input_tuple,output_tuple,probe_offset_hz,input_frequency_hz,output_frequency_hz,unit_response_real,unit_response_imaginary,response_real,response_imaginary,drive_magnitude,drive_phase_degrees,normalized_residual,qpss_identity\n",
    );
    for (offset, solution) in response.unit_solutions.iter().enumerate() {
        let mut write = |kind: &str,
                         signal: &str,
                         unit: &str,
                         index: usize,
                         value: rspice_core::Complex64|
         -> Option<()> {
            let scaled = value * drive;
            if !scaled.re.is_finite() || !scaled.im.is_finite() {
                return None;
            }
            let gain_unit = match (unit, metadata.input_quantity) {
                ("V", QpacInputQuantity::Voltage) | ("A", QpacInputQuantity::Current) => "1",
                ("V", QpacInputQuantity::Current) => "Ω",
                _ => "S",
            };
            contents.push_str(&format!("{},{},{},{},{source},{input_unit},{input_tuple},{},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{}\n",
                kind, csv_text(signal), unit, gain_unit, tuple(&metadata.tuples[index]),
                solution.offset_hz, metadata.input_frequencies_hz[offset], solution.offset_hz + grid.frequencies_hz()[index],
                value.re, value.im, scaled.re, scaled.im, request.magnitude, request.phase_degrees, solution.normalized_residual, metadata.operating_point_identity,
            ));
            Some(())
        };
        for (row, coefficients) in solution.spectra.iter().enumerate() {
            let (signal, unit) = if row < metadata.node_names.len() {
                (format!("V({})", metadata.node_names[row]), "V")
            } else {
                (
                    format!(
                        "I({})",
                        metadata.branch_names[row - metadata.node_names.len()]
                    ),
                    "A",
                )
            };
            for (index, coefficient) in coefficients.iter().enumerate() {
                write("mna", &signal, unit, index, *coefficient)?;
            }
        }
        write(
            "differential",
            &format!("V({},{})", request.output_node, request.output_ref),
            "V",
            grid.index_of(&request.output_lattice)?,
            response.output_transfer[offset],
        )?;
    }
    Some(PreparedTypedResultCsv {
        default_name: "qpac-response.csv",
        contents,
        detail: format!(
            "{} probe offsets × {} signed tuples; full complex MNA response and selected differential transfer",
            response.unit_solutions.len(),
            grid.len()
        ),
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn qpac_csv_preserves_every_signed_tuple_translated_axis_and_complex_unit() {
        let retained = crate::simulation::SimulationResult::qpac_retained_test_fixture();
        let prepared = super::super::prepare_typed_result_csv(&retained).unwrap();
        let mut reader = csv::Reader::from_reader(prepared.contents.as_bytes());
        let headers = reader.headers().unwrap().clone();
        let records = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
        let field = |name| headers.iter().position(|h| h == name).unwrap();
        // 2 nodes + 1 voltage-source branch, 9 full signed tuples, 3 offsets,
        // and one differential result per offset.
        assert_eq!(records.len(), 3 * (3 * 9 + 1));
        assert!(records.iter().any(|r| &r[field("output_tuple")] == "-1;1"));
        let row = records
            .iter()
            .find(|r| &r[field("kind")] == "differential")
            .unwrap();
        assert_eq!(&row[field("gain_unit")], "Ω");
        assert_eq!(&row[field("response_unit")], "V");
        let offset: f64 = row[field("probe_offset_hz")].parse().unwrap();
        let frequency: f64 = row[field("output_frequency_hz")].parse().unwrap();
        assert_eq!(offset, -37.0);
        assert!((frequency - (offset + 1000.0 - 1414.213562373095)).abs() < 1e-12);
        let unit = rspice_core::Complex64::new(
            row[field("unit_response_real")].parse().unwrap(),
            row[field("unit_response_imaginary")].parse().unwrap(),
        );
        let response = rspice_core::Complex64::new(
            row[field("response_real")].parse().unwrap(),
            row[field("response_imaginary")].parse().unwrap(),
        );
        assert!(
            (response - unit * rspice_core::Complex64::from_polar(0.002, 73.0_f64.to_radians()))
                .norm()
                < 1e-14
        );
    }
}
