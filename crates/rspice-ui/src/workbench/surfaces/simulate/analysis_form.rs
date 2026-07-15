//! Per-analysis configuration forms for the Simulate right panel.
//!
//! Each form edits one typed [`AnalysisDraft`] owned by a stable analysis
//! instance. The form returns a one-line note describing what the analysis
//! does; validation is rendered by the caller.

use egui::Ui;

use crate::simulation::plan::AnalysisDraft;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, check_row, choice_row, input_row};
use crate::workbench::design_system::property_row;

const SWEEP_KINDS: &[&str] = &["dec", "oct", "lin"];

/// Mono sub-header inside a form ("TONE 2", "PORT 1").
fn sub_header(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &text.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
            color: t.color.text_faint,
            extra_letter_spacing: 0.08 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.label(job);
}

/// A full-width ghost add/remove action line. Returns `true` on click.
fn action_line(ui: &mut Ui, label: &str) -> bool {
    Button::new(label)
        .ghost()
        .min_width(ui.available_width())
        .show(ui)
        .clicked()
}

/// Render the form for `draft`; returns the explanatory note.
pub(super) fn form(ui: &mut Ui, draft: &mut AnalysisDraft) -> &'static str {
    match draft {
        AnalysisDraft::OperatingPoint(setup) => {
            setup.ensure_initialized();
            property_row(ui, "Temperature", "PVT run set");
            input_row(ui, "GMIN steps", &mut setup.gmin_steps);
            check_row(ui, "Source stepping", &mut setup.source_stepping);
            check_row(ui, "Save all signals", &mut setup.save_all);
            check_row(ui, "Save OP details", &mut setup.save_op_info);
            "Solves the DC operating point; device bias lands in the OP inspector."
        }
        AnalysisDraft::Transient(setup) => {
            input_row(ui, "Stop time", &mut setup.stop);
            input_row(ui, "Step time", &mut setup.step);
            input_row(ui, "Start time", &mut setup.start);
            input_row(ui, "Max step", &mut setup.max_step);
            check_row(ui, "Use initial conditions", &mut setup.uic);
            "Local truncation error controls step size between limits."
        }
        AnalysisDraft::Ac(setup) => {
            input_row(ui, "Start", &mut setup.fstart);
            input_row(ui, "Stop", &mut setup.fstop);
            input_row(ui, "Points", &mut setup.points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep);
            "Small-signal sweep around the operating point."
        }
        AnalysisDraft::DcSweep(setup) => {
            input_row(ui, "Source", &mut setup.source);
            input_row(ui, "Start", &mut setup.start);
            input_row(ui, "Stop", &mut setup.stop);
            input_row(ui, "Step", &mut setup.step);
            check_row(ui, "Nested sweep", &mut setup.nested);
            if setup.nested {
                input_row(ui, "Source 2", &mut setup.source2);
                input_row(ui, "Start 2", &mut setup.start2);
                input_row(ui, "Stop 2", &mut setup.stop2);
                input_row(ui, "Step 2", &mut setup.step2);
            }
            "Sweeps a source over the operating range."
        }
        AnalysisDraft::Noise(setup) => {
            input_row(ui, "Output", &mut setup.output);
            input_row(ui, "Reference", &mut setup.reference);
            input_row(ui, "Input src", &mut setup.input);
            input_row(ui, "Start", &mut setup.fstart);
            input_row(ui, "Stop", &mut setup.fstop);
            input_row(ui, "Points", &mut setup.points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep);
            "Integrated and spot noise over its independent small-signal sweep."
        }
        AnalysisDraft::PoleZero(setup) => {
            input_row(ui, "Input +", &mut setup.input_pos);
            input_row(ui, "Input −", &mut setup.input_neg);
            input_row(ui, "Output +", &mut setup.output_pos);
            input_row(ui, "Output −", &mut setup.output_neg);
            choice_row(ui, "Transfer", &["V", "I"], &mut setup.transfer_idx);
            choice_row(
                ui,
                "Roots",
                &["both", "poles", "zeros"],
                &mut setup.analysis_idx,
            );
            "Extracts poles and zeros of the small-signal transfer."
        }
        AnalysisDraft::Sensitivity(setup) => {
            input_row(ui, "Output", &mut setup.output_expr);
            choice_row(ui, "Mode", &["DC", "AC"], &mut setup.sens_type_idx);
            if setup.sens_type_idx == 1 {
                input_row(ui, "Frequency", &mut setup.ac_freq);
            }
            check_row(ui, "Include parameters", &mut setup.include_params);
            check_row(ui, "Include devices", &mut setup.include_devices);
            "Sensitivity of the output to every parameter."
        }
        AnalysisDraft::MonteCarlo(setup) => {
            input_row(ui, "Samples", &mut setup.num_runs);
            input_row(ui, "Seed", &mut setup.seed);
            input_row(ui, "Spread %", &mut setup.variation_pct);
            choice_row(
                ui,
                "Vary",
                &["gauss", "uniform", "worst"],
                &mut setup.distribution_idx,
            );
            choice_row(ui, "Base", &["tran", "ac", "dc", "op"], &mut setup.base_idx);
            check_row(ui, "Process variations", &mut setup.process_variations);
            check_row(ui, "Mismatch variations", &mut setup.mismatch_variations);
            check_row(ui, "Save every run", &mut setup.save_all_runs);
            "Statistical sampling around the nominal design."
        }
        AnalysisDraft::Pss(setup) => {
            input_row(ui, "Fundamental", &mut setup.fund_freq);
            input_row(ui, "Harmonics", &mut setup.num_harmonics);
            input_row(ui, "Max iters", &mut setup.max_iter);
            choice_row(ui, "Method", &["shooting", "HB"], &mut setup.method_idx);
            check_row(ui, "Oscillator mode", &mut setup.osc_mode);
            if setup.osc_mode {
                input_row(ui, "Osc node", &mut setup.osc_node);
            }
            check_row(ui, "Save harmonics", &mut setup.save_harmonics);
            "Periodic steady state of the large-signal circuit."
        }
        AnalysisDraft::Stb(setup) => {
            input_row(ui, "Probe", &mut setup.probe_source);
            input_row(ui, "Start", &mut setup.start_freq);
            input_row(ui, "Stop", &mut setup.stop_freq);
            input_row(ui, "Points/dec", &mut setup.points_per_decade);
            check_row(ui, "Gain margin", &mut setup.gain_margin);
            check_row(ui, "Phase margin", &mut setup.phase_margin);
            check_row(ui, "Crossover freq", &mut setup.crossover_freq);
            "Loop gain and margins via the probe source."
        }
        AnalysisDraft::Temperature(setup) => {
            input_row(ui, "Start", &mut setup.temp_start);
            input_row(ui, "Stop", &mut setup.temp_stop);
            input_row(ui, "Step", &mut setup.temp_step);
            choice_row(ui, "Base", &["op", "tran", "ac", "dc"], &mut setup.base_idx);
            check_row(ui, "Corner temps only", &mut setup.corner_temps);
            "Repeats the base analysis across temperature."
        }
        AnalysisDraft::HarmonicBalance(setup) => {
            input_row(ui, "Fundamental", &mut setup.fundamental);
            input_row(ui, "Harmonics", &mut setup.harmonics);
            input_row(ui, "Source", &mut setup.fundamental_source);
            input_row(ui, "Oversample", &mut setup.oversample);
            input_row(ui, "Max iters", &mut setup.maxiter);
            choice_row(ui, "Solver", &["newton", "krylov"], &mut setup.solver_idx);
            check_row(ui, "Source stepping", &mut setup.source_stepping);
            let mut remove: Option<usize> = None;
            for (idx, tone) in setup.additional_tones.iter_mut().enumerate() {
                sub_header(ui, &format!("Tone {}", idx + 2));
                input_row(ui, "Frequency", &mut tone.frequency);
                input_row(ui, "Harmonics", &mut tone.harmonics);
                input_row(ui, "Source", &mut tone.source);
                if action_line(ui, "Remove tone") {
                    remove = Some(idx);
                }
            }
            if let Some(idx) = remove {
                setup.additional_tones.remove(idx);
            }
            ui.add_space(4.0);
            if action_line(ui, "+ Add tone") {
                setup.additional_tones.push(Default::default());
            }
            "Multi-tone steady state in the frequency domain."
        }
        AnalysisDraft::SParameter(setup) => {
            input_row(ui, "Start", &mut setup.start_freq);
            input_row(ui, "Stop", &mut setup.stop_freq);
            input_row(ui, "Points", &mut setup.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Z0", &mut setup.z0);
            check_row(ui, "Noise parameters", &mut setup.do_noise);
            check_row(ui, "Touchstone export", &mut setup.touchstone_export);
            let mut remove: Option<usize> = None;
            let port_count = setup.ports.len();
            for (idx, port) in setup.ports.iter_mut().enumerate() {
                sub_header(ui, &format!("Port {}", idx + 1));
                input_row(ui, "Node +", &mut port.node_pos);
                check_row(ui, "Differential", &mut port.differential);
                if port.differential {
                    input_row(ui, "Node −", &mut port.node_neg);
                }
                check_row(ui, "Z0 override", &mut port.z0_override);
                if port.z0_override {
                    input_row(ui, "Port Z0", &mut port.z0);
                }
                if port_count > 1 && action_line(ui, "Remove port") {
                    remove = Some(idx);
                }
            }
            if let Some(idx) = remove {
                setup.ports.remove(idx);
            }
            ui.add_space(4.0);
            if action_line(ui, "+ Add port") {
                setup.ports.push(Default::default());
            }
            "Scattering parameters between the defined ports."
        }
        AnalysisDraft::Pac(setup) => {
            input_row(ui, "Start", &mut setup.start_freq);
            input_row(ui, "Stop", &mut setup.stop_freq);
            input_row(ui, "Points", &mut setup.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Input src", &mut setup.input_source);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Magnitude", &mut setup.pac_magnitude);
            input_row(ui, "Max sideband", &mut setup.max_sideband);
            check_row(ui, "Include DC", &mut setup.include_dc);
            "Small-signal AC around the periodic steady state (needs PSS)."
        }
        AnalysisDraft::Pnoise(setup) => {
            input_row(ui, "Start", &mut setup.start_freq);
            input_row(ui, "Stop", &mut setup.stop_freq);
            input_row(ui, "Points", &mut setup.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Input src", &mut setup.input_source);
            input_row(ui, "Max sideband", &mut setup.max_sideband);
            choice_row(
                ui,
                "Refer to",
                &["output", "input", "phase"],
                &mut setup.noise_ref_idx,
            );
            check_row(ui, "Integrated noise", &mut setup.integrated_noise);
            check_row(ui, "Noise summary", &mut setup.noise_summary);
            "Cyclostationary noise around the periodic steady state (needs PSS)."
        }
        AnalysisDraft::Pxf(setup) => {
            input_row(ui, "Start", &mut setup.start_freq);
            input_row(ui, "Stop", &mut setup.stop_freq);
            input_row(ui, "Points", &mut setup.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Out sideband", &mut setup.output_sideband);
            input_row(ui, "Input src", &mut setup.input_source);
            input_row(ui, "Max sideband", &mut setup.max_sideband);
            "Transfer functions onto a periodic steady state (needs PSS)."
        }
        AnalysisDraft::Pstb(setup) => {
            input_row(ui, "Probe", &mut setup.probe);
            input_row(ui, "Harmonics", &mut setup.max_harmonics);
            input_row(ui, "Multipliers", &mut setup.num_multipliers);
            check_row(ui, "Annotate", &mut setup.annotate);
            check_row(ui, "Phase margin", &mut setup.phase_margin);
            check_row(ui, "Gain margin", &mut setup.gain_margin);
            "Loop stability around the periodic steady state (needs PSS)."
        }
        AnalysisDraft::TransferFunction(setup) => {
            input_row(ui, "Start", &mut setup.start_freq);
            input_row(ui, "Stop", &mut setup.stop_freq);
            input_row(ui, "Points", &mut setup.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Input src", &mut setup.input_source);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            check_row(ui, "Group delay", &mut setup.group_delay);
            check_row(ui, "Input impedance", &mut setup.input_impedance);
            check_row(ui, "Output impedance", &mut setup.output_impedance);
            "Small-signal transfer function and impedances."
        }
        AnalysisDraft::Corner(setup) => {
            sub_header(ui, "Process");
            check_row(ui, "TT — typical", &mut setup.process_tt);
            check_row(ui, "SS — slow/slow", &mut setup.process_ss);
            check_row(ui, "FF — fast/fast", &mut setup.process_ff);
            check_row(ui, "SF — slow/fast", &mut setup.process_sf);
            check_row(ui, "FS — fast/slow", &mut setup.process_fs);
            sub_header(ui, "Supply");
            check_row(ui, "Sweep voltage", &mut setup.enable_voltage_sweep);
            if setup.enable_voltage_sweep {
                input_row(ui, "Min", &mut setup.voltage_min);
                input_row(ui, "Nominal", &mut setup.voltage_nom);
                input_row(ui, "Max", &mut setup.voltage_max);
            }
            sub_header(ui, "Temperature");
            check_row(ui, "Sweep temperature", &mut setup.enable_temp_sweep);
            if setup.enable_temp_sweep {
                input_row(ui, "Cold", &mut setup.temp_cold);
                input_row(ui, "Room", &mut setup.temp_room);
                input_row(ui, "Hot", &mut setup.temp_hot);
            }
            ui.add_space(4.0);
            check_row(ui, "Full matrix", &mut setup.full_matrix);
            choice_row(
                ui,
                "Base",
                &["tran", "ac", "dc", "op"],
                &mut setup.base_analysis_idx,
            );
            "Repeats the base analysis across the selected corners."
        }
        AnalysisDraft::Envelope(setup) => {
            input_row(ui, "Fundamental", &mut setup.fundamental);
            input_row(ui, "Stop time", &mut setup.stop_time);
            input_row(ui, "Harmonics", &mut setup.harmonics);
            choice_row(
                ui,
                "Modulation",
                &["AM", "FM", "PM", "IQ"],
                &mut setup.modulation_idx,
            );
            "Envelope-following transient for modulated carriers."
        }
        AnalysisDraft::Fourier(setup) => {
            input_row(ui, "Fundamental", &mut setup.fundamental);
            input_row(ui, "Harmonics", &mut setup.harmonics);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "From", &mut setup.start_time);
            input_row(ui, "To", &mut setup.stop_time);
            check_row(ui, "Compute THD", &mut setup.compute_thd);
            check_row(ui, "Normalize", &mut setup.normalize);
            "Fourier components of a transient waveform window."
        }
        AnalysisDraft::Reliability(setup) => {
            input_row(ui, "Years", &mut setup.years_csv);
            input_row(ui, "Min stress V", &mut setup.min_stress_voltage);
            check_row(ui, "Hot carrier (HCI)", &mut setup.enable_hci);
            check_row(ui, "Bias instability (NBTI)", &mut setup.enable_nbti);
            check_row(ui, "Electromigration", &mut setup.enable_em);
            "Projects device aging across the lifetime points."
        }
        AnalysisDraft::Optimization(setup) => {
            input_row(ui, "Variables", &mut setup.variables_text);
            input_row(ui, "Objective", &mut setup.objective_node);
            input_row(ui, "Obj ref", &mut setup.objective_ref);
            choice_row(ui, "Goal", &["min", "max", "target"], &mut setup.goal_mode);
            if setup.goal_mode == 2 {
                input_row(ui, "Target", &mut setup.target_value);
            }
            choice_row(
                ui,
                "Method",
                &["gradient", "pattern", "anneal"],
                &mut setup.algorithm,
            );
            input_row(ui, "Max iters", &mut setup.max_iterations);
            input_row(ui, "Tolerance", &mut setup.cost_tolerance);
            "Tunes the variables (name:min:max[:initial]) toward the goal."
        }
        AnalysisDraft::Soa(setup) => {
            input_row(ui, "Stop time", &mut setup.stop_time);
            input_row(ui, "Step time", &mut setup.step_time);
            check_row(ui, "Check Vgs", &mut setup.check_vgs_max);
            if setup.check_vgs_max {
                input_row(ui, "Max Vgs", &mut setup.max_vgs);
            }
            check_row(ui, "Check Vds", &mut setup.check_vds_max);
            if setup.check_vds_max {
                input_row(ui, "Max Vds", &mut setup.max_vds);
            }
            check_row(ui, "Check Vbe", &mut setup.check_vbe_max);
            if setup.check_vbe_max {
                input_row(ui, "Max Vbe", &mut setup.max_vbe);
            }
            check_row(ui, "Check Vce", &mut setup.check_vce_max);
            if setup.check_vce_max {
                input_row(ui, "Max Vce", &mut setup.max_vce);
            }
            "Flags excursions outside the safe operating area during transient."
        }
        AnalysisDraft::Disto(setup) => {
            input_row(ui, "Start", &mut setup.sweep.fstart);
            input_row(ui, "Stop", &mut setup.sweep.fstop);
            input_row(ui, "Points", &mut setup.sweep.points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep.sweep);
            input_row(ui, "f2/f1", &mut setup.f2_over_f1);
            "Harmonic and intermodulation distortion; empty ratio means single-tone."
        }
    }
}
