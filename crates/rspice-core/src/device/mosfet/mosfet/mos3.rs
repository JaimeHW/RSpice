use super::{MosRegion, Mosfet};
use crate::Value;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Mos3State {
    pub ids: f64,
    pub gm: f64,
    pub gds: f64,
    pub gmb: f64,
    pub von: f64,
    pub vdsat: f64,
    pub qgs: f64,
    pub qgd: f64,
    pub qgb: f64,
    pub cgs: f64,
    pub cgd: f64,
    pub cgb: f64,
}

impl Mosfet {
    pub(in crate::device::mosfet::mosfet) fn calculate_id_mos3(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion) {
        // Temporary Task 3 routing shim: preserve the previous numeric
        // approximation until Task 4 replaces this with native MOS3 equations.
        let (id_forward, region_forward) = self.calculate_id_bsim3(vgs, vds, vbs);
        let (vgs_rev, vds_rev, vbs_rev) = Self::reverse_voltages(vgs, vds, vbs);
        let (id_reverse_fwd, region_reverse) =
            self.calculate_id_bsim3(vgs_rev, vds_rev, vbs_rev);
        let id = id_forward - id_reverse_fwd;
        let region = if id_forward.abs() >= id_reverse_fwd.abs() {
            region_forward
        } else {
            region_reverse
        };

        (id, region)
    }
}
