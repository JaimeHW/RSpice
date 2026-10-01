//! Noise quantity and carrier evidence from exact execution options.

use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::family_metadata::AnalysisResultFamilyMetadata;
use rspice_results::family_metadata::PeriodicNoiseOutputQuantity;

pub fn retain_periodic_noise_metadata<W: AsMut<rspice_results::waveform::RetainedWaveform>>(
    result: &mut AnalysisResult<W>,
    config: Option<&crate::periodic::PnoiseRunConfig>,
    carrier_frequency_hz: Option<f64>,
) {
    if !matches!(
        result.analysis_type,
        AnalysisType::Pnoise | AnalysisType::Qpnoise
    ) {
        return;
    }
    let Some(config) = config else {
        return;
    };
    let output_quantity = match config.noise_ref {
        crate::periodic::PnoiseReference::Phase => PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
        crate::periodic::PnoiseReference::Output | crate::periodic::PnoiseReference::Input => {
            if config
                .sampling
                .as_ref()
                .is_some_and(|sampling| sampling.is_timing())
            {
                PeriodicNoiseOutputQuantity::TimingNoisePowerSpectralDensity
            } else {
                PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity
            }
        }
    };
    if output_quantity == PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz
        && let Some(waveform) = result
            .waveforms
            .iter_mut()
            .map(AsMut::as_mut)
            .find(|waveform| waveform.name.eq_ignore_ascii_case("onoise"))
    {
        waveform.name = "phase_noise".to_owned();
    }
    // The carrier the run was actually solved against, captured from the
    // resolved periodic dependency at dispatch -- not
    // `config.pss_fundamental_freq`, which is the number this
    // configuration authored. For a driven carrier the drive sets the
    // period and the two are the same bits. For an autonomous one the
    // shooting solver holds the period as an unknown and moves it, so the
    // authored value is a guess the circuit never ran at: on the
    // workspace's own oscillator fixture the guess is 1.5873e5 Hz and the
    // carrier converges at 1.5912e5 Hz. Publishing the guess mislabels
    // every dBc/Hz spectrum that is stated relative to it, and the result
    // digest hashes this field, so it also mis-identifies the run.
    result.family_metadata = Some(AnalysisResultFamilyMetadata::PeriodicNoise {
        output_quantity,
        carrier_frequency_hz,
    });
}
