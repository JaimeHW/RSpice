//! Export periodic spectra without discarding their reference channels.
use super::*;

#[test]
fn periodic_port_noise_export_preserves_spectra_and_reference_context() {
    let mut analysis = AnalysisResult::new(1, AnalysisType::Psp, "periodic noise")
        .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm: vec![50.0, 50.0],
            noise_reference_temperature_kelvin: None,
        })
        .with_measurements(vec![
            rspice_core::MeasureResult::success(
                "periodic_noise_reference_temperature_kelvin",
                325.0,
            ),
            rspice_core::MeasureResult::success("periodic_noise_input_sideband", -1.0),
        ]);
    for name in ["S11", "S12", "S21", "S22"] {
        analysis.waveforms.push(
            WaveformData::new(format!("|{name}|"), vec![1e4], vec![0.5], "#fff")
                .with_complex_components(name, vec![0.5], vec![0.0]),
        );
    }
    analysis.waveforms.push(
        WaveformData::new("|Cw1_2[k=+1,m=-1]|", vec![1e4], vec![5e-20], "#fff")
            .with_complex_components("Cw1_2[k=+1,m=-1]", vec![-3e-20], vec![4e-20])
            .with_unit("W/Hz"),
    );
    analysis
        .waveforms
        .push(WaveformData::new("PN_F", vec![1e4], vec![2.0], "#fff").with_unit("1"));
    let waves = analysis.waveforms.iter().collect::<Vec<_>>();
    let dataset = prepare_single_analysis_dataset(&analysis, &waves, false)
        .unwrap()
        .dataset;
    let real = dataset
        .signals
        .iter()
        .find(|s| s.name.starts_with("re(Cw"))
        .unwrap();
    assert_eq!(real.unit, "W/Hz");
    assert_eq!(real.data, [-3e-20]);
    let csv = crate::io::WaveformWriter::new(crate::io::WaveformFormat::Csv)
        .write_text(&dataset)
        .unwrap();
    assert!(csv.contains("re(Cw1_2[k=+1,m=-1])"), "{csv}");
    assert!(csv.contains("periodic_noise_reference_temperature_kelvin"));
    assert!(csv.contains("325"));
    let dataset = prepare_single_analysis_dataset(&analysis, &waves, true)
        .unwrap()
        .dataset;
    let error = crate::io::WaveformWriter::new(crate::io::WaveformFormat::Touchstone)
        .write_text(&dataset)
        .unwrap_err();
    assert!(error.contains("periodic cross-frequency noise"), "{error}");
}
