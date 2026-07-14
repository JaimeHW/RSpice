//! Per-analysis configuration forms for the Simulate right panel.
//!
//! One form per analysis index, editing the typed `SimSetupState` drafts
//! the controller runs from. Each form returns a one-line note describing
//! what the analysis does; validation is rendered by the caller.

use egui::Ui;

use crate::common::app::SimSetupState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, check_row, choice_row, input_row, kv_row};

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

/// Render the form for `index`; returns the explanatory note.
pub(super) fn form(ui: &mut Ui, setup: &mut SimSetupState, index: usize) -> &'static str {
    match index {
        0 => {
            setup.op.ensure_initialized();
            input_row(ui, "Temperature", &mut setup.op.temperature);
            let _ = setup.apply_reference_temperature_from_op();
            input_row(ui, "GMIN steps", &mut setup.op.gmin_steps);
            check_row(ui, "Source stepping", &mut setup.op.source_stepping);
            check_row(ui, "Save all signals", &mut setup.op.save_all);
            check_row(ui, "Save OP details", &mut setup.op.save_op_info);
            "Solves the DC operating point; device bias lands in the OP inspector."
        }
        1 => {
            input_row(ui, "Stop time", &mut setup.tran.stop);
            input_row(ui, "Step time", &mut setup.tran.step);
            input_row(ui, "Start time", &mut setup.tran.start);
            input_row(ui, "Max step", &mut setup.tran.max_step);
            check_row(ui, "Use initial conditions", &mut setup.tran.uic);
            "Local truncation error controls step size between limits."
        }
        2 => {
            input_row(ui, "Start", &mut setup.ac.fstart);
            input_row(ui, "Stop", &mut setup.ac.fstop);
            input_row(ui, "Points", &mut setup.ac.points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.ac.sweep);
            "Small-signal sweep around the operating point."
        }
        3 => {
            input_row(ui, "Source", &mut setup.dc.source);
            input_row(ui, "Start", &mut setup.dc.start);
            input_row(ui, "Stop", &mut setup.dc.stop);
            input_row(ui, "Step", &mut setup.dc.step);
            check_row(ui, "Nested sweep", &mut setup.dc.nested);
            if setup.dc.nested {
                input_row(ui, "Source 2", &mut setup.dc.source2);
                input_row(ui, "Start 2", &mut setup.dc.start2);
                input_row(ui, "Stop 2", &mut setup.dc.stop2);
                input_row(ui, "Step 2", &mut setup.dc.step2);
            }
            "Sweeps a source over the operating range."
        }
        4 => {
            input_row(ui, "Output", &mut setup.noise.output);
            input_row(ui, "Reference", &mut setup.noise.reference);
            input_row(ui, "Input src", &mut setup.noise.input);
            input_row(ui, "Start", &mut setup.noise.fstart);
            input_row(ui, "Stop", &mut setup.noise.fstop);
            kv_row(ui, "Points", &setup.ac.points);
            "Integrated and spot noise at the chosen output; shares the AC point count."
        }
        5 => {
            input_row(ui, "Input +", &mut setup.pz.input_pos);
            input_row(ui, "Input −", &mut setup.pz.input_neg);
            input_row(ui, "Output +", &mut setup.pz.output_pos);
            input_row(ui, "Output −", &mut setup.pz.output_neg);
            choice_row(ui, "Transfer", &["V", "I"], &mut setup.pz.transfer_idx);
            choice_row(
                ui,
                "Roots",
                &["both", "poles", "zeros"],
                &mut setup.pz.analysis_idx,
            );
            "Extracts poles and zeros of the small-signal transfer."
        }
        6 => {
            input_row(ui, "Output", &mut setup.sens.output_expr);
            choice_row(ui, "Mode", &["DC", "AC"], &mut setup.sens.sens_type_idx);
            if setup.sens.sens_type_idx == 1 {
                input_row(ui, "Frequency", &mut setup.sens.ac_freq);
            }
            check_row(ui, "Include parameters", &mut setup.sens.include_params);
            check_row(ui, "Include devices", &mut setup.sens.include_devices);
            "Sensitivity of the output to every parameter."
        }
        7 => {
            input_row(ui, "Samples", &mut setup.mc.num_runs);
            input_row(ui, "Seed", &mut setup.mc.seed);
            input_row(ui, "Spread %", &mut setup.mc.variation_pct);
            choice_row(
                ui,
                "Vary",
                &["gauss", "uniform", "worst"],
                &mut setup.mc.distribution_idx,
            );
            choice_row(
                ui,
                "Base",
                &["tran", "ac", "dc", "op"],
                &mut setup.mc.base_idx,
            );
            check_row(ui, "Process variations", &mut setup.mc.process_variations);
            check_row(ui, "Mismatch variations", &mut setup.mc.mismatch_variations);
            check_row(ui, "Save every run", &mut setup.mc.save_all_runs);
            "Statistical sampling around the nominal design."
        }
        8 => {
            input_row(ui, "Fundamental", &mut setup.pss.fund_freq);
            input_row(ui, "Harmonics", &mut setup.pss.num_harmonics);
            input_row(ui, "Max iters", &mut setup.pss.max_iter);
            choice_row(ui, "Method", &["shooting", "HB"], &mut setup.pss.method_idx);
            check_row(ui, "Oscillator mode", &mut setup.pss.osc_mode);
            if setup.pss.osc_mode {
                input_row(ui, "Osc node", &mut setup.pss.osc_node);
            }
            check_row(ui, "Save harmonics", &mut setup.pss.save_harmonics);
            "Periodic steady state of the large-signal circuit."
        }
        9 => {
            input_row(ui, "Probe", &mut setup.stb.probe_source);
            input_row(ui, "Start", &mut setup.stb.start_freq);
            input_row(ui, "Stop", &mut setup.stb.stop_freq);
            input_row(ui, "Points/dec", &mut setup.stb.points_per_decade);
            check_row(ui, "Gain margin", &mut setup.stb.gain_margin);
            check_row(ui, "Phase margin", &mut setup.stb.phase_margin);
            check_row(ui, "Crossover freq", &mut setup.stb.crossover_freq);
            "Loop gain and margins via the probe source."
        }
        10 => {
            input_row(ui, "Start", &mut setup.temp.temp_start);
            input_row(ui, "Stop", &mut setup.temp.temp_stop);
            input_row(ui, "Step", &mut setup.temp.temp_step);
            choice_row(
                ui,
                "Base",
                &["op", "tran", "ac", "dc"],
                &mut setup.temp.base_idx,
            );
            check_row(ui, "Corner temps only", &mut setup.temp.corner_temps);
            "Repeats the base analysis across temperature."
        }
        11 => {
            input_row(ui, "Fundamental", &mut setup.hb.fundamental);
            input_row(ui, "Harmonics", &mut setup.hb.harmonics);
            input_row(ui, "Source", &mut setup.hb.fundamental_source);
            input_row(ui, "Oversample", &mut setup.hb.oversample);
            input_row(ui, "Max iters", &mut setup.hb.maxiter);
            choice_row(
                ui,
                "Solver",
                &["newton", "krylov"],
                &mut setup.hb.solver_idx,
            );
            check_row(ui, "Source stepping", &mut setup.hb.source_stepping);
            let mut remove: Option<usize> = None;
            for (idx, tone) in setup.hb.additional_tones.iter_mut().enumerate() {
                sub_header(ui, &format!("Tone {}", idx + 2));
                input_row(ui, "Frequency", &mut tone.frequency);
                input_row(ui, "Harmonics", &mut tone.harmonics);
                input_row(ui, "Source", &mut tone.source);
                if action_line(ui, "Remove tone") {
                    remove = Some(idx);
                }
            }
            if let Some(idx) = remove {
                setup.hb.additional_tones.remove(idx);
            }
            ui.add_space(4.0);
            if action_line(ui, "+ Add tone") {
                setup.hb.additional_tones.push(Default::default());
            }
            "Multi-tone steady state in the frequency domain."
        }
        12 => {
            input_row(ui, "Start", &mut setup.sp.start_freq);
            input_row(ui, "Stop", &mut setup.sp.stop_freq);
            input_row(ui, "Points", &mut setup.sp.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sp.sweep_type_idx);
            input_row(ui, "Z0", &mut setup.sp.z0);
            check_row(ui, "Noise parameters", &mut setup.sp.do_noise);
            check_row(ui, "Touchstone export", &mut setup.sp.touchstone_export);
            let mut remove: Option<usize> = None;
            let port_count = setup.sp.ports.len();
            for (idx, port) in setup.sp.ports.iter_mut().enumerate() {
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
                setup.sp.ports.remove(idx);
            }
            ui.add_space(4.0);
            if action_line(ui, "+ Add port") {
                setup.sp.ports.push(Default::default());
            }
            "Scattering parameters between the defined ports."
        }
        13 => {
            input_row(ui, "Start", &mut setup.pac.start_freq);
            input_row(ui, "Stop", &mut setup.pac.stop_freq);
            input_row(ui, "Points", &mut setup.pac.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.pac.sweep_type_idx);
            input_row(ui, "Input src", &mut setup.pac.input_source);
            input_row(ui, "Output", &mut setup.pac.output_node);
            input_row(ui, "Output ref", &mut setup.pac.output_ref);
            input_row(ui, "Magnitude", &mut setup.pac.pac_magnitude);
            input_row(ui, "Max sideband", &mut setup.pac.max_sideband);
            check_row(ui, "Include DC", &mut setup.pac.include_dc);
            "Small-signal AC around the periodic steady state (needs PSS)."
        }
        14 => {
            input_row(ui, "Start", &mut setup.pnoise.start_freq);
            input_row(ui, "Stop", &mut setup.pnoise.stop_freq);
            input_row(ui, "Points", &mut setup.pnoise.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.pnoise.sweep_type_idx);
            input_row(ui, "Output", &mut setup.pnoise.output_node);
            input_row(ui, "Output ref", &mut setup.pnoise.output_ref);
            input_row(ui, "Input src", &mut setup.pnoise.input_source);
            input_row(ui, "Max sideband", &mut setup.pnoise.max_sideband);
            choice_row(
                ui,
                "Refer to",
                &["output", "input", "phase"],
                &mut setup.pnoise.noise_ref_idx,
            );
            check_row(ui, "Integrated noise", &mut setup.pnoise.integrated_noise);
            check_row(ui, "Noise summary", &mut setup.pnoise.noise_summary);
            "Cyclostationary noise around the periodic steady state (needs PSS)."
        }
        15 => {
            input_row(ui, "Start", &mut setup.pxf.start_freq);
            input_row(ui, "Stop", &mut setup.pxf.stop_freq);
            input_row(ui, "Points", &mut setup.pxf.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.pxf.sweep_type_idx);
            input_row(ui, "Output", &mut setup.pxf.output_node);
            input_row(ui, "Output ref", &mut setup.pxf.output_ref);
            input_row(ui, "Out sideband", &mut setup.pxf.output_sideband);
            input_row(ui, "Input src", &mut setup.pxf.input_source);
            input_row(ui, "Max sideband", &mut setup.pxf.max_sideband);
            "Transfer functions onto a periodic steady state (needs PSS)."
        }
        16 => {
            input_row(ui, "Probe", &mut setup.pstb.probe);
            input_row(ui, "Harmonics", &mut setup.pstb.max_harmonics);
            input_row(ui, "Multipliers", &mut setup.pstb.num_multipliers);
            check_row(ui, "Annotate", &mut setup.pstb.annotate);
            check_row(ui, "Phase margin", &mut setup.pstb.phase_margin);
            check_row(ui, "Gain margin", &mut setup.pstb.gain_margin);
            "Loop stability around the periodic steady state (needs PSS)."
        }
        17 => {
            input_row(ui, "Start", &mut setup.xf.start_freq);
            input_row(ui, "Stop", &mut setup.xf.stop_freq);
            input_row(ui, "Points", &mut setup.xf.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.xf.sweep_type_idx);
            input_row(ui, "Input src", &mut setup.xf.input_source);
            input_row(ui, "Output", &mut setup.xf.output_node);
            input_row(ui, "Output ref", &mut setup.xf.output_ref);
            check_row(ui, "Group delay", &mut setup.xf.group_delay);
            check_row(ui, "Input impedance", &mut setup.xf.input_impedance);
            check_row(ui, "Output impedance", &mut setup.xf.output_impedance);
            "Small-signal transfer function and impedances."
        }
        18 => {
            sub_header(ui, "Process");
            check_row(ui, "TT — typical", &mut setup.corner.process_tt);
            check_row(ui, "SS — slow/slow", &mut setup.corner.process_ss);
            check_row(ui, "FF — fast/fast", &mut setup.corner.process_ff);
            check_row(ui, "SF — slow/fast", &mut setup.corner.process_sf);
            check_row(ui, "FS — fast/slow", &mut setup.corner.process_fs);
            sub_header(ui, "Supply");
            check_row(ui, "Sweep voltage", &mut setup.corner.enable_voltage_sweep);
            if setup.corner.enable_voltage_sweep {
                input_row(ui, "Min", &mut setup.corner.voltage_min);
                input_row(ui, "Nominal", &mut setup.corner.voltage_nom);
                input_row(ui, "Max", &mut setup.corner.voltage_max);
            }
            sub_header(ui, "Temperature");
            check_row(ui, "Sweep temperature", &mut setup.corner.enable_temp_sweep);
            if setup.corner.enable_temp_sweep {
                input_row(ui, "Cold", &mut setup.corner.temp_cold);
                input_row(ui, "Room", &mut setup.corner.temp_room);
                input_row(ui, "Hot", &mut setup.corner.temp_hot);
            }
            ui.add_space(4.0);
            check_row(ui, "Full matrix", &mut setup.corner.full_matrix);
            choice_row(
                ui,
                "Base",
                &["tran", "ac", "dc", "op"],
                &mut setup.corner.base_analysis_idx,
            );
            "Repeats the base analysis across the selected corners."
        }
        19 => {
            input_row(ui, "Fundamental", &mut setup.envelope.fundamental);
            input_row(ui, "Stop time", &mut setup.envelope.stop_time);
            input_row(ui, "Harmonics", &mut setup.envelope.harmonics);
            choice_row(
                ui,
                "Modulation",
                &["AM", "FM", "PM", "IQ"],
                &mut setup.envelope.modulation_idx,
            );
            "Envelope-following transient for modulated carriers."
        }
        20 => {
            input_row(ui, "Fundamental", &mut setup.fourier.fundamental);
            input_row(ui, "Harmonics", &mut setup.fourier.harmonics);
            input_row(ui, "Output", &mut setup.fourier.output_node);
            input_row(ui, "From", &mut setup.fourier.start_time);
            input_row(ui, "To", &mut setup.fourier.stop_time);
            check_row(ui, "Compute THD", &mut setup.fourier.compute_thd);
            check_row(ui, "Normalize", &mut setup.fourier.normalize);
            "Fourier components of a transient waveform window."
        }
        21 => {
            input_row(ui, "Years", &mut setup.reliability.years_csv);
            input_row(
                ui,
                "Min stress V",
                &mut setup.reliability.min_stress_voltage,
            );
            check_row(ui, "Hot carrier (HCI)", &mut setup.reliability.enable_hci);
            check_row(
                ui,
                "Bias instability (NBTI)",
                &mut setup.reliability.enable_nbti,
            );
            check_row(ui, "Electromigration", &mut setup.reliability.enable_em);
            "Projects device aging across the lifetime points."
        }
        22 => {
            input_row(ui, "Variables", &mut setup.optimization.variables_text);
            input_row(ui, "Objective", &mut setup.optimization.objective_node);
            input_row(ui, "Obj ref", &mut setup.optimization.objective_ref);
            choice_row(
                ui,
                "Goal",
                &["min", "max", "target"],
                &mut setup.optimization.goal_mode,
            );
            if setup.optimization.goal_mode == 2 {
                input_row(ui, "Target", &mut setup.optimization.target_value);
            }
            choice_row(
                ui,
                "Method",
                &["gradient", "pattern", "anneal"],
                &mut setup.optimization.algorithm,
            );
            input_row(ui, "Max iters", &mut setup.optimization.max_iterations);
            input_row(ui, "Tolerance", &mut setup.optimization.cost_tolerance);
            "Tunes the variables (name:min:max[:initial]) toward the goal."
        }
        23 => {
            input_row(ui, "Stop time", &mut setup.soa.stop_time);
            input_row(ui, "Step time", &mut setup.soa.step_time);
            check_row(ui, "Check Vgs", &mut setup.soa.check_vgs_max);
            if setup.soa.check_vgs_max {
                input_row(ui, "Max Vgs", &mut setup.soa.max_vgs);
            }
            check_row(ui, "Check Vds", &mut setup.soa.check_vds_max);
            if setup.soa.check_vds_max {
                input_row(ui, "Max Vds", &mut setup.soa.max_vds);
            }
            check_row(ui, "Check Vbe", &mut setup.soa.check_vbe_max);
            if setup.soa.check_vbe_max {
                input_row(ui, "Max Vbe", &mut setup.soa.max_vbe);
            }
            check_row(ui, "Check Vce", &mut setup.soa.check_vce_max);
            if setup.soa.check_vce_max {
                input_row(ui, "Max Vce", &mut setup.soa.max_vce);
            }
            "Flags excursions outside the safe operating area during transient."
        }
        24 => {
            input_row(ui, "Start", &mut setup.ac.fstart);
            input_row(ui, "Stop", &mut setup.ac.fstop);
            input_row(ui, "Points", &mut setup.ac.points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.ac.sweep);
            input_row(ui, "f2/f1", &mut setup.disto_f2_over_f1);
            "Harmonic and intermodulation distortion; empty ratio means single-tone."
        }
        _ => "",
    }
}
