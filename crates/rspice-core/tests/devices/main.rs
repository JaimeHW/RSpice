//! Native device models: semiconductors, passives, switches and transmission
//! lines.

#[path = "../common/family_src.rs"]
mod family_src;
#[path = "../common/log_capture.rs"]
mod log_capture;
#[path = "../common/op_point.rs"]
mod op_point;
#[path = "../common/waveform.rs"]
mod waveform;

#[allow(clippy::excessive_precision)]
mod b3soi_native;
mod bjt_excess_phase;
mod bjt_level_policy;
mod bjt_pspice_nk;
mod branch_resistor_noise;
mod bsim3_native;
mod bsim3_newton_ownership;
#[allow(clippy::excessive_precision)]
mod bsim4_native;
mod controlled_sources_ext;
mod cpl_telegrapher;
mod cpl_time_scale;
mod device_count_inventory;
mod diode_rectifier_oracle;
mod ekv26_native;
mod ekv3_native;
mod functional_parts;
mod generic_switch;
mod hfet2_native;
mod inductor_geometry;
mod inductor_transient;
#[allow(clippy::excessive_precision)]
mod jfet2_native;
mod jfet_family_native;
mod ltra_ac_oracle;
mod ltra_model_validation;
mod ltra_rg_line;
#[allow(clippy::excessive_precision)]
mod mos3_native;
mod mos9_native;
mod mos_level_policy;
mod multiplicity;
mod passive_model_errors;
mod resistor_model_policy;
mod resistor_model_sampling;
mod resistor_override_admission;
mod switch_model_policy;
mod switch_noise;
mod switch_transition_law;
mod thermal_model_evaluation;
mod vbic_excess_phase_oracle;
mod vdmos_native;
