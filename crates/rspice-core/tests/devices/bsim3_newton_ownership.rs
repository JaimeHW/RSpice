//! BSIM3 runs ngspice's per-iterate limiting itself (`DEVfetlim` /
//! `DEVlimvds` / `DEVpnjlim` against the previous iterate), so the engine must
//! treat its Newton steps the way it treats the classic MOS levels': the full
//! node step is the algorithm, and merit-driven damping laid on top of an
//! already-limited step stalls turn-on rather than helping it.

use rspice_core::engine::{ConvergenceConfig, DampingStrategy, Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// The paranoia corpus's four-bit NAND adder: 36 gates, 144 BSIM3 transistors
/// on default models, every input low so every internal stack node floats
/// between two off transistors. Under the `Combined` policy (voltage limit,
/// Bank-Rose alpha, line search) this never reached an operating point.
const FOUR_BIT_ADDER: &str = "\
* four-bit NAND adder, BSIM3 defaults, all inputs low
.SUBCKT NAND in1 in2 out VDD
M1 out in2 Vdd Vdd p1 W=7.5u L=0.35u pd=13.5u ad=22.5p ps=13.5u as=22.5p
M2 net.1 in2 0 0 n1   W=3u   L=0.35u pd=9u    ad=9p    ps=9u    as=9p
M3 out in1 Vdd Vdd p1 W=7.5u L=0.35u pd=13.5u ad=22.5p ps=13.5u as=22.5p
M4 out in1 net.1 0 n1 W=3u   L=0.35u pd=9u    ad=9p    ps=9u    as=9p
.ENDS NAND
.SUBCKT ONEBIT 1 2 3 4 5 6
X1   1  2  7  6   NAND
X2   1  7  8  6   NAND
X3   2  7  9  6   NAND
X4   8  9 10  6   NAND
X5   3 10 11  6   NAND
X6   3 11 12  6   NAND
X7  10 11 13  6   NAND
X8  12 13  4  6   NAND
X9  11  7  5  6   NAND
.ENDS ONEBIT
.SUBCKT TWOBIT 1 2 3 4 5 6 7 8 9
X1   1  2  7  5 10  9   ONEBIT
X2   3  4 10  6  8  9   ONEBIT
.ENDS TWOBIT
.SUBCKT FOURBIT 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
X1   1  2  3  4  9 10 13 16 15   TWOBIT
X2   5  6  7  8 11 12 16 14 15   TWOBIT
.ENDS FOURBIT
VCC   99  0   DC 3.3V
VIN1A  1  0   DC 0
VIN1B  2  0   DC 0
VIN2A  3  0   DC 0
VIN2B  4  0   DC 0
VIN3A  5  0   DC 0
VIN3B  6  0   DC 0
VIN4A  7  0   DC 0
VIN4B  8  0   DC 0
X1     1  2  3  4  5  6  7  8  9 10 11 12  0 13 99 FOURBIT
.model n1 nmos level=49 version=3.3.0
.model p1 pmos level=49 version=3.3.0
.op
.end
";

fn operating_point(damping: DampingStrategy) -> Result<usize, String> {
    let netlist = Netlist::parse(FOUR_BIT_ADDER).expect("deck parses");
    let defaults = SimulationConfig::default();
    let engine = Engine::new(SimulationConfig {
        max_iterations: defaults.max_iterations.max(1200),
        convergence_config: ConvergenceConfig {
            damping_strategy: damping,
            ..ConvergenceConfig::default()
        },
        ..defaults
    });
    engine
        .run_dc_op(&netlist)
        .map(|result| result.node_voltages.len())
        .map_err(|error| error.to_string())
}

/// Every damping policy has to reach the same operating point on ordinary
/// CMOS logic. The device's own limiting is what carries turn-on; the engine
/// policy must be inert around it, so none of the five may stall.
#[test]
fn bsim3_adder_operating_point_converges_under_every_damping_policy() {
    for damping in [
        DampingStrategy::None,
        DampingStrategy::VoltageLimiting,
        DampingStrategy::LineSearch,
        DampingStrategy::BankRose,
        DampingStrategy::Combined,
    ] {
        match operating_point(damping) {
            Ok(nodes) => assert!(nodes > 0, "{damping:?}: no nodes solved"),
            Err(error) => {
                panic!("{damping:?}: the BSIM3 adder operating point did not converge: {error}")
            }
        }
    }
}
