* BSIM4 v4.8 PMOS oracle sweeps (rspice bsim4v8 validation, T=27C)
.include models45.lib
.option gmin=1e-12

vd d 0 dc -0.05
vg g 0 dc 0
vs s 0 dc 0
vb b 0 dc 0

m1 d g s b p90 w=1u l=90n   ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0
m2 d g s b p90 w=2u l=0.25u ad=0.2p as=0.2p pd=4.2u ps=4.2u nrd=0 nrs=0

.control
set wr_vecnames
set wr_singlescale
save @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd]
+ @m1[isub] @m1[igidl] @m1[vgsteff] @m1[vdseff]
+ @m2[id] @m2[gm] @m2[gds] @m2[gmbs]
+ @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdg] @m1[cdd]
+ @m1[cds] @m1[cbg] @m1[cbd] @m1[cbs] @m1[capbd] @m1[capbs]
dc vg 0 -1.1 -0.025
wrdata pmos_idvg_vdm50m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd] @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdg] @m1[cdd] @m1[cds] @m1[cbg] @m1[cbd] @m1[cbs] @m1[capbd] @m1[capbs]
alter vb = 0.9
dc vg 0 -1.1 -0.025
wrdata pmos_idvg_vdm50m_vb09 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m2[id] @m2[gm] @m2[gds] @m2[gmbs]
alter vb = 0
alter vd = -1.1
dc vg 0 -1.1 -0.025
wrdata pmos_idvg_vdm1100m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[isub] @m1[igidl] @m1[vgsteff] @m1[vdseff] @m2[id] @m2[gm] @m2[gds] @m2[gmbs]
alter vg = -1.1
dc vd 0 -1.1 -0.025
wrdata pmos_idvd_vgm1100m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vdsat] @m2[id] @m2[gds] @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdd]
.endc
.end
