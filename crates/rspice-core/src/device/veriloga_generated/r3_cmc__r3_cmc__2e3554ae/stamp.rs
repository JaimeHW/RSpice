#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    ddt_previous_value_scale: f64,
    ddt_older_value_scale: f64,
    ddt_previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        let result = value * ddt_scale
            - previous_value * ddt_previous_value_scale
            - older_value * ddt_older_value_scale
            - derivative_previous[slot] * ddt_previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        current[slot] = value;
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

struct CommonStampValues {
    v0: f64,
    v1: f64,
    v2: f64,
    v58: f64,
    v67: f64,
    v107: f64,
    v153: f64,
    v162: f64,
    v169: f64,
    v225: f64,
    v227: f64,
    v228: f64,
    v229: f64,
    v249: f64,
    v254: f64,
    v256: f64,
    v257: f64,
    v305: f64,
    v398: f64,
    v401: f64,
    v402: f64,
    v407: f64,
    v409: f64,
    v428: f64,
    v430: f64,
    v431: f64,
    v482: f64,
    v531: f64,
    v541: f64,
    v1743: f64,
    v1744: f64,
    v1745: f64,
    v1773: f64,
    v1775: f64,
    v1776: f64,
    v1812: f64,
    v1814: f64,
    v4778: f64,
    v4779: f64,
    v4780: f64,
    v4781: f64,
    v4782: f64,
    v4783: f64,
    v4784: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let v0=1.0;
        let v1=0.0;
        let v2=multiplicity;
        let v12=273.15;
        let v40=1.3806505e-23;
        let v42=1.60217653e-19;
        let v58=2.0;
        let v67=0.5;
        let v107=(v2*self.scalar_static_f64[61]);
        let v108=(v107).sqrt();
        let v111=(if (self.scalar_static_f64[62]!=0.0){(self.scalar_static_f64[66]+(self.scalar_static_f64[69]/v108))}else{self.scalar_static_f64[55]});
        let v120=((v2*self.scalar_static_f64[60])).sqrt();
        let v123=(if (self.scalar_static_f64[62]!=0.0){(self.scalar_static_f64[73]+(self.scalar_static_f64[76]/v120))}else{self.scalar_static_f64[56]});
        let v146=(if self.scalar_static_bool[9]{(self.scalar_static_f64[68]/v108)}else{v1});
        let v153=(if self.scalar_static_bool[9]{(v111+(self.scalar_static_f64[63]*((self.scalar_static_f64[84]+(v146*v146))).sqrt()))}else{v111});
        let v162=(if self.scalar_static_bool[15]{(self.scalar_static_f64[75]/v120)}else{v146});
        let v169=(if self.scalar_static_bool[15]{(v123+(self.scalar_static_f64[70]*((self.scalar_static_f64[86]+(v162*v162))).sqrt()))}else{v123});
        let v196=(if self.scalar_static_bool[24]{self.scalar_static_f64[15]}else{(if (self.scalar_static_f64[91]!=0.0){v153}else{self.scalar_static_f64[60]})});
        let v197=(if self.scalar_static_bool[24]{self.scalar_static_f64[17]}else{(if (self.scalar_static_f64[91]!=0.0){v169}else{self.scalar_static_f64[61]})});
        let v200=(v0/f64::powf(v196,self.scalar_static_f64[92]));
        let v203=(v0/f64::powf(v197,self.scalar_static_f64[93]));
        let v224=((((self.scalar_static_f64[94]*(v0+(v200*self.scalar_static_f64[95])))*(v0+(v203*self.scalar_static_f64[96])))*(v0+(v203*(v200*self.scalar_static_f64[97]))))*self.scalar_static_f64[276]);
        let v225=0.1;
        let v227=(if (v224>v225){v224}else{v225});
        let v228=(v227).sqrt();
        let v229=10000.0;
        let v231=(v228/(v227+v229));
        let v244=(if (self.scalar_static_f64[100]!=0.0){v1}else{(self.scalar_static_f64[101]+((((v197*self.scalar_static_f64[102])+(v196*self.scalar_static_f64[103]))+self.scalar_static_f64[104])/(v196*v197)))});
        let v246=(if (v244<v231){v0}else{v1});
        let v249=(if (v246!=0.0){(if (v244>v1){v244}else{v1})}else{v244});
        let v254=(if (!(v246!=0.0)){(v249*v249)}else{(if (v246!=0.0){(v231*v231)}else{v1})});
        let v256=(v67*v227);
        let v257=((v67/v254)-v256);
        let v305=4.0;
        let v398=ctx.node_voltage(nodes[3]);
        let v401=ctx.node_voltage(nodes[5]);
        let v402=ctx.node_voltage(nodes[4]);
        let v405=ctx.node_voltage(nodes[1]);
        let v407=(self.scalar_static_f64[173]*(v405-v402));
        let v409=(self.scalar_static_f64[173]*(v405-v401));
        let v411=((self.scalar_static_f64[254]+v398)-v12);
        let v413=(if (v411<self.scalar_static_f64[11]){v0}else{v1});
        let v416=(((v411-self.scalar_static_f64[10])-v0)).exp();
        let v418=(if (v413!=0.0){(self.scalar_static_f64[10]+v416)}else{v411});
        let v422=(((if (v418>self.scalar_static_f64[13]){v0}else{v1})!=0.0)&&(!(v413!=0.0)));
        let v425=(((self.scalar_static_f64[12]-v418)-v0)).exp();
        let v428=(v12+(if v422{(self.scalar_static_f64[12]-v425)}else{v418}));
        let v430=((v40*v428)/v42);
        let v431=(v428/self.scalar_static_f64[8]);
        let v482=(v431).ln();
        let v525=(v58*(v430/v431));
        let v528=(v431*self.scalar_static_f64[189]);
        let v530=((v528/v430)).exp();
        let v531=-0.5;
        let v533=(v431*self.scalar_static_f64[190]);
        let v535=((v533/v430)).exp();
        let v536=(v530-v535);
        let v537=(v536).ln();
        let v539=(if (self.scalar_static_f64[187]!=0.0){(v525*v537)}else{v1});
        let v541=3.0;
        let v542=(v430*v541);
        let v543=(v482*v542);
        let v546=(self.scalar_static_f64[179]*(v431-v0));
        let v548=(if (self.scalar_static_f64[187]!=0.0){(((v431*v539)-v543)-v546)}else{v1});
        let v549=(v58*v430);
        let v550=(-v548);
        let v552=((v550/v430)).exp();
        let v555=((v0+(v305*v552))).sqrt();
        let v557=(v67*(v0+v555));
        let v558=(v557).ln();
        let v561=(if (self.scalar_static_f64[187]!=0.0){(v548+(v549*v558))}else{v1});
        let v562=(self.scalar_static_f64[188]/v561);
        let v568=(if self.scalar_static_bool[44]{self.scalar_static_f64[188]}else{v561});
        let v574=(v431*self.scalar_static_f64[194]);
        let v576=((v574/v430)).exp();
        let v578=(v431*self.scalar_static_f64[195]);
        let v580=((v578/v430)).exp();
        let v581=(v576-v580);
        let v582=(v581).ln();
        let v584=(if (self.scalar_static_f64[192]!=0.0){(v525*v582)}else{v1});
        let v588=(if (self.scalar_static_f64[192]!=0.0){(((v431*v584)-v543)-v546)}else{v1});
        let v589=(-v588);
        let v591=((v589/v430)).exp();
        let v594=((v0+(v305*v591))).sqrt();
        let v596=(v67*(v0+v594));
        let v597=(v596).ln();
        let v600=(if (self.scalar_static_f64[192]!=0.0){(v588+(v549*v597))}else{v1});
        let v601=(self.scalar_static_f64[193]/v600);
        let v607=(if self.scalar_static_bool[46]{self.scalar_static_f64[193]}else{v600});
        let v608=(if self.scalar_static_bool[46]{v1}else{(if (self.scalar_static_f64[192]!=0.0){(self.scalar_static_f64[167]*f64::powf(v601,self.scalar_static_f64[196]))}else{v1})});
        let v1393=(v257+v407);
        let v1395=0.04;
        let v1397=(((v1393*v1393)+v1395)).sqrt();
        let v1402=(if self.scalar_static_bool[61]{v407}else{(if self.scalar_static_bool[60]{(v67*((v407-v257)+v1397))}else{v1})});
        let v1403=(self.scalar_static_f64[20]*(if self.scalar_static_bool[44]{v1}else{(if (self.scalar_static_f64[187]!=0.0){(self.scalar_static_f64[165]*f64::powf(v562,self.scalar_static_f64[191]))}else{v1})}));
        let v1404=(if (self.scalar_static_f64[221]!=0.0){v1403}else{v1});
        let v1406=(if (self.scalar_static_f64[221]!=0.0){(self.scalar_static_f64[22]*v608)}else{v1});
        let v1408=(if (v1404>v1){v0}else{v1});
        let v1409=((self.scalar_static_f64[221]!=0.0)&&(v1408!=0.0));
        let v1410=(-v568);
        let v1412=(v1410*self.scalar_static_f64[222]);
        let v1413=(if v1409{v1412}else{v1});
        let v1417=(v1409&&(self.scalar_static_f64[224]!=0.0));
        let v1418=(v1402+v1413);
        let v1419=(if v1417{v1418}else{v1});
        let v1421=(if (v1419>v1){v0}else{v1});
        let v1422=(v1417&&(v1421!=0.0));
        let v1426=(if v1422{self.scalar_static_f64[227]}else{v1});
        let v1428=(v0-(self.scalar_static_f64[225]*v1426));
        let v1434=(v1419*self.scalar_static_f64[229]);
        let v1435=(v568*self.scalar_static_f64[225]);
        let v1437=(v0+(v1434/v1435));
        let v1442=(v1417&&(!(v1421!=0.0)));
        let v1444=(v0-(v1402/v568));
        let v1446=(v0-f64::powf(v1444,self.scalar_static_f64[228]));
        let v1449=(if v1442{((v568*v1446)/self.scalar_static_f64[228])}else{(if v1422{((v568*v1428)/self.scalar_static_f64[228])}else{v1})});
        let v1454=(v1409&&self.scalar_static_bool[63]);
        let v1459=(((v1413*v1413)+self.scalar_static_f64[231])).sqrt();
        let v1464=(if v1454{v1418}else{v1});
        let v1467=((self.scalar_static_f64[231]+(v1464*v1464))).sqrt();
        let v1472=(if v1454{((v67*(v1464-(if v1454{v1467}else{v1})))-v1413)}else{v1});
        let v1474=(v0-(v1472/v568));
        let v1475=f64::powf(v1474,self.scalar_static_f64[228]);
        let v1480=((if v1454{(v531*(v1413+(if v1454{v1459}else{v1})))}else{v1})+(v1402-v1472));
        let v1481=(self.scalar_static_f64[227]*v1480);
        let v1482=(self.scalar_static_f64[229]*v1480);
        let v1484=(v0+(v1482/v1435));
        let v1489=((self.scalar_static_f64[221]!=0.0)&&(!(v1408!=0.0)));
        let v1490=(if v1489{v1}else{(if v1454{((if v1454{((v1410*v1475)/self.scalar_static_f64[228])}else{v1449})+(v1481*v1484))}else{(if v1417{(v1449+(if v1442{v1}else{(if v1422{(v1426*(v1419*v1437))}else{v1})}))}else{v1})})});
        let v1492=(if (v1406>v1){v0}else{v1});
        let v1493=((self.scalar_static_f64[221]!=0.0)&&(v1492!=0.0));
        let v1494=(-v607);
        let v1495=(self.scalar_static_f64[222]*v1494);
        let v1496=(if v1493{v1495}else{v1});
        let v1500=(v1493&&(self.scalar_static_f64[233]!=0.0));
        let v1501=(v1402+v1496);
        let v1502=(if v1500{v1501}else{v1});
        let v1504=(if (v1502>v1){v0}else{v1});
        let v1505=(v1500&&(v1504!=0.0));
        let v1508=(if v1505{self.scalar_static_f64[235]}else{v1});
        let v1510=(v0-(self.scalar_static_f64[225]*v1508));
        let v1516=(v1502*self.scalar_static_f64[237]);
        let v1517=(v607*self.scalar_static_f64[225]);
        let v1519=(v0+(v1516/v1517));
        let v1524=(v1500&&(!(v1504!=0.0)));
        let v1526=(v0-(v1402/v607));
        let v1528=(v0-f64::powf(v1526,self.scalar_static_f64[236]));
        let v1531=(if v1524{((v607*v1528)/self.scalar_static_f64[236])}else{(if v1505{((v607*v1510)/self.scalar_static_f64[236])}else{v1})});
        let v1536=(v1493&&self.scalar_static_bool[65]);
        let v1541=(((v1496*v1496)+self.scalar_static_f64[239])).sqrt();
        let v1546=(if v1536{v1501}else{v1});
        let v1549=((self.scalar_static_f64[239]+(v1546*v1546))).sqrt();
        let v1554=(if v1536{((v67*(v1546-(if v1536{v1549}else{v1})))-v1496)}else{v1});
        let v1556=(v0-(v1554/v607));
        let v1557=f64::powf(v1556,self.scalar_static_f64[236]);
        let v1562=((if v1536{(v531*(v1496+(if v1536{v1541}else{v1})))}else{v1})+(v1402-v1554));
        let v1563=(self.scalar_static_f64[235]*v1562);
        let v1564=(self.scalar_static_f64[237]*v1562);
        let v1566=(v0+(v1564/v1517));
        let v1571=((self.scalar_static_f64[221]!=0.0)&&(!(v1492!=0.0)));
        let v1572=(if v1571{v1}else{(if v1536{((if v1536{((v1494*v1557)/self.scalar_static_f64[236])}else{v1531})+(v1563*v1566))}else{(if v1500{(v1531+(if v1524{v1}else{(if v1505{(v1508*(v1502*v1519))}else{v1})}))}else{v1})})});
        let v1583=(v257+v409);
        let v1586=((v1395+(v1583*v1583))).sqrt();
        let v1591=(if self.scalar_static_bool[69]{v409}else{(if self.scalar_static_bool[68]{(v67*((v409-v257)+v1586))}else{v1402})});
        let v1592=(if (self.scalar_static_f64[240]!=0.0){v1403}else{v1});
        let v1594=(if (self.scalar_static_f64[240]!=0.0){(self.scalar_static_f64[24]*v608)}else{v1});
        let v1596=(if (v1592>v1){v0}else{v1});
        let v1597=((self.scalar_static_f64[240]!=0.0)&&(v1596!=0.0));
        let v1598=(if v1597{v1412}else{v1});
        let v1599=((self.scalar_static_f64[224]!=0.0)&&v1597);
        let v1600=(v1591+v1598);
        let v1601=(if v1599{v1600}else{v1});
        let v1603=(if (v1601>v1){v0}else{v1});
        let v1604=(v1599&&(v1603!=0.0));
        let v1605=(if v1604{self.scalar_static_f64[227]}else{v1});
        let v1607=(v0-(self.scalar_static_f64[225]*v1605));
        let v1611=(self.scalar_static_f64[229]*v1601);
        let v1613=(v0+(v1611/v1435));
        let v1618=(v1599&&(!(v1603!=0.0)));
        let v1620=(v0-(v1591/v568));
        let v1622=(v0-f64::powf(v1620,self.scalar_static_f64[228]));
        let v1625=(if v1618{((v568*v1622)/self.scalar_static_f64[228])}else{(if v1604{((v568*v1607)/self.scalar_static_f64[228])}else{v1})});
        let v1629=(self.scalar_static_bool[63]&&v1597);
        let v1632=((self.scalar_static_f64[231]+(v1598*v1598))).sqrt();
        let v1637=(if v1629{v1600}else{v1});
        let v1640=((self.scalar_static_f64[231]+(v1637*v1637))).sqrt();
        let v1645=(if v1629{((v67*(v1637-(if v1629{v1640}else{v1})))-v1598)}else{v1});
        let v1647=(v0-(v1645/v568));
        let v1648=f64::powf(v1647,self.scalar_static_f64[228]);
        let v1653=((if v1629{(v531*(v1598+(if v1629{v1632}else{v1})))}else{v1})+(v1591-v1645));
        let v1654=(self.scalar_static_f64[227]*v1653);
        let v1655=(self.scalar_static_f64[229]*v1653);
        let v1657=(v0+(v1655/v1435));
        let v1662=((self.scalar_static_f64[240]!=0.0)&&(!(v1596!=0.0)));
        let v1663=(if v1662{v1}else{(if v1629{((if v1629{((v1410*v1648)/self.scalar_static_f64[228])}else{v1625})+(v1654*v1657))}else{(if v1599{(v1625+(if v1618{v1}else{(if v1604{(v1605*(v1601*v1613))}else{v1})}))}else{v1})})});
        let v1665=(if (v1594>v1){v0}else{v1});
        let v1666=((self.scalar_static_f64[240]!=0.0)&&(v1665!=0.0));
        let v1667=(if v1666{v1495}else{v1});
        let v1668=((self.scalar_static_f64[233]!=0.0)&&v1666);
        let v1669=(v1591+v1667);
        let v1670=(if v1668{v1669}else{v1});
        let v1672=(if (v1670>v1){v0}else{v1});
        let v1673=(v1668&&(v1672!=0.0));
        let v1674=(if v1673{self.scalar_static_f64[235]}else{v1});
        let v1676=(v0-(self.scalar_static_f64[225]*v1674));
        let v1680=(self.scalar_static_f64[237]*v1670);
        let v1682=(v0+(v1680/v1517));
        let v1687=(v1668&&(!(v1672!=0.0)));
        let v1689=(v0-(v1591/v607));
        let v1691=(v0-f64::powf(v1689,self.scalar_static_f64[236]));
        let v1694=(if v1687{((v607*v1691)/self.scalar_static_f64[236])}else{(if v1673{((v607*v1676)/self.scalar_static_f64[236])}else{v1})});
        let v1698=(self.scalar_static_bool[65]&&v1666);
        let v1701=((self.scalar_static_f64[239]+(v1667*v1667))).sqrt();
        let v1706=(if v1698{v1669}else{v1});
        let v1709=((self.scalar_static_f64[239]+(v1706*v1706))).sqrt();
        let v1714=(if v1698{((v67*(v1706-(if v1698{v1709}else{v1})))-v1667)}else{v1});
        let v1716=(v0-(v1714/v607));
        let v1717=f64::powf(v1716,self.scalar_static_f64[236]);
        let v1722=((if v1698{(v531*(v1667+(if v1698{v1701}else{v1})))}else{v1})+(v1591-v1714));
        let v1723=(self.scalar_static_f64[235]*v1722);
        let v1724=(self.scalar_static_f64[237]*v1722);
        let v1726=(v0+(v1724/v1517));
        let v1731=((self.scalar_static_f64[240]!=0.0)&&(!(v1665!=0.0)));
        let v1732=(if v1731{v1}else{(if v1698{((if v1698{((v1494*v1717)/self.scalar_static_f64[236])}else{v1694})+(v1723*v1726))}else{(if v1668{(v1694+(if v1687{v1}else{(if v1673{(v1674*(v1670*v1682))}else{v1})}))}else{v1})})});
        let v1743=(self.scalar_static_f64[173]*((if self.scalar_static_bool[66]{v1}else{(if (self.scalar_static_f64[221]!=0.0){((v1404*v1490)+(v1406*v1572))}else{v1})})+(self.scalar_static_f64[162]*v407)));
        let v1744=(self.scalar_static_f64[173]*((if self.scalar_static_bool[70]{v1}else{(if (self.scalar_static_f64[240]!=0.0){((v1592*v1663)+(v1594*v1732))}else{v1})})+(self.scalar_static_f64[164]*v409)));
        let v1745=(self.scalar_static_f64[149]*v398);
        let v1769=(if (v413!=0.0){v416}else{v0});
        let v1773=(if v422{(-(v425*(-v1769)))}else{v1769});
        let v1775=((v40*v1773)/v42);
        let v1776=(v1773/self.scalar_static_f64[8]);
        let v1812=(v430*v430);
        let v1814=(v1776/v431);
        let v1857=(v58*(((v431*v1775)-(v430*v1776))/(v431*v431)));
        let v1882=((v542*v1814)+(v482*(v541*v1775)));
        let v1884=(self.scalar_static_f64[179]*v1776);
        let v1886=(if (self.scalar_static_f64[187]!=0.0){((((v539*v1776)+(v431*(if (self.scalar_static_f64[187]!=0.0){((v537*v1857)+(v525*(((v530*(((v430*(self.scalar_static_f64[189]*v1776))-(v528*v1775))/v1812))-(v535*(((v430*(self.scalar_static_f64[190]*v1776))-(v533*v1775))/v1812)))/v536)))}else{v1})))-v1882)-v1884)}else{v1});
        let v1887=(v58*v1775);
        let v1903=(if (self.scalar_static_f64[187]!=0.0){(v1886+((v558*v1887)+(v549*((v67*((v305*(v552*(((v430*(-v1886))-(v550*v1775))/v1812)))/(v58*v555)))/v557))))}else{v1});
        let v1914=(if self.scalar_static_bool[44]{v1}else{v1903});
        let v1939=(if (self.scalar_static_f64[192]!=0.0){((((v584*v1776)+(v431*(if (self.scalar_static_f64[192]!=0.0){((v582*v1857)+(v525*(((v576*(((v430*(self.scalar_static_f64[194]*v1776))-(v574*v1775))/v1812))-(v580*(((v430*(self.scalar_static_f64[195]*v1776))-(v578*v1775))/v1812)))/v581)))}else{v1})))-v1882)-v1884)}else{v1});
        let v1955=(if (self.scalar_static_f64[192]!=0.0){(v1939+((v597*v1887)+(v549*((v67*((v305*(v591*(((v430*(-v1939))-(v589*v1775))/v1812)))/(v58*v594)))/v596))))}else{v1});
        let v1966=(if self.scalar_static_bool[46]{v1}else{v1955});
        let v1967=(if self.scalar_static_bool[46]{v1}else{(if (self.scalar_static_f64[192]!=0.0){(self.scalar_static_f64[167]*(((-(self.scalar_static_f64[193]*v1955))/(v600*v600))*(self.scalar_static_f64[196]*f64::powf(v601,self.scalar_static_f64[244]))))}else{v1})});
        let v3952=(self.scalar_static_f64[173]*v1393);
        let v3954=(self.scalar_static_f64[172]*v1393);
        let v3956=(v58*v1397);
        let v3965=(if self.scalar_static_bool[61]{self.scalar_static_f64[173]}else{(if self.scalar_static_bool[60]{(v67*(self.scalar_static_f64[173]+((v3952+v3952)/v3956)))}else{v1})});
        let v3966=(if self.scalar_static_bool[61]{self.scalar_static_f64[172]}else{(if self.scalar_static_bool[60]{(v67*(self.scalar_static_f64[172]+((v3954+v3954)/v3956)))}else{v1})});
        let v3967=(self.scalar_static_f64[20]*(if self.scalar_static_bool[44]{v1}else{(if (self.scalar_static_f64[187]!=0.0){(self.scalar_static_f64[165]*(((-(self.scalar_static_f64[188]*v1903))/(v561*v561))*(self.scalar_static_f64[191]*f64::powf(v562,self.scalar_static_f64[243]))))}else{v1})}));
        let v3971=(-v1914);
        let v3972=(self.scalar_static_f64[222]*v3971);
        let v3973=(if v1409{v3972}else{v1});
        let v3974=(if v1417{v3965}else{v1});
        let v3975=(if v1417{v3973}else{v1});
        let v3976=(if v1417{v3966}else{v1});
        let v3983=(self.scalar_static_f64[225]*v1914);
        let v3988=(v1435*v1435);
        let v4009=(v568*v568);
        let v4017=(self.scalar_static_f64[228]*f64::powf(v1444,self.scalar_static_f64[248]));
        let v4032=(if v1442{((v568*(-((-(v3965/v568))*v4017)))/self.scalar_static_f64[228])}else{v1});
        let v4033=(if v1442{(((v1446*v1914)+(v568*(-((-((-(v1402*v1914))/v4009))*v4017))))/self.scalar_static_f64[228])}else{(if v1422{((v1428*v1914)/self.scalar_static_f64[228])}else{v1})});
        let v4034=(if v1442{((v568*(-((-(v3966/v568))*v4017)))/self.scalar_static_f64[228])}else{v1});
        let v4044=(v1413*v3973);
        let v4052=(if v1454{v3965}else{v1});
        let v4053=(if v1454{v3973}else{v1});
        let v4054=(if v1454{v3966}else{v1});
        let v4055=(v1464*v4052);
        let v4057=(v1464*v4053);
        let v4059=(v1464*v4054);
        let v4061=(v58*v1467);
        let v4075=(if v1454{(v67*(v4052-(if v1454{((v4055+v4055)/v4061)}else{v1})))}else{v1});
        let v4076=(if v1454{((v67*(v4053-(if v1454{((v4057+v4057)/v4061)}else{v1})))-v3973)}else{v1});
        let v4077=(if v1454{(v67*(v4054-(if v1454{((v4059+v4059)/v4061)}else{v1})))}else{v1});
        let v4088=(self.scalar_static_f64[228]*f64::powf(v1474,self.scalar_static_f64[248]));
        let v4103=(v3965-v4075);
        let v4105=(v3966-v4077);
        let v4106=((if v1454{(v531*(v3973+(if v1454{((v4044+v4044)/(v58*v1459))}else{v1})))}else{v1})+(-v4076));
        let v4137=(-v1966);
        let v4138=(self.scalar_static_f64[222]*v4137);
        let v4139=(if v1493{v4138}else{v1});
        let v4140=(if v1500{v3965}else{v1});
        let v4141=(if v1500{v4139}else{v1});
        let v4142=(if v1500{v3966}else{v1});
        let v4149=(self.scalar_static_f64[225]*v1966);
        let v4154=(v1517*v1517);
        let v4175=(v607*v607);
        let v4183=(self.scalar_static_f64[236]*f64::powf(v1526,self.scalar_static_f64[249]));
        let v4198=(if v1524{((v607*(-((-(v3965/v607))*v4183)))/self.scalar_static_f64[236])}else{v1});
        let v4199=(if v1524{(((v1528*v1966)+(v607*(-((-((-(v1402*v1966))/v4175))*v4183))))/self.scalar_static_f64[236])}else{(if v1505{((v1510*v1966)/self.scalar_static_f64[236])}else{v1})});
        let v4200=(if v1524{((v607*(-((-(v3966/v607))*v4183)))/self.scalar_static_f64[236])}else{v1});
        let v4210=(v1496*v4139);
        let v4218=(if v1536{v3965}else{v1});
        let v4219=(if v1536{v4139}else{v1});
        let v4220=(if v1536{v3966}else{v1});
        let v4221=(v1546*v4218);
        let v4223=(v1546*v4219);
        let v4225=(v1546*v4220);
        let v4227=(v58*v1549);
        let v4241=(if v1536{(v67*(v4218-(if v1536{((v4221+v4221)/v4227)}else{v1})))}else{v1});
        let v4242=(if v1536{((v67*(v4219-(if v1536{((v4223+v4223)/v4227)}else{v1})))-v4139)}else{v1});
        let v4243=(if v1536{(v67*(v4220-(if v1536{((v4225+v4225)/v4227)}else{v1})))}else{v1});
        let v4254=(self.scalar_static_f64[236]*f64::powf(v1556,self.scalar_static_f64[249]));
        let v4269=(v3965-v4241);
        let v4271=(v3966-v4243);
        let v4272=((if v1536{(v531*(v4139+(if v1536{((v4210+v4210)/(v58*v1541))}else{v1})))}else{v1})+(-v4242));
        let v4314=(((v1490*(if (self.scalar_static_f64[221]!=0.0){v3967}else{v1}))+(v1404*(if v1489{v1}else{(if v1454{((if v1454{(((v1475*v3971)+(v1410*((-(((v568*v4076)-(v1472*v1914))/v4009))*v4088)))/self.scalar_static_f64[228])}else{v4033})+((v1484*(self.scalar_static_f64[227]*v4106))+(v1481*(((v1435*(self.scalar_static_f64[229]*v4106))-(v1482*v3983))/v3988))))}else{(if v1417{(v4033+(if v1442{v1}else{(if v1422{(v1426*((v1437*v3975)+(v1419*(((v1435*(self.scalar_static_f64[229]*v3975))-(v1434*v3983))/v3988))))}else{v1})}))}else{v1})})})))+((v1572*(if (self.scalar_static_f64[221]!=0.0){(self.scalar_static_f64[22]*v1967)}else{v1}))+(v1406*(if v1571{v1}else{(if v1536{((if v1536{(((v1557*v4137)+(v1494*((-(((v607*v4242)-(v1554*v1966))/v4175))*v4254)))/self.scalar_static_f64[236])}else{v4199})+((v1566*(self.scalar_static_f64[235]*v4272))+(v1563*(((v1517*(self.scalar_static_f64[237]*v4272))-(v1564*v4149))/v4154))))}else{(if v1500{(v4199+(if v1524{v1}else{(if v1505{(v1508*((v1519*v4141)+(v1502*(((v1517*(self.scalar_static_f64[237]*v4141))-(v1516*v4149))/v4154))))}else{v1})}))}else{v1})})}))));
        let v4322=(self.scalar_static_f64[173]*v1583);
        let v4324=(self.scalar_static_f64[172]*v1583);
        let v4326=(v58*v1586);
        let v4336=(if self.scalar_static_bool[69]{self.scalar_static_f64[173]}else{(if self.scalar_static_bool[68]{(v67*(self.scalar_static_f64[173]+((v4322+v4322)/v4326)))}else{v3965})});
        let v4337=(if self.scalar_static_bool[69]{v1}else{(if self.scalar_static_bool[68]{v1}else{v3966})});
        let v4338=(if self.scalar_static_bool[69]{self.scalar_static_f64[172]}else{(if self.scalar_static_bool[68]{(v67*(self.scalar_static_f64[172]+((v4324+v4324)/v4326)))}else{v1})});
        let v4342=(if v1597{v3972}else{v1});
        let v4343=(if v1599{v4336}else{v1});
        let v4344=(if v1599{v4342}else{v1});
        let v4345=(if v1599{v4337}else{v1});
        let v4346=(if v1599{v4338}else{v1});
        let v4392=(self.scalar_static_f64[228]*f64::powf(v1620,self.scalar_static_f64[248]));
        let v4411=(if v1618{((v568*(-((-(v4336/v568))*v4392)))/self.scalar_static_f64[228])}else{v1});
        let v4412=(if v1618{(((v1622*v1914)+(v568*(-((-((-(v1591*v1914))/v4009))*v4392))))/self.scalar_static_f64[228])}else{(if v1604{((v1607*v1914)/self.scalar_static_f64[228])}else{v1})});
        let v4413=(if v1618{((v568*(-((-(v4337/v568))*v4392)))/self.scalar_static_f64[228])}else{v1});
        let v4414=(if v1618{((v568*(-((-(v4338/v568))*v4392)))/self.scalar_static_f64[228])}else{v1});
        let v4427=(v1598*v4342);
        let v4435=(if v1629{v4336}else{v1});
        let v4436=(if v1629{v4342}else{v1});
        let v4437=(if v1629{v4337}else{v1});
        let v4438=(if v1629{v4338}else{v1});
        let v4439=(v1637*v4435);
        let v4441=(v1637*v4436);
        let v4443=(v1637*v4437);
        let v4445=(v1637*v4438);
        let v4447=(v58*v1640);
        let v4465=(if v1629{(v67*(v4435-(if v1629{((v4439+v4439)/v4447)}else{v1})))}else{v1});
        let v4466=(if v1629{((v67*(v4436-(if v1629{((v4441+v4441)/v4447)}else{v1})))-v4342)}else{v1});
        let v4467=(if v1629{(v67*(v4437-(if v1629{((v4443+v4443)/v4447)}else{v1})))}else{v1});
        let v4468=(if v1629{(v67*(v4438-(if v1629{((v4445+v4445)/v4447)}else{v1})))}else{v1});
        let v4481=(self.scalar_static_f64[228]*f64::powf(v1647,self.scalar_static_f64[248]));
        let v4500=(v4336-v4465);
        let v4502=(v4337-v4467);
        let v4503=(v4338-v4468);
        let v4504=((if v1629{(v531*(v4342+(if v1629{((v4427+v4427)/(v58*v1632))}else{v1})))}else{v1})+(-v4466));
        let v4544=(if v1666{v4138}else{v1});
        let v4545=(if v1668{v4336}else{v1});
        let v4546=(if v1668{v4544}else{v1});
        let v4547=(if v1668{v4337}else{v1});
        let v4548=(if v1668{v4338}else{v1});
        let v4594=(self.scalar_static_f64[236]*f64::powf(v1689,self.scalar_static_f64[249]));
        let v4613=(if v1687{((v607*(-((-(v4336/v607))*v4594)))/self.scalar_static_f64[236])}else{v1});
        let v4614=(if v1687{(((v1691*v1966)+(v607*(-((-((-(v1591*v1966))/v4175))*v4594))))/self.scalar_static_f64[236])}else{(if v1673{((v1676*v1966)/self.scalar_static_f64[236])}else{v1})});
        let v4615=(if v1687{((v607*(-((-(v4337/v607))*v4594)))/self.scalar_static_f64[236])}else{v1});
        let v4616=(if v1687{((v607*(-((-(v4338/v607))*v4594)))/self.scalar_static_f64[236])}else{v1});
        let v4629=(v1667*v4544);
        let v4637=(if v1698{v4336}else{v1});
        let v4638=(if v1698{v4544}else{v1});
        let v4639=(if v1698{v4337}else{v1});
        let v4640=(if v1698{v4338}else{v1});
        let v4641=(v1706*v4637);
        let v4643=(v1706*v4638);
        let v4645=(v1706*v4639);
        let v4647=(v1706*v4640);
        let v4649=(v58*v1709);
        let v4667=(if v1698{(v67*(v4637-(if v1698{((v4641+v4641)/v4649)}else{v1})))}else{v1});
        let v4668=(if v1698{((v67*(v4638-(if v1698{((v4643+v4643)/v4649)}else{v1})))-v4544)}else{v1});
        let v4669=(if v1698{(v67*(v4639-(if v1698{((v4645+v4645)/v4649)}else{v1})))}else{v1});
        let v4670=(if v1698{(v67*(v4640-(if v1698{((v4647+v4647)/v4649)}else{v1})))}else{v1});
        let v4683=(self.scalar_static_f64[236]*f64::powf(v1716,self.scalar_static_f64[249]));
        let v4702=(v4336-v4667);
        let v4704=(v4337-v4669);
        let v4705=(v4338-v4670);
        let v4706=((if v1698{(v531*(v4544+(if v1698{((v4629+v4629)/(v58*v1701))}else{v1})))}else{v1})+(-v4668));
        let v4759=(((v1663*(if (self.scalar_static_f64[240]!=0.0){v3967}else{v1}))+(v1592*(if v1662{v1}else{(if v1629{((if v1629{(((v1648*v3971)+(v1410*((-(((v568*v4466)-(v1645*v1914))/v4009))*v4481)))/self.scalar_static_f64[228])}else{v4412})+((v1657*(self.scalar_static_f64[227]*v4504))+(v1654*(((v1435*(self.scalar_static_f64[229]*v4504))-(v1655*v3983))/v3988))))}else{(if v1599{(v4412+(if v1618{v1}else{(if v1604{(v1605*((v1613*v4344)+(v1601*(((v1435*(self.scalar_static_f64[229]*v4344))-(v1611*v3983))/v3988))))}else{v1})}))}else{v1})})})))+((v1732*(if (self.scalar_static_f64[240]!=0.0){(self.scalar_static_f64[24]*v1967)}else{v1}))+(v1594*(if v1731{v1}else{(if v1698{((if v1698{(((v1717*v4137)+(v1494*((-(((v607*v4668)-(v1714*v1966))/v4175))*v4683)))/self.scalar_static_f64[236])}else{v4614})+((v1726*(self.scalar_static_f64[235]*v4706))+(v1723*(((v1517*(self.scalar_static_f64[237]*v4706))-(v1724*v4149))/v4154))))}else{(if v1668{(v4614+(if v1687{v1}else{(if v1673{(v1674*((v1682*v4546)+(v1670*(((v1517*(self.scalar_static_f64[237]*v4546))-(v1680*v4149))/v4154))))}else{v1})}))}else{v1})})}))));
        let v4778=(self.scalar_static_f64[173]*((if self.scalar_static_bool[66]{v1}else{(if (self.scalar_static_f64[221]!=0.0){((v1404*(if v1489{v1}else{(if v1454{((if v1454{((v1410*((-(v4075/v568))*v4088))/self.scalar_static_f64[228])}else{v4032})+((v1484*(self.scalar_static_f64[227]*v4103))+(v1481*((self.scalar_static_f64[229]*v4103)/v1435))))}else{(if v1417{(v4032+(if v1442{v1}else{(if v1422{(v1426*((v1437*v3974)+(v1419*((self.scalar_static_f64[229]*v3974)/v1435))))}else{v1})}))}else{v1})})}))+(v1406*(if v1571{v1}else{(if v1536{((if v1536{((v1494*((-(v4241/v607))*v4254))/self.scalar_static_f64[236])}else{v4198})+((v1566*(self.scalar_static_f64[235]*v4269))+(v1563*((self.scalar_static_f64[237]*v4269)/v1517))))}else{(if v1500{(v4198+(if v1524{v1}else{(if v1505{(v1508*((v1519*v4140)+(v1502*((self.scalar_static_f64[237]*v4140)/v1517))))}else{v1})}))}else{v1})})})))}else{v1})})+self.scalar_static_f64[250]));
        let v4779=(self.scalar_static_f64[173]*(if self.scalar_static_bool[66]{v1}else{(if (self.scalar_static_f64[221]!=0.0){v4314}else{v1})}));
        let v4780=(self.scalar_static_f64[173]*((if self.scalar_static_bool[66]{v1}else{(if (self.scalar_static_f64[221]!=0.0){((v1404*(if v1489{v1}else{(if v1454{((if v1454{((v1410*((-(v4077/v568))*v4088))/self.scalar_static_f64[228])}else{v4034})+((v1484*(self.scalar_static_f64[227]*v4105))+(v1481*((self.scalar_static_f64[229]*v4105)/v1435))))}else{(if v1417{(v4034+(if v1442{v1}else{(if v1422{(v1426*((v1437*v3976)+(v1419*((self.scalar_static_f64[229]*v3976)/v1435))))}else{v1})}))}else{v1})})}))+(v1406*(if v1571{v1}else{(if v1536{((if v1536{((v1494*((-(v4243/v607))*v4254))/self.scalar_static_f64[236])}else{v4200})+((v1566*(self.scalar_static_f64[235]*v4271))+(v1563*((self.scalar_static_f64[237]*v4271)/v1517))))}else{(if v1500{(v4200+(if v1524{v1}else{(if v1505{(v1508*((v1519*v4142)+(v1502*((self.scalar_static_f64[237]*v4142)/v1517))))}else{v1})}))}else{v1})})})))}else{v1})})+self.scalar_static_f64[251]));
        let v4781=(self.scalar_static_f64[173]*((if self.scalar_static_bool[70]{v1}else{(if (self.scalar_static_f64[240]!=0.0){((v1592*(if v1662{v1}else{(if v1629{((if v1629{((v1410*((-(v4465/v568))*v4481))/self.scalar_static_f64[228])}else{v4411})+((v1657*(self.scalar_static_f64[227]*v4500))+(v1654*((self.scalar_static_f64[229]*v4500)/v1435))))}else{(if v1599{(v4411+(if v1618{v1}else{(if v1604{(v1605*((v1613*v4343)+(v1601*((self.scalar_static_f64[229]*v4343)/v1435))))}else{v1})}))}else{v1})})}))+(v1594*(if v1731{v1}else{(if v1698{((if v1698{((v1494*((-(v4667/v607))*v4683))/self.scalar_static_f64[236])}else{v4613})+((v1726*(self.scalar_static_f64[235]*v4702))+(v1723*((self.scalar_static_f64[237]*v4702)/v1517))))}else{(if v1668{(v4613+(if v1687{v1}else{(if v1673{(v1674*((v1682*v4545)+(v1670*((self.scalar_static_f64[237]*v4545)/v1517))))}else{v1})}))}else{v1})})})))}else{v1})})+self.scalar_static_f64[252]));
        let v4782=(self.scalar_static_f64[173]*(if self.scalar_static_bool[70]{v1}else{(if (self.scalar_static_f64[240]!=0.0){v4759}else{v1})}));
        let v4783=(self.scalar_static_f64[173]*(if self.scalar_static_bool[70]{v1}else{(if (self.scalar_static_f64[240]!=0.0){((v1592*(if v1662{v1}else{(if v1629{((if v1629{((v1410*((-(v4467/v568))*v4481))/self.scalar_static_f64[228])}else{v4413})+((v1657*(self.scalar_static_f64[227]*v4502))+(v1654*((self.scalar_static_f64[229]*v4502)/v1435))))}else{(if v1599{(v4413+(if v1618{v1}else{(if v1604{(v1605*((v1613*v4345)+(v1601*((self.scalar_static_f64[229]*v4345)/v1435))))}else{v1})}))}else{v1})})}))+(v1594*(if v1731{v1}else{(if v1698{((if v1698{((v1494*((-(v4669/v607))*v4683))/self.scalar_static_f64[236])}else{v4615})+((v1726*(self.scalar_static_f64[235]*v4704))+(v1723*((self.scalar_static_f64[237]*v4704)/v1517))))}else{(if v1668{(v4615+(if v1687{v1}else{(if v1673{(v1674*((v1682*v4547)+(v1670*((self.scalar_static_f64[237]*v4547)/v1517))))}else{v1})}))}else{v1})})})))}else{v1})}));
        let v4784=(self.scalar_static_f64[173]*((if self.scalar_static_bool[70]{v1}else{(if (self.scalar_static_f64[240]!=0.0){((v1592*(if v1662{v1}else{(if v1629{((if v1629{((v1410*((-(v4468/v568))*v4481))/self.scalar_static_f64[228])}else{v4414})+((v1657*(self.scalar_static_f64[227]*v4503))+(v1654*((self.scalar_static_f64[229]*v4503)/v1435))))}else{(if v1599{(v4414+(if v1618{v1}else{(if v1604{(v1605*((v1613*v4346)+(v1601*((self.scalar_static_f64[229]*v4346)/v1435))))}else{v1})}))}else{v1})})}))+(v1594*(if v1731{v1}else{(if v1698{((if v1698{((v1494*((-(v4670/v607))*v4683))/self.scalar_static_f64[236])}else{v4616})+((v1726*(self.scalar_static_f64[235]*v4705))+(v1723*((self.scalar_static_f64[237]*v4705)/v1517))))}else{(if v1668{(v4616+(if v1687{v1}else{(if v1673{(v1674*((v1682*v4548)+(v1670*((self.scalar_static_f64[237]*v4548)/v1517))))}else{v1})}))}else{v1})})})))}else{v1})})+self.scalar_static_f64[253]));

        CommonStampValues {
            v0,
            v1,
            v2,
            v58,
            v67,
            v107,
            v153,
            v162,
            v169,
            v225,
            v227,
            v228,
            v229,
            v249,
            v254,
            v256,
            v257,
            v305,
            v398,
            v401,
            v402,
            v407,
            v409,
            v428,
            v430,
            v431,
            v482,
            v531,
            v541,
            v1743,
            v1744,
            v1745,
            v1773,
            v1775,
            v1776,
            v1812,
            v1814,
            v4778,
            v4779,
            v4780,
            v4781,
            v4782,
            v4783,
            v4784,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let timestep = self.timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_older = self.ddt_state_older.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_derivative_current = self.ddt_derivative_current.as_mut();
        let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_scale = self.ddt_coefficients.derivative_scale;
        let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;
        let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;
        let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;
        let common=self.eval_common_stamp_values(ctx);
        let v3=0.01;
        let v9=1000000.0;
        let v131=((self.scalar_static_f64[60]*common.v107)).sqrt();
        let v178=(if self.scalar_static_bool[21]{(self.scalar_static_f64[81]/v131)}else{common.v162});
        let v191=(common.v169+self.scalar_static_f64[90]);
        let v263=(self.scalar_static_f64[108]/common.v254);
        let v280=(if self.scalar_static_bool[30]{common.v257}else{(if self.scalar_static_bool[28]{(common.v257-(v263).sqrt())}else{(if (self.scalar_static_f64[106]!=0.0){(common.v257-v263)}else{common.v1})})});
        let v286=(self.scalar_static_f64[110]/(common.v0+(self.scalar_static_f64[111]/common.v169)));
        let v307=(v286*(v286*common.v305));
        let v310=(if self.scalar_static_bool[30]{v307}else{(if self.scalar_static_bool[28]{v307}else{(if (self.scalar_static_f64[106]!=0.0){(if self.scalar_static_bool[31]{(self.scalar_static_f64[279]*(common.v0+(((-v286)/self.scalar_static_f64[270])).exp()))}else{self.scalar_static_f64[280]})}else{common.v1})})});
        let v316=(common.v0-(common.v228*common.v249));
        let v317=((((if self.scalar_static_bool[23]{common.v0}else{(if self.scalar_static_bool[21]{((self.scalar_static_f64[88]*((self.scalar_static_f64[89]+(v178*v178))).sqrt())).exp()}else{(if (self.scalar_static_f64[62]!=0.0){((v3*(self.scalar_static_f64[79]+(self.scalar_static_f64[82]/v131)))).exp()}else{common.v1})})})*self.scalar_static_f64[114])*(common.v169/common.v153))*v316);
        let v383=((self.scalar_static_f64[154]+(self.scalar_static_f64[155]/common.v153))+((self.scalar_static_f64[32]*(self.scalar_static_f64[156]+(self.scalar_static_f64[157]/common.v153)))/common.v169));
        let v404=(self.scalar_static_f64[173]*(common.v401-common.v402));
        let v432=(common.v428-self.scalar_static_f64[8]);
        let v434=(((self.scalar_static_f64[150]+(self.scalar_static_f64[151]/common.v153))+((self.scalar_static_f64[32]*(self.scalar_static_f64[152]+(self.scalar_static_f64[153]/common.v153)))/common.v169))+(v383*v432));
        let v436=(common.v0+(v432*v434));
        let v437=0.11;
        let v439=(if (v436<v437){common.v0}else{common.v1});
        let v440=10.0;
        let v444=(((v440*(v436-v3))-common.v0)).exp();
        let v447=(if (v439!=0.0){(v3+(common.v225*v444))}else{v436});
        let v448=(v316*v317);
        let v449=(v447*v448);
        let v453=(v317*v447);
        let v455=(if self.scalar_static_bool[38]{(common.v0/v453)}else{(if (self.scalar_static_f64[105]!=0.0){(common.v0/v449)}else{common.v1})});
        let v459=(self.scalar_static_f64[174]+(v432*self.scalar_static_f64[175]));
        let v461=(common.v0+(v432*v459));
        let v463=(if (v461<v437){common.v0}else{common.v1});
        let v467=(((v440*(v461-v3))-common.v0)).exp();
        let v470=(if (v463!=0.0){(v3+(common.v225*v467))}else{v461});
        let v472=f64::powf(common.v431,self.scalar_static_f64[176]);
        let v479=(self.scalar_static_f64[180]*(common.v0-common.v431));
        let v484=((v479/common.v430)+(self.scalar_static_f64[181]*common.v482));
        let v487=((v484/self.scalar_static_f64[182])).exp();
        let v489=(if (self.scalar_static_f64[178]!=0.0){(self.scalar_static_f64[177]*v487)}else{common.v1});
        let v490=(common.v430*self.scalar_static_f64[182]);
        let v493=(common.v0+(self.scalar_static_f64[183]/v489));
        let v494=(v493).ln();
        let v499=(if self.scalar_static_bool[40]{common.v1}else{(if (self.scalar_static_f64[178]!=0.0){(v490*v494)}else{common.v1})});
        let v505=((v484/self.scalar_static_f64[186])).exp();
        let v507=(if (self.scalar_static_f64[185]!=0.0){(self.scalar_static_f64[184]*v505)}else{common.v1});
        let v508=(common.v430*self.scalar_static_f64[186]);
        let v510=(common.v0+(self.scalar_static_f64[183]/v507));
        let v511=(v510).ln();
        let v515=(if self.scalar_static_bool[42]{common.v1}else{v507});
        let v516=(if self.scalar_static_bool[42]{common.v1}else{(if (self.scalar_static_f64[185]!=0.0){(v508*v511)}else{common.v1})});
        let v517=(self.scalar_static_f64[20]*(if self.scalar_static_bool[40]{common.v1}else{v489}));
        let v518=(self.scalar_static_f64[22]*v515);
        let v520=(self.scalar_static_f64[24]*v515);
        let v615=(self.scalar_static_f64[199]+(v432*self.scalar_static_f64[200]));
        let v619=(if (self.scalar_static_f64[198]!=0.0){(self.scalar_static_f64[197]*(common.v0+(v432*v615)))}else{common.v1});
        let v620=(v619>common.v1);
        let v622=(if (self.scalar_static_f64[198]!=0.0){(if v620{v619}else{common.v1})}else{v619});
        let v628=(if (self.scalar_static_f64[198]!=0.0){(self.scalar_static_f64[201]*(common.v0+(v432*self.scalar_static_f64[202])))}else{common.v1});
        let v629=(common.v430*v628);
        let v630=(-v622);
        let v632=((v630/v629)).exp();
        let v635=(v632+self.scalar_static_f64[204]);
        let v636=(v635).ln();
        let v640=(if self.scalar_static_bool[48]{self.scalar_static_f64[197]}else{v622});
        let v641=(if self.scalar_static_bool[48]{self.scalar_static_f64[201]}else{v628});
        let v642=(if self.scalar_static_bool[48]{common.v0}else{(if (self.scalar_static_f64[198]!=0.0){(v629*v636)}else{common.v1})});
        let v650=(v472*self.scalar_static_f64[208]);
        let v653=(v472*self.scalar_static_f64[205]);
        let v658=(if self.scalar_static_bool[53]{self.scalar_static_f64[208]}else{(if self.scalar_static_bool[51]{(v447*v650)}else{common.v1})});
        let v659=(if self.scalar_static_bool[53]{self.scalar_static_f64[205]}else{(if self.scalar_static_bool[51]{(v447*v653)}else{common.v1})});
        let v664=(v659*self.scalar_static_f64[211]);
        let v667=(((v658*v658)+(v659*v664))).sqrt();
        let v671=(if (self.scalar_static_f64[206]!=0.0){(v667-(v659*self.scalar_static_f64[212]))}else{common.v1});
        let v672=(self.scalar_static_f64[209]*v671);
        let v674=(if (self.scalar_static_f64[206]!=0.0){(v672/v659)}else{common.v1});
        let v675=(v671*v671);
        let v676=(v659*v659);
        let v680=(((v675/v676)+(common.v305*v674))).sqrt();
        let v687=(if self.scalar_static_bool[54]{common.v1}else{v671});
        let v688=(if self.scalar_static_bool[54]{common.v1}else{v674});
        let v689=(if self.scalar_static_bool[54]{common.v1}else{(if (self.scalar_static_f64[206]!=0.0){v680}else{common.v1})});
        let v692=(if self.scalar_static_bool[54]{common.v1}else{(if (self.scalar_static_f64[206]!=0.0){(common.v0/v659)}else{common.v1})});
        let v693=(v191*(if self.scalar_static_bool[54]{1000.0}else{(if (self.scalar_static_f64[206]!=0.0){(v659-v658)}else{common.v1})}));
        let v694=100000.0;
        let v696=(if (v693>v694){common.v0}else{common.v1});
        let v697=(if (v696!=0.0){v694}else{v693});
        let v699=(if (v404<common.v1){common.v0}else{common.v1});
        let v700=-1.0;
        let v706=(!(v699!=0.0));
        let v707=(if v706{common.v0}else{(if (v699!=0.0){v700}else{common.v1})});
        let v709=(if v706{(-common.v407)}else{(if (v699!=0.0){(-common.v409)}else{common.v1})});
        let v710=(if v706{v404}else{(if (v699!=0.0){(-v404)}else{common.v1})});
        let v712=(if (v709>v280){common.v0}else{common.v1});
        let v715=(((v280-v709)/self.scalar_static_f64[283])).exp();
        let v716=(common.v0+v715);
        let v721=(!(v712!=0.0));
        let v724=(((v709-v280)/self.scalar_static_f64[283])).exp();
        let v725=(common.v0+v724);
        let v729=(if v721{(v709-(self.scalar_static_f64[283]*(v725).ln()))}else{(if (v712!=0.0){(v280-(self.scalar_static_f64[283]*(v716).ln()))}else{common.v1})});
        let v730=-0.4;
        let v731=(v280-v729);
        let v732=(v710<v731);
        let v735=(v730*(common.v227+(if v732{v710}else{v731})));
        let v737=(if (v729<v735){common.v0}else{common.v1});
        let v738=((self.scalar_static_f64[105]!=0.0)&&(v737!=0.0));
        let v741=((self.scalar_static_f64[105]!=0.0)&&(!(v737!=0.0)));
        let v743=(common.v227*v730);
        let v745=(if (v729<v743){common.v0}else{common.v1});
        let v746=(self.scalar_static_bool[38]&&(v745!=0.0));
        let v749=(self.scalar_static_bool[38]&&(!(v745!=0.0)));
        let v750=(if v749{v729}else{(if v746{v743}else{(if v741{v729}else{(if v738{v735}else{common.v1})})})});
        let v752=(common.v227+(common.v58*v750));
        let v754=(if (v692>common.v1){common.v0}else{common.v1});
        let v755=(common.v254*v752);
        let v758=(if (v754!=0.0){((v752*v755)-v752)}else{common.v1});
        let v759=(common.v254*common.v541);
        let v760=(v752*v759);
        let v762=(if (v754!=0.0){(v700+v760)}else{common.v1});
        let v763=9.0;
        let v768=(if (v754!=0.0){(common.v254*(2.25+(v752/v697)))}else{common.v1});
        let v769=1.5;
        let v770=(common.v254*v769);
        let v772=(if (v754!=0.0){(v770/v697)}else{common.v1});
        let v773=(common.v305*v697);
        let v776=(if (v754!=0.0){((v697*v773)/common.v254)}else{common.v1});
        let v778=(if (v754!=0.0){(v758*v776)}else{common.v1});
        let v780=(if (v754!=0.0){(v762*v776)}else{common.v1});
        let v782=(if (v754!=0.0){(v768*v776)}else{common.v1});
        let v784=(if (v754!=0.0){(v772*v776)}else{common.v1});
        let v786=(if (v754!=0.0){(v784*v784)}else{common.v1});
        let v788=(if (v754!=0.0){(-v782)}else{common.v1});
        let v792=(if (v754!=0.0){((v780*v784)-(common.v305*v778))}else{common.v1});
        let v793=(common.v305*v782);
        let v799=(if (v754!=0.0){(((v778*v793)-(v780*v780))-(v778*v786))}else{common.v1});
        let v801=0.3333333333333333;
        let v804=(if (v754!=0.0){(v792-((v788*v788)*v801))}else{common.v1});
        let v806=(v792+(common.v58*v804));
        let v810=(if (v754!=0.0){(v799-((v788*v806)/v763))}else{common.v1});
        let v811=(v804*v804);
        let v813=27.0;
        let v815=(if (v754!=0.0){((v804*v811)/v813)}else{common.v1});
        let v816=0.25;
        let v817=(v810*v816);
        let v821=((if (v754!=0.0){(v815+(v810*v817))}else{common.v1})).sqrt();
        let v822=(if (v754!=0.0){v821}else{common.v1});
        let v824=(if (v810<common.v1){common.v0}else{common.v1});
        let v825=((v754!=0.0)&&(v824!=0.0));
        let v826=(common.v531*v810);
        let v828=(if v825{(v822+v826)}else{common.v1});
        let v829=(-v815);
        let v833=((v754!=0.0)&&(!(v824!=0.0)));
        let v835=(if v833{(v826-v822)}else{(if v825{(v829/v828)}else{common.v1})});
        let v837=(if v833{(v829/v835)}else{v828});
        let v838=1e-6;
        let v840=(if (v837>v838){common.v0}else{common.v1});
        let v841=((v754!=0.0)&&(v840!=0.0));
        let v844=-1e-6;
        let v846=(if (v837<v844){common.v0}else{common.v1});
        let v848=((v754!=0.0)&&(!(v840!=0.0)));
        let v849=((v846!=0.0)&&v848);
        let v850=(-v837);
        let v855=(v848&&(!(v846!=0.0)));
        let v859=(if (v835>v838){common.v0}else{common.v1});
        let v860=((v754!=0.0)&&(v859!=0.0));
        let v864=(if (v835<v844){common.v0}else{common.v1});
        let v866=((v754!=0.0)&&(!(v859!=0.0)));
        let v867=((v864!=0.0)&&v866);
        let v868=(-v835);
        let v873=(v866&&(!(v864!=0.0)));
        let v880=(v786*v816);
        let v883=(((if (v754!=0.0){(((if v855{(common.v229*v837)}else{(if v849{(-f64::powf(v850,v801))}else{(if v841{f64::powf(v837,v801)}else{common.v1})})})+(if v873{(common.v229*v835)}else{(if v867{(-f64::powf(v868,v801))}else{(if v860{f64::powf(v835,v801)}else{common.v1})})}))-(v788*v801))}else{common.v1})+(v880-v782))).sqrt();
        let v884=(if (v754!=0.0){v883}else{v799});
        let v885=0.75;
        let v887=(v884*v884);
        let v891=(if (v754!=0.0){(((v786*v885)-v887)-(common.v58*v782))}else{common.v1});
        let v896=(((v782*v784)-(common.v58*v780))-(v784*v880));
        let v898=(if (v754!=0.0){(v896/v884)}else{common.v1});
        let v900=(if (v754!=0.0){(v891+v898)}else{common.v1});
        let v902=(if (v900>common.v1){common.v0}else{common.v1});
        let v903=((v754!=0.0)&&(v902!=0.0));
        let v904=(v900).sqrt();
        let v905=(if v903{v904}else{common.v1});
        let v906=-0.25;
        let v907=(v784*v906);
        let v913=((v754!=0.0)&&(!(v902!=0.0)));
        let v915=(if v913{(v891-v898)}else{common.v1});
        let v919=(((v915*v915)+0.0001)).sqrt();
        let v920=(v919).sqrt();
        let v927=(if (v750>(if self.scalar_static_bool[30]{common.v1}else{(if self.scalar_static_bool[28]{common.v1}else{(if (self.scalar_static_f64[106]!=0.0){((0.1666666666666667/common.v254)-common.v256)}else{common.v1})})})){common.v0}else{common.v1});
        let v928=(!(v754!=0.0));
        let v929=((v927!=0.0)&&v928);
        let v930=(common.v257-v750);
        let v932=(if v929{(common.v254*v930)}else{common.v1});
        let v935=(common.v58*(common.v0-(common.v58*v932)));
        let v936=(v930*v935);
        let v941=((common.v0-(v769*v932))).sqrt();
        let v942=((common.v0-(common.v541*v932))+v941);
        let v946=(v928&&(!(v927!=0.0)));
        let v947=(if v946{v760}else{v932});
        let v950=((common.v0+v947)).sqrt();
        let v953=(common.v254*4.5);
        let v955=(if v946{(((common.v0-v947)+v950)/v953)}else{(if v929{(v936/v942)}else{(if v913{(v907+(common.v67*((if v913{v920}else{v905})-v884)))}else{(if v903{(v907+(common.v67*(v884+v905)))}else{common.v1})})})});
        let v959=(if (self.scalar_static_bool[25]&&(common.v249>1e-9)){common.v0}else{common.v1});
        let v961=(if (v959!=0.0){(self.scalar_static_f64[270]+v955)}else{common.v1});
        let v962=(v752+v955);
        let v963=(v962).sqrt();
        let v965=(if (v959!=0.0){(common.v249*v963)}else{common.v1});
        let v966=((v754!=0.0)&&(v959!=0.0));
        let v967=(v961/v191);
        let v969=(common.v67*(v967-v687));
        let v971=(if v966{(v692*v969)}else{common.v1});
        let v973=(common.v67*(v687+v967));
        let v975=(if v966{(v692*v973)}else{common.v1});
        let v978=((v688+(v971*v971))).sqrt();
        let v979=(if v966{v978}else{common.v1});
        let v982=((v688+(v975*v975))).sqrt();
        let v983=(if v966{v982}else{common.v1});
        let v986=(if v966{((v979+v983)-v689)}else{common.v1});
        let v990=(common.v67*((v971/v979)+(v975/v983)));
        let v993=(if v966{((v692*v990)/v191)}else{common.v1});
        let v994=(common.v58*v965);
        let v995=(common.v0-v965);
        let v996=(v994*v995);
        let v997=(v961*v993);
        let v998=(common.v0+v986);
        let v1000=(common.v0-(v997/v998));
        let v1001=(v996*v1000);
        let v1003=((v1001/v961)).sqrt();
        let v1005=(v928&&(v959!=0.0));
        let v1007=((v996/v961)).sqrt();
        let v1008=(if v1005{v1007}else{(if v966{v1003}else{common.v1})});
        let v1009=(common.v254*v962);
        let v1010=(v1008*v1008);
        let v1014=(self.scalar_static_f64[110]*v955);
        let v1015=(self.scalar_static_f64[110]+v961);
        let v1018=(if (v959!=0.0){(v310+(v1014/v1015))}else{common.v1});
        let v1019=(common.v305*v1018);
        let v1021=(if (v959!=0.0){(v1018*v1019)}else{common.v1});
        let v1022=(common.v58*v710);
        let v1023=(v961*v1022);
        let v1024=(v710-v961);
        let v1025=(v1024*v1024);
        let v1027=((v1021+v1025)).sqrt();
        let v1028=(v710+v961);
        let v1029=(v1028*v1028);
        let v1031=((v1021+v1029)).sqrt();
        let v1032=(v1027+v1031);
        let v1034=(if (v959!=0.0){(v1023/v1032)}else{common.v1});
        let v1036=((v959!=0.0)&&(self.scalar_static_f64[213]!=0.0));
        let v1037=(self.scalar_static_f64[110]*v1034);
        let v1040=(if v1036{(v310+(v1037/v1015))}else{v1018});
        let v1041=(common.v305*v1040);
        let v1043=(if v1036{(v1040*v1041)}else{v1021});
        let v1045=((v1025+v1043)).sqrt();
        let v1047=((v1029+v1043)).sqrt();
        let v1048=(v1045+v1047);
        let v1050=(if v1036{(v1023/v1048)}else{v1034});
        let v1052=(((if (v959!=0.0){((v1009/v1010)-v961)}else{common.v1})+v1050)).sqrt();
        let v1056=(v1050/v191);
        let v1058=(common.v67*(v1056-v687));
        let v1060=(if v966{(v692*v1058)}else{v971});
        let v1062=(common.v67*(v687+v1056));
        let v1064=(if v966{(v692*v1062)}else{v975});
        let v1067=((v688+(v1060*v1060))).sqrt();
        let v1068=(if v966{v1067}else{v979});
        let v1071=((v688+(v1064*v1064))).sqrt();
        let v1072=(if v966{v1071}else{v983});
        let v1077=(!(v959!=0.0));
        let v1078=(v955*v1022);
        let v1079=(v710-v955);
        let v1082=((v310+(v1079*v1079))).sqrt();
        let v1083=(v710+v955);
        let v1086=((v310+(v1083*v1083))).sqrt();
        let v1087=(v1082+v1086);
        let v1089=(if v1077{(v1078/v1087)}else{v1050});
        let v1090=((v754!=0.0)&&v1077);
        let v1091=(v1089/v191);
        let v1093=(common.v67*(v1091-v687));
        let v1095=(if v1090{(v692*v1093)}else{v1060});
        let v1097=(common.v67*(v687+v1091));
        let v1099=(if v1090{(v692*v1097)}else{v1064});
        let v1102=((v688+(v1095*v1095))).sqrt();
        let v1106=((v688+(v1099*v1099))).sqrt();
        let v1111=(v928&&v1077);
        let v1114=((v752+v1089)).sqrt();
        let v1117=(if v1077{(common.v0-(common.v249*v1114))}else{(if (v959!=0.0){(common.v0-(v1008*v1052))}else{common.v1})});
        let v1119=(if (v1117<self.scalar_static_f64[107]){common.v0}else{common.v1});
        let v1120=(if (v1119!=0.0){self.scalar_static_f64[107]}else{v1117});
        let v1121=(v455*v1120);
        let v1122=(common.v0+(if v1111{common.v1}else{(if v1090{(((if v1090{v1102}else{v1068})+(if v1090{v1106}else{v1072}))-v689)}else{(if v1005{common.v1}else{(if v966{((v1068+v1072)-v689)}else{v986})})})}));
        let v1124=(v707*(v1121/v1122));
        let v1125=(v1089*v1124);
        let v1127=(if ((v517+v518)>common.v1){common.v0}else{common.v1});
        let v1128=(if (v1127!=0.0){v517}else{common.v1});
        let v1129=(if (v1127!=0.0){v518}else{common.v1});
        let v1131=(if (v1128>common.v1){common.v0}else{common.v1});
        let v1132=((v1127!=0.0)&&(v1131!=0.0));
        let v1133=(common.v0/v490);
        let v1134=(if v1132{v1133}else{common.v1});
        let v1136=(if (common.v407<v499){common.v0}else{common.v1});
        let v1137=(v1132&&(v1136!=0.0));
        let v1139=((common.v407*v1134)).exp();
        let v1142=(v1132&&(!(v1136!=0.0)));
        let v1144=((v499*v1134)).exp();
        let v1145=(common.v407-v499);
        let v1147=(common.v0+(v1134*v1145));
        let v1149=(if v1142{(v1144*v1147)}else{(if v1137{v1139}else{common.v1})});
        let v1150=(v1149-common.v0);
        let v1154=((v1127!=0.0)&&(!(v1131!=0.0)));
        let v1157=(if (v1129>common.v1){common.v0}else{common.v1});
        let v1158=((v1127!=0.0)&&(v1157!=0.0));
        let v1159=(common.v0/v508);
        let v1160=(if v1158{v1159}else{v1134});
        let v1162=(if (common.v407<v516){common.v0}else{common.v1});
        let v1163=(v1158&&(v1162!=0.0));
        let v1165=((common.v407*v1160)).exp();
        let v1168=(v1158&&(!(v1162!=0.0)));
        let v1170=((v516*v1160)).exp();
        let v1171=(common.v407-v516);
        let v1173=(common.v0+(v1160*v1171));
        let v1176=((if v1168{(v1170*v1173)}else{(if v1163{v1165}else{v1149})})-common.v0);
        let v1180=((v1127!=0.0)&&(!(v1157!=0.0)));
        let v1185=(if (v640>common.v1){common.v0}else{common.v1});
        let v1186=((v1127!=0.0)&&(v1185!=0.0));
        let v1187=(-v640);
        let v1189=(if v1186{(v1187-common.v407)}else{common.v1});
        let v1190=(common.v430*v641);
        let v1191=(common.v0/v1190);
        let v1192=(if v1186{v1191}else{common.v1});
        let v1194=(if (v1189<v642){common.v0}else{common.v1});
        let v1195=(v1186&&(v1194!=0.0));
        let v1197=((v1189*v1192)).exp();
        let v1200=(v1186&&(!(v1194!=0.0)));
        let v1202=((v642*v1192)).exp();
        let v1203=(v1189-v642);
        let v1205=(common.v0+(v1192*v1203));
        let v1210=((v1187*v1192)).exp();
        let v1214=(!(v1185!=0.0));
        let v1215=((v1127!=0.0)&&v1214);
        let v1221=(!(v1127!=0.0));
        let v1222=(if v1221{common.v1}else{(if (v1127!=0.0){(((if (v1127!=0.0){((if v1154{common.v1}else{(if v1132{(v1128*v1150)}else{common.v1})})+(if v1180{common.v1}else{(if v1158{(v1129*v1176)}else{common.v1})}))}else{common.v1})+(if v1215{common.v1}else{(if v1186{(self.scalar_static_f64[214]*((if v1200{(v1202*v1205)}else{(if v1195{v1197}else{common.v1})})-v1210))}else{common.v1})}))+(common.v1*common.v407))}else{common.v1})});
        let v1224=(if ((v517+v520)>common.v1){common.v0}else{common.v1});
        let v1225=(if (v1224!=0.0){v517}else{common.v1});
        let v1226=(if (v1224!=0.0){v520}else{common.v1});
        let v1228=(if (v1225>common.v1){common.v0}else{common.v1});
        let v1229=((v1224!=0.0)&&(v1228!=0.0));
        let v1230=(if v1229{v1133}else{common.v1});
        let v1232=(if (common.v409<v499){common.v0}else{common.v1});
        let v1233=(v1229&&(v1232!=0.0));
        let v1235=((common.v409*v1230)).exp();
        let v1238=(v1229&&(!(v1232!=0.0)));
        let v1240=((v499*v1230)).exp();
        let v1241=(common.v409-v499);
        let v1243=(common.v0+(v1230*v1241));
        let v1245=(if v1238{(v1240*v1243)}else{(if v1233{v1235}else{common.v1})});
        let v1246=(v1245-common.v0);
        let v1250=((v1224!=0.0)&&(!(v1228!=0.0)));
        let v1253=(if (v1226>common.v1){common.v0}else{common.v1});
        let v1254=((v1224!=0.0)&&(v1253!=0.0));
        let v1255=(if v1254{v1159}else{v1230});
        let v1257=(if (common.v409<v516){common.v0}else{common.v1});
        let v1258=(v1254&&(v1257!=0.0));
        let v1260=((common.v409*v1255)).exp();
        let v1263=(v1254&&(!(v1257!=0.0)));
        let v1265=((v516*v1255)).exp();
        let v1266=(common.v409-v516);
        let v1268=(common.v0+(v1255*v1266));
        let v1271=((if v1263{(v1265*v1268)}else{(if v1258{v1260}else{v1245})})-common.v0);
        let v1275=((v1224!=0.0)&&(!(v1253!=0.0)));
        let v1279=((v1185!=0.0)&&(v1224!=0.0));
        let v1281=(if v1279{(v1187-common.v409)}else{common.v1});
        let v1282=(if v1279{v1191}else{common.v1});
        let v1284=(if (v1281<v642){common.v0}else{common.v1});
        let v1285=(v1279&&(v1284!=0.0));
        let v1287=((v1281*v1282)).exp();
        let v1290=(v1279&&(!(v1284!=0.0)));
        let v1292=((v642*v1282)).exp();
        let v1293=(v1281-v642);
        let v1295=(common.v0+(v1282*v1293));
        let v1299=((v1187*v1282)).exp();
        let v1303=(v1214&&(v1224!=0.0));
        let v1309=(!(v1224!=0.0));
        let v1310=(if v1309{common.v1}else{(if (v1224!=0.0){(((if (v1224!=0.0){((if v1250{common.v1}else{(if v1229{(v1225*v1246)}else{common.v1})})+(if v1275{common.v1}else{(if v1254{(v1226*v1271)}else{common.v1})}))}else{common.v1})+(if v1303{common.v1}else{(if v1279{(self.scalar_static_f64[214]*((if v1290{(v1292*v1295)}else{(if v1285{v1287}else{common.v1})})-v1299))}else{common.v1})}))+(common.v1*common.v409))}else{common.v1})});
        let v1316=ctx.branch_current(branches[0]);
        let v1318=(ctx.node_voltage(nodes[0])-common.v402);
        let v1321=ctx.branch_current(branches[1]);
        let v1323=(ctx.node_voltage(nodes[2])-common.v401);
        let v1336=(self.scalar_static_f64[286]*common.v398);
        let v1368=(common.v0+(common.v398/self.scalar_static_f64[302]));
        let v1379=(common.v0+((common.v398*self.scalar_static_f64[220])/self.scalar_static_f64[302]));
        let v1386=(self.scalar_static_f64[173]*v1125);
        let v1387=(self.scalar_static_f64[173]*v1222);
        let v1388=(self.scalar_static_f64[173]*v1310);
        let v1749=(if ((self.scalar_static_f64[122]/common.v2)<=self.scalar_static_f64[241]){common.v0}else{common.v1});
        let v1752=(if ((self.scalar_static_f64[126]/common.v2)<=self.scalar_static_f64[241]){common.v0}else{common.v1});
        let v1753=(self.scalar_static_f64[122]*v470);
        let v1755=(self.scalar_static_f64[126]*v470);
        let v1757=(self.scalar_static_f64[122]*v1316);
        let v1760=(!(v1749!=0.0));
        let v1763=(self.scalar_static_f64[126]*v1321);
        let v1766=(!(v1752!=0.0));
        let v1780=((v434*common.v1773)+(v432*(v383*common.v1773)));
        let v1784=(if (v439!=0.0){(common.v225*(v444*(v440*v1780)))}else{v1780});
        let v1798=((v459*common.v1773)+(v432*(self.scalar_static_f64[175]*common.v1773)));
        let v1802=(if (v463!=0.0){(common.v225*(v467*(v440*v1798)))}else{v1798});
        let v1806=(common.v1776*(self.scalar_static_f64[176]*f64::powf(common.v431,self.scalar_static_f64[242])));
        let v1816=((((common.v430*(self.scalar_static_f64[180]*(-common.v1776)))-(v479*common.v1775))/common.v1812)+(self.scalar_static_f64[181]*common.v1814));
        let v1820=(if (self.scalar_static_f64[178]!=0.0){(self.scalar_static_f64[177]*(v487*(v1816/self.scalar_static_f64[182])))}else{common.v1});
        let v1821=(self.scalar_static_f64[182]*common.v1775);
        let v1832=(if self.scalar_static_bool[40]{common.v1}else{(if (self.scalar_static_f64[178]!=0.0){((v494*v1821)+(v490*(((-(self.scalar_static_f64[183]*v1820))/(v489*v489))/v493)))}else{common.v1})});
        let v1836=(if (self.scalar_static_f64[185]!=0.0){(self.scalar_static_f64[184]*(v505*(v1816/self.scalar_static_f64[186])))}else{common.v1});
        let v1837=(self.scalar_static_f64[186]*common.v1775);
        let v1847=(if self.scalar_static_bool[42]{common.v1}else{v1836});
        let v1848=(if self.scalar_static_bool[42]{common.v1}else{(if (self.scalar_static_f64[185]!=0.0){((v511*v1837)+(v508*(((-(self.scalar_static_f64[183]*v1836))/(v507*v507))/v510)))}else{common.v1})});
        let v1849=(self.scalar_static_f64[20]*(if self.scalar_static_bool[40]{common.v1}else{v1820}));
        let v1973=(if (self.scalar_static_f64[198]!=0.0){(self.scalar_static_f64[197]*((v615*common.v1773)+(v432*(self.scalar_static_f64[200]*common.v1773))))}else{common.v1});
        let v1975=(if (self.scalar_static_f64[198]!=0.0){(if v620{v1973}else{common.v1})}else{v1973});
        let v1978=(if (self.scalar_static_f64[198]!=0.0){(self.scalar_static_f64[201]*(self.scalar_static_f64[202]*common.v1773))}else{common.v1});
        let v1981=((v628*common.v1775)+(common.v430*v1978));
        let v1996=(if self.scalar_static_bool[48]{common.v1}else{(if (self.scalar_static_f64[198]!=0.0){((v636*v1981)+(v629*((v632*(((v629*(-v1975))-(v630*v1981))/(v629*v629)))/v635)))}else{common.v1})});
        let v2007=(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[51]{((v650*v1784)+(v447*(self.scalar_static_f64[208]*v1806)))}else{common.v1})});
        let v2008=(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[51]{((v653*v1784)+(v447*(self.scalar_static_f64[205]*v1806)))}else{common.v1})});
        let v2009=(v658*v2007);
        let v2020=(if (self.scalar_static_f64[206]!=0.0){((((v2009+v2009)+((v664*v2008)+(v659*(self.scalar_static_f64[211]*v2008))))/(common.v58*v667))-(self.scalar_static_f64[212]*v2008))}else{common.v1});
        let v2026=(if (self.scalar_static_f64[206]!=0.0){(((v659*(self.scalar_static_f64[209]*v2020))-(v672*v2008))/v676)}else{common.v1});
        let v2027=(v671*v2020);
        let v2029=(v659*v2008);
        let v2046=(if self.scalar_static_bool[54]{common.v1}else{v2020});
        let v2047=(if self.scalar_static_bool[54]{common.v1}else{v2026});
        let v2048=(if self.scalar_static_bool[54]{common.v1}else{(if (self.scalar_static_f64[206]!=0.0){(((((v676*(v2027+v2027))-(v675*(v2029+v2029)))/(v676*v676))+(common.v305*v2026))/(common.v58*v680))}else{common.v1})});
        let v2050=(if self.scalar_static_bool[54]{common.v1}else{(if (self.scalar_static_f64[206]!=0.0){((-v2008)/v676)}else{common.v1})});
        let v2052=(if (v696!=0.0){common.v1}else{(v191*(if self.scalar_static_bool[54]{common.v1}else{(if (self.scalar_static_f64[206]!=0.0){(v2008-v2007)}else{common.v1})}))});
        let v2053=(if (v699!=0.0){self.scalar_static_f64[172]}else{common.v1});
        let v2054=(if (v699!=0.0){self.scalar_static_f64[173]}else{common.v1});
        let v2055=(if v706{self.scalar_static_f64[172]}else{v2053});
        let v2056=(if v706{self.scalar_static_f64[173]}else{common.v1});
        let v2057=(if v706{common.v1}else{v2054});
        let v2058=(if v706{self.scalar_static_f64[172]}else{v2054});
        let v2059=(if v706{self.scalar_static_f64[173]}else{v2053});
        let v2096=(if v721{(v2055-(self.scalar_static_f64[283]*((v724*(v2055/self.scalar_static_f64[283]))/v725)))}else{(if (v712!=0.0){(-(self.scalar_static_f64[283]*((v715*((-v2055)/self.scalar_static_f64[283]))/v716)))}else{common.v1})});
        let v2097=(if v721{(v2056-(self.scalar_static_f64[283]*((v724*(v2056/self.scalar_static_f64[283]))/v725)))}else{(if (v712!=0.0){(-(self.scalar_static_f64[283]*((v715*((-v2056)/self.scalar_static_f64[283]))/v716)))}else{common.v1})});
        let v2098=(if v721{(v2057-(self.scalar_static_f64[283]*((v724*(v2057/self.scalar_static_f64[283]))/v725)))}else{(if (v712!=0.0){(-(self.scalar_static_f64[283]*((v715*((-v2057)/self.scalar_static_f64[283]))/v716)))}else{common.v1})});
        let v2117=(if v749{v2096}else{(if v746{common.v1}else{(if v741{v2096}else{(if v738{(v730*(if v732{common.v1}else{(-v2096)}))}else{common.v1})})})});
        let v2118=(if v749{v2097}else{(if v746{common.v1}else{(if v741{v2097}else{(if v738{(v730*(if v732{v2058}else{(-v2097)}))}else{common.v1})})})});
        let v2119=(if v749{v2098}else{(if v746{common.v1}else{(if v741{v2098}else{(if v738{(v730*(if v732{v2059}else{(-v2098)}))}else{common.v1})})})});
        let v2120=(common.v58*v2117);
        let v2121=(common.v58*v2118);
        let v2122=(common.v58*v2119);
        let v2141=(v759*v2120);
        let v2142=(v759*v2121);
        let v2143=(v759*v2122);
        let v2150=(v697*v697);
        let v2171=(if (v754!=0.0){(((v773*v2052)+(v697*(common.v305*v2052)))/common.v254)}else{common.v1});
        let v2176=(if (v754!=0.0){(v776*(if (v754!=0.0){(((v755*v2120)+(v752*(common.v254*v2120)))-v2120)}else{common.v1}))}else{common.v1});
        let v2177=(if (v754!=0.0){(v758*v2171)}else{common.v1});
        let v2178=(if (v754!=0.0){(v776*(if (v754!=0.0){(((v755*v2121)+(v752*(common.v254*v2121)))-v2121)}else{common.v1}))}else{common.v1});
        let v2179=(if (v754!=0.0){(v776*(if (v754!=0.0){(((v755*v2122)+(v752*(common.v254*v2122)))-v2122)}else{common.v1}))}else{common.v1});
        let v2184=(if (v754!=0.0){(v776*(if (v754!=0.0){v2141}else{common.v1}))}else{common.v1});
        let v2185=(if (v754!=0.0){(v762*v2171)}else{common.v1});
        let v2186=(if (v754!=0.0){(v776*(if (v754!=0.0){v2142}else{common.v1}))}else{common.v1});
        let v2187=(if (v754!=0.0){(v776*(if (v754!=0.0){v2143}else{common.v1}))}else{common.v1});
        let v2194=(if (v754!=0.0){(v776*(if (v754!=0.0){(common.v254*(v2120/v697))}else{common.v1}))}else{common.v1});
        let v2195=(if (v754!=0.0){((v776*(if (v754!=0.0){(common.v254*((-(v752*v2052))/v2150))}else{common.v1}))+(v768*v2171))}else{common.v1});
        let v2196=(if (v754!=0.0){(v776*(if (v754!=0.0){(common.v254*(v2121/v697))}else{common.v1}))}else{common.v1});
        let v2197=(if (v754!=0.0){(v776*(if (v754!=0.0){(common.v254*(v2122/v697))}else{common.v1}))}else{common.v1});
        let v2201=(if (v754!=0.0){((v776*(if (v754!=0.0){((-(v770*v2052))/v2150)}else{common.v1}))+(v772*v2171))}else{common.v1});
        let v2202=(v784*v2201);
        let v2204=(if (v754!=0.0){(v2202+v2202)}else{common.v1});
        let v2205=(-v2194);
        let v2207=(-v2196);
        let v2208=(-v2197);
        let v2209=(if (v754!=0.0){v2205}else{common.v1});
        let v2210=(if (v754!=0.0){(-v2195)}else{common.v1});
        let v2211=(if (v754!=0.0){v2207}else{common.v1});
        let v2212=(if (v754!=0.0){v2208}else{common.v1});
        let v2227=(if (v754!=0.0){((v784*v2184)-(common.v305*v2176))}else{common.v1});
        let v2228=(if (v754!=0.0){(((v784*v2185)+(v780*v2201))-(common.v305*v2177))}else{common.v1});
        let v2229=(if (v754!=0.0){((v784*v2186)-(common.v305*v2178))}else{common.v1});
        let v2230=(if (v754!=0.0){((v784*v2187)-(common.v305*v2179))}else{common.v1});
        let v2247=(v780*v2184);
        let v2249=(v780*v2185);
        let v2251=(v780*v2186);
        let v2253=(v780*v2187);
        let v2269=(if (v754!=0.0){((((v793*v2176)+(v778*(common.v305*v2194)))-(v2247+v2247))-(v786*v2176))}else{common.v1});
        let v2270=(if (v754!=0.0){((((v793*v2177)+(v778*(common.v305*v2195)))-(v2249+v2249))-((v786*v2177)+(v778*v2204)))}else{common.v1});
        let v2271=(if (v754!=0.0){((((v793*v2178)+(v778*(common.v305*v2196)))-(v2251+v2251))-(v786*v2178))}else{common.v1});
        let v2272=(if (v754!=0.0){((((v793*v2179)+(v778*(common.v305*v2197)))-(v2253+v2253))-(v786*v2179))}else{common.v1});
        let v2273=(v788*v2209);
        let v2275=(v788*v2210);
        let v2277=(v788*v2211);
        let v2279=(v788*v2212);
        let v2289=(if (v754!=0.0){(v2227-(v801*(v2273+v2273)))}else{common.v1});
        let v2290=(if (v754!=0.0){(v2228-(v801*(v2275+v2275)))}else{common.v1});
        let v2291=(if (v754!=0.0){(v2229-(v801*(v2277+v2277)))}else{common.v1});
        let v2292=(if (v754!=0.0){(v2230-(v801*(v2279+v2279)))}else{common.v1});
        let v2321=(if (v754!=0.0){(v2269-(((v806*v2209)+(v788*(v2227+(common.v58*v2289))))/v763))}else{common.v1});
        let v2322=(if (v754!=0.0){(v2270-(((v806*v2210)+(v788*(v2228+(common.v58*v2290))))/v763))}else{common.v1});
        let v2323=(if (v754!=0.0){(v2271-(((v806*v2211)+(v788*(v2229+(common.v58*v2291))))/v763))}else{common.v1});
        let v2324=(if (v754!=0.0){(v2272-(((v806*v2212)+(v788*(v2230+(common.v58*v2292))))/v763))}else{common.v1});
        let v2325=(v804*v2289);
        let v2327=(v804*v2290);
        let v2329=(v804*v2291);
        let v2331=(v804*v2292);
        let v2349=(if (v754!=0.0){(((v811*v2289)+(v804*(v2325+v2325)))/v813)}else{common.v1});
        let v2350=(if (v754!=0.0){(((v811*v2290)+(v804*(v2327+v2327)))/v813)}else{common.v1});
        let v2351=(if (v754!=0.0){(((v811*v2291)+(v804*(v2329+v2329)))/v813)}else{common.v1});
        let v2352=(if (v754!=0.0){(((v811*v2292)+(v804*(v2331+v2331)))/v813)}else{common.v1});
        let v2377=(common.v58*v821);
        let v2382=(if (v754!=0.0){((if (v754!=0.0){(v2349+((v817*v2321)+(v810*(v816*v2321))))}else{common.v1})/v2377)}else{common.v1});
        let v2383=(if (v754!=0.0){((if (v754!=0.0){(v2350+((v817*v2322)+(v810*(v816*v2322))))}else{common.v1})/v2377)}else{common.v1});
        let v2384=(if (v754!=0.0){((if (v754!=0.0){(v2351+((v817*v2323)+(v810*(v816*v2323))))}else{common.v1})/v2377)}else{common.v1});
        let v2385=(if (v754!=0.0){((if (v754!=0.0){(v2352+((v817*v2324)+(v810*(v816*v2324))))}else{common.v1})/v2377)}else{common.v1});
        let v2386=(common.v531*v2321);
        let v2387=(common.v531*v2322);
        let v2388=(common.v531*v2323);
        let v2389=(common.v531*v2324);
        let v2394=(if v825{(v2382+v2386)}else{common.v1});
        let v2395=(if v825{(v2383+v2387)}else{common.v1});
        let v2396=(if v825{(v2384+v2388)}else{common.v1});
        let v2397=(if v825{(v2385+v2389)}else{common.v1});
        let v2398=(-v2349);
        let v2399=(-v2350);
        let v2400=(-v2351);
        let v2401=(-v2352);
        let v2405=(v828*v828);
        let v2427=(if v833{(v2386-v2382)}else{(if v825{(((v828*v2398)-(v829*v2394))/v2405)}else{common.v1})});
        let v2428=(if v833{(v2387-v2383)}else{(if v825{(((v828*v2399)-(v829*v2395))/v2405)}else{common.v1})});
        let v2429=(if v833{(v2388-v2384)}else{(if v825{(((v828*v2400)-(v829*v2396))/v2405)}else{common.v1})});
        let v2430=(if v833{(v2389-v2385)}else{(if v825{(((v828*v2401)-(v829*v2397))/v2405)}else{common.v1})});
        let v2434=(v835*v835);
        let v2448=(if v833{(((v835*v2398)-(v829*v2427))/v2434)}else{v2394});
        let v2449=(if v833{(((v835*v2399)-(v829*v2428))/v2434)}else{v2395});
        let v2450=(if v833{(((v835*v2400)-(v829*v2429))/v2434)}else{v2396});
        let v2451=(if v833{(((v835*v2401)-(v829*v2430))/v2434)}else{v2397});
        let v2452=-0.6666666666666667;
        let v2454=(v801*f64::powf(v837,v2452));
        let v2468=(v801*f64::powf(v850,v2452));
        let v2490=(v801*f64::powf(v835,v2452));
        let v2504=(v801*f64::powf(v868,v2452));
        let v2541=(v816*v2204);
        let v2547=(common.v58*v883);
        let v2552=(if (v754!=0.0){((v2205+(if (v754!=0.0){(((if v855{(common.v229*v2448)}else{(if v849{(-((-v2448)*v2468))}else{(if v841{(v2448*v2454)}else{common.v1})})})+(if v873{(common.v229*v2427)}else{(if v867{(-((-v2427)*v2504))}else{(if v860{(v2427*v2490)}else{common.v1})})}))-(v801*v2209))}else{common.v1}))/v2547)}else{v2269});
        let v2553=(if (v754!=0.0){(((if (v754!=0.0){(((if v855{(common.v229*v2449)}else{(if v849{(-((-v2449)*v2468))}else{(if v841{(v2449*v2454)}else{common.v1})})})+(if v873{(common.v229*v2428)}else{(if v867{(-((-v2428)*v2504))}else{(if v860{(v2428*v2490)}else{common.v1})})}))-(v801*v2210))}else{common.v1})+(v2541-v2195))/v2547)}else{v2270});
        let v2554=(if (v754!=0.0){((v2207+(if (v754!=0.0){(((if v855{(common.v229*v2450)}else{(if v849{(-((-v2450)*v2468))}else{(if v841{(v2450*v2454)}else{common.v1})})})+(if v873{(common.v229*v2429)}else{(if v867{(-((-v2429)*v2504))}else{(if v860{(v2429*v2490)}else{common.v1})})}))-(v801*v2211))}else{common.v1}))/v2547)}else{v2271});
        let v2555=(if (v754!=0.0){((v2208+(if (v754!=0.0){(((if v855{(common.v229*v2451)}else{(if v849{(-((-v2451)*v2468))}else{(if v841{(v2451*v2454)}else{common.v1})})})+(if v873{(common.v229*v2430)}else{(if v867{(-((-v2430)*v2504))}else{(if v860{(v2430*v2490)}else{common.v1})})}))-(v801*v2212))}else{common.v1}))/v2547)}else{v2272});
        let v2557=(v884*v2552);
        let v2559=(v884*v2553);
        let v2561=(v884*v2554);
        let v2563=(v884*v2555);
        let v2577=(if (v754!=0.0){((-(v2557+v2557))-(common.v58*v2194))}else{common.v1});
        let v2578=(if (v754!=0.0){(((v885*v2204)-(v2559+v2559))-(common.v58*v2195))}else{common.v1});
        let v2579=(if (v754!=0.0){((-(v2561+v2561))-(common.v58*v2196))}else{common.v1});
        let v2580=(if (v754!=0.0){((-(v2563+v2563))-(common.v58*v2197))}else{common.v1});
        let v2615=(if (v754!=0.0){(((v884*((v784*v2194)-(common.v58*v2184)))-(v896*v2552))/v887)}else{common.v1});
        let v2616=(if (v754!=0.0){(((v884*((((v784*v2195)+(v782*v2201))-(common.v58*v2185))-((v880*v2201)+(v784*v2541))))-(v896*v2553))/v887)}else{common.v1});
        let v2617=(if (v754!=0.0){(((v884*((v784*v2196)-(common.v58*v2186)))-(v896*v2554))/v887)}else{common.v1});
        let v2618=(if (v754!=0.0){(((v884*((v784*v2197)-(common.v58*v2187)))-(v896*v2555))/v887)}else{common.v1});
        let v2627=(common.v58*v904);
        let v2632=(if v903{((if (v754!=0.0){(v2577+v2615)}else{common.v1})/v2627)}else{common.v1});
        let v2633=(if v903{((if (v754!=0.0){(v2578+v2616)}else{common.v1})/v2627)}else{common.v1});
        let v2634=(if v903{((if (v754!=0.0){(v2579+v2617)}else{common.v1})/v2627)}else{common.v1});
        let v2635=(if v903{((if (v754!=0.0){(v2580+v2618)}else{common.v1})/v2627)}else{common.v1});
        let v2636=(v906*v2201);
        let v2658=(v915*(if v913{(v2577-v2615)}else{common.v1}));
        let v2660=(v915*(if v913{(v2578-v2616)}else{common.v1}));
        let v2662=(v915*(if v913{(v2579-v2617)}else{common.v1}));
        let v2664=(v915*(if v913{(v2580-v2618)}else{common.v1}));
        let v2666=(common.v58*v919);
        let v2671=(common.v58*v920);
        let v2693=(-v2117);
        let v2694=(-v2118);
        let v2695=(-v2119);
        let v2699=(if v929{(common.v254*v2693)}else{common.v1});
        let v2700=(if v929{(common.v254*v2694)}else{common.v1});
        let v2701=(if v929{(common.v254*v2695)}else{common.v1});
        let v2732=(common.v58*v941);
        let v2742=(v942*v942);
        let v2756=(if v946{v2141}else{v2699});
        let v2757=(if v946{v2142}else{v2700});
        let v2758=(if v946{v2143}else{v2701});
        let v2762=(common.v58*v950);
        let v2772=(if v946{(((-v2756)+(v2756/v2762))/v953)}else{(if v929{(((v942*((v935*v2693)+(v930*(common.v58*(-(common.v58*v2699))))))-(v936*((-(common.v541*v2699))+((-(v769*v2699))/v2732))))/v2742)}else{(if v913{(common.v67*((if v913{(((v2658+v2658)/v2666)/v2671)}else{v2632})-v2552))}else{(if v903{(common.v67*(v2552+v2632))}else{common.v1})})})});
        let v2773=(if v946{common.v1}else{(if v929{common.v1}else{(if v913{(v2636+(common.v67*((if v913{(((v2660+v2660)/v2666)/v2671)}else{v2633})-v2553)))}else{(if v903{(v2636+(common.v67*(v2553+v2633)))}else{common.v1})})})});
        let v2774=(if v946{(((-v2757)+(v2757/v2762))/v953)}else{(if v929{(((v942*((v935*v2694)+(v930*(common.v58*(-(common.v58*v2700))))))-(v936*((-(common.v541*v2700))+((-(v769*v2700))/v2732))))/v2742)}else{(if v913{(common.v67*((if v913{(((v2662+v2662)/v2666)/v2671)}else{v2634})-v2554))}else{(if v903{(common.v67*(v2554+v2634))}else{common.v1})})})});
        let v2775=(if v946{(((-v2758)+(v2758/v2762))/v953)}else{(if v929{(((v942*((v935*v2695)+(v930*(common.v58*(-(common.v58*v2701))))))-(v936*((-(common.v541*v2701))+((-(v769*v2701))/v2732))))/v2742)}else{(if v913{(common.v67*((if v913{(((v2664+v2664)/v2666)/v2671)}else{v2635})-v2555))}else{(if v903{(common.v67*(v2555+v2635))}else{common.v1})})})});
        let v2776=(if (v959!=0.0){v2772}else{common.v1});
        let v2777=(if (v959!=0.0){v2773}else{common.v1});
        let v2778=(if (v959!=0.0){v2774}else{common.v1});
        let v2779=(if (v959!=0.0){v2775}else{common.v1});
        let v2780=(v2120+v2772);
        let v2781=(v2121+v2774);
        let v2782=(v2122+v2775);
        let v2783=(common.v58*v963);
        let v2792=(if (v959!=0.0){(common.v249*(v2780/v2783))}else{common.v1});
        let v2793=(if (v959!=0.0){(common.v249*(v2773/v2783))}else{common.v1});
        let v2794=(if (v959!=0.0){(common.v249*(v2781/v2783))}else{common.v1});
        let v2795=(if (v959!=0.0){(common.v249*(v2782/v2783))}else{common.v1});
        let v2797=(v2777/v191);
        let v2811=(if v966{(v692*(common.v67*(v2776/v191)))}else{common.v1});
        let v2812=(if v966{((v969*v2050)+(v692*(common.v67*(v2797-v2046))))}else{common.v1});
        let v2813=(if v966{(v692*(common.v67*(v2778/v191)))}else{common.v1});
        let v2814=(if v966{(v692*(common.v67*(v2779/v191)))}else{common.v1});
        let v2820=(if v966{((v973*v2050)+(v692*(common.v67*(v2046+v2797))))}else{common.v1});
        let v2821=(v971*v2811);
        let v2823=(v971*v2812);
        let v2825=(v971*v2813);
        let v2827=(v971*v2814);
        let v2830=(common.v58*v978);
        let v2835=(if v966{((v2821+v2821)/v2830)}else{common.v1});
        let v2836=(if v966{((v2047+(v2823+v2823))/v2830)}else{common.v1});
        let v2837=(if v966{((v2825+v2825)/v2830)}else{common.v1});
        let v2838=(if v966{((v2827+v2827)/v2830)}else{common.v1});
        let v2839=(v975*v2811);
        let v2841=(v975*v2820);
        let v2843=(v975*v2813);
        let v2845=(v975*v2814);
        let v2848=(common.v58*v982);
        let v2853=(if v966{((v2839+v2839)/v2848)}else{common.v1});
        let v2854=(if v966{((v2047+(v2841+v2841))/v2848)}else{common.v1});
        let v2855=(if v966{((v2843+v2843)/v2848)}else{common.v1});
        let v2856=(if v966{((v2845+v2845)/v2848)}else{common.v1});
        let v2862=(if v966{(v2835+v2853)}else{common.v1});
        let v2863=(if v966{((v2836+v2854)-v2048)}else{common.v1});
        let v2864=(if v966{(v2837+v2855)}else{common.v1});
        let v2865=(if v966{(v2838+v2856)}else{common.v1});
        let v2869=(v979*v979);
        let v2886=(v983*v983);
        let v2932=((v995*(common.v58*v2792))+(v994*(-v2792)));
        let v2935=((v995*(common.v58*v2793))+(v994*(-v2793)));
        let v2938=((v995*(common.v58*v2794))+(v994*(-v2794)));
        let v2941=((v995*(common.v58*v2795))+(v994*(-v2795)));
        let v2957=(v998*v998);
        let v2990=(v961*v961);
        let v3004=(common.v58*v1003);
        let v3029=(common.v58*v1007);
        let v3034=(if v1005{((((v961*v2932)-(v996*v2776))/v2990)/v3029)}else{(if v966{((((v961*((v1000*v2932)+(v996*(-(((v998*((v993*v2776)+(v961*(if v966{((v692*(common.v67*((((v979*v2811)-(v971*v2835))/v2869)+(((v983*v2811)-(v975*v2853))/v2886))))/v191)}else{common.v1}))))-(v997*v2862))/v2957)))))-(v1001*v2776))/v2990)/v3004)}else{common.v1})});
        let v3035=(if v1005{((((v961*v2935)-(v996*v2777))/v2990)/v3029)}else{(if v966{((((v961*((v1000*v2935)+(v996*(-(((v998*((v993*v2777)+(v961*(if v966{(((v990*v2050)+(v692*(common.v67*((((v979*v2812)-(v971*v2836))/v2869)+(((v983*v2820)-(v975*v2854))/v2886)))))/v191)}else{common.v1}))))-(v997*v2863))/v2957)))))-(v1001*v2777))/v2990)/v3004)}else{common.v1})});
        let v3036=(if v1005{((((v961*v2938)-(v996*v2778))/v2990)/v3029)}else{(if v966{((((v961*((v1000*v2938)+(v996*(-(((v998*((v993*v2778)+(v961*(if v966{((v692*(common.v67*((((v979*v2813)-(v971*v2837))/v2869)+(((v983*v2813)-(v975*v2855))/v2886))))/v191)}else{common.v1}))))-(v997*v2864))/v2957)))))-(v1001*v2778))/v2990)/v3004)}else{common.v1})});
        let v3037=(if v1005{((((v961*v2941)-(v996*v2779))/v2990)/v3029)}else{(if v966{((((v961*((v1000*v2941)+(v996*(-(((v998*((v993*v2779)+(v961*(if v966{((v692*(common.v67*((((v979*v2814)-(v971*v2838))/v2869)+(((v983*v2814)-(v975*v2856))/v2886))))/v191)}else{common.v1}))))-(v997*v2865))/v2957)))))-(v1001*v2779))/v2990)/v3004)}else{common.v1})});
        let v3042=(v1008*v3034);
        let v3044=(v1008*v3035);
        let v3046=(v1008*v3036);
        let v3048=(v1008*v3037);
        let v3053=(v1010*v1010);
        let v3082=(v1015*v1015);
        let v3096=(if (v959!=0.0){(((v1015*(self.scalar_static_f64[110]*v2772))-(v1014*v2776))/v3082)}else{common.v1});
        let v3097=(if (v959!=0.0){(((v1015*(self.scalar_static_f64[110]*v2773))-(v1014*v2777))/v3082)}else{common.v1});
        let v3098=(if (v959!=0.0){(((v1015*(self.scalar_static_f64[110]*v2774))-(v1014*v2778))/v3082)}else{common.v1});
        let v3099=(if (v959!=0.0){(((v1015*(self.scalar_static_f64[110]*v2775))-(v1014*v2779))/v3082)}else{common.v1});
        let v3116=(if (v959!=0.0){((v1019*v3096)+(v1018*(common.v305*v3096)))}else{common.v1});
        let v3117=(if (v959!=0.0){((v1019*v3097)+(v1018*(common.v305*v3097)))}else{common.v1});
        let v3118=(if (v959!=0.0){((v1019*v3098)+(v1018*(common.v305*v3098)))}else{common.v1});
        let v3119=(if (v959!=0.0){((v1019*v3099)+(v1018*(common.v305*v3099)))}else{common.v1});
        let v3120=(common.v58*v2058);
        let v3121=(common.v58*v2059);
        let v3122=(v1022*v2776);
        let v3123=(v1022*v2777);
        let v3126=((v1022*v2778)+(v961*v3120));
        let v3129=((v1022*v2779)+(v961*v3121));
        let v3134=(v1024*(-v2776));
        let v3135=(v3134+v3134);
        let v3136=(v1024*(-v2777));
        let v3137=(v3136+v3136);
        let v3138=(v1024*(v2058-v2778));
        let v3139=(v3138+v3138);
        let v3140=(v1024*(v2059-v2779));
        let v3141=(v3140+v3140);
        let v3146=(common.v58*v1027);
        let v3153=(v1028*v2776);
        let v3154=(v3153+v3153);
        let v3155=(v1028*v2777);
        let v3156=(v3155+v3155);
        let v3157=(v1028*(v2058+v2778));
        let v3158=(v3157+v3157);
        let v3159=(v1028*(v2059+v2779));
        let v3160=(v3159+v3159);
        let v3165=(common.v58*v1031);
        let v3177=(v1032*v1032);
        let v3191=(if (v959!=0.0){(((v1032*v3122)-(v1023*(((v3116+v3135)/v3146)+((v3116+v3154)/v3165))))/v3177)}else{common.v1});
        let v3192=(if (v959!=0.0){(((v1032*v3123)-(v1023*(((v3117+v3137)/v3146)+((v3117+v3156)/v3165))))/v3177)}else{common.v1});
        let v3193=(if (v959!=0.0){(((v1032*v3126)-(v1023*(((v3118+v3139)/v3146)+((v3118+v3158)/v3165))))/v3177)}else{common.v1});
        let v3194=(if (v959!=0.0){(((v1032*v3129)-(v1023*(((v3119+v3141)/v3146)+((v3119+v3160)/v3165))))/v3177)}else{common.v1});
        let v3215=(if v1036{(((v1015*(self.scalar_static_f64[110]*v3191))-(v1037*v2776))/v3082)}else{v3096});
        let v3216=(if v1036{(((v1015*(self.scalar_static_f64[110]*v3192))-(v1037*v2777))/v3082)}else{v3097});
        let v3217=(if v1036{(((v1015*(self.scalar_static_f64[110]*v3193))-(v1037*v2778))/v3082)}else{v3098});
        let v3218=(if v1036{(((v1015*(self.scalar_static_f64[110]*v3194))-(v1037*v2779))/v3082)}else{v3099});
        let v3235=(if v1036{((v1041*v3215)+(v1040*(common.v305*v3215)))}else{v3116});
        let v3236=(if v1036{((v1041*v3216)+(v1040*(common.v305*v3216)))}else{v3117});
        let v3237=(if v1036{((v1041*v3217)+(v1040*(common.v305*v3217)))}else{v3118});
        let v3238=(if v1036{((v1041*v3218)+(v1040*(common.v305*v3218)))}else{v3119});
        let v3243=(common.v58*v1045);
        let v3252=(common.v58*v1047);
        let v3264=(v1048*v1048);
        let v3278=(if v1036{(((v1048*v3122)-(v1023*(((v3135+v3235)/v3243)+((v3154+v3235)/v3252))))/v3264)}else{v3191});
        let v3279=(if v1036{(((v1048*v3123)-(v1023*(((v3137+v3236)/v3243)+((v3156+v3236)/v3252))))/v3264)}else{v3192});
        let v3280=(if v1036{(((v1048*v3126)-(v1023*(((v3139+v3237)/v3243)+((v3158+v3237)/v3252))))/v3264)}else{v3193});
        let v3281=(if v1036{(((v1048*v3129)-(v1023*(((v3141+v3238)/v3243)+((v3160+v3238)/v3252))))/v3264)}else{v3194});
        let v3286=(common.v58*v1052);
        let v3312=(v3279/v191);
        let v3326=(if v966{(v692*(common.v67*(v3278/v191)))}else{v2811});
        let v3327=(if v966{((v1058*v2050)+(v692*(common.v67*(v3312-v2046))))}else{v2812});
        let v3328=(if v966{(v692*(common.v67*(v3280/v191)))}else{v2813});
        let v3329=(if v966{(v692*(common.v67*(v3281/v191)))}else{v2814});
        let v3335=(if v966{((v1062*v2050)+(v692*(common.v67*(v2046+v3312))))}else{v2820});
        let v3336=(v1060*v3326);
        let v3338=(v1060*v3327);
        let v3340=(v1060*v3328);
        let v3342=(v1060*v3329);
        let v3345=(common.v58*v1067);
        let v3350=(if v966{((v3336+v3336)/v3345)}else{v2835});
        let v3351=(if v966{((v2047+(v3338+v3338))/v3345)}else{v2836});
        let v3352=(if v966{((v3340+v3340)/v3345)}else{v2837});
        let v3353=(if v966{((v3342+v3342)/v3345)}else{v2838});
        let v3354=(v1064*v3326);
        let v3356=(v1064*v3335);
        let v3358=(v1064*v3328);
        let v3360=(v1064*v3329);
        let v3363=(common.v58*v1071);
        let v3368=(if v966{((v3354+v3354)/v3363)}else{v2853});
        let v3369=(if v966{((v2047+(v3356+v3356))/v3363)}else{v2854});
        let v3370=(if v966{((v3358+v3358)/v3363)}else{v2855});
        let v3371=(if v966{((v3360+v3360)/v3363)}else{v2856});
        let v3397=(v1079*(-v2772));
        let v3399=(v1079*(-v2773));
        let v3401=(v1079*(v2058-v2774));
        let v3403=(v1079*(v2059-v2775));
        let v3405=(common.v58*v1082);
        let v3412=(v1083*v2772);
        let v3414=(v1083*v2773);
        let v3416=(v1083*(v2058+v2774));
        let v3418=(v1083*(v2059+v2775));
        let v3420=(common.v58*v1086);
        let v3432=(v1087*v1087);
        let v3446=(if v1077{(((v1087*(v1022*v2772))-(v1078*(((v3397+v3397)/v3405)+((v3412+v3412)/v3420))))/v3432)}else{v3278});
        let v3447=(if v1077{(((v1087*(v1022*v2773))-(v1078*(((v3399+v3399)/v3405)+((v3414+v3414)/v3420))))/v3432)}else{v3279});
        let v3448=(if v1077{(((v1087*((v1022*v2774)+(v955*v3120)))-(v1078*(((v3401+v3401)/v3405)+((v3416+v3416)/v3420))))/v3432)}else{v3280});
        let v3449=(if v1077{(((v1087*((v1022*v2775)+(v955*v3121)))-(v1078*(((v3403+v3403)/v3405)+((v3418+v3418)/v3420))))/v3432)}else{v3281});
        let v3451=(v3447/v191);
        let v3465=(if v1090{(v692*(common.v67*(v3446/v191)))}else{v3326});
        let v3467=(if v1090{(v692*(common.v67*(v3448/v191)))}else{v3328});
        let v3468=(if v1090{(v692*(common.v67*(v3449/v191)))}else{v3329});
        let v3475=(v1095*v3465);
        let v3477=(v1095*(if v1090{((v1093*v2050)+(v692*(common.v67*(v3451-v2046))))}else{v3327}));
        let v3479=(v1095*v3467);
        let v3481=(v1095*v3468);
        let v3484=(common.v58*v1102);
        let v3493=(v1099*v3465);
        let v3495=(v1099*(if v1090{((v1097*v2050)+(v692*(common.v67*(v2046+v3451))))}else{v3335}));
        let v3497=(v1099*v3467);
        let v3499=(v1099*v3468);
        let v3502=(common.v58*v1106);
        let v3527=(common.v58*v1114);
        let v3557=(v1122*v1122);
        let v3577=((v1124*v3446)+(v1089*(v707*(((v1122*(v455*(if (v1119!=0.0){common.v1}else{(if v1077{(-(common.v249*((v2120+v3446)/v3527)))}else{(if (v959!=0.0){(-((v1052*v3034)+(v1008*(((if (v959!=0.0){((((v1010*(common.v254*v2780))-(v1009*(v3042+v3042)))/v3053)-v2776)}else{common.v1})+v3278)/v3286))))}else{common.v1})})})))-(v1121*(if v1111{common.v1}else{(if v1090{((if v1090{((v3475+v3475)/v3484)}else{v3350})+(if v1090{((v3493+v3493)/v3502)}else{v3368}))}else{(if v1005{common.v1}else{(if v966{(v3350+v3368)}else{v2862})})})})))/v3557))));
        let v3580=((v1124*v3447)+(v1089*(v707*(((v1122*((v1120*(if self.scalar_static_bool[38]{((-(v317*v1784))/(v453*v453))}else{(if (self.scalar_static_f64[105]!=0.0){((-(v448*v1784))/(v449*v449))}else{common.v1})}))+(v455*(if (v1119!=0.0){common.v1}else{(if v1077{(-(common.v249*(v3447/v3527)))}else{(if (v959!=0.0){(-((v1052*v3035)+(v1008*(((if (v959!=0.0){((((v1010*(common.v254*v2773))-(v1009*(v3044+v3044)))/v3053)-v2777)}else{common.v1})+v3279)/v3286))))}else{common.v1})})}))))-(v1121*(if v1111{common.v1}else{(if v1090{(((if v1090{((v2047+(v3477+v3477))/v3484)}else{v3351})+(if v1090{((v2047+(v3495+v3495))/v3502)}else{v3369}))-v2048)}else{(if v1005{common.v1}else{(if v966{((v3351+v3369)-v2048)}else{v2863})})})})))/v3557))));
        let v3583=((v1124*v3448)+(v1089*(v707*(((v1122*(v455*(if (v1119!=0.0){common.v1}else{(if v1077{(-(common.v249*((v2121+v3448)/v3527)))}else{(if (v959!=0.0){(-((v1052*v3036)+(v1008*(((if (v959!=0.0){((((v1010*(common.v254*v2781))-(v1009*(v3046+v3046)))/v3053)-v2778)}else{common.v1})+v3280)/v3286))))}else{common.v1})})})))-(v1121*(if v1111{common.v1}else{(if v1090{((if v1090{((v3479+v3479)/v3484)}else{v3352})+(if v1090{((v3497+v3497)/v3502)}else{v3370}))}else{(if v1005{common.v1}else{(if v966{(v3352+v3370)}else{v2864})})})})))/v3557))));
        let v3586=((v1124*v3449)+(v1089*(v707*(((v1122*(v455*(if (v1119!=0.0){common.v1}else{(if v1077{(-(common.v249*((v2122+v3449)/v3527)))}else{(if (v959!=0.0){(-((v1052*v3037)+(v1008*(((if (v959!=0.0){((((v1010*(common.v254*v2782))-(v1009*(v3048+v3048)))/v3053)-v2779)}else{common.v1})+v3281)/v3286))))}else{common.v1})})})))-(v1121*(if v1111{common.v1}else{(if v1090{((if v1090{((v3481+v3481)/v3484)}else{v3353})+(if v1090{((v3499+v3499)/v3502)}else{v3371}))}else{(if v1005{common.v1}else{(if v966{(v3353+v3371)}else{v2865})})})})))/v3557))));
        let v3591=((-v1821)/(v490*v490));
        let v3592=(if v1132{v3591}else{common.v1});
        let v3593=(self.scalar_static_f64[173]*v1134);
        let v3595=(self.scalar_static_f64[172]*v1134);
        let v3606=(-v1832);
        let v3615=(if v1142{(v1144*v3593)}else{(if v1137{(v1139*v3593)}else{common.v1})});
        let v3616=(if v1142{((v1147*(v1144*((v1134*v1832)+(v499*v3592))))+(v1144*((v1145*v3592)+(v1134*v3606))))}else{(if v1137{(v1139*(common.v407*v3592))}else{common.v1})});
        let v3617=(if v1142{(v1144*v3595)}else{(if v1137{(v1139*v3595)}else{common.v1})});
        let v3631=((-v1837)/(v508*v508));
        let v3632=(if v1158{v3631}else{v3592});
        let v3633=(self.scalar_static_f64[173]*v1160);
        let v3635=(self.scalar_static_f64[172]*v1160);
        let v3646=(-v1848);
        let v3675=(-(if self.scalar_static_bool[48]{common.v1}else{v1975}));
        let v3677=(if v1186{v3675}else{common.v1});
        let v3684=((-((v641*common.v1775)+(common.v430*(if self.scalar_static_bool[48]{common.v1}else{v1978}))))/(v1190*v1190));
        let v3685=(if v1186{v3684}else{common.v1});
        let v3686=(v1192*(if v1186{self.scalar_static_f64[172]}else{common.v1}));
        let v3690=(v1192*(if v1186{self.scalar_static_f64[173]}else{common.v1}));
        let v3737=(if v1221{common.v1}else{(if (v1127!=0.0){(((if (v1127!=0.0){((if v1154{common.v1}else{(if v1132{(v1128*v3615)}else{common.v1})})+(if v1180{common.v1}else{(if v1158{(v1129*(if v1168{(v1170*v3633)}else{(if v1163{(v1165*v3633)}else{v3615})}))}else{common.v1})}))}else{common.v1})+(if v1215{common.v1}else{(if v1186{(self.scalar_static_f64[214]*(if v1200{(v1202*v3686)}else{(if v1195{(v1197*v3686)}else{common.v1})}))}else{common.v1})}))+self.scalar_static_f64[245])}else{common.v1})});
        let v3738=(if v1221{common.v1}else{(if (v1127!=0.0){((if (v1127!=0.0){((if v1154{common.v1}else{(if v1132{((v1150*(if (v1127!=0.0){v1849}else{common.v1}))+(v1128*v3616))}else{common.v1})})+(if v1180{common.v1}else{(if v1158{((v1176*(if (v1127!=0.0){(self.scalar_static_f64[22]*v1847)}else{common.v1}))+(v1129*(if v1168{((v1173*(v1170*((v1160*v1848)+(v516*v3632))))+(v1170*((v1171*v3632)+(v1160*v3646))))}else{(if v1163{(v1165*(common.v407*v3632))}else{v3616})})))}else{common.v1})}))}else{common.v1})+(if v1215{common.v1}else{(if v1186{(self.scalar_static_f64[214]*((if v1200{((v1205*(v1202*((v1192*v1996)+(v642*v3685))))+(v1202*((v1203*v3685)+(v1192*(v3677-v1996)))))}else{(if v1195{(v1197*((v1192*v3677)+(v1189*v3685)))}else{common.v1})})-(v1210*((v1192*v3675)+(v1187*v3685)))))}else{common.v1})}))}else{common.v1})});
        let v3739=(if v1221{common.v1}else{(if (v1127!=0.0){(((if (v1127!=0.0){((if v1154{common.v1}else{(if v1132{(v1128*v3617)}else{common.v1})})+(if v1180{common.v1}else{(if v1158{(v1129*(if v1168{(v1170*v3635)}else{(if v1163{(v1165*v3635)}else{v3617})}))}else{common.v1})}))}else{common.v1})+(if v1215{common.v1}else{(if v1186{(self.scalar_static_f64[214]*(if v1200{(v1202*v3690)}else{(if v1195{(v1197*v3690)}else{common.v1})}))}else{common.v1})}))+self.scalar_static_f64[246])}else{common.v1})});
        let v3742=(if v1229{v3591}else{common.v1});
        let v3743=(self.scalar_static_f64[173]*v1230);
        let v3745=(self.scalar_static_f64[172]*v1230);
        let v3764=(if v1238{(v1240*v3743)}else{(if v1233{(v1235*v3743)}else{common.v1})});
        let v3765=(if v1238{((v1243*(v1240*((v1230*v1832)+(v499*v3742))))+(v1240*((v1241*v3742)+(v1230*v3606))))}else{(if v1233{(v1235*(common.v409*v3742))}else{common.v1})});
        let v3766=(if v1238{(v1240*v3745)}else{(if v1233{(v1235*v3745)}else{common.v1})});
        let v3778=(if v1254{v3631}else{v3742});
        let v3779=(self.scalar_static_f64[173]*v1255);
        let v3781=(self.scalar_static_f64[172]*v1255);
        let v3821=(if v1279{v3675}else{common.v1});
        let v3823=(if v1279{v3684}else{common.v1});
        let v3824=(v1282*(if v1279{self.scalar_static_f64[172]}else{common.v1}));
        let v3828=(v1282*(if v1279{self.scalar_static_f64[173]}else{common.v1}));
        let v3873=(if v1309{common.v1}else{(if (v1224!=0.0){(self.scalar_static_f64[245]+((if (v1224!=0.0){((if v1250{common.v1}else{(if v1229{(v1225*v3764)}else{common.v1})})+(if v1275{common.v1}else{(if v1254{(v1226*(if v1263{(v1265*v3779)}else{(if v1258{(v1260*v3779)}else{v3764})}))}else{common.v1})}))}else{common.v1})+(if v1303{common.v1}else{(if v1279{(self.scalar_static_f64[214]*(if v1290{(v1292*v3824)}else{(if v1285{(v1287*v3824)}else{common.v1})}))}else{common.v1})})))}else{common.v1})});
        let v3874=(if v1309{common.v1}else{(if (v1224!=0.0){((if (v1224!=0.0){((if v1250{common.v1}else{(if v1229{((v1246*(if (v1224!=0.0){v1849}else{common.v1}))+(v1225*v3765))}else{common.v1})})+(if v1275{common.v1}else{(if v1254{((v1271*(if (v1224!=0.0){(self.scalar_static_f64[24]*v1847)}else{common.v1}))+(v1226*(if v1263{((v1268*(v1265*((v1255*v1848)+(v516*v3778))))+(v1265*((v1266*v3778)+(v1255*v3646))))}else{(if v1258{(v1260*(common.v409*v3778))}else{v3765})})))}else{common.v1})}))}else{common.v1})+(if v1303{common.v1}else{(if v1279{(self.scalar_static_f64[214]*((if v1290{((v1295*(v1292*((v1282*v1996)+(v642*v3823))))+(v1292*((v1293*v3823)+(v1282*(v3821-v1996)))))}else{(if v1285{(v1287*((v1282*v3821)+(v1281*v3823)))}else{common.v1})})-(v1299*((v1282*v3675)+(v1187*v3823)))))}else{common.v1})}))}else{common.v1})});
        let v3875=(if v1309{common.v1}else{(if (v1224!=0.0){(self.scalar_static_f64[246]+((if (v1224!=0.0){((if v1250{common.v1}else{(if v1229{(v1225*v3766)}else{common.v1})})+(if v1275{common.v1}else{(if v1254{(v1226*(if v1263{(v1265*v3781)}else{(if v1258{(v1260*v3781)}else{v3766})}))}else{common.v1})}))}else{common.v1})+(if v1303{common.v1}else{(if v1279{(self.scalar_static_f64[214]*(if v1290{(v1292*v3828)}else{(if v1285{(v1287*v3828)}else{common.v1})}))}else{common.v1})})))}else{common.v1})});
        let v3901=(-v1316);
        let v3903=(-v1321);

        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v1386),
            [1, 3, 4, 5],
            [(self.scalar_static_f64[173]*v3577), (self.scalar_static_f64[173]*v3580), (self.scalar_static_f64[173]*v3583), (self.scalar_static_f64[173]*v3586)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (v1387),
            1,
            multiplicity * ((self.scalar_static_f64[173]*v3737)),
            3,
            multiplicity * ((self.scalar_static_f64[173]*v3738)),
            4,
            multiplicity * ((self.scalar_static_f64[173]*v3739)),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * (v1388),
            1,
            multiplicity * ((self.scalar_static_f64[173]*v3873)),
            3,
            multiplicity * ((self.scalar_static_f64[173]*v3874)),
            5,
            multiplicity * ((self.scalar_static_f64[173]*v3875)),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[88]{(v9*common.v398)}else{(if self.scalar_static_bool[87]{(v1336*v1379)}else{(if self.scalar_static_bool[86]{((self.scalar_static_f64[303]*(f64::powf(v1368,self.scalar_static_f64[217])-common.v0))/self.scalar_static_f64[217])}else{(if self.scalar_static_bool[78]{v1336}else{common.v1})})})})),
            3,
            multiplicity * ((if self.scalar_static_bool[88]{v9}else{(if self.scalar_static_bool[87]{((self.scalar_static_f64[286]*v1379)+(v1336*self.scalar_static_f64[306]))}else{(if self.scalar_static_bool[86]{((self.scalar_static_f64[303]*(self.scalar_static_f64[305]*(self.scalar_static_f64[217]*f64::powf(v1368,self.scalar_static_f64[247]))))/self.scalar_static_f64[217])}else{self.scalar_static_f64[304]})})})),
        );
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[88]{common.v1}else{(if (self.scalar_static_f64[287]!=0.0){(-(((((v404*v1125)+(common.v407*v1222))+(common.v409*v1310))+(v1316*v1318))+(v1321*v1323)))}else{common.v1})})),
            &[(if self.scalar_static_bool[88]{common.v1}else{(if (self.scalar_static_f64[287]!=0.0){v3901}else{common.v1})}),(if self.scalar_static_bool[88]{common.v1}else{(if (self.scalar_static_f64[287]!=0.0){(-(((v404*v3577)+(v1387+(common.v407*v3737)))+(v1388+(common.v409*v3873))))}else{common.v1})}),(if self.scalar_static_bool[88]{common.v1}else{(if (self.scalar_static_f64[287]!=0.0){v3903}else{common.v1})}),(if self.scalar_static_bool[88]{common.v1}else{(if (self.scalar_static_f64[287]!=0.0){(-(((v404*v3580)+(common.v407*v3738))+(common.v409*v3874)))}else{common.v1})}),(if self.scalar_static_bool[88]{common.v1}else{(if (self.scalar_static_f64[287]!=0.0){(-((((self.scalar_static_f64[172]*v1125)+(v404*v3583))+((self.scalar_static_f64[172]*v1222)+(common.v407*v3739)))+v3901))}else{common.v1})}),(if self.scalar_static_bool[88]{common.v1}else{(if (self.scalar_static_f64[287]!=0.0){(-(((v1386+(v404*v3586))+((self.scalar_static_f64[172]*v1310)+(common.v409*v3875)))+v3903))}else{common.v1})})],
            &[(if self.scalar_static_bool[88]{common.v1}else{(if (self.scalar_static_f64[287]!=0.0){(-v1318)}else{common.v1})}),(if self.scalar_static_bool[88]{common.v1}else{(if (self.scalar_static_f64[287]!=0.0){(-v1323)}else{common.v1})})],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(4),
            0,
            multiplicity,
        );
        stamper.stamp_potential_node1_branch1_local(
            0,
            (if (v1749!=0.0){(v470*v1757)}else{common.v1}),
            3,
            (if (v1749!=0.0){(v1757*v1802)}else{common.v1}),
            0,
            (if (v1749!=0.0){v1753}else{common.v1}),
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * ((if v1760{(v1318/v1753)}else{common.v1})),
            0,
            multiplicity * ((if v1760{(common.v0/v1753)}else{common.v1})),
            3,
            multiplicity * ((if v1760{((-(v1318*(self.scalar_static_f64[122]*v1802)))/(v1753*v1753))}else{common.v1})),
            4,
            multiplicity * ((if v1760{(v700/v1753)}else{common.v1})),
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_node1_branch1_local(
            1,
            (if (v1752!=0.0){(v470*v1763)}else{common.v1}),
            3,
            (if (v1752!=0.0){(v1763*v1802)}else{common.v1}),
            1,
            (if (v1752!=0.0){v1755}else{common.v1}),
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(5),
            multiplicity * ((if v1766{(v1323/v1755)}else{common.v1})),
            2,
            multiplicity * ((if v1766{(common.v0/v1755)}else{common.v1})),
            3,
            multiplicity * ((if v1766{((-(v1323*(self.scalar_static_f64[126]*v1802)))/(v1755*v1755))}else{common.v1})),
            5,
            multiplicity * ((if v1766{(v700/v1755)}else{common.v1})),
        );
        let v1743_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v1743);
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (v1743_ddt),
            1,
            multiplicity * (((common.v4778) * ddt_scale)),
            3,
            multiplicity * (((common.v4779) * ddt_scale)),
            4,
            multiplicity * (((common.v4780) * ddt_scale)),
        );
        let v1744_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v1744);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (v1744_ddt),
            [1, 3, 4, 5],
            [((common.v4781) * ddt_scale), ((common.v4782) * ddt_scale), ((common.v4783) * ddt_scale), ((common.v4784) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1745_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v1745);
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v1745_ddt),
            3,
            multiplicity * (((self.scalar_static_f64[149]) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(5),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (common.v1),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        stamper.stamp_current_reactive_node3(
            Some(nodes[1]),
            Some(nodes[4]),
            nodes[1],
            multiplicity * (common.v4778),
            nodes[3],
            multiplicity * (common.v4779),
            nodes[4],
            multiplicity * (common.v4780),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &[nodes[1], nodes[3], nodes[4], nodes[5]],
            &[common.v4781, common.v4782, common.v4783, common.v4784],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (self.scalar_static_f64[149]),
        );
    }
}
