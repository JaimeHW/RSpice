use super::*;
use crate::ir::IrExpr;

// ========================================================================
// Bytecode Program Tests
// ========================================================================

#[test]
fn test_bytecode_program_creation() {
    let mut program = BytecodeProgram::default();
    program.instructions.push(Instruction::PushConst(1.0));
    program.instructions.push(Instruction::PushConst(2.0));
    program.instructions.push(Instruction::Add);
    assert_eq!(program.instructions.len(), 3);
}

#[test]
fn test_instruction_variants() {
    // Test all instruction variants can be created
    let instructions = vec![
        Instruction::PushConst(std::f64::consts::PI),
        Instruction::PushParam(0),
        Instruction::PushVoltage(0, 1),
        Instruction::PushCurrent(0, 1),
        Instruction::PushTime,
        Instruction::PushTemperature,
        Instruction::PushVt,
        Instruction::Add,
        Instruction::Sub,
        Instruction::Mul,
        Instruction::Div,
        Instruction::Neg,
        Instruction::Pow,
        Instruction::Abs,
        Instruction::Sqrt,
        Instruction::Exp,
        Instruction::Log,
        Instruction::Log10,
        Instruction::Sin,
        Instruction::Cos,
        Instruction::Tan,
        Instruction::Sinh,
        Instruction::Cosh,
        Instruction::Tanh,
        Instruction::Min,
        Instruction::Max,
        Instruction::Limexp,
        Instruction::IfElse,
    ];
    assert_eq!(instructions.len(), 28);
}

// ========================================================================
// Code Generation Tests
// ========================================================================

#[test]
fn test_compile_const_expr() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();
    let expr = IrExpr::Const(42.0);
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    assert_eq!(program.instructions.len(), 1);
    assert!(
        matches!(program.instructions[0], Instruction::PushConst(v) if (v - 42.0).abs() < 1e-10)
    );
}

#[test]
fn test_compile_voltage_expr() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();
    let expr = IrExpr::Voltage(0, 1);
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    assert_eq!(program.instructions.len(), 1);
    assert!(matches!(
        program.instructions[0],
        Instruction::PushVoltage(0, 1)
    ));
}

#[test]
fn test_compile_param_expr() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();
    let expr = IrExpr::Param("g".into());
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    assert_eq!(program.instructions.len(), 1);
    assert!(matches!(program.instructions[0], Instruction::PushParam(0)));
}

#[test]
fn test_compile_binary_expr() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();
    // g * V(0,1)
    let expr = IrExpr::Binary(
        BinaryOp::Mul,
        Box::new(IrExpr::Param("g".into())),
        Box::new(IrExpr::Voltage(0, 1)),
    );
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    assert_eq!(program.instructions.len(), 3);
    assert!(matches!(program.instructions[0], Instruction::PushParam(0)));
    assert!(matches!(
        program.instructions[1],
        Instruction::PushVoltage(0, 1)
    ));
    assert!(matches!(program.instructions[2], Instruction::Mul));
}

#[test]
fn test_compile_function_expr() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();
    // exp(V(0,1))
    let expr = IrExpr::Call(crate::ir::IrFunction::Exp, vec![IrExpr::Voltage(0, 1)]);
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    assert_eq!(program.instructions.len(), 2);
    assert!(matches!(
        program.instructions[0],
        Instruction::PushVoltage(0, 1)
    ));
    assert!(matches!(program.instructions[1], Instruction::Exp));
}

#[test]
fn test_compile_conditional_expr() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();
    // cond ? 1.0 : 0.0
    let expr = IrExpr::Conditional(
        Box::new(IrExpr::Voltage(0, 1)),
        Box::new(IrExpr::Const(1.0)),
        Box::new(IrExpr::Const(0.0)),
    );
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    assert_eq!(program.instructions.len(), 4);
    assert!(matches!(program.instructions[3], Instruction::IfElse));
}

#[test]
fn test_compile_limexp() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();
    let expr = IrExpr::Limexp(Box::new(IrExpr::Const(20.0)));
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    assert_eq!(program.instructions.len(), 2);
    assert!(matches!(program.instructions[1], Instruction::Limexp));
}

#[test]
fn test_compile_negation() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();
    let expr = IrExpr::Unary(crate::ast::UnaryOp::Neg, Box::new(IrExpr::Const(5.0)));
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    assert_eq!(program.instructions.len(), 2);
    assert!(matches!(program.instructions[0], Instruction::PushConst(_)));
    assert!(matches!(program.instructions[1], Instruction::Neg));
}

#[test]
fn test_compile_system_vars() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();

    // Temperature
    let temp = codegen.compile_expr(&IrExpr::Temperature, &ir).unwrap();
    assert!(matches!(temp.instructions[0], Instruction::PushTemperature));

    // Vt
    let vt = codegen.compile_expr(&IrExpr::Vt, &ir).unwrap();
    assert!(matches!(vt.instructions[0], Instruction::PushVt));

    // Time
    let time = codegen.compile_expr(&IrExpr::Time, &ir).unwrap();
    assert!(matches!(time.instructions[0], Instruction::PushTime));
}

#[test]
fn test_compile_complex_expr() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);

    let ir = create_test_ir();
    // is * (exp(V(0,1) / vt) - 1)
    // Simplified: exp(V) - 1
    let expr = IrExpr::Binary(
        BinaryOp::Sub,
        Box::new(IrExpr::Call(
            crate::ir::IrFunction::Exp,
            vec![IrExpr::Voltage(0, 1)],
        )),
        Box::new(IrExpr::Const(1.0)),
    );
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    // PushVoltage, Exp, PushConst(1), Sub
    assert_eq!(program.instructions.len(), 4);
}

#[test]
fn test_compile_table_lookup_registers_table_and_instruction() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);
    let ir = create_test_ir();

    let expr = IrExpr::TableLookup {
        input: Box::new(IrExpr::Voltage(0, 1)),
        x_data: vec![0.0, 1.0, 2.0],
        y_data: vec![0.0, 1.0, 4.0],
    };
    let program = codegen.compile_expr(&expr, &ir).unwrap();

    assert_eq!(program.instructions.len(), 2);
    assert!(matches!(
        program.instructions[0],
        Instruction::PushVoltage(0, 1)
    ));
    assert!(matches!(
        program.instructions[1],
        Instruction::TableLookup(0)
    ));
    let tables = codegen.lookup_tables.borrow();
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].x_data, vec![0.0, 1.0, 2.0]);
    assert_eq!(tables[0].y_data, vec![0.0, 1.0, 4.0]);
}

#[test]
fn test_compile_table_lookup_reuses_identical_table() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);
    let ir = create_test_ir();

    let expr_a = IrExpr::TableLookup {
        input: Box::new(IrExpr::Const(0.25)),
        x_data: vec![0.0, 1.0],
        y_data: vec![0.0, 2.0],
    };
    let expr_b = IrExpr::TableLookup {
        input: Box::new(IrExpr::Const(0.75)),
        x_data: vec![0.0, 1.0],
        y_data: vec![0.0, 2.0],
    };
    let prog_a = codegen.compile_expr(&expr_a, &ir).unwrap();
    let prog_b = codegen.compile_expr(&expr_b, &ir).unwrap();

    assert!(matches!(
        prog_a.instructions[1],
        Instruction::TableLookup(0)
    ));
    assert!(matches!(
        prog_b.instructions[1],
        Instruction::TableLookup(0)
    ));
    assert_eq!(codegen.lookup_tables.borrow().len(), 1);
}

#[test]
fn test_compile_table_lookup_assigns_distinct_ids_for_distinct_tables() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);
    let ir = create_test_ir();

    let expr_a = IrExpr::TableLookup {
        input: Box::new(IrExpr::Const(0.25)),
        x_data: vec![0.0, 1.0],
        y_data: vec![0.0, 1.0],
    };
    let expr_b = IrExpr::TableLookup {
        input: Box::new(IrExpr::Const(0.25)),
        x_data: vec![0.0, 2.0],
        y_data: vec![0.0, 1.0],
    };
    let prog_a = codegen.compile_expr(&expr_a, &ir).unwrap();
    let prog_b = codegen.compile_expr(&expr_b, &ir).unwrap();

    assert!(matches!(
        prog_a.instructions[1],
        Instruction::TableLookup(0)
    ));
    assert!(matches!(
        prog_b.instructions[1],
        Instruction::TableLookup(1)
    ));
    assert_eq!(codegen.lookup_tables.borrow().len(), 2);
}

#[test]
fn test_compile_limit_allocates_distinct_state_ids() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);
    let ir = create_test_ir();

    let expr_a = IrExpr::Limit(
        Box::new(IrExpr::Const(1.0)),
        Some(Box::new(IrExpr::Const(0.1))),
    );
    let expr_b = IrExpr::Limit(
        Box::new(IrExpr::Const(2.0)),
        Some(Box::new(IrExpr::Const(0.2))),
    );

    let prog_a = codegen.compile_expr(&expr_a, &ir).unwrap();
    let prog_b = codegen.compile_expr(&expr_b, &ir).unwrap();

    assert!(matches!(
        prog_a.instructions.last(),
        Some(Instruction::LimitState(0))
    ));
    assert!(matches!(
        prog_b.instructions.last(),
        Some(Instruction::LimitState(1))
    ));
}

#[test]
fn test_compile_absdelay_allocates_distinct_buffer_ids() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);
    let ir = create_test_ir();

    let expr_a = IrExpr::AbsDelay {
        expr: Box::new(IrExpr::Const(1.0)),
        delay_time: Box::new(IrExpr::Const(1e-9)),
    };
    let expr_b = IrExpr::AbsDelay {
        expr: Box::new(IrExpr::Const(2.0)),
        delay_time: Box::new(IrExpr::Const(2e-9)),
    };

    let prog_a = codegen.compile_expr(&expr_a, &ir).unwrap();
    let prog_b = codegen.compile_expr(&expr_b, &ir).unwrap();

    assert!(matches!(
        prog_a.instructions.last(),
        Some(Instruction::AbsDelayState(0))
    ));
    assert!(matches!(
        prog_b.instructions.last(),
        Some(Instruction::AbsDelayState(1))
    ));
}

#[test]
fn test_compile_transition_allocates_distinct_filter_ids() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);
    let ir = create_test_ir();

    let expr_a = IrExpr::Transition {
        expr: Box::new(IrExpr::Const(1.0)),
        delay: Some(Box::new(IrExpr::Const(0.0))),
        rise_time: Some(Box::new(IrExpr::Const(1e-9))),
        fall_time: Some(Box::new(IrExpr::Const(2e-9))),
    };
    let expr_b = IrExpr::Transition {
        expr: Box::new(IrExpr::Const(2.0)),
        delay: Some(Box::new(IrExpr::Const(0.0))),
        rise_time: Some(Box::new(IrExpr::Const(1e-9))),
        fall_time: Some(Box::new(IrExpr::Const(2e-9))),
    };

    let prog_a = codegen.compile_expr(&expr_a, &ir).unwrap();
    let prog_b = codegen.compile_expr(&expr_b, &ir).unwrap();

    assert!(matches!(
        prog_a.instructions.last(),
        Some(Instruction::TransitionState(0))
    ));
    assert!(matches!(
        prog_b.instructions.last(),
        Some(Instruction::TransitionState(1))
    ));
}

#[test]
fn test_compile_slew_allocates_distinct_filter_ids() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);
    let ir = create_test_ir();

    let expr_a = IrExpr::Slew {
        expr: Box::new(IrExpr::Const(1.0)),
        max_pos_slew: Some(Box::new(IrExpr::Const(10.0))),
        max_neg_slew: Some(Box::new(IrExpr::Const(20.0))),
    };
    let expr_b = IrExpr::Slew {
        expr: Box::new(IrExpr::Const(2.0)),
        max_pos_slew: Some(Box::new(IrExpr::Const(30.0))),
        max_neg_slew: Some(Box::new(IrExpr::Const(40.0))),
    };

    let prog_a = codegen.compile_expr(&expr_a, &ir).unwrap();
    let prog_b = codegen.compile_expr(&expr_b, &ir).unwrap();

    assert!(matches!(
        prog_a.instructions.last(),
        Some(Instruction::SlewState(0))
    ));
    assert!(matches!(
        prog_b.instructions.last(),
        Some(Instruction::SlewState(1))
    ));
}

#[test]
fn test_compile_cross_allocates_distinct_detector_ids() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);
    let ir = create_test_ir();

    let expr_a = IrExpr::Cross {
        expr: Box::new(IrExpr::Const(-1.0)),
        direction: Some(1),
        time_tol: None,
    };
    let expr_b = IrExpr::Cross {
        expr: Box::new(IrExpr::Const(1.0)),
        direction: Some(-1),
        time_tol: None,
    };

    let prog_a = codegen.compile_expr(&expr_a, &ir).unwrap();
    let prog_b = codegen.compile_expr(&expr_b, &ir).unwrap();

    assert!(matches!(
        prog_a.instructions.last(),
        Some(Instruction::CrossState(0))
    ));
    assert!(matches!(
        prog_b.instructions.last(),
        Some(Instruction::CrossState(1))
    ));
}

#[test]
fn test_compile_above_and_timer_allocate_distinct_ids() {
    let options = CompilerOptions::default();
    let codegen = CodeGenerator::new(&options);
    let ir = create_test_ir();

    let above_a = IrExpr::Above {
        expr: Box::new(IrExpr::Const(1.0)),
        threshold: Box::new(IrExpr::Const(0.0)),
        time_tol: None,
    };
    let above_b = IrExpr::Above {
        expr: Box::new(IrExpr::Const(2.0)),
        threshold: Box::new(IrExpr::Const(0.0)),
        time_tol: None,
    };

    let timer_a = IrExpr::Timer {
        start_time: Box::new(IrExpr::Const(0.0)),
        period: Some(Box::new(IrExpr::Const(1e-9))),
    };
    let timer_b = IrExpr::Timer {
        start_time: Box::new(IrExpr::Const(0.0)),
        period: Some(Box::new(IrExpr::Const(2e-9))),
    };

    let above_prog_a = codegen.compile_expr(&above_a, &ir).unwrap();
    let above_prog_b = codegen.compile_expr(&above_b, &ir).unwrap();
    let timer_prog_a = codegen.compile_expr(&timer_a, &ir).unwrap();
    let timer_prog_b = codegen.compile_expr(&timer_b, &ir).unwrap();

    assert!(matches!(
        above_prog_a.instructions.last(),
        Some(Instruction::AboveState(0))
    ));
    assert!(matches!(
        above_prog_b.instructions.last(),
        Some(Instruction::AboveState(1))
    ));
    assert!(matches!(
        timer_prog_a.instructions.last(),
        Some(Instruction::TimerState(0))
    ));
    assert!(matches!(
        timer_prog_b.instructions.last(),
        Some(Instruction::TimerState(1))
    ));
}

#[test]
fn test_stamp_location() {
    let loc = StampLocation {
        row: StampIndex::Terminal(0),
        col: StampIndex::Terminal(1),
        sign: 1.0,
    };
    assert!(matches!(loc.row, StampIndex::Terminal(0)));
    assert!(matches!(loc.col, StampIndex::Terminal(1)));
    assert!((loc.sign - 1.0).abs() < 1e-10);
}

#[test]
fn test_stamp_index_variants() {
    let term = StampIndex::Terminal(0);
    let internal = StampIndex::Internal(1);
    let ground = StampIndex::Ground;

    assert!(matches!(term, StampIndex::Terminal(0)));
    assert!(matches!(internal, StampIndex::Internal(1)));
    assert!(matches!(ground, StampIndex::Ground));
}

// ========================================================================
// Helper Functions
// ========================================================================

fn create_test_ir() -> crate::ir::DeviceIR {
    crate::ir::DeviceIR {
        name: "test".into(),
        terminals: vec![
            crate::ir::Terminal {
                name: "p".into(),
                index: 0,
            },
            crate::ir::Terminal {
                name: "n".into(),
                index: 1,
            },
        ],
        internal_nodes: vec![],
        parameters: vec![crate::ir::ParamDef {
            name: "g".into(),
            default: 0.001,
            min: Some(0.0),
            max: None,
        }],
        variables: vec![],
        assignments: vec![],
        equations: vec![],
        noise_sources: vec![],
    }
}

// ========================================================================
// LookupTable Tests - Comprehensive Commercial-Grade Coverage
// ========================================================================

#[test]
fn test_lookup_table_empty() {
    let table = LookupTable::new();
    assert!(table.is_empty());
    assert_eq!(table.len(), 0);
    // Empty table returns 0 for any input
    assert_eq!(table.interpolate(0.0), 0.0);
    assert_eq!(table.interpolate(1.0), 0.0);
    assert_eq!(table.interpolate(-100.0), 0.0);
    assert_eq!(table.derivative(0.0), 0.0);
}

#[test]
fn test_lookup_table_single_point() {
    let table = LookupTable::from_data(vec![1.0], vec![5.0]);
    assert!(!table.is_empty());
    assert_eq!(table.len(), 1);
    // Single point table returns constant for any input
    assert_eq!(table.interpolate(0.0), 5.0);
    assert_eq!(table.interpolate(1.0), 5.0);
    assert_eq!(table.interpolate(100.0), 5.0);
    assert_eq!(table.derivative(0.0), 0.0);
}

#[test]
fn test_lookup_table_two_points() {
    // Simple line from (0, 0) to (1, 1)
    let table = LookupTable::from_data(vec![0.0, 1.0], vec![0.0, 1.0]);

    // Exact points
    assert!((table.interpolate(0.0) - 0.0).abs() < 1e-12);
    assert!((table.interpolate(1.0) - 1.0).abs() < 1e-12);

    // Midpoint
    assert!((table.interpolate(0.5) - 0.5).abs() < 1e-12);

    // Other interpolation
    assert!((table.interpolate(0.25) - 0.25).abs() < 1e-12);
    assert!((table.interpolate(0.75) - 0.75).abs() < 1e-12);

    // Derivative is constant 1.0
    assert!((table.derivative(0.5) - 1.0).abs() < 1e-12);
}

#[test]
fn test_lookup_table_linear_extrapolation_below() {
    // Line from (1, 10) to (2, 20)
    let table = LookupTable::from_data(vec![1.0, 2.0], vec![10.0, 20.0]);

    // Below range - linear extrapolation with slope 10
    assert!((table.interpolate(0.0) - 0.0).abs() < 1e-12); // 10 - 10*1 = 0
    assert!((table.interpolate(-1.0) - (-10.0)).abs() < 1e-12); // 10 - 10*2 = -10
}

#[test]
fn test_lookup_table_linear_extrapolation_above() {
    // Line from (1, 10) to (2, 20)
    let table = LookupTable::from_data(vec![1.0, 2.0], vec![10.0, 20.0]);

    // Above range - linear extrapolation with slope 10
    assert!((table.interpolate(3.0) - 30.0).abs() < 1e-12); // 20 + 10*1 = 30
    assert!((table.interpolate(4.0) - 40.0).abs() < 1e-12); // 20 + 10*2 = 40
}

#[test]
fn test_lookup_table_multiple_segments() {
    // Piecewise linear: (0,0), (1,2), (2,1), (3,3)
    let table = LookupTable::from_data(vec![0.0, 1.0, 2.0, 3.0], vec![0.0, 2.0, 1.0, 3.0]);

    // Exact points
    assert!((table.interpolate(0.0) - 0.0).abs() < 1e-12);
    assert!((table.interpolate(1.0) - 2.0).abs() < 1e-12);
    assert!((table.interpolate(2.0) - 1.0).abs() < 1e-12);
    assert!((table.interpolate(3.0) - 3.0).abs() < 1e-12);

    // Interpolation in first segment (slope = 2)
    assert!((table.interpolate(0.5) - 1.0).abs() < 1e-12);

    // Interpolation in second segment (slope = -1)
    assert!((table.interpolate(1.5) - 1.5).abs() < 1e-12);

    // Interpolation in third segment (slope = 2)
    assert!((table.interpolate(2.5) - 2.0).abs() < 1e-12);
}

#[test]
fn test_lookup_table_derivative() {
    // Piecewise linear with different slopes
    let table = LookupTable::from_data(vec![0.0, 1.0, 3.0], vec![0.0, 2.0, 6.0]);

    // First segment: slope = 2
    assert!((table.derivative(0.5) - 2.0).abs() < 1e-12);

    // Second segment: slope = (6-2)/(3-1) = 2
    assert!((table.derivative(2.0) - 2.0).abs() < 1e-12);

    // Extrapolation uses endpoint slopes
    assert!((table.derivative(-1.0) - 2.0).abs() < 1e-12);
    assert!((table.derivative(5.0) - 2.0).abs() < 1e-12);
}

#[test]
fn test_lookup_table_negative_values() {
    // Table with negative x and y
    let table = LookupTable::from_data(vec![-2.0, -1.0, 0.0, 1.0], vec![-4.0, -1.0, 0.0, 1.0]);

    assert!((table.interpolate(-1.5) - (-2.5)).abs() < 1e-12);
    assert!((table.interpolate(-0.5) - (-0.5)).abs() < 1e-12);
    assert!((table.interpolate(0.5) - 0.5).abs() < 1e-12);
}

#[test]
fn test_lookup_table_validate_success() {
    let table = LookupTable::from_data(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0]);
    assert!(table.validate().is_ok());
}

#[test]
fn test_lookup_table_validate_unsorted() {
    let mut table = LookupTable::new();
    table.x_data = vec![0.0, 2.0, 1.0]; // Not sorted!
    table.y_data = vec![0.0, 1.0, 2.0];
    assert!(table.validate().is_err());
}

#[test]
fn test_lookup_table_validate_nan() {
    let mut table = LookupTable::new();
    table.x_data = vec![0.0, f64::NAN, 2.0];
    table.y_data = vec![0.0, 1.0, 2.0];
    assert!(table.validate().is_err());
}

#[test]
fn test_lookup_table_validate_inf() {
    let mut table = LookupTable::new();
    table.x_data = vec![0.0, 1.0, 2.0];
    table.y_data = vec![0.0, f64::INFINITY, 2.0];
    assert!(table.validate().is_err());
}

#[test]
fn test_lookup_table_very_close_x_values() {
    // Test numerical stability with very close x values
    let table = LookupTable::from_data(vec![1.0, 1.0 + 1e-15, 2.0], vec![10.0, 10.0, 20.0]);
    // Should not crash, may return first value for near-duplicate x
    let result = table.interpolate(1.0 + 1e-16);
    assert!(result.is_finite());
}

#[test]
fn test_lookup_table_large_scale() {
    // Large table for binary search performance
    let n = 1000;
    let x_data: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let y_data: Vec<f64> = (0..n).map(|i| (i as f64).powi(2)).collect();
    let table = LookupTable::from_data(x_data, y_data);

    // Test interpolation at various points
    assert!((table.interpolate(500.0) - 250000.0).abs() < 1e-12);
    // Linear interp between 500²=250000 and 501²=251001 at 500.5
    // = 250000 + (251001-250000)*0.5 = 250500.5
    assert!((table.interpolate(500.5) - 250500.5).abs() < 1e-9);

    // Verify it performs well (no timeout issues with binary search)
    for i in 0..100 {
        let x = i as f64 * 10.0;
        let _ = table.interpolate(x);
    }
}

#[test]
fn test_lookup_table_named() {
    let table = LookupTable::from_data_named(vec![0.0, 1.0], vec![0.0, 1.0], "diode_iv");
    assert!(table.name.is_some());
    assert_eq!(table.name.as_ref().unwrap().as_str(), "diode_iv");
}

#[test]
fn test_lookup_table_realistic_diode() {
    // Realistic diode I-V characteristic approximation
    // V:   -1.0, -0.5,  0.0,  0.2,  0.4,  0.5,  0.6,  0.7
    // I:    0.0,  0.0,  0.0, 1e-8, 1e-5, 1e-3, 0.01, 0.1
    let table = LookupTable::from_data(
        vec![-1.0, -0.5, 0.0, 0.2, 0.4, 0.5, 0.6, 0.7],
        vec![0.0, 0.0, 0.0, 1e-8, 1e-5, 1e-3, 0.01, 0.1],
    );

    // Reverse bias - zero current
    assert!(table.interpolate(-0.7).abs() < 1e-12);

    // Forward bias - increasing current
    assert!(table.interpolate(0.65) > 0.001);
    assert!(table.interpolate(0.7) > 0.01);
}
