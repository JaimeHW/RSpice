use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CadencePsfValue {
    Int(i64),
    Real(f64),
    Text(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedRealSignal {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedComplexSignal {
    pub name: String,
    pub values: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedCadencePsfBinary {
    pub header: HashMap<String, CadencePsfValue>,
    pub sweeps: Vec<NamedRealSignal>,
    pub real_signals: Vec<NamedRealSignal>,
    pub complex_signals: Vec<NamedComplexSignal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CadencePsfError {
    message: String,
}

impl CadencePsfError {
    pub(super) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for CadencePsfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CadencePsfError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DataType {
    Int8,
    Int32,
    Real,
    Complex,
    String,
    Array,
    Struct,
    Other(u32),
}

impl DataType {
    pub(super) fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::Int8,
            2 => Self::String,
            3 => Self::Array,
            5 => Self::Int32,
            11 => Self::Real,
            12 => Self::Complex,
            16 => Self::Struct,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct TypeDecl {
    pub(super) name: String,
    pub(super) kind: TypeKind,
}

#[derive(Debug, Clone)]
pub(super) enum TypeKind {
    Primitive(DataType),
    Array { element_type_raw: u32 },
    Struct { members: Vec<u32> },
}

#[derive(Debug, Clone, Copy)]
pub(super) enum ArrayElementType {
    Primitive(DataType),
    TypeRef(u32),
}

#[derive(Debug, Clone)]
pub(super) struct SignalRef {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) type_id: u32,
}

#[derive(Debug, Clone)]
pub(super) enum TraceDef {
    Signal(SignalRef),
    Group(Vec<SignalRef>),
}

#[derive(Debug, Clone)]
pub(super) enum SignalValues {
    Real(Vec<f64>),
    Complex(Vec<(f64, f64)>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ChannelKind {
    Real,
    Complex,
}

#[derive(Debug, Clone)]
pub(super) struct ChannelSpec {
    pub(super) suffix: String,
    pub(super) kind: ChannelKind,
}

#[derive(Debug, Clone)]
pub(super) struct SignalChannel {
    pub(super) suffix: String,
    pub(super) values: SignalValues,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum NumericSample {
    Real(f64),
    Complex((f64, f64)),
}
