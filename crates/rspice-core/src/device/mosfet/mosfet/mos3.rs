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
