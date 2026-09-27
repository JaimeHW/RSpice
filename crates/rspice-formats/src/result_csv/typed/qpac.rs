//! Lossless QPAC rows distinguish probe offsets from physical sideband frequencies.
use super::{EncodedTypedCsv, TypedCsvSummary, csv_text};
use rspice_core::engine::{QpacAnalysisResult, QpacInputQuantity};

pub(super) fn prepare(response: &QpacAnalysisResult) -> Option<EncodedTypedCsv> {
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
    Some(EncodedTypedCsv {
        contents,
        summary: TypedCsvSummary::Qpac {
            probe_offset_count: response.unit_solutions.len(),
            tuple_count: grid.len(),
        },
    })
}
