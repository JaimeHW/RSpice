# CMC Standard Models Provenance

Source: the Compact Model Coalition standard-model releases published by the
Silicon Integration Initiative, https://www.si2.org/cmc

Each subdirectory here is one upstream release package, kept under its upstream
release name so the version and release date stay visible on disk. The upstream
`LICENSE.txt`/`NOTICE.txt` files that ship with each package are preserved
alongside the sources.

License: stated per release, in that release's own license file.

- Educational Community License, Version 2.0, in `LICENSE.txt`:
  `ASM-ESD101.1.0_04042025`, `ASM-HEMT101.6.0_05132026`,
  `BSIM-BULK107.2.1_02112025`, `BSIM-CMG_111.2.1_06062022`,
  `BSIM-CMG_112.1.0_04282026`,
  `BSIM-IMG_103.0.0_20200102` (under `code/`), `BSIM_SOI_100.1.1_09152025`,
  `HiSIM_HV_2.5.1_Release_20230209`, `HiSIM_SOI_1.5.0_Release_20211008`,
  `HiSIM_SOTB_1.3.0_Release_20211116`, `L_UTSOI_102.9.0_code_package`,
  `MOSVAR140`, `PSP104.1.0_vacode`, `diode_cmc_3.0_20250714`,
  `hicumL0_v2p1p0_files`, `hicumL2_v320_files`, `mvsg_cmc_v4.0.0_official`,
  `r2_cmc_v1.0.2`, `r3_cmc_release1.1.2_2023Jun16`.
- `505p5p0_va`: the Mextram intellectual-property notice, disclaimer, and
  license in `IP_disclaimer_license.txt`, copyright NXP Semiconductors, Delft
  University of Technology, and Auburn University.
- `BSIM-SOI_4.7.0_05192025`: no separate license file; the University of
  California grant is carried in the header of `code/bsimsoi.va`, with
  `NOTICE.txt` naming the CMC standard.
- `BSIMCMG107.0.0_20130712`, `BSIMCMG108.0.0_20140822`,
  `BSIMCMG110.0.0_20160101`: no separate license file; the University of
  California grant is carried in the header of each `code/bsimcmg.va`.

- `vacode_504.12.1`: Mextram 504.12.1 (July 2016), taken from the Mextram
  site at Auburn University,
  https://www.eng.auburn.edu/~niuguof/mextram/codes/index.html, as
  `vacode_504.12.1.tar.gz` (SHA-256
  `17056001d10896dbc8c6f4816d501f8d10210186b3ae245837403253382d298c`) and
  vendored unmodified. The Mextram intellectual-property notice, disclaimer,
  and license are in `IP_NOTICE_DISCLAIMER_LICENSE`, copyright NXP
  Semiconductors, Delft University of Technology, and Auburn University. Xyce
  7.10 registers this release as Q LEVEL=504 (`bjt504va`) and LEVEL=505
  (`bjt504tva`) from a copy its `README_Xyce` records as modified (added
  junction limiting, a `tempx` temporary, `M` as an alias of `MULT`); that
  copy is not used here. The archive holds the four modules, their include
  files, and the license; it ships no test circuits or reference results.

- `PSP103p4_vacode`: PSP 103.4.0 with JUNCAP 200.5.0 (August 2016), taken
  from CEA-Leti's PSP support site,
  https://www.cea.fr/cea-tech/leti/pspsupport/Documents/PSP103p4_vacode.zip
  (SHA-256
  `1d53a9adc6792141691a8bbdeaa44896b936aefee1a57fd88bf77820532616c0`), and
  vendored unmodified under `vacode/` except that `juncap200.va` is left out:
  JUNCAP200 is already built from `PSP104.1.0_vacode`, and a second module of
  that name would rename it. The archive has no separate license file. The
  license is the Si2 Compact Model Coalition In-Code Statement that opens
  `vacode/releasenotesPSP103p4.txt` (NXP Semiconductors, Delft University of
  Technology and CEA: a royalty-free right to modify, copy and redistribute),
  copied verbatim into `LICENSE.txt`. Xyce 7.10 registers this release as M
  LEVEL=103 (`PSP103VA`) and LEVEL=1031 (`PSP103TVA`) from a copy its
  `README_Xyce` records as modified (a second correlated-noise node
  reinstated, `M` as an alias of `MULT`, unit strings); that copy is not used
  here. The archive ships no test circuits or reference results.
- `vacode_102.5.0`: PSP 102.5.0 (December 2013), taken from CEA-Leti's PSP
  support site,
  https://www.cea.fr/cea-tech/leti/pspsupport/Documents/Level%20102.5.0/vacode_102.5.0.tar.gz
  (SHA-256
  `245fdafcc9e0c7c65a38e187963b5b6c9cb6602180a43ef635e3428b6c416ea8`), and
  vendored unmodified: six modules (plain, binned and local, each with an NQS
  variant) and their include files. The intellectual-property notice,
  disclaimer and license are in `IP_NOTICE_DISCLAIMER_LICENSE` and
  `IP_ACKNOWLEDGMENT`, copyright NXP Semiconductors, Delft University of
  Technology and Arizona State University. Xyce 7.10 registers `PSP102VA` as
  M LEVEL=102 from a copy that adds `M` and the binning labels `LMIN`/`LMAX`/
  `WMIN`/`WMAX` to the unbinned module; that copy is not used here. The
  authors' reference results for this release are published beside it as
  `psp_VA_and_CMC_ref_data.tar.gz` and are not vendored.

- `BSIMCMG107.0.0_20130712`, `BSIMCMG108.0.0_20140822` and
  `BSIMCMG110.0.0_20160101`: BSIM-CMG 107.0.0 (July 2013), 108.0.0 (August
  2014) and 110.0.0 (January 2016), taken from the BSIM group at UC Berkeley,
  https://www.bsim.berkeley.edu/BSIMCMG/BSIMCMG107.0.0_20130712.tar.gz (SHA-256
  `315fa1cf6d52648b95acfdb0d9127605c51c46ebb72e073f470d2164deb7759d`),
  https://www.bsim.berkeley.edu/BSIMCMG/BSIMCMG108.0.0_20140822.tar.gz (SHA-256
  `c713997d043e79a77093bda2dc7c68713d94d8a13da712ff97909fa39e9cb1d8`) and
  https://www.bsim.berkeley.edu/BSIMCMG/BSIMCMG110.0.0_20160101.tar.gz (SHA-256
  `f2be93227964263ee7c4fa04df7c673a0a691a17d283cc406cb4d5f13afe93ca`), with
  `code/` vendored unmodified. No separate license file: the University of
  California grant is carried in the header of each `code/bsimcmg.va`. In 107
  and 108, `bsimcmg.va` is `bsimcmg_main.va` with `__RDSMOD__` defined, and
  `bsimcmg_nqsmod3.va` is a second `bsimcmg` module; both are kept as shipped
  and the compile profile builds `bsimcmg.va` alone. The archives' benchmark
  decks and HSPICE results are not vendored. Xyce 7.10 registers these
  releases as M LEVEL=107, 108 and 110 from copies its `README_Xyce` records
  as modified (instance-parameter attributes, renamed modules, 108's
  TNOIMOD=1 removed); those copies are not used here.
- `BSIM-CMG_111.2.1_06062022`: BSIM-CMG 111.2.1 (June 2022), taken from
  https://www.bsim.berkeley.edu/BSIMCMG/BSIM-CMG_111.2.1_06062022.tar.gz
  (SHA-256 `39f9ab340b24e51d56488b64b5d689314e0583d111a7c1c297ca0db2e013e266`),
  with `code/`, `LICENSE.txt` and `NOTICE.txt` vendored unmodified. Its module
  is `bsimcmg_va`, as is 112.1.0's; the compile profile names this one
  `bsimcmg111`. Xyce 7.10 registers it as M LEVEL=111.

None of these releases carries a non-commercial or no-derivatives restriction;
`tools/models/license_audit.py` enforces that across the whole `models/` tree.
