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
  `BSIM-BULK107.2.1_02112025`, `BSIM-CMG_112.1.0_04282026`,
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

None of these releases carries a non-commercial or no-derivatives restriction;
`tools/models/license_audit.py` enforces that across the whole `models/` tree.
