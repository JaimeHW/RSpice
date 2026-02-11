use super::*;

#[test]
fn test_execute_const() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushConst(42.0)]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 42.0).abs() < 1e-10);
}

#[test]
fn test_execute_add() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(2.0),
        Instruction::PushConst(3.0),
        Instruction::Add,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 5.0).abs() < 1e-10);
}

#[test]
fn test_execute_sub() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(10.0),
        Instruction::PushConst(3.0),
        Instruction::Sub,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 7.0).abs() < 1e-10);
}

#[test]
fn test_execute_mul() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(4.0),
        Instruction::PushConst(5.0),
        Instruction::Mul,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 20.0).abs() < 1e-10);
}

#[test]
fn test_execute_div() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(15.0),
        Instruction::PushConst(3.0),
        Instruction::Div,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 5.0).abs() < 1e-10);
}

#[test]
fn test_execute_voltage() {
    let mut ctx = VmContext::new(2);
    ctx.voltages[0] = 5.0;
    ctx.voltages[1] = 2.0;

    let mut vm = Vm::new(&mut ctx);
    let program = make_program(vec![Instruction::PushVoltage(0, 1)]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 3.0).abs() < 1e-10);
}

#[test]
fn test_execute_param() {
    let mut ctx = VmContext::default();
    ctx.set_param(0, 0.001);

    let mut vm = Vm::new(&mut ctx);
    let program = make_program(vec![Instruction::PushParam(0)]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 0.001).abs() < 1e-10);
}

#[test]
fn test_execute_exp() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::Exp]);
    let result = vm.execute(&program).unwrap();

    assert!((result - std::f64::consts::E).abs() < 1e-10);
}

#[test]
fn test_execute_sqrt() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushConst(16.0), Instruction::Sqrt]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 4.0).abs() < 1e-10);
}

#[test]
fn test_execute_neg() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushConst(5.0), Instruction::Neg]);
    let result = vm.execute(&program).unwrap();

    assert!((result + 5.0).abs() < 1e-10);
}

#[test]
fn test_execute_limexp() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // Small value - normal exp
    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::Limexp]);
    let result = vm.execute(&program).unwrap();
    assert!((result - std::f64::consts::E).abs() < 1e-10);

    // Large value - limited
    let program = make_program(vec![Instruction::PushConst(100.0), Instruction::Limexp]);
    let result = vm.execute(&program).unwrap();
    // Should be linearized above 40
    assert!(result < f64::INFINITY);
    assert!(result > 40.0_f64.exp()); // Must be larger than exp(40)
}

#[test]
fn test_execute_ifelse_true() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(1.0),  // condition (true)
        Instruction::PushConst(10.0), // then
        Instruction::PushConst(20.0), // else
        Instruction::IfElse,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 10.0).abs() < 1e-10);
}

#[test]
fn test_execute_ifelse_false() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(0.0),  // condition (false)
        Instruction::PushConst(10.0), // then
        Instruction::PushConst(20.0), // else
        Instruction::IfElse,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 20.0).abs() < 1e-10);
}

#[test]
fn test_execute_vt() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushVt]);
    let result = vm.execute(&program).unwrap();

    // At 300.15K (27°C), Vt ≈ 0.02587 V
    assert!((result - 0.02587).abs() < 0.0001);
}

#[test]
fn test_execute_diode_equation() {
    // I = Is * (exp(V/Vt) - 1)
    let mut ctx = VmContext::new(2);
    ctx.voltages[0] = 0.7;
    ctx.voltages[1] = 0.0;
    ctx.set_param(0, 1e-14); // Is

    let mut vm = Vm::new(&mut ctx);

    // Program: Is * (exp(V(0,1) / Vt) - 1)
    let program = make_program(vec![
        Instruction::PushParam(0),      // Is
        Instruction::PushVoltage(0, 1), // V
        Instruction::PushVt,            // Vt
        Instruction::Div,               // V/Vt
        Instruction::Exp,               // exp(V/Vt)
        Instruction::PushConst(1.0),
        Instruction::Sub, // exp(V/Vt) - 1
        Instruction::Mul, // Is * (exp(V/Vt) - 1)
    ]);
    let result = vm.execute(&program).unwrap();

    // At 0.7V, current should be in the mA range
    assert!(result > 1e-4);
    assert!(result < 1.0);
}

#[test]
fn test_execute_complex_expression() {
    // Test: (3 + 4) * (5 - 2) = 21
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::PushConst(4.0),
        Instruction::Add,
        Instruction::PushConst(5.0),
        Instruction::PushConst(2.0),
        Instruction::Sub,
        Instruction::Mul,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 21.0).abs() < 1e-10);
}

#[test]
fn test_execute_trig() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    // sin(0) = 0
    let program = make_program(vec![Instruction::PushConst(0.0), Instruction::Sin]);
    let result = vm.execute(&program).unwrap();
    assert!(result.abs() < 1e-10);

    // cos(0) = 1
    let program = make_program(vec![Instruction::PushConst(0.0), Instruction::Cos]);
    let result = vm.execute(&program).unwrap();
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_stack_underflow() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::Add]);
    let result = vm.execute(&program);

    assert!(result.is_err());
}

#[test]
fn test_context_vt_temperature_dependence() {
    let mut ctx = VmContext::default();
    ctx.temperature = 300.0;

    // At 300K
    let vt_300 = ctx.vt();

    // At 400K
    ctx.temperature = 400.0;
    let vt_400 = ctx.vt();

    // Vt should scale linearly with temperature
    assert!((vt_400 / vt_300 - 400.0 / 300.0).abs() < 0.001);
}

// ========================================================================
// Additional Coverage Tests
// ========================================================================

#[test]
fn test_execute_pow() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(2.0),
        Instruction::PushConst(3.0),
        Instruction::Pow,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 8.0).abs() < 1e-10);
}

#[test]
fn test_execute_log() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(std::f64::consts::E),
        Instruction::Log,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_execute_log10() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushConst(100.0), Instruction::Log10]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 2.0).abs() < 1e-10);
}

#[test]
fn test_execute_abs() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushConst(-7.5), Instruction::Abs]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 7.5).abs() < 1e-10);
}

#[test]
fn test_execute_min() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Min,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 3.0).abs() < 1e-10);
}

#[test]
fn test_execute_max() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(5.0),
        Instruction::PushConst(3.0),
        Instruction::Max,
    ]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 5.0).abs() < 1e-10);
}

#[test]
fn test_execute_sinh() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::Sinh]);
    let result = vm.execute(&program).unwrap();

    // sinh(1) ≈ 1.1752
    assert!((result - 1.0_f64.sinh()).abs() < 1e-10);
}

#[test]
fn test_execute_cosh() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::Cosh]);
    let result = vm.execute(&program).unwrap();

    // cosh(1) ≈ 1.5431
    assert!((result - 1.0_f64.cosh()).abs() < 1e-10);
}

#[test]
fn test_execute_tanh() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![Instruction::PushConst(1.0), Instruction::Tanh]);
    let result = vm.execute(&program).unwrap();

    // tanh(1) ≈ 0.7616
    assert!((result - 1.0_f64.tanh()).abs() < 1e-10);
}

#[test]
fn test_execute_tan() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![
        Instruction::PushConst(std::f64::consts::PI / 4.0),
        Instruction::Tan,
    ]);
    let result = vm.execute(&program).unwrap();

    // tan(pi/4) = 1
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_execute_time() {
    let mut ctx = VmContext::default();
    ctx.time = 1.5e-9;

    let mut vm = Vm::new(&mut ctx);
    let program = make_program(vec![Instruction::PushTime]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 1.5e-9).abs() < 1e-20);
}

#[test]
fn test_execute_temperature() {
    let mut ctx = VmContext::default();
    ctx.temperature = 350.0;

    let mut vm = Vm::new(&mut ctx);
    let program = make_program(vec![Instruction::PushTemperature]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 350.0).abs() < 1e-10);
}

#[test]
fn test_execute_current() {
    let mut ctx = VmContext::new(2);
    ctx.currents.push(0.001);

    let mut vm = Vm::new(&mut ctx);
    let program = make_program(vec![Instruction::PushCurrent(0, 1)]);
    let result = vm.execute(&program).unwrap();

    assert!((result - 0.001).abs() < 1e-10);
}

#[test]
fn test_context_branch_current_lookup_bidirectional() {
    let mut ctx = VmContext::new(2);
    ctx.set_branch_current(0, 1, 2.5e-3);

    assert!((ctx.current(0, 1) - 2.5e-3).abs() < 1e-12);
    assert!((ctx.current(1, 0) + 2.5e-3).abs() < 1e-12);
}

#[test]
fn test_context_branch_current_clear_reverts_to_vector_fallback() {
    let mut ctx = VmContext::new(2);
    ctx.currents.push(1.0e-3);
    ctx.set_branch_current(0, 1, 2.0e-3);
    assert!((ctx.current(0, 1) - 2.0e-3).abs() < 1e-12);

    ctx.clear_currents();
    assert!(ctx.currents.is_empty());
    assert!(ctx.current(0, 1).abs() < 1e-15);
}

#[test]
fn test_vm_error_display() {
    let err = VmError::StackUnderflow("test");
    assert!(err.to_string().contains("Stack underflow"));

    let err = VmError::InvalidInstruction("test");
    assert!(err.to_string().contains("Invalid instruction"));
}

#[test]
fn test_context_set_variable() {
    let mut ctx = VmContext::default();
    ctx.set_variable(2, std::f64::consts::PI);

    assert_eq!(ctx.variables.len(), 3);
    assert!((ctx.variables[2] - std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_empty_program() {
    let mut ctx = VmContext::default();
    let mut vm = Vm::new(&mut ctx);

    let program = make_program(vec![]);
    let result = vm.execute(&program);

    // Empty program = underflow (no result)
    assert!(result.is_err());
}

// =========================================================================
