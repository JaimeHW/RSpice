//! ngspice-compatible builtin metadata overlays.
//!
//! Builtin model implementations own behavior. This module owns descriptor
//! parity where the Rust model predates stricter `ifspec.ifs` metadata.

use std::sync::Arc;

use crate::{Complex64, Value};

use super::{CmContext, CmResult, CodeModel, ParamSpec, PortSpec, XspiceCheckpointSupport};

pub(crate) fn with_builtin_metadata(model: Arc<dyn CodeModel>) -> Arc<dyn CodeModel> {
    let mut ports = model.ports().to_vec();
    let mut parameters = model.parameters().to_vec();
    let changed = apply_builtin_metadata(model.name(), &mut ports, &mut parameters);

    if changed {
        Arc::new(MetadataOverlayModel {
            inner: model,
            ports,
            parameters,
        })
    } else {
        model
    }
}

struct MetadataOverlayModel {
    inner: Arc<dyn CodeModel>,
    ports: Vec<PortSpec>,
    parameters: Vec<ParamSpec>,
}

impl CodeModel for MetadataOverlayModel {
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    fn ports(&self) -> &[PortSpec] {
        &self.ports
    }

    fn parameters(&self) -> &[ParamSpec] {
        &self.parameters
    }

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        self.inner.can_skip_unchanged_event_inputs()
    }

    fn requires_conservative_newton_damping(&self) -> bool {
        self.inner.requires_conservative_newton_damping()
    }

    fn has_memoryless_linear_transient_stamp(&self) -> bool {
        self.inner.has_memoryless_linear_transient_stamp()
    }

    fn checkpoint_support(&self, ctx: &CmContext) -> XspiceCheckpointSupport {
        self.inner.checkpoint_support(ctx)
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        self.inner.init(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        self.inner.evaluate(ctx)
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        self.inner.ac_gain(ctx)
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        self.inner
            .excludes_output_from_transient_voltage_lte(output_port)
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        self.inner.output_input_partials(ctx, output_port)
    }

    fn output_input_vector_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
    ) -> Vec<(String, usize, Value)> {
        self.inner.output_input_vector_partials(ctx, output_port)
    }

    fn output_vector_input_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        output_index: usize,
    ) -> Vec<(String, Value)> {
        self.inner
            .output_vector_input_partials(ctx, output_port, output_index)
    }

    fn output_vector_input_vector_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        output_index: usize,
    ) -> Vec<(String, usize, Value)> {
        self.inner
            .output_vector_input_vector_partials(ctx, output_port, output_index)
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        self.inner
            .output_input_ac_partials(ctx, output_port, frequency)
    }

    fn output_input_vector_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, usize, Complex64)> {
        self.inner
            .output_input_vector_ac_partials(ctx, output_port, frequency)
    }

    fn output_vector_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        output_index: usize,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        self.inner
            .output_vector_input_ac_partials(ctx, output_port, output_index, frequency)
    }

    fn output_vector_input_vector_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        output_index: usize,
        frequency: Value,
    ) -> Vec<(String, usize, Complex64)> {
        self.inner
            .output_vector_input_vector_ac_partials(ctx, output_port, output_index, frequency)
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        self.inner.transient_breakpoints(ctx)
    }
}

fn apply_builtin_metadata(
    model: &str,
    _ports: &mut Vec<PortSpec>,
    params: &mut Vec<ParamSpec>,
) -> bool {
    let mut changed = false;

    match model {
        "cmeter" | "lmeter" => {
            let before = params.len();
            params.retain(|param| param.name != "__rspice_measured_value");
            changed |= params.len() != before;
        }
        _ => {}
    }

    match model {
        "adc_bridge" => {
            changed |= set_soft_min_many(params, &["rise_delay", "fall_delay"], 1.0e-12);
        }
        "bidi_bridge" => {
            changed |= set_soft_min_many(
                params,
                &["rise_delay", "fall_delay", "t_rise", "t_fall"],
                1.0e-12,
            );
            changed |= set_soft_min_many(params, &["r_low", "r_high", "r_stl", "r_sth"], 1.0e-6);
            changed |= set_soft_range_many(params, &["strength", "direction", "smooth"], 0.0, 2.0);
        }
        "dac_bridge" => {
            changed |= set_soft_min_many(params, &["t_rise", "t_fall"], 1.0e-12);
        }
        "d_and" | "d_buffer" | "d_inverter" | "d_lut" | "d_nand" | "d_nor" | "d_or" | "d_xnor"
        | "d_xor" => {
            changed |= set_soft_min_many(params, &["rise_delay", "fall_delay"], 1.0e-12);
        }
        "d_open_c" => {
            changed |= set_soft_min_many(params, &["open_delay", "fall_delay"], 1.0e-12);
        }
        "d_open_e" => {
            changed |= set_soft_min_many(params, &["open_delay", "rise_delay"], 1.0e-12);
        }
        "d_tristate" => {
            changed |= set_soft_min(params, "delay", 1.0e-12);
        }
        "d_genlut" => {
            changed |= set_real_vector_default(params, "rise_delay", &[1.0e-9]);
            changed |= set_real_vector_default(params, "fall_delay", &[1.0e-9]);
            changed |= set_real_vector_default(params, "input_load", &[1.0e-12]);
            changed |= set_real_vector_default(params, "input_delay", &[0.0]);
            changed |= set_soft_min_many(params, &["rise_delay", "fall_delay"], 1.0e-12);
        }
        "d_dff" | "d_jkff" | "d_srff" | "d_tff" => {
            changed |= set_soft_min_many(
                params,
                &[
                    "clk_delay",
                    "set_delay",
                    "reset_delay",
                    "rise_delay",
                    "fall_delay",
                ],
                1.0e-12,
            );
            changed |= set_soft_range(params, "ic", 0.0, 2.0);
        }
        "d_dlatch" => {
            changed |= set_soft_min_many(
                params,
                &[
                    "data_delay",
                    "enable_delay",
                    "set_delay",
                    "reset_delay",
                    "rise_delay",
                    "fall_delay",
                ],
                1.0e-12,
            );
            changed |= set_soft_range(params, "ic", 0.0, 2.0);
        }
        "d_srlatch" => {
            changed |= set_soft_min_many(
                params,
                &[
                    "sr_delay",
                    "enable_delay",
                    "set_delay",
                    "reset_delay",
                    "rise_delay",
                    "fall_delay",
                ],
                1.0e-12,
            );
            changed |= set_soft_range(params, "ic", 0.0, 2.0);
        }
        "d_fdiv" => {
            changed |= set_soft_min_many(params, &["div_factor", "high_cycles"], 1.0);
            changed |= set_soft_min(params, "i_count", 0.0);
            changed |= set_soft_min_many(params, &["rise_delay", "fall_delay"], 1.0e-12);
        }
        "d_ram" => {
            changed |= set_soft_range(params, "select_value", 0.0, 32767.0);
            changed |= set_soft_range(params, "ic", 0.0, 2.0);
            changed |= set_soft_min(params, "read_delay", 1.0e-12);
        }
        "d_osc" => {
            changed |= set_soft_min(params, "freq_array", 0.0);
            changed |= set_soft_range(params, "duty_cycle", 1.0e-6, 0.999999);
            changed |= set_soft_range(params, "init_phase", -180.0, 360.0);
            changed |= set_soft_min_many(params, &["rise_delay", "fall_delay"], 0.0);
        }
        "d_pwm" => {
            changed |= set_soft_range(params, "dc_array", 0.0, 1.0);
            changed |= set_soft_min(params, "frequency", 1.0e-6);
            changed |= set_soft_range(params, "init_phase", -180.0, 360.0);
            changed |= set_soft_min_many(params, &["rise_delay", "fall_delay"], 0.0);
        }
        "d_cosim" => {
            changed |= set_min(params, "delay", 1.0e-12);
            changed |= set_min(params, "queue_size", 1.0);
        }
        "d_to_real" => {
            changed |= set_min(params, "delay", 1.0e-15);
        }
        "real_delay" => {
            changed |= set_min(params, "delay", 1.0e-15);
        }
        "real_to_v" => {
            changed |= set_min(params, "transition_time", 1.0e-15);
        }
        "astate" => {
            changed |= set_soft_range(params, "astate_no", 0.0, 3.0);
        }
        "delay" => {
            changed |= set_soft_min(params, "buffer_size", 1.0);
            changed |= set_soft_min_many(params, &["delmin", "delmax"], 0.0);
        }
        "divide" => {
            changed |= set_soft_min(params, "den_lower_limit", 1.0e-10);
        }
        "d_dt" | "int" | "limit" => {
            changed |= set_required_many(params, &["out_lower_limit", "out_upper_limit"], true);
        }
        "hyst" => {
            changed |= set_soft_min(params, "hyst", 0.0);
        }
        "ilimit" => {
            changed |= set_soft_range_many(params, &["r_out_source", "r_out_sink"], 1.0e-9, 1.0e9);
            changed |= set_soft_min_many(params, &["i_limit_source", "i_limit_sink"], 1.0e-12);
            changed |= set_soft_min_many(
                params,
                &[
                    "v_pwr_range",
                    "i_source_range",
                    "i_sink_range",
                    "r_out_domain",
                ],
                1.0e-15,
            );
        }
        "oneshot" => {
            changed |= set_soft_min(params, "pw_array", 0.0);
        }
        "pwl" | "pwlts" => {
            changed |= set_soft_range(params, "input_domain", 1.0e-12, 0.5);
        }
        "s_xfer" => {
            changed |= set_required_many(params, &["num_coeff", "den_coeff"], true);
            changed |= set_real_vector_default(params, "int_ic", &[0.0]);
        }
        "xfer" => {
            changed |= set_real_vector_default(params, "table", &[0.0, 0.0, 0.0]);
            changed |= set_soft_min(params, "span", 3.0);
            changed |= set_soft_min(params, "offset", 1.0);
        }
        "mult" | "summer" => {
            changed |= set_real_vector_default(params, "in_offset", &[0.0]);
            changed |= set_real_vector_default(params, "in_gain", &[1.0]);
        }
        "sine" => {
            changed |= set_soft_min(params, "freq_array", 0.0);
        }
        "square" | "triangle" => {
            changed |= set_soft_min(params, "freq_array", 0.0);
            changed |= set_soft_range(params, "duty_cycle", 1.0e-6, 0.999999);
        }
        "table2d" | "table3d" => {
            changed |= set_soft_min(params, "order", 2.0);
            changed |= set_soft_range(params, "verbose", 0.0, 2.0);
        }
        "core" => {
            changed |= set_soft_range(params, "input_domain", 1.0e-12, 0.5);
            changed |= set_soft_range(params, "mode", 1.0, 2.0);
            changed |= set_soft_min(params, "hyst", 0.0);
        }
        "potentiometer" => {
            changed |= set_soft_range(params, "position", 0.0, 1.0);
        }
        "seegen" => {
            changed |= set_soft_range(params, "angle", 0.0, 1.57079);
        }
        "sidiode" => {
            changed |= set_soft_min(params, "ron", 1.0e-6);
            changed |= set_soft_min(params, "roff", 1.0e-12);
            changed |= set_soft_min_many(params, &["vfwd", "vrev", "epsilon", "revepsilon"], 0.0);
            changed |= set_soft_min_many(params, &["ilimit", "revilimit"], 1.0e-15);
        }
        "zener" => {
            changed |= set_soft_range(params, "v_breakdown", 1.0e-6, 1.0e6);
            changed |= set_soft_min(params, "i_breakdown", 1.0e-9);
            changed |= set_soft_min(params, "r_breakdown", 1.0e-12);
            changed |= set_soft_min(params, "i_rev", 1.0e-9);
            changed |= set_soft_min(params, "i_sat", 1.0e-15);
            changed |= set_soft_range(params, "n_forward", 0.1, 10.0);
        }
        _ => {}
    }

    changed
}

fn param_mut<'a>(params: &'a mut [ParamSpec], name: &str) -> Option<&'a mut ParamSpec> {
    params
        .iter_mut()
        .find(|param| param.name.eq_ignore_ascii_case(name))
}

fn set_min(params: &mut [ParamSpec], name: &str, min: Value) -> bool {
    let Some(param) = param_mut(params, name) else {
        return false;
    };
    if param.min == Some(min) && !param.min_is_soft {
        return false;
    }
    param.min = Some(min);
    param.min_is_soft = false;
    true
}

fn set_soft_min(params: &mut [ParamSpec], name: &str, min: Value) -> bool {
    let Some(param) = param_mut(params, name) else {
        return false;
    };
    if param.min == Some(min) && param.min_is_soft {
        return false;
    }
    param.min = Some(min);
    param.min_is_soft = true;
    true
}

fn set_soft_max(params: &mut [ParamSpec], name: &str, max: Value) -> bool {
    let Some(param) = param_mut(params, name) else {
        return false;
    };
    if param.max == Some(max) && param.max_is_soft {
        return false;
    }
    param.max = Some(max);
    param.max_is_soft = true;
    true
}

fn set_soft_range(params: &mut [ParamSpec], name: &str, min: Value, max: Value) -> bool {
    set_soft_min(params, name, min) | set_soft_max(params, name, max)
}

fn set_min_many(params: &mut [ParamSpec], names: &[&str], min: Value) -> bool {
    names
        .iter()
        .fold(false, |changed, name| set_min(params, name, min) | changed)
}

fn set_soft_min_many(params: &mut [ParamSpec], names: &[&str], min: Value) -> bool {
    names.iter().fold(false, |changed, name| {
        set_soft_min(params, name, min) | changed
    })
}

fn set_soft_range_many(params: &mut [ParamSpec], names: &[&str], min: Value, max: Value) -> bool {
    names.iter().fold(false, |changed, name| {
        set_soft_range(params, name, min, max) | changed
    })
}

fn set_required(params: &mut [ParamSpec], name: &str, required: bool) -> bool {
    let Some(param) = param_mut(params, name) else {
        return false;
    };
    if param.required == required {
        return false;
    }
    param.required = required;
    true
}

fn set_required_many(params: &mut [ParamSpec], names: &[&str], required: bool) -> bool {
    names.iter().fold(false, |changed, name| {
        set_required(params, name, required) | changed
    })
}

fn set_real_vector_default(params: &mut [ParamSpec], name: &str, values: &[Value]) -> bool {
    let Some(param) = param_mut(params, name) else {
        return false;
    };
    if param.real_vector_default.as_deref() == Some(values) {
        return false;
    }
    param.real_vector_default = Some(values.to_vec());
    true
}
