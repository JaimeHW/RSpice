//! Operating-point, DC, AC, noise, distortion, Fourier, measurement and
//! control-script analyses.

#[cfg(not(target_arch = "wasm32"))]
#[path = "../common/allocations.rs"]
mod allocations;
#[path = "../common/family_src.rs"]
mod family_src;
#[path = "../common/log_capture.rs"]
mod log_capture;
#[path = "../common/stop_at_completion.rs"]
mod stop_at_completion;

mod ac_physical_dependents;
mod ac_physical_operator;
mod analysis_card_transactions;
mod analysis_error_origins;
mod authored_card_runners;
mod continuous_measurement_limits;
mod control_dc;
mod control_dcmatch;
mod control_distortion;
mod control_execution;
mod control_frequency_data;
mod control_noise;
mod control_stability;
mod control_transfer_function;
mod dc_operating_point_aids;
mod dc_topology_policy;
mod dc_two_source;
mod distortion_physical_products;
mod fft_current_derivatives;
mod fourier_current_impulses;
mod fourier_fail_closed;
mod frequency_data;
mod frequency_data_expansion;
mod frequency_data_temperature;
mod frequency_data_zero;
mod gp_feedback_weil;
mod meas_deriv_param;
mod measure_bound_reference;
mod measure_current_derivatives;
mod measure_failvalue;
#[cfg(not(target_arch = "wasm32"))]
mod measurement_event_allocations;
mod monte_carlo_engine;
mod noise_input_reference;
mod noise_mechanism;
mod op_report;
mod operating_point_seed;
mod option_cshunt;
mod option_schedule_statistics;
mod pole_spectrum_circuit;
mod pole_zero_regression;
mod relative_spectrum_helpers;
mod rf_tier0;
mod sparam_extraction;
mod sparse_missing_positions;
#[cfg(not(target_arch = "wasm32"))]
mod startup_card_allocations;
mod stb_circuit_poles;
mod terminal_current_probes;
mod topology_diagnostics;
mod transfer_function;
mod transfer_function_units;
