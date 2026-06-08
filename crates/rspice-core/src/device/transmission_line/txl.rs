use super::*;

const TXL_SETUP_EPSI: Value = 1.0e-10;
const TXL_EXP_SETUP_EPSI: Value = 1.0e-10;
const TXL_ROOT_TOL: Value = 5.0e-4;

#[derive(Debug, Clone, Copy)]
struct Term {
    c: Value,
    x: Value,
    cnv_i: Value,
    cnv_o: Value,
}

impl Term {
    fn new(c: Value, x: Value) -> Self {
        Self {
            c,
            x,
            cnv_i: 0.0,
            cnv_o: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TxlHistorySample {
    time: Value,
    v_i: Value,
    v_o: Value,
    i_i: Value,
    i_o: Value,
}

#[derive(Debug, Clone, Copy)]
struct PreviousVi {
    v1_i: Value,
    v2_i: Value,
    i1_i: Value,
    i2_i: Value,
    v1_o: Value,
    v2_o: Value,
    i1_o: Value,
    i2_o: Value,
    ext: bool,
    ratio: Value,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TxlTransientStamp {
    pub(crate) local_coeff: Value,
    pub(crate) delayed_voltage_coeff: Value,
    pub(crate) delayed_current_coeff: Value,
    pub(crate) rhs1: Value,
    pub(crate) rhs2: Value,
}

#[derive(Debug, Clone)]
pub(in crate::device::transmission_line) struct TxlRuntime {
    sqt_cd_l: Value,
    taul: Value,
    h2_aten: Value,
    h3_aten: Value,
    h1c: Value,
    h1e: [Value; 3],
    if_img: bool,
    ext: bool,
    ratio: Value,
    h1_term: [Term; 3],
    h2_term: [Term; 3],
    h3_term: [Term; 6],
    dc1: Value,
    dc2: Value,
    history: VecDeque<TxlHistorySample>,
}

impl TxlRuntime {
    pub(in crate::device::transmission_line) fn setup(
        r: Value,
        l: Value,
        g: Value,
        c: Value,
        len: Value,
    ) -> Option<Self> {
        if !r.is_finite()
            || !l.is_finite()
            || !g.is_finite()
            || !c.is_finite()
            || !len.is_finite()
            || r <= 0.0
            || l <= 0.0
            || g < 0.0
            || c <= 0.0
            || len <= 0.0
        {
            return None;
        }

        let mut runtime = Self {
            sqt_cd_l: 0.0,
            taul: 0.0,
            h2_aten: 0.0,
            h3_aten: 0.0,
            h1c: 0.0,
            h1e: [1.0; 3],
            if_img: false,
            ext: false,
            ratio: 0.0,
            h1_term: [Term::new(0.0, 0.0); 3],
            h2_term: [Term::new(0.0, 0.0); 3],
            h3_term: [Term::new(0.0, 0.0); 6],
            dc1: 0.0,
            dc2: 0.0,
            history: VecDeque::new(),
        };

        runtime.y_pade(r, l, g, c)?;
        runtime.if_img = runtime.exp_pade(r, l, g, c, len)?;
        runtime.get_h3()?;
        runtime.update_h1c_c();
        Some(runtime)
    }

    pub(in crate::device::transmission_line) fn reset(&mut self) {
        self.ext = false;
        self.ratio = 0.0;
        for term in &mut self.h1_term {
            term.cnv_i = 0.0;
            term.cnv_o = 0.0;
        }
        for term in &mut self.h2_term {
            term.cnv_i = 0.0;
            term.cnv_o = 0.0;
        }
        for term in &mut self.h3_term {
            term.cnv_i = 0.0;
            term.cnv_o = 0.0;
        }
        self.history.clear();
    }

    pub(in crate::device::transmission_line) fn initialize(
        &mut self,
        time: Value,
        v1: Value,
        i1: Value,
        v2: Value,
        i2: Value,
    ) {
        self.reset();
        self.dc1 = v1;
        self.dc2 = v2;
        for term in &mut self.h1_term {
            term.cnv_i = -self.dc1 * term.c / term.x;
            term.cnv_o = -self.dc2 * term.c / term.x;
        }
        for term in &mut self.h3_term {
            term.cnv_i = -self.dc1 * term.c / term.x;
            term.cnv_o = -self.dc2 * term.c / term.x;
        }
        self.history.push_back(TxlHistorySample {
            time,
            v_i: v1,
            v_o: v2,
            i_i: i1,
            i_o: i2,
        });
    }

    pub(in crate::device::transmission_line) fn transient_stamp(
        &self,
        time: Value,
    ) -> Option<TxlTransientStamp> {
        let last = self.history.back()?;
        let h = time - last.time;
        if !(h.is_finite() && h > 0.0) {
            return Some(TxlTransientStamp {
                local_coeff: self.sqt_cd_l,
                delayed_voltage_coeff: 0.0,
                delayed_current_coeff: 0.0,
                rhs1: 0.0,
                rhs2: 0.0,
            });
        }

        let mut trial = self.clone();
        let (rhs1, rhs2, ext, ratio) = trial.right_consts(last.time, time, h)?;
        let h1 = 0.5 * h;
        let local_coeff = trial.sqt_cd_l + h1 * trial.h1c;
        let mut delayed_voltage_coeff = 0.0;
        let mut delayed_current_coeff = 0.0;
        if ext && ratio > 0.0 {
            delayed_voltage_coeff = ratio
                * (h1 * trial.h3_term.iter().map(|term| term.c).sum::<Value>() + trial.h3_aten);
            delayed_current_coeff = ratio
                * (h1 * trial.h2_term.iter().map(|term| term.c).sum::<Value>() + trial.h2_aten);
        }

        Some(TxlTransientStamp {
            local_coeff,
            delayed_voltage_coeff,
            delayed_current_coeff,
            rhs1,
            rhs2,
        })
    }

    pub(in crate::device::transmission_line) fn accept(
        &mut self,
        time: Value,
        v1: Value,
        i1: Value,
        v2: Value,
        i2: Value,
    ) {
        let Some(prev) = self.history.back().copied() else {
            self.initialize(time, v1, i1, v2, i2);
            return;
        };
        let h = time - prev.time;
        if !(h.is_finite() && h > 0.0) {
            return;
        }

        let mut accepted = self.clone();
        if let Some((_rhs1, _rhs2, ext, ratio)) = accepted.right_consts(prev.time, time, h) {
            accepted.ext = ext;
            accepted.ratio = ratio;
            accepted.history.push_back(TxlHistorySample {
                time,
                v_i: v1,
                v_o: v2,
                i_i: i1,
                i_o: i2,
            });
            accepted.update_cnv(prev, v1, v2, h);
            if accepted.ext {
                accepted.update_delayed_cnv(h);
            }
            accepted.compact_history(time);
            *self = accepted;
        }
    }

    fn y_pade(&mut self, r: Value, l: Value, g: Value, c: Value) -> Option<()> {
        let sqt_cd_l = (c / l).sqrt();
        let rd_l = r / l;
        let gd_c = g / c;
        if !(rd_l.is_finite() && rd_l > 0.0 && gd_c.is_finite()) {
            return None;
        }

        let (b1, b2, b3, b4, b5) = mac(gd_c, rd_l)?;
        let mut a = [
            [1.0 - (gd_c / rd_l).sqrt(), b1, b2, -b3],
            [b1, b2, b3, -b4],
            [b2, b3, b4, -b5],
        ];
        gaussian_elimination(&mut a, TXL_SETUP_EPSI)?;

        let p3 = a[0][3];
        let p2 = a[1][3];
        let p1 = a[2][3];
        let q1 = p1 + b1;
        let q2 = b1 * p1 + p2 + b2;
        let q3 = p3 * (gd_c / rd_l).sqrt();
        let (x1, x2, x3, roots_are_real) = find_roots(p1, p2, p3)?;
        if !roots_are_real {
            return None;
        }

        let c1 = eval2(q1 - p1, q2 - p2, q3 - p3, x1) / eval2(3.0, 2.0 * p1, p2, x1);
        let c2 = eval2(q1 - p1, q2 - p2, q3 - p3, x2) / eval2(3.0, 2.0 * p1, p2, x2);
        let c3 = eval2(q1 - p1, q2 - p2, q3 - p3, x3) / eval2(3.0, 2.0 * p1, p2, x3);

        if ![sqt_cd_l, c1, c2, c3, x1, x2, x3]
            .iter()
            .all(|v| v.is_finite())
        {
            return None;
        }

        self.sqt_cd_l = sqt_cd_l;
        self.h1_term = [Term::new(c1, x1), Term::new(c2, x2), Term::new(c3, x3)];
        Some(())
    }

    fn exp_pade(&mut self, r: Value, l: Value, g: Value, c: Value, len: Value) -> Option<bool> {
        let tau = (l * c).sqrt();
        let rd_l = r / l;
        let gd_c = g / c;
        let rg = r * g;
        if !(tau.is_finite() && tau > 0.0 && rd_l.is_finite() && gd_c.is_finite()) {
            return None;
        }

        let y1 = 0.5 * (rd_l + gd_c);
        let y2 = rd_l * gd_c - y1 * y1;
        let y3 = -3.0 * y1 * y2;
        let y4 = -3.0 * y2 * y2 - 4.0 * y1 * y3;
        let y5 = -5.0 * y1 * y4 - 10.0 * y2 * y3;
        let y6 = -10.0 * y3 * y3 - 15.0 * y2 * y4 - 6.0 * y1 * y5;

        let a0 = y1 * tau * len;
        let a1 = y2 * tau.powi(2) / 2.0 * len;
        let a2 = y3 * tau.powi(3) / 6.0 * len;
        let a3 = y4 * tau.powi(4) / 24.0 * len;
        let a4 = y5 * tau.powi(5) / 120.0 * len;
        let a5 = y6 * tau.powi(6) / 720.0 * len;

        let a = [0.0, -a1, -a2, -a3, -a4, -a5];
        let mut b = [0.0; 6];
        b[0] = 1.0;
        b[1] = a[1];
        for i in 2..=5 {
            let mut sum = 0.0;
            for j in 1..=i {
                sum += j as Value * a[j] * b[i - j];
            }
            b[i] = sum / i as Value;
        }

        let mut aa = [
            [1.0 - (a0 - len * rg.sqrt()).exp(), b[1], b[2], -b[3]],
            [b[1], b[2], b[3], -b[4]],
            [b[2], b[3], b[4], -b[5]],
        ];
        gaussian_elimination(&mut aa, TXL_EXP_SETUP_EPSI)?;

        let mut ep3 = aa[0][3];
        let mut ep2 = aa[1][3];
        let mut ep1 = aa[2][3];
        let mut eq1 = ep1 + b[1];
        let mut eq2 = b[1] * ep1 + ep2 + b[2];
        let mut eq3 = ep3 * (a0 - len * rg.sqrt()).exp();

        ep3 /= tau.powi(3);
        ep2 /= tau.powi(2);
        ep1 /= tau;
        eq3 /= tau.powi(3);
        eq2 /= tau.powi(2);
        eq1 /= tau;

        let (ex1, ex2, ex3, roots_are_real) = find_roots(ep1, ep2, ep3)?;
        let ec1 = eval2(eq1 - ep1, eq2 - ep2, eq3 - ep3, ex1) / eval2(3.0, 2.0 * ep1, ep2, ex1);
        let (ec2, ec3) = if roots_are_real {
            (
                eval2(eq1 - ep1, eq2 - ep2, eq3 - ep3, ex2) / eval2(3.0, 2.0 * ep1, ep2, ex2),
                eval2(eq1 - ep1, eq2 - ep2, eq3 - ep3, ex3) / eval2(3.0, 2.0 * ep1, ep2, ex3),
            )
        } else {
            get_c(eq1 - ep1, eq2 - ep2, eq3 - ep3, ep1, ep2, ex2, ex3)
        };

        self.taul = tau * len;
        self.h2_aten = (-a0).exp();
        self.h2_term = [
            Term::new(ec1, ex1),
            Term::new(ec2, ex2),
            Term::new(ec3, ex3),
        ];

        if ![self.taul, self.h2_aten, ec1, ec2, ec3, ex1, ex2, ex3]
            .iter()
            .all(|v| v.is_finite())
        {
            return None;
        }

        Some(!roots_are_real)
    }

    fn get_h3(&mut self) -> Option<()> {
        self.h3_aten = self.h2_aten * self.sqt_cd_l;
        for i in 0..3 {
            self.h3_term[i].x = self.h1_term[i].x;
        }
        for i in 0..3 {
            self.h3_term[i + 3].x = self.h2_term[i].x;
        }

        let cc1 = self.h1_term[0].c;
        let cc2 = self.h1_term[1].c;
        let cc3 = self.h1_term[2].c;
        let cc4 = self.h2_term[0].c;
        let cc5 = self.h2_term[1].c;
        let cc6 = self.h2_term[2].c;
        let xx1 = self.h1_term[0].x;
        let xx2 = self.h1_term[1].x;
        let xx3 = self.h1_term[2].x;
        let xx4 = self.h2_term[0].x;
        let xx5 = self.h2_term[1].x;
        let xx6 = self.h2_term[2].x;

        if self.if_img {
            self.h3_term[0].c = cc1
                + cc1
                    * (cc4 / (xx1 - xx4)
                        + 2.0 * (cc5 * xx1 - xx6 * cc6 - xx5 * cc5)
                            / (xx1 * xx1 - 2.0 * xx5 * xx1 + xx5 * xx5 + xx6 * xx6));
            self.h3_term[1].c = cc2
                + cc2
                    * (cc4 / (xx2 - xx4)
                        + 2.0 * (cc5 * xx2 - xx6 * cc6 - xx5 * cc5)
                            / (xx2 * xx2 - 2.0 * xx5 * xx2 + xx5 * xx5 + xx6 * xx6));
            self.h3_term[2].c = cc3
                + cc3
                    * (cc4 / (xx3 - xx4)
                        + 2.0 * (cc5 * xx3 - xx6 * cc6 - xx5 * cc5)
                            / (xx3 * xx3 - 2.0 * xx5 * xx3 + xx5 * xx5 + xx6 * xx6));
            self.h3_term[3].c =
                cc4 + cc4 * (cc1 / (xx4 - xx1) + cc2 / (xx4 - xx2) + cc3 / (xx4 - xx3));
            self.h3_term[4].c = cc5;
            self.h3_term[5].c = cc6;
            for (cc, xx) in [(cc1, xx1), (cc2, xx2), (cc3, xx3)] {
                let (r, i) = div_c(cc5, cc6, xx5 - xx, xx6)?;
                self.h3_term[4].c += r * cc;
                self.h3_term[5].c += i * cc;
            }
        } else {
            self.h3_term[0].c =
                cc1 + cc1 * (cc4 / (xx1 - xx4) + cc5 / (xx1 - xx5) + cc6 / (xx1 - xx6));
            self.h3_term[1].c =
                cc2 + cc2 * (cc4 / (xx2 - xx4) + cc5 / (xx2 - xx5) + cc6 / (xx2 - xx6));
            self.h3_term[2].c =
                cc3 + cc3 * (cc4 / (xx3 - xx4) + cc5 / (xx3 - xx5) + cc6 / (xx3 - xx6));
            self.h3_term[3].c =
                cc4 + cc4 * (cc1 / (xx4 - xx1) + cc2 / (xx4 - xx2) + cc3 / (xx4 - xx3));
            self.h3_term[4].c =
                cc5 + cc5 * (cc1 / (xx5 - xx1) + cc2 / (xx5 - xx2) + cc3 / (xx5 - xx3));
            self.h3_term[5].c =
                cc6 + cc6 * (cc1 / (xx6 - xx1) + cc2 / (xx6 - xx2) + cc3 / (xx6 - xx3));
        }

        if self.h3_term.iter().all(|term| term.c.is_finite()) {
            Some(())
        } else {
            None
        }
    }

    fn update_h1c_c(&mut self) {
        self.h1c = 0.0;
        for term in &mut self.h1_term {
            term.c *= self.sqt_cd_l;
            self.h1c += term.c;
        }
        for term in &mut self.h2_term {
            term.c *= self.h2_aten;
        }
        for term in &mut self.h3_term {
            term.c *= self.h3_aten;
        }
    }

    fn right_consts(
        &mut self,
        previous_time: Value,
        candidate_time: Value,
        h: Value,
    ) -> Option<(Value, Value, bool, Value)> {
        let h1 = 0.5 * h;
        let local = self.history.back().copied()?;
        let mut ff = 0.0;
        let mut gg = 0.0;
        let mut ff1 = 0.0;
        for i in 0..3 {
            let e = (self.h1_term[i].x * h).exp();
            self.h1e[i] = e;
            ff1 -= self.h1_term[i].c * e;
            ff -= self.h1_term[i].cnv_i * e;
            gg -= self.h1_term[i].cnv_o * e;
        }
        ff += ff1 * h1 * local.v_i;
        gg += ff1 * h1 * local.v_o;

        let previous = self.previous_vi(previous_time, candidate_time)?;
        if self.if_img {
            self.update_right_consts_complex(previous, h, h1, &mut ff, &mut gg)?;
        } else {
            for term in &mut self.h3_term {
                let e = (term.x * h).exp();
                term.cnv_i = term.cnv_i * e + h1 * term.c * (previous.v1_i * e + previous.v2_i);
                term.cnv_o = term.cnv_o * e + h1 * term.c * (previous.v1_o * e + previous.v2_o);
            }
            ff += self.h3_aten * previous.v2_o;
            gg += self.h3_aten * previous.v2_i;
            for term in &self.h3_term {
                ff += term.cnv_o;
                gg += term.cnv_i;
            }

            for term in &mut self.h2_term {
                let e = (term.x * h).exp();
                term.cnv_i = term.cnv_i * e + h1 * term.c * (previous.i1_i * e + previous.i2_i);
                term.cnv_o = term.cnv_o * e + h1 * term.c * (previous.i1_o * e + previous.i2_o);
            }
            ff += self.h2_aten * previous.i2_o;
            gg += self.h2_aten * previous.i2_i;
            for term in &self.h2_term {
                ff += term.cnv_o;
                gg += term.cnv_i;
            }
        }

        Some((ff, gg, previous.ext, previous.ratio))
    }

    fn update_right_consts_complex(
        &mut self,
        previous: PreviousVi,
        h: Value,
        h1: Value,
        ff: &mut Value,
        gg: &mut Value,
    ) -> Option<()> {
        for i in 0..4 {
            let term = &mut self.h3_term[i];
            let e = (term.x * h).exp();
            term.cnv_i = term.cnv_i * e + h1 * term.c * (previous.v1_i * e + previous.v2_i);
            term.cnv_o = term.cnv_o * e + h1 * term.c * (previous.v1_o * e + previous.v2_o);
        }
        let (er, ei) = exp_c(self.h3_term[4].x, self.h3_term[5].x, h);
        let a2 = h1 * self.h3_term[4].c;
        let b2 = h1 * self.h3_term[5].c;
        let (a, b) = mult_c(self.h3_term[4].cnv_i, self.h3_term[5].cnv_i, er, ei);
        let (a1, b1) = mult_c(
            a2,
            b2,
            previous.v1_i * er + previous.v2_i,
            previous.v1_i * ei,
        );
        self.h3_term[4].cnv_i = a + a1;
        self.h3_term[5].cnv_i = b + b1;
        let (a, b) = mult_c(self.h3_term[4].cnv_o, self.h3_term[5].cnv_o, er, ei);
        let (a1, b1) = mult_c(
            a2,
            b2,
            previous.v1_o * er + previous.v2_o,
            previous.v1_o * ei,
        );
        self.h3_term[4].cnv_o = a + a1;
        self.h3_term[5].cnv_o = b + b1;

        *ff += self.h3_aten * previous.v2_o;
        *gg += self.h3_aten * previous.v2_i;
        for i in 0..5 {
            *ff += self.h3_term[i].cnv_o;
            *gg += self.h3_term[i].cnv_i;
        }
        *ff += self.h3_term[4].cnv_o;
        *gg += self.h3_term[4].cnv_i;

        let e = (self.h2_term[0].x * h).exp();
        self.h2_term[0].cnv_i = self.h2_term[0].cnv_i * e
            + h1 * self.h2_term[0].c * (previous.i1_i * e + previous.i2_i);
        self.h2_term[0].cnv_o = self.h2_term[0].cnv_o * e
            + h1 * self.h2_term[0].c * (previous.i1_o * e + previous.i2_o);
        let (er, ei) = exp_c(self.h2_term[1].x, self.h2_term[2].x, h);
        let a2 = h1 * self.h2_term[1].c;
        let b2 = h1 * self.h2_term[2].c;
        let (a, b) = mult_c(self.h2_term[1].cnv_i, self.h2_term[2].cnv_i, er, ei);
        let (a1, b1) = mult_c(
            a2,
            b2,
            previous.i1_i * er + previous.i2_i,
            previous.i1_i * ei,
        );
        self.h2_term[1].cnv_i = a + a1;
        self.h2_term[2].cnv_i = b + b1;
        let (a, b) = mult_c(self.h2_term[1].cnv_o, self.h2_term[2].cnv_o, er, ei);
        let (a1, b1) = mult_c(
            a2,
            b2,
            previous.i1_o * er + previous.i2_o,
            previous.i1_o * ei,
        );
        self.h2_term[1].cnv_o = a + a1;
        self.h2_term[2].cnv_o = b + b1;

        *ff += self.h2_aten * previous.i2_o + self.h2_term[0].cnv_o + 2.0 * self.h2_term[1].cnv_o;
        *gg += self.h2_aten * previous.i2_i + self.h2_term[0].cnv_i + 2.0 * self.h2_term[1].cnv_i;
        Some(())
    }

    fn previous_vi(&mut self, previous_time: Value, candidate_time: Value) -> Option<PreviousVi> {
        let ta = previous_time - self.taul;
        let tb = candidate_time - self.taul;
        if tb <= 0.0 {
            return Some(PreviousVi {
                v1_i: self.dc1,
                v2_i: self.dc1,
                i1_i: 0.0,
                i2_i: 0.0,
                v1_o: self.dc2,
                v2_o: self.dc2,
                i1_o: 0.0,
                i2_o: 0.0,
                ext: false,
                ratio: 0.0,
            });
        }

        let (v1_i, v1_o, i1_i, i1_o) = if ta <= 0.0 {
            (self.dc1, self.dc2, 0.0, 0.0)
        } else {
            let sample = self.interpolate(ta)?;
            (sample.v_i, sample.v_o, sample.i_i, sample.i_o)
        };

        if tb > previous_time {
            let denom = candidate_time - previous_time;
            if !(denom.is_finite() && denom > 0.0) {
                return None;
            }
            let ratio = (tb - previous_time) / denom;
            let f = 1.0 - ratio;
            let tail = self.history.back().copied()?;
            return Some(PreviousVi {
                v1_i,
                v1_o,
                i1_i,
                i1_o,
                v2_i: tail.v_i * f,
                v2_o: tail.v_o * f,
                i2_i: tail.i_i * f,
                i2_o: tail.i_o * f,
                ext: true,
                ratio,
            });
        }

        let sample = self.interpolate(tb)?;
        Some(PreviousVi {
            v1_i,
            v1_o,
            i1_i,
            i1_o,
            v2_i: sample.v_i,
            v2_o: sample.v_o,
            i2_i: sample.i_i,
            i2_o: sample.i_o,
            ext: false,
            ratio: 0.0,
        })
    }

    fn interpolate(&mut self, time: Value) -> Option<TxlHistorySample> {
        if time <= 0.0 {
            return Some(TxlHistorySample {
                time,
                v_i: self.dc1,
                v_o: self.dc2,
                i_i: 0.0,
                i_o: 0.0,
            });
        }
        let first = self.history.front().copied()?;
        if time <= first.time {
            return Some(first);
        }
        let mut prev = first;
        let mut keep_index = 0;
        for (idx, sample) in self.history.iter().copied().enumerate().skip(1) {
            if sample.time >= time {
                if sample.time <= prev.time {
                    return Some(sample);
                }
                let f = (time - prev.time) / (sample.time - prev.time);
                if keep_index > 0 {
                    for _ in 0..keep_index {
                        self.history.pop_front();
                    }
                }
                return Some(TxlHistorySample {
                    time,
                    v_i: prev.v_i + f * (sample.v_i - prev.v_i),
                    v_o: prev.v_o + f * (sample.v_o - prev.v_o),
                    i_i: prev.i_i + f * (sample.i_i - prev.i_i),
                    i_o: prev.i_o + f * (sample.i_o - prev.i_o),
                });
            }
            prev = sample;
            keep_index = idx.saturating_sub(1);
        }
        self.history.back().copied()
    }

    fn update_cnv(&mut self, prev: TxlHistorySample, v1: Value, v2: Value, h: Value) {
        let mut scaled_slope_i = (v1 - prev.v_i) / h;
        let mut scaled_slope_o = (v2 - prev.v_o) / h;
        for i in 0..3 {
            let e = self.h1e[i];
            let term = &mut self.h1_term[i];
            let t = term.c / term.x;
            scaled_slope_i *= t;
            scaled_slope_o *= t;
            term.cnv_i = (term.cnv_i - scaled_slope_i * h) * e
                + (e - 1.0) * (v1 * t + scaled_slope_i / term.x);
            term.cnv_o = (term.cnv_o - scaled_slope_o * h) * e
                + (e - 1.0) * (v2 * t + scaled_slope_o / term.x);
        }
    }

    fn update_delayed_cnv(&mut self, h: Value) {
        if self.ratio <= 0.0 {
            return;
        }
        let Some(vi) = self.history.back().copied() else {
            return;
        };
        let h_half = 0.5 * h;
        let f = h_half * self.ratio * vi.v_i;
        for term in &mut self.h3_term {
            term.cnv_i += f * term.c;
        }
        let f = h_half * self.ratio * vi.v_o;
        for term in &mut self.h3_term {
            term.cnv_o += f * term.c;
        }
        let f = h_half * self.ratio * vi.i_i;
        for term in &mut self.h2_term {
            term.cnv_i += f * term.c;
        }
        let f = h_half * self.ratio * vi.i_o;
        for term in &mut self.h2_term {
            term.cnv_o += f * term.c;
        }
    }

    fn compact_history(&mut self, time: Value) {
        let horizon = self.taul * 1.25 + 1.0e-18;
        while self.history.len() > 2 {
            let Some(front) = self.history.front() else {
                break;
            };
            if time - front.time > horizon {
                self.history.pop_front();
            } else {
                break;
            }
        }
    }
}

fn eval2(a: Value, b: Value, c: Value, x: Value) -> Value {
    a * x * x + b * x + c
}

fn gaussian_elimination(a: &mut [[Value; 4]; 3], eps: Value) -> Option<()> {
    for i in 0..3 {
        let mut imax = i;
        let mut max = a[i][i].abs();
        for (j, row) in a.iter().enumerate().skip(i + 1) {
            if row[i].abs() > max {
                imax = j;
                max = row[i].abs();
            }
        }
        if max < eps {
            return None;
        }
        if imax != i {
            a.swap(i, imax);
        }

        let f = 1.0 / a[i][i];
        a[i][i] = 1.0;
        for j in i + 1..=3 {
            a[i][j] *= f;
        }

        for j in 0..3 {
            if i == j {
                continue;
            }
            let f = a[j][i];
            a[j][i] = 0.0;
            for k in i + 1..=3 {
                a[j][k] -= f * a[i][k];
            }
        }
    }
    Some(())
}

fn root3(a1: Value, a2: Value, a3: Value, x: Value) -> Value {
    let t1 = x * x * x + a1 * x * x + a2 * x + a3;
    let t2 = 3.0 * x * x + 2.0 * a1 * x + a2;
    x - t1 / t2
}

fn div3(a1: Value, _a2: Value, a3: Value, x: Value) -> Option<(Value, Value)> {
    if x == 0.0 {
        return None;
    }
    Some((a1 + x, -a3 / x))
}

fn find_roots(a1: Value, a2: Value, a3: Value) -> Option<(Value, Value, Value, bool)> {
    let q = (a1 * a1 - 3.0 * a2) / 9.0;
    let p = (2.0 * a1.powi(3) - 9.0 * a1 * a2 + 27.0 * a3) / 54.0;
    let t = q.powi(3) - p * p;
    let mut x = if t >= 0.0 {
        if q <= 0.0 {
            return None;
        }
        let arg = (p / (q * q.sqrt())).clamp(-1.0, 1.0);
        -2.0 * q.sqrt() * (arg.acos() / 3.0).cos() - a1 / 3.0
    } else if p > 0.0 {
        let cube = ((-t).sqrt() + p).cbrt();
        -(cube + q / cube) - a1 / 3.0
    } else if p == 0.0 {
        -a1 / 3.0
    } else {
        let cube = ((-t).sqrt() - p).cbrt();
        cube + q / cube - a1 / 3.0
    };

    let backup = x;
    for i in 0..32 {
        let next = root3(a1, a2, a3, x);
        if !next.is_finite() {
            x = backup;
            break;
        }
        if (next - x).abs() <= TXL_ROOT_TOL {
            x = next;
            break;
        }
        if i == 31 {
            x = backup;
        } else {
            x = next;
        }
    }

    let (quad_a1, quad_a2) = div3(a1, a2, a3, x)?;
    let discr = quad_a1 * quad_a1 - 4.0 * quad_a2;
    if discr < 0.0 {
        return Some((x, -0.5 * quad_a1, 0.5 * (-discr).sqrt(), false));
    }

    let discr_root = (discr * 1.0e-18).sqrt() * 1.0e9;
    let x2 = if quad_a1 >= 0.0 {
        -0.5 * (quad_a1 + discr_root)
    } else {
        -0.5 * (quad_a1 - discr_root)
    };
    if x2 == 0.0 {
        return None;
    }
    let x3 = quad_a2 / x2;
    Some((x, x2, x3, true))
}

fn mac(at: Value, bt: Value) -> Option<(Value, Value, Value, Value, Value)> {
    let a = at;
    let b = bt;
    if (a - b) == 0.0 {
        return None;
    }
    let y1 = 0.5 * (a - b);
    let y2 = 0.5 * (3.0 * b * b - 2.0 * a * b - a * a) * y1 / (a - b);
    let y3 = ((3.0 * b * b + a * a) * y1 * y1 + 0.5 * (3.0 * b * b - 2.0 * a * b - a * a) * y2)
        / (a - b);
    let y4 = ((3.0 * b * b - 3.0 * a * a) * y1.powi(3)
        + (9.0 * b * b + 3.0 * a * a) * y1 * y2
        + 0.5 * (3.0 * b * b - 2.0 * a * b - a * a) * y3)
        / (a - b);
    let y5 = (12.0 * a * a * y1.powi(4)
        + y1 * y1 * y2 * (18.0 * b * b - 18.0 * a * a)
        + (9.0 * b * b + 3.0 * a * a) * (y2 * y2 + y1 * y3)
        + (3.0 * b * b + a * a) * y1 * y3
        + 0.5 * (3.0 * b * b - 2.0 * a * b - a * a) * y4)
        / (a - b);
    let out = (y1, y2 / 2.0, y3 / 6.0, y4 / 24.0, y5 / 120.0);
    if [out.0, out.1, out.2, out.3, out.4]
        .iter()
        .all(|v| v.is_finite())
    {
        Some(out)
    } else {
        None
    }
}

fn get_c(
    eq1: Value,
    eq2: Value,
    eq3: Value,
    ep1: Value,
    ep2: Value,
    a: Value,
    b: Value,
) -> (Value, Value) {
    let d = (3.0 * (a * a - b * b) + 2.0 * ep1 * a + ep2).powi(2)
        + (6.0 * a * b + 2.0 * ep1 * b).powi(2);
    let ci = (-(eq1 * (a * a - b * b) + eq2 * a + eq3) * (6.0 * a * b + 2.0 * ep1 * b)
        + (2.0 * eq1 * a * b + eq2 * b) * (3.0 * (a * a - b * b) + 2.0 * ep1 * a + ep2))
        / d;
    let cr = ((3.0 * (a * a - b * b) + 2.0 * ep1 * a + ep2)
        * (eq1 * (a * a - b * b) + eq2 * a + eq3)
        + (6.0 * a * b + 2.0 * ep1 * b) * (2.0 * eq1 * a * b + eq2 * b))
        / d;
    (cr, ci)
}

fn div_c(ar: Value, ai: Value, br: Value, bi: Value) -> Option<(Value, Value)> {
    let denom = br * br + bi * bi;
    if denom == 0.0 {
        return None;
    }
    Some(((ar * br + ai * bi) / denom, (-ar * bi + ai * br) / denom))
}

fn exp_c(ar: Value, ai: Value, h: Value) -> (Value, Value) {
    let e = (ar * h).exp();
    (e * (ai * h).cos(), e * (ai * h).sin())
}

fn mult_c(ar: Value, ai: Value, br: Value, bi: Value) -> (Value, Value) {
    (ar * br - ai * bi, ar * bi + ai * br)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn txl_setup_matches_ngspice_txl1_trace_constants() {
        let txl =
            TxlRuntime::setup(12.45, 8.972e-9, 0.0, 0.468e-12, 16.0).expect("TXL setup succeeds");

        assert!(!txl.if_img);
        assert_close(txl.taul * 1.0e12, 1036.7822220698038, 1.0e-9);
        assert_close(txl.h1c, -5011045.9477987709, 1.0e-5);
        assert_close(txl.h2_aten, 0.48707085781090975, 1.0e-15);
        assert_close(txl.h3_aten, 0.0035177942924281133, 1.0e-15);
        assert_close(txl.sqt_cd_l, 0.0072223460632370425, 1.0e-16);
    }

    #[test]
    fn txl_update_cnv_matches_ngspice_compounded_h1_slope() {
        let mut txl =
            TxlRuntime::setup(12.45, 8.972e-9, 0.0, 0.468e-12, 16.0).expect("TXL setup succeeds");
        txl.initialize(0.0, 0.4, -1.0e-3, 0.2, -5.0e-4);
        let prev = *txl.history.back().expect("initialized history");
        let h = 1.0e-10;
        for i in 0..3 {
            txl.h1e[i] = (txl.h1_term[i].x * h).exp();
        }

        let original = txl.h1_term;
        let v1 = 0.44;
        let v2 = 0.31;
        txl.update_cnv(prev, v1, v2, h);

        let mut scaled_slope_i = (v1 - prev.v_i) / h;
        let mut scaled_slope_o = (v2 - prev.v_o) / h;
        for i in 0..3 {
            let term = original[i];
            let e = txl.h1e[i];
            let t = term.c / term.x;
            scaled_slope_i *= t;
            scaled_slope_o *= t;
            let expected_i = (term.cnv_i - scaled_slope_i * h) * e
                + (e - 1.0) * (v1 * t + scaled_slope_i / term.x);
            let expected_o = (term.cnv_o - scaled_slope_o * h) * e
                + (e - 1.0) * (v2 * t + scaled_slope_o / term.x);
            assert_close(txl.h1_term[i].cnv_i, expected_i, 1.0e-18);
            assert_close(txl.h1_term[i].cnv_o, expected_o, 1.0e-18);
        }
    }

    fn assert_close(actual: Value, expected: Value, tolerance: Value) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual:.17e} expected={expected:.17e} tolerance={tolerance:.17e}"
        );
    }
}
