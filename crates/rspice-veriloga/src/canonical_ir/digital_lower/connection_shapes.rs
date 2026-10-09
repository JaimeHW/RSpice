//! Port layout uses the same expression types as executable process lowering.
use super::*;

/// Prepared once per occurrence and reused across its mixed input connections.
pub(crate) struct ConnectionShapes<'a> {
    signals: Vec<DigitalSignal>,
    arrays: HashMap<DigitalSignalId, super::super::digital::DigitalArray>,
    index: HashMap<&'a str, DigitalSignalId>,
    array_names: HashSet<SmolStr>,
    constants: ResolvedConstants,
    analog_variables: HashMap<SmolStr, AnalogVariable>,
    time_scale: crate::time_scale::ModuleTimeScale,
}

impl<'a> ConnectionShapes<'a> {
    pub(crate) fn new(
        module: &'a crate::semantic::AnalyzedModule,
        source: &crate::ast::Module,
    ) -> Result<Self, Vec<DigitalLoweringDiagnostic>> {
        let digital = &module.digital;
        let (signals, arrays, ids) = lower_signals(&digital.signals)?;
        let constants = constants::resolve(
            &digital.constants,
            digital.time_scale,
            &[],
            &[],
            source.instances.iter().flat_map(|instance| {
                instance
                    .connections
                    .iter()
                    .filter_map(|connection| match connection {
                        crate::ast::Connection::Named { signal, .. }
                        | crate::ast::Connection::Ordered { signal, .. } => signal.as_ref(),
                    })
            }),
        )?;
        Ok(Self {
            array_names: digital
                .signals
                .iter()
                .filter(|signal| signal.unpacked.is_some())
                .map(|signal| signal.name.clone())
                .collect(),
            signals,
            arrays: arrays
                .into_iter()
                .map(|array| (array.storage.base, array))
                .collect(),
            index: digital
                .signals
                .iter()
                .zip(ids)
                .map(|(signal, id)| (signal.name.as_str(), id))
                .collect(),
            constants,
            analog_variables: analog_variables(module),
            time_scale: digital.time_scale,
        })
    }

    pub(crate) fn four_state_width(&mut self, expression: &Expression) -> Option<u32> {
        let mut probes = Vec::new();
        let lowerer = ProcessLowerer {
            analog_local_inputs: HashMap::new(),
            process: None,
            local_arrays: Vec::new(),
            constant_expression: false,
            time_scale: self.time_scale,
            signals: &mut self.signals,
            arrays: &self.arrays,
            index: &self.index,
            array_names: &self.array_names,
            constants: &self.constants,
            analog_variables: &self.analog_variables,
            probes: &mut probes,
            builder: ProcessBuilder::new(),
            diagnostics: Vec::new(),
            locals: Vec::new(),
            scopes: Vec::new(),
            static_scopes: HashMap::new(),
            static_local_count: 0,
        };
        let shape = expressions::shape(&lowerer, expression);
        (!shape.real).then_some(shape.width)
    }
}
