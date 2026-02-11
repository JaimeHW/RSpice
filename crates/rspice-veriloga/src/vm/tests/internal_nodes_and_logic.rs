use super::*;

// Internal Node Tests
// =========================================================================

#[test]
fn test_context_with_internal_nodes() {
    let ctx = VmContext::with_internal_nodes(2, 3);
    assert_eq!(ctx.voltages.len(), 2);
    assert_eq!(ctx.internal_voltages.len(), 3);
}

#[test]
fn test_internal_voltage_access() {
    let mut ctx = VmContext::with_internal_nodes(2, 3);
    ctx.internal_voltages[0] = 1.5;
    ctx.internal_voltages[1] = 2.5;
    ctx.internal_voltages[2] = 3.5;

    assert!((ctx.internal_voltage(0) - 1.5).abs() < 1e-10);
    assert!((ctx.internal_voltage(1) - 2.5).abs() < 1e-10);
    assert!((ctx.internal_voltage(2) - 3.5).abs() < 1e-10);
}

#[test]
fn test_internal_voltage_out_of_bounds() {
    let ctx = VmContext::with_internal_nodes(2, 1);
    // Out of bounds returns 0.0
    assert!((ctx.internal_voltage(100)).abs() < 1e-15);
}

#[test]
fn test_execute_push_internal_voltage() {
    let mut ctx = VmContext::with_internal_nodes(2, 2);
    ctx.internal_voltages[0] = 0.7;
    ctx.internal_voltages[1] = 0.3;

    let mut vm = Vm::new(&mut ctx);
    let program = make_program(vec![Instruction::PushInternalVoltage(0)]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 0.7).abs() < 1e-10);
}

#[test]
fn test_internal_voltage_in_expression() {
    let mut ctx = VmContext::with_internal_nodes(2, 2);
    ctx.voltages[0] = 1.0;
    ctx.internal_voltages[0] = 0.5;

    let mut vm = Vm::new(&mut ctx);
    // Expression: V(0,1) - V_internal(0) = 1.0 - 0.5 = 0.5
    let program = make_program(vec![
        Instruction::PushVoltage(0, 1),
        Instruction::PushInternalVoltage(0),
        Instruction::Sub,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 0.5).abs() < 1e-10);
}

#[test]
fn test_internal_voltage_with_parameters() {
    let mut ctx = VmContext::with_internal_nodes(2, 1);
    ctx.internal_voltages[0] = 0.7;
    ctx.set_param(0, 1e-14); // Is

    let mut vm = Vm::new(&mut ctx);
    // Expression: Is * exp(V_internal(0) / Vt) (simplified diode on internal node)
    let program = make_program(vec![
        Instruction::PushParam(0),           // Is
        Instruction::PushInternalVoltage(0), // V_internal
        Instruction::PushVt,                 // Vt
        Instruction::Div,                    // V_internal / Vt
        Instruction::Exp,                    // exp(...)
        Instruction::Mul,                    // Is * exp(...)
    ]);
    let result = vm.execute(&program).unwrap();

    // Verify non-zero reasonable result
    assert!(result > 0.0);
    assert!(result < 1.0); // Should be small current
}

// ========================================================================
// Comparison Operator Tests
// ========================================================================

#[test]
fn test_execute_gt() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // 5 > 3 = true (1.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Gt,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // 3 > 5 = false (0.0)
    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::PushConst(5.0),
        Instruction::Gt,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

#[test]
fn test_execute_lt() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // 3 < 5 = true (1.0)
    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::PushConst(5.0),
        Instruction::Lt,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // 5 < 3 = false (0.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Lt,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

#[test]
fn test_execute_ge() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // 5 >= 5 = true (1.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(5.0),
        Instruction::Ge,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // 5 >= 3 = true (1.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Ge,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // 3 >= 5 = false (0.0)
    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::PushConst(5.0),
        Instruction::Ge,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

#[test]
fn test_execute_le() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // 5 <= 5 = true (1.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(5.0),
        Instruction::Le,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // 3 <= 5 = true (1.0)
    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::PushConst(5.0),
        Instruction::Le,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // 5 <= 3 = false (0.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Le,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

#[test]
fn test_execute_eq() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // 5 == 5 = true (1.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(5.0),
        Instruction::Eq,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // 5 == 3 = false (0.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Eq,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

#[test]
fn test_execute_ne() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // 5 != 3 = true (1.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Ne,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // 5 != 5 = false (0.0)
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(5.0),
        Instruction::Ne,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

// ========================================================================
// Logical Operator Tests
// ========================================================================

#[test]
fn test_execute_and() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // true && true = true
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(1.0),
        Instruction::And,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // true && false = false
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(0.0),
        Instruction::And,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);

    // false && true = false
    let program = make_program(vec![
        Instruction::PushConst(0.0),
        Instruction::PushConst(1.0),
        Instruction::And,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);

    // false && false = false
    let program = make_program(vec![
        Instruction::PushConst(0.0),
        Instruction::PushConst(0.0),
        Instruction::And,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

#[test]
fn test_execute_or() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // true || true = true
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(1.0),
        Instruction::Or,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // true || false = true
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(0.0),
        Instruction::Or,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // false || false = false
    let program = make_program(vec![
        Instruction::PushConst(0.0),
        Instruction::PushConst(0.0),
        Instruction::Or,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

#[test]
fn test_execute_not() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // !true = false
    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::Not]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);

    // !false = true
    let program = make_program(vec![Instruction::PushConst(0.0), Instruction::Not]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);
}

// ========================================================================
// Variable Tests
// ========================================================================

#[test]
fn test_execute_push_variable() {
    let mut ctx = VmContext::new(2);
    ctx.variables = vec![std::f64::consts::PI, 2.71, 1.618];

    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushVariable(0)]);
    assert!((vm.execute(&program).unwrap() - std::f64::consts::PI).abs() < 1e-10);

    let program = make_program(vec![Instruction::PushVariable(1)]);
    assert!((vm.execute(&program).unwrap() - 2.71).abs() < 1e-10);

    let program = make_program(vec![Instruction::PushVariable(2)]);
    assert!((vm.execute(&program).unwrap() - 1.618).abs() < 1e-10);
}

#[test]
fn test_execute_variable_in_expression() {
    let mut ctx = VmContext::new(2);
    ctx.variables = vec![10.0, 3.0];

    let mut vm = Vm::new(&mut ctx);

    // var[0] + var[1] = 10 + 3 = 13
    let program = make_program(vec![
        Instruction::PushVariable(0),
        Instruction::PushVariable(1),
        Instruction::Add,
    ]);
    assert!((vm.execute(&program).unwrap() - 13.0).abs() < 1e-10);
}

#[test]
fn test_execute_variable_out_of_bounds() {
    let mut ctx = VmContext::new(2);
    ctx.variables = vec![1.0];

    let mut vm = Vm::new(&mut ctx);

    // Accessing out-of-bounds variable returns 0.0
    let program = make_program(vec![Instruction::PushVariable(10)]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

// ========================================================================
// Complex Expression Tests with New Operators
// ========================================================================

#[test]
fn test_conditional_with_comparison() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // if (5 > 3) then 100 else 200 = 100
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Gt,
        Instruction::PushConst(100.0),
        Instruction::PushConst(200.0),
        Instruction::IfElse,
    ]);
    assert!((vm.execute(&program).unwrap() - 100.0).abs() < 1e-10);

    // if (3 > 5) then 100 else 200 = 200
    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::PushConst(5.0),
        Instruction::Gt,
        Instruction::PushConst(100.0),
        Instruction::PushConst(200.0),
        Instruction::IfElse,
    ]);
    assert!((vm.execute(&program).unwrap() - 200.0).abs() < 1e-10);
}

#[test]
fn test_complex_logical_expression() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // (5 > 3) && (10 < 20) = true && true = true
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Gt,
        Instruction::PushConst(10.0),
        Instruction::PushConst(20.0),
        Instruction::Lt,
        Instruction::And,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // (5 < 3) || (10 < 20) = false || true = true
    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Lt,
        Instruction::PushConst(10.0),
        Instruction::PushConst(20.0),
        Instruction::Lt,
        Instruction::Or,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);
}

// ========================================================================
// Inverse Trigonometric Function Tests
// ========================================================================

#[test]
fn test_execute_asin() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // asin(0) = 0
    let program = make_program(vec![Instruction::PushConst(0.0), Instruction::Asin]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);

    // asin(1) = π/2
    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::Asin]);
    assert!((vm.execute(&program).unwrap() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);

    // asin(0.5) = π/6
    let program = make_program(vec![Instruction::PushConst(0.5), Instruction::Asin]);
    assert!((vm.execute(&program).unwrap() - std::f64::consts::FRAC_PI_6).abs() < 1e-10);
}

#[test]
fn test_execute_acos() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // acos(1) = 0
    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::Acos]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);

    // acos(0) = π/2
    let program = make_program(vec![Instruction::PushConst(0.0), Instruction::Acos]);
    assert!((vm.execute(&program).unwrap() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);

    // acos(0.5) = π/3
    let program = make_program(vec![Instruction::PushConst(0.5), Instruction::Acos]);
    assert!((vm.execute(&program).unwrap() - std::f64::consts::FRAC_PI_3).abs() < 1e-10);
}

#[test]
fn test_execute_atan() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // atan(0) = 0
    let program = make_program(vec![Instruction::PushConst(0.0), Instruction::Atan]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);

    // atan(1) = π/4
    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::Atan]);
    assert!((vm.execute(&program).unwrap() - std::f64::consts::FRAC_PI_4).abs() < 1e-10);
}

#[test]
fn test_execute_atan2() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // atan2(1, 1) = π/4
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(1.0),
        Instruction::Atan2,
    ]);
    assert!((vm.execute(&program).unwrap() - std::f64::consts::FRAC_PI_4).abs() < 1e-10);

    // atan2(1, 0) = π/2
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(0.0),
        Instruction::Atan2,
    ]);
    assert!((vm.execute(&program).unwrap() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);

    // atan2(-1, 0) = -π/2
    let program = make_program(vec![
        Instruction::PushConst(-1.0),
        Instruction::PushConst(0.0),
        Instruction::Atan2,
    ]);
    assert!((vm.execute(&program).unwrap() + std::f64::consts::FRAC_PI_2).abs() < 1e-10);

    // atan2(0, 1) = 0
    let program = make_program(vec![
        Instruction::PushConst(0.0),
        Instruction::PushConst(1.0),
        Instruction::Atan2,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);
}

// ========================================================================
// Rounding Function Tests
// ========================================================================

#[test]
fn test_execute_floor() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // floor(3.7) = 3
    let program = make_program(vec![Instruction::PushConst(3.7), Instruction::Floor]);
    assert!((vm.execute(&program).unwrap() - 3.0).abs() < 1e-10);

    // floor(-3.7) = -4
    let program = make_program(vec![Instruction::PushConst(-3.7), Instruction::Floor]);
    assert!((vm.execute(&program).unwrap() + 4.0).abs() < 1e-10);

    // floor(5.0) = 5
    let program = make_program(vec![Instruction::PushConst(5.0), Instruction::Floor]);
    assert!((vm.execute(&program).unwrap() - 5.0).abs() < 1e-10);
}

#[test]
fn test_execute_ceil() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // ceil(3.2) = 4
    let program = make_program(vec![Instruction::PushConst(3.2), Instruction::Ceil]);
    assert!((vm.execute(&program).unwrap() - 4.0).abs() < 1e-10);

    // ceil(-3.2) = -3
    let program = make_program(vec![Instruction::PushConst(-3.2), Instruction::Ceil]);
    assert!((vm.execute(&program).unwrap() + 3.0).abs() < 1e-10);

    // ceil(5.0) = 5
    let program = make_program(vec![Instruction::PushConst(5.0), Instruction::Ceil]);
    assert!((vm.execute(&program).unwrap() - 5.0).abs() < 1e-10);
}

// ========================================================================
// Power Function Tests
// ========================================================================

#[test]
fn test_execute_fnpow() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // 2^3 = 8
    let program = make_program(vec![
        Instruction::PushConst(2.0),
        Instruction::PushConst(3.0),
        Instruction::FnPow,
    ]);
    assert!((vm.execute(&program).unwrap() - 8.0).abs() < 1e-10);

    // 10^0 = 1
    let program = make_program(vec![
        Instruction::PushConst(10.0),
        Instruction::PushConst(0.0),
        Instruction::FnPow,
    ]);
    assert!((vm.execute(&program).unwrap() - 1.0).abs() < 1e-10);

    // 4^0.5 = 2
    let program = make_program(vec![
        Instruction::PushConst(4.0),
        Instruction::PushConst(0.5),
        Instruction::FnPow,
    ]);
    assert!((vm.execute(&program).unwrap() - 2.0).abs() < 1e-10);

    // 2^-1 = 0.5
    let program = make_program(vec![
        Instruction::PushConst(2.0),
        Instruction::PushConst(-1.0),
        Instruction::FnPow,
    ]);
    assert!((vm.execute(&program).unwrap() - 0.5).abs() < 1e-10);
}

#[test]
fn test_execute_fnpow_edge_cases() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // 0^2 = 0
    let program = make_program(vec![
        Instruction::PushConst(0.0),
        Instruction::PushConst(2.0),
        Instruction::FnPow,
    ]);
    assert!(vm.execute(&program).unwrap().abs() < 1e-10);

    // e^1 = e
    let program = make_program(vec![
        Instruction::PushConst(std::f64::consts::E),
        Instruction::PushConst(1.0),
        Instruction::FnPow,
    ]);
    assert!((vm.execute(&program).unwrap() - std::f64::consts::E).abs() < 1e-10);
}

// ========================================================================
