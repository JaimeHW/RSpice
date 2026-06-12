* BSIM3v3.3 PMOS oracle sweeps (rspice bsim3v3 validation, T=27C)
.include models018.lib
.option gmin=1e-12

vd d 0 dc -0.05
vg g 0 dc 0
vs s 0 dc 0
vb b 0 dc 0

m1 d g s b p018 w=10u l=0.18u ad=4.2p as=4.2p pd=20.84u ps=20.84u nrd=0 nrs=0
m2 d g s b p018 w=10u l=0.5u  ad=4.2p as=4.2p pd=20.84u ps=20.84u nrd=0 nrs=0

.control
set wr_vecnames
set wr_singlescale
save @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd]
+ @m2[id] @m2[gm] @m2[gds] @m2[gmbs]
+ @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdd] @m1[capbd] @m1[capbs]
dc vg 0 -1.8 -0.05
wrdata pmos_idvg_vdm50m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd] @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdd] @m1[capbd] @m1[capbs]
alter vb = 0.9
dc vg 0 -1.8 -0.05
wrdata pmos_idvg_vdm50m_vb09 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m2[id] @m2[gm] @m2[gds] @m2[gmbs]
alter vb = 0
alter vd = -1.2
dc vg 0 -1.8 -0.05
wrdata pmos_idvg_vdm1200m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m2[id] @m2[gm] @m2[gds] @m2[gmbs]
alter vg = -1.2
dc vd 0 -1.8 -0.025
wrdata pmos_idvd_vgm1200m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vdsat] @m2[id] @m2[gds] @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdd]
.endc
.end
