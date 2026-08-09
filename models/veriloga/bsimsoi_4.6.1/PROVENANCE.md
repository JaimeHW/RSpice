# BSIM-SOI 4.6.1 Provenance

Source: https://github.com/dwarning/VA-Models

Integrated into the generated RSpice Verilog-A source tree on 2026-06-25 from
VA-Models commit `ba3d04319aae2806962848bb712fd90501488d80`.

Upstream source path: `code/bsimsoi/vacode`.

The instance parameters that the BSIM-SOI 4.6.1 manual also permits on model
cards carry Xyce's `xyceAlsoModel` metadata. The authoritative mapping is from
Xyce 7.10's `utils/ADMS/examples/BSIM-SOI_4/bsimsoi4.6.1/bsimsoi.va`; only
parameter-scope metadata is carried over, not Xyce-generated C++.

The overlap-charge ordering includes the canonical BSIM-SOI 4.6.1 correction
from Xyce commit `a6b0c3df1359a205b15d96f7b2719cee20007347`. That upstream
fix keeps `qgate` intrinsic while its electrical `ddt` contribution is stamped,
then adds `qgso + qgdo` for operating-point reporting so overlap charge is not
counted twice.

License terms are carried in the header of `vacode/bsimsoi.va`; the terms grant
perpetual, irrevocable, worldwide, non-exclusive, royalty-free use subject to
the listed attribution, redistribution, export, and non-endorsement conditions.

RSpice integration status:

- Generated model name: `bsimsoi_va`.
- Xyce `.model ... LEVEL=70` BSIM-SOI 4.6.1 selectors route to this generated
  model when the model feature is available.
