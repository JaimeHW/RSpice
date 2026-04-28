use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn generate_decade_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_decade: usize,
    ) -> Vec<Value> {
        let mut freqs = Vec::new();
        let decades = (fstop / fstart).log10();
        let total_points = (decades * points_per_decade as f64).ceil() as usize;

        for i in 0..=total_points {
            let f = fstart * 10f64.powf(i as f64 / points_per_decade as f64);
            if f <= fstop {
                freqs.push(f);
            }
        }
        freqs
    }

    pub(in crate::testing::ngspice_runner) fn generate_octave_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_octave: usize,
    ) -> Vec<Value> {
        let mut freqs = Vec::new();
        let octaves = (fstop / fstart).log2();
        let total_points = (octaves * points_per_octave as f64).ceil() as usize;

        for i in 0..=total_points {
            let f = fstart * 2f64.powf(i as f64 / points_per_octave as f64);
            if f <= fstop {
                freqs.push(f);
            }
        }
        freqs
    }

    pub(in crate::testing::ngspice_runner) fn generate_linear_points(
        &self,
        fstart: Value,
        fstop: Value,
        num_points: usize,
    ) -> Vec<Value> {
        let step = (fstop - fstart) / (num_points - 1).max(1) as f64;
        (0..num_points).map(|i| fstart + i as f64 * step).collect()
    }
}
