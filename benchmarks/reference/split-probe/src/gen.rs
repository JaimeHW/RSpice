//! Emit a synthetic packed-lowering body, to measure what function splitting
//! costs at compile time and at run time.
//!
//! Shape mirrors what the real emitter will produce: a primal `let` per value
//! plus an `[f64; L]` derivative binding, chained so nothing can be eliminated.
//! With `block > 0` the body is cut into functions of that many values, each
//! taking the two live values across the boundary as arguments and returning
//! the two it leaves live — the traffic that splitting actually costs.
//!
//! usage: gen <values> <lane-width> <block-size|0> <out.rs>

use std::fmt::Write as _;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args[1].parse().expect("values");
    let l: usize = args[2].parse().expect("lane width");
    let block: usize = args[3].parse().expect("block size");
    let out_path = &args[4];

    let mut out = String::new();
    writeln!(out, "// generated: values={n} lanes={l} block={block}").unwrap();
    out.push_str("#![allow(unused_parens, dead_code)]\n");
    writeln!(out, "pub type D = [f64; {l}];\n").unwrap();

    // Every value's body, indexed by the value it defines.
    //
    // The mix matches bsimbulk's primal graph rather than being uniform:
    // 63% binary arithmetic, 19% select, 7% unary, the rest leaves. An earlier
    // revision made every fourth operation an `exp`, which put 3,007 transcendental
    // calls in a 12,028-value body and measured libm throughput instead of the
    // lowering. Transcendentals are rare in a compact model's inner graph and
    // must stay rare here.
    let body = |i: usize| -> String {
        let a = i - 1;
        let b = i - 2;
        match i % 100 {
            // 63 binary arithmetic: alternate mul, add, sub, div.
            m if m < 63 => match m % 4 {
                0 => format!(
                    "    let v{i} = v{a} * v{b};\n    let d{i}: D = {{ let l = &d{a}; let r = &d{b}; core::array::from_fn::<f64, {l}, _>(|k| l[k] * v{b} + v{a} * r[k]) }};\n"
                ),
                1 => format!(
                    "    let v{i} = v{a} + v{b};\n    let d{i}: D = {{ let l = &d{a}; let r = &d{b}; core::array::from_fn::<f64, {l}, _>(|k| l[k] + r[k]) }};\n"
                ),
                2 => format!(
                    "    let v{i} = v{a} - v{b} * 0.5;\n    let d{i}: D = {{ let l = &d{a}; let r = &d{b}; core::array::from_fn::<f64, {l}, _>(|k| l[k] - r[k] * 0.5) }};\n"
                ),
                _ => format!(
                    "    let t{i} = 1.0 + v{b} * v{b};\n    let v{i} = v{a} / t{i};\n    let d{i}: D = {{ let l = &d{a}; let r = &d{b}; core::array::from_fn::<f64, {l}, _>(|k| (l[k] - v{i} * r[k]) / t{i}) }};\n"
                ),
            },
            // 19 selects, branching on a primal comparison.
            m if m < 82 => format!(
                "    let c{i} = v{a} > 0.0;\n    let v{i} = if c{i} {{ v{a} }} else {{ v{b} }};\n    let d{i}: D = (if c{i} {{ d{a} }} else {{ d{b} }});\n"
            ),
            // 7 unary, only a third of them transcendental.
            m if m < 89 => match m % 3 {
                0 => format!(
                    "    let v{i} = (v{a} * 0.001).exp();\n    let d{i}: D = {{ let s = v{i} * 0.001; core::array::from_fn::<f64, {l}, _>(|k| s * d{a}[k]) }};\n"
                ),
                1 => format!(
                    "    let v{i} = -v{a};\n    let d{i}: D = core::array::from_fn::<f64, {l}, _>(|k| -d{a}[k]);\n"
                ),
                _ => format!(
                    "    let v{i} = v{a} * v{a};\n    let d{i}: D = {{ let s = 2.0 * v{a}; core::array::from_fn::<f64, {l}, _>(|k| s * d{a}[k]) }};\n"
                ),
            },
            // The remainder are leaves: parameters and constants folded into
            // the chain, carrying no derivative array at all.
            _ => format!("    let v{i} = v{a} * 1.000001 + 0.5;\n    let d{i}: D = d{a};\n"),
        }
    };

    let seed = format!(
        "    let v0 = seed;\n    let d0: D = {{ let mut d = [0.0f64; {l}]; d[0] = 1.0; d }};\n    let v1 = seed * 0.5 + 0.25;\n    let d1: D = {{ let mut d = [0.0f64; {l}]; d[1 % {l}] = 1.0; d }};\n"
    );

    if block == 0 {
        out.push_str("#[inline(never)]\npub fn run(seed: f64) -> f64 {\n");
        out.push_str(&seed);
        for i in 2..n {
            out.push_str(&body(i));
        }
        writeln!(
            out,
            "    v{} + d{}.iter().sum::<f64>()\n}}",
            n - 1,
            n - 1
        )
        .unwrap();
    } else {
        // Each block consumes the previous two values and produces the last two
        // it defines, so the chain stays genuinely sequential across the cut.
        let mut blocks = Vec::new();
        let mut i = 2usize;
        while i < n {
            let start = i;
            let end = (i + block).min(n);
            let mut f = String::new();
            writeln!(f, "#[inline(never)]").unwrap();
            writeln!(
                f,
                "fn block_{}(pa: f64, pda: &D, pb: f64, pdb: &D) -> (f64, D, f64, D) {{",
                blocks.len()
            )
            .unwrap();
            writeln!(
                f,
                "    let v{} = pa; let d{} = *pda; let v{} = pb; let d{} = *pdb;",
                start - 1,
                start - 1,
                start - 2,
                start - 2
            )
            .unwrap();
            for j in start..end {
                f.push_str(&body(j));
            }
            writeln!(
                f,
                "    (v{}, d{}, v{}, d{})\n}}",
                end - 1,
                end - 1,
                end - 2,
                end - 2
            )
            .unwrap();
            blocks.push(f);
            i = end;
        }

        for f in &blocks {
            out.push_str(f);
            out.push('\n');
        }

        out.push_str("#[inline(never)]\npub fn run(seed: f64) -> f64 {\n");
        out.push_str(&seed);
        out.push_str("    let (mut a, mut da, mut b, mut db) = (v1, d1, v0, d0);\n");
        for k in 0..blocks.len() {
            writeln!(
                out,
                "    let (na, nda, nb, ndb) = block_{k}(a, &da, b, &db);\n    a = na; da = nda; b = nb; db = ndb;"
            )
            .unwrap();
        }
        out.push_str("    a + da.iter().sum::<f64>() + b + db.iter().sum::<f64>()\n}\n");
    }

    // Timing harness, identical for every variant.
    out.push_str(
        r#"
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let mut samples = Vec::new();
    for _ in 0..7 {
        let started = Instant::now();
        let mut acc = 0.0;
        for i in 0..2000 {
            acc += run(black_box(0.3 + (i % 32) as f64 * 1.0e-6));
        }
        black_box(acc);
        samples.push(started.elapsed().as_secs_f64() * 1.0e9 / 2000.0);
    }
    samples.sort_by(f64::total_cmp);
    println!("{:.0}", samples[samples.len() / 2]);
}
"#,
    );

    std::fs::write(out_path, out).expect("write generated source");
}
