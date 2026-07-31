//! Hand-written compact models, measured the way the generated ones are.
//!
//! The gate for the Verilog-A backend is stated against hand-written C, and
//! until this module existed that comparison lived in prose: numbers from an
//! ngspice profile on one machine set beside numbers from
//! [`super::bench`] on another, with different biases, different model cards,
//! and different definitions of "one evaluation". Nothing in the repository
//! produced both sides.
//!
//! [`Bsim4v8Device`] is a direct port of ngspice's `b4ld.c` — the same
//! equations, the same operating-point structure, hand-maintained rather than
//! emitted. Driving it through the same loop as a generated device makes
//! "versus hand-written" a number this repository can print, on whatever
//! machine is asking.
//!
//! ## What is held equal
//!
//! * **Work per call.** A generated `stamp` evaluates the model and writes its
//!   contributions in one call. The hand-written device splits that into
//!   `update` (evaluate, with limiting) and `stamp_nonlinear` (write); both are
//!   timed, and `stamp_nonlinear` reuses the operating point `update` just
//!   computed, so exactly one evaluation is charged per iteration.
//! * **Limiting.** Both sides run with Newton limiting engaged, because that is
//!   what a solver pays for.
//! * **Bias motion.** The bias advances every iteration on both sides. Newton
//!   never stamps the same point twice, and a fixed vector would let either
//!   side report throughput that bias-keyed caching had manufactured.
//! * **Model card.** Defaults on both sides, for the reason
//!   [`super::bench`] gives: what a backend's cost depends on is operation
//!   count and derivative width, which defaults exercise faithfully, and no
//!   card exists that is equally representative for forty-odd models.
//!
//! ## What is not
//!
//! BSIM4 and, say, BSIM-BULK are different models — this is a comparison
//! between a hand-written MOSFET and a generated MOSFET of comparable
//! complexity, not two implementations of one specification. Read it as an
//! order-of-magnitude reference, and read the same-model tier gap in
//! [`super::bench`] for the exact figure.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use rspice_core::Value;
use rspice_core::NodeId;
use rspice_core::device::mosfet::bsim4v8::{Bsim4v8, Bsim4v8Device, Bsim4v8Geometry, Bsim4v8Model};
use rspice_core::device::{MatrixStamper, NonlinearDevice};

/// Bias applied to every ungrounded node, in volts.
///
/// Matches [`super::bench`] so the two tables can be read against each other.
const REFERENCE_NODE_BIAS: Value = 0.35;

/// Nominal and operating temperature, in kelvin.
const REFERENCE_TEMPERATURE: Value = 300.15;

#[derive(Debug, Clone)]
pub struct ReferenceStampBenchConfig {
    pub iterations: usize,
    pub samples: usize,
}

impl Default for ReferenceStampBenchConfig {
    fn default() -> Self {
        Self {
            iterations: 2_000,
            samples: 7,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReferenceStampBenchResult {
    pub model_name: &'static str,
    pub node_count: usize,
    pub ns_per_stamp_median: f64,
    pub ns_per_stamp_p95: f64,
    pub ns_per_stamp_min: f64,
}

/// Dense stamp target.
///
/// A generated device writes through a linked CSC index; this writes into a
/// dense block. The difference is a handful of stores either way and is not
/// what either measurement is about, but it is the reason the two numbers
/// should be compared as ratios rather than differenced.
struct DenseStamper {
    matrix: Vec<Value>,
    size: usize,
}

impl DenseStamper {
    fn new(size: usize) -> Self {
        Self {
            matrix: vec![0.0; size * size],
            size,
        }
    }

    fn clear(&mut self) {
        self.matrix.fill(0.0);
    }

    fn wrote_anything(&self) -> bool {
        self.matrix.iter().any(|value| *value != 0.0)
    }
}

impl MatrixStamper for DenseStamper {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if row > 0 && col > 0 && row <= self.size && col <= self.size {
            self.matrix[(row - 1) * self.size + (col - 1)] += value;
        }
    }

    fn stamp_rhs(&mut self, _index: NodeId, _value: Value) {
        // BSIM4's nonlinear stamp writes its equivalent currents through the
        // matrix path; the trait's rhs hook is unused by this device.
    }
}

/// Every terminal ungrounded, matching the generated harness.
const BSIM4_NODE_COUNT: usize = 13;

fn bsim4_default_device() -> Result<Bsim4v8Device, String> {
    // An empty card selects every BSIM4 default, which is the hand-written
    // analogue of instantiating a generated model with no overrides.
    let model = Arc::new(Bsim4v8Model::try_from_params(
        &HashMap::new(),
        false,
        REFERENCE_TEMPERATURE,
    )?);
    let geometry = Bsim4v8Geometry {
        l: 45.0e-9,
        w: 1.0e-6,
        nf: 1.0,
        m: 1.0,
        ..Bsim4v8Geometry::default()
    };
    let core = Bsim4v8::new("mref".to_string(), model, geometry, REFERENCE_TEMPERATURE)?;

    Ok(Bsim4v8Device::new(
        "mref".to_string(),
        1,  // drain external
        2,  // drain
        3,  // gate external
        4,  // gate mid
        5,  // gate
        6,  // source external
        7,  // source
        8,  // bulk external
        9,  // bulk
        10, // drain body
        11, // source body
        12, // charge deficit
        1.0,
        core,
    ))
}

/// Time the hand-written BSIM4 stamp, in the units [`super::bench`] reports.
pub fn run_reference_stamp_benchmark(
    config: &ReferenceStampBenchConfig,
) -> Result<ReferenceStampBenchResult, String> {
    let mut device = bsim4_default_device()?;
    let mut stamper = DenseStamper::new(BSIM4_NODE_COUNT);
    let mut rhs = vec![0.0; BSIM4_NODE_COUNT];
    let mut voltages = vec![REFERENCE_NODE_BIAS; BSIM4_NODE_COUNT];

    // One untimed pass, for the same reason the generated harness takes one:
    // it populates whatever the device caches on first evaluation, and timing
    // it would charge one-time setup to the steady-state cost.
    device.update(&voltages);
    device.stamp_nonlinear(&voltages, &mut stamper, &mut rhs);
    if !stamper.wrote_anything() {
        return Err(
            "hand-written BSIM4 wrote no matrix entries at the reference bias; the timing would be meaningless"
                .to_string(),
        );
    }

    let samples = config.samples.max(1);
    let iterations = config.iterations.max(1);
    let mut per_stamp_ns = Vec::with_capacity(samples);
    for _ in 0..samples {
        let started = Instant::now();
        for iteration in 0..iterations {
            voltages[0] = REFERENCE_NODE_BIAS + (iteration % 64) as Value * 1.0e-5;
            device.update(&voltages);
            stamper.clear();
            device.stamp_nonlinear(&voltages, &mut stamper, &mut rhs);
        }
        let elapsed = started.elapsed();
        per_stamp_ns.push(elapsed.as_secs_f64() * 1.0e9 / iterations as f64);
    }

    per_stamp_ns.sort_by(f64::total_cmp);
    Ok(ReferenceStampBenchResult {
        model_name: "bsim4v8 (hand-written)",
        node_count: BSIM4_NODE_COUNT,
        ns_per_stamp_median: super::bench::percentile(&per_stamp_ns, 0.50),
        ns_per_stamp_p95: super::bench::percentile(&per_stamp_ns, 0.95),
        ns_per_stamp_min: per_stamp_ns[0],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_reference_device_builds_from_defaults() {
        bsim4_default_device().expect("an empty card must select BSIM4 defaults");
    }

    #[test]
    fn the_reference_device_stamps_at_the_bench_bias() {
        let mut device = bsim4_default_device().expect("reference device");
        let mut stamper = DenseStamper::new(BSIM4_NODE_COUNT);
        let mut rhs = vec![0.0; BSIM4_NODE_COUNT];
        let voltages = vec![REFERENCE_NODE_BIAS; BSIM4_NODE_COUNT];
        device.update(&voltages);
        device.stamp_nonlinear(&voltages, &mut stamper, &mut rhs);
        assert!(
            stamper.wrote_anything(),
            "a measurement of a device that stamps nothing is not a measurement"
        );
    }

    #[test]
    fn the_dense_stamper_drops_ground_without_panicking() {
        let mut stamper = DenseStamper::new(2);
        stamper.stamp(0, 1, 1.0);
        stamper.stamp(1, 0, 1.0);
        stamper.stamp(9, 9, 1.0);
        assert!(!stamper.wrote_anything());
        stamper.stamp(1, 1, 1.0);
        assert!(stamper.wrote_anything());
    }
}
