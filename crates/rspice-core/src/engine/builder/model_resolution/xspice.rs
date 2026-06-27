use super::*;

pub(in crate::engine::builder) struct ResolvedXspiceModel {
    pub(in crate::engine::builder) code_model: std::sync::Arc<dyn crate::xspice::CodeModel>,
    pub(in crate::engine::builder) numeric_params: Vec<(String, f64)>,
    pub(in crate::engine::builder) string_params: Vec<(String, String)>,
    pub(in crate::engine::builder) real_vector_params: Vec<(String, Vec<f64>)>,
    pub(in crate::engine::builder) integer_vector_params: Vec<(String, Vec<i64>)>,
}

fn merge_numeric_params(base: &[(String, f64)], overrides: &[(String, f64)]) -> Vec<(String, f64)> {
    let mut merged = base.to_vec();

    for (name, value) in overrides {
        if let Some(existing) = merged
            .iter_mut()
            .find(|(existing_name, _)| existing_name.eq_ignore_ascii_case(name))
        {
            existing.1 = *value;
        } else {
            merged.push((name.clone(), *value));
        }
    }

    merged
}

fn reject_param_names_for_vector_specs<'a>(
    code_model: &dyn crate::xspice::CodeModel,
    names: impl IntoIterator<Item = &'a str>,
    channel: &str,
) -> Result<(), SimulationError> {
    for name in names {
        if let Some(spec) = code_model
            .parameters()
            .iter()
            .find(|spec| spec.name.eq_ignore_ascii_case(name))
            && matches!(
                spec.param_type,
                crate::xspice::ParamType::RealVector | crate::xspice::ParamType::IntegerVector
            )
        {
            return Err(SimulationError::Circuit(format!(
                "XSPICE model '{}' vector parameter '{}' was given a {} value",
                code_model.name(),
                name,
                channel
            )));
        }
    }
    Ok(())
}

fn reject_scalar_params_for_vector_specs(
    code_model: &dyn crate::xspice::CodeModel,
    params: &[(String, f64)],
) -> Result<(), SimulationError> {
    reject_param_names_for_vector_specs(
        code_model,
        params.iter().map(|(name, _)| name.as_str()),
        "scalar",
    )
}

fn resolve_vector_params(
    code_model: &dyn crate::xspice::CodeModel,
    vectors: &[(String, Vec<f64>)],
) -> Result<(Vec<(String, Vec<f64>)>, Vec<(String, Vec<i64>)>), SimulationError> {
    let param_specs: Vec<&crate::xspice::ParamSpec> = code_model.parameters().iter().collect();
    let mut real_vectors = Vec::new();
    let mut integer_vectors = Vec::new();

    for (name, values) in vectors {
        match param_specs
            .iter()
            .copied()
            .find(|spec| spec.name.eq_ignore_ascii_case(name))
            .map(|spec| spec.param_type)
        {
            Some(crate::xspice::ParamType::IntegerVector) => {
                let mut integer_values = Vec::with_capacity(values.len());
                for value in values {
                    if !value.is_finite() || (value.round() - value).abs() > 1.0e-12 {
                        return Err(SimulationError::Circuit(format!(
                            "XSPICE model '{}' integer-vector parameter '{}' expected integer value, got {}",
                            code_model.name(),
                            name,
                            value
                        )));
                    }
                    integer_values.push(value.round() as i64);
                }
                integer_vectors.push((name.clone(), integer_values));
            }
            Some(crate::xspice::ParamType::RealVector) | None => {
                real_vectors.push((name.clone(), values.clone()));
            }
            Some(other) => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE model '{}' parameter '{}' expects {:?}, got real-vector value",
                    code_model.name(),
                    name,
                    other
                )));
            }
        }
    }

    Ok((real_vectors, integer_vectors))
}

pub(in crate::engine::builder) fn resolve_xspice_model_instance(
    netlist: &Netlist,
    registry: &crate::xspice::CodeModelRegistry,
    model_name: &str,
    instance_params: &[(String, f64)],
) -> Result<ResolvedXspiceModel, SimulationError> {
    if let Some(code_model) = registry.get(model_name) {
        reject_scalar_params_for_vector_specs(code_model.as_ref(), instance_params)?;
        return Ok(ResolvedXspiceModel {
            code_model,
            numeric_params: instance_params.to_vec(),
            string_params: Vec::new(),
            real_vector_params: Vec::new(),
            integer_vector_params: Vec::new(),
        });
    }

    let model_def = find_model_def(netlist, model_name).ok_or_else(|| {
        SimulationError::Circuit(format!("Unknown XSPICE model '{}'", model_name))
    })?;

    let code_model = registry.get(&model_def.model_type).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Unknown XSPICE model '{}' (alias '{}' resolves to unregistered code model '{}')",
            model_name, model_def.name, model_def.model_type
        ))
    })?;

    reject_scalar_params_for_vector_specs(code_model.as_ref(), &model_def.params)?;
    reject_param_names_for_vector_specs(
        code_model.as_ref(),
        model_def
            .string_params
            .iter()
            .map(|(name, _)| name.as_str()),
        "string",
    )?;
    reject_param_names_for_vector_specs(
        code_model.as_ref(),
        model_def.expr_params.iter().map(|(name, _)| name.as_str()),
        "expression",
    )?;
    reject_scalar_params_for_vector_specs(code_model.as_ref(), instance_params)?;

    let (real_vector_params, mut integer_vector_params) =
        resolve_vector_params(code_model.as_ref(), &model_def.real_vector_params)?;
    integer_vector_params.extend(model_def.integer_vector_params.clone());

    Ok(ResolvedXspiceModel {
        code_model,
        numeric_params: merge_numeric_params(&model_def.params, instance_params),
        string_params: model_def.string_params.clone(),
        real_vector_params,
        integer_vector_params,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::{CmContext, CmResult, CodeModel, ParamSpec, PortSpec};
    use std::sync::Arc;

    struct ParamOnlyModel {
        name: &'static str,
        params: Vec<ParamSpec>,
        ports: Vec<PortSpec>,
    }

    impl ParamOnlyModel {
        fn new(name: &'static str, params: Vec<ParamSpec>) -> Self {
            Self {
                name,
                params,
                ports: Vec::new(),
            }
        }
    }

    impl CodeModel for ParamOnlyModel {
        fn name(&self) -> &str {
            self.name
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }
    }

    #[test]
    fn xspice_resolver_carries_real_vector_model_params() {
        let netlist = Netlist::parse(
            "xspice vector model\n\
             .model vp vector_probe (points=[1 2.5 4])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", vec![0.0])],
        )));

        let resolved = resolve_xspice_model_instance(&netlist, &registry, "vp", &[])
            .expect("xspice model resolves");

        assert_eq!(
            resolved
                .real_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("points"))
                .map(|(_, values)| values.as_slice()),
            Some(&[1.0, 2.5, 4.0][..])
        );
    }

    #[test]
    fn xspice_resolver_converts_integer_vector_model_params() {
        let netlist = Netlist::parse(
            "xspice integer vector model\n\
             .model bp bit_probe (bits=[1 0 1 1])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "bit_probe",
            vec![ParamSpec::integer_vector("bits", vec![0])],
        )));

        let resolved = resolve_xspice_model_instance(&netlist, &registry, "bp", &[])
            .expect("xspice model resolves");

        assert_eq!(
            resolved
                .integer_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("bits"))
                .map(|(_, values)| values.as_slice()),
            Some(&[1, 0, 1, 1][..])
        );
        assert!(
            resolved
                .real_vector_params
                .iter()
                .all(|(name, _)| !name.eq_ignore_ascii_case("bits"))
        );
    }

    #[test]
    fn xspice_resolver_rejects_fractional_integer_vector_model_params() {
        let netlist = Netlist::parse(
            "xspice invalid integer vector model\n\
             .model bp bit_probe (bits=[1 0.5])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "bit_probe",
            vec![ParamSpec::integer_vector("bits", vec![0])],
        )));

        let err = match resolve_xspice_model_instance(&netlist, &registry, "bp", &[]) {
            Ok(_) => panic!("fractional integer vector element must be rejected"),
            Err(err) => err,
        };

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("bits") && message.contains("0.5"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_resolver_rejects_scalar_model_param_for_known_vector_param() {
        let netlist = Netlist::parse(
            "xspice scalar where vector expected\n\
             .model vp vector_probe (points=3)\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", vec![0.0])],
        )));

        let err = match resolve_xspice_model_instance(&netlist, &registry, "vp", &[]) {
            Ok(_) => panic!("scalar value for known vector parameter must be rejected"),
            Err(err) => err,
        };

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("points") && lowered.contains("scalar"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_resolver_rejects_string_model_param_for_known_vector_param() {
        let netlist = Netlist::parse(
            "xspice string where vector expected\n\
             .model vp vector_probe (points=\"bad\")\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", vec![0.0])],
        )));

        let err = match resolve_xspice_model_instance(&netlist, &registry, "vp", &[]) {
            Ok(_) => panic!("string value for known vector parameter must be rejected"),
            Err(err) => err,
        };

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("points") && lowered.contains("string"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_resolver_rejects_expr_model_param_for_known_vector_param() {
        let netlist = Netlist::parse(
            "xspice expression where vector expected\n\
             .model vp vector_probe (points={missing_param})\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", vec![0.0])],
        )));

        let err = match resolve_xspice_model_instance(&netlist, &registry, "vp", &[]) {
            Ok(_) => panic!("expression value for known vector parameter must be rejected"),
            Err(err) => err,
        };

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("points") && lowered.contains("expression"),
            "unexpected error: {message}"
        );
    }
}
