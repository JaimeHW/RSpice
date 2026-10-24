* BSIM4 v4.8 NMOS oracle sweeps (rspice bsim4v8 validation, T=27C)
* Four geometries share source-pinned terminals, so each device sees the
* exact sweep bias; per-device probes are independent. m3 gives no
* AD/AS/PD/PS, so ngspice derives the diffusions via BSIM4PAeffGeo
* (geoMod=0, NF=4) exactly like the port.
.include models45.lib
.option gmin=1e-12

vd d 0 dc 0.05
vg g 0 dc 0
vs s 0 dc 0
vb b 0 dc 0

m1 d g s b n45 w=1u l=45n  ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0
m2 d g s b n45 w=2u l=0.2u ad=0.2p as=0.2p pd=4.2u ps=4.2u nrd=0 nrs=0
m3 d g s b n45 w=1u l=45n  nf=4 nrd=0 nrs=0
m4 d g s b n45 w=4u l=1u   ad=0.4p as=0.4p pd=8.2u ps=8.2u nrd=0 nrs=0

.control
set wr_vecnames
set wr_singlescale
save @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd]
+ @m1[isub] @m1[igidl] @m1[igisl] @m1[vgsteff] @m1[vdseff]
+ @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m3[id] @m3[gm] @m3[gds] @m3[gmbs]
+ @m4[id] @m4[gm] @m4[gds] @m4[gmbs]
+ @m1[qg] @m1[qb] @m1[qd] @m1[qs] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdg] @m1[cdd]
+ @m1[cds] @m1[cbg] @m1[cbd] @m1[cbs] @m1[csg] @m1[csd] @m1[css] @m1[cgb]
+ @m1[cdb] @m1[csb] @m1[cbb] @m1[capbd] @m1[capbs]
dc vg 0 1.1 0.025
wrdata nmos_idvg_vd50m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd] @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m3[id] @m3[gm] @m3[gds] @m3[gmbs] @m4[id] @m4[gm] @m4[gds] @m4[gmbs] @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdg] @m1[cdd] @m1[cds] @m1[cbg] @m1[cbd] @m1[cbs] @m1[capbd] @m1[capbs]
alter vb = -0.9
dc vg 0 1.1 0.025
wrdata nmos_idvg_vd50m_vbm09 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd] @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m3[id] @m3[gm] @m3[gds] @m3[gmbs] @m4[id] @m4[gm] @m4[gds] @m4[gmbs]
alter vb = 0
alter vd = 1.1
dc vg 0 1.1 0.025
wrdata nmos_idvg_vd1100m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[isub] @m1[igidl] @m1[vgsteff] @m1[vdseff] @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m3[id] @m3[gm] @m3[gds] @m3[gmbs] @m4[id] @m4[gm] @m4[gds] @m4[gmbs] @m1[qg] @m1[qb] @m1[qd] @m1[qs] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdg] @m1[cdd] @m1[cds] @m1[cbg] @m1[cbd] @m1[cbs] @m1[csg] @m1[csd] @m1[css] @m1[cgb] @m1[cdb] @m1[csb] @m1[cbb]
alter vb = -0.9
dc vg 0 1.1 0.025
wrdata nmos_idvg_vd1100m_vbm09 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[isub] @m1[igidl] @m2[id] @m2[gm] @m2[gds] @m2[gmbs] @m3[id] @m3[gm] @m3[gds] @m3[gmbs] @m4[id] @m4[gm] @m4[gds] @m4[gmbs]
alter vb = 0
alter vg = 0.5
dc vd 0 1.1 0.025
wrdata nmos_idvd_vg500m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vdsat] @m1[isub] @m1[igidl] @m2[id] @m2[gds] @m3[id] @m3[gds] @m4[id] @m4[gds]
alter vg = 0.8
dc vd 0 1.1 0.025
wrdata nmos_idvd_vg800m_vb0 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vdsat] @m1[isub] @m1[igidl] @m2[id] @m2[gds] @m3[id] @m3[gds] @m4[id] @m4[gds] @m1[qg] @m1[qb] @m1[qd] @m1[cgg] @m1[cgd] @m1[cgs] @m1[cdd] @m1[cbd] @m1[capbd] @m1[capbs]
alter vb = -0.45
alter vg = 1.1
dc vd 0 1.1 0.025
wrdata nmos_idvd_vg1100m_vbm045 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vdsat] @m1[isub] @m1[igidl] @m2[id] @m2[gds] @m3[id] @m3[gds] @m4[id] @m4[gds]
.endc
.end
