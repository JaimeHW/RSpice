#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
pub struct Parameters {
    pub level: f64,
    pub type_: f64,
    pub tr: f64,
    pub swgeo: f64,
    pub swigate: f64,
    pub swimpact: f64,
    pub swgidl: f64,
    pub swjuncap: f64,
    pub swjunasym: f64,
    pub swnud: f64,
    pub swedge: f64,
    pub swdelvtac: f64,
    pub swign: f64,
    pub qmc: f64,
    pub l: f64,
    pub w: f64,
    pub sa: f64,
    pub sb: f64,
    pub sd: f64,
    pub sca: f64,
    pub scb: f64,
    pub scc: f64,
    pub sc: f64,
    pub nf: f64,
    pub ngcon: f64,
    pub xgw: f64,
    pub nrs: f64,
    pub nrd: f64,
    pub jw: f64,
    pub delvto: f64,
    pub factuo: f64,
    pub delvtoedge: f64,
    pub factuoedge: f64,
    pub absource: f64,
    pub lssource: f64,
    pub lgsource: f64,
    pub abdrain: f64,
    pub lsdrain: f64,
    pub lgdrain: f64,
    pub as_: f64,
    pub ps: f64,
    pub ad: f64,
    pub pd: f64,
    pub mult: f64,
    pub vfb: f64,
    pub stvfb: f64,
    pub st2vfb: f64,
    pub tox: f64,
    pub epsrox: f64,
    pub neff: f64,
    pub facneffac: f64,
    pub gfacnud: f64,
    pub vsbnud: f64,
    pub dvsbnud: f64,
    pub vnsub: f64,
    pub nslp: f64,
    pub dnsub: f64,
    pub dphib: f64,
    pub delvtac: f64,
    pub np: f64,
    pub toxov: f64,
    pub toxovd: f64,
    pub nov: f64,
    pub novd: f64,
    pub ct: f64,
    pub ctg: f64,
    pub ctb: f64,
    pub stct: f64,
    pub cf: f64,
    pub cfd: f64,
    pub cfb: f64,
    pub psce: f64,
    pub psceb: f64,
    pub psced: f64,
    pub betn: f64,
    pub stbet: f64,
    pub mue: f64,
    pub stmue: f64,
    pub themu: f64,
    pub stthemu: f64,
    pub cs: f64,
    pub stcs: f64,
    pub thecs: f64,
    pub stthecs: f64,
    pub xcor: f64,
    pub stxcor: f64,
    pub feta: f64,
    pub rs: f64,
    pub strs: f64,
    pub rsb: f64,
    pub rsg: f64,
    pub thesat: f64,
    pub stthesat: f64,
    pub thesatb: f64,
    pub thesatg: f64,
    pub ax: f64,
    pub alp: f64,
    pub alp1: f64,
    pub alp2: f64,
    pub vp: f64,
    pub a1: f64,
    pub a2: f64,
    pub sta2: f64,
    pub a3: f64,
    pub a4: f64,
    pub gco: f64,
    pub iginv: f64,
    pub igov: f64,
    pub igovd: f64,
    pub stig: f64,
    pub gc2: f64,
    pub gc3: f64,
    pub chib: f64,
    pub agidl: f64,
    pub agidld: f64,
    pub bgidl: f64,
    pub bgidld: f64,
    pub stbgidl: f64,
    pub stbgidld: f64,
    pub cgidl: f64,
    pub cgidld: f64,
    pub cox: f64,
    pub cgov: f64,
    pub cgovd: f64,
    pub cgbov: f64,
    pub cfr: f64,
    pub cfrd: f64,
    pub fnt: f64,
    pub fntexc: f64,
    pub nfa: f64,
    pub nfb: f64,
    pub nfc: f64,
    pub ef: f64,
    pub vfbedge: f64,
    pub stvfbedge: f64,
    pub dphibedge: f64,
    pub neffedge: f64,
    pub ctedge: f64,
    pub betnedge: f64,
    pub stbetedge: f64,
    pub psceedge: f64,
    pub pscebedge: f64,
    pub pscededge: f64,
    pub cfedge: f64,
    pub cfdedge: f64,
    pub cfbedge: f64,
    pub fntedge: f64,
    pub nfaedge: f64,
    pub nfbedge: f64,
    pub nfcedge: f64,
    pub efedge: f64,
    pub rg: f64,
    pub rse: f64,
    pub rde: f64,
    pub rbulk: f64,
    pub rwell: f64,
    pub rjuns: f64,
    pub rjund: f64,
    pub povfb: f64,
    pub plvfb: f64,
    pub pwvfb: f64,
    pub plwvfb: f64,
    pub postvfb: f64,
    pub plstvfb: f64,
    pub pwstvfb: f64,
    pub plwstvfb: f64,
    pub post2vfb: f64,
    pub potox: f64,
    pub poepsrox: f64,
    pub poneff: f64,
    pub plneff: f64,
    pub pwneff: f64,
    pub plwneff: f64,
    pub pofacneffac: f64,
    pub plfacneffac: f64,
    pub pwfacneffac: f64,
    pub plwfacneffac: f64,
    pub pogfacnud: f64,
    pub plgfacnud: f64,
    pub pwgfacnud: f64,
    pub plwgfacnud: f64,
    pub povsbnud: f64,
    pub podvsbnud: f64,
    pub povnsub: f64,
    pub ponslp: f64,
    pub podnsub: f64,
    pub podphib: f64,
    pub pldphib: f64,
    pub pwdphib: f64,
    pub plwdphib: f64,
    pub podelvtac: f64,
    pub pldelvtac: f64,
    pub pwdelvtac: f64,
    pub plwdelvtac: f64,
    pub ponp: f64,
    pub plnp: f64,
    pub pwnp: f64,
    pub plwnp: f64,
    pub potoxov: f64,
    pub potoxovd: f64,
    pub ponov: f64,
    pub plnov: f64,
    pub pwnov: f64,
    pub plwnov: f64,
    pub ponovd: f64,
    pub plnovd: f64,
    pub pwnovd: f64,
    pub plwnovd: f64,
    pub poct: f64,
    pub plct: f64,
    pub pwct: f64,
    pub plwct: f64,
    pub poctg: f64,
    pub poctb: f64,
    pub postct: f64,
    pub pocf: f64,
    pub plcf: f64,
    pub pwcf: f64,
    pub plwcf: f64,
    pub pocfd: f64,
    pub pocfb: f64,
    pub popsce: f64,
    pub plpsce: f64,
    pub pwpsce: f64,
    pub plwpsce: f64,
    pub popsceb: f64,
    pub popsced: f64,
    pub pobetn: f64,
    pub plbetn: f64,
    pub pwbetn: f64,
    pub plwbetn: f64,
    pub postbet: f64,
    pub plstbet: f64,
    pub pwstbet: f64,
    pub plwstbet: f64,
    pub pomue: f64,
    pub plmue: f64,
    pub pwmue: f64,
    pub plwmue: f64,
    pub postmue: f64,
    pub pothemu: f64,
    pub postthemu: f64,
    pub pocs: f64,
    pub plcs: f64,
    pub pwcs: f64,
    pub plwcs: f64,
    pub postcs: f64,
    pub pothecs: f64,
    pub postthecs: f64,
    pub poxcor: f64,
    pub plxcor: f64,
    pub pwxcor: f64,
    pub plwxcor: f64,
    pub postxcor: f64,
    pub pofeta: f64,
    pub pors: f64,
    pub plrs: f64,
    pub pwrs: f64,
    pub plwrs: f64,
    pub postrs: f64,
    pub porsb: f64,
    pub porsg: f64,
    pub pothesat: f64,
    pub plthesat: f64,
    pub pwthesat: f64,
    pub plwthesat: f64,
    pub postthesat: f64,
    pub plstthesat: f64,
    pub pwstthesat: f64,
    pub plwstthesat: f64,
    pub pothesatb: f64,
    pub plthesatb: f64,
    pub pwthesatb: f64,
    pub plwthesatb: f64,
    pub pothesatg: f64,
    pub plthesatg: f64,
    pub pwthesatg: f64,
    pub plwthesatg: f64,
    pub poax: f64,
    pub plax: f64,
    pub pwax: f64,
    pub plwax: f64,
    pub poalp: f64,
    pub plalp: f64,
    pub pwalp: f64,
    pub plwalp: f64,
    pub poalp1: f64,
    pub plalp1: f64,
    pub pwalp1: f64,
    pub plwalp1: f64,
    pub poalp2: f64,
    pub plalp2: f64,
    pub pwalp2: f64,
    pub plwalp2: f64,
    pub povp: f64,
    pub poa1: f64,
    pub pla1: f64,
    pub pwa1: f64,
    pub plwa1: f64,
    pub poa2: f64,
    pub posta2: f64,
    pub poa3: f64,
    pub pla3: f64,
    pub pwa3: f64,
    pub plwa3: f64,
    pub poa4: f64,
    pub pla4: f64,
    pub pwa4: f64,
    pub plwa4: f64,
    pub pogco: f64,
    pub poiginv: f64,
    pub pliginv: f64,
    pub pwiginv: f64,
    pub plwiginv: f64,
    pub poigov: f64,
    pub pligov: f64,
    pub pwigov: f64,
    pub plwigov: f64,
    pub poigovd: f64,
    pub pligovd: f64,
    pub pwigovd: f64,
    pub plwigovd: f64,
    pub postig: f64,
    pub pogc2: f64,
    pub pogc3: f64,
    pub pochib: f64,
    pub poagidl: f64,
    pub plagidl: f64,
    pub pwagidl: f64,
    pub plwagidl: f64,
    pub poagidld: f64,
    pub plagidld: f64,
    pub pwagidld: f64,
    pub plwagidld: f64,
    pub pobgidl: f64,
    pub pobgidld: f64,
    pub postbgidl: f64,
    pub postbgidld: f64,
    pub pocgidl: f64,
    pub pocgidld: f64,
    pub pocox: f64,
    pub plcox: f64,
    pub pwcox: f64,
    pub plwcox: f64,
    pub pocgov: f64,
    pub plcgov: f64,
    pub pwcgov: f64,
    pub plwcgov: f64,
    pub pocgovd: f64,
    pub plcgovd: f64,
    pub pwcgovd: f64,
    pub plwcgovd: f64,
    pub pocgbov: f64,
    pub plcgbov: f64,
    pub pwcgbov: f64,
    pub plwcgbov: f64,
    pub pocfr: f64,
    pub plcfr: f64,
    pub pwcfr: f64,
    pub plwcfr: f64,
    pub pocfrd: f64,
    pub plcfrd: f64,
    pub pwcfrd: f64,
    pub plwcfrd: f64,
    pub pofnt: f64,
    pub pofntexc: f64,
    pub plfntexc: f64,
    pub pwfntexc: f64,
    pub plwfntexc: f64,
    pub ponfa: f64,
    pub plnfa: f64,
    pub pwnfa: f64,
    pub plwnfa: f64,
    pub ponfb: f64,
    pub plnfb: f64,
    pub pwnfb: f64,
    pub plwnfb: f64,
    pub ponfc: f64,
    pub plnfc: f64,
    pub pwnfc: f64,
    pub plwnfc: f64,
    pub poef: f64,
    pub povfbedge: f64,
    pub postvfbedge: f64,
    pub plstvfbedge: f64,
    pub pwstvfbedge: f64,
    pub plwstvfbedge: f64,
    pub podphibedge: f64,
    pub pldphibedge: f64,
    pub pwdphibedge: f64,
    pub plwdphibedge: f64,
    pub poneffedge: f64,
    pub plneffedge: f64,
    pub pwneffedge: f64,
    pub plwneffedge: f64,
    pub poctedge: f64,
    pub plctedge: f64,
    pub pwctedge: f64,
    pub plwctedge: f64,
    pub pobetnedge: f64,
    pub plbetnedge: f64,
    pub pwbetnedge: f64,
    pub plwbetnedge: f64,
    pub postbetedge: f64,
    pub plstbetedge: f64,
    pub pwstbetedge: f64,
    pub plwstbetedge: f64,
    pub popsceedge: f64,
    pub plpsceedge: f64,
    pub pwpsceedge: f64,
    pub plwpsceedge: f64,
    pub popscebedge: f64,
    pub popscededge: f64,
    pub pocfedge: f64,
    pub plcfedge: f64,
    pub pwcfedge: f64,
    pub plwcfedge: f64,
    pub pocfdedge: f64,
    pub pocfbedge: f64,
    pub pofntedge: f64,
    pub ponfaedge: f64,
    pub plnfaedge: f64,
    pub pwnfaedge: f64,
    pub plwnfaedge: f64,
    pub ponfbedge: f64,
    pub plnfbedge: f64,
    pub pwnfbedge: f64,
    pub plwnfbedge: f64,
    pub ponfcedge: f64,
    pub plnfcedge: f64,
    pub pwnfcedge: f64,
    pub plwnfcedge: f64,
    pub poefedge: f64,
    pub pokvthowe: f64,
    pub plkvthowe: f64,
    pub pwkvthowe: f64,
    pub plwkvthowe: f64,
    pub pokuowe: f64,
    pub plkuowe: f64,
    pub pwkuowe: f64,
    pub plwkuowe: f64,
    pub lmin: f64,
    pub lmax: f64,
    pub wmin: f64,
    pub wmax: f64,
    pub lvaro: f64,
    pub lvarl: f64,
    pub lvarw: f64,
    pub lap: f64,
    pub wvaro: f64,
    pub wvarl: f64,
    pub wvarw: f64,
    pub wot: f64,
    pub dlq: f64,
    pub dwq: f64,
    pub vfbo: f64,
    pub vfbl: f64,
    pub vfbw: f64,
    pub vfblw: f64,
    pub stvfbo: f64,
    pub stvfbl: f64,
    pub stvfbw: f64,
    pub stvfblw: f64,
    pub st2vfbo: f64,
    pub toxo: f64,
    pub epsroxo: f64,
    pub nsubo: f64,
    pub nsubw: f64,
    pub wseg: f64,
    pub npck: f64,
    pub npckw: f64,
    pub wsegp: f64,
    pub lpck: f64,
    pub lpckw: f64,
    pub fol1: f64,
    pub fol2: f64,
    pub facneffaco: f64,
    pub facneffacl: f64,
    pub facneffacw: f64,
    pub facneffaclw: f64,
    pub gfacnudo: f64,
    pub gfacnudl: f64,
    pub gfacnudlexp: f64,
    pub gfacnudw: f64,
    pub gfacnudlw: f64,
    pub vsbnudo: f64,
    pub dvsbnudo: f64,
    pub vnsubo: f64,
    pub nslpo: f64,
    pub dnsubo: f64,
    pub dphibo: f64,
    pub dphibl: f64,
    pub dphiblexp: f64,
    pub dphibw: f64,
    pub dphiblw: f64,
    pub delvtaco: f64,
    pub delvtacl: f64,
    pub delvtaclexp: f64,
    pub delvtacw: f64,
    pub delvtaclw: f64,
    pub npo: f64,
    pub npl: f64,
    pub toxovo: f64,
    pub toxovdo: f64,
    pub lov: f64,
    pub lovd: f64,
    pub novo: f64,
    pub novdo: f64,
    pub cto: f64,
    pub ctl: f64,
    pub ctlexp: f64,
    pub ctw: f64,
    pub ctlw: f64,
    pub ctgo: f64,
    pub ctbo: f64,
    pub stcto: f64,
    pub cfl: f64,
    pub cflexp: f64,
    pub cfw: f64,
    pub cfdo: f64,
    pub cfbo: f64,
    pub pscel: f64,
    pub pscelexp: f64,
    pub pscew: f64,
    pub pscebo: f64,
    pub pscedo: f64,
    pub uo: f64,
    pub fbet1: f64,
    pub fbet1w: f64,
    pub lp1: f64,
    pub lp1w: f64,
    pub fbet2: f64,
    pub lp2: f64,
    pub betw1: f64,
    pub betw2: f64,
    pub wbet: f64,
    pub stbeto: f64,
    pub stbetl: f64,
    pub stbetw: f64,
    pub stbetlw: f64,
    pub mueo: f64,
    pub muew: f64,
    pub stmueo: f64,
    pub themuo: f64,
    pub stthemuo: f64,
    pub cso: f64,
    pub csl: f64,
    pub cslexp: f64,
    pub csw: f64,
    pub cslw: f64,
    pub stcso: f64,
    pub thecso: f64,
    pub stthecso: f64,
    pub xcoro: f64,
    pub xcorl: f64,
    pub xcorw: f64,
    pub xcorlw: f64,
    pub stxcoro: f64,
    pub fetao: f64,
    pub rsw1: f64,
    pub rsw2: f64,
    pub strso: f64,
    pub rsbo: f64,
    pub rsgo: f64,
    pub thesato: f64,
    pub thesatl: f64,
    pub thesatlexp: f64,
    pub thesatw: f64,
    pub thesatlw: f64,
    pub stthesato: f64,
    pub stthesatl: f64,
    pub stthesatw: f64,
    pub stthesatlw: f64,
    pub thesatbo: f64,
    pub thesatgo: f64,
    pub axo: f64,
    pub axl: f64,
    pub alpl: f64,
    pub alplexp: f64,
    pub alpw: f64,
    pub alp1l1: f64,
    pub alp1lexp: f64,
    pub alp1l2: f64,
    pub alp1w: f64,
    pub alp2l1: f64,
    pub alp2lexp: f64,
    pub alp2l2: f64,
    pub alp2w: f64,
    pub vpo: f64,
    pub a1o: f64,
    pub a1l: f64,
    pub a1w: f64,
    pub a2o: f64,
    pub sta2o: f64,
    pub a3o: f64,
    pub a3l: f64,
    pub a3w: f64,
    pub a4o: f64,
    pub a4l: f64,
    pub a4w: f64,
    pub gcoo: f64,
    pub iginvlw: f64,
    pub igovw: f64,
    pub igovdw: f64,
    pub stigo: f64,
    pub gc2o: f64,
    pub gc3o: f64,
    pub chibo: f64,
    pub agidlw: f64,
    pub agidldw: f64,
    pub bgidlo: f64,
    pub bgidldo: f64,
    pub stbgidlo: f64,
    pub stbgidldo: f64,
    pub cgidlo: f64,
    pub cgidldo: f64,
    pub cgbovl: f64,
    pub cfrw: f64,
    pub cfrdw: f64,
    pub fnto: f64,
    pub fntexcl: f64,
    pub nfalw: f64,
    pub nfblw: f64,
    pub nfclw: f64,
    pub efo: f64,
    pub lintnoi: f64,
    pub alpnoi: f64,
    pub wedge: f64,
    pub wedgew: f64,
    pub vfbedgeo: f64,
    pub stvfbedgeo: f64,
    pub stvfbedgel: f64,
    pub stvfbedgew: f64,
    pub stvfbedgelw: f64,
    pub dphibedgeo: f64,
    pub dphibedgel: f64,
    pub dphibedgelexp: f64,
    pub dphibedgew: f64,
    pub dphibedgelw: f64,
    pub nsubedgeo: f64,
    pub nsubedgel: f64,
    pub nsubedgelexp: f64,
    pub nsubedgew: f64,
    pub nsubedgelw: f64,
    pub ctedgeo: f64,
    pub ctedgel: f64,
    pub ctedgelexp: f64,
    pub fbetedge: f64,
    pub lpedge: f64,
    pub betedgew: f64,
    pub stbetedgeo: f64,
    pub stbetedgel: f64,
    pub stbetedgew: f64,
    pub stbetedgelw: f64,
    pub psceedgel: f64,
    pub psceedgelexp: f64,
    pub psceedgew: f64,
    pub pscebedgeo: f64,
    pub pscededgeo: f64,
    pub cfedgel: f64,
    pub cfedgelexp: f64,
    pub cfedgew: f64,
    pub cfdedgeo: f64,
    pub cfbedgeo: f64,
    pub fntedgeo: f64,
    pub nfaedgelw: f64,
    pub nfbedgelw: f64,
    pub nfcedgelw: f64,
    pub efedgeo: f64,
    pub kvthoweo: f64,
    pub kvthowel: f64,
    pub kvthowew: f64,
    pub kvthowelw: f64,
    pub kuoweo: f64,
    pub kuowel: f64,
    pub kuowew: f64,
    pub kuowelw: f64,
    pub rgo: f64,
    pub rint: f64,
    pub rvpoly: f64,
    pub rshg: f64,
    pub dlsil: f64,
    pub rsh: f64,
    pub rshd: f64,
    pub rbulko: f64,
    pub rwello: f64,
    pub rjunso: f64,
    pub rjundo: f64,
    pub saref: f64,
    pub sbref: f64,
    pub wlod: f64,
    pub kuo: f64,
    pub kvsat: f64,
    pub tkuo: f64,
    pub lkuo: f64,
    pub wkuo: f64,
    pub pkuo: f64,
    pub llodkuo: f64,
    pub wlodkuo: f64,
    pub kvtho: f64,
    pub lkvtho: f64,
    pub wkvtho: f64,
    pub pkvtho: f64,
    pub llodvth: f64,
    pub wlodvth: f64,
    pub stetao: f64,
    pub lodetao: f64,
    pub scref: f64,
    pub web: f64,
    pub wec: f64,
    pub swsoa: f64,
    pub vgs_max: f64,
    pub vgd_max: f64,
    pub vgb_max: f64,
    pub vds_max: f64,
    pub vdb_max: f64,
    pub vsb_max: f64,
    pub imax: f64,
    pub trj: f64,
    pub frev: f64,
    pub cjorbot: f64,
    pub cjorsti: f64,
    pub cjorgat: f64,
    pub vbirbot: f64,
    pub vbirsti: f64,
    pub vbirgat: f64,
    pub pbot: f64,
    pub psti: f64,
    pub pgat: f64,
    pub phigbot: f64,
    pub phigsti: f64,
    pub phiggat: f64,
    pub idsatrbot: f64,
    pub idsatrsti: f64,
    pub idsatrgat: f64,
    pub csrhbot: f64,
    pub csrhsti: f64,
    pub csrhgat: f64,
    pub xjunsti: f64,
    pub xjungat: f64,
    pub ctatbot: f64,
    pub ctatsti: f64,
    pub ctatgat: f64,
    pub mefftatbot: f64,
    pub mefftatsti: f64,
    pub mefftatgat: f64,
    pub cbbtbot: f64,
    pub cbbtsti: f64,
    pub cbbtgat: f64,
    pub fbbtrbot: f64,
    pub fbbtrsti: f64,
    pub fbbtrgat: f64,
    pub stfbbtbot: f64,
    pub stfbbtsti: f64,
    pub stfbbtgat: f64,
    pub vbrbot: f64,
    pub vbrsti: f64,
    pub vbrgat: f64,
    pub pbrbot: f64,
    pub pbrsti: f64,
    pub pbrgat: f64,
    pub cjorbotd: f64,
    pub cjorstid: f64,
    pub cjorgatd: f64,
    pub vbirbotd: f64,
    pub vbirstid: f64,
    pub vbirgatd: f64,
    pub pbotd: f64,
    pub pstid: f64,
    pub pgatd: f64,
    pub phigbotd: f64,
    pub phigstid: f64,
    pub phiggatd: f64,
    pub idsatrbotd: f64,
    pub idsatrstid: f64,
    pub idsatrgatd: f64,
    pub csrhbotd: f64,
    pub csrhstid: f64,
    pub csrhgatd: f64,
    pub xjunstid: f64,
    pub xjungatd: f64,
    pub ctatbotd: f64,
    pub ctatstid: f64,
    pub ctatgatd: f64,
    pub mefftatbotd: f64,
    pub mefftatstid: f64,
    pub mefftatgatd: f64,
    pub cbbtbotd: f64,
    pub cbbtstid: f64,
    pub cbbtgatd: f64,
    pub fbbtrbotd: f64,
    pub fbbtrstid: f64,
    pub fbbtrgatd: f64,
    pub stfbbtbotd: f64,
    pub stfbbtstid: f64,
    pub stfbbtgatd: f64,
    pub vbrbotd: f64,
    pub vbrstid: f64,
    pub vbrgatd: f64,
    pub pbrbotd: f64,
    pub pbrstid: f64,
    pub pbrgatd: f64,
    pub swjunexp: f64,
    pub vjunref: f64,
    pub fjunq: f64,
    pub vjunrefd: f64,
    pub fjunqd: f64,
    pub dta: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        let mut params = Self {
            level: 0.0,
            type_: 0.0,
            tr: 0.0,
            swgeo: 0.0,
            swigate: 0.0,
            swimpact: 0.0,
            swgidl: 0.0,
            swjuncap: 0.0,
            swjunasym: 0.0,
            swnud: 0.0,
            swedge: 0.0,
            swdelvtac: 0.0,
            swign: 0.0,
            qmc: 0.0,
            l: 0.0,
            w: 0.0,
            sa: 0.0,
            sb: 0.0,
            sd: 0.0,
            sca: 0.0,
            scb: 0.0,
            scc: 0.0,
            sc: 0.0,
            nf: 0.0,
            ngcon: 0.0,
            xgw: 0.0,
            nrs: 0.0,
            nrd: 0.0,
            jw: 0.0,
            delvto: 0.0,
            factuo: 0.0,
            delvtoedge: 0.0,
            factuoedge: 0.0,
            absource: 0.0,
            lssource: 0.0,
            lgsource: 0.0,
            abdrain: 0.0,
            lsdrain: 0.0,
            lgdrain: 0.0,
            as_: 0.0,
            ps: 0.0,
            ad: 0.0,
            pd: 0.0,
            mult: 0.0,
            vfb: 0.0,
            stvfb: 0.0,
            st2vfb: 0.0,
            tox: 0.0,
            epsrox: 0.0,
            neff: 0.0,
            facneffac: 0.0,
            gfacnud: 0.0,
            vsbnud: 0.0,
            dvsbnud: 0.0,
            vnsub: 0.0,
            nslp: 0.0,
            dnsub: 0.0,
            dphib: 0.0,
            delvtac: 0.0,
            np: 0.0,
            toxov: 0.0,
            toxovd: 0.0,
            nov: 0.0,
            novd: 0.0,
            ct: 0.0,
            ctg: 0.0,
            ctb: 0.0,
            stct: 0.0,
            cf: 0.0,
            cfd: 0.0,
            cfb: 0.0,
            psce: 0.0,
            psceb: 0.0,
            psced: 0.0,
            betn: 0.0,
            stbet: 0.0,
            mue: 0.0,
            stmue: 0.0,
            themu: 0.0,
            stthemu: 0.0,
            cs: 0.0,
            stcs: 0.0,
            thecs: 0.0,
            stthecs: 0.0,
            xcor: 0.0,
            stxcor: 0.0,
            feta: 0.0,
            rs: 0.0,
            strs: 0.0,
            rsb: 0.0,
            rsg: 0.0,
            thesat: 0.0,
            stthesat: 0.0,
            thesatb: 0.0,
            thesatg: 0.0,
            ax: 0.0,
            alp: 0.0,
            alp1: 0.0,
            alp2: 0.0,
            vp: 0.0,
            a1: 0.0,
            a2: 0.0,
            sta2: 0.0,
            a3: 0.0,
            a4: 0.0,
            gco: 0.0,
            iginv: 0.0,
            igov: 0.0,
            igovd: 0.0,
            stig: 0.0,
            gc2: 0.0,
            gc3: 0.0,
            chib: 0.0,
            agidl: 0.0,
            agidld: 0.0,
            bgidl: 0.0,
            bgidld: 0.0,
            stbgidl: 0.0,
            stbgidld: 0.0,
            cgidl: 0.0,
            cgidld: 0.0,
            cox: 0.0,
            cgov: 0.0,
            cgovd: 0.0,
            cgbov: 0.0,
            cfr: 0.0,
            cfrd: 0.0,
            fnt: 0.0,
            fntexc: 0.0,
            nfa: 0.0,
            nfb: 0.0,
            nfc: 0.0,
            ef: 0.0,
            vfbedge: 0.0,
            stvfbedge: 0.0,
            dphibedge: 0.0,
            neffedge: 0.0,
            ctedge: 0.0,
            betnedge: 0.0,
            stbetedge: 0.0,
            psceedge: 0.0,
            pscebedge: 0.0,
            pscededge: 0.0,
            cfedge: 0.0,
            cfdedge: 0.0,
            cfbedge: 0.0,
            fntedge: 0.0,
            nfaedge: 0.0,
            nfbedge: 0.0,
            nfcedge: 0.0,
            efedge: 0.0,
            rg: 0.0,
            rse: 0.0,
            rde: 0.0,
            rbulk: 0.0,
            rwell: 0.0,
            rjuns: 0.0,
            rjund: 0.0,
            povfb: 0.0,
            plvfb: 0.0,
            pwvfb: 0.0,
            plwvfb: 0.0,
            postvfb: 0.0,
            plstvfb: 0.0,
            pwstvfb: 0.0,
            plwstvfb: 0.0,
            post2vfb: 0.0,
            potox: 0.0,
            poepsrox: 0.0,
            poneff: 0.0,
            plneff: 0.0,
            pwneff: 0.0,
            plwneff: 0.0,
            pofacneffac: 0.0,
            plfacneffac: 0.0,
            pwfacneffac: 0.0,
            plwfacneffac: 0.0,
            pogfacnud: 0.0,
            plgfacnud: 0.0,
            pwgfacnud: 0.0,
            plwgfacnud: 0.0,
            povsbnud: 0.0,
            podvsbnud: 0.0,
            povnsub: 0.0,
            ponslp: 0.0,
            podnsub: 0.0,
            podphib: 0.0,
            pldphib: 0.0,
            pwdphib: 0.0,
            plwdphib: 0.0,
            podelvtac: 0.0,
            pldelvtac: 0.0,
            pwdelvtac: 0.0,
            plwdelvtac: 0.0,
            ponp: 0.0,
            plnp: 0.0,
            pwnp: 0.0,
            plwnp: 0.0,
            potoxov: 0.0,
            potoxovd: 0.0,
            ponov: 0.0,
            plnov: 0.0,
            pwnov: 0.0,
            plwnov: 0.0,
            ponovd: 0.0,
            plnovd: 0.0,
            pwnovd: 0.0,
            plwnovd: 0.0,
            poct: 0.0,
            plct: 0.0,
            pwct: 0.0,
            plwct: 0.0,
            poctg: 0.0,
            poctb: 0.0,
            postct: 0.0,
            pocf: 0.0,
            plcf: 0.0,
            pwcf: 0.0,
            plwcf: 0.0,
            pocfd: 0.0,
            pocfb: 0.0,
            popsce: 0.0,
            plpsce: 0.0,
            pwpsce: 0.0,
            plwpsce: 0.0,
            popsceb: 0.0,
            popsced: 0.0,
            pobetn: 0.0,
            plbetn: 0.0,
            pwbetn: 0.0,
            plwbetn: 0.0,
            postbet: 0.0,
            plstbet: 0.0,
            pwstbet: 0.0,
            plwstbet: 0.0,
            pomue: 0.0,
            plmue: 0.0,
            pwmue: 0.0,
            plwmue: 0.0,
            postmue: 0.0,
            pothemu: 0.0,
            postthemu: 0.0,
            pocs: 0.0,
            plcs: 0.0,
            pwcs: 0.0,
            plwcs: 0.0,
            postcs: 0.0,
            pothecs: 0.0,
            postthecs: 0.0,
            poxcor: 0.0,
            plxcor: 0.0,
            pwxcor: 0.0,
            plwxcor: 0.0,
            postxcor: 0.0,
            pofeta: 0.0,
            pors: 0.0,
            plrs: 0.0,
            pwrs: 0.0,
            plwrs: 0.0,
            postrs: 0.0,
            porsb: 0.0,
            porsg: 0.0,
            pothesat: 0.0,
            plthesat: 0.0,
            pwthesat: 0.0,
            plwthesat: 0.0,
            postthesat: 0.0,
            plstthesat: 0.0,
            pwstthesat: 0.0,
            plwstthesat: 0.0,
            pothesatb: 0.0,
            plthesatb: 0.0,
            pwthesatb: 0.0,
            plwthesatb: 0.0,
            pothesatg: 0.0,
            plthesatg: 0.0,
            pwthesatg: 0.0,
            plwthesatg: 0.0,
            poax: 0.0,
            plax: 0.0,
            pwax: 0.0,
            plwax: 0.0,
            poalp: 0.0,
            plalp: 0.0,
            pwalp: 0.0,
            plwalp: 0.0,
            poalp1: 0.0,
            plalp1: 0.0,
            pwalp1: 0.0,
            plwalp1: 0.0,
            poalp2: 0.0,
            plalp2: 0.0,
            pwalp2: 0.0,
            plwalp2: 0.0,
            povp: 0.0,
            poa1: 0.0,
            pla1: 0.0,
            pwa1: 0.0,
            plwa1: 0.0,
            poa2: 0.0,
            posta2: 0.0,
            poa3: 0.0,
            pla3: 0.0,
            pwa3: 0.0,
            plwa3: 0.0,
            poa4: 0.0,
            pla4: 0.0,
            pwa4: 0.0,
            plwa4: 0.0,
            pogco: 0.0,
            poiginv: 0.0,
            pliginv: 0.0,
            pwiginv: 0.0,
            plwiginv: 0.0,
            poigov: 0.0,
            pligov: 0.0,
            pwigov: 0.0,
            plwigov: 0.0,
            poigovd: 0.0,
            pligovd: 0.0,
            pwigovd: 0.0,
            plwigovd: 0.0,
            postig: 0.0,
            pogc2: 0.0,
            pogc3: 0.0,
            pochib: 0.0,
            poagidl: 0.0,
            plagidl: 0.0,
            pwagidl: 0.0,
            plwagidl: 0.0,
            poagidld: 0.0,
            plagidld: 0.0,
            pwagidld: 0.0,
            plwagidld: 0.0,
            pobgidl: 0.0,
            pobgidld: 0.0,
            postbgidl: 0.0,
            postbgidld: 0.0,
            pocgidl: 0.0,
            pocgidld: 0.0,
            pocox: 0.0,
            plcox: 0.0,
            pwcox: 0.0,
            plwcox: 0.0,
            pocgov: 0.0,
            plcgov: 0.0,
            pwcgov: 0.0,
            plwcgov: 0.0,
            pocgovd: 0.0,
            plcgovd: 0.0,
            pwcgovd: 0.0,
            plwcgovd: 0.0,
            pocgbov: 0.0,
            plcgbov: 0.0,
            pwcgbov: 0.0,
            plwcgbov: 0.0,
            pocfr: 0.0,
            plcfr: 0.0,
            pwcfr: 0.0,
            plwcfr: 0.0,
            pocfrd: 0.0,
            plcfrd: 0.0,
            pwcfrd: 0.0,
            plwcfrd: 0.0,
            pofnt: 0.0,
            pofntexc: 0.0,
            plfntexc: 0.0,
            pwfntexc: 0.0,
            plwfntexc: 0.0,
            ponfa: 0.0,
            plnfa: 0.0,
            pwnfa: 0.0,
            plwnfa: 0.0,
            ponfb: 0.0,
            plnfb: 0.0,
            pwnfb: 0.0,
            plwnfb: 0.0,
            ponfc: 0.0,
            plnfc: 0.0,
            pwnfc: 0.0,
            plwnfc: 0.0,
            poef: 0.0,
            povfbedge: 0.0,
            postvfbedge: 0.0,
            plstvfbedge: 0.0,
            pwstvfbedge: 0.0,
            plwstvfbedge: 0.0,
            podphibedge: 0.0,
            pldphibedge: 0.0,
            pwdphibedge: 0.0,
            plwdphibedge: 0.0,
            poneffedge: 0.0,
            plneffedge: 0.0,
            pwneffedge: 0.0,
            plwneffedge: 0.0,
            poctedge: 0.0,
            plctedge: 0.0,
            pwctedge: 0.0,
            plwctedge: 0.0,
            pobetnedge: 0.0,
            plbetnedge: 0.0,
            pwbetnedge: 0.0,
            plwbetnedge: 0.0,
            postbetedge: 0.0,
            plstbetedge: 0.0,
            pwstbetedge: 0.0,
            plwstbetedge: 0.0,
            popsceedge: 0.0,
            plpsceedge: 0.0,
            pwpsceedge: 0.0,
            plwpsceedge: 0.0,
            popscebedge: 0.0,
            popscededge: 0.0,
            pocfedge: 0.0,
            plcfedge: 0.0,
            pwcfedge: 0.0,
            plwcfedge: 0.0,
            pocfdedge: 0.0,
            pocfbedge: 0.0,
            pofntedge: 0.0,
            ponfaedge: 0.0,
            plnfaedge: 0.0,
            pwnfaedge: 0.0,
            plwnfaedge: 0.0,
            ponfbedge: 0.0,
            plnfbedge: 0.0,
            pwnfbedge: 0.0,
            plwnfbedge: 0.0,
            ponfcedge: 0.0,
            plnfcedge: 0.0,
            pwnfcedge: 0.0,
            plwnfcedge: 0.0,
            poefedge: 0.0,
            pokvthowe: 0.0,
            plkvthowe: 0.0,
            pwkvthowe: 0.0,
            plwkvthowe: 0.0,
            pokuowe: 0.0,
            plkuowe: 0.0,
            pwkuowe: 0.0,
            plwkuowe: 0.0,
            lmin: 0.0,
            lmax: 0.0,
            wmin: 0.0,
            wmax: 0.0,
            lvaro: 0.0,
            lvarl: 0.0,
            lvarw: 0.0,
            lap: 0.0,
            wvaro: 0.0,
            wvarl: 0.0,
            wvarw: 0.0,
            wot: 0.0,
            dlq: 0.0,
            dwq: 0.0,
            vfbo: 0.0,
            vfbl: 0.0,
            vfbw: 0.0,
            vfblw: 0.0,
            stvfbo: 0.0,
            stvfbl: 0.0,
            stvfbw: 0.0,
            stvfblw: 0.0,
            st2vfbo: 0.0,
            toxo: 0.0,
            epsroxo: 0.0,
            nsubo: 0.0,
            nsubw: 0.0,
            wseg: 0.0,
            npck: 0.0,
            npckw: 0.0,
            wsegp: 0.0,
            lpck: 0.0,
            lpckw: 0.0,
            fol1: 0.0,
            fol2: 0.0,
            facneffaco: 0.0,
            facneffacl: 0.0,
            facneffacw: 0.0,
            facneffaclw: 0.0,
            gfacnudo: 0.0,
            gfacnudl: 0.0,
            gfacnudlexp: 0.0,
            gfacnudw: 0.0,
            gfacnudlw: 0.0,
            vsbnudo: 0.0,
            dvsbnudo: 0.0,
            vnsubo: 0.0,
            nslpo: 0.0,
            dnsubo: 0.0,
            dphibo: 0.0,
            dphibl: 0.0,
            dphiblexp: 0.0,
            dphibw: 0.0,
            dphiblw: 0.0,
            delvtaco: 0.0,
            delvtacl: 0.0,
            delvtaclexp: 0.0,
            delvtacw: 0.0,
            delvtaclw: 0.0,
            npo: 0.0,
            npl: 0.0,
            toxovo: 0.0,
            toxovdo: 0.0,
            lov: 0.0,
            lovd: 0.0,
            novo: 0.0,
            novdo: 0.0,
            cto: 0.0,
            ctl: 0.0,
            ctlexp: 0.0,
            ctw: 0.0,
            ctlw: 0.0,
            ctgo: 0.0,
            ctbo: 0.0,
            stcto: 0.0,
            cfl: 0.0,
            cflexp: 0.0,
            cfw: 0.0,
            cfdo: 0.0,
            cfbo: 0.0,
            pscel: 0.0,
            pscelexp: 0.0,
            pscew: 0.0,
            pscebo: 0.0,
            pscedo: 0.0,
            uo: 0.0,
            fbet1: 0.0,
            fbet1w: 0.0,
            lp1: 0.0,
            lp1w: 0.0,
            fbet2: 0.0,
            lp2: 0.0,
            betw1: 0.0,
            betw2: 0.0,
            wbet: 0.0,
            stbeto: 0.0,
            stbetl: 0.0,
            stbetw: 0.0,
            stbetlw: 0.0,
            mueo: 0.0,
            muew: 0.0,
            stmueo: 0.0,
            themuo: 0.0,
            stthemuo: 0.0,
            cso: 0.0,
            csl: 0.0,
            cslexp: 0.0,
            csw: 0.0,
            cslw: 0.0,
            stcso: 0.0,
            thecso: 0.0,
            stthecso: 0.0,
            xcoro: 0.0,
            xcorl: 0.0,
            xcorw: 0.0,
            xcorlw: 0.0,
            stxcoro: 0.0,
            fetao: 0.0,
            rsw1: 0.0,
            rsw2: 0.0,
            strso: 0.0,
            rsbo: 0.0,
            rsgo: 0.0,
            thesato: 0.0,
            thesatl: 0.0,
            thesatlexp: 0.0,
            thesatw: 0.0,
            thesatlw: 0.0,
            stthesato: 0.0,
            stthesatl: 0.0,
            stthesatw: 0.0,
            stthesatlw: 0.0,
            thesatbo: 0.0,
            thesatgo: 0.0,
            axo: 0.0,
            axl: 0.0,
            alpl: 0.0,
            alplexp: 0.0,
            alpw: 0.0,
            alp1l1: 0.0,
            alp1lexp: 0.0,
            alp1l2: 0.0,
            alp1w: 0.0,
            alp2l1: 0.0,
            alp2lexp: 0.0,
            alp2l2: 0.0,
            alp2w: 0.0,
            vpo: 0.0,
            a1o: 0.0,
            a1l: 0.0,
            a1w: 0.0,
            a2o: 0.0,
            sta2o: 0.0,
            a3o: 0.0,
            a3l: 0.0,
            a3w: 0.0,
            a4o: 0.0,
            a4l: 0.0,
            a4w: 0.0,
            gcoo: 0.0,
            iginvlw: 0.0,
            igovw: 0.0,
            igovdw: 0.0,
            stigo: 0.0,
            gc2o: 0.0,
            gc3o: 0.0,
            chibo: 0.0,
            agidlw: 0.0,
            agidldw: 0.0,
            bgidlo: 0.0,
            bgidldo: 0.0,
            stbgidlo: 0.0,
            stbgidldo: 0.0,
            cgidlo: 0.0,
            cgidldo: 0.0,
            cgbovl: 0.0,
            cfrw: 0.0,
            cfrdw: 0.0,
            fnto: 0.0,
            fntexcl: 0.0,
            nfalw: 0.0,
            nfblw: 0.0,
            nfclw: 0.0,
            efo: 0.0,
            lintnoi: 0.0,
            alpnoi: 0.0,
            wedge: 0.0,
            wedgew: 0.0,
            vfbedgeo: 0.0,
            stvfbedgeo: 0.0,
            stvfbedgel: 0.0,
            stvfbedgew: 0.0,
            stvfbedgelw: 0.0,
            dphibedgeo: 0.0,
            dphibedgel: 0.0,
            dphibedgelexp: 0.0,
            dphibedgew: 0.0,
            dphibedgelw: 0.0,
            nsubedgeo: 0.0,
            nsubedgel: 0.0,
            nsubedgelexp: 0.0,
            nsubedgew: 0.0,
            nsubedgelw: 0.0,
            ctedgeo: 0.0,
            ctedgel: 0.0,
            ctedgelexp: 0.0,
            fbetedge: 0.0,
            lpedge: 0.0,
            betedgew: 0.0,
            stbetedgeo: 0.0,
            stbetedgel: 0.0,
            stbetedgew: 0.0,
            stbetedgelw: 0.0,
            psceedgel: 0.0,
            psceedgelexp: 0.0,
            psceedgew: 0.0,
            pscebedgeo: 0.0,
            pscededgeo: 0.0,
            cfedgel: 0.0,
            cfedgelexp: 0.0,
            cfedgew: 0.0,
            cfdedgeo: 0.0,
            cfbedgeo: 0.0,
            fntedgeo: 0.0,
            nfaedgelw: 0.0,
            nfbedgelw: 0.0,
            nfcedgelw: 0.0,
            efedgeo: 0.0,
            kvthoweo: 0.0,
            kvthowel: 0.0,
            kvthowew: 0.0,
            kvthowelw: 0.0,
            kuoweo: 0.0,
            kuowel: 0.0,
            kuowew: 0.0,
            kuowelw: 0.0,
            rgo: 0.0,
            rint: 0.0,
            rvpoly: 0.0,
            rshg: 0.0,
            dlsil: 0.0,
            rsh: 0.0,
            rshd: 0.0,
            rbulko: 0.0,
            rwello: 0.0,
            rjunso: 0.0,
            rjundo: 0.0,
            saref: 0.0,
            sbref: 0.0,
            wlod: 0.0,
            kuo: 0.0,
            kvsat: 0.0,
            tkuo: 0.0,
            lkuo: 0.0,
            wkuo: 0.0,
            pkuo: 0.0,
            llodkuo: 0.0,
            wlodkuo: 0.0,
            kvtho: 0.0,
            lkvtho: 0.0,
            wkvtho: 0.0,
            pkvtho: 0.0,
            llodvth: 0.0,
            wlodvth: 0.0,
            stetao: 0.0,
            lodetao: 0.0,
            scref: 0.0,
            web: 0.0,
            wec: 0.0,
            swsoa: 0.0,
            vgs_max: 0.0,
            vgd_max: 0.0,
            vgb_max: 0.0,
            vds_max: 0.0,
            vdb_max: 0.0,
            vsb_max: 0.0,
            imax: 0.0,
            trj: 0.0,
            frev: 0.0,
            cjorbot: 0.0,
            cjorsti: 0.0,
            cjorgat: 0.0,
            vbirbot: 0.0,
            vbirsti: 0.0,
            vbirgat: 0.0,
            pbot: 0.0,
            psti: 0.0,
            pgat: 0.0,
            phigbot: 0.0,
            phigsti: 0.0,
            phiggat: 0.0,
            idsatrbot: 0.0,
            idsatrsti: 0.0,
            idsatrgat: 0.0,
            csrhbot: 0.0,
            csrhsti: 0.0,
            csrhgat: 0.0,
            xjunsti: 0.0,
            xjungat: 0.0,
            ctatbot: 0.0,
            ctatsti: 0.0,
            ctatgat: 0.0,
            mefftatbot: 0.0,
            mefftatsti: 0.0,
            mefftatgat: 0.0,
            cbbtbot: 0.0,
            cbbtsti: 0.0,
            cbbtgat: 0.0,
            fbbtrbot: 0.0,
            fbbtrsti: 0.0,
            fbbtrgat: 0.0,
            stfbbtbot: 0.0,
            stfbbtsti: 0.0,
            stfbbtgat: 0.0,
            vbrbot: 0.0,
            vbrsti: 0.0,
            vbrgat: 0.0,
            pbrbot: 0.0,
            pbrsti: 0.0,
            pbrgat: 0.0,
            cjorbotd: 0.0,
            cjorstid: 0.0,
            cjorgatd: 0.0,
            vbirbotd: 0.0,
            vbirstid: 0.0,
            vbirgatd: 0.0,
            pbotd: 0.0,
            pstid: 0.0,
            pgatd: 0.0,
            phigbotd: 0.0,
            phigstid: 0.0,
            phiggatd: 0.0,
            idsatrbotd: 0.0,
            idsatrstid: 0.0,
            idsatrgatd: 0.0,
            csrhbotd: 0.0,
            csrhstid: 0.0,
            csrhgatd: 0.0,
            xjunstid: 0.0,
            xjungatd: 0.0,
            ctatbotd: 0.0,
            ctatstid: 0.0,
            ctatgatd: 0.0,
            mefftatbotd: 0.0,
            mefftatstid: 0.0,
            mefftatgatd: 0.0,
            cbbtbotd: 0.0,
            cbbtstid: 0.0,
            cbbtgatd: 0.0,
            fbbtrbotd: 0.0,
            fbbtrstid: 0.0,
            fbbtrgatd: 0.0,
            stfbbtbotd: 0.0,
            stfbbtstid: 0.0,
            stfbbtgatd: 0.0,
            vbrbotd: 0.0,
            vbrstid: 0.0,
            vbrgatd: 0.0,
            pbrbotd: 0.0,
            pbrstid: 0.0,
            pbrgatd: 0.0,
            swjunexp: 0.0,
            vjunref: 0.0,
            fjunq: 0.0,
            vjunrefd: 0.0,
            fjunqd: 0.0,
            dta: 0.0,
        };
        params.level = 103.0;
        validate_parameter_level(params.level).expect("generated Verilog-A parameter default must satisfy declared range");
        params.type_ = 1.0;
        validate_parameter_type_(params.type_).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tr = 21.0;
        validate_parameter_tr(params.tr).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swgeo = 1.0;
        validate_parameter_swgeo(params.swgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swigate = 0.0;
        validate_parameter_swigate(params.swigate).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swimpact = 0.0;
        validate_parameter_swimpact(params.swimpact).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swgidl = 0.0;
        validate_parameter_swgidl(params.swgidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swjuncap = 0.0;
        validate_parameter_swjuncap(params.swjuncap).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swjunasym = 0.0;
        validate_parameter_swjunasym(params.swjunasym).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swnud = 0.0;
        validate_parameter_swnud(params.swnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swedge = 0.0;
        validate_parameter_swedge(params.swedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swdelvtac = 0.0;
        validate_parameter_swdelvtac(params.swdelvtac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swign = 1.0;
        validate_parameter_swign(params.swign).expect("generated Verilog-A parameter default must satisfy declared range");
        params.qmc = 1.0;
        validate_parameter_qmc(params.qmc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.l = 1e-5;
        validate_parameter_l(params.l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.w = 1e-5;
        validate_parameter_w(params.w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sa = 0.0;
        validate_parameter_sa(params.sa).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sb = 0.0;
        validate_parameter_sb(params.sb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sd = 0.0;
        validate_parameter_sd(params.sd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sca = 0.0;
        validate_parameter_sca(params.sca).expect("generated Verilog-A parameter default must satisfy declared range");
        params.scb = 0.0;
        validate_parameter_scb(params.scb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.scc = 0.0;
        validate_parameter_scc(params.scc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sc = 0.0;
        validate_parameter_sc(params.sc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nf = 1.0;
        validate_parameter_nf(params.nf).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ngcon = 1.0;
        validate_parameter_ngcon(params.ngcon).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xgw = 1e-7;
        validate_parameter_xgw(params.xgw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nrs = 0.0;
        validate_parameter_nrs(params.nrs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nrd = 0.0;
        validate_parameter_nrd(params.nrd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.jw = 1e-6;
        validate_parameter_jw(params.jw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.delvto = 0.0;
        validate_parameter_delvto(params.delvto).expect("generated Verilog-A parameter default must satisfy declared range");
        params.factuo = 1.0;
        validate_parameter_factuo(params.factuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.delvtoedge = 0.0;
        validate_parameter_delvtoedge(params.delvtoedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.factuoedge = 1.0;
        validate_parameter_factuoedge(params.factuoedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.absource = 1e-12;
        validate_parameter_absource(params.absource).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lssource = 1e-6;
        validate_parameter_lssource(params.lssource).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lgsource = 1e-6;
        validate_parameter_lgsource(params.lgsource).expect("generated Verilog-A parameter default must satisfy declared range");
        params.abdrain = 1e-12;
        validate_parameter_abdrain(params.abdrain).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lsdrain = 1e-6;
        validate_parameter_lsdrain(params.lsdrain).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lgdrain = 1e-6;
        validate_parameter_lgdrain(params.lgdrain).expect("generated Verilog-A parameter default must satisfy declared range");
        params.as_ = 1e-12;
        validate_parameter_as_(params.as_).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ps = 1e-6;
        validate_parameter_ps(params.ps).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ad = 1e-12;
        validate_parameter_ad(params.ad).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pd = 1e-6;
        validate_parameter_pd(params.pd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mult = 1.0;
        validate_parameter_mult(params.mult).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vfb = -1.0;
        validate_parameter_vfb(params.vfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfb = 0.0005;
        validate_parameter_stvfb(params.stvfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.st2vfb = 0.0;
        validate_parameter_st2vfb(params.st2vfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tox = 2e-9;
        validate_parameter_tox(params.tox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.epsrox = 3.9;
        validate_parameter_epsrox(params.epsrox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.neff = 5e23;
        validate_parameter_neff(params.neff).expect("generated Verilog-A parameter default must satisfy declared range");
        params.facneffac = 1.0;
        validate_parameter_facneffac(params.facneffac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gfacnud = 1.0;
        validate_parameter_gfacnud(params.gfacnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vsbnud = 0.0;
        validate_parameter_vsbnud(params.vsbnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dvsbnud = 1.0;
        validate_parameter_dvsbnud(params.dvsbnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vnsub = 0.0;
        validate_parameter_vnsub(params.vnsub).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nslp = 0.05;
        validate_parameter_nslp(params.nslp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dnsub = 0.0;
        validate_parameter_dnsub(params.dnsub).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphib = 0.0;
        validate_parameter_dphib(params.dphib).expect("generated Verilog-A parameter default must satisfy declared range");
        params.delvtac = 0.0;
        validate_parameter_delvtac(params.delvtac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.np = 1e26;
        validate_parameter_np(params.np).expect("generated Verilog-A parameter default must satisfy declared range");
        params.toxov = 2e-9;
        validate_parameter_toxov(params.toxov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.toxovd = 2e-9;
        validate_parameter_toxovd(params.toxovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nov = 5e25;
        validate_parameter_nov(params.nov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.novd = 5e25;
        validate_parameter_novd(params.novd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ct = 0.0;
        validate_parameter_ct(params.ct).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctg = 0.0;
        validate_parameter_ctg(params.ctg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctb = 0.0;
        validate_parameter_ctb(params.ctb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stct = 1.0;
        validate_parameter_stct(params.stct).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cf = 0.0;
        validate_parameter_cf(params.cf).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfd = 0.0;
        validate_parameter_cfd(params.cfd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfb = 0.0;
        validate_parameter_cfb(params.cfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.psce = 0.0;
        validate_parameter_psce(params.psce).expect("generated Verilog-A parameter default must satisfy declared range");
        params.psceb = 0.0;
        validate_parameter_psceb(params.psceb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.psced = 0.0;
        validate_parameter_psced(params.psced).expect("generated Verilog-A parameter default must satisfy declared range");
        params.betn = 0.07;
        validate_parameter_betn(params.betn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbet = 1.0;
        validate_parameter_stbet(params.stbet).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mue = 0.5;
        validate_parameter_mue(params.mue).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stmue = 0.0;
        validate_parameter_stmue(params.stmue).expect("generated Verilog-A parameter default must satisfy declared range");
        params.themu = 1.5;
        validate_parameter_themu(params.themu).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stthemu = 1.5;
        validate_parameter_stthemu(params.stthemu).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cs = 0.0;
        validate_parameter_cs(params.cs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stcs = 0.0;
        validate_parameter_stcs(params.stcs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thecs = 2.0;
        validate_parameter_thecs(params.thecs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stthecs = 0.0;
        validate_parameter_stthecs(params.stthecs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xcor = 0.0;
        validate_parameter_xcor(params.xcor).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stxcor = 0.0;
        validate_parameter_stxcor(params.stxcor).expect("generated Verilog-A parameter default must satisfy declared range");
        params.feta = 1.0;
        validate_parameter_feta(params.feta).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rs = 30.0;
        validate_parameter_rs(params.rs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.strs = 1.0;
        validate_parameter_strs(params.strs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rsb = 0.0;
        validate_parameter_rsb(params.rsb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rsg = 0.0;
        validate_parameter_rsg(params.rsg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesat = 1.0;
        validate_parameter_thesat(params.thesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stthesat = 1.0;
        validate_parameter_stthesat(params.stthesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesatb = 0.0;
        validate_parameter_thesatb(params.thesatb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesatg = 0.0;
        validate_parameter_thesatg(params.thesatg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ax = 3.0;
        validate_parameter_ax(params.ax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp = 0.01;
        validate_parameter_alp(params.alp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp1 = 0.0;
        validate_parameter_alp1(params.alp1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp2 = 0.0;
        validate_parameter_alp2(params.alp2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vp = 0.05;
        validate_parameter_vp(params.vp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a1 = 1.0;
        validate_parameter_a1(params.a1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a2 = 10.0;
        validate_parameter_a2(params.a2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sta2 = 0.0;
        validate_parameter_sta2(params.sta2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a3 = 1.0;
        validate_parameter_a3(params.a3).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a4 = 0.0;
        validate_parameter_a4(params.a4).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gco = 0.0;
        validate_parameter_gco(params.gco).expect("generated Verilog-A parameter default must satisfy declared range");
        params.iginv = 0.0;
        validate_parameter_iginv(params.iginv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.igov = 0.0;
        validate_parameter_igov(params.igov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.igovd = 0.0;
        validate_parameter_igovd(params.igovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stig = 2.0;
        validate_parameter_stig(params.stig).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gc2 = 0.375;
        validate_parameter_gc2(params.gc2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gc3 = 0.063;
        validate_parameter_gc3(params.gc3).expect("generated Verilog-A parameter default must satisfy declared range");
        params.chib = 3.1;
        validate_parameter_chib(params.chib).expect("generated Verilog-A parameter default must satisfy declared range");
        params.agidl = 0.0;
        validate_parameter_agidl(params.agidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.agidld = 0.0;
        validate_parameter_agidld(params.agidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.bgidl = 41.0;
        validate_parameter_bgidl(params.bgidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.bgidld = 41.0;
        validate_parameter_bgidld(params.bgidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbgidl = 0.0;
        validate_parameter_stbgidl(params.stbgidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbgidld = 0.0;
        validate_parameter_stbgidld(params.stbgidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cgidl = 0.0;
        validate_parameter_cgidl(params.cgidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cgidld = 0.0;
        validate_parameter_cgidld(params.cgidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cox = 1e-14;
        validate_parameter_cox(params.cox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cgov = 1e-15;
        validate_parameter_cgov(params.cgov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cgovd = 1e-15;
        validate_parameter_cgovd(params.cgovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cgbov = 0.0;
        validate_parameter_cgbov(params.cgbov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfr = 0.0;
        validate_parameter_cfr(params.cfr).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfrd = 0.0;
        validate_parameter_cfrd(params.cfrd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fnt = 1.0;
        validate_parameter_fnt(params.fnt).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fntexc = 0.0;
        validate_parameter_fntexc(params.fntexc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfa = 8e22;
        validate_parameter_nfa(params.nfa).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfb = 30000000.0;
        validate_parameter_nfb(params.nfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfc = 0.0;
        validate_parameter_nfc(params.nfc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ef = 1.0;
        validate_parameter_ef(params.ef).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vfbedge = -1.0;
        validate_parameter_vfbedge(params.vfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfbedge = 0.0005;
        validate_parameter_stvfbedge(params.stvfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphibedge = 0.0;
        validate_parameter_dphibedge(params.dphibedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.neffedge = 5e23;
        validate_parameter_neffedge(params.neffedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctedge = 0.0;
        validate_parameter_ctedge(params.ctedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.betnedge = 0.0005;
        validate_parameter_betnedge(params.betnedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbetedge = 1.0;
        validate_parameter_stbetedge(params.stbetedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.psceedge = 0.0;
        validate_parameter_psceedge(params.psceedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pscebedge = 0.0;
        validate_parameter_pscebedge(params.pscebedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pscededge = 0.0;
        validate_parameter_pscededge(params.pscededge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfedge = 0.0;
        validate_parameter_cfedge(params.cfedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfdedge = 0.0;
        validate_parameter_cfdedge(params.cfdedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfbedge = 0.0;
        validate_parameter_cfbedge(params.cfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fntedge = 1.0;
        validate_parameter_fntedge(params.fntedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfaedge = 8e22;
        validate_parameter_nfaedge(params.nfaedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfbedge = 30000000.0;
        validate_parameter_nfbedge(params.nfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfcedge = 0.0;
        validate_parameter_nfcedge(params.nfcedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.efedge = 1.0;
        validate_parameter_efedge(params.efedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rg = 0.0;
        validate_parameter_rg(params.rg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rse = 0.0;
        validate_parameter_rse(params.rse).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rde = 0.0;
        validate_parameter_rde(params.rde).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rbulk = 0.0;
        validate_parameter_rbulk(params.rbulk).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rwell = 0.0;
        validate_parameter_rwell(params.rwell).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rjuns = 0.0;
        validate_parameter_rjuns(params.rjuns).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rjund = 0.0;
        validate_parameter_rjund(params.rjund).expect("generated Verilog-A parameter default must satisfy declared range");
        params.povfb = -1.0;
        validate_parameter_povfb(params.povfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plvfb = 0.0;
        validate_parameter_plvfb(params.plvfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwvfb = 0.0;
        validate_parameter_pwvfb(params.pwvfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwvfb = 0.0;
        validate_parameter_plwvfb(params.plwvfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postvfb = 0.0005;
        validate_parameter_postvfb(params.postvfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plstvfb = 0.0;
        validate_parameter_plstvfb(params.plstvfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwstvfb = 0.0;
        validate_parameter_pwstvfb(params.pwstvfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwstvfb = 0.0;
        validate_parameter_plwstvfb(params.plwstvfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.post2vfb = 0.0;
        validate_parameter_post2vfb(params.post2vfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.potox = 2e-9;
        validate_parameter_potox(params.potox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poepsrox = 3.9;
        validate_parameter_poepsrox(params.poepsrox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poneff = 5e23;
        validate_parameter_poneff(params.poneff).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plneff = 0.0;
        validate_parameter_plneff(params.plneff).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwneff = 0.0;
        validate_parameter_pwneff(params.pwneff).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwneff = 0.0;
        validate_parameter_plwneff(params.plwneff).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pofacneffac = 1.0;
        validate_parameter_pofacneffac(params.pofacneffac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plfacneffac = 0.0;
        validate_parameter_plfacneffac(params.plfacneffac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwfacneffac = 0.0;
        validate_parameter_pwfacneffac(params.pwfacneffac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwfacneffac = 0.0;
        validate_parameter_plwfacneffac(params.plwfacneffac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pogfacnud = 1.0;
        validate_parameter_pogfacnud(params.pogfacnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plgfacnud = 0.0;
        validate_parameter_plgfacnud(params.plgfacnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwgfacnud = 0.0;
        validate_parameter_pwgfacnud(params.pwgfacnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwgfacnud = 0.0;
        validate_parameter_plwgfacnud(params.plwgfacnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.povsbnud = 0.0;
        validate_parameter_povsbnud(params.povsbnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.podvsbnud = 1.0;
        validate_parameter_podvsbnud(params.podvsbnud).expect("generated Verilog-A parameter default must satisfy declared range");
        params.povnsub = 0.0;
        validate_parameter_povnsub(params.povnsub).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponslp = 0.05;
        validate_parameter_ponslp(params.ponslp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.podnsub = 0.0;
        validate_parameter_podnsub(params.podnsub).expect("generated Verilog-A parameter default must satisfy declared range");
        params.podphib = 0.0;
        validate_parameter_podphib(params.podphib).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pldphib = 0.0;
        validate_parameter_pldphib(params.pldphib).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwdphib = 0.0;
        validate_parameter_pwdphib(params.pwdphib).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwdphib = 0.0;
        validate_parameter_plwdphib(params.plwdphib).expect("generated Verilog-A parameter default must satisfy declared range");
        params.podelvtac = 0.0;
        validate_parameter_podelvtac(params.podelvtac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pldelvtac = 0.0;
        validate_parameter_pldelvtac(params.pldelvtac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwdelvtac = 0.0;
        validate_parameter_pwdelvtac(params.pwdelvtac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwdelvtac = 0.0;
        validate_parameter_plwdelvtac(params.plwdelvtac).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponp = 1e26;
        validate_parameter_ponp(params.ponp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plnp = 0.0;
        validate_parameter_plnp(params.plnp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwnp = 0.0;
        validate_parameter_pwnp(params.pwnp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwnp = 0.0;
        validate_parameter_plwnp(params.plwnp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.potoxov = 2e-9;
        validate_parameter_potoxov(params.potoxov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.potoxovd = 2e-9;
        validate_parameter_potoxovd(params.potoxovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponov = 5e25;
        validate_parameter_ponov(params.ponov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plnov = 0.0;
        validate_parameter_plnov(params.plnov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwnov = 0.0;
        validate_parameter_pwnov(params.pwnov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwnov = 0.0;
        validate_parameter_plwnov(params.plwnov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponovd = 5e25;
        validate_parameter_ponovd(params.ponovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plnovd = 0.0;
        validate_parameter_plnovd(params.plnovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwnovd = 0.0;
        validate_parameter_pwnovd(params.pwnovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwnovd = 0.0;
        validate_parameter_plwnovd(params.plwnovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poct = 0.0;
        validate_parameter_poct(params.poct).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plct = 0.0;
        validate_parameter_plct(params.plct).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwct = 0.0;
        validate_parameter_pwct(params.pwct).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwct = 0.0;
        validate_parameter_plwct(params.plwct).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poctg = 0.0;
        validate_parameter_poctg(params.poctg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poctb = 0.0;
        validate_parameter_poctb(params.poctb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postct = 1.0;
        validate_parameter_postct(params.postct).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocf = 0.0;
        validate_parameter_pocf(params.pocf).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plcf = 0.0;
        validate_parameter_plcf(params.plcf).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwcf = 0.0;
        validate_parameter_pwcf(params.pwcf).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwcf = 0.0;
        validate_parameter_plwcf(params.plwcf).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocfd = 0.0;
        validate_parameter_pocfd(params.pocfd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocfb = 0.0;
        validate_parameter_pocfb(params.pocfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.popsce = 0.0;
        validate_parameter_popsce(params.popsce).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plpsce = 0.0;
        validate_parameter_plpsce(params.plpsce).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwpsce = 0.0;
        validate_parameter_pwpsce(params.pwpsce).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwpsce = 0.0;
        validate_parameter_plwpsce(params.plwpsce).expect("generated Verilog-A parameter default must satisfy declared range");
        params.popsceb = 0.0;
        validate_parameter_popsceb(params.popsceb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.popsced = 0.0;
        validate_parameter_popsced(params.popsced).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pobetn = 0.07;
        validate_parameter_pobetn(params.pobetn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plbetn = 0.0;
        validate_parameter_plbetn(params.plbetn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwbetn = 0.0;
        validate_parameter_pwbetn(params.pwbetn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwbetn = 0.0;
        validate_parameter_plwbetn(params.plwbetn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postbet = 1.0;
        validate_parameter_postbet(params.postbet).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plstbet = 0.0;
        validate_parameter_plstbet(params.plstbet).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwstbet = 0.0;
        validate_parameter_pwstbet(params.pwstbet).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwstbet = 0.0;
        validate_parameter_plwstbet(params.plwstbet).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pomue = 0.5;
        validate_parameter_pomue(params.pomue).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plmue = 0.0;
        validate_parameter_plmue(params.plmue).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwmue = 0.0;
        validate_parameter_pwmue(params.pwmue).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwmue = 0.0;
        validate_parameter_plwmue(params.plwmue).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postmue = 0.0;
        validate_parameter_postmue(params.postmue).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pothemu = 1.5;
        validate_parameter_pothemu(params.pothemu).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postthemu = 1.5;
        validate_parameter_postthemu(params.postthemu).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocs = 0.0;
        validate_parameter_pocs(params.pocs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plcs = 0.0;
        validate_parameter_plcs(params.plcs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwcs = 0.0;
        validate_parameter_pwcs(params.pwcs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwcs = 0.0;
        validate_parameter_plwcs(params.plwcs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postcs = 0.0;
        validate_parameter_postcs(params.postcs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pothecs = 2.0;
        validate_parameter_pothecs(params.pothecs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postthecs = 0.0;
        validate_parameter_postthecs(params.postthecs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poxcor = 0.0;
        validate_parameter_poxcor(params.poxcor).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plxcor = 0.0;
        validate_parameter_plxcor(params.plxcor).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwxcor = 0.0;
        validate_parameter_pwxcor(params.pwxcor).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwxcor = 0.0;
        validate_parameter_plwxcor(params.plwxcor).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postxcor = 0.0;
        validate_parameter_postxcor(params.postxcor).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pofeta = 1.0;
        validate_parameter_pofeta(params.pofeta).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pors = 30.0;
        validate_parameter_pors(params.pors).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plrs = 0.0;
        validate_parameter_plrs(params.plrs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwrs = 0.0;
        validate_parameter_pwrs(params.pwrs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwrs = 0.0;
        validate_parameter_plwrs(params.plwrs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postrs = 1.0;
        validate_parameter_postrs(params.postrs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.porsb = 0.0;
        validate_parameter_porsb(params.porsb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.porsg = 0.0;
        validate_parameter_porsg(params.porsg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pothesat = 1.0;
        validate_parameter_pothesat(params.pothesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plthesat = 0.0;
        validate_parameter_plthesat(params.plthesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwthesat = 0.0;
        validate_parameter_pwthesat(params.pwthesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwthesat = 0.0;
        validate_parameter_plwthesat(params.plwthesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postthesat = 1.0;
        validate_parameter_postthesat(params.postthesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plstthesat = 0.0;
        validate_parameter_plstthesat(params.plstthesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwstthesat = 0.0;
        validate_parameter_pwstthesat(params.pwstthesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwstthesat = 0.0;
        validate_parameter_plwstthesat(params.plwstthesat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pothesatb = 0.0;
        validate_parameter_pothesatb(params.pothesatb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plthesatb = 0.0;
        validate_parameter_plthesatb(params.plthesatb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwthesatb = 0.0;
        validate_parameter_pwthesatb(params.pwthesatb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwthesatb = 0.0;
        validate_parameter_plwthesatb(params.plwthesatb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pothesatg = 0.0;
        validate_parameter_pothesatg(params.pothesatg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plthesatg = 0.0;
        validate_parameter_plthesatg(params.plthesatg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwthesatg = 0.0;
        validate_parameter_pwthesatg(params.pwthesatg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwthesatg = 0.0;
        validate_parameter_plwthesatg(params.plwthesatg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poax = 3.0;
        validate_parameter_poax(params.poax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plax = 0.0;
        validate_parameter_plax(params.plax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwax = 0.0;
        validate_parameter_pwax(params.pwax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwax = 0.0;
        validate_parameter_plwax(params.plwax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poalp = 0.01;
        validate_parameter_poalp(params.poalp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plalp = 0.0;
        validate_parameter_plalp(params.plalp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwalp = 0.0;
        validate_parameter_pwalp(params.pwalp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwalp = 0.0;
        validate_parameter_plwalp(params.plwalp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poalp1 = 0.0;
        validate_parameter_poalp1(params.poalp1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plalp1 = 0.0;
        validate_parameter_plalp1(params.plalp1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwalp1 = 0.0;
        validate_parameter_pwalp1(params.pwalp1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwalp1 = 0.0;
        validate_parameter_plwalp1(params.plwalp1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poalp2 = 0.0;
        validate_parameter_poalp2(params.poalp2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plalp2 = 0.0;
        validate_parameter_plalp2(params.plalp2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwalp2 = 0.0;
        validate_parameter_pwalp2(params.pwalp2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwalp2 = 0.0;
        validate_parameter_plwalp2(params.plwalp2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.povp = 0.05;
        validate_parameter_povp(params.povp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poa1 = 1.0;
        validate_parameter_poa1(params.poa1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pla1 = 0.0;
        validate_parameter_pla1(params.pla1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwa1 = 0.0;
        validate_parameter_pwa1(params.pwa1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwa1 = 0.0;
        validate_parameter_plwa1(params.plwa1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poa2 = 10.0;
        validate_parameter_poa2(params.poa2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.posta2 = 0.0;
        validate_parameter_posta2(params.posta2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poa3 = 1.0;
        validate_parameter_poa3(params.poa3).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pla3 = 0.0;
        validate_parameter_pla3(params.pla3).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwa3 = 0.0;
        validate_parameter_pwa3(params.pwa3).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwa3 = 0.0;
        validate_parameter_plwa3(params.plwa3).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poa4 = 0.0;
        validate_parameter_poa4(params.poa4).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pla4 = 0.0;
        validate_parameter_pla4(params.pla4).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwa4 = 0.0;
        validate_parameter_pwa4(params.pwa4).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwa4 = 0.0;
        validate_parameter_plwa4(params.plwa4).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pogco = 0.0;
        validate_parameter_pogco(params.pogco).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poiginv = 0.0;
        validate_parameter_poiginv(params.poiginv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pliginv = 0.0;
        validate_parameter_pliginv(params.pliginv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwiginv = 0.0;
        validate_parameter_pwiginv(params.pwiginv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwiginv = 0.0;
        validate_parameter_plwiginv(params.plwiginv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poigov = 0.0;
        validate_parameter_poigov(params.poigov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pligov = 0.0;
        validate_parameter_pligov(params.pligov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwigov = 0.0;
        validate_parameter_pwigov(params.pwigov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwigov = 0.0;
        validate_parameter_plwigov(params.plwigov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poigovd = 0.0;
        validate_parameter_poigovd(params.poigovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pligovd = 0.0;
        validate_parameter_pligovd(params.pligovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwigovd = 0.0;
        validate_parameter_pwigovd(params.pwigovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwigovd = 0.0;
        validate_parameter_plwigovd(params.plwigovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postig = 2.0;
        validate_parameter_postig(params.postig).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pogc2 = 0.375;
        validate_parameter_pogc2(params.pogc2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pogc3 = 0.063;
        validate_parameter_pogc3(params.pogc3).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pochib = 3.1;
        validate_parameter_pochib(params.pochib).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poagidl = 0.0;
        validate_parameter_poagidl(params.poagidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plagidl = 0.0;
        validate_parameter_plagidl(params.plagidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwagidl = 0.0;
        validate_parameter_pwagidl(params.pwagidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwagidl = 0.0;
        validate_parameter_plwagidl(params.plwagidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poagidld = 0.0;
        validate_parameter_poagidld(params.poagidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plagidld = 0.0;
        validate_parameter_plagidld(params.plagidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwagidld = 0.0;
        validate_parameter_pwagidld(params.pwagidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwagidld = 0.0;
        validate_parameter_plwagidld(params.plwagidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pobgidl = 41.0;
        validate_parameter_pobgidl(params.pobgidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pobgidld = 41.0;
        validate_parameter_pobgidld(params.pobgidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postbgidl = 0.0;
        validate_parameter_postbgidl(params.postbgidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postbgidld = 0.0;
        validate_parameter_postbgidld(params.postbgidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocgidl = 0.0;
        validate_parameter_pocgidl(params.pocgidl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocgidld = 0.0;
        validate_parameter_pocgidld(params.pocgidld).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocox = 1e-14;
        validate_parameter_pocox(params.pocox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plcox = 0.0;
        validate_parameter_plcox(params.plcox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwcox = 0.0;
        validate_parameter_pwcox(params.pwcox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwcox = 0.0;
        validate_parameter_plwcox(params.plwcox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocgov = 1e-15;
        validate_parameter_pocgov(params.pocgov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plcgov = 0.0;
        validate_parameter_plcgov(params.plcgov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwcgov = 0.0;
        validate_parameter_pwcgov(params.pwcgov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwcgov = 0.0;
        validate_parameter_plwcgov(params.plwcgov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocgovd = 1e-15;
        validate_parameter_pocgovd(params.pocgovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plcgovd = 0.0;
        validate_parameter_plcgovd(params.plcgovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwcgovd = 0.0;
        validate_parameter_pwcgovd(params.pwcgovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwcgovd = 0.0;
        validate_parameter_plwcgovd(params.plwcgovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocgbov = 0.0;
        validate_parameter_pocgbov(params.pocgbov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plcgbov = 0.0;
        validate_parameter_plcgbov(params.plcgbov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwcgbov = 0.0;
        validate_parameter_pwcgbov(params.pwcgbov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwcgbov = 0.0;
        validate_parameter_plwcgbov(params.plwcgbov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocfr = 0.0;
        validate_parameter_pocfr(params.pocfr).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plcfr = 0.0;
        validate_parameter_plcfr(params.plcfr).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwcfr = 0.0;
        validate_parameter_pwcfr(params.pwcfr).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwcfr = 0.0;
        validate_parameter_plwcfr(params.plwcfr).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocfrd = 0.0;
        validate_parameter_pocfrd(params.pocfrd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plcfrd = 0.0;
        validate_parameter_plcfrd(params.plcfrd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwcfrd = 0.0;
        validate_parameter_pwcfrd(params.pwcfrd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwcfrd = 0.0;
        validate_parameter_plwcfrd(params.plwcfrd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pofnt = 1.0;
        validate_parameter_pofnt(params.pofnt).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pofntexc = 0.0;
        validate_parameter_pofntexc(params.pofntexc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plfntexc = 0.0;
        validate_parameter_plfntexc(params.plfntexc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwfntexc = 0.0;
        validate_parameter_pwfntexc(params.pwfntexc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwfntexc = 0.0;
        validate_parameter_plwfntexc(params.plwfntexc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponfa = 8e22;
        validate_parameter_ponfa(params.ponfa).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plnfa = 0.0;
        validate_parameter_plnfa(params.plnfa).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwnfa = 0.0;
        validate_parameter_pwnfa(params.pwnfa).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwnfa = 0.0;
        validate_parameter_plwnfa(params.plwnfa).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponfb = 30000000.0;
        validate_parameter_ponfb(params.ponfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plnfb = 0.0;
        validate_parameter_plnfb(params.plnfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwnfb = 0.0;
        validate_parameter_pwnfb(params.pwnfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwnfb = 0.0;
        validate_parameter_plwnfb(params.plwnfb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponfc = 0.0;
        validate_parameter_ponfc(params.ponfc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plnfc = 0.0;
        validate_parameter_plnfc(params.plnfc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwnfc = 0.0;
        validate_parameter_pwnfc(params.pwnfc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwnfc = 0.0;
        validate_parameter_plwnfc(params.plwnfc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poef = 1.0;
        validate_parameter_poef(params.poef).expect("generated Verilog-A parameter default must satisfy declared range");
        params.povfbedge = -1.0;
        validate_parameter_povfbedge(params.povfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postvfbedge = 0.0;
        validate_parameter_postvfbedge(params.postvfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plstvfbedge = 0.0;
        validate_parameter_plstvfbedge(params.plstvfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwstvfbedge = 0.0;
        validate_parameter_pwstvfbedge(params.pwstvfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwstvfbedge = 0.0;
        validate_parameter_plwstvfbedge(params.plwstvfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.podphibedge = 0.0;
        validate_parameter_podphibedge(params.podphibedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pldphibedge = 0.0;
        validate_parameter_pldphibedge(params.pldphibedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwdphibedge = 0.0;
        validate_parameter_pwdphibedge(params.pwdphibedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwdphibedge = 0.0;
        validate_parameter_plwdphibedge(params.plwdphibedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poneffedge = 5e23;
        validate_parameter_poneffedge(params.poneffedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plneffedge = 0.0;
        validate_parameter_plneffedge(params.plneffedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwneffedge = 0.0;
        validate_parameter_pwneffedge(params.pwneffedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwneffedge = 0.0;
        validate_parameter_plwneffedge(params.plwneffedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poctedge = 0.0;
        validate_parameter_poctedge(params.poctedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plctedge = 0.0;
        validate_parameter_plctedge(params.plctedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwctedge = 0.0;
        validate_parameter_pwctedge(params.pwctedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwctedge = 0.0;
        validate_parameter_plwctedge(params.plwctedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pobetnedge = 0.0005;
        validate_parameter_pobetnedge(params.pobetnedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plbetnedge = 0.0;
        validate_parameter_plbetnedge(params.plbetnedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwbetnedge = 0.0;
        validate_parameter_pwbetnedge(params.pwbetnedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwbetnedge = 0.0;
        validate_parameter_plwbetnedge(params.plwbetnedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.postbetedge = 1.0;
        validate_parameter_postbetedge(params.postbetedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plstbetedge = 0.0;
        validate_parameter_plstbetedge(params.plstbetedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwstbetedge = 0.0;
        validate_parameter_pwstbetedge(params.pwstbetedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwstbetedge = 0.0;
        validate_parameter_plwstbetedge(params.plwstbetedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.popsceedge = 0.0;
        validate_parameter_popsceedge(params.popsceedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plpsceedge = 0.0;
        validate_parameter_plpsceedge(params.plpsceedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwpsceedge = 0.0;
        validate_parameter_pwpsceedge(params.pwpsceedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwpsceedge = 0.0;
        validate_parameter_plwpsceedge(params.plwpsceedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.popscebedge = 0.0;
        validate_parameter_popscebedge(params.popscebedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.popscededge = 0.0;
        validate_parameter_popscededge(params.popscededge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocfedge = 0.0;
        validate_parameter_pocfedge(params.pocfedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plcfedge = 0.0;
        validate_parameter_plcfedge(params.plcfedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwcfedge = 0.0;
        validate_parameter_pwcfedge(params.pwcfedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwcfedge = 0.0;
        validate_parameter_plwcfedge(params.plwcfedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocfdedge = 0.0;
        validate_parameter_pocfdedge(params.pocfdedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pocfbedge = 0.0;
        validate_parameter_pocfbedge(params.pocfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pofntedge = 1.0;
        validate_parameter_pofntedge(params.pofntedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponfaedge = 8e22;
        validate_parameter_ponfaedge(params.ponfaedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plnfaedge = 0.0;
        validate_parameter_plnfaedge(params.plnfaedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwnfaedge = 0.0;
        validate_parameter_pwnfaedge(params.pwnfaedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwnfaedge = 0.0;
        validate_parameter_plwnfaedge(params.plwnfaedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponfbedge = 30000000.0;
        validate_parameter_ponfbedge(params.ponfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plnfbedge = 0.0;
        validate_parameter_plnfbedge(params.plnfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwnfbedge = 0.0;
        validate_parameter_pwnfbedge(params.pwnfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwnfbedge = 0.0;
        validate_parameter_plwnfbedge(params.plwnfbedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ponfcedge = 0.0;
        validate_parameter_ponfcedge(params.ponfcedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plnfcedge = 0.0;
        validate_parameter_plnfcedge(params.plnfcedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwnfcedge = 0.0;
        validate_parameter_pwnfcedge(params.pwnfcedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwnfcedge = 0.0;
        validate_parameter_plwnfcedge(params.plwnfcedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.poefedge = 1.0;
        validate_parameter_poefedge(params.poefedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pokvthowe = 0.0;
        validate_parameter_pokvthowe(params.pokvthowe).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plkvthowe = 0.0;
        validate_parameter_plkvthowe(params.plkvthowe).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwkvthowe = 0.0;
        validate_parameter_pwkvthowe(params.pwkvthowe).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwkvthowe = 0.0;
        validate_parameter_plwkvthowe(params.plwkvthowe).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pokuowe = 0.0;
        validate_parameter_pokuowe(params.pokuowe).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plkuowe = 0.0;
        validate_parameter_plkuowe(params.plkuowe).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pwkuowe = 0.0;
        validate_parameter_pwkuowe(params.pwkuowe).expect("generated Verilog-A parameter default must satisfy declared range");
        params.plwkuowe = 0.0;
        validate_parameter_plwkuowe(params.plwkuowe).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lmin = 0.0;
        validate_parameter_lmin(params.lmin).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lmax = 1.0;
        validate_parameter_lmax(params.lmax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wmin = 0.0;
        validate_parameter_wmin(params.wmin).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wmax = 1.0;
        validate_parameter_wmax(params.wmax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lvaro = 0.0;
        validate_parameter_lvaro(params.lvaro).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lvarl = 0.0;
        validate_parameter_lvarl(params.lvarl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lvarw = 0.0;
        validate_parameter_lvarw(params.lvarw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lap = 0.0;
        validate_parameter_lap(params.lap).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wvaro = 0.0;
        validate_parameter_wvaro(params.wvaro).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wvarl = 0.0;
        validate_parameter_wvarl(params.wvarl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wvarw = 0.0;
        validate_parameter_wvarw(params.wvarw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wot = 0.0;
        validate_parameter_wot(params.wot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dlq = 0.0;
        validate_parameter_dlq(params.dlq).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dwq = 0.0;
        validate_parameter_dwq(params.dwq).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vfbo = -1.0;
        validate_parameter_vfbo(params.vfbo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vfbl = 0.0;
        validate_parameter_vfbl(params.vfbl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vfbw = 0.0;
        validate_parameter_vfbw(params.vfbw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vfblw = 0.0;
        validate_parameter_vfblw(params.vfblw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfbo = 0.0005;
        validate_parameter_stvfbo(params.stvfbo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfbl = 0.0;
        validate_parameter_stvfbl(params.stvfbl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfbw = 0.0;
        validate_parameter_stvfbw(params.stvfbw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfblw = 0.0;
        validate_parameter_stvfblw(params.stvfblw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.st2vfbo = 0.0;
        validate_parameter_st2vfbo(params.st2vfbo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.toxo = 2e-9;
        validate_parameter_toxo(params.toxo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.epsroxo = 3.9;
        validate_parameter_epsroxo(params.epsroxo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsubo = 3e23;
        validate_parameter_nsubo(params.nsubo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsubw = 0.0;
        validate_parameter_nsubw(params.nsubw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wseg = 1e-8;
        validate_parameter_wseg(params.wseg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.npck = 1e24;
        validate_parameter_npck(params.npck).expect("generated Verilog-A parameter default must satisfy declared range");
        params.npckw = 0.0;
        validate_parameter_npckw(params.npckw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wsegp = 1e-8;
        validate_parameter_wsegp(params.wsegp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lpck = 1e-8;
        validate_parameter_lpck(params.lpck).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lpckw = 0.0;
        validate_parameter_lpckw(params.lpckw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fol1 = 0.0;
        validate_parameter_fol1(params.fol1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fol2 = 0.0;
        validate_parameter_fol2(params.fol2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.facneffaco = 1.0;
        validate_parameter_facneffaco(params.facneffaco).expect("generated Verilog-A parameter default must satisfy declared range");
        params.facneffacl = 0.0;
        validate_parameter_facneffacl(params.facneffacl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.facneffacw = 0.0;
        validate_parameter_facneffacw(params.facneffacw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.facneffaclw = 0.0;
        validate_parameter_facneffaclw(params.facneffaclw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gfacnudo = 1.0;
        validate_parameter_gfacnudo(params.gfacnudo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gfacnudl = 0.0;
        validate_parameter_gfacnudl(params.gfacnudl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gfacnudlexp = 1.0;
        validate_parameter_gfacnudlexp(params.gfacnudlexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gfacnudw = 0.0;
        validate_parameter_gfacnudw(params.gfacnudw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gfacnudlw = 0.0;
        validate_parameter_gfacnudlw(params.gfacnudlw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vsbnudo = 0.0;
        validate_parameter_vsbnudo(params.vsbnudo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dvsbnudo = 1.0;
        validate_parameter_dvsbnudo(params.dvsbnudo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vnsubo = 0.0;
        validate_parameter_vnsubo(params.vnsubo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nslpo = 0.05;
        validate_parameter_nslpo(params.nslpo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dnsubo = 0.0;
        validate_parameter_dnsubo(params.dnsubo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphibo = 0.0;
        validate_parameter_dphibo(params.dphibo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphibl = 0.0;
        validate_parameter_dphibl(params.dphibl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphiblexp = 1.0;
        validate_parameter_dphiblexp(params.dphiblexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphibw = 0.0;
        validate_parameter_dphibw(params.dphibw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphiblw = 0.0;
        validate_parameter_dphiblw(params.dphiblw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.delvtaco = 0.0;
        validate_parameter_delvtaco(params.delvtaco).expect("generated Verilog-A parameter default must satisfy declared range");
        params.delvtacl = 0.0;
        validate_parameter_delvtacl(params.delvtacl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.delvtaclexp = 1.0;
        validate_parameter_delvtaclexp(params.delvtaclexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.delvtacw = 0.0;
        validate_parameter_delvtacw(params.delvtacw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.delvtaclw = 0.0;
        validate_parameter_delvtaclw(params.delvtaclw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.npo = 1e26;
        validate_parameter_npo(params.npo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.npl = 0.0;
        validate_parameter_npl(params.npl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.toxovo = 2e-9;
        validate_parameter_toxovo(params.toxovo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.toxovdo = 2e-9;
        validate_parameter_toxovdo(params.toxovdo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lov = 0.0;
        validate_parameter_lov(params.lov).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lovd = 0.0;
        validate_parameter_lovd(params.lovd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.novo = 5e25;
        validate_parameter_novo(params.novo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.novdo = 5e25;
        validate_parameter_novdo(params.novdo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cto = 0.0;
        validate_parameter_cto(params.cto).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctl = 0.0;
        validate_parameter_ctl(params.ctl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctlexp = 1.0;
        validate_parameter_ctlexp(params.ctlexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctw = 0.0;
        validate_parameter_ctw(params.ctw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctlw = 0.0;
        validate_parameter_ctlw(params.ctlw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctgo = 0.0;
        validate_parameter_ctgo(params.ctgo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctbo = 0.0;
        validate_parameter_ctbo(params.ctbo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stcto = 1.0;
        validate_parameter_stcto(params.stcto).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfl = 0.0;
        validate_parameter_cfl(params.cfl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cflexp = 2.0;
        validate_parameter_cflexp(params.cflexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfw = 0.0;
        validate_parameter_cfw(params.cfw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfdo = 0.0;
        validate_parameter_cfdo(params.cfdo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfbo = 0.0;
        validate_parameter_cfbo(params.cfbo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pscel = 0.0;
        validate_parameter_pscel(params.pscel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pscelexp = 2.0;
        validate_parameter_pscelexp(params.pscelexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pscew = 0.0;
        validate_parameter_pscew(params.pscew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pscebo = 0.0;
        validate_parameter_pscebo(params.pscebo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pscedo = 0.0;
        validate_parameter_pscedo(params.pscedo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.uo = 0.05;
        validate_parameter_uo(params.uo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbet1 = 0.0;
        validate_parameter_fbet1(params.fbet1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbet1w = 0.0;
        validate_parameter_fbet1w(params.fbet1w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lp1 = 1e-8;
        validate_parameter_lp1(params.lp1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lp1w = 0.0;
        validate_parameter_lp1w(params.lp1w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbet2 = 0.0;
        validate_parameter_fbet2(params.fbet2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lp2 = 1e-8;
        validate_parameter_lp2(params.lp2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.betw1 = 0.0;
        validate_parameter_betw1(params.betw1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.betw2 = 0.0;
        validate_parameter_betw2(params.betw2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wbet = 1e-9;
        validate_parameter_wbet(params.wbet).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbeto = 1.0;
        validate_parameter_stbeto(params.stbeto).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbetl = 0.0;
        validate_parameter_stbetl(params.stbetl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbetw = 0.0;
        validate_parameter_stbetw(params.stbetw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbetlw = 0.0;
        validate_parameter_stbetlw(params.stbetlw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mueo = 0.5;
        validate_parameter_mueo(params.mueo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.muew = 0.0;
        validate_parameter_muew(params.muew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stmueo = 0.0;
        validate_parameter_stmueo(params.stmueo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.themuo = 1.5;
        validate_parameter_themuo(params.themuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stthemuo = 1.5;
        validate_parameter_stthemuo(params.stthemuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cso = 0.0;
        validate_parameter_cso(params.cso).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csl = 0.0;
        validate_parameter_csl(params.csl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cslexp = 1.0;
        validate_parameter_cslexp(params.cslexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csw = 0.0;
        validate_parameter_csw(params.csw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cslw = 0.0;
        validate_parameter_cslw(params.cslw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stcso = 0.0;
        validate_parameter_stcso(params.stcso).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thecso = 2.0;
        validate_parameter_thecso(params.thecso).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stthecso = 0.0;
        validate_parameter_stthecso(params.stthecso).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xcoro = 0.0;
        validate_parameter_xcoro(params.xcoro).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xcorl = 0.0;
        validate_parameter_xcorl(params.xcorl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xcorw = 0.0;
        validate_parameter_xcorw(params.xcorw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xcorlw = 0.0;
        validate_parameter_xcorlw(params.xcorlw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stxcoro = 0.0;
        validate_parameter_stxcoro(params.stxcoro).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fetao = 1.0;
        validate_parameter_fetao(params.fetao).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rsw1 = 50.0;
        validate_parameter_rsw1(params.rsw1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rsw2 = 0.0;
        validate_parameter_rsw2(params.rsw2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.strso = 1.0;
        validate_parameter_strso(params.strso).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rsbo = 0.0;
        validate_parameter_rsbo(params.rsbo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rsgo = 0.0;
        validate_parameter_rsgo(params.rsgo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesato = 0.0;
        validate_parameter_thesato(params.thesato).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesatl = 0.05;
        validate_parameter_thesatl(params.thesatl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesatlexp = 1.0;
        validate_parameter_thesatlexp(params.thesatlexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesatw = 0.0;
        validate_parameter_thesatw(params.thesatw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesatlw = 0.0;
        validate_parameter_thesatlw(params.thesatlw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stthesato = 1.0;
        validate_parameter_stthesato(params.stthesato).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stthesatl = 0.0;
        validate_parameter_stthesatl(params.stthesatl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stthesatw = 0.0;
        validate_parameter_stthesatw(params.stthesatw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stthesatlw = 0.0;
        validate_parameter_stthesatlw(params.stthesatlw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesatbo = 0.0;
        validate_parameter_thesatbo(params.thesatbo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.thesatgo = 0.0;
        validate_parameter_thesatgo(params.thesatgo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.axo = 18.0;
        validate_parameter_axo(params.axo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.axl = 0.4;
        validate_parameter_axl(params.axl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alpl = 0.0005;
        validate_parameter_alpl(params.alpl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alplexp = 1.0;
        validate_parameter_alplexp(params.alplexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alpw = 0.0;
        validate_parameter_alpw(params.alpw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp1l1 = 0.0;
        validate_parameter_alp1l1(params.alp1l1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp1lexp = 0.5;
        validate_parameter_alp1lexp(params.alp1lexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp1l2 = 0.0;
        validate_parameter_alp1l2(params.alp1l2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp1w = 0.0;
        validate_parameter_alp1w(params.alp1w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp2l1 = 0.0;
        validate_parameter_alp2l1(params.alp2l1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp2lexp = 0.5;
        validate_parameter_alp2lexp(params.alp2lexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp2l2 = 0.0;
        validate_parameter_alp2l2(params.alp2l2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alp2w = 0.0;
        validate_parameter_alp2w(params.alp2w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vpo = 0.05;
        validate_parameter_vpo(params.vpo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a1o = 1.0;
        validate_parameter_a1o(params.a1o).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a1l = 0.0;
        validate_parameter_a1l(params.a1l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a1w = 0.0;
        validate_parameter_a1w(params.a1w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a2o = 10.0;
        validate_parameter_a2o(params.a2o).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sta2o = 0.0;
        validate_parameter_sta2o(params.sta2o).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a3o = 1.0;
        validate_parameter_a3o(params.a3o).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a3l = 0.0;
        validate_parameter_a3l(params.a3l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a3w = 0.0;
        validate_parameter_a3w(params.a3w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a4o = 0.0;
        validate_parameter_a4o(params.a4o).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a4l = 0.0;
        validate_parameter_a4l(params.a4l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a4w = 0.0;
        validate_parameter_a4w(params.a4w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gcoo = 0.0;
        validate_parameter_gcoo(params.gcoo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.iginvlw = 0.0;
        validate_parameter_iginvlw(params.iginvlw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.igovw = 0.0;
        validate_parameter_igovw(params.igovw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.igovdw = 0.0;
        validate_parameter_igovdw(params.igovdw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stigo = 2.0;
        validate_parameter_stigo(params.stigo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gc2o = 0.375;
        validate_parameter_gc2o(params.gc2o).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gc3o = 0.063;
        validate_parameter_gc3o(params.gc3o).expect("generated Verilog-A parameter default must satisfy declared range");
        params.chibo = 3.1;
        validate_parameter_chibo(params.chibo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.agidlw = 0.0;
        validate_parameter_agidlw(params.agidlw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.agidldw = 0.0;
        validate_parameter_agidldw(params.agidldw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.bgidlo = 41.0;
        validate_parameter_bgidlo(params.bgidlo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.bgidldo = 41.0;
        validate_parameter_bgidldo(params.bgidldo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbgidlo = 0.0;
        validate_parameter_stbgidlo(params.stbgidlo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbgidldo = 0.0;
        validate_parameter_stbgidldo(params.stbgidldo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cgidlo = 0.0;
        validate_parameter_cgidlo(params.cgidlo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cgidldo = 0.0;
        validate_parameter_cgidldo(params.cgidldo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cgbovl = 0.0;
        validate_parameter_cgbovl(params.cgbovl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfrw = 0.0;
        validate_parameter_cfrw(params.cfrw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfrdw = 0.0;
        validate_parameter_cfrdw(params.cfrdw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fnto = 1.0;
        validate_parameter_fnto(params.fnto).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fntexcl = 0.0;
        validate_parameter_fntexcl(params.fntexcl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfalw = 8e22;
        validate_parameter_nfalw(params.nfalw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfblw = 30000000.0;
        validate_parameter_nfblw(params.nfblw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfclw = 0.0;
        validate_parameter_nfclw(params.nfclw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.efo = 1.0;
        validate_parameter_efo(params.efo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lintnoi = 0.0;
        validate_parameter_lintnoi(params.lintnoi).expect("generated Verilog-A parameter default must satisfy declared range");
        params.alpnoi = 2.0;
        validate_parameter_alpnoi(params.alpnoi).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wedge = 1e-8;
        validate_parameter_wedge(params.wedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wedgew = 0.0;
        validate_parameter_wedgew(params.wedgew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vfbedgeo = -1.0;
        validate_parameter_vfbedgeo(params.vfbedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfbedgeo = 0.0005;
        validate_parameter_stvfbedgeo(params.stvfbedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfbedgel = 0.0;
        validate_parameter_stvfbedgel(params.stvfbedgel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfbedgew = 0.0;
        validate_parameter_stvfbedgew(params.stvfbedgew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stvfbedgelw = 0.0;
        validate_parameter_stvfbedgelw(params.stvfbedgelw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphibedgeo = 0.0;
        validate_parameter_dphibedgeo(params.dphibedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphibedgel = 0.0;
        validate_parameter_dphibedgel(params.dphibedgel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphibedgelexp = 1.0;
        validate_parameter_dphibedgelexp(params.dphibedgelexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphibedgew = 0.0;
        validate_parameter_dphibedgew(params.dphibedgew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dphibedgelw = 0.0;
        validate_parameter_dphibedgelw(params.dphibedgelw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsubedgeo = 5e23;
        validate_parameter_nsubedgeo(params.nsubedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsubedgel = 0.0;
        validate_parameter_nsubedgel(params.nsubedgel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsubedgelexp = 1.0;
        validate_parameter_nsubedgelexp(params.nsubedgelexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsubedgew = 0.0;
        validate_parameter_nsubedgew(params.nsubedgew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsubedgelw = 0.0;
        validate_parameter_nsubedgelw(params.nsubedgelw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctedgeo = 0.0;
        validate_parameter_ctedgeo(params.ctedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctedgel = 0.0;
        validate_parameter_ctedgel(params.ctedgel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctedgelexp = 1.0;
        validate_parameter_ctedgelexp(params.ctedgelexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbetedge = 0.0;
        validate_parameter_fbetedge(params.fbetedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lpedge = 1e-8;
        validate_parameter_lpedge(params.lpedge).expect("generated Verilog-A parameter default must satisfy declared range");
        params.betedgew = 0.0;
        validate_parameter_betedgew(params.betedgew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbetedgeo = 1.0;
        validate_parameter_stbetedgeo(params.stbetedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbetedgel = 0.0;
        validate_parameter_stbetedgel(params.stbetedgel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbetedgew = 0.0;
        validate_parameter_stbetedgew(params.stbetedgew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stbetedgelw = 0.0;
        validate_parameter_stbetedgelw(params.stbetedgelw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.psceedgel = 0.0;
        validate_parameter_psceedgel(params.psceedgel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.psceedgelexp = 2.0;
        validate_parameter_psceedgelexp(params.psceedgelexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.psceedgew = 0.0;
        validate_parameter_psceedgew(params.psceedgew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pscebedgeo = 0.0;
        validate_parameter_pscebedgeo(params.pscebedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pscededgeo = 0.0;
        validate_parameter_pscededgeo(params.pscededgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfedgel = 0.0;
        validate_parameter_cfedgel(params.cfedgel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfedgelexp = 2.0;
        validate_parameter_cfedgelexp(params.cfedgelexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfedgew = 0.0;
        validate_parameter_cfedgew(params.cfedgew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfdedgeo = 0.0;
        validate_parameter_cfdedgeo(params.cfdedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cfbedgeo = 0.0;
        validate_parameter_cfbedgeo(params.cfbedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fntedgeo = 1.0;
        validate_parameter_fntedgeo(params.fntedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfaedgelw = 8e22;
        validate_parameter_nfaedgelw(params.nfaedgelw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfbedgelw = 30000000.0;
        validate_parameter_nfbedgelw(params.nfbedgelw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nfcedgelw = 0.0;
        validate_parameter_nfcedgelw(params.nfcedgelw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.efedgeo = 1.0;
        validate_parameter_efedgeo(params.efedgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kvthoweo = 0.0;
        validate_parameter_kvthoweo(params.kvthoweo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kvthowel = 0.0;
        validate_parameter_kvthowel(params.kvthowel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kvthowew = 0.0;
        validate_parameter_kvthowew(params.kvthowew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kvthowelw = 0.0;
        validate_parameter_kvthowelw(params.kvthowelw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kuoweo = 0.0;
        validate_parameter_kuoweo(params.kuoweo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kuowel = 0.0;
        validate_parameter_kuowel(params.kuowel).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kuowew = 0.0;
        validate_parameter_kuowew(params.kuowew).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kuowelw = 0.0;
        validate_parameter_kuowelw(params.kuowelw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rgo = 0.0;
        validate_parameter_rgo(params.rgo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rint = 0.0;
        validate_parameter_rint(params.rint).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rvpoly = 0.0;
        validate_parameter_rvpoly(params.rvpoly).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rshg = 0.0;
        validate_parameter_rshg(params.rshg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dlsil = 0.0;
        validate_parameter_dlsil(params.dlsil).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rsh = 0.0;
        validate_parameter_rsh(params.rsh).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rshd = 0.0;
        validate_parameter_rshd(params.rshd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rbulko = 0.0;
        validate_parameter_rbulko(params.rbulko).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rwello = 0.0;
        validate_parameter_rwello(params.rwello).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rjunso = 0.0;
        validate_parameter_rjunso(params.rjunso).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rjundo = 0.0;
        validate_parameter_rjundo(params.rjundo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.saref = 1e-6;
        validate_parameter_saref(params.saref).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sbref = 1e-6;
        validate_parameter_sbref(params.sbref).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wlod = 0.0;
        validate_parameter_wlod(params.wlod).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kuo = 0.0;
        validate_parameter_kuo(params.kuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kvsat = 0.0;
        validate_parameter_kvsat(params.kvsat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tkuo = 0.0;
        validate_parameter_tkuo(params.tkuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lkuo = 0.0;
        validate_parameter_lkuo(params.lkuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wkuo = 0.0;
        validate_parameter_wkuo(params.wkuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pkuo = 0.0;
        validate_parameter_pkuo(params.pkuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.llodkuo = 0.0;
        validate_parameter_llodkuo(params.llodkuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wlodkuo = 0.0;
        validate_parameter_wlodkuo(params.wlodkuo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kvtho = 0.0;
        validate_parameter_kvtho(params.kvtho).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lkvtho = 0.0;
        validate_parameter_lkvtho(params.lkvtho).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wkvtho = 0.0;
        validate_parameter_wkvtho(params.wkvtho).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pkvtho = 0.0;
        validate_parameter_pkvtho(params.pkvtho).expect("generated Verilog-A parameter default must satisfy declared range");
        params.llodvth = 0.0;
        validate_parameter_llodvth(params.llodvth).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wlodvth = 0.0;
        validate_parameter_wlodvth(params.wlodvth).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stetao = 0.0;
        validate_parameter_stetao(params.stetao).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lodetao = 1.0;
        validate_parameter_lodetao(params.lodetao).expect("generated Verilog-A parameter default must satisfy declared range");
        params.scref = 1e-6;
        validate_parameter_scref(params.scref).expect("generated Verilog-A parameter default must satisfy declared range");
        params.web = 0.0;
        validate_parameter_web(params.web).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wec = 0.0;
        validate_parameter_wec(params.wec).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swsoa = 0.0;
        validate_parameter_swsoa(params.swsoa).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vgs_max = 1e99;
        validate_parameter_vgs_max(params.vgs_max).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vgd_max = 1e99;
        validate_parameter_vgd_max(params.vgd_max).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vgb_max = 1e99;
        validate_parameter_vgb_max(params.vgb_max).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vds_max = 1e99;
        validate_parameter_vds_max(params.vds_max).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vdb_max = 1e99;
        validate_parameter_vdb_max(params.vdb_max).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vsb_max = 1e99;
        validate_parameter_vsb_max(params.vsb_max).expect("generated Verilog-A parameter default must satisfy declared range");
        params.imax = 1000.0;
        validate_parameter_imax(params.imax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.trj = 21.0;
        validate_parameter_trj(params.trj).expect("generated Verilog-A parameter default must satisfy declared range");
        params.frev = 1000.0;
        validate_parameter_frev(params.frev).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjorbot = 0.001;
        validate_parameter_cjorbot(params.cjorbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjorsti = 1e-9;
        validate_parameter_cjorsti(params.cjorsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjorgat = 1e-9;
        validate_parameter_cjorgat(params.cjorgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbirbot = 1.0;
        validate_parameter_vbirbot(params.vbirbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbirsti = 1.0;
        validate_parameter_vbirsti(params.vbirsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbirgat = 1.0;
        validate_parameter_vbirgat(params.vbirgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbot = 0.5;
        validate_parameter_pbot(params.pbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.psti = 0.5;
        validate_parameter_psti(params.psti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pgat = 0.5;
        validate_parameter_pgat(params.pgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phigbot = 1.16;
        validate_parameter_phigbot(params.phigbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phigsti = 1.16;
        validate_parameter_phigsti(params.phigsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phiggat = 1.16;
        validate_parameter_phiggat(params.phiggat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.idsatrbot = 1e-12;
        validate_parameter_idsatrbot(params.idsatrbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.idsatrsti = 1e-18;
        validate_parameter_idsatrsti(params.idsatrsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.idsatrgat = 1e-18;
        validate_parameter_idsatrgat(params.idsatrgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csrhbot = 100.0;
        validate_parameter_csrhbot(params.csrhbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csrhsti = 0.0001;
        validate_parameter_csrhsti(params.csrhsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csrhgat = 0.0001;
        validate_parameter_csrhgat(params.csrhgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xjunsti = 1e-7;
        validate_parameter_xjunsti(params.xjunsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xjungat = 1e-7;
        validate_parameter_xjungat(params.xjungat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctatbot = 100.0;
        validate_parameter_ctatbot(params.ctatbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctatsti = 0.0001;
        validate_parameter_ctatsti(params.ctatsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctatgat = 0.0001;
        validate_parameter_ctatgat(params.ctatgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mefftatbot = 0.25;
        validate_parameter_mefftatbot(params.mefftatbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mefftatsti = 0.25;
        validate_parameter_mefftatsti(params.mefftatsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mefftatgat = 0.25;
        validate_parameter_mefftatgat(params.mefftatgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cbbtbot = 1e-12;
        validate_parameter_cbbtbot(params.cbbtbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cbbtsti = 1e-18;
        validate_parameter_cbbtsti(params.cbbtsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cbbtgat = 1e-18;
        validate_parameter_cbbtgat(params.cbbtgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbbtrbot = 1000000000.0;
        validate_parameter_fbbtrbot(params.fbbtrbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbbtrsti = 1000000000.0;
        validate_parameter_fbbtrsti(params.fbbtrsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbbtrgat = 1000000000.0;
        validate_parameter_fbbtrgat(params.fbbtrgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stfbbtbot = -0.001;
        validate_parameter_stfbbtbot(params.stfbbtbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stfbbtsti = -0.001;
        validate_parameter_stfbbtsti(params.stfbbtsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stfbbtgat = -0.001;
        validate_parameter_stfbbtgat(params.stfbbtgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbrbot = 10.0;
        validate_parameter_vbrbot(params.vbrbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbrsti = 10.0;
        validate_parameter_vbrsti(params.vbrsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbrgat = 10.0;
        validate_parameter_vbrgat(params.vbrgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbrbot = 4.0;
        validate_parameter_pbrbot(params.pbrbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbrsti = 4.0;
        validate_parameter_pbrsti(params.pbrsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbrgat = 4.0;
        validate_parameter_pbrgat(params.pbrgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjorbotd = 0.001;
        validate_parameter_cjorbotd(params.cjorbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjorstid = 1e-9;
        validate_parameter_cjorstid(params.cjorstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjorgatd = 1e-9;
        validate_parameter_cjorgatd(params.cjorgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbirbotd = 1.0;
        validate_parameter_vbirbotd(params.vbirbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbirstid = 1.0;
        validate_parameter_vbirstid(params.vbirstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbirgatd = 1.0;
        validate_parameter_vbirgatd(params.vbirgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbotd = 0.5;
        validate_parameter_pbotd(params.pbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pstid = 0.5;
        validate_parameter_pstid(params.pstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pgatd = 0.5;
        validate_parameter_pgatd(params.pgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phigbotd = 1.16;
        validate_parameter_phigbotd(params.phigbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phigstid = 1.16;
        validate_parameter_phigstid(params.phigstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phiggatd = 1.16;
        validate_parameter_phiggatd(params.phiggatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.idsatrbotd = 1e-12;
        validate_parameter_idsatrbotd(params.idsatrbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.idsatrstid = 1e-18;
        validate_parameter_idsatrstid(params.idsatrstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.idsatrgatd = 1e-18;
        validate_parameter_idsatrgatd(params.idsatrgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csrhbotd = 100.0;
        validate_parameter_csrhbotd(params.csrhbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csrhstid = 0.0001;
        validate_parameter_csrhstid(params.csrhstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csrhgatd = 0.0001;
        validate_parameter_csrhgatd(params.csrhgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xjunstid = 1e-7;
        validate_parameter_xjunstid(params.xjunstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xjungatd = 1e-7;
        validate_parameter_xjungatd(params.xjungatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctatbotd = 100.0;
        validate_parameter_ctatbotd(params.ctatbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctatstid = 0.0001;
        validate_parameter_ctatstid(params.ctatstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctatgatd = 0.0001;
        validate_parameter_ctatgatd(params.ctatgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mefftatbotd = 0.25;
        validate_parameter_mefftatbotd(params.mefftatbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mefftatstid = 0.25;
        validate_parameter_mefftatstid(params.mefftatstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mefftatgatd = 0.25;
        validate_parameter_mefftatgatd(params.mefftatgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cbbtbotd = 1e-12;
        validate_parameter_cbbtbotd(params.cbbtbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cbbtstid = 1e-18;
        validate_parameter_cbbtstid(params.cbbtstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cbbtgatd = 1e-18;
        validate_parameter_cbbtgatd(params.cbbtgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbbtrbotd = 1000000000.0;
        validate_parameter_fbbtrbotd(params.fbbtrbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbbtrstid = 1000000000.0;
        validate_parameter_fbbtrstid(params.fbbtrstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbbtrgatd = 1000000000.0;
        validate_parameter_fbbtrgatd(params.fbbtrgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stfbbtbotd = -0.001;
        validate_parameter_stfbbtbotd(params.stfbbtbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stfbbtstid = -0.001;
        validate_parameter_stfbbtstid(params.stfbbtstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stfbbtgatd = -0.001;
        validate_parameter_stfbbtgatd(params.stfbbtgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbrbotd = 10.0;
        validate_parameter_vbrbotd(params.vbrbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbrstid = 10.0;
        validate_parameter_vbrstid(params.vbrstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbrgatd = 10.0;
        validate_parameter_vbrgatd(params.vbrgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbrbotd = 4.0;
        validate_parameter_pbrbotd(params.pbrbotd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbrstid = 4.0;
        validate_parameter_pbrstid(params.pbrstid).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbrgatd = 4.0;
        validate_parameter_pbrgatd(params.pbrgatd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swjunexp = 0.0;
        validate_parameter_swjunexp(params.swjunexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vjunref = 2.5;
        validate_parameter_vjunref(params.vjunref).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fjunq = 0.03;
        validate_parameter_fjunq(params.fjunq).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vjunrefd = 2.5;
        validate_parameter_vjunrefd(params.vjunrefd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fjunqd = 0.03;
        validate_parameter_fjunqd(params.fjunqd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dta = 0.0;
        validate_parameter_dta(params.dta).expect("generated Verilog-A parameter default must satisfy declared range");
        params
    }
}

fn validate_parameter_level(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LEVEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_type_(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TYPE' must be finite, got {}", value));
    }
    if value < -1.0 {
        return Err(format!("parameter 'TYPE' must be >= -1.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'TYPE' must be <= 1.0, got {}", value));
    }
    if value == 0.0 {
        return Err(format!("parameter 'TYPE' must not equal 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tr(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TR' must be finite, got {}", value));
    }
    if value < -273.0 {
        return Err(format!("parameter 'TR' must be >= -273.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWGEO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWGEO' must be >= 0.0, got {}", value));
    }
    if value > 2.0 {
        return Err(format!("parameter 'SWGEO' must be <= 2.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swigate(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWIGATE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWIGATE' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWIGATE' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swimpact(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWIMPACT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWIMPACT' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWIMPACT' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swgidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWGIDL' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWGIDL' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWGIDL' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swjuncap(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWJUNCAP' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWJUNCAP' must be >= 0.0, got {}", value));
    }
    if value > 3.0 {
        return Err(format!("parameter 'SWJUNCAP' must be <= 3.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swjunasym(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWJUNASYM' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWJUNASYM' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWJUNASYM' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWNUD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWNUD' must be >= 0.0, got {}", value));
    }
    if value > 2.0 {
        return Err(format!("parameter 'SWNUD' must be <= 2.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWEDGE' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWEDGE' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swdelvtac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWDELVTAC' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWDELVTAC' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWDELVTAC' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swign(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWIGN' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWIGN' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWIGN' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_qmc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'QMC' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'QMC' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'L' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'L' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'W' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'W' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sa(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SA' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sca(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SCA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SCA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_scb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SCB' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SCB' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_scc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SCC' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SCC' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nf(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NF' must be finite, got {}", value));
    }
    if value < 1.0 {
        return Err(format!("parameter 'NF' must be >= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ngcon(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NGCON' must be finite, got {}", value));
    }
    if value < 1.0 {
        return Err(format!("parameter 'NGCON' must be >= 1.0, got {}", value));
    }
    if value > 2.0 {
        return Err(format!("parameter 'NGCON' must be <= 2.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xgw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XGW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nrs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NRS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nrd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NRD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_jw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'JW' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'JW' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_delvto(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DELVTO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_factuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FACTUO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FACTUO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_delvtoedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DELVTOEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_factuoedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FACTUOEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FACTUOEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_absource(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ABSOURCE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ABSOURCE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lssource(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LSSOURCE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LSSOURCE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lgsource(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LGSOURCE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LGSOURCE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_abdrain(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ABDRAIN' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ABDRAIN' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lsdrain(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LSDRAIN' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LSDRAIN' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lgdrain(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LGDRAIN' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LGDRAIN' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_as_(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ps(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ad(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mult(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MULT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'MULT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_st2vfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ST2VFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TOX' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'TOX' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_epsrox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'EPSROX' must be finite, got {}", value));
    }
    if value < 1.0 {
        return Err(format!("parameter 'EPSROX' must be >= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_neff(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NEFF' must be finite, got {}", value));
    }
    if value < 1e20 {
        return Err(format!("parameter 'NEFF' must be >= 1e20, got {}", value));
    }
    if value > 1e26 {
        return Err(format!("parameter 'NEFF' must be <= 1e26, got {}", value));
    }
    Ok(())
}

fn validate_parameter_facneffac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FACNEFFAC' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FACNEFFAC' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gfacnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GFACNUD' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'GFACNUD' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vsbnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VSBNUD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'VSBNUD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dvsbnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DVSBNUD' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'DVSBNUD' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vnsub(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VNSUB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nslp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NSLP' must be finite, got {}", value));
    }
    if value < 0.001 {
        return Err(format!("parameter 'NSLP' must be >= 0.001, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dnsub(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DNSUB' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'DNSUB' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'DNSUB' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphib(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_delvtac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DELVTAC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_np(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NP' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NP' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_toxov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TOXOV' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'TOXOV' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_toxovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TOXOVD' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'TOXOVD' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NOV' must be finite, got {}", value));
    }
    if value < 1e23 {
        return Err(format!("parameter 'NOV' must be >= 1e23, got {}", value));
    }
    if value > 1e27 {
        return Err(format!("parameter 'NOV' must be <= 1e27, got {}", value));
    }
    Ok(())
}

fn validate_parameter_novd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NOVD' must be finite, got {}", value));
    }
    if value < 1e23 {
        return Err(format!("parameter 'NOVD' must be >= 1e23, got {}", value));
    }
    if value > 1e27 {
        return Err(format!("parameter 'NOVD' must be <= 1e27, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ct(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTG' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTG' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stct(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STCT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cf(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CF' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CF' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFB' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFB' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'CFB' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_psce(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_psceb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEB' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCEB' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'PSCEB' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_psced(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCED' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCED' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_betn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BETN' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'BETN' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbet(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBET' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mue(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MUE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'MUE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stmue(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STMUE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_themu(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THEMU' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'THEMU' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stthemu(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STTHEMU' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stcs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STCS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thecs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THECS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'THECS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stthecs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STTHECS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xcor(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XCOR' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'XCOR' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stxcor(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STXCOR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_feta(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FETA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FETA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_strs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STRS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rsb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSB' must be finite, got {}", value));
    }
    if value < -0.5 {
        return Err(format!("parameter 'RSB' must be >= -0.5, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'RSB' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rsg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSG' must be finite, got {}", value));
    }
    if value < -0.5 {
        return Err(format!("parameter 'RSG' must be >= -0.5, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESAT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'THESAT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stthesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STTHESAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesatb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESATB' must be finite, got {}", value));
    }
    if value < -0.5 {
        return Err(format!("parameter 'THESATB' must be >= -0.5, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'THESATB' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesatg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESATG' must be finite, got {}", value));
    }
    if value < -0.5 {
        return Err(format!("parameter 'THESATG' must be >= -0.5, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AX' must be finite, got {}", value));
    }
    if value < 2.0 {
        return Err(format!("parameter 'AX' must be >= 2.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ALP' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP1' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ALP1' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP2' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ALP2' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VP' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'VP' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A1' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'A1' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A2' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'A2' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sta2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STA2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a3(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A3' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'A3' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a4(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A4' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'A4' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gco(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GCO' must be finite, got {}", value));
    }
    if value < -10.0 {
        return Err(format!("parameter 'GCO' must be >= -10.0, got {}", value));
    }
    if value > 10.0 {
        return Err(format!("parameter 'GCO' must be <= 10.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_iginv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IGINV' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IGINV' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_igov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IGOV' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IGOV' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_igovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IGOVD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IGOVD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stig(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STIG' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gc2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GC2' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'GC2' must be >= 0.0, got {}", value));
    }
    if value > 10.0 {
        return Err(format!("parameter 'GC2' must be <= 10.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gc3(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GC3' must be finite, got {}", value));
    }
    if value < -2.0 {
        return Err(format!("parameter 'GC3' must be >= -2.0, got {}", value));
    }
    if value > 2.0 {
        return Err(format!("parameter 'GC3' must be <= 2.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_chib(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CHIB' must be finite, got {}", value));
    }
    if value < 1.0 {
        return Err(format!("parameter 'CHIB' must be >= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_agidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AGIDL' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AGIDL' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_agidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AGIDLD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AGIDLD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_bgidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BGIDL' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'BGIDL' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_bgidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BGIDLD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'BGIDLD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbgidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBGIDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbgidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBGIDLD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cgidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CGIDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cgidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CGIDLD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'COX' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'COX' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cgov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CGOV' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CGOV' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cgovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CGOVD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CGOVD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cgbov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CGBOV' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CGBOV' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfr(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFR' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFR' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfrd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFRD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFRD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fnt(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FNT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FNT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fntexc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FNTEXC' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FNTEXC' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfa(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NFA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFB' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NFB' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFC' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NFC' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ef(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'EF' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'EF' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphibedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_neffedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NEFFEDGE' must be finite, got {}", value));
    }
    if value < 1e20 {
        return Err(format!("parameter 'NEFFEDGE' must be >= 1e20, got {}", value));
    }
    if value > 1e26 {
        return Err(format!("parameter 'NEFFEDGE' must be <= 1e26, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_betnedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BETNEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'BETNEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbetedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBETEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_psceedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCEEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pscebedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEBEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCEBEDGE' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'PSCEBEDGE' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pscededge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEDEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCEDEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfdedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFDEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFDEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFBEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFBEDGE' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'CFBEDGE' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fntedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FNTEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FNTEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfaedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFAEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NFAEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFBEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NFBEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfcedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFCEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NFCEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_efedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'EFEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'EFEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RG' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RG' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rse(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RSE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rde(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RDE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RDE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rbulk(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RBULK' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RBULK' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rwell(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RWELL' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RWELL' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rjuns(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RJUNS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RJUNS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rjund(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RJUND' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RJUND' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_povfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POVFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plvfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLVFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwvfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWVFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwvfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWVFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postvfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTVFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plstvfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLSTVFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwstvfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWSTVFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwstvfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWSTVFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_post2vfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POST2VFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_potox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POTOX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poepsrox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POEPSROX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poneff(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONEFF' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plneff(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNEFF' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwneff(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNEFF' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwneff(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNEFF' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pofacneffac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POFACNEFFAC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plfacneffac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLFACNEFFAC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwfacneffac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWFACNEFFAC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwfacneffac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWFACNEFFAC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pogfacnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POGFACNUD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plgfacnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLGFACNUD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwgfacnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWGFACNUD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwgfacnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWGFACNUD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_povsbnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POVSBNUD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_podvsbnud(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PODVSBNUD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_povnsub(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POVNSUB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponslp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONSLP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_podnsub(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PODNSUB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_podphib(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PODPHIB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pldphib(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLDPHIB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwdphib(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWDPHIB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwdphib(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWDPHIB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_podelvtac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PODELVTAC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pldelvtac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLDELVTAC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwdelvtac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWDELVTAC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwdelvtac(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWDELVTAC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plnp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwnp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwnp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_potoxov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POTOXOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_potoxovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POTOXOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plnov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwnov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwnov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plnovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwnovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwnovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poct(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plct(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwct(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwct(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poctg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCTG' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poctb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCTB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postct(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTCT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocf(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCF' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plcf(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCF' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwcf(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCF' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwcf(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCF' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocfd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCFD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_popsce(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POPSCE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plpsce(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLPSCE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwpsce(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWPSCE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwpsce(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWPSCE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_popsceb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POPSCEB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_popsced(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POPSCED' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pobetn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POBETN' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plbetn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLBETN' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwbetn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWBETN' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwbetn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWBETN' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postbet(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTBET' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plstbet(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLSTBET' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwstbet(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWSTBET' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwstbet(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWSTBET' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pomue(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POMUE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plmue(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLMUE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwmue(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWMUE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwmue(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWMUE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postmue(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTMUE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pothemu(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POTHEMU' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postthemu(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTTHEMU' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plcs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwcs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwcs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postcs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTCS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pothecs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POTHECS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postthecs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTTHECS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poxcor(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POXCOR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plxcor(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLXCOR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwxcor(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWXCOR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwxcor(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWXCOR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postxcor(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTXCOR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pofeta(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POFETA' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pors(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PORS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plrs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLRS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwrs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWRS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwrs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWRS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postrs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTRS' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_porsb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PORSB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_porsg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PORSG' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pothesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POTHESAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plthesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLTHESAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwthesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWTHESAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwthesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWTHESAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postthesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTTHESAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plstthesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLSTTHESAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwstthesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWSTTHESAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwstthesat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWSTTHESAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pothesatb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POTHESATB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plthesatb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLTHESATB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwthesatb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWTHESATB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwthesatb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWTHESATB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pothesatg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POTHESATG' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plthesatg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLTHESATG' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwthesatg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWTHESATG' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwthesatg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWTHESATG' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POAX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLAX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWAX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWAX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poalp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POALP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plalp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLALP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwalp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWALP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwalp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWALP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poalp1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POALP1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plalp1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLALP1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwalp1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWALP1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwalp1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWALP1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poalp2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POALP2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plalp2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLALP2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwalp2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWALP2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwalp2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWALP2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_povp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POVP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poa1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POA1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pla1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLA1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwa1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWA1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwa1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWA1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poa2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POA2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_posta2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTA2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poa3(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POA3' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pla3(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLA3' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwa3(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWA3' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwa3(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWA3' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poa4(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POA4' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pla4(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLA4' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwa4(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWA4' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwa4(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWA4' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pogco(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POGCO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poiginv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POIGINV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pliginv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLIGINV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwiginv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWIGINV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwiginv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWIGINV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poigov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POIGOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pligov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLIGOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwigov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWIGOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwigov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWIGOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poigovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POIGOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pligovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLIGOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwigovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWIGOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwigovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWIGOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postig(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTIG' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pogc2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POGC2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pogc3(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POGC3' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pochib(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCHIB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poagidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POAGIDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plagidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLAGIDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwagidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWAGIDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwagidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWAGIDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poagidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POAGIDLD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plagidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLAGIDLD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwagidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWAGIDLD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwagidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWAGIDLD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pobgidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POBGIDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pobgidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POBGIDLD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postbgidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTBGIDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postbgidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTBGIDLD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocgidl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCGIDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocgidld(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCGIDLD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCOX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plcox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCOX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwcox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCOX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwcox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCOX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocgov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCGOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plcgov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCGOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwcgov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCGOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwcgov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCGOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocgovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCGOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plcgovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCGOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwcgovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCGOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwcgovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCGOVD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocgbov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCGBOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plcgbov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCGBOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwcgbov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCGBOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwcgbov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCGBOV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocfr(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCFR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plcfr(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCFR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwcfr(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCFR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwcfr(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCFR' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocfrd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCFRD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plcfrd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCFRD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwcfrd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCFRD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwcfrd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCFRD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pofnt(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POFNT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pofntexc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POFNTEXC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plfntexc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLFNTEXC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwfntexc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWFNTEXC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwfntexc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWFNTEXC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponfa(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONFA' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plnfa(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNFA' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwnfa(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNFA' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwnfa(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNFA' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plnfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwnfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwnfb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNFB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponfc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONFC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plnfc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNFC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwnfc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNFC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwnfc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNFC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poef(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POEF' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_povfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POVFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postvfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTVFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plstvfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLSTVFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwstvfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWSTVFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwstvfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWSTVFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_podphibedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PODPHIBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pldphibedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLDPHIBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwdphibedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWDPHIBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwdphibedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWDPHIBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poneffedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONEFFEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plneffedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNEFFEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwneffedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNEFFEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwneffedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNEFFEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poctedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCTEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plctedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCTEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwctedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCTEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwctedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCTEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pobetnedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POBETNEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plbetnedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLBETNEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwbetnedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWBETNEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwbetnedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWBETNEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_postbetedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POSTBETEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plstbetedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLSTBETEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwstbetedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWSTBETEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwstbetedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWSTBETEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_popsceedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POPSCEEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plpsceedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLPSCEEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwpsceedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWPSCEEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwpsceedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWPSCEEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_popscebedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POPSCEBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_popscededge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POPSCEDEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocfedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCFEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plcfedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLCFEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwcfedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWCFEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwcfedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWCFEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocfdedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCFDEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pocfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POCFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pofntedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POFNTEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponfaedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONFAEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plnfaedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNFAEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwnfaedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNFAEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwnfaedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNFAEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plnfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwnfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwnfbedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNFBEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ponfcedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PONFCEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plnfcedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLNFCEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwnfcedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWNFCEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwnfcedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWNFCEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_poefedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POEFEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pokvthowe(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POKVTHOWE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plkvthowe(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLKVTHOWE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwkvthowe(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWKVTHOWE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwkvthowe(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWKVTHOWE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pokuowe(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'POKUOWE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plkuowe(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLKUOWE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pwkuowe(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PWKUOWE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_plwkuowe(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PLWKUOWE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lmin(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LMIN' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lmax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LMAX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wmin(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WMIN' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wmax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WMAX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lvaro(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LVARO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lvarl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LVARL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lvarw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LVARW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lap(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LAP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wvaro(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WVARO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wvarl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WVARL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wvarw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WVARW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WOT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dlq(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DLQ' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dwq(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DWQ' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vfbo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VFBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vfbl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VFBL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vfbw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VFBW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vfblw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VFBLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfbo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfbl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFBL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfbw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFBW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfblw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFBLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_st2vfbo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ST2VFBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_toxo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TOXO' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'TOXO' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_epsroxo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'EPSROXO' must be finite, got {}", value));
    }
    if value < 1.0 {
        return Err(format!("parameter 'EPSROXO' must be >= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsubo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NSUBO' must be finite, got {}", value));
    }
    if value < 1e20 {
        return Err(format!("parameter 'NSUBO' must be >= 1e20, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsubw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NSUBW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wseg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WSEG' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'WSEG' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_npck(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NPCK' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NPCK' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_npckw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NPCKW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wsegp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WSEGP' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'WSEGP' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lpck(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LPCK' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'LPCK' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lpckw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LPCKW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fol1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FOL1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fol2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FOL2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_facneffaco(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FACNEFFACO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_facneffacl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FACNEFFACL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_facneffacw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FACNEFFACW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_facneffaclw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FACNEFFACLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gfacnudo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GFACNUDO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gfacnudl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GFACNUDL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gfacnudlexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GFACNUDLEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gfacnudw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GFACNUDW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gfacnudlw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GFACNUDLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vsbnudo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VSBNUDO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dvsbnudo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DVSBNUDO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vnsubo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VNSUBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nslpo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NSLPO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dnsubo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DNSUBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphibo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphibl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphiblexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBLEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphibw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphiblw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_delvtaco(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DELVTACO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_delvtacl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DELVTACL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_delvtaclexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DELVTACLEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_delvtacw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DELVTACW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_delvtaclw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DELVTACLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_npo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NPO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_npl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NPL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_toxovo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TOXOVO' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'TOXOVO' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_toxovdo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TOXOVDO' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'TOXOVDO' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lov(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LOV' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LOV' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lovd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LOVD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LOVD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_novo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NOVO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_novdo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NOVDO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cto(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctlexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTLEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctlw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctgo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTGO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTGO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctbo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stcto(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STCTO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cflexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFLEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfdo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFDO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFDO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfbo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pscel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pscelexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCELEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pscew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pscebo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEBO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCEBO' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'PSCEBO' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pscedo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEDO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCEDO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_uo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'UO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'UO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbet1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBET1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbet1w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBET1W' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lp1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LP1' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'LP1' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lp1w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LP1W' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbet2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBET2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lp2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LP2' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'LP2' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_betw1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BETW1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_betw2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BETW2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wbet(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WBET' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'WBET' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbeto(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBETO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbetl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBETL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbetw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBETW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbetlw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBETLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mueo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MUEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_muew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MUEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stmueo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STMUEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_themuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THEMUO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stthemuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STTHEMUO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cso(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cslexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSLEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cslw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stcso(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STCSO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thecso(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THECSO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'THECSO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stthecso(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STTHECSO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xcoro(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XCORO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xcorl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XCORL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xcorw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XCORW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xcorlw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XCORLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stxcoro(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STXCORO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fetao(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FETAO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rsw1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSW1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rsw2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSW2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_strso(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STRSO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rsbo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rsgo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSGO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesato(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESATO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesatl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESATL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesatlexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESATLEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesatw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESATW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesatlw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESATLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stthesato(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STTHESATO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stthesatl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STTHESATL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stthesatw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STTHESATW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stthesatlw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STTHESATLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesatbo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESATBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_thesatgo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THESATGO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_axo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AXO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_axl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AXL' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AXL' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alpl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALPL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alplexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALPLEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alpw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALPW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp1l1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP1L1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp1lexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP1LEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp1l2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP1L2' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ALP1L2' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp1w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP1W' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp2l1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP2L1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp2lexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP2LEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp2l2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP2L2' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ALP2L2' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alp2w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALP2W' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vpo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VPO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a1o(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A1O' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a1l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A1L' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a1w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A1W' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a2o(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A2O' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sta2o(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STA2O' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a3o(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A3O' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a3l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A3L' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a3w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A3W' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a4o(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A4O' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a4l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A4L' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a4w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'A4W' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gcoo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GCOO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_iginvlw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IGINVLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_igovw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IGOVW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_igovdw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IGOVDW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stigo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STIGO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gc2o(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GC2O' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gc3o(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GC3O' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_chibo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CHIBO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_agidlw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AGIDLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_agidldw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AGIDLDW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_bgidlo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BGIDLO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_bgidldo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BGIDLDO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbgidlo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBGIDLO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbgidldo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBGIDLDO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cgidlo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CGIDLO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cgidldo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CGIDLDO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cgbovl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CGBOVL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfrw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFRW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfrdw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFRDW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fnto(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FNTO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fntexcl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FNTEXCL' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FNTEXCL' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfalw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFALW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfblw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFBLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfclw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFCLW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_efo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'EFO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lintnoi(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LINTNOI' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_alpnoi(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ALPNOI' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WEDGE' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'WEDGE' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wedgew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WEDGEW' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'WEDGEW' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vfbedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VFBEDGEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfbedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFBEDGEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfbedgel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFBEDGEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfbedgew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFBEDGEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stvfbedgelw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STVFBEDGELW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphibedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBEDGEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphibedgel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBEDGEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphibedgelexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBEDGELEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphibedgew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBEDGEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dphibedgelw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DPHIBEDGELW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsubedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NSUBEDGEO' must be finite, got {}", value));
    }
    if value < 1e20 {
        return Err(format!("parameter 'NSUBEDGEO' must be >= 1e20, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsubedgel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NSUBEDGEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsubedgelexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NSUBEDGELEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsubedgew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NSUBEDGEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsubedgelw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NSUBEDGELW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTEDGEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctedgel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTEDGEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctedgelexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTEDGELEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbetedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBETEDGE' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lpedge(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LPEDGE' must be finite, got {}", value));
    }
    if value < 1e-10 {
        return Err(format!("parameter 'LPEDGE' must be >= 1e-10, got {}", value));
    }
    Ok(())
}

fn validate_parameter_betedgew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BETEDGEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbetedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBETEDGEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbetedgel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBETEDGEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbetedgew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBETEDGEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stbetedgelw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STBETEDGELW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_psceedgel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEEDGEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_psceedgelexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEEDGELEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_psceedgew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEEDGEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pscebedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEBEDGEO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCEBEDGEO' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'PSCEBEDGEO' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pscededgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSCEDEDGEO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PSCEDEDGEO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfedgel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFEDGEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfedgelexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFEDGELEXP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfedgew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFEDGEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfdedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFDEDGEO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFDEDGEO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cfbedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CFBEDGEO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CFBEDGEO' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'CFBEDGEO' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fntedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FNTEDGEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfaedgelw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFAEDGELW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfbedgelw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFBEDGELW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nfcedgelw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NFCEDGELW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_efedgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'EFEDGEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kvthoweo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KVTHOWEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kvthowel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KVTHOWEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kvthowew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KVTHOWEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kvthowelw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KVTHOWELW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kuoweo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KUOWEO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kuowel(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KUOWEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kuowew(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KUOWEW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kuowelw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KUOWELW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rgo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RGO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rint(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RINT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RINT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rvpoly(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RVPOLY' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RVPOLY' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rshg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSHG' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RSHG' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dlsil(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DLSIL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rsh(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSH' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rshd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSHD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rbulko(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RBULKO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rwello(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RWELLO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rjunso(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RJUNSO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rjundo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RJUNDO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_saref(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SAREF' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'SAREF' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sbref(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SBREF' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'SBREF' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wlod(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WLOD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KUO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kvsat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KVSAT' must be finite, got {}", value));
    }
    if value < -1.0 {
        return Err(format!("parameter 'KVSAT' must be >= -1.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'KVSAT' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tkuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TKUO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lkuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LKUO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wkuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WKUO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pkuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PKUO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_llodkuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LLODKUO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LLODKUO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wlodkuo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WLODKUO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'WLODKUO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kvtho(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KVTHO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lkvtho(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LKVTHO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wkvtho(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WKVTHO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pkvtho(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PKVTHO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_llodvth(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LLODVTH' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LLODVTH' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wlodvth(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WLODVTH' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'WLODVTH' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stetao(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STETAO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lodetao(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LODETAO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LODETAO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_scref(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SCREF' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SCREF' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_web(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WEB' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wec(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WEC' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swsoa(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWSOA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWSOA' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWSOA' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vgs_max(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VGS_MAX' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'VGS_MAX' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vgd_max(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VGD_MAX' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'VGD_MAX' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vgb_max(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VGB_MAX' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'VGB_MAX' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vds_max(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VDS_MAX' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'VDS_MAX' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vdb_max(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VDB_MAX' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'VDB_MAX' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vsb_max(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VSB_MAX' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'VSB_MAX' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_imax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IMAX' must be finite, got {}", value));
    }
    if value < 1e-12 {
        return Err(format!("parameter 'IMAX' must be >= 1e-12, got {}", value));
    }
    Ok(())
}

fn validate_parameter_trj(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TRJ' must be finite, got {}", value));
    }
    if value < -250.0 {
        return Err(format!("parameter 'TRJ' must be >= -250.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_frev(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FREV' must be finite, got {}", value));
    }
    if value < 10.0 {
        return Err(format!("parameter 'FREV' must be >= 10.0, got {}", value));
    }
    if value > 10000000000.0 {
        return Err(format!("parameter 'FREV' must be <= 10000000000.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjorbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CJORBOT' must be finite, got {}", value));
    }
    if value < 1e-12 {
        return Err(format!("parameter 'CJORBOT' must be >= 1e-12, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjorsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CJORSTI' must be finite, got {}", value));
    }
    if value < 1e-18 {
        return Err(format!("parameter 'CJORSTI' must be >= 1e-18, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjorgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CJORGAT' must be finite, got {}", value));
    }
    if value < 1e-18 {
        return Err(format!("parameter 'CJORGAT' must be >= 1e-18, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbirbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBIRBOT' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'VBIRBOT' must be >= 0.05, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbirsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBIRSTI' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'VBIRSTI' must be >= 0.05, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbirgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBIRGAT' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'VBIRGAT' must be >= 0.05, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBOT' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'PBOT' must be >= 0.05, got {}", value));
    }
    if value > 0.95 {
        return Err(format!("parameter 'PBOT' must be <= 0.95, got {}", value));
    }
    Ok(())
}

fn validate_parameter_psti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSTI' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'PSTI' must be >= 0.05, got {}", value));
    }
    if value > 0.95 {
        return Err(format!("parameter 'PSTI' must be <= 0.95, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PGAT' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'PGAT' must be >= 0.05, got {}", value));
    }
    if value > 0.95 {
        return Err(format!("parameter 'PGAT' must be <= 0.95, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phigbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHIGBOT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phigsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHIGSTI' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phiggat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHIGGAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_idsatrbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IDSATRBOT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IDSATRBOT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_idsatrsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IDSATRSTI' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IDSATRSTI' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_idsatrgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IDSATRGAT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IDSATRGAT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csrhbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSRHBOT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CSRHBOT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csrhsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSRHSTI' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CSRHSTI' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csrhgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSRHGAT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CSRHGAT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xjunsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XJUNSTI' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'XJUNSTI' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xjungat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XJUNGAT' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'XJUNGAT' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctatbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTATBOT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTATBOT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctatsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTATSTI' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTATSTI' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctatgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTATGAT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTATGAT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mefftatbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MEFFTATBOT' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'MEFFTATBOT' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mefftatsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MEFFTATSTI' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'MEFFTATSTI' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mefftatgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MEFFTATGAT' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'MEFFTATGAT' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cbbtbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CBBTBOT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CBBTBOT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cbbtsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CBBTSTI' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CBBTSTI' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cbbtgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CBBTGAT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CBBTGAT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbbtrbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBBTRBOT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbbtrsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBBTRSTI' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbbtrgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBBTRGAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stfbbtbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STFBBTBOT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stfbbtsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STFBBTSTI' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stfbbtgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STFBBTGAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbrbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBRBOT' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'VBRBOT' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbrsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBRSTI' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'VBRSTI' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbrgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBRGAT' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'VBRGAT' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbrbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBRBOT' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'PBRBOT' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbrsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBRSTI' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'PBRSTI' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbrgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBRGAT' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'PBRGAT' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjorbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CJORBOTD' must be finite, got {}", value));
    }
    if value < 1e-12 {
        return Err(format!("parameter 'CJORBOTD' must be >= 1e-12, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjorstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CJORSTID' must be finite, got {}", value));
    }
    if value < 1e-18 {
        return Err(format!("parameter 'CJORSTID' must be >= 1e-18, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjorgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CJORGATD' must be finite, got {}", value));
    }
    if value < 1e-18 {
        return Err(format!("parameter 'CJORGATD' must be >= 1e-18, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbirbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBIRBOTD' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'VBIRBOTD' must be >= 0.05, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbirstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBIRSTID' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'VBIRSTID' must be >= 0.05, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbirgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBIRGATD' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'VBIRGATD' must be >= 0.05, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBOTD' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'PBOTD' must be >= 0.05, got {}", value));
    }
    if value > 0.95 {
        return Err(format!("parameter 'PBOTD' must be <= 0.95, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSTID' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'PSTID' must be >= 0.05, got {}", value));
    }
    if value > 0.95 {
        return Err(format!("parameter 'PSTID' must be <= 0.95, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PGATD' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'PGATD' must be >= 0.05, got {}", value));
    }
    if value > 0.95 {
        return Err(format!("parameter 'PGATD' must be <= 0.95, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phigbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHIGBOTD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phigstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHIGSTID' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phiggatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHIGGATD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_idsatrbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IDSATRBOTD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IDSATRBOTD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_idsatrstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IDSATRSTID' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IDSATRSTID' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_idsatrgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IDSATRGATD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IDSATRGATD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csrhbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSRHBOTD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CSRHBOTD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csrhstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSRHSTID' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CSRHSTID' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csrhgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSRHGATD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CSRHGATD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xjunstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XJUNSTID' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'XJUNSTID' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xjungatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XJUNGATD' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'XJUNGATD' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctatbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTATBOTD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTATBOTD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctatstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTATSTID' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTATSTID' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctatgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTATGATD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTATGATD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mefftatbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MEFFTATBOTD' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'MEFFTATBOTD' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mefftatstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MEFFTATSTID' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'MEFFTATSTID' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mefftatgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MEFFTATGATD' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'MEFFTATGATD' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cbbtbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CBBTBOTD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CBBTBOTD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cbbtstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CBBTSTID' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CBBTSTID' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cbbtgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CBBTGATD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CBBTGATD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbbtrbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBBTRBOTD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbbtrstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBBTRSTID' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbbtrgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBBTRGATD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stfbbtbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STFBBTBOTD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stfbbtstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STFBBTSTID' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stfbbtgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STFBBTGATD' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbrbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBRBOTD' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'VBRBOTD' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbrstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBRSTID' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'VBRSTID' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbrgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBRGATD' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'VBRGATD' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbrbotd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBRBOTD' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'PBRBOTD' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbrstid(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBRSTID' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'PBRSTID' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbrgatd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBRGATD' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'PBRGATD' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swjunexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWJUNEXP' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWJUNEXP' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWJUNEXP' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vjunref(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VJUNREF' must be finite, got {}", value));
    }
    if value < 0.5 {
        return Err(format!("parameter 'VJUNREF' must be >= 0.5, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fjunq(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FJUNQ' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FJUNQ' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vjunrefd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VJUNREFD' must be finite, got {}", value));
    }
    if value < 0.5 {
        return Err(format!("parameter 'VJUNREFD' must be >= 0.5, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fjunqd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FJUNQD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FJUNQD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dta(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DTA' must be finite, got {}", value));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub nodes: [usize; 12],
    pub branches: [usize; 7],
    pub params: Parameters,
    pub(crate) param_given: [bool; 808],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 11],
    pub(crate) ddt_state_previous: [f64; 11],
    pub(crate) ddt_state_initialized: [bool; 11],
    pub(crate) time: f64,
    pub(crate) timestep: f64,
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["NOI", "GP", "SI", "DI", "BP", "BI", "BS", "BD"];

    pub const BRANCH_COUNT: usize = 7;
    pub const PARAMETER_COUNT: usize = 808;
    pub const VARIABLE_COUNT: usize = 2609;
    pub const DDT_STATE_COUNT: usize = 11;
    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;
    pub const DDT_EPSILON: f64 = 1.0e-20;

    pub fn new(nodes: &[usize]) -> Self {
        assert_eq!(nodes.len(), Self::NODE_COUNT, "generated Verilog-A node count mismatch");
        let mut mapped = [0usize; Self::NODE_COUNT];
        mapped.copy_from_slice(nodes);
        Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::default(),
            param_given: [false; Self::PARAMETER_COUNT],
            multiplicity: 1.0,
            ddt_state_current: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_previous: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_initialized: [false; Self::DDT_STATE_COUNT],
            time: 0.0,
            timestep: 0.0,
        }
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "level" => { validate_parameter_level(value)?; self.params.level = value; self.mark_param_given(0); Ok(()) }
            "type" => { validate_parameter_type_(value)?; self.params.type_ = value; self.mark_param_given(1); Ok(()) }
            "tr" => { validate_parameter_tr(value)?; self.params.tr = value; self.mark_param_given(2); Ok(()) }
            "swgeo" => { validate_parameter_swgeo(value)?; self.params.swgeo = value; self.mark_param_given(3); Ok(()) }
            "swigate" => { validate_parameter_swigate(value)?; self.params.swigate = value; self.mark_param_given(4); Ok(()) }
            "swimpact" => { validate_parameter_swimpact(value)?; self.params.swimpact = value; self.mark_param_given(5); Ok(()) }
            "swgidl" => { validate_parameter_swgidl(value)?; self.params.swgidl = value; self.mark_param_given(6); Ok(()) }
            "swjuncap" => { validate_parameter_swjuncap(value)?; self.params.swjuncap = value; self.mark_param_given(7); Ok(()) }
            "swjunasym" => { validate_parameter_swjunasym(value)?; self.params.swjunasym = value; self.mark_param_given(8); Ok(()) }
            "swnud" => { validate_parameter_swnud(value)?; self.params.swnud = value; self.mark_param_given(9); Ok(()) }
            "swedge" => { validate_parameter_swedge(value)?; self.params.swedge = value; self.mark_param_given(10); Ok(()) }
            "swdelvtac" => { validate_parameter_swdelvtac(value)?; self.params.swdelvtac = value; self.mark_param_given(11); Ok(()) }
            "swign" => { validate_parameter_swign(value)?; self.params.swign = value; self.mark_param_given(12); Ok(()) }
            "qmc" => { validate_parameter_qmc(value)?; self.params.qmc = value; self.mark_param_given(13); Ok(()) }
            "l" => { validate_parameter_l(value)?; self.params.l = value; self.mark_param_given(14); Ok(()) }
            "w" => { validate_parameter_w(value)?; self.params.w = value; self.mark_param_given(15); Ok(()) }
            "sa" => { validate_parameter_sa(value)?; self.params.sa = value; self.mark_param_given(16); Ok(()) }
            "sb" => { validate_parameter_sb(value)?; self.params.sb = value; self.mark_param_given(17); Ok(()) }
            "sd" => { validate_parameter_sd(value)?; self.params.sd = value; self.mark_param_given(18); Ok(()) }
            "sca" => { validate_parameter_sca(value)?; self.params.sca = value; self.mark_param_given(19); Ok(()) }
            "scb" => { validate_parameter_scb(value)?; self.params.scb = value; self.mark_param_given(20); Ok(()) }
            "scc" => { validate_parameter_scc(value)?; self.params.scc = value; self.mark_param_given(21); Ok(()) }
            "sc" => { validate_parameter_sc(value)?; self.params.sc = value; self.mark_param_given(22); Ok(()) }
            "nf" => { validate_parameter_nf(value)?; self.params.nf = value; self.mark_param_given(23); Ok(()) }
            "ngcon" => { validate_parameter_ngcon(value)?; self.params.ngcon = value; self.mark_param_given(24); Ok(()) }
            "xgw" => { validate_parameter_xgw(value)?; self.params.xgw = value; self.mark_param_given(25); Ok(()) }
            "nrs" => { validate_parameter_nrs(value)?; self.params.nrs = value; self.mark_param_given(26); Ok(()) }
            "nrd" => { validate_parameter_nrd(value)?; self.params.nrd = value; self.mark_param_given(27); Ok(()) }
            "jw" => { validate_parameter_jw(value)?; self.params.jw = value; self.mark_param_given(28); Ok(()) }
            "delvto" => { validate_parameter_delvto(value)?; self.params.delvto = value; self.mark_param_given(29); Ok(()) }
            "factuo" => { validate_parameter_factuo(value)?; self.params.factuo = value; self.mark_param_given(30); Ok(()) }
            "delvtoedge" => { validate_parameter_delvtoedge(value)?; self.params.delvtoedge = value; self.mark_param_given(31); Ok(()) }
            "factuoedge" => { validate_parameter_factuoedge(value)?; self.params.factuoedge = value; self.mark_param_given(32); Ok(()) }
            "absource" => { validate_parameter_absource(value)?; self.params.absource = value; self.mark_param_given(33); Ok(()) }
            "lssource" => { validate_parameter_lssource(value)?; self.params.lssource = value; self.mark_param_given(34); Ok(()) }
            "lgsource" => { validate_parameter_lgsource(value)?; self.params.lgsource = value; self.mark_param_given(35); Ok(()) }
            "abdrain" => { validate_parameter_abdrain(value)?; self.params.abdrain = value; self.mark_param_given(36); Ok(()) }
            "lsdrain" => { validate_parameter_lsdrain(value)?; self.params.lsdrain = value; self.mark_param_given(37); Ok(()) }
            "lgdrain" => { validate_parameter_lgdrain(value)?; self.params.lgdrain = value; self.mark_param_given(38); Ok(()) }
            "as" => { validate_parameter_as_(value)?; self.params.as_ = value; self.mark_param_given(39); Ok(()) }
            "ps" => { validate_parameter_ps(value)?; self.params.ps = value; self.mark_param_given(40); Ok(()) }
            "ad" => { validate_parameter_ad(value)?; self.params.ad = value; self.mark_param_given(41); Ok(()) }
            "pd" => { validate_parameter_pd(value)?; self.params.pd = value; self.mark_param_given(42); Ok(()) }
            "mult" => { validate_parameter_mult(value)?; self.params.mult = value; self.mark_param_given(43); Ok(()) }
            "vfb" => { validate_parameter_vfb(value)?; self.params.vfb = value; self.mark_param_given(44); Ok(()) }
            "stvfb" => { validate_parameter_stvfb(value)?; self.params.stvfb = value; self.mark_param_given(45); Ok(()) }
            "st2vfb" => { validate_parameter_st2vfb(value)?; self.params.st2vfb = value; self.mark_param_given(46); Ok(()) }
            "tox" => { validate_parameter_tox(value)?; self.params.tox = value; self.mark_param_given(47); Ok(()) }
            "epsrox" => { validate_parameter_epsrox(value)?; self.params.epsrox = value; self.mark_param_given(48); Ok(()) }
            "neff" => { validate_parameter_neff(value)?; self.params.neff = value; self.mark_param_given(49); Ok(()) }
            "facneffac" => { validate_parameter_facneffac(value)?; self.params.facneffac = value; self.mark_param_given(50); Ok(()) }
            "gfacnud" => { validate_parameter_gfacnud(value)?; self.params.gfacnud = value; self.mark_param_given(51); Ok(()) }
            "vsbnud" => { validate_parameter_vsbnud(value)?; self.params.vsbnud = value; self.mark_param_given(52); Ok(()) }
            "dvsbnud" => { validate_parameter_dvsbnud(value)?; self.params.dvsbnud = value; self.mark_param_given(53); Ok(()) }
            "vnsub" => { validate_parameter_vnsub(value)?; self.params.vnsub = value; self.mark_param_given(54); Ok(()) }
            "nslp" => { validate_parameter_nslp(value)?; self.params.nslp = value; self.mark_param_given(55); Ok(()) }
            "dnsub" => { validate_parameter_dnsub(value)?; self.params.dnsub = value; self.mark_param_given(56); Ok(()) }
            "dphib" => { validate_parameter_dphib(value)?; self.params.dphib = value; self.mark_param_given(57); Ok(()) }
            "delvtac" => { validate_parameter_delvtac(value)?; self.params.delvtac = value; self.mark_param_given(58); Ok(()) }
            "np" => { validate_parameter_np(value)?; self.params.np = value; self.mark_param_given(59); Ok(()) }
            "toxov" => { validate_parameter_toxov(value)?; self.params.toxov = value; self.mark_param_given(60); Ok(()) }
            "toxovd" => { validate_parameter_toxovd(value)?; self.params.toxovd = value; self.mark_param_given(61); Ok(()) }
            "nov" => { validate_parameter_nov(value)?; self.params.nov = value; self.mark_param_given(62); Ok(()) }
            "novd" => { validate_parameter_novd(value)?; self.params.novd = value; self.mark_param_given(63); Ok(()) }
            "ct" => { validate_parameter_ct(value)?; self.params.ct = value; self.mark_param_given(64); Ok(()) }
            "ctg" => { validate_parameter_ctg(value)?; self.params.ctg = value; self.mark_param_given(65); Ok(()) }
            "ctb" => { validate_parameter_ctb(value)?; self.params.ctb = value; self.mark_param_given(66); Ok(()) }
            "stct" => { validate_parameter_stct(value)?; self.params.stct = value; self.mark_param_given(67); Ok(()) }
            "cf" => { validate_parameter_cf(value)?; self.params.cf = value; self.mark_param_given(68); Ok(()) }
            "cfd" => { validate_parameter_cfd(value)?; self.params.cfd = value; self.mark_param_given(69); Ok(()) }
            "cfb" => { validate_parameter_cfb(value)?; self.params.cfb = value; self.mark_param_given(70); Ok(()) }
            "psce" => { validate_parameter_psce(value)?; self.params.psce = value; self.mark_param_given(71); Ok(()) }
            "psceb" => { validate_parameter_psceb(value)?; self.params.psceb = value; self.mark_param_given(72); Ok(()) }
            "psced" => { validate_parameter_psced(value)?; self.params.psced = value; self.mark_param_given(73); Ok(()) }
            "betn" => { validate_parameter_betn(value)?; self.params.betn = value; self.mark_param_given(74); Ok(()) }
            "stbet" => { validate_parameter_stbet(value)?; self.params.stbet = value; self.mark_param_given(75); Ok(()) }
            "mue" => { validate_parameter_mue(value)?; self.params.mue = value; self.mark_param_given(76); Ok(()) }
            "stmue" => { validate_parameter_stmue(value)?; self.params.stmue = value; self.mark_param_given(77); Ok(()) }
            "themu" => { validate_parameter_themu(value)?; self.params.themu = value; self.mark_param_given(78); Ok(()) }
            "stthemu" => { validate_parameter_stthemu(value)?; self.params.stthemu = value; self.mark_param_given(79); Ok(()) }
            "cs" => { validate_parameter_cs(value)?; self.params.cs = value; self.mark_param_given(80); Ok(()) }
            "stcs" => { validate_parameter_stcs(value)?; self.params.stcs = value; self.mark_param_given(81); Ok(()) }
            "thecs" => { validate_parameter_thecs(value)?; self.params.thecs = value; self.mark_param_given(82); Ok(()) }
            "stthecs" => { validate_parameter_stthecs(value)?; self.params.stthecs = value; self.mark_param_given(83); Ok(()) }
            "xcor" => { validate_parameter_xcor(value)?; self.params.xcor = value; self.mark_param_given(84); Ok(()) }
            "stxcor" => { validate_parameter_stxcor(value)?; self.params.stxcor = value; self.mark_param_given(85); Ok(()) }
            "feta" => { validate_parameter_feta(value)?; self.params.feta = value; self.mark_param_given(86); Ok(()) }
            "rs" => { validate_parameter_rs(value)?; self.params.rs = value; self.mark_param_given(87); Ok(()) }
            "strs" => { validate_parameter_strs(value)?; self.params.strs = value; self.mark_param_given(88); Ok(()) }
            "rsb" => { validate_parameter_rsb(value)?; self.params.rsb = value; self.mark_param_given(89); Ok(()) }
            "rsg" => { validate_parameter_rsg(value)?; self.params.rsg = value; self.mark_param_given(90); Ok(()) }
            "thesat" => { validate_parameter_thesat(value)?; self.params.thesat = value; self.mark_param_given(91); Ok(()) }
            "stthesat" => { validate_parameter_stthesat(value)?; self.params.stthesat = value; self.mark_param_given(92); Ok(()) }
            "thesatb" => { validate_parameter_thesatb(value)?; self.params.thesatb = value; self.mark_param_given(93); Ok(()) }
            "thesatg" => { validate_parameter_thesatg(value)?; self.params.thesatg = value; self.mark_param_given(94); Ok(()) }
            "ax" => { validate_parameter_ax(value)?; self.params.ax = value; self.mark_param_given(95); Ok(()) }
            "alp" => { validate_parameter_alp(value)?; self.params.alp = value; self.mark_param_given(96); Ok(()) }
            "alp1" => { validate_parameter_alp1(value)?; self.params.alp1 = value; self.mark_param_given(97); Ok(()) }
            "alp2" => { validate_parameter_alp2(value)?; self.params.alp2 = value; self.mark_param_given(98); Ok(()) }
            "vp" => { validate_parameter_vp(value)?; self.params.vp = value; self.mark_param_given(99); Ok(()) }
            "a1" => { validate_parameter_a1(value)?; self.params.a1 = value; self.mark_param_given(100); Ok(()) }
            "a2" => { validate_parameter_a2(value)?; self.params.a2 = value; self.mark_param_given(101); Ok(()) }
            "sta2" => { validate_parameter_sta2(value)?; self.params.sta2 = value; self.mark_param_given(102); Ok(()) }
            "a3" => { validate_parameter_a3(value)?; self.params.a3 = value; self.mark_param_given(103); Ok(()) }
            "a4" => { validate_parameter_a4(value)?; self.params.a4 = value; self.mark_param_given(104); Ok(()) }
            "gco" => { validate_parameter_gco(value)?; self.params.gco = value; self.mark_param_given(105); Ok(()) }
            "iginv" => { validate_parameter_iginv(value)?; self.params.iginv = value; self.mark_param_given(106); Ok(()) }
            "igov" => { validate_parameter_igov(value)?; self.params.igov = value; self.mark_param_given(107); Ok(()) }
            "igovd" => { validate_parameter_igovd(value)?; self.params.igovd = value; self.mark_param_given(108); Ok(()) }
            "stig" => { validate_parameter_stig(value)?; self.params.stig = value; self.mark_param_given(109); Ok(()) }
            "gc2" => { validate_parameter_gc2(value)?; self.params.gc2 = value; self.mark_param_given(110); Ok(()) }
            "gc3" => { validate_parameter_gc3(value)?; self.params.gc3 = value; self.mark_param_given(111); Ok(()) }
            "chib" => { validate_parameter_chib(value)?; self.params.chib = value; self.mark_param_given(112); Ok(()) }
            "agidl" => { validate_parameter_agidl(value)?; self.params.agidl = value; self.mark_param_given(113); Ok(()) }
            "agidld" => { validate_parameter_agidld(value)?; self.params.agidld = value; self.mark_param_given(114); Ok(()) }
            "bgidl" => { validate_parameter_bgidl(value)?; self.params.bgidl = value; self.mark_param_given(115); Ok(()) }
            "bgidld" => { validate_parameter_bgidld(value)?; self.params.bgidld = value; self.mark_param_given(116); Ok(()) }
            "stbgidl" => { validate_parameter_stbgidl(value)?; self.params.stbgidl = value; self.mark_param_given(117); Ok(()) }
            "stbgidld" => { validate_parameter_stbgidld(value)?; self.params.stbgidld = value; self.mark_param_given(118); Ok(()) }
            "cgidl" => { validate_parameter_cgidl(value)?; self.params.cgidl = value; self.mark_param_given(119); Ok(()) }
            "cgidld" => { validate_parameter_cgidld(value)?; self.params.cgidld = value; self.mark_param_given(120); Ok(()) }
            "cox" => { validate_parameter_cox(value)?; self.params.cox = value; self.mark_param_given(121); Ok(()) }
            "cgov" => { validate_parameter_cgov(value)?; self.params.cgov = value; self.mark_param_given(122); Ok(()) }
            "cgovd" => { validate_parameter_cgovd(value)?; self.params.cgovd = value; self.mark_param_given(123); Ok(()) }
            "cgbov" => { validate_parameter_cgbov(value)?; self.params.cgbov = value; self.mark_param_given(124); Ok(()) }
            "cfr" => { validate_parameter_cfr(value)?; self.params.cfr = value; self.mark_param_given(125); Ok(()) }
            "cfrd" => { validate_parameter_cfrd(value)?; self.params.cfrd = value; self.mark_param_given(126); Ok(()) }
            "fnt" => { validate_parameter_fnt(value)?; self.params.fnt = value; self.mark_param_given(127); Ok(()) }
            "fntexc" => { validate_parameter_fntexc(value)?; self.params.fntexc = value; self.mark_param_given(128); Ok(()) }
            "nfa" => { validate_parameter_nfa(value)?; self.params.nfa = value; self.mark_param_given(129); Ok(()) }
            "nfb" => { validate_parameter_nfb(value)?; self.params.nfb = value; self.mark_param_given(130); Ok(()) }
            "nfc" => { validate_parameter_nfc(value)?; self.params.nfc = value; self.mark_param_given(131); Ok(()) }
            "ef" => { validate_parameter_ef(value)?; self.params.ef = value; self.mark_param_given(132); Ok(()) }
            "vfbedge" => { validate_parameter_vfbedge(value)?; self.params.vfbedge = value; self.mark_param_given(133); Ok(()) }
            "stvfbedge" => { validate_parameter_stvfbedge(value)?; self.params.stvfbedge = value; self.mark_param_given(134); Ok(()) }
            "dphibedge" => { validate_parameter_dphibedge(value)?; self.params.dphibedge = value; self.mark_param_given(135); Ok(()) }
            "neffedge" => { validate_parameter_neffedge(value)?; self.params.neffedge = value; self.mark_param_given(136); Ok(()) }
            "ctedge" => { validate_parameter_ctedge(value)?; self.params.ctedge = value; self.mark_param_given(137); Ok(()) }
            "betnedge" => { validate_parameter_betnedge(value)?; self.params.betnedge = value; self.mark_param_given(138); Ok(()) }
            "stbetedge" => { validate_parameter_stbetedge(value)?; self.params.stbetedge = value; self.mark_param_given(139); Ok(()) }
            "psceedge" => { validate_parameter_psceedge(value)?; self.params.psceedge = value; self.mark_param_given(140); Ok(()) }
            "pscebedge" => { validate_parameter_pscebedge(value)?; self.params.pscebedge = value; self.mark_param_given(141); Ok(()) }
            "pscededge" => { validate_parameter_pscededge(value)?; self.params.pscededge = value; self.mark_param_given(142); Ok(()) }
            "cfedge" => { validate_parameter_cfedge(value)?; self.params.cfedge = value; self.mark_param_given(143); Ok(()) }
            "cfdedge" => { validate_parameter_cfdedge(value)?; self.params.cfdedge = value; self.mark_param_given(144); Ok(()) }
            "cfbedge" => { validate_parameter_cfbedge(value)?; self.params.cfbedge = value; self.mark_param_given(145); Ok(()) }
            "fntedge" => { validate_parameter_fntedge(value)?; self.params.fntedge = value; self.mark_param_given(146); Ok(()) }
            "nfaedge" => { validate_parameter_nfaedge(value)?; self.params.nfaedge = value; self.mark_param_given(147); Ok(()) }
            "nfbedge" => { validate_parameter_nfbedge(value)?; self.params.nfbedge = value; self.mark_param_given(148); Ok(()) }
            "nfcedge" => { validate_parameter_nfcedge(value)?; self.params.nfcedge = value; self.mark_param_given(149); Ok(()) }
            "efedge" => { validate_parameter_efedge(value)?; self.params.efedge = value; self.mark_param_given(150); Ok(()) }
            "rg" => { validate_parameter_rg(value)?; self.params.rg = value; self.mark_param_given(151); Ok(()) }
            "rse" => { validate_parameter_rse(value)?; self.params.rse = value; self.mark_param_given(152); Ok(()) }
            "rde" => { validate_parameter_rde(value)?; self.params.rde = value; self.mark_param_given(153); Ok(()) }
            "rbulk" => { validate_parameter_rbulk(value)?; self.params.rbulk = value; self.mark_param_given(154); Ok(()) }
            "rwell" => { validate_parameter_rwell(value)?; self.params.rwell = value; self.mark_param_given(155); Ok(()) }
            "rjuns" => { validate_parameter_rjuns(value)?; self.params.rjuns = value; self.mark_param_given(156); Ok(()) }
            "rjund" => { validate_parameter_rjund(value)?; self.params.rjund = value; self.mark_param_given(157); Ok(()) }
            "povfb" => { validate_parameter_povfb(value)?; self.params.povfb = value; self.mark_param_given(158); Ok(()) }
            "plvfb" => { validate_parameter_plvfb(value)?; self.params.plvfb = value; self.mark_param_given(159); Ok(()) }
            "pwvfb" => { validate_parameter_pwvfb(value)?; self.params.pwvfb = value; self.mark_param_given(160); Ok(()) }
            "plwvfb" => { validate_parameter_plwvfb(value)?; self.params.plwvfb = value; self.mark_param_given(161); Ok(()) }
            "postvfb" => { validate_parameter_postvfb(value)?; self.params.postvfb = value; self.mark_param_given(162); Ok(()) }
            "plstvfb" => { validate_parameter_plstvfb(value)?; self.params.plstvfb = value; self.mark_param_given(163); Ok(()) }
            "pwstvfb" => { validate_parameter_pwstvfb(value)?; self.params.pwstvfb = value; self.mark_param_given(164); Ok(()) }
            "plwstvfb" => { validate_parameter_plwstvfb(value)?; self.params.plwstvfb = value; self.mark_param_given(165); Ok(()) }
            "post2vfb" => { validate_parameter_post2vfb(value)?; self.params.post2vfb = value; self.mark_param_given(166); Ok(()) }
            "potox" => { validate_parameter_potox(value)?; self.params.potox = value; self.mark_param_given(167); Ok(()) }
            "poepsrox" => { validate_parameter_poepsrox(value)?; self.params.poepsrox = value; self.mark_param_given(168); Ok(()) }
            "poneff" => { validate_parameter_poneff(value)?; self.params.poneff = value; self.mark_param_given(169); Ok(()) }
            "plneff" => { validate_parameter_plneff(value)?; self.params.plneff = value; self.mark_param_given(170); Ok(()) }
            "pwneff" => { validate_parameter_pwneff(value)?; self.params.pwneff = value; self.mark_param_given(171); Ok(()) }
            "plwneff" => { validate_parameter_plwneff(value)?; self.params.plwneff = value; self.mark_param_given(172); Ok(()) }
            "pofacneffac" => { validate_parameter_pofacneffac(value)?; self.params.pofacneffac = value; self.mark_param_given(173); Ok(()) }
            "plfacneffac" => { validate_parameter_plfacneffac(value)?; self.params.plfacneffac = value; self.mark_param_given(174); Ok(()) }
            "pwfacneffac" => { validate_parameter_pwfacneffac(value)?; self.params.pwfacneffac = value; self.mark_param_given(175); Ok(()) }
            "plwfacneffac" => { validate_parameter_plwfacneffac(value)?; self.params.plwfacneffac = value; self.mark_param_given(176); Ok(()) }
            "pogfacnud" => { validate_parameter_pogfacnud(value)?; self.params.pogfacnud = value; self.mark_param_given(177); Ok(()) }
            "plgfacnud" => { validate_parameter_plgfacnud(value)?; self.params.plgfacnud = value; self.mark_param_given(178); Ok(()) }
            "pwgfacnud" => { validate_parameter_pwgfacnud(value)?; self.params.pwgfacnud = value; self.mark_param_given(179); Ok(()) }
            "plwgfacnud" => { validate_parameter_plwgfacnud(value)?; self.params.plwgfacnud = value; self.mark_param_given(180); Ok(()) }
            "povsbnud" => { validate_parameter_povsbnud(value)?; self.params.povsbnud = value; self.mark_param_given(181); Ok(()) }
            "podvsbnud" => { validate_parameter_podvsbnud(value)?; self.params.podvsbnud = value; self.mark_param_given(182); Ok(()) }
            "povnsub" => { validate_parameter_povnsub(value)?; self.params.povnsub = value; self.mark_param_given(183); Ok(()) }
            "ponslp" => { validate_parameter_ponslp(value)?; self.params.ponslp = value; self.mark_param_given(184); Ok(()) }
            "podnsub" => { validate_parameter_podnsub(value)?; self.params.podnsub = value; self.mark_param_given(185); Ok(()) }
            "podphib" => { validate_parameter_podphib(value)?; self.params.podphib = value; self.mark_param_given(186); Ok(()) }
            "pldphib" => { validate_parameter_pldphib(value)?; self.params.pldphib = value; self.mark_param_given(187); Ok(()) }
            "pwdphib" => { validate_parameter_pwdphib(value)?; self.params.pwdphib = value; self.mark_param_given(188); Ok(()) }
            "plwdphib" => { validate_parameter_plwdphib(value)?; self.params.plwdphib = value; self.mark_param_given(189); Ok(()) }
            "podelvtac" => { validate_parameter_podelvtac(value)?; self.params.podelvtac = value; self.mark_param_given(190); Ok(()) }
            "pldelvtac" => { validate_parameter_pldelvtac(value)?; self.params.pldelvtac = value; self.mark_param_given(191); Ok(()) }
            "pwdelvtac" => { validate_parameter_pwdelvtac(value)?; self.params.pwdelvtac = value; self.mark_param_given(192); Ok(()) }
            "plwdelvtac" => { validate_parameter_plwdelvtac(value)?; self.params.plwdelvtac = value; self.mark_param_given(193); Ok(()) }
            "ponp" => { validate_parameter_ponp(value)?; self.params.ponp = value; self.mark_param_given(194); Ok(()) }
            "plnp" => { validate_parameter_plnp(value)?; self.params.plnp = value; self.mark_param_given(195); Ok(()) }
            "pwnp" => { validate_parameter_pwnp(value)?; self.params.pwnp = value; self.mark_param_given(196); Ok(()) }
            "plwnp" => { validate_parameter_plwnp(value)?; self.params.plwnp = value; self.mark_param_given(197); Ok(()) }
            "potoxov" => { validate_parameter_potoxov(value)?; self.params.potoxov = value; self.mark_param_given(198); Ok(()) }
            "potoxovd" => { validate_parameter_potoxovd(value)?; self.params.potoxovd = value; self.mark_param_given(199); Ok(()) }
            "ponov" => { validate_parameter_ponov(value)?; self.params.ponov = value; self.mark_param_given(200); Ok(()) }
            "plnov" => { validate_parameter_plnov(value)?; self.params.plnov = value; self.mark_param_given(201); Ok(()) }
            "pwnov" => { validate_parameter_pwnov(value)?; self.params.pwnov = value; self.mark_param_given(202); Ok(()) }
            "plwnov" => { validate_parameter_plwnov(value)?; self.params.plwnov = value; self.mark_param_given(203); Ok(()) }
            "ponovd" => { validate_parameter_ponovd(value)?; self.params.ponovd = value; self.mark_param_given(204); Ok(()) }
            "plnovd" => { validate_parameter_plnovd(value)?; self.params.plnovd = value; self.mark_param_given(205); Ok(()) }
            "pwnovd" => { validate_parameter_pwnovd(value)?; self.params.pwnovd = value; self.mark_param_given(206); Ok(()) }
            "plwnovd" => { validate_parameter_plwnovd(value)?; self.params.plwnovd = value; self.mark_param_given(207); Ok(()) }
            "poct" => { validate_parameter_poct(value)?; self.params.poct = value; self.mark_param_given(208); Ok(()) }
            "plct" => { validate_parameter_plct(value)?; self.params.plct = value; self.mark_param_given(209); Ok(()) }
            "pwct" => { validate_parameter_pwct(value)?; self.params.pwct = value; self.mark_param_given(210); Ok(()) }
            "plwct" => { validate_parameter_plwct(value)?; self.params.plwct = value; self.mark_param_given(211); Ok(()) }
            "poctg" => { validate_parameter_poctg(value)?; self.params.poctg = value; self.mark_param_given(212); Ok(()) }
            "poctb" => { validate_parameter_poctb(value)?; self.params.poctb = value; self.mark_param_given(213); Ok(()) }
            "postct" => { validate_parameter_postct(value)?; self.params.postct = value; self.mark_param_given(214); Ok(()) }
            "pocf" => { validate_parameter_pocf(value)?; self.params.pocf = value; self.mark_param_given(215); Ok(()) }
            "plcf" => { validate_parameter_plcf(value)?; self.params.plcf = value; self.mark_param_given(216); Ok(()) }
            "pwcf" => { validate_parameter_pwcf(value)?; self.params.pwcf = value; self.mark_param_given(217); Ok(()) }
            "plwcf" => { validate_parameter_plwcf(value)?; self.params.plwcf = value; self.mark_param_given(218); Ok(()) }
            "pocfd" => { validate_parameter_pocfd(value)?; self.params.pocfd = value; self.mark_param_given(219); Ok(()) }
            "pocfb" => { validate_parameter_pocfb(value)?; self.params.pocfb = value; self.mark_param_given(220); Ok(()) }
            "popsce" => { validate_parameter_popsce(value)?; self.params.popsce = value; self.mark_param_given(221); Ok(()) }
            "plpsce" => { validate_parameter_plpsce(value)?; self.params.plpsce = value; self.mark_param_given(222); Ok(()) }
            "pwpsce" => { validate_parameter_pwpsce(value)?; self.params.pwpsce = value; self.mark_param_given(223); Ok(()) }
            "plwpsce" => { validate_parameter_plwpsce(value)?; self.params.plwpsce = value; self.mark_param_given(224); Ok(()) }
            "popsceb" => { validate_parameter_popsceb(value)?; self.params.popsceb = value; self.mark_param_given(225); Ok(()) }
            "popsced" => { validate_parameter_popsced(value)?; self.params.popsced = value; self.mark_param_given(226); Ok(()) }
            "pobetn" => { validate_parameter_pobetn(value)?; self.params.pobetn = value; self.mark_param_given(227); Ok(()) }
            "plbetn" => { validate_parameter_plbetn(value)?; self.params.plbetn = value; self.mark_param_given(228); Ok(()) }
            "pwbetn" => { validate_parameter_pwbetn(value)?; self.params.pwbetn = value; self.mark_param_given(229); Ok(()) }
            "plwbetn" => { validate_parameter_plwbetn(value)?; self.params.plwbetn = value; self.mark_param_given(230); Ok(()) }
            "postbet" => { validate_parameter_postbet(value)?; self.params.postbet = value; self.mark_param_given(231); Ok(()) }
            "plstbet" => { validate_parameter_plstbet(value)?; self.params.plstbet = value; self.mark_param_given(232); Ok(()) }
            "pwstbet" => { validate_parameter_pwstbet(value)?; self.params.pwstbet = value; self.mark_param_given(233); Ok(()) }
            "plwstbet" => { validate_parameter_plwstbet(value)?; self.params.plwstbet = value; self.mark_param_given(234); Ok(()) }
            "pomue" => { validate_parameter_pomue(value)?; self.params.pomue = value; self.mark_param_given(235); Ok(()) }
            "plmue" => { validate_parameter_plmue(value)?; self.params.plmue = value; self.mark_param_given(236); Ok(()) }
            "pwmue" => { validate_parameter_pwmue(value)?; self.params.pwmue = value; self.mark_param_given(237); Ok(()) }
            "plwmue" => { validate_parameter_plwmue(value)?; self.params.plwmue = value; self.mark_param_given(238); Ok(()) }
            "postmue" => { validate_parameter_postmue(value)?; self.params.postmue = value; self.mark_param_given(239); Ok(()) }
            "pothemu" => { validate_parameter_pothemu(value)?; self.params.pothemu = value; self.mark_param_given(240); Ok(()) }
            "postthemu" => { validate_parameter_postthemu(value)?; self.params.postthemu = value; self.mark_param_given(241); Ok(()) }
            "pocs" => { validate_parameter_pocs(value)?; self.params.pocs = value; self.mark_param_given(242); Ok(()) }
            "plcs" => { validate_parameter_plcs(value)?; self.params.plcs = value; self.mark_param_given(243); Ok(()) }
            "pwcs" => { validate_parameter_pwcs(value)?; self.params.pwcs = value; self.mark_param_given(244); Ok(()) }
            "plwcs" => { validate_parameter_plwcs(value)?; self.params.plwcs = value; self.mark_param_given(245); Ok(()) }
            "postcs" => { validate_parameter_postcs(value)?; self.params.postcs = value; self.mark_param_given(246); Ok(()) }
            "pothecs" => { validate_parameter_pothecs(value)?; self.params.pothecs = value; self.mark_param_given(247); Ok(()) }
            "postthecs" => { validate_parameter_postthecs(value)?; self.params.postthecs = value; self.mark_param_given(248); Ok(()) }
            "poxcor" => { validate_parameter_poxcor(value)?; self.params.poxcor = value; self.mark_param_given(249); Ok(()) }
            "plxcor" => { validate_parameter_plxcor(value)?; self.params.plxcor = value; self.mark_param_given(250); Ok(()) }
            "pwxcor" => { validate_parameter_pwxcor(value)?; self.params.pwxcor = value; self.mark_param_given(251); Ok(()) }
            "plwxcor" => { validate_parameter_plwxcor(value)?; self.params.plwxcor = value; self.mark_param_given(252); Ok(()) }
            "postxcor" => { validate_parameter_postxcor(value)?; self.params.postxcor = value; self.mark_param_given(253); Ok(()) }
            "pofeta" => { validate_parameter_pofeta(value)?; self.params.pofeta = value; self.mark_param_given(254); Ok(()) }
            "pors" => { validate_parameter_pors(value)?; self.params.pors = value; self.mark_param_given(255); Ok(()) }
            "plrs" => { validate_parameter_plrs(value)?; self.params.plrs = value; self.mark_param_given(256); Ok(()) }
            "pwrs" => { validate_parameter_pwrs(value)?; self.params.pwrs = value; self.mark_param_given(257); Ok(()) }
            "plwrs" => { validate_parameter_plwrs(value)?; self.params.plwrs = value; self.mark_param_given(258); Ok(()) }
            "postrs" => { validate_parameter_postrs(value)?; self.params.postrs = value; self.mark_param_given(259); Ok(()) }
            "porsb" => { validate_parameter_porsb(value)?; self.params.porsb = value; self.mark_param_given(260); Ok(()) }
            "porsg" => { validate_parameter_porsg(value)?; self.params.porsg = value; self.mark_param_given(261); Ok(()) }
            "pothesat" => { validate_parameter_pothesat(value)?; self.params.pothesat = value; self.mark_param_given(262); Ok(()) }
            "plthesat" => { validate_parameter_plthesat(value)?; self.params.plthesat = value; self.mark_param_given(263); Ok(()) }
            "pwthesat" => { validate_parameter_pwthesat(value)?; self.params.pwthesat = value; self.mark_param_given(264); Ok(()) }
            "plwthesat" => { validate_parameter_plwthesat(value)?; self.params.plwthesat = value; self.mark_param_given(265); Ok(()) }
            "postthesat" => { validate_parameter_postthesat(value)?; self.params.postthesat = value; self.mark_param_given(266); Ok(()) }
            "plstthesat" => { validate_parameter_plstthesat(value)?; self.params.plstthesat = value; self.mark_param_given(267); Ok(()) }
            "pwstthesat" => { validate_parameter_pwstthesat(value)?; self.params.pwstthesat = value; self.mark_param_given(268); Ok(()) }
            "plwstthesat" => { validate_parameter_plwstthesat(value)?; self.params.plwstthesat = value; self.mark_param_given(269); Ok(()) }
            "pothesatb" => { validate_parameter_pothesatb(value)?; self.params.pothesatb = value; self.mark_param_given(270); Ok(()) }
            "plthesatb" => { validate_parameter_plthesatb(value)?; self.params.plthesatb = value; self.mark_param_given(271); Ok(()) }
            "pwthesatb" => { validate_parameter_pwthesatb(value)?; self.params.pwthesatb = value; self.mark_param_given(272); Ok(()) }
            "plwthesatb" => { validate_parameter_plwthesatb(value)?; self.params.plwthesatb = value; self.mark_param_given(273); Ok(()) }
            "pothesatg" => { validate_parameter_pothesatg(value)?; self.params.pothesatg = value; self.mark_param_given(274); Ok(()) }
            "plthesatg" => { validate_parameter_plthesatg(value)?; self.params.plthesatg = value; self.mark_param_given(275); Ok(()) }
            "pwthesatg" => { validate_parameter_pwthesatg(value)?; self.params.pwthesatg = value; self.mark_param_given(276); Ok(()) }
            "plwthesatg" => { validate_parameter_plwthesatg(value)?; self.params.plwthesatg = value; self.mark_param_given(277); Ok(()) }
            "poax" => { validate_parameter_poax(value)?; self.params.poax = value; self.mark_param_given(278); Ok(()) }
            "plax" => { validate_parameter_plax(value)?; self.params.plax = value; self.mark_param_given(279); Ok(()) }
            "pwax" => { validate_parameter_pwax(value)?; self.params.pwax = value; self.mark_param_given(280); Ok(()) }
            "plwax" => { validate_parameter_plwax(value)?; self.params.plwax = value; self.mark_param_given(281); Ok(()) }
            "poalp" => { validate_parameter_poalp(value)?; self.params.poalp = value; self.mark_param_given(282); Ok(()) }
            "plalp" => { validate_parameter_plalp(value)?; self.params.plalp = value; self.mark_param_given(283); Ok(()) }
            "pwalp" => { validate_parameter_pwalp(value)?; self.params.pwalp = value; self.mark_param_given(284); Ok(()) }
            "plwalp" => { validate_parameter_plwalp(value)?; self.params.plwalp = value; self.mark_param_given(285); Ok(()) }
            "poalp1" => { validate_parameter_poalp1(value)?; self.params.poalp1 = value; self.mark_param_given(286); Ok(()) }
            "plalp1" => { validate_parameter_plalp1(value)?; self.params.plalp1 = value; self.mark_param_given(287); Ok(()) }
            "pwalp1" => { validate_parameter_pwalp1(value)?; self.params.pwalp1 = value; self.mark_param_given(288); Ok(()) }
            "plwalp1" => { validate_parameter_plwalp1(value)?; self.params.plwalp1 = value; self.mark_param_given(289); Ok(()) }
            "poalp2" => { validate_parameter_poalp2(value)?; self.params.poalp2 = value; self.mark_param_given(290); Ok(()) }
            "plalp2" => { validate_parameter_plalp2(value)?; self.params.plalp2 = value; self.mark_param_given(291); Ok(()) }
            "pwalp2" => { validate_parameter_pwalp2(value)?; self.params.pwalp2 = value; self.mark_param_given(292); Ok(()) }
            "plwalp2" => { validate_parameter_plwalp2(value)?; self.params.plwalp2 = value; self.mark_param_given(293); Ok(()) }
            "povp" => { validate_parameter_povp(value)?; self.params.povp = value; self.mark_param_given(294); Ok(()) }
            "poa1" => { validate_parameter_poa1(value)?; self.params.poa1 = value; self.mark_param_given(295); Ok(()) }
            "pla1" => { validate_parameter_pla1(value)?; self.params.pla1 = value; self.mark_param_given(296); Ok(()) }
            "pwa1" => { validate_parameter_pwa1(value)?; self.params.pwa1 = value; self.mark_param_given(297); Ok(()) }
            "plwa1" => { validate_parameter_plwa1(value)?; self.params.plwa1 = value; self.mark_param_given(298); Ok(()) }
            "poa2" => { validate_parameter_poa2(value)?; self.params.poa2 = value; self.mark_param_given(299); Ok(()) }
            "posta2" => { validate_parameter_posta2(value)?; self.params.posta2 = value; self.mark_param_given(300); Ok(()) }
            "poa3" => { validate_parameter_poa3(value)?; self.params.poa3 = value; self.mark_param_given(301); Ok(()) }
            "pla3" => { validate_parameter_pla3(value)?; self.params.pla3 = value; self.mark_param_given(302); Ok(()) }
            "pwa3" => { validate_parameter_pwa3(value)?; self.params.pwa3 = value; self.mark_param_given(303); Ok(()) }
            "plwa3" => { validate_parameter_plwa3(value)?; self.params.plwa3 = value; self.mark_param_given(304); Ok(()) }
            "poa4" => { validate_parameter_poa4(value)?; self.params.poa4 = value; self.mark_param_given(305); Ok(()) }
            "pla4" => { validate_parameter_pla4(value)?; self.params.pla4 = value; self.mark_param_given(306); Ok(()) }
            "pwa4" => { validate_parameter_pwa4(value)?; self.params.pwa4 = value; self.mark_param_given(307); Ok(()) }
            "plwa4" => { validate_parameter_plwa4(value)?; self.params.plwa4 = value; self.mark_param_given(308); Ok(()) }
            "pogco" => { validate_parameter_pogco(value)?; self.params.pogco = value; self.mark_param_given(309); Ok(()) }
            "poiginv" => { validate_parameter_poiginv(value)?; self.params.poiginv = value; self.mark_param_given(310); Ok(()) }
            "pliginv" => { validate_parameter_pliginv(value)?; self.params.pliginv = value; self.mark_param_given(311); Ok(()) }
            "pwiginv" => { validate_parameter_pwiginv(value)?; self.params.pwiginv = value; self.mark_param_given(312); Ok(()) }
            "plwiginv" => { validate_parameter_plwiginv(value)?; self.params.plwiginv = value; self.mark_param_given(313); Ok(()) }
            "poigov" => { validate_parameter_poigov(value)?; self.params.poigov = value; self.mark_param_given(314); Ok(()) }
            "pligov" => { validate_parameter_pligov(value)?; self.params.pligov = value; self.mark_param_given(315); Ok(()) }
            "pwigov" => { validate_parameter_pwigov(value)?; self.params.pwigov = value; self.mark_param_given(316); Ok(()) }
            "plwigov" => { validate_parameter_plwigov(value)?; self.params.plwigov = value; self.mark_param_given(317); Ok(()) }
            "poigovd" => { validate_parameter_poigovd(value)?; self.params.poigovd = value; self.mark_param_given(318); Ok(()) }
            "pligovd" => { validate_parameter_pligovd(value)?; self.params.pligovd = value; self.mark_param_given(319); Ok(()) }
            "pwigovd" => { validate_parameter_pwigovd(value)?; self.params.pwigovd = value; self.mark_param_given(320); Ok(()) }
            "plwigovd" => { validate_parameter_plwigovd(value)?; self.params.plwigovd = value; self.mark_param_given(321); Ok(()) }
            "postig" => { validate_parameter_postig(value)?; self.params.postig = value; self.mark_param_given(322); Ok(()) }
            "pogc2" => { validate_parameter_pogc2(value)?; self.params.pogc2 = value; self.mark_param_given(323); Ok(()) }
            "pogc3" => { validate_parameter_pogc3(value)?; self.params.pogc3 = value; self.mark_param_given(324); Ok(()) }
            "pochib" => { validate_parameter_pochib(value)?; self.params.pochib = value; self.mark_param_given(325); Ok(()) }
            "poagidl" => { validate_parameter_poagidl(value)?; self.params.poagidl = value; self.mark_param_given(326); Ok(()) }
            "plagidl" => { validate_parameter_plagidl(value)?; self.params.plagidl = value; self.mark_param_given(327); Ok(()) }
            "pwagidl" => { validate_parameter_pwagidl(value)?; self.params.pwagidl = value; self.mark_param_given(328); Ok(()) }
            "plwagidl" => { validate_parameter_plwagidl(value)?; self.params.plwagidl = value; self.mark_param_given(329); Ok(()) }
            "poagidld" => { validate_parameter_poagidld(value)?; self.params.poagidld = value; self.mark_param_given(330); Ok(()) }
            "plagidld" => { validate_parameter_plagidld(value)?; self.params.plagidld = value; self.mark_param_given(331); Ok(()) }
            "pwagidld" => { validate_parameter_pwagidld(value)?; self.params.pwagidld = value; self.mark_param_given(332); Ok(()) }
            "plwagidld" => { validate_parameter_plwagidld(value)?; self.params.plwagidld = value; self.mark_param_given(333); Ok(()) }
            "pobgidl" => { validate_parameter_pobgidl(value)?; self.params.pobgidl = value; self.mark_param_given(334); Ok(()) }
            "pobgidld" => { validate_parameter_pobgidld(value)?; self.params.pobgidld = value; self.mark_param_given(335); Ok(()) }
            "postbgidl" => { validate_parameter_postbgidl(value)?; self.params.postbgidl = value; self.mark_param_given(336); Ok(()) }
            "postbgidld" => { validate_parameter_postbgidld(value)?; self.params.postbgidld = value; self.mark_param_given(337); Ok(()) }
            "pocgidl" => { validate_parameter_pocgidl(value)?; self.params.pocgidl = value; self.mark_param_given(338); Ok(()) }
            "pocgidld" => { validate_parameter_pocgidld(value)?; self.params.pocgidld = value; self.mark_param_given(339); Ok(()) }
            "pocox" => { validate_parameter_pocox(value)?; self.params.pocox = value; self.mark_param_given(340); Ok(()) }
            "plcox" => { validate_parameter_plcox(value)?; self.params.plcox = value; self.mark_param_given(341); Ok(()) }
            "pwcox" => { validate_parameter_pwcox(value)?; self.params.pwcox = value; self.mark_param_given(342); Ok(()) }
            "plwcox" => { validate_parameter_plwcox(value)?; self.params.plwcox = value; self.mark_param_given(343); Ok(()) }
            "pocgov" => { validate_parameter_pocgov(value)?; self.params.pocgov = value; self.mark_param_given(344); Ok(()) }
            "plcgov" => { validate_parameter_plcgov(value)?; self.params.plcgov = value; self.mark_param_given(345); Ok(()) }
            "pwcgov" => { validate_parameter_pwcgov(value)?; self.params.pwcgov = value; self.mark_param_given(346); Ok(()) }
            "plwcgov" => { validate_parameter_plwcgov(value)?; self.params.plwcgov = value; self.mark_param_given(347); Ok(()) }
            "pocgovd" => { validate_parameter_pocgovd(value)?; self.params.pocgovd = value; self.mark_param_given(348); Ok(()) }
            "plcgovd" => { validate_parameter_plcgovd(value)?; self.params.plcgovd = value; self.mark_param_given(349); Ok(()) }
            "pwcgovd" => { validate_parameter_pwcgovd(value)?; self.params.pwcgovd = value; self.mark_param_given(350); Ok(()) }
            "plwcgovd" => { validate_parameter_plwcgovd(value)?; self.params.plwcgovd = value; self.mark_param_given(351); Ok(()) }
            "pocgbov" => { validate_parameter_pocgbov(value)?; self.params.pocgbov = value; self.mark_param_given(352); Ok(()) }
            "plcgbov" => { validate_parameter_plcgbov(value)?; self.params.plcgbov = value; self.mark_param_given(353); Ok(()) }
            "pwcgbov" => { validate_parameter_pwcgbov(value)?; self.params.pwcgbov = value; self.mark_param_given(354); Ok(()) }
            "plwcgbov" => { validate_parameter_plwcgbov(value)?; self.params.plwcgbov = value; self.mark_param_given(355); Ok(()) }
            "pocfr" => { validate_parameter_pocfr(value)?; self.params.pocfr = value; self.mark_param_given(356); Ok(()) }
            "plcfr" => { validate_parameter_plcfr(value)?; self.params.plcfr = value; self.mark_param_given(357); Ok(()) }
            "pwcfr" => { validate_parameter_pwcfr(value)?; self.params.pwcfr = value; self.mark_param_given(358); Ok(()) }
            "plwcfr" => { validate_parameter_plwcfr(value)?; self.params.plwcfr = value; self.mark_param_given(359); Ok(()) }
            "pocfrd" => { validate_parameter_pocfrd(value)?; self.params.pocfrd = value; self.mark_param_given(360); Ok(()) }
            "plcfrd" => { validate_parameter_plcfrd(value)?; self.params.plcfrd = value; self.mark_param_given(361); Ok(()) }
            "pwcfrd" => { validate_parameter_pwcfrd(value)?; self.params.pwcfrd = value; self.mark_param_given(362); Ok(()) }
            "plwcfrd" => { validate_parameter_plwcfrd(value)?; self.params.plwcfrd = value; self.mark_param_given(363); Ok(()) }
            "pofnt" => { validate_parameter_pofnt(value)?; self.params.pofnt = value; self.mark_param_given(364); Ok(()) }
            "pofntexc" => { validate_parameter_pofntexc(value)?; self.params.pofntexc = value; self.mark_param_given(365); Ok(()) }
            "plfntexc" => { validate_parameter_plfntexc(value)?; self.params.plfntexc = value; self.mark_param_given(366); Ok(()) }
            "pwfntexc" => { validate_parameter_pwfntexc(value)?; self.params.pwfntexc = value; self.mark_param_given(367); Ok(()) }
            "plwfntexc" => { validate_parameter_plwfntexc(value)?; self.params.plwfntexc = value; self.mark_param_given(368); Ok(()) }
            "ponfa" => { validate_parameter_ponfa(value)?; self.params.ponfa = value; self.mark_param_given(369); Ok(()) }
            "plnfa" => { validate_parameter_plnfa(value)?; self.params.plnfa = value; self.mark_param_given(370); Ok(()) }
            "pwnfa" => { validate_parameter_pwnfa(value)?; self.params.pwnfa = value; self.mark_param_given(371); Ok(()) }
            "plwnfa" => { validate_parameter_plwnfa(value)?; self.params.plwnfa = value; self.mark_param_given(372); Ok(()) }
            "ponfb" => { validate_parameter_ponfb(value)?; self.params.ponfb = value; self.mark_param_given(373); Ok(()) }
            "plnfb" => { validate_parameter_plnfb(value)?; self.params.plnfb = value; self.mark_param_given(374); Ok(()) }
            "pwnfb" => { validate_parameter_pwnfb(value)?; self.params.pwnfb = value; self.mark_param_given(375); Ok(()) }
            "plwnfb" => { validate_parameter_plwnfb(value)?; self.params.plwnfb = value; self.mark_param_given(376); Ok(()) }
            "ponfc" => { validate_parameter_ponfc(value)?; self.params.ponfc = value; self.mark_param_given(377); Ok(()) }
            "plnfc" => { validate_parameter_plnfc(value)?; self.params.plnfc = value; self.mark_param_given(378); Ok(()) }
            "pwnfc" => { validate_parameter_pwnfc(value)?; self.params.pwnfc = value; self.mark_param_given(379); Ok(()) }
            "plwnfc" => { validate_parameter_plwnfc(value)?; self.params.plwnfc = value; self.mark_param_given(380); Ok(()) }
            "poef" => { validate_parameter_poef(value)?; self.params.poef = value; self.mark_param_given(381); Ok(()) }
            "povfbedge" => { validate_parameter_povfbedge(value)?; self.params.povfbedge = value; self.mark_param_given(382); Ok(()) }
            "postvfbedge" => { validate_parameter_postvfbedge(value)?; self.params.postvfbedge = value; self.mark_param_given(383); Ok(()) }
            "plstvfbedge" => { validate_parameter_plstvfbedge(value)?; self.params.plstvfbedge = value; self.mark_param_given(384); Ok(()) }
            "pwstvfbedge" => { validate_parameter_pwstvfbedge(value)?; self.params.pwstvfbedge = value; self.mark_param_given(385); Ok(()) }
            "plwstvfbedge" => { validate_parameter_plwstvfbedge(value)?; self.params.plwstvfbedge = value; self.mark_param_given(386); Ok(()) }
            "podphibedge" => { validate_parameter_podphibedge(value)?; self.params.podphibedge = value; self.mark_param_given(387); Ok(()) }
            "pldphibedge" => { validate_parameter_pldphibedge(value)?; self.params.pldphibedge = value; self.mark_param_given(388); Ok(()) }
            "pwdphibedge" => { validate_parameter_pwdphibedge(value)?; self.params.pwdphibedge = value; self.mark_param_given(389); Ok(()) }
            "plwdphibedge" => { validate_parameter_plwdphibedge(value)?; self.params.plwdphibedge = value; self.mark_param_given(390); Ok(()) }
            "poneffedge" => { validate_parameter_poneffedge(value)?; self.params.poneffedge = value; self.mark_param_given(391); Ok(()) }
            "plneffedge" => { validate_parameter_plneffedge(value)?; self.params.plneffedge = value; self.mark_param_given(392); Ok(()) }
            "pwneffedge" => { validate_parameter_pwneffedge(value)?; self.params.pwneffedge = value; self.mark_param_given(393); Ok(()) }
            "plwneffedge" => { validate_parameter_plwneffedge(value)?; self.params.plwneffedge = value; self.mark_param_given(394); Ok(()) }
            "poctedge" => { validate_parameter_poctedge(value)?; self.params.poctedge = value; self.mark_param_given(395); Ok(()) }
            "plctedge" => { validate_parameter_plctedge(value)?; self.params.plctedge = value; self.mark_param_given(396); Ok(()) }
            "pwctedge" => { validate_parameter_pwctedge(value)?; self.params.pwctedge = value; self.mark_param_given(397); Ok(()) }
            "plwctedge" => { validate_parameter_plwctedge(value)?; self.params.plwctedge = value; self.mark_param_given(398); Ok(()) }
            "pobetnedge" => { validate_parameter_pobetnedge(value)?; self.params.pobetnedge = value; self.mark_param_given(399); Ok(()) }
            "plbetnedge" => { validate_parameter_plbetnedge(value)?; self.params.plbetnedge = value; self.mark_param_given(400); Ok(()) }
            "pwbetnedge" => { validate_parameter_pwbetnedge(value)?; self.params.pwbetnedge = value; self.mark_param_given(401); Ok(()) }
            "plwbetnedge" => { validate_parameter_plwbetnedge(value)?; self.params.plwbetnedge = value; self.mark_param_given(402); Ok(()) }
            "postbetedge" => { validate_parameter_postbetedge(value)?; self.params.postbetedge = value; self.mark_param_given(403); Ok(()) }
            "plstbetedge" => { validate_parameter_plstbetedge(value)?; self.params.plstbetedge = value; self.mark_param_given(404); Ok(()) }
            "pwstbetedge" => { validate_parameter_pwstbetedge(value)?; self.params.pwstbetedge = value; self.mark_param_given(405); Ok(()) }
            "plwstbetedge" => { validate_parameter_plwstbetedge(value)?; self.params.plwstbetedge = value; self.mark_param_given(406); Ok(()) }
            "popsceedge" => { validate_parameter_popsceedge(value)?; self.params.popsceedge = value; self.mark_param_given(407); Ok(()) }
            "plpsceedge" => { validate_parameter_plpsceedge(value)?; self.params.plpsceedge = value; self.mark_param_given(408); Ok(()) }
            "pwpsceedge" => { validate_parameter_pwpsceedge(value)?; self.params.pwpsceedge = value; self.mark_param_given(409); Ok(()) }
            "plwpsceedge" => { validate_parameter_plwpsceedge(value)?; self.params.plwpsceedge = value; self.mark_param_given(410); Ok(()) }
            "popscebedge" => { validate_parameter_popscebedge(value)?; self.params.popscebedge = value; self.mark_param_given(411); Ok(()) }
            "popscededge" => { validate_parameter_popscededge(value)?; self.params.popscededge = value; self.mark_param_given(412); Ok(()) }
            "pocfedge" => { validate_parameter_pocfedge(value)?; self.params.pocfedge = value; self.mark_param_given(413); Ok(()) }
            "plcfedge" => { validate_parameter_plcfedge(value)?; self.params.plcfedge = value; self.mark_param_given(414); Ok(()) }
            "pwcfedge" => { validate_parameter_pwcfedge(value)?; self.params.pwcfedge = value; self.mark_param_given(415); Ok(()) }
            "plwcfedge" => { validate_parameter_plwcfedge(value)?; self.params.plwcfedge = value; self.mark_param_given(416); Ok(()) }
            "pocfdedge" => { validate_parameter_pocfdedge(value)?; self.params.pocfdedge = value; self.mark_param_given(417); Ok(()) }
            "pocfbedge" => { validate_parameter_pocfbedge(value)?; self.params.pocfbedge = value; self.mark_param_given(418); Ok(()) }
            "pofntedge" => { validate_parameter_pofntedge(value)?; self.params.pofntedge = value; self.mark_param_given(419); Ok(()) }
            "ponfaedge" => { validate_parameter_ponfaedge(value)?; self.params.ponfaedge = value; self.mark_param_given(420); Ok(()) }
            "plnfaedge" => { validate_parameter_plnfaedge(value)?; self.params.plnfaedge = value; self.mark_param_given(421); Ok(()) }
            "pwnfaedge" => { validate_parameter_pwnfaedge(value)?; self.params.pwnfaedge = value; self.mark_param_given(422); Ok(()) }
            "plwnfaedge" => { validate_parameter_plwnfaedge(value)?; self.params.plwnfaedge = value; self.mark_param_given(423); Ok(()) }
            "ponfbedge" => { validate_parameter_ponfbedge(value)?; self.params.ponfbedge = value; self.mark_param_given(424); Ok(()) }
            "plnfbedge" => { validate_parameter_plnfbedge(value)?; self.params.plnfbedge = value; self.mark_param_given(425); Ok(()) }
            "pwnfbedge" => { validate_parameter_pwnfbedge(value)?; self.params.pwnfbedge = value; self.mark_param_given(426); Ok(()) }
            "plwnfbedge" => { validate_parameter_plwnfbedge(value)?; self.params.plwnfbedge = value; self.mark_param_given(427); Ok(()) }
            "ponfcedge" => { validate_parameter_ponfcedge(value)?; self.params.ponfcedge = value; self.mark_param_given(428); Ok(()) }
            "plnfcedge" => { validate_parameter_plnfcedge(value)?; self.params.plnfcedge = value; self.mark_param_given(429); Ok(()) }
            "pwnfcedge" => { validate_parameter_pwnfcedge(value)?; self.params.pwnfcedge = value; self.mark_param_given(430); Ok(()) }
            "plwnfcedge" => { validate_parameter_plwnfcedge(value)?; self.params.plwnfcedge = value; self.mark_param_given(431); Ok(()) }
            "poefedge" => { validate_parameter_poefedge(value)?; self.params.poefedge = value; self.mark_param_given(432); Ok(()) }
            "pokvthowe" => { validate_parameter_pokvthowe(value)?; self.params.pokvthowe = value; self.mark_param_given(433); Ok(()) }
            "plkvthowe" => { validate_parameter_plkvthowe(value)?; self.params.plkvthowe = value; self.mark_param_given(434); Ok(()) }
            "pwkvthowe" => { validate_parameter_pwkvthowe(value)?; self.params.pwkvthowe = value; self.mark_param_given(435); Ok(()) }
            "plwkvthowe" => { validate_parameter_plwkvthowe(value)?; self.params.plwkvthowe = value; self.mark_param_given(436); Ok(()) }
            "pokuowe" => { validate_parameter_pokuowe(value)?; self.params.pokuowe = value; self.mark_param_given(437); Ok(()) }
            "plkuowe" => { validate_parameter_plkuowe(value)?; self.params.plkuowe = value; self.mark_param_given(438); Ok(()) }
            "pwkuowe" => { validate_parameter_pwkuowe(value)?; self.params.pwkuowe = value; self.mark_param_given(439); Ok(()) }
            "plwkuowe" => { validate_parameter_plwkuowe(value)?; self.params.plwkuowe = value; self.mark_param_given(440); Ok(()) }
            "lmin" => { validate_parameter_lmin(value)?; self.params.lmin = value; self.mark_param_given(441); Ok(()) }
            "lmax" => { validate_parameter_lmax(value)?; self.params.lmax = value; self.mark_param_given(442); Ok(()) }
            "wmin" => { validate_parameter_wmin(value)?; self.params.wmin = value; self.mark_param_given(443); Ok(()) }
            "wmax" => { validate_parameter_wmax(value)?; self.params.wmax = value; self.mark_param_given(444); Ok(()) }
            "lvaro" => { validate_parameter_lvaro(value)?; self.params.lvaro = value; self.mark_param_given(445); Ok(()) }
            "lvarl" => { validate_parameter_lvarl(value)?; self.params.lvarl = value; self.mark_param_given(446); Ok(()) }
            "lvarw" => { validate_parameter_lvarw(value)?; self.params.lvarw = value; self.mark_param_given(447); Ok(()) }
            "lap" => { validate_parameter_lap(value)?; self.params.lap = value; self.mark_param_given(448); Ok(()) }
            "wvaro" => { validate_parameter_wvaro(value)?; self.params.wvaro = value; self.mark_param_given(449); Ok(()) }
            "wvarl" => { validate_parameter_wvarl(value)?; self.params.wvarl = value; self.mark_param_given(450); Ok(()) }
            "wvarw" => { validate_parameter_wvarw(value)?; self.params.wvarw = value; self.mark_param_given(451); Ok(()) }
            "wot" => { validate_parameter_wot(value)?; self.params.wot = value; self.mark_param_given(452); Ok(()) }
            "dlq" => { validate_parameter_dlq(value)?; self.params.dlq = value; self.mark_param_given(453); Ok(()) }
            "dwq" => { validate_parameter_dwq(value)?; self.params.dwq = value; self.mark_param_given(454); Ok(()) }
            "vfbo" => { validate_parameter_vfbo(value)?; self.params.vfbo = value; self.mark_param_given(455); Ok(()) }
            "vfbl" => { validate_parameter_vfbl(value)?; self.params.vfbl = value; self.mark_param_given(456); Ok(()) }
            "vfbw" => { validate_parameter_vfbw(value)?; self.params.vfbw = value; self.mark_param_given(457); Ok(()) }
            "vfblw" => { validate_parameter_vfblw(value)?; self.params.vfblw = value; self.mark_param_given(458); Ok(()) }
            "stvfbo" => { validate_parameter_stvfbo(value)?; self.params.stvfbo = value; self.mark_param_given(459); Ok(()) }
            "stvfbl" => { validate_parameter_stvfbl(value)?; self.params.stvfbl = value; self.mark_param_given(460); Ok(()) }
            "stvfbw" => { validate_parameter_stvfbw(value)?; self.params.stvfbw = value; self.mark_param_given(461); Ok(()) }
            "stvfblw" => { validate_parameter_stvfblw(value)?; self.params.stvfblw = value; self.mark_param_given(462); Ok(()) }
            "st2vfbo" => { validate_parameter_st2vfbo(value)?; self.params.st2vfbo = value; self.mark_param_given(463); Ok(()) }
            "toxo" => { validate_parameter_toxo(value)?; self.params.toxo = value; self.mark_param_given(464); Ok(()) }
            "epsroxo" => { validate_parameter_epsroxo(value)?; self.params.epsroxo = value; self.mark_param_given(465); Ok(()) }
            "nsubo" => { validate_parameter_nsubo(value)?; self.params.nsubo = value; self.mark_param_given(466); Ok(()) }
            "nsubw" => { validate_parameter_nsubw(value)?; self.params.nsubw = value; self.mark_param_given(467); Ok(()) }
            "wseg" => { validate_parameter_wseg(value)?; self.params.wseg = value; self.mark_param_given(468); Ok(()) }
            "npck" => { validate_parameter_npck(value)?; self.params.npck = value; self.mark_param_given(469); Ok(()) }
            "npckw" => { validate_parameter_npckw(value)?; self.params.npckw = value; self.mark_param_given(470); Ok(()) }
            "wsegp" => { validate_parameter_wsegp(value)?; self.params.wsegp = value; self.mark_param_given(471); Ok(()) }
            "lpck" => { validate_parameter_lpck(value)?; self.params.lpck = value; self.mark_param_given(472); Ok(()) }
            "lpckw" => { validate_parameter_lpckw(value)?; self.params.lpckw = value; self.mark_param_given(473); Ok(()) }
            "fol1" => { validate_parameter_fol1(value)?; self.params.fol1 = value; self.mark_param_given(474); Ok(()) }
            "fol2" => { validate_parameter_fol2(value)?; self.params.fol2 = value; self.mark_param_given(475); Ok(()) }
            "facneffaco" => { validate_parameter_facneffaco(value)?; self.params.facneffaco = value; self.mark_param_given(476); Ok(()) }
            "facneffacl" => { validate_parameter_facneffacl(value)?; self.params.facneffacl = value; self.mark_param_given(477); Ok(()) }
            "facneffacw" => { validate_parameter_facneffacw(value)?; self.params.facneffacw = value; self.mark_param_given(478); Ok(()) }
            "facneffaclw" => { validate_parameter_facneffaclw(value)?; self.params.facneffaclw = value; self.mark_param_given(479); Ok(()) }
            "gfacnudo" => { validate_parameter_gfacnudo(value)?; self.params.gfacnudo = value; self.mark_param_given(480); Ok(()) }
            "gfacnudl" => { validate_parameter_gfacnudl(value)?; self.params.gfacnudl = value; self.mark_param_given(481); Ok(()) }
            "gfacnudlexp" => { validate_parameter_gfacnudlexp(value)?; self.params.gfacnudlexp = value; self.mark_param_given(482); Ok(()) }
            "gfacnudw" => { validate_parameter_gfacnudw(value)?; self.params.gfacnudw = value; self.mark_param_given(483); Ok(()) }
            "gfacnudlw" => { validate_parameter_gfacnudlw(value)?; self.params.gfacnudlw = value; self.mark_param_given(484); Ok(()) }
            "vsbnudo" => { validate_parameter_vsbnudo(value)?; self.params.vsbnudo = value; self.mark_param_given(485); Ok(()) }
            "dvsbnudo" => { validate_parameter_dvsbnudo(value)?; self.params.dvsbnudo = value; self.mark_param_given(486); Ok(()) }
            "vnsubo" => { validate_parameter_vnsubo(value)?; self.params.vnsubo = value; self.mark_param_given(487); Ok(()) }
            "nslpo" => { validate_parameter_nslpo(value)?; self.params.nslpo = value; self.mark_param_given(488); Ok(()) }
            "dnsubo" => { validate_parameter_dnsubo(value)?; self.params.dnsubo = value; self.mark_param_given(489); Ok(()) }
            "dphibo" => { validate_parameter_dphibo(value)?; self.params.dphibo = value; self.mark_param_given(490); Ok(()) }
            "dphibl" => { validate_parameter_dphibl(value)?; self.params.dphibl = value; self.mark_param_given(491); Ok(()) }
            "dphiblexp" => { validate_parameter_dphiblexp(value)?; self.params.dphiblexp = value; self.mark_param_given(492); Ok(()) }
            "dphibw" => { validate_parameter_dphibw(value)?; self.params.dphibw = value; self.mark_param_given(493); Ok(()) }
            "dphiblw" => { validate_parameter_dphiblw(value)?; self.params.dphiblw = value; self.mark_param_given(494); Ok(()) }
            "delvtaco" => { validate_parameter_delvtaco(value)?; self.params.delvtaco = value; self.mark_param_given(495); Ok(()) }
            "delvtacl" => { validate_parameter_delvtacl(value)?; self.params.delvtacl = value; self.mark_param_given(496); Ok(()) }
            "delvtaclexp" => { validate_parameter_delvtaclexp(value)?; self.params.delvtaclexp = value; self.mark_param_given(497); Ok(()) }
            "delvtacw" => { validate_parameter_delvtacw(value)?; self.params.delvtacw = value; self.mark_param_given(498); Ok(()) }
            "delvtaclw" => { validate_parameter_delvtaclw(value)?; self.params.delvtaclw = value; self.mark_param_given(499); Ok(()) }
            "npo" => { validate_parameter_npo(value)?; self.params.npo = value; self.mark_param_given(500); Ok(()) }
            "npl" => { validate_parameter_npl(value)?; self.params.npl = value; self.mark_param_given(501); Ok(()) }
            "toxovo" => { validate_parameter_toxovo(value)?; self.params.toxovo = value; self.mark_param_given(502); Ok(()) }
            "toxovdo" => { validate_parameter_toxovdo(value)?; self.params.toxovdo = value; self.mark_param_given(503); Ok(()) }
            "lov" => { validate_parameter_lov(value)?; self.params.lov = value; self.mark_param_given(504); Ok(()) }
            "lovd" => { validate_parameter_lovd(value)?; self.params.lovd = value; self.mark_param_given(505); Ok(()) }
            "novo" => { validate_parameter_novo(value)?; self.params.novo = value; self.mark_param_given(506); Ok(()) }
            "novdo" => { validate_parameter_novdo(value)?; self.params.novdo = value; self.mark_param_given(507); Ok(()) }
            "cto" => { validate_parameter_cto(value)?; self.params.cto = value; self.mark_param_given(508); Ok(()) }
            "ctl" => { validate_parameter_ctl(value)?; self.params.ctl = value; self.mark_param_given(509); Ok(()) }
            "ctlexp" => { validate_parameter_ctlexp(value)?; self.params.ctlexp = value; self.mark_param_given(510); Ok(()) }
            "ctw" => { validate_parameter_ctw(value)?; self.params.ctw = value; self.mark_param_given(511); Ok(()) }
            "ctlw" => { validate_parameter_ctlw(value)?; self.params.ctlw = value; self.mark_param_given(512); Ok(()) }
            "ctgo" => { validate_parameter_ctgo(value)?; self.params.ctgo = value; self.mark_param_given(513); Ok(()) }
            "ctbo" => { validate_parameter_ctbo(value)?; self.params.ctbo = value; self.mark_param_given(514); Ok(()) }
            "stcto" => { validate_parameter_stcto(value)?; self.params.stcto = value; self.mark_param_given(515); Ok(()) }
            "cfl" => { validate_parameter_cfl(value)?; self.params.cfl = value; self.mark_param_given(516); Ok(()) }
            "cflexp" => { validate_parameter_cflexp(value)?; self.params.cflexp = value; self.mark_param_given(517); Ok(()) }
            "cfw" => { validate_parameter_cfw(value)?; self.params.cfw = value; self.mark_param_given(518); Ok(()) }
            "cfdo" => { validate_parameter_cfdo(value)?; self.params.cfdo = value; self.mark_param_given(519); Ok(()) }
            "cfbo" => { validate_parameter_cfbo(value)?; self.params.cfbo = value; self.mark_param_given(520); Ok(()) }
            "pscel" => { validate_parameter_pscel(value)?; self.params.pscel = value; self.mark_param_given(521); Ok(()) }
            "pscelexp" => { validate_parameter_pscelexp(value)?; self.params.pscelexp = value; self.mark_param_given(522); Ok(()) }
            "pscew" => { validate_parameter_pscew(value)?; self.params.pscew = value; self.mark_param_given(523); Ok(()) }
            "pscebo" => { validate_parameter_pscebo(value)?; self.params.pscebo = value; self.mark_param_given(524); Ok(()) }
            "pscedo" => { validate_parameter_pscedo(value)?; self.params.pscedo = value; self.mark_param_given(525); Ok(()) }
            "uo" => { validate_parameter_uo(value)?; self.params.uo = value; self.mark_param_given(526); Ok(()) }
            "fbet1" => { validate_parameter_fbet1(value)?; self.params.fbet1 = value; self.mark_param_given(527); Ok(()) }
            "fbet1w" => { validate_parameter_fbet1w(value)?; self.params.fbet1w = value; self.mark_param_given(528); Ok(()) }
            "lp1" => { validate_parameter_lp1(value)?; self.params.lp1 = value; self.mark_param_given(529); Ok(()) }
            "lp1w" => { validate_parameter_lp1w(value)?; self.params.lp1w = value; self.mark_param_given(530); Ok(()) }
            "fbet2" => { validate_parameter_fbet2(value)?; self.params.fbet2 = value; self.mark_param_given(531); Ok(()) }
            "lp2" => { validate_parameter_lp2(value)?; self.params.lp2 = value; self.mark_param_given(532); Ok(()) }
            "betw1" => { validate_parameter_betw1(value)?; self.params.betw1 = value; self.mark_param_given(533); Ok(()) }
            "betw2" => { validate_parameter_betw2(value)?; self.params.betw2 = value; self.mark_param_given(534); Ok(()) }
            "wbet" => { validate_parameter_wbet(value)?; self.params.wbet = value; self.mark_param_given(535); Ok(()) }
            "stbeto" => { validate_parameter_stbeto(value)?; self.params.stbeto = value; self.mark_param_given(536); Ok(()) }
            "stbetl" => { validate_parameter_stbetl(value)?; self.params.stbetl = value; self.mark_param_given(537); Ok(()) }
            "stbetw" => { validate_parameter_stbetw(value)?; self.params.stbetw = value; self.mark_param_given(538); Ok(()) }
            "stbetlw" => { validate_parameter_stbetlw(value)?; self.params.stbetlw = value; self.mark_param_given(539); Ok(()) }
            "mueo" => { validate_parameter_mueo(value)?; self.params.mueo = value; self.mark_param_given(540); Ok(()) }
            "muew" => { validate_parameter_muew(value)?; self.params.muew = value; self.mark_param_given(541); Ok(()) }
            "stmueo" => { validate_parameter_stmueo(value)?; self.params.stmueo = value; self.mark_param_given(542); Ok(()) }
            "themuo" => { validate_parameter_themuo(value)?; self.params.themuo = value; self.mark_param_given(543); Ok(()) }
            "stthemuo" => { validate_parameter_stthemuo(value)?; self.params.stthemuo = value; self.mark_param_given(544); Ok(()) }
            "cso" => { validate_parameter_cso(value)?; self.params.cso = value; self.mark_param_given(545); Ok(()) }
            "csl" => { validate_parameter_csl(value)?; self.params.csl = value; self.mark_param_given(546); Ok(()) }
            "cslexp" => { validate_parameter_cslexp(value)?; self.params.cslexp = value; self.mark_param_given(547); Ok(()) }
            "csw" => { validate_parameter_csw(value)?; self.params.csw = value; self.mark_param_given(548); Ok(()) }
            "cslw" => { validate_parameter_cslw(value)?; self.params.cslw = value; self.mark_param_given(549); Ok(()) }
            "stcso" => { validate_parameter_stcso(value)?; self.params.stcso = value; self.mark_param_given(550); Ok(()) }
            "thecso" => { validate_parameter_thecso(value)?; self.params.thecso = value; self.mark_param_given(551); Ok(()) }
            "stthecso" => { validate_parameter_stthecso(value)?; self.params.stthecso = value; self.mark_param_given(552); Ok(()) }
            "xcoro" => { validate_parameter_xcoro(value)?; self.params.xcoro = value; self.mark_param_given(553); Ok(()) }
            "xcorl" => { validate_parameter_xcorl(value)?; self.params.xcorl = value; self.mark_param_given(554); Ok(()) }
            "xcorw" => { validate_parameter_xcorw(value)?; self.params.xcorw = value; self.mark_param_given(555); Ok(()) }
            "xcorlw" => { validate_parameter_xcorlw(value)?; self.params.xcorlw = value; self.mark_param_given(556); Ok(()) }
            "stxcoro" => { validate_parameter_stxcoro(value)?; self.params.stxcoro = value; self.mark_param_given(557); Ok(()) }
            "fetao" => { validate_parameter_fetao(value)?; self.params.fetao = value; self.mark_param_given(558); Ok(()) }
            "rsw1" => { validate_parameter_rsw1(value)?; self.params.rsw1 = value; self.mark_param_given(559); Ok(()) }
            "rsw2" => { validate_parameter_rsw2(value)?; self.params.rsw2 = value; self.mark_param_given(560); Ok(()) }
            "strso" => { validate_parameter_strso(value)?; self.params.strso = value; self.mark_param_given(561); Ok(()) }
            "rsbo" => { validate_parameter_rsbo(value)?; self.params.rsbo = value; self.mark_param_given(562); Ok(()) }
            "rsgo" => { validate_parameter_rsgo(value)?; self.params.rsgo = value; self.mark_param_given(563); Ok(()) }
            "thesato" => { validate_parameter_thesato(value)?; self.params.thesato = value; self.mark_param_given(564); Ok(()) }
            "thesatl" => { validate_parameter_thesatl(value)?; self.params.thesatl = value; self.mark_param_given(565); Ok(()) }
            "thesatlexp" => { validate_parameter_thesatlexp(value)?; self.params.thesatlexp = value; self.mark_param_given(566); Ok(()) }
            "thesatw" => { validate_parameter_thesatw(value)?; self.params.thesatw = value; self.mark_param_given(567); Ok(()) }
            "thesatlw" => { validate_parameter_thesatlw(value)?; self.params.thesatlw = value; self.mark_param_given(568); Ok(()) }
            "stthesato" => { validate_parameter_stthesato(value)?; self.params.stthesato = value; self.mark_param_given(569); Ok(()) }
            "stthesatl" => { validate_parameter_stthesatl(value)?; self.params.stthesatl = value; self.mark_param_given(570); Ok(()) }
            "stthesatw" => { validate_parameter_stthesatw(value)?; self.params.stthesatw = value; self.mark_param_given(571); Ok(()) }
            "stthesatlw" => { validate_parameter_stthesatlw(value)?; self.params.stthesatlw = value; self.mark_param_given(572); Ok(()) }
            "thesatbo" => { validate_parameter_thesatbo(value)?; self.params.thesatbo = value; self.mark_param_given(573); Ok(()) }
            "thesatgo" => { validate_parameter_thesatgo(value)?; self.params.thesatgo = value; self.mark_param_given(574); Ok(()) }
            "axo" => { validate_parameter_axo(value)?; self.params.axo = value; self.mark_param_given(575); Ok(()) }
            "axl" => { validate_parameter_axl(value)?; self.params.axl = value; self.mark_param_given(576); Ok(()) }
            "alpl" => { validate_parameter_alpl(value)?; self.params.alpl = value; self.mark_param_given(577); Ok(()) }
            "alplexp" => { validate_parameter_alplexp(value)?; self.params.alplexp = value; self.mark_param_given(578); Ok(()) }
            "alpw" => { validate_parameter_alpw(value)?; self.params.alpw = value; self.mark_param_given(579); Ok(()) }
            "alp1l1" => { validate_parameter_alp1l1(value)?; self.params.alp1l1 = value; self.mark_param_given(580); Ok(()) }
            "alp1lexp" => { validate_parameter_alp1lexp(value)?; self.params.alp1lexp = value; self.mark_param_given(581); Ok(()) }
            "alp1l2" => { validate_parameter_alp1l2(value)?; self.params.alp1l2 = value; self.mark_param_given(582); Ok(()) }
            "alp1w" => { validate_parameter_alp1w(value)?; self.params.alp1w = value; self.mark_param_given(583); Ok(()) }
            "alp2l1" => { validate_parameter_alp2l1(value)?; self.params.alp2l1 = value; self.mark_param_given(584); Ok(()) }
            "alp2lexp" => { validate_parameter_alp2lexp(value)?; self.params.alp2lexp = value; self.mark_param_given(585); Ok(()) }
            "alp2l2" => { validate_parameter_alp2l2(value)?; self.params.alp2l2 = value; self.mark_param_given(586); Ok(()) }
            "alp2w" => { validate_parameter_alp2w(value)?; self.params.alp2w = value; self.mark_param_given(587); Ok(()) }
            "vpo" => { validate_parameter_vpo(value)?; self.params.vpo = value; self.mark_param_given(588); Ok(()) }
            "a1o" => { validate_parameter_a1o(value)?; self.params.a1o = value; self.mark_param_given(589); Ok(()) }
            "a1l" => { validate_parameter_a1l(value)?; self.params.a1l = value; self.mark_param_given(590); Ok(()) }
            "a1w" => { validate_parameter_a1w(value)?; self.params.a1w = value; self.mark_param_given(591); Ok(()) }
            "a2o" => { validate_parameter_a2o(value)?; self.params.a2o = value; self.mark_param_given(592); Ok(()) }
            "sta2o" => { validate_parameter_sta2o(value)?; self.params.sta2o = value; self.mark_param_given(593); Ok(()) }
            "a3o" => { validate_parameter_a3o(value)?; self.params.a3o = value; self.mark_param_given(594); Ok(()) }
            "a3l" => { validate_parameter_a3l(value)?; self.params.a3l = value; self.mark_param_given(595); Ok(()) }
            "a3w" => { validate_parameter_a3w(value)?; self.params.a3w = value; self.mark_param_given(596); Ok(()) }
            "a4o" => { validate_parameter_a4o(value)?; self.params.a4o = value; self.mark_param_given(597); Ok(()) }
            "a4l" => { validate_parameter_a4l(value)?; self.params.a4l = value; self.mark_param_given(598); Ok(()) }
            "a4w" => { validate_parameter_a4w(value)?; self.params.a4w = value; self.mark_param_given(599); Ok(()) }
            "gcoo" => { validate_parameter_gcoo(value)?; self.params.gcoo = value; self.mark_param_given(600); Ok(()) }
            "iginvlw" => { validate_parameter_iginvlw(value)?; self.params.iginvlw = value; self.mark_param_given(601); Ok(()) }
            "igovw" => { validate_parameter_igovw(value)?; self.params.igovw = value; self.mark_param_given(602); Ok(()) }
            "igovdw" => { validate_parameter_igovdw(value)?; self.params.igovdw = value; self.mark_param_given(603); Ok(()) }
            "stigo" => { validate_parameter_stigo(value)?; self.params.stigo = value; self.mark_param_given(604); Ok(()) }
            "gc2o" => { validate_parameter_gc2o(value)?; self.params.gc2o = value; self.mark_param_given(605); Ok(()) }
            "gc3o" => { validate_parameter_gc3o(value)?; self.params.gc3o = value; self.mark_param_given(606); Ok(()) }
            "chibo" => { validate_parameter_chibo(value)?; self.params.chibo = value; self.mark_param_given(607); Ok(()) }
            "agidlw" => { validate_parameter_agidlw(value)?; self.params.agidlw = value; self.mark_param_given(608); Ok(()) }
            "agidldw" => { validate_parameter_agidldw(value)?; self.params.agidldw = value; self.mark_param_given(609); Ok(()) }
            "bgidlo" => { validate_parameter_bgidlo(value)?; self.params.bgidlo = value; self.mark_param_given(610); Ok(()) }
            "bgidldo" => { validate_parameter_bgidldo(value)?; self.params.bgidldo = value; self.mark_param_given(611); Ok(()) }
            "stbgidlo" => { validate_parameter_stbgidlo(value)?; self.params.stbgidlo = value; self.mark_param_given(612); Ok(()) }
            "stbgidldo" => { validate_parameter_stbgidldo(value)?; self.params.stbgidldo = value; self.mark_param_given(613); Ok(()) }
            "cgidlo" => { validate_parameter_cgidlo(value)?; self.params.cgidlo = value; self.mark_param_given(614); Ok(()) }
            "cgidldo" => { validate_parameter_cgidldo(value)?; self.params.cgidldo = value; self.mark_param_given(615); Ok(()) }
            "cgbovl" => { validate_parameter_cgbovl(value)?; self.params.cgbovl = value; self.mark_param_given(616); Ok(()) }
            "cfrw" => { validate_parameter_cfrw(value)?; self.params.cfrw = value; self.mark_param_given(617); Ok(()) }
            "cfrdw" => { validate_parameter_cfrdw(value)?; self.params.cfrdw = value; self.mark_param_given(618); Ok(()) }
            "fnto" => { validate_parameter_fnto(value)?; self.params.fnto = value; self.mark_param_given(619); Ok(()) }
            "fntexcl" => { validate_parameter_fntexcl(value)?; self.params.fntexcl = value; self.mark_param_given(620); Ok(()) }
            "nfalw" => { validate_parameter_nfalw(value)?; self.params.nfalw = value; self.mark_param_given(621); Ok(()) }
            "nfblw" => { validate_parameter_nfblw(value)?; self.params.nfblw = value; self.mark_param_given(622); Ok(()) }
            "nfclw" => { validate_parameter_nfclw(value)?; self.params.nfclw = value; self.mark_param_given(623); Ok(()) }
            "efo" => { validate_parameter_efo(value)?; self.params.efo = value; self.mark_param_given(624); Ok(()) }
            "lintnoi" => { validate_parameter_lintnoi(value)?; self.params.lintnoi = value; self.mark_param_given(625); Ok(()) }
            "alpnoi" => { validate_parameter_alpnoi(value)?; self.params.alpnoi = value; self.mark_param_given(626); Ok(()) }
            "wedge" => { validate_parameter_wedge(value)?; self.params.wedge = value; self.mark_param_given(627); Ok(()) }
            "wedgew" => { validate_parameter_wedgew(value)?; self.params.wedgew = value; self.mark_param_given(628); Ok(()) }
            "vfbedgeo" => { validate_parameter_vfbedgeo(value)?; self.params.vfbedgeo = value; self.mark_param_given(629); Ok(()) }
            "stvfbedgeo" => { validate_parameter_stvfbedgeo(value)?; self.params.stvfbedgeo = value; self.mark_param_given(630); Ok(()) }
            "stvfbedgel" => { validate_parameter_stvfbedgel(value)?; self.params.stvfbedgel = value; self.mark_param_given(631); Ok(()) }
            "stvfbedgew" => { validate_parameter_stvfbedgew(value)?; self.params.stvfbedgew = value; self.mark_param_given(632); Ok(()) }
            "stvfbedgelw" => { validate_parameter_stvfbedgelw(value)?; self.params.stvfbedgelw = value; self.mark_param_given(633); Ok(()) }
            "dphibedgeo" => { validate_parameter_dphibedgeo(value)?; self.params.dphibedgeo = value; self.mark_param_given(634); Ok(()) }
            "dphibedgel" => { validate_parameter_dphibedgel(value)?; self.params.dphibedgel = value; self.mark_param_given(635); Ok(()) }
            "dphibedgelexp" => { validate_parameter_dphibedgelexp(value)?; self.params.dphibedgelexp = value; self.mark_param_given(636); Ok(()) }
            "dphibedgew" => { validate_parameter_dphibedgew(value)?; self.params.dphibedgew = value; self.mark_param_given(637); Ok(()) }
            "dphibedgelw" => { validate_parameter_dphibedgelw(value)?; self.params.dphibedgelw = value; self.mark_param_given(638); Ok(()) }
            "nsubedgeo" => { validate_parameter_nsubedgeo(value)?; self.params.nsubedgeo = value; self.mark_param_given(639); Ok(()) }
            "nsubedgel" => { validate_parameter_nsubedgel(value)?; self.params.nsubedgel = value; self.mark_param_given(640); Ok(()) }
            "nsubedgelexp" => { validate_parameter_nsubedgelexp(value)?; self.params.nsubedgelexp = value; self.mark_param_given(641); Ok(()) }
            "nsubedgew" => { validate_parameter_nsubedgew(value)?; self.params.nsubedgew = value; self.mark_param_given(642); Ok(()) }
            "nsubedgelw" => { validate_parameter_nsubedgelw(value)?; self.params.nsubedgelw = value; self.mark_param_given(643); Ok(()) }
            "ctedgeo" => { validate_parameter_ctedgeo(value)?; self.params.ctedgeo = value; self.mark_param_given(644); Ok(()) }
            "ctedgel" => { validate_parameter_ctedgel(value)?; self.params.ctedgel = value; self.mark_param_given(645); Ok(()) }
            "ctedgelexp" => { validate_parameter_ctedgelexp(value)?; self.params.ctedgelexp = value; self.mark_param_given(646); Ok(()) }
            "fbetedge" => { validate_parameter_fbetedge(value)?; self.params.fbetedge = value; self.mark_param_given(647); Ok(()) }
            "lpedge" => { validate_parameter_lpedge(value)?; self.params.lpedge = value; self.mark_param_given(648); Ok(()) }
            "betedgew" => { validate_parameter_betedgew(value)?; self.params.betedgew = value; self.mark_param_given(649); Ok(()) }
            "stbetedgeo" => { validate_parameter_stbetedgeo(value)?; self.params.stbetedgeo = value; self.mark_param_given(650); Ok(()) }
            "stbetedgel" => { validate_parameter_stbetedgel(value)?; self.params.stbetedgel = value; self.mark_param_given(651); Ok(()) }
            "stbetedgew" => { validate_parameter_stbetedgew(value)?; self.params.stbetedgew = value; self.mark_param_given(652); Ok(()) }
            "stbetedgelw" => { validate_parameter_stbetedgelw(value)?; self.params.stbetedgelw = value; self.mark_param_given(653); Ok(()) }
            "psceedgel" => { validate_parameter_psceedgel(value)?; self.params.psceedgel = value; self.mark_param_given(654); Ok(()) }
            "psceedgelexp" => { validate_parameter_psceedgelexp(value)?; self.params.psceedgelexp = value; self.mark_param_given(655); Ok(()) }
            "psceedgew" => { validate_parameter_psceedgew(value)?; self.params.psceedgew = value; self.mark_param_given(656); Ok(()) }
            "pscebedgeo" => { validate_parameter_pscebedgeo(value)?; self.params.pscebedgeo = value; self.mark_param_given(657); Ok(()) }
            "pscededgeo" => { validate_parameter_pscededgeo(value)?; self.params.pscededgeo = value; self.mark_param_given(658); Ok(()) }
            "cfedgel" => { validate_parameter_cfedgel(value)?; self.params.cfedgel = value; self.mark_param_given(659); Ok(()) }
            "cfedgelexp" => { validate_parameter_cfedgelexp(value)?; self.params.cfedgelexp = value; self.mark_param_given(660); Ok(()) }
            "cfedgew" => { validate_parameter_cfedgew(value)?; self.params.cfedgew = value; self.mark_param_given(661); Ok(()) }
            "cfdedgeo" => { validate_parameter_cfdedgeo(value)?; self.params.cfdedgeo = value; self.mark_param_given(662); Ok(()) }
            "cfbedgeo" => { validate_parameter_cfbedgeo(value)?; self.params.cfbedgeo = value; self.mark_param_given(663); Ok(()) }
            "fntedgeo" => { validate_parameter_fntedgeo(value)?; self.params.fntedgeo = value; self.mark_param_given(664); Ok(()) }
            "nfaedgelw" => { validate_parameter_nfaedgelw(value)?; self.params.nfaedgelw = value; self.mark_param_given(665); Ok(()) }
            "nfbedgelw" => { validate_parameter_nfbedgelw(value)?; self.params.nfbedgelw = value; self.mark_param_given(666); Ok(()) }
            "nfcedgelw" => { validate_parameter_nfcedgelw(value)?; self.params.nfcedgelw = value; self.mark_param_given(667); Ok(()) }
            "efedgeo" => { validate_parameter_efedgeo(value)?; self.params.efedgeo = value; self.mark_param_given(668); Ok(()) }
            "kvthoweo" => { validate_parameter_kvthoweo(value)?; self.params.kvthoweo = value; self.mark_param_given(669); Ok(()) }
            "kvthowel" => { validate_parameter_kvthowel(value)?; self.params.kvthowel = value; self.mark_param_given(670); Ok(()) }
            "kvthowew" => { validate_parameter_kvthowew(value)?; self.params.kvthowew = value; self.mark_param_given(671); Ok(()) }
            "kvthowelw" => { validate_parameter_kvthowelw(value)?; self.params.kvthowelw = value; self.mark_param_given(672); Ok(()) }
            "kuoweo" => { validate_parameter_kuoweo(value)?; self.params.kuoweo = value; self.mark_param_given(673); Ok(()) }
            "kuowel" => { validate_parameter_kuowel(value)?; self.params.kuowel = value; self.mark_param_given(674); Ok(()) }
            "kuowew" => { validate_parameter_kuowew(value)?; self.params.kuowew = value; self.mark_param_given(675); Ok(()) }
            "kuowelw" => { validate_parameter_kuowelw(value)?; self.params.kuowelw = value; self.mark_param_given(676); Ok(()) }
            "rgo" => { validate_parameter_rgo(value)?; self.params.rgo = value; self.mark_param_given(677); Ok(()) }
            "rint" => { validate_parameter_rint(value)?; self.params.rint = value; self.mark_param_given(678); Ok(()) }
            "rvpoly" => { validate_parameter_rvpoly(value)?; self.params.rvpoly = value; self.mark_param_given(679); Ok(()) }
            "rshg" => { validate_parameter_rshg(value)?; self.params.rshg = value; self.mark_param_given(680); Ok(()) }
            "dlsil" => { validate_parameter_dlsil(value)?; self.params.dlsil = value; self.mark_param_given(681); Ok(()) }
            "rsh" => { validate_parameter_rsh(value)?; self.params.rsh = value; self.mark_param_given(682); Ok(()) }
            "rshd" => { validate_parameter_rshd(value)?; self.params.rshd = value; self.mark_param_given(683); Ok(()) }
            "rbulko" => { validate_parameter_rbulko(value)?; self.params.rbulko = value; self.mark_param_given(684); Ok(()) }
            "rwello" => { validate_parameter_rwello(value)?; self.params.rwello = value; self.mark_param_given(685); Ok(()) }
            "rjunso" => { validate_parameter_rjunso(value)?; self.params.rjunso = value; self.mark_param_given(686); Ok(()) }
            "rjundo" => { validate_parameter_rjundo(value)?; self.params.rjundo = value; self.mark_param_given(687); Ok(()) }
            "saref" => { validate_parameter_saref(value)?; self.params.saref = value; self.mark_param_given(688); Ok(()) }
            "sbref" => { validate_parameter_sbref(value)?; self.params.sbref = value; self.mark_param_given(689); Ok(()) }
            "wlod" => { validate_parameter_wlod(value)?; self.params.wlod = value; self.mark_param_given(690); Ok(()) }
            "kuo" => { validate_parameter_kuo(value)?; self.params.kuo = value; self.mark_param_given(691); Ok(()) }
            "kvsat" => { validate_parameter_kvsat(value)?; self.params.kvsat = value; self.mark_param_given(692); Ok(()) }
            "tkuo" => { validate_parameter_tkuo(value)?; self.params.tkuo = value; self.mark_param_given(693); Ok(()) }
            "lkuo" => { validate_parameter_lkuo(value)?; self.params.lkuo = value; self.mark_param_given(694); Ok(()) }
            "wkuo" => { validate_parameter_wkuo(value)?; self.params.wkuo = value; self.mark_param_given(695); Ok(()) }
            "pkuo" => { validate_parameter_pkuo(value)?; self.params.pkuo = value; self.mark_param_given(696); Ok(()) }
            "llodkuo" => { validate_parameter_llodkuo(value)?; self.params.llodkuo = value; self.mark_param_given(697); Ok(()) }
            "wlodkuo" => { validate_parameter_wlodkuo(value)?; self.params.wlodkuo = value; self.mark_param_given(698); Ok(()) }
            "kvtho" => { validate_parameter_kvtho(value)?; self.params.kvtho = value; self.mark_param_given(699); Ok(()) }
            "lkvtho" => { validate_parameter_lkvtho(value)?; self.params.lkvtho = value; self.mark_param_given(700); Ok(()) }
            "wkvtho" => { validate_parameter_wkvtho(value)?; self.params.wkvtho = value; self.mark_param_given(701); Ok(()) }
            "pkvtho" => { validate_parameter_pkvtho(value)?; self.params.pkvtho = value; self.mark_param_given(702); Ok(()) }
            "llodvth" => { validate_parameter_llodvth(value)?; self.params.llodvth = value; self.mark_param_given(703); Ok(()) }
            "wlodvth" => { validate_parameter_wlodvth(value)?; self.params.wlodvth = value; self.mark_param_given(704); Ok(()) }
            "stetao" => { validate_parameter_stetao(value)?; self.params.stetao = value; self.mark_param_given(705); Ok(()) }
            "lodetao" => { validate_parameter_lodetao(value)?; self.params.lodetao = value; self.mark_param_given(706); Ok(()) }
            "scref" => { validate_parameter_scref(value)?; self.params.scref = value; self.mark_param_given(707); Ok(()) }
            "web" => { validate_parameter_web(value)?; self.params.web = value; self.mark_param_given(708); Ok(()) }
            "wec" => { validate_parameter_wec(value)?; self.params.wec = value; self.mark_param_given(709); Ok(()) }
            "swsoa" => { validate_parameter_swsoa(value)?; self.params.swsoa = value; self.mark_param_given(710); Ok(()) }
            "vgs_max" => { validate_parameter_vgs_max(value)?; self.params.vgs_max = value; self.mark_param_given(711); Ok(()) }
            "vgd_max" => { validate_parameter_vgd_max(value)?; self.params.vgd_max = value; self.mark_param_given(712); Ok(()) }
            "vgb_max" => { validate_parameter_vgb_max(value)?; self.params.vgb_max = value; self.mark_param_given(713); Ok(()) }
            "vds_max" => { validate_parameter_vds_max(value)?; self.params.vds_max = value; self.mark_param_given(714); Ok(()) }
            "vdb_max" => { validate_parameter_vdb_max(value)?; self.params.vdb_max = value; self.mark_param_given(715); Ok(()) }
            "vsb_max" => { validate_parameter_vsb_max(value)?; self.params.vsb_max = value; self.mark_param_given(716); Ok(()) }
            "imax" => { validate_parameter_imax(value)?; self.params.imax = value; self.mark_param_given(717); Ok(()) }
            "trj" => { validate_parameter_trj(value)?; self.params.trj = value; self.mark_param_given(718); Ok(()) }
            "frev" => { validate_parameter_frev(value)?; self.params.frev = value; self.mark_param_given(719); Ok(()) }
            "cjorbot" => { validate_parameter_cjorbot(value)?; self.params.cjorbot = value; self.mark_param_given(720); Ok(()) }
            "cjorsti" => { validate_parameter_cjorsti(value)?; self.params.cjorsti = value; self.mark_param_given(721); Ok(()) }
            "cjorgat" => { validate_parameter_cjorgat(value)?; self.params.cjorgat = value; self.mark_param_given(722); Ok(()) }
            "vbirbot" => { validate_parameter_vbirbot(value)?; self.params.vbirbot = value; self.mark_param_given(723); Ok(()) }
            "vbirsti" => { validate_parameter_vbirsti(value)?; self.params.vbirsti = value; self.mark_param_given(724); Ok(()) }
            "vbirgat" => { validate_parameter_vbirgat(value)?; self.params.vbirgat = value; self.mark_param_given(725); Ok(()) }
            "pbot" => { validate_parameter_pbot(value)?; self.params.pbot = value; self.mark_param_given(726); Ok(()) }
            "psti" => { validate_parameter_psti(value)?; self.params.psti = value; self.mark_param_given(727); Ok(()) }
            "pgat" => { validate_parameter_pgat(value)?; self.params.pgat = value; self.mark_param_given(728); Ok(()) }
            "phigbot" => { validate_parameter_phigbot(value)?; self.params.phigbot = value; self.mark_param_given(729); Ok(()) }
            "phigsti" => { validate_parameter_phigsti(value)?; self.params.phigsti = value; self.mark_param_given(730); Ok(()) }
            "phiggat" => { validate_parameter_phiggat(value)?; self.params.phiggat = value; self.mark_param_given(731); Ok(()) }
            "idsatrbot" => { validate_parameter_idsatrbot(value)?; self.params.idsatrbot = value; self.mark_param_given(732); Ok(()) }
            "idsatrsti" => { validate_parameter_idsatrsti(value)?; self.params.idsatrsti = value; self.mark_param_given(733); Ok(()) }
            "idsatrgat" => { validate_parameter_idsatrgat(value)?; self.params.idsatrgat = value; self.mark_param_given(734); Ok(()) }
            "csrhbot" => { validate_parameter_csrhbot(value)?; self.params.csrhbot = value; self.mark_param_given(735); Ok(()) }
            "csrhsti" => { validate_parameter_csrhsti(value)?; self.params.csrhsti = value; self.mark_param_given(736); Ok(()) }
            "csrhgat" => { validate_parameter_csrhgat(value)?; self.params.csrhgat = value; self.mark_param_given(737); Ok(()) }
            "xjunsti" => { validate_parameter_xjunsti(value)?; self.params.xjunsti = value; self.mark_param_given(738); Ok(()) }
            "xjungat" => { validate_parameter_xjungat(value)?; self.params.xjungat = value; self.mark_param_given(739); Ok(()) }
            "ctatbot" => { validate_parameter_ctatbot(value)?; self.params.ctatbot = value; self.mark_param_given(740); Ok(()) }
            "ctatsti" => { validate_parameter_ctatsti(value)?; self.params.ctatsti = value; self.mark_param_given(741); Ok(()) }
            "ctatgat" => { validate_parameter_ctatgat(value)?; self.params.ctatgat = value; self.mark_param_given(742); Ok(()) }
            "mefftatbot" => { validate_parameter_mefftatbot(value)?; self.params.mefftatbot = value; self.mark_param_given(743); Ok(()) }
            "mefftatsti" => { validate_parameter_mefftatsti(value)?; self.params.mefftatsti = value; self.mark_param_given(744); Ok(()) }
            "mefftatgat" => { validate_parameter_mefftatgat(value)?; self.params.mefftatgat = value; self.mark_param_given(745); Ok(()) }
            "cbbtbot" => { validate_parameter_cbbtbot(value)?; self.params.cbbtbot = value; self.mark_param_given(746); Ok(()) }
            "cbbtsti" => { validate_parameter_cbbtsti(value)?; self.params.cbbtsti = value; self.mark_param_given(747); Ok(()) }
            "cbbtgat" => { validate_parameter_cbbtgat(value)?; self.params.cbbtgat = value; self.mark_param_given(748); Ok(()) }
            "fbbtrbot" => { validate_parameter_fbbtrbot(value)?; self.params.fbbtrbot = value; self.mark_param_given(749); Ok(()) }
            "fbbtrsti" => { validate_parameter_fbbtrsti(value)?; self.params.fbbtrsti = value; self.mark_param_given(750); Ok(()) }
            "fbbtrgat" => { validate_parameter_fbbtrgat(value)?; self.params.fbbtrgat = value; self.mark_param_given(751); Ok(()) }
            "stfbbtbot" => { validate_parameter_stfbbtbot(value)?; self.params.stfbbtbot = value; self.mark_param_given(752); Ok(()) }
            "stfbbtsti" => { validate_parameter_stfbbtsti(value)?; self.params.stfbbtsti = value; self.mark_param_given(753); Ok(()) }
            "stfbbtgat" => { validate_parameter_stfbbtgat(value)?; self.params.stfbbtgat = value; self.mark_param_given(754); Ok(()) }
            "vbrbot" => { validate_parameter_vbrbot(value)?; self.params.vbrbot = value; self.mark_param_given(755); Ok(()) }
            "vbrsti" => { validate_parameter_vbrsti(value)?; self.params.vbrsti = value; self.mark_param_given(756); Ok(()) }
            "vbrgat" => { validate_parameter_vbrgat(value)?; self.params.vbrgat = value; self.mark_param_given(757); Ok(()) }
            "pbrbot" => { validate_parameter_pbrbot(value)?; self.params.pbrbot = value; self.mark_param_given(758); Ok(()) }
            "pbrsti" => { validate_parameter_pbrsti(value)?; self.params.pbrsti = value; self.mark_param_given(759); Ok(()) }
            "pbrgat" => { validate_parameter_pbrgat(value)?; self.params.pbrgat = value; self.mark_param_given(760); Ok(()) }
            "cjorbotd" => { validate_parameter_cjorbotd(value)?; self.params.cjorbotd = value; self.mark_param_given(761); Ok(()) }
            "cjorstid" => { validate_parameter_cjorstid(value)?; self.params.cjorstid = value; self.mark_param_given(762); Ok(()) }
            "cjorgatd" => { validate_parameter_cjorgatd(value)?; self.params.cjorgatd = value; self.mark_param_given(763); Ok(()) }
            "vbirbotd" => { validate_parameter_vbirbotd(value)?; self.params.vbirbotd = value; self.mark_param_given(764); Ok(()) }
            "vbirstid" => { validate_parameter_vbirstid(value)?; self.params.vbirstid = value; self.mark_param_given(765); Ok(()) }
            "vbirgatd" => { validate_parameter_vbirgatd(value)?; self.params.vbirgatd = value; self.mark_param_given(766); Ok(()) }
            "pbotd" => { validate_parameter_pbotd(value)?; self.params.pbotd = value; self.mark_param_given(767); Ok(()) }
            "pstid" => { validate_parameter_pstid(value)?; self.params.pstid = value; self.mark_param_given(768); Ok(()) }
            "pgatd" => { validate_parameter_pgatd(value)?; self.params.pgatd = value; self.mark_param_given(769); Ok(()) }
            "phigbotd" => { validate_parameter_phigbotd(value)?; self.params.phigbotd = value; self.mark_param_given(770); Ok(()) }
            "phigstid" => { validate_parameter_phigstid(value)?; self.params.phigstid = value; self.mark_param_given(771); Ok(()) }
            "phiggatd" => { validate_parameter_phiggatd(value)?; self.params.phiggatd = value; self.mark_param_given(772); Ok(()) }
            "idsatrbotd" => { validate_parameter_idsatrbotd(value)?; self.params.idsatrbotd = value; self.mark_param_given(773); Ok(()) }
            "idsatrstid" => { validate_parameter_idsatrstid(value)?; self.params.idsatrstid = value; self.mark_param_given(774); Ok(()) }
            "idsatrgatd" => { validate_parameter_idsatrgatd(value)?; self.params.idsatrgatd = value; self.mark_param_given(775); Ok(()) }
            "csrhbotd" => { validate_parameter_csrhbotd(value)?; self.params.csrhbotd = value; self.mark_param_given(776); Ok(()) }
            "csrhstid" => { validate_parameter_csrhstid(value)?; self.params.csrhstid = value; self.mark_param_given(777); Ok(()) }
            "csrhgatd" => { validate_parameter_csrhgatd(value)?; self.params.csrhgatd = value; self.mark_param_given(778); Ok(()) }
            "xjunstid" => { validate_parameter_xjunstid(value)?; self.params.xjunstid = value; self.mark_param_given(779); Ok(()) }
            "xjungatd" => { validate_parameter_xjungatd(value)?; self.params.xjungatd = value; self.mark_param_given(780); Ok(()) }
            "ctatbotd" => { validate_parameter_ctatbotd(value)?; self.params.ctatbotd = value; self.mark_param_given(781); Ok(()) }
            "ctatstid" => { validate_parameter_ctatstid(value)?; self.params.ctatstid = value; self.mark_param_given(782); Ok(()) }
            "ctatgatd" => { validate_parameter_ctatgatd(value)?; self.params.ctatgatd = value; self.mark_param_given(783); Ok(()) }
            "mefftatbotd" => { validate_parameter_mefftatbotd(value)?; self.params.mefftatbotd = value; self.mark_param_given(784); Ok(()) }
            "mefftatstid" => { validate_parameter_mefftatstid(value)?; self.params.mefftatstid = value; self.mark_param_given(785); Ok(()) }
            "mefftatgatd" => { validate_parameter_mefftatgatd(value)?; self.params.mefftatgatd = value; self.mark_param_given(786); Ok(()) }
            "cbbtbotd" => { validate_parameter_cbbtbotd(value)?; self.params.cbbtbotd = value; self.mark_param_given(787); Ok(()) }
            "cbbtstid" => { validate_parameter_cbbtstid(value)?; self.params.cbbtstid = value; self.mark_param_given(788); Ok(()) }
            "cbbtgatd" => { validate_parameter_cbbtgatd(value)?; self.params.cbbtgatd = value; self.mark_param_given(789); Ok(()) }
            "fbbtrbotd" => { validate_parameter_fbbtrbotd(value)?; self.params.fbbtrbotd = value; self.mark_param_given(790); Ok(()) }
            "fbbtrstid" => { validate_parameter_fbbtrstid(value)?; self.params.fbbtrstid = value; self.mark_param_given(791); Ok(()) }
            "fbbtrgatd" => { validate_parameter_fbbtrgatd(value)?; self.params.fbbtrgatd = value; self.mark_param_given(792); Ok(()) }
            "stfbbtbotd" => { validate_parameter_stfbbtbotd(value)?; self.params.stfbbtbotd = value; self.mark_param_given(793); Ok(()) }
            "stfbbtstid" => { validate_parameter_stfbbtstid(value)?; self.params.stfbbtstid = value; self.mark_param_given(794); Ok(()) }
            "stfbbtgatd" => { validate_parameter_stfbbtgatd(value)?; self.params.stfbbtgatd = value; self.mark_param_given(795); Ok(()) }
            "vbrbotd" => { validate_parameter_vbrbotd(value)?; self.params.vbrbotd = value; self.mark_param_given(796); Ok(()) }
            "vbrstid" => { validate_parameter_vbrstid(value)?; self.params.vbrstid = value; self.mark_param_given(797); Ok(()) }
            "vbrgatd" => { validate_parameter_vbrgatd(value)?; self.params.vbrgatd = value; self.mark_param_given(798); Ok(()) }
            "pbrbotd" => { validate_parameter_pbrbotd(value)?; self.params.pbrbotd = value; self.mark_param_given(799); Ok(()) }
            "pbrstid" => { validate_parameter_pbrstid(value)?; self.params.pbrstid = value; self.mark_param_given(800); Ok(()) }
            "pbrgatd" => { validate_parameter_pbrgatd(value)?; self.params.pbrgatd = value; self.mark_param_given(801); Ok(()) }
            "swjunexp" => { validate_parameter_swjunexp(value)?; self.params.swjunexp = value; self.mark_param_given(802); Ok(()) }
            "vjunref" => { validate_parameter_vjunref(value)?; self.params.vjunref = value; self.mark_param_given(803); Ok(()) }
            "fjunq" => { validate_parameter_fjunq(value)?; self.params.fjunq = value; self.mark_param_given(804); Ok(()) }
            "vjunrefd" => { validate_parameter_vjunrefd(value)?; self.params.vjunrefd = value; self.mark_param_given(805); Ok(()) }
            "fjunqd" => { validate_parameter_fjunqd(value)?; self.params.fjunqd = value; self.mark_param_given(806); Ok(()) }
            "dta" => { validate_parameter_dta(value)?; self.params.dta = value; self.mark_param_given(807); Ok(()) }
            "trise" => { validate_parameter_dta(value)?; self.params.dta = value; self.mark_param_given(807); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'PSP103VA'", name)),
        }
    }

    #[inline]
    fn mark_param_given(&mut self, index: usize) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        self.param_given[index] = true;
    }

    #[inline]
    pub fn set_multiplicity(&mut self, multiplicity: f64) {
        if multiplicity.is_finite() && multiplicity > 0.0 {
            self.multiplicity = multiplicity;
        }
    }

    #[inline]
    pub fn set_timepoint(&mut self, time: f64, timestep: f64) {
        self.time = time;
        self.timestep = timestep;
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        let mut index = 0usize;
        while index < Self::DDT_STATE_COUNT {
            self.ddt_state_previous[index] = self.ddt_state_current[index];
            self.ddt_state_initialized[index] = true;
            index += 1;
        }
    }

    #[inline]
    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {
        debug_assert!(slot < Self::DDT_STATE_COUNT, "generated ddt state slot out of range");
        let previous = if self.ddt_state_initialized[slot] {
            self.ddt_state_previous[slot]
        } else {
            value
        };
        self.ddt_state_current[slot] = value;
        if self.timestep.abs() > Self::DDT_EPSILON {
            (value - previous) / self.timestep
        } else {
            self.ddt_state_previous[slot] = value;
            self.ddt_state_initialized[slot] = true;
            0.0
        }
    }

    #[inline]
    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {
        if self.timestep.abs() > Self::DDT_EPSILON {
            derivative / self.timestep
        } else {
            0.0
        }
    }
}
