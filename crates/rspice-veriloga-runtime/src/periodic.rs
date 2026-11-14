//! Operators with state a periodic solver carries as unknowns of its own.
//!
//! Harmonic balance gives a generated model's `idt` sites, and its `ddt`
//! sites that are not the derivative of a charge the model contributes, two
//! waveforms each: the site's input `u` and its output `y`, tied by the
//! operator's transfer at every harmonic. To build that system it needs, at
//! every collocation sample, the model's residual with each output held at a
//! supplied value, each site's input there, and the exact partials of both.
//!
//! A [`GeneratedPeriodicFrame`] on the evaluation context supplies the
//! outputs and records the inputs. A carried site's primal records its input
//! and returns the supplied output; its derivative records the input's
//! partial along the lane (the model unknown) it is taken along and returns
//! the output's tangent there: one on the lane its site is seeded along,
//! zero everywhere else. Forward-mode derivatives are linear in their
//! tangents, so an evaluation with no seed gives the partials with every
//! output held, and a seeded evaluation's difference from it, read along the
//! seeded lane, gives the partials with respect to that output.

use std::cell::{Cell, RefCell};

/// The operator a carried site applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedPeriodicKind {
    /// `ddt(x)`: `jω` at each harmonic.
    Derivative,
    /// `idt(x, ic)`: `1/(jω)` at each harmonic, its DC output fixed by `ic`.
    Integral,
}

/// Supplied outputs and the record of one evaluation, one entry per carried
/// site in the model's site order.
#[derive(Debug, Clone, Default)]
pub struct GeneratedPeriodicFrame {
    outputs: Vec<f64>,
    /// The lane each site's output tangent is one along, if any.
    seeds: Vec<Option<u32>>,
    inputs: Vec<Cell<f64>>,
    initial_conditions: Vec<Cell<f64>>,
    /// Whether a site's initial condition had a nonzero partial.
    dependent_initial_conditions: Vec<Cell<bool>>,
    /// `(site, lane, d input / d lane)` for every derivative evaluated.
    input_partials: RefCell<Vec<(usize, u32, f64)>>,
}

impl GeneratedPeriodicFrame {
    /// A frame for `sites` carried sites, every output zero and no seed.
    pub fn new(sites: usize) -> Self {
        Self {
            outputs: vec![0.0; sites],
            seeds: vec![None; sites],
            inputs: (0..sites).map(|_| Cell::new(f64::NAN)).collect(),
            initial_conditions: (0..sites).map(|_| Cell::new(f64::NAN)).collect(),
            dependent_initial_conditions: (0..sites).map(|_| Cell::new(false)).collect(),
            input_partials: RefCell::new(Vec::new()),
        }
    }

    /// Supply the outputs and the lane each site is seeded along for the
    /// next evaluation, and clear the record. Allocates nothing once the
    /// record has grown to a full evaluation's size.
    pub fn prepare(&mut self, outputs: &[f64], seeds: &[Option<u32>]) {
        for (slot, value) in self.outputs.iter_mut().zip(outputs) {
            *slot = *value;
        }
        for (slot, seed) in self.seeds.iter_mut().enumerate() {
            *seed = seeds.get(slot).copied().flatten();
        }
        for cell in self.inputs.iter().chain(&self.initial_conditions) {
            cell.set(f64::NAN);
        }
        for cell in &self.dependent_initial_conditions {
            cell.set(false);
        }
        self.input_partials.get_mut().clear();
    }

    /// Record `site`'s input (and an `idt`'s initial condition) and return
    /// its supplied output.
    #[inline]
    pub fn primal(&self, site: usize, input: f64, initial_condition: f64) -> f64 {
        self.inputs[site].set(input);
        self.initial_conditions[site].set(initial_condition);
        self.outputs[site]
    }

    /// Record the partial of `site`'s input (and of its initial condition)
    /// along `lane` and return its output's tangent there.
    #[inline]
    pub fn derivative(
        &self,
        site: usize,
        input_derivative: f64,
        initial_condition_derivative: f64,
        lane: u32,
    ) -> f64 {
        self.input_partials
            .borrow_mut()
            .push((site, lane, input_derivative));
        if initial_condition_derivative != 0.0 {
            self.dependent_initial_conditions[site].set(true);
        }
        if self.seeds[site] == Some(lane) {
            1.0
        } else {
            0.0
        }
    }

    /// The input `site`'s primal saw, `NaN` when it did not run.
    pub fn input(&self, site: usize) -> f64 {
        self.inputs[site].get()
    }

    /// The initial condition `site`'s primal saw, `NaN` when it did not run
    /// or the site has none.
    pub fn initial_condition(&self, site: usize) -> f64 {
        self.initial_conditions[site].get()
    }

    /// Whether `site`'s initial condition depended on the model's unknowns.
    pub fn initial_condition_is_dependent(&self, site: usize) -> bool {
        self.dependent_initial_conditions[site].get()
    }

    /// The partial of `site`'s input along `lane`, `None` when no
    /// derivative was taken there. A lane evaluated twice keeps its first
    /// value: both evaluations differentiate the same input.
    pub fn input_partial(&self, site: usize, lane: u32) -> Option<f64> {
        self.input_partials
            .borrow()
            .iter()
            .find(|&&(at, along, _)| at == site && along == lane)
            .map(|&(_, _, value)| value)
    }

    /// Every lane a derivative of `site`'s input was taken along, in the
    /// order they were evaluated, each once.
    pub fn input_lanes(&self, site: usize, lanes: &mut Vec<u32>) {
        lanes.clear();
        for &(at, lane, _) in self.input_partials.borrow().iter() {
            if at == site && !lanes.contains(&lane) {
                lanes.push(lane);
            }
        }
    }
}
