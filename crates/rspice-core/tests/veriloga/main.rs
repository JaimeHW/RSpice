//! Verilog-A, Verilog-AMS and mixed-signal integration tests.

#[cfg(feature = "veriloga")]
#[path = "../common/mod.rs"]
mod common;
#[cfg(feature = "veriloga")]
#[path = "../common/digital_trace_invariants.rs"]
mod digital_trace_invariants;
#[path = "../common/family_src.rs"]
mod family_src;
#[cfg(feature = "veriloga")]
#[path = "../common/log_capture.rs"]
mod log_capture;
#[cfg(feature = "veriloga")]
#[path = "../common/model_outcome.rs"]
mod model_outcome;
#[cfg(feature = "veriloga")]
#[path = "../common/toggle_peer.rs"]
mod toggle_peer;
#[cfg(feature = "veriloga")]
#[path = "../common/veriloga_source_fixture.rs"]
mod veriloga_source_fixture;

#[cfg(feature = "veriloga")]
mod analog_event_subscriptions;
mod assertions;
#[cfg(feature = "veriloga")]
mod authored_connect_modules;
#[cfg(feature = "veriloga")]
mod boundary_supply_derivation;
mod capability_refusals;
#[cfg(feature = "veriloga")]
mod connect_module_insertion;
#[cfg(feature = "veriloga")]
mod connect_rule_parameters;
#[cfg(feature = "veriloga")]
mod d_process_coupled;
mod device_engine_roundtrip;
mod diode_native;
mod expression_stack_safety;
#[cfg(feature = "veriloga")]
mod hdl_external_paths;
#[cfg(feature = "veriloga")]
mod measure_event_operands;
#[cfg(feature = "veriloga")]
mod mixed_bit_aliases;
#[cfg(feature = "veriloga")]
mod mixed_dc_sweep_state;
#[cfg(feature = "veriloga")]
mod mixed_live_supply;
#[cfg(feature = "veriloga")]
mod mixed_small_signal_held;
#[cfg(feature = "veriloga")]
mod pole_zero_veriloga_rational;
mod sensitivity_source_waveforms;
mod small_signal_transient_state;
mod spectre_analog_statements;
mod transient_checkpoint;
mod transient_events_at_stop_time;
#[cfg(feature = "veriloga")]
mod verilog_on_top;
#[cfg(feature = "veriloga")]
mod veriloga_ac;
#[cfg(feature = "veriloga")]
mod veriloga_bsim4_ac_oracle;
#[cfg(feature = "veriloga")]
mod veriloga_bsim4_oracle;
#[cfg(feature = "veriloga")]
mod veriloga_connect_source_closure;
#[cfg(feature = "veriloga")]
mod veriloga_dc_lifecycle;
#[cfg(feature = "veriloga")]
mod veriloga_elaboration_diagnostics;
#[cfg(feature = "veriloga")]
mod veriloga_hb;
#[cfg(feature = "veriloga")]
mod veriloga_indirect;
#[cfg(feature = "veriloga")]
mod veriloga_instance_parameters;
#[cfg(feature = "veriloga")]
mod veriloga_mfactor;
#[cfg(feature = "veriloga")]
mod veriloga_mixed_signal_regressions;
#[cfg(feature = "veriloga")]
mod veriloga_mixed_signal_route;
#[cfg(feature = "veriloga")]
mod veriloga_module_timing;
#[cfg(feature = "veriloga")]
mod veriloga_noise;
#[cfg(feature = "veriloga")]
mod veriloga_noise_process;
#[cfg(feature = "veriloga")]
mod veriloga_operating_points;
#[cfg(feature = "veriloga")]
mod veriloga_project_runtime_batch;
#[cfg(feature = "veriloga")]
mod veriloga_project_virtual_identity;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
mod veriloga_source_dependencies;
#[cfg(feature = "veriloga")]
mod veriloga_source_directive_spellings;
#[cfg(feature = "veriloga")]
mod veriloga_timestep;
mod xspice_vcd_source;
