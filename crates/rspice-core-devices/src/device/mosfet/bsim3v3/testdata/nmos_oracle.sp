* BSIM3v3.3 NMOS oracle sweeps (rspice bsim3v3 validation, T=27C)
* Four geometries share source-pinned terminals, so each device sees the
* exact sweep bias; per-device probes are independent.
.include models018.lib
.option gmin=1e-12

vd d 0 dc 0.05
vg g 0 dc 0
vs s 0 dc 0
vb b 0 dc 0

m1 d g s b n018 w=10u  l=0.18u ad=4.2p  as=4.2p  pd=20.84u ps=20.84u nrd=0 nrs=0
m2 d g s b n018 w=10u  l=10u   ad=4.2p  as=4.2p  pd=20.84u ps=20.84u nrd=0 nrs=0
m3 d g s b n018 w=10u  l=0.5u  ad=4.2p  as=4.2p  pd=20.84u ps=20.84u nrd=0 nrs=0
m4 d g s b n018 w=0.6u l=0.18u ad=0.252p as=0.252p pd=2.04u ps=2.04u nrd=0 nrs=0

.control
set wr_vecnames
set wr_singlescale
save @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd]
+ @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m3[id] @m3[gm] @m3[gds] @m3[gmbs]
+ @m4[id] @m4[gm] @m4[gds] @m4[gmbs]
+ @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdg] @m1[cdd] @m1[cds]
+ @m1[cbg] @m1[cbd] @m1[cbs] @m1[capbd] @m1[capbs]
dc vg 0 1.8 0.05
wrdata nmos_idvg_vd50m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd] @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m3[id] @m3[gm] @m3[gds] @m3[gmbs] @m4[id] @m4[gm] @m4[gds] @m4[gmbs] @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdg] @m1[cdd] @m1[cds] @m1[cbg] @m1[cbd] @m1[cbs] @m1[capbd] @m1[capbs]
alter vb = -0.9
dc vg 0 1.8 0.05
wrdata nmos_idvg_vd50m_vbm09 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m3[id] @m3[gm] @m3[gds] @m3[gmbs] @m4[id] @m4[gm] @m4[gds] @m4[gmbs]
alter vb = 0
alter vd = 1.2
dc vg 0 1.8 0.05
wrdata nmos_idvg_vd1200m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m3[id] @m3[gm] @m3[gds] @m3[gmbs] @m4[id] @m4[gm] @m4[gds] @m4[gmbs] @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdg] @m1[cdd] @m1[cds] @m1[cbg] @m1[cbd] @m1[cbs]
alter vg = 1.2
dc vd 0 1.8 0.025
wrdata nmos_idvd_vg1200m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vdsat] @m2[id] @m2[gds] @m3[id] @m3[gds] @m4[id] @m4[gds] @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdd]
alter vg = 0.6
dc vd 0 1.8 0.025
wrdata nmos_idvd_vg600m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vdsat] @m2[id] @m2[gds] @m3[id] @m3[gds] @m4[id] @m4[gds]
alter vb = -0.45
alter vg = 0.9
dc vd 0 1.8 0.025
wrdata nmos_idvd_vg900m_vbm045 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vdsat] @m2[id] @m2[gds] @m3[id] @m3[gds] @m4[id] @m4[gds]
.endc
.end
