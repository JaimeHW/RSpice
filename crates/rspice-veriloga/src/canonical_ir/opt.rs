use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashSet;

use super::{
    CompilerPhase, EquationId, IrDiagnostic, IrValidationResult, MirModel, ScheduleId, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvalidationClass {
    InstanceStatic,
    TemperatureStatic,
    TimestepStatic,
    OperatingPointStatic,
    NewtonIteration,
    AcFrequency,
    NoiseFrequency,
    OperatingPointReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptValueType {
    Real,
    Boolean,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptValue {
    pub id: ValueId,
    pub value_type: OptValueType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptOp {
    EvaluateEquation { equation: EquationId },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptSchedule {
    pub id: ScheduleId,
    pub invalidation: InvalidationClass,
    pub ops: Vec<OptOp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptModel {
    pub module_name: SmolStr,
    pub equation_count: u32,
    pub values: Vec<OptValue>,
    pub schedules: Vec<OptSchedule>,
}

impl OptModel {
    pub fn from_mir(mir: &MirModel) -> Result<Self, Vec<IrDiagnostic>> {
        mir.validate()?;

        let mut schedules = Vec::new();
        if !mir.parameters.is_empty() {
            schedules.push(OptSchedule {
                id: ScheduleId::from(schedules.len()),
                invalidation: InvalidationClass::InstanceStatic,
                ops: Vec::new(),
            });
        }

        let newton_ops = mir
            .equations
            .iter()
            .map(|equation| OptOp::EvaluateEquation {
                equation: equation.id,
            })
            .collect();
        schedules.push(OptSchedule {
            id: ScheduleId::from(schedules.len()),
            invalidation: InvalidationClass::NewtonIteration,
            ops: newton_ops,
        });

        let opt = Self {
            module_name: mir.module_name.clone(),
            equation_count: u32::try_from(mir.equations.len())
                .expect("MIR equation count exceeds u32::MAX"),
            values: Vec::new(),
            schedules,
        };

        opt.validate().map(|()| opt)
    }

    pub fn validate(&self) -> IrValidationResult {
        let mut diagnostics = Vec::new();

        if self.module_name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                "OptIR module name must not be empty",
            ));
        }

        validate_dense_value_ids(&mut diagnostics, &self.values);
        validate_dense_schedule_ids(&mut diagnostics, &self.schedules);
        validate_schedules(&mut diagnostics, &self.schedules, self.equation_count);

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
}

fn validate_dense_value_ids(diagnostics: &mut Vec<IrDiagnostic>, values: &[OptValue]) {
    for (expected, value) in values.iter().enumerate() {
        let expected = u32::try_from(expected).expect("OptIR value count exceeds u32::MAX");
        if value.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR value IDs must be dense: expected ValueId({}) at index {}, found {}",
                    expected, expected, value.id
                ),
            ));
        }
    }
}

fn validate_dense_schedule_ids(diagnostics: &mut Vec<IrDiagnostic>, schedules: &[OptSchedule]) {
    for (expected, schedule) in schedules.iter().enumerate() {
        let expected = u32::try_from(expected).expect("OptIR schedule count exceeds u32::MAX");
        if schedule.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR schedule IDs must be dense: expected ScheduleId({}) at index {}, found {}",
                    expected, expected, schedule.id
                ),
            ));
        }
    }
}

fn validate_schedules(
    diagnostics: &mut Vec<IrDiagnostic>,
    schedules: &[OptSchedule],
    equation_count: u32,
) {
    let mut invalidations = HashSet::new();
    let mut newton_count = 0;

    for schedule in schedules {
        if !invalidations.insert(schedule.invalidation) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR duplicate schedule for invalidation {:?}",
                    schedule.invalidation
                ),
            ));
        }

        if schedule.invalidation == InvalidationClass::NewtonIteration {
            newton_count += 1;
            validate_newton_schedule(diagnostics, schedule, equation_count);
        } else {
            validate_schedule_ops(diagnostics, schedule, equation_count);
        }
    }

    if newton_count != 1 {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR must contain exactly one NewtonIteration schedule, found {}",
                newton_count
            ),
        ));
    }
}

fn validate_newton_schedule(
    diagnostics: &mut Vec<IrDiagnostic>,
    schedule: &OptSchedule,
    equation_count: u32,
) {
    validate_schedule_ops(diagnostics, schedule, equation_count);

    if schedule.ops.len() != equation_count as usize {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR NewtonIteration schedule must contain one op per equation: expected {}, found {}",
                equation_count,
                schedule.ops.len()
            ),
        ));
    }

    for (expected, op) in schedule.ops.iter().enumerate() {
        let expected = u32::try_from(expected).expect("OptIR op count exceeds u32::MAX");
        let OptOp::EvaluateEquation { equation } = op;
        if equation.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR NewtonIteration op at index {} must evaluate EquationId({}), found {}",
                    expected, expected, equation
                ),
            ));
        }
    }
}

fn validate_schedule_ops(
    diagnostics: &mut Vec<IrDiagnostic>,
    schedule: &OptSchedule,
    equation_count: u32,
) {
    let mut equations = HashSet::new();

    for op in &schedule.ops {
        let OptOp::EvaluateEquation { equation } = op;
        if equation.index() >= equation_count {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR schedule {} EvaluateEquation {} is out of range for {} equations",
                    schedule.id, equation, equation_count
                ),
            ));
        }

        if !equations.insert(*equation) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR schedule {} has duplicate equation {}",
                    schedule.id, equation
                ),
            ));
        }
    }
}
