//! Operators with state evaluated at outputs a periodic solver supplies.
//!
//! Harmonic balance carries every `idt`, `laplace_*` and `absdelay` site, and
//! every `ddt` site that is not the derivative of a charge the module
//! contributes, as unknowns of its own: the site's output is a waveform the solver owns, tied
//! to the site's input by the operator's transfer at each harmonic. To build
//! that system it needs, at every collocation sample, the module's residual
//! with each output held at a supplied value, each site's input there, and the
//! exact partials of both.
//!
//! A [`PeriodicOperatorFrame`] installed on a context supplies the outputs and
//! records the inputs. A site's primal instruction records its input (and its
//! initial condition or delay) and pushes the supplied output; it touches no
//! operator state. A site's derivative instruction records the input's
//! derivative along the column the running program differentiates, and pushes
//! the output's tangent: one for the seeded site, zero for every other.
//!
//! Forward-mode derivative programs are linear in their tangents, so an
//! evaluation with no seed gives the partials with every output held, and the
//! difference of a seeded evaluation from it gives the partials with respect
//! to the seeded output — on the residual, through the stamp, and on every
//! other site's input, through its record.

use super::Axis;
use std::collections::BTreeMap;
use std::sync::Arc;

/// The family of an operator a periodic solver carries as its own unknowns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PeriodicOperatorKind {
    /// `idt(expr, ic)`: `1/(jω)` at each harmonic, its DC term fixed by `ic`.
    Integral,
    /// A rational `laplace_*` filter: `H(jω)` at each harmonic.
    Laplace,
    /// `absdelay(expr, td)`: `e^{-jωτ}` at each harmonic.
    Delay,
    /// `ddt(expr)` the residual cannot carry as the derivative of a charge it
    /// contributes: `jω` at each harmonic.
    Derivative,
}

/// One operator site, by family and the slot its instructions address: the
/// integration slot of an `idt`, the filter of a `laplace_*`, the buffer of an
/// `absdelay`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeriodicOperatorSite {
    pub kind: PeriodicOperatorKind,
    pub slot: usize,
}

/// A derivative column of the compiled model: a node (terminals, then
/// internal nodes) or a branch-current unknown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PeriodicOperatorAxis {
    Node(usize),
    Branch(usize),
}

impl PeriodicOperatorAxis {
    pub(crate) fn of(axis: Axis) -> Option<Self> {
        match axis {
            Axis::Node(node) => Some(Self::Node(node)),
            Axis::Branch(branch) => Some(Self::Branch(branch)),
            Axis::Noise(_) => None,
        }
    }
}

/// What one evaluation under a frame saw, indexed like the frame's sites.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PeriodicOperatorRecord {
    /// The input each site's primal instruction saw.
    pub inputs: Vec<Option<f64>>,
    /// The initial condition each `idt` site's primal instruction saw.
    pub initial_conditions: Vec<Option<f64>>,
    /// The delay each `absdelay` site's primal instruction saw.
    pub delays: Vec<Option<f64>>,
    /// `d input / d axis` for each site along each column a derivative
    /// program evaluated it.
    pub input_partials: BTreeMap<(usize, PeriodicOperatorAxis), f64>,
    /// Sites whose initial condition or delay has a nonzero derivative.
    pub dependent_parameters: Vec<bool>,
}

/// Supplied outputs and the record of one evaluation.
#[derive(Clone, Debug)]
pub(crate) struct PeriodicOperatorFrame {
    sites: Arc<[PeriodicOperatorSite]>,
    /// The derivative column each variable slot's assignment computes, for
    /// first-order derivative shadows only.
    columns: Arc<[Option<PeriodicOperatorAxis>]>,
    outputs: Vec<f64>,
    seed: Option<usize>,
    column: Option<PeriodicOperatorAxis>,
    pub(crate) record: PeriodicOperatorRecord,
}

impl PeriodicOperatorFrame {
    pub(crate) fn new(
        sites: Arc<[PeriodicOperatorSite]>,
        columns: Arc<[Option<PeriodicOperatorAxis>]>,
    ) -> Self {
        let count = sites.len();
        Self {
            sites,
            columns,
            outputs: vec![0.0; count],
            seed: None,
            column: None,
            record: PeriodicOperatorRecord::empty(count),
        }
    }

    /// Supply the outputs and the seeded site for the next evaluation, and
    /// clear the record.
    pub(crate) fn prepare(&mut self, outputs: &[f64], seed: Option<usize>) {
        self.outputs.clear();
        self.outputs.extend_from_slice(outputs);
        self.outputs.resize(self.sites.len(), 0.0);
        self.seed = seed.filter(|&site| site < self.sites.len());
        self.column = None;
        self.record = PeriodicOperatorRecord::empty(self.sites.len());
    }

    /// The column the next program differentiates along, or `None` for a
    /// primal program.
    pub(crate) fn set_column(&mut self, column: Option<PeriodicOperatorAxis>) {
        self.column = column;
    }

    /// The column an assignment to variable `slot` differentiates along.
    pub(crate) fn set_assignment_column(&mut self, slot: usize) {
        self.column = self.columns.get(slot).copied().flatten();
    }

    pub(crate) fn site(&self, kind: PeriodicOperatorKind, slot: usize) -> Option<usize> {
        self.sites
            .iter()
            .position(|site| site.kind == kind && site.slot == slot)
    }

    /// Record a primal evaluation of `site` and return its supplied output.
    pub(crate) fn primal(
        &mut self,
        site: usize,
        input: f64,
        initial_condition: Option<f64>,
        delay: Option<f64>,
    ) -> f64 {
        self.record.inputs[site] = Some(input);
        if initial_condition.is_some() {
            self.record.initial_conditions[site] = initial_condition;
        }
        if delay.is_some() {
            self.record.delays[site] = delay;
        }
        self.outputs[site]
    }

    /// Record a first derivative of `site` along the running column and
    /// return the output's tangent.
    pub(crate) fn derivative(
        &mut self,
        site: usize,
        input_derivative: f64,
        parameter_derivative: f64,
    ) -> f64 {
        if let Some(column) = self.column {
            self.record
                .input_partials
                .insert((site, column), input_derivative);
        }
        if parameter_derivative != 0.0 {
            self.record.dependent_parameters[site] = true;
        }
        if self.seed == Some(site) { 1.0 } else { 0.0 }
    }
}

impl PeriodicOperatorRecord {
    fn empty(count: usize) -> Self {
        Self {
            inputs: vec![None; count],
            initial_conditions: vec![None; count],
            delays: vec![None; count],
            input_partials: BTreeMap::new(),
            dependent_parameters: vec![false; count],
        }
    }
}
