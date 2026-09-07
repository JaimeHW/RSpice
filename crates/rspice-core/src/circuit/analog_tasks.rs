//! Accepted analog task delivery across every Verilog-A instance route.

use super::CircuitData;
use rspice_veriloga_runtime::{AnalogTaskEvent, AnalogTaskKind};

impl CircuitData {
    /// Inspect a solved equilibrium point before advancing any model history.
    /// Mixed transient candidates belong to the host's committable trial and
    /// must not be inferred from its last rejected numerical probe.
    pub(crate) fn first_equilibrium_candidate_analog_task(
        &self,
        kind: AnalogTaskKind,
    ) -> Result<Option<AnalogTaskEvent<'_>>, String> {
        #[cfg(feature = "veriloga")]
        {
            self.ensure_no_mixed_signal_hosts("equilibrium task acceptance")
                .map_err(|error| error.to_string())?;
        }
        // Ordinary points are validated by acceptance itself. Validate early
        // only when a control request would cause another finalization solve.
        #[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
        let validate = || -> Result<(), String> {
            #[cfg(feature = "veriloga")]
            self.veriloga_devices.validate_timestep_acceptance()?;
            #[cfg(feature = "veriloga-builtins-base")]
            self.generated_veriloga_devices
                .validate_state_acceptance()?;
            Ok(())
        };
        #[cfg(feature = "veriloga")]
        for device in self.veriloga_devices.iter() {
            if let Some(event) = device
                .first_candidate_analog_task(kind)
                .map_err(|error| error.to_string())?
            {
                validate()?;
                return Ok(Some(event));
            }
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if let Some(event) = self
            .generated_veriloga_devices
            .first_candidate_analog_task(kind)?
        {
            validate()?;
            return Ok(Some(event));
        }
        let _ = kind;
        Ok(None)
    }

    /// Deliver accepted analog calls exactly once, in runtime, generated, then
    /// mixed-signal instance order and each instance's source execution order.
    /// Rejected candidates are never delivered.
    ///
    /// Mixed-signal trials must be closed before delivery, because a later
    /// rollback must not restore output already delivered to the host. Every
    /// host is checked before the first call is consumed. The visitor is
    /// synchronous and receives borrowed names plus an owned argument snapshot.
    pub fn visit_accepted_analog_tasks(
        &mut self,
        consume: &mut dyn FnMut(AnalogTaskEvent<'_>),
    ) -> Result<(), String> {
        #[cfg(feature = "veriloga")]
        for host in &self.mixed_signal_hosts {
            host.validate_analog_task_delivery()
                .map_err(|error| error.to_string())?;
        }
        #[cfg(feature = "veriloga")]
        for device in self.veriloga_devices.iter_mut() {
            device.visit_accepted_analog_tasks(consume);
        }
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .visit_accepted_analog_tasks(consume);
        #[cfg(feature = "veriloga")]
        for host in &mut self.mixed_signal_hosts {
            host.visit_accepted_analog_tasks(consume)
                .map_err(|error| error.to_string())?;
        }
        let _ = consume;
        Ok(())
    }
}

#[cfg(all(test, feature = "veriloga"))]
mod tests {
    use super::*;
    use crate::xspice::event_scheduler::SchedulerLimits;
    use crate::xspice::verilog::MixedSignalHost;
    use rspice_veriloga::vm::IntegrationCoefficients;
    use rspice_veriloga::{VerilogACompiler, device::VerilogADevice};
    use rspice_veriloga_runtime::{AnalogTaskArgument, AnalogTaskKind};

    fn calls(circuit: &mut CircuitData) -> Vec<(String, String, u32, f64, i64)> {
        let mut calls = Vec::new();
        circuit
            .visit_accepted_analog_tasks(&mut |event| {
                assert_eq!(event.call.kind, AnalogTaskKind::Finish);
                let [AnalogTaskArgument::Integer(level)] = event.call.arguments.as_ref() else {
                    panic!("finish argument snapshot is missing");
                };
                calls.push((
                    event.instance.into(),
                    event.model.into(),
                    event.call.site,
                    event.call.time,
                    *level,
                ));
            })
            .unwrap();
        calls
    }

    #[test]
    fn delivery_preserves_identity_order_and_acceptance_across_runtime_and_mixed_hosts() {
        let source = r#"module task_source(p,n);
inout p,n; electrical p,n;
analog initial $finish(0);
analog begin
  if (V(p,n)>0) begin $finish(1); $finish(2); end
  I(p,n)<+V(p,n);
end
endmodule"#;
        let compiler = VerilogACompiler::default();
        let runtime = compiler.compile_runtime(source, None).unwrap();
        let device = VerilogADevice::try_new_with_canonical_ir(
            "xruntime",
            runtime.model,
            &runtime.canonical_ir,
            &[1, 0],
        )
        .unwrap();
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("p");
        circuit.add_veriloga_device(device);
        circuit.begin_veriloga_analysis(0).unwrap();

        let mixed_source = source
            .replace("module task_source", "module mixed_tasks")
            .replace(
                "analog initial",
                "reg started; initial started=1; analog initial",
            );
        let mut host = MixedSignalHost::compile(
            &mixed_source,
            None,
            "xmixed",
            &[1, 0],
            SchedulerLimits::default(),
        )
        .unwrap();
        let mut initial_calls = Vec::new();
        host.visit_accepted_analog_tasks(&mut |event| initial_calls.push(event.call))
            .unwrap();
        assert_eq!(initial_calls.len(), 1);
        assert_eq!(initial_calls[0].kind, AnalogTaskKind::Finish);
        // This transport-level test deliberately resumes after consuming the
        // initial call. The engine instead returns a normal finish outcome.
        host.start_digital_execution().unwrap();
        circuit.add_mixed_signal_host(host);
        circuit.mixed_signal_hosts[0]
            .begin_trial(0.0, 0.0, IntegrationCoefficients::inactive(), true, false)
            .unwrap();
        let mut delivered = false;
        assert!(
            circuit
                .visit_accepted_analog_tasks(&mut |_| delivered = true)
                .is_err()
        );
        assert!(
            !delivered,
            "one active mixed trial must prevent partial circuit delivery"
        );
        circuit.mixed_signal_hosts[0].reject_trial().unwrap();
        assert_eq!(
            calls(&mut circuit),
            vec![("xruntime".into(), "task_source".into(), 2, 0.0, 0)]
        );
        assert!(calls(&mut circuit).is_empty());

        circuit
            .veriloga_devices
            .get_mut(0)
            .unwrap()
            .try_stamp(&[1.0], |_, _, _| {}, |_, _| {})
            .unwrap();
        assert!(
            calls(&mut circuit).is_empty(),
            "Newton candidates are not accepted calls"
        );
        let host = &mut circuit.mixed_signal_hosts[0];
        host.begin_trial(
            1e-9,
            1e-9,
            IntegrationCoefficients::inactive(),
            false,
            false,
        )
        .unwrap();
        host.stamp(&[1.0], |_, _, _| {}, |_, _| {}).unwrap();
        assert!(!host.settle_analog_bridges(&[1.0]).unwrap());
        host.accept_trial().unwrap();
        assert_eq!(
            calls(&mut circuit),
            vec![
                ("xmixed".into(), "mixed_tasks".into(), 4, 1e-9, 1),
                ("xmixed".into(), "mixed_tasks".into(), 5, 1e-9, 2),
            ]
        );
        let device = circuit.veriloga_devices.get_mut(0).unwrap();
        device.try_stamp(&[0.0], |_, _, _| {}, |_, _| {}).unwrap();
        device.try_advance_state().unwrap();
        assert!(
            calls(&mut circuit).is_empty(),
            "the replacement candidate must discard earlier calls"
        );
        let host = &mut circuit.mixed_signal_hosts[0];
        host.begin_trial(
            2e-9,
            1e-9,
            IntegrationCoefficients::inactive(),
            false,
            false,
        )
        .unwrap();
        host.stamp(&[1.0], |_, _, _| {}, |_, _| {}).unwrap();
        host.reject_trial().unwrap();
        assert!(
            calls(&mut circuit).is_empty(),
            "a rejected trial must not publish calls"
        );
    }
}
