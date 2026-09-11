//! The real interpreter and scheduler, with an explicit analytic sample provider.
//! Circuit producer evaluation and numerical rollback are qualified separately.
use super::*;
use rspice_veriloga::VerilogACompiler;

fn host(source: &str) -> DigitalHost {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir_module(source, None)
        .unwrap();
    DigitalHost::new(
        &artifact.digital,
        TimeResolution::new(-9).unwrap(),
        SchedulerLimits::default(),
    )
}

fn signal(host: &DigitalHost, name: &str) -> DigitalSignalId {
    host.plan
        .signals
        .iter()
        .find(|s| s.name == name)
        .unwrap()
        .id
}

#[test]
fn analog_sample_barrier_preserves_active_inactive_nba_order_and_input_validity() {
    let mut host = host(
        r#"
`timescale 1ns/1ns
module sampled(p); inout p; electrical p;
real measured,offset,first,second,third,fourth,fifth,sixth;
reg [7:0] code,direct_changes; reg late,unused,saw_sign;
analog begin measured=offset+code; I(p)<+V(p)/1000; end
initial begin
  first=measured;
  offset=2; unused=1; second=measured;
  code=4; third=measured;
  #0; offset=3; fourth=measured;
  offset=0; fifth=measured;
  offset=$bitstoreal(64'h8000000000000000); sixth=measured;
end
initial begin offset=2; code=3; late=0; late<=1; direct_changes=0; saw_sign=0; end
always @(offset) direct_changes=direct_changes+1;
always @($realtobits(offset)) if ($realtobits(offset)==64'h8000000000000000) saw_sign=1;
endmodule
"#,
    );
    let offset = signal(&host, "offset");
    let code = signal(&host, "code");
    let late = signal(&host, "late");
    let probe = host.plan.analog_probes[0].id;
    host.bind_analog_variable_inputs(&[(probe, vec![offset, code])])
        .unwrap();
    host = host.fresh();

    struct Producer {
        offset: DigitalSignalId,
        code: DigitalSignalId,
        late: DigitalSignalId,
        values: Vec<f64>,
        negative_offsets: Vec<bool>,
    }
    impl DigitalActiveParticipant for Producer {
        fn settle_active(
            &mut self,
            _: &mut DigitalActiveExchange<'_>,
        ) -> Result<bool, DigitalRunError> {
            Ok(false)
        }
        fn sample_analog(
            &mut self,
            exchange: &mut DigitalActiveExchange<'_>,
        ) -> Result<(), DigitalRunError> {
            // Other initial blocks have run, but NBA has not passed an Active
            // reader or the subsequent #0 reader in the inactive region.
            assert_eq!(exchange.read_signal(self.late).unwrap().spelling(), "0");
            let offset = exchange.read_real_signal(self.offset).unwrap();
            self.negative_offsets.push(offset.is_sign_negative());
            let value = offset + exchange.read_signal(self.code).unwrap().to_u64().unwrap() as f64;
            self.values.push(value);
            let samples: Vec<_> = exchange
                .analog_sample_requests()
                .into_iter()
                .map(|probe| (probe, value))
                .collect();
            exchange.publish_analog_variables(&samples)
        }
    }
    let mut producer = Producer {
        offset,
        code,
        late,
        values: Vec::new(),
        negative_offsets: Vec::new(),
    };
    host.prepare_start().unwrap();
    host.advance_to_with(0, &mut producer).unwrap();
    assert_eq!(producer.values, [5.0, 6.0, 7.0, 4.0, 4.0]);
    assert_eq!(
        producer.negative_offsets,
        [false, false, false, false, true]
    );
    for (name, expected) in [
        ("first", 5.0),
        ("second", 5.0),
        ("third", 6.0),
        ("fourth", 7.0),
        ("fifth", 4.0),
        ("sixth", 4.0),
    ] {
        assert_eq!(host.read_real(signal(&host, name)), Some(expected));
    }
    assert_eq!(host.read(late).unwrap().spelling(), "1");
    assert_eq!(
        host.read(signal(&host, "direct_changes")).unwrap().to_u64(),
        Some(2)
    );
    assert_eq!(
        host.read(signal(&host, "saw_sign")).unwrap().spelling(),
        "1"
    );
    assert!(host.analog_waiters.is_empty());
}

#[test]
fn analog_sample_barrier_keeps_physical_activation_time_and_refuses_missing_provider() {
    let mut host = host(
        r#"
`timescale 1ns/1ns
module sampled(p); inout p; electrical p;
real measured,captured; reg trigger,clock_fired;
analog begin measured=V(p); I(p)<+V(p)/1000; end
initial begin trigger=0; clock_fired=0; #1 clock_fired=1; end
always @(posedge trigger) captured=measured;
endmodule
"#,
    );
    let trigger = signal(&host, "trigger");
    let timer = signal(&host, "clock_fired");
    let captured = signal(&host, "captured");
    let probe = host.plan.analog_probes[0].id;
    host.bind_analog_variable_inputs(&[(probe, vec![])])
        .unwrap();
    host.start().unwrap();
    let accepted = host.clone();
    let drives = [(trigger, FourStateValue::from_integer(1, 1))];
    // A circuit participant is required; the blocked process must never be
    // reported as a successful, settled result when nobody supplies its read.
    assert!(matches!(
        host.force_many_from_analog(&drives, 1, 0.75e-9),
        Err(DigitalRunError::ExternalExecution { .. })
    ));
    host = accepted;

    struct Producer {
        time: Option<f64>,
    }
    impl DigitalActiveParticipant for Producer {
        fn settle_active(
            &mut self,
            _: &mut DigitalActiveExchange<'_>,
        ) -> Result<bool, DigitalRunError> {
            Ok(false)
        }
        fn sample_analog(
            &mut self,
            exchange: &mut DigitalActiveExchange<'_>,
        ) -> Result<(), DigitalRunError> {
            self.time = Some(exchange.physical_seconds());
            let samples: Vec<_> = exchange
                .analog_sample_requests()
                .into_iter()
                .map(|probe| (probe, 2.5))
                .collect();
            exchange.publish_analog_variables(&samples)
        }
    }
    let mut producer = Producer { time: None };
    host.force_many_from_analog_with(&drives, 1, 0.75e-9, &mut producer)
        .unwrap();
    assert_eq!(producer.time, Some(0.75e-9));
    assert_eq!(host.read_real(captured), Some(2.5));
    assert_eq!(host.read(timer).unwrap().spelling(), "0");
    assert_eq!(host.next_tick(), Some(1));
    host.advance_to(1).unwrap();
    assert_eq!(host.read(timer).unwrap().spelling(), "1");
}
