
vgnd gnd! 0 ac 0 0 

*============================================
* Library: fb180a
* Cell: CMOS_MIxer
*============================================
VI_ifout_mon N__81 N__8 0 ac 0 0
VI_rfin_mon N__135 N__155 0 ac 0 0
LL_139 N__130 N__131 3e-09
Cc_132 N__130 N__135 1e-10
Rc_132 N__130 N__135 1T
Cc_136 N__131 N__159 1e-10
Rc_136 N__131 N__159 1T
Cc_143 N__130 N__131 3.5e-13
Rc_143 N__130 N__131 1T
Cc_15 vlo_p N__18 5e-12
Rc_15 vlo_p N__18 1T
Cc_19 vlo_m N__14 5e-12
Rc_19 vlo_m N__14 1T
Cc_37 vlo_m N__36 5e-12
Rc_37 vlo_m N__36 1T
Cc_40 vlo_p N__32 5e-12
Rc_40 vlo_p N__32 1T
Mmn_11 N__30 N__14 N__9 switcher_sub  NCH w=1e-05 l=1.8e-07 m=12    
Mmn_29 N__30 N__32 N__31 switcher_sub  NCH w=1e-05 l=1.8e-07 m=12    
Mmn_33 N__8 N__36 N__31 switcher_sub  NCH w=1e-05 l=1.8e-07 m=12    
Mmn_6 N__123 N__18 N__9 switcher_sub  NCH w=1e-05 l=1.8e-07 m=12    
Rr_146 N__130 gnd! 20000 
Rr_150 gnd! N__131 20000 
Rr_153 N__155 N__156 30 
Rr_22 N__18 N__25 20000 
Rr_26 N__14 N__25 20000 
Rr_43 N__36 N__25 20000 
Rr_46 N__32 N__25 20000 
Rr_68 switcher_sub gnd! 1e-06 
Rr_82 N__81 N__30 5 
Vv_116 vlo_p vlo_m 1 ac 0 0 pulse(1 -1 0 1e-11 1e-11 'period/4.8' 'period/2.4')
Vv_120 N__8 N__123 0 ac 0 0
Vv_124 N__9 N__130 0 ac 0 0
Vv_127 N__31 N__131 0 ac 0 0
Vv_157 N__156 N__159 0 ac 0 0 sin(0 0.08 '2.41/period' 0 0 0)
Vv_49 N__25 gnd! 'Vg' ac 0 0


* Adding Resistor Shunts

RShunt_N__25 N__25 0 10e9 temp=-273
RShunt_N__30 N__30 0 10e9 temp=-273
RShunt_N__31 N__31 0 10e9 temp=-273
RShunt_N__32 N__32 0 10e9 temp=-273
RShunt_N__123 N__123 0 10e9 temp=-273
RShunt_N__36 N__36 0 10e9 temp=-273
RShunt_N__81 N__81 0 10e9 temp=-273
RShunt_gnd! gnd! 0 10e9 temp=-273
RShunt_N__14 N__14 0 10e9 temp=-273
RShunt_vlo_m vlo_m 0 10e9 temp=-273
RShunt_vlo_p vlo_p 0 10e9 temp=-273
RShunt_N__8 N__8 0 10e9 temp=-273
RShunt_N__18 N__18 0 10e9 temp=-273
RShunt_N__9 N__9 0 10e9 temp=-273
RShunt_N__130 N__130 0 10e9 temp=-273
RShunt_N__155 N__155 0 10e9 temp=-273
RShunt_N__131 N__131 0 10e9 temp=-273
RShunt_N__156 N__156 0 10e9 temp=-273
RShunt_switcher_sub switcher_sub 0 10e9 temp=-273
RShunt_N__159 N__159 0 10e9 temp=-273
RShunt_N__135 N__135 0 10e9 temp=-273


* Adding Capacitor Shunts

CShunt_N__25 N__25 0 10e-15
CShunt_N__30 N__30 0 10e-15
CShunt_N__31 N__31 0 10e-15
CShunt_N__32 N__32 0 10e-15
CShunt_N__123 N__123 0 10e-15
CShunt_N__36 N__36 0 10e-15
CShunt_N__81 N__81 0 10e-15
CShunt_gnd! gnd! 0 10e-15
CShunt_N__14 N__14 0 10e-15
CShunt_vlo_m vlo_m 0 10e-15
CShunt_vlo_p vlo_p 0 10e-15
CShunt_N__8 N__8 0 10e-15
CShunt_N__18 N__18 0 10e-15
CShunt_N__9 N__9 0 10e-15
CShunt_N__130 N__130 0 10e-15
CShunt_N__155 N__155 0 10e-15
CShunt_N__131 N__131 0 10e-15
CShunt_N__156 N__156 0 10e-15
CShunt_switcher_sub switcher_sub 0 10e-15
CShunt_N__159 N__159 0 10e-15
CShunt_N__135 N__135 0 10e-15
