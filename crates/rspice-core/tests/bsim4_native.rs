//! Engine-level validation of the native BSIM4 v4.8 (MOS LEVEL=14/54) wiring.
//!
//! The model module (`device/mosfet/bsim4v8`) is pinned against ngspice-46
//! standalone; these tests prove the *engine* reproduces those values
//! through its own Newton solve, DC sweep, and transient integration:
//!
//! - a single-NMOS `.op` must hit the module's pinned oracle table;
//! - a CMOS inverter VTC `.dc` sweep is compared point-by-point against an
//!   ngspice-46 run of the same deck;
//! - a 3-stage ring oscillator `.tran` must oscillate with the period
//!   ngspice produces for the same deck (5% tolerance);
//! - a LEVEL=54 card builds and runs natively, without the
//!   `allow_simplified_mos` escape hatch;
//! - RDSMOD=1 external source/drain resistance creates native prime-node
//!   topology and matches ngspice on a one-point operating point.

#![allow(clippy::excessive_precision)]

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// The module's own oracle model cards (n45/p90, LEVEL=54, CAPMOD=2).
fn models45() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/device/mosfet/bsim4v8/testdata/models45.lib"
    );
    std::fs::read_to_string(path).expect("read models45.lib")
}

fn models45_mobmod(mob_mod: i32) -> String {
    models45().replace(
        "mobmod=0 u0=0.045 ua=5.0e-10 ub=1.3e-18 uc=8.0e-11 ud=1.0e15 eu=1.67",
        &format!("mobmod={mob_mod} u0=0.045 ua=5.0e-10 ub=1.3e-18 uc=8.0e-11 ud=1.0e-3 eu=1.67"),
    )
}

fn models45_wpemod() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         wpemod=1 kvth0we=4.11e-3 k2we=1.77e-3 ku0we=2e-2 web=2 wec=3",
    )
}

fn models45_stress() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         saref=1e-6 sbref=1e-6 wlod=0 ku0=2e-8 kvsat=0.25 \
         kvth0=1.5e-9 stk2=2e-9 steta0=2e-10",
    )
}

fn models45_rdsmod1() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         rdsmod=1 rdw=300 rsw=280 rdwmin=20 rswmin=18",
    )
}

fn models45_rbodymod1_high_resistance() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         rbodymod=1 rbpb=100k rbpd=100k rbps=100k rbdb=100k rbsb=100k gbmin=1e-12",
    )
}

fn models45_rbodymod2_defaults() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         rbodymod=2 rbpb=5 rbpd=15 rbps=15 rbdb=15 rbsb=15 gbmin=1e-10",
    )
}

fn models45_rgatemod2() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         rgatemod=2 rshg=5e8 xrcrg1=12 xrcrg2=1 xgw=0 xgl=0 ngcon=1 \
         trnqsmod=0 acnqsmod=0",
    )
}

fn models45_rgatemod3() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         rgatemod=3 rshg=5e8 xrcrg1=12 xrcrg2=1 xgw=0 xgl=0 ngcon=1 \
         trnqsmod=0 acnqsmod=0",
    )
}

fn models45_acnqsmod1() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 acnqsmod=1 trnqsmod=0 rgatemod=0",
    )
}

fn models45_acnqsmod1_trnqsmod1() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 acnqsmod=1 trnqsmod=1 rgatemod=0",
    )
}

fn models45_acnqsmod1_rdsmod1() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         acnqsmod=1 trnqsmod=0 rgatemod=0 rdsmod=1 rdw=300 rsw=280 rdwmin=20 rswmin=18",
    )
}

fn models45_acnqsmod1_rdsmod1_rgatemod2() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         acnqsmod=1 trnqsmod=0 rdsmod=1 rdw=300 rsw=280 rdwmin=20 rswmin=18 \
         rgatemod=2 rshg=5e8 xrcrg1=12 xrcrg2=1 xgw=0 xgl=0 ngcon=1",
    )
}

fn models45_acnqsmod1_rbodymod1_high_resistance() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         acnqsmod=1 trnqsmod=0 rbodymod=1 rbpb=100k rbpd=100k rbps=100k \
         rbdb=100k rbsb=100k gbmin=1e-12",
    )
}

fn models45_acnqsmod1_rbodymod2_defaults() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         acnqsmod=1 trnqsmod=0 rbodymod=2 rbpb=5 rbpd=15 rbps=15 \
         rbdb=15 rbsb=15 gbmin=1e-10",
    )
}

fn models45_acnqsmod1_rgatemod1() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         rgatemod=1 rshg=5e8 xgw=0 xgl=0 ngcon=1 trnqsmod=0 acnqsmod=1",
    )
}

fn models45_acnqsmod1_rgatemod2() -> String {
    models45_rgatemod2().replace("acnqsmod=0", "acnqsmod=1")
}

fn models45_acnqsmod1_rgatemod3() -> String {
    models45_rgatemod3().replace("acnqsmod=0", "acnqsmod=1")
}

fn models45_trnqsmod(trnqs_mod: i32) -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        &format!(
            ".model n45 nmos level=54 version=4.8 \
             trnqsmod={trnqs_mod} acnqsmod=0 rgatemod=0"
        ),
    )
}

fn models45_trnqsmod_rdsmod1(trnqs_mod: i32) -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        &format!(
            ".model n45 nmos level=54 version=4.8 \
             trnqsmod={trnqs_mod} acnqsmod=0 rgatemod=0 \
             rdsmod=1 rdw=300 rsw=280 rdwmin=20 rswmin=18"
        ),
    )
}

fn models45_trnqsmod_rbodymod1_high_resistance(trnqs_mod: i32) -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        &format!(
            ".model n45 nmos level=54 version=4.8 \
             trnqsmod={trnqs_mod} acnqsmod=0 rgatemod=0 \
             rbodymod=1 rbpb=100k rbpd=100k rbps=100k rbdb=100k rbsb=100k gbmin=1e-12"
        ),
    )
}

fn models45_trnqsmod_rgatemod1(trnqs_mod: i32) -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        &format!(
            ".model n45 nmos level=54 version=4.8 \
             trnqsmod={trnqs_mod} acnqsmod=0 rgatemod=1 \
             rshg=5e3 xgw=0 xgl=0 ngcon=1"
        ),
    )
}

fn models45_trnqsmod_rgatemod2(trnqs_mod: i32) -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        &format!(
            ".model n45 nmos level=54 version=4.8 \
             trnqsmod={trnqs_mod} acnqsmod=0 rgatemod=2 \
             rshg=5e3 xgw=0 xgl=0 ngcon=1"
        ),
    )
}

fn models45_trnqsmod_rgatemod3(trnqs_mod: i32) -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        &format!(
            ".model n45 nmos level=54 version=4.8 \
             trnqsmod={trnqs_mod} acnqsmod=0 rgatemod=3 \
             rshg=5e3 xgw=0 xgl=0 ngcon=1"
        ),
    )
}

fn models45_trnqsmod_rdsmod1_rgatemod1(trnqs_mod: i32) -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        &format!(
            ".model n45 nmos level=54 version=4.8 \
             trnqsmod={trnqs_mod} acnqsmod=0 rgatemod=1 \
             rshg=5e3 xgw=0 xgl=0 ngcon=1 \
             rdsmod=1 rdw=300 rsw=280 rdwmin=20 rswmin=18"
        ),
    )
}

fn models45_pmos_rgatemod2_gate_tunnel() -> String {
    models45().replace(
        ".model p90 pmos level=54 version=4.8",
        ".model p90 pmos level=54 version=4.8 \
         rgatemod=2 rshg=5e10 xrcrg1=12 xrcrg2=1 xgw=0 xgl=0 ngcon=1 \
         trnqsmod=0 acnqsmod=0 igcmod=1 igbmod=1",
    )
}

fn rgatemod2_common_source_deck() -> String {
    format!(
        "* bsim4 rgatemod=2 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1 pulse(0.3 0.8 20p 5p 5p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_rgatemod2()
    )
}

fn rgatemod3_common_source_deck() -> String {
    format!(
        "* bsim4 rgatemod=3 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1 pulse(0.3 0.8 20p 5p 5p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_rgatemod3()
    )
}

fn acnqsmod1_common_source_deck() -> String {
    format!(
        "* bsim4 acnqsmod=1 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_acnqsmod1()
    )
}

fn acnqsmod1_trnqsmod1_common_source_ac_deck() -> String {
    format!(
        "* bsim4 acnqsmod=1 trnqsmod=1 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_acnqsmod1_trnqsmod1()
    )
}

fn acnqsmod1_rdsmod1_common_source_deck() -> String {
    format!(
        "* bsim4 acnqsmod=1 rdsmod=1 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=1 nrs=1\n\
         {}\n\
         .end\n",
        models45_acnqsmod1_rdsmod1()
    )
}

fn acnqsmod1_rdsmod1_rgatemod2_common_source_deck() -> String {
    format!(
        "* bsim4 acnqsmod=1 rdsmod=1 rgatemod=2 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=1 nrs=1\n\
         {}\n\
         .end\n",
        models45_acnqsmod1_rdsmod1_rgatemod2()
    )
}

fn acnqsmod1_rbodymod1_high_resistance_common_source_ac_deck() -> String {
    format!(
        "* bsim4 acnqsmod=1 rbodymod=1 ac high body resistance\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         rd vdd out 10k\n\
         vin g 0 dc 0.7 ac 1\n\
         vb b 0 dc 0\n\
         m1 out g 0 b n45 w=1u l=45n nf=2 ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_acnqsmod1_rbodymod1_high_resistance()
    )
}

fn acnqsmod1_rbodymod2_common_source_ac_deck() -> String {
    format!(
        "* bsim4 acnqsmod=1 rbodymod=2 ac common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         rd vdd out 10k\n\
         vin g 0 dc 0.7 ac 1\n\
         vb b 0 dc 0\n\
         m1 out g 0 b n45 w=1u l=45n nf=2 ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_acnqsmod1_rbodymod2_defaults()
    )
}

fn acnqsmod1_rgatemod1_common_source_deck() -> String {
    format!(
        "* bsim4 acnqsmod=1 rgatemod=1 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_acnqsmod1_rgatemod1()
    )
}

fn acnqsmod1_rgatemod2_common_source_deck() -> String {
    format!(
        "* bsim4 acnqsmod=1 rgatemod=2 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1 pulse(0.3 0.8 20p 5p 5p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_acnqsmod1_rgatemod2()
    )
}

fn acnqsmod1_rgatemod3_common_source_deck() -> String {
    format!(
        "* bsim4 acnqsmod=1 rgatemod=3 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1 pulse(0.3 0.8 20p 5p 5p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_acnqsmod1_rgatemod3()
    )
}

fn trnqsmod_common_source_deck(trnqs_mod: i32) -> String {
    format!(
        "* bsim4 trnqsmod={trnqs_mod} common source transient\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 pulse(0.3 0.8 20p 2p 2p 500p 1n)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .tran 0.1p 80p\n\
         .end\n",
        models45_trnqsmod(trnqs_mod)
    )
}

fn trnqsmod_rdsmod1_common_source_deck(trnqs_mod: i32) -> String {
    format!(
        "* bsim4 trnqsmod={trnqs_mod} rdsmod=1 common source transient\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 pulse(0.3 0.8 20p 2p 2p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=1 nrs=1\n\
         {}\n\
         .tran 0.1p 80p\n\
         .end\n",
        models45_trnqsmod_rdsmod1(trnqs_mod)
    )
}

fn trnqsmod_rbodymod1_common_source_deck(trnqs_mod: i32) -> String {
    format!(
        "* bsim4 trnqsmod={trnqs_mod} rbodymod=1 common source transient\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 pulse(0.3 0.8 20p 2p 2p 500p 1n)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .tran 0.1p 80p\n\
         .end\n",
        models45_trnqsmod_rbodymod1_high_resistance(trnqs_mod)
    )
}

fn trnqsmod_rgatemod1_common_source_deck(trnqs_mod: i32) -> String {
    format!(
        "* bsim4 trnqsmod={trnqs_mod} rgatemod=1 common source transient\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 pulse(0.3 0.8 20p 2p 2p 500p 1n)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .tran 0.1p 200p\n\
         .end\n",
        models45_trnqsmod_rgatemod1(trnqs_mod)
    )
}

fn trnqsmod_rgatemod2_common_source_deck(trnqs_mod: i32) -> String {
    format!(
        "* bsim4 trnqsmod={trnqs_mod} rgatemod=2 common source transient\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 pulse(0.3 0.8 20p 2p 2p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .tran 0.1p 200p\n\
         .end\n",
        models45_trnqsmod_rgatemod2(trnqs_mod)
    )
}

fn trnqsmod_rgatemod3_common_source_deck(trnqs_mod: i32) -> String {
    format!(
        "* bsim4 trnqsmod={trnqs_mod} rgatemod=3 common source transient\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 pulse(0.3 0.8 20p 2p 2p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .tran 0.1p 200p\n\
         .end\n",
        models45_trnqsmod_rgatemod3(trnqs_mod)
    )
}

fn trnqsmod_rdsmod1_rgatemod1_common_source_deck(trnqs_mod: i32) -> String {
    format!(
        "* bsim4 trnqsmod={trnqs_mod} rdsmod=1 rgatemod=1 common source transient\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 pulse(0.3 0.8 20p 2p 2p 500p 1n)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=1 nrs=1\n\
         {}\n\
         .tran 0.1p 200p\n\
         .end\n",
        models45_trnqsmod_rdsmod1_rgatemod1(trnqs_mod)
    )
}

fn rgatemod2_pmos_gate_tunnel_deck() -> String {
    format!(
        "* bsim4 pmos rgatemod=2 gate tunneling\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0 ac 1\n\
         vb b 0 dc 1.1\n\
         rd out 0 2k\n\
         m1 out in vdd b p90 w=10u l=90n nf=2 ad=1p as=1p pd=20u ps=20u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_pmos_rgatemod2_gate_tunnel()
    )
}

fn rbodymod1_high_resistance_common_source_ac_deck() -> String {
    format!(
        "* bsim4 rbodymod=1 ac high body resistance\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         rd vdd out 10k\n\
         vin g 0 dc 0.7 ac 1\n\
         vb b 0 dc 0\n\
         m1 out g 0 b n45 w=1u l=45n nf=2 ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_rbodymod1_high_resistance()
    )
}

fn models45_mtrlmod1_compat1() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         mtrlmod=1 mtrlcompatmod=1 phig=4.05 epsrgate=11.7 epsrsub=11.7",
    )
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn bsim4_nmos_op_id(instance_suffix: &str) -> Result<f64, String> {
    let deck = format!(
        "* bsim4 native multiplier op\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         m1 d g 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0 {instance_suffix}\n\
         {}\n\
         .op\n\
         .end\n",
        models45()
    );
    let netlist = Netlist::parse(&deck).expect("BSIM4 multiplier deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .map_err(|error| error.to_string())?;
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    Ok(entry
        .params
        .iter()
        .find(|(key, _)| *key == "id")
        .map(|(_, value)| *value)
        .expect("m1 id op param"))
}

#[test]
fn native_bsim4_rejects_non_numeric_model_params_before_defaulting() {
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 rdsmod=\"1\"",
    );
    let deck = format!(
        "* bsim4 non-numeric native model param policy\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 0.7\n\
         m1 d g 0 0 n45 w=1u l=45n\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let message = engine()
        .run_dc_op_with_report(&netlist)
        .expect_err("non-numeric BSIM4 model parameter must not fall back to the default")
        .to_string();

    assert!(
        message.contains("BSIM4") && message.contains("RDSMOD"),
        "error should identify the non-numeric native BSIM4 model parameter: {message}"
    );
    assert!(
        message.contains("non-numeric") && message.contains("finite numeric literal"),
        "error should explain native BSIM4 params must be numeric: {message}"
    );
}

#[test]
fn native_bsim4_accepts_dotted_version_metadata() {
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.6.1",
    );
    let deck = format!(
        "* bsim4 dotted VERSION metadata policy\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 0.7\n\
         m1 d g 0 0 n45 w=1u l=45n\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("dotted VERSION deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("native BSIM4 accepts dotted VERSION metadata");

    let id = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .and_then(|entry| {
            entry
                .params
                .iter()
                .find(|(key, _)| *key == "id")
                .map(|(_, value)| *value)
        })
        .expect("m1 drain current reported");
    assert!(
        id.is_finite(),
        "dotted VERSION run produced non-finite id {id}"
    );
}

#[test]
fn native_bsim4_rejects_invalid_integer_model_selectors_without_defaulting() {
    for (param, value) in [
        ("RDSMOD", "2"),
        ("RGATEMOD", "4"),
        ("GIDLMOD", "2"),
        ("MOBMOD", "6.5"),
        ("GEOMOD", "10.5"),
        ("ACNQSMOD", "1.5"),
    ] {
        let models = if param == "MOBMOD" {
            models45().replace(
                "mobmod=0 u0=0.045 ua=5.0e-10 ub=1.3e-18 uc=8.0e-11 ud=1.0e15 eu=1.67",
                &format!(
                    "mobmod={value} u0=0.045 ua=5.0e-10 ub=1.3e-18 uc=8.0e-11 ud=1.0e15 eu=1.67"
                ),
            )
        } else {
            models45().replace(
                ".model n45 nmos level=54 version=4.8",
                &format!(".model n45 nmos level=54 version=4.8 {param}={value}"),
            )
        };
        let deck = format!(
            "* bsim4 invalid integer selector policy\n\
             vd d 0 dc 1.1\n\
             vg g 0 dc 0.7\n\
             m1 d g 0 0 n45 w=1u l=45n\n\
             {models}\n\
             .op\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let message = match engine().run_dc_op_with_report(&netlist) {
            Ok(_) => {
                panic!("BSIM4 {param}={value} must reject instead of defaulting/truncating")
            }
            Err(error) => error.to_string(),
        };

        assert!(
            message.contains("BSIM4") && message.contains(param),
            "error should identify invalid BSIM4 selector {param}={value}: {message}"
        );
        assert!(
            message.contains("finite integer"),
            "error should explain selector integer policy for {param}={value}: {message}"
        );
    }
}

#[test]
fn native_bsim4_capmod3_and_diomod_neg1_reset_to_defaults_like_ngspice46() {
    // ngspice-46 warns and resets out-of-range integer CAPMOD to 2.
    let capmod_models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 capmod=3",
    );
    let capmod_deck = format!(
        "* bsim4 capmod3 default reset op\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         m1 d g 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {capmod_models}\n\
         .op\n\
         .end\n"
    );
    let capmod_netlist = Netlist::parse(&capmod_deck).expect("CAPMOD=3 reset deck parses");
    let (_, capmod_report) = engine()
        .run_dc_op_with_report(&capmod_netlist)
        .expect("CAPMOD=3 should reset to default CAPMOD=2");
    let capmod_id = capmod_report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry")
        .params
        .iter()
        .find(|(key, _)| *key == "id")
        .map(|(_, value)| *value)
        .expect("m1 id op param");
    let capmod_reference = 1.408_919_35e-3;
    let capmod_rel = (capmod_id - capmod_reference).abs() / capmod_reference;
    assert!(
        capmod_rel < 1.0e-6,
        "CAPMOD=3 reset id mismatch: rspice={capmod_id:.9e} ngspice={capmod_reference:.9e} rel={capmod_rel:.3e}"
    );

    // ngspice-46 warns and resets out-of-range integer DIOMOD to 1.
    let diomod_models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 diomod=-1",
    );
    let diomod_deck = format!(
        "* bsim4 diomod negative default reset op\n\
         vs s 0 dc 0\n\
         vd d 0 dc 0\n\
         vg g 0 dc 0\n\
         vb b 0 dc -12\n\
         m1 d g s b n45 w=1u l=45n ad=0 as=0.1p pd=0 ps=2.2u nrd=0 nrs=0\n\
         {diomod_models}\n\
         .op\n\
         .end\n"
    );
    let diomod_netlist = Netlist::parse(&diomod_deck).expect("DIOMOD=-1 reset deck parses");
    let diomod_result = engine()
        .run_dc_op(&diomod_netlist)
        .expect("DIOMOD=-1 should reset to default DIOMOD=1");
    let body_current = diomod_result
        .branch_current_named("vb")
        .unwrap_or_else(|| panic!("missing vb branch in {:?}", diomod_result.branch_names));
    let diomod_reference = 2.400_299e-11;
    let diomod_rel = (body_current - diomod_reference).abs() / diomod_reference.abs();
    assert!(
        diomod_rel < 1.0e-3,
        "DIOMOD=-1 reset body branch mismatch: rspice={body_current:.9e} ngspice={diomod_reference:.9e} rel={diomod_rel:.3e}"
    );
}

#[test]
fn native_bsim4_rejects_invalid_integer_instance_selectors_without_truncating() {
    for (param, value) in [
        ("GEOMOD", "10.5"),
        ("GEOMOD", "11"),
        ("RGEOMOD", "8.5"),
        ("RGEOMOD", "9"),
        ("MIN", "1.5"),
        ("MIN", "2"),
    ] {
        let deck = format!(
            "* bsim4 invalid instance integer selector policy\n\
             vd d 0 dc 1.1\n\
             vg g 0 dc 0.7\n\
             m1 d g 0 0 n45 w=1u l=45n nf=2 {param}={value}\n\
             {}\n\
             .op\n\
             .end\n",
            models45()
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let message = match engine().run_dc_op_with_report(&netlist) {
            Ok(_) => {
                panic!("BSIM4 instance {param}={value} must reject instead of truncating")
            }
            Err(error) => error.to_string(),
        };

        assert!(
            message.contains("BSIM4") && message.contains(param),
            "error should identify invalid BSIM4 instance selector {param}={value}: {message}"
        );
        assert!(
            message.contains("finite integer"),
            "error should explain instance selector integer policy for {param}={value}: {message}"
        );
    }
}

#[test]
fn native_bsim4_fractional_instance_selectors_round_like_ngspice46() {
    let geometry_sensitive_models = |geo_mod: i32| {
        models45().replace(
            ".model n45 nmos level=54 version=4.8",
            &format!(
                ".model n45 nmos level=54 version=4.8 geomod={geo_mod} \
                 jss=1e-3 jsd=1e-3 jsws=2e-6 jswd=2e-6 jswgs=2e-6 jswgd=2e-6"
            ),
        )
    };
    let model_geomod1 = geometry_sensitive_models(1);
    let model_geomod0 = geometry_sensitive_models(0);
    let body_branch_current = |models: &str, instance_tail: &str| {
        let deck = format!(
            "* bsim4 fractional instance selector rounding\n\
             vd d 0 dc 0\n\
             vg g 0 dc 0\n\
             vb b 0 dc -0.45\n\
             m1 d g 0 b n45 w=1u l=45n nf=3 {instance_tail}\n\
             {models}\n\
             .op\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let result = engine()
            .run_dc_op(&netlist)
            .expect("fractional GEOMOD deck runs natively");
        let body_branch = result
            .branch_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("vb"))
            .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names));
        result.branch_currents[body_branch]
    };

    let geomod0 = body_branch_current(&model_geomod1, "geomod=0");
    let geomod0_fractional = body_branch_current(&model_geomod1, "geomod=0.4");
    assert!(
        (geomod0_fractional - geomod0).abs() < 1e-12,
        "GEOMOD=0.4 should round to GEOMOD=0 like ngspice-46"
    );

    let geomod1 = body_branch_current(&model_geomod1, "");
    let geomod1_fractional = body_branch_current(&model_geomod0, "geomod=0.5");
    assert!(
        (geomod1_fractional - geomod1).abs() < 1e-12,
        "GEOMOD=0.5 should round to GEOMOD=1 like ngspice-46"
    );

    let rgeo_deck = |tail: &str| {
        format!(
            "* bsim4 fractional rgeomod selector rounding\n\
             vd d 0 dc 1.1\n\
             vg g 0 dc 1.1\n\
             vb b 0 dc 0\n\
             m1 d g 0 b n45 w=1u l=45n nf=3 geomod=1 {tail}\n\
             {}\n\
             .op\n\
             .end\n",
            models45()
        )
    };
    for name in ["m1.__rd", "m1.__rs"] {
        let rounded = bsim4_resistor_conductance(&rgeo_deck("rgeomod=1.9"), name);
        let integer = bsim4_resistor_conductance(&rgeo_deck("rgeomod=2"), name);
        assert!(
            (rounded - integer).abs() < 1e-12,
            "RGEOMOD=1.9 should round to RGEOMOD=2 for {name}: rounded={rounded:.12e} integer={integer:.12e}"
        );
    }

    for tail in ["min=0.4", "min=0.5"] {
        let deck = format!(
            "* bsim4 fractional min selector rounding\n\
             vd d 0 dc 0\n\
             vg g 0 dc 0\n\
             vb b 0 dc -0.45\n\
             m1 d g 0 b n45 w=1u l=45n nf=2 geomod=1 {tail}\n\
             {model_geomod1}\n\
             .op\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        engine()
            .run_dc_op(&netlist)
            .unwrap_or_else(|err| panic!("{tail} should round like ngspice-46: {err}"));
    }
}

#[test]
fn native_bsim4_mult_alias_matches_m_multiplier() {
    let base_id = bsim4_nmos_op_id("").expect("BSIM4 default multiplier op converges");
    let m_id = bsim4_nmos_op_id("m=3").expect("BSIM4 M=3 op converges");
    let mult_id = bsim4_nmos_op_id("mult=3").expect("BSIM4 MULT=3 op converges");
    let rel = (mult_id - m_id).abs() / m_id.abs().max(1e-30);
    let m_ratio = (m_id - 3.0 * base_id).abs() / m_id.abs().max(1e-30);
    let mult_ratio = (mult_id - 3.0 * base_id).abs() / mult_id.abs().max(1e-30);

    assert!(
        rel < 1e-12,
        "BSIM4 MULT=3 must match M=3: MULT id={mult_id:.9e}, M id={m_id:.9e}, rel={rel:.3e}"
    );
    assert!(
        m_ratio < 1e-12 && mult_ratio < 1e-12,
        "BSIM4 M/MULT=3 must scale default current by 3: base={base_id:.9e}, M={m_id:.9e}, MULT={mult_id:.9e}"
    );
}

#[test]
fn native_bsim4_rejects_invalid_multiplicity_aliases() {
    for suffix in [
        "M=0",
        "MULT=0",
        "M=-1",
        "MULT=-1",
        "M=3 MULT=0",
        "MULT=3 M=0",
    ] {
        let message =
            bsim4_nmos_op_id(suffix).expect_err("invalid BSIM4 multiplicity must fail closed");
        assert!(
            message.contains("BSIM4") && message.contains("finite"),
            "unexpected invalid BSIM4 {suffix} error: {message}"
        );
    }
}

fn bsim4_resistor_conductance(deck: &str, name: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let circuit = engine().build_circuit(&netlist).expect("circuit builds");
    let resistors = circuit.resistor_storage();
    let idx = resistors
        .names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("resistor {name} missing; have {:?}", resistors.names));
    resistors.conductances[idx]
}

fn bsim4_has_resistor(deck: &str, name: &str) -> bool {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let circuit = engine().build_circuit(&netlist).expect("circuit builds");
    circuit
        .resistor_storage()
        .names
        .iter()
        .any(|n| n.eq_ignore_ascii_case(name))
}

#[test]
fn single_nmos_op_matches_module_oracle() {
    // m1 = 1u/45n at vds=1.1, vgs=1.1, vbs=0, T=27C: the exact bias of the
    // module's ngspice_pinned_nmos_idvg_saturation row (and the geometry of
    // testdata/nmos_oracle.sp m1). The engine must reproduce what the
    // module produced standalone — that proves the builder/stamp/solve
    // wiring, not just the device math.
    let deck = format!(
        "* bsim4 native op\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         m1 d g 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .op\n\
         .end\n",
        models45()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("op converges");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(
        entry.device_kind, "BSIM4",
        "native port, not an approximation"
    );
    assert_eq!(entry.region, Some("saturation"));
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    // ngspice-46 references (module tests.rs, 9 significant digits; the
    // vth row is the vgs=0 entry of the same vds=1.1/vbs=0 sweep — Vth does
    // not depend on vgs). The module matches them at <= 4.9e-9 relative;
    // through the engine solve the only extra error is the Newton stopping
    // tolerance on the (source-driven, exactly-biased) terminals — keep 1e-6.
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs();
        assert!(
            rel < 1e-6,
            "{what}: engine={ours:.9e} oracle={reference:.9e} rel={rel:.2e}"
        );
    };
    assert_rel("id", get("id"), 1.40891935e-03);
    assert_rel("gm", get("gm"), 1.87452469e-03);
    assert_rel("gds", get("gds"), 3.04461834e-04);
    assert_rel("gmb", get("gmb"), -1.78776023e-03);
    assert_rel("vth", get("vth"), 3.16523792e-01);
    assert_rel("vdsat", get("vdsat"), 3.92689365e-01);
}

/// The inverter used by the VTC, AC, and ring tests: 2u/90n p90 PMOS over
/// 1u/45n n45 NMOS on a 1.1 V rail, junction geometry spelled out.
fn inverter_pair(name: &str, input: &str, output: &str) -> String {
    format!(
        "mp{name} {output} {input} vdd vdd p90 w=2u l=90n ad=0.2p as=0.2p pd=4.2u ps=4.2u nrd=0 nrs=0\n\
         mn{name} {output} {input} 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n"
    )
}

#[test]
fn cmos_inverter_vtc_matches_ngspice() {
    let deck = format!(
        "* bsim4 inverter vtc\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0\n\
         {}\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models45()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let results = engine()
        .run_dc_sweep(&netlist, "vin", 0.0, 1.1, 0.05)
        .expect("vtc sweep converges");

    // ngspice-46 reference: `ngspice -b` on this same deck (models45.lib,
    // `dc vin 0 1.1 0.05` with `.option reltol=1e-6`, wrdata v(out)), run
    // 2026-06-12 from the local ngspice-46 source build. GIDL and the
    // junction/TAT leakage set the sub-mV rail offsets, so they are part of
    // the comparison.
    let reference: &[(f64, f64)] = &[
        (0.2, 1.09852267e+00),
        (0.3, 1.08160866e+00),
        (0.4, 9.57362956e-01),
        (0.45, 5.96754014e-01), // steepest point of the transition
        (0.5, 1.14564480e-01),
        (0.6, 2.24662187e-02),
        (0.8, 1.13518376e-03),
        (1.0, 1.27938201e-05),
        (1.1, 1.04583797e-06),
    ];
    for &(vin, vout_ref) in reference {
        let (_, result) = results
            .iter()
            .find(|(v, _)| (v - vin).abs() < 1e-9)
            .unwrap_or_else(|| panic!("sweep point vin={vin} present"));
        let node = result.node_index_named("out").expect("node out in result");
        let vout = result.voltage(node);
        // The device math matches ngspice to ~5e-9; the budget here is the
        // two solvers' Newton stopping criteria. The high-gain transition
        // points are input-error amplified, so they get a wider (still
        // sub-mV) gate.
        let tol = if (vin - 0.45).abs() < 1e-9 || (vin - 0.4).abs() < 1e-9 {
            5e-4
        } else {
            1e-5 * vout_ref.abs() + 1e-6
        };
        println!(
            "VTC vin={vin:.2}: engine={vout:.9e} ngspice={vout_ref:.9e} delta={:.2e}",
            (vout - vout_ref).abs()
        );
        assert!(
            (vout - vout_ref).abs() < tol,
            "VTC at vin={vin}: engine={vout:.9e} ngspice={vout_ref:.9e} (tol {tol:.1e})"
        );
    }
}

/// Rising-edge crossing times of `threshold`, linearly interpolated.
fn rising_crossings(time: &[f64], wave: &[f64], threshold: f64) -> Vec<f64> {
    let mut crossings = Vec::new();
    for i in 1..time.len() {
        if wave[i - 1] < threshold && wave[i] >= threshold {
            let f = (threshold - wave[i - 1]) / (wave[i] - wave[i - 1]);
            crossings.push(time[i - 1] + f * (time[i] - time[i - 1]));
        }
    }
    crossings
}

fn interp_waveform(time: &[f64], wave: &[f64], target: f64) -> f64 {
    assert_eq!(time.len(), wave.len(), "time/waveform length mismatch");
    if target <= time[0] {
        return wave[0];
    }
    for i in 1..time.len() {
        if time[i] >= target {
            let f = (target - time[i - 1]) / (time[i] - time[i - 1]);
            return wave[i - 1] + f * (wave[i] - wave[i - 1]);
        }
    }
    *wave.last().expect("nonempty waveform")
}

#[test]
fn ring_oscillator_period_matches_ngspice() {
    // 3-stage ring, CAPMOD=2 intrinsic + overlap + junction charges as the
    // only load. `.ic v(n1)=0` kicks it off the metastable rail.
    let deck = format!(
        "* bsim4 ring oscillator\n\
         vdd vdd 0 dc 1.1\n\
         {}\
         {}\
         {}\
         .ic v(n1)=0\n\
         {}\n\
         .tran 0.5p 2n\n\
         .end\n",
        inverter_pair("1", "n1", "n2"),
        inverter_pair("2", "n2", "n3"),
        inverter_pair("3", "n3", "n1"),
        models45()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // 0.5 ps step cap, the deck's own tstep: at this 67 ps period the
    // engine's period error is step-resolution dominated and first-order
    // in the cap (2p -> 5.0%, 1p -> 2.4%, 0.5p -> 1.2%, 0.25p -> 0.6%
    // against the ngspice reference below).
    let result = engine()
        .run_tran(&netlist, 2e-9, 0.5e-12)
        .expect("ring transient runs");
    let wave = result
        .try_voltage_waveform_named("n1")
        .expect("n1 waveform");
    let crossings = rising_crossings(&result.time, wave, 0.55);
    assert!(
        crossings.len() >= 10,
        "ring must oscillate: only {} rising crossings of 0.55 V in 2 ns",
        crossings.len()
    );
    let periods: Vec<f64> = crossings.windows(2).map(|w| w[1] - w[0]).collect();
    let tail = &periods[periods.len().saturating_sub(5)..];
    let period = tail.iter().sum::<f64>() / tail.len() as f64;

    // ngspice-46 reference: same deck (`.tran 0.5p 2n`, `.ic v(n1)=0`),
    // run 2026-06-12; the period settles to 6.7417e-11 s (14.8 GHz) with
    // <0.001% cycle-to-cycle spread after the first cycle (and is
    // tolerance-converged: reltol=1e-5/trtol=1/tstep=0.1p moves it only
    // to 6.7398e-11).
    let reference = 6.7417e-11;
    let rel = (period - reference).abs() / reference;
    println!(
        "ring: {} crossings, period engine={period:.6e} ngspice={reference:.4e} rel={rel:.4}",
        crossings.len()
    );
    assert!(
        rel < 0.05,
        "ring period: engine={period:.4e} ngspice={reference:.4e} rel={rel:.3}"
    );
}

#[test]
fn inverter_ac_response_matches_ngspice() {
    // Small-signal check of the AC path: DC linearization (gm/gds/gmbs +
    // junction/GIDL conductances) on the real axis, the mode-assembled
    // BSIM4 capacitance matrix (intrinsic CAPMOD=2 + overlaps +
    // capbd/capbs) on the imaginary axis. The inverter is biased
    // mid-transition with a 10 fF load so the pole sits inside the sweep.
    let deck = format!(
        "* bsim4 inverter ac\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models45()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // ngspice-46 reference: `ac dec 2 1e6 1e11` on this deck with
    // `.option reltol=1e-6`, run 2026-06-12 (vdb(out), ph(out)).
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 2.18108888e+01, 3.14106759e+00),
        (1.000000e7, 2.18107735e+01, 3.13634209e+00),
        (1.000000e8, 2.17992505e+01, 3.08913285e+00),
        (1.000000e9, 2.07784764e+01, 2.65660742e+00),
        (1.000000e10, 7.38496114e+00, 1.69188063e+00),
        (1.000000e11, -1.07592260e+01, 9.81387293e-01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, db_ref, ph_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let ph = v.arg();
        println!("AC f={freq:.3e}: engine ({db:.5} dB, {ph:.5} rad) ngspice ({db_ref}, {ph_ref})");
        assert!(
            (db - db_ref).abs() < 1e-3,
            "AC magnitude at {freq:.3e} Hz: engine={db:.5} dB ngspice={db_ref} dB"
        );
        assert!(
            (ph - ph_ref).abs() < 1e-3,
            "AC phase at {freq:.3e} Hz: engine={ph:.5} ngspice={ph_ref}"
        );
    }
}

#[test]
fn inverter_ac_response_with_cvchargemod1_matches_ngspice() {
    // Same inverter as the default BSIM4 AC oracle, but with the n45 model's
    // CAPMOD=2 charge path using CVCHARGEMOD=1. ngspice-46 reference from a
    // local `ngspice_con.exe -b` run on this deck with `.option reltol=1e-6`.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 cvchargemod=1",
    );
    let deck = format!(
        "* bsim4 cvchargemod1 inverter ac\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 2.18108888e+01, 3.14106762e+00),
        (1.000000e7, 2.18107733e+01, 3.13634232e+00),
        (1.000000e8, 2.17992373e+01, 3.08913520e+00),
        (1.000000e9, 2.07774344e+01, 2.65669123e+00),
        (1.000000e10, 7.37846671e+00, 1.69487478e+00),
        (1.000000e11, -1.08898361e+01, 1.00262907e+00),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, db_ref, ph_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let ph = v.arg();
        assert!(
            (db - db_ref).abs() < 1e-3,
            "CVCHARGEMOD=1 AC magnitude at {freq:.3e} Hz: engine={db:.5} dB ngspice={db_ref} dB"
        );
        assert!(
            (ph - ph_ref).abs() < 1e-3,
            "CVCHARGEMOD=1 AC phase at {freq:.3e} Hz: engine={ph:.5} ngspice={ph_ref}"
        );
    }
}

#[test]
fn inverter_ac_response_with_cvchargemod2_matches_ngspice_nonzero_path() {
    // ngspice-46 stores CVCHARGEMOD as an integer selector but BSIM4load
    // branches only on == 0; selector 2 therefore uses the same nonzero
    // capacitance path as selector 1.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 cvchargemod=2",
    );
    let deck = format!(
        "* bsim4 cvchargemod2 inverter ac\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 2.18108888e+01, 3.14106762e+00),
        (1.000000e7, 2.18107733e+01, 3.13634232e+00),
        (1.000000e8, 2.17992373e+01, 3.08913520e+00),
        (1.000000e9, 2.07774344e+01, 2.65669123e+00),
        (1.000000e10, 7.37846671e+00, 1.69487478e+00),
        (1.000000e11, -1.08898361e+01, 1.00262907e+00),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, db_ref, ph_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let ph = v.arg();
        assert!(
            (db - db_ref).abs() < 1e-3,
            "CVCHARGEMOD=2 AC magnitude at {freq:.3e} Hz: engine={db:.5} dB ngspice={db_ref} dB"
        );
        assert!(
            (ph - ph_ref).abs() < 1e-3,
            "CVCHARGEMOD=2 AC phase at {freq:.3e} Hz: engine={ph:.5} ngspice={ph_ref}"
        );
    }
}

#[test]
fn cvchargemod_outside_supported_integer_set_is_rejected_for_ac() {
    let selector = "2.5";
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        &format!(".model n45 nmos level=54 version=4.8 cvchargemod={selector}"),
    );
    let deck = format!(
        "* bsim4 unsupported cvchargemod ac\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let err = engine()
        .run_ac(&netlist, &[1.0e6])
        .expect_err("unsupported CVCHARGEMOD should reject AC");
    let message = err.to_string();
    assert!(
        message.contains("CVCHARGEMOD"),
        "CVCHARGEMOD={selector}: unexpected error: {message}"
    );
}

#[test]
fn inverter_ac_response_with_cvchargemod1_matches_xyce710() {
    // Same CVCHARGEMOD=1 AC deck as the ngspice oracle above. Xyce 7.10's
    // BSIM4 front accepts the deck as LEVEL=14/54 and maps VERSION=4.8 to
    // its supported 4.8.2 implementation; these pins keep that compatibility
    // explicit without making Verilog-A the default path.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 cvchargemod=1",
    );
    let deck = format!(
        "* bsim4 cvchargemod1 inverter ac xyce\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 1.23181203e+01, 1.79969933e+02),
        (1.000000e7, 1.23179567e+01, 1.79699333e+02),
        (1.000000e8, 1.23016247e+01, 1.76995951e+02),
        (1.000000e9, 1.09375381e+01, 1.52229378e+02),
        (1.000000e10, 2.33959817e+00, 9.71147197e+01),
        (1.000000e11, 2.85584192e-01, 5.74469892e+01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, mag_ref, phase_deg_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let mag = v.norm();
        let phase_deg = v.arg().to_degrees();
        let mag_rel = (mag - mag_ref).abs() / mag_ref;
        assert!(
            mag_rel < 1e-3,
            "CVCHARGEMOD=1 AC magnitude vs Xyce at {freq:.3e} Hz: engine={mag:.9e} xyce={mag_ref:.9e} rel={mag_rel:.3e}"
        );
        assert!(
            (phase_deg - phase_deg_ref).abs() < 5e-2,
            "CVCHARGEMOD=1 AC phase vs Xyce at {freq:.3e} Hz: engine={phase_deg:.6} deg xyce={phase_deg_ref:.6} deg"
        );
    }
}

#[test]
fn inverter_ac_response_with_cvchargemod2_matches_xyce710_nonzero_path() {
    // Xyce 7.10's BSIM4 v4.8.2 source mirrors ngspice here: CVCHARGEMOD=0
    // takes the voffcv/noff branch, while every nonzero selector takes the
    // alternate branch. These pins keep selector 2 native and Xyce-compatible.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 cvchargemod=2",
    );
    let deck = format!(
        "* bsim4 cvchargemod2 inverter ac xyce\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 1.23181203e+01, 1.79969933e+02),
        (1.000000e7, 1.23179567e+01, 1.79699333e+02),
        (1.000000e8, 1.23016247e+01, 1.76995951e+02),
        (1.000000e9, 1.09375381e+01, 1.52229378e+02),
        (1.000000e10, 2.33959817e+00, 9.71147197e+01),
        (1.000000e11, 2.85584192e-01, 5.74469892e+01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, mag_ref, phase_deg_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let mag = v.norm();
        let phase_deg = v.arg().to_degrees();
        let mag_rel = (mag - mag_ref).abs() / mag_ref;
        assert!(
            mag_rel < 1e-3,
            "CVCHARGEMOD=2 AC magnitude vs Xyce at {freq:.3e} Hz: engine={mag:.9e} xyce={mag_ref:.9e} rel={mag_rel:.3e}"
        );
        assert!(
            (phase_deg - phase_deg_ref).abs() < 5e-2,
            "CVCHARGEMOD=2 AC phase vs Xyce at {freq:.3e} Hz: engine={phase_deg:.6} deg xyce={phase_deg_ref:.6} deg"
        );
    }
}

#[test]
fn inverter_ac_response_with_cvchargemod3_matches_xyce710_nonzero_path() {
    // Xyce 7.10 accepts integer CVCHARGEMOD=3 and evaluates it through the
    // same nonzero charge branch as selectors 1 and 2.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 cvchargemod=3",
    );
    let deck = format!(
        "* bsim4 cvchargemod3 inverter ac xyce\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 1.23181203e+01, 1.79969933e+02),
        (1.000000e7, 1.23179567e+01, 1.79699333e+02),
        (1.000000e8, 1.23016247e+01, 1.76995951e+02),
        (1.000000e9, 1.09375381e+01, 1.52229378e+02),
        (1.000000e10, 2.33959817e+00, 9.71147197e+01),
        (1.000000e11, 2.85584192e-01, 5.74469892e+01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, mag_ref, phase_deg_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let mag = v.norm();
        let phase_deg = v.arg().to_degrees();
        let mag_rel = (mag - mag_ref).abs() / mag_ref;
        assert!(
            mag_rel < 1e-3,
            "CVCHARGEMOD=3 AC magnitude vs Xyce at {freq:.3e} Hz: engine={mag:.9e} xyce={mag_ref:.9e} rel={mag_rel:.3e}"
        );
        assert!(
            (phase_deg - phase_deg_ref).abs() < 5e-2,
            "CVCHARGEMOD=3 AC phase vs Xyce at {freq:.3e} Hz: engine={phase_deg:.6} deg xyce={phase_deg_ref:.6} deg"
        );
    }
}

#[test]
fn rgatemod1_common_source_ac_matches_ngspice46() {
    // Reduced from Xyce Regression `ACtests/bsim4/gstage.cir`: one BSIM4
    // common-source gain stage, resistive drain load, and an AC gate drive.
    // The model is forced to RGATEMOD=1 with MTRLMOD/RBODY/NQS disabled. The
    // large RSHG value makes the external-gate electrode resistance visible at
    // the tested frequencies. ngspice-46 reference from `ngspice_con.exe -b`.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 rgatemod=1 rshg=5e8 xgw=0 xgl=0 ngcon=1",
    );
    let deck = format!(
        "* bsim4 rgatemod1 ac\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         rd vdd out 1k\n\
         vin g 0 dc 0.7 ac 1\n\
         m1 out g 0 0 n45 w=1u l=45n nf=2 ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {models}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e3, 1.461966e0, 3.123483e0),
        (3.333340e8, 2.421919e-4, 1.569688e0),
        (6.666670e8, 1.210961e-4, 1.568332e0),
        (1.000000e9, 8.073078e-5, 1.567031e0),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, mag_ref, phase_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let mag = v.norm();
        let phase = v.arg();
        let mag_rel = (mag - mag_ref).abs() / mag_ref.abs().max(1e-30);
        assert!(
            mag_rel < 2e-5,
            "RGATEMOD=1 AC magnitude at {freq:.3e} Hz: rspice={mag:.9e} ngspice={mag_ref:.9e} rel={mag_rel:.3e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-5,
            "RGATEMOD=1 AC phase at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
    }
}

#[test]
fn rgatemod2_common_source_ac_matches_xyce710_and_ngspice46() {
    // Xyce 7.10 reference for the reduced BSIM4 common-source gain stage.
    // ngspice-46 agrees at these points within 3.3e-4 magnitude relative and
    // 8.3e-4 rad phase, so Xyce is the primary oracle and ngspice is the
    // compatibility cross-check.
    let deck = rgatemod2_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let xyce_reference: &[(f64, f64, f64, f64, f64)] = &[
        (
            1.0e6,
            -3.579_290_770e-4,
            3.641_152_210e-2,
            -28.774_803_677,
            1.580_626_115,
        ),
        (
            1.0e8,
            1.761_420_470e-7,
            3.641_503_600e-4,
            -68.774_384_111,
            1.570_312_620,
        ),
        (
            1.0e10,
            2.116_279_810e-7,
            3.633_171_810e-6,
            -108.779_570_938,
            1.512_613_248,
        ),
        (
            1.0e11,
            1.835_105_830e-7,
            2.918_960_420e-7,
            -129.248_932_735,
            1.009_551_708,
        ),
    ];
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (
            1.0e6,
            -3.579_287_294e-4,
            3.641_152_209e-2,
            -28.774_803_681,
            1.580_626_105,
        ),
        (
            1.0e8,
            1.764_896_756e-7,
            3.641_503_601e-4,
            -68.774_384_105,
            1.570_311_665,
        ),
        (
            1.0e10,
            2.119_734_788e-7,
            3.633_137_976e-6,
            -108.779_603_561,
            1.512_517_933,
        ),
        (
            1.0e11,
            1.836_918_535e-7,
            2.916_493_122e-7,
            -129.251_761_810,
            1.008_725_479,
        ),
    ];
    let freqs: Vec<f64> = xyce_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for (
        ((freq, real_ref, imag_ref, db_ref, phase_ref), (_, ng_real, ng_imag, ng_db, ng_phase)),
        result,
    ) in xyce_reference
        .iter()
        .zip(ngspice_reference.iter())
        .zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_err = (v.re - real_ref).abs();
        let imag_err = (v.im - imag_ref).abs();
        assert!(
            real_err <= 5e-10 + 3e-3 * real_ref.abs(),
            "RGATEMOD=2 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} xyce={real_ref:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-10 + 3e-3 * imag_ref.abs(),
            "RGATEMOD=2 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} xyce={imag_ref:.9e} abs_err={imag_err:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 5e-3,
            "RGATEMOD=2 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} xyce={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 1e-3,
            "RGATEMOD=2 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} xyce={phase_ref:.9e}"
        );
        let ng_real_err = (v.re - ng_real).abs();
        let ng_imag_err = (v.im - ng_imag).abs();
        assert!(
            ng_real_err <= 5e-10 + 3e-3 * ng_real.abs(),
            "RGATEMOD=2 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={ng_real:.9e} abs_err={ng_real_err:.3e}",
            v.re
        );
        assert!(
            ng_imag_err <= 5e-10 + 3e-3 * ng_imag.abs(),
            "RGATEMOD=2 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={ng_imag:.9e} abs_err={ng_imag_err:.3e}",
            v.im
        );
        assert!(
            (db - ng_db).abs() < 5e-3,
            "RGATEMOD=2 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={ng_db:.9e}"
        );
        assert!(
            (phase - ng_phase).abs() < 1e-3,
            "RGATEMOD=2 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={ng_phase:.9e}"
        );
    }
}

#[test]
fn rgatemod3_common_source_ac_matches_xyce710_and_ngspice46() {
    // Xyce 7.10 and ngspice-46 references for the same reduced BSIM4 gain
    // stage as the RGATEMOD=2 oracle, with RGATEMOD=3 adding a middle-gate
    // node and keeping NQS disabled. Xyce maps VERSION=4.8 to 4.8.2 for this
    // open-model path, so ngspice remains the tighter oracle and Xyce is a
    // compatibility cross-check.
    let deck = rgatemod3_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let xyce_reference: &[(f64, f64, f64, f64, f64)] = &[
        (
            1.0e6,
            -3.579_282_444e-4,
            3.641_152_222e-2,
            -28.774_803_650,
            1.580_626_092,
        ),
        (
            1.0e8,
            1.769_972_938e-7,
            3.641_503_622e-4,
            -68.774_384_049,
            1.570_310_271,
        ),
        (
            1.0e10,
            2.124_876_315e-7,
            3.633_215_672e-6,
            -108.779_346_886,
            1.512_378_144,
        ),
        (
            1.0e11,
            1.846_866_368e-7,
            2.921_203_443e-7,
            -129.228_376_632,
            1.007_016_284,
        ),
    ];
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (
            1.0e6,
            -3.579_278_895_132_175e-4,
            3.641_152_337_555_114e-2,
            -28.774_803_375_960_15,
            1.580_626_081_996_169,
        ),
        (
            1.0e8,
            1.773_449_299_200_657e-7,
            3.641_503_733_762_768e-4,
            -68.774_383_778_184_63,
            1.570_309_316_678_022,
        ),
        (
            1.0e10,
            2.128_331_527_030_522e-7,
            3.633_181_986_400_360e-6,
            -108.779_378_958_419_4,
            1.512_282_826_357_814,
        ),
        (
            1.0e11,
            1.848_686_798_964_113e-7,
            2.918_728_680_715_895e-7,
            -129.231_186_318_753_2,
            1.006_188_143_044_513,
        ),
    ];
    let freqs: Vec<f64> = xyce_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for (
        ((freq, xyce_real, xyce_imag, xyce_db, xyce_phase), (_, ng_real, ng_imag, ng_db, ng_phase)),
        result,
    ) in xyce_reference
        .iter()
        .zip(ngspice_reference.iter())
        .zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let xyce_real_err = (v.re - xyce_real).abs();
        let xyce_imag_err = (v.im - xyce_imag).abs();
        assert!(
            xyce_real_err <= 1e-9 + 6e-3 * xyce_real.abs(),
            "RGATEMOD=3 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} xyce={xyce_real:.9e} abs_err={xyce_real_err:.3e}",
            v.re
        );
        assert!(
            xyce_imag_err <= 1e-9 + 6e-3 * xyce_imag.abs(),
            "RGATEMOD=3 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} xyce={xyce_imag:.9e} abs_err={xyce_imag_err:.3e}",
            v.im
        );
        assert!(
            (db - xyce_db).abs() < 1e-2,
            "RGATEMOD=3 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} xyce={xyce_db:.9e}"
        );
        assert!(
            (phase - xyce_phase).abs() < 2e-3,
            "RGATEMOD=3 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} xyce={xyce_phase:.9e}"
        );

        let real_err = (v.re - ng_real).abs();
        let imag_err = (v.im - ng_imag).abs();
        assert!(
            real_err <= 5e-10 + 3e-3 * ng_real.abs(),
            "RGATEMOD=3 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={ng_real:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-10 + 3e-3 * ng_imag.abs(),
            "RGATEMOD=3 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={ng_imag:.9e} abs_err={imag_err:.3e}",
            v.im
        );
        assert!(
            (db - ng_db).abs() < 5e-3,
            "RGATEMOD=3 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={ng_db:.9e}"
        );
        assert!(
            (phase - ng_phase).abs() < 1e-3,
            "RGATEMOD=3 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={ng_phase:.9e}"
        );
    }
}

#[test]
fn acnqsmod1_common_source_ac_matches_ngspice46() {
    // ngspice-46 reference for the same reduced BSIM4 gain stage as the
    // RGATEMOD oracle decks, with `ACNQSMOD=1` and `TRNQSMOD=0`. The local
    // Xyce 7.10 binary accepts the selector but produced the same AC output
    // as `ACNQSMOD=0` for this deck, so Xyce is not used as the AC-NQS
    // physics oracle for this slice.
    let deck = acnqsmod1_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (1.0e6, -3.702_25, 4.295_778e-5, 11.369_31, 3.141_581),
        (1.0e7, -3.702_25, 4.295_778e-4, 11.369_31, 3.141_477),
        (1.0e8, -3.702_24, 4.295_774e-3, 11.369_31, 3.140_432),
        (1.0e9, -3.701_84, 4.295_381e-2, 11.368_93, 3.129_990),
        (1.0e10, -3.661_32, 4.256_449e-1, 11.331_05, 3.025_858),
        (1.0e11, -1.554_58, 2.232_057, 8.691_610, 2.179_155),
    ];
    let freqs: Vec<f64> = ngspice_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, real_ref, imag_ref, db_ref, phase_ref), result) in
        ngspice_reference.iter().zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_err = (v.re - real_ref).abs();
        let imag_err = (v.im - imag_ref).abs();
        assert!(
            real_err <= 5e-5 + 5e-4 * real_ref.abs(),
            "ACNQSMOD=1 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-7 + 5e-4 * imag_ref.abs(),
            "ACNQSMOD=1 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} abs_err={imag_err:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 2e-3,
            "ACNQSMOD=1 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-4,
            "ACNQSMOD=1 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
    }
}

#[test]
fn acnqsmod1_trnqsmod1_common_source_ac_matches_ngspice46() {
    // ngspice-46 allows both NQS selectors on the same BSIM4 card. Its AC
    // load isolates the transient q node with a unit diagonal, so the AC
    // output matches the ACNQSMOD-only physics oracle for this deck.
    let deck = acnqsmod1_trnqsmod1_common_source_ac_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let circuit = engine()
        .build_circuit(&netlist)
        .expect("ACNQSMOD=1 with TRNQSMOD=1 deck builds natively");
    assert!(
        circuit.get_node_by_name("m1.__charge").is_some(),
        "TRNQSMOD=1 must still allocate the hidden charge-deficit node"
    );

    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (1.0e6, -3.702_25, 4.295_778e-5, 11.369_31, 3.141_581),
        (1.0e7, -3.702_25, 4.295_778e-4, 11.369_31, 3.141_477),
        (1.0e8, -3.702_24, 4.295_774e-3, 11.369_31, 3.140_432),
        (1.0e9, -3.701_84, 4.295_381e-2, 11.368_93, 3.129_990),
        (1.0e10, -3.661_32, 4.256_449e-1, 11.331_05, 3.025_858),
        (1.0e11, -1.554_58, 2.232_057, 8.691_610, 2.179_155),
    ];
    let freqs: Vec<f64> = ngspice_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, real_ref, imag_ref, db_ref, phase_ref), result) in
        ngspice_reference.iter().zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_err = (v.re - real_ref).abs();
        let imag_err = (v.im - imag_ref).abs();
        assert!(
            real_err <= 5e-5 + 5e-4 * real_ref.abs(),
            "ACNQSMOD=1 TRNQSMOD=1 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-7 + 5e-4 * imag_ref.abs(),
            "ACNQSMOD=1 TRNQSMOD=1 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} abs_err={imag_err:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 2e-3,
            "ACNQSMOD=1 TRNQSMOD=1 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-4,
            "ACNQSMOD=1 TRNQSMOD=1 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
    }
}

#[test]
fn acnqsmod1_rdsmod1_common_source_ac_matches_ngspice46() {
    // RDSMOD=1 forces intrinsic D'/S' nodes. The AC-NQS correction belongs on
    // those intrinsic rows while the external nonlinear resistance remains in
    // the real small-signal Jacobian. ngspice-46 is the physics oracle here.
    let deck = acnqsmod1_rdsmod1_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (1.0e6, -2.870_21, 3.657_309e-5, 9.158_286, 3.141_580),
        (1.0e7, -2.870_21, 3.657_309e-4, 9.158_286, 3.141_465),
        (1.0e8, -2.870_21, 3.657_305e-3, 9.158_282, 3.140_318),
        (1.0e9, -2.869_84, 3.656_919e-2, 9.157_847, 3.128_851),
        (1.0e10, -2.832_79, 3.618_744e-1, 9.114_597, 3.014_536),
        (1.0e11, -1.036_30, 1.766_660, 6.227_325, 2.101_295),
    ];
    let qs_reference: &[(f64, f64)] = &[
        (-2.870_21, 3.648_074e-5),
        (-2.870_21, 3.648_074e-4),
        (-2.870_21, 3.648_070e-3),
        (-2.869_84, 3.647_684e-2),
        (-2.832_80, 3.609_426e-1),
        (-1.041_42, 1.758_038),
    ];
    let freqs: Vec<f64> = ngspice_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for (((freq, real_ref, imag_ref, db_ref, phase_ref), (qs_re, qs_im)), result) in
        ngspice_reference.iter().zip(qs_reference).zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_err = (v.re - real_ref).abs();
        let imag_err = (v.im - imag_ref).abs();
        assert!(
            real_err <= 5e-5 + 5e-4 * real_ref.abs(),
            "ACNQSMOD=1 RDSMOD=1 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-7 + 5e-4 * imag_ref.abs(),
            "ACNQSMOD=1 RDSMOD=1 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} abs_err={imag_err:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 2e-3,
            "ACNQSMOD=1 RDSMOD=1 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-4,
            "ACNQSMOD=1 RDSMOD=1 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
        if *freq >= 1.0e11 {
            let qs_delta = ((v.re - qs_re).powi(2) + (v.im - qs_im).powi(2)).sqrt();
            assert!(
                qs_delta > 5.0e-3,
                "ACNQSMOD=1 RDSMOD=1 must not silently degrade to QS at {freq:.3e} Hz: delta={qs_delta:.3e}"
            );
        }
    }
}

#[test]
fn acnqsmod1_rdsmod1_rgatemod2_common_source_ac_matches_ngspice46() {
    // The native AC-NQS delta must compose with both intrinsic D'/S' nodes
    // from RDSMOD=1 and the bias-dependent RGATEMOD=2 gate branch.
    let deck = acnqsmod1_rdsmod1_rgatemod2_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (1.0e6, 9.599_247e-3, 3.189_609e-3, -3.990_04e1, 3.207_995e-1),
        (1.0e7, 1.063_020e-4, 3.499_864e-6, -7.946_45e1, 3.291_191e-2),
        (1.0e8, 1.306_723e-6, 2.105_430e-9, -1.176_76e2, 1.611_227e-3),
        (1.0e9, 2.556_080e-7, -3.726_09e-9, -1.318_48e2, -1.457_63e-2),
        (
            1.0e10,
            2.397_881e-7,
            -3.545_59e-8,
            -1.323_10e2,
            -1.468_00e-1,
        ),
        (
            1.0e11,
            7.860_173e-8,
            -1.103_77e-7,
            -1.373_61e2,
            -9.519_84e-1,
        ),
    ];
    let freqs: Vec<f64> = ngspice_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, real_ref, imag_ref, db_ref, phase_ref), result) in
        ngspice_reference.iter().zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_err = (v.re - real_ref).abs();
        let imag_err = (v.im - imag_ref).abs();
        assert!(
            real_err <= 5e-10 + 2e-3 * real_ref.abs(),
            "ACNQSMOD=1 RDSMOD=1 RGATEMOD=2 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-10 + 2e-3 * imag_ref.abs(),
            "ACNQSMOD=1 RDSMOD=1 RGATEMOD=2 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} abs_err={imag_err:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 2e-2,
            "ACNQSMOD=1 RDSMOD=1 RGATEMOD=2 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-3,
            "ACNQSMOD=1 RDSMOD=1 RGATEMOD=2 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
    }
}

#[test]
fn acnqsmod1_rgatemod1_common_source_ac_matches_ngspice46() {
    // `RGATEMOD=1` is a linear GE-GP resistor with the AC-NQS intrinsic
    // rows stamped on GP in ngspice-46 `b4acld.c`. RSpice lowers the same
    // resistor in the builder, so ngspice is the tight physics oracle here.
    let deck = acnqsmod1_rgatemod1_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (
            1.0e6,
            -3.579_280_95e-4,
            3.641_152_23e-2,
            -28.774_803_6,
            1.580_626_09,
        ),
        (
            1.0e7,
            -3.368_808_27e-6,
            3.641_500_96e-3,
            -48.774_387_7,
            1.571_721_44,
        ),
        (
            1.0e8,
            1.771_275_46e-7,
            3.641_503_63e-4,
            -68.774_384_0,
            1.570_309_91,
        ),
        (
            1.0e9,
            2.125_837_51e-7,
            3.641_421_67e-5,
            -88.774_432_6,
            1.564_958_46,
        ),
        (
            1.0e10,
            2.126_199_85e-7,
            3.633_234_51e-6,
            -108.779_284,
            1.512_342_14,
        ),
        (
            1.0e11,
            1.849_686_27e-7,
            2.922_043_24e-7,
            -129.222_806,
            1.006_456_84,
        ),
    ];
    let freqs: Vec<f64> = ngspice_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, real_ref, imag_ref, db_ref, phase_ref), result) in
        ngspice_reference.iter().zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_err = (v.re - real_ref).abs();
        let imag_err = (v.im - imag_ref).abs();
        assert!(
            real_err <= 5e-10 + 3e-3 * real_ref.abs(),
            "ACNQSMOD=1 RGATEMOD=1 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-10 + 3e-3 * imag_ref.abs(),
            "ACNQSMOD=1 RGATEMOD=1 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} abs_err={imag_err:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 2e-2,
            "ACNQSMOD=1 RGATEMOD=1 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-3,
            "ACNQSMOD=1 RGATEMOD=1 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
    }
}

#[test]
fn acnqsmod1_rgatemod2_common_source_ac_matches_ngspice46() {
    // ngspice-46 reference from the same common-source deck shape as the
    // RGATEMOD=2 oracle, with charge-deficit AC-NQS enabled. ngspice warns
    // that Rg and charge-deficit NQS are both selected, but still evaluates
    // this topology. Xyce 7.10 accepts the same deck but produces the
    // ACNQSMOD=0 RGATEMOD=2 output, so ngspice is the numerical oracle.
    let deck = acnqsmod1_rgatemod2_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (1.0e6, 7.791_500e-3, 1.938_628e-3, -41.906_7, 2.438_614e-1),
        (1.0e7, 8.273_123e-5, 2.040_029e-6, -81.644_0, 2.465_352e-2),
        (1.0e8, 1.076_789e-6, 9.830_259e-10, -119.357, 9.129_233e-4),
        (1.0e9, 2.597_115e-7, -3.651_06e-9, -131.709, -1.405_72e-2),
        (1.0e10, 2.465_890e-7, -3.512_57e-8, -132.073, -1.414_94e-1),
        (1.0e11, 8.301_734e-8, -1.182_70e-7, -136.803, -9.587_77e-1),
    ];
    let freqs: Vec<f64> = ngspice_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, real_ref, imag_ref, db_ref, phase_ref), result) in
        ngspice_reference.iter().zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_err = (v.re - real_ref).abs();
        let imag_err = (v.im - imag_ref).abs();
        assert!(
            real_err <= 5e-10 + 2e-3 * real_ref.abs(),
            "ACNQSMOD=1 RGATEMOD=2 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-10 + 2e-3 * imag_ref.abs(),
            "ACNQSMOD=1 RGATEMOD=2 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} abs_err={imag_err:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 2e-2,
            "ACNQSMOD=1 RGATEMOD=2 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-3,
            "ACNQSMOD=1 RGATEMOD=2 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
    }
}

#[test]
fn acnqsmod1_rgatemod3_common_source_ac_matches_ngspice46() {
    // ngspice-46 applies charge-deficit AC-NQS to the intrinsic gate-prime
    // rows while keeping RGATEMOD=3 overlap charge on the middle-gate node.
    // Xyce 7.10 accepts ACNQSMOD but does not expose the AC-NQS delta for
    // this reduced deck, so ngspice is the numerical oracle.
    let deck = acnqsmod1_rgatemod3_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (
            1.0e6,
            -3.579_272_44e-4,
            3.641_152_34e-2,
            -28.774_803_4,
            1.580_626_06,
        ),
        (
            1.0e7,
            -3.367_953_06e-6,
            3.641_501_07e-3,
            -48.774_387_4,
            1.571_721_21,
        ),
        (
            1.0e8,
            1.779_828_00e-7,
            3.641_503_74e-4,
            -68.774_383_7,
            1.570_307_57,
        ),
        (
            1.0e9,
            2.134_390_50e-7,
            3.641_422_22e-5,
            -88.774_430_0,
            1.564_934_97,
        ),
        (
            1.0e10,
            2.134_796_80e-7,
            3.633_278_22e-6,
            -108.779_059,
            1.512_107_04,
        ),
        (
            1.0e11,
            1.861_488_84e-7,
            2.924_245_47e-7,
            -129.202_273,
            1.003_919_78,
        ),
    ];
    let freqs: Vec<f64> = ngspice_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, real_ref, imag_ref, db_ref, phase_ref), result) in
        ngspice_reference.iter().zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_err = (v.re - real_ref).abs();
        let imag_err = (v.im - imag_ref).abs();
        assert!(
            real_err <= 5e-10 + 3e-3 * real_ref.abs(),
            "ACNQSMOD=1 RGATEMOD=3 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-10 + 3e-3 * imag_ref.abs(),
            "ACNQSMOD=1 RGATEMOD=3 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} abs_err={imag_err:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 2e-2,
            "ACNQSMOD=1 RGATEMOD=3 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-3,
            "ACNQSMOD=1 RGATEMOD=3 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
    }
}

#[test]
fn acnqsmod1_is_rejected_for_pole_zero_until_charge_deficit_state_exists() {
    let deck = acnqsmod1_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let circuit = engine().build_circuit(&netlist).expect("circuit builds");
    let input = circuit.get_node_by_name("in").expect("input node");
    let output = circuit.get_node_by_name("out").expect("output node");
    let err = engine()
        .run_pz(&netlist, input, output)
        .expect_err("ACNQSMOD=1 is rational and must not use G+sC PZ extraction");
    let message = err.to_string();
    assert!(
        message.contains("Pole-zero")
            && message.contains("ACNQSMOD=1")
            && message.contains("charge-deficit"),
        "typed PZ rejection should name ACNQSMOD=1 and the missing state: {message}"
    );
}

#[test]
fn acnqsmod1_rbodymod1_ac_matches_ngspice46() {
    // AC-NQS belongs on the intrinsic D'/G/S'/body-prime rows while the
    // RBODYMOD=1 junction capacitances remain routed through dbody/sbody.
    let deck = acnqsmod1_rbodymod1_high_resistance_common_source_ac_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (1.0e6, -6.305_46e-2, 1.466_701e-6, -2.400_57e1, 3.141_569),
        (1.0e7, -6.305_46e-2, 1.466_701e-5, -2.400_57e1, 3.141_360),
        (1.0e8, -6.305_44e-2, 1.466_666e-4, -2.400_57e1, 3.139_267),
        (1.0e9, -6.303_52e-2, 1.463_333e-3, -2.400_60e1, 3.118_382),
        (1.0e10, -6.246_49e-2, 1.390_416e-2, -2.387_73e1, 2.922_572),
        (1.0e11, -4.716_48e-2, 1.345_290e-1, -1.692_02e1, 1.907_999),
    ];
    let qs_reference: &[(f64, f64)] = &[
        (-6.305_464_522_572_059e-2, 1.466_560_146_708_436e-6),
        (-6.305_464_323_451_500e-2, 1.466_559_800_179_500e-5),
        (-6.305_444_418_162_238e-2, 1.466_525_160_795_036e-4),
        (-6.303_518_930_650_295e-2, 1.463_190_960_354_446e-3),
        (-6.245_988_548_165_470e-2, 1.390_243_926_406_921e-2),
        (-4.670_179_795_137_225e-2, 1.343_934_147_655_109e-1),
    ];
    let freqs: Vec<f64> = ngspice_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for (((freq, real_ref, imag_ref, db_ref, phase_ref), (qs_re, qs_im)), result) in
        ngspice_reference.iter().zip(qs_reference).zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_rel = (v.re - real_ref).abs() / real_ref.abs().max(1e-30);
        let imag_rel = (v.im - imag_ref).abs() / imag_ref.abs().max(1e-30);
        assert!(
            real_rel < 1e-4,
            "ACNQSMOD=1 RBODYMOD=1 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} rel={real_rel:.3e}",
            v.re
        );
        assert!(
            imag_rel < 1e-4,
            "ACNQSMOD=1 RBODYMOD=1 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} rel={imag_rel:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 2e-3,
            "ACNQSMOD=1 RBODYMOD=1 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-4,
            "ACNQSMOD=1 RBODYMOD=1 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
        if *freq >= 1.0e11 {
            let qs_delta = ((v.re - qs_re).powi(2) + (v.im - qs_im).powi(2)).sqrt();
            assert!(
                qs_delta > 1.0e-4,
                "ACNQSMOD=1 RBODYMOD=1 must not silently degrade to QS at {freq:.3e} Hz: delta={qs_delta:.3e}"
            );
        }
    }
}

#[test]
fn acnqsmod1_rbodymod2_ac_matches_ngspice46() {
    // Same AC-NQS/RBODY composition as RBODYMOD=1, with ngspice's
    // geometry-scaled substrate-resistance selector.
    let deck = acnqsmod1_rbodymod2_common_source_ac_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let ngspice_reference: &[(f64, f64, f64, f64, f64)] = &[
        (1.0e6, -6.305_46e-2, 1.461_074e-6, -2.400_57e1, 3.141_569),
        (1.0e7, -6.305_46e-2, 1.461_074e-5, -2.400_57e1, 3.141_361),
        (1.0e8, -6.305_46e-2, 1.461_074e-4, -2.400_56e1, 3.139_276),
        (1.0e9, -6.305_14e-2, 1.461_067e-3, -2.400_38e1, 3.118_424),
        (1.0e10, -6.272_65e-2, 1.460_308e-2, -2.382_18e1, 2.912_861),
        (1.0e11, -3.191_96e-2, 1.388_416e-1, -1.692_59e1, 1.796_769),
    ];
    let qs_reference: &[(f64, f64)] = &[
        (-6.305_46e-2, 1.460_933e-6),
        (-6.305_46e-2, 1.460_933e-5),
        (-6.305_46e-2, 1.460_933e-4),
        (-6.305_13e-2, 1.460_925e-3),
        (-6.272_11e-2, 1.460_140e-2),
        (-3.147_60e-2, 1.385_873e-1),
    ];
    let freqs: Vec<f64> = ngspice_reference.iter().map(|&(f, _, _, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for (((freq, real_ref, imag_ref, db_ref, phase_ref), (qs_re, qs_im)), result) in
        ngspice_reference.iter().zip(qs_reference).zip(&results)
    {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let phase = v.arg();
        let real_rel = (v.re - real_ref).abs() / real_ref.abs().max(1e-30);
        let imag_rel = (v.im - imag_ref).abs() / imag_ref.abs().max(1e-30);
        assert!(
            real_rel < 1e-4,
            "ACNQSMOD=1 RBODYMOD=2 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} rel={real_rel:.3e}",
            v.re
        );
        assert!(
            imag_rel < 1e-4,
            "ACNQSMOD=1 RBODYMOD=2 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} rel={imag_rel:.3e}",
            v.im
        );
        assert!(
            (db - db_ref).abs() < 2e-3,
            "ACNQSMOD=1 RBODYMOD=2 AC dB(vout) at {freq:.3e} Hz: rspice={db:.9e} ngspice={db_ref:.9e}"
        );
        assert!(
            (phase - phase_ref).abs() < 2e-4,
            "ACNQSMOD=1 RBODYMOD=2 AC phase(vout) at {freq:.3e} Hz: rspice={phase:.9e} ngspice={phase_ref:.9e}"
        );
        if *freq >= 1.0e11 {
            let qs_delta = ((v.re - qs_re).powi(2) + (v.im - qs_im).powi(2)).sqrt();
            assert!(
                qs_delta > 1.0e-4,
                "ACNQSMOD=1 RBODYMOD=2 must not silently degrade to QS at {freq:.3e} Hz: delta={qs_delta:.3e}"
            );
        }
    }
}

#[test]
fn rbodymod1_ac_junction_caps_match_ngspice46() {
    // RBODYMOD=1 routes the drain/source junction capacitances to the
    // dbody/sbody nodes in ngspice's AC load. Large substrate resistors make
    // that internal network visible in a common-source gain stage.
    let deck = rbodymod1_high_resistance_common_source_ac_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, -6.305464522572059e-2, 1.466560146708436e-6),
        (1.000000e7, -6.305464323451500e-2, 1.466559800179500e-5),
        (1.000000e8, -6.305444418162238e-2, 1.466525160795036e-4),
        (1.000000e9, -6.303518930650295e-2, 1.463190960354446e-3),
        (1.000000e10, -6.245988548165470e-2, 1.390243926406921e-2),
        (1.000000e11, -4.670179795137225e-2, 1.343934147655109e-1),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, real_ref, imag_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let real_rel = (v.re - real_ref).abs() / real_ref.abs().max(1e-30);
        let imag_rel = (v.im - imag_ref).abs() / imag_ref.abs().max(1e-30);
        assert!(
            real_rel < 2e-5,
            "RBODYMOD=1 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} rel={real_rel:.3e}",
            v.re
        );
        assert!(
            imag_rel < 2e-5,
            "RBODYMOD=1 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} rel={imag_rel:.3e}",
            v.im
        );
    }
}

#[test]
fn rbodymod1_ac_junction_caps_match_xyce710_compatibility() {
    // Xyce 7.10 accepts the same native deck as BSIM4, maps VERSION=4.8 to
    // 4.8.2, and exercises the same RBODYMOD=1 AC topology. This is a
    // compatibility pin rather than a local Xyce regression-suite gold file.
    let deck = rbodymod1_high_resistance_common_source_ac_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, -6.30546449e-2, 1.46650205e-6),
        (1.000000e7, -6.30546429e-2, 1.46650171e-5),
        (1.000000e8, -6.30544441e-2, 1.46646713e-4),
        (1.000000e9, -6.30352082e-2, 1.46313840e-3),
        (1.000000e10, -6.24599322e-2, 1.39025111e-2),
        (1.000000e11, -4.67003057e-2, 1.34390845e-1),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, real_ref, imag_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let real_rel = (v.re - real_ref).abs() / real_ref.abs().max(1e-30);
        let imag_rel = (v.im - imag_ref).abs() / imag_ref.abs().max(1e-30);
        assert!(
            real_rel < 1e-4,
            "RBODYMOD=1 AC real(vout) vs Xyce at {freq:.3e} Hz: rspice={:.9e} xyce={real_ref:.9e} rel={real_rel:.3e}",
            v.re
        );
        assert!(
            imag_rel < 1e-4,
            "RBODYMOD=1 AC imag(vout) vs Xyce at {freq:.3e} Hz: rspice={:.9e} xyce={imag_ref:.9e} rel={imag_rel:.3e}",
            v.im
        );
    }
}

#[test]
fn rbodymod1_transient_drain_current_matches_xyce710_and_ngspice46() {
    // Xyce 7.10 reference for this one-device transient deck; ngspice-46
    // agrees within 0.13% at the same sample points. The high substrate
    // resistances make the drain-body junction charge routing visible in the
    // externally observable current through VD.
    let deck = format!(
        "* bsim4 rbodymod=1 transient drain-current oracle\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vd d 0 pulse(0.2 0.8 20p 5p 5p 200p 500p)\n\
         vg g 0 dc 0.5\n\
         vb b 0 dc 0.8\n\
         m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .tran 5p 100p\n\
         .end\n",
        models45_rbodymod1_high_resistance()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // The oracle points sit on the 5 ps drain-voltage edge; use a tighter
    // internal step so interpolation checks the RBODY charge split rather
    // than accepted-step placement.
    let result = engine()
        .run_tran(&netlist, 100.0e-12, 0.1e-12)
        .expect("RBODYMOD=1 transient runs");
    let current = result
        .try_branch_current_waveform_named("vd")
        .unwrap_or_else(|| panic!("missing VD branch in {:?}", result.branch_names));
    let reference: &[(f64, f64)] = &[
        (20.5e-12, -4.034_928_860_085e-4),
        (21.0e-12, -4.118_570_515_397e-4),
        (22.5e-12, -4.256_686_206_512e-4),
        (25.0e-12, -4.174_273_300_000e-4),
        (30.0e-12, -3.423_536_971_046e-4),
        (50.0e-12, -3.642_530_963_388e-4),
        (80.0e-12, -3.943_129_587_951e-4),
        (100.0e-12, -4.001_519_370_000e-4),
    ];
    for &(time, expected) in reference {
        let got = interp_waveform(&result.time, current, time);
        let rel = (got - expected).abs() / expected.abs().max(1e-30);
        assert!(
            rel < 5e-3,
            "RBODYMOD=1 transient I(VD) at {time:.3e}s: rspice={got:.9e} xyce={expected:.9e} rel={rel:.3e}"
        );
    }
}

#[test]
fn trnqsmod1_common_source_transient_matches_ngspice46() {
    // ngspice-46 console reference for the canonical BSIM4 charge-deficit
    // transient NQS topology (`TRNQSMOD=1`, `ACNQSMOD=0`, no RDS/RBODY/RGATE
    // optional networks). The local Xyce 7.10 regression tree has BSIM4
    // coverage, but no matching `TRNQSMOD=1` transient oracle; Xyce model
    // cards found there keep `TRNQSMOD=0`.
    let deck = trnqsmod_common_source_deck(1);
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 80.0e-12, 0.1e-12)
        .expect("TRNQSMOD=1 transient runs natively");
    let vout = result
        .try_voltage_waveform_named("out")
        .expect("out waveform");

    let qs_deck = trnqsmod_common_source_deck(0);
    let qs_netlist = Netlist::parse(&qs_deck).expect("QS deck parses");
    let qs_result = engine()
        .run_tran(&qs_netlist, 80.0e-12, 0.1e-12)
        .expect("TRNQSMOD=0 transient runs");
    let qs_vout = qs_result
        .try_voltage_waveform_named("out")
        .expect("QS out waveform");

    let reference: &[(f64, f64, f64)] = &[
        (20.5e-12, 1.073_368_075_679, -7.013_011e-4),
        (21.0e-12, 9.761_701_272_506e-1, -4.560_308e-4),
        (21.5e-12, 7.565_004_829_099e-1, -2.372_303e-4),
        (22.0e-12, 4.803_026_630_000e-1, -2.624_930e-4),
        (22.5e-12, 1.833_046_107_846e-1, -1.031_226e-3),
        (23.0e-12, 9.233_953_871_709e-2, -2.188_013e-3),
        (24.0e-12, 7.500_728_937_211e-2, -2.924_351e-4),
        (25.0e-12, 7.449_882_471_507e-2, -1.778_145e-5),
    ];
    let mut max_qs_delta: f64 = 0.0;
    for &(time, expected, expected_delta_vs_qs) in reference {
        let got = interp_waveform(&result.time, vout, time);
        let abs_err = (got - expected).abs();
        assert!(
            abs_err < 7e-4,
            "TRNQSMOD=1 transient v(out) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} abs_err={abs_err:.3e}"
        );

        let qs = interp_waveform(&qs_result.time, qs_vout, time);
        let qs_delta = got - qs;
        max_qs_delta = max_qs_delta.max(qs_delta.abs());
        let delta_err = (qs_delta - expected_delta_vs_qs).abs();
        assert!(
            delta_err < 8e-4,
            "TRNQSMOD=1 delta vs QS at {time:.3e}s: rspice={qs_delta:.9e} ngspice={expected_delta_vs_qs:.9e} abs_err={delta_err:.3e}"
        );
    }
    assert!(
        max_qs_delta > 1e-3,
        "TRNQSMOD=1 must not silently degrade to QS; max |delta v(out)|={max_qs_delta:.3e}"
    );
}

#[test]
fn trnqsmod1_rdsmod1_common_source_transient_matches_ngspice46() {
    // ngspice-46 release oracle from the exact deck with
    // `ngspice_con.exe -b`, `set numdgt=15`, and `wrdata v(out)`.
    // The local Xyce 7.10 regression tree covers BSIM4 RDS/RGATE/RBODY
    // topologies, but did not contain a matching TRNQSMOD=1 transient NQS
    // card, so ngspice is the physics oracle for this transient slice.
    let deck = trnqsmod_rdsmod1_common_source_deck(1);
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 80.0e-12, 0.1e-12)
        .expect("TRNQSMOD=1 with RDSMOD=1 transient runs natively");
    let vout = result
        .try_voltage_waveform_named("out")
        .expect("out waveform");

    let qs_deck = trnqsmod_rdsmod1_common_source_deck(0);
    let qs_netlist = Netlist::parse(&qs_deck).expect("QS deck parses");
    let qs_result = engine()
        .run_tran(&qs_netlist, 80.0e-12, 0.1e-12)
        .expect("TRNQSMOD=0 with RDSMOD=1 transient runs");
    let qs_vout = qs_result
        .try_voltage_waveform_named("out")
        .expect("QS out waveform");

    let reference: &[(f64, f64, f64)] = &[
        (20.5e-12, 1.072_418_965_074_154, -4.328_208_966_051_239e-4),
        (
            21.0e-12,
            9.787_524_399_343_329e-1,
            -8.295_258_697_810_226e-5,
        ),
        (21.5e-12, 7.859_162_190_406_547e-1, 4.794_026_381_316_208e-5),
        (
            22.0e-12,
            5.682_037_393_571_626e-1,
            -2.978_275_706_522_204e-5,
        ),
        (
            22.5e-12,
            3.339_543_691_624_751e-1,
            -4.327_014_697_546_594e-4,
        ),
        (
            23.0e-12,
            2.300_838_893_394_644e-1,
            -1.001_830_842_124_851e-3,
        ),
        (
            24.0e-12,
            1.775_298_364_155_602e-1,
            -3.005_354_497_151_447e-4,
        ),
        (
            25.0e-12,
            1.664_264_922_468_640e-1,
            -8.927_298_134_916_994e-5,
        ),
    ];
    let mut max_qs_delta: f64 = 0.0;
    for &(time, expected, expected_delta_vs_qs) in reference {
        let got = interp_waveform(&result.time, vout, time);
        let abs_err = (got - expected).abs();
        assert!(
            abs_err < 9e-4,
            "TRNQSMOD=1 RDSMOD=1 transient v(out) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} abs_err={abs_err:.3e}"
        );

        let qs = interp_waveform(&qs_result.time, qs_vout, time);
        let qs_delta = got - qs;
        max_qs_delta = max_qs_delta.max(qs_delta.abs());
        let delta_err = (qs_delta - expected_delta_vs_qs).abs();
        assert!(
            delta_err < 9e-4,
            "TRNQSMOD=1 RDSMOD=1 delta vs QS at {time:.3e}s: rspice={qs_delta:.9e} ngspice={expected_delta_vs_qs:.9e} abs_err={delta_err:.3e}"
        );
    }
    assert!(
        max_qs_delta > 8e-4,
        "TRNQSMOD=1 with RDSMOD=1 must not silently degrade to QS; max |delta v(out)|={max_qs_delta:.3e}"
    );
}

#[test]
fn trnqsmod1_rbodymod1_common_source_transient_matches_ngspice46() {
    // ngspice-46 release oracle from the exact deck with
    // `ngspice_con.exe -b`, `set numdgt=15`, and `wrdata v(out)`.
    // This composes charge-deficit transient NQS with the native RBODYMOD=1
    // substrate network; Xyce covers the body network topology, but not this
    // TRNQSMOD transient card.
    let deck = trnqsmod_rbodymod1_common_source_deck(1);
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let circuit = engine()
        .build_circuit(&netlist)
        .expect("TRNQSMOD=1 RBODYMOD=1 deck builds natively");
    assert!(
        circuit.get_node_by_name("m1.__body").is_some(),
        "RBODYMOD=1 must create the native body-prime node"
    );
    assert!(
        circuit.get_node_by_name("m1.__dbody").is_some(),
        "RBODYMOD=1 must create the native drain-body node"
    );
    assert!(
        circuit.get_node_by_name("m1.__sbody").is_some(),
        "RBODYMOD=1 must create the native source-body node"
    );
    assert!(
        circuit.get_node_by_name("m1.__charge").is_some(),
        "TRNQSMOD=1 must create the hidden charge-deficit node"
    );
    let result = engine()
        .run_tran(&netlist, 80.0e-12, 0.1e-12)
        .expect("TRNQSMOD=1 with RBODYMOD=1 transient runs natively");
    let vout = result
        .try_voltage_waveform_named("out")
        .expect("out waveform");

    let qs_deck = trnqsmod_rbodymod1_common_source_deck(0);
    let qs_netlist = Netlist::parse(&qs_deck).expect("QS deck parses");
    let qs_result = engine()
        .run_tran(&qs_netlist, 80.0e-12, 0.1e-12)
        .expect("TRNQSMOD=0 with RBODYMOD=1 transient runs");
    let qs_vout = qs_result
        .try_voltage_waveform_named("out")
        .expect("QS out waveform");

    let reference: &[(f64, f64, f64)] = &[
        (20.5e-12, 1.108_265_888_438_446, -2.825_467_054_738_184e-2),
        (
            21.0e-12,
            9.228_127_788_145_025e-1,
            -1.156_302_559_354_542e-1,
        ),
        (
            21.5e-12,
            4.940_745_371_717_650e-1,
            -1.680_559_006_198_611e-1,
        ),
        (
            22.0e-12,
            1.665_693_704_588_173e-1,
            -4.187_538_203_978_528e-2,
        ),
        (22.5e-12, 8.337_212_477_921_546e-2, 8.505_851_007_997_917e-3),
        (23.0e-12, 8.182_408_779_385_036e-2, 1.058_426_890_257_090e-2),
        (24.0e-12, 8.161_922_225_990_839e-2, 9.456_455_239_337_802e-3),
        (25.0e-12, 8.144_913_394_164_591e-2, 8.251_753_227_523_917e-3),
    ];
    let mut max_qs_delta: f64 = 0.0;
    for &(time, expected, expected_delta_vs_qs) in reference {
        let got = interp_waveform(&result.time, vout, time);
        let abs_err = (got - expected).abs();
        assert!(
            abs_err < 2e-3,
            "TRNQSMOD=1 RBODYMOD=1 transient v(out) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} abs_err={abs_err:.3e}"
        );

        let qs = interp_waveform(&qs_result.time, qs_vout, time);
        let qs_delta = got - qs;
        max_qs_delta = max_qs_delta.max(qs_delta.abs());
        let delta_err = (qs_delta - expected_delta_vs_qs).abs();
        assert!(
            delta_err < 3e-3,
            "TRNQSMOD=1 RBODYMOD=1 delta vs QS at {time:.3e}s: rspice={qs_delta:.9e} ngspice={expected_delta_vs_qs:.9e} abs_err={delta_err:.3e}"
        );
    }
    assert!(
        max_qs_delta > 1e-1,
        "TRNQSMOD=1 with RBODYMOD=1 must not silently degrade to QS; max |delta v(out)|={max_qs_delta:.3e}"
    );
}

#[test]
fn trnqsmod1_rgatemod1_common_source_transient_matches_ngspice46() {
    // ngspice-46 release oracle from the exact deck with
    // `ngspice_con.exe -b`, `set numdgt=15`, and `wrdata v(out)`.
    // Xyce regression coverage includes BSIM4 RGATEMOD topologies, but no
    // matching TRNQSMOD=1 transient-NQS oracle; keep ngspice as the physics
    // oracle for this charge-deficit slice.
    let deck = trnqsmod_rgatemod1_common_source_deck(1);
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 200.0e-12, 0.1e-12)
        .expect("TRNQSMOD=1 with RGATEMOD=1 transient runs natively");
    let vout = result
        .try_voltage_waveform_named("out")
        .expect("out waveform");

    let qs_deck = trnqsmod_rgatemod1_common_source_deck(0);
    let qs_netlist = Netlist::parse(&qs_deck).expect("QS deck parses");
    let qs_result = engine()
        .run_tran(&qs_netlist, 200.0e-12, 0.1e-12)
        .expect("TRNQSMOD=0 with RGATEMOD=1 transient runs");
    let qs_vout = qs_result
        .try_voltage_waveform_named("out")
        .expect("QS out waveform");

    let reference: &[(f64, f64, f64)] = &[
        (21.0e-12, 1.043_700_960_542, 1.352_398_456_023e-4),
        (25.0e-12, 1.021_327_482_108, -3.596_991_580_699e-3),
        (50.0e-12, 7.520_064_843_996e-1, -2.479_646_149_617e-2),
        (100.0e-12, 3.432_018_415_062e-1, -3.502_507_618_773e-2),
        (200.0e-12, 9.058_323_667_115e-2, -3.803_900_420_606e-3),
    ];
    let mut max_qs_delta: f64 = 0.0;
    for &(time, expected, expected_delta_vs_qs) in reference {
        let got = interp_waveform(&result.time, vout, time);
        let abs_err = (got - expected).abs();
        assert!(
            abs_err < 2e-3,
            "TRNQSMOD=1 RGATEMOD=1 transient v(out) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} abs_err={abs_err:.3e}"
        );

        let qs = interp_waveform(&qs_result.time, qs_vout, time);
        let qs_delta = got - qs;
        max_qs_delta = max_qs_delta.max(qs_delta.abs());
        let delta_err = (qs_delta - expected_delta_vs_qs).abs();
        assert!(
            delta_err < 3e-3,
            "TRNQSMOD=1 RGATEMOD=1 delta vs QS at {time:.3e}s: rspice={qs_delta:.9e} ngspice={expected_delta_vs_qs:.9e} abs_err={delta_err:.3e}"
        );
    }
    assert!(
        max_qs_delta > 2e-2,
        "TRNQSMOD=1 with RGATEMOD=1 must not silently degrade to QS; max |delta v(out)|={max_qs_delta:.3e}"
    );
}

#[test]
fn trnqsmod1_rgatemod2_common_source_transient_matches_ngspice46() {
    // ngspice-46 release oracle from the exact deck with
    // `ngspice_con.exe -b`, `set numdgt=15`, and `wrdata v(out)`.
    // ngspice warns when charge-deficit transient NQS and gate resistance are
    // both selected, but still evaluates this topology; pin it natively rather
    // than falling back or silently degrading to QS.
    let deck = trnqsmod_rgatemod2_common_source_deck(1);
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let circuit = engine()
        .build_circuit(&netlist)
        .expect("TRNQSMOD=1 RGATEMOD=2 deck builds natively");
    assert!(
        circuit.get_node_by_name("m1.__gint").is_some(),
        "RGATEMOD=2 must create the native gate-prime node"
    );
    assert!(
        circuit.get_node_by_name("m1.__charge").is_some(),
        "TRNQSMOD=1 must create the hidden charge-deficit node"
    );
    let result = engine()
        .run_tran(&netlist, 200.0e-12, 0.1e-12)
        .expect("TRNQSMOD=1 with RGATEMOD=2 transient runs natively");
    let vout = result
        .try_voltage_waveform_named("out")
        .expect("out waveform");

    let qs_deck = trnqsmod_rgatemod2_common_source_deck(0);
    let qs_netlist = Netlist::parse(&qs_deck).expect("QS deck parses");
    let qs_result = engine()
        .run_tran(&qs_netlist, 200.0e-12, 0.1e-12)
        .expect("TRNQSMOD=0 with RGATEMOD=2 transient runs");
    let qs_vout = qs_result
        .try_voltage_waveform_named("out")
        .expect("QS out waveform");

    let reference: &[(f64, f64, f64)] = &[
        (21.0e-12, 1.043_660_244_024_935, 9.511_995_185_129_685e-5),
        (25.0e-12, 1.018_768_336_492_094, -6.174_411_768_187_227e-3),
        (
            50.0e-12,
            7.332_033_696_871_683e-1,
            -4.378_514_206_716_566e-2,
        ),
        (
            100.0e-12,
            3.258_788_211_342_819e-1,
            -5.263_640_893_241_278e-2,
        ),
        (
            200.0e-12,
            7.809_612_053_963_345e-1,
            1.367_941_577_354_548e-2,
        ),
    ];
    let mut max_qs_delta: f64 = 0.0;
    for &(time, expected, expected_delta_vs_qs) in reference {
        let got = interp_waveform(&result.time, vout, time);
        let abs_err = (got - expected).abs();
        assert!(
            abs_err < 2e-3,
            "TRNQSMOD=1 RGATEMOD=2 transient v(out) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} abs_err={abs_err:.3e}"
        );

        let qs = interp_waveform(&qs_result.time, qs_vout, time);
        let qs_delta = got - qs;
        max_qs_delta = max_qs_delta.max(qs_delta.abs());
        let delta_err = (qs_delta - expected_delta_vs_qs).abs();
        assert!(
            delta_err < 3e-3,
            "TRNQSMOD=1 RGATEMOD=2 delta vs QS at {time:.3e}s: rspice={qs_delta:.9e} ngspice={expected_delta_vs_qs:.9e} abs_err={delta_err:.3e}"
        );
    }
    assert!(
        max_qs_delta > 3e-2,
        "TRNQSMOD=1 with RGATEMOD=2 must not silently degrade to QS; max |delta v(out)|={max_qs_delta:.3e}"
    );
}

#[test]
fn trnqsmod1_rgatemod3_common_source_transient_matches_ngspice46() {
    // ngspice-46 release oracle from the exact deck with
    // `ngspice_con.exe -b`, `set numdgt=15`, and `wrdata v(out)`.
    // This composes the charge-deficit transient-NQS state with the native
    // RGATEMOD=3 middle-gate topology.
    let deck = trnqsmod_rgatemod3_common_source_deck(1);
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let circuit = engine()
        .build_circuit(&netlist)
        .expect("TRNQSMOD=1 RGATEMOD=3 deck builds natively");
    assert!(
        circuit.get_node_by_name("m1.__gmid").is_some(),
        "RGATEMOD=3 must create the native middle-gate node"
    );
    assert!(
        circuit.get_node_by_name("m1.__gint").is_some(),
        "RGATEMOD=3 must create the native gate-prime node"
    );
    assert!(
        circuit.get_node_by_name("m1.__charge").is_some(),
        "TRNQSMOD=1 must create the hidden charge-deficit node"
    );
    let result = engine()
        .run_tran(&netlist, 200.0e-12, 0.1e-12)
        .expect("TRNQSMOD=1 with RGATEMOD=3 transient runs natively");
    let vout = result
        .try_voltage_waveform_named("out")
        .expect("out waveform");

    let qs_deck = trnqsmod_rgatemod3_common_source_deck(0);
    let qs_netlist = Netlist::parse(&qs_deck).expect("QS deck parses");
    let qs_result = engine()
        .run_tran(&qs_netlist, 200.0e-12, 0.1e-12)
        .expect("TRNQSMOD=0 with RGATEMOD=3 transient runs");
    let qs_vout = qs_result
        .try_voltage_waveform_named("out")
        .expect("QS out waveform");

    let reference: &[(f64, f64, f64)] = &[
        (21.0e-12, 1.043_705_732_163_425, 1.254_894_203_346_346e-4),
        (25.0e-12, 1.021_352_526_021_268, -3.625_744_297_955_036e-3),
        (
            50.0e-12,
            7.520_417_384_538_742e-1,
            -2.481_817_689_164_045e-2,
        ),
        (
            100.0e-12,
            3.432_362_814_044_708e-1,
            -3.504_368_911_048_506e-2,
        ),
        (
            200.0e-12,
            7.661_957_866_240_539e-1,
            -1.064_657_453_059_947e-3,
        ),
    ];
    let mut max_qs_delta: f64 = 0.0;
    for &(time, expected, expected_delta_vs_qs) in reference {
        let got = interp_waveform(&result.time, vout, time);
        let abs_err = (got - expected).abs();
        assert!(
            abs_err < 2e-3,
            "TRNQSMOD=1 RGATEMOD=3 transient v(out) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} abs_err={abs_err:.3e}"
        );

        let qs = interp_waveform(&qs_result.time, qs_vout, time);
        let qs_delta = got - qs;
        max_qs_delta = max_qs_delta.max(qs_delta.abs());
        let delta_err = (qs_delta - expected_delta_vs_qs).abs();
        assert!(
            delta_err < 3e-3,
            "TRNQSMOD=1 RGATEMOD=3 delta vs QS at {time:.3e}s: rspice={qs_delta:.9e} ngspice={expected_delta_vs_qs:.9e} abs_err={delta_err:.3e}"
        );
    }
    assert!(
        max_qs_delta > 2e-2,
        "TRNQSMOD=1 with RGATEMOD=3 must not silently degrade to QS; max |delta v(out)|={max_qs_delta:.3e}"
    );
}

#[test]
fn trnqsmod1_rdsmod1_rgatemod1_common_source_transient_matches_ngspice46() {
    // The two validation-only transient-NQS unlocks must compose: RDSMOD=1
    // maps D/S to internal primes while RGATEMOD=1 lowers a linear resistor
    // into the intrinsic gate-prime. ngspice-46 is the physics oracle because
    // the local Xyce regression tree has no matching TRNQSMOD=1 transient card.
    let deck = trnqsmod_rdsmod1_rgatemod1_common_source_deck(1);
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 200.0e-12, 0.1e-12)
        .expect("TRNQSMOD=1 with RDSMOD=1 and RGATEMOD=1 transient runs natively");
    let vout = result
        .try_voltage_waveform_named("out")
        .expect("out waveform");

    let qs_deck = trnqsmod_rdsmod1_rgatemod1_common_source_deck(0);
    let qs_netlist = Netlist::parse(&qs_deck).expect("QS deck parses");
    let qs_result = engine()
        .run_tran(&qs_netlist, 200.0e-12, 0.1e-12)
        .expect("TRNQSMOD=0 with RDSMOD=1 and RGATEMOD=1 transient runs");
    let qs_vout = qs_result
        .try_voltage_waveform_named("out")
        .expect("QS out waveform");

    let reference: &[(f64, f64, f64)] = &[
        (21.0e-12, 1.044_822_111_014_900, 1.661_128_851_568_883e-4),
        (25.0e-12, 1.022_150_396_873_794, -3.870_435_096_143_821e-3),
        (
            50.0e-12,
            7.603_819_926_870_015e-1,
            -2.467_159_878_587_910e-2,
        ),
        (
            100.0e-12,
            3.984_975_174_651_587e-1,
            -3.302_114_335_234_280e-2,
        ),
        (
            200.0e-12,
            1.799_145_729_018_396e-1,
            -3.510_942_533_033_690e-3,
        ),
    ];
    let mut max_qs_delta: f64 = 0.0;
    for &(time, expected, expected_delta_vs_qs) in reference {
        let got = interp_waveform(&result.time, vout, time);
        let abs_err = (got - expected).abs();
        assert!(
            abs_err < 2e-3,
            "TRNQSMOD=1 RDSMOD=1 RGATEMOD=1 transient v(out) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} abs_err={abs_err:.3e}"
        );

        let qs = interp_waveform(&qs_result.time, qs_vout, time);
        let qs_delta = got - qs;
        max_qs_delta = max_qs_delta.max(qs_delta.abs());
        let delta_err = (qs_delta - expected_delta_vs_qs).abs();
        assert!(
            delta_err < 3e-3,
            "TRNQSMOD=1 RDSMOD=1 RGATEMOD=1 delta vs QS at {time:.3e}s: rspice={qs_delta:.9e} ngspice={expected_delta_vs_qs:.9e} abs_err={delta_err:.3e}"
        );
    }
    assert!(
        max_qs_delta > 2e-2,
        "TRNQSMOD=1 with RDSMOD=1 and RGATEMOD=1 must not silently degrade to QS; max |delta v(out)|={max_qs_delta:.3e}"
    );
}

#[test]
fn level54_runs_without_simplified_mos_optin() {
    // The full LEVEL=54 card must build and solve natively: no
    // `.options allow_simplified_mos`, no rejection, and the OP report
    // names the BSIM4 port for every instance.
    let deck = format!(
        "* level 54 native\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45\n\
         {}\
         {}\n\
         .op\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models45()
    );
    assert!(
        !deck.contains("allow_simplified_mos"),
        "deck must not opt into the approximation"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("LEVEL=54 deck runs natively");
    let bsim4_count = report
        .entries
        .iter()
        .filter(|e| e.device_kind == "BSIM4")
        .count();
    assert_eq!(bsim4_count, 2, "both transistors use the native port");
}

#[test]
fn igc_igb_gate_tunneling_matches_ngspice() {
    // Same n45 fixture as the OP oracle, with BSIM4 gate-current modes
    // enabled. ngspice-46 reference from `ngspice_con.exe -b` on this deck:
    // i(vg) = -1.24172312e-09 A.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 igcmod=1 igbmod=1",
    );
    let deck = format!(
        "* bsim4 gate current op\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         m1 d g 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine().run_dc_op(&netlist).expect("op converges");
    let gate_branch = result
        .branch_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("vg"))
        .unwrap_or_else(|| panic!("missing vg branch in {:?}", result.branch_names));
    let gate_current_into_device = -result.branch_currents[gate_branch];
    let reference = 1.241_723_12e-9;
    let rel = (gate_current_into_device - reference).abs() / reference;
    assert!(
        rel < 1e-5,
        "gate tunneling mismatch: rspice={gate_current_into_device:.9e} ngspice={reference:.9e} rel={rel:.3e}"
    );
}

#[test]
fn diomod2_reverse_breakdown_runs_through_engine() {
    // ngspice-46 reference for this full deck:
    // i(vb) = 7.213453e-01 A, @m1[ibs] = -7.21345e-01 A.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 diomod=2",
    );
    let deck = format!(
        "* bsim4 diomod2 reverse body diode\n\
         vs s 0 dc 0\n\
         vd d 0 dc 0\n\
         vg g 0 dc 0\n\
         vb b 0 dc -12\n\
         m1 d g s b n45 w=1u l=45n ad=0 as=0.1p pd=0 ps=2.2u nrd=0 nrs=0\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine().run_dc_op(&netlist).expect("op converges");
    let body_branch = result
        .branch_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("vb"))
        .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names));
    let reference = 7.213_453e-1;
    let rel = (result.branch_currents[body_branch] - reference).abs() / reference;
    assert!(
        rel < 2e-5,
        "dioMod=2 body branch mismatch: rspice={:.9e} ngspice={reference:.9e} rel={rel:.3e}",
        result.branch_currents[body_branch]
    );
}

#[test]
fn capmod1_dc_runs_natively_and_matches_ngspice_dc_oracle() {
    // CAPMOD selects the intrinsic charge model; BSIM4's DC load path is
    // independent of that selector. A DC-only deck must therefore still run
    // through the native BSIM4 evaluator instead of being rejected at build.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 capmod=1",
    );
    let deck = format!(
        "* bsim4 capmod1 dc op\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         m1 d g 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("CAPMOD=1 DC op runs natively");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(entry.device_kind, "BSIM4");
    let id = entry
        .params
        .iter()
        .find(|(k, _)| *k == "id")
        .map(|(_, v)| *v)
        .expect("id op param");
    let reference = 1.408_919_35e-3;
    let rel = (id - reference).abs() / reference;
    assert!(
        rel < 1e-6,
        "CAPMOD=1 DC id mismatch: rspice={id:.9e} ngspice={reference:.9e} rel={rel:.3e}"
    );
}

#[test]
fn inverter_ac_response_with_capmod1_matches_ngspice() {
    // Same inverter as the default BSIM4 AC oracle, but with the n45 model's
    // CAPMOD=1 intrinsic charge model. ngspice-46 reference from a local
    // `ngspice_con.exe -b` run on this deck with `.option reltol=1e-6`.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 capmod=1",
    );
    let deck = format!(
        "* bsim4 capmod1 inverter ac\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 2.18108888e+01, 3.14106759e+00),
        (1.000000e7, 2.18107735e+01, 3.13634209e+00),
        (1.000000e8, 2.17992504e+01, 3.08913284e+00),
        (1.000000e9, 2.07784702e+01, 2.65660771e+00),
        (1.000000e10, 7.38492361e+00, 1.69189634e+00),
        (1.000000e11, -1.07599229e+01, 9.81497621e-01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, db_ref, ph_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let ph = v.arg();
        assert!(
            (db - db_ref).abs() < 1e-3,
            "CAPMOD=1 AC magnitude at {freq:.3e} Hz: engine={db:.5} dB ngspice={db_ref} dB"
        );
        assert!(
            (ph - ph_ref).abs() < 1e-3,
            "CAPMOD=1 AC phase at {freq:.3e} Hz: engine={ph:.5} ngspice={ph_ref}"
        );
    }
}

#[test]
fn inverter_ac_response_with_capmod1_matches_xyce710() {
    // Same CAPMOD=1 AC deck as the ngspice oracle above. Xyce 7.10 maps
    // VERSION=4.8 to its 4.8.2 implementation and ignores ngspice-only KETAC,
    // so this is a compatibility pin with production-grade tolerance.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 capmod=1",
    );
    let deck = format!(
        "* bsim4 capmod1 inverter ac xyce\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 1.23181203e+01, 1.79969932e+02),
        (1.000000e7, 1.23179569e+01, 1.79699319e+02),
        (1.000000e8, 1.23016432e+01, 1.76995815e+02),
        (1.000000e9, 1.09388421e+01, 1.52224596e+02),
        (1.000000e10, 2.34133862e+00, 9.69440735e+01),
        (1.000000e11, 2.89887807e-01, 5.62362472e+01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, mag_ref, phase_deg_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let mag = v.norm();
        let phase_deg = v.arg().to_degrees();
        let mag_rel = (mag - mag_ref).abs() / mag_ref;
        assert!(
            mag_rel < 1e-3,
            "CAPMOD=1 AC magnitude vs Xyce at {freq:.3e} Hz: engine={mag:.9e} xyce={mag_ref:.9e} rel={mag_rel:.3e}"
        );
        assert!(
            (phase_deg - phase_deg_ref).abs() < 5e-2,
            "CAPMOD=1 AC phase vs Xyce at {freq:.3e} Hz: engine={phase_deg:.6} deg xyce={phase_deg_ref:.6} deg"
        );
    }
}

#[test]
fn inverter_ac_response_with_capmod0_matches_ngspice() {
    // Same inverter as the default BSIM4 AC oracle, but with the n45 model's
    // CAPMOD=0 compatibility charge model. ngspice-46 reference from a local
    // `ngspice_con.exe -b` run on this deck with `.option reltol=1e-6`.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 capmod=0",
    );
    let deck = format!(
        "* bsim4 capmod0 inverter ac\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 2.18108889e+01, 3.14107213e+00),
        (1.000000e7, 2.18107754e+01, 3.13638750e+00),
        (1.000000e8, 2.17994422e+01, 3.08958584e+00),
        (1.000000e9, 2.07936534e+01, 2.66024638e+00),
        (1.000000e10, 7.45313279e+00, 1.69591336e+00),
        (1.000000e11, -1.07878067e+01, 9.98505133e-01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, db_ref, ph_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let ph = v.arg();
        assert!(
            (db - db_ref).abs() < 1e-3,
            "CAPMOD=0 AC magnitude at {freq:.3e} Hz: engine={db:.5} dB ngspice={db_ref} dB"
        );
        assert!(
            (ph - ph_ref).abs() < 1e-3,
            "CAPMOD=0 AC phase at {freq:.3e} Hz: engine={ph:.5} ngspice={ph_ref}"
        );
    }
}

#[test]
fn inverter_ac_response_with_capmod0_matches_xyce710() {
    // Same CAPMOD=0 AC deck as the ngspice oracle above. Xyce 7.10 maps
    // VERSION=4.8 to its 4.8.2 implementation and ignores ngspice-only KETAC,
    // so this is a compatibility pin with production-grade tolerance.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 capmod=0",
    );
    let deck = format!(
        "* bsim4 capmod0 inverter ac xyce\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 1.23181204e+01, 1.79970192e+02),
        (1.000000e7, 1.23179596e+01, 1.79701921e+02),
        (1.000000e8, 1.23019146e+01, 1.77021771e+02),
        (1.000000e9, 1.09579744e+01, 1.52433119e+02),
        (1.000000e10, 2.35980642e+00, 9.71743215e+01),
        (1.000000e11, 2.88959944e-01, 5.72107155e+01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, mag_ref, phase_deg_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let mag = v.norm();
        let phase_deg = v.arg().to_degrees();
        let mag_rel = (mag - mag_ref).abs() / mag_ref;
        assert!(
            mag_rel < 1e-3,
            "CAPMOD=0 AC magnitude vs Xyce at {freq:.3e} Hz: engine={mag:.9e} xyce={mag_ref:.9e} rel={mag_rel:.3e}"
        );
        assert!(
            (phase_deg - phase_deg_ref).abs() < 5e-2,
            "CAPMOD=0 AC phase vs Xyce at {freq:.3e} Hz: engine={phase_deg:.6} deg xyce={phase_deg_ref:.6} deg"
        );
    }
}

#[test]
fn capmod0_xpart_suppressed_ac_runs_natively() {
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 capmod=0 xpart=-1",
    );
    let deck = format!(
        "* bsim4 capmod0 xpart suppression ac\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1 ac 1\n\
         m1 d g 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {models}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let results = engine()
        .run_ac(&netlist, &[1.0e6])
        .expect("XPART<0 CAPMOD=0 uses native overlap/junction charge path");
    let idx = results[0]
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("g"))
        .expect("g in ac result");
    assert!(results[0].voltages[idx].norm().is_finite());
}

#[test]
fn mobmod3_to_6_op_matches_xyce710_currents() {
    // Xyce 7.10 (BSIM4 4.8.2 backend) matches ngspice on the terminal
    // current and gm/gds at this bias. Xyce's printed Vdsat is a separate
    // compatibility output and is intentionally not used as the internal
    // physics oracle here.
    let reference: &[(i32, f64, f64, f64, f64)] = &[
        (
            3,
            1.55272304e-3,
            2.07661758e-3,
            2.98005557e-4,
            3.16523792e-1,
        ),
        (
            4,
            1.35441038e-3,
            1.83139236e-3,
            2.71841048e-4,
            3.16523792e-1,
        ),
        (
            5,
            1.35441038e-3,
            1.83139236e-3,
            2.71841048e-4,
            3.16523792e-1,
        ),
        (
            6,
            2.56111251e-8,
            3.68188920e-8,
            5.07524870e-9,
            3.16523792e-1,
        ),
    ];

    for &(mob_mod, id_ref, gm_ref, gds_ref, vth_ref) in reference {
        let deck = format!(
            "* bsim4 mobmod={mob_mod} xyce op\n\
             vd d 0 dc 1.1\n\
             vg g 0 dc 1.1\n\
             vb b 0 dc 0\n\
             m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
             {}\n\
             .op\n\
             .end\n",
            models45_mobmod(mob_mod)
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let (_, report) = engine()
            .run_dc_op_with_report(&netlist)
            .expect("MOBMOD op runs natively");
        let entry = report
            .entries
            .iter()
            .find(|e| e.name.eq_ignore_ascii_case("m1"))
            .expect("m1 op entry");
        assert_eq!(entry.device_kind, "BSIM4");
        let get = |key: &str| {
            entry
                .params
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| *v)
                .unwrap_or_else(|| panic!("missing op param {key}"))
        };
        let check = |what: &str, got: f64, expected: f64| {
            let abs = (got - expected).abs();
            let rel = (got - expected).abs() / expected.abs().max(1e-30);
            assert!(
                rel < 1e-5 || abs < 2e-12,
                "MOBMOD={mob_mod} {what}: rspice={got:.9e} xyce={expected:.9e} abs={abs:.3e} rel={rel:.3e}"
            );
        };
        check("id", get("id"), id_ref);
        check("gm", get("gm"), gm_ref);
        check("gds", get("gds"), gds_ref);
        check("vth", get("vth"), vth_ref);
    }
}

#[test]
fn instance_geomod_overrides_model_geomod_for_implicit_diffusions() {
    let geometry_sensitive_models = |geo_mod: i32| {
        models45().replace(
            ".model n45 nmos level=54 version=4.8",
            &format!(
                ".model n45 nmos level=54 version=4.8 geomod={geo_mod} \
                 jss=1e-3 jsd=1e-3 jsws=2e-6 jswd=2e-6 jswgs=2e-6 jswgd=2e-6"
            ),
        )
    };
    let model_geomod1 = geometry_sensitive_models(1);
    let model_geomod0 = geometry_sensitive_models(0);
    let deck = |models: &str, instance_tail: &str| {
        format!(
            "* bsim4 instance geomod override\n\
             vd d 0 dc 0\n\
             vg g 0 dc 0\n\
             vb b 0 dc -0.45\n\
             m1 d g 0 b n45 w=1u l=45n nf=3 {instance_tail}\n\
             {models}\n\
             .op\n\
             .end\n"
        )
    };
    let body_branch_current = |deck: String| {
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let result = engine()
            .run_dc_op(&netlist)
            .expect("GEOMOD deck runs natively");
        let body_branch = result
            .branch_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("vb"))
            .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names));
        result.branch_currents[body_branch]
    };

    let explicit_geo0 = body_branch_current(deck(&model_geomod1, "geomod=0"));
    let baseline_geo0 = body_branch_current(deck(&model_geomod0, ""));
    let model_geo1 = body_branch_current(deck(&model_geomod1, ""));

    assert!(
        (explicit_geo0 - baseline_geo0).abs() < 1e-12,
        "instance GEOMOD=0 must override model GEOMOD=1: explicit={explicit_geo0:.9e} baseline={baseline_geo0:.9e}"
    );
    assert!(
        (model_geo1 - baseline_geo0).abs() > 1e-13,
        "fixture must distinguish model GEOMOD=1 from GEOMOD=0: model={model_geo1:.9e} baseline={baseline_geo0:.9e}"
    );
}

#[test]
fn wpemod_explicit_sca_scb_scc_matches_xyce710_current() {
    // Xyce 7.10 BSIM4 4.8.2 reference for the same one-point DC deck:
    // I(VD) = -1.38255399e-03 A. RSpice reports positive device drain
    // current, so the device `id` target is +1.38255399e-03 A.
    let deck = format!(
        "* bsim4 wpemod explicit sca/scb/scc\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u \
            nrd=0 nrs=0 sca=5 scb=0.01 scc=0.0001\n\
         {}\n\
         .op\n\
         .end\n",
        models45_wpemod()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("WPEMOD explicit SCA/SCB/SCC deck runs natively");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(entry.device_kind, "BSIM4");
    let id = entry
        .params
        .iter()
        .find(|(k, _)| *k == "id")
        .map(|(_, v)| *v)
        .expect("id op param");
    let reference = 1.382_553_99e-3;
    let rel = (id - reference).abs() / reference;
    assert!(
        rel < 2e-5,
        "WPEMOD explicit SCA/SCB/SCC current vs Xyce: rspice={id:.9e} xyce={reference:.9e} rel={rel:.3e}"
    );
}

#[test]
fn wpemod_sc_derives_integrals_and_matches_xyce710_current() {
    // Xyce 7.10 reference for the same one-point DC deck with only SC given:
    // I(VD) = -1.38720867e-03 A. This proves the builder forwards SC and the
    // model derives SCA/SCB/SCC instead of requiring the integrals directly.
    let deck = format!(
        "* bsim4 wpemod sc-derived integrals\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u \
            nrd=0 nrs=0 sc=0.2u\n\
         {}\n\
         .op\n\
         .end\n",
        models45_wpemod()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("WPEMOD SC-derived deck runs natively");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(entry.device_kind, "BSIM4");
    let id = entry
        .params
        .iter()
        .find(|(k, _)| *k == "id")
        .map(|(_, v)| *v)
        .expect("id op param");
    let reference = 1.387_208_67e-3;
    let rel = (id - reference).abs() / reference;
    assert!(
        rel < 2e-5,
        "WPEMOD SC-derived current vs Xyce: rspice={id:.9e} xyce={reference:.9e} rel={rel:.3e}"
    );
}

#[test]
fn stress_layout_distances_match_xyce710_current() {
    // Xyce 7.10 BSIM4 4.8.2 reference for the same one-point DC deck:
    // I(VD) = -1.47063491e-03 A. RSpice reports positive device drain
    // current, so the device `id` target is +1.47063491e-03 A.
    let deck = format!(
        "* bsim4 stress layout distances\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u \
            nrd=0 nrs=0 sa=0.2u sb=0.4u\n\
         {}\n\
         .op\n\
         .end\n",
        models45_stress()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("BSIM4 stress deck runs natively");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(entry.device_kind, "BSIM4");
    let id = entry
        .params
        .iter()
        .find(|(k, _)| *k == "id")
        .map(|(_, v)| *v)
        .expect("id op param");
    let reference = 1.470_634_91e-3;
    let rel = (id - reference).abs() / reference;
    assert!(
        rel < 2e-5,
        "BSIM4 stress current vs Xyce: rspice={id:.9e} xyce={reference:.9e} rel={rel:.3e}"
    );
}

#[test]
fn rdsmod1_external_resistance_matches_ngspice46_current() {
    // ngspice-46 console reference for the same one-point OP deck with the
    // repo's n45 card plus RDSMOD=1/RDW/RSW terms:
    // I(VD) = -1.23762e-03 A and @m1[id] = +1.237623e-03 A.
    // The reported BSIM4 voltages are intrinsic D'/S' biases after the
    // external nonlinear source/drain drops, not the forced terminal voltages.
    let deck = format!(
        "* bsim4 rdsmod=1 external resistance\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=1 nrs=1\n\
         {}\n\
         .op\n\
         .end\n",
        models45_rdsmod1()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("BSIM4 RDSMOD=1 deck runs natively");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(entry.device_kind, "BSIM4");
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs().max(1e-30);
        assert!(
            rel < 2e-5,
            "{what}: rspice={ours:.9e} ngspice={reference:.9e} rel={rel:.3e}"
        );
    };
    assert_rel("id", get("id"), 1.237_623e-3);
    assert_rel("gm", get("gm"), 2.986_704e-3);
    assert_rel("gds", get("gds"), 1.540_179e-3);
    assert_rel("gmb", get("gmb"), -8.167_67e-4);
    assert_rel("vth", get("vth"), 3.042_757e-1);
    assert_rel("vdsat", get("vdsat"), 1.691_025e-1);
    assert_rel("vds", get("vds"), 4.319_486e-1);
    assert_rel("vgs", get("vgs"), 7.884_280e-1);
    assert_rel("vbs", get("vbs"), -3.115_72e-1);
}

#[test]
fn rbodymod1_internal_body_network_matches_ngspice46_forward_body_oracle() {
    // ngspice-46 console reference for this one-point OP deck:
    // RBODYMOD=1 creates dbody/body/sbody nodes. The deliberately large body
    // resistors make the source junction current pull BODY below the external
    // substrate terminal, so accepting the selector without the internal
    // topology cannot match the reported intrinsic VBS.
    let deck = format!(
        "* bsim4 rbodymod=1 high body resistance\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vd d 0 dc 0.2\n\
         vg g 0 dc 0.5\n\
         vb b 0 dc 0.8\n\
         m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .op\n\
         .end\n",
        models45_rbodymod1_high_resistance()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("BSIM4 RBODYMOD=1 deck runs natively");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(entry.device_kind, "BSIM4");
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs().max(1e-30);
        assert!(
            rel < 2e-5,
            "{what}: rspice={ours:.9e} ngspice={reference:.9e} rel={rel:.3e}"
        );
    };
    assert_rel("id", get("id"), 3.456_511e-4);
    assert_rel("gm", get("gm"), 1.673_018e-3);
    assert_rel("gds", get("gds"), 3.426_212e-4);
    assert_rel("gmb", get("gmb"), -5.193_54e-5);
    assert_rel("vth", get("vth"), 1.770_289e-1);
    assert_rel("vdsat", get("vdsat"), 1.434_455e-1);
    assert_rel("vds", get("vds"), 2.0e-1);
    assert_rel("vgs", get("vgs"), 5.0e-1);
    assert_rel("vbs", get("vbs"), 7.855_149e-1);
}

#[test]
fn rbodymod2_geometry_scaled_body_network_matches_xyce710_and_ngspice46() {
    // Xyce 7.10 and ngspice-46 agree on this one-point deck. With no
    // RBPS0/RBPD0 geometry parameters supplied, RBODYMOD=2 selects ngspice
    // bodymode=1: BP-DB/BP-SB are idealized, DB-B/SB-B are gbmin only, and
    // BP-B uses the geometry-scaled RBPB X/Y defaults.
    let deck = format!(
        "* bsim4 rbodymod=2 geometry-scaled body network\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd d 0 dc 0.2\n\
         vg g 0 dc 0.5\n\
         vb b 0 dc 0.8\n\
         m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .op\n\
         .end\n",
        models45_rbodymod2_defaults()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (result, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("BSIM4 RBODYMOD=2 deck runs natively");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(entry.device_kind, "BSIM4");
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs().max(1e-30);
        assert!(
            rel < 2e-5,
            "{what}: rspice={ours:.9e} oracle={reference:.9e} rel={rel:.3e}"
        );
    };
    assert_rel("id", get("id"), 3.440_365_541e-4);
    assert_rel("gm", get("gm"), 1.674_528_257e-3);
    assert_rel("gds", get("gds"), 3.349_612_054e-4);
    assert_rel("gmb", get("gmb"), -1.758_262_579e-4);
    assert_rel("vth", get("vth"), 1.707_711_867e-1);
    assert_rel("vdsat", get("vdsat"), 1.417_635_876e-1);
    assert_rel("vbs", get("vbs"), 7.998_908_706e-1);

    let branch = result
        .branch_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("vdd"))
        .unwrap_or_else(|| panic!("missing vdd branch in {:?}", result.branch_names));
    let branch_ref = -3.440_351_720e-4;
    let branch_rel =
        (result.branch_currents[branch] - branch_ref).abs() / branch_ref.abs().max(1e-30);
    assert!(
        branch_rel < 5e-5,
        "i(vdd): rspice={:.9e} oracle={branch_ref:.9e} rel={branch_rel:.3e}",
        result.branch_currents[branch]
    );
}

#[test]
fn mtrlmod1_compat1_op_matches_ngspice46_material_oracle() {
    // ngspice-46 console reference for this deck:
    // MTRLMOD=1/MTRLCOMPATMOD=1 maps the effective material oxide to EOT
    // while keeping the compatibility TOXP path. RSpice must run it through
    // the native BSIM4 path, not reject it or fall back to a simplified MOS.
    let deck = format!(
        "* bsim4 mtrlmod=1 mtrlcompatmod=1 op\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .op\n\
         .end\n",
        models45_mtrlmod1_compat1()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("MTRLMOD=1 deck runs natively");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(entry.device_kind, "BSIM4");
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs().max(1e-30);
        assert!(
            rel < 2e-5,
            "{what}: rspice={ours:.9e} ngspice={reference:.9e} rel={rel:.3e}"
        );
    };
    assert_rel("id", get("id"), 1.559_362e-3);
    assert_rel("gm", get("gm"), 1.811_402e-3);
    assert_rel("gds", get("gds"), 2.894_781e-4);
    assert_rel("gmb", get("gmb"), -1.445_55e-3);
    assert_rel("vth", get("vth"), 2.309_880e-1);
    assert_rel("vdsat", get("vdsat"), 4.948_787e-1);
}

#[test]
fn mtrlmod1_compat0_op_matches_ngspice46_material_oracle() {
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 mtrlmod=1",
    );
    let deck = format!(
        "* bsim4 mtrlmod=1 compat0 op\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("MTRLMOD=1/MTRLCOMPATMOD=0 deck runs natively");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(entry.device_kind, "BSIM4");
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs().max(1e-30);
        assert!(
            rel < 2e-5,
            "{what}: rspice={ours:.9e} ngspice={reference:.9e} rel={rel:.3e}"
        );
    };
    // ngspice-46 reference for this deck. Xyce coverage is exercised with
    // Xyce's own BSIM4 regression cards rather than by remapping this fixture.
    assert_rel("id", get("id"), 1.093_876e-3);
    assert_rel("gm", get("gm"), 1.535_774e-3);
    assert_rel("vth", get("vth"), 3.378_658_15e-1);
    assert_rel("vdsat", get("vdsat"), 3.840_276e-1);
}

#[test]
fn rgatemod1_creates_external_gate_resistance() {
    // RGATEMOD=1 is the constant electrode gate-resistance topology:
    // external gate -> linear Rg -> intrinsic gate-prime. The resistance is
    // ngspice b4temp.c's grgeltd formula, using Lnew rather than Leff.
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 rgatemod=1 rshg=5 xgw=0 xgl=0 ngcon=1",
    );
    let deck = format!(
        "* bsim4 rgatemod=1 gate resistance topology\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n nf=2 ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let wnew = 1.0e-6 / 2.0;
    let weff_cj = wnew - 2.0 * 2.0e-9;
    let lnew = 45.0e-9 - 20.0e-9;
    let rgeltd = 5.0 * (weff_cj / 3.0) / (2.0 * lnew);
    let expected_g = 1.0 / rgeltd;
    let conductance = bsim4_resistor_conductance(&deck, "m1.__rg");
    let rel = (conductance - expected_g).abs() / expected_g;
    assert!(
        rel < 1e-12,
        "RGATEMOD=1 gate resistance: conductance={conductance:.12e} expected={expected_g:.12e} rel={rel:.3e}"
    );
}

#[test]
fn rgatemod2_runs_without_simplified_mos_optin() {
    let deck = rgatemod2_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine().run_dc_op(&netlist);
    assert!(
        result.is_ok(),
        "RGATEMOD=2 must build and run natively, got {result:?}"
    );
}

#[test]
fn rgatemod3_runs_without_simplified_mos_optin() {
    let deck = rgatemod3_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine().run_dc_op(&netlist);
    assert!(
        result.is_ok(),
        "RGATEMOD=3 must build and run natively, got {result:?}"
    );
}

#[test]
fn rgatemod2_pmos_gate_tunneling_op_matches_ngspice46() {
    // ngspice-46 reference from the same deck with `.print op v(out) v(m1#gate)
    // i(vdd) i(vin) i(vb)`. The stressed PMOS gate-current path keeps the
    // native RGATEMOD=2 internal gate separated from the external gate in DC.
    let deck = rgatemod2_pmos_gate_tunnel_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_dc_op(&netlist)
        .expect("PMOS RGATEMOD=2 gate-tunneling op converges");

    let out = result
        .try_voltage_named("out")
        .expect("out voltage in operating point");
    let gint = result
        .try_voltage_named("m1.__gint")
        .expect("internal RGATEMOD=2 gate node");
    let vdd_current = result
        .branch_current_named("vdd")
        .unwrap_or_else(|| panic!("missing vdd branch in {:?}", result.branch_names));
    let vin_current = result
        .branch_current_named("vin")
        .unwrap_or_else(|| panic!("missing vin branch in {:?}", result.branch_names));
    let vb_current = result
        .branch_current_named("vb")
        .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names));

    let checks = [
        ("V(out)", out, 9.820_910e-1, 1e-5),
        ("V(m1.__gint)", gint, 2.279_344e-1, 2e-4),
        ("I(VDD)", vdd_current, -4.910_45e-4, 2e-5),
        ("I(VIN)", vin_current, 4.929_299e-13, 5e-3),
        ("I(VB)", vb_current, -5.056_25e-13, 5e-3),
    ];

    for (label, got, expected, rel_tol) in checks {
        let rel = (got - expected).abs() / expected.abs().max(1e-30);
        assert!(
            rel < rel_tol,
            "{label}: rspice={got:.9e} ngspice={expected:.9e} rel={rel:.3e}"
        );
    }
}

#[test]
fn rgatemod2_transient_gate_current_matches_xyce710_and_ngspice46() {
    // Xyce 7.10 and ngspice-46 agree within 1e-7 relative for I(VIN) on
    // this RGATEMOD=2 pulse deck. The current is small but directly exposes
    // the bias-dependent gate-resistance branch.
    let deck = rgatemod2_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 80.0e-12, 0.1e-12)
        .expect("RGATEMOD=2 transient runs");
    let current = result
        .try_branch_current_waveform_named("vin")
        .unwrap_or_else(|| panic!("missing VIN branch in {:?}", result.branch_names));
    let reference: &[(f64, f64, f64)] = &[
        (20.5e-12, -1.506_024_124e-11, -1.506_024_003e-11),
        (22.5e-12, -7.530_118_610e-11, -7.530_118_671e-11),
        (25.0e-12, -1.506_023_450e-10, -1.506_023_448e-10),
        (40.0e-12, -1.506_020_335e-10, -1.506_020_338e-10),
        (80.0e-12, -1.506_012_150e-10, -1.506_012_148e-10),
    ];
    for &(time, xyce_expected, ngspice_expected) in reference {
        let got = interp_waveform(&result.time, current, time);
        let rel = (got - xyce_expected).abs() / xyce_expected.abs().max(1e-30);
        assert!(
            rel < 2e-3,
            "RGATEMOD=2 transient I(VIN) at {time:.3e}s: rspice={got:.9e} xyce={xyce_expected:.9e} rel={rel:.3e}"
        );
        let ng_rel = (got - ngspice_expected).abs() / ngspice_expected.abs().max(1e-30);
        assert!(
            ng_rel < 2e-3,
            "RGATEMOD=2 transient I(VIN) at {time:.3e}s: rspice={got:.9e} ngspice={ngspice_expected:.9e} rel={ng_rel:.3e}"
        );
    }
}

#[test]
fn rgatemod3_transient_gate_current_matches_ngspice46() {
    // ngspice-46 reference from `.tran 0.1p 80p` on the RGATEMOD=3 deck.
    // The branch current directly exercises the external-gate to middle-gate
    // resistor plus the native middle-gate to gate-prime gcrg branch.
    let deck = rgatemod3_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 80.0e-12, 0.1e-12)
        .expect("RGATEMOD=3 transient runs");
    let current = result
        .try_branch_current_waveform_named("vin")
        .unwrap_or_else(|| panic!("missing VIN branch in {:?}", result.branch_names));
    let reference: &[(f64, f64)] = &[
        (20.5e-12, -1.506_024_059_462_801e-11),
        (22.5e-12, -7.530_118_777_680_897e-11),
        (25.0e-12, -1.506_023_465_090_800e-10),
        (40.0e-12, -1.506_020_354_799_510e-10),
        (80.0e-12, -1.506_012_165_469_856e-10),
    ];
    for &(time, expected) in reference {
        let got = interp_waveform(&result.time, current, time);
        let rel = (got - expected).abs() / expected.abs().max(1e-30);
        assert!(
            rel < 2e-3,
            "RGATEMOD=3 transient I(VIN) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} rel={rel:.3e}"
        );
    }
}

#[test]
fn rgatemod2_gate_resistance_noise_matches_ngspice46() {
    let deck = format!(
        "{}\n.noise v(out) vin dec 1 1e6 1e6\n.end\n",
        rgatemod2_common_source_deck().trim_end_matches(".end\n")
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let output = engine()
        .build_circuit(&netlist)
        .expect("circuit builds")
        .get_node_by_name("out")
        .expect("output node");
    let results = engine()
        .run_noise_with_input_source(&netlist, output, None, "VIN", &[1.0e6], 300.15)
        .expect("RGATEMOD=2 noise runs natively");
    let rg_noise = results[0]
        .contributions
        .iter()
        .find(|contrib| {
            (contrib.device_name.eq_ignore_ascii_case("m1.rg")
                || contrib.device_name.eq_ignore_ascii_case("m1.__rg"))
                && contrib.noise_type.label() == "thermal"
        })
        .unwrap_or_else(|| {
            panic!(
                "RGATEMOD=2 gate-resistance thermal noise missing: {:?}",
                results[0].contributions
            )
        });
    let rg_noise_density = rg_noise.output_contribution.sqrt();
    let ngspice_rg_density = 1.350_639_972e-7;
    let rel = (rg_noise_density - ngspice_rg_density).abs() / ngspice_rg_density;
    assert!(
        rel < 5e-3,
        "RGATEMOD=2 gate-resistance noise density mismatch: rspice={rg_noise_density:.9e} ngspice={ngspice_rg_density:.9e} rel={rel:.3e}"
    );
}

#[test]
fn rgatemod3_gate_resistance_noise_matches_ngspice46() {
    let deck = format!(
        "{}\n.noise v(out) vin dec 1 1e6 1e6\n.end\n",
        rgatemod3_common_source_deck().trim_end_matches(".end\n")
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let output = engine()
        .build_circuit(&netlist)
        .expect("circuit builds")
        .get_node_by_name("out")
        .expect("output node");
    let results = engine()
        .run_noise_with_input_source(&netlist, output, None, "VIN", &[1.0e6], 300.15)
        .expect("RGATEMOD=3 noise runs natively");
    let rg_noise = results[0]
        .contributions
        .iter()
        .find(|contrib| {
            contrib.device_name.eq_ignore_ascii_case("m1.__rg")
                && contrib.noise_type.label() == "thermal"
        })
        .unwrap_or_else(|| {
            panic!(
                "RGATEMOD=3 external-to-middle gate thermal noise missing: {:?}",
                results[0].contributions
            )
        });
    let rg_noise_density = rg_noise.output_contribution.sqrt();
    // ngspice-46 detailed noise summary for this deck gives
    // `onoise.m1.rg = 2.701280e-07` V/sqrt(Hz) at 1 MHz.
    let ngspice_rg_density = 2.701_280e-7;
    let rel = (rg_noise_density - ngspice_rg_density).abs() / ngspice_rg_density;
    assert!(
        rel < 5e-3,
        "RGATEMOD=3 gate-resistance noise density mismatch: rspice={rg_noise_density:.9e} ngspice={ngspice_rg_density:.9e} rel={rel:.3e}"
    );
}

#[test]
fn rgatemod1_gate_resistor_contributes_noise() {
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 rgatemod=1 rshg=5e8 xgw=0 xgl=0 ngcon=1",
    );
    let deck = format!(
        "* bsim4 rgatemod=1 gate-resistor noise\n\
         vdd vdd 0 dc 1.1\n\
         rd vdd out 1k\n\
         vin g 0 dc 0.7 ac 1\n\
         m1 out g 0 0 n45 w=1u l=45n nf=2 ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {models}\n\
         .noise v(out) vin dec 1 1k 1k\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let output = engine()
        .build_circuit(&netlist)
        .expect("circuit builds")
        .get_node_by_name("out")
        .expect("output node");
    let results = engine()
        .run_noise_with_input_source(&netlist, output, None, "VIN", &[1.0e3], 300.15)
        .expect("RGATEMOD=1 noise runs natively");
    let rg_noise = results[0]
        .contributions
        .iter()
        .find(|contrib| {
            contrib.device_name.eq_ignore_ascii_case("m1.__rg")
                && contrib.noise_type.label() == "thermal"
        })
        .unwrap_or_else(|| {
            panic!(
                "m1.__rg thermal noise missing: {:?}",
                results[0].contributions
            )
        });
    assert!(
        rg_noise.output_contribution.is_finite() && rg_noise.output_contribution > 0.0,
        "m1.__rg thermal noise must reach the output, got {rg_noise:?}"
    );
    // ngspice-46 detailed noise summary for the same deck, with `.noise ...
    // 1` source reporting enabled, gives `onoise.m1.rg = 7.653467e-06`
    // V/sqrt(Hz) at 1 kHz. RSpice stores contribution power density, so compare
    // the square root.
    let rg_noise_density = rg_noise.output_contribution.sqrt();
    let ngspice_rg_density = 7.653_467e-6;
    let rel = (rg_noise_density - ngspice_rg_density).abs() / ngspice_rg_density;
    assert!(
        rel < 1.0e-5,
        "m1.__rg noise density mismatch: rspice={rg_noise_density:.9e} ngspice={ngspice_rg_density:.9e} rel={rel:.3e}"
    );
}

#[test]
fn rgeomod1_implicit_geometry_lowers_to_series_resistors() {
    // Explicit instance RGEOMOD=1 uses Berkeley/Xyce `RdseffGeo` when NRD/NRS
    // are omitted: the builder must create D/S prime-node sheet resistors,
    // not silently collapse to ideal external terminals.
    let weff_cj = 1.0e-6 / 3.0 - 2.0 * 2.0e-9;
    let rint = 5.0 * 4.0e-8 / (weff_cj * 2.0);
    let rend = 5.0 * 4.0e-8 / weff_cj;
    let rtot = rint * rend / (rint + rend);
    let expected_g = 1.0 / rtot;

    let xyce_style_model = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 rgeomod=1",
    );
    let cases = [
        (
            "instance",
            "m1 d g 0 b n45 w=1u l=45n nf=3 geomod=1 rgeomod=1",
            models45(),
        ),
        (
            "xyce-model",
            "m1 d g 0 b n45 w=1u l=45n nf=3 geomod=1",
            xyce_style_model.clone(),
        ),
    ];

    for (case, mos, model) in cases {
        let deck = format!(
            "* bsim4 rgeomod=1 implicit S/D geometry ({case})\n\
             vd d 0 dc 1.1\n\
             vg g 0 dc 1.1\n\
             vb b 0 dc 0\n\
             {mos}\n\
             {model}\n\
             .op\n\
             .end\n",
        );

        for name in ["m1.__rd", "m1.__rs"] {
            let conductance = bsim4_resistor_conductance(&deck, name);
            let rel = (conductance - expected_g).abs() / expected_g;
            assert!(
                rel < 1e-12,
                "{case} {name}: conductance={conductance:.12e} expected={expected_g:.12e} \
                 rel={rel:.3e}"
            );
        }
    }

    let explicit_zero = format!(
        "* bsim4 explicit rgeomod=0 overrides Xyce-style model default\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n nf=3 geomod=1 rgeomod=0\n\
         {xyce_style_model}\n\
         .op\n\
         .end\n",
    );
    assert!(
        !bsim4_has_resistor(&explicit_zero, "m1.__rd"),
        "explicit instance RGEOMOD=0 must override model-card RGEOMOD=1"
    );
    assert!(
        !bsim4_has_resistor(&explicit_zero, "m1.__rs"),
        "explicit instance RGEOMOD=0 must override model-card RGEOMOD=1"
    );

    let explicit_squares = format!(
        "* bsim4 explicit NRD/NRS override implicit rgeomod geometry\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n nf=3 geomod=1 rgeomod=1 nrd=2 nrs=3\n\
         {}\n\
         .op\n\
         .end\n",
        models45()
    );
    let rd_g = bsim4_resistor_conductance(&explicit_squares, "m1.__rd");
    let rs_g = bsim4_resistor_conductance(&explicit_squares, "m1.__rs");
    assert!(
        (rd_g - 0.1).abs() < 1.0e-14,
        "explicit NRD must win over RGEOMOD: rd conductance={rd_g:.12e}"
    );
    assert!(
        (rs_g - (1.0 / 15.0)).abs() < 1.0e-14,
        "explicit NRS must win over RGEOMOD: rs conductance={rs_g:.12e}"
    );
}
