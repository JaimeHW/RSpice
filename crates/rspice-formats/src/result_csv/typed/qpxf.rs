//! QPXF export preserves anchored axes, unit-transfer paths, delay statuses and adjoints.
use super::{EncodedTypedCsv, TypedCsvSummary, csv_text};
use rspice_core::engine::{
    QpxfAnalysisResult, QpxfFrequencyAxis, QpxfGroupDelay, QpxfOutput, QpxfQuantity,
};

pub(super) fn prepare(response: &QpxfAnalysisResult) -> Option<EncodedTypedCsv> {
    let grid = response
        .validate_retained_payload_with_abort(
            &rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .ok()?;
    let m = &response.metadata;
    let r = &m.request;
    let tuple = |v: &[i32]| v.iter().map(i32::to_string).collect::<Vec<_>>().join(";");
    let number = |v: f64| format!("{v:.17e}");
    let output = match &r.output {
        QpxfOutput::Voltage { positive, negative } => format!("V({positive},{negative})"),
        QpxfOutput::BranchCurrent { branch } => format!("I({branch})"),
    };
    let output_unit = if r.output.quantity() == QpxfQuantity::Voltage {
        "V"
    } else {
        "A"
    };
    let axis = match r.frequency_axis {
        QpxfFrequencyAxis::Output => "output",
        QpxfFrequencyAxis::Offset => "offset",
    };
    let anchor = m.frequency_anchor();
    let mut contents = String::from(
        "kind,signal,unit,input_source,input_unit,input_tuple,output,output_unit,output_tuple,authored_axis,authored_frequency_hz,frequency_anchor,rounded_probe_offset_hz,input_frequency_hz,output_frequency_hz,coordinate_kind,coordinate_tuple,coordinate_frequency_hz,real,imaginary,group_delay_seconds,group_delay_status,group_delay_magnitude_floor,normalized_adjoint_residual,qpss_identity,result_identity\n",
    );
    let mut write = |kind: &str,
                     signal: String,
                     unit: &str,
                     source: &str,
                     input_unit: &str,
                     input_tuple: String,
                     input_frequency: String,
                     coordinate_kind: &str,
                     coordinate_tuple: String,
                     coordinate_frequency: String,
                     value: rspice_core::Complex64,
                     delay: Option<&QpxfGroupDelay>,
                     point: usize| {
        let (delay_seconds, status) = match delay {
            None => (String::new(), "not_requested"),
            Some(QpxfGroupDelay::Finite(v)) => (number(*v), "finite"),
            Some(QpxfGroupDelay::BelowMagnitudeFloor) => (String::new(), "below_magnitude_floor"),
            Some(QpxfGroupDelay::InsufficientSamples) => (String::new(), "insufficient_samples"),
            Some(QpxfGroupDelay::AmbiguousPhaseStep) => (String::new(), "ambiguous_phase_step"),
            Some(QpxfGroupDelay::NumericalLimit) => (String::new(), "numerical_limit"),
        };
        let fields = [
            kind.to_owned(),
            signal,
            unit.to_owned(),
            source.to_owned(),
            input_unit.to_owned(),
            input_tuple,
            output.clone(),
            output_unit.to_owned(),
            tuple(&r.output_lattice),
            axis.to_owned(),
            number(r.frequencies_hz[point]),
            tuple(&anchor),
            number(m.probe_offsets_hz[point]),
            input_frequency,
            number(m.output_frequencies_hz[point]),
            coordinate_kind.to_owned(),
            coordinate_tuple,
            coordinate_frequency,
            number(value.re),
            number(value.im),
            delay_seconds,
            if kind == "adjoint" {
                "not_applicable"
            } else {
                status
            }
            .to_owned(),
            number(r.group_delay_magnitude_floor),
            number(m.normalized_residuals[point]),
            m.operating_point_identity.clone(),
            m.retained_identity().to_owned(),
        ];
        contents.push_str(
            &fields
                .iter()
                .map(|s| csv_text(s))
                .collect::<Vec<_>>()
                .join(","),
        );
        contents.push('\n');
    };
    for t in &response.transfers {
        let source = &m.input_sources[t.input_source];
        let input_unit = if source.quantity == QpxfQuantity::Voltage {
            "V"
        } else {
            "A"
        };
        let unit = match (r.output.quantity(), source.quantity) {
            (QpxfQuantity::Voltage, QpxfQuantity::Voltage)
            | (QpxfQuantity::Current, QpxfQuantity::Current) => "1",
            (QpxfQuantity::Voltage, QpxfQuantity::Current) => "Ω",
            (QpxfQuantity::Current, QpxfQuantity::Voltage) => "S",
        };
        for (point, value) in t.values.iter().enumerate() {
            write(
                "transfer",
                format!(
                    "H({output}/{}({}))",
                    if source.quantity == QpxfQuantity::Voltage {
                        "V"
                    } else {
                        "I"
                    },
                    source.name
                ),
                unit,
                &source.name,
                input_unit,
                tuple(&t.input_lattice),
                number(t.input_frequencies_hz[point]),
                "",
                String::new(),
                String::new(),
                *value,
                t.group_delay.as_ref().map(|d| &d[point]),
                point,
            );
        }
    }
    // These are dual sensitivities to equation right-hand sides. They are not
    // physical node voltages or branch currents and are not conjugated here.
    for (point, solution) in response.solutions.iter().enumerate() {
        for (row, values) in solution.sensitivities.iter().enumerate() {
            let (kind, name) = if row < m.node_names.len() {
                ("node_equation", &m.node_names[row])
            } else {
                ("branch_equation", &m.branch_names[row - m.node_names.len()])
            };
            for (index, value) in values.iter().enumerate() {
                let coordinate = &grid.indices()[index];
                let frequency = grid
                    .frequency_relative_to(r.frequencies_hz[point], &anchor, coordinate)
                    .ok()?;
                write(
                    "adjoint",
                    format!("adjoint({kind}:{name})"),
                    &format!("{output_unit} / equation RHS"),
                    "",
                    "",
                    String::new(),
                    String::new(),
                    kind,
                    tuple(coordinate),
                    number(frequency),
                    *value,
                    None,
                    point,
                );
            }
        }
    }
    Some(EncodedTypedCsv {
        contents,
        summary: TypedCsvSummary::Qpxf {
            source_count: m.input_sources.len(),
            input_tuple_count: m.input_lattices.len(),
            output_frequency_count: m.output_frequencies_hz.len(),
        },
    })
}
