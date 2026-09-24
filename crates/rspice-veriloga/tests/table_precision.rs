use rspice_veriloga::codegen::LookupTable;

#[test]
fn table_retains_distinct_subnormal_abscissae() {
    let width = f64::from_bits(8);
    let table = LookupTable::from_data(vec![0.0, width], vec![0.0, width]);
    assert_eq!(table.interpolate(width / 2.0), width / 2.0);
    assert_eq!(table.interpolate(-width), -width);
    assert_eq!(table.interpolate(2.0 * width), 2.0 * width);
    assert_eq!(table.derivative(width / 2.0), 1.0);
}

#[test]
fn table_value_and_slope_survive_overflowing_endpoint_differences() {
    let large = 2.0_f64.powi(1023);
    for table in [
        LookupTable::from_data(vec![0.0, 2.0], vec![-large, large]),
        LookupTable::from_data(vec![-large, large], vec![-large, large]),
    ] {
        let x = if table.x_data[0] == 0.0 {
            1.5
        } else {
            large / 2.0
        };
        assert_eq!(table.interpolate(x), large / 2.0);
        let expected = if table.x_data[0] == 0.0 { large } else { 1.0 };
        assert_eq!(table.derivative(x), expected);
    }
}

#[test]
fn table_weight_does_not_underflow_before_multiplying_the_signal() {
    let large = 2.0_f64.powi(1000);
    let small = 2.0_f64.powi(-1000);
    let table = LookupTable::from_data(vec![0.0, large], vec![0.0, large]);
    assert_eq!(table.interpolate(small), small);
}

#[test]
fn table_cancellation_retains_the_small_interpolated_value() {
    let large = 2.0_f64.powi(500);
    let table = LookupTable::from_data(vec![0.0, 3.0], vec![-large, large]);
    let input = 1.5 + 2.0_f64.powi(-51);
    let expected = 2.0_f64.powi(450) / 3.0;
    assert_eq!(table.interpolate(input), expected);
}
