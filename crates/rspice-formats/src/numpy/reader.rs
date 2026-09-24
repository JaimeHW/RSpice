//! Bounded NPY decoding and projection into numeric waveform columns.

use std::io::Cursor;

#[derive(Debug)]
pub struct NpyArray {
    shape: Vec<usize>,
    fortran: bool,
    real: Vec<f64>,
    imag: Option<Vec<f64>>,
}

impl NpyArray {
    pub fn is_complex(&self) -> bool {
        self.imag.is_some()
    }
}

#[derive(Debug)]
pub struct NpySignal {
    pub name: String,
    pub real: Vec<f64>,
    pub imag: Option<Vec<f64>>,
}

fn adapter_error(format: &str, detail: impl std::fmt::Display) -> String {
    format!("{format} import: {detail}")
}

pub fn decode_npy(bytes: &[u8], max_values: usize, format: &str) -> Result<NpyArray, String> {
    use npyz::{DType, Order, TypeChar};
    let file = npyz::NpyFile::new(Cursor::new(bytes))
        .map_err(|error| adapter_error(format, format_args!("invalid NPY header: {error}")))?;
    let shape = file
        .shape()
        .iter()
        .map(|value| {
            usize::try_from(*value)
                .map_err(|_| adapter_error(format, "NPY dimension exceeds this platform"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if shape.is_empty() || shape.len() > 2 {
        return Err(adapter_error(
            format,
            format_args!("NPY shape {shape:?} is not a one- or two-dimensional waveform table"),
        ));
    }
    let count = shape
        .iter()
        .try_fold(1_usize, |count, dim| count.checked_mul(*dim))
        .ok_or_else(|| adapter_error(format, "NPY shape product overflow"))?;
    if count > max_values {
        return Err(adapter_error(format, "NPY numeric-value limit exceeded"));
    }
    let fortran = file.order() == Order::Fortran;
    let DType::Plain(type_string) = file.dtype() else {
        return Err(adapter_error(
            format,
            "structured and nested NPY dtypes require an explicit mapping and are not accepted",
        ));
    };
    macro_rules! real {
        ($ty:ty) => {{
            let values = file.into_vec::<$ty>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
            NpyArray {
                shape,
                fortran,
                real: values.into_iter().map(|value| value as f64).collect(),
                imag: None,
            }
        }};
    }
    macro_rules! complex {
        ($ty:ty) => {{
            let values = file.into_vec::<$ty>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
            NpyArray {
                shape,
                fortran,
                real: values.iter().map(|value| value.re as f64).collect(),
                imag: Some(values.iter().map(|value| value.im as f64).collect()),
            }
        }};
    }
    let array = match (type_string.type_char(), type_string.size_field()) {
        (TypeChar::Float, 4) => real!(f32),
        (TypeChar::Float, 8) => real!(f64),
        (TypeChar::Int, 1) => real!(i8),
        (TypeChar::Int, 2) => real!(i16),
        (TypeChar::Int, 4) => real!(i32),
        (TypeChar::Int, 8) => {
            let values = file.into_vec::<i64>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
            NpyArray {
                shape,
                fortran,
                real: values
                    .into_iter()
                    .map(|value| {
                        crate::numeric::exact_signed_integer("NPY array", value)
                            .map_err(|detail| adapter_error(format, detail))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                imag: None,
            }
        }
        (TypeChar::Uint, 1) => real!(u8),
        (TypeChar::Uint, 2) => real!(u16),
        (TypeChar::Uint, 4) => real!(u32),
        (TypeChar::Uint, 8) => {
            let values = file.into_vec::<u64>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
            NpyArray {
                shape,
                fortran,
                real: values
                    .into_iter()
                    .map(|value| {
                        crate::numeric::exact_unsigned_integer("NPY array", value)
                            .map_err(|detail| adapter_error(format, detail))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                imag: None,
            }
        }
        (TypeChar::Bool, 1) => {
            let values = file.into_vec::<bool>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
            NpyArray {
                shape,
                fortran,
                real: values
                    .into_iter()
                    .map(|value| if value { 1.0 } else { 0.0 })
                    .collect(),
                imag: None,
            }
        }
        (TypeChar::Complex, 8) => complex!(num_complex::Complex32),
        (TypeChar::Complex, 16) => complex!(num_complex::Complex64),
        (kind, size) => {
            return Err(adapter_error(
                format,
                format_args!("unsupported NPY dtype {kind:?}{size}"),
            ));
        }
    };
    Ok(array)
}

pub fn npy_vector(
    array: &NpyArray,
    format: &str,
    name: &str,
) -> Result<(Vec<f64>, Option<Vec<f64>>), String> {
    let len = match array.shape.as_slice() {
        [len] => *len,
        [rows, 1] => *rows,
        [1, columns] => *columns,
        _ => {
            return Err(adapter_error(
                format,
                format_args!("NPZ array '{name}' has non-vector shape {:?}", array.shape),
            ));
        }
    };
    if array.real.len() != len || array.imag.as_ref().is_some_and(|imag| imag.len() != len) {
        return Err(adapter_error(
            format,
            format_args!("NPZ array '{name}' payload does not match its shape"),
        ));
    }
    Ok((array.real.clone(), array.imag.clone()))
}

pub fn npy_matrix_to_dataset(
    array: NpyArray,
    max_rows: usize,
    max_columns: usize,
    format: &str,
) -> Result<(Vec<f64>, Vec<NpySignal>), String> {
    let (rows, columns) = match array.shape.as_slice() {
        [rows] => (*rows, 1),
        [rows, columns] => (*rows, *columns),
        _ => unreachable!(),
    };
    if !(1..=max_rows).contains(&rows) || columns == 0 || columns > max_columns {
        return Err(adapter_error(
            format,
            format_args!("NPY waveform shape {:?} exceeds import bounds", array.shape),
        ));
    }
    let index = |row: usize, column: usize| {
        if array.fortran {
            column * rows + row
        } else {
            row * columns + column
        }
    };
    if array.imag.is_none() && columns >= 2 {
        let coordinate = (0..rows).map(|row| array.real[index(row, 0)]).collect();
        let signals = (1..columns)
            .map(|column| NpySignal {
                name: format!("signal_{column}"),
                real: (0..rows)
                    .map(|row| array.real[index(row, column)])
                    .collect(),
                imag: None,
            })
            .collect();
        return Ok((coordinate, signals));
    }
    let coordinate = (0..rows).map(|row| row as f64).collect();
    let signals = (0..columns)
        .map(|column| NpySignal {
            name: if columns == 1 {
                "value".to_owned()
            } else {
                format!("signal_{}", column + 1)
            },
            real: (0..rows)
                .map(|row| array.real[index(row, column)])
                .collect(),
            imag: array
                .imag
                .as_ref()
                .map(|imag| (0..rows).map(|row| imag[index(row, column)]).collect()),
        })
        .collect();
    Ok((coordinate, signals))
}

#[cfg(test)]
mod tests {
    use super::*;
    use npyz::WriterBuilder as _;

    #[test]
    fn projects_fortran_order_without_transposing_samples() {
        let mut bytes = Vec::new();
        let mut writer = npyz::WriteOptions::<f64>::new()
            .default_dtype()
            .shape(&[3, 2])
            .order(npyz::Order::Fortran)
            .writer(&mut bytes)
            .begin_nd()
            .expect("NPY writer");
        writer
            .extend([0.0, 1.0, 2.0, 10.0, 20.0, 30.0])
            .expect("NPY values");
        writer.finish().expect("NPY finish");

        let array = decode_npy(&bytes, 6, "numpy_npy").expect("decode");
        let (coordinate, signals) =
            npy_matrix_to_dataset(array, 3, 2, "numpy_npy").expect("project");
        assert_eq!(coordinate, [0.0, 1.0, 2.0]);
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].real, [10.0, 20.0, 30.0]);
    }

    #[test]
    fn projects_complex_columns_with_implicit_sample_coordinate() {
        let values = [
            num_complex::Complex64::new(1.0, -2.0),
            num_complex::Complex64::new(3.0, 4.0),
        ];
        let bytes = crate::numpy::encode_complex_array(&[2], &values).expect("NPY fixture");
        let array = decode_npy(&bytes, 2, "numpy_npy").expect("decode");
        let (coordinate, signals) =
            npy_matrix_to_dataset(array, 2, 1, "numpy_npy").expect("project");
        assert_eq!(coordinate, [0.0, 1.0]);
        assert_eq!(signals[0].name, "value");
        assert_eq!(signals[0].real, [1.0, 3.0]);
        assert_eq!(signals[0].imag.as_deref(), Some(&[-2.0, 4.0][..]));
    }

    #[test]
    fn rejects_integer_values_that_would_lose_precision() {
        let mut bytes = Vec::new();
        let mut writer = npyz::WriteOptions::<u64>::new()
            .default_dtype()
            .shape(&[1])
            .writer(&mut bytes)
            .begin_nd()
            .expect("NPY writer");
        writer
            .extend([crate::numeric::MAX_EXACT_F64_INTEGER + 1])
            .expect("NPY value");
        writer.finish().expect("NPY finish");

        let error = decode_npy(&bytes, 1, "numpy_npy").expect_err("precision loss");
        assert!(error.contains("cannot be represented exactly as f64"));
    }
}
