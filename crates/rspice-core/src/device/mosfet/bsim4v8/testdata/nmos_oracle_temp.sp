* BSIM4 v4.8 NMOS oracle sweeps at 125C (rspice bsim4v8 validation)
.include models45.lib
.option gmin=1e-12
.temp 125

vd d 0 dc 0.05
vg g 0 dc 0
vs s 0 dc 0
vb b 0 dc 0

m1 d g s b n45 w=1u l=45n  ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0
m3 d g s b n45 w=1u l=45n  nf=4 nrd=0 nrs=0

.control
set wr_vecnames
set wr_singlescale
save @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd]
+ @m1[isub] @m1[igidl] @m3[id] @m3[gm] @m3[gds] @m3[gmbs]
dc vg 0 1.1 0.025
wrdata nmos_idvg_vd50m_t125 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd] @m3[id] @m3[gm] @m3[gds] @m3[gmbs]
alter vd = 1.1
dc vg 0 1.1 0.025
wrdata nmos_idvg_vd1100m_t125 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[isub] @m1[igidl] @m3[id] @m3[gm] @m3[gds] @m3[gmbs]
.endc
.end
