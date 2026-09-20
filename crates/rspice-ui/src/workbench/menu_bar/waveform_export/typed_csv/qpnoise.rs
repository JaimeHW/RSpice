//! Exact noise measurement tables with full covariance, source laws and adjoints.
use super::*;
use rspice_core::analysis::quasi_periodic::QuasiPeriodicNoiseSpectrum;
use rspice_core::engine::{
    QpnoiseAnalysisResult, QpnoiseObservation, QpnoiseUnavailable, QpnoiseValue, QpxfQuantity,
};
const HEADERS: &str = "kind,output_index,output,output_tuple,output_frequency_hz,other_output_index,other_output,other_output_tuple,other_output_frequency_hz,source,quantity,unit,authored_frequency_hz,coordinate,coordinate_tuple,real,imaginary,roundoff_bound,status,rank,percentage,qpss_identity,result_identity,details\n";
fn number(v: f64) -> String {
    format!("{v:.17e}")
}
fn tuple(v: &[i32]) -> String {
    v.iter().map(i32::to_string).collect::<Vec<_>>().join(";")
}
fn observation(o: &QpnoiseObservation) -> String {
    match o {
        QpnoiseObservation::Voltage { positive, negative } => format!("V({positive},{negative})"),
        QpnoiseObservation::BranchCurrent { branch } => format!("I({branch})"),
    }
}
fn unit(o: &QpnoiseObservation) -> &'static str {
    if matches!(o, QpnoiseObservation::Voltage { .. }) {
        "V"
    } else {
        "A"
    }
}
fn status(v: &QpnoiseValue) -> (&'static str, String) {
    match v {
        QpnoiseValue::Finite(v) => ("finite", number(*v)),
        QpnoiseValue::Unavailable(why) => (
            match why {
                QpnoiseUnavailable::ZeroInputTransfer => "zero_input_transfer",
                QpnoiseUnavailable::OutsideNumericRange => "outside_numeric_range",
                QpnoiseUnavailable::NoFrequencyInterval => "no_frequency_interval",
                QpnoiseUnavailable::NegativeIntegrationFrequency => {
                    "negative_integration_frequency"
                }
                QpnoiseUnavailable::IncompleteIntegrationBand => "incomplete_integration_band",
                QpnoiseUnavailable::UndefinedIntegrationSample => "undefined_integration_sample",
            },
            String::new(),
        ),
    }
}
struct Table<'a> {
    result: &'a QpnoiseAnalysisResult,
    contents: String,
}
impl Table<'_> {
    fn row(
        &mut self,
        kind: &str,
        output: Option<usize>,
        point: Option<usize>,
        fields: &[(usize, String)],
    ) {
        let mut cells: Vec<String> = vec![String::new(); 24];
        cells[0] = kind.into();
        if let Some(index) = output {
            let o = &self.result.metadata.request.outputs[index];
            cells[1] = (index + 1).to_string();
            cells[2] = observation(&o.observation);
            cells[3] = tuple(&o.lattice);
            if let Some(p) = point {
                cells[4] = number(self.result.outputs[index].frequencies_hz[p]);
            }
        }
        if let Some(p) = point {
            cells[12] = number(self.result.metadata.request.frequencies_hz[p]);
        }
        cells[21] = self.result.metadata.operating_point_identity.clone();
        cells[22] = self.result.metadata.retained_identity.clone();
        for (index, value) in fields {
            cells[*index] = value.clone();
        }
        self.contents.push_str(
            &cells
                .iter()
                .map(|v| csv_text(v))
                .collect::<Vec<_>>()
                .join(","),
        );
        self.contents.push('\n');
    }
    fn value(
        &mut self,
        kind: &str,
        output: usize,
        point: Option<usize>,
        source: &str,
        quantity: &str,
        unit: &str,
        value: QpnoiseValue,
    ) {
        let (status, real) = status(&value);
        self.row(
            kind,
            Some(output),
            point,
            &[
                (9, source.into()),
                (10, quantity.into()),
                (11, unit.into()),
                (15, real),
                (18, status.into()),
            ],
        );
    }
}
pub(super) fn prepare(response: &QpnoiseAnalysisResult) -> Option<PreparedTypedResultCsv> {
    let grid = response
        .validate_retained_payload_with_abort(
            &rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .ok()?;
    let mut table = Table {
        result: response,
        contents: HEADERS.into(),
    };
    let request = &response.metadata.request;
    // Includes authored axis/anchor, solver, selected source/window/reference,
    // resolved MNA bindings, temperatures and exact producer/result identities.
    table.row(
        "metadata",
        None,
        None,
        &[(23, serde_json::to_string(&response.metadata).ok()?)],
    );
    for (index, source) in response.sources.iter().enumerate() {
        let label = format!("{}: {}", index + 1, source.name);
        for (coordinate, value) in &source.injections {
            table.row(
                "source_injection",
                None,
                None,
                &[
                    (9, label.clone()),
                    (13, coordinate.to_string()),
                    (15, number(value.re)),
                    (16, number(value.im)),
                ],
            );
        }
        match &source.spectrum {
            QuasiPeriodicNoiseSpectrum::White {
                density,
                binary_scale_exponent,
            } => {
                table.row("source_law",None,None,&[(9,label.clone()),(23,format!("white; density * 2^{binary_scale_exponent}; phase sample order from grid"))]);
                for (sample, value) in density.iter().enumerate() {
                    table.row(
                        "source_density",
                        None,
                        None,
                        &[
                            (9, label.clone()),
                            (13, sample.to_string()),
                            (15, number(*value)),
                        ],
                    );
                }
            }
            QuasiPeriodicNoiseSpectrum::PowerLaw {
                coefficient,
                exponent,
                modulation,
                modulation_lattices,
                binary_scale_exponent,
            } => {
                table.row("source_law",None,None,&[(9,label.clone()),(23,format!("power_law; coefficient={} * 2^{binary_scale_exponent}; exponent={}",number(*coefficient),number(*exponent)))]);
                for (index, value) in modulation.iter().enumerate() {
                    let coordinate = modulation_lattices.as_deref().unwrap_or(grid.indices());
                    table.row(
                        "source_modulation",
                        None,
                        None,
                        &[
                            (9, label.clone()),
                            (14, tuple(&coordinate[index])),
                            (15, number(value.re)),
                            (16, number(value.im)),
                        ],
                    );
                }
            }
        }
    }
    let n = response.outputs.len();
    for (row, spectrum) in response.outputs.iter().enumerate() {
        let output_unit = unit(&request.outputs[row].observation);
        let input_unit = response.metadata.input_source.as_ref().map(|i| {
            if i.quantity == QpxfQuantity::Voltage {
                "V"
            } else {
                "A"
            }
        });
        for p in 0..response.points.len() {
            let diagonal = response.total_covariances[p].values[row * n + row].re;
            table.value(
                "measurement",
                row,
                Some(p),
                "",
                "output_psd",
                &format!("{output_unit}²/Hz"),
                QpnoiseValue::Finite(diagonal),
            );
            table.value(
                "measurement",
                row,
                Some(p),
                "",
                "output_asd",
                &format!("{output_unit}/√Hz"),
                QpnoiseValue::Finite(diagonal.sqrt()),
            );
            if let (Some(values), Some(input_unit)) = (&spectrum.input_noise, input_unit) {
                table.value(
                    "measurement",
                    row,
                    Some(p),
                    "",
                    "input_psd",
                    &format!("{input_unit}²/Hz"),
                    values[p],
                );
                let value = match values[p] {
                    QpnoiseValue::Finite(v) => QpnoiseValue::Finite(v.sqrt()),
                    v => v,
                };
                table.value(
                    "measurement",
                    row,
                    Some(p),
                    "",
                    "input_asd",
                    &format!("{input_unit}/√Hz"),
                    value,
                );
            }
            if let Some(values) = &spectrum.noise_figure_db {
                table.value(
                    "measurement",
                    row,
                    Some(p),
                    "",
                    "noise_figure",
                    "dB",
                    values[p],
                );
            }
            if let Some(values) = &spectrum.input_transfer {
                table.row(
                    "input_transfer",
                    Some(row),
                    Some(p),
                    &[
                        (
                            11,
                            format!("{output_unit}/{}", input_unit.unwrap_or("input")),
                        ),
                        (15, number(values[p].re)),
                        (16, number(values[p].im)),
                    ],
                );
            }
            for column in 0..n {
                for source in 0..=response.sources.len() {
                    let (covariance, label) = if source == response.sources.len() {
                        (&response.total_covariances[p], "total".to_owned())
                    } else {
                        (
                            &response.points[p].source_covariances[source],
                            format!("{}: {}", source + 1, response.sources[source].name),
                        )
                    };
                    let other = &request.outputs[column];
                    let value = covariance.values[row * n + column];
                    table.row(
                        "covariance",
                        Some(row),
                        Some(p),
                        &[
                            (5, (column + 1).to_string()),
                            (6, observation(&other.observation)),
                            (7, tuple(&other.lattice)),
                            (8, number(response.outputs[column].frequencies_hz[p])),
                            (9, label),
                            (11, format!("{output_unit}·{}/Hz", unit(&other.observation))),
                            (15, number(value.re)),
                            (16, number(value.im)),
                            (17, number(covariance.roundoff_bounds[row * n + column])),
                            (23, "C[row,column] = E[y_row conj(y_column)]".into()),
                        ],
                    );
                }
            }
            let adjoint = &response.points[p].adjoints[row];
            for (coordinate, values) in adjoint.sensitivities.iter().enumerate() {
                let m = &response.metadata;
                let name = if coordinate < m.node_names.len() {
                    format!("node_equation:{}", m.node_names[coordinate])
                } else {
                    format!(
                        "branch_equation:{}",
                        m.branch_names[coordinate - m.node_names.len()]
                    )
                };
                for (index, value) in values.iter().enumerate() {
                    table.row(
                        "adjoint",
                        Some(row),
                        Some(p),
                        &[
                            (11, format!("{output_unit} / equation RHS")),
                            (13, name.clone()),
                            (14, tuple(&grid.indices()[index])),
                            (15, number(value.re)),
                            (16, number(value.im)),
                            (
                                23,
                                format!(
                                    "normalized residual={}",
                                    number(adjoint.normalized_residual)
                                ),
                            ),
                        ],
                    );
                }
            }
        }
        if let Some(integral) = &spectrum.integrated {
            table.value(
                "integrated",
                row,
                None,
                "",
                "output_rms",
                output_unit,
                integral.output_rms,
            );
            if let Some(value) = integral.input_rms {
                table.value(
                    "integrated",
                    row,
                    None,
                    "",
                    "input_rms",
                    input_unit.unwrap_or("input"),
                    value,
                );
            }
            for (index, value) in integral.contributor_rms.iter().enumerate() {
                table.value(
                    "integrated",
                    row,
                    None,
                    &response.sources[index].name,
                    "contributor_rms",
                    output_unit,
                    *value,
                );
            }
        }
        if let Some(ranking) = &spectrum.ranking {
            for (rank, entry) in ranking.iter().enumerate() {
                table.row(
                    "ranking",
                    Some(row),
                    None,
                    &[
                        (9, response.sources[entry.source_index].name.clone()),
                        (19, (rank + 1).to_string()),
                        (20, number(entry.percentage)),
                    ],
                );
            }
        }
    }
    Some(PreparedTypedResultCsv {
        default_name: "qpnoise-results.csv",
        contents: table.contents,
        detail: format!(
            "{} outputs; {} frequencies; {} noise mechanisms; PSD, referral, integration, ranking, covariance and complete adjoints",
            n,
            response.points.len(),
            response.sources.len()
        ),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn qpnoise_result_csv_retains_measurements_statuses_covariance_and_primary_evidence() {
        let result = crate::simulation::SimulationResult::qpnoise_retained_test_fixture();
        let export = super::super::prepare_typed_result_csv(&result).unwrap();
        let mut reader = csv::Reader::from_reader(export.contents.as_bytes());
        let headers = reader.headers().unwrap().clone();
        let rows = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
        let index = |name| headers.iter().position(|h| h == name).unwrap();
        for kind in [
            "metadata",
            "source_law",
            "source_injection",
            "source_density",
            "measurement",
            "input_transfer",
            "covariance",
            "adjoint",
            "integrated",
            "ranking",
        ] {
            assert!(rows.iter().any(|r| &r[0] == kind), "missing {kind}");
        }
        assert!(rows.iter().any(|r| &r[index("quantity")] == "input_psd"
            && &r[index("status")] == "zero_input_transfer"));
        assert!(rows.iter().any(|r| &r[index("quantity")] == "output_rms"
            && &r[index("status")] == "negative_integration_frequency"));
        assert!(
            rows.iter()
                .any(|r| &r[index("unit")] == "V·A/Hz" && &r[0] == "covariance")
        );
        assert!(
            rows.iter()
                .any(|r| &r[0] == "adjoint"
                    && r[index("coordinate")].starts_with("branch_equation:"))
        );
        assert!(
            rows.iter().all(|r| !r[index("qpss_identity")].is_empty()
                && !r[index("result_identity")].is_empty())
        );
    }
}
