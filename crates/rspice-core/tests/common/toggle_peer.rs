//! The toggle peer of the ngspice `d_process` protocol, run in process, and
//! the ngspice-46 events its deck must reproduce. Shared by the XSPICE suite,
//! which runs the deck alone, and the Verilog-A suite, which runs it beside a
//! Verilog module.

use rspice_core::engine::TransientResult;
use rspice_core::xspice::{
    CmResult, DigitalProcessRuntime, DigitalProcessRuntimeFactory, DigitalProcessSpec,
    set_digital_process_runtime_factory,
};
use std::sync::{Arc, Mutex, MutexGuard};

/// Every exchange a `d_process` peer saw: the signed time and the packed
/// input bytes.
pub type ProcessExchangeLog = Arc<Mutex<Vec<(f64, Vec<u8>)>>>;

/// The toggle peer of the ngspice process protocol, in process: it answers
/// every clocked exchange with a bit that flips on each call, as the C peer
/// ngspice-46 ran against does, and logs the exchange.
struct TogglePeer {
    q: u8,
    exchanges: ProcessExchangeLog,
}

impl DigitalProcessRuntime for TogglePeer {
    fn exchange(
        &mut self,
        signed_time: f64,
        input_bytes: &[u8],
        output_bytes: &mut [u8],
    ) -> CmResult<()> {
        self.exchanges
            .lock()
            .expect("lock the exchange log")
            .push((signed_time, input_bytes.to_vec()));
        self.q ^= 1;
        output_bytes.fill(0);
        output_bytes[0] = self.q;
        Ok(())
    }
}

struct TogglePeerFactory {
    exchanges: ProcessExchangeLog,
}

impl DigitalProcessRuntimeFactory for TogglePeerFactory {
    fn start(&self, spec: &DigitalProcessSpec) -> CmResult<Box<dyn DigitalProcessRuntime>> {
        assert_eq!((spec.input_count, spec.output_count), (1, 1));
        Ok(Box::new(TogglePeer {
            q: 0,
            exchanges: Arc::clone(&self.exchanges),
        }))
    }
}

static PROCESS_FACTORY_LOCK: Mutex<()> = Mutex::new(());

/// Installs a process provider for one test and puts back the one it
/// replaced. The provider is global, so tests that install one run one at a
/// time.
pub struct ProcessFactoryGuard {
    previous: Option<Arc<dyn DigitalProcessRuntimeFactory>>,
    _serial: MutexGuard<'static, ()>,
}

impl ProcessFactoryGuard {
    pub fn toggle_peer(exchanges: &ProcessExchangeLog) -> Self {
        let serial = PROCESS_FACTORY_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let previous = set_digital_process_runtime_factory(Some(Arc::new(TogglePeerFactory {
            exchanges: Arc::clone(exchanges),
        })));
        Self {
            previous,
            _serial: serial,
        }
    }
}

impl Drop for ProcessFactoryGuard {
    fn drop(&mut self) {
        set_digital_process_runtime_factory(self.previous.take());
    }
}

/// ngspice-46 `eprint d` for [`TOGGLE_PEER_DECK`] with the C toggle peer:
/// unknown at the operating point, then a toggle one clk_delay after each
/// rising edge of the 1 MHz clock.
const TOGGLE_PEER_NGSPICE_D: [(f64, &str); 21] = [
    (0.0, "Uz"),
    (5.01e-7, "1s"),
    (1.501e-6, "0s"),
    (2.501e-6, "1s"),
    (3.501e-6, "0s"),
    (4.501e-6, "1s"),
    (5.501e-6, "0s"),
    (6.501e-6, "1s"),
    (7.501e-6, "0s"),
    (8.501e-6, "1s"),
    (9.501e-6, "0s"),
    (1.0501e-5, "1s"),
    (1.1501e-5, "0s"),
    (1.2501e-5, "1s"),
    (1.3501e-5, "0s"),
    (1.4501e-5, "1s"),
    (1.5501e-5, "0s"),
    (1.6501e-5, "1s"),
    (1.7501e-5, "0s"),
    (1.8501e-5, "1s"),
    (1.9501e-5, "0s"),
];

/// A 1 MHz `d_osc` clocks a `d_process` toggle peer whose output drives an
/// RC load through a `dac_bridge`; the run is 20 us.
pub const TOGGLE_PEER_DECK: &str = "\
vc cntl 0 0.5
aosc cntl dosc osc1
.model osc1 d_osc(cntl_array=[0 1] freq_array=[1e6 1e6] duty_cycle=0.5 init_phase=0 rise_delay=1e-10 fall_delay=1e-10)
aproc [dosc] dosc null [d] proc1
.model proc1 d_process(process_file=\"toggle\" clk_delay=1e-9 reset_delay=1e-9)
abr [d] [dd] dac1
.model dac1 dac_bridge(out_low=0 out_high=1 t_rise=1n t_fall=1n)
r1 dd out 1k
c1 out 0 1p
.tran 1n 20u 0 400n
";

/// `d` against [`TOGGLE_PEER_NGSPICE_D`] to 1 fs, and the peer asked exactly
/// once per rising edge, with the clock high, at the edge.
pub fn assert_toggle_peer_matches_ngspice(
    result: &TransientResult,
    exchanges: &ProcessExchangeLog,
    expected_inputs: impl Fn(usize) -> Vec<u8>,
) {
    let d: Vec<(f64, String)> = result
        .digital_trace_named("d")
        .unwrap_or_else(|| panic!("digital trace d missing from {:?}", result.node_names))
        .iter()
        .map(|point| (point.time, point.value.to_ngspice_token()))
        .collect();
    assert_eq!(
        d.len(),
        TOGGLE_PEER_NGSPICE_D.len(),
        "d events against ngspice-46: {d:?}"
    );
    for ((time, token), (ngspice_time, ngspice_token)) in d.iter().zip(TOGGLE_PEER_NGSPICE_D) {
        assert!(
            (time - ngspice_time).abs() <= 1.0e-15 && token == ngspice_token,
            "d = {token} at {time:e}, ngspice-46 has {ngspice_token} at {ngspice_time:e}: {d:?}"
        );
    }

    let exchanges = exchanges.lock().expect("lock the exchange log");
    assert_eq!(
        exchanges.len(),
        20,
        "one exchange per rising edge: {exchanges:?}"
    );
    for (edge, (signed_time, input_bytes)) in exchanges.iter().enumerate() {
        let edge_time = (edge as f64 + 0.5) * 1.0e-6;
        assert!(
            (signed_time - edge_time).abs() <= 1.0e-15 && *input_bytes == expected_inputs(edge),
            "exchange {edge} at {signed_time:e} with {input_bytes:?}, expected {:?} at \
             {edge_time:e}",
            expected_inputs(edge)
        );
    }
}
