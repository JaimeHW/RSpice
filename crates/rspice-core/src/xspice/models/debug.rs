//! XSPICE debug and example code models.

use crate::Value;
use crate::xspice::{CmContext, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType};

#[derive(Debug, Default)]
pub struct PrintParamTypes;

impl CodeModel for PrintParamTypes {
    fn name(&self) -> &str {
        "print_param_types"
    }

    fn description(&self) -> &str {
        "ngspice example model that accepts every XSPICE parameter channel"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![PortSpec {
                name: "in".to_string(),
                direction: PortDirection::In,
                default_type: PortType::Voltage,
                allowed_types: vec![
                    PortType::Voltage,
                    PortType::DifferentialVoltage,
                    PortType::Current,
                    PortType::DifferentialCurrent,
                    PortType::VoltageName,
                ],
                is_vector: true,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Input vector".to_string(),
            }]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::integer("integer", 1).with_description("Integer parameter"),
                ParamSpec::real("real", 1.0).with_description("Real parameter"),
                ParamSpec::string("complex", "<1 1>").with_description("Complex parameter"),
                ParamSpec::string("string", "one").with_description("String parameter"),
                ParamSpec::integer_vector("integer_array", vec![1])
                    .with_description("Integer array parameter"),
                ParamSpec::real_vector("real_array", vec![1.0 as Value])
                    .with_description("Real array parameter"),
                ParamSpec::string_vector("complex_array", vec!["<1 1>".to_string()])
                    .with_description("Complex array parameter"),
                ParamSpec::string_vector("string_array", vec!["one".to_string()])
                    .with_description("String array parameter"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
}
