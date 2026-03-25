use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rspice_ui::analysis::fft::{
    FftData, FftInputOptions, FftInputPolicy, FftState, WindowFunction,
    prepare_fft_input_with_options, render_fft_plot,
};
use rspice_ui::common::AppState;
use rspice_ui::state::WaveformData;
use rspice_ui::waveform::render_waveform_viewer;

fn run_ui_frame(ctx: &egui::Context, mut f: impl FnMut(&mut egui::Ui)) {
    let _ = ctx.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_size(egui::vec2(1280.0, 720.0));
            f(ui);
        });
    });
}

fn build_uniform_transient(
    sample_count: usize,
    sample_rate: f64,
    tone_hz: f64,
) -> (Vec<f64>, Vec<f64>) {
    let time: Vec<f64> = (0..sample_count)
        .map(|idx| idx as f64 / sample_rate)
        .collect();
    let values: Vec<f64> = time
        .iter()
        .map(|t| {
            (2.0 * std::f64::consts::PI * tone_hz * t).sin()
                + 0.1 * (2.0 * std::f64::consts::PI * tone_hz * 3.0 * t).sin()
        })
        .collect();
    (time, values)
}

fn build_nonuniform_transient(
    sample_count: usize,
    sample_rate: f64,
    tone_hz: f64,
) -> (Vec<f64>, Vec<f64>) {
    let dt = 1.0 / sample_rate;
    let mut time = Vec::with_capacity(sample_count);
    let mut values = Vec::with_capacity(sample_count);
    let mut t = 0.0;

    for idx in 0..sample_count {
        let jitter = if idx % 5 == 0 { 0.92 } else { 1.08 };
        t += jitter * dt;
        time.push(t);
        values.push(
            (2.0 * std::f64::consts::PI * tone_hz * t).sin()
                + 0.05 * (2.0 * std::f64::consts::PI * tone_hz * 7.0 * t + 0.2).sin(),
        );
    }

    (time, values)
}

fn bench_waveform_render(c: &mut Criterion) {
    let sample_rate = 5_000_000.0;
    let sample_count = 1_000_000usize;
    let (time, values) = build_uniform_transient(sample_count, sample_rate, 125_000.0);

    let mut app_state = AppState::default();
    app_state.replace_waveform_results(vec![WaveformData::new("V(out)", time, values, "#4aa3ff")]);

    let ctx = egui::Context::default();
    run_ui_frame(&ctx, |ui| render_waveform_viewer(ui, &mut app_state));
    app_state
        .set_waveform_view_x_range(0.20e-3, 0.24e-3)
        .expect("valid waveform benchmark window");

    c.bench_function("waveform_render_dense_transient_redraw_1m", |b| {
        b.iter(|| {
            run_ui_frame(&ctx, |ui| render_waveform_viewer(ui, &mut app_state));
        });
    });
}

fn bench_fft_pipeline(c: &mut Criterion) {
    let sample_rate = 10_000_000.0;
    let sample_count = 1_000_000usize;
    let (uniform_time, uniform_values) =
        build_uniform_transient(sample_count, sample_rate, 250_000.0);
    let (nonuniform_time, nonuniform_values) =
        build_nonuniform_transient(sample_count, sample_rate, 250_000.0);

    let reference_options = FftInputOptions::with_policy(FftInputPolicy::reference());
    let interactive_options = FftInputOptions::with_policy(FftInputPolicy::interactive_default());

    c.bench_function("fft_prepare_reference_nonuniform_1m", |b| {
        b.iter(|| {
            black_box(
                prepare_fft_input_with_options(
                    "V(out)",
                    &nonuniform_time,
                    &nonuniform_values,
                    reference_options,
                )
                .expect("reference FFT input"),
            )
        });
    });

    c.bench_function("fft_prepare_interactive_uniform_1m", |b| {
        b.iter(|| {
            black_box(
                prepare_fft_input_with_options(
                    "V(out)",
                    &uniform_time,
                    &uniform_values,
                    interactive_options,
                )
                .expect("interactive FFT input"),
            )
        });
    });

    let prepared =
        prepare_fft_input_with_options("V(out)", &uniform_time, &uniform_values, reference_options)
            .expect("prepared FFT input");

    c.bench_function("fft_compute_reference_spectrum_1m", |b| {
        b.iter(|| {
            black_box(FftData::from_time_domain(
                "V(out)",
                &prepared.samples,
                prepared.sample_rate,
                WindowFunction::Hanning,
            ))
        });
    });
}

fn bench_fft_render(c: &mut Criterion) {
    let sample_rate = 10_000_000.0;
    let sample_count = 1_000_000usize;
    let (time, values) = build_uniform_transient(sample_count, sample_rate, 400_000.0);
    let prepared = prepare_fft_input_with_options(
        "V(out)",
        &time,
        &values,
        FftInputOptions::with_policy(FftInputPolicy::reference()),
    )
    .expect("prepared FFT input");

    let mut fft_state = FftState::new();
    fft_state.load_prepared_input(prepared);
    fft_state.freq_auto = false;
    fft_state.freq_min = 0.0;
    fft_state.freq_max = sample_rate * 0.5;
    fft_state.mag_auto = true;

    let ctx = egui::Context::default();
    run_ui_frame(&ctx, |ui| render_fft_plot(ui, &mut fft_state));

    c.bench_function("fft_render_reference_spectrum_redraw", |b| {
        b.iter(|| {
            run_ui_frame(&ctx, |ui| render_fft_plot(ui, &mut fft_state));
        });
    });
}

fn bench_config() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = waveform_fft;
    config = bench_config();
    targets = bench_waveform_render, bench_fft_pipeline, bench_fft_render
}
criterion_main!(waveform_fft);
