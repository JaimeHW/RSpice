use super::*;

// Transient State Tracking Tests
// ========================================================================

#[test]
fn test_context_with_states() {
    let ctx = VmContext::with_states(2, 3);
    assert_eq!(ctx.state_values.len(), 3);
    assert_eq!(ctx.state_values_prev.len(), 3);
    assert_eq!(ctx.timestep, 0.0);
}

#[test]
fn test_context_allocate_states() {
    let mut ctx = VmContext::new(2);
    assert!(ctx.state_values.is_empty());

    ctx.allocate_states(5);
    assert_eq!(ctx.state_values.len(), 5);
    assert_eq!(ctx.state_values_prev.len(), 5);
}

#[test]
fn test_context_advance_state() {
    let mut ctx = VmContext::with_states(2, 2);
    ctx.state_values[0] = 1.0;
    ctx.state_values[1] = 2.0;

    ctx.advance_state();

    assert!((ctx.state_values_prev[0] - 1.0).abs() < 1e-10);
    assert!((ctx.state_values_prev[1] - 2.0).abs() < 1e-10);
}

#[test]
fn test_context_set_timestep() {
    let mut ctx = VmContext::new(2);
    ctx.set_timestep(1e-6);
    assert!((ctx.timestep - 1e-6).abs() < 1e-15);
}

#[test]
fn test_execute_ddt_state_transient() {
    // Test ddt with known values
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(0.001); // 1ms timestep
    ctx.state_values_prev[0] = 1.0; // Previous value

    let mut vm = Vm::new(&mut ctx);

    // ddt(2.0) with prev=1.0, dt=0.001 => (2.0 - 1.0) / 0.001 = 1000
    let program = make_program(vec![
        Instruction::PushConst(2.0), // Current value
        Instruction::DdtState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 1000.0).abs() < 1e-10,
        "ddt: expected 1000, got {result}"
    );
}

#[test]
fn test_execute_ddt_state_dc() {
    // In DC analysis (dt=0), ddt should return 0
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(0.0); // DC: no timestep
    ctx.state_values_prev[0] = 1.0;

    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(100.0),
        Instruction::DdtState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(result.abs() < 1e-10, "ddt in DC should be 0, got {result}");
}

#[test]
fn test_execute_idt_state_transient() {
    // Test idt accumulation
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(0.001); // 1ms
    ctx.state_values_prev[0] = 5.0; // Previous integral value

    let mut vm = Vm::new(&mut ctx);

    // idt(current=10) => prev + current * dt = 5.0 + 10.0 * 0.001 = 5.01
    let program = make_program(vec![Instruction::PushConst(10.0), Instruction::IdtState(0)]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 5.01).abs() < 1e-10,
        "idt: expected 5.01, got {result}"
    );
}

#[test]
fn test_execute_idt_state_dc() {
    // In DC analysis, idt returns previous accumulated value
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(0.0); // DC
    ctx.state_values_prev[0] = std::f64::consts::PI;

    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(100.0),
        Instruction::IdtState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - std::f64::consts::PI).abs() < 1e-10,
        "idt in DC should be prev value (pi), got {result}"
    );
}

#[test]
fn test_ddt_capacitor_current() {
    // Simulate I = C * dV/dt for a capacitor
    // C = 1e-6 F, dV = 1V over dt = 1e-3s => I = 1e-6 * 1 / 1e-3 = 1e-3 A
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(1e-3);
    ctx.state_values_prev[0] = 0.0; // Previous voltage

    let mut vm = Vm::new(&mut ctx);

    // I = C * ddt(V)
    // ddt(1.0) with prev=0, dt=1e-3 => 1000
    // I = 1e-6 * 1000 = 1e-3
    let program = make_program(vec![
        Instruction::PushConst(1e-6), // C
        Instruction::PushConst(1.0),  // Current voltage
        Instruction::DdtState(0),     // dV/dt
        Instruction::Mul,             // C * dV/dt
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 1e-3).abs() < 1e-12,
        "Capacitor current: expected 1e-3, got {result}"
    );
}

#[test]
fn test_multiple_state_variables() {
    // Test that different state indices work independently
    let mut ctx = VmContext::with_states(2, 3);
    ctx.set_timestep(0.01);
    ctx.state_values_prev[0] = 0.0;
    ctx.state_values_prev[1] = 10.0;
    ctx.state_values_prev[2] = 100.0;

    let mut vm = Vm::new(&mut ctx);

    // ddt at index 0: (1.0 - 0.0) / 0.01 = 100
    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::DdtState(0)]);
    assert!((vm.execute(&program).unwrap() - 100.0).abs() < 1e-10);

    // ddt at index 1: (11.0 - 10.0) / 0.01 = 100
    let program = make_program(vec![Instruction::PushConst(11.0), Instruction::DdtState(1)]);
    assert!((vm.execute(&program).unwrap() - 100.0).abs() < 1e-10);

    // ddt at index 2: (101.0 - 100.0) / 0.01 = 100
    let program = make_program(vec![
        Instruction::PushConst(101.0),
        Instruction::DdtState(2),
    ]);
    assert!((vm.execute(&program).unwrap() - 100.0).abs() < 1e-10);
}

#[test]
fn test_ddt_negative_slope() {
    // Test ddt with decreasing values (negative derivative)
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(0.001);
    ctx.state_values_prev[0] = 5.0;

    let mut vm = Vm::new(&mut ctx);

    // ddt(2.0) with prev=5.0 => (2.0 - 5.0) / 0.001 = -3000
    let program = make_program(vec![Instruction::PushConst(2.0), Instruction::DdtState(0)]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result + 3000.0).abs() < 1e-10,
        "ddt negative slope: expected -3000, got {result}"
    );
}

#[test]
fn test_idt_accumulation_sequence() {
    // Simulate multiple timesteps of integration
    // idt(constant) over 3 timesteps should accumulate
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(0.01);

    // Timestep 1: idt(10) with prev=0 => 0 + 10*0.01 = 0.1
    ctx.state_values_prev[0] = 0.0;
    let mut vm = Vm::new(&mut ctx);
    let program = make_program(vec![Instruction::PushConst(10.0), Instruction::IdtState(0)]);
    let result1 = vm.execute(&program).unwrap();
    assert!((result1 - 0.1).abs() < 1e-10);

    // Timestep 2: idt(10) with prev=0.1 => 0.1 + 10*0.01 = 0.2
    ctx.state_values_prev[0] = 0.1;
    let vm = Vm::new(&mut ctx);
    let mut vm = vm;
    let result2 = vm.execute(&program).unwrap();
    assert!((result2 - 0.2).abs() < 1e-10);

    // Timestep 3: idt(10) with prev=0.2 => 0.2 + 10*0.01 = 0.3
    ctx.state_values_prev[0] = 0.2;
    let mut vm = Vm::new(&mut ctx);
    let result3 = vm.execute(&program).unwrap();
    assert!((result3 - 0.3).abs() < 1e-10);
}

#[test]
fn test_idt_out_of_bounds_state() {
    // Out of bounds state index should use 0.0 as default prev
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(0.01);

    let mut vm = Vm::new(&mut ctx);

    // idt at index 999 (doesn't exist) => 0 + 5*0.01 = 0.05
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::IdtState(999),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 0.05).abs() < 1e-10,
        "Out of bounds idt should use prev=0: got {result}"
    );
}

#[test]
fn test_inductor_voltage() {
    // Simulate V = L * dI/dt for an inductor
    // L = 1e-3 H, dI = 2A over dt = 1e-3s => V = 1e-3 * 2 / 1e-3 = 2V
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(1e-3);
    ctx.state_values_prev[0] = 1.0; // Previous current

    let mut vm = Vm::new(&mut ctx);

    // V = L * ddt(I)
    // ddt(3.0) with prev=1.0, dt=1e-3 => 2000
    // V = 1e-3 * 2000 = 2.0
    let program = make_program(vec![
        Instruction::PushConst(1e-3), // L
        Instruction::PushConst(3.0),  // Current (I)
        Instruction::DdtState(0),     // dI/dt
        Instruction::Mul,             // L * dI/dt
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 2.0).abs() < 1e-10,
        "Inductor voltage: expected 2.0, got {result}"
    );
}

#[test]
fn test_very_small_timestep() {
    // Verify numerical stability with very small timestep
    let mut ctx = VmContext::with_states(2, 1);
    ctx.set_timestep(1e-15); // femtosecond
    ctx.state_values_prev[0] = 1.0;

    let mut vm = Vm::new(&mut ctx);

    // ddt(1.0 + 1e-15) => (1+1e-15 - 1) / 1e-15 = 1
    let program = make_program(vec![
        Instruction::PushConst(1.0 + 1e-15),
        Instruction::DdtState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    // At e-15 scale, floating point precision gives ~11% error which is expected
    assert!(
        (result - 1.0).abs() < 0.2,
        "Very small timestep ddt should be ~1: got {result}"
    );
}

// ========================================================================
// $limit Function Tests
// ========================================================================

#[test]
fn test_limit_no_clamping_needed() {
    // When delta is within step limit, value passes through unchanged
    let mut ctx = VmContext::with_states(2, 1);
    ctx.state_values_prev[0] = 0.5; // Previous value

    let mut vm = Vm::new(&mut ctx);

    // $limit(0.6, 0.7) with prev=0.5 => delta=0.1 < 0.7, so result=0.6
    let program = make_program(vec![
        Instruction::PushConst(0.6), // new value
        Instruction::PushConst(0.7), // step limit
        Instruction::LimitState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 0.6).abs() < 1e-10,
        "$limit: expected 0.6 (no clamping), got {result}"
    );
}

#[test]
fn test_limit_positive_clamping() {
    // When positive delta exceeds step limit, value is clamped
    let mut ctx = VmContext::with_states(2, 1);
    ctx.state_values_prev[0] = 0.5; // Previous value

    let mut vm = Vm::new(&mut ctx);

    // $limit(2.0, 0.3) with prev=0.5 => delta=1.5 > 0.3, so result=0.5+0.3=0.8
    let program = make_program(vec![
        Instruction::PushConst(2.0), // new value (big jump)
        Instruction::PushConst(0.3), // step limit
        Instruction::LimitState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 0.8).abs() < 1e-10,
        "$limit: expected 0.8 (clamped), got {result}"
    );
}

#[test]
fn test_limit_negative_clamping() {
    // When negative delta exceeds step limit, value is clamped
    let mut ctx = VmContext::with_states(2, 1);
    ctx.state_values_prev[0] = 1.0; // Previous value

    let mut vm = Vm::new(&mut ctx);

    // $limit(-1.0, 0.5) with prev=1.0 => delta=-2.0 < -0.5, so result=1.0-0.5=0.5
    let program = make_program(vec![
        Instruction::PushConst(-1.0), // new value (big negative jump)
        Instruction::PushConst(0.5),  // step limit
        Instruction::LimitState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 0.5).abs() < 1e-10,
        "$limit: expected 0.5 (negative clamped), got {result}"
    );
}

#[test]
fn test_limit_first_iteration() {
    // On first iteration (no state), new value should pass through
    let mut ctx = VmContext::with_states(2, 0); // No states allocated

    let mut vm = Vm::new(&mut ctx);

    // First iteration: prev defaults to new_value, so no clamping occurs
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(0.1),
        Instruction::LimitState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 5.0).abs() < 1e-10,
        "$limit first iter: expected 5.0, got {result}"
    );
}

#[test]
fn test_limit_diode_convergence_scenario() {
    // Simulate $limit usage in diode: prevents voltage from jumping too far
    // This mimics: V_new = $limit(V_calc, 0.7) where 0.7 ~ 2*Vt
    let mut ctx = VmContext::with_states(2, 1);
    ctx.state_values_prev[0] = 0.3; // Previous diode voltage

    let mut vm = Vm::new(&mut ctx);

    // Newton iteration wants to jump to V=10 (bad guess), but $limit clamps
    let program = make_program(vec![
        Instruction::PushConst(10.0), // Calculated new voltage (way off)
        Instruction::PushConst(0.7),  // Typical PN step limit (~2*Vt)
        Instruction::LimitState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    // Expected: 0.3 + 0.7 = 1.0 (clamped by step limit)
    assert!(
        (result - 1.0).abs() < 1e-10,
        "Diode $limit: expected 1.0, got {result}"
    );
}

#[test]
fn test_limit_exact_boundary() {
    // When delta exactly equals step limit, no clamping needed
    let mut ctx = VmContext::with_states(2, 1);
    ctx.state_values_prev[0] = 0.0;

    let mut vm = Vm::new(&mut ctx);

    // $limit(0.5, 0.5) with prev=0 => delta=0.5 == 0.5, so result=0.5
    let program = make_program(vec![
        Instruction::PushConst(0.5),
        Instruction::PushConst(0.5),
        Instruction::LimitState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 0.5).abs() < 1e-10,
        "$limit boundary: expected 0.5, got {result}"
    );
}

#[test]
fn test_limit_tiny_step() {
    // With very tiny step limit, value changes very slowly
    let mut ctx = VmContext::with_states(2, 1);
    ctx.state_values_prev[0] = 0.0;

    let mut vm = Vm::new(&mut ctx);

    // $limit(100.0, 0.001) with prev=0 => result=0.001
    let program = make_program(vec![
        Instruction::PushConst(100.0),
        Instruction::PushConst(0.001),
        Instruction::LimitState(0),
    ]);
    let result = vm.execute(&program).unwrap();
    assert!(
        (result - 0.001).abs() < 1e-10,
        "$limit tiny step: expected 0.001, got {result}"
    );
}

#[test]
fn test_limit_multiple_states() {
    // Multiple independent $limit states
    let mut ctx = VmContext::with_states(2, 3);
    ctx.state_values_prev[0] = 1.0;
    ctx.state_values_prev[1] = 2.0;
    ctx.state_values_prev[2] = 3.0;

    let mut vm = Vm::new(&mut ctx);

    // State 0: 5.0 from 1.0, step 0.5 => 1.5
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(0.5),
        Instruction::LimitState(0),
    ]);
    let r0 = vm.execute(&program).unwrap();
    assert!((r0 - 1.5).abs() < 1e-10, "State 0: expected 1.5, got {r0}");

    // State 1: 1.0 from 2.0, step 0.3 => 1.7
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(0.3),
        Instruction::LimitState(1),
    ]);
    let r1 = vm.execute(&program).unwrap();
    assert!((r1 - 1.7).abs() < 1e-10, "State 1: expected 1.7, got {r1}");

    // State 2: 3.1 from 3.0, step 0.5 => 3.1 (within limit)
    let program = make_program(vec![
        Instruction::PushConst(3.1),
        Instruction::PushConst(0.5),
        Instruction::LimitState(2),
    ]);
    let r2 = vm.execute(&program).unwrap();
    assert!((r2 - 3.1).abs() < 1e-10, "State 2: expected 3.1, got {r2}");
}

// ========================================================================
