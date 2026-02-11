use super::*;
use crate::lexer::Lexer;
use crate::source::SourceId;

fn parse(source: &str) -> Result<SourceFile, ParseError> {
    let tokens = Lexer::new(source, SourceId::new(0))
        .collect_tokens()
        .unwrap();
    let source_map = SourceMap::new();
    Parser::new(&tokens, &source_map).parse()
}

#[test]
fn test_simple_module() {
    let source = r#"
            module resistor(p, n);
                inout p, n;
                electrical p, n;
                parameter real r = 1.0;
                analog I(p, n) <+ V(p, n) / r;
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
    let file = result.unwrap();
    assert_eq!(file.items.len(), 1);
}

#[test]
fn test_parse_number_scales() {
    assert_eq!(parse_number("1k"), 1000.0);
    assert_eq!(parse_number("1M"), 1e6);
    assert_eq!(parse_number("1u"), 1e-6);
    assert_eq!(parse_number("1n"), 1e-9);
    assert_eq!(parse_number("1p"), 1e-12);
}

#[test]
fn test_parse_all_scales() {
    assert_eq!(parse_number("1T"), 1e12);
    assert_eq!(parse_number("1G"), 1e9);
    assert_eq!(parse_number("1K"), 1e3);
    assert_eq!(parse_number("1f"), 1e-15);
    assert_eq!(parse_number("1a"), 1e-18);
    assert_eq!(parse_number("2.5meg"), 2.5e6);
}

#[test]
fn test_diode_model() {
    let source = r#"
            module diode(a, c);
                inout a, c;
                electrical a, c;
                parameter real Is = 1e-14 from (0:inf);
                parameter real N = 1.0 from (0:inf);
                real Vd, Id, Gd;
                analog begin
                    Vd = V(a, c);
                    Id = Is * (limexp(Vd / ($vt * N)) - 1.0);
                    Gd = Is / ($vt * N) * limexp(Vd / ($vt * N));
                    I(a, c) <+ Id;
                end
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
    let file = result.unwrap();
    assert_eq!(file.items.len(), 1);
    if let Item::Module(m) = &file.items[0] {
        assert_eq!(m.name.as_str(), "diode");
        assert_eq!(m.ports.len(), 2);
        assert!(m.parameters.len() >= 2);
    }
}

#[test]
fn test_module_with_if_else() {
    let source = r#"
            module conditional(p, n);
                inout p, n;
                electrical p, n;
                parameter real threshold = 0.5;
                real x;
                analog begin
                    if (V(p, n) > threshold) begin
                        x = 1.0;
                    end else begin
                        x = 0.0;
                    end
                    I(p, n) <+ x;
                end
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_module_with_for_loop() {
    let source = r#"
            module forloop(p, n);
                inout p, n;
                electrical p, n;
                integer i;
                real sum;
                analog begin
                    sum = 0.0;
                    for (i = 0; i < 10; i = i + 1) begin
                        sum = sum + 1.0;
                    end
                    I(p, n) <+ sum;
                end
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_module_with_while_loop() {
    let source = r#"
            module whileloop(p, n);
                inout p, n;
                electrical p, n;
                integer i;
                real sum;
                analog begin
                    sum = 0.0;
                    i = 0;
                    while (i < 5) begin
                        sum = sum + 1.0;
                        i = i + 1;
                    end
                    I(p, n) <+ sum;
                end
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_module_with_case() {
    let source = r#"
            module casetest(p, n);
                inout p, n;
                electrical p, n;
                integer mode;
                real val;
                analog begin
                    case (mode)
                        0: val = 1.0;
                        1: val = 2.0;
                        default: val = 0.0;
                    endcase
                    I(p, n) <+ val;
                end
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_complex_expression() {
    let source = r#"
            module expr(p, n);
                inout p, n;
                electrical p, n;
                parameter real a = 1.0;
                parameter real b = 2.0;
                analog I(p, n) <+ (a + b) * exp(-V(p, n)) / (1.0 + V(p, n) ** 2);
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_ternary_expression() {
    let source = r#"
            module ternary(p, n);
                inout p, n;
                electrical p, n;
                analog I(p, n) <+ V(p, n) > 0 ? 1.0 : -1.0;
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_system_functions() {
    let source = r#"
            module sysfunc(p, n);
                inout p, n;
                electrical p, n;
                analog I(p, n) <+ V(p, n) / $vt * $temperature;
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_function_calls() {
    let source = r#"
            module funcs(p, n);
                inout p, n;
                electrical p, n;
                analog I(p, n) <+ sin(V(p, n)) + cos(V(p, n)) + exp(V(p, n)) + log(abs(V(p, n) + 1));
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_multiple_contributions() {
    let source = r#"
            module multi(p, n);
                inout p, n;
                electrical p, n;
                analog begin
                    I(p, n) <+ V(p, n) / 1k;
                    I(p, n) <+ 0.5 * V(p, n);
                end
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_discipline_definition() {
    let source = r#"
            discipline electrical;
                potential Voltage;
                flow Current;
            enddiscipline
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
    let file = result.unwrap();
    assert_eq!(file.items.len(), 1);
    assert!(matches!(file.items[0], Item::Discipline(_)));
}

#[test]
fn test_nature_definition() {
    let source = r#"
            nature Voltage;
                access = V;
                units = "V";
                abstol = 1e-6;
            endnature
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
    let file = result.unwrap();
    assert_eq!(file.items.len(), 1);
    assert!(matches!(file.items[0], Item::Nature(_)));
}

#[test]
fn test_multiple_parameters() {
    let source = r#"
            module multi_param(p, n);
                inout p, n;
                electrical p, n;
                parameter real r1 = 1k;
                parameter real r2 = 2k;
                parameter integer count = 5;
                analog I(p, n) <+ V(p, n) / (r1 + r2);
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
    let file = result.unwrap();
    if let Item::Module(m) = &file.items[0] {
        assert_eq!(m.parameters.len(), 3);
    }
}

#[test]
fn test_real_variables() {
    let source = r#"
            module realvars(p, n);
                inout p, n;
                electrical p, n;
                real x, y, z;
                analog begin
                    x = V(p, n);
                    y = x * 2.0;
                    z = y + 1.0;
                    I(p, n) <+ z;
                end
            endmodule
        "#;
    let result = parse(source);
    if let Err(ref e) = result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
    let file = result.unwrap();
    if let Item::Module(m) = &file.items[0] {
        assert!(!m.variables.is_empty());
    }
}
