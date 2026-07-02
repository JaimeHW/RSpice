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
    v56: f64,
    v65: f64,
    v105: f64,
    v150: f64,
    v158: f64,
    v165: f64,
    v220: f64,
    v222: f64,
    v223: f64,
    v224: f64,
    v243: f64,
    v248: f64,
    v250: f64,
    v251: f64,
    v297: f64,
    v388: f64,
    v391: f64,
    v392: f64,
    v397: f64,
    v399: f64,
    v416: f64,
    v418: f64,
    v419: f64,
    v467: f64,
    v514: f64,
    v524: f64,
    v1677: f64,
    v1678: f64,
    v1679: f64,
    v1705: f64,
    v1707: f64,
    v1708: f64,
    v1744: f64,
    v1746: f64,
    v4710: f64,
    v4711: f64,
    v4712: f64,
    v4713: f64,
    v4714: f64,
    v4715: f64,
    v4716: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let v0=1.0;
        let v1=multiplicity;
        let v2=0.0;
        let v12=273.15;
        let v38=1.3806505e-23;
        let v40=1.60217653e-19;
        let v56=2.0;
        let v65=0.5;
        let v105=(v1*self.scalar_static_f64[61]);
        let v106=(v105).sqrt();
        let v109=(if (self.scalar_static_f64[62]!=0.0){(self.scalar_static_f64[66]+(self.scalar_static_f64[69]/v106))}else{self.scalar_static_f64[55]});
        let v118=((v1*self.scalar_static_f64[60])).sqrt();
        let v121=(if (self.scalar_static_f64[62]!=0.0){(self.scalar_static_f64[73]+(self.scalar_static_f64[76]/v118))}else{self.scalar_static_f64[56]});
        let v143=(if self.scalar_static_bool[9]{(self.scalar_static_f64[68]/v106)}else{v2});
        let v150=(if self.scalar_static_bool[9]{(v109+(self.scalar_static_f64[63]*((self.scalar_static_f64[83]+(v143*v143))).sqrt()))}else{v109});
        let v158=(if self.scalar_static_bool[15]{(self.scalar_static_f64[75]/v118)}else{v143});
        let v165=(if self.scalar_static_bool[15]{(v121+(self.scalar_static_f64[70]*((self.scalar_static_f64[84]+(v158*v158))).sqrt()))}else{v121});
        let v191=(if self.scalar_static_bool[24]{self.scalar_static_f64[15]}else{(if (self.scalar_static_f64[88]!=0.0){v150}else{self.scalar_static_f64[60]})});
        let v192=(if self.scalar_static_bool[24]{self.scalar_static_f64[17]}else{(if (self.scalar_static_f64[88]!=0.0){v165}else{self.scalar_static_f64[61]})});
        let v195=(v0/f64::powf(v191,self.scalar_static_f64[89]));
        let v198=(v0/f64::powf(v192,self.scalar_static_f64[90]));
        let v219=((((self.scalar_static_f64[91]*(v0+(v195*self.scalar_static_f64[92])))*(v0+(v198*self.scalar_static_f64[93])))*(v0+(v198*(v195*self.scalar_static_f64[94]))))*self.scalar_static_f64[254]);
        let v220=0.1;
        let v222=(if (v219>v220){v219}else{v220});
        let v223=(v222).sqrt();
        let v224=10000.0;
        let v226=(v223/(v222+v224));
        let v239=(if (self.scalar_static_f64[97]!=0.0){v2}else{(self.scalar_static_f64[98]+((((v192*self.scalar_static_f64[99])+(v191*self.scalar_static_f64[100]))+self.scalar_static_f64[101])/(v191*v192)))});
        let v240=(v239<v226);
        let v243=(if v240{(if (v239>v2){v239}else{v2})}else{v239});
        let v248=(if (!v240){(v243*v243)}else{(if v240{(v226*v226)}else{v2})});
        let v250=(v65*v222);
        let v251=((v65/v248)-v250);
        let v297=4.0;
        let v388=ctx.node_voltage(nodes[3]);
        let v391=ctx.node_voltage(nodes[5]);
        let v392=ctx.node_voltage(nodes[4]);
        let v395=ctx.node_voltage(nodes[1]);
        let v397=(self.scalar_static_f64[166]*(v395-v392));
        let v399=(self.scalar_static_f64[166]*(v395-v391));
        let v401=((self.scalar_static_f64[234]+v388)-v12);
        let v402=(v401<self.scalar_static_f64[11]);
        let v405=(((v401-self.scalar_static_f64[10])-v0)).exp();
        let v407=(if v402{(self.scalar_static_f64[10]+v405)}else{v401});
        let v410=((v407>self.scalar_static_f64[13])&&(!v402));
        let v413=(((self.scalar_static_f64[12]-v407)-v0)).exp();
        let v416=(v12+(if v410{(self.scalar_static_f64[12]-v413)}else{v407}));
        let v418=((v38*v416)/v40);
        let v419=(v416/self.scalar_static_f64[8]);
        let v467=(v419).ln();
        let v508=(v56*(v418/v419));
        let v511=(v419*self.scalar_static_f64[179]);
        let v513=((v511/v418)).exp();
        let v514=-0.5;
        let v516=(v419*self.scalar_static_f64[180]);
        let v518=((v516/v418)).exp();
        let v519=(v513-v518);
        let v520=(v519).ln();
        let v522=(if self.scalar_static_bool[43]{(v508*v520)}else{v2});
        let v524=3.0;
        let v525=(v418*v524);
        let v526=(v467*v525);
        let v529=(self.scalar_static_f64[171]*(v419-v0));
        let v531=(if self.scalar_static_bool[43]{(((v419*v522)-v526)-v529)}else{v2});
        let v532=(v56*v418);
        let v533=(-v531);
        let v535=((v533/v418)).exp();
        let v538=((v0+(v297*v535))).sqrt();
        let v540=(v65*(v0+v538));
        let v541=(v540).ln();
        let v544=(if self.scalar_static_bool[43]{(v531+(v532*v541))}else{v2});
        let v545=(self.scalar_static_f64[178]/v544);
        let v551=(if self.scalar_static_bool[44]{self.scalar_static_f64[178]}else{v544});
        let v556=(v419*self.scalar_static_f64[183]);
        let v558=((v556/v418)).exp();
        let v560=(v419*self.scalar_static_f64[184]);
        let v562=((v560/v418)).exp();
        let v563=(v558-v562);
        let v564=(v563).ln();
        let v566=(if self.scalar_static_bool[45]{(v508*v564)}else{v2});
        let v570=(if self.scalar_static_bool[45]{(((v419*v566)-v526)-v529)}else{v2});
        let v571=(-v570);
        let v573=((v571/v418)).exp();
        let v576=((v0+(v297*v573))).sqrt();
        let v578=(v65*(v0+v576));
        let v579=(v578).ln();
        let v582=(if self.scalar_static_bool[45]{(v570+(v532*v579))}else{v2});
        let v583=(self.scalar_static_f64[182]/v582);
        let v589=(if self.scalar_static_bool[46]{self.scalar_static_f64[182]}else{v582});
        let v590=(if self.scalar_static_bool[46]{v2}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[160]*f64::powf(v583,self.scalar_static_f64[185]))}else{v2})});
        let v1338=(v251+v397);
        let v1340=0.04;
        let v1342=(((v1338*v1338)+v1340)).sqrt();
        let v1347=(if self.scalar_static_bool[61]{v397}else{(if self.scalar_static_bool[60]{(v65*((v397-v251)+v1342))}else{v2})});
        let v1348=(self.scalar_static_f64[20]*(if self.scalar_static_bool[44]{v2}else{(if self.scalar_static_bool[43]{(self.scalar_static_f64[158]*f64::powf(v545,self.scalar_static_f64[181]))}else{v2})}));
        let v1349=(if self.scalar_static_bool[59]{v1348}else{v2});
        let v1351=(if self.scalar_static_bool[59]{(self.scalar_static_f64[22]*v590)}else{v2});
        let v1352=(v1349>v2);
        let v1353=(self.scalar_static_bool[59]&&v1352);
        let v1354=(-v551);
        let v1356=(v1354*self.scalar_static_f64[205]);
        let v1357=(if v1353{v1356}else{v2});
        let v1360=(v1353&&self.scalar_static_bool[62]);
        let v1361=(v1347+v1357);
        let v1362=(if v1360{v1361}else{v2});
        let v1363=(v1362>v2);
        let v1364=(v1360&&v1363);
        let v1368=(if v1364{self.scalar_static_f64[209]}else{v2});
        let v1370=(v0-(self.scalar_static_f64[207]*v1368));
        let v1376=(v1362*self.scalar_static_f64[211]);
        let v1377=(v551*self.scalar_static_f64[207]);
        let v1379=(v0+(v1376/v1377));
        let v1384=(v1360&&(!v1363));
        let v1386=(v0-(v1347/v551));
        let v1388=(v0-f64::powf(v1386,self.scalar_static_f64[210]));
        let v1391=(if v1384{((v551*v1388)/self.scalar_static_f64[210])}else{(if v1364{((v551*v1370)/self.scalar_static_f64[210])}else{v2})});
        let v1396=(v1353&&self.scalar_static_bool[63]);
        let v1401=(((v1357*v1357)+self.scalar_static_f64[213])).sqrt();
        let v1406=(if v1396{v1361}else{v2});
        let v1409=((self.scalar_static_f64[213]+(v1406*v1406))).sqrt();
        let v1414=(if v1396{((v65*(v1406-(if v1396{v1409}else{v2})))-v1357)}else{v2});
        let v1416=(v0-(v1414/v551));
        let v1417=f64::powf(v1416,self.scalar_static_f64[210]);
        let v1422=((if v1396{(v514*(v1357+(if v1396{v1401}else{v2})))}else{v2})+(v1347-v1414));
        let v1423=(self.scalar_static_f64[209]*v1422);
        let v1424=(self.scalar_static_f64[211]*v1422);
        let v1426=(v0+(v1424/v1377));
        let v1431=(self.scalar_static_bool[59]&&(!v1352));
        let v1432=(if v1431{v2}else{(if v1396{((if v1396{((v1354*v1417)/self.scalar_static_f64[210])}else{v1391})+(v1423*v1426))}else{(if v1360{(v1391+(if v1384{v2}else{(if v1364{(v1368*(v1362*v1379))}else{v2})}))}else{v2})})});
        let v1433=(v1351>v2);
        let v1434=(self.scalar_static_bool[59]&&v1433);
        let v1435=(-v589);
        let v1436=(self.scalar_static_f64[205]*v1435);
        let v1437=(if v1434{v1436}else{v2});
        let v1440=(v1434&&self.scalar_static_bool[64]);
        let v1441=(v1347+v1437);
        let v1442=(if v1440{v1441}else{v2});
        let v1443=(v1442>v2);
        let v1444=(v1440&&v1443);
        let v1447=(if v1444{self.scalar_static_f64[216]}else{v2});
        let v1449=(v0-(self.scalar_static_f64[207]*v1447));
        let v1455=(v1442*self.scalar_static_f64[218]);
        let v1456=(v589*self.scalar_static_f64[207]);
        let v1458=(v0+(v1455/v1456));
        let v1463=(v1440&&(!v1443));
        let v1465=(v0-(v1347/v589));
        let v1467=(v0-f64::powf(v1465,self.scalar_static_f64[217]));
        let v1470=(if v1463{((v589*v1467)/self.scalar_static_f64[217])}else{(if v1444{((v589*v1449)/self.scalar_static_f64[217])}else{v2})});
        let v1475=(v1434&&self.scalar_static_bool[65]);
        let v1480=(((v1437*v1437)+self.scalar_static_f64[220])).sqrt();
        let v1485=(if v1475{v1441}else{v2});
        let v1488=((self.scalar_static_f64[220]+(v1485*v1485))).sqrt();
        let v1493=(if v1475{((v65*(v1485-(if v1475{v1488}else{v2})))-v1437)}else{v2});
        let v1495=(v0-(v1493/v589));
        let v1496=f64::powf(v1495,self.scalar_static_f64[217]);
        let v1501=((if v1475{(v514*(v1437+(if v1475{v1480}else{v2})))}else{v2})+(v1347-v1493));
        let v1502=(self.scalar_static_f64[216]*v1501);
        let v1503=(self.scalar_static_f64[218]*v1501);
        let v1505=(v0+(v1503/v1456));
        let v1510=(self.scalar_static_bool[59]&&(!v1433));
        let v1511=(if v1510{v2}else{(if v1475{((if v1475{((v1435*v1496)/self.scalar_static_f64[217])}else{v1470})+(v1502*v1505))}else{(if v1440{(v1470+(if v1463{v2}else{(if v1444{(v1447*(v1442*v1458))}else{v2})}))}else{v2})})});
        let v1521=(v251+v399);
        let v1524=((v1340+(v1521*v1521))).sqrt();
        let v1529=(if self.scalar_static_bool[69]{v399}else{(if self.scalar_static_bool[68]{(v65*((v399-v251)+v1524))}else{v1347})});
        let v1530=(if self.scalar_static_bool[67]{v1348}else{v2});
        let v1532=(if self.scalar_static_bool[67]{(self.scalar_static_f64[24]*v590)}else{v2});
        let v1533=(v1530>v2);
        let v1534=(self.scalar_static_bool[67]&&v1533);
        let v1535=(if v1534{v1356}else{v2});
        let v1536=(self.scalar_static_bool[62]&&v1534);
        let v1537=(v1529+v1535);
        let v1538=(if v1536{v1537}else{v2});
        let v1539=(v1538>v2);
        let v1540=(v1536&&v1539);
        let v1541=(if v1540{self.scalar_static_f64[209]}else{v2});
        let v1543=(v0-(self.scalar_static_f64[207]*v1541));
        let v1547=(self.scalar_static_f64[211]*v1538);
        let v1549=(v0+(v1547/v1377));
        let v1554=(v1536&&(!v1539));
        let v1556=(v0-(v1529/v551));
        let v1558=(v0-f64::powf(v1556,self.scalar_static_f64[210]));
        let v1561=(if v1554{((v551*v1558)/self.scalar_static_f64[210])}else{(if v1540{((v551*v1543)/self.scalar_static_f64[210])}else{v2})});
        let v1565=(self.scalar_static_bool[63]&&v1534);
        let v1568=((self.scalar_static_f64[213]+(v1535*v1535))).sqrt();
        let v1573=(if v1565{v1537}else{v2});
        let v1576=((self.scalar_static_f64[213]+(v1573*v1573))).sqrt();
        let v1581=(if v1565{((v65*(v1573-(if v1565{v1576}else{v2})))-v1535)}else{v2});
        let v1583=(v0-(v1581/v551));
        let v1584=f64::powf(v1583,self.scalar_static_f64[210]);
        let v1589=((if v1565{(v514*(v1535+(if v1565{v1568}else{v2})))}else{v2})+(v1529-v1581));
        let v1590=(self.scalar_static_f64[209]*v1589);
        let v1591=(self.scalar_static_f64[211]*v1589);
        let v1593=(v0+(v1591/v1377));
        let v1598=(self.scalar_static_bool[67]&&(!v1533));
        let v1599=(if v1598{v2}else{(if v1565{((if v1565{((v1354*v1584)/self.scalar_static_f64[210])}else{v1561})+(v1590*v1593))}else{(if v1536{(v1561+(if v1554{v2}else{(if v1540{(v1541*(v1538*v1549))}else{v2})}))}else{v2})})});
        let v1600=(v1532>v2);
        let v1601=(self.scalar_static_bool[67]&&v1600);
        let v1602=(if v1601{v1436}else{v2});
        let v1603=(self.scalar_static_bool[64]&&v1601);
        let v1604=(v1529+v1602);
        let v1605=(if v1603{v1604}else{v2});
        let v1606=(v1605>v2);
        let v1607=(v1603&&v1606);
        let v1608=(if v1607{self.scalar_static_f64[216]}else{v2});
        let v1610=(v0-(self.scalar_static_f64[207]*v1608));
        let v1614=(self.scalar_static_f64[218]*v1605);
        let v1616=(v0+(v1614/v1456));
        let v1621=(v1603&&(!v1606));
        let v1623=(v0-(v1529/v589));
        let v1625=(v0-f64::powf(v1623,self.scalar_static_f64[217]));
        let v1628=(if v1621{((v589*v1625)/self.scalar_static_f64[217])}else{(if v1607{((v589*v1610)/self.scalar_static_f64[217])}else{v2})});
        let v1632=(self.scalar_static_bool[65]&&v1601);
        let v1635=((self.scalar_static_f64[220]+(v1602*v1602))).sqrt();
        let v1640=(if v1632{v1604}else{v2});
        let v1643=((self.scalar_static_f64[220]+(v1640*v1640))).sqrt();
        let v1648=(if v1632{((v65*(v1640-(if v1632{v1643}else{v2})))-v1602)}else{v2});
        let v1650=(v0-(v1648/v589));
        let v1651=f64::powf(v1650,self.scalar_static_f64[217]);
        let v1656=((if v1632{(v514*(v1602+(if v1632{v1635}else{v2})))}else{v2})+(v1529-v1648));
        let v1657=(self.scalar_static_f64[216]*v1656);
        let v1658=(self.scalar_static_f64[218]*v1656);
        let v1660=(v0+(v1658/v1456));
        let v1665=(self.scalar_static_bool[67]&&(!v1600));
        let v1666=(if v1665{v2}else{(if v1632{((if v1632{((v1435*v1651)/self.scalar_static_f64[217])}else{v1628})+(v1657*v1660))}else{(if v1603{(v1628+(if v1621{v2}else{(if v1607{(v1608*(v1605*v1616))}else{v2})}))}else{v2})})});
        let v1677=(self.scalar_static_f64[166]*((if self.scalar_static_bool[66]{v2}else{(if self.scalar_static_bool[59]{((v1349*v1432)+(v1351*v1511))}else{v2})})+(self.scalar_static_f64[155]*v397)));
        let v1678=(self.scalar_static_f64[166]*((if self.scalar_static_bool[70]{v2}else{(if self.scalar_static_bool[67]{((v1530*v1599)+(v1532*v1666))}else{v2})})+(self.scalar_static_f64[157]*v399)));
        let v1679=(self.scalar_static_f64[142]*v388);
        let v1701=(if v402{v405}else{v0});
        let v1705=(if v410{(-(v413*(-v1701)))}else{v1701});
        let v1707=((v38*v1705)/v40);
        let v1708=(v1705/self.scalar_static_f64[8]);
        let v1744=(v418*v418);
        let v1746=(v1708/v419);
        let v1789=(v56*(((v419*v1707)-(v418*v1708))/(v419*v419)));
        let v1814=((v525*v1746)+(v467*(v524*v1707)));
        let v1816=(self.scalar_static_f64[171]*v1708);
        let v1818=(if self.scalar_static_bool[43]{((((v522*v1708)+(v419*(if self.scalar_static_bool[43]{((v520*v1789)+(v508*(((v513*(((v418*(self.scalar_static_f64[179]*v1708))-(v511*v1707))/v1744))-(v518*(((v418*(self.scalar_static_f64[180]*v1708))-(v516*v1707))/v1744)))/v519)))}else{v2})))-v1814)-v1816)}else{v2});
        let v1819=(v56*v1707);
        let v1835=(if self.scalar_static_bool[43]{(v1818+((v541*v1819)+(v532*((v65*((v297*(v535*(((v418*(-v1818))-(v533*v1707))/v1744)))/(v56*v538)))/v540))))}else{v2});
        let v1846=(if self.scalar_static_bool[44]{v2}else{v1835});
        let v1871=(if self.scalar_static_bool[45]{((((v566*v1708)+(v419*(if self.scalar_static_bool[45]{((v564*v1789)+(v508*(((v558*(((v418*(self.scalar_static_f64[183]*v1708))-(v556*v1707))/v1744))-(v562*(((v418*(self.scalar_static_f64[184]*v1708))-(v560*v1707))/v1744)))/v563)))}else{v2})))-v1814)-v1816)}else{v2});
        let v1887=(if self.scalar_static_bool[45]{(v1871+((v579*v1819)+(v532*((v65*((v297*(v573*(((v418*(-v1871))-(v571*v1707))/v1744)))/(v56*v576)))/v578))))}else{v2});
        let v1898=(if self.scalar_static_bool[46]{v2}else{v1887});
        let v1899=(if self.scalar_static_bool[46]{v2}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[160]*(((-(self.scalar_static_f64[182]*v1887))/(v582*v582))*(self.scalar_static_f64[185]*f64::powf(v583,self.scalar_static_f64[224]))))}else{v2})});
        let v3884=(self.scalar_static_f64[166]*v1338);
        let v3886=(self.scalar_static_f64[165]*v1338);
        let v3888=(v56*v1342);
        let v3897=(if self.scalar_static_bool[61]{self.scalar_static_f64[166]}else{(if self.scalar_static_bool[60]{(v65*(self.scalar_static_f64[166]+((v3884+v3884)/v3888)))}else{v2})});
        let v3898=(if self.scalar_static_bool[61]{self.scalar_static_f64[165]}else{(if self.scalar_static_bool[60]{(v65*(self.scalar_static_f64[165]+((v3886+v3886)/v3888)))}else{v2})});
        let v3899=(self.scalar_static_f64[20]*(if self.scalar_static_bool[44]{v2}else{(if self.scalar_static_bool[43]{(self.scalar_static_f64[158]*(((-(self.scalar_static_f64[178]*v1835))/(v544*v544))*(self.scalar_static_f64[181]*f64::powf(v545,self.scalar_static_f64[223]))))}else{v2})}));
        let v3903=(-v1846);
        let v3904=(self.scalar_static_f64[205]*v3903);
        let v3905=(if v1353{v3904}else{v2});
        let v3906=(if v1360{v3897}else{v2});
        let v3907=(if v1360{v3905}else{v2});
        let v3908=(if v1360{v3898}else{v2});
        let v3915=(self.scalar_static_f64[207]*v1846);
        let v3920=(v1377*v1377);
        let v3941=(v551*v551);
        let v3949=(self.scalar_static_f64[210]*f64::powf(v1386,self.scalar_static_f64[228]));
        let v3964=(if v1384{((v551*(-((-(v3897/v551))*v3949)))/self.scalar_static_f64[210])}else{v2});
        let v3965=(if v1384{(((v1388*v1846)+(v551*(-((-((-(v1347*v1846))/v3941))*v3949))))/self.scalar_static_f64[210])}else{(if v1364{((v1370*v1846)/self.scalar_static_f64[210])}else{v2})});
        let v3966=(if v1384{((v551*(-((-(v3898/v551))*v3949)))/self.scalar_static_f64[210])}else{v2});
        let v3976=(v1357*v3905);
        let v3984=(if v1396{v3897}else{v2});
        let v3985=(if v1396{v3905}else{v2});
        let v3986=(if v1396{v3898}else{v2});
        let v3987=(v1406*v3984);
        let v3989=(v1406*v3985);
        let v3991=(v1406*v3986);
        let v3993=(v56*v1409);
        let v4007=(if v1396{(v65*(v3984-(if v1396{((v3987+v3987)/v3993)}else{v2})))}else{v2});
        let v4008=(if v1396{((v65*(v3985-(if v1396{((v3989+v3989)/v3993)}else{v2})))-v3905)}else{v2});
        let v4009=(if v1396{(v65*(v3986-(if v1396{((v3991+v3991)/v3993)}else{v2})))}else{v2});
        let v4020=(self.scalar_static_f64[210]*f64::powf(v1416,self.scalar_static_f64[228]));
        let v4035=(v3897-v4007);
        let v4037=(v3898-v4009);
        let v4038=((if v1396{(v514*(v3905+(if v1396{((v3976+v3976)/(v56*v1401))}else{v2})))}else{v2})+(-v4008));
        let v4069=(-v1898);
        let v4070=(self.scalar_static_f64[205]*v4069);
        let v4071=(if v1434{v4070}else{v2});
        let v4072=(if v1440{v3897}else{v2});
        let v4073=(if v1440{v4071}else{v2});
        let v4074=(if v1440{v3898}else{v2});
        let v4081=(self.scalar_static_f64[207]*v1898);
        let v4086=(v1456*v1456);
        let v4107=(v589*v589);
        let v4115=(self.scalar_static_f64[217]*f64::powf(v1465,self.scalar_static_f64[229]));
        let v4130=(if v1463{((v589*(-((-(v3897/v589))*v4115)))/self.scalar_static_f64[217])}else{v2});
        let v4131=(if v1463{(((v1467*v1898)+(v589*(-((-((-(v1347*v1898))/v4107))*v4115))))/self.scalar_static_f64[217])}else{(if v1444{((v1449*v1898)/self.scalar_static_f64[217])}else{v2})});
        let v4132=(if v1463{((v589*(-((-(v3898/v589))*v4115)))/self.scalar_static_f64[217])}else{v2});
        let v4142=(v1437*v4071);
        let v4150=(if v1475{v3897}else{v2});
        let v4151=(if v1475{v4071}else{v2});
        let v4152=(if v1475{v3898}else{v2});
        let v4153=(v1485*v4150);
        let v4155=(v1485*v4151);
        let v4157=(v1485*v4152);
        let v4159=(v56*v1488);
        let v4173=(if v1475{(v65*(v4150-(if v1475{((v4153+v4153)/v4159)}else{v2})))}else{v2});
        let v4174=(if v1475{((v65*(v4151-(if v1475{((v4155+v4155)/v4159)}else{v2})))-v4071)}else{v2});
        let v4175=(if v1475{(v65*(v4152-(if v1475{((v4157+v4157)/v4159)}else{v2})))}else{v2});
        let v4186=(self.scalar_static_f64[217]*f64::powf(v1495,self.scalar_static_f64[229]));
        let v4201=(v3897-v4173);
        let v4203=(v3898-v4175);
        let v4204=((if v1475{(v514*(v4071+(if v1475{((v4142+v4142)/(v56*v1480))}else{v2})))}else{v2})+(-v4174));
        let v4246=(((v1432*(if self.scalar_static_bool[59]{v3899}else{v2}))+(v1349*(if v1431{v2}else{(if v1396{((if v1396{(((v1417*v3903)+(v1354*((-(((v551*v4008)-(v1414*v1846))/v3941))*v4020)))/self.scalar_static_f64[210])}else{v3965})+((v1426*(self.scalar_static_f64[209]*v4038))+(v1423*(((v1377*(self.scalar_static_f64[211]*v4038))-(v1424*v3915))/v3920))))}else{(if v1360{(v3965+(if v1384{v2}else{(if v1364{(v1368*((v1379*v3907)+(v1362*(((v1377*(self.scalar_static_f64[211]*v3907))-(v1376*v3915))/v3920))))}else{v2})}))}else{v2})})})))+((v1511*(if self.scalar_static_bool[59]{(self.scalar_static_f64[22]*v1899)}else{v2}))+(v1351*(if v1510{v2}else{(if v1475{((if v1475{(((v1496*v4069)+(v1435*((-(((v589*v4174)-(v1493*v1898))/v4107))*v4186)))/self.scalar_static_f64[217])}else{v4131})+((v1505*(self.scalar_static_f64[216]*v4204))+(v1502*(((v1456*(self.scalar_static_f64[218]*v4204))-(v1503*v4081))/v4086))))}else{(if v1440{(v4131+(if v1463{v2}else{(if v1444{(v1447*((v1458*v4073)+(v1442*(((v1456*(self.scalar_static_f64[218]*v4073))-(v1455*v4081))/v4086))))}else{v2})}))}else{v2})})}))));
        let v4254=(self.scalar_static_f64[166]*v1521);
        let v4256=(self.scalar_static_f64[165]*v1521);
        let v4258=(v56*v1524);
        let v4268=(if self.scalar_static_bool[69]{self.scalar_static_f64[166]}else{(if self.scalar_static_bool[68]{(v65*(self.scalar_static_f64[166]+((v4254+v4254)/v4258)))}else{v3897})});
        let v4269=(if self.scalar_static_bool[69]{v2}else{(if self.scalar_static_bool[68]{v2}else{v3898})});
        let v4270=(if self.scalar_static_bool[69]{self.scalar_static_f64[165]}else{(if self.scalar_static_bool[68]{(v65*(self.scalar_static_f64[165]+((v4256+v4256)/v4258)))}else{v2})});
        let v4274=(if v1534{v3904}else{v2});
        let v4275=(if v1536{v4268}else{v2});
        let v4276=(if v1536{v4274}else{v2});
        let v4277=(if v1536{v4269}else{v2});
        let v4278=(if v1536{v4270}else{v2});
        let v4324=(self.scalar_static_f64[210]*f64::powf(v1556,self.scalar_static_f64[228]));
        let v4343=(if v1554{((v551*(-((-(v4268/v551))*v4324)))/self.scalar_static_f64[210])}else{v2});
        let v4344=(if v1554{(((v1558*v1846)+(v551*(-((-((-(v1529*v1846))/v3941))*v4324))))/self.scalar_static_f64[210])}else{(if v1540{((v1543*v1846)/self.scalar_static_f64[210])}else{v2})});
        let v4345=(if v1554{((v551*(-((-(v4269/v551))*v4324)))/self.scalar_static_f64[210])}else{v2});
        let v4346=(if v1554{((v551*(-((-(v4270/v551))*v4324)))/self.scalar_static_f64[210])}else{v2});
        let v4359=(v1535*v4274);
        let v4367=(if v1565{v4268}else{v2});
        let v4368=(if v1565{v4274}else{v2});
        let v4369=(if v1565{v4269}else{v2});
        let v4370=(if v1565{v4270}else{v2});
        let v4371=(v1573*v4367);
        let v4373=(v1573*v4368);
        let v4375=(v1573*v4369);
        let v4377=(v1573*v4370);
        let v4379=(v56*v1576);
        let v4397=(if v1565{(v65*(v4367-(if v1565{((v4371+v4371)/v4379)}else{v2})))}else{v2});
        let v4398=(if v1565{((v65*(v4368-(if v1565{((v4373+v4373)/v4379)}else{v2})))-v4274)}else{v2});
        let v4399=(if v1565{(v65*(v4369-(if v1565{((v4375+v4375)/v4379)}else{v2})))}else{v2});
        let v4400=(if v1565{(v65*(v4370-(if v1565{((v4377+v4377)/v4379)}else{v2})))}else{v2});
        let v4413=(self.scalar_static_f64[210]*f64::powf(v1583,self.scalar_static_f64[228]));
        let v4432=(v4268-v4397);
        let v4434=(v4269-v4399);
        let v4435=(v4270-v4400);
        let v4436=((if v1565{(v514*(v4274+(if v1565{((v4359+v4359)/(v56*v1568))}else{v2})))}else{v2})+(-v4398));
        let v4476=(if v1601{v4070}else{v2});
        let v4477=(if v1603{v4268}else{v2});
        let v4478=(if v1603{v4476}else{v2});
        let v4479=(if v1603{v4269}else{v2});
        let v4480=(if v1603{v4270}else{v2});
        let v4526=(self.scalar_static_f64[217]*f64::powf(v1623,self.scalar_static_f64[229]));
        let v4545=(if v1621{((v589*(-((-(v4268/v589))*v4526)))/self.scalar_static_f64[217])}else{v2});
        let v4546=(if v1621{(((v1625*v1898)+(v589*(-((-((-(v1529*v1898))/v4107))*v4526))))/self.scalar_static_f64[217])}else{(if v1607{((v1610*v1898)/self.scalar_static_f64[217])}else{v2})});
        let v4547=(if v1621{((v589*(-((-(v4269/v589))*v4526)))/self.scalar_static_f64[217])}else{v2});
        let v4548=(if v1621{((v589*(-((-(v4270/v589))*v4526)))/self.scalar_static_f64[217])}else{v2});
        let v4561=(v1602*v4476);
        let v4569=(if v1632{v4268}else{v2});
        let v4570=(if v1632{v4476}else{v2});
        let v4571=(if v1632{v4269}else{v2});
        let v4572=(if v1632{v4270}else{v2});
        let v4573=(v1640*v4569);
        let v4575=(v1640*v4570);
        let v4577=(v1640*v4571);
        let v4579=(v1640*v4572);
        let v4581=(v56*v1643);
        let v4599=(if v1632{(v65*(v4569-(if v1632{((v4573+v4573)/v4581)}else{v2})))}else{v2});
        let v4600=(if v1632{((v65*(v4570-(if v1632{((v4575+v4575)/v4581)}else{v2})))-v4476)}else{v2});
        let v4601=(if v1632{(v65*(v4571-(if v1632{((v4577+v4577)/v4581)}else{v2})))}else{v2});
        let v4602=(if v1632{(v65*(v4572-(if v1632{((v4579+v4579)/v4581)}else{v2})))}else{v2});
        let v4615=(self.scalar_static_f64[217]*f64::powf(v1650,self.scalar_static_f64[229]));
        let v4634=(v4268-v4599);
        let v4636=(v4269-v4601);
        let v4637=(v4270-v4602);
        let v4638=((if v1632{(v514*(v4476+(if v1632{((v4561+v4561)/(v56*v1635))}else{v2})))}else{v2})+(-v4600));
        let v4691=(((v1599*(if self.scalar_static_bool[67]{v3899}else{v2}))+(v1530*(if v1598{v2}else{(if v1565{((if v1565{(((v1584*v3903)+(v1354*((-(((v551*v4398)-(v1581*v1846))/v3941))*v4413)))/self.scalar_static_f64[210])}else{v4344})+((v1593*(self.scalar_static_f64[209]*v4436))+(v1590*(((v1377*(self.scalar_static_f64[211]*v4436))-(v1591*v3915))/v3920))))}else{(if v1536{(v4344+(if v1554{v2}else{(if v1540{(v1541*((v1549*v4276)+(v1538*(((v1377*(self.scalar_static_f64[211]*v4276))-(v1547*v3915))/v3920))))}else{v2})}))}else{v2})})})))+((v1666*(if self.scalar_static_bool[67]{(self.scalar_static_f64[24]*v1899)}else{v2}))+(v1532*(if v1665{v2}else{(if v1632{((if v1632{(((v1651*v4069)+(v1435*((-(((v589*v4600)-(v1648*v1898))/v4107))*v4615)))/self.scalar_static_f64[217])}else{v4546})+((v1660*(self.scalar_static_f64[216]*v4638))+(v1657*(((v1456*(self.scalar_static_f64[218]*v4638))-(v1658*v4081))/v4086))))}else{(if v1603{(v4546+(if v1621{v2}else{(if v1607{(v1608*((v1616*v4478)+(v1605*(((v1456*(self.scalar_static_f64[218]*v4478))-(v1614*v4081))/v4086))))}else{v2})}))}else{v2})})}))));
        let v4710=(self.scalar_static_f64[166]*((if self.scalar_static_bool[66]{v2}else{(if self.scalar_static_bool[59]{((v1349*(if v1431{v2}else{(if v1396{((if v1396{((v1354*((-(v4007/v551))*v4020))/self.scalar_static_f64[210])}else{v3964})+((v1426*(self.scalar_static_f64[209]*v4035))+(v1423*((self.scalar_static_f64[211]*v4035)/v1377))))}else{(if v1360{(v3964+(if v1384{v2}else{(if v1364{(v1368*((v1379*v3906)+(v1362*((self.scalar_static_f64[211]*v3906)/v1377))))}else{v2})}))}else{v2})})}))+(v1351*(if v1510{v2}else{(if v1475{((if v1475{((v1435*((-(v4173/v589))*v4186))/self.scalar_static_f64[217])}else{v4130})+((v1505*(self.scalar_static_f64[216]*v4201))+(v1502*((self.scalar_static_f64[218]*v4201)/v1456))))}else{(if v1440{(v4130+(if v1463{v2}else{(if v1444{(v1447*((v1458*v4072)+(v1442*((self.scalar_static_f64[218]*v4072)/v1456))))}else{v2})}))}else{v2})})})))}else{v2})})+self.scalar_static_f64[230]));
        let v4711=(self.scalar_static_f64[166]*(if self.scalar_static_bool[66]{v2}else{(if self.scalar_static_bool[59]{v4246}else{v2})}));
        let v4712=(self.scalar_static_f64[166]*((if self.scalar_static_bool[66]{v2}else{(if self.scalar_static_bool[59]{((v1349*(if v1431{v2}else{(if v1396{((if v1396{((v1354*((-(v4009/v551))*v4020))/self.scalar_static_f64[210])}else{v3966})+((v1426*(self.scalar_static_f64[209]*v4037))+(v1423*((self.scalar_static_f64[211]*v4037)/v1377))))}else{(if v1360{(v3966+(if v1384{v2}else{(if v1364{(v1368*((v1379*v3908)+(v1362*((self.scalar_static_f64[211]*v3908)/v1377))))}else{v2})}))}else{v2})})}))+(v1351*(if v1510{v2}else{(if v1475{((if v1475{((v1435*((-(v4175/v589))*v4186))/self.scalar_static_f64[217])}else{v4132})+((v1505*(self.scalar_static_f64[216]*v4203))+(v1502*((self.scalar_static_f64[218]*v4203)/v1456))))}else{(if v1440{(v4132+(if v1463{v2}else{(if v1444{(v1447*((v1458*v4074)+(v1442*((self.scalar_static_f64[218]*v4074)/v1456))))}else{v2})}))}else{v2})})})))}else{v2})})+self.scalar_static_f64[231]));
        let v4713=(self.scalar_static_f64[166]*((if self.scalar_static_bool[70]{v2}else{(if self.scalar_static_bool[67]{((v1530*(if v1598{v2}else{(if v1565{((if v1565{((v1354*((-(v4397/v551))*v4413))/self.scalar_static_f64[210])}else{v4343})+((v1593*(self.scalar_static_f64[209]*v4432))+(v1590*((self.scalar_static_f64[211]*v4432)/v1377))))}else{(if v1536{(v4343+(if v1554{v2}else{(if v1540{(v1541*((v1549*v4275)+(v1538*((self.scalar_static_f64[211]*v4275)/v1377))))}else{v2})}))}else{v2})})}))+(v1532*(if v1665{v2}else{(if v1632{((if v1632{((v1435*((-(v4599/v589))*v4615))/self.scalar_static_f64[217])}else{v4545})+((v1660*(self.scalar_static_f64[216]*v4634))+(v1657*((self.scalar_static_f64[218]*v4634)/v1456))))}else{(if v1603{(v4545+(if v1621{v2}else{(if v1607{(v1608*((v1616*v4477)+(v1605*((self.scalar_static_f64[218]*v4477)/v1456))))}else{v2})}))}else{v2})})})))}else{v2})})+self.scalar_static_f64[232]));
        let v4714=(self.scalar_static_f64[166]*(if self.scalar_static_bool[70]{v2}else{(if self.scalar_static_bool[67]{v4691}else{v2})}));
        let v4715=(self.scalar_static_f64[166]*(if self.scalar_static_bool[70]{v2}else{(if self.scalar_static_bool[67]{((v1530*(if v1598{v2}else{(if v1565{((if v1565{((v1354*((-(v4399/v551))*v4413))/self.scalar_static_f64[210])}else{v4345})+((v1593*(self.scalar_static_f64[209]*v4434))+(v1590*((self.scalar_static_f64[211]*v4434)/v1377))))}else{(if v1536{(v4345+(if v1554{v2}else{(if v1540{(v1541*((v1549*v4277)+(v1538*((self.scalar_static_f64[211]*v4277)/v1377))))}else{v2})}))}else{v2})})}))+(v1532*(if v1665{v2}else{(if v1632{((if v1632{((v1435*((-(v4601/v589))*v4615))/self.scalar_static_f64[217])}else{v4547})+((v1660*(self.scalar_static_f64[216]*v4636))+(v1657*((self.scalar_static_f64[218]*v4636)/v1456))))}else{(if v1603{(v4547+(if v1621{v2}else{(if v1607{(v1608*((v1616*v4479)+(v1605*((self.scalar_static_f64[218]*v4479)/v1456))))}else{v2})}))}else{v2})})})))}else{v2})}));
        let v4716=(self.scalar_static_f64[166]*((if self.scalar_static_bool[70]{v2}else{(if self.scalar_static_bool[67]{((v1530*(if v1598{v2}else{(if v1565{((if v1565{((v1354*((-(v4400/v551))*v4413))/self.scalar_static_f64[210])}else{v4346})+((v1593*(self.scalar_static_f64[209]*v4435))+(v1590*((self.scalar_static_f64[211]*v4435)/v1377))))}else{(if v1536{(v4346+(if v1554{v2}else{(if v1540{(v1541*((v1549*v4278)+(v1538*((self.scalar_static_f64[211]*v4278)/v1377))))}else{v2})}))}else{v2})})}))+(v1532*(if v1665{v2}else{(if v1632{((if v1632{((v1435*((-(v4602/v589))*v4615))/self.scalar_static_f64[217])}else{v4548})+((v1660*(self.scalar_static_f64[216]*v4637))+(v1657*((self.scalar_static_f64[218]*v4637)/v1456))))}else{(if v1603{(v4548+(if v1621{v2}else{(if v1607{(v1608*((v1616*v4480)+(v1605*((self.scalar_static_f64[218]*v4480)/v1456))))}else{v2})}))}else{v2})})})))}else{v2})})+self.scalar_static_f64[233]));

        CommonStampValues {
            v0,
            v1,
            v2,
            v56,
            v65,
            v105,
            v150,
            v158,
            v165,
            v220,
            v222,
            v223,
            v224,
            v243,
            v248,
            v250,
            v251,
            v297,
            v388,
            v391,
            v392,
            v397,
            v399,
            v416,
            v418,
            v419,
            v467,
            v514,
            v524,
            v1677,
            v1678,
            v1679,
            v1705,
            v1707,
            v1708,
            v1744,
            v1746,
            v4710,
            v4711,
            v4712,
            v4713,
            v4714,
            v4715,
            v4716,
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
        let v129=((self.scalar_static_f64[60]*common.v105)).sqrt();
        let v173=(if self.scalar_static_bool[21]{(self.scalar_static_f64[81]/v129)}else{common.v158});
        let v186=(common.v165+self.scalar_static_f64[87]);
        let v256=(self.scalar_static_f64[104]/common.v248);
        let v272=(if self.scalar_static_bool[30]{common.v251}else{(if self.scalar_static_bool[28]{(common.v251-(v256).sqrt())}else{(if self.scalar_static_bool[25]{(common.v251-v256)}else{common.v2})})});
        let v278=(self.scalar_static_f64[105]/(common.v0+(self.scalar_static_f64[106]/common.v165)));
        let v299=(v278*(v278*common.v297));
        let v302=(if self.scalar_static_bool[30]{v299}else{(if self.scalar_static_bool[28]{v299}else{(if self.scalar_static_bool[25]{(if self.scalar_static_bool[31]{(self.scalar_static_f64[257]*(common.v0+(((-v278)/self.scalar_static_f64[248])).exp()))}else{self.scalar_static_f64[258]})}else{common.v2})})});
        let v308=(common.v0-(common.v223*common.v243));
        let v309=((((if self.scalar_static_bool[23]{common.v0}else{(if self.scalar_static_bool[21]{((self.scalar_static_f64[85]*((self.scalar_static_f64[86]+(v173*v173))).sqrt())).exp()}else{(if (self.scalar_static_f64[62]!=0.0){((v3*(self.scalar_static_f64[79]+(self.scalar_static_f64[82]/v129)))).exp()}else{common.v2})})})*self.scalar_static_f64[109])*(common.v165/common.v150))*v308);
        let v373=((self.scalar_static_f64[147]+(self.scalar_static_f64[148]/common.v150))+((self.scalar_static_f64[32]*(self.scalar_static_f64[149]+(self.scalar_static_f64[150]/common.v150)))/common.v165));
        let v394=(self.scalar_static_f64[166]*(common.v391-common.v392));
        let v420=(common.v416-self.scalar_static_f64[8]);
        let v422=(((self.scalar_static_f64[143]+(self.scalar_static_f64[144]/common.v150))+((self.scalar_static_f64[32]*(self.scalar_static_f64[145]+(self.scalar_static_f64[146]/common.v150)))/common.v165))+(v373*v420));
        let v424=(common.v0+(v420*v422));
        let v425=0.11;
        let v426=(v424<v425);
        let v427=10.0;
        let v431=(((v427*(v424-v3))-common.v0)).exp();
        let v434=(if v426{(v3+(common.v220*v431))}else{v424});
        let v435=(v308*v309);
        let v436=(v434*v435);
        let v440=(v309*v434);
        let v442=(if self.scalar_static_bool[38]{(common.v0/v440)}else{(if (self.scalar_static_f64[102]!=0.0){(common.v0/v436)}else{common.v2})});
        let v446=(self.scalar_static_f64[167]+(v420*self.scalar_static_f64[168]));
        let v448=(common.v0+(v420*v446));
        let v449=(v448<v425);
        let v453=(((v427*(v448-v3))-common.v0)).exp();
        let v456=(if v449{(v3+(common.v220*v453))}else{v448});
        let v458=f64::powf(common.v419,self.scalar_static_f64[169]);
        let v464=(self.scalar_static_f64[172]*(common.v0-common.v419));
        let v469=((v464/common.v418)+(self.scalar_static_f64[173]*common.v467));
        let v472=((v469/self.scalar_static_f64[174])).exp();
        let v474=(if self.scalar_static_bool[39]{(self.scalar_static_f64[170]*v472)}else{common.v2});
        let v475=(common.v418*self.scalar_static_f64[174]);
        let v478=(common.v0+(self.scalar_static_f64[175]/v474));
        let v479=(v478).ln();
        let v484=(if self.scalar_static_bool[40]{common.v2}else{(if self.scalar_static_bool[39]{(v475*v479)}else{common.v2})});
        let v489=((v469/self.scalar_static_f64[177])).exp();
        let v491=(if self.scalar_static_bool[41]{(self.scalar_static_f64[176]*v489)}else{common.v2});
        let v492=(common.v418*self.scalar_static_f64[177]);
        let v494=(common.v0+(self.scalar_static_f64[175]/v491));
        let v495=(v494).ln();
        let v499=(if self.scalar_static_bool[42]{common.v2}else{v491});
        let v500=(if self.scalar_static_bool[42]{common.v2}else{(if self.scalar_static_bool[41]{(v492*v495)}else{common.v2})});
        let v501=(self.scalar_static_f64[20]*(if self.scalar_static_bool[40]{common.v2}else{v474}));
        let v502=(self.scalar_static_f64[22]*v499);
        let v504=(self.scalar_static_f64[24]*v499);
        let v596=(self.scalar_static_f64[187]+(v420*self.scalar_static_f64[188]));
        let v600=(if self.scalar_static_bool[47]{(self.scalar_static_f64[186]*(common.v0+(v420*v596)))}else{common.v2});
        let v601=(v600>common.v2);
        let v603=(if self.scalar_static_bool[47]{(if v601{v600}else{common.v2})}else{v600});
        let v609=(if self.scalar_static_bool[47]{(self.scalar_static_f64[189]*(common.v0+(v420*self.scalar_static_f64[190])))}else{common.v2});
        let v610=(common.v418*v609);
        let v611=(-v603);
        let v613=((v611/v610)).exp();
        let v616=(v613+self.scalar_static_f64[192]);
        let v617=(v616).ln();
        let v621=(if self.scalar_static_bool[48]{self.scalar_static_f64[186]}else{v603});
        let v622=(if self.scalar_static_bool[48]{self.scalar_static_f64[189]}else{v609});
        let v623=(if self.scalar_static_bool[48]{common.v0}else{(if self.scalar_static_bool[47]{(v610*v617)}else{common.v2})});
        let v630=(v458*self.scalar_static_f64[195]);
        let v633=(v458*self.scalar_static_f64[193]);
        let v638=(if self.scalar_static_bool[53]{self.scalar_static_f64[195]}else{(if self.scalar_static_bool[51]{(v434*v630)}else{common.v2})});
        let v639=(if self.scalar_static_bool[53]{self.scalar_static_f64[193]}else{(if self.scalar_static_bool[51]{(v434*v633)}else{common.v2})});
        let v644=(v639*self.scalar_static_f64[198]);
        let v647=(((v638*v638)+(v639*v644))).sqrt();
        let v651=(if self.scalar_static_bool[50]{(v647-(v639*self.scalar_static_f64[199]))}else{common.v2});
        let v652=(self.scalar_static_f64[196]*v651);
        let v654=(if self.scalar_static_bool[50]{(v652/v639)}else{common.v2});
        let v655=(v651*v651);
        let v656=(v639*v639);
        let v660=(((v655/v656)+(common.v297*v654))).sqrt();
        let v667=(if self.scalar_static_bool[54]{common.v2}else{v651});
        let v668=(if self.scalar_static_bool[54]{common.v2}else{v654});
        let v669=(if self.scalar_static_bool[54]{common.v2}else{(if self.scalar_static_bool[50]{v660}else{common.v2})});
        let v672=(if self.scalar_static_bool[54]{common.v2}else{(if self.scalar_static_bool[50]{(common.v0/v639)}else{common.v2})});
        let v673=(v186*(if self.scalar_static_bool[54]{1000.0}else{(if self.scalar_static_bool[50]{(v639-v638)}else{common.v2})}));
        let v674=100000.0;
        let v675=(v673>v674);
        let v676=(if v675{v674}else{v673});
        let v677=(v394<common.v2);
        let v678=-1.0;
        let v684=(!v677);
        let v685=(if v684{common.v0}else{(if v677{v678}else{common.v2})});
        let v687=(if v684{(-common.v397)}else{(if v677{(-common.v399)}else{common.v2})});
        let v688=(if v684{v394}else{(if v677{(-v394)}else{common.v2})});
        let v689=(v687>v272);
        let v692=(((v272-v687)/self.scalar_static_f64[261])).exp();
        let v693=(common.v0+v692);
        let v698=(!v689);
        let v701=(((v687-v272)/self.scalar_static_f64[261])).exp();
        let v702=(common.v0+v701);
        let v706=(if v698{(v687-(self.scalar_static_f64[261]*(v702).ln()))}else{(if v689{(v272-(self.scalar_static_f64[261]*(v693).ln()))}else{common.v2})});
        let v707=-0.4;
        let v708=(v272-v706);
        let v709=(v688<v708);
        let v712=(v707*(common.v222+(if v709{v688}else{v708})));
        let v713=(v706<v712);
        let v714=((self.scalar_static_f64[102]!=0.0)&&v713);
        let v717=((self.scalar_static_f64[102]!=0.0)&&(!v713));
        let v719=(common.v222*v707);
        let v720=(v706<v719);
        let v721=(self.scalar_static_bool[38]&&v720);
        let v724=(self.scalar_static_bool[38]&&(!v720));
        let v725=(if v724{v706}else{(if v721{v719}else{(if v717{v706}else{(if v714{v712}else{common.v2})})})});
        let v727=(common.v222+(common.v56*v725));
        let v728=(v672>common.v2);
        let v729=(common.v248*v727);
        let v732=(if v728{((v727*v729)-v727)}else{common.v2});
        let v733=(common.v248*common.v524);
        let v734=(v727*v733);
        let v736=(if v728{(v678+v734)}else{common.v2});
        let v737=9.0;
        let v742=(if v728{(common.v248*(2.25+(v727/v676)))}else{common.v2});
        let v743=1.5;
        let v744=(common.v248*v743);
        let v746=(if v728{(v744/v676)}else{common.v2});
        let v747=(common.v297*v676);
        let v750=(if v728{((v676*v747)/common.v248)}else{common.v2});
        let v752=(if v728{(v732*v750)}else{common.v2});
        let v754=(if v728{(v736*v750)}else{common.v2});
        let v756=(if v728{(v742*v750)}else{common.v2});
        let v758=(if v728{(v746*v750)}else{common.v2});
        let v760=(if v728{(v758*v758)}else{common.v2});
        let v762=(if v728{(-v756)}else{common.v2});
        let v766=(if v728{((v754*v758)-(common.v297*v752))}else{common.v2});
        let v767=(common.v297*v756);
        let v773=(if v728{(((v752*v767)-(v754*v754))-(v752*v760))}else{common.v2});
        let v775=0.3333333333333333;
        let v778=(if v728{(v766-((v762*v762)*v775))}else{common.v2});
        let v780=(v766+(common.v56*v778));
        let v784=(if v728{(v773-((v762*v780)/v737))}else{common.v2});
        let v785=(v778*v778);
        let v787=27.0;
        let v789=(if v728{((v778*v785)/v787)}else{common.v2});
        let v790=0.25;
        let v791=(v784*v790);
        let v795=((if v728{(v789+(v784*v791))}else{common.v2})).sqrt();
        let v796=(if v728{v795}else{common.v2});
        let v797=(v784<common.v2);
        let v798=(v728&&v797);
        let v799=(common.v514*v784);
        let v801=(if v798{(v796+v799)}else{common.v2});
        let v802=(-v789);
        let v806=(v728&&(!v797));
        let v808=(if v806{(v799-v796)}else{(if v798{(v802/v801)}else{common.v2})});
        let v810=(if v806{(v802/v808)}else{v801});
        let v811=1e-6;
        let v812=(v810>v811);
        let v813=(v728&&v812);
        let v816=-1e-6;
        let v817=(v810<v816);
        let v819=(v728&&(!v812));
        let v820=(v817&&v819);
        let v821=(-v810);
        let v826=(v819&&(!v817));
        let v829=(v808>v811);
        let v830=(v728&&v829);
        let v833=(v808<v816);
        let v835=(v728&&(!v829));
        let v836=(v833&&v835);
        let v837=(-v808);
        let v842=(v835&&(!v833));
        let v849=(v760*v790);
        let v852=(((if v728{(((if v826{(common.v224*v810)}else{(if v820{(-f64::powf(v821,v775))}else{(if v813{f64::powf(v810,v775)}else{common.v2})})})+(if v842{(common.v224*v808)}else{(if v836{(-f64::powf(v837,v775))}else{(if v830{f64::powf(v808,v775)}else{common.v2})})}))-(v762*v775))}else{common.v2})+(v849-v756))).sqrt();
        let v853=(if v728{v852}else{v773});
        let v854=0.75;
        let v856=(v853*v853);
        let v860=(if v728{(((v760*v854)-v856)-(common.v56*v756))}else{common.v2});
        let v865=(((v756*v758)-(common.v56*v754))-(v758*v849));
        let v867=(if v728{(v865/v853)}else{common.v2});
        let v869=(if v728{(v860+v867)}else{common.v2});
        let v870=(v869>common.v2);
        let v871=(v728&&v870);
        let v872=(v869).sqrt();
        let v873=(if v871{v872}else{common.v2});
        let v874=-0.25;
        let v875=(v758*v874);
        let v881=(v728&&(!v870));
        let v883=(if v881{(v860-v867)}else{common.v2});
        let v887=(((v883*v883)+0.0001)).sqrt();
        let v888=(v887).sqrt();
        let v894=(v725>(if self.scalar_static_bool[30]{common.v2}else{(if self.scalar_static_bool[28]{common.v2}else{(if self.scalar_static_bool[25]{((0.1666666666666667/common.v248)-common.v250)}else{common.v2})})}));
        let v895=(!v728);
        let v896=(v894&&v895);
        let v897=(common.v251-v725);
        let v899=(if v896{(common.v248*v897)}else{common.v2});
        let v902=(common.v56*(common.v0-(common.v56*v899)));
        let v903=(v897*v902);
        let v908=((common.v0-(v743*v899))).sqrt();
        let v909=((common.v0-(common.v524*v899))+v908);
        let v913=(v895&&(!v894));
        let v914=(if v913{v734}else{v899});
        let v917=((common.v0+v914)).sqrt();
        let v920=(common.v248*4.5);
        let v922=(if v913{(((common.v0-v914)+v917)/v920)}else{(if v896{(v903/v909)}else{(if v881{(v875+(common.v65*((if v881{v888}else{v873})-v853)))}else{(if v871{(v875+(common.v65*(v853+v873)))}else{common.v2})})})});
        let v925=(self.scalar_static_bool[25]&&(common.v243>1e-9));
        let v927=(if v925{(self.scalar_static_f64[248]+v922)}else{common.v2});
        let v928=(v727+v922);
        let v929=(v928).sqrt();
        let v931=(if v925{(common.v243*v929)}else{common.v2});
        let v932=(v728&&v925);
        let v933=(v927/v186);
        let v935=(common.v65*(v933-v667));
        let v937=(if v932{(v672*v935)}else{common.v2});
        let v939=(common.v65*(v667+v933));
        let v941=(if v932{(v672*v939)}else{common.v2});
        let v944=((v668+(v937*v937))).sqrt();
        let v945=(if v932{v944}else{common.v2});
        let v948=((v668+(v941*v941))).sqrt();
        let v949=(if v932{v948}else{common.v2});
        let v952=(if v932{((v945+v949)-v669)}else{common.v2});
        let v956=(common.v65*((v937/v945)+(v941/v949)));
        let v959=(if v932{((v672*v956)/v186)}else{common.v2});
        let v960=(common.v56*v931);
        let v961=(common.v0-v931);
        let v962=(v960*v961);
        let v963=(v927*v959);
        let v964=(common.v0+v952);
        let v966=(common.v0-(v963/v964));
        let v967=(v962*v966);
        let v969=((v967/v927)).sqrt();
        let v971=(v895&&v925);
        let v973=((v962/v927)).sqrt();
        let v974=(if v971{v973}else{(if v932{v969}else{common.v2})});
        let v975=(common.v248*v928);
        let v976=(v974*v974);
        let v980=(self.scalar_static_f64[105]*v922);
        let v981=(self.scalar_static_f64[105]+v927);
        let v984=(if v925{(v302+(v980/v981))}else{common.v2});
        let v985=(common.v297*v984);
        let v987=(if v925{(v984*v985)}else{common.v2});
        let v988=(common.v56*v688);
        let v989=(v927*v988);
        let v990=(v688-v927);
        let v991=(v990*v990);
        let v993=((v987+v991)).sqrt();
        let v994=(v688+v927);
        let v995=(v994*v994);
        let v997=((v987+v995)).sqrt();
        let v998=(v993+v997);
        let v1000=(if v925{(v989/v998)}else{common.v2});
        let v1001=(self.scalar_static_bool[31]&&v925);
        let v1002=(self.scalar_static_f64[105]*v1000);
        let v1005=(if v1001{(v302+(v1002/v981))}else{v984});
        let v1006=(common.v297*v1005);
        let v1008=(if v1001{(v1005*v1006)}else{v987});
        let v1010=((v991+v1008)).sqrt();
        let v1012=((v995+v1008)).sqrt();
        let v1013=(v1010+v1012);
        let v1015=(if v1001{(v989/v1013)}else{v1000});
        let v1017=(((if v925{((v975/v976)-v927)}else{common.v2})+v1015)).sqrt();
        let v1021=(v1015/v186);
        let v1023=(common.v65*(v1021-v667));
        let v1025=(if v932{(v672*v1023)}else{v937});
        let v1027=(common.v65*(v667+v1021));
        let v1029=(if v932{(v672*v1027)}else{v941});
        let v1032=((v668+(v1025*v1025))).sqrt();
        let v1033=(if v932{v1032}else{v945});
        let v1036=((v668+(v1029*v1029))).sqrt();
        let v1037=(if v932{v1036}else{v949});
        let v1042=(!v925);
        let v1043=(v922*v988);
        let v1044=(v688-v922);
        let v1047=((v302+(v1044*v1044))).sqrt();
        let v1048=(v688+v922);
        let v1051=((v302+(v1048*v1048))).sqrt();
        let v1052=(v1047+v1051);
        let v1054=(if v1042{(v1043/v1052)}else{v1015});
        let v1055=(v728&&v1042);
        let v1056=(v1054/v186);
        let v1058=(common.v65*(v1056-v667));
        let v1060=(if v1055{(v672*v1058)}else{v1025});
        let v1062=(common.v65*(v667+v1056));
        let v1064=(if v1055{(v672*v1062)}else{v1029});
        let v1067=((v668+(v1060*v1060))).sqrt();
        let v1071=((v668+(v1064*v1064))).sqrt();
        let v1076=(v895&&v1042);
        let v1079=((v727+v1054)).sqrt();
        let v1082=(if v1042{(common.v0-(common.v243*v1079))}else{(if v925{(common.v0-(v974*v1017))}else{common.v2})});
        let v1083=(v1082<self.scalar_static_f64[103]);
        let v1084=(if v1083{self.scalar_static_f64[103]}else{v1082});
        let v1085=(v442*v1084);
        let v1086=(common.v0+(if v1076{common.v2}else{(if v1055{(((if v1055{v1067}else{v1033})+(if v1055{v1071}else{v1037}))-v669)}else{(if v971{common.v2}else{(if v932{((v1033+v1037)-v669)}else{v952})})})}));
        let v1088=(v685*(v1085/v1086));
        let v1089=(v1054*v1088);
        let v1090=((v501+v502)>common.v2);
        let v1091=(if v1090{v501}else{common.v2});
        let v1092=(if v1090{v502}else{common.v2});
        let v1093=(v1091>common.v2);
        let v1094=(v1090&&v1093);
        let v1095=(common.v0/v475);
        let v1096=(if v1094{v1095}else{common.v2});
        let v1097=(common.v397<v484);
        let v1098=(v1094&&v1097);
        let v1100=((common.v397*v1096)).exp();
        let v1103=(v1094&&(!v1097));
        let v1105=((v484*v1096)).exp();
        let v1106=(common.v397-v484);
        let v1108=(common.v0+(v1096*v1106));
        let v1110=(if v1103{(v1105*v1108)}else{(if v1098{v1100}else{common.v2})});
        let v1111=(v1110-common.v0);
        let v1115=(v1090&&(!v1093));
        let v1117=(v1092>common.v2);
        let v1118=(v1090&&v1117);
        let v1119=(common.v0/v492);
        let v1120=(if v1118{v1119}else{v1096});
        let v1121=(common.v397<v500);
        let v1122=(v1118&&v1121);
        let v1124=((common.v397*v1120)).exp();
        let v1127=(v1118&&(!v1121));
        let v1129=((v500*v1120)).exp();
        let v1130=(common.v397-v500);
        let v1132=(common.v0+(v1120*v1130));
        let v1135=((if v1127{(v1129*v1132)}else{(if v1122{v1124}else{v1110})})-common.v0);
        let v1139=(v1090&&(!v1117));
        let v1143=(v621>common.v2);
        let v1144=(v1090&&v1143);
        let v1145=(-v621);
        let v1147=(if v1144{(v1145-common.v397)}else{common.v2});
        let v1148=(common.v418*v622);
        let v1149=(common.v0/v1148);
        let v1150=(if v1144{v1149}else{common.v2});
        let v1151=(v1147<v623);
        let v1152=(v1144&&v1151);
        let v1154=((v1147*v1150)).exp();
        let v1157=(v1144&&(!v1151));
        let v1159=((v623*v1150)).exp();
        let v1160=(v1147-v623);
        let v1162=(common.v0+(v1150*v1160));
        let v1167=((v1145*v1150)).exp();
        let v1171=(!v1143);
        let v1172=(v1090&&v1171);
        let v1178=(!v1090);
        let v1179=(if v1178{common.v2}else{(if v1090{(((if v1090{((if v1115{common.v2}else{(if v1094{(v1091*v1111)}else{common.v2})})+(if v1139{common.v2}else{(if v1118{(v1092*v1135)}else{common.v2})}))}else{common.v2})+(if v1172{common.v2}else{(if v1144{(self.scalar_static_f64[200]*((if v1157{(v1159*v1162)}else{(if v1152{v1154}else{common.v2})})-v1167))}else{common.v2})}))+(common.v2*common.v397))}else{common.v2})});
        let v1180=((v501+v504)>common.v2);
        let v1181=(if v1180{v501}else{common.v2});
        let v1182=(if v1180{v504}else{common.v2});
        let v1183=(v1181>common.v2);
        let v1184=(v1180&&v1183);
        let v1185=(if v1184{v1095}else{common.v2});
        let v1186=(common.v399<v484);
        let v1187=(v1184&&v1186);
        let v1189=((common.v399*v1185)).exp();
        let v1192=(v1184&&(!v1186));
        let v1194=((v484*v1185)).exp();
        let v1195=(common.v399-v484);
        let v1197=(common.v0+(v1185*v1195));
        let v1199=(if v1192{(v1194*v1197)}else{(if v1187{v1189}else{common.v2})});
        let v1200=(v1199-common.v0);
        let v1204=(v1180&&(!v1183));
        let v1206=(v1182>common.v2);
        let v1207=(v1180&&v1206);
        let v1208=(if v1207{v1119}else{v1185});
        let v1209=(common.v399<v500);
        let v1210=(v1207&&v1209);
        let v1212=((common.v399*v1208)).exp();
        let v1215=(v1207&&(!v1209));
        let v1217=((v500*v1208)).exp();
        let v1218=(common.v399-v500);
        let v1220=(common.v0+(v1208*v1218));
        let v1223=((if v1215{(v1217*v1220)}else{(if v1210{v1212}else{v1199})})-common.v0);
        let v1227=(v1180&&(!v1206));
        let v1231=(v1143&&v1180);
        let v1233=(if v1231{(v1145-common.v399)}else{common.v2});
        let v1234=(if v1231{v1149}else{common.v2});
        let v1235=(v1233<v623);
        let v1236=(v1231&&v1235);
        let v1238=((v1233*v1234)).exp();
        let v1241=(v1231&&(!v1235));
        let v1243=((v623*v1234)).exp();
        let v1244=(v1233-v623);
        let v1246=(common.v0+(v1234*v1244));
        let v1250=((v1145*v1234)).exp();
        let v1254=(v1171&&v1180);
        let v1260=(!v1180);
        let v1261=(if v1260{common.v2}else{(if v1180{(((if v1180{((if v1204{common.v2}else{(if v1184{(v1181*v1200)}else{common.v2})})+(if v1227{common.v2}else{(if v1207{(v1182*v1223)}else{common.v2})}))}else{common.v2})+(if v1254{common.v2}else{(if v1231{(self.scalar_static_f64[200]*((if v1241{(v1243*v1246)}else{(if v1236{v1238}else{common.v2})})-v1250))}else{common.v2})}))+(common.v2*common.v399))}else{common.v2})});
        let v1267=ctx.branch_current(branches[0]);
        let v1269=(ctx.node_voltage(nodes[0])-common.v392);
        let v1272=ctx.branch_current(branches[1]);
        let v1274=(ctx.node_voltage(nodes[2])-common.v391);
        let v1285=(self.scalar_static_f64[264]*common.v388);
        let v1314=(common.v0+(common.v388/self.scalar_static_f64[277]));
        let v1325=(common.v0+((common.v388*self.scalar_static_f64[204])/self.scalar_static_f64[277]));
        let v1332=(self.scalar_static_f64[166]*v1089);
        let v1333=(self.scalar_static_f64[166]*v1179);
        let v1334=(self.scalar_static_f64[166]*v1261);
        let v1682=((self.scalar_static_f64[116]/common.v1)<=self.scalar_static_f64[221]);
        let v1684=((self.scalar_static_f64[119]/common.v1)<=self.scalar_static_f64[221]);
        let v1685=(self.scalar_static_f64[116]*v456);
        let v1687=(self.scalar_static_f64[119]*v456);
        let v1689=(self.scalar_static_f64[116]*v1267);
        let v1692=(!v1682);
        let v1695=(self.scalar_static_f64[119]*v1272);
        let v1698=(!v1684);
        let v1712=((v422*common.v1705)+(v420*(v373*common.v1705)));
        let v1716=(if v426{(common.v220*(v431*(v427*v1712)))}else{v1712});
        let v1730=((v446*common.v1705)+(v420*(self.scalar_static_f64[168]*common.v1705)));
        let v1734=(if v449{(common.v220*(v453*(v427*v1730)))}else{v1730});
        let v1738=(common.v1708*(self.scalar_static_f64[169]*f64::powf(common.v419,self.scalar_static_f64[222])));
        let v1748=((((common.v418*(self.scalar_static_f64[172]*(-common.v1708)))-(v464*common.v1707))/common.v1744)+(self.scalar_static_f64[173]*common.v1746));
        let v1752=(if self.scalar_static_bool[39]{(self.scalar_static_f64[170]*(v472*(v1748/self.scalar_static_f64[174])))}else{common.v2});
        let v1753=(self.scalar_static_f64[174]*common.v1707);
        let v1764=(if self.scalar_static_bool[40]{common.v2}else{(if self.scalar_static_bool[39]{((v479*v1753)+(v475*(((-(self.scalar_static_f64[175]*v1752))/(v474*v474))/v478)))}else{common.v2})});
        let v1768=(if self.scalar_static_bool[41]{(self.scalar_static_f64[176]*(v489*(v1748/self.scalar_static_f64[177])))}else{common.v2});
        let v1769=(self.scalar_static_f64[177]*common.v1707);
        let v1779=(if self.scalar_static_bool[42]{common.v2}else{v1768});
        let v1780=(if self.scalar_static_bool[42]{common.v2}else{(if self.scalar_static_bool[41]{((v495*v1769)+(v492*(((-(self.scalar_static_f64[175]*v1768))/(v491*v491))/v494)))}else{common.v2})});
        let v1781=(self.scalar_static_f64[20]*(if self.scalar_static_bool[40]{common.v2}else{v1752}));
        let v1905=(if self.scalar_static_bool[47]{(self.scalar_static_f64[186]*((v596*common.v1705)+(v420*(self.scalar_static_f64[188]*common.v1705))))}else{common.v2});
        let v1907=(if self.scalar_static_bool[47]{(if v601{v1905}else{common.v2})}else{v1905});
        let v1910=(if self.scalar_static_bool[47]{(self.scalar_static_f64[189]*(self.scalar_static_f64[190]*common.v1705))}else{common.v2});
        let v1913=((v609*common.v1707)+(common.v418*v1910));
        let v1928=(if self.scalar_static_bool[48]{common.v2}else{(if self.scalar_static_bool[47]{((v617*v1913)+(v610*((v613*(((v610*(-v1907))-(v611*v1913))/(v610*v610)))/v616)))}else{common.v2})});
        let v1939=(if self.scalar_static_bool[53]{common.v2}else{(if self.scalar_static_bool[51]{((v630*v1716)+(v434*(self.scalar_static_f64[195]*v1738)))}else{common.v2})});
        let v1940=(if self.scalar_static_bool[53]{common.v2}else{(if self.scalar_static_bool[51]{((v633*v1716)+(v434*(self.scalar_static_f64[193]*v1738)))}else{common.v2})});
        let v1941=(v638*v1939);
        let v1952=(if self.scalar_static_bool[50]{((((v1941+v1941)+((v644*v1940)+(v639*(self.scalar_static_f64[198]*v1940))))/(common.v56*v647))-(self.scalar_static_f64[199]*v1940))}else{common.v2});
        let v1958=(if self.scalar_static_bool[50]{(((v639*(self.scalar_static_f64[196]*v1952))-(v652*v1940))/v656)}else{common.v2});
        let v1959=(v651*v1952);
        let v1961=(v639*v1940);
        let v1978=(if self.scalar_static_bool[54]{common.v2}else{v1952});
        let v1979=(if self.scalar_static_bool[54]{common.v2}else{v1958});
        let v1980=(if self.scalar_static_bool[54]{common.v2}else{(if self.scalar_static_bool[50]{(((((v656*(v1959+v1959))-(v655*(v1961+v1961)))/(v656*v656))+(common.v297*v1958))/(common.v56*v660))}else{common.v2})});
        let v1982=(if self.scalar_static_bool[54]{common.v2}else{(if self.scalar_static_bool[50]{((-v1940)/v656)}else{common.v2})});
        let v1984=(if v675{common.v2}else{(v186*(if self.scalar_static_bool[54]{common.v2}else{(if self.scalar_static_bool[50]{(v1940-v1939)}else{common.v2})}))});
        let v1985=(if v677{self.scalar_static_f64[165]}else{common.v2});
        let v1986=(if v677{self.scalar_static_f64[166]}else{common.v2});
        let v1987=(if v684{self.scalar_static_f64[165]}else{v1985});
        let v1988=(if v684{self.scalar_static_f64[166]}else{common.v2});
        let v1989=(if v684{common.v2}else{v1986});
        let v1990=(if v684{self.scalar_static_f64[165]}else{v1986});
        let v1991=(if v684{self.scalar_static_f64[166]}else{v1985});
        let v2028=(if v698{(v1987-(self.scalar_static_f64[261]*((v701*(v1987/self.scalar_static_f64[261]))/v702)))}else{(if v689{(-(self.scalar_static_f64[261]*((v692*((-v1987)/self.scalar_static_f64[261]))/v693)))}else{common.v2})});
        let v2029=(if v698{(v1988-(self.scalar_static_f64[261]*((v701*(v1988/self.scalar_static_f64[261]))/v702)))}else{(if v689{(-(self.scalar_static_f64[261]*((v692*((-v1988)/self.scalar_static_f64[261]))/v693)))}else{common.v2})});
        let v2030=(if v698{(v1989-(self.scalar_static_f64[261]*((v701*(v1989/self.scalar_static_f64[261]))/v702)))}else{(if v689{(-(self.scalar_static_f64[261]*((v692*((-v1989)/self.scalar_static_f64[261]))/v693)))}else{common.v2})});
        let v2049=(if v724{v2028}else{(if v721{common.v2}else{(if v717{v2028}else{(if v714{(v707*(if v709{common.v2}else{(-v2028)}))}else{common.v2})})})});
        let v2050=(if v724{v2029}else{(if v721{common.v2}else{(if v717{v2029}else{(if v714{(v707*(if v709{v1990}else{(-v2029)}))}else{common.v2})})})});
        let v2051=(if v724{v2030}else{(if v721{common.v2}else{(if v717{v2030}else{(if v714{(v707*(if v709{v1991}else{(-v2030)}))}else{common.v2})})})});
        let v2052=(common.v56*v2049);
        let v2053=(common.v56*v2050);
        let v2054=(common.v56*v2051);
        let v2073=(v733*v2052);
        let v2074=(v733*v2053);
        let v2075=(v733*v2054);
        let v2082=(v676*v676);
        let v2103=(if v728{(((v747*v1984)+(v676*(common.v297*v1984)))/common.v248)}else{common.v2});
        let v2108=(if v728{(v750*(if v728{(((v729*v2052)+(v727*(common.v248*v2052)))-v2052)}else{common.v2}))}else{common.v2});
        let v2109=(if v728{(v732*v2103)}else{common.v2});
        let v2110=(if v728{(v750*(if v728{(((v729*v2053)+(v727*(common.v248*v2053)))-v2053)}else{common.v2}))}else{common.v2});
        let v2111=(if v728{(v750*(if v728{(((v729*v2054)+(v727*(common.v248*v2054)))-v2054)}else{common.v2}))}else{common.v2});
        let v2116=(if v728{(v750*(if v728{v2073}else{common.v2}))}else{common.v2});
        let v2117=(if v728{(v736*v2103)}else{common.v2});
        let v2118=(if v728{(v750*(if v728{v2074}else{common.v2}))}else{common.v2});
        let v2119=(if v728{(v750*(if v728{v2075}else{common.v2}))}else{common.v2});
        let v2126=(if v728{(v750*(if v728{(common.v248*(v2052/v676))}else{common.v2}))}else{common.v2});
        let v2127=(if v728{((v750*(if v728{(common.v248*((-(v727*v1984))/v2082))}else{common.v2}))+(v742*v2103))}else{common.v2});
        let v2128=(if v728{(v750*(if v728{(common.v248*(v2053/v676))}else{common.v2}))}else{common.v2});
        let v2129=(if v728{(v750*(if v728{(common.v248*(v2054/v676))}else{common.v2}))}else{common.v2});
        let v2133=(if v728{((v750*(if v728{((-(v744*v1984))/v2082)}else{common.v2}))+(v746*v2103))}else{common.v2});
        let v2134=(v758*v2133);
        let v2136=(if v728{(v2134+v2134)}else{common.v2});
        let v2137=(-v2126);
        let v2139=(-v2128);
        let v2140=(-v2129);
        let v2141=(if v728{v2137}else{common.v2});
        let v2142=(if v728{(-v2127)}else{common.v2});
        let v2143=(if v728{v2139}else{common.v2});
        let v2144=(if v728{v2140}else{common.v2});
        let v2159=(if v728{((v758*v2116)-(common.v297*v2108))}else{common.v2});
        let v2160=(if v728{(((v758*v2117)+(v754*v2133))-(common.v297*v2109))}else{common.v2});
        let v2161=(if v728{((v758*v2118)-(common.v297*v2110))}else{common.v2});
        let v2162=(if v728{((v758*v2119)-(common.v297*v2111))}else{common.v2});
        let v2179=(v754*v2116);
        let v2181=(v754*v2117);
        let v2183=(v754*v2118);
        let v2185=(v754*v2119);
        let v2201=(if v728{((((v767*v2108)+(v752*(common.v297*v2126)))-(v2179+v2179))-(v760*v2108))}else{common.v2});
        let v2202=(if v728{((((v767*v2109)+(v752*(common.v297*v2127)))-(v2181+v2181))-((v760*v2109)+(v752*v2136)))}else{common.v2});
        let v2203=(if v728{((((v767*v2110)+(v752*(common.v297*v2128)))-(v2183+v2183))-(v760*v2110))}else{common.v2});
        let v2204=(if v728{((((v767*v2111)+(v752*(common.v297*v2129)))-(v2185+v2185))-(v760*v2111))}else{common.v2});
        let v2205=(v762*v2141);
        let v2207=(v762*v2142);
        let v2209=(v762*v2143);
        let v2211=(v762*v2144);
        let v2221=(if v728{(v2159-(v775*(v2205+v2205)))}else{common.v2});
        let v2222=(if v728{(v2160-(v775*(v2207+v2207)))}else{common.v2});
        let v2223=(if v728{(v2161-(v775*(v2209+v2209)))}else{common.v2});
        let v2224=(if v728{(v2162-(v775*(v2211+v2211)))}else{common.v2});
        let v2253=(if v728{(v2201-(((v780*v2141)+(v762*(v2159+(common.v56*v2221))))/v737))}else{common.v2});
        let v2254=(if v728{(v2202-(((v780*v2142)+(v762*(v2160+(common.v56*v2222))))/v737))}else{common.v2});
        let v2255=(if v728{(v2203-(((v780*v2143)+(v762*(v2161+(common.v56*v2223))))/v737))}else{common.v2});
        let v2256=(if v728{(v2204-(((v780*v2144)+(v762*(v2162+(common.v56*v2224))))/v737))}else{common.v2});
        let v2257=(v778*v2221);
        let v2259=(v778*v2222);
        let v2261=(v778*v2223);
        let v2263=(v778*v2224);
        let v2281=(if v728{(((v785*v2221)+(v778*(v2257+v2257)))/v787)}else{common.v2});
        let v2282=(if v728{(((v785*v2222)+(v778*(v2259+v2259)))/v787)}else{common.v2});
        let v2283=(if v728{(((v785*v2223)+(v778*(v2261+v2261)))/v787)}else{common.v2});
        let v2284=(if v728{(((v785*v2224)+(v778*(v2263+v2263)))/v787)}else{common.v2});
        let v2309=(common.v56*v795);
        let v2314=(if v728{((if v728{(v2281+((v791*v2253)+(v784*(v790*v2253))))}else{common.v2})/v2309)}else{common.v2});
        let v2315=(if v728{((if v728{(v2282+((v791*v2254)+(v784*(v790*v2254))))}else{common.v2})/v2309)}else{common.v2});
        let v2316=(if v728{((if v728{(v2283+((v791*v2255)+(v784*(v790*v2255))))}else{common.v2})/v2309)}else{common.v2});
        let v2317=(if v728{((if v728{(v2284+((v791*v2256)+(v784*(v790*v2256))))}else{common.v2})/v2309)}else{common.v2});
        let v2318=(common.v514*v2253);
        let v2319=(common.v514*v2254);
        let v2320=(common.v514*v2255);
        let v2321=(common.v514*v2256);
        let v2326=(if v798{(v2314+v2318)}else{common.v2});
        let v2327=(if v798{(v2315+v2319)}else{common.v2});
        let v2328=(if v798{(v2316+v2320)}else{common.v2});
        let v2329=(if v798{(v2317+v2321)}else{common.v2});
        let v2330=(-v2281);
        let v2331=(-v2282);
        let v2332=(-v2283);
        let v2333=(-v2284);
        let v2337=(v801*v801);
        let v2359=(if v806{(v2318-v2314)}else{(if v798{(((v801*v2330)-(v802*v2326))/v2337)}else{common.v2})});
        let v2360=(if v806{(v2319-v2315)}else{(if v798{(((v801*v2331)-(v802*v2327))/v2337)}else{common.v2})});
        let v2361=(if v806{(v2320-v2316)}else{(if v798{(((v801*v2332)-(v802*v2328))/v2337)}else{common.v2})});
        let v2362=(if v806{(v2321-v2317)}else{(if v798{(((v801*v2333)-(v802*v2329))/v2337)}else{common.v2})});
        let v2366=(v808*v808);
        let v2380=(if v806{(((v808*v2330)-(v802*v2359))/v2366)}else{v2326});
        let v2381=(if v806{(((v808*v2331)-(v802*v2360))/v2366)}else{v2327});
        let v2382=(if v806{(((v808*v2332)-(v802*v2361))/v2366)}else{v2328});
        let v2383=(if v806{(((v808*v2333)-(v802*v2362))/v2366)}else{v2329});
        let v2384=-0.6666666666666667;
        let v2386=(v775*f64::powf(v810,v2384));
        let v2400=(v775*f64::powf(v821,v2384));
        let v2422=(v775*f64::powf(v808,v2384));
        let v2436=(v775*f64::powf(v837,v2384));
        let v2473=(v790*v2136);
        let v2479=(common.v56*v852);
        let v2484=(if v728{((v2137+(if v728{(((if v826{(common.v224*v2380)}else{(if v820{(-((-v2380)*v2400))}else{(if v813{(v2380*v2386)}else{common.v2})})})+(if v842{(common.v224*v2359)}else{(if v836{(-((-v2359)*v2436))}else{(if v830{(v2359*v2422)}else{common.v2})})}))-(v775*v2141))}else{common.v2}))/v2479)}else{v2201});
        let v2485=(if v728{(((if v728{(((if v826{(common.v224*v2381)}else{(if v820{(-((-v2381)*v2400))}else{(if v813{(v2381*v2386)}else{common.v2})})})+(if v842{(common.v224*v2360)}else{(if v836{(-((-v2360)*v2436))}else{(if v830{(v2360*v2422)}else{common.v2})})}))-(v775*v2142))}else{common.v2})+(v2473-v2127))/v2479)}else{v2202});
        let v2486=(if v728{((v2139+(if v728{(((if v826{(common.v224*v2382)}else{(if v820{(-((-v2382)*v2400))}else{(if v813{(v2382*v2386)}else{common.v2})})})+(if v842{(common.v224*v2361)}else{(if v836{(-((-v2361)*v2436))}else{(if v830{(v2361*v2422)}else{common.v2})})}))-(v775*v2143))}else{common.v2}))/v2479)}else{v2203});
        let v2487=(if v728{((v2140+(if v728{(((if v826{(common.v224*v2383)}else{(if v820{(-((-v2383)*v2400))}else{(if v813{(v2383*v2386)}else{common.v2})})})+(if v842{(common.v224*v2362)}else{(if v836{(-((-v2362)*v2436))}else{(if v830{(v2362*v2422)}else{common.v2})})}))-(v775*v2144))}else{common.v2}))/v2479)}else{v2204});
        let v2489=(v853*v2484);
        let v2491=(v853*v2485);
        let v2493=(v853*v2486);
        let v2495=(v853*v2487);
        let v2509=(if v728{((-(v2489+v2489))-(common.v56*v2126))}else{common.v2});
        let v2510=(if v728{(((v854*v2136)-(v2491+v2491))-(common.v56*v2127))}else{common.v2});
        let v2511=(if v728{((-(v2493+v2493))-(common.v56*v2128))}else{common.v2});
        let v2512=(if v728{((-(v2495+v2495))-(common.v56*v2129))}else{common.v2});
        let v2547=(if v728{(((v853*((v758*v2126)-(common.v56*v2116)))-(v865*v2484))/v856)}else{common.v2});
        let v2548=(if v728{(((v853*((((v758*v2127)+(v756*v2133))-(common.v56*v2117))-((v849*v2133)+(v758*v2473))))-(v865*v2485))/v856)}else{common.v2});
        let v2549=(if v728{(((v853*((v758*v2128)-(common.v56*v2118)))-(v865*v2486))/v856)}else{common.v2});
        let v2550=(if v728{(((v853*((v758*v2129)-(common.v56*v2119)))-(v865*v2487))/v856)}else{common.v2});
        let v2559=(common.v56*v872);
        let v2564=(if v871{((if v728{(v2509+v2547)}else{common.v2})/v2559)}else{common.v2});
        let v2565=(if v871{((if v728{(v2510+v2548)}else{common.v2})/v2559)}else{common.v2});
        let v2566=(if v871{((if v728{(v2511+v2549)}else{common.v2})/v2559)}else{common.v2});
        let v2567=(if v871{((if v728{(v2512+v2550)}else{common.v2})/v2559)}else{common.v2});
        let v2568=(v874*v2133);
        let v2590=(v883*(if v881{(v2509-v2547)}else{common.v2}));
        let v2592=(v883*(if v881{(v2510-v2548)}else{common.v2}));
        let v2594=(v883*(if v881{(v2511-v2549)}else{common.v2}));
        let v2596=(v883*(if v881{(v2512-v2550)}else{common.v2}));
        let v2598=(common.v56*v887);
        let v2603=(common.v56*v888);
        let v2625=(-v2049);
        let v2626=(-v2050);
        let v2627=(-v2051);
        let v2631=(if v896{(common.v248*v2625)}else{common.v2});
        let v2632=(if v896{(common.v248*v2626)}else{common.v2});
        let v2633=(if v896{(common.v248*v2627)}else{common.v2});
        let v2664=(common.v56*v908);
        let v2674=(v909*v909);
        let v2688=(if v913{v2073}else{v2631});
        let v2689=(if v913{v2074}else{v2632});
        let v2690=(if v913{v2075}else{v2633});
        let v2694=(common.v56*v917);
        let v2704=(if v913{(((-v2688)+(v2688/v2694))/v920)}else{(if v896{(((v909*((v902*v2625)+(v897*(common.v56*(-(common.v56*v2631))))))-(v903*((-(common.v524*v2631))+((-(v743*v2631))/v2664))))/v2674)}else{(if v881{(common.v65*((if v881{(((v2590+v2590)/v2598)/v2603)}else{v2564})-v2484))}else{(if v871{(common.v65*(v2484+v2564))}else{common.v2})})})});
        let v2705=(if v913{common.v2}else{(if v896{common.v2}else{(if v881{(v2568+(common.v65*((if v881{(((v2592+v2592)/v2598)/v2603)}else{v2565})-v2485)))}else{(if v871{(v2568+(common.v65*(v2485+v2565)))}else{common.v2})})})});
        let v2706=(if v913{(((-v2689)+(v2689/v2694))/v920)}else{(if v896{(((v909*((v902*v2626)+(v897*(common.v56*(-(common.v56*v2632))))))-(v903*((-(common.v524*v2632))+((-(v743*v2632))/v2664))))/v2674)}else{(if v881{(common.v65*((if v881{(((v2594+v2594)/v2598)/v2603)}else{v2566})-v2486))}else{(if v871{(common.v65*(v2486+v2566))}else{common.v2})})})});
        let v2707=(if v913{(((-v2690)+(v2690/v2694))/v920)}else{(if v896{(((v909*((v902*v2627)+(v897*(common.v56*(-(common.v56*v2633))))))-(v903*((-(common.v524*v2633))+((-(v743*v2633))/v2664))))/v2674)}else{(if v881{(common.v65*((if v881{(((v2596+v2596)/v2598)/v2603)}else{v2567})-v2487))}else{(if v871{(common.v65*(v2487+v2567))}else{common.v2})})})});
        let v2708=(if v925{v2704}else{common.v2});
        let v2709=(if v925{v2705}else{common.v2});
        let v2710=(if v925{v2706}else{common.v2});
        let v2711=(if v925{v2707}else{common.v2});
        let v2712=(v2052+v2704);
        let v2713=(v2053+v2706);
        let v2714=(v2054+v2707);
        let v2715=(common.v56*v929);
        let v2724=(if v925{(common.v243*(v2712/v2715))}else{common.v2});
        let v2725=(if v925{(common.v243*(v2705/v2715))}else{common.v2});
        let v2726=(if v925{(common.v243*(v2713/v2715))}else{common.v2});
        let v2727=(if v925{(common.v243*(v2714/v2715))}else{common.v2});
        let v2729=(v2709/v186);
        let v2743=(if v932{(v672*(common.v65*(v2708/v186)))}else{common.v2});
        let v2744=(if v932{((v935*v1982)+(v672*(common.v65*(v2729-v1978))))}else{common.v2});
        let v2745=(if v932{(v672*(common.v65*(v2710/v186)))}else{common.v2});
        let v2746=(if v932{(v672*(common.v65*(v2711/v186)))}else{common.v2});
        let v2752=(if v932{((v939*v1982)+(v672*(common.v65*(v1978+v2729))))}else{common.v2});
        let v2753=(v937*v2743);
        let v2755=(v937*v2744);
        let v2757=(v937*v2745);
        let v2759=(v937*v2746);
        let v2762=(common.v56*v944);
        let v2767=(if v932{((v2753+v2753)/v2762)}else{common.v2});
        let v2768=(if v932{((v1979+(v2755+v2755))/v2762)}else{common.v2});
        let v2769=(if v932{((v2757+v2757)/v2762)}else{common.v2});
        let v2770=(if v932{((v2759+v2759)/v2762)}else{common.v2});
        let v2771=(v941*v2743);
        let v2773=(v941*v2752);
        let v2775=(v941*v2745);
        let v2777=(v941*v2746);
        let v2780=(common.v56*v948);
        let v2785=(if v932{((v2771+v2771)/v2780)}else{common.v2});
        let v2786=(if v932{((v1979+(v2773+v2773))/v2780)}else{common.v2});
        let v2787=(if v932{((v2775+v2775)/v2780)}else{common.v2});
        let v2788=(if v932{((v2777+v2777)/v2780)}else{common.v2});
        let v2794=(if v932{(v2767+v2785)}else{common.v2});
        let v2795=(if v932{((v2768+v2786)-v1980)}else{common.v2});
        let v2796=(if v932{(v2769+v2787)}else{common.v2});
        let v2797=(if v932{(v2770+v2788)}else{common.v2});
        let v2801=(v945*v945);
        let v2818=(v949*v949);
        let v2864=((v961*(common.v56*v2724))+(v960*(-v2724)));
        let v2867=((v961*(common.v56*v2725))+(v960*(-v2725)));
        let v2870=((v961*(common.v56*v2726))+(v960*(-v2726)));
        let v2873=((v961*(common.v56*v2727))+(v960*(-v2727)));
        let v2889=(v964*v964);
        let v2922=(v927*v927);
        let v2936=(common.v56*v969);
        let v2961=(common.v56*v973);
        let v2966=(if v971{((((v927*v2864)-(v962*v2708))/v2922)/v2961)}else{(if v932{((((v927*((v966*v2864)+(v962*(-(((v964*((v959*v2708)+(v927*(if v932{((v672*(common.v65*((((v945*v2743)-(v937*v2767))/v2801)+(((v949*v2743)-(v941*v2785))/v2818))))/v186)}else{common.v2}))))-(v963*v2794))/v2889)))))-(v967*v2708))/v2922)/v2936)}else{common.v2})});
        let v2967=(if v971{((((v927*v2867)-(v962*v2709))/v2922)/v2961)}else{(if v932{((((v927*((v966*v2867)+(v962*(-(((v964*((v959*v2709)+(v927*(if v932{(((v956*v1982)+(v672*(common.v65*((((v945*v2744)-(v937*v2768))/v2801)+(((v949*v2752)-(v941*v2786))/v2818)))))/v186)}else{common.v2}))))-(v963*v2795))/v2889)))))-(v967*v2709))/v2922)/v2936)}else{common.v2})});
        let v2968=(if v971{((((v927*v2870)-(v962*v2710))/v2922)/v2961)}else{(if v932{((((v927*((v966*v2870)+(v962*(-(((v964*((v959*v2710)+(v927*(if v932{((v672*(common.v65*((((v945*v2745)-(v937*v2769))/v2801)+(((v949*v2745)-(v941*v2787))/v2818))))/v186)}else{common.v2}))))-(v963*v2796))/v2889)))))-(v967*v2710))/v2922)/v2936)}else{common.v2})});
        let v2969=(if v971{((((v927*v2873)-(v962*v2711))/v2922)/v2961)}else{(if v932{((((v927*((v966*v2873)+(v962*(-(((v964*((v959*v2711)+(v927*(if v932{((v672*(common.v65*((((v945*v2746)-(v937*v2770))/v2801)+(((v949*v2746)-(v941*v2788))/v2818))))/v186)}else{common.v2}))))-(v963*v2797))/v2889)))))-(v967*v2711))/v2922)/v2936)}else{common.v2})});
        let v2974=(v974*v2966);
        let v2976=(v974*v2967);
        let v2978=(v974*v2968);
        let v2980=(v974*v2969);
        let v2985=(v976*v976);
        let v3014=(v981*v981);
        let v3028=(if v925{(((v981*(self.scalar_static_f64[105]*v2704))-(v980*v2708))/v3014)}else{common.v2});
        let v3029=(if v925{(((v981*(self.scalar_static_f64[105]*v2705))-(v980*v2709))/v3014)}else{common.v2});
        let v3030=(if v925{(((v981*(self.scalar_static_f64[105]*v2706))-(v980*v2710))/v3014)}else{common.v2});
        let v3031=(if v925{(((v981*(self.scalar_static_f64[105]*v2707))-(v980*v2711))/v3014)}else{common.v2});
        let v3048=(if v925{((v985*v3028)+(v984*(common.v297*v3028)))}else{common.v2});
        let v3049=(if v925{((v985*v3029)+(v984*(common.v297*v3029)))}else{common.v2});
        let v3050=(if v925{((v985*v3030)+(v984*(common.v297*v3030)))}else{common.v2});
        let v3051=(if v925{((v985*v3031)+(v984*(common.v297*v3031)))}else{common.v2});
        let v3052=(common.v56*v1990);
        let v3053=(common.v56*v1991);
        let v3054=(v988*v2708);
        let v3055=(v988*v2709);
        let v3058=((v988*v2710)+(v927*v3052));
        let v3061=((v988*v2711)+(v927*v3053));
        let v3066=(v990*(-v2708));
        let v3067=(v3066+v3066);
        let v3068=(v990*(-v2709));
        let v3069=(v3068+v3068);
        let v3070=(v990*(v1990-v2710));
        let v3071=(v3070+v3070);
        let v3072=(v990*(v1991-v2711));
        let v3073=(v3072+v3072);
        let v3078=(common.v56*v993);
        let v3085=(v994*v2708);
        let v3086=(v3085+v3085);
        let v3087=(v994*v2709);
        let v3088=(v3087+v3087);
        let v3089=(v994*(v1990+v2710));
        let v3090=(v3089+v3089);
        let v3091=(v994*(v1991+v2711));
        let v3092=(v3091+v3091);
        let v3097=(common.v56*v997);
        let v3109=(v998*v998);
        let v3123=(if v925{(((v998*v3054)-(v989*(((v3048+v3067)/v3078)+((v3048+v3086)/v3097))))/v3109)}else{common.v2});
        let v3124=(if v925{(((v998*v3055)-(v989*(((v3049+v3069)/v3078)+((v3049+v3088)/v3097))))/v3109)}else{common.v2});
        let v3125=(if v925{(((v998*v3058)-(v989*(((v3050+v3071)/v3078)+((v3050+v3090)/v3097))))/v3109)}else{common.v2});
        let v3126=(if v925{(((v998*v3061)-(v989*(((v3051+v3073)/v3078)+((v3051+v3092)/v3097))))/v3109)}else{common.v2});
        let v3147=(if v1001{(((v981*(self.scalar_static_f64[105]*v3123))-(v1002*v2708))/v3014)}else{v3028});
        let v3148=(if v1001{(((v981*(self.scalar_static_f64[105]*v3124))-(v1002*v2709))/v3014)}else{v3029});
        let v3149=(if v1001{(((v981*(self.scalar_static_f64[105]*v3125))-(v1002*v2710))/v3014)}else{v3030});
        let v3150=(if v1001{(((v981*(self.scalar_static_f64[105]*v3126))-(v1002*v2711))/v3014)}else{v3031});
        let v3167=(if v1001{((v1006*v3147)+(v1005*(common.v297*v3147)))}else{v3048});
        let v3168=(if v1001{((v1006*v3148)+(v1005*(common.v297*v3148)))}else{v3049});
        let v3169=(if v1001{((v1006*v3149)+(v1005*(common.v297*v3149)))}else{v3050});
        let v3170=(if v1001{((v1006*v3150)+(v1005*(common.v297*v3150)))}else{v3051});
        let v3175=(common.v56*v1010);
        let v3184=(common.v56*v1012);
        let v3196=(v1013*v1013);
        let v3210=(if v1001{(((v1013*v3054)-(v989*(((v3067+v3167)/v3175)+((v3086+v3167)/v3184))))/v3196)}else{v3123});
        let v3211=(if v1001{(((v1013*v3055)-(v989*(((v3069+v3168)/v3175)+((v3088+v3168)/v3184))))/v3196)}else{v3124});
        let v3212=(if v1001{(((v1013*v3058)-(v989*(((v3071+v3169)/v3175)+((v3090+v3169)/v3184))))/v3196)}else{v3125});
        let v3213=(if v1001{(((v1013*v3061)-(v989*(((v3073+v3170)/v3175)+((v3092+v3170)/v3184))))/v3196)}else{v3126});
        let v3218=(common.v56*v1017);
        let v3244=(v3211/v186);
        let v3258=(if v932{(v672*(common.v65*(v3210/v186)))}else{v2743});
        let v3259=(if v932{((v1023*v1982)+(v672*(common.v65*(v3244-v1978))))}else{v2744});
        let v3260=(if v932{(v672*(common.v65*(v3212/v186)))}else{v2745});
        let v3261=(if v932{(v672*(common.v65*(v3213/v186)))}else{v2746});
        let v3267=(if v932{((v1027*v1982)+(v672*(common.v65*(v1978+v3244))))}else{v2752});
        let v3268=(v1025*v3258);
        let v3270=(v1025*v3259);
        let v3272=(v1025*v3260);
        let v3274=(v1025*v3261);
        let v3277=(common.v56*v1032);
        let v3282=(if v932{((v3268+v3268)/v3277)}else{v2767});
        let v3283=(if v932{((v1979+(v3270+v3270))/v3277)}else{v2768});
        let v3284=(if v932{((v3272+v3272)/v3277)}else{v2769});
        let v3285=(if v932{((v3274+v3274)/v3277)}else{v2770});
        let v3286=(v1029*v3258);
        let v3288=(v1029*v3267);
        let v3290=(v1029*v3260);
        let v3292=(v1029*v3261);
        let v3295=(common.v56*v1036);
        let v3300=(if v932{((v3286+v3286)/v3295)}else{v2785});
        let v3301=(if v932{((v1979+(v3288+v3288))/v3295)}else{v2786});
        let v3302=(if v932{((v3290+v3290)/v3295)}else{v2787});
        let v3303=(if v932{((v3292+v3292)/v3295)}else{v2788});
        let v3329=(v1044*(-v2704));
        let v3331=(v1044*(-v2705));
        let v3333=(v1044*(v1990-v2706));
        let v3335=(v1044*(v1991-v2707));
        let v3337=(common.v56*v1047);
        let v3344=(v1048*v2704);
        let v3346=(v1048*v2705);
        let v3348=(v1048*(v1990+v2706));
        let v3350=(v1048*(v1991+v2707));
        let v3352=(common.v56*v1051);
        let v3364=(v1052*v1052);
        let v3378=(if v1042{(((v1052*(v988*v2704))-(v1043*(((v3329+v3329)/v3337)+((v3344+v3344)/v3352))))/v3364)}else{v3210});
        let v3379=(if v1042{(((v1052*(v988*v2705))-(v1043*(((v3331+v3331)/v3337)+((v3346+v3346)/v3352))))/v3364)}else{v3211});
        let v3380=(if v1042{(((v1052*((v988*v2706)+(v922*v3052)))-(v1043*(((v3333+v3333)/v3337)+((v3348+v3348)/v3352))))/v3364)}else{v3212});
        let v3381=(if v1042{(((v1052*((v988*v2707)+(v922*v3053)))-(v1043*(((v3335+v3335)/v3337)+((v3350+v3350)/v3352))))/v3364)}else{v3213});
        let v3383=(v3379/v186);
        let v3397=(if v1055{(v672*(common.v65*(v3378/v186)))}else{v3258});
        let v3399=(if v1055{(v672*(common.v65*(v3380/v186)))}else{v3260});
        let v3400=(if v1055{(v672*(common.v65*(v3381/v186)))}else{v3261});
        let v3407=(v1060*v3397);
        let v3409=(v1060*(if v1055{((v1058*v1982)+(v672*(common.v65*(v3383-v1978))))}else{v3259}));
        let v3411=(v1060*v3399);
        let v3413=(v1060*v3400);
        let v3416=(common.v56*v1067);
        let v3425=(v1064*v3397);
        let v3427=(v1064*(if v1055{((v1062*v1982)+(v672*(common.v65*(v1978+v3383))))}else{v3267}));
        let v3429=(v1064*v3399);
        let v3431=(v1064*v3400);
        let v3434=(common.v56*v1071);
        let v3459=(common.v56*v1079);
        let v3489=(v1086*v1086);
        let v3509=((v1088*v3378)+(v1054*(v685*(((v1086*(v442*(if v1083{common.v2}else{(if v1042{(-(common.v243*((v2052+v3378)/v3459)))}else{(if v925{(-((v1017*v2966)+(v974*(((if v925{((((v976*(common.v248*v2712))-(v975*(v2974+v2974)))/v2985)-v2708)}else{common.v2})+v3210)/v3218))))}else{common.v2})})})))-(v1085*(if v1076{common.v2}else{(if v1055{((if v1055{((v3407+v3407)/v3416)}else{v3282})+(if v1055{((v3425+v3425)/v3434)}else{v3300}))}else{(if v971{common.v2}else{(if v932{(v3282+v3300)}else{v2794})})})})))/v3489))));
        let v3512=((v1088*v3379)+(v1054*(v685*(((v1086*((v1084*(if self.scalar_static_bool[38]{((-(v309*v1716))/(v440*v440))}else{(if (self.scalar_static_f64[102]!=0.0){((-(v435*v1716))/(v436*v436))}else{common.v2})}))+(v442*(if v1083{common.v2}else{(if v1042{(-(common.v243*(v3379/v3459)))}else{(if v925{(-((v1017*v2967)+(v974*(((if v925{((((v976*(common.v248*v2705))-(v975*(v2976+v2976)))/v2985)-v2709)}else{common.v2})+v3211)/v3218))))}else{common.v2})})}))))-(v1085*(if v1076{common.v2}else{(if v1055{(((if v1055{((v1979+(v3409+v3409))/v3416)}else{v3283})+(if v1055{((v1979+(v3427+v3427))/v3434)}else{v3301}))-v1980)}else{(if v971{common.v2}else{(if v932{((v3283+v3301)-v1980)}else{v2795})})})})))/v3489))));
        let v3515=((v1088*v3380)+(v1054*(v685*(((v1086*(v442*(if v1083{common.v2}else{(if v1042{(-(common.v243*((v2053+v3380)/v3459)))}else{(if v925{(-((v1017*v2968)+(v974*(((if v925{((((v976*(common.v248*v2713))-(v975*(v2978+v2978)))/v2985)-v2710)}else{common.v2})+v3212)/v3218))))}else{common.v2})})})))-(v1085*(if v1076{common.v2}else{(if v1055{((if v1055{((v3411+v3411)/v3416)}else{v3284})+(if v1055{((v3429+v3429)/v3434)}else{v3302}))}else{(if v971{common.v2}else{(if v932{(v3284+v3302)}else{v2796})})})})))/v3489))));
        let v3518=((v1088*v3381)+(v1054*(v685*(((v1086*(v442*(if v1083{common.v2}else{(if v1042{(-(common.v243*((v2054+v3381)/v3459)))}else{(if v925{(-((v1017*v2969)+(v974*(((if v925{((((v976*(common.v248*v2714))-(v975*(v2980+v2980)))/v2985)-v2711)}else{common.v2})+v3213)/v3218))))}else{common.v2})})})))-(v1085*(if v1076{common.v2}else{(if v1055{((if v1055{((v3413+v3413)/v3416)}else{v3285})+(if v1055{((v3431+v3431)/v3434)}else{v3303}))}else{(if v971{common.v2}else{(if v932{(v3285+v3303)}else{v2797})})})})))/v3489))));
        let v3523=((-v1753)/(v475*v475));
        let v3524=(if v1094{v3523}else{common.v2});
        let v3525=(self.scalar_static_f64[166]*v1096);
        let v3527=(self.scalar_static_f64[165]*v1096);
        let v3538=(-v1764);
        let v3547=(if v1103{(v1105*v3525)}else{(if v1098{(v1100*v3525)}else{common.v2})});
        let v3548=(if v1103{((v1108*(v1105*((v1096*v1764)+(v484*v3524))))+(v1105*((v1106*v3524)+(v1096*v3538))))}else{(if v1098{(v1100*(common.v397*v3524))}else{common.v2})});
        let v3549=(if v1103{(v1105*v3527)}else{(if v1098{(v1100*v3527)}else{common.v2})});
        let v3563=((-v1769)/(v492*v492));
        let v3564=(if v1118{v3563}else{v3524});
        let v3565=(self.scalar_static_f64[166]*v1120);
        let v3567=(self.scalar_static_f64[165]*v1120);
        let v3578=(-v1780);
        let v3607=(-(if self.scalar_static_bool[48]{common.v2}else{v1907}));
        let v3609=(if v1144{v3607}else{common.v2});
        let v3616=((-((v622*common.v1707)+(common.v418*(if self.scalar_static_bool[48]{common.v2}else{v1910}))))/(v1148*v1148));
        let v3617=(if v1144{v3616}else{common.v2});
        let v3618=(v1150*(if v1144{self.scalar_static_f64[165]}else{common.v2}));
        let v3622=(v1150*(if v1144{self.scalar_static_f64[166]}else{common.v2}));
        let v3669=(if v1178{common.v2}else{(if v1090{(((if v1090{((if v1115{common.v2}else{(if v1094{(v1091*v3547)}else{common.v2})})+(if v1139{common.v2}else{(if v1118{(v1092*(if v1127{(v1129*v3565)}else{(if v1122{(v1124*v3565)}else{v3547})}))}else{common.v2})}))}else{common.v2})+(if v1172{common.v2}else{(if v1144{(self.scalar_static_f64[200]*(if v1157{(v1159*v3618)}else{(if v1152{(v1154*v3618)}else{common.v2})}))}else{common.v2})}))+self.scalar_static_f64[225])}else{common.v2})});
        let v3670=(if v1178{common.v2}else{(if v1090{((if v1090{((if v1115{common.v2}else{(if v1094{((v1111*(if v1090{v1781}else{common.v2}))+(v1091*v3548))}else{common.v2})})+(if v1139{common.v2}else{(if v1118{((v1135*(if v1090{(self.scalar_static_f64[22]*v1779)}else{common.v2}))+(v1092*(if v1127{((v1132*(v1129*((v1120*v1780)+(v500*v3564))))+(v1129*((v1130*v3564)+(v1120*v3578))))}else{(if v1122{(v1124*(common.v397*v3564))}else{v3548})})))}else{common.v2})}))}else{common.v2})+(if v1172{common.v2}else{(if v1144{(self.scalar_static_f64[200]*((if v1157{((v1162*(v1159*((v1150*v1928)+(v623*v3617))))+(v1159*((v1160*v3617)+(v1150*(v3609-v1928)))))}else{(if v1152{(v1154*((v1150*v3609)+(v1147*v3617)))}else{common.v2})})-(v1167*((v1150*v3607)+(v1145*v3617)))))}else{common.v2})}))}else{common.v2})});
        let v3671=(if v1178{common.v2}else{(if v1090{(((if v1090{((if v1115{common.v2}else{(if v1094{(v1091*v3549)}else{common.v2})})+(if v1139{common.v2}else{(if v1118{(v1092*(if v1127{(v1129*v3567)}else{(if v1122{(v1124*v3567)}else{v3549})}))}else{common.v2})}))}else{common.v2})+(if v1172{common.v2}else{(if v1144{(self.scalar_static_f64[200]*(if v1157{(v1159*v3622)}else{(if v1152{(v1154*v3622)}else{common.v2})}))}else{common.v2})}))+self.scalar_static_f64[226])}else{common.v2})});
        let v3674=(if v1184{v3523}else{common.v2});
        let v3675=(self.scalar_static_f64[166]*v1185);
        let v3677=(self.scalar_static_f64[165]*v1185);
        let v3696=(if v1192{(v1194*v3675)}else{(if v1187{(v1189*v3675)}else{common.v2})});
        let v3697=(if v1192{((v1197*(v1194*((v1185*v1764)+(v484*v3674))))+(v1194*((v1195*v3674)+(v1185*v3538))))}else{(if v1187{(v1189*(common.v399*v3674))}else{common.v2})});
        let v3698=(if v1192{(v1194*v3677)}else{(if v1187{(v1189*v3677)}else{common.v2})});
        let v3710=(if v1207{v3563}else{v3674});
        let v3711=(self.scalar_static_f64[166]*v1208);
        let v3713=(self.scalar_static_f64[165]*v1208);
        let v3753=(if v1231{v3607}else{common.v2});
        let v3755=(if v1231{v3616}else{common.v2});
        let v3756=(v1234*(if v1231{self.scalar_static_f64[165]}else{common.v2}));
        let v3760=(v1234*(if v1231{self.scalar_static_f64[166]}else{common.v2}));
        let v3805=(if v1260{common.v2}else{(if v1180{(self.scalar_static_f64[225]+((if v1180{((if v1204{common.v2}else{(if v1184{(v1181*v3696)}else{common.v2})})+(if v1227{common.v2}else{(if v1207{(v1182*(if v1215{(v1217*v3711)}else{(if v1210{(v1212*v3711)}else{v3696})}))}else{common.v2})}))}else{common.v2})+(if v1254{common.v2}else{(if v1231{(self.scalar_static_f64[200]*(if v1241{(v1243*v3756)}else{(if v1236{(v1238*v3756)}else{common.v2})}))}else{common.v2})})))}else{common.v2})});
        let v3806=(if v1260{common.v2}else{(if v1180{((if v1180{((if v1204{common.v2}else{(if v1184{((v1200*(if v1180{v1781}else{common.v2}))+(v1181*v3697))}else{common.v2})})+(if v1227{common.v2}else{(if v1207{((v1223*(if v1180{(self.scalar_static_f64[24]*v1779)}else{common.v2}))+(v1182*(if v1215{((v1220*(v1217*((v1208*v1780)+(v500*v3710))))+(v1217*((v1218*v3710)+(v1208*v3578))))}else{(if v1210{(v1212*(common.v399*v3710))}else{v3697})})))}else{common.v2})}))}else{common.v2})+(if v1254{common.v2}else{(if v1231{(self.scalar_static_f64[200]*((if v1241{((v1246*(v1243*((v1234*v1928)+(v623*v3755))))+(v1243*((v1244*v3755)+(v1234*(v3753-v1928)))))}else{(if v1236{(v1238*((v1234*v3753)+(v1233*v3755)))}else{common.v2})})-(v1250*((v1234*v3607)+(v1145*v3755)))))}else{common.v2})}))}else{common.v2})});
        let v3807=(if v1260{common.v2}else{(if v1180{(self.scalar_static_f64[226]+((if v1180{((if v1204{common.v2}else{(if v1184{(v1181*v3698)}else{common.v2})})+(if v1227{common.v2}else{(if v1207{(v1182*(if v1215{(v1217*v3713)}else{(if v1210{(v1212*v3713)}else{v3698})}))}else{common.v2})}))}else{common.v2})+(if v1254{common.v2}else{(if v1231{(self.scalar_static_f64[200]*(if v1241{(v1243*v3760)}else{(if v1236{(v1238*v3760)}else{common.v2})}))}else{common.v2})})))}else{common.v2})});
        let v3833=(-v1267);
        let v3835=(-v1272);

        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v1332),
            [1, 3, 4, 5],
            [(self.scalar_static_f64[166]*v3509), (self.scalar_static_f64[166]*v3512), (self.scalar_static_f64[166]*v3515), (self.scalar_static_f64[166]*v3518)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (v1333),
            1,
            multiplicity * ((self.scalar_static_f64[166]*v3669)),
            3,
            multiplicity * ((self.scalar_static_f64[166]*v3670)),
            4,
            multiplicity * ((self.scalar_static_f64[166]*v3671)),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * (v1334),
            1,
            multiplicity * ((self.scalar_static_f64[166]*v3805)),
            3,
            multiplicity * ((self.scalar_static_f64[166]*v3806)),
            5,
            multiplicity * ((self.scalar_static_f64[166]*v3807)),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[88]{(v9*common.v388)}else{(if self.scalar_static_bool[87]{(v1285*v1325)}else{(if self.scalar_static_bool[86]{((self.scalar_static_f64[278]*(f64::powf(v1314,self.scalar_static_f64[202])-common.v0))/self.scalar_static_f64[202])}else{(if self.scalar_static_bool[78]{v1285}else{common.v2})})})})),
            3,
            multiplicity * ((if self.scalar_static_bool[88]{v9}else{(if self.scalar_static_bool[87]{((self.scalar_static_f64[264]*v1325)+(v1285*self.scalar_static_f64[281]))}else{(if self.scalar_static_bool[86]{((self.scalar_static_f64[278]*(self.scalar_static_f64[280]*(self.scalar_static_f64[202]*f64::powf(v1314,self.scalar_static_f64[227]))))/self.scalar_static_f64[202])}else{self.scalar_static_f64[279]})})})),
        );
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[88]{common.v2}else{(if self.scalar_static_bool[77]{(-(((((v394*v1089)+(common.v397*v1179))+(common.v399*v1261))+(v1267*v1269))+(v1272*v1274)))}else{common.v2})})),
            &[(if self.scalar_static_bool[88]{common.v2}else{(if self.scalar_static_bool[77]{v3833}else{common.v2})}),(if self.scalar_static_bool[88]{common.v2}else{(if self.scalar_static_bool[77]{(-(((v394*v3509)+(v1333+(common.v397*v3669)))+(v1334+(common.v399*v3805))))}else{common.v2})}),(if self.scalar_static_bool[88]{common.v2}else{(if self.scalar_static_bool[77]{v3835}else{common.v2})}),(if self.scalar_static_bool[88]{common.v2}else{(if self.scalar_static_bool[77]{(-(((v394*v3512)+(common.v397*v3670))+(common.v399*v3806)))}else{common.v2})}),(if self.scalar_static_bool[88]{common.v2}else{(if self.scalar_static_bool[77]{(-((((self.scalar_static_f64[165]*v1089)+(v394*v3515))+((self.scalar_static_f64[165]*v1179)+(common.v397*v3671)))+v3833))}else{common.v2})}),(if self.scalar_static_bool[88]{common.v2}else{(if self.scalar_static_bool[77]{(-(((v1332+(v394*v3518))+((self.scalar_static_f64[165]*v1261)+(common.v399*v3807)))+v3835))}else{common.v2})})],
            &[(if self.scalar_static_bool[88]{common.v2}else{(if self.scalar_static_bool[77]{(-v1269)}else{common.v2})}),(if self.scalar_static_bool[88]{common.v2}else{(if self.scalar_static_bool[77]{(-v1274)}else{common.v2})})],
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
            (if v1682{(v456*v1689)}else{common.v2}),
            3,
            (if v1682{(v1689*v1734)}else{common.v2}),
            0,
            (if v1682{v1685}else{common.v2}),
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * ((if v1692{(v1269/v1685)}else{common.v2})),
            0,
            multiplicity * ((if v1692{(common.v0/v1685)}else{common.v2})),
            3,
            multiplicity * ((if v1692{((-(v1269*(self.scalar_static_f64[116]*v1734)))/(v1685*v1685))}else{common.v2})),
            4,
            multiplicity * ((if v1692{(v678/v1685)}else{common.v2})),
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_node1_branch1_local(
            1,
            (if v1684{(v456*v1695)}else{common.v2}),
            3,
            (if v1684{(v1695*v1734)}else{common.v2}),
            1,
            (if v1684{v1687}else{common.v2}),
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(5),
            multiplicity * ((if v1698{(v1274/v1687)}else{common.v2})),
            2,
            multiplicity * ((if v1698{(common.v0/v1687)}else{common.v2})),
            3,
            multiplicity * ((if v1698{((-(v1274*(self.scalar_static_f64[119]*v1734)))/(v1687*v1687))}else{common.v2})),
            5,
            multiplicity * ((if v1698{(v678/v1687)}else{common.v2})),
        );
        let v1677_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v1677);
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (v1677_ddt),
            1,
            multiplicity * (((common.v4710) * ddt_scale)),
            3,
            multiplicity * (((common.v4711) * ddt_scale)),
            4,
            multiplicity * (((common.v4712) * ddt_scale)),
        );
        let v1678_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v1678);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (v1678_ddt),
            [1, 3, 4, 5],
            [((common.v4713) * ddt_scale), ((common.v4714) * ddt_scale), ((common.v4715) * ddt_scale), ((common.v4716) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1679_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v1679);
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v1679_ddt),
            3,
            multiplicity * (((self.scalar_static_f64[142]) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(5),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (common.v2),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (common.v2),
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
            multiplicity * (common.v4710),
            nodes[3],
            multiplicity * (common.v4711),
            nodes[4],
            multiplicity * (common.v4712),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &[nodes[1], nodes[3], nodes[4], nodes[5]],
            &[common.v4713, common.v4714, common.v4715, common.v4716],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (self.scalar_static_f64[142]),
        );
    }
}
