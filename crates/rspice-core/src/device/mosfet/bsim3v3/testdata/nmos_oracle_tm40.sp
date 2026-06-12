* BSIM3v3.3 NMOS oracle sweeps at -40C (rspice bsim3v3 validation)
.include models018.lib
.option gmin=1e-12
.temp -40

vd d 0 dc 0.05
vg g 0 dc 0
vs s 0 dc 0
vb b 0 dc 0

m1 d g s b n018 w=10u l=0.18u ad=4.2p as=4.2p pd=20.84u ps=20.84u nrd=0 nrs=0
m2 d g s b n018 w=10u l=0.5u  ad=4.2p as=4.2p pd=20.84u ps=20.84u nrd=0 nrs=0

.control
set wr_vecnames
set wr_singlescale
save @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd]
+ @m2[id] @m2[gm] @m2[gds] @m2[gmbs]
dc vg 0 1.8 0.05
wrdata nmos_idvg_vd50m_tm40 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m1[ibs] @m1[ibd] @m2[id] @m2[gm] @m2[gds] @m2[gmbs]
alter vd = 1.2
dc vg 0 1.8 0.05
wrdata nmos_idvg_vd1200m_tm40 @m1[id] @m1[gm] @m1[gds] @m1[gmbs] @m1[vth] @m1[vdsat] @m2[id] @m2[gm] @m2[gds] @m2[gmbs]
.endc
.end
