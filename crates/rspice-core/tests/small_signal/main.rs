//! Small-signal analyses that also run in a WebAssembly worker: gain-phase,
//! pole-zero, stability, sensitivity and control transfer.

#[path = "../common/stop_at_completion.rs"]
mod stop_at_completion;

mod analysis_numeric_fields;
mod bjt_excess_phase_transient;
mod control_complex_scalars;
mod control_pole_zero;
mod control_sensitivity;
mod control_transfer_scalars;
mod forward_analyses;
mod frequency_grid_errors;
mod gp_history_resources;
mod gp_phase_cancellation;
mod gp_phase_regimes;
mod pole_zero_descriptor_rank;
mod pole_zero_high_frequency;
mod pole_zero_public_contract;
mod sensitivity_linear_devices;
mod sensitivity_probe_contract;
mod stability_projection;
mod stb_availability;
mod stb_dc_gain;
mod stb_loopgain;
mod stb_precision;
mod stb_zero_gain;
