#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs4_offset_indices(716, 635, p.p992, 637, p.p994, 638, p.p995, 639, p.p996, ((p.p991) + ((s.v[636] * p.p993))));

        s.store_add_scaled_inputs4_offset_indices(719, 635, p.p1010, 637, p.p1012, 638, p.p1013, 639, p.p1014, ((p.p1009) + ((s.v[636] * p.p1011))));

        s.store_add_scaled_inputs4_offset_indices(720, 635, p.p1016, 637, p.p1018, 638, p.p1019, 639, p.p1020, ((p.p1015) + ((s.v[636] * p.p1017))));

        s.store_add_scaled_inputs4_offset_indices(721, 635, p.p1120, 637, p.p1122, 638, p.p1123, 639, p.p1124, ((p.p1119) + ((s.v[636] * p.p1121))));

        s.store_add_scaled_inputs4_offset_indices(722, 635, p.p1126, 637, p.p1128, 638, p.p1129, 639, p.p1130, ((p.p1125) + ((s.v[636] * p.p1127))));

        s.store_add_scaled_inputs4_offset_indices(723, 635, p.p1132, 637, p.p1134, 638, p.p1135, 639, p.p1136, ((p.p1131) + ((s.v[636] * p.p1133))));

        s.store_add_scaled_inputs4_offset_indices(724, 635, p.p1138, 637, p.p1140, 638, p.p1141, 639, p.p1142, ((p.p1137) + ((s.v[636] * p.p1139))));

        s.store_add_scaled_inputs4_offset_indices(725, 635, p.p1144, 637, p.p1146, 638, p.p1147, 639, p.p1148, ((p.p1143) + ((s.v[636] * p.p1145))));

        s.store_add_scaled_inputs4_offset_indices(726, 635, p.p1150, 637, p.p1152, 638, p.p1153, 639, p.p1154, ((p.p1149) + ((s.v[636] * p.p1151))));

        s.store_add_scaled_inputs4_offset_indices(727, 635, p.p1156, 637, p.p1158, 638, p.p1159, 639, p.p1160, ((p.p1155) + ((s.v[636] * p.p1157))));

        s.store_add_scaled_inputs4_offset_indices(728, 635, p.p1162, 637, p.p1164, 638, p.p1165, 639, p.p1166, ((p.p1161) + ((s.v[636] * p.p1163))));

        s.store_add_scaled_inputs4_offset_indices(729, 635, p.p1168, 637, p.p1170, 638, p.p1171, 639, p.p1172, ((p.p1167) + ((s.v[636] * p.p1169))));

        s.store_add_scaled_inputs4_offset_indices(730, 635, p.p1174, 637, p.p1176, 638, p.p1177, 639, p.p1178, ((p.p1173) + ((s.v[636] * p.p1175))));

        s.store_add_scaled_inputs4_offset_indices(731, 635, p.p1180, 637, p.p1182, 638, p.p1183, 639, p.p1184, ((p.p1179) + ((s.v[636] * p.p1181))));

        s.store_add_scaled_inputs4_offset_indices(732, 635, p.p1186, 637, p.p1188, 638, p.p1189, 639, p.p1190, ((p.p1185) + ((s.v[636] * p.p1187))));

        s.store_add_scaled_inputs4_offset_indices(733, 635, p.p1192, 637, p.p1194, 638, p.p1195, 639, p.p1196, ((p.p1191) + ((s.v[636] * p.p1193))));

        s.store_add_scaled_inputs4_offset_indices(734, 635, p.p1198, 637, p.p1200, 638, p.p1201, 639, p.p1202, ((p.p1197) + ((s.v[636] * p.p1199))));

        s.store_add_scaled_inputs4_offset_indices(735, 635, p.p1204, 637, p.p1206, 638, p.p1207, 639, p.p1208, ((p.p1203) + ((s.v[636] * p.p1205))));

        s.store_add_scaled_inputs4_offset_indices(736, 635, p.p1210, 637, p.p1212, 638, p.p1213, 639, p.p1214, ((p.p1209) + ((s.v[636] * p.p1211))));

        s.store_add_scaled_inputs4_offset_indices(737, 635, p.p1216, 637, p.p1218, 638, p.p1219, 639, p.p1220, ((p.p1215) + ((s.v[636] * p.p1217))));

        s.store_add_scaled_inputs4_offset_indices(738, 635, p.p1222, 637, p.p1224, 638, p.p1225, 639, p.p1226, ((p.p1221) + ((s.v[636] * p.p1223))));

        s.store_add_scaled_inputs4_offset_indices(739, 635, p.p1228, 637, p.p1230, 638, p.p1231, 639, p.p1232, ((p.p1227) + ((s.v[636] * p.p1229))));

        s.store_add_scaled_inputs4_offset_indices(740, 635, p.p1234, 637, p.p1236, 638, p.p1237, 639, p.p1238, ((p.p1233) + ((s.v[636] * p.p1235))));

        s.store_add_scaled_inputs4_offset_indices(743, 635, p.p1240, 637, p.p1242, 638, p.p1243, 639, p.p1244, ((p.p1239) + ((s.v[636] * p.p1241))));

        s.store_add_scaled_inputs4_offset_indices(744, 635, p.p1246, 637, p.p1248, 638, p.p1249, 639, p.p1250, ((p.p1245) + ((s.v[636] * p.p1247))));

        s.store_add_scaled_inputs4_offset_indices(745, 635, p.p1252, 637, p.p1254, 638, p.p1255, 639, p.p1256, ((p.p1251) + ((s.v[636] * p.p1253))));

        s.store_add_scaled_inputs4_offset_indices(746, 635, p.p1258, 637, p.p1260, 638, p.p1261, 639, p.p1262, ((p.p1257) + ((s.v[636] * p.p1259))));

        s.store_add_scaled_inputs4_offset_indices(742, 635, p.p1264, 637, p.p1266, 638, p.p1267, 639, p.p1268, ((p.p1263) + ((s.v[636] * p.p1265))));

        s.store_add_scaled_inputs4_offset_indices(747, 635, p.p1270, 637, p.p1272, 638, p.p1273, 639, p.p1274, ((p.p1269) + ((s.v[636] * p.p1271))));

        s.store_add_scaled_inputs4_offset_indices(748, 635, p.p1276, 637, p.p1278, 638, p.p1279, 639, p.p1280, ((p.p1275) + ((s.v[636] * p.p1277))));

        s.store_add_scaled_inputs4_offset_indices(749, 635, p.p1282, 637, p.p1284, 638, p.p1285, 639, p.p1286, ((p.p1281) + ((s.v[636] * p.p1283))));

        s.store_add_scaled_inputs4_offset_indices(750, 635, p.p1288, 637, p.p1290, 638, p.p1291, 639, p.p1292, ((p.p1287) + ((s.v[636] * p.p1289))));

        s.store_add_scaled_inputs4_offset_indices(751, 635, p.p1294, 637, p.p1296, 638, p.p1297, 639, p.p1298, ((p.p1293) + ((s.v[636] * p.p1295))));

        s.store_add_scaled_inputs4_offset_indices(752, 635, p.p1330, 637, p.p1332, 638, p.p1333, 639, p.p1334, ((p.p1329) + ((s.v[636] * p.p1331))));

        s.store_add_scaled_inputs4_offset_indices(753, 635, p.p1336, 637, p.p1338, 638, p.p1339, 639, p.p1340, ((p.p1335) + ((s.v[636] * p.p1337))));

        s.store_add_scaled_inputs4_offset_indices(754, 635, p.p1342, 637, p.p1344, 638, p.p1345, 639, p.p1346, ((p.p1341) + ((s.v[636] * p.p1343))));

        s.store_add_scaled_inputs4_offset_indices(755, 635, p.p1348, 637, p.p1350, 638, p.p1351, 639, p.p1352, ((p.p1347) + ((s.v[636] * p.p1349))));

        s.store_add_scaled_inputs4_offset_indices(761, 635, p.p1300, 637, p.p1302, 638, p.p1303, 639, p.p1304, ((p.p1299) + ((s.v[636] * p.p1301))));

        s.store_add_scaled_inputs4_offset_indices(762, 635, p.p1306, 637, p.p1308, 638, p.p1309, 639, p.p1310, ((p.p1305) + ((s.v[636] * p.p1307))));

        s.store_add_scaled_inputs4_offset_indices(763, 635, p.p1312, 637, p.p1314, 638, p.p1315, 639, p.p1316, ((p.p1311) + ((s.v[636] * p.p1313))));

        s.store_add_scaled_inputs4_offset_indices(764, 635, p.p1318, 637, p.p1320, 638, p.p1321, 639, p.p1322, ((p.p1317) + ((s.v[636] * p.p1319))));

        s.store_add_scaled_inputs4_offset_indices(765, 635, p.p1324, 637, p.p1326, 638, p.p1327, 639, p.p1328, ((p.p1323) + ((s.v[636] * p.p1325))));

        s.store_add_scaled_inputs4_offset_indices(766, 635, p.p1354, 637, p.p1356, 638, p.p1357, 639, p.p1358, ((p.p1353) + ((s.v[636] * p.p1355))));

        s.store_add_scaled_inputs4_offset_indices(767, 635, p.p1360, 637, p.p1362, 638, p.p1363, 639, p.p1364, ((p.p1359) + ((s.v[636] * p.p1361))));

        s.store_add_scaled_inputs4_offset_indices(768, 635, p.p1366, 637, p.p1368, 638, p.p1369, 639, p.p1370, ((p.p1365) + ((s.v[636] * p.p1367))));

        s.store_add_scaled_inputs4_offset_indices(769, 635, p.p1372, 637, p.p1374, 638, p.p1375, 639, p.p1376, ((p.p1371) + ((s.v[636] * p.p1373))));

        s.store_add_scaled_inputs4_offset_indices(775, 635, p.p1445, 637, p.p1447, 638, p.p1448, 639, p.p1449, ((p.p1444) + ((s.v[636] * p.p1446))));

        s.store_add_scaled_inputs4_offset_indices(776, 635, p.p1451, 637, p.p1453, 638, p.p1454, 639, p.p1455, ((p.p1450) + ((s.v[636] * p.p1452))));

        s.store_add_scaled_inputs4_offset_indices(777, 635, p.p1463, 637, p.p1465, 638, p.p1466, 639, p.p1467, ((p.p1462) + ((s.v[636] * p.p1464))));

        s.store_add_scaled_inputs4_offset_indices(778, 635, p.p1469, 637, p.p1471, 638, p.p1472, 639, p.p1473, ((p.p1468) + ((s.v[636] * p.p1470))));

        s.store_add_scaled_inputs4_offset_indices(779, 635, p.p1457, 637, p.p1459, 638, p.p1460, 639, p.p1461, ((p.p1456) + ((s.v[636] * p.p1458))));

        s.store_add_scaled_inputs4_offset_indices(780, 635, p.p1475, 637, p.p1477, 638, p.p1478, 639, p.p1479, ((p.p1474) + ((s.v[636] * p.p1476))));

        s.store_add_scaled_inputs4_offset_indices(781, 635, p.p1481, 637, p.p1483, 638, p.p1484, 639, p.p1485, ((p.p1480) + ((s.v[636] * p.p1482))));

        s.store_add_scaled_inputs4_offset_indices(782, 635, p.p1487, 637, p.p1489, 638, p.p1490, 639, p.p1491, ((p.p1486) + ((s.v[636] * p.p1488))));

        s.store_add_scaled_inputs4_offset_indices(783, 635, p.p1493, 637, p.p1495, 638, p.p1496, 639, p.p1497, ((p.p1492) + ((s.v[636] * p.p1494))));

        s.store_add_scaled_inputs4_offset_indices(784, 635, p.p1499, 637, p.p1501, 638, p.p1502, 639, p.p1503, ((p.p1498) + ((s.v[636] * p.p1500))));

        s.store_add_scaled_inputs4_offset_indices(785, 635, p.p1505, 637, p.p1507, 638, p.p1508, 639, p.p1509, ((p.p1504) + ((s.v[636] * p.p1506))));

        s.store_add_scaled_inputs4_offset_indices(786, 635, p.p1511, 637, p.p1513, 638, p.p1514, 639, p.p1515, ((p.p1510) + ((s.v[636] * p.p1512))));

        s.store_add_scaled_inputs4_offset_indices(787, 635, p.p1517, 637, p.p1519, 638, p.p1520, 639, p.p1521, ((p.p1516) + ((s.v[636] * p.p1518))));

        s.store_add_scaled_inputs4_offset_indices(788, 635, p.p1523, 637, p.p1525, 638, p.p1526, 639, p.p1527, ((p.p1522) + ((s.v[636] * p.p1524))));

        s.store_add_scaled_inputs4_offset_indices(789, 635, p.p1763, 637, p.p1765, 638, p.p1766, 639, p.p1767, ((p.p1762) + ((s.v[636] * p.p1764))));

        s.store_add_scaled_inputs4_offset_indices(643, 635, p.p1531, 637, p.p1533, 638, p.p1534, 639, p.p1535, ((p.p1530) + ((s.v[636] * p.p1532))));

        s.store_add_scaled_inputs4_offset_indices(642, 635, p.p1537, 637, p.p1539, 638, p.p1540, 639, p.p1541, ((p.p1536) + ((s.v[636] * p.p1538))));

        s.store_add_scaled_inputs4_offset_indices(644, 635, p.p29, 637, p.p31, 638, p.p32, 639, p.p33, ((p.p28) + ((s.v[636] * p.p30))));

        s.store_add_scaled_inputs4_offset_indices(645, 635, p.p35, 637, p.p37, 638, p.p38, 639, p.p39, ((p.p34) + ((s.v[636] * p.p36))));

        s.store_add_scaled_inputs4_offset_indices(648, 635, p.p1548, 637, p.p1550, 638, p.p1551, 639, p.p1552, ((p.p1547) + ((s.v[636] * p.p1549))));

        s.store_add_scaled_inputs4_offset_indices(649, 635, p.p1554, 637, p.p1556, 638, p.p1557, 639, p.p1558, ((p.p1553) + ((s.v[636] * p.p1555))));

        s.store_add_scaled_inputs4_offset_indices(650, 635, p.p1560, 637, p.p1562, 638, p.p1563, 639, p.p1564, ((p.p1559) + ((s.v[636] * p.p1561))));

        s.store_add_scaled_inputs4_offset_indices(651, 635, p.p1566, 637, p.p1568, 638, p.p1569, 639, p.p1570, ((p.p1565) + ((s.v[636] * p.p1567))));

        s.store_add_scaled_inputs4_offset_indices(652, 635, p.p1572, 637, p.p1574, 638, p.p1575, 639, p.p1576, ((p.p1571) + ((s.v[636] * p.p1573))));

        s.store_add_scaled_inputs4_offset_indices(653, 635, p.p1578, 637, p.p1580, 638, p.p1581, 639, p.p1582, ((p.p1577) + ((s.v[636] * p.p1579))));

        s.store_add_scaled_inputs4_offset_indices(865, 635, p.p1657, 637, p.p1659, 638, p.p1660, 639, p.p1661, ((p.p1656) + ((s.v[636] * p.p1658))));

        s.store_add_scaled_inputs4_offset_indices(866, 635, p.p1663, 637, p.p1665, 638, p.p1666, 639, p.p1667, ((p.p1662) + ((s.v[636] * p.p1664))));

        s.store_add_scaled_inputs4_offset_indices(836, 635, p.p738, 637, p.p740, 638, p.p741, 639, p.p742, ((p.p737) + ((s.v[636] * p.p739))));

        s.store_add_scaled_inputs4_offset_indices(837, 635, p.p756, 637, p.p758, 638, p.p759, 639, p.p760, ((p.p755) + ((s.v[636] * p.p757))));

        s.store_add_scaled_inputs4_offset_indices(838, 635, p.p768, 637, p.p770, 638, p.p771, 639, p.p772, ((p.p767) + ((s.v[636] * p.p769))));

        s.store_add_scaled_inputs4_offset_indices(842, 635, p.p786, 637, p.p788, 638, p.p789, 639, p.p790, ((p.p785) + ((s.v[636] * p.p787))));

        s.store_add_scaled_inputs4_offset_indices(823, 635, p.p792, 637, p.p794, 638, p.p795, 639, p.p796, ((p.p791) + ((s.v[636] * p.p793))));

        s.store_add_scaled_inputs4_offset_indices(824, 635, p.p810, 637, p.p812, 638, p.p813, 639, p.p814, ((p.p809) + ((s.v[636] * p.p811))));

        s.store_add_scaled_inputs4_offset_indices(847, 635, p.p822, 637, p.p824, 638, p.p825, 639, p.p826, ((p.p821) + ((s.v[636] * p.p823))));

        s.store_add_scaled_inputs4_offset_indices(830, 635, p.p846, 637, p.p848, 638, p.p849, 639, p.p850, ((p.p845) + ((s.v[636] * p.p847))));

        s.store_add_scaled_inputs4_offset_indices(831, 635, p.p864, 637, p.p866, 638, p.p867, 639, p.p868, ((p.p863) + ((s.v[636] * p.p865))));

        s.store_add_scaled_inputs4_offset_indices(834, 635, p.p876, 637, p.p878, 638, p.p879, 639, p.p880, ((p.p875) + ((s.v[636] * p.p877))));

        s.store_add_scaled_inputs4_offset_indices(835, 635, p.p882, 637, p.p884, 638, p.p885, 639, p.p886, ((p.p881) + ((s.v[636] * p.p883))));

        s.store_add_scaled_inputs4_offset_indices(848, 635, p.p576, 637, p.p578, 638, p.p579, 639, p.p580, ((p.p575) + ((s.v[636] * p.p577))));

        s.store_add_scaled_inputs4_offset_indices(849, 635, p.p556, 637, p.p558, 638, p.p559, 639, p.p560, ((p.p555) + ((s.v[636] * p.p557))));

        s.store_add_scaled_inputs4_offset_indices(850, 635, p.p569, 637, p.p571, 638, p.p572, 639, p.p573, ((p.p568) + ((s.v[636] * p.p570))));

        s.store_add_scaled_inputs4_offset_indices(854, 635, p.p962, 637, p.p964, 638, p.p965, 639, p.p966, ((p.p961) + ((s.v[636] * p.p963))));

        s.store_add_scaled_inputs4_offset_indices(855, 635, p.p968, 637, p.p970, 638, p.p971, 639, p.p972, ((p.p967) + ((s.v[636] * p.p969))));

        s.store_add_scaled_inputs4_offset_indices(856, 635, p.p974, 637, p.p976, 638, p.p977, 639, p.p978, ((p.p973) + ((s.v[636] * p.p975))));

        s.store_add_scaled_inputs4_offset_indices(857, 635, p.p980, 637, p.p982, 638, p.p983, 639, p.p984, ((p.p979) + ((s.v[636] * p.p981))));

        s.store_add_scaled_inputs4_offset_indices(858, 635, p.p1742, 637, p.p1744, 638, p.p1745, 639, p.p1746, ((p.p1741) + ((s.v[636] * p.p1743))));

        s.store_add_scaled_inputs4_offset_indices(859, 635, p.p1751, 637, p.p1753, 638, p.p1754, 639, p.p1755, ((p.p1750) + ((s.v[636] * p.p1752))));

        s.store_add_scaled_inputs4_offset_indices(860, 635, p.p1757, 637, p.p1759, 638, p.p1760, 639, p.p1761, ((p.p1756) + ((s.v[636] * p.p1758))));

        s.store_add_scaled_inputs4_offset_indices(862, 635, p.p1769, 637, p.p1771, 638, p.p1772, 639, p.p1773, ((p.p1768) + ((s.v[636] * p.p1770))));

        s.store_add_scaled_inputs4_offset_indices(863, 635, p.p1775, 637, p.p1777, 638, p.p1778, 639, p.p1779, ((p.p1774) + ((s.v[636] * p.p1776))));

        s.store_add_scaled_inputs4_offset_indices(681, 635, p.p177, 637, p.p179, 638, p.p180, 639, p.p181, ((p.p176) + ((s.v[636] * p.p178))));

        s.store_add_scaled_inputs4_offset_indices(682, 635, p.p183, 637, p.p185, 638, p.p186, 639, p.p187, ((p.p182) + ((s.v[636] * p.p184))));

        s.store_add_scaled_inputs4_offset_indices(574, 635, p.p1690, 637, p.p1692, 638, p.p1693, 639, p.p1694, ((p.p1689) + ((s.v[636] * p.p1691))));

        s.store_add_scaled_inputs4_offset_indices(576, 635, p.p1702, 637, p.p1704, 638, p.p1705, 639, p.p1706, ((p.p1701) + ((s.v[636] * p.p1703))));

        s.store_add_scaled_inputs4_offset_indices(575, 635, p.p1696, 637, p.p1698, 638, p.p1699, 639, p.p1700, ((p.p1695) + ((s.v[636] * p.p1697))));

        s.b[1096] = (p.p61 != 0.0);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if s.b[1096] {
            s.store_add_scaled_inputs4_offset_indices(689, 635, p.p357, 637, p.p359, 638, p.p360, 639, p.p361, ((p.p356) + ((s.v[636] * p.p358))));
            s.store_add_scaled_inputs4_offset_indices(690, 635, p.p363, 637, p.p365, 638, p.p366, 639, p.p367, ((p.p362) + ((s.v[636] * p.p364))));
            s.store_add_scaled_inputs4_offset_indices(691, 635, p.p369, 637, p.p371, 638, p.p372, 639, p.p373, ((p.p368) + ((s.v[636] * p.p370))));
            s.store_add_scaled_inputs4_offset_indices(809, 635, p.p660, 637, p.p662, 638, p.p663, 639, p.p664, ((p.p659) + ((s.v[636] * p.p661))));
            s.store_add_scaled_inputs4_offset_indices(828, 635, p.p828, 637, p.p830, 638, p.p831, 639, p.p832, ((p.p827) + ((s.v[636] * p.p829))));
        }

        s.b[1097] = (p.p61 == 2.0);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (s.b[1096] && s.b[1097]) {
            s.store_add_scaled_inputs4_offset_indices(871, 635, p.p387, 637, p.p389, 638, p.p390, 639, p.p391, ((p.p386) + ((s.v[636] * p.p388))));
            s.store_add_scaled_inputs4_offset_indices(872, 635, p.p393, 637, p.p395, 638, p.p396, 639, p.p397, ((p.p392) + ((s.v[636] * p.p394))));
            s.store_add_scaled_inputs4_offset_indices(692, 635, p.p375, 637, p.p377, 638, p.p378, 639, p.p379, ((p.p374) + ((s.v[636] * p.p376))));
            s.store_add_scaled_inputs4_offset_indices(693, 635, p.p381, 637, p.p383, 638, p.p384, 639, p.p385, ((p.p380) + ((s.v[636] * p.p382))));
        }

        s.b[1098] = (((p.p70 == 2.0) || (p.p70 == 3.0)) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if (s.b[1096] && s.b[1098]) {
            s.store_add_scaled_inputs4_offset_indices(756, 635, p.p1378, 637, p.p1380, 638, p.p1381, 639, p.p1382, ((p.p1377) + ((s.v[636] * p.p1379))));
            s.store_add_scaled_inputs4_offset_indices(757, 635, p.p1384, 637, p.p1386, 638, p.p1387, 639, p.p1388, ((p.p1383) + ((s.v[636] * p.p1385))));
            s.store_add_scaled_inputs4_offset_indices(758, 635, p.p1390, 637, p.p1392, 638, p.p1393, 639, p.p1394, ((p.p1389) + ((s.v[636] * p.p1391))));
            s.store_add_scaled_inputs4_offset_indices(759, 635, p.p1396, 637, p.p1398, 638, p.p1399, 639, p.p1400, ((p.p1395) + ((s.v[636] * p.p1397))));
            s.store_add_scaled_inputs4_offset_indices(760, 635, p.p1402, 637, p.p1404, 638, p.p1405, 639, p.p1406, ((p.p1401) + ((s.v[636] * p.p1403))));
            s.store_add_scaled_inputs4_offset_indices(770, 635, p.p1408, 637, p.p1410, 638, p.p1411, 639, p.p1412, ((p.p1407) + ((s.v[636] * p.p1409))));
            s.store_add_scaled_inputs4_offset_indices(771, 635, p.p1414, 637, p.p1416, 638, p.p1417, 639, p.p1418, ((p.p1413) + ((s.v[636] * p.p1415))));
            s.store_add_scaled_inputs4_offset_indices(772, 635, p.p1420, 637, p.p1422, 638, p.p1423, 639, p.p1424, ((p.p1419) + ((s.v[636] * p.p1421))));
            s.store_add_scaled_inputs4_offset_indices(773, 635, p.p1426, 637, p.p1428, 638, p.p1429, 639, p.p1430, ((p.p1425) + ((s.v[636] * p.p1427))));
            s.store_add_scaled_inputs4_offset_indices(774, 635, p.p1432, 637, p.p1434, 638, p.p1435, 639, p.p1436, ((p.p1431) + ((s.v[636] * p.p1433))));
        }

        s.b[1099] = (p.p66 != 0.0);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if s.b[1099] {
            s.store_add_scaled_inputs4_offset_indices(665, 635, p.p213, 637, p.p215, 638, p.p216, 639, p.p217, ((p.p212) + ((s.v[636] * p.p214))));
            s.store_add_scaled_inputs4_offset_indices(668, 635, p.p195, 637, p.p197, 638, p.p198, 639, p.p199, ((p.p194) + ((s.v[636] * p.p196))));
            s.store_add_scaled_inputs4_offset_indices(677, 635, p.p255, 637, p.p257, 638, p.p258, 639, p.p259, ((p.p254) + ((s.v[636] * p.p256))));
            s.store_add_scaled_inputs4_offset_indices(699, 635, p.p474, 637, p.p476, 638, p.p477, 639, p.p478, ((p.p473) + ((s.v[636] * p.p475))));
            s.store_add_scaled_inputs4_offset_indices(791, 635, p.p538, 637, p.p540, 638, p.p541, 639, p.p542, ((p.p537) + ((s.v[636] * p.p539))));
            s.store_add_scaled_inputs4_offset_indices(701, 635, p.p550, 637, p.p552, 638, p.p553, 639, p.p554, ((p.p549) + ((s.v[636] * p.p551))));
            s.store_add_scaled_inputs4_offset_indices(715, 635, p.p998, 637, p.p1000, 638, p.p1001, 639, p.p1002, ((p.p997) + ((s.v[636] * p.p999))));
            s.store_add_scaled_inputs4_offset_indices(717, 635, p.p1004, 637, p.p1006, 638, p.p1007, 639, p.p1008, ((p.p1003) + ((s.v[636] * p.p1005))));
            s.store_add_scaled_inputs4_offset_indices(796, 635, p.p1033, 637, p.p1035, 638, p.p1036, 639, p.p1037, ((p.p1032) + ((s.v[636] * p.p1034))));
            s.store_add_scaled_inputs4_offset_indices(806, 635, p.p291, 637, p.p293, 638, p.p294, 639, p.p295, ((p.p290) + ((s.v[636] * p.p292))));
            s.store_add_scaled_inputs4_offset_indices(680, 635, p.p462, 637, p.p464, 638, p.p465, 639, p.p466, ((p.p461) + ((s.v[636] * p.p463))));
            s.store_add_scaled_inputs4_offset_indices(658, 635, p.p501, 637, p.p503, 638, p.p504, 639, p.p505, ((p.p500) + ((s.v[636] * p.p502))));
            s.store_add_scaled_inputs4_offset_indices(706, 635, p.p612, 637, p.p614, 638, p.p615, 639, p.p616, ((p.p611) + ((s.v[636] * p.p613))));
            s.store_add_scaled_inputs4_offset_indices(815, 635, p.p648, 637, p.p650, 638, p.p651, 639, p.p652, ((p.p647) + ((s.v[636] * p.p649))));
            s.store_add_scaled_inputs4_offset_indices(710, 635, p.p636, 637, p.p638, 638, p.p639, 639, p.p640, ((p.p635) + ((s.v[636] * p.p637))));
            s.store_add_scaled_inputs4_offset_indices(816, 635, p.p684, 637, p.p686, 638, p.p687, 639, p.p688, ((p.p683) + ((s.v[636] * p.p685))));
            s.store_add_scaled_inputs4_offset_indices(818, 635, p.p696, 637, p.p698, 638, p.p699, 639, p.p700, ((p.p695) + ((s.v[636] * p.p697))));
            s.store_add_scaled_inputs4_offset_indices(845, 635, p.p744, 637, p.p746, 638, p.p747, 639, p.p748, ((p.p743) + ((s.v[636] * p.p745))));
            s.store_add_scaled_inputs4_offset_indices(846, 635, p.p774, 637, p.p776, 638, p.p777, 639, p.p778, ((p.p773) + ((s.v[636] * p.p775))));
            s.store_add_scaled_inputs4_offset_indices(825, 635, p.p798, 637, p.p800, 638, p.p801, 639, p.p802, ((p.p797) + ((s.v[636] * p.p799))));
            s.store_add_scaled_inputs4_offset_indices(844, 635, p.p852, 637, p.p854, 638, p.p855, 639, p.p856, ((p.p851) + ((s.v[636] * p.p853))));
            s.store_add_scaled_inputs4_offset_indices(851, 635, p.p563, 637, p.p565, 638, p.p566, 639, p.p567, ((p.p562) + ((s.v[636] * p.p564))));
        }

        s.b[1100] = (p.p61 != 0.0);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if (s.b[1099] && s.b[1100]) {
            s.store_add_scaled_inputs4_offset_indices(817, 635, p.p666, 637, p.p668, 638, p.p669, 639, p.p670, ((p.p665) + ((s.v[636] * p.p667))));
            s.store_add_scaled_inputs4_offset_indices(843, 635, p.p834, 637, p.p836, 638, p.p837, 639, p.p838, ((p.p833) + ((s.v[636] * p.p835))));
        }

        s.b[1101] = (p.p67 == 1.0);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if s.b[1101] {
            s.store_add_scaled_inputs4_offset_indices(705, 635, p.p618, 637, p.p620, 638, p.p621, 639, p.p622, ((p.p617) + ((s.v[636] * p.p619))));
        }

        s.b[1102] = (p.p582 != 0.0);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if (s.b[1101] && s.b[1102]) {
            s.store_scale(705, 705, (1.0 + ((p.p582 / p.p5) * (if (!((1.0 + (p.p5 / p.p585)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p585)) > 1e-38) { (((1.0 + (p.p5 / p.p585))) as f64).ln() } else { 0.0 }) }))));
        }

        if s.b[1101] {
            s.store_add_scaled_inputs4_offset_indices(808, 635, p.p654, 637, p.p656, 638, p.p657, 639, p.p658, ((p.p653) + ((s.v[636] * p.p655))));
            s.store_add_scaled_inputs4_offset_indices(813, 635, p.p702, 637, p.p704, 638, p.p705, 639, p.p706, ((p.p701) + ((s.v[636] * p.p703))));
            s.store_add_scaled_inputs4_offset_indices(839, 635, p.p750, 637, p.p752, 638, p.p753, 639, p.p754, ((p.p749) + ((s.v[636] * p.p751))));
            s.store_add_scaled_inputs4_offset_indices(840, 635, p.p762, 637, p.p764, 638, p.p765, 639, p.p766, ((p.p761) + ((s.v[636] * p.p763))));
            s.store_add_scaled_inputs4_offset_indices(841, 635, p.p780, 637, p.p782, 638, p.p783, 639, p.p784, ((p.p779) + ((s.v[636] * p.p781))));
            s.store_add_scaled_inputs4_offset_indices(826, 635, p.p804, 637, p.p806, 638, p.p807, 639, p.p808, ((p.p803) + ((s.v[636] * p.p805))));
            s.store_add_scaled_inputs4_offset_indices(827, 635, p.p816, 637, p.p818, 638, p.p819, 639, p.p820, ((p.p815) + ((s.v[636] * p.p817))));
            s.store_add_scaled_inputs4_offset_indices(832, 635, p.p858, 637, p.p860, 638, p.p861, 639, p.p862, ((p.p857) + ((s.v[636] * p.p859))));
            s.store_add_scaled_inputs4_offset_indices(833, 635, p.p870, 637, p.p872, 638, p.p873, 639, p.p874, ((p.p869) + ((s.v[636] * p.p871))));
        }

        s.b[1103] = (p.p61 != 0.0);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if (s.b[1101] && s.b[1103]) {
            s.store_add_scaled_inputs4_offset_indices(810, 635, p.p672, 637, p.p674, 638, p.p675, 639, p.p676, ((p.p671) + ((s.v[636] * p.p673))));
            s.store_add_scaled_inputs4_offset_indices(829, 635, p.p840, 637, p.p842, 638, p.p843, 639, p.p844, ((p.p839) + ((s.v[636] * p.p841))));
        }

        if s.b[1101] {
            s.store_add_scaled_inputs4_offset_indices(675, 635, p.p261, 637, p.p263, 638, p.p264, 639, p.p265, ((p.p260) + ((s.v[636] * p.p262))));
        }

        s.b[1104] = (p.p161 != 0.0);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if (s.b[1101] && s.b[1104]) {
            s.store_scale(675, 675, (1.0 + ((p.p161 / p.p5) * (if (!((1.0 + (p.p5 / p.p162)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p162)) > 1e-38) { (((1.0 + (p.p5 / p.p162))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1105] = (p.p21 != 0.0);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if (s.b[1101] && s.b[1105]) {
            s.store_mul_scale_offset_rhs(705, 705, 153, ((p.p5 - p.p21) * p.p588), 1.0);
            s.store_mul_scale_offset_rhs(675, 675, 153, ((p.p5 - p.p21) * p.p163), 1.0);
        }

        s.b[1107] = (p.p57 == 1.0);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if s.b[1107] {
            s.store_add_scaled_inputs4_offset_indices(882, 635, p.p1808, 637, p.p1810, 638, p.p1811, 639, p.p1812, ((p.p1807) + ((s.v[636] * p.p1809))));
            s.store_add_scaled_inputs4_offset_indices(883, 635, p.p1815, 637, p.p1817, 638, p.p1818, 639, p.p1819, ((p.p1814) + ((s.v[636] * p.p1816))));
            s.store_add_scaled_inputs4_offset_indices(884, 635, p.p1822, 637, p.p1824, 638, p.p1825, 639, p.p1826, ((p.p1821) + ((s.v[636] * p.p1823))));
            s.store_add_scaled_inputs4_offset_indices(885, 635, p.p1830, 637, p.p1832, 638, p.p1833, 639, p.p1834, ((p.p1829) + ((s.v[636] * p.p1831))));
            s.store_add_scaled_inputs4_offset_indices(886, 635, p.p1836, 637, p.p1838, 638, p.p1839, 639, p.p1840, ((p.p1835) + ((s.v[636] * p.p1837))));
            s.store_add_scaled_inputs4_offset_indices(887, 635, p.p1842, 637, p.p1844, 638, p.p1845, 639, p.p1846, ((p.p1841) + ((s.v[636] * p.p1843))));
            s.store_add_scaled_inputs4_offset_indices(888, 635, p.p1854, 637, p.p1856, 638, p.p1857, 639, p.p1858, ((p.p1853) + ((s.v[636] * p.p1855))));
            s.store_add_scaled_inputs4_offset_indices(889, 635, p.p1860, 637, p.p1862, 638, p.p1863, 639, p.p1864, ((p.p1859) + ((s.v[636] * p.p1861))));
            s.store_add_scaled_inputs4_offset_indices(890, 635, p.p1870, 637, p.p1872, 638, p.p1873, 639, p.p1874, ((p.p1869) + ((s.v[636] * p.p1871))));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1107] {
            s.store_add_scaled_inputs4_offset_indices(891, 635, p.p1876, 637, p.p1878, 638, p.p1879, 639, p.p1880, ((p.p1875) + ((s.v[636] * p.p1877))));
            s.store_add_scaled_inputs4_offset_indices(892, 635, p.p1882, 637, p.p1884, 638, p.p1885, 639, p.p1886, ((p.p1881) + ((s.v[636] * p.p1883))));
        }

        s.b[1108] = (p.p100 != 0.0);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if s.b[1108] {
            s.store_scale(641, 641, (1.0 + ((p.p100 / p.p5) * (if (!((1.0 + (p.p5 / p.p101)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p101)) > 1e-38) { (((1.0 + (p.p5 / p.p101))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1109] = (p.p158 != 0.0);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if s.b[1109] {
            s.store_scale(673, 673, (1.0 + ((p.p158 / p.p5) * (if (!((1.0 + (p.p5 / p.p159)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p159)) > 1e-38) { (((1.0 + (p.p5 / p.p159))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1110] = (p.p152 != 0.0);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if s.b[1110] {
            s.store_scale(662, 662, (1.0 + ((p.p152 / p.p5) * (if (!((1.0 + (p.p5 / p.p153)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p153)) > 1e-38) { (((1.0 + (p.p5 / p.p153))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1111] = (p.p154 != 0.0);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if s.b[1111] {
            s.store_scale(663, 663, (1.0 + ((p.p154 / p.p5) * (if (!((1.0 + (p.p5 / p.p155)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p155)) > 1e-38) { (((1.0 + (p.p5 / p.p155))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1112] = (p.p156 != 0.0);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if s.b[1112] {
            s.store_scale(665, 665, (1.0 + ((p.p156 / p.p5) * (if (!((1.0 + (p.p5 / p.p157)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p157)) > 1e-38) { (((1.0 + (p.p5 / p.p157))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1113] = (p.p428 != 0.0);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if s.b[1113] {
            s.store_scale(679, 679, (1.0 + ((p.p428 / p.p5) * (if (!((1.0 + (p.p5 / p.p429)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p429)) > 1e-38) { (((1.0 + (p.p5 / p.p429))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1114] = (p.p432 != 0.0);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if s.b[1114] {
            s.store_scale(698, 698, (1.0 + ((p.p432 / p.p5) * (if (!((1.0 + (p.p5 / p.p433)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p433)) > 1e-38) { (((1.0 + (p.p5 / p.p433))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1115] = (p.p434 != 0.0);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if s.b[1115] {
            s.store_scale(699, 699, (1.0 + ((p.p434 / p.p5) * (if (!((1.0 + (p.p5 / p.p435)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p435)) > 1e-38) { (((1.0 + (p.p5 / p.p435))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1116] = (p.p581 != 0.0);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if s.b[1116] {
            s.store_scale(704, 704, (1.0 + ((p.p581 / p.p5) * (if (!((1.0 + (p.p5 / p.p584)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p584)) > 1e-38) { (((1.0 + (p.p5 / p.p584))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1117] = (p.p583 != 0.0);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if s.b[1117] {
            s.store_scale(706, 706, (1.0 + ((p.p583 / p.p5) * (if (!((1.0 + (p.p5 / p.p586)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p586)) > 1e-38) { (((1.0 + (p.p5 / p.p586))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1118] = (p.p21 != 0.0);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if s.b[1118] {
            s.store_mul_scale_offset_rhs(641, 641, 153, ((p.p5 - p.p21) * p.p99), 1.0);
            s.store_mul_scale_offset_rhs(673, 673, 153, ((p.p5 - p.p21) * p.p160), 1.0);
            s.store_mul_scale_offset_rhs(704, 704, 153, ((p.p5 - p.p21) * p.p587), 1.0);
        }

        s.store_ln(154, 153);

        s.store_add_scaled_inputs(641, 641, 1.0, 153, p.p98);

        s.store_add_scaled_inputs(661, 661, 1.0, 153, p.p427);

        s.b[1119] = (p.p589 > 0.0);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if s.b[1119] {
            s.store_mul_sub_from_scalar_ad_rhs(704, 704, 1.0, A::mul(s.ad_value(703), A::exp_scaled_input(s.ad_value(154), (-p.p589))));
        }

        if (!s.b[1119]) {
            s.store_mul_sub_from_scalar_rhs(704, 704, 1.0, 703);
        }

        s.store_add_scaled_inputs_ad_rhs(807, 807, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p593))), p.p591);

        s.store_add_scaled_inputs_ad_rhs(812, 812, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p601))), p.p599);

        s.store_add_scaled_inputs_ad_rhs(811, 811, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p597))), p.p595);

        s.b[1120] = (p.p66 != 0.0);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if s.b[1120] {
            s.store_add_scaled_inputs_ad_rhs(815, 815, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p594))), p.p592);
            s.store_add_scaled_inputs_ad_rhs(818, 818, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p602))), p.p600);
            s.store_add_scaled_inputs_ad_rhs(816, 816, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p598))), p.p596);
        }

        s.b[1121] = (p.p590 > 0.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (s.b[1120] && s.b[1121]) {
            s.store_mul_sub_from_scalar_ad_rhs(706, 706, 1.0, A::mul(s.ad_value(710), A::exp_scaled_input(s.ad_value(154), (-p.p590))));
        }

        if (s.b[1120] && (!s.b[1121])) {
            s.store_mul_sub_from_scalar_rhs(706, 706, 1.0, 710);
        }

        s.b[1122] = (p.p64 == 1.0);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if s.b[1122] {
            s.store_add_scaled_inputs_ad_rhs(853, 853, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p913))), p.p912);
            s.store_add_scaled_inputs_ad_rhs(852, 852, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p916))), p.p915);
        }

        if (!s.b[1122]) {
            s.store_add_scaled_inputs_ad_rhs(709, 709, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p910))), p.p909);
        }

        s.store_add_scaled_inputs_ad_rhs(792, 792, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p1023))), p.p1021);

        s.b[1123] = (p.p66 != 0.0);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if s.b[1123] {
            s.store_add_scaled_inputs_ad_rhs(796, 796, 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p1024)), p.p1022);
        }

        s.store_add_scaled_inputs_ad_rhs(790, 790, 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p445)), p.p444);

        s.b[1124] = (p.p66 != 0.0);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if s.b[1124] {
            s.store_add_scaled_inputs_ad_rhs(791, 791, 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p447)), p.p446);
        }

        s.store_add_scaled_inputs_ad_rhs(700, 700, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p449))), p.p448);

        s.b[1125] = (p.p66 != 0.0);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if s.b[1125] {
            s.store_add_scaled_inputs_ad_rhs(701, 701, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p449))), p.p448);
        }

        s.store_add_scaled_inputs_ad_rhs(679, 679, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p431))), p.p430);

        s.store_add_scaled_inputs_ad_rhs(698, 698, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p437))), p.p436);

        s.b[1126] = (p.p66 != 0.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if s.b[1126] {
            s.store_add_scaled_inputs_ad_rhs(699, 699, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p437))), p.p436);
        }

        s.store_add_scaled_inputs_ad_rhs(695, 695, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p439))), p.p438);

        s.store_add_scaled_inputs_ad_rhs(697, 697, 1.0, A::limited_exp_scaled_input(s.ad_value(156), (-1.0 / (p.p443))), p.p442);

        s.store_add_scaled_inputs_ad_rhs(702, 702, 1.0, A::limited_exp_scaled_input(s.ad_value(156), (-1.0 / (p.p441))), p.p440);

        s.store_add_scaled_inputs_ad_rhs(681, 681, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p168))), p.p167);

        s.store_add_scaled_inputs_ad_rhs(682, 682, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p170))), p.p169);

        s.b[1127] = ((s.v[655] > 0.0) || (s.v[656] > 0.0));
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if s.b[1127] {
            s.store_offset_scaled_ad(376, A::limited_exp_scaled_input(A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0), (-1.0 / (p.p399))), p.p398, 1.0);
            s.store_mul_div_scaled_inputs_rhs(373, 376, s.ad_value(894), 2.0, s.ad_value(895), 1.0);
        }

        s.b[1130] = (s.v[576] <= 0.0);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if s.b[1130] {
            s.store_scalar(576, 0.05);
        }

        s.b[1135] = (s.v[641] <= 0.0);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        if s.b[1135] {
            s.store_scalar(641, 4.61);
        }

        s.b[1136] = (p.p61 != 0.0);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        s.b[1137] = (s.v[690] < 1e-6);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if (s.b[1136] && s.b[1137]) {
            s.store_scalar(690, 1e-6);
        }

        s.b[1138] = (s.v[857] < 0.0);
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if s.b[1138] {
            s.store_scalar(857, 0.01);
        }

        s.b[1139] = (s.v[576] < 0.0);
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if s.b[1139] {
            s.store_scalar(576, 0.05);
        }

        s.b[1140] = (s.v[574] < 0.0);
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        if s.b[1140] {
            s.store_scalar(574, p.p1682);
        }

        s.b[1141] = (s.v[575] < 0.0);
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if s.b[1141] {
            s.store_scalar(575, 1.2);
        }

        s.b[1142] = (s.v[644] < 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.store_scalar(644, 0.0);
        }

        s.b[1143] = (s.v[645] < 0.0);
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_scalar(645, 0.0);
        }

        s.b[1144] = (s.v[679] <= 0.0);
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if s.b[1144] {
            s.store_scalar(679, 85000.0);
        }

        s.b[1145] = (s.v[698] <= 0.0);
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if s.b[1145] {
            s.store_scalar(698, 85000.0);
        }

        s.b[1146] = ((p.p66 != 0.0) && (s.v[699] <= 0.0));
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if s.b[1146] {
            s.store_scalar(699, 85000.0);
        }

        s.b[1147] = (s.v[670] <= 0.0);
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if s.b[1147] {
            s.store_scalar(670, 0.6);
        }

        s.b[1148] = (s.v[671] <= 0.0);
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if s.b[1148] {
            s.store_scalar(671, 0.6);
        }

        s.b[1152] = (s.v[678] <= 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if s.b[1152] {
            s.store_scalar(678, 1.06);
        }

        s.b[1153] = (s.v[673] < 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if s.b[1153] {
            s.store_scalar(673, 0.0);
        }

        s.b[1154] = (s.v[677] < 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if s.b[1154] {
            s.store_scalar(677, 0.0);
        }

        s.b[1155] = (s.v[803] < (-s.v[153]));
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if s.b[1155] {
            s.store_scalar(803, 0.0);
        }

        s.b[1156] = (s.v[685] < 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_scalar(685, 0.0);
        }

        s.b[1157] = (s.v[687] < 0.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if s.b[1157] {
            s.store_scalar(687, 0.0);
        }

        s.b[1158] = ((p.p61 != 0.0) && (s.v[689] < 0.2));
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if s.b[1158] {
            s.store_scalar(689, 0.2);
        }

        s.b[1159] = ((p.p61 != 0.0) && (s.v[689] > 1.2));
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if s.b[1159] {
            s.store_scalar(689, 1.2);
        }

        s.b[1160] = (s.v[695] < 2.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if s.b[1160] {
            s.store_scalar(695, 2.0);
        }

        s.b[1161] = (s.v[697] < 2.0);
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if s.b[1161] {
            s.store_scalar(697, 2.0);
        }

        s.b[1162] = (s.v[704] < 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if s.b[1162] {
            s.store_scalar(704, 0.03);
        }

        s.b[1163] = (s.v[807] < 0.0);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if s.b[1163] {
            s.store_scalar(807, 0.0);
        }

        s.b[1164] = (s.v[811] < 0.0);
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if s.b[1164] {
            s.store_scalar(811, 0.0);
        }

        s.b[1165] = (s.v[812] < 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if s.b[1165] {
            s.store_scalar(812, 0.0);
        }

        s.b[1166] = (s.v[814] < 0.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if s.b[1166] {
            s.store_scalar(814, 0.0);
        }

        s.b[1167] = (s.v[707] < 0.0);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if s.b[1167] {
            s.store_scalar(707, 0.0);
        }

        s.b[1168] = (s.v[709] < 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if s.b[1168] {
            s.store_scalar(709, 0.0);
        }

        s.b[1169] = (s.v[853] < 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if s.b[1169] {
            s.store_scalar(853, 0.0);
        }

        s.b[1170] = (s.v[852] < 0.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if s.b[1170] {
            s.store_scalar(852, 0.0);
        }

        s.b[1171] = (s.v[712] < 0.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if s.b[1171] {
            s.store_scalar(712, 0.0);
        }

        s.b[1172] = (s.v[711] < 0.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if s.b[1172] {
            s.store_scalar(711, 0.0);
        }

        s.b[1175] = (p.p66 != 0.0);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        s.b[1178] = (s.v[706] < 0.0);
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if (s.b[1175] && s.b[1178]) {
            s.store_scalar(706, 0.0);
        }

        s.b[1179] = (s.v[815] < 0.0);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1175] && s.b[1179]) {
            s.store_scalar(815, 0.0);
        }

        s.b[1180] = (s.v[816] < 0.0);
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if (s.b[1175] && s.b[1180]) {
            s.store_scalar(816, 0.0);
        }

        s.b[1181] = (s.v[818] < 0.0);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1175] && s.b[1181]) {
            s.store_scalar(818, 0.0);
        }

        s.b[1183] = (s.v[719] <= 0.0);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if s.b[1183] {
            s.store_scalar(719, 1.06);
        }

        s.b[1184] = (s.v[790] < 2.0);
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if s.b[1184] {
            s.store_scalar(790, 2.0);
        }

        s.b[1185] = (p.p66 != 0.0);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1186] = (s.v[791] < 2.0);
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if (s.b[1185] && s.b[1186]) {
            s.store_scalar(791, 2.0);
        }

        s.b[1187] = (s.v[700] < 0.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if s.b[1187] {
            s.store_scalar(700, 0.0);
        }

        s.b[1188] = (s.v[749] < 0.0);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if s.b[1188] {
            s.store_scalar(749, 0.0);
        }

        s.b[1189] = (s.v[763] < 0.0);
        s.v[1189] = if s.b[1189] { 1.0 } else { 0.0 };

        if s.b[1189] {
            s.store_scalar(763, 0.0);
        }

        s.b[1190] = (p.p69 != 0.0);
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        s.b[1191] = (s.v[726] <= 0.0);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        if (s.b[1190] && s.b[1191]) {
            s.store_scalar(726, 3.0);
        }

        s.b[1192] = (s.v[731] <= 0.0);
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        if (s.b[1190] && s.b[1192]) {
            s.store_scalar(731, 1.0);
        }

        s.b[1193] = (p.p68 != 0.0);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        s.b[1194] = (s.v[742] <= 0.0);
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if (s.b[1193] && s.b[1194]) {
            s.store_scalar(742, 1.0);
        }

        s.b[1195] = (s.v[736] <= 0.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if (s.b[1193] && s.b[1195]) {
            s.store_scalar(736, 1.0);
        }

        s.b[1213] = (s.v[648] < 0.0);
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if s.b[1213] {
            s.store_scalar(648, 0.0);
        }

        s.b[1214] = (s.v[649] < 0.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if s.b[1214] {
            s.store_scalar(649, 0.0);
        }

        s.b[1215] = (s.v[643] < 0.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if s.b[1215] {
            s.store_scalar(643, 0.0);
        }

        s.b[1216] = (s.v[642] < 0.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if s.b[1216] {
            s.store_scalar(642, 0.0);
        }

        s.b[1217] = (s.v[650] < 0.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if s.b[1217] {
            s.store_scalar(650, 0.0);
        }

        s.b[1218] = (s.v[651] <= 0.02);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if s.b[1218] {
            s.store_scalar(651, 0.02);
        }

        s.b[1219] = (s.v[652] <= 0.02);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if s.b[1219] {
            s.store_scalar(652, 0.02);
        }

        s.b[1220] = (s.v[653] <= 0.02);
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if s.b[1220] {
            s.store_scalar(653, 0.02);
        }

        s.b[1221] = (s.v[446] < (-p.p4));
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if s.b[1221] {
            s.store_scalar(446, 0.0);
        }

        s.b[1222] = (p.p57 == 1.0);
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        s.b[1223] = ((s.v[882] < 1.0) || (s.v[882] > 3.0));
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1223]) {
            s.store_scalar(882, 2.0);
        }

        s.b[1224] = ((s.v[883] < 1.0) || (s.v[883] > 3.0));
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1224]) {
            s.store_scalar(883, 2.6);
        }

        s.b[1225] = ((s.v[884] < 1.0) || (s.v[884] > 3.0));
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1225]) {
            s.store_scalar(884, 2.6);
        }

        s.b[1226] = (s.v[885] < 0.0);
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1226]) {
            s.store_scalar(885, 14.0);
        }

        s.b[1227] = (s.v[886] < 0.0);
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1227]) {
            s.store_scalar(886, 24.0);
        }

        s.b[1228] = (s.v[887] < 0.0);
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1228]) {
            s.store_scalar(887, 24.0);
        }

        s.b[1229] = (s.v[888] < 0.0);
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1229]) {
            s.store_scalar(888, 0.139);
        }

        s.b[1230] = (s.v[889] < 0.0);
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1230]) {
            s.store_scalar(889, 2.0);
        }

        s.b[1231] = (s.v[890] < 0.0);
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1231]) {
            s.store_scalar(890, 11.2);
        }

        s.b[1232] = (s.v[891] < 0.0);
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1232]) {
            s.store_scalar(891, 8.02);
        }

        s.b[1233] = (s.v[892] < 0.0);
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        if (s.b[1222] && s.b[1233]) {
            s.store_scalar(892, 6.18);
        }

        s.b[1234] = ((p.p74 != 0.0) && (p.p1791 > 0.0));
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        s.b[1235] = (p.p1795 != 0.0);
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if (s.b[1234] && s.b[1235]) {
            s.store_scalar(169, (p.p1793 * ((p.p59) as f64).powf(p.p1795)));
        }

        if (s.b[1234] && (!s.b[1235])) {
            s.store_scalar(169, p.p1793);
        }

        s.b[1236] = (p.p1794 != 0.0);
        s.v[1236] = if s.b[1236] { 1.0 } else { 0.0 };

        if (s.b[1234] && s.b[1236]) {
            s.store_scalar(170, ((p.p1797 * p.p4) * ((s.v[115]) as f64).powf(p.p1794)));
        }

        if (s.b[1234] && (!s.b[1236])) {
            s.store_scalar(170, (p.p1797 * p.p4));
        }

        s.b[1237] = (p.p62 == 5.0);
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        s.b[1238] = (p.p1796 != 0.0);
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if ((s.b[1234] && s.b[1237]) && s.b[1238]) {
            s.store_scalar(171, (((p.p1798 * p.p59) * p.p43) * ((p.p56) as f64).powf(p.p1796)));
        }

        if ((s.b[1234] && s.b[1237]) && (!s.b[1238])) {
            s.store_scalar(171, ((p.p1798 * p.p59) * p.p43));
        }

        if (s.b[1234] && (!s.b[1237])) {
            s.store_scalar(171, 0.0);
        }

        if s.b[1234] {
            s.store_add_scaled_inputs3_indices(634, 169, p.p1792, 170, p.p1792, 171, p.p1792);
        }

        s.b[1241] = (p.p77 == 0.0);
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        if s.b[1241] {
            s.store_scalar(190, (p.p1078 * p.p18));
            s.store_scalar(191, (p.p1079 * p.p19));
        }

        s.b[1242] = (p.p1080 > 0.0);
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if ((!s.b[1241]) && s.b[1242]) {
            s.store_scalar(444, ((p.p4 * p.p92) + ((p.p3 + ((p.p4 - p.p3) * p.p1084)) * p.p1080)));
        }

        if ((!s.b[1241]) && (!s.b[1242])) {
            s.store_scalar(444, (p.p4 * (1e-9_f64).max((p.p92 + p.p1080))));
        }

        if (!s.b[1241]) {
            s.store_offset(445, 446, p.p4);
        }

        s.b[1243] = param_given[1083];
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if ((!s.b[1241]) && s.b[1243]) {
            s.store_scalar(431, p.p1083);
        }

        if ((!s.b[1241]) && (!s.b[1243])) {
            s.store_scalar(429, (if (p.p60 == 1.0) { 1417.0 } else { 470.5 }));
        }

        s.b[1244] = (p.p60 == 1.0);
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if (((!s.b[1241]) && (!s.b[1243])) && s.b[1244]) {
            s.store_scalar(168, (((p.p97 / 9.68e22)) as f64).powf(0.68));
            s.store_scalar(169, (3.43e26 / p.p97));
            s.store_scaled_sub_ad(430, A::offset(A::div_scaled_offset_numerator(s.ad_value(429), 1.0, (-52.2), A::offset(s.ad_value(168), 1.0), 1.0), 52.2), A::div_scalar_offset_denominator(43.4, A::square(s.ad_value(169)), 1.0, 1.0), 0.0001);
        }

        if (((!s.b[1241]) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_scalar(168, (((p.p97 / 2.23e22)) as f64).powf(0.719));
            s.store_scalar(169, (6.1e26 / p.p97));
            s.store_scaled_sub_ad(430, A::offset(A::div_scaled_offset_numerator(s.ad_value(429), 1.0, (-44.9), A::offset(s.ad_value(168), 1.0), 1.0), 44.9), A::div_scalar_offset_denominator(29.0, A::square(s.ad_value(169)), 1.0, 1.0), 0.0001);
        }

        if ((!s.b[1241]) && (!s.b[1243])) {
            s.store_div_from_scalar_scaled_input(431, 1.0, 430, (1.60219e-19 * p.p97));
        }

        if (!s.b[1241]) {
            s.store_scalar(433, ((55.0 * 3.141592653589793) / 180.0));
            s.store_min_with_scalar(432, 444, (1e-18_f64).max((p.p3 * (p.p92 + (0.0_f64).min(p.p1080)))));
            s.store_scaled_mul_ad(434, A::div(s.ad_value(431), A::tan(s.ad_value(433))), A::add_scaled_inputs3(A::div_from_scalar(1.0, A::sqrt(s.ad_value(432))), 1.0, A::div_from_scalar(2.0, A::sqrt(s.ad_value(444))), (-1.0), A::sqrt(A::div(s.ad_value(432), A::square(s.ad_value(444)))), 1.0), 1.0 / ((((3.141592653589793) as f64).sqrt() * p.p5)));
            s.store_offset_scaled(436, 444, p.p5, p.p1092);
            s.store_offset_scaled(437, 445, p.p5, p.p1093);
            s.store_sqrt_ad(435, A::div_scaled_inputs(s.ad_value(436), p.p1082, A::mul(s.ad_value(431), s.ad_value(437)), 1.0));
            s.store_div_from_scalar(438, p.p20, 435);
            s.store_limited_exp_scaled_input(168, 438, 2.0);
        }

        s.b[1245] = (p.p1086 == 1.0);
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        if ((!s.b[1241]) && s.b[1245]) {
            s.store_scaled_mul(439, 431, 435, 1.0 / (p.p1082));
            s.store_mul_offset_rhs(169, 168, 439, 1.0);
            s.store_sub_offset_lhs(170, 169, 1.0, 439);
            s.store_add_offset_lhs(171, 169, (-1.0), 439);
        }

        if ((!s.b[1241]) && (!s.b[1245])) {
            s.store_offset(170, 168, 1.0);
            s.store_offset(171, 168, (-1.0));
        }

        if (!s.b[1241]) {
            s.store_div_scaled_product3_by_product(440, s.ad_value(431), s.ad_value(435), s.ad_value(170), 1.0, s.ad_value(436), s.ad_value(171), 1.0);
        }

        s.b[1246] = (p.p1080 < (-1e-10));
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if ((!s.b[1241]) && s.b[1246]) {
            s.store_scalar(441, (p.p1082 / (((-p.p1080) * p.p3) * p.p5)));
            s.store_div_scaled_product_mixed_aia(442, A::add(s.ad_value(440), s.ad_value(434)), 441, 1.0, A::add_scaled_inputs3(s.ad_value(440), 1.0, s.ad_value(434), 1.0, s.ad_value(441), 1.0), 1.0);
        }

        if ((!s.b[1241]) && (!s.b[1246])) {
            s.store_add(442, 440, 434);
        }

        if (!s.b[1241]) {
            s.store_scale(443, 442, (1.0 / (p.p59) * (0.0_f64).max(((((p.p1094 + (p.p1095 * p.p3)) + (p.p1096 * p.p4)) + (p.p1097 * p.p20)) + (p.p1098 * p.p1080)))));
            s.copy_ad(190, 443);
            s.copy_ad(191, 443);
        }

        s.b[1247] = (p.p64 == 0.0);
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        s.b[1248] = (s.v[190] < p.p151);
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if (s.b[1247] && s.b[1248]) {
            s.store_scalar(190, 0.0);
        }

        s.b[1249] = (s.v[191] < p.p151);
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if (s.b[1247] && s.b[1249]) {
            s.store_scalar(191, 0.0);
        }

        s.b[1250] = (s.v[190] <= p.p151);
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if ((!s.b[1247]) && s.b[1250]) {
            s.store_scalar(190, p.p151);
        }

        s.b[1251] = (s.v[191] <= p.p151);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if ((!s.b[1247]) && s.b[1251]) {
            s.store_scalar(191, p.p151);
        }

        s.b[1252] = (p.p78 != 1.0);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        s.b[1253] = param_given[1542];
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if (s.b[1252] && s.b[1253]) {
            s.store_scalar(646, p.p1542);
        }

        s.b[1254] = (param_given[85] && (p.p85 > 0.0));
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if ((s.b[1252] && (!s.b[1253])) && s.b[1254]) {
            s.store_max_from_scalar_ad(646, 0.0, A::sub_scaled_inputs(s.ad_value(163), p.p85, s.ad_value(648), 1.0));
        }

        s.b[1255] = (p.p78 == 3.0);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && s.b[1255]) {
            s.store_scale(646, 163, (0.3 * p.p43));
        }

        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && (!s.b[1255])) {
            s.store_scale(646, 163, (0.3 * p.p3));
        }

        s.b[1256] = param_given[1543];
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if (s.b[1252] && s.b[1256]) {
            s.store_scalar(647, p.p1543);
        }

        s.b[1257] = (param_given[85] && (p.p85 > 0.0));
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if ((s.b[1252] && (!s.b[1256])) && s.b[1257]) {
            s.store_max_from_scalar_ad(647, 0.0, A::sub_scaled_inputs(s.ad_value(163), p.p85, s.ad_value(649), 1.0));
        }

        s.b[1258] = (p.p78 == 3.0);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && s.b[1258]) {
            s.store_scale(647, 163, (0.3 * p.p43));
        }

        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && (!s.b[1258])) {
            s.store_scale(647, 163, (0.3 * p.p3));
        }

        s.b[1259] = (p.p78 == 2.0);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if s.b[1259] {
            s.store_scalar(447, (p.p1089 + p.p1090));
            s.store_scalar(449, (0.5 * (p.p4 - p.p3)));
            s.store_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p.p90)));
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1259] {
            s.store_scalar(450, (0.0_f64).max((p.p1080 + p.p1081)));
        }

        s.b[1260] = (p.p1090 > 0.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if (s.b[1259] && s.b[1260]) {
            s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38) { ((((1e-7 * p.p1088) / (3.9 * p.p1087))) as f64).ln() } else { 0.0 }) })));
        }

        if (s.b[1259] && s.b[1260]) {
            s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p.p1087))));
            s.store_scaled_add(451, 168, 169, (p.p3 + ((p.p4 - p.p3) * p.p1084)));
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(447), 0.2, (p.p90 * 0.2), s.ad_value(450), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(447), p.p90), s.ad_value(450)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(450), A::offset(s.ad_value(447), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1261] = (s.v[933] > 80.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if ((s.b[1259] && (!s.b[1260])) && s.b[1261]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1259] && (!s.b[1260])) && (!s.b[1261])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(450), 1.0, s.ad_value(447), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(447), 1.0, p.p90, s.ad_value(450), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scaled_add(938, 934, 937, p.p3);
            s.store_div(930, 928, 447);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(447), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(447)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(447), s.ad_value(930), 1.0), 447);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_mul_ad(946, A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p3, 933, ((-0.5) * p.p3), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p3));
            s.store_add(451, 938, 947);
        }

        s.b[1262] = (p.p1090 > 0.0);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if (s.b[1259] && s.b[1262]) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1263] = (s.v[933] > 80.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if ((s.b[1259] && s.b[1262]) && s.b[1263]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1259] && s.b[1262]) && (!s.b[1263])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scaled_add(938, 934, 937, p.p92);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_mul_ad(946, A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), s.ad_value(944), 1.0), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p92, 933, ((-0.5) * p.p92), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p92));
            s.store_add(452, 938, 947);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1264] = (s.v[933] > 80.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((s.b[1259] && (!s.b[1262])) && s.b[1264]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1259] && (!s.b[1262])) && (!s.b[1264])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scaled_add(938, 934, 937, p.p92);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && (!s.b[1262])) {
            s.store_mul_ad(946, A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p92, 933, ((-0.5) * p.p92), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p92));
            s.store_add(452, 938, 947);
        }

        s.b[1265] = (p.p1090 > 0.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (s.b[1259] && s.b[1265]) {
            s.store_scalar(454, 0.0);
        }

        s.b[1266] = (p.p1080 > 0.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((s.b[1259] && (!s.b[1265])) && s.b[1266]) {
            s.store_scalar(454, ((p.p4 - p.p3) * ((p.p1080 * p.p1084) + p.p1081)));
        }

        if ((s.b[1259] && (!s.b[1265])) && (!s.b[1266])) {
            s.store_scale(454, 450, (p.p4 - p.p3));
        }

        if s.b[1259] {
            s.store_offset_scaled(455, 454, ((p.p5) * ((s.v[144] * 1.0 / (p.p1087)))), ((((p.p1092) + (p.p1091))) * ((s.v[144] * 1.0 / (p.p1087)))));
            s.store_add_scaled_inputs3_indices(453, 455, p.p59, 451, (p.p5 * p.p59), 452, ((p.p1103 * (p.p5 * 2.0)) * p.p59));
            s.store_scale(453, 453, (0.0_f64).max((((p.p1099 + (p.p1100 * p.p3)) + (p.p1101 * p.p4)) + (p.p1102 * p.p20))));
        }

        s.b[1267] = (p.p78 == 3.0);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if s.b[1267] {
            s.store_scalar(447, (p.p1089 + p.p1090));
            s.store_scalar(449, (0.5 * (p.p4 - p.p43)));
            s.store_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p.p90)));
            s.store_scalar(450, (0.0_f64).max((p.p1080 + p.p1081)));
            s.store_scalar(1031, (0.5 * p.p41));
        }

        s.b[1268] = (p.p1090 > 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1268]) {
            s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38) { ((((1e-7 * p.p1088) / (3.9 * p.p1087))) as f64).ln() } else { 0.0 }) })));
        }

        if (s.b[1267] && s.b[1268]) {
            s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p.p1087))));
            s.store_scaled_add(1034, 168, 169, (p.p43 + ((p.p4 - p.p43) * p.p1084)));
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(447), 0.2, (p.p90 * 0.2), s.ad_value(450), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(447), p.p90), s.ad_value(450)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(450), A::offset(s.ad_value(447), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1269] = (s.v[933] > 80.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if ((s.b[1267] && (!s.b[1268])) && s.b[1269]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1267] && (!s.b[1268])) && (!s.b[1269])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(450), 1.0, s.ad_value(447), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(447), 1.0, p.p90, s.ad_value(450), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scaled_add(938, 934, 937, p.p43);
            s.store_div(930, 928, 447);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(447), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(447)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(447), s.ad_value(930), 1.0), 447);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_mul_ad(946, A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p43, 933, ((-0.5) * p.p43), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p43));
            s.store_add(1034, 938, 947);
        }

        if s.b[1267] {
            s.store_offset_div_from_scalar_ad(925, (0.2 * (p.p1089 + p.p90)), s.ad_value(1031), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub_from_scalar((p.p1089 + p.p90), s.ad_value(1031)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_with_scalar(929, 1031, (p.p1089 + p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1270] = (s.v[933] > 80.0);
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1270]) {
            s.copy_ad(934, 932);
        }

        if (s.b[1267] && (!s.b[1270])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if s.b[1267] {
            s.store_scale_ad(935, A::min(A::scale(s.ad_value(1031), 1.0 / ((p.p1089 + p.p90))), A::div_from_scalar((p.p1089 + p.p90), s.ad_value(1031))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if s.b[1267] {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if s.b[1267] {
            s.store_scaled_add(938, 934, 937, p.p43);
            s.store_scale(930, 928, 1.0 / (p.p1089));
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_offset_add_scaled_inputs_mixed_ai(940, A::offset(A::mul(A::sqrt(A::scale_offset(s.ad_value(930), (p.p1089 * p.p1089), (((p.p1089 * p.p1089)) + (((p.p90 * p.p90) + ((2.0 * p.p1089) * p.p90)))))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, 930, p.p1089, p.p1089);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if s.b[1267] {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if s.b[1267] {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if s.b[1267] {
            s.store_mul_ad(946, A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1267] {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p43, 933, ((-0.5) * p.p43), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p43));
            s.store_add(1035, 938, 947);
        }

        s.b[1271] = (p.p1090 > 0.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1271]) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1272] = (s.v[933] > 80.0);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if ((s.b[1267] && s.b[1271]) && s.b[1272]) {
            s.copy_ad(934, 932);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1267] && s.b[1271]) && (!s.b[1272])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scaled_add(938, 934, 937, p.p40);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_mul_ad(946, A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), s.ad_value(944), 1.0), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p40, 933, ((-0.5) * p.p40), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p40));
            s.store_add(1036, 938, 947);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1273] = (s.v[933] > 80.0);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if ((s.b[1267] && (!s.b[1271])) && s.b[1273]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1267] && (!s.b[1271])) && (!s.b[1273])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scaled_add(938, 934, 937, p.p40);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_mul_ad(946, A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p40, 933, ((-0.5) * p.p40), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p40));
            s.store_add(1036, 938, 947);
        }

        if s.b[1267] {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1274] = (s.v[933] > 80.0);
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1274]) {
            s.copy_ad(934, 932);
        }

        if (s.b[1267] && (!s.b[1274])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if s.b[1267] {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if s.b[1267] {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if s.b[1267] {
            s.store_scaled_add(938, 934, 937, p.p40);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if s.b[1267] {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if s.b[1267] {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if s.b[1267] {
            s.store_mul_ad(946, A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1267] {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p40, 933, ((-0.5) * p.p40), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p40));
            s.store_add(1037, 938, 947);
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1275] = (s.v[933] > 80.0);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1275]) {
            s.copy_ad(934, 932);
        }

        if (s.b[1267] && (!s.b[1275])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if s.b[1267] {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if s.b[1267] {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if s.b[1267] {
            s.store_scaled_add(938, 934, 937, p.p42);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if s.b[1267] {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if s.b[1267] {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if s.b[1267] {
            s.store_mul_ad(946, A::div_scaled_inputs(s.ad_value(943), ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), s.ad_value(944), 1.0), {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1267] {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p42, 933, ((-0.5) * p.p42), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p42));
            s.store_add(1038, 938, 947);
        }

        s.b[1276] = (p.p1090 > 0.0);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        if (s.b[1267] && s.b[1276]) {
            s.store_scalar(1032, 0.0);
        }

        s.b[1277] = (p.p1080 > 0.0);
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        if ((s.b[1267] && (!s.b[1276])) && s.b[1277]) {
            s.store_scalar(1032, ((p.p4 - p.p43) * ((p.p1080 * p.p1084) + p.p1081)));
        }

        if ((s.b[1267] && (!s.b[1276])) && (!s.b[1277])) {
            s.store_scale(1032, 450, (p.p4 - p.p43));
        }

        if s.b[1267] {
            s.store_scale(1033, 1031, (p.p4 - p.p43));
            s.store_scaled_offset_ad(455, A::add_scaled_inputs(s.ad_value(1032), p.p5, s.ad_value(1033), ((2.0 * p.p56) * p.p5)), ((p.p1092) + (p.p1091)), (s.v[144] * 1.0 / (p.p1087)));
            s.store_scaled_add_ad(453, A::add_scaled_inputs3(s.ad_value(455), 1.0, s.ad_value(1034), p.p5, s.ad_value(1035), ((2.0 * p.p56) * p.p5)), A::add_scaled_inputs3(s.ad_value(1036), (p.p1103 * (p.p5 * 2.0)), s.ad_value(1037), ((p.p56 - 1.0) * (p.p1103 * (p.p5 * 2.0))), s.ad_value(1038), (p.p1103 * (p.p5 * 2.0))), p.p59);
            s.store_scale(453, 453, (0.0_f64).max((((p.p1099 + (p.p1100 * p.p43)) + (p.p1101 * p.p4)) + (p.p1102 * p.p20))));
        }

        s.v[168] = (p.p1583 * (if (!((1.0 + (p.p92 / p.p91)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p92 / p.p91)) > 1e-38) { (((1.0 + (p.p92 / p.p91))) as f64).ln() } else { 0.0 }) }));

        s.v[515] = ((s.v[165] * p.p7) + (s.v[168] * (0.0_f64).max((p.p9 - (p.p4 * s.v[115])))));

        s.v[516] = ((s.v[165] * p.p8) + (s.v[168] * (0.0_f64).max((p.p10 - (p.p4 * s.v[115])))));

        s.b[1278] = (p.p62 != 5.0);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if s.b[1278] {
            s.store_scale(517, 149, (((p.p1544 * p.p59) * p.p6) + (p.p1545 * s.v[115])));
        }

        if (!s.b[1278]) {
            s.store_mul_scale_offset_rhs(517, 149, 161, ((p.p1546) * (s.v[115])), ((((p.p1545) * (s.v[115]))) + (((p.p1544 * p.p59) * p.p6))));
        }

        s.v[420] = (1e-8 / (s.v[145] * p.p89));

        s.store_div_from_scalar_scaled_ad(189, 1.0, A::pow(A::scale(s.ad_value(158), 1000000.0), s.ad_value(713)), s.v[115]);

        s.v[578] = (((((s.v[145] * p.p89) * 0.5) * p.p3)) as f64).sqrt();

        s.store_sqrt_ad(351, A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(894), s.v[143], s.ad_value(893), 1.0), A::div_scaled_product_by_product(s.ad_value(894), s.ad_value(893), 1.0, s.ad_value(895), s.ad_value(895), (2.0 * s.v[143])), 1.0));

        s.b[1279] = (!param_given[172]);
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if s.b[1279] {
            s.store_offset_div_scaled_product(360, s.ad_value(670), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);
        }

        s.b[1280] = (s.v[360] < 40.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (s.b[1279] && s.b[1280]) {
            s.store_div_from_scalar_offset_ad(361, 0.5, A::cosh(s.ad_value(360)), (-1.0));
        }

        if (s.b[1279] && (!s.b[1280])) {
            s.store_limited_exp_neg_input(361, 360);
        }

        if (!s.b[1279]) {
            s.store_scalar(361, p.p172);
        }

        s.b[1281] = (!param_given[174]);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if s.b[1281] {
            s.store_offset_div_scaled_product(360, s.ad_value(671), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);
        }

        s.b[1282] = (s.v[360] < 40.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if (s.b[1281] && s.b[1282]) {
            s.store_div_from_scalar_offset_ad(362, 0.5, A::cosh(s.ad_value(360)), (-1.0));
        }

        if (s.b[1281] && (!s.b[1282])) {
            s.store_limited_exp_neg_input(362, 360);
        }

        if (!s.b[1281]) {
            s.store_scalar(362, p.p174);
        }

        s.b[1283] = (!param_given[173]);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if s.b[1283] {
            s.store_offset_div_scaled_product(360, s.ad_value(678), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);
        }

        s.b[1284] = (s.v[360] < 40.0);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if (s.b[1283] && s.b[1284]) {
            s.store_div_from_scalar_offset_ad(363, 0.5, A::cosh(s.ad_value(360)), (-1.0));
        }

        if (s.b[1283] && (!s.b[1284])) {
            s.store_limited_exp_neg_input(363, 360);
        }

        if (!s.b[1283]) {
            s.store_scalar(363, p.p173);
        }

        s.store_offset_sqrt_ad(364, A::offset(A::div(s.ad_value(803), s.ad_value(153)), 1.0), (-1.0));

        s.store_offset_div_scaled_product(360, s.ad_value(678), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);

        s.b[1285] = (s.v[360] < 40.0);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if s.b[1285] {
            s.store_div_from_scalar_ad(365, 1.0, A::max_with_scalar(A::scale_offset(A::cosh(s.ad_value(360)), p.p171, (((((-2.0)) * (p.p171))) + (1.0))), 1e-6));
        }

        if (!s.b[1285]) {
            let assign13360_ad_e17673: A = A::limited_exp_scaled_input(s.ad_value(360), -1.0);
            s.store_div_ad(365, assign13360_ad_e17673, A::max_with_scalar(A::offset(assign13360_ad_e17673, p.p171), 1e-6));
        }

        s.store_div_scaled_product_indices(396, 640, 894, 1.60219e-19, 893, 1.0);

        s.b[1286] = (p.p60 == 1.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if s.b[1286] {
            s.store_scalar(485, 745669000000.0);
        }

        if (!s.b[1286]) {
            s.store_scalar(485, 1166450000000.0);
        }

        s.v[168] = (p.p1109 * p.p1109);

        s.store_scale(169, 742, p.p1109);

        s.store_square(170, 169);

        s.b[1287] = (p.p1717 < (-273.15));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if s.b[1287] {
            s.store_scalar(228, 300.15);
        }

        if (!s.b[1287]) {
            s.store_scalar(228, (p.p1717 + 273.15));
        }

        s.b[1288] = (p.p57 == 1.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if s.b[1288] {
            s.store_add_ad_lhs(960, A::scale_offset(s.ad_value(882), (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p.p1806) * (1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 882);
        }

        if s.b[1288] {
            s.store_add_ad_lhs(961, A::scale_offset(s.ad_value(883), (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p.p1813) * (1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 883);
        }

        if s.b[1288] {
            s.store_add_ad_lhs(962, A::scale_offset(s.ad_value(884), (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p.p1820) * (1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 884);
        }

        if s.b[1288] {
            s.store_scaled_add_sqrt_square_offset_ad(963, A::offset(s.ad_value(885), ((-p.p1847) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);
        }

        if s.b[1288] {
            s.store_scaled_add_sqrt_square_offset_ad(964, A::offset(s.ad_value(886), ((-p.p1848) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);
        }

        if s.b[1288] {
            s.store_scaled_add_sqrt_square_offset_ad(965, A::offset(s.ad_value(887), ((-p.p1849) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);
        }

        if s.b[1288] {
            let assign13590_ad_e18065: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(966, 960, ((0.5 * 1.001) * 0.5), assign13590_ad_e18065, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), 0.5, assign13590_ad_e18065, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13600_ad_e18185: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(969, 960, ((0.5 * 1.001) * 0.5), assign13600_ad_e18185, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), 0.5, assign13600_ad_e18185, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13610_ad_e18305: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(967, 961, ((0.5 * 1.001) * 0.5), assign13610_ad_e18305, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), 0.5, assign13610_ad_e18305, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13620_ad_e18425: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(970, 961, ((0.5 * 1.001) * 0.5), assign13620_ad_e18425, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), 0.5, assign13620_ad_e18425, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13630_ad_e18545: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(968, 962, ((0.5 * 1.001) * 0.5), assign13630_ad_e18545, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), 0.5, assign13630_ad_e18545, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13640_ad_e18665: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(971, 962, ((0.5 * 1.001) * 0.5), assign13640_ad_e18665, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), 0.5, assign13640_ad_e18665, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            s.store_mul_ad(976, A::pow(s.ad_value(158), s.ad_value(966)), A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(969)));
            s.store_div(979, 976, 893);
            s.store_mul_ad(977, A::pow(s.ad_value(158), s.ad_value(967)), A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(970)));
            s.store_div(980, 977, 893);
            s.store_mul_ad(978, A::pow(s.ad_value(158), s.ad_value(968)), A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(971)));
            s.store_div(981, 978, 893);
        }

        if s.b[1288] {
            s.store_scalar(982, (0.5 * (((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p.p40 * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) + 0.5) + ((((((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p.p40 * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5) * ((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p.p40 * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5)) + ((0.25 * 0.003) * 0.003))) as f64).sqrt())));
        }

        if s.b[1288] {
            s.store_add_div_lhs(983, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(982), A::sub(s.ad_value(960), s.ad_value(882))), A::sub_from_scalar(p.p1806, s.ad_value(882)), 982);
            s.store_div_from_scalar_offset_ad(984, 1.0, A::limited_exp_scaled_input(A::offset(s.ad_value(983), (-0.999)), 1.0 / (0.0001)), 1.0);
            s.store_scalar(1013, (((((0.5 * p.p40) * p.p40) * 1e18) - ((1.5 * p.p40) * 1000000000.0)) + 2.0));
            s.store_offset_sub_scaled_inputs(1014, A::offset(s.ad_value(1013), 4.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(1013), (-4.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));
        }

        if s.b[1288] {
            let assign13760_ad_e18948: A = A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893);
            let assign13760_ad_e19005: A = A::sqrt_square_offset(A::scale_offset(assign13760_ad_e18948, ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + ((-18100.0)))), ((0.25 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs3_offset(974, assign13760_ad_e18948, ((0.5 * ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))) * 0.5), assign13760_ad_e19005, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(assign13760_ad_e18948, ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + (18100.0))), 0.5, assign13760_ad_e19005, 0.5), (-924000.0)), ((0.25 * 9240.0) * 9240.0)), (-0.5), ((924000.0 + (0.5 * ((s.v[168]) + (18100.0)))) * 0.5), (0.25 * 9240.0));
        }

        if s.b[1288] {
            let assign13770_ad_e19176: A = A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894);
            let assign13770_ad_e19233: A = A::sqrt_square_offset(A::scale_offset(assign13770_ad_e19176, ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), ((0.25 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs3_offset(975, assign13770_ad_e19176, ((0.5 * ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))) * 0.5), assign13770_ad_e19233, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(assign13770_ad_e19176, ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), 0.5, assign13770_ad_e19233, 0.5), (-8.0)), ((0.25 * 0.01) * 0.01)), (-0.5), ((8.0 + (0.5 * 5.5)) * 0.5), (0.25 * 0.01));
        }

        if s.b[1288] {
            s.store_scalar(972, ((120.66 * ((4.0) as f64).powf(p.p1895)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1895)));
            s.store_scalar(973, ((2.0 * ((4.0) as f64).powf(p.p1896)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1896)));
            s.store_scalar(989, ((107.0 * ((4.0) as f64).powf(p.p1897)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1897)));
        }

        if s.b[1288] {
            let assign13810_ad_e19446: A = A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898);
            let assign13810_ad_e19485: A = A::sqrt_square_offset(A::scale_offset(assign13810_ad_e19446, 0.1, ((0.7) + ((-0.5)))), ((0.25 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs3_offset(990, assign13810_ad_e19446, ((0.5 * 0.1) * 0.5), assign13810_ad_e19485, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(assign13810_ad_e19446, 0.1, ((0.7) + (0.5))), 0.5, assign13810_ad_e19485, 0.5), (-1.0)), ((0.25 * 0.01) * 0.01)), (-0.5), ((1.0 + (0.5 * ((0.7) + (0.5)))) * 0.5), (0.25 * 0.01));
        }

        if s.b[1288] {
            s.store_scalar(991, ((103.0 * ((4.0) as f64).powf(p.p1899)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1899)));
            s.store_scalar(992, ((1.5 * ((4.0) as f64).powf(p.p1900)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1900)));
            s.store_scalar(993, ((833.0 * ((4.0) as f64).powf(p.p1901)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1901)));
            s.store_scalar(994, ((3.4 * ((4.0) as f64).powf(p.p1902)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1902)));
            s.store_div_ad_rhs(987, 974, A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(975), p.p1867)));
            s.store_div_ad_rhs(988, 972, A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(973), p.p1868)));
        }

        if s.b[1288] {
            let assign13880_ad_e19701: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(975), p.p1867));
            s.store_add_scaled_inputs4_mixed_iaia(985, 888, 0.5, A::div(s.ad_value(974), assign13880_ad_e19701), (p.p1865 * 0.5), 987, ((-p.p1865) * 0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(888), 1.0, A::div(s.ad_value(974), assign13880_ad_e19701), p.p1865, s.ad_value(987), (-p.p1865)), ((0.25 * 0.01) * 0.01)), 0.5);
        }

        if s.b[1288] {
            let assign13890_ad_e19766: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(973), p.p1868));
            s.store_add_scaled_inputs4_mixed_iaia(986, 889, 0.5, A::div(s.ad_value(972), assign13890_ad_e19766), (p.p1866 * 0.5), 988, ((-p.p1866) * 0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(889), 1.0, A::div(s.ad_value(972), assign13890_ad_e19766), p.p1866, s.ad_value(988), (-p.p1866)), ((0.25 * 0.01) * 0.01)), 0.5);
        }

        if s.b[1288] {
            let assign13900_ad_e19831: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(990), p.p1890));
            let assign13900_ad_e19835: A = A::powf(A::scale_offset(assign13900_ad_e19831, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(995, A::div(s.ad_value(989), assign13900_ad_e19835), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            let assign13910_ad_e19896: A = A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(990), p.p1890));
            let assign13910_ad_e19900: A = A::powf(A::scale_offset(assign13910_ad_e19896, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(996, A::div(s.ad_value(989), assign13910_ad_e19900), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            s.store_add_scaled_inputs3_indices(997, 890, 1.0, 995, p.p1887, 996, (-p.p1887));
        }

        if s.b[1288] {
            let assign13930_ad_e19971: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(992), p.p1891));
            let assign13930_ad_e19975: A = A::powf(A::scale_offset(assign13930_ad_e19971, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(998, A::div(s.ad_value(991), assign13930_ad_e19975), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            let assign13940_ad_e20036: A = A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(992), p.p1891));
            let assign13940_ad_e20040: A = A::powf(A::scale_offset(assign13940_ad_e20036, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(999, A::div(s.ad_value(991), assign13940_ad_e20040), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            s.store_add_scaled_inputs3_indices(1000, 891, 1.0, 998, p.p1888, 999, (-p.p1888));
        }

        if s.b[1288] {
            let assign13960_ad_e20111: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(994), p.p1892));
            let assign13960_ad_e20115: A = A::powf(A::scale_offset(assign13960_ad_e20111, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(1001, A::div(s.ad_value(993), assign13960_ad_e20115), ((0.25 * 0.1) * 0.1), 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[1288] {
            let assign13970_ad_e20176: A = A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(994), p.p1892));
            let assign13970_ad_e20180: A = A::powf(A::scale_offset(assign13970_ad_e20176, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(1002, A::div(s.ad_value(993), assign13970_ad_e20180), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            s.store_add_scaled_inputs3_indices(1003, 892, 1.0, 1001, p.p1889, 1002, (-p.p1889));
        }

        if s.b[1288] {
            let assign13990_ad_e20287: A = A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));
            s.store_mul_product3_rhs(1010, 979, s.ad_value(960), A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(960), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(960), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powf(A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8.0), 7.5893e-7, A::powf(assign13990_ad_e20287, 6.0), 6.9583e-5, A::powf(assign13990_ad_e20287, 5.0), (-0.0006583)), 1.0, A::powf(assign13990_ad_e20287, 4.0), 0.0065), 1.0, A::powf(assign13990_ad_e20287, 3.0), 0.026), 1.0, A::powf(assign13990_ad_e20287, 2.0), 0.1371), A::scale_offset(s.ad_value(960), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(997), 1000000.0), s.ad_value(960)), (1.0 / (2.0) * 1.60219e-19));
        }

        if s.b[1288] {
            let assign14000_ad_e20421: A = A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));
            s.store_mul_product3_rhs(1011, 980, s.ad_value(961), A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(961), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(961), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powf(A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8.0), 7.5893e-7, A::powf(assign14000_ad_e20421, 6.0), 6.9583e-5, A::powf(assign14000_ad_e20421, 5.0), (-0.0006583)), 1.0, A::powf(assign14000_ad_e20421, 4.0), 0.0065), 1.0, A::powf(assign14000_ad_e20421, 3.0), 0.026), 1.0, A::powf(assign14000_ad_e20421, 2.0), 0.1371), A::scale_offset(s.ad_value(961), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(1000), 1000000.0), s.ad_value(961)), (1.0 / (2.0) * 1.60219e-19));
        }

        if s.b[1288] {
            let assign14010_ad_e20555: A = A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));
            s.store_mul_product3_rhs(1012, 981, s.ad_value(962), A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(962), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(962), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powf(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8.0), 7.5893e-7, A::powf(assign14010_ad_e20555, 6.0), 6.9583e-5, A::powf(assign14010_ad_e20555, 5.0), (-0.0006583)), 1.0, A::powf(assign14010_ad_e20555, 4.0), 0.0065), 1.0, A::powf(assign14010_ad_e20555, 3.0), 0.026), 1.0, A::powf(assign14010_ad_e20555, 2.0), 0.1371), A::scale_offset(s.ad_value(962), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(1003), 1000000.0), s.ad_value(962)), (1.0 / (2.0) * 1.60219e-19));
        }

        s.b[1289] = (p.p58 == 1.0);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if s.b[1289] {
            s.store_offset_scaled(707, 707, 1.0 / (({ let limited_exp_arg = (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0)), (((((-p.p889)) * (1.0 / (({ let limited_exp_arg = (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0))))) + (p.p889)));
        }

        if s.b[1289] {
            s.store_offset(1024, 807, (((-p.p892)) + ((-((p.p893 * 1000000000.0) * p.p894)))));
        }

        if s.b[1289] {
            s.store_scaled_offset(1025, 1024, ((p.p40 * 1000000000.0) * p.p894), 1.0 / ((1.0 + { let limited_exp_arg = (((p.p895 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p896); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
        }

        if s.b[1289] {
            s.store_add_scaled_inputs3_offset_mixed_iia(807, 1025, 0.5, 807, 0.5, A::sqrt_square_offset(A::sub(A::offset(s.ad_value(1025), p.p892), A::offset(s.ad_value(807), 0.2)), ((0.25 * 0.6) * 0.6)), (-0.5), ((p.p892 + 0.2) * 0.5));
        }

        if s.b[1289] {
            s.store_add_scaled_inputs3_offset_indices(1026, 811, (-(370.0 * 1.0 / ((((p.p40 * 1000000000.0)) as f64).powf(p.p898)))), 811, (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), 811, 1.0, (((p.p897) * ((370.0 * 1.0 / ((((p.p40 * 1000000000.0)) as f64).powf(p.p898))))) + ((p.p897) * (1.0 / ((1.0 + { let limited_exp_arg = (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))));
        }

        if s.b[1289] {
            s.store_scaled_sub_offset_sqrt_square_offset(811, 1026, p.p897, (-p.p897), ((0.25 * 0.2) * 0.2), 0.5);
            s.store_scalar(1027, (p.p43 / (p.p43 + p.p40)));
            s.store_scalar(1028, ((((p.p905 * p.p40) * p.p40) * 1e18) - (p.p906 * 0.001)));
            s.store_scaled_add_ad_rhs(1029, 1028, A::powf(A::offset(A::square(s.ad_value(1028)), ((((((4.0 * p.p906) * 0.001) * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)), 0.5), 1.0 / (((((2.0 * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)));
            s.store_scaled_sub_offset_sqrt_square_offset_ad(1030, A::div_scalar_offset_denominator(0.0001, s.ad_value(1029), (((-0.8208)) + ((-(p.p907 * 1e-5)))), 1.0), 1.0, (-1.0), ((0.25 * 0.06) * 0.06), 0.5);
            s.store_mul_ad_product_lhs(704, s.ad_value(704), A::add(s.ad_value(1027), A::scale_offset(s.ad_value(1027), (-p.p904), p.p904)), 1030);
            s.store_add_ad_lhs(812, A::scale_offset(s.ad_value(812), (-(((0.5 * (((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) + ((((((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) * ((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p.p903)), ((p.p901) * ((((0.5 * (((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) + ((((((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) * ((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p.p903)))), 812);
        }

        s.b[1290] = ((p.p74 != 0.0) && (p.p1791 > 0.0));
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if s.b[1290] {
            s.store_offset_voltage(116, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p22)));
        }

        if (!s.b[1290]) {
            s.store_scalar(116, (ctx_temp + p.p22));
        }

        s.store_div(229, 116, 228);

        s.store_offset(230, 229, (-1.0));

        s.store_sub(232, 116, 228);

        s.store_scale(179, 116, 8.617087e-5);

        s.store_scale(180, 228, 8.617087e-5);

        s.v[121] = p.p1786;

        s.b[1291] = (p.p80 != 0.0);
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if s.b[1291] {
            s.store_scaled_add_offset_sqrt_square_offset(119, 116, s.v[121], (-s.v[121]), ((0.25 * p.p1788) * p.p1788), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(120, A::scaled_offset(s.ad_value(116), (-p.p1787), (-p.p1790)), ((0.25 * p.p1789) * p.p1789), 0.5);
        }

        s.b[1292] = (p.p80 == 1.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if (s.b[1291] && s.b[1292]) {
            s.store_scaled_add_offset_sqrt_square_offset(169, 228, s.v[121], (-s.v[121]), ((0.25 * p.p1788) * p.p1788), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(170, A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), ((0.25 * p.p1789) * p.p1789), 0.5);
        }

        s.b[1293] = (s.v[228] > s.v[121]);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if ((s.b[1291] && s.b[1292]) && s.b[1293]) {
            s.store_add_ad_lhs(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 228);
        }

        if ((s.b[1291] && s.b[1292]) && (!s.b[1293])) {
            s.store_add_scaled_inputs4_offset_indices(171, 119, 1.0, 120, 1.0, 169, -1.0, 170, -1.0, s.v[121]);
        }

        if (s.b[1291] && s.b[1292]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(118, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);
        }

        s.b[1294] = (s.v[121] > 210.0);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if ((s.b[1291] && (!s.b[1292])) && s.b[1294]) {
            s.store_scalar(121, 210.0);
        }

        if (s.b[1291] && (!s.b[1292])) {
            s.store_offset_scaled_ad(312, A::tanh_scaled_input(A::offset(s.ad_value(116), (-210.0)), 0.5), 0.5, 0.5);
            s.store_sub_from_scalar(313, 1.0, 312);
        }

        s.b[1295] = (s.v[228] > 210.0);
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if ((s.b[1291] && (!s.b[1292])) && s.b[1295]) {
            s.store_scaled_add_ad(169, A::offset(s.ad_value(121), 210.0), A::sqrt_square_offset(A::sub_from_scalar(210.0, s.ad_value(121)), ((0.25 * p.p1788) * p.p1788)), 0.5);
            s.store_scalar(170, (0.5 * (((-p.p1790) * (210.0 - p.p1787)) + ((((((-p.p1790) * (210.0 - p.p1787)) * ((-p.p1790) * (210.0 - p.p1787))) + ((0.25 * p.p1789) * p.p1789))) as f64).sqrt())));
            s.store_add_scaled_inputs4_offset_indices(171, 119, 1.0, 120, 1.0, 169, -1.0, 170, -1.0, 210.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(118, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);
        }

        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(169, 228, 0.5, 121, 0.5, 228, 121, ((0.25 * p.p1788) * p.p1788), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(170, A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), ((0.25 * p.p1789) * p.p1789), 0.5);
        }

        s.b[1296] = (s.v[228] > s.v[121]);
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && s.b[1296]) {
            s.store_add_ad_lhs(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 228);
        }

        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && (!s.b[1296])) {
            s.store_add_ad_lhs(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 121);
        }

        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(172, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);
            s.store_add_scaled_products_indices(118, 313, 172, 1.0, 312, 116, 1.0);
        }

        if (s.b[1291] && (!s.b[1292])) {
            s.store_scaled_sub_offset_sqrt_square_offset(117, 116, 210.0, (-210.0), ((0.25 * 0.2) * 0.2), 0.5);
            s.store_add_scaled_inputs3_offset_mixed_iia(233, 117, 1.0, 228, (-0.5), A::sqrt_square_offset(A::offset(s.ad_value(228), (-210.0)), ((0.25 * 0.2) * 0.2)), (-(-0.5)), ((-0.5) * 210.0));
            s.store_div_scaled_offset_numerator(234, s.ad_value(117), 1.0, (-210.0), s.ad_value(228), 1.0);
        }

        if s.b[1291] {
            s.store_scale(182, 118, 8.617087e-5);
        }

        s.store_sub_from_scalar_ad(146, p.p106, A::div_scaled_product_offset_denominator(s.ad_value(116), s.ad_value(116), p.p1718, s.ad_value(116), p.p1719, 1.0));

        s.store_sub_from_scalar_ad(147, p.p106, A::div_scaled_product_offset_denominator(s.ad_value(228), s.ad_value(228), p.p1718, s.ad_value(228), p.p1719, 1.0));

        s.store_mul_scaled_sqrt_scaled_input_rhs(169, 116, 1.0 / (300.15), 116, 1.0 / (300.15));

        s.store_mul_scaled_limited_exp_ad_rhs(141, 169, p.p105, A::sub_from_scalar((p.p106 / ((2.0 * 8.617087e-5) * 300.15)), A::div_scaled_inputs(s.ad_value(146), 1.0, s.ad_value(179), 2.0)));

        s.b[1297] = (p.p80 == 0.0);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if s.b[1297] {
            s.store_scale(148, 169, p.p107);
        }

        if (!s.b[1297]) {
            s.store_mul_scaled_sqrt_scaled_input_rhs(148, 118, (1.0 / (300.15) * p.p107), 118, 1.0 / (300.15));
        }

        if (!s.b[1297]) {
            s.store_sub_ad(142, A::offset({
                if (!((p.p105 * s.v[169]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((p.p105 * s.v[169]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(169), p.p105)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (p.p106 / ((2.0 * 8.617087e-5) * 300.15))), A::div_scaled_inputs(s.ad_value(146), 1.0, s.ad_value(179), 2.0));
        }

        if (!(((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
            s.store_scaled_add_sqrt_square_offset_ad(235, A::offset(A::mul(s.ad_value(859), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
        } else {
            if (((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_offset_product(235, ((-0.001) * 0.001), 859, 232, ((1.0) + ((-1e-6))));
            } else {
                s.store_scalar(235, 0.0);
            }
        }

        s.store_scale(389, 179, 1.60219e-19);

        s.store_div_from_scalar_ad(168, (1.05457e-34 * 3.141592653589793), A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0));

        s.store_scaled_square(377, 168, 1.0 / ((2.0 * s.v[381])));

        s.store_scaled_square(378, 168, 1.0 / ((2.0 * s.v[382])));

        s.store_scale(379, 377, 4.0);

        s.store_scale(380, 378, 4.0);

        s.v[169] = ((s.v[385] * s.v[384]) / (s.v[386] * s.v[383]));

        s.store_offset_scaled_ad(387, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(378), (-1.0), s.ad_value(389), 1.0)), s.v[169], 1.0);

        s.store_add_scaled_inputs3_mixed_iaa(388, 387, 1.0, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(379), (-1.0), s.ad_value(389), 1.0)), 1.0, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(380), (-1.0), s.ad_value(389), 1.0)), s.v[169]);

        s.store_mul_scaled_ad_rhs(170, 179, -1.0, {
            if (!((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if ((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38) {
                        A::ln(A::mul(A::div_scaled_value_by_product(s.ad_value(389), (s.v[386] * s.v[383]), A::scale(s.ad_value(148), ((3.141592653589793 * 1.05457e-34) * 1.05457e-34)), A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0), 1.0), s.ad_value(388)))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        });

        s.store_mul_add_scaled_inputs_rhs(375, 654, s.ad_value(377), 6.241457005723417e18, s.ad_value(170), 1.0);

        s.store_ln(418, 229);

        s.b[1298] = (p.p80 == 0.0);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if s.b[1298] {
            s.store_mul_exp_ad_rhs(169, 704, A::mul(s.ad_value(836), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
        }

        s.b[1299] = (p.p66 == 1.0);
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1299]) {
            s.store_mul_exp_ad_rhs(169, 706, A::mul(s.ad_value(845), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.copy_ad(417, 321);
        }

        if s.b[1298] {
            s.store_add_scaled_inputs4_offset_mixed_iaai(303, 807, 1.0, A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(807), (-(4.0 * 1e-6)))), 0.5, 807, (-1.0), (0.5 * (-1e-6)));
            s.copy_ad(323, 811);
        }

        s.b[1300] = (p.p66 != 0.0);
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1300]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(305, 815, 1.0, A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(815), (-(4.0 * 1e-6)))), 0.5, 815, (-1.0), (0.5 * (-1e-6)));
        }

        if s.b[1298] {
            s.store_mul_exp_ad_rhs(318, 812, A::mul(s.ad_value(830), s.ad_value(418)));
        }

        s.b[1301] = (p.p66 != 0.0);
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1301]) {
            s.store_mul_exp_ad_rhs(320, 818, A::mul(s.ad_value(844), s.ad_value(418)));
        }

        if s.b[1298] {
            s.store_mul_exp_ad_rhs(317, 814, A::mul(s.ad_value(834), s.ad_value(418)));
        }

        if s.b[1298] {
            if (!(((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(194, A::offset(A::mul(s.ad_value(854), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_product(194, ((-0.001) * 0.001), 854, 232, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(194, 0.0);
                }
            }
        }

        s.b[1302] = (p.p75 != 0.0);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1302]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(332, 679, 1.0, A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, 679, (-1.0), (0.5 * (-1e-6)));
        }

        if (s.b[1298] && (!s.b[1302])) {
            s.store_mul_ad_rhs(332, 679, {
                if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1303] = (p.p66 != 0.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        s.b[1304] = (p.p75 != 0.0);
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if ((s.b[1298] && s.b[1303]) && s.b[1304]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(333, 680, 1.0, A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, 680, (-1.0), (0.5 * (-1e-6)));
        }

        if ((s.b[1298] && s.b[1303]) && (!s.b[1304])) {
            s.store_mul_ad_rhs(333, 680, {
                if (!(((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1305] = (s.v[333] < 1000.0);
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((s.b[1298] && s.b[1303]) && s.b[1305]) {
            s.store_scalar(333, 1000.0);
        }

        s.b[1306] = (p.p67 == 1.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1306]) {
            s.store_mul_exp_ad_rhs(169, 705, A::mul(s.ad_value(839), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.store_add_scaled_inputs4_offset_mixed_iaai(304, 808, 1.0, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(808), (-(4.0 * 1e-6)))), 0.5, 808, (-1.0), (0.5 * (-1e-6)));
            s.store_mul_exp_ad_rhs(319, 813, A::mul(s.ad_value(832), s.ad_value(418)));
        }

        s.b[1307] = (p.p75 != 0.0);
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1307]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(334, 698, 1.0, A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, 698, (-1.0), (0.5 * (-1e-6)));
        }

        if (s.b[1298] && (!s.b[1307])) {
            s.store_mul_ad_rhs(334, 698, {
                if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1308] = (p.p66 != 0.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        s.b[1309] = (p.p75 != 0.0);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((s.b[1298] && s.b[1308]) && s.b[1309]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(335, 699, 1.0, A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, 699, (-1.0), (0.5 * (-1e-6)));
        }

        if ((s.b[1298] && s.b[1308]) && (!s.b[1309])) {
            s.store_mul_ad_rhs(335, 699, {
                if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1310] = (s.v[335] < 1000.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if ((s.b[1298] && s.b[1308]) && s.b[1310]) {
            s.store_scalar(335, 1000.0);
        }

        s.b[1311] = (p.p75 != 0.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1311]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(336, 702, 1.0, A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, 702, (-1.0), (0.5 * (-1e-6)));
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1298] && (!s.b[1311])) {
            s.store_mul_ad_rhs(336, 702, {
                if (!(((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1298] {
            s.store_offset_ad(337, {
                if (!(((s.v[790] * (1.0 + (p.p450 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[790] * (1.0 + (p.p450 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }

        s.b[1312] = (p.p66 != 0.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if (s.b[1298] && s.b[1312]) {
            s.store_offset_ad(338, {
                if (!(((s.v[791] * (1.0 + (p.p452 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[791] * (1.0 + (p.p452 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }

        if s.b[1298] {
            s.copy_ad(660, 657);
            s.copy_ad(797, 792);
            s.store_mul_add_ad_lhs(231, s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153)), 230);
        }

        s.b[1313] = (p.p80 == 1.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
        }

        s.b[1314] = (p.p66 == 1.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1314]) {
            s.store_mul_exp_ad_rhs(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.copy_ad(417, 321);
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(303, 807, A::mul(A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        s.b[1315] = (p.p66 != 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1315]) {
            s.store_mul_exp_ad_rhs(305, 815, A::mul(A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(318, 812, A::mul(A::add_scaled_product(s.ad_value(830), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        s.b[1316] = (p.p66 != 0.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1316]) {
            s.store_mul_exp_ad_rhs(320, 818, A::mul(A::add_scaled_product(s.ad_value(844), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(317, 814, A::mul(A::add_scaled_inputs(s.ad_value(834), 1.0, s.ad_value(229), p.p881), s.ad_value(418)));
            s.store_mul_offset_ad_rhs(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(230))), (-1.0));
            s.store_mul_offset_ad_rhs(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(230))), (-1.0));
            s.store_offset(330, 324, 0.5);
            s.store_offset(331, 327, 0.5);
        }

        s.b[1317] = (p.p75 != 0.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1317]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(323, 811, 1.0, A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(811), (-(4.0 * 1e-6)))), 0.5, 811, (-1.0), (0.5 * (-1e-6)));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1317])) {
            s.store_mul_ad_rhs(323, 811, {
                if (!(((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1318] = (p.p67 == 1.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1318]) {
            s.store_mul_exp_ad_rhs(169, 705, A::mul(A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(840), s.ad_value(229), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.store_mul_exp_ad_rhs(304, 808, A::mul(A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), s.ad_value(229), 1.0), s.ad_value(418)));
            s.store_mul_exp_ad_rhs(319, 813, A::mul(A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(833), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        s.b[1319] = (s.v[854] == s.v[855]);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1319]) {
            s.store_offset_mul(170, 854, 232, 1.0);
        }

        s.b[1320] = (s.v[856] < s.v[228]);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_mul_ad(171, A::sub(s.ad_value(854), s.ad_value(855)), A::sub(s.ad_value(856), s.ad_value(228)));
        }

        s.b[1321] = (s.v[855] < s.v[854]);
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && s.b[1321]) {
            s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
        }

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && (!s.b[1321])) {
            s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
        }

        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) {
            s.store_offset_mul_ad(196, s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(228)), 1.0);
            s.store_add_scaled_product_mixed_aia(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_mul_ad(171, A::sub(s.ad_value(855), s.ad_value(854)), A::sub(s.ad_value(856), s.ad_value(228)));
        }

        s.b[1322] = (s.v[855] < s.v[854]);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) && s.b[1322]) {
            s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(196), 0.5, s.ad_value(195), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(196), s.ad_value(195)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
        }

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) && (!s.b[1322])) {
            s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(196), 0.5, s.ad_value(195), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(196), s.ad_value(195)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
        }

        if ((!s.b[1298]) && s.b[1313]) {
            if (!((s.v[170] - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(194, A::offset(s.ad_value(170), (-1e-6)), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if ((s.v[170] - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_input(194, ((-0.001) * 0.001), 170, (-1e-6));
                } else {
                    s.store_scalar(194, 0.0);
                }
            }
        }

        s.b[1323] = (p.p75 != 0.0);
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1323]) {
            s.store_add_scaled_inputs3_mixed_iai(332, 679, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(679), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 679, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1323])) {
            s.store_mul_ad_rhs(332, 679, {
                if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1324] = (p.p66 != 0.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        s.b[1325] = (p.p75 != 0.0);
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && s.b[1325]) {
            s.store_add_scaled_inputs3_mixed_iai(333, 680, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(680), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 680, (-1.0));
        }

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && (!s.b[1325])) {
            s.store_mul_ad_rhs(333, 680, {
                if (!((((1.0 + ((-s.v[851]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[851]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1326] = (s.v[333] < 1000.0);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && s.b[1326]) {
            s.store_scalar(333, 1000.0);
        }

        s.b[1327] = (p.p75 != 0.0);
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1327]) {
            s.store_add_scaled_inputs3_mixed_iai(334, 698, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(698), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 698, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1327])) {
            s.store_mul_ad_rhs(334, 698, {
                if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1328] = (p.p66 != 0.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        s.b[1329] = (p.p75 != 0.0);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && s.b[1329]) {
            s.store_add_scaled_inputs3_mixed_iai(335, 699, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(699), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 699, (-1.0));
        }

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && (!s.b[1329])) {
            s.store_mul_ad_rhs(335, 699, {
                if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1330] = (s.v[335] < 1000.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && s.b[1330]) {
            s.store_scalar(335, 1000.0);
        }

        s.b[1331] = (p.p75 != 0.0);
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1331]) {
            s.store_add_scaled_inputs3_mixed_iai(336, 702, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p574), 0.5, s.ad_value(702), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 702, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1331])) {
            s.store_mul_ad_rhs(336, 702, {
                if (!((((1.0 + ((-s.v[850]) * s.v[232])) + ((p.p574 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 + ((-s.v[850]) * s.v[232])) + ((p.p574 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_offset_ad(337, {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }

        s.b[1332] = (p.p66 != 0.0);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1332]) {
            s.store_offset_ad(338, {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }

        s.b[1333] = (p.p75 != 0.0);
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1333]) {
            s.store_add_scaled_inputs3_mixed_iai(660, 657, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 0.5, s.ad_value(657), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6))), 1.0, s.ad_value(657), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 657, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1333])) {
            s.store_mul_ad_rhs(660, 657, {
                if (!((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1334] = (p.p75 != 0.0);
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && s.b[1313]) && s.b[1334]) {
            s.store_add_scaled_inputs3_mixed_iai(797, 792, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1026 * 0.5), s.ad_value(792), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1026, s.ad_value(792), -1.0), (-1e-6))), 1.0, s.ad_value(792), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 792, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1334])) {
            s.store_mul_ad_rhs(797, 792, {
                if (!(((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_sub_ad(231, A::add_scaled_product(A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(116), (-p.p1749)), p.p1748), 1.0, 1.0), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230), 1.0), A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(228), (-p.p1749)), p.p1748), 1.0, 1.0));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_mul_exp_ad_rhs(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
        }

        s.b[1335] = (p.p66 == 1.0);
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1335]) {
            s.store_mul_exp_ad_rhs(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.copy_ad(417, 321);
        }

        s.b[1336] = (s.v[228] > 210.0);
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {
            s.store_scaled_sub_ad(170, A::div(s.ad_value(823), A::add(s.ad_value(807), A::mul_sub_from_scalar_rhs(s.ad_value(823), 210.0, s.ad_value(228)))), A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 210.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {
            s.store_div_scaled_inputs2_mixed_iaa(169, 807, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(823), 210.0, s.ad_value(228)), 1.0, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {
            s.store_mul_pow_ad_rhs(306, 169, s.ad_value(229), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(307, 807, 1.0, 823, 232, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1336])) {
            s.store_mul_ad_product_rhs(170, 807, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), A::add_scaled_inputs(s.ad_value(823), 0.004761904761904762, A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1336])) {
            s.store_add_scaled_product_mixed_aia(169, A::mul_sub_from_scalar_rhs(s.ad_value(170), 210.0, s.ad_value(228)), (-1.0), 807, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(306, 807, s.ad_value(229), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(307, 169, 1.0, 170, 232, 1.0);
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_add_scaled_products_indices(168, 313, 306, 1.0, 312, 307, 1.0);
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_rhs(303, 168, 168, ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(303, ((-1e-6) * 1e-6), 168);
                } else {
                    s.store_scalar(303, 0.0);
                }
            }
        }

        s.b[1337] = (p.p66 != 0.0);
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        s.b[1338] = (s.v[228] > 210.0);
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && s.b[1338]) {
            s.store_scaled_sub_ad(170, A::div(s.ad_value(825), A::add(s.ad_value(815), A::mul_sub_from_scalar_rhs(s.ad_value(825), 210.0, s.ad_value(228)))), A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 210.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && s.b[1338]) {
            s.store_div_scaled_inputs2_mixed_iaa(169, 815, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(825), 210.0, s.ad_value(228)), 1.0, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(310, 169, s.ad_value(229), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(311, 815, 1.0, 825, 232, 1.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && (!s.b[1338])) {
            s.store_mul_ad_product_rhs(170, 815, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), A::add_scaled_inputs(s.ad_value(825), 0.004761904761904762, A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 1.0));
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && (!s.b[1338])) {
            s.store_add_scaled_product_mixed_aia(169, A::mul_sub_from_scalar_rhs(s.ad_value(170), 210.0, s.ad_value(228)), (-1.0), 815, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(310, 815, s.ad_value(229), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(311, 169, 1.0, 170, 232, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) {
            s.store_add_scaled_products_indices(168, 313, 310, 1.0, 312, 311, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) {
            if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_rhs(305, 168, 168, ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(305, ((-1e-6) * 1e-6), 168);
                } else {
                    s.store_scalar(305, 0.0);
                }
            }
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_mul_exp_ad_rhs(318, 812, A::mul(A::add_scaled_product(s.ad_value(830), 1.0, s.ad_value(831), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        s.b[1339] = (p.p66 != 0.0);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1339]) {
            s.store_mul_exp_ad_rhs(320, 818, A::mul(A::add_scaled_product(s.ad_value(844), 1.0, s.ad_value(831), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_mul_exp_ad_rhs(317, 814, A::mul(A::add_scaled_product(s.ad_value(834), 1.0, s.ad_value(835), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        s.b[1340] = (((((s.v[326] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1340]) {
            s.store_mul_offset_ad_rhs(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), (-1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1340])) {
            s.store_div_scaled_product_offset_rhs(324, s.ad_value(325), A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), (-1.0), 1.0, A::abs(A::offset(A::limited_exp(A::div_scaled_product_offset_rhs(s.ad_value(326), s.ad_value(228), (-210.0), 1.0, s.ad_value(228), 1.0)), (-1.0))), 1.0);
        }

        s.b[1341] = (((((s.v[329] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1341]) {
            s.store_mul_offset_ad_rhs(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), (-1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1341])) {
            s.store_div_scaled_product_offset_rhs(327, s.ad_value(328), A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), (-1.0), 1.0, A::abs(A::offset(A::limited_exp(A::div_scaled_product_offset_rhs(s.ad_value(329), s.ad_value(228), (-210.0), 1.0, s.ad_value(228), 1.0)), (-1.0))), 1.0);
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_offset(330, 324, 0.5);
            s.store_offset(331, 327, 0.5);
        }

        s.b[1342] = (p.p75 != 0.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(323, 811, 1.0, A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(233), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(233), 1.0), (-1e-6))), 1.0, s.ad_value(811), (-(4.0 * 1e-6)))), 0.5, 811, (-1.0), (0.5 * (-1e-6)));
            s.store_add_scaled_inputs3_mixed_iai(332, 679, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(679), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 679, (-1.0));
        }

        s.b[1343] = (p.p66 != 0.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1343]) {
            s.store_add_scaled_inputs3_mixed_iai(333, 680, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(680), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 680, (-1.0));
        }

        s.b[1344] = (s.v[333] < 1000.0);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1343]) && s.b[1344]) {
            s.store_scalar(333, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            s.store_add_scaled_inputs3_mixed_iai(334, 698, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(698), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 698, (-1.0));
        }

        s.b[1345] = (p.p66 != 0.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1345]) {
            s.store_add_scaled_inputs3_mixed_iai(335, 699, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(699), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 699, (-1.0));
        }

        s.b[1346] = (s.v[335] < 1000.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1345]) && s.b[1346]) {
            s.store_scalar(335, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            s.store_add_scaled_inputs3_mixed_iai(336, 702, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p574), 0.5, s.ad_value(702), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 702, (-1.0));
            s.store_add_scaled_inputs3_mixed_iai(660, 657, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(233), p.p498, s.ad_value(233), s.ad_value(233), p.p499), 0.5, s.ad_value(657), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(233), p.p498, s.ad_value(233), s.ad_value(233), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6))), 1.0, s.ad_value(657), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 657, (-1.0));
            s.store_add_scaled_inputs3_mixed_iai(797, 792, 1.0, A::add_scaled_inputs3_offset(s.ad_value(233), (p.p1026 * 0.5), s.ad_value(792), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(233), p.p1026, s.ad_value(792), -1.0), (-1e-6))), 1.0, s.ad_value(792), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 792, (-1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(323, 811, {
                if (!(((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(332, 679, {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1347] = (p.p66 != 0.0);
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1347]) {
            s.store_mul_ad_rhs(333, 680, {
                if (!((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(851), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(851), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(851), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1348] = (s.v[333] < 1000.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1347]) && s.b[1348]) {
            s.store_scalar(333, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(334, 698, {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1349] = (p.p66 != 0.0);
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1349]) {
            s.store_mul_ad_rhs(335, 699, {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1350] = (s.v[335] < 1000.0);
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1349]) && s.b[1350]) {
            s.store_scalar(335, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(336, 702, {
                if (!((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(850), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(850), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(850), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(660, 657, {
                if (!((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(797, 792, {
                if (!(((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_offset_ad(337, {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }

        s.b[1351] = (p.p66 != 0.0);
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1351]) {
            s.store_offset_ad(338, {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }

        s.b[1352] = (p.p67 == 1.0);
        s.v[1352] = if s.b[1352] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_mul_exp_ad_rhs(169, 705, A::mul(A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(840), s.ad_value(234), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
        }

        s.b[1353] = (s.v[228] > 210.0);
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && s.b[1353]) {
            s.store_scaled_sub_ad(170, A::div(s.ad_value(826), A::add(s.ad_value(808), A::mul_sub_from_scalar_rhs(s.ad_value(826), 210.0, s.ad_value(228)))), A::div_scaled_product_offset_rhs(s.ad_value(827), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 210.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && s.b[1353]) {
            s.store_div_scaled_inputs2_mixed_iaa(169, 808, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(826), 210.0, s.ad_value(228)), 1.0, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(308, 169, s.ad_value(229), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(827), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(309, 808, 1.0, 826, 232, 1.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && (!s.b[1353])) {
            s.store_mul_ad_product_rhs(170, 808, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), A::add_scaled_inputs(s.ad_value(826), 0.004761904761904762, A::div_scaled_product_offset_rhs(s.ad_value(827), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 1.0));
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && (!s.b[1353])) {
            s.store_add_scaled_product_mixed_aia(169, A::mul_sub_from_scalar_rhs(s.ad_value(170), 210.0, s.ad_value(228)), (-1.0), 808, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(308, 808, s.ad_value(229), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(309, 169, 1.0, 170, 232, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_add_scaled_products_indices(168, 313, 308, 1.0, 312, 309, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_rhs(304, 168, 168, ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(304, ((-1e-6) * 1e-6), 168);
                } else {
                    s.store_scalar(304, 0.0);
                }
            }
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_mul_exp_ad_rhs(319, 813, A::mul(A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(833), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        s.b[1354] = (s.v[854] == s.v[855]);
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1354]) {
            s.store_offset_mul(170, 854, 232, 1.0);
        }

        s.b[1355] = (s.v[856] < 210.0);
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        s.b[1356] = (s.v[228] > 210.0);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_offset_ad(171, A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)), 1.0);
            s.store_add_scaled_product_mixed_aia(172, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(856)), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
        }

        s.b[1357] = (s.v[855] < s.v[854]);
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && s.b[1357]) {
            s.store_add_ad_lhs(174, A::sub(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs3(s.ad_value(171), 0.5, s.ad_value(172), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(171), s.ad_value(172)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5)), 171);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 174, 0.5, 195, 0.5, 174, 195, ((0.25 * 0.001) * 0.001), 0.5);
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && (!s.b[1357])) {
            s.store_add_ad_lhs(174, A::sub(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::add_scaled_inputs3(s.ad_value(171), 0.5, s.ad_value(172), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(171), s.ad_value(172)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5))), 171);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 174, 0.5, 195, 0.5, 174, 195, ((0.25 * 0.001) * 0.001), (-0.5));
        }

        s.b[1358] = (s.v[228] > s.v[856]);
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_mul_ad(171, A::sub(s.ad_value(854), s.ad_value(855)), A::sub(s.ad_value(856), s.ad_value(228)));
            s.store_offset_ad(172, A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)), 1.0);
            s.store_add_scaled_product_mixed_aia(174, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(856)), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
        }

        s.b[1359] = (s.v[855] < s.v[854]);
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && s.b[1359]) {
            s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), 0.5);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && (!s.b[1359])) {
            s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), (-0.5));
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) {
            s.store_offset_mul(196, 855, 232, 1.0);
            s.store_add_scaled_product_mixed_aia(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_mul_ad(171, A::sub(s.ad_value(855), s.ad_value(854)), A::sub(s.ad_value(856), s.ad_value(228)));
            s.store_offset_ad(172, A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(228)), 1.0);
            s.store_add_scaled_product_mixed_aia(174, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(856)), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
        }

        s.b[1360] = (s.v[855] < s.v[854]);
        s.v[1360] = if s.b[1360] { 1.0 } else { 0.0 };

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && s.b[1360]) {
            s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), 0.5);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && (!s.b[1360])) {
            s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), (-0.5));
        }

        s.b[1361] = (s.v[228] > 210.0);
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_add_ad(196, A::offset(A::mul_offset_rhs(s.ad_value(855), s.ad_value(116), (-210.0)), 1.0), A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)));
        }

        s.b[1362] = (s.v[855] < s.v[854]);
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && s.b[1362]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), 0.5);
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && (!s.b[1362])) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), (-0.5));
        }

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) {
            s.store_offset_mul(196, 855, 232, 1.0);
            s.store_add_ad(195, A::offset(A::mul_offset_rhs(s.ad_value(854), s.ad_value(116), (-210.0)), 1.0), A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(228)));
        }

        s.b[1363] = (s.v[855] < s.v[854]);
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && s.b[1363]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), 0.5);
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && (!s.b[1363])) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), (-0.5));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            if (!((s.v[170] - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(194, A::offset(s.ad_value(170), (-1e-6)), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if ((s.v[170] - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_input(194, ((-0.001) * 0.001), 170, (-1e-6));
                } else {
                    s.store_scalar(194, 0.0);
                }
            }
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_scaled_sub_offset_sqrt_square_offset(172, 228, 210.0, (-210.0), ((0.25 * 0.2) * 0.2), 0.5);
            s.store_sub_ad(231, A::add_scaled_product(A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(117), (-p.p1749)), p.p1748), 1.0, 1.0), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230), 1.0), A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(172), (-p.p1749)), p.p1748), 1.0, 1.0));
        }

        s.b[1364] = (s.v[332] < 1000.0);
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if s.b[1364] {
            s.store_scalar(332, 1000.0);
        }

        s.b[1365] = (s.v[334] < 1000.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if s.b[1365] {
            s.store_scalar(334, 1000.0);
        }

        s.b[1366] = (s.v[336] < 1000.0);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if s.b[1366] {
            s.store_scalar(336, 1000.0);
        }

        s.b[1367] = (p.p61 != 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        s.b[1368] = (p.p75 == 0.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        s.b[1369] = (p.p75 != 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if ((s.b[1367] && s.b[1368]) && s.b[1369]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(314, 809, 1.0, A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(809), (-(4.0 * 1e-6)))), 0.5, 809, (-1.0), (0.5 * (-1e-6)));
        }

        if ((s.b[1367] && s.b[1368]) && (!s.b[1369])) {
            s.store_mul_ad_rhs(314, 809, {
                if (!(((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1370] = (p.p67 == 1.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        s.b[1371] = (p.p75 != 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && s.b[1371]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(315, 810, 1.0, A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(810), (-(4.0 * 1e-6)))), 0.5, 810, (-1.0), (0.5 * (-1e-6)));
        }

        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && (!s.b[1371])) {
            s.store_mul_ad_rhs(315, 810, {
                if (!(((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1372] = (p.p66 != 0.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        s.b[1373] = (p.p75 != 0.0);
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && s.b[1373]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(316, 817, 1.0, A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(817), (-(4.0 * 1e-6)))), 0.5, 817, (-1.0), (0.5 * (-1e-6)));
        }

        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && (!s.b[1373])) {
            s.store_mul_ad_rhs(316, 817, {
                if (!(((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1367] && (!s.b[1368])) {
            s.store_add_scaled_product_indices(314, 809, 1.0, 828, 232, 1.0);
        }

        s.b[1374] = (p.p67 == 1.0);
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if ((s.b[1367] && (!s.b[1368])) && s.b[1374]) {
            s.store_add_scaled_product_indices(315, 810, 1.0, 829, 232, 1.0);
        }

        s.b[1375] = (p.p66 != 0.0);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        if ((s.b[1367] && (!s.b[1368])) && s.b[1375]) {
            s.store_add_scaled_product_indices(316, 817, 1.0, 843, 232, 1.0);
        }

        s.b[1376] = (p.p75 != 0.0);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if s.b[1376] {
            s.store_add_scaled_inputs3_mixed_iai(296, 673, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p164 * 0.5), s.ad_value(673), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p164, s.ad_value(673), -1.0), (-1e-6))), 1.0, s.ad_value(673), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 673, (-1.0));
        }

        if (!s.b[1376]) {
            s.store_mul_ad_rhs(296, 673, {
                if (!(((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1377] = (p.p67 == 1.0);
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        s.b[1378] = (p.p75 != 0.0);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if (s.b[1377] && s.b[1378]) {
            s.store_add_scaled_inputs3_mixed_iai(297, 675, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p165 * 0.5), s.ad_value(675), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p165, s.ad_value(675), -1.0), (-1e-6))), 1.0, s.ad_value(675), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 675, (-1.0));
        }

        if (s.b[1377] && (!s.b[1378])) {
            s.store_mul_ad_rhs(297, 675, {
                if (!(((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1379] = (p.p75 != 0.0);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if s.b[1379] {
            s.store_add_scaled_inputs3_mixed_iai(298, 677, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p166 * 0.5), s.ad_value(677), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p166, s.ad_value(677), -1.0), (-1e-6))), 1.0, s.ad_value(677), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 677, (-1.0));
        }

        if (!s.b[1379]) {
            s.store_mul_ad_rhs(298, 677, {
                if (!(((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1380] = (p.p75 != 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if s.b[1380] {
            s.store_add_scaled_inputs4_offset_mixed_iaai(322, 707, 1.0, A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(707), (-(4.0 * 1e-6)))), 0.5, 707, (-1.0), (0.5 * (-1e-6)));
        }

        if (!s.b[1380]) {
            s.store_mul_ad_rhs(322, 707, {
                if (!(((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1381] = (p.p75 != 0.0);
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if s.b[1381] {
            s.store_offset_add_scaled_inputs(299, A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p917))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p917))) + ((-1e-6)))), (-((4.0 * (-p.p917)) * 1e-6))), 0.5, (((-p.p917)) + (p.p917)));
        }

        if (!s.b[1381]) {
            s.store_scale_ad(299, {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p917);
        }

        s.b[1382] = (p.p66 != 0.0);
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        s.b[1383] = (p.p75 != 0.0);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if (s.b[1382] && s.b[1383]) {
            s.store_offset_add_scaled_inputs(300, A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p918))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p918))) + ((-1e-6)))), (-((4.0 * (-p.p918)) * 1e-6))), 0.5, (((-p.p918)) + (p.p918)));
        }

        if (s.b[1382] && (!s.b[1383])) {
            s.store_scale_ad(300, {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p918);
        }

        s.b[1384] = (p.p75 != 0.0);
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if s.b[1384] {
            s.store_offset_add_scaled_inputs(301, A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p919))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p919))) + ((-1e-6)))), (-((4.0 * (-p.p919)) * 1e-6))), 0.5, (((-p.p919)) + (p.p919)));
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1384]) {
            s.store_scale_ad(301, {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p919);
        }

        s.b[1385] = (p.p66 != 0.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        s.b[1386] = (p.p75 != 0.0);
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        if (s.b[1385] && s.b[1386]) {
            s.store_offset_add_scaled_inputs(302, A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p920))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p920))) + ((-1e-6)))), (-((4.0 * (-p.p920)) * 1e-6))), 0.5, (((-p.p920)) + (p.p920)));
        }

        if (s.b[1385] && (!s.b[1386])) {
            s.store_scale_ad(302, {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p920);
        }

        s.b[1387] = (p.p75 != 0.0);
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        if s.b[1387] {
            s.store_add_scaled_inputs4_offset_mixed_iaai(257, 700, 1.0, A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(700), (-(4.0 * 1e-6)))), 0.5, 700, (-1.0), (0.5 * (-1e-6)));
        }

        if (!s.b[1387]) {
            s.store_mul_ad_rhs(257, 700, {
                if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1388] = (p.p66 != 0.0);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        s.b[1389] = (p.p75 != 0.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if (s.b[1388] && s.b[1389]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(258, 701, 1.0, A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(701), (-(4.0 * 1e-6)))), 0.5, 701, (-1.0), (0.5 * (-1e-6)));
        }

        if (s.b[1388] && (!s.b[1389])) {
            s.store_mul_ad_rhs(258, 701, {
                if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.store_mul_exp_ad_rhs(248, 779, A::mul(s.ad_value(860), s.ad_value(418)));

        s.store_mul_offset_ad_rhs(249, 785, {
            if (!(((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01))), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);

        s.store_add_scaled_product_indices(236, 683, 1.0, 684, 232, 1.0);

        s.store_add_scaled_inputs4_offset_mixed_iaai(237, 685, 1.0, A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(685), (-(4.0 * 1e-6)))), 0.5, 685, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(238, 687, 1.0, A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(687), (-(4.0 * 1e-6)))), 0.5, 687, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(239, 690, 1.0, A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(690), (-(4.0 * 1e-6)))), 0.5, 690, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_product_indices(240, 692, 1.0, 693, 232, 1.0);

        s.store_add_scaled_product_indices(241, 798, 1.0, 800, 232, 1.0);

        s.store_add_scaled_product_indices(242, 799, 1.0, 801, 232, 1.0);

        s.store_add_scaled_inputs4_offset_mixed_iaai(293, 871, 1.0, A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(871), (-(4.0 * 1e-6)))), 0.5, 871, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_product_indices(294, 867, 1.0, 868, 232, 1.0);

        s.store_add_scaled_product_indices(295, 869, 1.0, 870, 232, 1.0);

        s.store_add_scaled_inputs4_offset_mixed_iaai(243, 721, 1.0, A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(721), (-(4.0 * 1e-6)))), 0.5, 721, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(244, 727, 1.0, A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(727), (-(4.0 * 1e-6)))), 0.5, 727, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(245, 732, 1.0, A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(732), (-(4.0 * 1e-6)))), 0.5, 732, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(246, 737, 1.0, A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(737), (-(4.0 * 1e-6)))), 0.5, 737, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(247, 743, 1.0, A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(743), (-(4.0 * 1e-6)))), 0.5, 743, (-1.0), (0.5 * (-1e-6)));

        s.store_mul_ad_rhs(252, 748, {
            if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        });

        s.store_mul_ad_rhs(250, 762, {
            if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        });

        s.store_add_scaled_inputs3_mixed_iai(259, 775, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1437 * 0.5), s.ad_value(775), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1437, s.ad_value(775), -1.0), (-1e-6))), 1.0, s.ad_value(775), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 775, (-1.0));

        s.store_add_scaled_inputs3_mixed_iai(260, 776, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1438 * 0.5), s.ad_value(776), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1438, s.ad_value(776), -1.0), (-1e-6))), 1.0, s.ad_value(776), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 776, (-1.0));

        s.store_add_scaled_inputs3_mixed_iai(261, 777, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1439 * 0.5), s.ad_value(777), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1439, s.ad_value(777), -1.0), (-1e-25))), 1.0, s.ad_value(777), (-(4.0 * 1e-25)))), 0.5, ((-1e-25) * 0.5)), 1.0, 777, (-1.0));

        s.store_add_scaled_inputs3_mixed_iai(262, 778, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1440 * 0.5), s.ad_value(778), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1440, s.ad_value(778), -1.0), (-1e-20))), 1.0, s.ad_value(778), (-(4.0 * 1e-20)))), 0.5, ((-1e-20) * 0.5)), 1.0, 778, (-1.0));

        s.b[1390] = (p.p61 != 0.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        s.b[1391] = (p.p75 != 0.0);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1391]) {
            s.store_offset_add_scaled_inputs(263, A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1584))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1584))) + ((-1e-6)))), (-((4.0 * (-p.p1584)) * 1e-6))), 0.5, (((-p.p1584)) + (p.p1584)));
        }

        if (s.b[1390] && (!s.b[1391])) {
            s.store_scale_ad(263, {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1584);
        }

        s.b[1392] = (p.p75 != 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1392]) {
            s.store_offset_add_scaled_inputs(266, A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1585))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1585))) + ((-1e-6)))), (-((4.0 * (-p.p1585)) * 1e-6))), 0.5, (((-p.p1585)) + (p.p1585)));
        }

        if (s.b[1390] && (!s.b[1392])) {
            s.store_scale_ad(266, {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1585);
        }

        s.b[1393] = (p.p75 != 0.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1393]) {
            s.store_offset_add_scaled_inputs(264, A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1586))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1586))) + ((-1e-6)))), (-((4.0 * (-p.p1586)) * 1e-6))), 0.5, (((-p.p1586)) + (p.p1586)));
        }

        if (s.b[1390] && (!s.b[1393])) {
            s.store_scale_ad(264, {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1586);
        }

        s.b[1394] = (p.p75 != 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1394]) {
            s.store_offset_add_scaled_inputs(267, A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1587))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1587))) + ((-1e-6)))), (-((4.0 * (-p.p1587)) * 1e-6))), 0.5, (((-p.p1587)) + (p.p1587)));
        }

        if (s.b[1390] && (!s.b[1394])) {
            s.store_scale_ad(267, {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1587);
        }

        s.b[1395] = (p.p75 != 0.0);
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1395]) {
            s.store_offset_add_scaled_inputs(268, A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1588))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1588))) + ((-1e-6)))), (-((4.0 * (-p.p1588)) * 1e-6))), 0.5, (((-p.p1588)) + (p.p1588)));
        }

        if (s.b[1390] && (!s.b[1395])) {
            s.store_scale_ad(268, {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1588);
        }

        s.b[1396] = (p.p75 != 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if (s.b[1390] && s.b[1396]) {
            s.store_offset_add_scaled_inputs(265, A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1589))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1589))) + ((-1e-6)))), (-((4.0 * (-p.p1589)) * 1e-6))), 0.5, (((-p.p1589)) + (p.p1589)));
        }

        if (s.b[1390] && (!s.b[1396])) {
            s.store_scale_ad(265, {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1589);
        }

        if s.b[1390] {
            s.store_offset_ad(269, {
                if (!(((p.p1590 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1590 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(272, {
                if (!(((p.p1591 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1591 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(270, {
                if (!(((p.p1592 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1592 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(273, {
                if (!(((p.p1593 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1593 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(271, {
                if (!(((p.p1594 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1594 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(274, {
                if (!(((p.p1595 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1595 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_sub_ad(168, A::div(s.ad_value(147), s.ad_value(180)), A::div(s.ad_value(146), s.ad_value(179)));
            s.store_limited_exp_scaled_input_ad(171, A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p.p1727), 1.0 / (p.p1620));
            s.store_scale(275, 171, p.p1614);
            s.store_scale(276, 171, p.p1616);
            s.store_scale(277, 171, p.p1618);
            s.store_limited_exp_scaled_input_ad(171, A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p.p1728), 1.0 / (p.p1621));
            s.store_scale(278, 171, p.p1615);
            s.store_scale(279, 171, p.p1617);
            s.store_scale(280, 171, p.p1619);
            s.store_scaled_limited_exp_ad(281, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1729, s.ad_value(179), 1.0), p.p1630);
            s.store_scaled_limited_exp_ad(282, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1730, s.ad_value(179), 1.0), p.p1631);
            s.store_scaled_limited_exp_ad(283, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1731, s.ad_value(179), 1.0), p.p1632);
            s.store_scaled_limited_exp_ad(284, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1732, s.ad_value(179), 1.0), p.p1633);
            s.store_scaled_mul_ad(285, A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1733, s.ad_value(179), 1.0)), p.p1634);
            s.store_scaled_mul_ad(286, A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1734, s.ad_value(179), 1.0)), p.p1635);
        }

        if s.b[1390] {
            s.store_offset_ad(287, {
                if (!(((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(288, {
                if (!(((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1390] {
            s.store_offset_ad(289, {
                if (!(((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(290, {
                if (!(((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(291, {
                if (!(((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(292, {
                if (!(((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        s.b[1397] = (!param_given[1106]);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        s.b[1398] = (p.p145 > 0.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        s.b[1399] = (p.p80 == 0.0);
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if ((s.b[1397] && s.b[1398]) && s.b[1399]) {
            let assign18720_ad_e35490: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p145 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p145 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p145 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p145 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p.p145 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p145 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            }, (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            let assign18720_ad_e35683: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p.p97 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p97 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            }, (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_sub_ad_rhs(479, 114, assign18720_ad_e35490, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, assign18720_ad_e35683, 1.0), (-1.0)));
        }

        if ((s.b[1397] && s.b[1398]) && (!s.b[1399])) {
            let assign18730_ad_e36032: A = A::sub({
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), (-1.0)));
            s.store_mul_ad_rhs(479, 114, assign18730_ad_e36032);
        }

        s.b[1400] = (p.p80 == 0.0);
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if ((s.b[1397] && (!s.b[1398])) && s.b[1400]) {
            let assign18750_ad_e36241: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p.p97 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p97 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            }, (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_sub_ad_rhs(479, 114, s.ad_value(641), A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p.p104), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, assign18750_ad_e36241, 1.0), (-1.0)));
        }

        if ((s.b[1397] && (!s.b[1398])) && (!s.b[1400])) {
            s.store_mul_sub_ad_rhs(479, 114, s.ad_value(641), A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p.p104), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), (-1.0)));
        }

        if (!s.b[1397]) {
            s.store_scalar(479, p.p1106);
        }

        s.b[1401] = (!param_given[1107]);
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if s.b[1401] {
            s.copy_ad(518, 479);
        }

        if (!s.b[1401]) {
            s.store_scalar(518, p.p1107);
        }

        s.b[1402] = (p.p80 == 0.0);
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        if s.b[1402] {
            s.store_mul_ad_rhs(166, 179, {
                if (!((s.v[640] / s.v[141]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] / s.v[141]) > 1e-38) {
                            A::ln(A::div(s.ad_value(640), s.ad_value(141)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1402] {
            s.store_scaled_add_sqrt_square_offset_rhs(166, 166, 166, ((0.25 * 1e-10) * 1e-10), 0.5);
        }

        if s.b[1402] {
            s.store_mul_ad_rhs(352, 179, {
                if (!(((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(640), p.p97, A::square(s.ad_value(141)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (!s.b[1402]) {
            s.store_mul_sub_ad_rhs(166, 179, {
                if (!(s.v[640] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[640] > 1e-38) {
                            A::ln(s.ad_value(640))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(142));
        }

        if (!s.b[1402]) {
            s.store_scaled_add_sqrt_square_offset_rhs(166, 166, 166, ((0.25 * 1e-10) * 1e-10), 0.5);
        }

        if (!s.b[1402]) {
            s.store_mul_sub_scaled_inputs_rhs(352, 179, {
                if (!((s.v[640] * p.p97) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] * p.p97) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(640), p.p97)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, s.ad_value(142), 2.0);
        }

        s.store_mul_sub_ad_rhs(167, 114, s.ad_value(641), A::offset({
            if (p.p60 == 1.0) {
                A::constant(0.0)
            } else {
                s.ad_value(146)
            }
        }, p.p104));

        s.store_scale(407, 322, 0.5);

        s.v[408] = 0.5;

        s.b[1403] = (p.p60 != 1.0);
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        if s.b[1403] {
            s.store_scale(407, 322, 0.333333333);
            s.store_scalar(408, 0.333333333);
        }

        s.b[1404] = (p.p61 != 0.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if s.b[1404] {
            s.store_add_scaled_inputs3_indices(537, 275, p.p11, 276, p.p13, 277, (p.p3 * s.v[115]));
        }

        s.b[1405] = (s.v[537] > 0.0);
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1405]) {
            s.store_scale(539, 179, p.p1620);
            s.store_scaled_limited_exp_ad(547, A::div_from_scalar((-p.p1626), s.ad_value(539)), p.p1628);
            s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1622, s.ad_value(537)), 10.0);
            s.store_sub_offset_lhs(226, 170, 1.0, 547);
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_mul_ad_rhs(546, 539, {
                if (!((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38) {
                            A::ln_scaled_input(A::add(s.ad_value(226), A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(226)), 1.0, s.ad_value(547), 4.0))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_limited_exp_div(168, 546, 539);
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_offset_ad(170, {
                if (!(((p.p1624 / s.v[537]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1624 / s.v[537]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(543, (-p.p1626), 539, {
                if (!(((s.v[170] - 1.0) / p.p1628) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p.p1628) > 1e-38) {
                            A::ln_scaled_input(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p.p1628))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_scaled_limited_exp_ad(169, A::div_scaled_inputs(A::offset(s.ad_value(543), p.p1626), -1.0, s.ad_value(539), 1.0), p.p1628);
            s.store_mul_offset_rhs(542, 537, 169, 1.0);
            s.store_div_scaled_product_indices(541, 537, 169, -1.0, 539, 1.0);
        }

        if s.b[1404] {
            s.store_add_scaled_inputs3_indices(538, 278, p.p12, 279, p.p14, 280, (p.p3 * s.v[115]));
        }

        s.b[1406] = (s.v[538] > 0.0);
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1406]) {
            s.store_scale(540, 179, p.p1621);
            s.store_scaled_limited_exp_ad(554, A::div_from_scalar((-p.p1627), s.ad_value(540)), p.p1629);
            s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1623, s.ad_value(538)), 10.0);
            s.store_sub_offset_lhs(226, 170, 1.0, 554);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1404] && s.b[1406]) {
            s.store_mul_ad_rhs(553, 540, {
                if (!((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[554]))) as f64).sqrt())) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[554]))) as f64).sqrt())) > 1e-38) {
                            A::ln_scaled_input(A::add(s.ad_value(226), A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(226)), 1.0, s.ad_value(554), 4.0))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1404] && s.b[1406]) {
            s.store_limited_exp_div(168, 553, 540);
        }

        if (s.b[1404] && s.b[1406]) {
            s.store_offset_ad(170, {
                if (!(((p.p1625 / s.v[538]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1625 / s.v[538]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }

        if (s.b[1404] && s.b[1406]) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(550, (-p.p1627), 540, {
                if (!(((s.v[170] - 1.0) / p.p1629) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p.p1629) > 1e-38) {
                            A::ln_scaled_input(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p.p1629))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (s.b[1404] && s.b[1406]) {
            s.store_scaled_limited_exp_ad(169, A::div_scaled_inputs(A::offset(s.ad_value(550), p.p1627), -1.0, s.ad_value(540), 1.0), p.p1629);
            s.store_mul_offset_rhs(549, 538, 169, 1.0);
            s.store_div_scaled_product_indices(548, 538, 169, -1.0, 540, 1.0);
        }

        if s.b[1404] {
            s.store_scale(523, 263, p.p11);
            s.store_scale(524, 264, p.p13);
            s.store_scaled_mul(525, 268, 158, s.v[115]);
            s.store_scale(526, 266, p.p12);
            s.store_scale(527, 267, p.p14);
            s.store_scaled_mul(528, 265, 158, s.v[115]);
        }

        s.b[1407] = (p.p1602 > 0.0);
        s.v[1407] = if s.b[1407] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1407]) {
            s.store_scale(557, 269, (1.0 - (((1.0 / p.p1602)) as f64).powf((1.0 / p.p1596))));
            s.store_div_scaled_inputs_mixed_ia(558, 269, (p.p1602 * (p.p1608 * 1.0 / (p.p1596))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(557), s.ad_value(269))), (-(1.0 + p.p1596))), 1.0);
        }

        s.b[1408] = (p.p1604 > 0.0);
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1408]) {
            s.store_scale(559, 270, (1.0 - (((1.0 / p.p1604)) as f64).powf((1.0 / p.p1598))));
            s.store_div_scaled_inputs_mixed_ia(560, 270, (p.p1604 * (p.p1610 * 1.0 / (p.p1598))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(559), s.ad_value(270))), (-(1.0 + p.p1598))), 1.0);
        }

        s.b[1409] = (p.p1606 > 0.0);
        s.v[1409] = if s.b[1409] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1409]) {
            s.store_scale(561, 271, (1.0 - (((1.0 / p.p1606)) as f64).powf((1.0 / p.p1600))));
            s.store_div_scaled_inputs_mixed_ia(562, 271, (p.p1606 * (p.p1612 * 1.0 / (p.p1600))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(271))), (-(1.0 + p.p1600))), 1.0);
        }

        s.b[1410] = (p.p1603 > 0.0);
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1410]) {
            s.store_scale(563, 272, (1.0 - (((1.0 / p.p1603)) as f64).powf((1.0 / p.p1597))));
            s.store_div_scaled_inputs_mixed_ia(564, 272, (p.p1603 * (p.p1609 * 1.0 / (p.p1597))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(563), s.ad_value(272))), (-(1.0 + p.p1597))), 1.0);
        }

        s.b[1411] = (p.p1605 > 0.0);
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1411]) {
            s.store_scale(565, 273, (1.0 - (((1.0 / p.p1605)) as f64).powf((1.0 / p.p1599))));
            s.store_div_scaled_inputs_mixed_ia(566, 273, (p.p1605 * (p.p1611 * 1.0 / (p.p1599))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(565), s.ad_value(273))), (-(1.0 + p.p1599))), 1.0);
        }

        s.b[1412] = (p.p1607 > 0.0);
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

        if (s.b[1404] && s.b[1412]) {
            s.store_scale(567, 274, (1.0 - (((1.0 / p.p1607)) as f64).powf((1.0 / p.p1601))));
            s.store_div_scaled_inputs_mixed_ia(568, 274, (p.p1607 * (p.p1613 * 1.0 / (p.p1601))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(567), s.ad_value(274))), (-(1.0 + p.p1601))), 1.0);
        }

        s.store_mul_voltage_ad(134, s.ad_value(114), ctx, nodes, Some(11), Some(6));

        s.store_mul_voltage_ad(135, s.ad_value(114), ctx, nodes, Some(5), Some(6));

        s.store_mul_voltage_ad(136, s.ad_value(114), ctx, nodes, Some(11), Some(5));

        s.store_mul_voltage_ad(521, s.ad_value(114), ctx, nodes, Some(3), Some(6));

        s.store_mul_voltage_ad(522, s.ad_value(114), ctx, nodes, Some(3), Some(5));

        s.store_mul_voltage_ad(497, s.ad_value(114), ctx, nodes, Some(11), Some(3));

        s.b[1413] = (p.p76 != 2.0);
        s.v[1413] = if s.b[1413] { 1.0 } else { 0.0 };

        if s.b[1413] {
            s.store_mul_voltage_ad(132, s.ad_value(114), ctx, nodes, Some(10), Some(5));
            s.store_mul_voltage_ad(133, s.ad_value(114), ctx, nodes, Some(10), Some(6));
        }

        if (!s.b[1413]) {
            s.store_mul_voltage_ad(132, s.ad_value(114), ctx, nodes, Some(14), Some(5));
            s.store_mul_voltage_ad(133, s.ad_value(114), ctx, nodes, Some(13), Some(6));
        }

        s.v[128] = 1.0;

        s.b[1414] = (s.v[135] < 0.0);
        s.v[1414] = if s.b[1414] { 1.0 } else { 0.0 };

        if s.b[1414] {
            s.store_scalar(128, (-1.0));
            s.store_sub(125, 134, 135);
            s.store_scale(126, 135, (-1.0));
            s.copy_ad(367, 522);
        }

        if (!s.b[1414]) {
            s.copy_ad(125, 134);
            s.copy_ad(126, 135);
            s.copy_ad(367, 521);
        }

        s.store_sub(347, 125, 167);

        s.store_offset_sqrt_ad(127, A::offset(A::square(s.ad_value(126)), 0.01), (-0.1));

        s.b[1415] = (p.p61 != 0.0);
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if s.b[1415] {
            s.store_add_scaled_inputs3_indices(368, 367, 1.0, 126, (-0.5), 127, (-(-0.5)));
            s.store_scale(369, 689, 0.95);
            s.store_offset_sub(170, 369, 368, (-0.001));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(370, 369, 1.0, 170, (-0.5), A::add_scaled_inputs(A::square(s.ad_value(170)), 1.0, s.ad_value(369), 0.004), (-0.5));
        }

        s.store_tanh_ad(168, A::div_scaled_inputs(s.ad_value(135), 0.6, s.ad_value(179), 1.0));

        s.store_offset_scaled(186, 168, 0.5, 0.5);

        s.store_sub_from_scalar(187, 1.0, 186);

        s.b[1416] = (p.p66 != 0.0);
        s.v[1416] = if s.b[1416] { 1.0 } else { 0.0 };

        if s.b[1416] {
            s.store_add_scaled_products_indices(664, 665, 187, 1.0, 663, 186, 1.0);
            s.store_add_scaled_products_indices(676, 298, 187, 1.0, 296, 186, 1.0);
            s.store_add_scaled_products_indices(427, 715, 187, 1.0, 714, 186, 1.0);
            s.store_add_scaled_products_indices(718, 717, 187, 1.0, 716, 186, 1.0);
            s.store_add_scaled_products_indices(423, 338, 187, 1.0, 337, 186, 1.0);
            s.store_add_scaled_products_indices(424, 258, 187, 1.0, 257, 186, 1.0);
            s.store_add_scaled_products_indices(422, 335, 187, 1.0, 334, 186, 1.0);
            s.store_add_scaled_products_indices(425, 300, 187, 1.0, 299, 186, 1.0);
            s.store_add_scaled_products_indices(426, 302, 187, 1.0, 301, 186, 1.0);
            s.store_add_scaled_products_indices(795, 796, 187, 1.0, 797, 186, 1.0);
            s.store_add_scaled_products_indices(428, 333, 187, 1.0, 332, 186, 1.0);
            s.store_add_scaled_products_indices(659, 658, 187, 1.0, 660, 186, 1.0);
            s.store_add_scaled_products_indices(805, 806, 187, 1.0, 804, 186, 1.0);
            s.store_add_scaled_products_indices(669, 668, 187, 1.0, 666, 186, 1.0);
            s.store_add_scaled_products_indices(416, 417, 187, 1.0, 413, 186, 1.0);
            s.store_add_scaled_products_indices(819, 305, 187, 1.0, 303, 186, 1.0);
            s.store_add_scaled_products_indices(820, 320, 187, 1.0, 318, 186, 1.0);
            s.store_add_scaled_products_indices(821, 316, 187, 1.0, 314, 186, 1.0);
            s.store_add_scaled_products_indices(822, 816, 187, 1.0, 323, 186, 1.0);
        }

        if (!s.b[1416]) {
            s.copy_ad(664, 663);
            s.copy_ad(676, 296);
            s.copy_ad(427, 714);
            s.copy_ad(718, 716);
            s.copy_ad(423, 337);
            s.copy_ad(424, 257);
            s.copy_ad(422, 334);
            s.copy_ad(425, 299);
            s.copy_ad(426, 301);
            s.copy_ad(795, 797);
            s.copy_ad(428, 332);
            s.copy_ad(659, 660);
            s.copy_ad(805, 804);
            s.copy_ad(669, 666);
            s.copy_ad(416, 413);
            s.copy_ad(819, 303);
            s.copy_ad(820, 318);
            s.copy_ad(821, 314);
            s.copy_ad(822, 323);
        }

        s.store_div_from_scalar(212, 1.0, 423);

        s.store_add_offset_lhs(353, 166, 0.4, 672);

        s.store_div_scaled_value_by_product(169, s.ad_value(893), 2.0, s.ad_value(895), A::offset(s.ad_value(898), 2.0), 1.0);

        s.store_mul_add_scaled_product_rhs(164, 362, s.ad_value(662), 1.0, s.ad_value(664), s.ad_value(127), 1.0);

        s.b[1417] = (p.p175 == 0.0);
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        s.b[1418] = (p.p80 == 0.0);
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if (s.b[1417] && s.b[1418]) {
            s.store_mul_ad_product_rhs(181, 179, s.ad_value(235), A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));
        }

        if (s.b[1417] && (!s.b[1418])) {
            s.store_mul_ad_product_rhs(181, 182, s.ad_value(235), A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));
        }

        if (!s.b[1417]) {
            s.store_scalar(181, p.p175);
        }

        s.store_div(897, 903, 181);

        if (!(((s.v[893] * s.v[181]) / (((1.60219e-19 * s.v[148]) * 2.0) * s.v[894])) > 1e-38)) {
            s.store_scalar(900, (-87.498233534));
        } else {
            if (((s.v[893] * s.v[181]) / (((1.60219e-19 * s.v[148]) * 2.0) * s.v[894])) > 1e-38) {
                s.store_ln_ad(900, A::div_scaled_product_by_product(s.ad_value(893), s.ad_value(181), 1.0, s.ad_value(148), s.ad_value(894), (1.60219e-19 * 2.0)));
            } else {
                s.store_scalar(900, 0.0);
            }
        }

        s.store_add_ad_lhs(899, {
            if (!(A::div_scaled_product_offset_denominator(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898)), 1.0, A::add_scaled_product(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), 1.0, s.ad_value(897), s.ad_value(898), (-1.0)), (-1.0), 1.0).value > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if (A::div_scaled_product_offset_denominator(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898)), 1.0, A::add_scaled_product(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), 1.0, s.ad_value(897), s.ad_value(898), (-1.0)), (-1.0), 1.0).value > 1e-38) {
                        A::ln(A::div_scaled_product_offset_denominator(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898)), 1.0, A::add_scaled_product(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), 1.0, s.ad_value(897), s.ad_value(898), (-1.0)), (-1.0), 1.0))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 900);

        s.store_add_scaled_ad_lhs(339, A::div_scaled_inputs(s.ad_value(181), 10.0, s.ad_value(898), 1.0), 396, 2.0);

        s.store_div_scaled_product_indices(912, 179, 893, 1.0, 895, s.v[143]);

        s.v[913] = ((((((4.5 * 1.05457e-34) * 3.141592653589793) * 1.60219e-19) / (4.0 * (((2.0 * s.v[381])) as f64).sqrt()))) as f64).powf(0.666666667);

        s.store_div_scaled_inputs_mixed_ai(914, A::powf(s.ad_value(912), 0.666666667), (p.p1804 * s.v[913]), 179, 1.60219e-19);

        s.store_mul_ad_affine_product_rhs(354, 667, s.ad_value(361), A::sub(s.ad_value(352), s.ad_value(353)), -1.0, 0.0);

        s.store_add_ad(355, A::mul3_scaled_output(s.ad_value(676), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));

        s.store_mul_ad_product_rhs(357, 802, s.ad_value(364), A::sqrt(s.ad_value(353)));

        s.store_add_ad_lhs(358, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(355), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);

        s.store_sub(347, 347, 358);

        s.store_div_scaled_product3_indices(184, 416, 163, 158, 1.0, 153, 1.0);

        s.b[1419] = (p.p80 == 0.0);
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

        if s.b[1419] {
            s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(184), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0), s.ad_value(181));
        }

        if s.b[1419] {
            s.store_neg_ad(168, A::add(s.ad_value(375), {
                if (!(s.v[171] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[171] > 1e-38) {
                            A::ln(s.ad_value(171))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if s.b[1419] {
            s.store_offset_add(169, 347, 168, p.p23);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1419] {
            s.store_sub_ad_lhs(348, {
                if (!(s.v[169] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt_square_offset(s.ad_value(169), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 168);
        }

        if (!s.b[1419]) {
            s.store_mul_scaled_ad_rhs(168, 181, -1.0, {
                if (!((((2.0 * s.v[163]) * p.p108) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p.p108) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(184), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (!s.b[1419]) {
            s.store_sub_ad_lhs(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);
            s.store_offset_add(170, 347, 169, p.p23);
        }

        if (!s.b[1419]) {
            s.store_sub_ad_lhs(348, {
                if (!(s.v[170] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(170), 0.5, A::sqrt_square_offset(s.ad_value(170), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (s.v[170] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(170))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 169);
        }

        s.copy_ad(129, 375);

        s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);

        s.b[1420] = (p.p61 != 0.0);
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

        if s.b[1420] {
            if (!((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 129, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if s.b[1420] {
            s.store_mul_ad(171, A::div_scaled_inputs(s.ad_value(239), -1.0, s.ad_value(181), 2.0), A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)));
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (!s.b[1420]) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);
            s.store_sub(169, 900, 897);
        }

        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);

        s.store_sub(924, 169, 170);

        s.store_scaled_sub(171, 170, 168, 0.5);

        s.store_limited_exp(901, 171);

        s.b[1421] = (s.v[901] > 1e-7);
        s.v[1421] = if s.b[1421] { 1.0 } else { 0.0 };

        if s.b[1421] {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if s.b[1421] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if s.b[1421] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if s.b[1421] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if s.b[1421] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (!s.b[1421]) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        s.store_mul_neg_lhs(392, 901, 181);

        s.b[1422] = (p.p57 == 1.0);
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if s.b[1422] {
            s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs(1004, 1010, s.ad_value(1017), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs(1005, 1011, s.ad_value(1020), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs(1006, 1012, s.ad_value(1023), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(392, 983, 392, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);
        }

        s.store_div_from_scalar(406, 0.01, 163);

        s.store_add_scaled_product_indices(419, 396, s.v[420], 407, 392, s.v[420]);

        s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(392), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));

        s.store_pow_ad(171, s.ad_value(419), s.ad_value(822));

        s.b[1423] = (p.p61 != 0.0);
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if s.b[1423] {
            s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), 171, 1.0);
        }

        if (!s.b[1423]) {
            s.store_add_scaled_product_value_ad(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, 171, 1.0);
        }

        s.store_offset(397, 171, 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(397, 397, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);

        s.store_scale(397, 397, 1.0 / (p.p24));

        s.b[1424] = (p.p64 == 1.0);
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

        if s.b[1424] {
            s.store_scalar(198, 0.0);
        }

        s.b[1425] = (p.p64 == 0.0);
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

        if ((!s.b[1424]) && s.b[1425]) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_affine_product_lhs(198, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115], 0.0, 194);
        }

        if ((!s.b[1424]) && (!s.b[1425])) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_lhs(198, A::add_scaled_inputs_product(s.ad_value(190), 1.0, s.ad_value(191), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115]), 194);
        }

        s.store_mul_div_scaled_inputs_rhs(216, 397, s.ad_value(428), 2.0, s.ad_value(416), 1.0);

        s.store_mul(217, 216, 153);

        s.b[1426] = (p.p80 == 0.0);
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

        if s.b[1426] {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(392), 1.0, s.ad_value(179), 2.0);
        }

        if (!s.b[1426]) {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(392), 1.0, s.ad_value(182), 2.0);
        }

        s.b[1427] = (s.v[198] > 0.0);
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if s.b[1427] {
            s.store_mul3_lhs(224, 158, 428, 163);
            s.store_mul(168, 224, 198);
            s.store_scale(225, 168, 2.0);
            s.store_add_scaled_inputs_product_indices(226, 175, 1.0, 217, 1.0, 175, 168, 3.0);
            s.store_mul_add_scaled_product_rhs(227, 175, s.ad_value(217), 1.0, s.ad_value(175), s.ad_value(168), 2.0);
            s.store_div_scaled_inputs2(210, A::square(s.ad_value(226)), 1.0, A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)), (-1.0), A::mul(A::add(s.ad_value(226), A::sqrt(A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), s.ad_value(225)), 1.0);
        }

        if (!s.b[1427]) {
            s.store_div_scaled_product_add_scaled_denominator_indices(210, 217, 175, 1.0, 217, 1.0, 175, 1.0, 1.0);
        }

        s.store_offset_ad(210, {
            if (!((s.v[210] - 0.001) < ((-10000.0) * 1e-5))) {
                A::add_scaled_inputs(A::offset(s.ad_value(210), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(210), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5)
            } else {
                {
                    if ((s.v[210] - 0.001) < ((-10000.0) * 1e-5)) {
                        A::div_scalar_offset_denominator(((-1e-5) * 1e-5), s.ad_value(210), (-0.001), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.001);

        s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(210)), 1e-6), s.ad_value(423));

        s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));

        s.store_min_ad(390, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));

        s.store_add(129, 390, 375);

        s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);

        s.b[1428] = (p.p61 != 0.0);
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        if s.b[1428] {
            if (!((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 129, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if s.b[1428] {
            s.store_mul_ad(171, A::div_scaled_inputs(s.ad_value(239), -1.0, s.ad_value(181), 2.0), A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)));
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 170, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (!s.b[1428]) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 170, 1.0);
            s.store_sub(169, 900, 897);
        }

        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);

        s.store_sub(924, 169, 170);

        s.store_scaled_sub(171, 170, 168, 0.5);

        s.store_limited_exp(901, 171);

        s.b[1429] = (s.v[901] > 1e-7);
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if s.b[1429] {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1429] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if s.b[1429] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if s.b[1429] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if s.b[1429] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (!s.b[1429]) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        s.store_mul_neg_lhs(393, 901, 181);

        s.b[1430] = (p.p57 == 1.0);
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if s.b[1430] {
            s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs(1007, 1010, s.ad_value(1017), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs(1008, 1011, s.ad_value(1020), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs(1009, 1012, s.ad_value(1023), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(393, 983, 393, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0);
        }

        s.b[1431] = (p.p67 == 1.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if s.b[1431] {
            s.store_add_ad(356, A::mul3_scaled_output(s.ad_value(297), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));
            s.store_add_ad_lhs(359, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(356), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);
            s.store_add_scaled_inputs3_indices(349, 125, 1.0, 167, (-1.0), 359, -1.0);
            s.store_div_scaled_product3_indices(185, 414, 163, 158, 1.0, 153, 1.0);
        }

        s.b[1432] = (p.p80 == 0.0);
        s.v[1432] = if s.b[1432] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1432]) {
            s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(185), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0), s.ad_value(181));
        }

        if (s.b[1431] && s.b[1432]) {
            s.store_neg_ad(168, A::add(s.ad_value(375), {
                if (!(s.v[171] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[171] > 1e-38) {
                            A::ln(s.ad_value(171))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (s.b[1431] && s.b[1432]) {
            s.store_offset_add(169, 349, 168, p.p23);
        }

        if (s.b[1431] && s.b[1432]) {
            s.store_sub_ad_lhs(350, {
                if (!(s.v[169] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt_square_offset(s.ad_value(169), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 168);
        }

        if (s.b[1431] && (!s.b[1432])) {
            s.store_mul_scaled_ad_rhs(168, 181, -1.0, {
                if (!((((2.0 * s.v[163]) * p.p108) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p.p108) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(185), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1431] && (!s.b[1432])) {
            s.store_sub_ad_lhs(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);
            s.store_offset_add(170, 349, 169, p.p23);
        }

        if (s.b[1431] && (!s.b[1432])) {
            s.store_sub_ad_lhs(350, {
                if (!(s.v[170] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(170), 0.5, A::sqrt_square_offset(s.ad_value(170), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (s.v[170] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(170))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 169);
        }

        if s.b[1431] {
            s.copy_ad(130, 375);
            s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);
        }

        s.b[1433] = (p.p61 != 0.0);
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1433]) {
            if (!((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 130, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if (s.b[1431] && s.b[1433]) {
            s.store_mul_ad(171, A::div_scaled_inputs(s.ad_value(239), -1.0, s.ad_value(181), 2.0), A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)));
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (s.b[1431] && (!s.b[1433])) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);
            s.store_sub(169, 900, 897);
        }

        if s.b[1431] {
            s.store_div_scaled_inputs2_indices(170, 350, 1.0, 130, (-1.0), 181, 1.0);
            s.store_sub(924, 169, 170);
            s.store_scaled_sub(171, 170, 168, 0.5);
            s.store_limited_exp(901, 171);
        }

        s.b[1434] = (s.v[901] > 1e-7);
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1434]) {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (s.b[1431] && (!s.b[1434])) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        if s.b[1431] {
            s.store_mul_neg_lhs(394, 901, 181);
        }

        s.b[1435] = (p.p57 == 1.0);
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1435]) {
            s.store_div_scaled_inputs2_indices(1015, 349, 1.0, 130, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs(1004, 1010, s.ad_value(1017), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 349, 1.0, 130, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs(1005, 1011, s.ad_value(1020), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 349, 1.0, 130, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs(1006, 1012, s.ad_value(1023), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(394, 983, 394, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);
        }

        if s.b[1431] {
            s.store_add_scaled_product_indices(421, 396, s.v[420], 407, 394, s.v[420]);
        }

    }
}
