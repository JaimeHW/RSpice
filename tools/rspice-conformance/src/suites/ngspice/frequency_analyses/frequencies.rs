use super::*;
use rspice_core::analysis::ac::ac_sweep_frequencies;
use rspice_core::netlist::FreqVariation;

impl TestRunner {
    pub(in crate::suites::ngspice) fn generate_decade_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_decade: usize,
    ) -> Vec<Value> {
        ac_sweep_frequencies(FreqVariation::Dec, points_per_decade, fstart, fstop)
    }

    pub(in crate::suites::ngspice) fn generate_octave_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_octave: usize,
    ) -> Vec<Value> {
        ac_sweep_frequencies(FreqVariation::Oct, points_per_octave, fstart, fstop)
    }

    pub(in crate::suites::ngspice) fn generate_linear_points(
        &self,
        fstart: Value,
        fstop: Value,
        num_points: usize,
    ) -> Vec<Value> {
        ac_sweep_frequencies(FreqVariation::Lin, num_points, fstart, fstop)
    }
}
