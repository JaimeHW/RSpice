# EKV2.6 Provenance

Source: https://github.com/ekv26/model

Integrated into the generated RSpice Verilog-A source tree on 2026-06-25 from
commit `137e7779c66d282113ce5802843f546869cb8c05`.

License stated by upstream README and LICENSE file: Educational Community
License, Version 2.0.

RSpice integration status:

- Generated model name: `ekv_va`.
- `.model ... NMOS LEVEL=260` and `.model ... PMOS LEVEL=260` route to this
  generated model when `veriloga-builtins` are available.
- The `L`, `W`, `M`, `NS`, `AS`, `AD`, `PS`, and `PD` declarations carry the
  Xyce 7.10 EKV compatibility attributes `type="instance"` and
  `xyceAlsoModel="yes"`. These preserve Xyce's instance-override/model-card-
  fallback semantics without changing the published EKV equations.
