//! ngspice's whole-vector functions (maths/cmaths/cmath1.c, cmath2.c and
//! cmath4.c). Each takes one vector and returns another; the angles they
//! return are in radians, ngspice's default `units`.

use super::*;
use crate::engine::control::shell::{ShellValue, number_prefix};

/// Whether `name` is a function this module evaluates on a whole vector.
pub(super) fn is_vector_function(name: &str) -> bool {
    matches!(
        name,
        "VECTOR"
            | "CVECTOR"
            | "UNITVEC"
            | "LENGTH"
            | "MEAN"
            | "VECMAX"
            | "MAXIMUM"
            | "VECMIN"
            | "MINIMUM"
            | "DB"
            | "PH"
            | "PHASE"
            | "CPH"
            | "CPHASE"
            | "J"
            | "CONJ"
            | "LOG"
            | "LN"
            | "LOG10"
            | "EXP"
            | "SQRT"
            | "SIN"
            | "COS"
            | "TAN"
            | "SINH"
            | "COSH"
            | "TANH"
            | "ATAN"
            | "ATANH"
            | "FLOOR"
            | "CEIL"
            | "NINT"
            | "POS"
            | "DERIV"
            | "INTEG"
            | "MTIMEAVG"
    )
}

/// Apply the function `name` (as `is_vector_function` accepts it).
pub(super) fn apply(
    circuit: &ControlCircuit,
    name: &str,
    argument: VectorValue,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<VectorValue, ControlError> {
    let VectorValue {
        samples,
        complex,
        scale,
        unit,
    } = argument;
    let length = samples.len();
    // ngspice reads the magnitude of a complex count (cmath2.c:541).
    let count = || {
        let first = samples[0];
        let count = if complex {
            first.norm()
        } else {
            first.re.abs()
        };
        (count as usize).max(1)
    };
    let one = |value: ComplexValue, complex: bool| VectorValue {
        samples: vec![value],
        complex,
        scale: None,
        unit: unit.clone(),
    };
    let real = |samples: Vec<ComplexValue>, unit: SignalUnit| VectorValue {
        samples,
        complex: false,
        scale: scale.clone(),
        unit,
    };
    let map = |f: &dyn Fn(ComplexValue) -> ComplexValue| -> Vec<ComplexValue> {
        samples.iter().map(|value| f(*value)).collect()
    };
    let checked = |name: &str, ok: bool| {
        if ok {
            Ok(())
        } else {
            Err(command_error(
                line,
                format!("argument out of range for {}", name.to_ascii_lowercase()),
            ))
        }
    };
    let value = match name {
        "VECTOR" | "CVECTOR" | "UNITVEC" => {
            let length = count();
            if length > circuit.value_limit / 2 {
                return Err(ControlError::new(
                    line,
                    ControlErrorKind::ResourceLimit,
                    format!(
                        "{} of {length} elements exceeds the result values limit",
                        name.to_ascii_lowercase()
                    ),
                ));
            }
            let samples = (0..length)
                .map(|index| {
                    ComplexValue::from(if name == "UNITVEC" {
                        1.0
                    } else {
                        index as Value
                    })
                })
                .collect();
            VectorValue {
                samples,
                complex: name == "CVECTOR",
                scale: None,
                unit: SignalUnit::Dimensionless,
            }
        }
        "LENGTH" => one((length as Value).into(), false),
        "MEAN" => {
            let sum = samples
                .iter()
                .fold(ComplexValue::ZERO, |sum, value| sum + value);
            one(sum / length as Value, complex)
        }
        // cmath2.c:822 takes the real and imaginary extremes apart.
        "VECMAX" | "MAXIMUM" | "VECMIN" | "MINIMUM" => {
            let max = matches!(name, "VECMAX" | "MAXIMUM");
            let pick = |a: Value, b: Value| if (b > a) == max && b != a { b } else { a };
            let first = samples[0];
            let extreme = samples[1..].iter().fold(first, |extreme, value| {
                ComplexValue::new(pick(extreme.re, value.re), pick(extreme.im, value.im))
            });
            one(extreme, complex)
        }
        "DB" => {
            let mut values = Vec::with_capacity(length);
            for value in &samples {
                let magnitude = if complex { value.norm() } else { value.re };
                checked(name, magnitude > 0.0)?;
                values.push((20.0 * magnitude.log10()).into());
            }
            real(values, SignalUnit::Dimensionless)
        }
        "PH" | "PHASE" => real(
            map(&|value| {
                if complex {
                    value.im.atan2(value.re)
                } else {
                    0.0
                }
                .into()
            }),
            SignalUnit::Dimensionless,
        ),
        "CPH" | "CPHASE" => {
            let mut values = Vec::with_capacity(length);
            if complex {
                let mut last = samples[0].im.atan2(samples[0].re);
                values.push(last.into());
                for value in &samples[1..] {
                    let phase = value.im.atan2(value.re);
                    last = phase
                        - 2.0
                            * std::f64::consts::PI
                            * ((phase - last) / (2.0 * std::f64::consts::PI) + 0.5).floor();
                    values.push(last.into());
                }
            } else {
                values.resize(length, ComplexValue::ZERO);
            }
            real(values, SignalUnit::Dimensionless)
        }
        "J" => VectorValue {
            samples: map(&|value| ComplexValue::new(-value.im, value.re)),
            complex: true,
            scale,
            unit,
        },
        "CONJ" => VectorValue {
            samples: map(&|value| value.conj()),
            complex,
            scale,
            unit,
        },
        // cmath1.c:388 and :341: a zero magnitude gives -log(HUGE), which
        // is no finite value, and a complex logarithm keeps atan2 as its
        // imaginary part in both bases.
        "LOG" | "LN" | "LOG10" => {
            let ten = name == "LOG10";
            let mut values = Vec::with_capacity(length);
            for value in &samples {
                let magnitude = if complex { value.norm() } else { value.re };
                checked(name, magnitude > 0.0)?;
                let logarithm = if ten {
                    magnitude.log10()
                } else {
                    magnitude.ln()
                };
                values.push(ComplexValue::new(
                    logarithm,
                    if complex {
                        value.im.atan2(value.re)
                    } else {
                        0.0
                    },
                ));
            }
            VectorValue {
                samples: values,
                complex,
                scale,
                unit: SignalUnit::Dimensionless,
            }
        }
        "EXP" => VectorValue {
            samples: map(&|value| {
                if complex {
                    let magnitude = value.re.exp();
                    ComplexValue::new(magnitude * value.im.cos(), magnitude * value.im.sin())
                } else {
                    value.re.exp().into()
                }
            }),
            complex,
            scale,
            unit: SignalUnit::Dimensionless,
        },
        "SQRT" => {
            let negative = !complex && samples.iter().any(|value| value.re < 0.0);
            VectorValue {
                samples: map(&|value| {
                    if complex {
                        complex_sqrt(value)
                    } else if value.re < 0.0 {
                        ComplexValue::new(0.0, (-value.re).sqrt())
                    } else {
                        value.re.sqrt().into()
                    }
                }),
                complex: complex || negative,
                scale,
                unit,
            }
        }
        "SIN" | "COS" | "TAN" | "SINH" | "COSH" | "TANH" | "ATAN" | "ATANH" => {
            let f: fn(ComplexValue) -> ComplexValue = match name {
                "SIN" => |value| value.sin(),
                "COS" => |value| value.cos(),
                "TAN" => |value| value.tan(),
                "SINH" => |value| value.sinh(),
                "COSH" => |value| value.cosh(),
                "TANH" => |value| value.tanh(),
                "ATAN" => |value| value.atan(),
                _ => |value| value.atanh(),
            };
            VectorValue {
                samples: map(&|value| {
                    if complex {
                        f(value)
                    } else {
                        f(value.re.into()).re.into()
                    }
                }),
                complex,
                scale,
                unit: SignalUnit::Dimensionless,
            }
        }
        "FLOOR" | "CEIL" | "NINT" => {
            let f: fn(Value) -> Value = match name {
                "FLOOR" => Value::floor,
                "CEIL" => Value::ceil,
                _ => Value::round_ties_even,
            };
            VectorValue {
                samples: map(&|value| ComplexValue::new(f(value.re), f(value.im))),
                complex,
                scale,
                unit,
            }
        }
        "POS" => real(
            map(&|value| Value::from(u8::from(value.re > 0.0)).into()),
            SignalUnit::Dimensionless,
        ),
        "DERIV" => {
            let (scale_values, _) =
                circuit
                    .scale_values(scale.as_deref(), length)
                    .ok_or_else(|| {
                        command_error(line, "deriv requires a vector on its plot's scale")
                    })?;
            if abort.is_aborted() {
                return Err(ControlError::new(
                    line,
                    ControlErrorKind::Aborted,
                    "control execution aborted",
                ));
            }
            let re: Vec<_> = samples.iter().map(|value| value.re).collect();
            let mut values: Vec<ComplexValue> = derivative(&scale_values, &re)
                .into_iter()
                .map(ComplexValue::from)
                .collect();
            if complex {
                let im: Vec<_> = samples.iter().map(|value| value.im).collect();
                for (value, im) in values.iter_mut().zip(derivative(&scale_values, &im)) {
                    value.im = im;
                }
            }
            VectorValue {
                samples: values,
                complex,
                scale,
                unit: SignalUnit::Unspecified,
            }
        }
        // cmath4.c:439 cx_integ: the trapezoidal rule over the plot's
        // scale, from zero at its first point.
        "INTEG" => {
            if complex {
                return Err(command_error(
                    line,
                    "Function integ is not supported for complex data",
                ));
            }
            let (scale_values, _) =
                circuit
                    .scale_values(scale.as_deref(), length)
                    .ok_or_else(|| {
                        command_error(line, "integ requires a vector on its plot's scale")
                    })?;
            let mut total = 0.0;
            let mut values = Vec::with_capacity(length);
            for (index, sample) in samples.iter().enumerate() {
                if let Some(previous) = index.checked_sub(1) {
                    let step = scale_values[index] - scale_values[previous];
                    total += (sample.re + samples[previous].re) * step / 2.0;
                }
                values.push(ComplexValue::from(total));
            }
            VectorValue {
                samples: values,
                complex,
                scale,
                unit,
            }
        }
        // cmath4.c:1005 cx_mtimeavg: each point's mean over `mtimeavgwindow`
        // centred on its time, ten of the circuit's tran steps when unset.
        "MTIMEAVG" => {
            let times = circuit
                .scale_values(scale.as_deref(), length)
                .filter(|(_, unit)| *unit == SignalUnit::Second)
                .map(|(times, _)| times)
                .ok_or_else(|| command_error(line, "mtimeavg needs real time scale"))?;
            if complex {
                return Err(command_error(line, "mtimeavg needs a real valued vector."));
            }
            let window = match circuit.shell.get("mtimeavgwindow") {
                Some(ShellValue::Word(text)) => number_prefix(text),
                _ => None,
            }
            .unwrap_or_else(|| {
                circuit
                    .datasets
                    .iter()
                    .rev()
                    .find_map(|dataset| match dataset.command {
                        AnalysisCommand::Tran { step, .. } => Some(10.0 * step),
                        _ => None,
                    })
                    .unwrap_or(1e-6)
            });
            let data: Vec<Value> = samples.iter().map(|sample| sample.re).collect();
            let values = moving_time_average(&times, &data, window)
                .into_iter()
                .map(ComplexValue::from)
                .collect();
            VectorValue {
                samples: values,
                complex,
                scale,
                unit,
            }
        }
        _ => {
            return Err(command_error(
                line,
                format!("control vector function '{name}' is not supported"),
            ));
        }
    };
    Ok(value)
}

/// The mean of the piecewise-linear `data` over the `window` centred on
/// each of its `times`, the window cut to the times' span at its ends, as
/// ngspice documents mtimeavg. cx_mtimeavg itself (cmath4.c:1089) sums
/// rectangles and widens, rather than trims, the window at its edges, so a
/// 0-1-0 V triangle averages to 1.625 V there.
fn moving_time_average(times: &[Value], data: &[Value], window: Value) -> Vec<Value> {
    let (Some(&first), Some(&last)) = (times.first(), times.last()) else {
        return Vec::new();
    };
    let half = window / 2.0;
    times
        .iter()
        .zip(data)
        .map(|(&time, &value)| {
            let (low, high) = ((time - half).max(first), (time + half).min(last));
            if high <= low {
                return value;
            }
            let mut integral = 0.0;
            for (point, next) in times.iter().zip(data).zip(times.iter().zip(data).skip(1)) {
                let ((&start, &from), (&stop, &to)) = (point, next);
                let (left, right) = (start.max(low), stop.min(high));
                if right <= left {
                    continue;
                }
                let at = |x: Value| from + (to - from) * (x - start) / (stop - start);
                integral += (at(left) + at(right)) * (right - left) / 2.0;
            }
            integral / (high - low)
        })
        .collect()
}

/// ngspice `cx_sqrt` for a complex element (cmath1.c:487), whose branch
/// differs from the principal root where the real part is positive and the
/// imaginary part negative.
fn complex_sqrt(value: ComplexValue) -> ComplexValue {
    let (re, im) = (value.re, value.im);
    let magnitude = value.norm();
    if re == 0.0 {
        if im == 0.0 {
            ComplexValue::ZERO
        } else if im > 0.0 {
            let root = (0.5 * im).sqrt();
            ComplexValue::new(root, root)
        } else {
            let root = (-0.5 * im).sqrt();
            ComplexValue::new(-root, root)
        }
    } else if re > 0.0 {
        if im == 0.0 {
            ComplexValue::new(re.sqrt(), 0.0)
        } else {
            let mut root = (0.5 * (magnitude + re)).sqrt();
            if im < 0.0 {
                root = -root;
            }
            ComplexValue::new(root, im / (2.0 * root))
        }
    } else if im == 0.0 {
        ComplexValue::new(0.0, (-re).sqrt())
    } else {
        let mut root = (0.5 * (magnitude - re)).sqrt();
        if im < 0.0 {
            root = -root;
        }
        ComplexValue::new(im / (2.0 * root), root)
    }
}

/// ngspice `cx_deriv` for real data (cmath4.c:246) with its default
/// `dpolydegree` of 2: a quadratic through each three consecutive points,
/// differentiated at the points up to its middle one, and the last fit
/// carried to the end.
fn derivative(scale: &[Value], data: &[Value]) -> Vec<Value> {
    const DEGREE: usize = 2;
    let length = data.len();
    let mut output = vec![0.0; length];
    let mut coefficients = [0.0; DEGREE + 1];
    let mut next = 0;
    for i in DEGREE..length {
        // A failed fit leaves ngspice's zeroed coefficients.
        coefficients =
            polynomial_fit(&scale[i - DEGREE..=i], &data[i - DEGREE..=i]).unwrap_or_default();
        differentiate(&mut coefficients);
        let last = i - DEGREE / 2;
        for j in next..=last {
            output[j] = evaluate(scale[j], &coefficients[..DEGREE]);
        }
        next = last + 1;
    }
    for j in next..length {
        output[j] = evaluate(scale[j], &coefficients[..DEGREE]);
    }
    output
}

/// ngspice `ft_polyfit` (maths/poly/polyfit.c): Gauss-Jordan elimination on
/// the Vandermonde system, with its pivot choice.
fn polynomial_fit(x: &[Value], y: &[Value]) -> Option<[Value; 3]> {
    const N: usize = 3;
    let mut matrix = [[0.0; N]; N];
    let mut right = [0.0; N];
    for i in 0..N {
        let mut power = 1.0;
        for cell in matrix[i].iter_mut() {
            *cell = power;
            power *= x[i];
        }
        right[i] = y[i];
    }
    for i in 0..N {
        let mut largest = matrix[i][i];
        let mut index = i;
        for (j, row) in matrix.iter().enumerate().skip(i) {
            if row[i].abs() > largest {
                largest = row[i].abs();
                index = j;
            }
        }
        if index != i {
            matrix.swap(i, index);
            right.swap(i, index);
        }
        if matrix[i][i] == 0.0 {
            return None;
        }
        for j in i + 1..N {
            let factor = matrix[j][i] / matrix[i][i];
            let pivot = matrix[i];
            for (cell, value) in matrix[j].iter_mut().zip(pivot) {
                *cell -= factor * value;
            }
            right[j] -= factor * right[i];
        }
    }
    for i in (1..N).rev() {
        for j in (0..i).rev() {
            let factor = matrix[j][i] / matrix[i][i];
            let pivot = matrix[i];
            for (cell, value) in matrix[j].iter_mut().zip(pivot) {
                *cell -= factor * value;
            }
            right[j] -= factor * right[i];
        }
    }
    let mut result = [0.0; N];
    for i in 0..N {
        result[i] = right[i] / matrix[i][i];
    }
    Some(result)
}

/// ngspice `ft_polyderiv`.
fn differentiate(coefficients: &mut [Value; 3]) {
    for i in 0..coefficients.len() - 1 {
        coefficients[i] = (i + 1) as Value * coefficients[i + 1];
    }
}

/// ngspice `ft_peval`: Horner's rule on `coefficients`, lowest power first.
fn evaluate(x: Value, coefficients: &[Value]) -> Value {
    coefficients
        .iter()
        .rev()
        .fold(0.0, |value, coefficient| value * x + coefficient)
}
