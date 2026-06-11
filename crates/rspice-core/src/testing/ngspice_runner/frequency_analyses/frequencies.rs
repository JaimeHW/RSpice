use super::*;
use crate::analysis::ac::ac_sweep_frequencies;
use crate::netlist::FreqVariation;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn generate_decade_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_decade: usize,
    ) -> Vec<Value> {
        ac_sweep_frequencies(FreqVariation::Dec, points_per_decade, fstart, fstop)
    }

    pub(in crate::testing::ngspice_runner) fn generate_octave_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_octave: usize,
    ) -> Vec<Value> {
        ac_sweep_frequencies(FreqVariation::Oct, points_per_octave, fstart, fstop)
    }

    pub(in crate::testing::ngspice_runner) fn generate_linear_points(
        &self,
        fstart: Value,
        fstop: Value,
        num_points: usize,
    ) -> Vec<Value> {
        ac_sweep_frequencies(FreqVariation::Lin, num_points, fstart, fstop)
    }
}
