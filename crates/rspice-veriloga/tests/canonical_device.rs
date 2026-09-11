//! The canonical backend produces a device, and the device compiles.
//!
//! A generator test that only inspects the emitted text proves the text looks
//! right, which is not the property anyone needs. What matters is that the two
//! files fit together and fit the runtime: that `stamp.rs` names fields
//! `state.rs` actually declares, that the borrows inside a stamp are disjoint,
//! and that every stamper call matches a real signature with the right arity.
//! Only `rustc` can answer those, so this hands them to `rustc`.
//!
//! The runtime is stubbed rather than linked, because linking `rspice-core`
//! would mean writing the device into its source tree. The stub carries the
//! exact signatures the emitted code calls and nothing else — if a call shape
//! drifts, this fails at the call site with the same message the real build
//! would give.

use rspice_veriloga::rust_backend::{
    RustBackendErrorKind, RustTranspileOptions, RustTranspiler, canonical,
};
use rspice_veriloga::{PipelineControl, PipelinePhase, VerilogACompiler};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[test]
fn generated_indirect_sources_preserve_constraint_precision_and_noise() {
    let (state, stamp, noise) = generated_parts(
        "module constraint(p,q); inout p,q; electrical p,q; parameter integer enabled=1;
         analog if(enabled) V(p): V(q)==1e-20*V(p)+white_noise(4,\"input\"); endmodule",
        "indirect constraint",
    );
    run_generated_main("indirect constraint", &state, &stamp, &noise, r#"
#[derive(Default)] struct Capture(Vec<runtime::GeneratedNoiseComplex>);
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self, _:usize, process:runtime::GeneratedNoiseProcessEvaluationRef<'_>)->bool {
        if process.active {
            assert_eq!(process.psd,4.0);
            assert_eq!(process.injections.len(),1);
            self.0.push(process.injections[0].gain);
        }
        true
    }
}
let mut instance=device::state::Instance::new(&[0,1]);
assert_eq!(device::state::Instance::BRANCH_COUNT,1);
instance.set_branch_indices(&[2]);
instance.multiplicity=4.0;
let bias=[0.0,2.0,0.0];
let ctx=runtime::GeneratedEvalContext {voltages:&bias,temperature:300.15};
for enabled in [1.0,0.0,1.0] {
    instance.set_parameter("enabled",enabled).unwrap();
    instance.finalize_parameters().unwrap();
    let mut sink=[0.0;10];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
    assert_eq!(sink[0],-4.0*enabled,"KCL-only source coupling");
    assert_eq!(sink[1],1.0-enabled,"inactive source has one identity row");
    assert_eq!(sink[2],-2.0*enabled,"constraint companion input");
    assert_eq!(sink[3],1e-20*enabled,"tiny self derivative must survive");
    let mut capture=Capture::default();
    instance.evaluate_noise_processes_at_frequency(&ctx,1.0,&mut capture).unwrap();
    assert_eq!(capture.0.len(),enabled as usize);
    if enabled!=0.0 {
        assert_eq!((capture.0[0].re,capture.0[0].im),(0.5,0.0));
    }
    assert!(!ctx.evaluation_failed());
}
"#).unwrap_or_else(|report|panic!("{report}"));
}

#[test]
fn generated_indirect_sources_preserve_dynamic_constraint_signs_and_history() {
    for (index, lhs, rhs, imaginary, real) in [
        (0, "ddt(V(p,n))", "2*I(a)", "-w", "0.0"),
        (
            1,
            "V(p,n)",
            "sin(ddt(I(a)))+ddt(ddt(V(p,n)))",
            "0.0",
            "-w*w",
        ),
        (2, "idt(V(p,n),0.0)", "I(a)", "1.0/w", "0.0"),
    ] {
        let source = format!(
            "module constraint(p,n); inout p,n; electrical p,n;
            branch(n,p) a; analog V(a): {lhs}=={rhs}; endmodule"
        );
        let name = format!("indirect frequency {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(&name, &state, &stamp, &noise, &format!(r#"
let mut instance=device::state::Instance::new(&[0,1]);
assert_eq!(device::state::Instance::BRANCH_COUNT,1);
instance.set_branch_indices(&[2]);
instance.finalize_parameters().unwrap();
runtime::set_dynamic_operators_enabled(false);
let ctx=runtime::GeneratedEvalContext {{voltages:&[0.5,0.0,0.25],temperature:300.15}};
instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
let history=instance.capture_rollback_state();
for w in [0.125_f64,1.0,8.0] {{
    runtime::FREQUENCY_OMEGA.store(w.to_bits(),std::sync::atomic::Ordering::SeqCst);
    let mut response=[0.0;6];
    instance.stamp_reactive(&ctx,&mut runtime::GeneratedReactiveStamper {{sink:Some(&mut response)}});
    assert_eq!(response[0],if {index}==0 {{1.0}} else {{{imaginary}}},"node imaginary input: {{response:?}}");
    assert_eq!(response[3],{real},"node real input: {{response:?}}");
    // A single linear ddt uses the cached charge row; its capture records
    // branch ordinal + 1 and unscaled charge derivative. Operator chains
    // use frequency coefficients instead.
    assert_eq!(response[1],if {index}==0 {{-1.0}} else if {index}==1 {{w}} else {{0.0}},"dynamic coefficient input: {{response:?}}");
    assert_eq!(instance.capture_rollback_state(),history);
    assert!(!ctx.evaluation_failed());
}}
"#)).unwrap_or_else(|report|panic!("{report}"));
    }

    let (state, stamp, noise) = generated_parts(
        "module constraint(p,q); inout p,q; electrical p,q;
         analog V(p): ddt(V(q))==2*I(p); endmodule",
        "indirect transient",
    );
    run_generated_main(
        "indirect transient",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.set_branch_indices(&[2]);
instance.finalize_parameters().unwrap();
instance.set_timepoint(0.0,0.0,runtime::GeneratedDdtCoefficients::inactive());
let initial=runtime::GeneratedEvalContext {voltages:&[0.0,2.0,0.0],temperature:300.15};
instance.stamp(&initial,&mut runtime::GeneratedStamper::default());
instance.set_timepoint(0.5,0.5,runtime::GeneratedDdtCoefficients {
    active:true,derivative_scale:2.0,previous_value_scale:2.0,
    older_value_scale:0.0,previous_derivative_scale:0.0,
});
let accepted=instance.capture_rollback_state();
for _ in 0..2 {
    instance.begin_stateful_evaluation();
    let ctx=runtime::GeneratedEvalContext {voltages:&[0.0,4.0,0.5],temperature:300.15};
    let mut sink=[0.0;10];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
    assert_eq!(sink[0],-1.0);
    assert_eq!(sink[2],-3.0,"negative constraint residual");
    assert_eq!(sink[3],-2.0,"negative ddt Jacobian");
    assert_eq!(sink[4],2.0,"positive feedback Jacobian input");
    assert_eq!(instance.capture_persistent_state().ddt_previous,vec![2.0]);
    instance.restore_rollback_state(&accepted);
    assert!(!ctx.evaluation_failed());
}
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn module_time_queries_execute_in_generated_hierarchy() {
    let (state, stamp, noise) = generated_parts_selected(
        include_str!("testdata/module_time_queries.va"),
        "module time queries",
        Some("top"),
    );
    let main = r#"
use runtime::{GeneratedSimulationParameters,SimulationParameter};
let mut environment=GeneratedSimulationParameters::default();
environment.try_set(SimulationParameter::TimeUnit,Some(17.0)).unwrap();
environment.try_set(SimulationParameter::TimePrecision,Some(23.0)).unwrap();
let mut instance=device::state::Instance::try_new_with_simulation_parameters(&[0],&[],&environment).unwrap();
let bias=[1.0];
let ctx=runtime::GeneratedEvalContext {voltages:&bias,temperature:300.15};
for time in [0.0,2e-9,7.25e-9] {
    instance.time=time;
    let mut sink=[0.0;32];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
    let expected=4.0+time/1e-9+time/1e-8;
    // The extended capture applies each branch's reference direction to KCL.
    for actual in [sink[12],sink[28]] {
        assert!((actual-expected).abs()<1e-12,"t={time:e}: {actual}, expected {expected}");
    }
    assert!(!ctx.evaluation_failed());
}
"#;
    run_generated_main("module time queries", &state, &stamp, &noise, main)
        .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_potential_sources_preserve_parallel_branch_identity() {
    for (source, active, sum) in [
        (
            "module parallel(p); inout p; thermal p,g; ground g; branch(p,g) a,b; analog begin Temp(a)<+2*Pwr(a); Temp(b)<+3*Pwr(b); end endmodule",
            2,
            8.0,
        ),
        (
            "module parallel(p); inout p; electrical p; branch(p) a,b; analog begin V(a)<+2*I(a); V(b)<+3*I(b); end endmodule",
            2,
            8.0,
        ),
        (
            "module parallel(p); inout p; electrical p; branch(p) a; analog begin V(a)<+2*I(a); V(a)<+3*I(a); end endmodule",
            1,
            5.0,
        ),
        (
            "module resistor(p); inout p; electrical p; parameter real r=2; analog V(p)<+r*I(p); endmodule module parallel(p); inout p; electrical p; resistor #(.r(2)) a(p); resistor #(.r(3)) b(p); endmodule",
            2,
            8.0,
        ),
    ] {
        let (state, stamp, noise) =
            generated_parts_selected(source, "parallel potential branches", Some("parallel"));
        let main = format!(
            r#"
let mut instance=device::state::Instance::new(&[0]);
assert_eq!(device::state::Instance::BRANCH_COUNT,{active});
instance.set_branch_indices(&(1..={active}).collect::<Vec<_>>());
instance.finalize_parameters().unwrap();
let bias=[0.0,1.0,2.0];
let ctx=runtime::GeneratedEvalContext {{ voltages:&bias, temperature:300.15 }};
let mut sink=[0.0;10];
instance.stamp(&ctx,&mut runtime::GeneratedStamper {{ sink:Some(&mut sink) }});
assert_eq!(sink[0],{active}.0,"each branch has its own structural coupling: {{sink:?}}");
assert_eq!(sink[1],0.0,"no unused solver rows: {{sink:?}}");
assert_eq!(sink[2],{sum:?},"each source reads its own current: {{sink:?}}");
assert_eq!(sink[4],5.0,"sum of source derivatives: {{sink:?}}");
assert_eq!(sink[9],{sum:?},"derivative columns retain branch identity: {{sink:?}}");
assert!(!ctx.evaluation_failed());
"#
        );
        run_generated_main("parallel potential branches", &state, &stamp, &noise, &main)
            .unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_parallel_potential_noise_preserves_branch_currents_and_destinations() {
    let (state, stamp, noise) = generated_parts(
        "module parallel(p); inout p; electrical p; branch(p) a,b; analog begin
         V(a)<+2*I(a)+white_noise(abs(I(a))+1,\"a\");
         V(b)<+3*I(b)+white_noise(abs(I(b))+2,\"b\"); end endmodule",
        "parallel potential noise",
    );
    run_generated_main(
        "parallel potential noise",
        &state,
        &stamp,
        &noise,
        r#"
#[derive(Default)] struct Capture(Vec<f64>);
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self, _index:usize, value:runtime::GeneratedNoiseEvaluationRef<'_>)->bool {
        assert!(value.active); self.0.push(value.psd); true
    }
}
assert_eq!(device::noise::NOISE_SOURCES[0].branch_ordinal,Some(0));
assert_eq!(device::noise::NOISE_SOURCES[1].branch_ordinal,Some(1));
let mut instance=device::state::Instance::new(&[0]);
instance.set_branch_indices(&[1,2]);
instance.finalize_parameters().unwrap();
let bias=[0.0,1.0,2.0];
let ctx=runtime::GeneratedEvalContext { voltages:&bias, temperature:300.15 };
let mut capture=Capture::default();
instance.evaluate_noise_sources(&ctx,&mut capture).unwrap();
assert_eq!(capture.0,vec![2.0,4.0]);
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_hierarchy_preserves_instance_port_currents() {
    let (state, stamp, noise) = generated_parts_selected(
        "module child(p,q); inout p,q; electrical p,q; parameter real gain=1;
         analog begin if(gain>0) I(p)<+gain*V(p); I(q)<+3*I(<p>); end endmodule
         module top(p,q); inout p,q; electrical p,q;
         child #(.gain(1)) a(p,q); child #(.gain(2)) b(p,q); endmodule",
        "hierarchical port currents",
        Some("top"),
    );
    run_generated_main(
        "hierarchical port currents",
        &state,
        &stamp,
        &noise,
        r#"
assert_eq!(device::state::Instance::INTERNAL_STATE_NODES.len(), 2);
let mut instance=device::state::Instance::new(&[0,1,2,3]);
instance.finalize_parameters().unwrap();
let bias=[1.0,0.0,0.0,0.0];
let ctx=runtime::GeneratedEvalContext { voltages:&bias, temperature:300.15 };
let mut sink=[0.0;32];
instance.stamp(&ctx,&mut runtime::GeneratedStamper { sink:Some(&mut sink) });
let mut matrix=[[0.0;4];4];
for row in 0..4 {
    for col in 0..4 { matrix[row][col]=sink[12+4*row+col]; }
    let rhs=(0..4).map(|col| matrix[row][col]*bias[col]).sum::<f64>()-sink[28+row];
    assert!(rhs.abs()<1e-12,"{row}: {rhs}");
}
for pivot in (2..4).rev() {
    assert!((matrix[pivot][pivot]-1.0).abs()<1e-12);
    for row in 0..pivot {
        for col in 0..pivot {
            matrix[row][col]-=matrix[row][pivot]*matrix[pivot][col]/matrix[pivot][pivot];
        }
    }
}
assert!((matrix[0][0]-3.0).abs()<1e-12,"{matrix:?}");
assert!((matrix[1][0]-9.0).abs()<1e-12,"{matrix:?}");
assert!(!ctx.evaluation_failed());
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_flow_probes_preserve_simultaneous_jacobians() {
    let (state, stamp, noise) = generated_parts(
        "module flow(p,q); inout p,q; electrical p,q; analog begin I(q)<+3*I(p); I(p)<+2*V(p)+0.1*I(p); end endmodule",
        "simultaneous flow probes",
    );
    run_generated_main(
        "simultaneous flow probes",
        &state,
        &stamp,
        &noise,
        r#"
assert_eq!(device::state::Instance::INTERNAL_STATE_NODES, &[0]);
let mut instance=device::state::Instance::new(&[0,1,2]);
instance.finalize_parameters().unwrap();
let bias=[1.0,0.0,-2.0/0.9];
let ctx=runtime::GeneratedEvalContext { voltages:&bias, temperature:300.15 };
let mut sink=[0.0;32];
instance.stamp(&ctx,&mut runtime::GeneratedStamper { sink:Some(&mut sink) });
let matrix=|row:usize,col:usize| sink[12+4*row+col];
let pivot=matrix(2,2);
assert!((pivot-0.9).abs()<1e-12);
for (row,expected) in [(0,2.0/0.9),(1,6.0/0.9)] {
    let reduced=matrix(row,0)-matrix(row,2)*matrix(2,0)/pivot;
    assert!((reduced-expected).abs()<1e-12,"{row}: {reduced}");
}

for row in 0..3 {
    let rhs=(0..3).map(|col| matrix(row,col)*bias[col]).sum::<f64>()-sink[28+row];
    assert!(rhs.abs()<1e-12,"{row}: {rhs}");
}
assert!(!ctx.evaluation_failed());
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_switch_sources_preserve_current_and_parameter_restaging() {
    let (state, stamp, noise) = generated_parts(
        "module switched(p,q); inout p,q; electrical p,q; parameter integer mode=0; analog begin I(q)<+I(p); if(mode==0) I(p)<+3*V(p); else V(p)<+2*I(p); if(mode==2) begin I(p)<+5*V(p); V(p)<+3*I(p); V(p)<+4*I(p); end end endmodule",
        "switched source retention",
    );
    run_generated_main(
        "switched source retention",
        &state,
        &stamp,
        &noise,
        r#"
assert_eq!(device::state::Instance::INTERNAL_STATE_NODES,&[0]);
assert_eq!(device::state::Instance::BRANCH_COUNT,0);
let mut instance=device::state::Instance::new(&[0,1,2]);
for (mode,gain) in [(0.0,3.0),(1.0,0.5),(2.0,1.0/7.0),(0.0,3.0)] {
    instance.set_parameter("mode",mode).unwrap();
    instance.finalize_parameters().unwrap();
    let bias=[1.0,0.0,-gain];
    let ctx=runtime::GeneratedEvalContext{voltages:&bias,temperature:300.15};
    let mut sink=[0.0;32];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper{sink:Some(&mut sink)});
    let matrix=|row:usize,col:usize|sink[12+4*row+col];
    let pivot=matrix(2,2);
    assert!(pivot.abs()>0.1,"mode={mode}: {sink:?}");
    for row in [0,1] {
        let reduced=matrix(row,0)-matrix(row,2)*matrix(2,0)/pivot;
        assert!((reduced-gain).abs()<1e-12,"mode={mode},row={row}: {sink:?}");
    }
    assert!(!ctx.evaluation_failed());
}
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_switch_discontinuities_follow_acceptance_and_rollback() {
    let (state, stamp, noise) = generated_parts(
        "module switched(p); inout p; electrical p; analog if(V(p)>0) V(p)<+2*I(p); else I(p)<+3*V(p); endmodule",
        "switch discontinuity lifecycle",
    );
    run_generated_main(
        "switch discontinuity lifecycle",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let evaluate=|instance:&mut device::state::Instance,voltage:f64| {
    let ctx=runtime::GeneratedEvalContext {voltages:&[voltage,0.0],temperature:300.0};
    instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
    assert!(!ctx.evaluation_failed());
};
evaluate(&mut instance,-1.0);
assert!(!instance.discontinuity_rising());
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let rollback=instance.capture_rollback_state();
for _ in 0..3 {
    evaluate(&mut instance,1.0);
    assert!(instance.discontinuity_rising());
}
instance.restore_rollback_state(&rollback);
evaluate(&mut instance,-1.0);
assert!(!instance.discontinuity_rising());
evaluate(&mut instance,1.0);
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let checkpoint=instance.capture_persistent_state();
assert!(!instance.discontinuity_rising());
evaluate(&mut instance,-1.0);
assert!(instance.discontinuity_rising());
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
instance.restore_persistent_state(&checkpoint).unwrap();
evaluate(&mut instance,1.0);
assert!(!instance.discontinuity_rising());
let mut invalid=checkpoint;
invalid.event_variables[0]=2.0;
assert!(instance.restore_persistent_state(&invalid).unwrap_err().contains("switch-branch"));
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_switch_sources_preserve_noise_and_discarded_rhs_validation() {
    let (state, stamp, noise) = generated_parts(
        "module switched(p); inout p; electrical p; real process;
        analog begin process=white_noise(1,\"shared\"); V(p)<+2*process; I(p)<+5*process; V(p)<+3*process; V(p)<+4*process; end endmodule",
        "switch noise retention",
    );
    run_generated_main("switch noise retention", &state, &stamp, &noise, r#"
struct Capture(usize);
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self,_:usize,process:runtime::GeneratedNoiseProcessEvaluationRef<'_>)->bool {
        assert!(process.active);
        assert_eq!(process.psd,1.0);
        assert_eq!(process.injections.len(),1);
        assert_eq!(process.injections[0].gain.re,-7.0);
        assert_eq!(process.injections[0].gain.im,0.0);
        self.0+=1;
        true
    }
}
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let ctx=runtime::GeneratedEvalContext{voltages:&[0.0,0.0],temperature:300.0};
let mut capture=Capture(0);
instance.evaluate_noise_processes_at_frequency(&ctx,1.0,&mut capture).unwrap();
assert_eq!(capture.0,1);
assert!(!ctx.evaluation_failed());
"#).unwrap_or_else(|report|panic!("{report}"));
    let (state, stamp, noise) = generated_parts(
        "module switched(p); inout p; electrical p; analog begin I(p)<+ln(V(p)); V(p)<+I(p); end endmodule",
        "switch discarded RHS validation",
    );
    run_generated_main(
        "switch discarded RHS validation",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let ctx=runtime::GeneratedEvalContext{voltages:&[-1.0,0.0],temperature:300.0};
instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
assert!(ctx.evaluation_failed());
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_limit_function_identifiers_preserve_state_and_unit_slope() {
    let source = r#"
module callback(p);
 inout p; electrical p;
 analog function real pnjlim;
  input real proposed,previous;
  input integer increment;
  pnjlim=min(proposed,previous+0.25*increment);
 endfunction
 analog I(p)<+$limit(V(p),pnjlim,1.6);
endmodule
"#;
    for typed in [false, true] {
        let source = if typed {
            source
                .replace("pnjlim", "clip")
                .replace("clip,1.6", "\"clip\",\"typed\",-1.0,1.6")
        } else {
            source.into()
        };
        let (state, stamp, noise) = generated_parts(&source, "limiter callback");
        let main = r#"
let mut instance=device::state::Instance::new(&[0]);
instance.finalize_parameters().unwrap();
for (proposed,expected) in [(0.0,0.0),(2.0,0.5),(2.0,1.0),(2.0,1.5),(2.0,2.0)] {
 let voltages=[proposed];
 let ctx=runtime::GeneratedEvalContext {voltages:&voltages,temperature:300.0};
 let mut sink=[0.0;12];
 instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
 assert_eq!(instance.canonical_limit.previous.as_slice(),[expected]);
 assert_eq!(instance.limiter_converged(),proposed==expected);
 assert_eq!(sink[10],1.0,"limiter callback slope must not replace the proposed-value Jacobian");
 assert!(!ctx.evaluation_failed());
}

"#;
        let main = if typed {
            main.replace("voltages=[proposed]", "voltages=[-proposed]")
                .replace("sink[10],1.0", "sink[10],-1.0")
        } else {
            main.into()
        };
        run_generated_main("limiter callback", &state, &stamp, &noise, &main)
            .unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_default_limit_preserves_state_unit_slope_and_affine_correction() {
    for (suffix, step) in [(",0.25", 0.25), ("", 0.7)] {
        let source = format!(
            "module bounded(p); inout p; electrical p;
            analog I(p)<+$limit(V(p){suffix}); endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, "default limiter");
        let main = format!(
            r#"
let mut instance=device::state::Instance::new(&[0]);
instance.finalize_parameters().unwrap();
for (proposed,expected) in [(0.0,0.0),(2.0,{step}),(2.0,2.0*{step})] {{
 let voltages=[proposed];
 let ctx=runtime::GeneratedEvalContext {{voltages:&voltages,temperature:300.0}};
 let mut sink=[0.0;12];
 instance.stamp(&ctx,&mut runtime::GeneratedStamper {{sink:Some(&mut sink)}});
 assert_eq!(instance.canonical_limit.previous.as_slice(),[expected]);
 assert_eq!(instance.limiter_converged(),proposed==expected);
 assert_eq!(sink[10],1.0,"limiting must preserve a unit Jacobian");
 assert_eq!(sink[9],proposed,"the corrected linear residual must equal its proposal");
 assert!(!ctx.evaluation_failed());
}}
"#
        );
        run_generated_main("default limiter", &state, &stamp, &noise, &main)
            .unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_default_limit_rejects_invalid_steps_and_preserves_extreme_proposals() {
    let source = "module bounded(p); inout p; electrical p; parameter real maxstep=0.25;
        analog I(p)<+$limit(V(p),maxstep); endmodule";
    let (state, stamp, noise) = generated_parts(source, "default limiter validation");
    run_generated_main(
        "default limiter validation",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0]);
instance.finalize_parameters().unwrap();
let voltage=[0.0];
let ctx=runtime::GeneratedEvalContext {voltages:&voltage,temperature:300.0};
instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
instance.set_parameter("maxstep",-1.0).unwrap();
instance.finalize_parameters().unwrap();
let voltage=[1.0];
let ctx=runtime::GeneratedEvalContext {voltages:&voltage,temperature:300.0};
instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
assert!(ctx.evaluation_failed());
assert_eq!(instance.canonical_limit.previous.as_slice(),[0.0]);
runtime::clear_evaluation_error();
instance.set_parameter("maxstep",0.25).unwrap();
instance.finalize_parameters().unwrap();
instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
assert!(!ctx.evaluation_failed());
assert_eq!(instance.canonical_limit.previous.as_slice(),[0.25]);
instance.set_parameter("maxstep",2.0_f64.powi(54)).unwrap();
instance.finalize_parameters().unwrap();
instance.canonical_limit.previous[0]=2.0_f64.powi(54);
instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
assert!(!ctx.evaluation_failed());
assert_eq!(instance.canonical_limit.previous.as_slice(),[1.0]);
assert!(instance.limiter_converged());
instance.set_parameter("maxstep",f64::MAX).unwrap();
instance.finalize_parameters().unwrap();
for proposed in [-f64::MAX,f64::MAX] {
 instance.canonical_limit.previous[0]=-proposed;
 let voltage=[proposed];
 let ctx=runtime::GeneratedEvalContext {voltages:&voltage,temperature:300.0};
 instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
 assert!(!ctx.evaluation_failed());
 assert_eq!(instance.canonical_limit.previous.as_slice(),[0.0]);
 assert!(!instance.limiter_converged());
}
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_quotient_range_preserves_higher_derivatives() {
    for (expression, bias, expected) in [
        ("ddx(1e308/(1e308*V(p)),V(p))", 0.01, [-1e4, 2e6]),
        ("ddx(ddx(1e308/(1e308*V(p)),V(p)),V(p))", 0.01, [2e6, -6e8]),
        ("ddx(1.6e308*V(p)/(3*V(p)-1),V(p))", 1.0, [-4e307, 1.2e308]),
        ("ddx(V(p)/V(p),V(p))", 1e-309, [0.0, 0.0]),
        ("ddx((2*V(p))/(3*V(p)),V(p))", 1e-309, [0.0, 0.0]),
        ("ddx((5*V(p))/(7*V(p)),V(p))", -1e-309, [0.0, 0.0]),
        ("ddx(ddx((2*V(p))/(3*V(p)),V(p)),V(p))", 1e-100, [0.0, 0.0]),
        ("ddx(1e308/(1e200+1e-200*V(p)),V(p))", 0.0, [-1e-292, 0.0]),
    ] {
        let source = format!(
            "module quotient(p); inout p; electrical p; analog I(p)<+{expression}; endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, expression);
        let main = format!(
            r#"
let mut instance=device::state::Instance::new(&[0]);
instance.finalize_parameters().unwrap();
let bias=[{bias:e}];
let ctx=runtime::GeneratedEvalContext {{voltages:&bias,temperature:300.0}};
let mut sink=[0.0;12];
instance.stamp(&ctx,&mut runtime::GeneratedStamper {{sink:Some(&mut sink)}});
for (actual,expected) in [sink[9],sink[10]].into_iter().zip([{:e},{:e}]) {{
    if expected==0.0 {{assert_eq!(actual,expected);}}
    else {{assert!((actual/expected-1.0).abs()<1e-12,"{{actual:e}} != {{expected:e}}");}}
}}
assert!(!ctx.evaluation_failed());
"#,
            expected[0], expected[1]
        );
        run_generated_main("quotient range", &state, &stamp, &noise, &main)
            .unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_packed_quotients_preserve_independent_tangents() {
    for (expression, bias, expected) in [
        (
            "1e308/(1e308*(V(p)+0.5*V(q)))",
            [0.005, 0.01],
            [100.0, -1e4, -5e3],
        ),
        (
            "ddx(1e308/(1e308*(V(p)+0.5*V(q))),V(p))",
            [0.005, 0.01],
            [-1e4, 2e6, 1e6],
        ),
        (
            "1.6e308*V(p)/(3*V(p)+V(q)-1)",
            [1.0, 0.0],
            [8e307, -4e307, -4e307],
        ),
    ] {
        let source = format!(
            "module packed_quotient(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, expression);
        let main = format!(
            r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let bias=[{:e},{:e}];
let ctx=runtime::GeneratedEvalContext {{voltages:&bias,temperature:300.0}};
let mut sink=[0.0;12];
instance.stamp(&ctx,&mut runtime::GeneratedStamper {{sink:Some(&mut sink)}});
for (actual,expected) in sink[9..12].iter().copied().zip([{:e},{:e},{:e}]) {{
    assert!((actual/expected-1.0).abs()<1e-12,"{{actual:e}} != {{expected:e}}");
}}
assert!(!ctx.evaluation_failed());
"#,
            bias[0], bias[1], expected[0], expected[1], expected[2]
        );
        run_generated_main("packed quotient", &state, &stamp, &noise, &main)
            .unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_simparam_noise_apis_preserve_values_and_required_query_errors() {
    let source = r#"module query_noise(p); inout p; electrical p;
analog I(p)<+white_noise($simparam("pnjmaxi",1/V(p))+$simparam("tnom",1000)+$simparam("unavailable",25),"query");
endmodule"#;
    let (state, stamp, noise) = generated_parts(source, "query noise values");
    let capture = r#"
struct Capture(f64);
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self,_:usize,value:runtime::GeneratedNoiseEvaluationRef<'_>)->bool { self.0=value.psd; true }
}
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self,_:usize,value:runtime::GeneratedNoiseProcessEvaluationRef<'_>)->bool { self.0=value.psd; true }
}
"#;
    let values = format!(
        r#"{capture}
let instance=device::state::Instance::new(&[0]);
for value in [None,Some(0.0),Some(3.0)] {{
    runtime::set_simparam_override(value);
    for voltage in [0.0,2.0] {{
        let ctx=runtime::GeneratedEvalContext {{voltages:&[voltage],temperature:300.15}};
        let expected=52.0+value.unwrap_or(1.0/voltage);
        for grouped in [false,true] {{
            runtime::clear_evaluation_error();
            let mut actual=Capture(-1.0);
            let result=if grouped {{instance.evaluate_noise_processes_at_frequency(&ctx,1.0,&mut actual)}}
                else {{instance.evaluate_noise_sources(&ctx,&mut actual)}};
            assert_eq!(result.is_ok(),expected.is_finite(),"grouped={{grouped}}, value={{value:?}}, voltage={{voltage}}");
            if expected.is_finite() {{ assert_eq!(actual.0,expected); assert!(!ctx.evaluation_failed()); }}
        }}
    }}
}}
"#
    );
    run_generated_main("query noise values", &state, &stamp, &noise, &values).unwrap();
    let source = r#"module query_guard(p); inout p; electrical p;
analog if ($simparam("imax")>0) I(p)<+white_noise(1,"guard"); endmodule"#;
    let (state, stamp, noise) = generated_parts(source, "required noise query");
    let required = format!(
        r#"{capture}
let instance=device::state::Instance::new(&[0]);
let ctx=runtime::GeneratedEvalContext {{voltages:&[0.0],temperature:300.15}};
for grouped in [false,true] {{
    runtime::clear_evaluation_error();
    let mut actual=Capture(-1.0);
    let result=if grouped {{instance.evaluate_noise_processes_at_frequency(&ctx,1.0,&mut actual)}}
        else {{instance.evaluate_noise_sources(&ctx,&mut actual)}};
    assert!(result.is_err(),"missing query hidden in noise predicate: grouped={{grouped}}");
    assert_eq!(actual.0,-1.0,"an invalid evaluation must not publish noise");
}}
"#
    );
    run_generated_main("required noise query", &state, &stamp, &noise, &required).unwrap();
}

#[test]
fn generated_simparam_defaults_capture_environment_and_validate_ranges() {
    let source = r#"module queries(p); inout p; electrical p;
parameter real supplied=0.0;
parameter real x=$simparam("imax",1.0/supplied);
parameter real y=2*x from [0:x*3];
analog I(p)<+x+y;
endmodule"#;
    let (state, stamp, noise) = generated_parts(source, "parameter query capture");
    let main = r#"
use runtime::{GeneratedSimulationParameters,SimulationParameter,GeneratedParameterAssignment as Assignment};
let mut environment=GeneratedSimulationParameters::default();
assert!(device::state::Instance::try_new_with_simulation_parameters(&[0],&[],&environment).is_err());
environment.try_set(SimulationParameter::Imax,Some(2.0)).unwrap();
let mut instance=device::state::Instance::try_new_with_simulation_parameters(&[0],&[],&environment).unwrap();
let bias=[0.0];
let ctx=runtime::GeneratedEvalContext {voltages:&bias,temperature:300.15};
let mut sink=[0.0;12];
instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
assert_eq!(sink[9],6.0);
environment.try_set(SimulationParameter::Imax,Some(9.0)).unwrap();
instance.finalize_parameters().unwrap();
assert_eq!(instance.params.values[1],2.0);
let before=instance.params.values;
assert!(instance.apply_parameters(&[Assignment::for_declared_scope("y",7.0)]).is_err());
assert_eq!(instance.params.values,before);
instance.apply_parameters(&[Assignment::for_declared_scope("y",5.0)]).unwrap();
let mut cloned=instance.clone();
cloned.finalize_parameters().unwrap();
assert_eq!(cloned.params.values,[0.0,2.0,5.0]);
let next=device::state::Instance::try_new_with_simulation_parameters(&[0],&[],&environment).unwrap();
assert_eq!(next.params.values,[0.0,9.0,18.0]);
"#;
    run_generated_main("parameter query capture", &state, &stamp, &noise, main)
        .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_simparam_required_defaults_respect_explicit_parameter_overrides() {
    let source = "module required(p); inout p; electrical p; parameter real x=$simparam(\"imax\"); analog I(p)<+x; endmodule";
    let (state, stamp, noise) = generated_parts(source, "required parameter query");
    let main = r#"
use runtime::{GeneratedSimulationParameters,SimulationParameter,GeneratedParameterAssignment as Assignment};
let mut environment=GeneratedSimulationParameters::default();
let error=device::state::Instance::try_new_with_simulation_parameters(&[0],&[],&environment).err().unwrap();
assert!(error.contains("imax"));
let mut given=device::state::Instance::try_new_with_simulation_parameters(&[0],&[Assignment::for_declared_scope("x",4.0)],&environment).unwrap();
given.finalize_parameters().unwrap();
assert_eq!(given.params.values,[4.0]);
environment.try_set(SimulationParameter::Imax,Some(0.0)).unwrap();
let zero=device::state::Instance::try_new_with_simulation_parameters(&[0],&[],&environment).unwrap();
assert_eq!(zero.params.values,[0.0]);
"#;
    run_generated_main("required parameter query", &state, &stamp, &noise, main)
        .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_simparams_preserve_selected_values_derivatives_and_missing_errors() {
    for order in 0..=2 {
        let mut expression = "$simparam(\"pnjmaxi\",V(p)*V(p)*V(p))".to_owned();
        for _ in 0..order {
            expression = format!("ddx({expression},V(p))");
        }
        let source = format!(
            "module queries(p); inout p; electrical p; analog I(p)<+{expression}; endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, "simulation parameter fallback");
        let main = r#"
let mut instance=device::state::Instance::new(&[0]);
instance.finalize_parameters().unwrap();
for configured in [f64::NAN,0.0,9.0,f64::NAN] {
    runtime::set_simparam_override((!configured.is_nan()).then_some(configured));
    for v in [0.25_f64,2.0,-3.0] {
        runtime::clear_evaluation_error();
        let bias=[v];
        let ctx=runtime::GeneratedEvalContext {voltages:&bias,temperature:300.15};
        let (expected,slope)=if configured.is_nan() {
            match ORDER {0=>(v.powi(3),3.0*v*v),1=>(3.0*v*v,6.0*v),_=>(6.0*v,6.0)}
        } else { (if ORDER==0 {configured} else {0.0},0.0) };
        let mut sink=[0.0;12];
        instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
        assert_eq!(sink[9],expected,"value at {v}");
        assert_eq!(sink[10],slope,"derivative at {v}");
        assert!(!ctx.evaluation_failed());
    }
}
"#
        .replace("ORDER", &order.to_string());
        run_generated_main(
            "simulation parameter fallback",
            &state,
            &stamp,
            &noise,
            &main,
        )
        .unwrap_or_else(|report| panic!("{report}"));
    }
    for (expression, expected, failed) in [
        ("$simparam(\"gmin\",1.0/V(p))", 1e-12, false),
        ("$simparam(\"tnom\")", 27.0, false),
        ("$simparam(\"imax\")", 0.0, true),
    ] {
        let source = format!(
            "module queries(p); inout p; electrical p; analog I(p)<+{expression}; endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, "required simulation parameter");
        let main = format!(
            r#"
let mut instance=device::state::Instance::new(&[0]);
instance.finalize_parameters().unwrap();
let bias=[0.0];
let ctx=runtime::GeneratedEvalContext {{voltages:&bias,temperature:300.15}};
runtime::clear_evaluation_error();
let mut sink=[0.0;12];
instance.stamp(&ctx,&mut runtime::GeneratedStamper {{sink:Some(&mut sink)}});
assert_eq!(ctx.evaluation_failed(),{failed});
if !{failed} {{ assert_eq!(sink[9],{expected:?}); }}
"#
        );
        run_generated_main(
            "required simulation parameter",
            &state,
            &stamp,
            &noise,
            &main,
        )
        .unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_homogeneous_math_preserves_values_and_derivatives_across_scales() {
    for op in ["hypot", "atan2"] {
        for derivative in 0..3 {
            let expression = format!("{op}(V(p),V(q))");
            let expression = match derivative {
                1 => format!("ddx({expression},V(p))"),
                2 => format!("ddx({expression},V(q))"),
                _ => expression,
            };
            let source = format!(
                "module planar(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
            );
            let (state, stamp, noise) = generated_parts(&source, &expression);
            let main=r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let scales=if DERIVATIVE==0 { [1e-200,1.0,1e200] } else { [1e-100,1.0,1e100] };
for scale in scales {
    for (a,b) in [(-1.0_f64,2.0_f64),(0.0,-2.0),(1.0,-1.0),(1.0,1.0)] {
        let (p,q)=(a*scale,b*scale);
        let expected=if HYPOT { let r=a.hypot(b); [p.hypot(q),a/r,b/r] }
                     else { let d=a*a+b*b; [p.atan2(q),(b/d)/scale,(-a/d)/scale] };
        let bias=[p,q];
        let ctx=runtime::GeneratedEvalContext {voltages:&bias,temperature:300.0};
        let mut sink=[0.0;12];
        instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
        let pairs=if DERIVATIVE==0 {vec![(sink[9],expected[0]),(sink[10],expected[1]),(sink[11],expected[2])]}
                  else {vec![(sink[9],expected[DERIVATIVE])]};
        for (actual,expected) in pairs {
            if expected==0.0 {assert_eq!(actual,expected);} else {assert!((actual/expected-1.0).abs()<1e-12,"{p},{q}: expected {expected}, got {actual}");}
        }
        assert!(!ctx.evaluation_failed());
    }
}
"#.replace("DERIVATIVE",&derivative.to_string()).replace("HYPOT",if op=="hypot" {"true"} else {"false"});
            run_generated_main(&expression, &state, &stamp, &noise, &main)
                .unwrap_or_else(|report| panic!("{report}"));
        }
    }
}

#[test]
fn generated_extrema_select_values_tangents_and_noise() {
    for (expression, p) in [
        ("max(V(p),sqrt(V(q)))", 1.0),
        ("max(sqrt(V(q)),V(p))", 1.0),
        ("min(V(p),sqrt(V(q)))", -1.0),
        ("min(sqrt(V(q)),V(p))", -1.0),
    ] {
        let source = format!(
            "module extrema(p,q); inout p,q; electrical p,q; parameter integer derivative=0; real a; analog begin a={expression}; if (derivative==1) a=ddx(a,V(p)); else if (derivative==2) a=ddx(a,V(q)); else if (derivative==3) a=ddx(ddx(a,V(q)),V(q)); I(p)<+a+white_noise(4.0+a,\"selected\"); end endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, "extrema device");
        let main = r#"
struct Capture(f64);
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self,_index:usize,value:runtime::GeneratedNoiseEvaluationRef<'_>)->bool { self.0=value.psd; true }
}
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for derivative in 0..4 {
    instance.set_parameter("derivative",f64::from(derivative)).unwrap();
    let expected=match derivative {0=>P,1=>1.0,_=>0.0};
    for q in [-1.0,0.0] {
        runtime::clear_evaluation_error();
        let bias=[P,q];
        let ctx=runtime::GeneratedEvalContext {voltages:&bias,temperature:300.0};
        let mut sink=[0.0;12];
        instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
        assert_eq!(sink[9],expected,"value, derivative={derivative}, q={q}");
        assert_eq!(sink[10],if derivative==0 {1.0} else {0.0},"p tangent, derivative={derivative}, q={q}");
        assert_eq!(sink[11],0.0,"q tangent, derivative={derivative}, q={q}");
        let mut capture=Capture(-1.0);
        instance.evaluate_noise_sources(&ctx,&mut capture).unwrap();
        assert_eq!(capture.0,4.0+expected);
        assert!(!ctx.evaluation_failed());
    }
}
"#.replace("P", &format!("{p:.1}"));
        run_generated_main("extrema device", &state, &stamp, &noise, &main)
            .unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_mathematical_parameter_defaults_follow_overrides_atomically() {
    let expressions = [
        ("abs(-x)", "(-x).abs()"),
        ("fabs(-x)", "(-x).abs()"),
        ("sqrt(x)", "x.sqrt()"),
        ("exp(x)", "x.exp()"),
        ("ln(x)", "x.ln()"),
        ("log(x)", "x.ln()"),
        ("log10(x)", "x.log10()"),
        ("sin(x)", "x.sin()"),
        ("cos(x)", "x.cos()"),
        ("tan(x)", "x.tan()"),
        ("asin(x/4.0)", "(x/4.0).asin()"),
        ("acos(x/4.0)", "(x/4.0).acos()"),
        ("atan(x)", "x.atan()"),
        ("sinh(x)", "x.sinh()"),
        ("cosh(x)", "x.cosh()"),
        ("tanh(x)", "x.tanh()"),
        ("asinh(x)", "x.asinh()"),
        ("acosh(x)", "x.acosh()"),
        ("atanh(x/4.0)", "(x/4.0).atanh()"),
        ("floor(x/2.0)", "(x/2.0).floor()"),
        ("ceil(x/2.0)", "(x/2.0).ceil()"),
        ("pow(x,0.5)", "x.powf(0.5)"),
        ("x**1.5", "x.powf(1.5)"),
        ("hypot(x,2.0)", "x.hypot(2.0)"),
        ("atan2(x,-1.0)", "x.atan2(-1.0)"),
        ("min(x,2.0)", "x.min(2.0)"),
        ("max(x,2.0)", "x.max(2.0)"),
        ("min(x,sqrt(-1.0))", "x"),
        ("min(sqrt(-1.0),x)", "x"),
        ("max(x,sqrt(-1.0))", "x"),
        ("max(sqrt(-1.0),x)", "x"),
        (
            "x>1.0 ? ln(x-1.0) : 0.0",
            "if x>1.0 { (x-1.0).ln() } else {0.0}",
        ),
    ];
    let declarations = expressions
        .iter()
        .enumerate()
        .map(|(i, (expression, _))| format!("parameter real p{i}={expression};"))
        .collect::<String>();
    let source = format!(
        "module math_defaults(p); inout p; electrical p; parameter real x=2.0; {declarations} analog I(p)<+p0*V(p); endmodule"
    );
    let expected = expressions
        .iter()
        .map(|(_, expected)| *expected)
        .collect::<Vec<_>>()
        .join(",");
    let main = format!(
        r#"
let mut instance=device::state::Instance::new(&[0]);
instance.finalize_parameters().unwrap();
for x in [1.0_f64, 1.25, 2.0, 3.0] {{
    instance.set_parameter("x",x).unwrap();
    let expected=[{expected}];
    for (i,expected) in expected.into_iter().enumerate() {{
        let actual=instance.params.values[i+1];
        assert!((actual-expected).abs() <= 1e-14*expected.abs().max(1.0),"parameter {{i}}, x={{x}}, expected {{expected}}, got {{actual}}");
    }}
}}
let before=instance.params.values;
let given=instance.param_given.clone();
assert!(instance.set_parameter("x",0.0).is_err());
assert_eq!(instance.params.values,before);
assert_eq!(instance.param_given,given);
"#
    );
    let name = "mathematical defaults";
    let (state, stamp, noise) = generated_parts(&source, name);
    run_generated_main(name, &state, &stamp, &noise, &main)
        .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_signed_zero_factors_preserve_stamps_defaults_and_noise() {
    let source = r#"
module signed_zero(p,q);
inout p,q; electrical p,q;
parameter integer mode=0;
parameter real sign=-1, zero=0.0/sign;
parameter real minimum=min(zero,0.0), maximum=max(zero,0.0);
real z,a;
analog begin
    if (mode==0) z=0.0*V(q);
    else if (mode==1) z=0.0/V(q);
    else if (mode==2) z=0.0+V(q);
    else if (mode==3) z=V(q)-(-0.0);
    else if (mode==4) z=pow(0.0*V(q),0.5);
    else if (mode==5) z=pow(sqrt(0.0*V(q)),0.5);
    else if (mode==6) z=min(0.0*V(q),-0.0);
    else z=max(0.0*V(q),-0.0);
    a=atan2(z,-1.0);
    I(p)<+V(p)*a+white_noise(4.0+a,"branch_cut");
end
endmodule
"#;
    let name = "signed zero device";
    let (state, stamp, noise) = generated_parts(source, name);
    run_generated_main(name, &state, &stamp, &noise, r#"
struct Capture(f64);
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self,_index:usize,value:runtime::GeneratedNoiseEvaluationRef<'_>)->bool { self.0=value.psd; true }
}
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self,_index:usize,value:runtime::GeneratedNoiseProcessEvaluationRef<'_>)->bool { self.0=value.psd; true }
}
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for sign in [-1.0_f64,1.0] {
    instance.set_parameter("sign",sign).unwrap();
    assert_eq!(instance.params.values[2].to_bits(), (0.0/sign).to_bits());
    assert_eq!(instance.params.values[3].to_bits(), (0.0/sign).to_bits());
    assert_eq!(instance.params.values[4].to_bits(), (0.0/sign).to_bits());
}
for mode in 0..8 {
    instance.set_parameter("mode",f64::from(mode)).unwrap();
    for q in if mode==2 || mode==3 {[-0.0_f64,0.0]} else {[-2.0,2.0]} {
        let z:f64=match mode {0|6|7=>0.0*q,1=>0.0/q,2=>0.0+q,3=>q-(-0.0),4=>(0.0*q).powf(0.5),_=>(0.0*q).sqrt().powf(0.5)};
        let expected=z.atan2(-1.0);
        let bias=[1.0,q];
        let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
        let mut sink=[0.0;12];
        instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
        assert_eq!(sink[9],expected,"value mode={mode}, q={q:?}");
        assert_eq!(sink[10],expected,"Jacobian mode={mode}, q={q:?}");
        let mut source=Capture(-1.0);
        instance.evaluate_noise_sources(&ctx,&mut source).unwrap();
        let mut process=Capture(-1.0);
        instance.evaluate_noise_processes_at_frequency(&ctx,1.0,&mut process).unwrap();
        assert_eq!(source.0,4.0+expected,"source mode={mode}, q={q:?}");
        assert_eq!(process.0,4.0+expected,"process mode={mode}, q={q:?}");
        assert!(!ctx.evaluation_failed());
    }
}
"#).unwrap_or_else(|report|panic!("{report}"));
}

#[test]
fn generated_ddx_preserves_domain_errors_in_stamps_and_noise() {
    for (expression, valid_psd) in [
        ("ddx(V(p)%V(q),V(p))", "1.0"),
        ("ddx(ddx(V(p)%V(q),V(p)),V(p))", "2.0"),
        ("min(ddx(0.0/V(q),V(p))+V(p),V(p))", "1.0"),
        ("max(ddx(0.0/V(q),V(p))+V(p),V(p))", "1.0"),
    ] {
        let source = format!(
            "module derivative_domain(p,q); inout p,q; electrical p,q; real d; analog begin
             d=V(q)<0 ? 3 : {expression};
             I(p)<+(d>0 ? 1 : 0)+white_noise(d>0 ? 1 : 2,\"checked\"); end endmodule"
        );
        let name = "checked derivative device";
        let (state, stamp, noise) = generated_parts(&source, name);
        let main = r#"
struct Capture(f64);
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self,_index:usize,value:runtime::GeneratedNoiseEvaluationRef<'_>)->bool { self.0=value.psd; true }
}
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self,_index:usize,value:runtime::GeneratedNoiseProcessEvaluationRef<'_>)->bool { self.0=value.psd; true }
}
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for (q,fails) in [(-1.0,false),(0.0,true),(2.0,false)] {
    let bias=[5.0,q];
    let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
    runtime::clear_evaluation_error();
    instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
    assert_eq!(ctx.evaluation_failed(),fails,"stamp at {q}");
    runtime::clear_evaluation_error();
    let mut source=Capture(-1.0);
    assert_eq!(instance.evaluate_noise_sources(&ctx,&mut source).is_err(),fails,"noise at {q}");
    runtime::clear_evaluation_error();
    let mut process=Capture(-1.0);
    assert_eq!(instance.evaluate_noise_processes_at_frequency(&ctx,1.0,&mut process).is_err(),fails,"noise process at {q}");
    if !fails {
        let expected=if q<0.0 {1.0} else {VALID_PSD};
        assert_eq!(source.0,expected,"noise at {q}");
        assert_eq!(process.0,expected,"noise process at {q}");
    }
}
"#.replace("VALID_PSD", valid_psd);
        run_generated_main(name, &state, &stamp, &noise, &main)
            .unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_ddx_preprocessing_does_not_cache_failed_operands() {
    let work = "g=g+sin(g);".repeat(12);
    let source = format!(
        "module derivative_stage(p); inout p; electrical p; real g,d; analog begin
         g=$temperature; {work} d=ddx(g%$temperature,V(p)); I(p)<+(d>0 ? 1 : 2)*V(p); end endmodule"
    );
    let name = "checked derivative preprocessing";
    let (state, stamp, noise) = generated_parts(&source, name);
    assert!(
        stamp.contains("_preprocess("),
        "fixture must use cached preprocessing"
    );
    run_generated_main(
        name,
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0]);
instance.finalize_parameters().unwrap();
for (temperature,fails) in [(0.0,true),(300.0,false),(0.0,true)] {
    for _ in 0..2 {
        runtime::clear_evaluation_error();
        let ctx=runtime::GeneratedEvalContext { voltages:&[1.0],temperature };
        instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
        assert_eq!(ctx.evaluation_failed(),fails,"at {temperature}");
    }
}
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_integer_arithmetic_finalizes_defaults_before_evaluation() {
    let (state, stamp, noise) = generated_parts(
        "module integer_defaults(p,n); inout p,n; electrical p,n; parameter integer numerator=5, denominator=2; parameter real quotient=numerator/denominator; parameter real overflow=2147483647+1; localparam real reciprocal=2**-1; integer q; analog begin q=V(p,n); I(p,n)<+quotient+(q/denominator)+0.25*V(p,n)+reciprocal; end endmodule",
        "integer arithmetic defaults",
    );
    run_generated_main(
        "integer arithmetic defaults",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0,1]);
assert_eq!(instance.params.values, [5.0,2.0,2.0,-2147483648.0]);
instance.set_parameter("numerator", -5.0).unwrap();
assert_eq!(instance.params.values[2], -2.0);
let bias=[5.0,0.0];
let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
let mut sink=[0.0;12];
instance.stamp(&ctx,&mut runtime::GeneratedStamper { sink:Some(&mut sink) });
assert_eq!(sink[9],1.25);
assert_eq!(sink[10],0.25);
let valid=instance.params.values;
assert!(instance.set_parameter("denominator",0.0).is_err());
assert_eq!(instance.params.values, valid);
assert!(!ctx.evaluation_failed());
"#,
    )
    .unwrap();
}

#[test]
fn generated_integer_assignments_and_bitwise_operations_execute_shared_semantics() {
    let source = r#"
module integer_generated(p,n);
inout p,n; electrical p,n;
parameter integer mask=3;
parameter integer shifted=mask<<1;
integer q, a[0:1];
analog function integer round_input;
    input x; real x;
    round_input=x;
endfunction
analog begin
    q=V(p,n); a[0]=V(p,n); a[1]=-V(p,n);
    I(p,n)<+q+a[0]-a[1]+round_input(V(p,n))+0.25*V(p,n)+shifted+(q&3)+(q^2)+(q|4)+~q+(q<<1)+(q>>1);
end
endmodule
"#;
    let name = "generated checked integers";
    let (state, stamp, noise) = generated_parts(source, name);
    run_generated_main(name,&state,&stamp,&noise,r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for v in [-2.5_f64,-1.5,-0.5,0.49,0.5,1.25,1.5,2.5] {
    let q=v.round() as i32;
    let expected=4.0*f64::from(q)+0.25*v+6.0+f64::from((q&3)+(q^2)+(q|4)+!q+(q<<1))+f64::from(((q as u32)>>1) as i32);
    let bias=[v,0.0];
    let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
    let mut real=[0.0;12];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper { sink:Some(&mut real) });
    assert_eq!(real[9],expected,"at {v}: {real:?}");
    assert_eq!(real[10],0.25);
    assert!(!ctx.evaluation_failed());
}
"#).unwrap_or_else(|report|panic!("{report}"));
}

#[test]
fn generated_integer_noise_conditions_cannot_hide_conversion_errors() {
    let source = r#"module bad_integer_noise(p,n); inout p,n; electrical p,n; integer q; analog begin q=V(p,n); I(p,n)<+white_noise(q ? 1.0 : 2.0,"integer"); end endmodule"#;
    let name = "checked integer noise condition";
    let (state, stamp, noise) = generated_parts(source, name);
    run_generated_main(name,&state,&stamp,&noise,r#"
struct Capture;
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self,_index:usize,_value:runtime::GeneratedNoiseEvaluationRef<'_>)->bool { true }
}
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self,_index:usize,_value:runtime::GeneratedNoiseProcessEvaluationRef<'_>)->bool { true }
}
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for (v,fails) in [(1.5,false),(2147483647.5,true)] {
    let bias=[v,0.0];
    let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
    runtime::clear_evaluation_error();
    assert_eq!(instance.evaluate_noise_sources(&ctx,&mut Capture).is_err(),fails);
    runtime::clear_evaluation_error();
    assert_eq!(instance.evaluate_noise_processes_at_frequency(&ctx,1.0,&mut Capture).is_err(),fails);
}
"#).unwrap_or_else(|report|panic!("{report}"));
}

#[test]
fn generated_integer_preprocessing_does_not_cache_failed_conversions() {
    for (name, declaration, rhs, configure) in [
        (
            "integer_model_stage",
            "parameter real bias=2.5;",
            "bias",
            "instance.set_parameter(\"bias\",input).unwrap(); instance.finalize_parameters().unwrap();",
        ),
        ("integer_temperature_stage", "", "$temperature", ""),
    ] {
        let work = "g=g+sin(g);".repeat(12);
        let source = format!(
            "module {name}(p,n); inout p,n; electrical p,n; {declaration} integer q; real g; analog begin g={rhs}; {work} q=g; I(p,n)<+(q ? g : 2.0)*V(p,n); end endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, name);
        assert!(
            stamp.contains("_preprocess("),
            "fixture must execute cached preprocessing"
        );
        run_generated_main(
            name,
            &state,
            &stamp,
            &noise,
            &format!(
                r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for input in [3000000000.0,0.49] {{
    {configure}
    for _ in 0..2 {{
        runtime::clear_evaluation_error();
        let ctx=runtime::GeneratedEvalContext {{ voltages:&[1.0,0.0],temperature:input }};
        instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
        assert_eq!(ctx.evaluation_failed(),input>1e9,"input {{input}}");
    }}
}}
"#
            ),
        )
        .unwrap_or_else(|report| panic!("{name}: {report}"));
    }
}

#[test]
fn generated_integer_initializers_import_the_shared_conversion() {
    let name = "generated integer initializer";
    let (state, stamp, noise) = generated_parts(
        "module init_integer(p,n); inout p,n; electrical p,n; integer q=1.5; analog I(p,n)<+q+0.25*V(p,n); endmodule",
        name,
    );
    run_generated_main(
        name,
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let ctx=runtime::GeneratedEvalContext { voltages:&[1.0,0.0],temperature:300.0 };
let mut values=[0.0;12];
instance.stamp(&ctx,&mut runtime::GeneratedStamper { sink:Some(&mut values) });
assert_eq!(values[9],2.25);
assert_eq!(values[10],0.25);
assert!(!ctx.evaluation_failed());
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_integer_noise_metadata_uses_finalized_parameters_and_rounded_assignments() {
    let source = r#"module integer_noise(p,n); inout p,n; electrical p,n; parameter integer shift=1; parameter integer mask=1<<shift; integer q; analog begin q=V(p,n); I(p,n)<+white_noise((q&3)+mask,"integer"); end endmodule"#;
    let name = "generated integer noise";
    let (state, stamp, noise) = generated_parts(source, name);
    run_generated_main(name,&state,&stamp,&noise,r#"
struct Capture(f64);
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self,_index:usize,value:runtime::GeneratedNoiseEvaluationRef<'_>)->bool { self.0=value.psd; true }
}
let mut instance=device::state::Instance::new(&[0,1]);
for (shift,mask) in [(1.0,2.0),(32.0,0.0)] {
    instance.set_parameter("shift",shift).unwrap();
    instance.finalize_parameters().unwrap();
    for v in [-1.5_f64,0.5,2.5] {
        let bias=[v,0.0];
        let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
        let mut capture=Capture(f64::NAN);
        instance.evaluate_noise_sources(&ctx,&mut capture).unwrap();
        assert_eq!(capture.0,f64::from((v.round() as i32)&3)+mask);
        assert!(!ctx.evaluation_failed());
    }
}
"#).unwrap_or_else(|report|panic!("{report}"));
}

#[test]
fn generated_integer_failures_survive_boolean_control_flow() {
    let source = "module checked_integer(p,n); inout p,n; electrical p,n; integer q; analog begin q=0; if(V(p,n)>0.0) q=V(p,n); if(q) I(p,n)<+1.0; else I(p,n)<+2.0; end endmodule";
    let name = "generated invalid integer condition";
    let (state, stamp, noise) = generated_parts(source, name);
    run_generated_main(
        name,
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for (v,fails) in [(-2147483649.0,false),(1.5,false),(2147483647.5,true)] {
    let bias=[v,0.0];
    let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
    runtime::clear_evaluation_error();
    instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
    assert_eq!(ctx.evaluation_failed(),fails,"at {v}");
}
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_real_modulo_does_not_erase_invalid_noise_metadata() {
    let source = "module invalid_remainder(p,n); inout p,n; electrical p,n; analog I(p,n)<+white_noise(0.0%V(p,n),\"source\"); endmodule";
    let name = "zero numerator remainder noise metadata";
    let (state, stamp, noise) = generated_parts(source, name);
    run_generated_main(
        name,
        &state,
        &stamp,
        &noise,
        r#"
struct Capture;
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self,_index:usize,_value:runtime::GeneratedNoiseEvaluationRef<'_>)->bool { true }
}
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for v in [1.0,0.0] {
    let bias=[v,0.0];
    let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
    let result=instance.evaluate_noise_sources(&ctx,&mut Capture);
    assert_eq!(result.is_err(),v==0.0,"0 % {v}: {result:?}");
}
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_real_modulo_preserves_reactive_jacobians() {
    let source = "module reactive_remainder(p,n); inout p,n; electrical p,n; analog I(p,n)<+ddt(V(p,n)%(2.0+V(p,n))); endmodule";
    let name = "generated reactive real modulo";
    let (state, stamp, noise) = generated_parts(source, name);
    run_generated_main(name,&state,&stamp,&noise,r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
runtime::set_dynamic_operators_enabled(false);
for v in [-2.75_f64,-0.75,0.5,1.25] {
    let bias=[v,0.0];
    let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
    instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
    let mut reactive=[0.0;6];
    instance.stamp_reactive(&ctx,&mut runtime::GeneratedReactiveStamper { sink:Some(&mut reactive) });
    // This first-order interface reports capacitance; the solver applies j*w.
    assert_eq!(reactive[0],1.0-(v/(2.0+v)).trunc());
    assert!(!ctx.evaluation_failed());
}
"#).unwrap_or_else(|report|panic!("{report}"));
}

#[test]
fn generated_real_modulo_preserves_values_and_derivatives() {
    for (index, (expression, expected_value, expected_slope)) in [
        (
            "(10.0+V(p,n)*V(p,n)*V(p,n))%(2.0+V(p,n)*V(p,n)*V(p,n))",
            "a%b",
            "(1.0-q)*3.0*v*v",
        ),
        (
            "ddx((10.0+V(p,n)*V(p,n)*V(p,n))%(2.0+V(p,n)*V(p,n)*V(p,n)),V(p,n))",
            "(1.0-q)*3.0*v*v",
            "(1.0-q)*6.0*v",
        ),
        (
            "ddx(ddx((10.0+V(p,n)*V(p,n)*V(p,n))%(2.0+V(p,n)*V(p,n)*V(p,n)),V(p,n)),V(p,n))",
            "(1.0-q)*6.0*v",
            "(1.0-q)*6.0",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let name = format!("generated real modulo {index}");
        let source = format!(
            "module remainder(p,n); inout p,n; electrical p,n; analog I(p,n)<+{expression}; endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(
            &name,
            &state,
            &stamp,
            &noise,
            &format!(
                r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for v in [-3.0_f64,-0.75,0.5,1.25] {{
    let bias=[v,0.0];
    let a=10.0+v*v*v;
    let b=2.0+v*v*v;
    let q=(a/b).trunc();
    let ctx=runtime::GeneratedEvalContext {{ voltages:&bias,temperature:300.0 }};
    let mut real=[0.0;12];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper {{ sink:Some(&mut real) }});
    assert!((real[9]-({expected_value})).abs()<1e-10,"value: {{real:?}}");
    assert!((real[10]-({expected_slope})).abs()<1e-10,"Jacobian: {{real:?}}");
    assert!(!ctx.evaluation_failed());
}}
"#
            ),
        )
        .unwrap_or_else(|report| panic!("{expression}: {report}"));
    }
}

#[test]
fn generated_nested_ddx_mathematical_jacobians_include_both_operands() {
    for (op, second, third) in [
        ("hypot", "9.0/d.powf(1.5)", "-27.0*(2.0*v+3.0)/d.powf(2.5)"),
        (
            "atan2",
            "-3.0*dp/d.powi(2)",
            "6.0*dp*dp/d.powi(3)-12.0/d.powi(2)",
        ),
    ] {
        let source = format!(
            "module nested_math(p,n); inout p,n; electrical p,n; analog I(p,n)<+ddx(ddx({op}(V(p,n),V(p,n)+3.0),V(p,n)),V(p,n)); endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, op);
        let main = format!(
            r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for v in [0.25_f64,0.75,1.25,3.0,5.0] {{
    let d=2.0*v*v+6.0*v+9.0;
    let dp=4.0*v+6.0;
    let bias=[v,0.0];
    let ctx=runtime::GeneratedEvalContext {{voltages:&bias,temperature:300.0}};
    let mut sink=[0.0;12];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper {{sink:Some(&mut sink)}});
    for (actual,expected) in [(sink[9],{second}),(sink[10],{third})] {{
        assert!((actual/expected-1.0).abs()<1e-11,"V={{v}}: expected {{expected}}, got {{actual}}");
    }}
    assert!(!ctx.evaluation_failed());
}}
"#
        );
        run_generated_main(op, &state, &stamp, &noise, &main)
            .unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_nested_ddx_stamps_higher_order_jacobians() {
    for (index, body) in [
        "analog I(p,n)<+ddx(ddx(V(p,n)*V(p,n)*V(p,n),V(p,n)),V(p,n));",
        "real x,y; analog begin x=V(p,n)*V(p,n)*V(p,n); y=ddx(x,V(p,n)); I(p,n)<+ddx(y,V(p,n)); end",
        "parameter integer count=3; real x; integer k; analog begin x=0; for(k=0;k<count;k=k+1) x=x+V(p,n)*V(p,n)*V(p,n); I(p,n)<+ddx(ddx(x,V(p,n)),V(p,n))/3; end",
    ].into_iter().enumerate() {
        let source=format!("module nested(p,n); inout p,n; electrical p,n; {body} endmodule");
        let name=format!("generated nested ddx {index}");
        let (state,stamp,noise)=generated_parts(&source,&name);
        run_generated_main(&name,&state,&stamp,&noise,r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for v in [-0.8,0.0,1.3] {
    let bias=[v,0.0];
    let ctx=runtime::GeneratedEvalContext { voltages:&bias,temperature:300.0 };
    let mut real=[0.0;12];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper { sink:Some(&mut real) });
    assert!((real[10]-6.0).abs()<1e-10,"Jacobian: {real:?}");
    assert!(!ctx.evaluation_failed());
}
"#).unwrap_or_else(|report|panic!("{body}: {report}"));
    }
}

#[test]
fn generated_dynamic_expressions_preserve_small_signal_chain_rules() {
    for (index, (expression, static_real, dynamic_real, imaginary)) in [
        (
            "max(V(p,n)+ddt(V(p,n)),0.0)",
            "if v>0.0 {1.0} else {0.0}",
            "0.0",
            "if v>0.0 {w} else {0.0}",
        ),
        (
            "min(V(p,n)+idt(V(p,n),0.0),0.0)",
            "if v<0.0 {1.0} else {0.0}",
            "0.0",
            "if v<0.0 {-1.0/w} else {0.0}",
        ),
        (
            "ddx(max(V(p,n)*V(p,n)*V(p,n)+ddt(V(p,n)*V(p,n)*V(p,n)),1.0),V(p,n))",
            "if v>1.0 {6.0*v} else {0.0}",
            "0.0",
            "if v>1.0 {6.0*v*w} else {0.0}",
        ),
        (
            "ddx(ddx(ddt(V(p,n)*V(p,n)*V(p,n)),V(p,n)),V(p,n))",
            "0.0",
            "0.0",
            "6.0*w",
        ),
        (
            "hypot(V(p,n)+ddt(V(p,n)),2.0)",
            "v/v.hypot(2.0)",
            "0.0",
            "v/v.hypot(2.0)*w",
        ),
        (
            "atan2(V(p,n)+idt(V(p,n),0.0),2.0)",
            "2.0/(v*v+4.0)",
            "0.0",
            "-2.0/(v*v+4.0)/w",
        ),
        // ddx(f(V+ddt(V)),V)=f'(V+ddt(V))*(1+s). Linearizing the
        // remaining f' adds another (1+s), giving f''*(1+2s+s*s).
        (
            "ddx(hypot(V(p,n)+ddt(V(p,n)),2.0),V(p,n))",
            "4.0/(v*v+4.0).powf(1.5)",
            "-4.0/(v*v+4.0).powf(1.5)*w*w",
            "8.0/(v*v+4.0).powf(1.5)*w",
        ),
        (
            "ddx(atan2(V(p,n)+ddt(V(p,n)),2.0),V(p,n))",
            "-4.0*v/(v*v+4.0).powi(2)",
            "4.0*v/(v*v+4.0).powi(2)*w*w",
            "-8.0*v/(v*v+4.0).powi(2)*w",
        ),
        (
            "ddx(1e308/(1e308*(V(p,n)+ddt(V(p,n)))),V(p,n))",
            "2.0/v.powi(3)",
            "-2.0/v.powi(3)*w*w",
            "4.0/v.powi(3)*w",
        ),
        ("sin(ddt(V(p,n)))", "0.0", "0.0", "w"),
        ("exp(ddt(V(p,n)))", "0.0", "0.0", "w"),
        ("sin(V(p,n)+ddt(V(p,n)))", "v.cos()", "0.0", "v.cos()*w"),
        ("(V(p,n)+ddt(V(p,n)))*ddt(V(p,n))", "0.0", "0.0", "v*w"),
        ("ddt(ddt(V(p,n)))", "0.0", "-w*w", "0.0"),
        ("idt(V(p,n),0.0)", "0.0", "0.0", "-1.0/w"),
        ("idt(idt(V(p,n),0.0),0.0)", "0.0", "-1.0/(w*w)", "0.0"),
        ("ddt(idt(V(p,n),0.0))", "0.0", "1.0", "0.0"),
        ("idt(ddt(V(p,n)),0.0)", "0.0", "1.0", "0.0"),
        ("ddt(ddt(ddt(V(p,n))))", "0.0", "0.0", "-w*w*w"),
        (
            "sin(V(p,n)+idt(V(p,n),0.25))",
            "(v+0.25).cos()",
            "0.0",
            "-(v+0.25).cos()/w",
        ),
        ("exp(ddt(ddt(V(p,n))))", "0.0", "-w*w", "0.0"),
        (
            "ddt(sin(V(p,n)+ddt(V(p,n))))",
            "0.0",
            "-v.cos()*w*w",
            "v.cos()*w",
        ),
        ("ddx(ddt(V(p,n)*V(p,n)),V(p,n))", "0.0", "0.0", "2.0*w"),
        (
            "ddx(ddt(V(p,n)*V(p,n)),V(p,n))*V(p,n)",
            "0.0",
            "0.0",
            "2.0*v*w",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let source = format!(
            "module nonlinear_dynamic(p,n); inout p,n; electrical p,n; analog I(p,n)<+{expression}; endmodule"
        );
        let name = format!("nonlinear dynamic chain {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        let body = format!(
            r#"
let mut instance = device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
runtime::set_dynamic_operators_enabled(false);
for v in [0.5_f64, -0.75, 1.25] {{
let bias = [v,0.0];
let ctx = runtime::GeneratedEvalContext {{ voltages: &bias, temperature: 123.0 }};
let mut real = [0.0;12];
instance.stamp(&ctx, &mut runtime::GeneratedStamper {{ sink: Some(&mut real) }});
assert!((real[10]-({static_real})).abs()<1e-12, "static derivative: {{real:?}}");
let history = instance.capture_rollback_state();
for w in [0.125_f64, 1.0, 3.5, 100.0] {{
runtime::FREQUENCY_OMEGA.store(w.to_bits(), std::sync::atomic::Ordering::SeqCst);
let mut reactive = [0.0;6];
instance.stamp_reactive(&ctx, &mut runtime::GeneratedReactiveStamper {{ sink: Some(&mut reactive) }});
let expected_imaginary: f64 = {imaginary};
let expected_real: f64 = {dynamic_real};
assert!((reactive[0]-expected_imaginary).abs()<1e-12*(1.0+expected_imaginary.abs()), "imaginary derivative at {{v}}, {{w}}: {{reactive:?}}");
assert!((reactive[3]-expected_real).abs()<1e-12*(1.0+expected_real.abs()), "real correction at {{v}}, {{w}}: {{reactive:?}}");
assert_eq!(instance.capture_rollback_state(), history, "frequency evaluation changed operator history");
assert!(!ctx.evaluation_failed());
}}
}}
assert!(!device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE);
"#
        );
        run_generated_main(&name, &state, &stamp, &noise, &body)
            .unwrap_or_else(|report| panic!("{expression}: {report}"));
    }
}

#[test]
fn generated_dynamic_coefficients_apply_multiplicity_before_range_conversion() {
    let (state, stamp, noise) = generated_parts(
        "module scaled_dynamic(p,n); inout p,n; electrical p,n; parameter real gain=1.0; analog I(p,n)<+gain*sin(ddt(V(p,n))); endmodule",
        "scaled dynamic coefficient",
    );
    run_generated_main("scaled dynamic coefficient", &state, &stamp, &noise, r#"
runtime::set_dynamic_operators_enabled(false);
for (gain, scale, w) in [(1e200_f64, 1e-200, 1e200_f64), (1e-200, 1e200, 1e-200)] {
    let mut instance = device::state::Instance::new(&[0,1]);
    instance.set_parameter("gain", gain).unwrap();
    instance.set_multiplicity(scale).unwrap();
    instance.finalize_parameters().unwrap();
    let ctx = runtime::GeneratedEvalContext { voltages: &[0.5,0.0], temperature: 300.15 };
    instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
    runtime::FREQUENCY_OMEGA.store(w.to_bits(), std::sync::atomic::Ordering::SeqCst);
    let mut response = [0.0;6];
    instance.stamp_reactive(&ctx, &mut runtime::GeneratedReactiveStamper { sink: Some(&mut response) });
    assert!((response[0]/w - 1.0).abs() <= 8.0*f64::EPSILON, "{gain}, {scale}, {w}: {response:?}");
    assert_eq!(response[3], 0.0);
    assert!(!ctx.evaluation_failed());
}
"#).unwrap_or_else(|report| panic!("scaled dynamic coefficient: {report}"));
}

#[test]
fn generated_dynamic_coefficients_follow_loop_and_branch_merges() {
    for (index, (body, expected)) in [
        ("real x,y; integer k; analog begin x=ddt(V(p,n)); y=0; for(k=0;k<3;k=k+1) y=y+(k+1)*sin(x); I(p,n)<+y; end", "6.0*w"),
        ("real x,y; analog begin x=idt(V(p,n),0.0); if(V(p,n)>0) y=sin(x); else y=2*sin(x); I(p,n)<+y; end", "if v>0.0 { -1.0/w } else { -2.0/w }"),
    ].into_iter().enumerate() {
        let source = format!("module dynamic_merges(p,n); inout p,n; electrical p,n; {body} endmodule");
        let name = format!("dynamic merge {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(&name, &state, &stamp, &noise, &format!(r#"
let mut instance = device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
runtime::set_dynamic_operators_enabled(false);
for v in [0.5_f64, -0.75, 1.25] {{
let bias = [v,0.0];
let ctx = runtime::GeneratedEvalContext {{ voltages: &bias, temperature: 123.0 }};
instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
for w in [0.25_f64, 2.0] {{
runtime::FREQUENCY_OMEGA.store(w.to_bits(), std::sync::atomic::Ordering::SeqCst);
let mut response = [0.0;6];
instance.stamp_reactive(&ctx, &mut runtime::GeneratedReactiveStamper {{ sink: Some(&mut response) }});
let expected: f64 = {expected};
assert!((response[0]-expected).abs()<1e-12, "{{v}}, {{w}}: {{response:?}}");
assert_eq!(response[3], 0.0);
assert!(!ctx.evaluation_failed());
}}
}}
"#)).unwrap_or_else(|report| panic!("{body}: {report}"));
    }
}

#[test]
fn generated_dynamic_potential_rows_preserve_branch_orientation_and_flow_axes() {
    let (state, stamp, noise) = generated_parts(
        "module dynamic_branch(p,n); inout p,n; electrical p,n; analog begin V(p,n)<+0; V(n,p)<+sin(ddt(I(p,n)))+ddt(ddt(V(p,n))); end endmodule",
        "dynamic potential orientation",
    );
    run_generated_main("dynamic potential orientation", &state, &stamp, &noise, r#"
let mut instance = device::state::Instance::new(&[0,1]);
assert_eq!(device::state::Instance::BRANCH_COUNT,1);
instance.set_branch_indices(&[2]);
instance.finalize_parameters().unwrap();
runtime::set_dynamic_operators_enabled(false);
let ctx = runtime::GeneratedEvalContext { voltages: &[0.5,0.0,0.25,100.0], temperature: 123.0 };
instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
for w in [0.25_f64, 2.0] {
    runtime::FREQUENCY_OMEGA.store(w.to_bits(), std::sync::atomic::Ordering::SeqCst);
    let mut response = [0.0;6];
    instance.stamp_reactive(&ctx, &mut runtime::GeneratedReactiveStamper { sink: Some(&mut response) });
    assert_eq!(response[0], 0.0);
    assert_eq!(response[1], -w, "reversed source flow derivative: {response:?}");
    assert_eq!(response[3], w*w, "reversed source voltage derivative: {response:?}");
    assert!(!ctx.evaluation_failed());
}
"#).unwrap_or_else(|report| panic!("dynamic potential orientation: {report}"));
}

#[test]
fn generated_reactive_stamping_holds_external_derivative_coefficients_at_the_bias_point() {
    for (index, expression, capacitances, split_safe) in [
        (0, "V(p,n)*ddt(V(p,n))", [3.0_f64, -2.0], false),
        (1, "V(c,n)*ddt(V(p,n))", [2.0, -4.0], false),
        (2, "ddt(V(p,n))/V(c,n)", [0.5, -0.25], false),
        (3, "(2.0+V(c,n))*ddt(V(p,n)*V(p,n))", [24.0, 8.0], false),
        (4, "2.0*ddt(V(p,n))", [2.0, 2.0], true),
        (5, "gain*ddt(V(p,n))", [2.0, 2.0], true),
        (6, "$temperature*ddt(V(p,n))", [300.15, 300.15], true),
        (7, "$abstime*ddt(V(p,n))", [0.0, 0.0], false),
        (8, "((V(c,n)>0)?2.0:4.0)*ddt(V(p,n))", [2.0, 4.0], false),
    ] {
        let source = format!(
            "module weighted_derivative(p,n,c); inout p,n,c; electrical p,n,c; parameter real gain=2.0; analog I(p,n)<+{expression}; endmodule"
        );
        let name = format!("weighted derivative {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        let body = format!(
            r#"
let mut instance = device::state::Instance::new(&[0, 1, 2]);
instance.finalize_parameters().unwrap();
assert_eq!(device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE, {split_safe});
for (bias, capacitance) in [[3.0, 0.0, 2.0], [-2.0, 0.0, -4.0]].into_iter().zip({capacitances:?}) {{
let ctx = runtime::GeneratedEvalContext {{ voltages: &bias, temperature: 300.15 }};
instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
let mut reactive = [0.0; 3];
instance.stamp_reactive(&ctx, &mut runtime::GeneratedReactiveStamper {{ sink: Some(&mut reactive) }});
assert_eq!(reactive[0], capacitance, "driven-port capacitance");
assert_eq!(reactive[2], 2.0 * capacitance.abs(), "only the driven terminal pair has a reactive derivative");
}}
"#,
        );
        run_generated_main(&name, &state, &stamp, &noise, &body)
            .unwrap_or_else(|report| panic!("{expression}: {report}"));
    }
}

#[test]
fn generated_operator_chains_preserve_association_and_function_effects() {
    let chain = " + 1.0e16 + 1.0 - 1.0e16".repeat(16);
    let source = format!(
        "module operator_effects(p,n);\n\
         inout p,n; electrical p,n; integer count;\n\
         analog function integer bump;\n\
         inout counter; integer counter;\n\
         begin counter=counter+1; bump=counter; end\n\
         endfunction\n\
         analog begin count=0; I(p,n)<+(V(p,n){chain})+bump(count)-bump(count); end\n\
         endmodule\n"
    );
    let (state, stamp, noise) = generated_parts(&source, "operator traversal");
    run_generated_main(
        "operator traversal",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance = device::state::Instance::new(&[0,1]);
let ctx = runtime::GeneratedEvalContext { voltages: &[0.0,0.0], temperature: 300.0 };
for _ in 0..2 {
    let mut sink = [0.0];
    instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut sink) });
    assert_eq!(sink[0],-1.0);
    assert!(!ctx.evaluation_failed());
}
"#,
    )
    .expect("generated operators preserve association and left-to-right effects");
}

#[test]
fn nodeset_capability_tracks_simulation_code_across_runtime_and_generated_models() {
    for (body, expected) in [
        ("analog I(p,n)<+V(p,n);", false),
        (
            "analog initial state=analysis(\"nodeset\"); analog I(p,n)<+state*V(p,n);",
            false,
        ),
        (
            "analog begin state=analysis(\"NODESET\"); I(p,n)<+state*V(p,n); end",
            true,
        ),
        (
            "analog begin if (analysis(\"nodeset\")) V(p,n)<+1; else I(p,n)<+V(p,n); end",
            true,
        ),
        (
            "analog begin while (analysis(\"nodeset\") && state<1) state=state+1; I(p,n)<+state*V(p,n); end",
            true,
        ),
        (
            "analog begin $finish(analysis(\"nodeset\")); I(p,n)<+V(p,n); end",
            true,
        ),
        (
            "analog initial state=analysis(\"nodeset\"); analog I(p,n)<+V(p,n)+analysis(\"nodeset\");",
            true,
        ),
    ] {
        let source = format!(
            "module nodeset_capability(p,n); inout p,n; electrical p,n; real state; {body} endmodule"
        );
        let compiled = VerilogACompiler::default().compile(&source).unwrap();
        assert_eq!(compiled.requires_nodeset_phase, expected, "{body}");
        let (state, stamp, noise) = generated_parts(&source, "nodeset capability");
        run_generated_main(
            "nodeset capability",
            &state,
            &stamp,
            &noise,
            &format!("assert_eq!(device::state::Instance::REQUIRES_NODESET_PHASE, {expected});"),
        )
        .expect("generated nodeset capability compiles and executes");
    }
}

#[test]
fn generated_boolean_initializers_and_uninitialized_loop_locals_are_numeric() {
    let (state, stamp, noise) = generated_parts(
        r#"module initial_loop(p,n);
inout p,n; electrical p,n; real enabled, count;
analog initial enabled=($temperature>0);
analog begin
    while (V(p,n)>0 && count<1) count=count+1;
    I(p,n)<+enabled*count*V(p,n);
end
endmodule"#,
        "numeric initialization and loop entry",
    );
    run_generated_main(
        "numeric initialization and loop entry",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance = device::state::Instance::new(&[0,1]);
for (voltage, expected) in [(1.0,1.0),(-1.0,0.0)] {
    let ctx = runtime::GeneratedEvalContext { voltages: &[voltage,0.0], temperature: 300.0 };
    let mut sink = [0.0];
    instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut sink) });
    assert_eq!(sink[0],expected);
    assert!(!ctx.evaluation_failed());
    assert_eq!(&*instance.event_state_accepted, &[1.0]);
}
"#,
    )
    .expect("initialization and loop entry preserve numeric values");
}

#[test]
fn generated_analysis_restart_is_atomic_and_clears_operator_history() {
    let (state, stamp, noise) = generated_parts(
        r#"module restart(p,n);
inout p,n; electrical p,n;
parameter real gain=2;
integer starts; real scale;
analog initial begin starts=starts+1; scale=gain*$temperature; $finish(0); end
analog I(p,n)<+scale*V(p,n)+ddt(V(p,n))+idt(V(p,n),0.0);
endmodule"#,
        "analysis restart",
    );
    run_generated_main("analysis restart", &state, &stamp, &noise, r#"
let ctx = runtime::GeneratedEvalContext { voltages: &[1.0,0.0], temperature: 300.0 };
let invalid = runtime::GeneratedEvalContext { voltages: &[1.0,0.0], temperature: f64::NAN };
let mut instance = device::state::Instance::new(&[0,1]);
assert!(!instance.has_point_analog_tasks(), "initialization tasks do not serialize frequency sweeps");
instance.set_parameter("gain", 4.0).unwrap();
instance.set_multiplicity(3.0).unwrap();
instance.begin_analysis(&ctx);
assert_eq!(&*instance.event_state_accepted, &[1.0,1200.0]);
assert_eq!(instance.drain_analog_tasks().count(), 1);
instance.stamp_state.ddt_previous.fill(7.0);
assert!(!instance.stamp_state.idt_previous.is_empty());
instance.stamp_state.idt_previous.fill(8.0);
instance.stamp_state.ddt_initialized.fill(true);
instance.stamp_state.idt_initialized.fill(true);
instance.time = 2.0;
instance.timestep = 0.25;
instance.analog_effects.as_mut().unwrap().record_finish(0,2.0,1.0).unwrap();
let before = instance.capture_rollback_state();
instance.begin_analysis(&invalid);
assert!(invalid.evaluation_failed());
assert_eq!(instance.capture_rollback_state(), before);
assert_eq!((instance.time,instance.timestep), (2.0,0.25));
runtime::clear_evaluation_error();
instance.begin_analysis(&ctx);
assert_eq!(&*instance.event_state_accepted, &[1.0,1200.0]);
assert_eq!(instance.multiplicity, 3.0);
assert_eq!((instance.time,instance.timestep), (0.0,0.0));
assert!(instance.stamp_state.ddt_previous.iter().chain(&instance.stamp_state.idt_previous).all(|value| *value == 0.0));
assert!(instance.stamp_state.ddt_initialized.iter().chain(&instance.stamp_state.idt_initialized).all(|value| !value));
let calls = instance.drain_analog_tasks().collect::<Vec<_>>();
assert_eq!(calls.len(),1);
assert_eq!(&*calls[0].arguments, &[runtime::AnalogTaskArgument::Integer(0)]);
assert_eq!(calls[0].time,0.0);
let accepted = instance.capture_persistent_state();
let mut uninitialized = device::state::Instance::new(&[0,1]);
let uninitialized_state = uninitialized.capture_rollback_state();
assert!(uninitialized.restore_analysis_continuation_state(&accepted, &invalid).is_err());
assert_eq!(uninitialized.capture_rollback_state(),uninitialized_state);
let hot = runtime::GeneratedEvalContext { voltages: &[1.0,0.0], temperature: 320.0 };
let mut rebuilt = device::state::Instance::new(&[0,1]);
rebuilt.set_parameter("gain",4.0).unwrap();
rebuilt.restore_analysis_continuation_state(&accepted, &hot).unwrap();
rebuilt.stamp(&hot, &mut runtime::GeneratedStamper::default());
assert_eq!(&*rebuilt.event_state_accepted, &[1.0,1200.0]);
assert_eq!(rebuilt.drain_analog_tasks().count(),0,"rebuilding must not replay the initializer when temperature changes");
"#).unwrap();
}

#[test]
fn generated_phase_changes_preserve_initialization_and_refresh_analysis_queries() {
    let (state, stamp, noise) = generated_parts(
        r#"module initialized_phase(p,n);
inout p,n; electrical p,n; real saved;
analog initial saved=analysis("static") ? 100 : 200;
analog I(p,n)<+saved+analysis("static");
endmodule"#,
        "analysis phase lifetime",
    );
    run_generated_main(
        "analysis phase lifetime",
        &state,
        &stamp,
        &noise,
        r#"
let ctx = runtime::GeneratedEvalContext { voltages: &[0.0,0.0], temperature: 123.0 };
let mut instance = device::state::Instance::new(&[0,1]);
runtime::set_event_analysis(false, true);
instance.begin_analysis(&ctx);
let mut sink = [0.0];
instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut sink) });
assert_eq!(sink[0], 101.0);
runtime::set_event_analysis(false, false);
sink[0] = 0.0;
instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut sink) });
assert_eq!(sink[0], 100.0, "frequency transition replayed analog initial or cached the phase");
instance.begin_analysis(&ctx);
sink[0] = 0.0;
instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut sink) });
assert_eq!(sink[0], 200.0, "fresh analysis did not initialize");
"#,
    )
    .expect("generated phase contract executes");
}

#[test]
fn generated_continuation_does_not_execute_a_now_invalid_initializer() {
    let (state, stamp, noise) = generated_parts(
        r#"module continued_initial(p,n);
inout p,n; electrical p,n;
real scale;
analog initial scale=sqrt(301.0-$temperature);
analog I(p,n)<+scale*V(p,n);
endmodule"#,
        "continuation initializer",
    );
    run_generated_main(
        "continuation initializer",
        &state,
        &stamp,
        &noise,
        r#"
let cold = runtime::GeneratedEvalContext { voltages: &[1.0,0.0], temperature: 300.0 };
let hot = runtime::GeneratedEvalContext { voltages: &[1.0,0.0], temperature: 302.0 };
let mut initial = device::state::Instance::new(&[0,1]);
initial.begin_analysis(&cold);
assert!(!cold.evaluation_failed());
let accepted = initial.capture_persistent_state();
let mut rebuilt = device::state::Instance::new(&[0,1]);
rebuilt.restore_analysis_continuation_state(&accepted, &hot).unwrap();
let mut values = [0.0; 10];
rebuilt.stamp(&hot, &mut runtime::GeneratedStamper { sink: Some(&mut values) });
assert!(!hot.evaluation_failed(), "the initializer must not run again at the new temperature");
assert_eq!(values[9], 1.0);
assert_eq!(&*rebuilt.event_state_accepted, &[1.0]);
"#,
    )
    .unwrap();
}

#[test]
fn generated_initialization_tracks_analysis_and_simparam_presence() {
    let (state, stamp, noise) = generated_parts(
        r#"module initialized_context(p,n);
inout p,n; electrical p,n;
integer starts; real scale;
analog initial begin starts=starts+1; scale=analysis("tran") ? $simparam("pnjmaxi",4) : 2; end
analog I(p,n)<+scale*V(p,n)+white_noise(scale,"context");
endmodule"#,
        "initialization context",
    );
    run_generated_main(
        "initialization context",
        &state,
        &stamp,
        &noise,
        r#"
struct Noise;
impl runtime::GeneratedNoiseVisitor for Noise {
    fn visit(&mut self, _: usize, _: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool { true }
}
let ctx = runtime::GeneratedEvalContext { voltages: &[1.0,0.0], temperature: 300.15 };
let mut instance = device::state::Instance::new(&[0,1]);
instance.initialize_analysis(&ctx);
assert_eq!(&*instance.event_state_accepted, &[1.0,2.0]);
runtime::set_event_analysis(true, false);
assert!(instance.evaluate_noise_sources(&ctx, &mut Noise).is_err());
instance.initialize_analysis(&ctx);
assert_eq!(&*instance.event_state_accepted, &[2.0,4.0]);
let checkpoint = instance.capture_persistent_state();
assert_eq!(checkpoint.event_variables.len(), 5);
assert_eq!(checkpoint.event_variables[2], 2.0, "physical transient analysis key");
for (slot, value) in [(2, 5.0), (2, -1.0), (2, 0.5), (2, f64::NAN), (3, -1.0), (4, f64::NAN)] {
    let mut invalid = checkpoint.clone();
    invalid.event_variables[slot] = value;
    let before = instance.capture_rollback_state();
    assert!(instance.restore_persistent_state(&invalid).is_err());
    assert_eq!(instance.capture_rollback_state(), before);
}
runtime::set_simparam_override(Some(0.0));
assert!(instance.evaluate_noise_sources(&ctx, &mut Noise).is_err());
instance.initialize_analysis(&ctx);
assert_eq!(&*instance.event_state_accepted, &[3.0,0.0]);
runtime::set_simparam_override(None);
instance.initialize_analysis(&ctx);
assert_eq!(&*instance.event_state_accepted, &[4.0,4.0]);
instance.restore_persistent_state(&checkpoint).unwrap();
instance.initialize_analysis(&ctx);
assert_eq!(&*instance.event_state_accepted, &[2.0,4.0]);
instance.begin_analysis(&ctx);
assert_eq!(&*instance.event_state_accepted, &[1.0,4.0]);
"#,
    )
    .unwrap();
}

#[test]
fn generated_simparam_names_preserve_string_escaping() {
    let (state, stamp, noise) = generated_parts(
        r#"module escaped_context(p,n);
inout p,n; electrical p,n;
real scale;
analog initial scale=$simparam("key\"quoted",3);
analog I(p,n)<+scale*V(p,n)+$simparam("key\\slash",4);
endmodule"#,
        "escaped context names",
    );
    run_generated_main(
        "escaped context names",
        &state,
        &stamp,
        &noise,
        r#"
let ctx = runtime::GeneratedEvalContext { voltages: &[1.0,0.0], temperature: 300.15 };
let mut instance = device::state::Instance::new(&[0,1]);
let mut values = [0.0; 10];
instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut values) });
assert!(!ctx.evaluation_failed());
assert_eq!(values[9], 7.0);
"#,
    )
    .unwrap();
}

#[test]
fn generated_localparams_reach_residual_derivatives_and_noise() {
    let (state, stamp, noise) = generated_parts(
        r#"module localparam_device(p,n);
inout p,n; electrical p,n; parameter real gain=2;
localparam real scale=gain+1;
analog V(p,n)<+scale*V(p,n)+white_noise(scale,"localparam");
endmodule"#,
        "localparam values",
    );
    run_generated_main("localparam values", &state, &stamp, &noise, r#"
#[derive(Default)]
struct Noise(f64);
impl runtime::GeneratedNoiseVisitor for Noise {
    fn visit(&mut self, _: usize, value: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool { self.0 += value.psd; true }
}
let ctx = runtime::GeneratedEvalContext { voltages: &[2.0, 0.0], temperature: 300.15 };
let mut instance = device::state::Instance::new(&[0,1]);
for (gain, scale) in [(2.0, 3.0), (4.0, 5.0)] {
    instance.set_parameter("gain", gain).unwrap();
    let mut values = [0.0; 5];
    instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut values) });
    assert_eq!(values[2], 2.0 * scale);
    assert_eq!(values[3], scale);
    let mut noise = Noise::default();
    instance.evaluate_noise_sources(&ctx, &mut noise).unwrap();
    assert_eq!(noise.0, scale);
}
"#).unwrap();
}

#[test]
fn generated_initialization_tracks_temperature_and_rejects_stale_noise_state() {
    let (state, stamp, noise) = generated_parts(
        r#"module initialized_temperature(p,n);
inout p,n; electrical p,n;
integer starts;
real scale;
analog initial begin starts=starts+1; scale=$temperature; end
analog I(p,n)<+scale*V(p,n)+white_noise(scale,"thermal");
endmodule"#,
        "initialization temperature",
    );
    run_generated_main(
        "initialization temperature",
        &state,
        &stamp,
        &noise,
        r#"
struct Noise;
impl runtime::GeneratedNoiseVisitor for Noise {
    fn visit(&mut self, _: usize, _: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool { true }
}
let cold = runtime::GeneratedEvalContext { voltages: &[1.0, 0.0], temperature: 300.0 };
let hot = runtime::GeneratedEvalContext { voltages: &[1.0, 0.0], temperature: 320.0 };
let mut instance = device::state::Instance::new(&[0,1]);
assert!(instance.validate_checkpoint_ready().is_err());
assert!(instance.evaluate_noise_sources(&cold, &mut Noise).is_err());
instance.initialize_analysis(&cold);
instance.validate_checkpoint_ready().unwrap();
assert_eq!(&*instance.event_state_accepted, &[1.0, 300.0]);
instance.evaluate_noise_sources(&cold, &mut Noise).unwrap();
let checkpoint = instance.capture_persistent_state();
assert!(instance.evaluate_noise_sources(&hot, &mut Noise).is_err());
instance.stamp(&hot, &mut runtime::GeneratedStamper::default());
assert_eq!(&*instance.event_state_accepted, &[2.0, 320.0]);
instance.restore_persistent_state(&checkpoint).unwrap();
instance.stamp(&cold, &mut runtime::GeneratedStamper::default());
assert_eq!(&*instance.event_state_accepted, &[1.0, 300.0]);
let invalid = runtime::GeneratedEvalContext { voltages: &[1.0, 0.0], temperature: f64::NAN };
let mut values = [0.0];
instance.stamp(&invalid, &mut runtime::GeneratedStamper { sink: Some(&mut values) });
assert!(invalid.evaluation_failed());
assert_eq!(values, [0.0]);
assert!(instance.validate_advance_state().is_err());
runtime::clear_evaluation_error();
instance.initialize_analysis(&cold);
assert!(!cold.evaluation_failed());
"#,
    )
    .unwrap();
}

#[test]
fn generated_initialization_is_persistent_and_precedes_the_numerical_body() {
    let (state, stamp, noise) = generated_parts(
        r#"
module initialized(p,n);
inout p,n; electrical p,n;
parameter real gain=2;
localparam real offset=gain+1;
integer count=5;
real scale;
analog initial begin count=count+1; scale=offset; end
analog begin count=count+1; I(p,n)<+count+scale*V(p,n); end
endmodule
"#,
        "generated initialization",
    );
    run_generated_main(
        "generated initialization",
        &state,
        &stamp,
        &noise,
        r#"
let ctx = runtime::GeneratedEvalContext { voltages: &[2.0, 0.0], temperature: 300.15 };
let mut instance = device::state::Instance::new(&[0,1]);
instance.initialize_analysis(&ctx);
assert!(!ctx.evaluation_failed());
assert_eq!(&*instance.event_state_accepted, &[6.0, 3.0]);
for _ in 0..2 {
    instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
    assert_eq!(&*instance.event_state_candidate, &[7.0, 3.0]);
}
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let checkpoint = instance.capture_persistent_state();
let rollback = instance.capture_rollback_state();
instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
assert_eq!(&*instance.event_state_candidate, &[8.0, 3.0]);
instance.restore_rollback_state(&rollback);
instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
assert_eq!(&*instance.event_state_candidate, &[8.0, 3.0]);
instance.set_parameter("gain", 4.0).unwrap();
instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
assert_eq!(&*instance.event_state_candidate, &[7.0, 5.0]);
instance.restore_persistent_state(&checkpoint).unwrap();
instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
assert_eq!(&*instance.event_state_candidate, &[8.0, 3.0]);
"#,
    )
    .unwrap();
}

#[test]
fn generated_system_tasks_follow_acceptance_rollback_and_observation_boundaries() {
    let (state, stamp, noise) = generated_parts(
        r#"
module generated_tasks(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    integer i;
    analog begin
        if (V(p, n) > 0.0) begin
            i = 0;
            while (i < 3) begin
                $finish(i);
                i = i + 1;
            end
        end
        if (V(p, n) < 0.0) $finish(99);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
        "generated system tasks",
    );
    run_generated_main(
        "generated system tasks",
        &state,
        &stamp,
        &noise,
        r#"
fn stamp(instance: &mut device::state::Instance, voltage: f64) {
    let voltages = [voltage, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
}
fn accept(instance: &mut device::state::Instance) -> Vec<runtime::AnalogTaskInvocation> {
    instance.validate_advance_state().unwrap();
    instance.apply_validated_advance_state();
    instance.drain_analog_tasks().collect()
}
let mut instance = device::state::Instance::new(&[0, 1]);
assert!(instance.capture_rollback_state().analog_effects.is_none());
assert!(instance.has_point_analog_tasks(), "tasks nested inside loops require ordered point publication");
instance.set_timepoint(1.0, 0.0, runtime::GeneratedDdtCoefficients::inactive());
stamp(&mut instance, 1.0);
assert_eq!(instance.drain_analog_tasks().count(), 0);
assert_eq!(instance.candidate_analog_tasks().unwrap().len(), 3);
assert!(instance.validate_checkpoint_ready().is_err());
let candidate = instance.capture_rollback_state();
runtime::set_tasks_enabled(false);
stamp(&mut instance, -1.0);
assert_eq!(candidate, instance.capture_rollback_state());
runtime::set_tasks_enabled(true);
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
assert!(instance.validate_checkpoint_ready().is_err());
assert!(instance.candidate_analog_tasks().unwrap().is_empty());
let calls = instance.drain_analog_tasks().collect::<Vec<_>>();
assert_eq!(calls.len(), 3);
for (level, call) in calls.iter().enumerate() {
    assert_eq!(call.kind, runtime::AnalogTaskKind::Finish);
    assert_eq!(call.site, calls[0].site);
    assert_eq!(call.time, 1.0);
    assert_eq!(&*call.arguments, &[runtime::AnalogTaskArgument::Integer(level as i64)]);
}
instance.validate_checkpoint_ready().unwrap();
assert_eq!(instance.drain_analog_tasks().count(), 0);
stamp(&mut instance, 1.0);
stamp(&mut instance, 0.0);
assert!(accept(&mut instance).is_empty());
stamp(&mut instance, 1.0);
let candidate = instance.capture_rollback_state();
stamp(&mut instance, -1.0);
assert!(instance.validate_advance_state().is_err());
assert!(instance.candidate_analog_tasks().is_err());
assert!(instance.drain_analog_tasks().next().is_none());
instance.restore_rollback_state(&candidate);
runtime::clear_evaluation_error();
assert_eq!(accept(&mut instance).len(), 3);
stamp(&mut instance, 1.0);
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
stamp(&mut instance, 1.0);
instance.reset_analog_tasks();
instance.validate_checkpoint_ready().unwrap();
assert_eq!(instance.drain_analog_tasks().count(), 0);
let checkpoint = instance.capture_persistent_state();
stamp(&mut instance, 1.0);
instance.restore_persistent_state(&checkpoint).unwrap();
instance.validate_checkpoint_ready().unwrap();
"#,
    )
    .unwrap_or_else(|error| panic!("generated task lifecycle: {error}"));
}

#[test]
fn a_generated_device_compiles_against_the_runtime_contract() {
    for (name, source) in fixtures() {
        let artifact = VerilogACompiler::default()
            .compile_canonical_ir(source)
            .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
        let device = canonical::generate_device(&artifact, &options())
            .unwrap_or_else(|error| panic!("{name}: generation: {error}"));

        let files: Vec<(&str, &str)> = device
            .files
            .iter()
            .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
            .collect();
        let state = find(&files, "state.rs", name);
        let stamp = find(&files, "stamp.rs", name);
        let noise = find(&files, "noise.rs", name);

        if let Err(report) = compile(name, state, stamp, noise) {
            panic!("{name}: the generated device does not compile:\n{report}");
        }
    }
}

#[test]
fn generated_terminal_metadata_preserves_source_order_spelling_and_current_names() {
    let (state, _, _) = generated_parts(
        r#"
module terminal_metadata(d, FG, s, Tnode);
    input d;
    output FG;
    inout s, Tnode;
    electrical d, FG, s, Tnode;
    analog I(d, s) <+ V(d, s);
endmodule
"#,
        "terminal metadata",
    );

    assert!(state.contains(
        "GeneratedVerilogATerminalDescriptor { name: \"d\", direction: GeneratedVerilogATerminalDirection::Input, discipline: \"electrical\", current_parameter: \"id\" }"
    ));
    assert!(state.contains(
        "GeneratedVerilogATerminalDescriptor { name: \"FG\", direction: GeneratedVerilogATerminalDirection::Output, discipline: \"electrical\", current_parameter: \"ifg\" }"
    ));
    assert!(state.contains(
        "GeneratedVerilogATerminalDescriptor { name: \"Tnode\", direction: GeneratedVerilogATerminalDirection::InOut, discipline: \"electrical\", current_parameter: \"itnode\" }"
    ));
    assert!(!state.contains("TERMINAL_NAMES"));
}

#[test]
fn generated_parameter_descriptors_are_the_public_scope_authority() {
    let (state, _, _) = generated_parts(
        r#"
module parameter_metadata(p, n);
    inout p, n;
    electrical p, n;
    parameter real limit = 10.0;
    (* type = "instance", xyceAlsoModel = "yes" *)
        parameter integer gain = 2 from [0.0:limit];
    aliasparam GAIN_ALIAS = gain;
    analog I(p, n) <+ gain * V(p, n);
endmodule
"#,
        "parameter descriptor metadata",
    );

    assert!(state.contains(
        "P::dual(\"gain\", Some(2.0)).integer().aliases(&[\"GAIN_ALIAS\"]).minimum(B::inclusive(0.0)).dynamic_constraints()"
    ));
    assert!(!state.contains("GeneratedVerilogAParameterDescriptor {"));
    assert!(!state.contains("pub fn parameter_scope"));
}

/// The zeros are the point, so they are checked separately from compiling.
///
/// A two-terminal resistor reaches two unknowns and no more. The tier this
/// replaces writes `multiplicity * 0.0` for the rest of the row; here the
/// entries do not exist, so the literal never appears.
#[test]
fn a_stamp_writes_no_literal_zero_entries() {
    let source = r#"
module divider(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r1 = 100.0;
    parameter real r2 = 220.0;
    analog begin
        I(p, mid) <+ V(p, mid) / r1;
        I(mid, n) <+ V(mid, n) / r2;
    end
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files: Vec<(&str, &str)> = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect();
    let stamp = find(&files, "stamp.rs", "divider");

    for (index, line) in stamp.lines().enumerate() {
        assert!(
            !line.contains("multiplicity * 0.0"),
            "divider: stamp.rs line {} writes a literal zero: {line}",
            index + 1
        );
    }
    assert!(
        stamp.contains("stamp_current_sparse_local::<2, 0>"),
        "each branch of the divider reaches exactly two nodes; stamp.rs was:\n{stamp}"
    );
}

#[test]
fn generated_integer_constant_comparisons_preserve_exact_defaults() {
    let (state, stamp, noise) = generated_parts(
        r#"
module exact_defaults(p, n);
    inout p, n;
    electrical p, n;
    parameter integer eq = (9007199254740992 + 1) == 9007199254740992;
    parameter integer ne = (9007199254740992 + 1) != 9007199254740992;
    parameter integer lt = -(9007199254740992 + 1) < -9007199254740992;
    parameter integer le = (9007199254740992 + 1) <= 9007199254740992;
    parameter integer gt = (9007199254740992 + 1) > 9007199254740992;
    parameter integer ge = -(9007199254740992 + 1) >= -9007199254740992;
    parameter integer mixed = (9007199254740992 + 1) == 9007199254740992.0;
    analog I(p, n) <+ (eq + ne + lt + le + gt + ge + mixed) * V(p, n);
endmodule
"#,
        "exact integer defaults",
    );
    let body = r#"
let instance = device::state::Instance::new(&[0, 1]);
assert_eq!(instance.params.values, [0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
"#;
    run_generated_main("exact integer defaults", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated integer defaults failed:\n{report}"));
}

#[test]
fn generated_integer_parameters_round_defaults_and_atomic_overrides() {
    let (state, stamp, noise) = generated_parts(
        "module rounded_parameters(p,n); inout p,n; electrical p,n; parameter real input_value=1.5; parameter integer fixed=-1.5, derived=input_value; analog I(p,n)<+(fixed+derived)*V(p,n); endmodule",
        "rounded parameters",
    );
    let body = r#"
use runtime::GeneratedParameterAssignment as Assignment;
let mut instance = device::state::Instance::new(&[0, 1]);
assert_eq!(instance.params.values, [1.5, -2.0, 2.0]);
assert_eq!(device::state::Parameters::default().values, instance.params.values);
for value in [-2.5_f64, -0.5, 0.49, 0.5, 1.5, 2.5] {
    instance.apply_parameters(&[
        Assignment::for_declared_scope("fixed", value),
        Assignment::for_declared_scope("input_value", value),
    ]).unwrap();
    assert_eq!(instance.params.values, [value, value.round(), value.round()]);
    let bias = [3.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &bias, temperature: 300.0 };
    let mut sink = [0.0; 12];
    instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut sink) });
    assert_eq!(sink[9], 6.0 * value.round());
    assert_eq!(sink[10], 2.0 * value.round());
}
let valid = instance.params.values;
for invalid in [f64::NAN, f64::INFINITY, 2147483647.5, -2147483648.5] {
    assert!(instance.apply_parameters(&[
        Assignment::for_declared_scope("input_value", 10.0),
        Assignment::for_declared_scope("fixed", invalid),
    ]).is_err());
    assert_eq!(instance.params.values, valid);
}
"#;
    run_generated_main("rounded parameters", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated parameter conversion failed:\n{report}"));
}

#[test]
fn generated_parameter_construction_validates_the_final_instance() {
    let (state, stamp, noise) = generated_parts(
        "module required_parameters(p,n); inout p,n; electrical p,n; parameter real width=0 from (0:inf); parameter integer sections=1.5; parameter integer derived=sections/width; parameter real alias=width; (* type = \"instance\", xyceAlsoModel = \"yes\" *) parameter real dual=width-2 from (0:inf); analog I(p,n)<+derived*alias*V(p,n); endmodule",
        "required parameters",
    );
    let body = r#"
use runtime::GeneratedParameterAssignment as Assignment;
assert!(device::state::Instance::try_new_with_parameters(&[0, 1], &[]).is_err());
let mut instance = device::state::Instance::try_new_with_parameters(&[0, 1], &[
    Assignment::for_declared_scope("width", 2.0),
    Assignment::for_declared_scope("sections", 4.5),
    Assignment::new("dual", 3.0, runtime::GeneratedParameterOrigin::Instance),
]).unwrap();
assert_eq!(instance.params.values, [2.0, 5.0, 3.0, 2.0, 3.0]);
let valid = instance.params.values;
assert!(instance.set_parameter("width", 0.0).is_err());
assert_eq!(instance.params.values, valid);
let bias = [3.0, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &bias, temperature: 300.0 };
let mut sink = [0.0; 12];
instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut sink) });
assert_eq!(sink[9], 18.0);
assert_eq!(sink[10], 6.0);
"#;
    run_generated_main("required parameters", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated parameter construction failed:\n{report}"));
}

#[test]
fn generated_dependent_parameter_defaults_finalize_after_all_overrides() {
    let source = r#"
module dependent_defaults(p, n);
    inout p, n;
    electrical p, n;
    parameter real base = 2.0;
    parameter real alias = base;
    parameter real chain = alias * 3.0;
    parameter real mode = 0.0;
    parameter real choice = mode > 0.5 ? chain + 1.0 : chain - 1.0;
    parameter real given_sensitive = $param_given(base) ? choice + 100.0 : choice;
    parameter real bounded_source = 1.0;
    parameter real bounded_dependent = bounded_source * 2.0 from [0.0:10.0];
    analog I(p, n) <+ given_sensitive * V(p, n);
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect::<Vec<_>>();
    let state = find(&files, "state.rs", "dependent defaults");
    let stamp = find(&files, "stamp.rs", "dependent defaults");
    let noise = find(&files, "noise.rs", "dependent defaults");

    assert!(state.contains("pub fn finalize_parameters(&mut self)"));
    if let Err(report) =
        run_dependent_parameter_defaults("dependent parameter defaults", state, stamp, noise)
    {
        panic!("generated dependent parameter defaults failed:\n{report}");
    }
}

#[test]
fn generated_dependent_parameter_defaults_use_bounded_ordered_helpers() {
    let mut parameters = String::from("    parameter real p0 = 1.0;\n");
    for index in 1..=17 {
        parameters.push_str(&format!(
            "    parameter real p{index} = p{} + 1.0;\n",
            index - 1
        ));
    }
    let source = format!(
        "module bounded_defaults(p, n);\n    inout p, n;\n    electrical p, n;\n{parameters}    analog I(p, n) <+ p17 * V(p, n);\nendmodule\n"
    );
    let (state, stamp, noise) = generated_parts(&source, "bounded dependent defaults");

    assert!(state.contains("fn finalize_parameter_vector_chunk_0"));
    assert!(state.contains("fn finalize_parameter_vector_chunk_1"));
    assert!(state.contains("fn finalize_parameter_vector_chunk_2"));
    assert!(!state.contains("fn finalize_parameter_vector_chunk_3"));
    assert_eq!(state.matches("#[inline(never)]").count(), 3);

    let body = r#"
let instance = device::state::Instance::new(&[0, 1]);
assert_eq!(instance.params.values[17], 18.0, "dependent defaults must finalize in source order across helper boundaries");
assert_eq!(instance.transient_event_refinement_time(), None);
assert_eq!(instance.transient_timer_event_time(), None);
assert_eq!(instance.transient_timer_step_bound(), None);
"#;
    run_generated_main("bounded dependent defaults", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated bounded dependent defaults failed:\n{report}"));
}

#[test]
fn generated_parameter_assignment_scope_and_dual_fallback_are_preserved() {
    let (state, stamp, noise) = generated_parts(
        r#"
module scoped_parameters(p, n);
    inout p, n;
    electrical p, n;
    parameter real model_only = 2.0;
    (* type = "instance" *) parameter real instance_only = 3.0;
    (* type = "instance", xyceAlsoModel = "yes" *) parameter real dual = 4.0;
    parameter real model_from_dual = dual * 2.0;
    (* type = "instance" *) parameter real dual_seen_given = $param_given(dual) ? 1.0 : 0.0;
    real shape;
    analog begin
        shape = model_only * model_only + instance_only * dual + model_from_dual;
        shape = shape * shape + dual_seen_given;
        I(p, n) <+ shape * V(p, n);
    end
endmodule
"#,
        "scoped parameter assignments",
    );
    let body = r#"
use runtime::{GeneratedParameterAssignment as Assignment, GeneratedParameterOrigin as Origin, GeneratedVerilogAParameterScope as Scope};
let mut instance = device::state::Instance::new(&[0, 1]);
let defaults = instance.params.values;

let scope = |name: &str| device::state::Instance::PARAMETER_DESCRIPTORS
    .iter()
    .find(|parameter| parameter.name.eq_ignore_ascii_case(name))
    .map(|parameter| parameter.scope);
assert_eq!(scope("MODEL_ONLY"), Some(Scope::Model));
assert_eq!(scope("instance_only"), Some(Scope::Instance));
assert_eq!(scope("dual"), Some(Scope::Dual));
assert_eq!(scope("missing"), None);

let error = instance.apply_parameters(&[
    Assignment::new("model_only", 7.0, Origin::Instance),
]).unwrap_err();
assert!(error.contains("model-card"), "{error}");
assert_eq!(instance.params.values, defaults);

let error = instance.apply_parameters(&[
    Assignment::new("instance_only", 7.0, Origin::ModelCard),
]).unwrap_err();
assert!(error.contains("instance"), "{error}");
assert_eq!(instance.params.values, defaults);

instance.apply_parameters(&[
    Assignment::new("MODEL_ONLY", 5.0, Origin::ModelCard),
    Assignment::new("dual", 8.0, Origin::ModelCard),
]).unwrap();
assert_eq!(instance.params.values[0], 5.0);
assert_eq!(instance.params.values[2], 8.0);
assert_eq!(instance.params.values[3], 16.0);
assert_eq!(instance.params.values[4], 0.0, "model fallback must not set the instance given bit");

instance.apply_parameters(&[
    Assignment::new("dual", 9.0, Origin::Instance),
]).unwrap();
assert_eq!(instance.params.values[2], 9.0);
assert_eq!(instance.params.values[3], 16.0, "model default must keep using dual-scope model storage");
assert_eq!(instance.params.values[4], 1.0, "instance override must set the instance given bit");

instance.apply_parameters(&[
    Assignment::new("dual", 11.0, Origin::ModelCard),
]).unwrap();
assert_eq!(instance.params.values[2], 9.0, "instance override must outrank the model fallback");
assert_eq!(instance.params.values[3], 22.0, "model default must follow the changed model fallback");
assert_eq!(instance.params.values[4], 1.0);
"#;
    run_generated_main("scoped parameter assignments", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated parameter scope probe failed:\n{report}"));
}

#[test]
fn generated_dual_scope_dependent_defaults_follow_xyce_instance_finalization() {
    let (state, stamp, noise) = generated_parts(
        r#"
module dual_dependent_defaults(p, n);
    inout p, n;
    electrical p, n;
    (* type = "instance", xyceAlsoModel = "yes" *) parameter real base = 2.0;
    (* type = "instance", xyceAlsoModel = "yes" *) parameter real dependent = base * 3.0;
    (* type = "instance" *) parameter real dependent_seen_given = $param_given(dependent) ? 1.0 : 0.0;
    (* type = "instance", xyceAlsoModel = "yes" *) parameter real bounded = base * 2.0 from [0.0:10.0];
    analog I(p, n) <+ (dependent + dependent_seen_given + bounded) * V(p, n);
endmodule
"#,
        "dual-scope dependent defaults",
    );
    let body = r#"
use runtime::{GeneratedParameterAssignment as Assignment, GeneratedParameterOrigin as Origin};
let mut instance = device::state::Instance::new(&[0, 1]);

instance.apply_parameters(&[
    Assignment::new("base", 4.0, Origin::ModelCard),
    Assignment::new("dependent", 99.0, Origin::ModelCard),
]).unwrap();
assert_eq!(instance.params.values[0], 4.0);
assert_eq!(instance.params.values[1], 12.0, "an unset dual instance parameter must recompute its dependent default after model fallback");
assert_eq!(instance.params.values[2], 0.0, "model fallback must not set the instance given bit");
assert_eq!(instance.params.values[3], 8.0);

instance.apply_parameters(&[
    Assignment::new("base", 5.0, Origin::Instance),
]).unwrap();
assert_eq!(instance.params.values[0], 5.0);
assert_eq!(instance.params.values[1], 15.0, "a dependent dual default must use the effective instance value");
assert_eq!(instance.params.values[2], 0.0);
assert_eq!(instance.params.values[3], 10.0);

let before_values = instance.params.values;
let before_given = instance.param_given.clone();
let error = instance.apply_parameters(&[
    Assignment::new("base", 6.0, Origin::Instance),
]).unwrap_err();
assert!(error.contains("bounded"), "{error}");
assert_eq!(instance.params.values, before_values, "failed dependent-default validation must roll back every effective value");
assert_eq!(instance.param_given, before_given, "failed dependent-default validation must roll back every instance given bit");

instance.apply_parameters(&[
    Assignment::new("dependent", 21.0, Origin::Instance),
]).unwrap();
assert_eq!(instance.params.values[1], 21.0, "an explicit instance assignment must outrank the dependent default");
assert_eq!(instance.params.values[2], 1.0, "an explicit instance assignment must set the instance given bit");

instance.apply_parameters(&[
    Assignment::new("dependent", 30.0, Origin::ModelCard),
]).unwrap();
assert_eq!(instance.params.values[1], 21.0, "a later model-card assignment must not replace an explicit instance assignment");
assert_eq!(instance.params.values[2], 1.0);
"#;
    run_generated_main(
        "dual-scope dependent defaults",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("generated dual-scope dependent defaults failed:\n{report}"));
}

#[test]
fn generated_model_ranges_validate_against_completed_instance_geometry() {
    let (state, stamp, noise) = generated_parts(
        r#"
module cross_scope_range(p, n);
    inout p, n;
    electrical p, n;
    parameter real overlap = 1.0 from [0.0:length];
    parameter real scale = 1.0;
    (* type = "instance" *) parameter real length = 1.0;
    (* type = "instance" *) parameter real derived = scale * 2.0 from [0.0:3.0];
    analog I(p, n) <+ overlap * length * derived * V(p, n);
endmodule
"#,
        "cross scope parameter range",
    );
    let body = r#"
use runtime::{GeneratedParameterAssignment as Assignment, GeneratedParameterOrigin as Origin};
let mut instance = device::state::Instance::new(&[0, 1]);
instance.apply_parameters(&[
    Assignment::new("overlap", 2.0, Origin::ModelCard),
    Assignment::new("scale", 2.0, Origin::ModelCard),
    Assignment::new("length", 3.0, Origin::Instance),
    Assignment::new("derived", 1.0, Origin::Instance),
]).unwrap();
assert_eq!(instance.params.values, [2.0, 2.0, 3.0, 1.0]);

let before = instance.params.values;
let error = instance.apply_parameters(&[
    Assignment::new("overlap", 4.0, Origin::ModelCard),
    Assignment::new("length", 3.0, Origin::Instance),
]).unwrap_err();
assert!(error.contains("overlap"), "{error}");
assert_eq!(instance.params.values, before, "failed cross-scope validation must roll back atomically");
"#;
    run_generated_main("cross scope parameter range", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("cross-scope range probe failed:\n{report}"));
}

#[test]
fn zero_parameter_generated_state_defines_empty_scope_metadata() {
    let (state, stamp, noise) = generated_parts(
        r#"
module parameterless(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 2.0 * V(p, n);
endmodule
"#,
        "parameterless state",
    );
    assert!(state.contains("PARAMETER_MODEL_FLAGS: [bool; 0] = []"));
    assert!(state.contains("PARAMETER_DUAL_SCOPE_FLAGS: [bool; 0] = []"));
    compile("parameterless state", &state, &stamp, &noise)
        .unwrap_or_else(|report| panic!("zero-parameter generated state failed:\n{report}"));
}

#[test]
fn generated_static_potential_guard_opens_and_closes_one_branch() {
    let (state, stamp, noise) = generated_parts(
        r#"
module guarded_short(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enabled = 0;
    real guard;
    analog begin
        guard = enabled;
        if (guard > 0)
            V(p, n) <+ 0.0;
    end
endmodule
"#,
        "guarded short",
    );
    assert!(stamp.contains("stamp_inactive_potential_branch_local"));
    let body = r#"
fn sample(enabled: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2]);
    instance.set_parameter("enabled", enabled).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [0.75, 0.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let open = sample(0.0);
assert_eq!(open[0], 0.0, "open structural coupling: {open:?}");
assert_eq!(open[1], 1.0, "open branch identity: {open:?}");
assert_eq!(open[8], 1.0, "leader ordinal must be pinned: {open:?}");
let closed = sample(1.0);
assert_eq!(closed[0], 1.0, "closed structural coupling: {closed:?}");
assert_eq!(closed[1], 0.0, "closed branch must not be pinned: {closed:?}");
assert_eq!(closed[7], 1.0, "leader ordinal must carry topology: {closed:?}");
"#;
    run_generated_main("guarded short activation", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated guarded short failed:\n{report}"));
}

#[test]
fn generated_static_guard_uses_the_reaching_definition() {
    let (state, stamp, noise) = generated_parts(
        r#"
module captured_mode(p, n, sense);
    inout p, n, sense;
    electrical p, n, sense;
    parameter integer selector = 0;
    real scratch, mode;
    analog begin
        scratch = selector;
        mode = scratch;
        scratch = V(sense);
        if (mode == 2)
            V(p, n) <+ 0.0;
    end
endmodule
"#,
        "reaching-definition static guard",
    );
    let body = r#"
fn sample(selector: f64, sense: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1, 2]);
    instance.set_branch_indices(&[3]);
    instance.set_parameter("selector", selector).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [0.0, 0.0, sense, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let open = sample(0.0, 2.0);
assert_eq!(open[0], 0.0, "later dynamic scratch reuse must not close topology: {open:?}");
assert_eq!(open[1], 1.0, "inactive branch must be pinned: {open:?}");
let closed = sample(2.0, -3.0);
assert_eq!(closed[0], 1.0, "captured static mode must close topology: {closed:?}");
assert_eq!(closed[1], 0.0, "active branch must not be pinned: {closed:?}");
"#;
    run_generated_main(
        "reaching-definition static guard",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("reaching-definition guard probe failed:\n{report}"));
}

#[test]
fn generated_newton_value_uses_parameter_before_shadowing_block_local() {
    let (state, stamp, noise) = generated_parts(
        r#"
module parameter_shadow(p, n);
    inout p, n;
    electrical p, n;
    parameter real scale = 0.5;
    real captured;
    analog begin
        captured = scale;
        begin : load
            real scale;
            scale = 0.25;
            I(p, n) <+ (captured + 10.0 * scale) * V(p, n);
        end
    end
endmodule
"#,
        "parameter shadowing block local",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
let voltages = [2.0, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut sink = [0.0; 10];
let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
instance.stamp(&ctx, &mut stamper);
assert_eq!(sink[0], 6.0, "the parameter and shadowing local must remain distinct: {sink:?}");
"#;
    run_generated_main(
        "parameter shadowing block local",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("parameter-shadowing probe failed:\n{report}"));
}

#[test]
fn generated_guard_uses_a_later_dynamic_redefinition() {
    let (state, stamp, noise) = generated_parts(
        r#"
module redefined_mode(p, n, sense);
    inout p, n, sense;
    electrical p, n, sense;
    parameter integer selector = 0;
    real mode;
    analog begin
        mode = selector;
        mode = V(sense);
        if (mode > 0.0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "dynamic redefinition guard",
    );
    let body = r#"
fn sample(selector: f64, sense: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1, 2]);
    instance.set_branch_indices(&[3]);
    instance.set_parameter("selector", selector).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [0.0, 0.0, sense, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let runtime_false = sample(2.0, -1.0);
assert_eq!(runtime_false[0], 1.0, "dynamic guard must retain topology: {runtime_false:?}");
assert_eq!(runtime_false[1], 0.0, "dynamic branch must not be pinned: {runtime_false:?}");
assert_eq!(runtime_false[2], 0.0, "untaken dynamic contribution is zero: {runtime_false:?}");
let runtime_true = sample(0.0, 1.0);
assert_eq!(runtime_true[0], 1.0, "dynamic topology must remain fixed: {runtime_true:?}");
assert_eq!(runtime_true[2], 2.0, "taken dynamic contribution: {runtime_true:?}");
"#;
    run_generated_main("dynamic redefinition guard", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("dynamic-redefinition guard probe failed:\n{report}"));
}

#[test]
fn generated_temperature_and_analysis_guards_are_topology_static() {
    let (state, stamp, noise) = generated_parts(
        r#"
module environment_guards(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if ($temperature > 350.0)
            V(p, n) <+ 1.0;
        if (analysis("ac"))
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "environment topology guards",
    );
    let body = r#"
fn sample(temperature: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    assert_eq!(device::state::Instance::BRANCH_COUNT, 1);
instance.set_branch_indices(&[2]);
    instance.finalize_parameters().unwrap();
    let voltages = [0.0, 0.0, 0.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let open = sample(300.15);
assert_eq!(open[0], 0.0, "both environment guards false open: {open:?}");
assert_eq!(open[1], 1.0, "the inactive physical current is pinned: {open:?}");
let hot = sample(400.0);
assert_eq!(hot[0], 1.0, "temperature guard closes topology: {hot:?}");
assert_eq!(hot[1], 0.0, "active branch has no unused rows: {hot:?}");
assert_eq!(hot[2], 1.0, "temperature contribution: {hot:?}");
let ac = sample(123.0);
assert_eq!(ac[0], 1.0, "analysis guard closes topology: {ac:?}");
assert_eq!(ac[2], 2.0, "analysis contribution: {ac:?}");
"#;
    run_generated_main("environment topology guards", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("environment guard probe failed:\n{report}"));
}

#[test]
fn generated_unguarded_and_guarded_potentials_sum_on_one_branch() {
    let (state, stamp, noise) = generated_parts(
        r#"
module mixed_sources(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enabled = 0;
    analog begin
        V(p, n) <+ 1.0;
        if (enabled > 0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "unguarded and guarded potentials",
    );
    let body = r#"
fn sample(enabled: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    assert_eq!(device::state::Instance::BRANCH_COUNT, 1);
instance.set_branch_indices(&[2]);
    instance.set_parameter("enabled", enabled).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [0.0; 4];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let base = sample(0.0);
assert_eq!(base[0], 1.0, "unguarded source owns topology: {base:?}");
assert_eq!(base[1], 0.0, "active branch has no unused rows: {base:?}");
assert_eq!(base[2], 1.0, "unguarded residual: {base:?}");
let summed = sample(1.0);
assert_eq!(summed[0], 1.0, "physical branch still couples once: {summed:?}");
assert_eq!(summed[2], 3.0, "contributions sum: {summed:?}");
"#;
    run_generated_main(
        "unguarded and guarded potentials",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("mixed potential probe failed:\n{report}"));
}

#[test]
fn generated_one_terminal_port_flow_reads_leader_once() {
    let (state, stamp, noise) = generated_parts(
        r#"
module port_flow(p, n, out);
    inout p, n, out;
    electrical p, n, out;
    parameter integer mode = 0;
    analog begin
        if (mode == 1)
            V(p, n) <+ 0.0;
        if (mode == 2)
            V(p, n) <+ 0.0;
        I(out, n) <+ I(<p>);
    end
endmodule
"#,
        "one-terminal physical port flow",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1, 2]);
assert_eq!(device::state::Instance::BRANCH_COUNT, 1);
instance.set_branch_indices(&[3]);
instance.set_parameter("mode", 2.0).unwrap();
instance.finalize_parameters().unwrap();
// The first contribution is inactive but its branch is the physical leader.
// A later active duplicate must not redirect or double-count I(<p>).
let voltages = [0.0, 0.0, 0.0, 0.5, 40.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut sink = [0.0; 10];
let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
instance.stamp(&ctx, &mut stamper);
assert_eq!(sink[0], 1.5, "one topology call plus 0.5 A current: {sink:?}");
assert_eq!(sink[9], 0.5, "I(<p>) reads the physical leader exactly once: {sink:?}");
assert_eq!(sink[1], 0.0, "active branch has no unused rows: {sink:?}");
"#;
    run_generated_main(
        "one terminal physical port flow",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("one-terminal port-flow probe failed:\n{report}"));
}

#[test]
fn generated_potential_contributions_share_one_physical_branch() {
    let (state, stamp, noise) = generated_parts(
        r#"
module grouped_sources(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 0;
    analog begin
        if (mode == 1)
            V(p, n) <+ 1.0;
        if (mode == 2)
            V(n, p) <+ 2.0;
        if (mode == 3) begin
            V(p, n) <+ 4.0;
            V(n, p) <+ 1.5;
        end
        if (mode == 4)
            V(p, n) <+ 3.0 * I(p, n);
    end
endmodule
"#,
        "grouped potential sources",
    );
    let body = r#"
fn sample(mode: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    assert_eq!(device::state::Instance::BRANCH_COUNT, 1);
instance.set_branch_indices(&[2]);
    instance.set_parameter("mode", mode).unwrap();
    instance.finalize_parameters().unwrap();
    // The shared physical current (solver index 2) is 0.5 A. Unrelated
    // solver values are large so a wrong I(p,n) mapping is obvious.
    let voltages = [0.25, 0.0, 0.5, 10.0, 20.0, 30.0, 40.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let open = sample(0.0);
assert_eq!(open[0], 0.0, "all inactive must stay open: {open:?}");
assert_eq!(open[1], 1.0, "the inactive physical current is pinned: {open:?}");
assert_eq!(open[8], 1.0, "physical current is pinned exactly once: {open:?}");

let forward = sample(1.0);
assert_eq!(forward[0], 1.0, "one physical coupling: {forward:?}");
assert_eq!(forward[1], 0.0, "active branch has no unused rows: {forward:?}");
assert_eq!(forward[2], 1.0, "forward residual: {forward:?}");
assert_eq!(forward[5], 1.0, "residual targets leader branch: {forward:?}");

let reversed = sample(2.0);
assert_eq!(reversed[0], 1.0, "one reversed physical coupling: {reversed:?}");
assert_eq!(reversed[2], -2.0, "reversed residual must be negated: {reversed:?}");
assert_eq!(reversed[5], 1.0, "reversed residual targets leader: {reversed:?}");

let summed = sample(3.0);
assert_eq!(summed[0], 1.0, "simultaneous contributions still couple once: {summed:?}");
assert_eq!(summed[1], 0.0, "active branch has no unused rows: {summed:?}");
assert_eq!(summed[2], 2.5, "4.0 + reversed 1.5 must sum: {summed:?}");
assert_eq!(summed[5], 2.0, "both residuals target leader ordinal zero: {summed:?}");

let flow = sample(4.0);
assert_eq!(flow[2], 1.5, "I(p,n) must read leader flow 0.5 A: {flow:?}");
assert_eq!(flow[4], 3.0, "flow derivative: {flow:?}");
assert_eq!(flow[6], 1.0, "flow derivative must name leader ordinal zero: {flow:?}");
"#;
    run_generated_main("grouped potential sources", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated grouped sources failed:\n{report}"));
}

#[test]
fn generated_potential_noise_uses_the_physical_branch_leader() {
    let (state, stamp, noise) = generated_parts(
        r#"
module grouped_potential_noise(p, n);
    inout p, n;
    electrical p, n;
    parameter integer reverse_enabled = 1;
    analog begin
        V(p, n) <+ white_noise(abs(I(p, n)) + 1.0, "forward");
        if (reverse_enabled > 0)
            V(n, p) <+ white_noise(abs(I(n, p)) + 2.0, "reverse");
    end
endmodule
"#,
        "grouped potential noise",
    );
    let body = r#"
#[derive(Default)]
struct Capture(Vec<(bool, f64)>);
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self, _index: usize, value: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool {
        self.0.push((value.active, value.psd));
        true
    }
}

assert_eq!(device::noise::NOISE_SOURCES.len(), 2);
assert_eq!(device::noise::NOISE_SOURCES[0].branch_ordinal, Some(0));
assert_eq!(device::noise::NOISE_SOURCES[1].branch_ordinal, Some(0),
    "reversed duplicate descriptor must name the physical leader");

let mut instance = device::state::Instance::new(&[0, 1]);
assert_eq!(device::state::Instance::BRANCH_COUNT, 1);
instance.set_branch_indices(&[2]);
instance.finalize_parameters().unwrap();
// The physical current is 0.5 A; an unrelated solver value is 40 A.
// Both I(p,n) and reversed I(n,p) must read ordinal zero exactly once.
let voltages = [0.0, 0.0, 0.5, 40.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut capture = Capture::default();
instance.evaluate_noise_sources(&ctx, &mut capture).unwrap();
assert_eq!(capture.0, vec![(true, 1.5), (true, 2.5)]);

instance.set_parameter("reverse_enabled", 0.0).unwrap();
instance.finalize_parameters().unwrap();
let mut disabled = Capture::default();
instance.evaluate_noise_sources(&ctx, &mut disabled).unwrap();
assert_eq!(disabled.0, vec![(true, 1.5), (false, 0.0)]);
"#;
    run_generated_main("grouped potential noise", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("grouped potential-noise probe failed:\n{report}"));
}

#[test]
fn generated_noise_powers_freeze_dynamic_values_without_touching_history() {
    for (index, (dynamic, dc_value)) in [
        ("ddt(V(p,n))", 0.0),
        ("idt(V(p,n),3.0)", 3.0),
        ("ddt(idt(V(p,n),3.0))", 0.0),
        ("idt(ddt(V(p,n)),3.0)", 3.0),
        ("last_crossing(V(p,n),1)", -1.0),
    ]
    .into_iter()
    .enumerate()
    {
        let source = format!(
            "module dynamic_noise_power(p,n); inout p,n; electrical p,n; real d; analog begin d={dynamic}; I(p,n)<+V(p,n)+d+white_noise(limexp(V(p,n)+d),\"input\"); end endmodule"
        );
        let name = format!("dynamic noise power {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(&name, &state, &stamp, &noise, &format!(r#"
#[derive(Default)]
struct Capture(Vec<f64>);
impl runtime::GeneratedNoiseVisitor for Capture {{
    fn visit(&mut self, _: usize, source: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool {{
        assert!(source.active);
        self.0.push(source.psd);
        true
    }}
}}
impl runtime::GeneratedNoiseProcessVisitor for Capture {{
    fn visit_process(&mut self, _: usize, process: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {{
        assert!(process.active);
        self.0.push(process.psd);
        true
    }}
}}
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let ctx=runtime::GeneratedEvalContext {{ voltages:&[2.0,0.0], temperature:300.15 }};
let history=instance.capture_rollback_state();
for frequency in [0.0,1.0,1e6] {{
    let mut sources=Capture::default();
    instance.evaluate_noise_sources(&ctx,&mut sources).unwrap();
    let mut processes=Capture::default();
    instance.evaluate_noise_processes_at_frequency(&ctx,frequency,&mut processes).unwrap();
    assert_eq!(sources.0.len(),1);
    assert_eq!(processes.0.len(),1);
    let expected=(2.0_f64+{dc_value:e}).exp();
    for psd in sources.0.into_iter().chain(processes.0) {{
        assert!((psd/expected-1.0).abs()<=8.0*f64::EPSILON);
    }}
    assert!(!ctx.evaluation_failed());
    assert_eq!(instance.capture_rollback_state(),history);
}}
"#)).unwrap_or_else(|report| panic!("{name}: {report}"));
    }
}

#[test]
fn generated_unused_noise_processes_import_and_evaluate_math_helpers() {
    for (index, (power, expected)) in [
        ("limexp(V(p,n))", "2.0_f64.exp()"),
        ("limexp(-V(p,n))", "(-2.0_f64).exp()"),
        ("limexp(45.0*V(p,n))", "80.0_f64.exp()*11.0"),
        ("limexp(last_crossing(V(p,n),1))", "(-1.0_f64).exp()"),
    ]
    .into_iter()
    .enumerate()
    {
        // An unused process has no source-wise injection plan. Its grouped
        // evaluator still computes the power, including any runtime helpers.
        let source = format!(
            "module unused_noise(p,n); inout p,n; electrical p,n; real d; analog begin d=white_noise({power},\"input\"); I(p,n)<+V(p,n); end endmodule"
        );
        let name = format!("unused noise math {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(&name, &state, &stamp, &noise, &format!(r#"
#[derive(Default)]
struct Capture(Vec<f64>);
impl runtime::GeneratedNoiseProcessVisitor for Capture {{
    fn visit_process(&mut self, _: usize, process: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {{
        assert!(process.active);
        assert!(process.injections.is_empty());
        self.0.push(process.psd);
        true
    }}
}}
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let ctx=runtime::GeneratedEvalContext {{ voltages:&[2.0,0.0], temperature:300.15 }};
let mut capture=Capture::default();
instance.evaluate_noise_processes_at_frequency(&ctx,1.0,&mut capture).unwrap();
assert_eq!(capture.0.len(),1);
assert!((capture.0[0]/({expected})-1.0).abs() <= 8.0*f64::EPSILON);
assert!(!ctx.evaluation_failed());
"#)).unwrap_or_else(|report| panic!("{name}: {report}"));
    }
}

#[test]
fn generated_grouped_noise_preserves_frequency_and_multiplicity_range() {
    for (kind, current) in [("I", true), ("V", false)] {
        let source = format!(
            "module ranged_noise(p,n); inout p,n; electrical p,n; parameter real gain=1.0; real source; analog begin source=white_noise(1.0,\"n\"); {kind}(p,n)<+0.25*source+gain*ddt(source); end endmodule"
        );
        let name = format!("ranged {kind} noise");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(&name, &state, &stamp, &noise, &format!(r#"
struct Capture(runtime::GeneratedNoiseComplex);
impl runtime::GeneratedNoiseProcessVisitor for Capture {{
    fn visit_process(&mut self, _: usize, process: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {{
        assert!(process.active);
        assert_eq!(process.psd, 1.0);
        assert_eq!(process.injections.len(), 1);
        self.0=process.injections[0].gain;
        true
    }}
}}
for (gain, scale, frequency) in [
    (1e200_f64,1e-100_f64,1e200_f64),
    (1e-200,1e100,1e-200),
    (0.1,1.0,1e308),
    (1e308,1.0,f64::from_bits(1)),
    (0.0,1.0,1e308),
] {{
    let mut instance=device::state::Instance::new(&[0,1]);
    if !{current} {{ instance.set_branch_indices(&[2]); }}
    instance.set_parameter("gain",gain).unwrap();
    let multiplicity=if {current} {{ scale*scale }} else {{ scale.recip()*scale.recip() }};
    instance.set_multiplicity(multiplicity).unwrap();
    instance.finalize_parameters().unwrap();
    let ctx=runtime::GeneratedEvalContext {{ voltages:&[0.0;3], temperature:300.15 }};
    let mut capture=Capture(runtime::GeneratedNoiseComplex::default());
    instance.evaluate_noise_processes_at_frequency(&ctx,frequency,&mut capture).unwrap();
    let sign=if {current} {{ -1.0 }} else {{ 1.0 }};
    let expected=((gain*scale)*frequency)*core::f64::consts::TAU*sign;
    assert!((capture.0.re/(0.25*scale*sign)-1.0).abs()<=8.0*f64::EPSILON);
    if expected==0.0 {{ assert_eq!(capture.0.im,0.0); }}
    else {{ assert!((capture.0.im/expected-1.0).abs()<=8.0*f64::EPSILON,"{{gain}}, {{scale}}, {{frequency}}: {{:?}}, expected {{expected}}",capture.0); }}
    assert!(!ctx.evaluation_failed());
}}
"#)).unwrap_or_else(|report| panic!("{name}: {report}"));
    }
}

#[test]
fn generated_noise_skips_inactive_integrator_frequency_terms() {
    for (index, route) in conditional_integrator_routes().into_iter().enumerate() {
        let source = format!(
            "module conditional_integrator_noise(p,n); inout p,n; electrical p,n; parameter integer enabled=0; parameter real gain=1; real source,routed; integer k; analog begin source=white_noise(1.0,\"n\"); {route} end endmodule"
        );
        let name = format!("conditional integrator noise {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(&name, &state, &stamp, &noise, r#"
#[derive(Default)]
struct Capture(Vec<runtime::GeneratedNoiseComplex>);
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self, _: usize, process: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {
        assert!(process.active, "the process is outside the routing condition");
        let mut gain=runtime::GeneratedNoiseComplex::default();
        for injection in process.injections { gain.re+=injection.gain.re; gain.im+=injection.gain.im; }
        self.0.push(gain);
        true
    }
}
let mut instance=device::state::Instance::new(&[0,1]);
for enabled in [0.0,1.0,0.0] {
    instance.set_parameter("enabled",enabled).unwrap();
    for gain in [1.0,0.0] {
        instance.set_parameter("gain",gain).unwrap();
        instance.finalize_parameters().unwrap();
        for frequency in [0.0,1.0] {
            runtime::clear_evaluation_error();
            let ctx=runtime::GeneratedEvalContext {voltages:&[1.0,0.0],temperature:300.15};
            let mut capture=Capture::default();
            let result=instance.evaluate_noise_processes_at_frequency(&ctx,frequency,&mut capture);
            if enabled>0.0 && frequency==0.0 {
                assert!(result.is_err(), "an executed integral remains singular even with a zero coefficient");
            } else {
                result.unwrap();
                assert_eq!(capture.0.len(),1);
                let expected=if enabled>0.0 {(0.0,gain/std::f64::consts::TAU)} else {(-1.0,0.0)};
                assert_eq!((capture.0[0].re,capture.0[0].im),expected);
                assert!(!ctx.evaluation_failed());
            }
        }
    }
}
"#).unwrap_or_else(|report| panic!("{report}"));
    }
}

fn conditional_integrator_routes() -> [&'static str; 4] {
    [
        "if(enabled>0) I(p,n)<+gain*idt(source,0.0); else I(p,n)<+source;",
        "if(enabled>0) routed=gain*idt(source,0.0); else routed=source; I(p,n)<+routed;",
        "routed=enabled>0 ? gain*idt(source,0.0) : source; I(p,n)<+routed;",
        "if(enabled>0) routed=gain*idt(source,0.0); else routed=source; for(k=0;k<enabled;k=k+1) source=routed; I(p,n)<+source;",
    ]
}

#[test]
fn generated_ac_skips_inactive_integrator_frequency_terms() {
    for (index, route) in conditional_integrator_routes().into_iter().enumerate() {
        let source = format!(
            "module conditional_integrator_ac(p,n); inout p,n; electrical p,n; parameter integer enabled=0; parameter real gain=1; real source,routed; integer k; analog begin source=V(p,n); {route} end endmodule"
        );
        let name = format!("conditional integrator ac {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(&name, &state, &stamp, &noise, r#"
let mut instance=device::state::Instance::new(&[0,1]);
runtime::set_dynamic_operators_enabled(false);
for enabled in [0.0,1.0,0.0] {
    instance.set_parameter("enabled",enabled).unwrap();
    for gain in [1.0,0.0] {
        instance.set_parameter("gain",gain).unwrap();
        instance.finalize_parameters().unwrap();
        for omega in [0.0_f64,1.0] {
            runtime::clear_evaluation_error();
            let ctx=runtime::GeneratedEvalContext {voltages:&[1.0,0.0],temperature:300.15};
            let mut real=[0.0;12];
            instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut real)});
            let history=instance.capture_rollback_state();
            runtime::FREQUENCY_OMEGA.store(omega.to_bits(),std::sync::atomic::Ordering::SeqCst);
            let mut reactive=[0.0;6];
            instance.stamp_reactive(&ctx,&mut runtime::GeneratedReactiveStamper {sink:Some(&mut reactive)});
            assert_eq!(ctx.evaluation_failed(), enabled>0.0 && omega==0.0);
            if omega>0.0 { assert_eq!(reactive[0], if enabled>0.0 {-gain} else {0.0}); }
            assert_eq!(instance.capture_rollback_state(),history);
        }
    }
}
"#).unwrap_or_else(|report| panic!("{report}"));
    }
}

#[test]
fn generated_implicit_integrator_stamps_preserve_guards_and_noise() {
    for (index, route) in [
        "if(enabled) y=idt(e); else y=0.25;",
        "y=enabled ? idt(e) : 0.25;",
        "case(enabled) 0:y=0.25; default:y=idt(e); endcase",
    ]
    .into_iter()
    .enumerate()
    {
        let source = format!(
            "module implicit_integrator_generated(p,n); inout p,n; electrical p,n; parameter integer enabled=1; real e,y; analog begin e=1000*(V(p)-V(n))+white_noise(1,\"input\"); {route} e=17; V(n)<+y; end endmodule"
        );
        let name = format!("implicit integrator generated {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(&name,&state,&stamp,&noise,r#"
struct Capture(Vec<(f64,f64)>);
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self, _:usize, process:runtime::GeneratedNoiseProcessEvaluationRef<'_>)->bool {
        assert_eq!(process.psd,1.0);
        self.0.extend(process.injections.iter().map(|injection| (injection.gain.re,injection.gain.im)));
        true
    }
}
assert_eq!(device::state::Instance::INTERNAL_STATE_NODES, &[0]);
let mut instance=device::state::Instance::new(&[0,1,2]);
instance.set_branch_indices(&[3]);
runtime::set_dynamic_operators_enabled(false);
for enabled in [1.0,0.0,1.0] {
    instance.set_parameter("enabled",enabled).unwrap();
    instance.finalize_parameters().unwrap();
    let ctx=runtime::GeneratedEvalContext {voltages:&[2.0,1.5,9.0,0.0],temperature:300.15};
    let mut real=[0.0;12];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut real)});
    assert_eq!(real[2],if enabled>0.0 {9.0} else {0.25});
    assert_eq!(real[9],if enabled>0.0 {-500.0} else {9.0});
    assert_eq!(real[10],if enabled>0.0 {-1000.0} else {0.0});
    assert_eq!(real[11],if enabled>0.0 {1000.0} else {0.0});
    let history=instance.capture_rollback_state();
    for frequency in [0.0,1.0,1e6] {
        let mut capture=Capture(Vec::new());
        instance.evaluate_noise_processes_at_frequency(&ctx,frequency,&mut capture).unwrap();
        assert_eq!(capture.0.iter().map(|gain|gain.0).sum::<f64>(),enabled);
        assert!(capture.0.iter().all(|gain|gain.1==0.0));
        assert!(!ctx.evaluation_failed());
        assert_eq!(instance.capture_rollback_state(),history);
    }
}
"#).unwrap_or_else(|report|panic!("{name}: {report}"));
    }
}

#[test]
fn generated_noise_integrator_activity_is_specific_to_each_packed_lane() {
    let source = "module lane_integrator_noise(p,n); inout p,n; electrical p,n; real a,b; analog begin a=white_noise(1.0,\"a\"); b=white_noise(1.0,\"b\"); I(p,n)<+a+idt(b,0.0); end endmodule";
    let (state, stamp, noise) = generated_parts(source, "lane integrator noise");
    run_generated_main("lane integrator noise", &state, &stamp, &noise, r#"
struct Capture { first_only:bool, gains:Vec<runtime::GeneratedNoiseComplex> }
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self, _: usize, process: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {
        assert!(process.active);
        assert_eq!(process.injections.len(),1);
        self.gains.push(process.injections[0].gain);
        !self.first_only
    }
}

let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for (frequency,first_only) in [(0.0,true),(0.0,false),(1.0,false),(0.0,true)] {
    runtime::clear_evaluation_error();
    let ctx=runtime::GeneratedEvalContext {voltages:&[1.0,0.0],temperature:300.15};
    let mut capture=Capture {first_only,gains:Vec::new()};
    let result=instance.evaluate_noise_processes_at_frequency(&ctx,frequency,&mut capture);
    assert_eq!(result.is_err(),frequency==0.0 && !first_only);
    assert_eq!((capture.gains[0].re,capture.gains[0].im),(-1.0,0.0));
    if frequency>0.0 {
        assert_eq!(capture.gains.len(),2);
        assert_eq!((capture.gains[1].re,capture.gains[1].im),(0.0,1.0/std::f64::consts::TAU));
    }
}
"#).unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_noise_integrator_activity_survives_frequency_products() {
    let source = "module product_integrator_noise(p,n); inout p,n; electrical p,n; parameter integer enabled=0; parameter real gain=1; real a,b,routed; analog begin a=white_noise(1.0,\"a\"); b=white_noise(1.0,\"b\"); if(enabled>0) routed=gain*idt(a,0.0); else routed=a; I(p,n)<+V(p,n)*(routed+ddt(routed)+2*b+2*ddt(b))/(1+V(p,n)); end endmodule";
    let (state, stamp, noise) = generated_parts(source, "product integrator noise");
    run_generated_main("product integrator noise", &state, &stamp, &noise, r#"
#[derive(Default)]
struct Capture(Vec<runtime::GeneratedNoiseComplex>);
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self, _: usize, process: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {
        assert!(process.active);
        assert_eq!(process.injections.len(),1);
        self.0.push(process.injections[0].gain);
        true
    }
}
let mut instance=device::state::Instance::new(&[0,1]);
for enabled in [0.0,1.0,0.0] {
    instance.set_parameter("enabled",enabled).unwrap();
    for gain in [1.0,0.0] {
        instance.set_parameter("gain",gain).unwrap();
        instance.finalize_parameters().unwrap();
        for frequency in [0.0,0.25,2.0] {
            runtime::clear_evaluation_error();
            let ctx=runtime::GeneratedEvalContext {voltages:&[1.0,0.0],temperature:300.15};
            let mut capture=Capture::default();
            let result=instance.evaluate_noise_processes_at_frequency(&ctx,frequency,&mut capture);
            if enabled>0.0 && frequency==0.0 { assert!(result.is_err()); continue; }
            result.unwrap();
            assert_eq!(capture.0.len(),2);
            let omega=std::f64::consts::TAU*frequency;
            let expected=if enabled>0.0 {[-gain/2.0,gain/(2.0*omega)]} else {[-0.5,-0.5*omega]};
            for (actual,expected) in [(capture.0[0].re,expected[0]),(capture.0[0].im,expected[1]),(capture.0[1].re,-1.0),(capture.0[1].im,-omega)] {
                assert!((actual-expected).abs()<=1e-12,"{actual} != {expected}");
            }
        }
    }
}
"#).unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_grouped_noise_propagates_cancellation() {
    #[derive(Default)]
    struct CancelDuringNoise {
        state_emitted: AtomicBool,
        polls: AtomicUsize,
    }
    impl PipelineControl for CancelDuringNoise {
        fn is_cancelled(&self) -> bool {
            self.state_emitted.load(Ordering::Relaxed)
                && self.polls.fetch_add(1, Ordering::Relaxed) > 0
        }
        fn phase_completed(
            &self,
            timing: rspice_veriloga::PhaseTiming,
            _: &rspice_veriloga::PipelineMetrics,
        ) {
            if timing.phase == PipelinePhase::StateEmission {
                self.state_emitted.store(true, Ordering::Relaxed);
            }
        }
    }
    let source = "module cancel_noise(p,n); inout p,n; electrical p,n; analog I(p,n)<+sin(V(p,n)+ddt(white_noise(1.0,\"n\"))); endmodule";
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap();
    let control = CancelDuringNoise::default();
    let error = RustTranspiler::new(options())
        .transpile_measured_with_control(&artifact, &control)
        .expect_err("cancellation inside grouped noise must propagate");
    assert_eq!(error.kind, RustBackendErrorKind::Cancelled);
    assert!(control.polls.load(Ordering::Relaxed) >= 2);
}

#[test]
fn generated_nonlinear_noise_preserves_primal_domain_errors() {
    for (index, (expression, invalid_at_zero)) in [
        ("sqrt(V(p,n)+source)", true),
        ("ln(V(p,n)+source)", true),
        ("ddt(sqrt(V(p,n))+source)", false),
        ("ddt(sqrt(V(p,n))+sin(source))", false),
        ("idt(ln(V(p,n))+source,0.25)", true),
    ]
    .into_iter()
    .enumerate()
    {
        let source = format!(
            "module noise_domain(p,n); inout p,n; electrical p,n; parameter integer enabled=1; real source; analog if(enabled>0) begin source=white_noise(1.0,\"n\"); I(p,n)<+{expression}; end endmodule"
        );
        let name = format!("noise domain {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(&name, &state, &stamp, &noise, &format!(r#"
struct Capture;
impl runtime::GeneratedNoiseProcessVisitor for Capture {{
    fn visit_process(&mut self, _: usize, _: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {{ true }}
}}
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for (bias, invalid) in [(-1.0,true),(1.0,false),(0.0,{invalid_at_zero}),(2.0,false)] {{
    runtime::clear_evaluation_error();
    let ctx=runtime::GeneratedEvalContext {{voltages:&[bias,0.0],temperature:300.15}};
    let result=instance.evaluate_noise_processes_at_frequency(&ctx,1.0,&mut Capture);
    assert_eq!(result.is_err(),invalid,"bias={{bias}}: {{result:?}}");
}}
instance.set_parameter("enabled",0.0).unwrap();
instance.finalize_parameters().unwrap();
runtime::clear_evaluation_error();
let ctx=runtime::GeneratedEvalContext {{voltages:&[-1.0,0.0],temperature:300.15}};
instance.evaluate_noise_processes_at_frequency(&ctx,0.0,&mut Capture).unwrap();
assert!(!ctx.evaluation_failed());
"#)).unwrap_or_else(|report| panic!("{name}: {report}"));
    }
}

#[test]
fn generated_nonlinear_noise_uses_complex_frequency_coefficients() {
    // Each expression contains one syntactic source. Reusing an assigned
    // realization and spelling the primitive directly must linearize alike.
    for (case, (expression, expected, integral)) in [
        ("sin(V(p,n)+N)", "(bias.cos(),0.0)", false),
        ("cos(V(p,n)+N)", "(-bias.sin(),0.0)", false),
        ("exp(V(p,n)+N)", "(bias.exp(),0.0)", false),
        ("ln(V(p,n)+N)", "(bias.recip(),0.0)", false),
        ("sqrt(V(p,n)+N)", "(0.5/bias.sqrt(),0.0)", false),
        ("tanh(V(p,n)+N)", "(1.0/bias.cosh().powi(2),0.0)", false),
        ("abs(V(p,n)+N)", "(1.0,0.0)", false),
        ("pow(V(p,n)+N,2.0)", "(2.0*bias,0.0)", false),
        ("hypot(V(p,n)+N,2.0)", "(bias/bias.hypot(2.0),0.0)", false),
        ("atan2(V(p,n)+N,2.0)", "(2.0/(bias*bias+4.0),0.0)", false),
        ("max(V(p,n)+N,0.0)", "(1.0,0.0)", false),
        ("sin(V(p,n)+ddt(N))", "(0.0,omega*bias.cos())", false),
        ("ddt(sin(V(p,n)+N))", "(0.0,omega*bias.cos())", false),
        ("ddt(ddt(N))", "(-omega*omega,0.0)", false),
        ("idt(N,0.25)", "(0.0,-1.0/omega)", true),
        (
            "sin(V(p,n)+idt(N,0.25))",
            "(0.0,-(bias+0.25).cos()/omega)",
            true,
        ),
        ("ddt(idt(N,0.25))", "(1.0,0.0)", true),
    ]
    .into_iter()
    .enumerate()
    {
        for contribution in ["I", "V"] {
            for assigned in [false, true] {
                let primitive = "white_noise(2.0/V(p,n),\"n\")";
                let assignment = if assigned {
                    format!("source={primitive};")
                } else {
                    String::new()
                };
                let expression =
                    expression.replace('N', if assigned { "source" } else { primitive });
                let source = format!(
                    "module general_noise(p,n); inout p,n; electrical p,n; parameter integer enabled=1; real source; analog if(enabled>0) begin {assignment} {contribution}(p,n)<+{expression}; end endmodule"
                );
                let name = format!("general noise {case} {contribution} {assigned}");
                let (state, stamp, noise) = generated_parts(&source, &name);
                let scale = if contribution == "I" { -2.0 } else { 0.5 };
                run_generated_main(&name, &state, &stamp, &noise, &format!(r#"
#[derive(Default)]
struct Capture(Vec<runtime::GeneratedNoiseComplex>);
impl runtime::GeneratedNoiseProcessVisitor for Capture {{
    fn visit_process(&mut self, _: usize, process: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {{
        if process.active {{
            assert_eq!(process.injections.len(),1);
            self.0.push(process.injections[0].gain);
        }}
        true
    }}
}}
let mut instance=device::state::Instance::new(&[0,1]);
instance.set_multiplicity(4.0).unwrap();
instance.finalize_parameters().unwrap();
for bias in [0.25_f64,0.75,2.0] {{
    for frequency in [0.0,0.1,1.0,1000.0] {{
        runtime::clear_evaluation_error();
        let ctx=runtime::GeneratedEvalContext {{voltages:&[bias,0.0],temperature:300.15}};
        let mut capture=Capture::default();
        let result=instance.evaluate_noise_processes_at_frequency(&ctx,frequency,&mut capture);
        if {integral} && frequency==0.0 {{
            assert!(result.is_err(),"integrator must remain singular even when D and I cancel");
            continue;
        }}
        result.unwrap();
        assert_eq!(capture.0.len(),1);
        let omega=std::f64::consts::TAU*frequency;
        let (re,im)={expected};
        for (actual,expected) in [(capture.0[0].re,re*{scale:?}),(capture.0[0].im,im*{scale:?})] {{
            assert!((actual-expected).abs()<=expected.abs()*2e-13+1e-14,"bias={{bias}} f={{frequency}} actual={{actual}} expected={{expected}}");
        }}
    }}
}}
instance.set_parameter("enabled",0.0).unwrap();
instance.finalize_parameters().unwrap();
let ctx=runtime::GeneratedEvalContext {{voltages:&[0.0,0.0],temperature:300.15}};
let mut capture=Capture::default();
instance.evaluate_noise_processes_at_frequency(&ctx,0.0,&mut capture).unwrap();
assert!(capture.0.is_empty());
assert!(!ctx.evaluation_failed());
"#)).unwrap_or_else(|report| panic!("{name}: {report}"));
            }
        }
    }
}

#[test]
fn generated_direct_noise_routing_matches_assigned_processes() {
    for operator in ["ddt", "slew"] {
        for contribution in ["I", "V"] {
            for assigned in [false, true] {
                let body = if assigned {
                    format!(
                        "source=white_noise(2.0/V(p,n),\"same\"); {contribution}(p,n)<+{operator}(3.0*source)+white_noise(5.0,\"same\");"
                    )
                } else {
                    format!(
                        "{contribution}(p,n)<+{operator}(3.0*white_noise(2.0/V(p,n),\"same\"))+white_noise(5.0,\"same\");"
                    )
                };
                let source = format!(
                    "module direct_dynamic_noise(p,n); inout p,n; electrical p,n; parameter real enabled=1; real source; analog if(enabled>0) begin {body} end endmodule"
                );
                let name = format!("direct {operator} noise {contribution} assigned={assigned}");
                let (state, stamp, noise) = generated_parts(&source, &name);
                let sign = if contribution == "I" { -1.0 } else { 1.0 };
                let derivative = operator == "ddt";
                let static_powers = if assigned || derivative {
                    vec![5.0]
                } else {
                    vec![9.0, 5.0]
                };
                run_generated_main(&name, &state, &stamp, &noise, &format!(r#"
#[derive(Default)]
struct Capture(Vec<(usize, f64, f64, f64)>);
impl runtime::GeneratedNoiseProcessVisitor for Capture {{
    fn visit_process(&mut self, index: usize, process: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {{
        if process.active {{
            assert_eq!(process.injections.len(), 1);
            let gain=process.injections[0].gain;
            self.0.push((index,process.psd,gain.re,gain.im));
        }}
        true
    }}
}}
#[derive(Default)]
struct StaticCapture(Vec<f64>);
impl runtime::GeneratedNoiseVisitor for StaticCapture {{
    fn visit(&mut self, _: usize, value: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool {{
        if value.active {{ self.0.push(value.psd); }}
        true
    }}
}}
assert_eq!(device::noise::GROUPED_NOISE_PROCESSES.len(), 2, "equal labels must not merge distinct calls");
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let ctx=runtime::GeneratedEvalContext {{voltages:&[2.0,0.0],temperature:300.15}};
let mut static_capture=StaticCapture::default();
instance.evaluate_noise_sources(&ctx,&mut static_capture).unwrap();
assert_eq!(static_capture.0.as_slice(), &{static_powers:?});
for frequency in [0.0,1.0,17.0] {{
    let mut capture=Capture::default();
    instance.evaluate_noise_processes_at_frequency(&ctx,frequency,&mut capture).unwrap();
    assert_eq!(capture.0.len(),2);
    for (index,psd,re,im) in capture.0 {{
        let (expected_psd,expected_re,expected_im)=if index==0 {{
            if {derivative} {{ (1.0,0.0,{sign:?}*3.0*std::f64::consts::TAU*frequency) }}
            else {{ (1.0,{sign:?}*3.0,0.0) }}
        }} else {{ (5.0,{sign:?},0.0) }};
        assert_eq!(psd,expected_psd);
        assert_eq!(re,expected_re);
        assert!((im-expected_im).abs() <= expected_im.abs()*1e-14);
    }}
}}
instance.set_parameter("enabled",0.0).unwrap();
instance.finalize_parameters().unwrap();
let disabled=runtime::GeneratedEvalContext {{voltages:&[0.0,0.0],temperature:300.15}};
let mut capture=Capture::default();
instance.evaluate_noise_processes_at_frequency(&disabled,1.0,&mut capture).unwrap();
assert!(capture.0.is_empty(), "disabled sources must not evaluate their singular PSD");
let mut static_capture=StaticCapture::default();
instance.evaluate_noise_sources(&disabled,&mut static_capture).unwrap();
assert!(static_capture.0.is_empty());
assert!(!disabled.evaluation_failed());
"#)).unwrap_or_else(|report| panic!("{name}: {report}"));
            }
        }
    }
}

#[test]
fn generated_grouped_noise_retains_static_and_reactive_paths_in_one_contribution() {
    let (state, stamp, noise) = generated_parts(
        r#"
module mixed_noise(p, n);
    inout p, n; electrical p, n;
    real source;
    analog begin
        source = white_noise(4.0, "mixed");
        I(p, n) <+ 2.0 * source + ddt(3.0 * source);
        I(p, n) <+ source + ddt(V(p, n));
        I(p, n) <+ ddt(5.0 * source);
    end
endmodule
"#,
        "mixed static reactive noise",
    );
    run_generated_main(
        "mixed static reactive noise",
        &state,
        &stamp,
        &noise,
        r#"
#[derive(Default)]
struct Capture(Vec<(usize, f64, f64)>);
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self, _: usize, process: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {
        assert!(process.active);
        assert_eq!(process.psd, 4.0);
        self.0.extend(process.injections.iter().map(|injection| {
            (device::noise::GROUPED_NOISE_INJECTIONS[injection.descriptor].equation,
             injection.gain.re, injection.gain.im)
        }));
        true
    }
}
let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
let ctx = runtime::GeneratedEvalContext { voltages: &[2.0, 0.0], temperature: 300.15 };
for frequency in [0.0, 1.0, 17.0] {
    let mut capture = Capture::default();
    instance.evaluate_noise_processes_at_frequency(&ctx, frequency, &mut capture).unwrap();
    assert_eq!(capture.0.len(), 3, "every coherent path must survive: {:?}", capture.0);
    for (equation, re, im) in capture.0 {
        let (static_gain, reactive_gain) = [(-2.0, -3.0), (-1.0, 0.0), (0.0, -5.0)][equation];
        assert_eq!(re, static_gain, "equation {equation}");
        assert_eq!(im, 2.0 * std::f64::consts::PI * frequency * reactive_gain, "equation {equation}");
    }
}
"#,
    )
    .unwrap_or_else(|report| panic!("mixed noise probe failed:\n{report}"));
}

#[test]
fn generated_one_step_split_does_not_hide_unsupported_dynamic_subexpressions() {
    for (index, expression) in [
        "ddt(V(p,n)) + sin(ddt(V(p,n)))",
        "sin(ddt(V(p,n))) + ddt(V(p,n))",
        "ddt(V(p,n)) * sin(ddt(V(p,n)))",
        "ddt(V(p,n)) / (1.0 + sin(ddt(V(p,n))))",
        "ddt(ddt(V(p,n)))",
    ]
    .iter()
    .enumerate()
    {
        let source = format!(
            "module nonlinear_dynamic(p,n); inout p,n; electrical p,n; analog I(p,n)<+{expression}; endmodule"
        );
        let name = format!("unsupported dynamic split {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        run_generated_main(
            &name,
            &state,
            &stamp,
            &noise,
            "assert!(!device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE);",
        )
        .unwrap_or_else(|report| panic!("{expression}: {report}"));
    }
}

#[test]
fn generated_split_validation_preserves_existing_reactive_linearization() {
    let (state, stamp, noise) = generated_parts(
        "module reactive_cosine(p,n); inout p,n; electrical p,n; analog I(p,n)<+ddt(V(p,n))+cos(ddt(V(p,n))); endmodule",
        "reactive split validation",
    );
    run_generated_main(
        "reactive split validation",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
let ctx = runtime::GeneratedEvalContext { voltages: &[2.0, 0.0], temperature: 300.15 };
instance.stamp(&ctx, &mut runtime::GeneratedStamper::default());
let mut reactive = [0.0];
instance.stamp_reactive(&ctx, &mut runtime::GeneratedReactiveStamper { sink: Some(&mut reactive) });
assert_eq!(reactive[0], 1.0, "cos(ddt(V)) has zero first variation at DC");
assert!(!device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE);
"#,
    )
    .unwrap_or_else(|report| panic!("reactive split validation: {report}"));
}

#[test]
fn generated_nested_history_operators_compile_and_evaluate_in_source_order() {
    for (index, expression, expected) in [
        (0, "ddt(ddt(V(p,n)))", 12.0),
        (1, "idt(idt(V(p,n),0.0),0.0)", 0.75),
        (2, "ddt(idt(V(p,n),0.0))", 3.0),
        (3, "idt(ddt(V(p,n)),0.0)", 3.0),
    ] {
        let source = format!(
            "module nested_history(p,n); inout p,n; electrical p,n; analog I(p,n)<+{expression}; endmodule"
        );
        let name = format!("nested history operator {index}");
        let (state, stamp, noise) = generated_parts(&source, &name);
        let body = format!(
            r#"
let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
instance.set_timepoint(1.0, 0.5, runtime::GeneratedDdtCoefficients {{
    active: true, derivative_scale: 2.0, previous_value_scale: 0.0,
    older_value_scale: 0.0, previous_derivative_scale: 0.0,
}});
let ctx = runtime::GeneratedEvalContext {{ voltages: &[3.0, 0.0], temperature: 300.15 }};
for _ in 0..2 {{
    let mut sink = [0.0; 10];
    instance.stamp(&ctx, &mut runtime::GeneratedStamper {{ sink: Some(&mut sink) }});
    assert_eq!(sink[9], {expected:?});
    assert!(!ctx.evaluation_failed());
}}
"#
        );
        run_generated_main(&name, &state, &stamp, &noise, &body)
            .unwrap_or_else(|report| panic!("{expression}: {report}"));
    }
}

#[test]
fn generated_grouped_noise_preserves_process_identity_coherence_and_rhs_orientation() {
    let (state, stamp, noise) = generated_parts(
        r#"
module grouped_processes(p, n, q);
    inout p, n, q;
    electrical p, n, q;
    real reused, first, second, cancelled;
    analog begin
        reused = white_noise(1.0, "reused");
        first = white_noise(1.0, "same");
        second = white_noise(1.0, "same");
        cancelled = white_noise(1.0, "cancelled");
        I(p, n) <+ reused + reused + first + second + cancelled - cancelled;
        V(q, n) <+ reused;
    end
endmodule
"#,
        "grouped process identity",
    );
    let body = r#"
#[derive(Default)]
struct Capture(Vec<(bool, f64, Vec<(usize, f64, f64)>)>);
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(
        &mut self,
        _index: usize,
        value: runtime::GeneratedNoiseProcessEvaluationRef<'_>,
    ) -> bool {
        self.0.push((
            value.active,
            value.psd,
            value.injections
                .iter()
                .map(|injection| (injection.descriptor, injection.gain.re, injection.gain.im))
                .collect(),
        ));
        true
    }
}

assert_eq!(device::noise::GROUPED_NOISE_PROCESSES.len(), 4);
assert_eq!(device::noise::GROUPED_NOISE_PROCESSES[1].label, Some("same"));
assert_eq!(device::noise::GROUPED_NOISE_PROCESSES[2].label, Some("same"));
assert_eq!(device::noise::GROUPED_NOISE_PROCESSES[1].process_id, 1);
assert_eq!(device::noise::GROUPED_NOISE_PROCESSES[2].process_id, 2);

let mut instance = device::state::Instance::new(&[0, 1, 2]);
instance.set_branch_indices(&[3]);
instance.finalize_parameters().unwrap();
let voltages = [0.0, 0.0, 0.0, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut capture = Capture::default();
instance.evaluate_noise_processes_at_frequency(&ctx, 1.0, &mut capture).unwrap();

assert_eq!(capture.0.len(), 4);
assert_eq!(capture.0[0].0, true);
assert_eq!(capture.0[0].1, 1.0);
assert_eq!(capture.0[0].2.len(), 2);
let mut reused_gains = capture.0[0].2.iter().map(|entry| entry.1).collect::<Vec<_>>();
reused_gains.sort_by(f64::total_cmp);
assert_eq!(reused_gains, vec![-2.0, 1.0],
    "current residual uses -RHS orientation while potential uses +RHS");
assert_eq!(capture.0[1].2.len(), 1);
assert_eq!(capture.0[1].2[0].1, -1.0);
assert_eq!(capture.0[2].2.len(), 1);
assert_eq!(capture.0[2].2[0].1, -1.0);
assert!(capture.0[3].2.iter().all(|entry| entry.1 == 0.0 && entry.2 == 0.0),
    "a fully cancelled process may retain a zero lane but must inject no amplitude");
assert!(capture.0.iter().all(|process| process.2.iter().all(|entry| entry.2 == 0.0)));
"#;
    run_generated_main("grouped process identity", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("grouped process identity probe failed:\n{report}"));
}

#[test]
fn generated_dynamic_potential_guards_keep_static_prefix_topology() {
    let (state, stamp, noise) = generated_parts(
        r#"
module static_then_dynamic(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enabled = 0;
    analog begin
        if (enabled > 0) begin
            if (V(p, n) > 0.0)
                V(p, n) <+ 2.0;
        end
    end
endmodule
"#,
        "static then dynamic guard",
    );
    let body = r#"
fn sample(enabled: f64, voltage: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2]);
    instance.set_parameter("enabled", enabled).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [voltage, 0.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let disabled = sample(0.0, 1.0);
assert_eq!(disabled[0], 0.0, "static outer false opens: {disabled:?}");
assert_eq!(disabled[1], 1.0, "static outer false pins: {disabled:?}");
let runtime_false = sample(1.0, -1.0);
assert_eq!(runtime_false[0], 1.0, "dynamic inner false retains topology: {runtime_false:?}");
assert_eq!(runtime_false[1], 0.0, "active topology is not pinned: {runtime_false:?}");
assert_eq!(runtime_false[2], 0.0, "dynamic inner false contributes zero: {runtime_false:?}");
assert_eq!(runtime_false[5], 1.0, "zero residual is still stamped on active branch: {runtime_false:?}");
let runtime_true = sample(1.0, 1.0);
assert_eq!(runtime_true[0], 1.0);
assert_eq!(runtime_true[2], 2.0, "dynamic inner true residual: {runtime_true:?}");
"#;
    run_generated_main("static then dynamic guard", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("static/dynamic guard probe failed:\n{report}"));

    let (state, stamp, noise) = generated_parts(
        r#"
module dynamic_then_static(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enabled = 0;
    analog begin
        if (V(p, n) > 0.0) begin
            if (enabled > 0)
                V(p, n) <+ 2.0;
        end
    end
endmodule
"#,
        "dynamic then static guard",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1]);
instance.set_branch_indices(&[2]);
instance.set_parameter("enabled", 0.0).unwrap();
instance.finalize_parameters().unwrap();
let voltages = [-1.0, 0.0, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut sink = [0.0; 9];
let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
instance.stamp(&ctx, &mut stamper);
assert_eq!(sink[0], 1.0, "first dynamic guard stops static peeling: {sink:?}");
assert_eq!(sink[1], 0.0, "dynamic-prefix topology remains active: {sink:?}");
assert_eq!(sink[2], 0.0, "untaken residual stays zero: {sink:?}");
"#;
    run_generated_main("dynamic then static guard", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("dynamic/static guard probe failed:\n{report}"));

    let (state, stamp, noise) = generated_parts(
        r#"
module time_guard(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if ($abstime > 1.0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "time-dependent potential guard",
    );
    let body = r#"
fn sample(time: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2]);
    instance.finalize_parameters().unwrap();
    instance.set_timepoint(time, 0.1, runtime::GeneratedDdtCoefficients::inactive());
    let voltages = [0.0; 3];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let before = sample(0.0);
assert_eq!(before[0], 1.0, "time guard must not change topology: {before:?}");
assert_eq!(before[1], 0.0, "time-guarded branch stays active: {before:?}");
assert_eq!(before[2], 0.0, "before threshold residual is zero: {before:?}");
let after = sample(2.0);
assert_eq!(after[0], 1.0);
assert_eq!(after[2], 2.0, "after threshold residual: {after:?}");
"#;
    run_generated_main(
        "time dependent potential guard",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("time guard probe failed:\n{report}"));

    let (state, stamp, noise) = generated_parts(
        r#"
module stateful_call_guard(p, n);
    inout p, n;
    electrical p, n;
    parameter real selector = 1.0;
    analog begin
        if (ddt(selector) > 0.0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "stateful-call potential guard",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1]);
instance.set_branch_indices(&[2]);
instance.finalize_parameters().unwrap();
// A constant operand does not make ddt instance-static. At the first
// evaluation its value is zero, so the residual path is untaken while the
// physical potential topology must remain active.
let voltages = [0.0; 3];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut sink = [0.0; 9];
let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
instance.stamp(&ctx, &mut stamper);
assert_eq!(sink[0], 1.0, "ddt guard must not change topology: {sink:?}");
assert_eq!(sink[1], 0.0, "stateful-call branch stays active: {sink:?}");
assert_eq!(sink[2], 0.0, "false ddt guard contributes zero: {sink:?}");
"#;
    run_generated_main(
        "stateful call potential guard",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("stateful-call guard probe failed:\n{report}"));
}

#[test]
fn generated_case_distinct_dynamic_guard_keeps_topology_active() {
    let (state, stamp, noise) = generated_parts(
        r#"
module case_distinct_guard(p, n);
    inout p, n;
    electrical p, n;
    parameter integer Enabled = 0;
    real enabled;
    analog begin
        enabled = V(p, n);
        if (enabled > 0.0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "case-distinct dynamic guard",
    );
    let body = r#"
fn sample(voltage: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2]);
    instance.finalize_parameters().unwrap();
    let voltages = [voltage, 0.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let runtime_false = sample(-1.0);
assert_eq!(runtime_false[0], 1.0, "case-distinct dynamic guard retains topology: {runtime_false:?}");
assert_eq!(runtime_false[1], 0.0, "dynamic guard must not pin the branch: {runtime_false:?}");
assert_eq!(runtime_false[2], 0.0, "untaken contribution has zero residual: {runtime_false:?}");
let runtime_true = sample(1.0);
assert_eq!(runtime_true[0], 1.0, "topology remains fixed when guard becomes true: {runtime_true:?}");
assert_eq!(runtime_true[2], 2.0, "taken contribution residual: {runtime_true:?}");
"#;
    run_generated_main("case distinct dynamic guard", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("case-distinct dynamic guard probe failed:\n{report}"));
}

#[test]
fn generated_guarded_reactive_potential_uses_group_orientation_and_clears_cache() {
    let (state, stamp, noise) = generated_parts(
        r#"
module guarded_flux(p, n);
    inout p, n;
    electrical p, n;
    parameter integer dc_enabled = 0;
    parameter integer reactive_enabled = 0;
    parameter real c = 3.0;
    analog begin
        if (dc_enabled > 0)
            V(p, n) <+ 0.0;
        if (reactive_enabled > 0)
            V(n, p) <+ ddt(c * V(p, n));
    end
endmodule
"#,
        "guarded reactive potential",
    );
    let body = r#"
fn evaluate(instance: &mut device::state::Instance, enabled: f64) -> ([f64; 9], [f64; 3]) {
    instance.set_parameter("dc_enabled", 0.0).unwrap();
    instance.set_parameter("reactive_enabled", enabled).unwrap();
    instance.finalize_parameters().unwrap();
    instance.set_timepoint(1.0, 0.5, runtime::GeneratedDdtCoefficients {
        active: true,
        derivative_scale: 2.0,
        previous_value_scale: 0.0,
        older_value_scale: 0.0,
        previous_derivative_scale: 0.0,
    });
    let voltages = [1.0, 0.0, 0.0, 99.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut real = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut real) };
    instance.stamp(&ctx, &mut stamper);
    let mut reactive = [0.0; 3];
    let mut reactive_stamper = runtime::GeneratedReactiveStamper { sink: Some(&mut reactive) };
    instance.stamp_reactive(&ctx, &mut reactive_stamper);
    (real, reactive)
}
let mut instance = device::state::Instance::new(&[0, 1]);
assert_eq!(device::state::Instance::BRANCH_COUNT, 1);
instance.set_branch_indices(&[2]);
let (active_real, active_reactive) = evaluate(&mut instance, 1.0);
assert_eq!(active_real[0], 1.0, "group couples once: {active_real:?}");
assert_eq!(active_real[1], 0.0, "active branch has no unused rows: {active_real:?}");
assert_eq!(active_real[3], -6.0, "reversed ddt derivative includes scale and sign: {active_real:?}");
assert_eq!(active_reactive[0], 1.0, "reactive row targets leader ordinal: {active_reactive:?}");
assert_eq!(active_reactive[1], -3.0, "reversed charge derivative sign: {active_reactive:?}");

let (inactive_real, inactive_reactive) = evaluate(&mut instance, 0.0);
assert_eq!(inactive_real[0], 0.0, "all static guards false open group: {inactive_real:?}");
assert_eq!(inactive_real[1], 1.0, "the inactive physical current is pinned: {inactive_real:?}");
assert_eq!(inactive_reactive[1], 0.0, "inactive evaluation clears cached derivative: {inactive_reactive:?}");

let (_, active_again) = evaluate(&mut instance, 1.0);
assert_eq!(active_again[1], -3.0, "reactive cache restores after re-enable: {active_again:?}");
"#;
    run_generated_main("guarded reactive potential", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("guarded reactive probe failed:\n{report}"));
}

#[test]
fn generated_static_dae_event_bodies_retain_the_settled_candidate() {
    let (state, stamp, noise) = generated_parts(
        include_str!("fixtures/static_dae_events.va"),
        "static DAE event observation",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1, 2]);
instance.finalize_parameters().unwrap();
runtime::set_event_analysis(true, false);
for (time, voltage, initial, final_step, expected) in [
    (0.0, -1.0, true, false, -1.0),
    (0.5, 1.0, false, true, 11113.0),
] {
    runtime::set_analysis_steps(initial, final_step);
    let voltages = [voltage, 0.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    instance.set_timepoint(time, 0.5, runtime::GeneratedDdtCoefficients {
        active: true, derivative_scale: 2.0, previous_value_scale: 2.0,
        older_value_scale: 0.0, previous_derivative_scale: 0.0,
    });
    instance.begin_stateful_evaluation();
    let mut sink = [0.0; 10];
    instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut sink) });
    let before = instance.capture_rollback_state();
    runtime::set_dynamic_operators_enabled(false);
    for _ in 0..2 {
        let mut sink = [0.0; 10];
        instance.stamp(&ctx, &mut runtime::GeneratedStamper { sink: Some(&mut sink) });
        assert_eq!(sink[9], expected, "event static current: {sink:?}");
        assert_eq!(instance.capture_rollback_state(), before);
    }
    runtime::set_dynamic_operators_enabled(true);
    instance.validate_advance_state().unwrap();
    instance.apply_validated_advance_state();
}
"#;
    run_generated_main("static DAE event observation", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("static DAE event observation failed:\n{report}"));
}

#[test]
fn generated_static_dae_probe_excludes_dynamic_current_without_mutating_state() {
    let (state, stamp, noise) = generated_parts(
        r#"
module static_dae_current(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 2.0;
    parameter real c = 3.0;
    analog begin
        I(p, n) <+ g * V(p, n) + ddt(c * V(p, n));
    end
endmodule
"#,
        "static DAE current probe",
    );
    let body = r#"
fn instance() -> device::state::Instance {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.finalize_parameters().unwrap();
    instance.set_timepoint(1.0, 0.5, runtime::GeneratedDdtCoefficients {
        active: true,
        derivative_scale: 2.0,
        previous_value_scale: 0.0,
        older_value_scale: 0.0,
        previous_derivative_scale: 0.0,
    });
    instance
}

let voltages = [1.0, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
assert!(device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE);

let mut dynamic_instance = instance();
let mut dynamic_sink = [0.0; 10];
let mut dynamic_stamper = runtime::GeneratedStamper { sink: Some(&mut dynamic_sink) };
dynamic_instance.stamp(&ctx, &mut dynamic_stamper);
assert_eq!(dynamic_sink[9], 8.0, "static plus ddt residual: {dynamic_sink:?}");

let mut static_instance = instance();
let mut physical_sink = [0.0; 10];
let mut physical_stamper = runtime::GeneratedStamper { sink: Some(&mut physical_sink) };
static_instance.begin_stateful_evaluation();
static_instance.stamp(&ctx, &mut physical_stamper);
let rollback_before = static_instance.capture_rollback_state();
runtime::set_dynamic_operators_enabled(false);
let mut static_sink = [0.0; 10];
let mut static_stamper = runtime::GeneratedStamper { sink: Some(&mut static_sink) };
static_instance.stamp(&ctx, &mut static_stamper);
runtime::set_dynamic_operators_enabled(true);
let rollback_after = static_instance.capture_rollback_state();
assert_eq!(static_sink[9], 2.0, "static probe must retain only F: {static_sink:?}");
assert_eq!(rollback_after, rollback_before, "static DAE probe mutated trial history");
"#;
    run_generated_main("static DAE current probe", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("static DAE current probe failed:\n{report}"));
}

#[test]
fn generated_integral_derivatives_follow_each_sites_initialization() {
    let (state, stamp, noise) = generated_parts(
        "module initialized_integral(p,n); inout p,n; electrical p,n;
         analog I(p,n)<+idt(V(p),V(n)*V(n)); endmodule",
        "initialized integral derivatives",
    );
    run_generated_main("initialized integral derivatives", &state, &stamp, &noise, r#"
let active = runtime::GeneratedDdtCoefficients {
    active:true, derivative_scale:8.0, previous_value_scale:8.0,
    older_value_scale:0.0, previous_derivative_scale:1.0,
};
for (coefficients, input_gain) in [(runtime::GeneratedDdtCoefficients::inactive(),0.0), (active,0.25)] {
    let mut instance=device::state::Instance::new(&[0,1]);
    instance.finalize_parameters().unwrap();
    instance.set_timepoint(0.25,0.25,coefficients);
    instance.begin_stateful_evaluation();
    let ctx=runtime::GeneratedEvalContext {voltages:&[1.5,5.0],temperature:300.15};
    let mut sink=[0.0;32];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
    assert_eq!((sink[12],sink[13]),(input_gain,10.0),"first candidate {sink:?}");
    instance.validate_advance_state().unwrap();
    instance.apply_validated_advance_state();
    instance.set_timepoint(0.5,0.25,active);
    instance.begin_stateful_evaluation();
    let mut sink=[0.0;32];
    instance.stamp(&ctx,&mut runtime::GeneratedStamper {sink:Some(&mut sink)});
    assert_eq!((sink[12],sink[13]),(0.125,0.0),"accepted history {sink:?}");
}
"#).unwrap_or_else(|report|panic!("{report}"));
}

#[test]
fn generated_integral_recovers_intermediate_overflow() {
    let (state, stamp, noise) = generated_parts(
        r#"
module large_integral(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idt(V(p, n), 1.0e308);
endmodule
"#,
        "finite large generated integral",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
let coefficients = runtime::GeneratedDdtCoefficients {
    active: true,
    derivative_scale: 4.0,
    previous_value_scale: 4.0,
    older_value_scale: 0.0,
    previous_derivative_scale: 0.0,
};
let voltages = [0.0, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
for time in [0.25, 0.5] {
    instance.set_timepoint(time, 0.25, coefficients);
    instance.begin_stateful_evaluation();
    let mut sink = [0.0; 10];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    instance.validate_advance_state().unwrap();
    instance.apply_validated_advance_state();
    let accepted = instance.capture_persistent_state();
    assert_eq!(accepted.idt_previous, vec![1.0e308]);
    assert_eq!(accepted.idt_older, vec![1.0e308]);
    assert_eq!(accepted.idt_input_previous, vec![0.0]);
}
"#;
    run_generated_main(
        "finite large generated integral",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("large integral probe failed:\n{report}"));
}

#[test]
fn generated_ddt_idt_candidates_are_transactional_and_skipped_retry_is_canonical() {
    let (state, stamp, noise) = generated_parts(
        r#"
module integrated_dynamic_current(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ ddt(V(p, n));
        I(p, n) <+ idt(V(p, n), 1.0);
    end
endmodule
"#,
        "transactional ddt and idt",
    );
    let body = r#"
fn stamp(instance: &mut device::state::Instance, voltage: f64) {
    let voltages = [voltage, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 10];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
}

let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
instance.set_timepoint(0.0, 0.0, runtime::GeneratedDdtCoefficients::inactive());
instance.begin_stateful_evaluation();
stamp(&mut instance, 2.0);
let before_promotion = instance.capture_persistent_state();
assert!(!before_promotion.ddt_initialized[0]);
assert!(!before_promotion.idt_initialized[0]);

let backward_euler = runtime::GeneratedDdtCoefficients {
    active: true,
    derivative_scale: 2.0,
    previous_value_scale: 2.0,
    older_value_scale: 0.0,
    previous_derivative_scale: 0.0,
};
instance.set_timepoint(0.5, 0.5, backward_euler);
let promoted = instance.capture_persistent_state();
assert_eq!(promoted.ddt_previous, vec![2.0]);
assert_eq!(promoted.ddt_older, vec![2.0]);
assert_eq!(promoted.idt_previous, vec![1.0]);
assert_eq!(promoted.idt_older, vec![1.0]);
assert_eq!(promoted.idt_input_previous, vec![2.0]);

instance.begin_stateful_evaluation();
stamp(&mut instance, 4.0);
assert_eq!(instance.capture_persistent_state(), promoted, "candidate escaped before acceptance");
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let accepted = instance.capture_persistent_state();
assert_eq!(accepted.ddt_previous, vec![4.0]);
assert_eq!(accepted.ddt_older, vec![2.0]);
assert_eq!(accepted.ddt_derivative_previous, vec![4.0]);
assert_eq!(accepted.idt_previous, vec![3.0]);
assert_eq!(accepted.idt_older, vec![1.0]);
assert_eq!(accepted.idt_input_previous, vec![4.0]);

instance.begin_stateful_evaluation();
stamp(&mut instance, f64::NAN);
instance.begin_stateful_evaluation();
runtime::set_dynamic_operators_enabled(false);
stamp(&mut instance, 9.0);
runtime::set_dynamic_operators_enabled(true);
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
assert_eq!(instance.capture_persistent_state(), accepted, "failed candidate or skipped retry leaked into accepted state");
let canonical = instance.capture_rollback_state();
assert!(canonical.values.iter().all(|value| value.is_finite()));
assert_eq!(canonical.flags, vec![true, true, false, false]);

let mut malformed_op = device::state::Instance::new(&[0, 1]);
malformed_op.finalize_parameters().unwrap();
malformed_op.set_timepoint(0.0, 0.0, runtime::GeneratedDdtCoefficients::inactive());
malformed_op.begin_stateful_evaluation();
stamp(&mut malformed_op, f64::NAN);
malformed_op.set_timepoint(0.5, 0.5, backward_euler);
let malformed_promoted = malformed_op.capture_persistent_state();
assert!(!malformed_promoted.ddt_initialized[0]);
assert!(!malformed_promoted.idt_initialized[0]);
assert!(malformed_promoted.ddt_previous.iter().all(|value| value.is_finite()));
assert!(malformed_promoted.idt_previous.iter().all(|value| value.is_finite()));
"#;
    run_generated_main("transactional ddt and idt", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("transactional ddt/idt probe failed:\n{report}"));
}

#[test]
fn generated_uic_initial_step_tran_commits_once_after_origin() {
    let (state, stamp, noise) = generated_parts(
        r#"
module uic_initial_step_state(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("tran")) count = count + 1.0;
        I(p, n) <+ count * V(p, n);
    end
endmodule
"#,
        "generated UIC initial-step lifecycle",
    );
    let body = r#"
fn stamp(instance: &mut device::state::Instance, ctx: &runtime::GeneratedEvalContext<'_>) {
    let mut sink = [0.0; 10];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(ctx, &mut stamper);
}

let voltages = [0.5, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
assert_eq!(device::state::Instance::EVENT_STATE_COUNT, 1);
runtime::set_event_analysis(true, false);

// UIC skips the operating-point initial flag. Its explicit t=0 evaluation
// accepts the ordinary event-variable baseline before integration begins.
runtime::set_analysis_steps(false, false);
instance.set_timepoint(0.0, 0.0, runtime::GeneratedDdtCoefficients::inactive());
instance.begin_stateful_evaluation();
stamp(&mut instance, &ctx);
assert_eq!(instance.capture_persistent_state().event_variables, vec![0.0]);
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
assert_eq!(instance.capture_persistent_state().event_variables, vec![0.0]);

// Every Newton retry of the first positive point starts from that accepted
// baseline, so the initial-step increment is speculative and idempotent.
let backward_euler = runtime::GeneratedDdtCoefficients {
    active: true,
    derivative_scale: 2.0,
    previous_value_scale: 2.0,
    older_value_scale: 0.0,
    previous_derivative_scale: 0.0,
};
instance.set_timepoint(0.5, 0.5, backward_euler);
runtime::set_analysis_steps(true, false);
instance.begin_stateful_evaluation();
stamp(&mut instance, &ctx);
let first_positive_trial = instance.capture_rollback_state();
assert_eq!(instance.capture_persistent_state().event_variables, vec![0.0]);
instance.begin_stateful_evaluation();
stamp(&mut instance, &ctx);
assert_eq!(
    instance.capture_rollback_state(),
    first_positive_trial,
    "a repeated first-positive trial accumulated initial_step(\"tran\")"
);
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
assert_eq!(instance.capture_persistent_state().event_variables, vec![1.0]);

// The following accepted point no longer carries the analysis initial flag.
instance.set_timepoint(1.0, 0.5, backward_euler);
runtime::set_analysis_steps(false, false);
instance.begin_stateful_evaluation();
stamp(&mut instance, &ctx);
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
assert_eq!(instance.capture_persistent_state().event_variables, vec![1.0]);
"#;
    run_generated_main(
        "generated UIC initial-step lifecycle",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("generated UIC initial-step lifecycle failed:\n{report}"));
}

#[test]
fn generated_event_variables_are_transactional_across_real_reactive_and_noise_evaluations() {
    let (state, stamp, noise) = generated_parts(
        r#"
module transactional_event_state(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("ac", "noise")) count = count + 1.0;
        @(final_step("ac", "noise")) count = count + 1.0;
        I(p, n) <+ count * V(p, n);
        I(p, n) <+ ddt(count * V(p, n));
        I(p, n) <+ white_noise(count, "event_count");
    end
endmodule
"#,
        "transactional generated event state",
    );
    let body = r#"
#[derive(Default)]
struct Capture(Vec<(bool, f64)>);
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self, _index: usize, value: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool {
        self.0.push((value.active, value.psd));
        true
    }
}

fn stamp(instance: &mut device::state::Instance, ctx: &runtime::GeneratedEvalContext<'_>) {
    let mut sink = [0.0; 12];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(ctx, &mut stamper);
}

fn noise(instance: &device::state::Instance, ctx: &runtime::GeneratedEvalContext<'_>) -> Vec<(bool, f64)> {
    let mut capture = Capture::default();
    instance.evaluate_noise_sources(ctx, &mut capture).unwrap();
    capture.0
}

let voltages = [0.5, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 123.0 };
let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
assert_eq!(device::state::Instance::EVENT_STATE_COUNT, 1);

runtime::set_analysis_steps(true, false);
instance.begin_stateful_evaluation();
stamp(&mut instance, &ctx);
let first_initial_trial = instance.capture_rollback_state();
assert_eq!(instance.capture_persistent_state().event_variables, vec![0.0]);
instance.begin_stateful_evaluation();
stamp(&mut instance, &ctx);
assert_eq!(instance.capture_rollback_state(), first_initial_trial,
    "a repeated OP trial accumulated its initial-step assignment");
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let accepted_initial = instance.capture_persistent_state();
assert_eq!(accepted_initial.event_variables, vec![1.0]);
let accepted_rollback = instance.capture_rollback_state();

let mut infinite_state = accepted_initial.clone();
infinite_state.event_variables[0] = f64::INFINITY;
let mut infinite_instance = device::state::Instance::new(&[0, 1]);
infinite_instance.restore_persistent_state(&infinite_state).unwrap();
infinite_instance.validate_advance_state().unwrap();
let mut nan_state = accepted_initial.clone();
nan_state.event_variables[0] = f64::NAN;
assert!(device::state::Instance::new(&[0, 1]).restore_persistent_state(&nan_state).is_err());

runtime::set_analysis_steps(false, true);
instance.begin_event_state_evaluation();
stamp(&mut instance, &ctx);
let first_final_trial = instance.capture_rollback_state();
let mut reactive = [0.0; 4];
let mut reactive_stamper = runtime::GeneratedReactiveStamper { sink: Some(&mut reactive) };
instance.stamp_reactive(&ctx, &mut reactive_stamper);
assert_eq!(reactive[0], 2.0,
    "reactive cache was not derived from the same accepted-state final-step trial");
assert_eq!(noise(&instance, &ctx), vec![(true, 2.0)]);
assert_eq!(instance.capture_persistent_state(), accepted_initial,
    "speculative real/reactive/noise evaluation changed accepted event state");

instance.restore_rollback_state(&accepted_rollback);
instance.begin_event_state_evaluation();
stamp(&mut instance, &ctx);
assert_eq!(instance.capture_rollback_state(), first_final_trial,
    "rollback did not restore the exact accepted/candidate event state");
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
assert_eq!(instance.capture_persistent_state().event_variables, vec![2.0]);

let mut restored = device::state::Instance::new(&[0, 1]);
restored.restore_persistent_state(&accepted_initial).unwrap();
assert_eq!(restored.capture_rollback_state(), accepted_rollback,
    "persistent restore did not seed both accepted and candidate state");
restored.begin_event_state_evaluation();
stamp(&mut restored, &ctx);
assert_eq!(noise(&restored, &ctx), vec![(true, 2.0)]);
restored.validate_advance_state().unwrap();
restored.apply_validated_advance_state();
assert_eq!(restored.capture_persistent_state().event_variables, vec![2.0]);
"#;
    run_generated_main(
        "transactional generated event state",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("transactional generated event-state probe failed:\n{report}"));
}

#[test]
fn generated_last_crossing_interpolates_without_refinement_and_restores_history() {
    for (expression, rising, falling) in [
        ("last_crossing(V(p,n))", 0.5, 2.75),
        ("last_crossing(V(p,n), 1)", 0.5, 0.5),
        ("last_crossing(V(p,n), -1)", -1.0, 2.75),
    ] {
        let source = format!(
            "module last_time(p,n); inout p,n; electrical p,n; analog I(p,n) <+ {expression}; endmodule"
        );
        let (state, stamp, noise) = generated_parts(&source, expression);
        run_generated_main(
            expression,
            &state,
            &stamp,
            &noise,
            &format!(
                r#"
fn stamp(instance: &mut device::state::Instance, voltage: f64) -> f64 {{
    let voltages = [voltage,0.0];
    let ctx = runtime::GeneratedEvalContext {{ voltages: &voltages, temperature: 300.15 }};
    let mut sink = [0.0;12];
    instance.begin_stateful_evaluation();
    instance.stamp(&ctx, &mut runtime::GeneratedStamper {{ sink: Some(&mut sink) }});
    assert!(!ctx.evaluation_failed());
    assert_eq!(instance.transient_event_refinement_time(), None);
    sink[0]
}}
let mut instance = device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
let coefficients = runtime::GeneratedDdtCoefficients::inactive();
runtime::set_event_analysis(false, true);
instance.set_timepoint(0.0,0.0,coefficients);
assert_eq!(stamp(&mut instance,-1.0), -1.0);
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let initial = instance.capture_persistent_state();
runtime::set_event_analysis(true, false);
instance.set_timepoint(2.0,2.0,coefficients);
let rollback = instance.capture_rollback_state();
assert_eq!(stamp(&mut instance,3.0), {rising:?});
let candidate = instance.capture_rollback_state();
assert_eq!(stamp(&mut instance,3.0), {rising:?});
assert_eq!(instance.capture_rollback_state(), candidate);
assert_eq!(instance.capture_persistent_state(), initial);
instance.restore_rollback_state(&rollback);
assert_eq!(stamp(&mut instance,-0.25), -1.0);
instance.restore_rollback_state(&rollback);
assert_eq!(stamp(&mut instance,3.0), {rising:?});
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let accepted = instance.capture_persistent_state();
let mut restored = device::state::Instance::new(&[0,1]);
restored.finalize_parameters().unwrap();
restored.restore_persistent_state(&accepted).unwrap();
for target in [&mut instance, &mut restored] {{
    target.set_timepoint(3.0,1.0,coefficients);
    assert_eq!(stamp(target,-1.0), {falling:?});
    target.validate_advance_state().unwrap();
    target.apply_validated_advance_state();
}}
assert_eq!(instance.capture_persistent_state(), restored.capture_persistent_state());
// An AC/static probe must keep the transient crossing time out of its value.
runtime::set_event_analysis(false, true);
assert_eq!(stamp(&mut instance,-1.0), -1.0);
"#
            ),
        )
        .unwrap_or_else(|report| panic!("{expression}: {report}"));
    }
}

#[test]
fn generated_last_crossing_preserves_real_arithmetic_and_procedural_directions() {
    let (state, stamp, noise) = generated_parts(
        "module crossing(p,n); inout p,n; electrical p,n; parameter integer dir=1; integer choice; real first; analog begin choice=dir; first=last_crossing(V(p,n),choice); choice=-dir; I(p,n) <+ first+10*last_crossing(V(p,n),choice); end endmodule",
        "typed generated last crossing",
    );
    run_generated_main(
        "typed generated last crossing",
        &state,
        &stamp,
        &noise,
        r#"
runtime::set_event_analysis(true, false);
for (direction, rising, falling) in [(1.0,-9.5,28.0),(-1.0,4.0,7.75)] {
    let mut instance = device::state::Instance::new(&[0,1]);
    instance.set_parameter("dir",direction).unwrap();
    instance.finalize_parameters().unwrap();
    for (time, voltage, expected) in [(0.0,-1.0,-11.0),(2.0,3.0,rising),(3.0,-1.0,falling)] {
        instance.set_timepoint(time,1.0,runtime::GeneratedDdtCoefficients::inactive());
        instance.begin_stateful_evaluation();
        let voltages = [voltage,0.0];
        let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature:300.15 };
        let mut sink = [0.0;12];
        instance.stamp(&ctx,&mut runtime::GeneratedStamper { sink:Some(&mut sink) });
        assert!(!ctx.evaluation_failed());
        assert_eq!(sink[0], expected);
        instance.validate_advance_state().unwrap();
        instance.apply_validated_advance_state();
    }
}
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_last_crossing_reports_invalid_direction_without_committing_history() {
    let (state, stamp, noise) = generated_parts(
        "module last_time(p,n); inout p,n; electrical p,n; parameter real dir=1; real t; analog begin t=last_crossing(V(p,n),dir); I(p,n) <+ t+white_noise(t<0 ? 1.0 : 2.0,\"input\"); end endmodule",
        "invalid last crossing direction",
    );
    run_generated_main(
        "invalid last crossing direction",
        &state,
        &stamp,
        &noise,
        r#"
struct Capture;
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self, _: usize, _: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool {
        panic!("invalid direction reached the noise visitor")
    }
}
impl runtime::GeneratedNoiseProcessVisitor for Capture {
    fn visit_process(&mut self, _: usize, _: runtime::GeneratedNoiseProcessEvaluationRef<'_>) -> bool {
        panic!("invalid direction reached the grouped noise visitor")
    }
}
runtime::set_event_analysis(true, false);
for direction in [-2.0, 2.0, 0.5] {
    runtime::clear_evaluation_error();
    let mut instance = device::state::Instance::new(&[0,1]);
    instance.set_parameter("dir",direction).unwrap();
    instance.finalize_parameters().unwrap();
    let accepted = instance.capture_persistent_state();
    let ctx = runtime::GeneratedEvalContext { voltages: &[-1.0,0.0], temperature:300.15 };
    instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
    assert!(ctx.evaluation_failed(), "invalid direction was silently accepted: {direction}");
    assert_eq!(instance.capture_persistent_state(), accepted);
    let history = instance.capture_rollback_state();
    runtime::clear_evaluation_error();
    assert!(instance.evaluate_noise_sources(&ctx, &mut Capture).is_err());
    assert!(ctx.evaluation_failed());
    runtime::clear_evaluation_error();
    assert!(instance.evaluate_noise_processes_at_frequency(&ctx, 1.0, &mut Capture).is_err());
    assert!(ctx.evaluation_failed());
    assert_eq!(instance.capture_rollback_state(), history);
}
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}

#[test]
fn generated_cross_above_and_timer_are_transactional_and_checkpointed() {
    let (state, stamp, noise) = generated_parts(
        r#"
module generated_event_controls(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(cross(V(p, n), 1, 0.0, 0.0, 1)) count = count + 1.0;
        @(above(V(p, n), 0.0, 0.0, 1)) count = count + 10.0;
        @(timer(1.0, 2.0, 0.0, 1)) count = count + 100.0;
        I(p, n) <+ count;
    end
endmodule
"#,
        "transactional generated event controls",
    );
    let body = r#"
fn stamp(instance: &mut device::state::Instance, voltage: f64) -> f64 {
    let voltages = [voltage, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 12];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink[0]
}

let inactive = runtime::GeneratedDdtCoefficients::inactive();

// `above` is true at a positive equilibrium point, and repeated static probes
// are evaluations from accepted state rather than accumulations.
runtime::set_event_analysis(false, true);
let mut static_instance = device::state::Instance::new(&[0, 1]);
static_instance.finalize_parameters().unwrap();
static_instance.set_timepoint(0.0, 0.0, inactive);
static_instance.begin_stateful_evaluation();
assert_eq!(stamp(&mut static_instance, 0.5), 10.0);
let static_trial = static_instance.capture_rollback_state();
assert_eq!(static_instance.capture_persistent_state().event_variables[0], 0.0);
static_instance.begin_stateful_evaluation();
assert_eq!(stamp(&mut static_instance, 0.5), 10.0);
assert_eq!(static_instance.capture_rollback_state(), static_trial);
static_instance.validate_advance_state().unwrap();
static_instance.apply_validated_advance_state();
assert_eq!(static_instance.capture_persistent_state().event_variables[0], 10.0);

runtime::set_event_analysis(true, false);
let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
instance.set_timepoint(0.0, 0.0, inactive);
instance.begin_stateful_evaluation();
assert_eq!(stamp(&mut instance, -1.0), 0.0);
assert_eq!(instance.transient_event_refinement_time(), None);
assert_eq!(instance.transient_timer_event_time(), None,
    "a speculative timer target must not become externally schedulable before acceptance");
assert_eq!(instance.transient_timer_step_bound(), Some(1.0));
let speculative_initial = instance.capture_persistent_state();
assert_eq!(speculative_initial.event_variables[0], 0.0);
assert_eq!(*speculative_initial.event_variables.last().unwrap(), f64::INFINITY,
    "a speculative timer request leaked into accepted checkpoint state");
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let accepted_initial = instance.capture_persistent_state();
assert_eq!(accepted_initial.event_variables[0], 0.0);
assert_eq!(*accepted_initial.event_variables.last().unwrap(), 1.0);
assert_eq!(instance.transient_timer_event_time(), Some(1.0));

// A malformed detector lane is rejected with its generated slot provenance.
let mut malformed_side = accepted_initial.clone();
malformed_side.event_variables[1 + 2] = 2.0;
let error = device::state::Instance::new(&[0, 1])
    .restore_persistent_state(&malformed_side)
    .expect_err("malformed crossing side must fail");
assert!(error.contains("slot 0") && error.contains("side"), "{error}");
let mut malformed_initialized = accepted_initial.clone();
malformed_initialized.event_variables[1 + 5] = 0.5;
let error = device::state::Instance::new(&[0, 1])
    .restore_persistent_state(&malformed_initialized)
    .expect_err("fractional crossing initialized flag must fail");
assert!(error.contains("initialized flag") && error.contains("finite integer"), "{error}");

instance.set_timepoint(1.0, 1.0, inactive);
instance.begin_stateful_evaluation();
let accepted_endpoint = instance.capture_rollback_state();
assert_eq!(stamp(&mut instance, 1.0), 111.0);
let refinement = f64::from_bits(0.5f64.to_bits() + 1);
assert_eq!(instance.transient_event_refinement_time(), Some(refinement));
assert_eq!(instance.transient_timer_event_time(), Some(1.0),
    "a trial endpoint must not replace the accepted absolute timer target");
assert_eq!(instance.transient_timer_step_bound(), Some(2.0));
let positive_trial = instance.capture_rollback_state();
assert_eq!(instance.capture_persistent_state(), accepted_initial,
    "cross/above/timer candidates changed the accepted checkpoint image");

instance.begin_stateful_evaluation();
assert_eq!(stamp(&mut instance, 1.0), 111.0);
assert_eq!(instance.capture_rollback_state(), positive_trial,
    "repeated Newton evaluation changed the event candidate");

instance.restore_rollback_state(&accepted_endpoint);
instance.begin_stateful_evaluation();
assert_eq!(stamp(&mut instance, -0.5), 100.0,
    "a rejected positive endpoint consumed cross/above state");
assert_eq!(instance.transient_event_refinement_time(), None);
assert_eq!(instance.transient_timer_step_bound(), Some(2.0));

instance.restore_rollback_state(&accepted_endpoint);
instance.begin_stateful_evaluation();
assert_eq!(stamp(&mut instance, 1.0), 111.0);
assert_eq!(instance.capture_rollback_state(), positive_trial,
    "rollback did not preserve the exact accepted/candidate event transaction");

// Accept the refined zero endpoint, then take the timer endpoint. The crossing
// is consumed only at the accepted root and the periodic next bound advances.
instance.restore_rollback_state(&accepted_endpoint);
instance.set_timepoint(refinement, refinement, inactive);
instance.begin_stateful_evaluation();
assert_eq!(stamp(&mut instance, 0.0), 11.0);
assert_eq!(instance.transient_event_refinement_time(), None);
assert_eq!(instance.transient_timer_step_bound(), Some(1.0 - refinement));
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
assert_eq!(instance.capture_persistent_state().event_variables[0], 11.0);

instance.set_timepoint(1.0, 1.0 - refinement, inactive);
instance.begin_stateful_evaluation();
assert_eq!(stamp(&mut instance, 1.0), 111.0);
assert_eq!(instance.transient_event_refinement_time(), None);
assert_eq!(instance.transient_timer_step_bound(), Some(2.0));
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let accepted = instance.capture_persistent_state();
assert_eq!(accepted.event_variables[0], 111.0);
assert_eq!(*accepted.event_variables.last().unwrap(), 3.0);

let accepted_rollback = instance.capture_rollback_state();
let mut restored = device::state::Instance::new(&[0, 1]);
restored.restore_persistent_state(&accepted).unwrap();
assert_eq!(restored.transient_timer_event_time(), Some(3.0),
    "the persisted absolute timer target must be usable before evaluation time is re-primed on resume");
restored.set_timepoint(1.0, 0.0, inactive);
assert_eq!(restored.transient_timer_step_bound(), Some(2.0));
assert_eq!(restored.capture_persistent_state(), accepted);
assert_eq!(restored.capture_rollback_state(), accepted_rollback);
"#;
    run_generated_main(
        "transactional generated event controls",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| {
        panic!("transactional generated event-control probe failed:\n{report}")
    });
}

#[test]
fn generated_one_step_split_capability_rejects_dynamic_control_and_idt() {
    let (state, stamp, noise) = generated_parts(
        r#"
module ddt_control(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (ddt(V(p, n)) > 0.0)
            I(p, n) <+ 1.0;
    end
endmodule
"#,
        "ddt control-flow capability",
    );
    run_generated_main(
        "ddt control-flow capability",
        &state,
        &stamp,
        &noise,
        "assert!(!device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE);",
    )
    .unwrap_or_else(|report| panic!("ddt control capability probe failed:\n{report}"));

    let (state, stamp, noise) = generated_parts(
        r#"
module integrated_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idt(V(p, n), 0.0);
endmodule
"#,
        "idt capability",
    );
    run_generated_main(
        "idt capability",
        &state,
        &stamp,
        &noise,
        "assert!(!device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE);",
    )
    .unwrap_or_else(|report| panic!("idt capability probe failed:\n{report}"));
}

#[test]
fn generated_stages_follow_model_and_instance_parameter_scope() {
    let source = r#"
module scoped_stage(p, n);
    inout p, n;
    electrical p, n;
    parameter real model_gain = 2.0;
    (* type = "instance" *) parameter real width = 1.0e-6;
    (* type = "instance" *) parameter real area = width * width from [0.0:1.0e-6];
    real model_shape, geometry;
    analog begin
        model_shape = model_gain * model_gain;
        model_shape = model_shape * model_shape + 3.0 * model_gain;
        geometry = area;
        geometry = geometry * geometry * model_shape;
        I(p, n) <+ geometry * V(p, n);
    end
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files: Vec<(&str, &str)> = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect();
    let state = find(&files, "state.rs", "scoped stage");
    let stamp = find(&files, "stamp.rs", "scoped stage");
    let noise = find(&files, "noise.rs", "scoped stage");

    assert!(stamp.contains("fn canonical_model_stage"));
    assert!(stamp.contains("fn canonical_instance_stage"));
    assert!(stamp.contains("self.canonical_model_stage(ctx);"));
    assert!(stamp.contains("self.canonical_instance_stage(ctx);"));
    assert!(stamp.contains("static CANONICAL_MODEL_CACHE"));
    assert!(stamp.contains("canonical_model_cache_lookup"));
    assert!(stamp.contains("canonical_model_cache_intern"));
    assert!(state.contains("pub(crate) type CanonicalModelValues"));
    assert!(state.contains("Option<std::sync::Arc<CanonicalModelValues>>"));
    assert!(state.contains("pub(crate) const PARAMETER_MODEL_FLAGS: [bool; 3]"));
    assert!(state.contains("true, false, false"));
    assert!(state.contains("if PARAMETER_MODEL_FLAGS[index]"));
    assert!(state.contains("self.canonical_model_values = None;"));
    assert!(state.contains("let changed = self.multiplicity.to_bits()"));
    assert!(state.contains("self.canonical_instance_valid = false;"));

    if let Err(report) = compile("scoped stage", state, stamp, noise) {
        panic!("scoped stage: generated device does not compile:\n{report}");
    }
    if let Err(report) = run_shared_model_cache("scoped stage cache", state, stamp, noise) {
        panic!("scoped stage: shared model cache failed:\n{report}");
    }
}

#[test]
fn shared_noise_preprocessing_is_fresh_and_call_order_independent() {
    let source = fixtures()
        .into_iter()
        .find_map(|(name, source)| (name == "shared noise preprocessing").then_some(source))
        .expect("shared-noise fixture");
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect::<Vec<_>>();
    let state = find(&files, "state.rs", "shared noise preprocessing");
    let stamp = find(&files, "stamp.rs", "shared noise preprocessing");
    let noise = find(&files, "noise.rs", "shared noise preprocessing");

    assert!(stamp.contains("pub(super) fn canonical_model_preprocess"));
    assert!(noise.contains("let mut prepared = [0.0;"));
    assert!(noise.contains("canonical_model_preprocess("));
    assert!(!noise.contains("vec![0.0;"));
    if let Err(report) = run_noise_call_order("shared noise call order", state, stamp, noise) {
        panic!("shared preprocessing changed independent noise evaluation:\n{report}");
    }
}

#[test]
fn repeated_static_hot_guards_are_specialized_with_a_source_size_cap() {
    let source = repeated_structure_source();
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(&source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files: Vec<(&str, &str)> = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect();
    let state = find(&files, "state.rs", "repeated structure");
    let stamp = find(&files, "stamp.rs", "repeated structure");
    let noise = find(&files, "noise.rs", "repeated structure");

    assert!(
        stamp.contains("Bounded structural specialization: one dispatch replaces 3"),
        "three uses of one cached model condition should become one bounded dispatch; \
         stamp bytes={}, relevant lines:\n{}",
        stamp.len(),
        stamp
            .lines()
            .filter(|line| line.contains("if ") || line.contains("staged["))
            .take(40)
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        stamp.contains("if staged[") && stamp.contains("canonical_structural_output_0"),
        "the specialized variants must rejoin through explicit scalar outputs"
    );
    if let Err(report) = compile("repeated structure", state, stamp, noise) {
        panic!("repeated structure: generated specialization does not compile:\n{report}");
    }
    if let Err(report) = run_structural_variants("repeated structure runtime", state, stamp, noise)
    {
        panic!("repeated structure: generated specialization changed behavior:\n{report}");
    }
}

fn repeated_structure_source() -> String {
    let mut coefficient_work = String::new();
    for index in 0..80 {
        coefficient_work.push_str(&format!(
            "        coefficient = coefficient * 1.0000001 + {}.0e-12;\n",
            index + 1
        ));
    }
    let mut guarded_work = String::new();
    for branch in 0..3 {
        guarded_work.push_str("        if (mode > 0.0) begin\n");
        for index in 0..64 {
            // Distinct nonlinear work gives specialization a realistic source
            // budget; unavailable simulator queries fold to their fallbacks.
            let factor = branch * 64 + index + 1;
            guarded_work.push_str(&format!(
                "            current = current + limexp(V(p, n) * {factor}.0e-3) * 1.0e-9 * coefficient * V(p, n);\n"
            ));
        }
        guarded_work.push_str("        end\n");
    }
    format!(
        "module repeated_structure(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   parameter real mode = 1.0;\n\
         \x20   real coefficient, current;\n\
         \x20   analog begin\n\
         \x20       coefficient = mode + 1.0;\n\
         {coefficient_work}\
         \x20       current = 0.0;\n\
         {guarded_work}\
         \x20       I(p, n) <+ current;\n\
         \x20   end\n\
         endmodule\n"
    )
}

#[test]
fn structural_specialization_rejects_source_growth_over_two_percent() {
    let mut coefficient_work = String::new();
    for index in 0..80 {
        coefficient_work.push_str(&format!(
            "        coefficient = coefficient * 1.0000001 + {}.0e-12;\n",
            index + 1
        ));
    }
    let mut common_work = String::new();
    for index in 0..240 {
        common_work.push_str(&format!(
            "        current = current + coefficient * V(p, n) * {}.0e-9;\n",
            index + 1
        ));
    }
    let source = format!(
        "module rejected_structure(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   parameter real mode = 1.0;\n\
         \x20   real coefficient, current;\n\
         \x20   analog begin\n\
         \x20       coefficient = mode + 1.0;\n\
         {coefficient_work}\
         \x20       current = 0.0;\n\
         {common_work}\
         \x20       if (mode > 0.0) current = current + V(p, n) * 1.0e-12;\n\
         \x20       if (mode > 0.0) current = current + V(p, n) * 2.0e-12;\n\
         \x20       if (mode > 0.0) current = current + V(p, n) * 3.0e-12;\n\
         \x20       I(p, n) <+ current;\n\
         \x20   end\n\
         endmodule\n"
    );

    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(&source)
        .expect("front end");
    let stamp = canonical::generate_device(&artifact, &options())
        .expect("generation")
        .files
        .into_iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp.rs")
        .contents;

    assert!(
        stamp.contains("fn canonical_model_stage"),
        "the parameter prologue must be split so its repeated condition is cacheable"
    );
    assert!(
        !stamp.contains("Bounded structural specialization"),
        "duplicating the large common Newton path would violate the 2% source-size cap"
    );
    assert!(
        stamp
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                line.starts_with("if ") && line.ends_with(" {") && !line.contains("staged[")
            })
            .count()
            >= 3,
        "the rejected candidate must retain its three ordinary branches"
    );
}

/// A model whose residual is a `ddt` gets a reactive stamp, and one without
/// gets an empty one rather than the conduction Jacobian by mistake.
#[test]
fn charge_storage_reaches_the_reactive_matrix_and_conduction_does_not() {
    let capacitor = r#"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#;
    let resistor = r#"
module res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 250.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

    let stamp = stamp_of(capacitor, "cap");
    assert!(
        stamp.contains("stamp_current_reactive_indexed_dense_local"),
        "a capacitor stores charge and must write the reactive matrix:\n{stamp}"
    );

    let stamp = stamp_of(resistor, "res");
    assert!(
        !stamp.contains("stamp_current_reactive_indexed_dense_local"),
        "a resistor stores no charge, so its reactive stamp writes nothing:\n{stamp}"
    );
    assert!(
        stamp.contains("pub fn stamp_reactive"),
        "the reactive entry point exists whether or not it has work:\n{stamp}"
    );
}

/// A charge stored under a guard still reaches the reactive matrix.
///
/// `EPFL_HEMT_10a` writes `if (rth != 0) Pwr(t) <+ ddt(cth * Temp(t))`, and the
/// golden replay caught the whole capacitance entry going missing. A guarded
/// contribution arrives at its equation as a *merge* — the `ddt` from the arm
/// that ran, zero from the arm that did not — so a rule that matches the
/// residual against `Ddt` finds nothing and drops the charge silently. It is
/// silent in DC too: only AC and transient ever read the reactive matrix, which
/// is why a whole corpus of DC-shaped fixtures never noticed. Self-heating
/// blocks are guarded as a matter of course, so this is the common shape.
#[test]
fn a_guarded_charge_still_reaches_the_reactive_matrix() {
    let guarded = r#"
module guarded_cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    parameter real enable = 1.0;
    analog begin
        if (enable != 0.0) begin
            I(p, n) <+ ddt(c * V(p, n));
        end
        I(p, n) <+ V(p, n) * 1.0e-6;
    end
endmodule
"#;
    let stamp = stamp_of(guarded, "guarded_cap");
    assert!(
        stamp.contains("stamp_current_reactive_indexed_dense_local"),
        "a guarded capacitor still stores charge on the path that runs:\n{stamp}"
    );
}

/// Linear arithmetic around a `ddt` is pushed inside it.
///
/// `EKV` writes `I(db) <+ TYPE * ddt_QD` and `I(d,b) <+ ddt(qjd)*TYPE*M`;
/// scaling a charge by a polarity or a multiplicity is idiomatic, and a rule
/// that matched only a bare `ddt` dropped every one of them. `k * ddt(q)`
/// stores `k * q` and `ddt(q1) + ddt(q2)` stores `q1 + q2`, so the operations
/// that commute with `d/dt` are followed into the charge and the product exists
/// nowhere until it is built.
#[test]
fn linear_arithmetic_around_a_ddt_still_stores_charge() {
    let scaled = r#"
module scaled_cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    parameter real polarity = 1.0;
    analog I(p, n) <+ polarity * ddt(c * V(p, n));
endmodule
"#;
    let summed = r#"
module summed_cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c1 = 1.0e-12;
    parameter real c2 = 3.0e-12;
    analog I(p, n) <+ ddt(c1 * V(p, n)) + ddt(c2 * V(p, n));
endmodule
"#;
    for (source, module) in [(scaled, "scaled_cap"), (summed, "summed_cap")] {
        let stamp = stamp_of(source, module);
        assert!(
            stamp.contains("stamp_current_reactive_indexed_dense_local"),
            "{module} stores charge through linear arithmetic:\n{stamp}"
        );
    }
}

/// How far the canonical backend gets across the shipped models, and why it
/// stops where it does.
///
/// Numbers and reasons, not assertions. What it answers is the only question
/// that decides when this backend takes over from the tiers: which models it
/// carries end to end, and what each of the rest is waiting on.
#[test]
#[ignore = "generates every shipped model through the canonical backend; run with --ignored"]
fn the_whole_corpus_reports_what_the_canonical_backend_carries() {
    let root = model_root();
    let candidates =
        rspice_veriloga::rust_backend::discover_veriloga_sources(&root).expect("model tree");
    let mut carried = 0usize;
    let mut refused = 0usize;
    let mut bytes = 0usize;
    let mut stamp_bytes = 0usize;
    let mut noise_bytes = 0usize;
    let mut noise_fallbacks = 0usize;

    for candidate in &candidates {
        for module in &candidate.modules {
            let mut options = rspice_veriloga::CompilerOptions::default();
            options.include_paths.push(root.clone());
            options.defines = candidate.compile_profile.defines.clone();
            options.undefines = candidate.compile_profile.undefines.clone();
            let compiled = match VerilogACompiler::new(options)
                .compile_file_canonical_ir_with_metadata(&candidate.path, Some(module))
            {
                Ok(compiled) => compiled,
                Err(error) => {
                    refused += 1;
                    eprintln!("{module:>24}  front end: {error}");
                    continue;
                }
            };
            match std::panic::catch_unwind(|| {
                canonical::generate_device(&compiled.artifact, &RustTranspileOptions::default())
            }) {
                Ok(Ok(device)) => {
                    carried += 1;
                    let total: usize = device
                        .files
                        .iter()
                        .map(|file| file.contents.len())
                        .sum::<usize>();
                    let sized = |name: &str| {
                        device
                            .files
                            .iter()
                            .find(|file| file.relative_path == name)
                            .map_or(0, |file| file.contents.len())
                    };
                    let (stamp, noise) = (sized("stamp.rs"), sized("noise.rs"));
                    // The replaced generator replays statements through a
                    // workspace array; the canonical one emits a body. Which
                    // wrote this file is the difference between a model whose
                    // noise the CFG carries and one that fell back to keep its
                    // device, and a byte count alone does not say which.
                    let fell_back = device
                        .files
                        .iter()
                        .find(|file| file.relative_path == "noise.rs")
                        .is_some_and(|file| file.contents.contains("let mut w = [0.0;"));
                    if fell_back {
                        noise_fallbacks += 1;
                    }
                    bytes += total;
                    stamp_bytes += stamp;
                    noise_bytes += noise;
                    eprintln!(
                        "{module:>24}  {total:>10} bytes  ({stamp} stamp, {noise} noise{})",
                        if fell_back { ", fell back" } else { "" }
                    );
                }
                Ok(Err(error)) => {
                    refused += 1;
                    eprintln!("{module:>24}  refused: {error}");
                }
                Err(payload) => {
                    refused += 1;
                    eprintln!("{module:>24}  panicked: {}", panic_reason(&payload));
                }
            }
        }
    }
    eprintln!(
        "\n{carried} carried in {bytes} bytes, {refused} not \
         ({stamp_bytes} stamp, {noise_bytes} noise, \
         {noise_fallbacks} of them from the replaced generator)"
    );
}

fn panic_reason(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    payload.downcast_ref::<&str>().map_or_else(
        || "no known payload".to_string(),
        |message| (*message).to_string(),
    )
}

fn model_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    assert!(root.exists(), "model tree missing: {}", root.display());
    root
}

fn stamp_of(source: &str, name: &str) -> String {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
    let device = canonical::generate_device(&artifact, &options())
        .unwrap_or_else(|error| panic!("{name}: generation: {error}"));
    device
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .map(|file| file.contents.clone())
        .unwrap_or_else(|| panic!("{name}: no stamp.rs"))
}

fn options() -> RustTranspileOptions {
    RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
        ..RustTranspileOptions::default()
    }
}

#[test]
fn transpiler_reports_hot_phases_and_exact_output_size() {
    let (name, source) = fixtures()[0];
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
    let generated = RustTranspiler::new(options())
        .transpile_measured(&artifact)
        .unwrap_or_else(|error| panic!("{name}: measured generation: {error}"));

    for phase in [
        PipelinePhase::CfgLowering,
        PipelinePhase::DerivativePreparation,
        PipelinePhase::Differentiation,
        PipelinePhase::DerivativeExtraction,
        PipelinePhase::NoisePlanning,
        PipelinePhase::StampPlanning,
        PipelinePhase::CfgOptimization,
        PipelinePhase::Scheduling,
        PipelinePhase::StampEmission,
        PipelinePhase::StateEmission,
        PipelinePhase::NoiseEmission,
        PipelinePhase::CheckpointFinalization,
    ] {
        assert!(
            generated.metrics.has_phase(phase),
            "missing structured metric for {phase}"
        );
    }
    let bytes = generated
        .output
        .files
        .iter()
        .map(|file| file.contents.len() as u64)
        .sum::<u64>();
    let lines = generated
        .output
        .files
        .iter()
        .map(|file| file.contents.lines().count() as u64)
        .sum::<u64>();
    assert_eq!(generated.metrics.generated_rust_bytes, bytes);
    assert_eq!(generated.metrics.generated_rust_lines, lines);
    assert!(generated.metrics.derivative_seed_count > 0);
    let derivative_values = generated
        .metrics
        .scalar_derivative_value_count
        .saturating_add(generated.metrics.packed_derivative_value_count);
    assert!(derivative_values > 0);
    assert!(generated.metrics.derivative_lane_entry_count >= derivative_values);
    assert!(generated.metrics.max_derivative_width > 0);
    assert!(generated.metrics.primal_cfg.value_count > 0);
    assert!(
        generated.metrics.differentiated_cfg.value_count
            >= generated.metrics.primal_cfg.value_count
    );
    assert!(
        generated.metrics.optimized_cfg.value_count
            <= generated.metrics.differentiated_cfg.value_count
    );
}

#[test]
fn generated_discontinuity_preserves_degrees_and_transactional_state() {
    let source = r#"
module controlled(p,n);
 inout p,n; electrical p,n;
 parameter real degree=0.0;
 parameter integer passes=1;
 integer i;
 real ticks;
 analog begin
  for(i=0;i<passes;i=i+1) begin
   $bound_step(1e-9);
   if(V(p,n)>0.0) $discontinuity(degree);
  end
  if(V(p,n)>1.0) $discontinuity(-1);
  @(timer(1.0,2.0)) ticks=ticks+1;
  I(p,n)<+V(p,n)*1e-3;
 end
endmodule
"#;
    let (state, stamp, noise) = generated_parts(source, "discontinuity state");
    run_generated_main(
        "discontinuity state",
        &state,
        &stamp,
        &noise,
        r#"
fn stamp(instance: &mut device::state::Instance, voltage:f64) -> bool {
 runtime::clear_evaluation_error();
 let voltages=[voltage,0.0];
 let ctx=runtime::GeneratedEvalContext { voltages:&voltages, temperature:300.0 };
 instance.begin_stateful_evaluation();
 instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
 !ctx.evaluation_failed()
}
runtime::set_event_analysis(true,false);
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
instance.set_timepoint(0.0,0.0,runtime::GeneratedDdtCoefficients::inactive());
assert!(stamp(&mut instance,0.0));
assert!(!instance.discontinuity_rising());
assert!(instance.limiter_converged());
assert!(stamp(&mut instance,1.0));
assert!(instance.discontinuity_rising());
assert!(instance.limiter_converged());
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
let accepted=instance.capture_persistent_state();
let rollback=instance.capture_rollback_state();
assert!(stamp(&mut instance,2.0));
assert!(!instance.discontinuity_rising());
assert!(!instance.limiter_converged());
assert_eq!(instance.capture_persistent_state(),accepted);
assert!(!instance.clone().limiter_converged());
instance.restore_rollback_state(&rollback);
assert!(instance.limiter_converged());
assert!(!instance.discontinuity_rising());
instance.set_parameter("degree",-1.0).unwrap();
assert!(stamp(&mut instance,1.0));
assert!(!instance.discontinuity_rising());
assert!(!instance.limiter_converged());
instance.set_parameter("passes",0.0).unwrap();
assert!(stamp(&mut instance,1.0));
assert!(!instance.discontinuity_rising());
assert!(instance.limiter_converged());
assert_eq!(instance.transient_step_bound().unwrap(),None);
instance.validate_advance_state().unwrap();
instance.apply_validated_advance_state();
instance.restore_persistent_state(&accepted).unwrap();
assert!(instance.limiter_converged());
assert!(!instance.discontinuity_rising());
assert_eq!(instance.transient_step_bound().unwrap(),Some(1e-9));
assert_eq!(instance.transient_timer_step_bound(),Some(1.0));
let mut invalid=accepted.clone();
invalid.event_variables[2]=2.0;
assert!(instance.restore_persistent_state(&invalid).unwrap_err().contains("discontinuity"));
assert_eq!(instance.capture_persistent_state(),accepted);
instance.set_parameter("passes",1.0).unwrap();
instance.set_parameter("degree",0.5).unwrap();
assert!(!stamp(&mut instance,1.0));
assert!(instance.validate_advance_state().unwrap_err().contains("$discontinuity"));
assert!(!instance.limiter_converged());
assert_eq!(instance.capture_persistent_state(),accepted);
runtime::clear_evaluation_error();
let ctx=runtime::GeneratedEvalContext { voltages:&[0.0,0.0],temperature:300.0 };
instance.begin_analysis(&ctx);
assert!(!instance.discontinuity_rising());
assert!(instance.limiter_converged());
"#,
    )
    .unwrap_or_else(|error| panic!("{error}"));
}

#[test]
fn generated_bound_step_preserves_loop_minimum_and_transactional_state() {
    let source = r#"
module controlled(p,n);
 inout p,n; electrical p,n;
 parameter integer passes=3;
 integer i;
 real ticks;
 analog begin
  for(i=1;i<=passes;i=i+1) $bound_step(i*1e-9);
  if(V(p,n)>0.0) $bound_step(ddx(V(p,n)*V(p,n),V(p,n))*1e-10);
  @(timer(1.0,2.0)) ticks=ticks+1;
  I(p,n)<+V(p,n)*1e-3;
 end
endmodule
"#;
    let (state, stamp, noise) = generated_parts(source, "bound step");
    run_generated_main(
        "bound step",
        &state,
        &stamp,
        &noise,
        r#"
fn stamp(instance: &mut device::state::Instance, voltage:f64) {
 let voltages=[voltage,0.0];
 let ctx=runtime::GeneratedEvalContext { voltages:&voltages, temperature:300.0 };
 instance.begin_stateful_evaluation();
 instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
 assert!(!ctx.evaluation_failed());
 instance.validate_advance_state().unwrap();
}
runtime::set_event_analysis(true,false);
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
instance.set_timepoint(0.0,0.0,runtime::GeneratedDdtCoefficients::inactive());
stamp(&mut instance,-1.0);
assert_eq!(instance.transient_step_bound().unwrap(),Some(1e-9));
instance.apply_validated_advance_state();
let accepted=instance.capture_persistent_state();
let rollback=instance.capture_rollback_state();
stamp(&mut instance,2.0);
assert_eq!(instance.transient_step_bound().unwrap(),Some(4e-10));
assert_eq!(instance.capture_persistent_state(),accepted);
assert_eq!(instance.clone().transient_step_bound().unwrap(),Some(4e-10));
instance.restore_rollback_state(&rollback);
assert_eq!(instance.transient_step_bound().unwrap(),Some(1e-9));
instance.set_parameter("passes",0.0).unwrap();
stamp(&mut instance,-1.0);
assert_eq!(instance.transient_step_bound().unwrap(),None);
instance.restore_persistent_state(&accepted).unwrap();
assert_eq!(instance.transient_step_bound().unwrap(),Some(1e-9));
assert_eq!(instance.transient_timer_step_bound(),Some(1.0));
let mut invalid=accepted.clone();
invalid.event_variables[1]=-1.0;
assert!(instance.restore_persistent_state(&invalid).is_err());
"#,
    )
    .unwrap_or_else(|error| panic!("{error}"));
}

#[test]
fn generated_bound_step_preserves_zero_and_rejects_invalid_trials() {
    let (state, stamp, noise) = generated_parts(
        "module bounded(p,n); inout p,n; electrical p,n; analog begin $bound_step(V(p,n)); I(p,n)<+V(p,n); end endmodule",
        "bound validation",
    );
    run_generated_main(
        "bound validation",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for voltage in [0.0, 1e-30, -1.0] {
 let voltages=[voltage,0.0];
 let ctx=runtime::GeneratedEvalContext { voltages:&voltages,temperature:300.0 };
 instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
 assert!(!ctx.evaluation_failed());
 if voltage>=0.0 {
  assert_eq!(instance.transient_step_bound().unwrap(),Some(voltage));
  instance.validate_advance_state().unwrap();
  instance.apply_validated_advance_state();
 } else {
  assert!(instance.transient_step_bound().unwrap_err().contains("$bound_step"));
  assert!(instance.validate_advance_state().is_err());
 }
}
assert_eq!(instance.capture_persistent_state().event_variables,[1e-30]);
"#,
    )
    .unwrap_or_else(|error| panic!("{error}"));
}

#[test]
fn generated_bound_step_in_child_modules_reaches_the_parent() {
    let source = r#"
module leaf(p,n);
 inout p,n; electrical p,n;
 parameter real cap=2e-9;
 analog begin
  if(V(p,n)>0.0) $bound_step(cap);
  I(p,n)<+V(p,n)*1e-3;
 end
endmodule
module nested(p,n);
 inout p,n; electrical p,n;
 leaf inner(p,n);
endmodule
module top(p,n);
 inout p,n; electrical p,n;
 nested a(p,n);
 leaf #(.cap(1e-9)) b(p,n);
endmodule
"#;
    let (state, stamp, noise) = generated_parts_selected(source, "hierarchical bound", Some("top"));
    run_generated_main(
        "hierarchical bound",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for voltage in [1.0,-1.0,1.0] {
 let voltages=[voltage,0.0];
 let ctx=runtime::GeneratedEvalContext { voltages:&voltages,temperature:300.0 };
 instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
 assert!(!ctx.evaluation_failed());
 assert_eq!(instance.transient_step_bound().unwrap(),(voltage>0.0).then_some(1e-9));
 instance.validate_advance_state().unwrap();
 instance.apply_validated_advance_state();
}
"#,
    )
    .unwrap_or_else(|error| panic!("{error}"));
}

struct ImmediatePipelineCancellation;

impl PipelineControl for ImmediatePipelineCancellation {
    fn is_cancelled(&self) -> bool {
        true
    }
}

#[test]
fn transpiler_honors_cancellation_before_cfg_lowering() {
    let (name, source) = fixtures()[0];
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
    let error = RustTranspiler::new(options())
        .transpile_measured_with_control(&artifact, &ImmediatePipelineCancellation)
        .expect_err("immediate cancellation must prevent CFG lowering");

    assert_eq!(error.kind, RustBackendErrorKind::Cancelled);
    assert!(error.message.contains("cfg_lowering"), "{error}");
}

struct CancelInsideDifferentiation {
    preparation_complete: AtomicBool,
    polls_after_preparation: AtomicUsize,
}

impl PipelineControl for CancelInsideDifferentiation {
    fn is_cancelled(&self) -> bool {
        if !self.preparation_complete.load(Ordering::Relaxed) {
            return false;
        }
        // The first poll is the Differentiation phase boundary. Let the pass
        // enter, then cancel at its first internal cooperative checkpoint.
        self.polls_after_preparation.fetch_add(1, Ordering::Relaxed) >= 1
    }

    fn phase_completed(
        &self,
        timing: rspice_veriloga::PhaseTiming,
        _metrics: &rspice_veriloga::PipelineMetrics,
    ) {
        if timing.phase == PipelinePhase::DerivativePreparation {
            self.preparation_complete.store(true, Ordering::Relaxed);
        }
    }
}

#[test]
fn transpiler_polls_for_cancellation_inside_differentiation() {
    let (name, source) = fixtures()[0];
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
    let control = CancelInsideDifferentiation {
        preparation_complete: AtomicBool::new(false),
        polls_after_preparation: AtomicUsize::new(0),
    };
    let error = RustTranspiler::new(options())
        .transpile_measured_with_control(&artifact, &control)
        .expect_err("cancellation poll inside differentiation must stop lowering");

    assert_eq!(error.kind, RustBackendErrorKind::Cancelled);
    assert!(error.message.contains("differentiation"), "{error}");
}

struct CancelInsideStructuralSpecialization {
    scheduling_complete: AtomicBool,
    polls_after_scheduling: AtomicUsize,
}

impl PipelineControl for CancelInsideStructuralSpecialization {
    fn is_cancelled(&self) -> bool {
        if !self.scheduling_complete.load(Ordering::Relaxed) {
            return false;
        }
        // The first poll is the StampEmission boundary. Let it enter, then
        // cancel at the first poll in the variant's CFG optimization.
        self.polls_after_scheduling.fetch_add(1, Ordering::Relaxed) >= 1
    }

    fn phase_completed(
        &self,
        timing: rspice_veriloga::PhaseTiming,
        _metrics: &rspice_veriloga::PipelineMetrics,
    ) {
        if timing.phase == PipelinePhase::Scheduling {
            self.scheduling_complete.store(true, Ordering::Relaxed);
        }
    }
}

#[test]
fn structural_specialization_propagates_cancellation() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(&repeated_structure_source())
        .expect("front end");
    let control = CancelInsideStructuralSpecialization {
        scheduling_complete: AtomicBool::new(false),
        polls_after_scheduling: AtomicUsize::new(0),
    };
    let error = RustTranspiler::new(options())
        .transpile_measured_with_control(&artifact, &control)
        .expect_err("the variant optimizer must honor cancellation");

    assert_eq!(error.kind, RustBackendErrorKind::Cancelled);
    assert!(error.message.contains("cfg_optimization"), "{error}");
    assert!(
        control.polls_after_scheduling.load(Ordering::Relaxed) >= 2,
        "the cancellation must occur after stamp emission began"
    );
}

fn find<'a>(files: &[(&'a str, &'a str)], name: &str, model: &str) -> &'a str {
    files
        .iter()
        .find(|(path, _)| *path == name)
        .map(|(_, contents)| *contents)
        .unwrap_or_else(|| panic!("{model}: no {name} was generated"))
}

fn generated_parts(source: &str, model: &str) -> (String, String, String) {
    generated_parts_selected(source, model, None)
}

fn generated_parts_selected(
    source: &str,
    model: &str,
    selected: Option<&str>,
) -> (String, String, String) {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir_module(source, selected)
        .unwrap_or_else(|error| panic!("{model}: front end failed: {error:?}"));
    let device = canonical::generate_device(&artifact, &options())
        .unwrap_or_else(|error| panic!("{model}: generation failed: {error}"));
    let file = |name: &str| {
        device
            .files
            .iter()
            .find(|file| file.relative_path == name)
            .map(|file| file.contents.clone())
            .unwrap_or_else(|| panic!("{model}: no {name} was generated"))
    };
    (file("state.rs"), file("stamp.rs"), file("noise.rs"))
}

fn run_generated_main(
    name: &str,
    state: &str,
    stamp: &str,
    noise: &str,
    main_body: &str,
) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             fn main() {{\n{}\n}}\n",
            indent(state),
            indent(stamp),
            indent(noise),
            indent(main_body),
        ),
    )
    .map_err(|error| error.to_string())?;
    let binary = root.join(format!("probe{}", std::env::consts::EXE_SUFFIX));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "rustc exited with {}:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "generated probe exited with {}:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn compile(name: &str, state: &str, stamp: &str, noise: &str) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let lib = root.join("lib.rs");
    std::fs::write(
        &lib,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("--crate-type=lib")
        .arg("-A")
        .arg("warnings")
        .arg("--out-dir")
        .arg(&root)
        .arg(&lib)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    Err(String::from_utf8_lossy(&output.stderr).into_owned())
}

fn run_shared_model_cache(name: &str, state: &str, stamp: &str, noise: &str) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n\
             pub fn parameter(instance: &Instance, name: &str) -> f64 {{\n\
             \x20   let index = parameter_index_for_name(name).unwrap();\n\
             \x20   instance.params.values[index]\n\
             }}\n\
             pub fn instance_cache_valid(instance: &Instance) -> bool {{ instance.canonical_instance_valid }}\n\
}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             fn main() {{\n\
             \x20   let mut first = device::state::Instance::new(&[0, 1]);\n\
             \x20   let mut second = device::state::Instance::new(&[0, 1]);\n\
             \x20   first.set_parameter(\"width\", 1.0e-6).unwrap();\n\
             \x20   second.set_parameter(\"width\", 2.0e-6).unwrap();\n\
             \x20   let voltages = [0.25, 0.0];\n\
             \x20   let ctx = runtime::GeneratedEvalContext {{ voltages: &voltages, temperature: 300.15 }};\n\
             \x20   let mut stamper = runtime::GeneratedStamper::default();\n\
             \x20   first.stamp(&ctx, &mut stamper);\n\
             \x20   second.stamp(&ctx, &mut stamper);\n\
             \x20   assert!(device::state::instance_cache_valid(&first));\n\
             \x20   let first_card = first.canonical_model_values.as_ref().unwrap();\n\
             \x20   let second_card = second.canonical_model_values.as_ref().unwrap();\n\
             \x20   assert!(std::sync::Arc::ptr_eq(first_card, second_card));\n\
             \x20   let before_values = first.params.values;\n\
             \x20   let before_given = first.param_given.clone();\n\
             \x20   assert!(first.set_parameter(\"width\", 2.0).is_err());\n\
             \x20   assert_eq!(first.params.values, before_values);\n\
             \x20   assert_eq!(first.param_given, before_given);\n\
             \x20   assert!(device::state::instance_cache_valid(&first));\n\
             \x20   first.set_parameter(\"width\", 3.0e-6).unwrap();\n\
             \x20   assert!((device::state::parameter(&first, \"area\") - 9.0e-12).abs() < 1.0e-24);\n\
             \x20   assert!(!device::state::instance_cache_valid(&first));\n\
             \x20   first.finalize_parameters().unwrap();\n\
             \x20   first.finalize_parameters().unwrap();\n\
             \x20   assert!((device::state::parameter(&first, \"area\") - 9.0e-12).abs() < 1.0e-24);\n\
             \x20   assert!(!device::state::instance_cache_valid(&first));\n\
             \x20   first.stamp(&ctx, &mut stamper);\n\
             \x20   assert!(device::state::instance_cache_valid(&first));\n\
             \x20   first.set_parameter(\"model_gain\", 4.0).unwrap();\n\
             \x20   first.stamp(&ctx, &mut stamper);\n\
             \x20   let changed_card = first.canonical_model_values.as_ref().unwrap();\n\
             \x20   assert!(!std::sync::Arc::ptr_eq(changed_card, second_card));\n\
             }}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let binary = root.join(format!(
        "shared_model_cache{}",
        std::env::consts::EXE_SUFFIX
    ));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated cache probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

fn run_dependent_parameter_defaults(
    name: &str,
    state: &str,
    stamp: &str,
    noise: &str,
) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n\
             pub fn parameter(instance: &Instance, name: &str) -> f64 {{\n\
             \x20   let index = parameter_index_for_name(name).unwrap();\n\
             \x20   read_parameter_slot(instance.params.as_ref(), index)\n\
             }}\n\
             pub fn parameter_given(instance: &Instance, name: &str) -> bool {{\n\
             \x20   let index = parameter_index_for_name(name).unwrap();\n\
             \x20   instance.param_given[index]\n\
             }}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             fn value(instance: &device::state::Instance, name: &str) -> f64 {{\n\
             \x20   device::state::parameter(instance, name)\n\
             }}\n\
             fn main() {{\n\
             \x20   let mut defaults = device::state::Instance::new(&[0, 1]);\n\
             \x20   defaults.finalize_parameters().unwrap();\n\
             \x20   defaults.validate_parameters().unwrap();\n\
             \x20   assert_eq!(value(&defaults, \"alias\"), 2.0);\n\
             \x20   assert_eq!(value(&defaults, \"chain\"), 6.0);\n\
             \x20   assert_eq!(value(&defaults, \"choice\"), 5.0);\n\
             \x20   assert_eq!(value(&defaults, \"given_sensitive\"), 5.0);\n\
             \x20   assert!(!device::state::parameter_given(&defaults, \"alias\"));\n\
             \x20   let mut overridden_base = device::state::Instance::new(&[0, 1]);\n\
             \x20   overridden_base.set_parameter(\"mode\", 1.0).unwrap();\n\
             \x20   overridden_base.set_parameter(\"base\", 4.0).unwrap();\n\
             \x20   overridden_base.finalize_parameters().unwrap();\n\
             \x20   overridden_base.validate_parameters().unwrap();\n\
             \x20   assert_eq!(value(&overridden_base, \"alias\"), 4.0);\n\
             \x20   assert_eq!(value(&overridden_base, \"chain\"), 12.0);\n\
             \x20   assert_eq!(value(&overridden_base, \"choice\"), 13.0);\n\
             \x20   assert_eq!(value(&overridden_base, \"given_sensitive\"), 113.0);\n\
             \x20   assert!(!device::state::parameter_given(&overridden_base, \"choice\"));\n\
             \x20   let mut explicit_dependent = device::state::Instance::new(&[0, 1]);\n\
             \x20   explicit_dependent.set_parameter(\"choice\", 7.0).unwrap();\n\
             \x20   explicit_dependent.set_parameter(\"base\", 4.0).unwrap();\n\
             \x20   explicit_dependent.set_parameter(\"mode\", 1.0).unwrap();\n\
             \x20   explicit_dependent.finalize_parameters().unwrap();\n\
             \x20   explicit_dependent.validate_parameters().unwrap();\n\
             \x20   assert_eq!(value(&explicit_dependent, \"chain\"), 12.0);\n\
             \x20   assert_eq!(value(&explicit_dependent, \"choice\"), 7.0);\n\
             \x20   assert_eq!(value(&explicit_dependent, \"given_sensitive\"), 107.0);\n\
             \x20   assert!(device::state::parameter_given(&explicit_dependent, \"choice\"));\n\
             \x20   let mut same_value_given = device::state::Instance::new(&[0, 1]);\n\
             \x20   same_value_given.set_parameter(\"base\", 2.0).unwrap();\n\
             \x20   same_value_given.finalize_parameters().unwrap();\n\
             \x20   same_value_given.validate_parameters().unwrap();\n\
             \x20   assert_eq!(value(&same_value_given, \"given_sensitive\"), 105.0);\n\
             \x20   let mut post_construction = device::state::Instance::new(&[0, 1]);\n\
             \x20   post_construction.set_parameter(\"base\", 3.0).unwrap();\n\
             \x20   assert_eq!(value(&post_construction, \"chain\"), 9.0);\n\
             \x20   post_construction.set_parameter(\"base\", 5.0).unwrap();\n\
             \x20   assert_eq!(value(&post_construction, \"chain\"), 15.0);\n\
             \x20   post_construction.finalize_parameters().unwrap();\n\
             \x20   post_construction.finalize_parameters().unwrap();\n\
             \x20   assert_eq!(value(&post_construction, \"chain\"), 15.0);\n\
             \x20   let before_values = post_construction.params.values;\n\
             \x20   let before_given = post_construction.param_given.clone();\n\
             \x20   let error = post_construction.set_parameter(\"bounded_source\", 6.0).unwrap_err();\n\
             \x20   assert!(error.contains(\"bounded_dependent\"), \"{{error}}\");\n\
             \x20   assert_eq!(post_construction.params.values, before_values);\n\
             \x20   assert_eq!(post_construction.param_given, before_given);\n\
             }}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let binary = root.join(format!(
        "dependent_parameter_defaults{}",
        std::env::consts::EXE_SUFFIX
    ));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated parameter probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

fn run_noise_call_order(name: &str, state: &str, stamp: &str, noise: &str) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             #[derive(Default)]\n\
             struct Capture(Vec<(usize, bool, u64, Option<u64>, Vec<u64>)>);\n\
             impl runtime::GeneratedNoiseVisitor for Capture {{\n\
             \x20   fn visit(&mut self, index: usize, value: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool {{\n\
             \x20       self.0.push((index, value.active, value.psd.to_bits(), value.exponent.map(f64::to_bits), value.table_operands.iter().map(|value| value.to_bits()).collect()));\n\
             \x20       true\n\
             \x20   }}\n\
             }}\n\
             fn noise(instance: &device::state::Instance, ctx: &runtime::GeneratedEvalContext<'_>) -> Vec<(usize, bool, u64, Option<u64>, Vec<u64>)> {{\n\
             \x20   let mut capture = Capture::default();\n\
             \x20   instance.evaluate_noise_sources(ctx, &mut capture).unwrap();\n\
             \x20   capture.0\n\
             }}\n\
             fn stamp(instance: &mut device::state::Instance, ctx: &runtime::GeneratedEvalContext<'_>) {{\n\
             \x20   let mut stamper = runtime::GeneratedStamper::default();\n\
             \x20   instance.stamp(ctx, &mut stamper);\n\
             }}\n\
             fn main() {{\n\
             \x20   let bias_a = [0.25, 0.0];\n\
             \x20   let bias_b = [-0.4, 0.1];\n\
             \x20   let ctx_a = runtime::GeneratedEvalContext {{ voltages: &bias_a, temperature: 300.15 }};\n\
             \x20   let ctx_b = runtime::GeneratedEvalContext {{ voltages: &bias_b, temperature: 340.0 }};\n\
             \x20   let mut instance = device::state::Instance::new(&[0, 1]);\n\
             \x20   let fresh = noise(&instance, &ctx_a);\n\
             \x20   assert_eq!(noise(&instance, &ctx_a), fresh);\n\
             \x20   stamp(&mut instance, &ctx_a);\n\
             \x20   assert_eq!(noise(&instance, &ctx_a), fresh);\n\
             \x20   stamp(&mut instance, &ctx_b);\n\
             \x20   assert_eq!(noise(&instance, &ctx_a), fresh);\n\
             \x20   let saved = instance.capture_persistent_state();\n\
             \x20   let mut restored = device::state::Instance::new(&[0, 1]);\n\
             \x20   restored.restore_persistent_state(&saved).unwrap();\n\
             \x20   assert_eq!(noise(&restored, &ctx_a), fresh);\n\
             \x20   instance.set_parameter(\"width\", 2.0e-6).unwrap();\n\
             \x20   let changed = noise(&instance, &ctx_a);\n\
             \x20   let mut changed_fresh = device::state::Instance::new(&[0, 1]);\n\
             \x20   changed_fresh.set_parameter(\"width\", 2.0e-6).unwrap();\n\
             \x20   assert_eq!(changed, noise(&changed_fresh, &ctx_a));\n\
             \x20   assert_eq!(noise(&instance, &ctx_b), noise(&changed_fresh, &ctx_b));\n\
             }}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let binary = root.join(format!("noise_call_order{}", std::env::consts::EXE_SUFFIX));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-O")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated noise probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

fn run_structural_variants(
    name: &str,
    state: &str,
    stamp: &str,
    noise: &str,
) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             fn evaluate(instance: &mut device::state::Instance, mode: f64) -> f64 {{\n\
             \x20   instance.set_parameter(\"mode\", mode).unwrap();\n\
             \x20   let voltages = [0.25, 0.0];\n\
             \x20   let ctx = runtime::GeneratedEvalContext {{ voltages: &voltages, temperature: 300.15 }};\n\
             \x20   let mut sink = [0.0];\n\
             \x20   let mut stamper = runtime::GeneratedStamper {{ sink: Some(&mut sink) }};\n\
             \x20   instance.stamp(&ctx, &mut stamper);\n\
             \x20   sink[0]\n\
             }}\n\
             fn main() {{\n\
             \x20   let mut instance = device::state::Instance::new(&[0, 1]);\n\
             \x20   let enabled = evaluate(&mut instance, 1.0);\n\
             \x20   let disabled = evaluate(&mut instance, -1.0);\n\
             \x20   let enabled_again = evaluate(&mut instance, 1.0);\n\
             \x20   assert!(enabled.is_finite() && enabled > 0.0, \"{{enabled}}\");\n\
             \x20   assert_eq!(disabled.to_bits(), 0.0f64.to_bits());\n\
             \x20   assert_eq!(enabled_again.to_bits(), enabled.to_bits());\n\
             }}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let binary = root.join(format!(
        "structural_variants{}",
        std::env::consts::EXE_SUFFIX
    ));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-O")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated specialization probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

/// One generated file, as a module beside its siblings.
///
/// The three become sibling modules of one crate rather than one flat module,
/// which is the shape the real tree has: `stamp.rs` and `noise.rs` both reach
/// `Instance` through `super::state`, and both import from the runtime under
/// their own names. Flattening them makes those imports collide over nothing.
///
/// Only the inner attributes come out, because a `#![..]` is legal at the top of
/// a module but not after the module's first item, and the generated file writes
/// one that the surrounding stub already covers.
fn indent(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.starts_with("#!["))
        .collect::<Vec<_>>()
        .join("\n")
}

fn scratch() -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("canonical-device");
    std::fs::create_dir_all(&root).expect("scratch directory");
    root
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "resistor",
            r#"
module resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 250.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        ),
        (
            "capacitor",
            r#"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#,
        ),
        (
            "diode",
            r#"
module diode(a, c);
    inout a, c;
    electrical a, c;
    parameter real is = 1.0e-14;
    parameter real n = 1.0;
    analog begin
        I(a, c) <+ is * (exp(V(a, c) / (n * $vt)) - 1.0);
    end
endmodule
"#,
        ),
        // A guard, a temperature fold and a parameter prologue: the shape that
        // makes the invalidation split worth taking, so this exercises the
        // staged slots as well as the body.
        (
            "staged transistor",
            r#"
module staged(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real width = 1.0e-6;
    parameter real vth0 = 0.4;
    parameter real tnom = 300.15;
    real geometry, vth, vov, ids;
    analog begin
        geometry = width * width * 1.0e12;
        if (geometry > 1.0e-3) begin
            geometry = geometry * 2.0;
        end
        vth = vth0 - 1.0e-3 * ($temperature - tnom);
        vov = V(g, s) - vth;
        if (vov > 0.0) begin
            ids = geometry * vov * vov;
        end else begin
            ids = 0.0;
        end
        I(d, s) <+ 1.0e-6 * ids;
    end
endmodule
"#,
        ),
        // The same prologue, plus a contribution that reads no unknown at all.
        // Its residual is instance-class, so the stamp reads it from a slot —
        // and the Newton body has no staged operand of its own, which is what
        // makes the slot array's binding independent of what the body reads.
        (
            "staged transistor with a leakage floor",
            r#"
module floored(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real width = 1.0e-6;
    parameter real vth0 = 0.4;
    parameter real tnom = 300.15;
    parameter real ileak = 1.0e-12;
    real geometry, vth, vov, ids;
    analog begin
        geometry = width * width * 1.0e12;
        if (geometry > 1.0e-3) begin
            geometry = geometry * 2.0;
        end
        vth = vth0 - 1.0e-3 * ($temperature - tnom);
        vov = V(g, s) - vth;
        if (vov > 0.0) begin
            ids = geometry * vov * vov;
        end else begin
            ids = 0.0;
        end
        I(d, s) <+ 1.0e-6 * ids;
        I(d, s) <+ ileak * ileak;
    end
endmodule
"#,
        ),
        // A potential contribution, which stamps through a branch unknown
        // rather than a node pair.
        (
            "voltage source",
            r#"
module vsrc(p, n);
    inout p, n;
    electrical p, n;
    parameter real dc = 1.0;
    parameter real rs = 1.0e-3;
    analog V(p, n) <+ dc + rs * I(p, n);
endmodule
"#,
        ),
        // `idt`, which needs a history slot of its own and an initial condition
        // that is returned rather than integrated when there is no step.
        (
            "integrator",
            r#"
module integrator(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0e-6;
    parameter real start = 0.25;
    analog begin
        I(p, n) <+ gain * idt(V(p, n), start);
    end
endmodule
"#,
        ),
        // Noise, in the three shapes the descriptors distinguish. The table one
        // is here because its operands are the only magnitudes that reach the
        // visitor as a slice, and the guarded flicker because an inactive source
        // still has to be visited with the index its descriptor sits at.
        (
            "noisy resistor",
            r#"
module noisy_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0;
    analog begin
        I(p, n) <+ V(p, n) / r;
        I(p, n) <+ white_noise(4.0 * 1.380649e-23 * $temperature / r, "thermal");
    end
endmodule
"#,
        ),
        (
            "shared noise preprocessing",
            r#"
module shared_noise_preprocessing(p, n);
    inout p, n;
    electrical p, n;
    parameter real model_gain = 2.0;
    (* type = "instance" *) parameter real width = 1.0e-6;
    real a, b, c, d, e, geometry, thermal, current;
    analog begin
        a = model_gain * model_gain + 1.0;
        b = a * a + model_gain;
        c = sqrt(b + a);
        d = ln(c + b);
        e = exp(d * 0.01) + c;
        geometry = width * width * e;
        thermal = geometry * ($temperature + 273.15);
        current = thermal * V(p, n);
        I(p, n) <+ current;
        I(p, n) <+ white_noise(abs(thermal) * (1.0 + abs(V(p, n))), "shared");
    end
endmodule
"#,
        ),
        // A coherent noise injection that reads `ddx` is emitted by the
        // grouped-process extension, independently of the source-wise noise
        // slice. The recognized bounded-exponential idiom then leaves its
        // derivative helper only in that extension, where the enclosing
        // `noise.rs` still needs to import it.
        (
            "limited exponential derivative in noise",
            r#"
module limited_exp_derivative_noise(p, n);
    inout p, n;
    electrical p, n;
    real slope, process;
    analog function real bounded_exp;
        input x;
        begin
            if (x > 80.0) begin
                bounded_exp = 5.540622384e34 * (1.0 + x - 80.0);
            end else if (x < -80.0) begin
                bounded_exp = 1.804851387e-35;
            end else begin
                bounded_exp = exp(x);
            end
        end
    endfunction
    analog begin
        slope = ddx(bounded_exp(V(p, n)), V(p, n));
        process = white_noise(1.0, "bounded-exp-slope");
        I(p, n) <+ bounded_exp(V(p, n));
        I(p, n) <+ slope * process;
    end
endmodule
"#,
        ),
        (
            "guarded flicker and table noise",
            r#"
module noisy_transistor(d, g, s);
    inout d, g, s;
    electrical d, g, s;
    parameter real kf = 1.0e-25;
    parameter real af = 1.2;
    parameter real beta = 1.0e-3;
    parameter real vth = 0.4;
    real ids;
    analog begin
        ids = 0.0;
        if (V(g, s) > vth) begin
            ids = beta * (V(g, s) - vth) * (V(g, s) - vth);
            I(d, s) <+ flicker_noise(kf * ids, af, "flicker");
        end
        I(d, s) <+ ids;
        I(d, s) <+ white_noise(2.0 * 1.602176634e-19 * ids, "shot");
        I(g, s) <+ noise_table({1.0, 1.0e-20, 1.0e6, 1.0e-22}, "gate");
    end
endmodule
"#,
        ),
        // Parameter arithmetic *inside* a guard, read by a bias-dependent
        // expression inside the same guard. That makes the split's export a
        // value defined in an `if` arm, which has no name after it in Rust —
        // and the export list is emitted at the end of the stage.
        //
        // Every fixture above is one function deep, so none of them reaches
        // this and the corpus shipped source that would not compile. The
        // arithmetic is deliberately several operations long: `worth_splitting`
        // only slices when a stage removes enough work, and a two-line
        // instance section would decline and prove nothing.
        (
            "a guarded stage export",
            r#"
module guarded_stage_export(p, n);
    inout p, n;
    electrical p, n;
    parameter real sel = 1.0;
    parameter real a = 2.0;
    parameter real b = 3.0;
    real t1, t2, t3, t4, t5, t6;
    analog begin
        if (sel > 0.5) begin
            t1 = a * b;
            t2 = sqrt(t1 + a);
            t3 = ln(t2 + b);
            t4 = exp(t3 * 0.1);
            t5 = t4 * t3 + t2;
            t6 = t5 / (t1 + 1.0);
            I(p, n) <+ t6 * V(p, n);
        end else begin
            I(p, n) <+ a * V(p, n);
        end
    end
endmodule
"#,
        ),
        (
            "generated event controls",
            r#"
module generated_event_controls(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(cross(V(p, n), 1, 0.0, 0.0, 1)) count = count + 1.0;
        @(above(V(p, n), 0.0, 0.0, 1)) count = count + 10.0;
        @(timer(1.0, 2.0, 0.0, 1)) count = count + 100.0;
        I(p, n) <+ count;
    end
endmodule
"#,
        ),
    ]
}

/// Only what the emitted code calls, with the signatures it calls them by.
const RUNTIME_STUB: &str = concat!(
    r#"
#![allow(dead_code, non_snake_case, unused_parens, unused_variables, unused_mut, unused_imports)]

pub type Value = f64;
pub const DEFAULT_GMIN: f64 = 1e-12;
mod simparam {
"#,
    include_str!("../../rspice-veriloga-runtime/src/simparam.rs"),
    r#"
}
mod analog_effects {
"#,
    include_str!("../../rspice-veriloga-runtime/src/analog_effects.rs"),
    r#"
}
pub mod runtime {
    pub mod arithmetic {
        mod scalar {
"#,
    include_str!("../../rspice-veriloga-runtime/src/arithmetic/scalar.rs"),
    r#"
        }
        pub use scalar::*;
        mod circular {
"#,
    include_str!("../../rspice-veriloga-runtime/src/arithmetic/circular.rs"),
    r#"
        }
        pub use circular::{IdtModOrigin, IdtModOriginCheckpoint};
        mod scaled {
"#,
    include_str!("../../rspice-veriloga-runtime/src/arithmetic/scaled.rs"),
    r#"
        }
        pub use scaled::ScaledValue;
    }
    mod noise_frequency {
"#,
    include_str!("../../rspice-veriloga-runtime/src/noise_frequency.rs"),
    r#"
    }
    pub mod integer {
"#,
    include_str!("../../rspice-veriloga-runtime/src/integer.rs"),
    r#"
    }
    pub type Value = f64;
    pub use crate::simparam::{GeneratedSimulationParameters,SimulationParameter};
    pub use crate::analog_effects::*;

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct GeneratedVerilogAAcceptedStateShapeIdentity([u8; 32]);

    impl GeneratedVerilogAAcceptedStateShapeIdentity {
        pub const fn from_bytes(bytes: [u8; 32]) -> Self {
            Self(bytes)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GeneratedParameterOrigin {
        DeclaredScope,
        ModelCard,
        Instance,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GeneratedVerilogAParameterScope {
        Model,
        Instance,
        Dual,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GeneratedVerilogATerminalDirection {
        Input,
        Output,
        InOut,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedVerilogATerminalDescriptor {
        pub name: &'static str,
        pub direction: GeneratedVerilogATerminalDirection,
        pub discipline: &'static str,
        pub current_parameter: &'static str,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedVerilogAParameterBound {
        pub value: Value,
        pub exclusive: bool,
    }

    impl GeneratedVerilogAParameterBound {
        pub const fn inclusive(value: Value) -> Self {
            Self { value, exclusive: false }
        }

        pub const fn exclusive(value: Value) -> Self {
            Self { value, exclusive: true }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedVerilogAParameterDescriptor {
        pub name: &'static str,
        pub aliases: &'static [&'static str],
        pub scope: GeneratedVerilogAParameterScope,
        pub is_integer: bool,
        pub default: Option<Value>,
        pub minimum: Option<GeneratedVerilogAParameterBound>,
        pub maximum: Option<GeneratedVerilogAParameterBound>,
        pub excluded_values: &'static [Value],
        pub has_dynamic_constraints: bool,
    }

    impl GeneratedVerilogAParameterDescriptor {
        const fn with_scope(
            name: &'static str,
            default: Option<Value>,
            scope: GeneratedVerilogAParameterScope,
        ) -> Self {
            Self {
                name,
                aliases: &[],
                scope,
                is_integer: false,
                default,
                minimum: None,
                maximum: None,
                excluded_values: &[],
                has_dynamic_constraints: false,
            }
        }

        pub const fn model(name: &'static str, default: Option<Value>) -> Self {
            Self::with_scope(name, default, GeneratedVerilogAParameterScope::Model)
        }

        pub const fn instance(name: &'static str, default: Option<Value>) -> Self {
            Self::with_scope(name, default, GeneratedVerilogAParameterScope::Instance)
        }

        pub const fn dual(name: &'static str, default: Option<Value>) -> Self {
            Self::with_scope(name, default, GeneratedVerilogAParameterScope::Dual)
        }

        pub const fn integer(mut self) -> Self {
            self.is_integer = true;
            self
        }

        pub const fn aliases(mut self, aliases: &'static [&'static str]) -> Self {
            self.aliases = aliases;
            self
        }

        pub const fn minimum(mut self, minimum: GeneratedVerilogAParameterBound) -> Self {
            self.minimum = Some(minimum);
            self
        }

        pub const fn maximum(mut self, maximum: GeneratedVerilogAParameterBound) -> Self {
            self.maximum = Some(maximum);
            self
        }

        pub const fn excluded_values(mut self, excluded_values: &'static [Value]) -> Self {
            self.excluded_values = excluded_values;
            self
        }

        pub const fn dynamic_constraints(mut self) -> Self {
            self.has_dynamic_constraints = true;
            self
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedParameterAssignment<'a> {
        pub name: &'a str,
        pub value: Value,
        pub origin: GeneratedParameterOrigin,
    }

    impl<'a> GeneratedParameterAssignment<'a> {
        pub const fn new(
            name: &'a str,
            value: Value,
            origin: GeneratedParameterOrigin,
        ) -> Self {
            Self { name, value, origin }
        }

        pub const fn for_declared_scope(name: &'a str, value: Value) -> Self {
            Self::new(name, value, GeneratedParameterOrigin::DeclaredScope)
        }
    }

    #[derive(Clone, Copy)]
    pub struct Lanes<const N: usize>(pub [f64; N]);

    impl<const N: usize> core::ops::Add for Lanes<N> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            let mut out = self.0;
            let mut i = 0;
            while i < N {
                out[i] = self.0[i] + rhs.0[i];
                i += 1;
            }
            Self(out)
        }
    }

    impl<const N: usize> core::ops::Sub for Lanes<N> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            let mut out = self.0;
            let mut i = 0;
            while i < N {
                out[i] = self.0[i] - rhs.0[i];
                i += 1;
            }
            Self(out)
        }
    }

    impl<const N: usize> core::ops::Mul<f64> for Lanes<N> {
        type Output = Self;
        fn mul(self, rhs: f64) -> Self {
            let mut out = self.0;
            let mut i = 0;
            while i < N {
                out[i] = self.0[i] * rhs;
                i += 1;
            }
            Self(out)
        }
    }

    impl<const N: usize> core::ops::Div<f64> for Lanes<N> {
        type Output = Self;
        fn div(self, rhs: f64) -> Self {
            let mut out = self.0;
            let mut i = 0;
            while i < N {
                out[i] = self.0[i] / rhs;
                i += 1;
            }
            Self(out)
        }
    }

    impl<const N: usize> core::ops::Index<usize> for Lanes<N> {
        type Output = f64;
        fn index(&self, index: usize) -> &f64 {
            &self.0[index]
        }
    }

    macro_rules! define_fixed_lanes {
        ($name:ident, $width:literal, [$($index:tt),+ $(,)?]) => {
            #[repr(transparent)]
            #[derive(Clone, Copy)]
            pub struct $name(pub [f64; $width]);

            impl $name {
                pub fn product_div(self, scalar: f64, divisor: f64) -> Self {
                    Self([$(arithmetic::product_div(self.0[$index], scalar, divisor)),+])
                }
                pub fn product_sum_div(self, scalar: f64, right: Self, right_scalar: f64, divisor: f64) -> Self {
                    Self([$(arithmetic::product_sum_div(
                        self.0[$index], scalar, right.0[$index], right_scalar, divisor
                    )),+])
                }
            }

            impl core::ops::Add for $name {
                type Output = Self;
                fn add(self, rhs: Self) -> Self {
                    Self([$((self.0[$index] + rhs.0[$index])),+])
                }
            }
            impl core::ops::Sub for $name {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self {
                    Self([$((self.0[$index] - rhs.0[$index])),+])
                }
            }
            impl core::ops::Mul<f64> for $name {
                type Output = Self;
                fn mul(self, rhs: f64) -> Self {
                    Self([$((self.0[$index] * rhs)),+])
                }
            }
            impl core::ops::Div<f64> for $name {
                type Output = Self;
                fn div(self, rhs: f64) -> Self {
                    Self([$((self.0[$index] / rhs)),+])
                }
            }
            impl core::ops::Index<usize> for $name {
                type Output = f64;
                fn index(&self, index: usize) -> &f64 {
                    &self.0[index]
                }
            }
        };
    }

    define_fixed_lanes!(L2, 2, [0, 1]);
    define_fixed_lanes!(L3, 3, [0, 1, 2]);
    define_fixed_lanes!(L4, 4, [0, 1, 2, 3]);
    define_fixed_lanes!(L5, 5, [0, 1, 2, 3, 4]);
    define_fixed_lanes!(L6, 6, [0, 1, 2, 3, 4, 5]);
    define_fixed_lanes!(L7, 7, [0, 1, 2, 3, 4, 5, 6]);
    define_fixed_lanes!(L8, 8, [0, 1, 2, 3, 4, 5, 6, 7]);
    define_fixed_lanes!(L9, 9, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    define_fixed_lanes!(L10, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    define_fixed_lanes!(L11, 11, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    define_fixed_lanes!(L12, 12, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    define_fixed_lanes!(L13, 13, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    define_fixed_lanes!(L14, 14, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    define_fixed_lanes!(L15, 15, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    define_fixed_lanes!(L16, 16, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    define_fixed_lanes!(L17, 17, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    define_fixed_lanes!(L18, 18, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]);
    define_fixed_lanes!(L19, 19, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
    define_fixed_lanes!(L20, 20, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
    define_fixed_lanes!(L21, 21, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    define_fixed_lanes!(L22, 22, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]);
    define_fixed_lanes!(L23, 23, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]);
    define_fixed_lanes!(L24, 24, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]);
    define_fixed_lanes!(L25, 25, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]);
    define_fixed_lanes!(L26, 26, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]);
    define_fixed_lanes!(L27, 27, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]);
    define_fixed_lanes!(L28, 28, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27]);
    define_fixed_lanes!(L29, 29, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28]);
    define_fixed_lanes!(L30, 30, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]);
    define_fixed_lanes!(L31, 31, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
    define_fixed_lanes!(L32, 32, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);

    pub fn install_generated_stage_values(
        destination: &mut [f64],
        values: &[f64],
        slots: &[u32],
    ) {
        assert_eq!(values.len(), slots.len());
        for (&value, &slot) in values.iter().zip(slots) {
            destination[slot as usize] = value;
        }
    }

    pub fn find_generated_parameter_index(
        sorted_names: &[&str],
        parameter_indices: &[u16],
        name: &str,
    ) -> Option<usize> {
        assert_eq!(sorted_names.len(), parameter_indices.len());
        let mut left = 0usize;
        let mut right = sorted_names.len();
        while left < right {
            let middle = left + (right - left) / 2;
            if sorted_names[middle] < name {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        (sorted_names.get(left).copied() == Some(name))
            .then(|| usize::from(parameter_indices[left]))
    }

    pub fn rspice_limexp(x: f64) -> f64 {
        if x < 80.0 {
            x.exp()
        } else {
            (80.0f64).exp() * (x - 80.0 + 1.0)
        }
    }

    pub fn rspice_limited_exp(x: f64) -> f64 {
        if x > 80.0 {
            5.54062238439351e34 * (x - 80.0 + 1.0)
        } else if x < -80.0 {
            1.804851387e-35
        } else {
            x.exp()
        }
    }

    pub fn rspice_limited_exp_derivative(x: f64) -> f64 {
        if x > 80.0 {
            5.54062238439351e34
        } else if x < -80.0 {
            0.0
        } else {
            x.exp()
        }
    }

    mod integration {
"#,
    include_str!("../../rspice-veriloga-runtime/src/integration.rs"),
    r#"
    }
    pub use integration::*;

    #[derive(Debug, Clone, Copy)]
    pub struct GeneratedDdtCandidateError;

    #[allow(clippy::too_many_arguments)]
    pub fn rspice_eval_ddt<const STATE_COUNT: usize>(
        current: &mut [f64; STATE_COUNT],
        previous: &[f64; STATE_COUNT],
        older: &[f64; STATE_COUNT],
        initialized: &[bool; STATE_COUNT],
        derivative_current: &mut [f64; STATE_COUNT],
        derivative_previous: &[f64; STATE_COUNT],
        candidate_valid: &mut [bool; STATE_COUNT],
        coefficients: GeneratedDdtCoefficients,
        slot: usize,
        value: f64,
    ) -> Result<f64, GeneratedDdtCandidateError> {
        candidate_valid[slot] = false;
        let previous_value = if initialized[slot] { previous[slot] } else { value };
        let older_value = if initialized[slot] { older[slot] } else { value };
        let previous_derivative = if initialized[slot] { derivative_previous[slot] } else { 0.0 };
        let result = if coefficients.active {
            value * coefficients.derivative_scale
                - previous_value * coefficients.previous_value_scale
                - older_value * coefficients.older_value_scale
                - previous_derivative * coefficients.previous_derivative_scale
        } else {
            0.0
        };
        if !result.is_finite() {
            return Err(GeneratedDdtCandidateError);
        }
        current[slot] = value;
        derivative_current[slot] = result;
        candidate_valid[slot] = true;
        Ok(result)
    }

    #[derive(Copy, Clone)]
    pub struct GeneratedParameterBound {
        pub value: f64,
        pub label: &'static str,
    }

    pub const GENERATED_PARAMETER_BOUND_NONE: u16 = 0;
    pub const GENERATED_PARAMETER_MIN_EXCLUSIVE_FLAG: u8 = 1;
    pub const GENERATED_PARAMETER_MAX_EXCLUSIVE_FLAG: u8 = 2;

    pub fn validate_generated_finite_parameter(name: &str, value: f64) -> Result<(), String> {
        if !value.is_finite() {
            return Err(format!("parameter '{}' must be finite, got {}", name, value));
        }
        Ok(())
    }

    pub fn validate_generated_parameter_bounds(
        name: &str,
        value: f64,
        flags: u8,
        min: Option<GeneratedParameterBound>,
        max: Option<GeneratedParameterBound>,
        excluded: &[GeneratedParameterBound],
    ) -> Result<(), String> {
        if let Some(min) = min {
            let invalid = if flags & GENERATED_PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
                value <= min.value
            } else {
                value < min.value
            };
            if invalid {
                let operator = if flags & GENERATED_PARAMETER_MIN_EXCLUSIVE_FLAG != 0 { ">" } else { ">=" };
                return Err(format!("parameter '{}' must be {} {}, got {}", name, operator, min.label, value));
            }
        }
        if let Some(max) = max {
            let invalid = if flags & GENERATED_PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
                value >= max.value
            } else {
                value > max.value
            };
            if invalid {
                let operator = if flags & GENERATED_PARAMETER_MAX_EXCLUSIVE_FLAG != 0 { "<" } else { "<=" };
                return Err(format!("parameter '{}' must be {} {}, got {}", name, operator, max.label, value));
            }
        }
        for excluded in excluded {
            if value == excluded.value {
                return Err(format!("parameter '{}' must not equal {}, got {}", name, excluded.label, value));
            }
        }
        Ok(())
    }

    pub fn resolve_generated_parameter_bound(
        pool: &[GeneratedParameterBound],
        encoded: u16,
    ) -> Option<GeneratedParameterBound> {
        if encoded == GENERATED_PARAMETER_BOUND_NONE {
            None
        } else {
            Some(*pool.get(usize::from(encoded - 1)).expect("generated parameter-bound index is outside its pool"))
        }
    }

    pub fn validate_generated_parameter_bound_indices(
        name: &str,
        value: f64,
        flags: u8,
        pool: &[GeneratedParameterBound],
        min: u16,
        max: u16,
        excluded: &[u16],
    ) -> Result<(), String> {
        validate_generated_parameter_bounds(
            name,
            value,
            flags,
            resolve_generated_parameter_bound(pool, min),
            resolve_generated_parameter_bound(pool, max),
            &[],
        )?;
        for &encoded in excluded {
            let excluded = resolve_generated_parameter_bound(pool, encoded)
                .expect("generated parameter exclusion uses the absence sentinel");
            if value == excluded.value {
                return Err(format!("parameter '{}' must not equal {}, got {}", name, excluded.label, value));
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn validate_generated_parameter(
        name: &str,
        value: f64,
        integer: bool,
        min: Option<(f64, &str)>,
        min_exclusive: bool,
        max: Option<(f64, &str)>,
        max_exclusive: bool,
        excluded: &[(f64, &str)],
    ) -> Result<(), String> {
        validate_generated_finite_parameter(name, value)?;
        if integer && value.fract() != 0.0 {
            return Err(format!("parameter '{}' must be an integer, got {}", name, value));
        }
        if integer && (value < i32::MIN as f64 || value > i32::MAX as f64) {
            return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
        }
        if let Some((min, label)) = min {
            if (min_exclusive && value <= min) || (!min_exclusive && value < min) {
                let operator = if min_exclusive { ">" } else { ">=" };
                return Err(format!("parameter '{}' must be {} {}, got {}", name, operator, label, value));
            }
        }
        if let Some((max, label)) = max {
            if (max_exclusive && value >= max) || (!max_exclusive && value > max) {
                let operator = if max_exclusive { "<" } else { "<=" };
                return Err(format!("parameter '{}' must be {} {}, got {}", name, operator, label, value));
            }
        }
        for (excluded, label) in excluded {
            if value == *excluded {
                return Err(format!("parameter '{}' must not equal {}, got {}", name, label, value));
            }
        }
        Ok(())
    }

    pub fn boxed_zero_f64_array<const N: usize>() -> Box<[f64; N]> {
        let mut boxed = Box::<[f64; N]>::new_uninit();
        unsafe {
            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
            boxed.assume_init()
        }
    }

    pub fn boxed_zero_bool_array<const N: usize>() -> Box<[bool; N]> {
        let mut boxed = Box::<[bool; N]>::new_uninit();
        unsafe {
            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
            boxed.assume_init()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedDdtCoefficients {
        pub active: bool,
        pub derivative_scale: Value,
        pub previous_value_scale: Value,
        pub older_value_scale: Value,
        pub previous_derivative_scale: Value,
    }

    impl GeneratedDdtCoefficients {
        pub const fn inactive() -> Self {
            Self {
                active: false,
                derivative_scale: 0.0,
                previous_value_scale: 0.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            }
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct GeneratedVerilogAPersistentState {
        pub ddt_previous: Vec<Value>,
        pub ddt_older: Vec<Value>,
        pub ddt_derivative_previous: Vec<Value>,
        pub ddt_initialized: Vec<bool>,
        pub idt_previous: Vec<Value>,
        pub idt_older: Vec<Value>,
        pub idt_input_previous: Vec<Value>,
        pub idt_initialized: Vec<bool>,
        pub event_variables: Vec<Value>,
        pub limiter_anchor: Vec<Value>,
        pub limiter_initialized: Vec<bool>,
    }

    mod event_control {
"#,
    include_str!("../../rspice-veriloga-runtime/src/event_control.rs"),
    r#"
    }
    pub use event_control::*;

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct GeneratedVerilogARollbackState {
        pub values: Vec<Value>,
        pub flags: Vec<bool>,
        pub analog_effects: Option<Box<AnalogEffectJournal>>,
    }

    static TASKS_ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
    static SIMPARAM_OVERRIDE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(f64::NAN.to_bits());
    pub fn set_simparam_override(value: Option<f64>) { SIMPARAM_OVERRIDE.store(value.unwrap_or(f64::NAN).to_bits(), std::sync::atomic::Ordering::SeqCst); }
    static EVALUATION_FAILED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    pub fn set_tasks_enabled(enabled: bool) { TASKS_ENABLED.store(enabled, std::sync::atomic::Ordering::SeqCst); }
    pub fn clear_evaluation_error() { EVALUATION_FAILED.store(false, std::sync::atomic::Ordering::SeqCst); }

    static DYNAMIC_OPERATORS_ENABLED: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(true);
    static ANALYSIS_INITIAL_STEP: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);
    static ANALYSIS_FINAL_STEP: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);
    static ANALYSIS_TRAN: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);
    static ANALYSIS_STATIC: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);

    pub fn set_dynamic_operators_enabled(enabled: bool) {
        DYNAMIC_OPERATORS_ENABLED.store(enabled, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_analysis_steps(initial_step: bool, final_step: bool) {
        ANALYSIS_INITIAL_STEP.store(initial_step, std::sync::atomic::Ordering::SeqCst);
        ANALYSIS_FINAL_STEP.store(final_step, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_event_analysis(transient: bool, static_analysis: bool) {
        ANALYSIS_TRAN.store(transient, std::sync::atomic::Ordering::SeqCst);
        ANALYSIS_STATIC.store(static_analysis, std::sync::atomic::Ordering::SeqCst);
    }

    pub struct GeneratedEvalContext<'a> {
        pub voltages: &'a [Value],
        pub temperature: Value,
    }

    impl GeneratedEvalContext<'_> {
        pub fn limiting_enabled(&self) -> bool { true }
        pub fn node_voltage(&self, node: usize) -> Value {
            self.voltages.get(node).copied().unwrap_or(0.0)
        }
        pub fn branch_current(&self, branch: usize) -> Value {
            self.voltages.get(branch).copied().unwrap_or(0.0)
        }
        pub fn temperature(&self) -> Value {
            self.temperature
        }
        pub fn thermal_voltage(&self) -> Value {
            self.temperature * 8.617_333_262e-5
        }
        pub fn analysis(&self, query: &str) -> bool {
            ((query.eq_ignore_ascii_case("ac") || query == "__rspice_scope_ac") && self.temperature == 123.0)
                || ((query.eq_ignore_ascii_case("tran") || query == "__rspice_scope_tran")
                    && ANALYSIS_TRAN.load(std::sync::atomic::Ordering::SeqCst))
                || (query.eq_ignore_ascii_case("static") && self.analysis_static())
        }
        pub fn analysis_code(&self) -> u8 {
            if self.analysis("ac") { 1 } else if self.analysis("tran") { 2 } else { 0 }
        }
        pub fn analysis_initial_step(&self) -> bool {
            self.dynamic_operators_enabled() && ANALYSIS_INITIAL_STEP.load(std::sync::atomic::Ordering::SeqCst)
        }
        pub fn analysis_final_step(&self) -> bool {
            self.dynamic_operators_enabled() && ANALYSIS_FINAL_STEP.load(std::sync::atomic::Ordering::SeqCst)
        }
        pub fn analysis_tran(&self) -> bool {
            ANALYSIS_TRAN.load(std::sync::atomic::Ordering::SeqCst)
        }
        pub fn analysis_static(&self) -> bool {
            ANALYSIS_STATIC.load(std::sync::atomic::Ordering::SeqCst)
        }
        pub fn simparam_or(&self, name: &str, fallback: Value) -> Value {
            let value = f64::from_bits(SIMPARAM_OVERRIDE.load(std::sync::atomic::Ordering::SeqCst));
            match name {
                "gmin" => 1e-12, "tnom" => 27.0, "simulatorVersion" => 1.0, "simulatorSubversion" => 0.0,
                "pnjmaxi" if !value.is_nan() => value,
                _ => fallback,
            }
        }
        pub fn has_simparam(&self, name: &str) -> bool {
            !self.simparam_or(name,f64::NAN).is_nan()
        }
        pub fn simparam_required(&self, name: &str) -> Value {
            let value=self.simparam_or(name,f64::NAN);
            if value.is_nan() { EVALUATION_FAILED.store(true,std::sync::atomic::Ordering::SeqCst); }
            value
        }
        pub fn dynamic_operators_enabled(&self) -> bool {
            DYNAMIC_OPERATORS_ENABLED.load(std::sync::atomic::Ordering::SeqCst)
        }
        pub fn report_ddt_candidate_error(&self, _slot: usize, _source: GeneratedDdtCandidateError) {}
        pub fn report_idt_candidate_error(&self, _slot: usize, _source: GeneratedIdtCandidateError) {}
        pub fn report_event_control_error(&self, _operator: &'static str, _slot: usize, _source: GeneratedEventControlError) { EVALUATION_FAILED.store(true, std::sync::atomic::Ordering::SeqCst); }
        pub fn analog_tasks_enabled(&self) -> bool { TASKS_ENABLED.load(std::sync::atomic::Ordering::SeqCst) }
        pub fn evaluation_failed(&self) -> bool { EVALUATION_FAILED.load(std::sync::atomic::Ordering::SeqCst) }
        pub fn integer_result(&self, result: Result<f64, integer::IntegerRuntimeError>) -> f64 {
            result.unwrap_or_else(|_| { EVALUATION_FAILED.store(true, std::sync::atomic::Ordering::SeqCst); f64::NAN })
        }
        pub fn checked_derivative_value(&self, primal: f64, derivative: f64) -> f64 {
            if primal.is_finite() && derivative.is_finite() { derivative }
            else { EVALUATION_FAILED.store(true, std::sync::atomic::Ordering::SeqCst); f64::NAN }
        }
        pub fn report_small_signal_error(&self, _: &'static str) { EVALUATION_FAILED.store(true,std::sync::atomic::Ordering::SeqCst); }
        pub fn check_noise_evaluation(&self) -> Result<(), GeneratedNoiseEvaluationError> {
            if self.evaluation_failed() { Err(GeneratedNoiseEvaluationError::NonFinite { index:0,quantity:"evaluation",value:f64::NAN }) } else { Ok(()) }
        }
        pub fn report_discontinuity_degree_error(&self) { EVALUATION_FAILED.store(true, std::sync::atomic::Ordering::SeqCst); }
        pub fn report_initialization_error(&self, _slot: usize) { EVALUATION_FAILED.store(true, std::sync::atomic::Ordering::SeqCst); }
        pub fn report_analog_task_error(&self, _site: u32, _source: AnalogEffectError) { EVALUATION_FAILED.store(true, std::sync::atomic::Ordering::SeqCst); }
    }

    #[derive(Default)]
    pub struct GeneratedStamper<'a> {
        pub sink: Option<&'a mut [Value]>,
    }

    impl GeneratedStamper<'_> {
        pub fn stamp_current_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _value: Value,
            _node_indices: [usize; NODE_COUNT],
            _node_derivatives: [Value; NODE_COUNT],
            _branch_indices: [usize; BRANCH_COUNT],
            _branch_derivatives: [Value; BRANCH_COUNT],
            _scale: Value,
        ) {
            // Extended captures retain a small complete matrix and residual
            // for tests that eliminate private solver unknowns.
            if let Some(sink) = self.sink.as_deref_mut().filter(|sink| sink.len() >= 32) {
                for (row, sign) in [(_pos, 1.0), (_neg, -1.0)] {
                    if let Some(row) = row.filter(|row| *row < 4) {
                        sink[28 + row] += sign * _value * _scale;
                        for (col, derivative) in _node_indices.iter().zip(_node_derivatives) {
                            if *col < 4 { sink[12 + 4 * row + *col] += sign * derivative * _scale; }
                        }
                    }
                }
            }
            if let Some(sink) = self.sink.as_deref_mut()
                && let Some(first) = sink.first_mut()
            {
                *first += _value;
            }
            if let Some(sink) = self.sink.as_deref_mut()
                && let Some(value) = sink.get_mut(9)
            {
                *value += _value;
            }
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(10) {
                    *value += _node_indices.iter().zip(_node_derivatives).filter(|(node, _)| **node == 0).map(|(_, value)| value).sum::<f64>() * _scale;
                }
                if let Some(value) = sink.get_mut(11) {
                    *value += _node_indices.iter().zip(_node_derivatives).filter(|(node, _)| **node == 1).map(|(_, value)| value).sum::<f64>() * _scale;
                }
            }
        }

        pub fn stamp_branch_current_local(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _branch: usize,
            _multiplicity: Value,
        ) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(0) { *value -= _multiplicity; }
            }
        }

        pub fn stamp_potential_branch_local(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _branch: usize,
            _multiplicity: Value,
        ) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(0) { *value += 1.0; }
                if let Some(value) = sink.get_mut(7) { *value += _branch as f64 + 1.0; }
            }
        }

        pub fn stamp_inactive_potential_branch_local(&mut self, _branch: usize) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(1) { *value += 1.0; }
                if let Some(value) = sink.get_mut(8) { *value += _branch as f64 + 1.0; }
            }
        }

        pub fn stamp_potential_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
            &mut self,
            _branch: usize,
            _value: Value,
            _node_indices: [usize; NODE_COUNT],
            _node_derivatives: [Value; NODE_COUNT],
            _branch_indices: [usize; BRANCH_COUNT],
            _branch_derivatives: [Value; BRANCH_COUNT],
        ) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(2) { *value += _value; }
                if let Some(value) = sink.get_mut(3) {
                    *value += _node_derivatives.first().copied().unwrap_or(0.0);
                }
                if let Some(value) = sink.get_mut(4) {
                    *value += _branch_derivatives.iter().sum::<f64>();
                }
                if let Some(value) = sink.get_mut(5) { *value += _branch as f64 + 1.0; }
                if let Some(value) = sink.get_mut(6) {
                    *value += _branch_indices.iter().map(|index| *index as f64 + 1.0).sum::<f64>();
                }
                if let Some(value) = sink.get_mut(9) {
                    *value += _branch_indices.iter().zip(_branch_derivatives).map(|(index, derivative)| (*index as f64 + 1.0) * derivative).sum::<f64>();
                }
            }
        }
    }

    pub static FREQUENCY_OMEGA: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1.0_f64.to_bits());

    pub struct GeneratedDerivative { node: Option<usize>, value: Value }
    impl GeneratedDerivative {
        pub fn node(node: usize, value: Value) -> Self { Self { node: Some(node), value } }
        pub fn branch(_branch: usize, value: Value) -> Self { Self { node: None, value } }
    }

    #[derive(Default)]
    pub struct GeneratedReactiveStamper<'a> {
        pub sink: Option<&'a mut [Value]>,
    }

    impl GeneratedReactiveStamper<'_> {
        pub fn scaled_frequency_coefficient(&self, ctx: &GeneratedEvalContext<'_>, coefficient: Value, scale: Value, ddt: u32, idt: u32) -> Option<Value> {
            let omega = f64::from_bits(FREQUENCY_OMEGA.load(std::sync::atomic::Ordering::SeqCst));
            if !omega.is_finite() || omega < 0.0 || (omega == 0.0 && idt > 0) {
                ctx.report_initialization_error(0);
                return None;
            }
            let power = ddt as i32 - idt as i32;
            let value = (coefficient * scale) * omega.powi(power);
            Some(if power.rem_euclid(4) >= 2 { -value } else { value })
        }

        pub fn stamp_current_frequency_local<const REAL: bool>(&mut self, _pos: Option<usize>, _neg: Option<usize>, derivative: GeneratedDerivative) {
            let offset = if REAL { 3 } else { 0 };
            if let Some(sink) = self.sink.as_deref_mut() {
                if derivative.node == Some(0) {
                    if let Some(value) = sink.get_mut(offset) { *value += derivative.value; }
                } else if derivative.node.is_none() {
                    if let Some(value) = sink.get_mut(offset + 1) { *value += derivative.value; }
                }
                if derivative.node.is_some() {
                    if let Some(value) = sink.get_mut(offset + 2) { *value += derivative.value.abs(); }
                }
            }
        }

        pub fn stamp_potential_frequency_local<const REAL: bool>(&mut self, _branch: usize, derivative: GeneratedDerivative) {
            self.stamp_current_frequency_local::<REAL>(None, None, derivative);
        }

        pub fn stamp_current_reactive_indexed_dense_local(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _nodes: &[usize],
            _node_derivatives: &[Value],
            _branches: &[usize],
            _branch_derivatives: &[Value],
            _scale: Value,
        ) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(0) {
                    *value += _node_derivatives.first().copied().unwrap_or(0.0) * _scale;
                }
                if let Some(value) = sink.get_mut(1) {
                    *value += _branch_derivatives.iter().sum::<f64>() * _scale;
                }
                if let Some(value) = sink.get_mut(2) {
                    *value += _node_derivatives.iter().map(|entry| entry.abs()).sum::<f64>() * _scale;
                }
            }
        }

        pub fn stamp_potential_reactive_indexed_dense_local(
            &mut self,
            _branch: usize,
            _nodes: &[usize],
            _node_derivatives: &[Value],
            _branches: &[usize],
            _branch_derivatives: &[Value],
        ) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(0) { *value += _branch as f64 + 1.0; }
                if let Some(value) = sink.get_mut(1) {
                    *value += _node_derivatives.first().copied().unwrap_or(0.0);
                }
                if let Some(value) = sink.get_mut(2) {
                    *value += _branch_derivatives.iter().sum::<f64>();
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GeneratedNoiseKind {
        White,
        Flicker,
        Table,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedNoiseEndpoint {
        pub local_node: Option<usize>,
        pub name: &'static str,
        pub is_internal: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedNoiseDescriptor {
        pub mechanism: &'static str,
        pub label: Option<&'static str>,
        pub kind: GeneratedNoiseKind,
        pub equation: usize,
        pub is_current: bool,
        pub branch_ordinal: Option<usize>,
        pub pos: GeneratedNoiseEndpoint,
        pub neg: GeneratedNoiseEndpoint,
        pub table_len: usize,
        pub table_log_interp: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedNoiseProcessDescriptor {
        pub process_id: usize,
        pub label: Option<&'static str>,
        pub kind: GeneratedNoiseKind,
        pub table_len: usize,
        pub table_log_interp: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedNoiseInjectionDescriptor {
        pub process_id: usize,
        pub equation: usize,
        pub is_current: bool,
        pub branch_ordinal: Option<usize>,
        pub pos: GeneratedNoiseEndpoint,
        pub neg: GeneratedNoiseEndpoint,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    pub struct GeneratedNoiseComplex { pub re: Value, pub im: Value }
    impl GeneratedNoiseComplex {
        pub fn scaled_transfer(real: Value, reactive: Value, frequency_hz: Value, scale: Value) -> Self {
            let omega=core::f64::consts::TAU*frequency_hz;
            let ordinary=(omega*reactive)*scale;
            let im=if omega.is_normal() && ordinary.is_normal() { ordinary }
                else { ((reactive*scale)*frequency_hz)*core::f64::consts::TAU };
            Self { re: real*scale, im }
        }
        pub fn is_finite(self) -> bool { self.re.is_finite() && self.im.is_finite() }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedNoiseInjectionEvaluation {
        pub descriptor: usize,
        pub gain: GeneratedNoiseComplex,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedNoiseProcessEvaluationRef<'a> {
        pub active: bool,
        pub psd: Value,
        pub exponent: Option<Value>,
        pub table_operands: &'a [Value],
        pub injections: &'a [GeneratedNoiseInjectionEvaluation],
    }

    pub trait GeneratedNoiseProcessVisitor {
        fn visit_process(&mut self, index: usize, evaluation: GeneratedNoiseProcessEvaluationRef<'_>) -> bool;
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GeneratedNoiseEvaluation {
        pub active: bool,
        pub psd: Value,
        pub exponent: Option<Value>,
        pub table_operands: Vec<Value>,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedNoiseEvaluationRef<'a> {
        pub active: bool,
        pub psd: Value,
        pub exponent: Option<Value>,
        pub table_operands: &'a [Value],
    }

    pub trait GeneratedNoiseVisitor {
        fn visit(&mut self, index: usize, evaluation: GeneratedNoiseEvaluationRef<'_>) -> bool;
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum GeneratedNoiseEvaluationError {
        UninitializedAnalogState,
        SourceIndexOutOfRange { index: usize, count: usize },
        NonFinite { index: usize, quantity: &'static str, value: Value },
        NegativePower { index: usize, value: Value },
        InvalidMultiplicity { value: Value },
        InvalidFrequency { value: Value },
        NonFiniteGain { process: usize, injection: usize, re: Value, im: Value },
    }
}
"#
);

#[test]
fn generated_discontinuity_in_children_preserves_both_hints() {
    let source = r#"
module leaf(p,n);
 inout p,n; electrical p,n;
 parameter real degree=0;
 analog begin
  if(V(p,n)>0.0) $discontinuity(degree);
  I(p,n)<+V(p,n)*1e-3;
 end
endmodule
module top(p,n);
 inout p,n; electrical p,n;
 leaf #(.degree(1)) transient_hint(p,n);
 leaf #(.degree(-1)) newton_hint(p,n);
endmodule
"#;
    let (state, stamp, noise) =
        generated_parts_selected(source, "hierarchical discontinuity", Some("top"));
    run_generated_main(
        "hierarchical discontinuity",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0,1]);
instance.finalize_parameters().unwrap();
for voltage in [1.0,-1.0,1.0] {
 let voltages=[voltage,0.0];
 let ctx=runtime::GeneratedEvalContext { voltages:&voltages,temperature:300.0 };
 instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
 assert!(!ctx.evaluation_failed());
 assert_eq!(instance.discontinuity_rising(),voltage>0.0);
 assert_eq!(instance.limiter_converged(),voltage<0.0);
 instance.validate_advance_state().unwrap();
 instance.apply_validated_advance_state();
}
"#,
    )
    .unwrap_or_else(|error| panic!("{error}"));
}

#[test]
fn generated_indirect_tolerances_follow_parameter_updates_and_activation() {
    let (state, stamp, noise) = generated_parts(
        "module tolerance(p); inout p; electrical p;
        parameter real tolerance=2e-9; parameter integer enabled=1;
        analog if(enabled) V(p): ddt(V(p),tolerance*2)==I(p); endmodule",
        "indirect tolerances",
    );
    run_generated_main(
        "indirect tolerances",
        &state,
        &stamp,
        &noise,
        r#"
let mut instance=device::state::Instance::new(&[0]);
instance.set_branch_indices(&[1]);
let ctx=runtime::GeneratedEvalContext {voltages:&[0.0,0.0],temperature:300.15};
for (value,enabled,expected) in [(3e-9,1.0,6e-9),(8e-9,0.0,1e-12),(0.0,1.0,0.0)] {
    instance.set_parameter("tolerance",value).unwrap();
    instance.set_parameter("enabled",enabled).unwrap();
    instance.stamp(&ctx,&mut runtime::GeneratedStamper::default());
    let mut values=Vec::new();
    instance.visit_equation_abstols(7, 1e-12, |row,tol|values.push((row,tol)));
    assert_eq!(values,[(8,expected)],"branch ordinal 1 follows seven solver nodes");
    assert!(!ctx.evaluation_failed());
}
assert!(instance.set_parameter("tolerance",-1.0).unwrap_err().contains("absolute tolerance"));
instance.visit_equation_abstols(7, 1e-12, |_,tol|assert_eq!(tol,0.0));
assert!(instance.set_parameter("tolerance",f64::MAX).is_err());
instance.set_parameter("tolerance",1e-9).unwrap();
instance.visit_equation_abstols(7, 1e-12, |_,tol|assert_eq!(tol,2e-9));
"#,
    )
    .unwrap_or_else(|report| panic!("{report}"));
}
