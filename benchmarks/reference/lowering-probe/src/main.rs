//! Does LLVM turn `[f64; L]` derivative locals into registers?
//!
//! Phase 1 of the Verilog-A backend rewrite rests on one assumption: that a
//! fixed-width derivative array held in a local can be promoted by SROA to
//! individual SSA values, so a compact `array::from_fn` chain rule compiles to
//! the same straight-line FMAs the flattened emitter writes out by hand. If it
//! does not, source has to keep scaling by operations x lanes and the whole
//! design is dead.
//!
//! Three forms of the same arithmetic are timed against each other:
//!
//!   workspace — today's StructuredKernel: values indexed out of one big array
//!               behind a `&mut`, derivative loop driven by a runtime mask, and
//!               the per-op helper marked `#[inline(never)]`.
//!   array     — the proposal: values in locals, derivatives in `[f64; L]`.
//!   flat      — today's SparseLocalKernel: one named scalar per lane.
//!
//! `flat` is the target to match and `workspace` the thing to beat. The op mix
//! (mul, add, exp, div) and chain depth are chosen to look like a compact model
//! rather than to flatter any one form.

use std::hint::black_box;
use std::time::Instant;

const CHAIN: usize = 512;
const ITERS: usize = 20_000;
const SAMPLES: usize = 9;

// ---------------------------------------------------------------- workspace

const WS_VALUES: usize = 4096;

struct Workspace<const L: usize> {
    v: Box<[f64; WS_VALUES]>,
    d: Box<[[f64; L]; WS_VALUES]>,
    masks: &'static [u128],
}

struct ActiveAxes {
    mask: u128,
}

impl ActiveAxes {
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.mask == 0 {
            return None;
        }
        let axis = self.mask.trailing_zeros() as usize;
        self.mask &= self.mask - 1;
        Some(axis)
    }
}

impl<const L: usize> Workspace<L> {
    fn new(masks: &'static [u128]) -> Self {
        Self {
            v: vec![0.0; WS_VALUES].into_boxed_slice().try_into().unwrap(),
            d: vec![[0.0; L]; WS_VALUES].into_boxed_slice().try_into().unwrap(),
            masks,
        }
    }

    #[inline]
    fn axes(&self, index: usize) -> ActiveAxes {
        ActiveAxes {
            mask: self.masks.get(index).copied().unwrap_or(u128::MAX),
        }
    }

    #[inline(never)]
    fn store_mul(&mut self, i: usize, a: usize, b: usize) {
        let av = self.v[a];
        let bv = self.v[b];
        self.v[i] = av * bv;
        let mut axes = self.axes(i);
        while let Some(k) = axes.next() {
            self.d[i][k] = self.d[a][k] * bv + av * self.d[b][k];
        }
    }

    #[inline(never)]
    fn store_add(&mut self, i: usize, a: usize, b: usize) {
        self.v[i] = self.v[a] + self.v[b];
        let mut axes = self.axes(i);
        while let Some(k) = axes.next() {
            self.d[i][k] = self.d[a][k] + self.d[b][k];
        }
    }

    #[inline(never)]
    fn store_exp(&mut self, i: usize, a: usize) {
        let e = self.v[a].exp();
        self.v[i] = e;
        let mut axes = self.axes(i);
        while let Some(k) = axes.next() {
            self.d[i][k] = e * self.d[a][k];
        }
    }

    #[inline(never)]
    fn store_div(&mut self, i: usize, a: usize, b: usize) {
        let av = self.v[a];
        let bv = self.v[b];
        let r = 1.0 / bv;
        let q = av * r;
        self.v[i] = q;
        let mut axes = self.axes(i);
        while let Some(k) = axes.next() {
            self.d[i][k] = (self.d[a][k] - q * self.d[b][k]) * r;
        }
    }
}

fn run_workspace<const L: usize>(ws: &mut Workspace<L>, seed: f64) -> f64 {
    ws.v[0] = seed;
    ws.d[0][0] = 1.0;
    ws.v[1] = seed * 0.5 + 0.25;
    ws.d[1][1 % L] = 1.0;
    for step in 0..CHAIN {
        let base = 2 + step * 4;
        ws.store_mul(base, base.wrapping_sub(2).max(0) % (base), 1);
        ws.store_add(base + 1, base, 0);
        ws.store_exp(base + 2, base + 1);
        ws.store_div(base + 3, base + 2, base + 1);
    }
    let last = 2 + (CHAIN - 1) * 4 + 3;
    ws.v[last] + ws.d[last].iter().sum::<f64>()
}

// -------------------------------------------------------------------- array

#[inline(always)]
fn a_mul<const L: usize>(av: f64, ad: &[f64; L], bv: f64, bd: &[f64; L]) -> (f64, [f64; L]) {
    (av * bv, core::array::from_fn(|k| ad[k] * bv + av * bd[k]))
}

#[inline(always)]
fn a_add<const L: usize>(av: f64, ad: &[f64; L], bv: f64, bd: &[f64; L]) -> (f64, [f64; L]) {
    (av + bv, core::array::from_fn(|k| ad[k] + bd[k]))
}

#[inline(always)]
fn a_exp<const L: usize>(av: f64, ad: &[f64; L]) -> (f64, [f64; L]) {
    let e = av.exp();
    (e, core::array::from_fn(|k| e * ad[k]))
}

#[inline(always)]
fn a_div<const L: usize>(av: f64, ad: &[f64; L], bv: f64, bd: &[f64; L]) -> (f64, [f64; L]) {
    let r = 1.0 / bv;
    let q = av * r;
    (q, core::array::from_fn(|k| (ad[k] - q * bd[k]) * r))
}

fn run_array<const L: usize>(seed: f64) -> f64 {
    let v0 = seed;
    let mut d0 = [0.0; L];
    d0[0] = 1.0;
    let v1 = seed * 0.5 + 0.25;
    let mut d1 = [0.0; L];
    d1[1 % L] = 1.0;

    let mut cv = v0;
    let mut cd = d0;
    for _ in 0..CHAIN {
        let (m, md) = a_mul(cv, &cd, v1, &d1);
        let (s, sd) = a_add(m, &md, v0, &d0);
        let (e, ed) = a_exp(s, &sd);
        let (q, qd) = a_div(e, &ed, s, &sd);
        cv = q;
        cd = qd;
    }
    cv + cd.iter().sum::<f64>()
}

// --------------------------------------------------------------------- flat
//
// L is fixed at 12 here on purpose: this form cannot be written generically,
// which is precisely the property that makes it cost 14 MB of source.

fn run_flat(seed: f64) -> f64 {
    let v0 = seed;
    let (mut c0, mut c1, mut c2, mut c3, mut c4, mut c5) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let (mut c6, mut c7, mut c8, mut c9, mut c10, mut c11) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let v1 = seed * 0.5 + 0.25;
    let (e0, e1, e2, e3, e4, e5) = (0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let (e6, e7, e8, e9, e10, e11) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let (b0, b1, b2, b3, b4, b5) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let (b6, b7, b8, b9, b10, b11) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    let mut cv = v0;
    for _ in 0..CHAIN {
        // mul by (v1, e*)
        let mv = cv * v1;
        let m0 = c0 * v1 + cv * e0; let m1 = c1 * v1 + cv * e1;
        let m2 = c2 * v1 + cv * e2; let m3 = c3 * v1 + cv * e3;
        let m4 = c4 * v1 + cv * e4; let m5 = c5 * v1 + cv * e5;
        let m6 = c6 * v1 + cv * e6; let m7 = c7 * v1 + cv * e7;
        let m8 = c8 * v1 + cv * e8; let m9 = c9 * v1 + cv * e9;
        let m10 = c10 * v1 + cv * e10; let m11 = c11 * v1 + cv * e11;
        // add (v0, b*)
        let sv = mv + v0;
        let s0 = m0 + b0; let s1 = m1 + b1; let s2 = m2 + b2; let s3 = m3 + b3;
        let s4 = m4 + b4; let s5 = m5 + b5; let s6 = m6 + b6; let s7 = m7 + b7;
        let s8 = m8 + b8; let s9 = m9 + b9; let s10 = m10 + b10; let s11 = m11 + b11;
        // exp
        let ev = sv.exp();
        let x0 = ev * s0; let x1 = ev * s1; let x2 = ev * s2; let x3 = ev * s3;
        let x4 = ev * s4; let x5 = ev * s5; let x6 = ev * s6; let x7 = ev * s7;
        let x8 = ev * s8; let x9 = ev * s9; let x10 = ev * s10; let x11 = ev * s11;
        // div by (sv, s*)
        let r = 1.0 / sv;
        let q = ev * r;
        c0 = (x0 - q * s0) * r; c1 = (x1 - q * s1) * r;
        c2 = (x2 - q * s2) * r; c3 = (x3 - q * s3) * r;
        c4 = (x4 - q * s4) * r; c5 = (x5 - q * s5) * r;
        c6 = (x6 - q * s6) * r; c7 = (x7 - q * s7) * r;
        c8 = (x8 - q * s8) * r; c9 = (x9 - q * s9) * r;
        c10 = (x10 - q * s10) * r; c11 = (x11 - q * s11) * r;
        cv = q;
    }
    cv + c0+c1+c2+c3+c4+c5+c6+c7+c8+c9+c10+c11
}

// --------------------------------------------------------------------- main

fn bench(label: &str, mut f: impl FnMut(f64) -> f64) {
    let mut samples = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let started = Instant::now();
        let mut acc = 0.0;
        for i in 0..ITERS {
            acc += f(black_box(0.3 + (i % 32) as f64 * 1.0e-6));
        }
        black_box(acc);
        let ns = started.elapsed().as_secs_f64() * 1.0e9 / (ITERS * CHAIN * 4) as f64;
        samples.push(ns);
    }
    samples.sort_by(f64::total_cmp);
    println!("{label:<28} {:>9.3} ns/op   (min {:>7.3})", samples[SAMPLES / 2], samples[0]);
}

static MASK12: [u128; WS_VALUES] = [0xfff; WS_VALUES];
static MASK32: [u128; WS_VALUES] = [0xffff_ffff; WS_VALUES];

fn main() {
    println!("chain={CHAIN} ops/iter={} iters={ITERS} samples={SAMPLES}\n", CHAIN * 4);

    let mut ws12 = Workspace::<12>::new(&MASK12);
    bench("workspace  L=12", |s| run_workspace(&mut ws12, s));
    bench("array      L=12", |s| run_array::<12>(s));
    bench("flat       L=12", run_flat);

    println!();
    let mut ws32 = Workspace::<32>::new(&MASK32);
    bench("workspace  L=32", |s| run_workspace(&mut ws32, s));
    bench("array      L=32", |s| run_array::<32>(s));

    println!();
    bench("array      L=4 ", |s| run_array::<4>(s));
    bench("array      L=8 ", |s| run_array::<8>(s));
    bench("array      L=16", |s| run_array::<16>(s));
}
