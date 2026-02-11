use super::*;

// $table_model / TableLookup Tests - VM Execution
// ========================================================================

#[test]
fn test_table_lookup_basic() {
    // Simple linear table: y = x
    let mut ctx = VmContext::new(2);
    ctx.lookup_tables.push(LookupTable::from_data(
        vec![0.0, 1.0, 2.0],
        vec![0.0, 1.0, 2.0],
    ));

    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::TableLookup(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 1.0).abs() < 1e-12,
        "Table lookup at 1.0 should be 1.0, got {result}"
    );

    // Test at interpolation point
    let program = make_program(vec![
        Instruction::PushConst(0.5),
        Instruction::TableLookup(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 0.5).abs() < 1e-12,
        "Table lookup at 0.5 should be 0.5, got {result}"
    );
}

#[test]
fn test_table_lookup_extrapolation() {
    let mut ctx = VmContext::new(2);
    // Slope = 10
    ctx.lookup_tables
        .push(LookupTable::from_data(vec![1.0, 2.0], vec![10.0, 20.0]));

    // Below range
    let program = make_program(vec![
        Instruction::PushConst(0.0),
        Instruction::TableLookup(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 0.0).abs() < 1e-12,
        "Extrapolation below: expected 0, got {result}"
    );

    // Above range
    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::TableLookup(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 30.0).abs() < 1e-12,
        "Extrapolation above: expected 30, got {result}"
    );
}

#[test]
fn test_table_lookup_empty_table() {
    let mut ctx = VmContext::new(2);
    ctx.lookup_tables.push(LookupTable::new()); // Empty table

    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::TableLookup(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "Empty table should return 0");
}

#[test]
fn test_table_lookup_missing_table() {
    let mut ctx = VmContext::new(2); // No tables

    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::TableLookup(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "Missing table should return 0");
}

#[test]
fn test_table_lookup_multiple_tables() {
    let mut ctx = VmContext::new(2);
    // Table 0: y = x
    ctx.lookup_tables
        .push(LookupTable::from_data(vec![0.0, 1.0], vec![0.0, 1.0]));
    // Table 1: y = 2*x
    ctx.lookup_tables
        .push(LookupTable::from_data(vec![0.0, 1.0], vec![0.0, 2.0]));

    // Use table 0
    let program = make_program(vec![
        Instruction::PushConst(0.5),
        Instruction::TableLookup(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!((result - 0.5).abs() < 1e-12, "Table 0 at 0.5 should be 0.5");

    // Use table 1
    let program = make_program(vec![
        Instruction::PushConst(0.5),
        Instruction::TableLookup(1),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!((result - 1.0).abs() < 1e-12, "Table 1 at 0.5 should be 1.0");
}

#[test]
fn test_table_lookup_nonlinear() {
    let mut ctx = VmContext::new(2);
    // Nonlinear (quadratic-ish): (0,0), (1,1), (2,4)
    ctx.lookup_tables.push(LookupTable::from_data(
        vec![0.0, 1.0, 2.0],
        vec![0.0, 1.0, 4.0],
    ));

    // Between 1 and 2: linear interp gives (4-1)/(2-1) * (1.5-1) + 1 = 2.5
    let program = make_program(vec![
        Instruction::PushConst(1.5),
        Instruction::TableLookup(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 2.5).abs() < 1e-12,
        "Nonlinear interpolation at 1.5 should be 2.5"
    );
}

#[test]
fn test_table_lookup_in_expression() {
    let mut ctx = VmContext::new(2);
    ctx.lookup_tables
        .push(LookupTable::from_data(vec![0.0, 1.0], vec![0.0, 10.0]));

    // expr = 2 * table(0.5) = 2 * 5 = 10
    let program = make_program(vec![
        Instruction::PushConst(2.0),
        Instruction::PushConst(0.5),
        Instruction::TableLookup(0),
        Instruction::Mul,
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!((result - 10.0).abs() < 1e-12, "2 * table(0.5) should be 10");
}

// ========================================================================
// DelayBuffer Tests - Comprehensive Commercial-Grade Coverage
// ========================================================================

#[test]
fn test_delay_buffer_new() {
    let buffer = DelayBuffer::new(10);
    assert_eq!(buffer.capacity, 10);
    assert_eq!(buffer.count, 0);
}

#[test]
fn test_delay_buffer_record_and_get() {
    let mut buffer = DelayBuffer::new(10);

    // Record some samples
    buffer.record(0.0, 0.0);
    buffer.record(1.0, 10.0);
    buffer.record(2.0, 20.0);

    assert_eq!(buffer.count, 3);

    // Get value at t=1.0 with delay=0
    let result = buffer.get_delayed(2.0, 1.0);
    assert!((result - 10.0).abs() < 1e-12, "At t=1, value should be 10");
}

#[test]
fn test_delay_buffer_interpolation() {
    let mut buffer = DelayBuffer::new(10);

    buffer.record(0.0, 0.0);
    buffer.record(2.0, 20.0);

    // Get interpolated value at t=1.0
    let result = buffer.get_delayed(2.0, 1.0);
    assert!(
        (result - 10.0).abs() < 1e-12,
        "Interpolated at t=1 should be 10"
    );
}

#[test]
fn test_delay_buffer_empty() {
    let buffer = DelayBuffer::new(10);
    // Empty buffer returns 0
    assert_eq!(buffer.get_delayed(1.0, 0.5), 0.0);
}

#[test]
fn test_delay_buffer_circular_wrap() {
    let mut buffer = DelayBuffer::new(4);

    // Fill buffer and wrap around
    buffer.record(0.0, 0.0);
    buffer.record(1.0, 10.0);
    buffer.record(2.0, 20.0);
    buffer.record(3.0, 30.0);
    buffer.record(4.0, 40.0); // Overwrites first entry
    buffer.record(5.0, 50.0); // Overwrites second entry

    assert_eq!(buffer.count, 4); // Capacity is 4

    // Old data (t=0, t=1) should be gone
    // Available: t=2,3,4,5
    let result = buffer.get_delayed(5.0, 1.0);
    assert!((result - 40.0).abs() < 1e-12, "At t=4, value should be 40");
}

#[test]
fn test_delay_buffer_clear() {
    let mut buffer = DelayBuffer::new(10);
    buffer.record(1.0, 10.0);
    buffer.record(2.0, 20.0);
    buffer.clear();
    assert_eq!(buffer.count, 0);
    assert_eq!(buffer.get_delayed(3.0, 1.0), 0.0);
}

#[test]
fn test_delay_buffer_single_sample() {
    let mut buffer = DelayBuffer::new(10);
    buffer.record(1.0, 100.0);

    // Single sample - returns that value for any delay
    assert!((buffer.get_delayed(2.0, 1.0) - 100.0).abs() < 1e-12);
    assert!((buffer.get_delayed(5.0, 4.0) - 100.0).abs() < 1e-12);
}

#[test]
fn test_delay_buffer_exact_match() {
    let mut buffer = DelayBuffer::new(10);
    buffer.record(0.0, 0.0);
    buffer.record(1.0, 10.0);
    buffer.record(2.0, 20.0);
    buffer.record(3.0, 30.0);

    // Exact match at t=2 (current_time=4, delay=2)
    let result = buffer.get_delayed(4.0, 2.0);
    assert!((result - 20.0).abs() < 1e-12, "Exact match at t=2");
}

#[test]
fn test_delay_buffer_minimum_capacity() {
    // Capacity should be at least 2
    let buffer = DelayBuffer::new(0);
    assert!(buffer.capacity >= 2);

    let buffer = DelayBuffer::new(1);
    assert!(buffer.capacity >= 2);
}

// ========================================================================
// AbsDelay VM Execution Tests
// ========================================================================

#[test]
fn test_absdelay_dc_returns_current() {
    // At time=0 (DC), absdelay should return current value
    let mut ctx = VmContext::new(2);

    let program = make_program(vec![
        Instruction::PushConst(42.0), // expr value
        Instruction::PushConst(1e-9), // delay time
        Instruction::AbsDelayState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 42.0).abs() < 1e-12,
        "DC absdelay returns current value"
    );
}

#[test]
fn test_absdelay_zero_delay() {
    // Zero delay returns current value
    let mut ctx = VmContext::new(2);
    ctx.time = 1.0;

    let program = make_program(vec![
        Instruction::PushConst(100.0), // expr value
        Instruction::PushConst(0.0),   // zero delay
        Instruction::AbsDelayState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 100.0).abs() < 1e-12,
        "Zero delay returns current value"
    );
}

#[test]
fn test_absdelay_with_buffer() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 2; // transient
    ctx.time = 2.0;

    // Pre-populate delay buffer
    let mut buffer = DelayBuffer::new(10);
    buffer.record(0.0, 0.0);
    buffer.record(1.0, 10.0);
    buffer.record(2.0, 20.0);
    ctx.delay_buffers.push(buffer);

    // Get value delayed by 1 second (should be 10.0)
    let program = make_program(vec![
        Instruction::PushConst(25.0), // current expr value
        Instruction::PushConst(1.0),  // delay time
        Instruction::AbsDelayState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 10.0).abs() < 1e-12,
        "Delayed by 1s should be 10.0"
    );
}

#[test]
fn test_absdelay_no_buffer_returns_current() {
    // No buffer exists - return current value
    let mut ctx = VmContext::new(2);
    ctx.time = 1.0;

    let program = make_program(vec![
        Instruction::PushConst(50.0),
        Instruction::PushConst(0.5),
        Instruction::AbsDelayState(0), // Buffer 0 doesn't exist
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!((result - 50.0).abs() < 1e-12, "No buffer returns current");
}

#[test]
fn test_absdelay_negative_delay() {
    // Negative delay returns current value
    let mut ctx = VmContext::new(2);
    ctx.time = 1.0;

    let program = make_program(vec![
        Instruction::PushConst(75.0),
        Instruction::PushConst(-0.5), // Negative delay
        Instruction::AbsDelayState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 75.0).abs() < 1e-12,
        "Negative delay returns current"
    );
}

#[test]
fn test_absdelay_interpolation() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 2; // transient
    ctx.time = 3.0;

    let mut buffer = DelayBuffer::new(10);
    buffer.record(0.0, 0.0);
    buffer.record(2.0, 100.0);
    ctx.delay_buffers.push(buffer);

    // Get value at t=1 (interpolated: 50.0)
    let program = make_program(vec![
        Instruction::PushConst(200.0), // current
        Instruction::PushConst(2.0),   // delay -> target t=1
        Instruction::AbsDelayState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 50.0).abs() < 1e-12,
        "Interpolated delay should be 50"
    );
}

// ========================================================================
