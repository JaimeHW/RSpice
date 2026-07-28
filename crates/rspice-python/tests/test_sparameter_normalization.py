"""Closed-form checks on the N-port S-parameter path.

The conversion itself is unit-tested in
`rspice_core::analysis::advanced::s_param::network`. What these add is the
end-to-end path: deck, AC solve, admittance extraction, normalization. A sign
error on the extracted branch current or a mis-ordered port list would pass
core's tests and fail here.

Coverage the 50-ohm cases in `test_s_parameters.py` cannot give:

- **Non-uniform reference impedances.** With every port at 50 ohms the
  `D^-1 (I - ZY)(I + ZY)^-1 D` normalization is indistinguishable from dropping
  `D` entirely, because every `sqrt(Z0_j / Z0_i)` factor is 1.
- **Complex, frequency-dependent Y**, which a purely resistive network never
  produces.
"""

from __future__ import annotations

import numpy as np
import pytest

import rspice

# A bare series resistor between two ports with unequal reference impedances.
# Purely resistive and reciprocal, so the S-matrix has an exact closed form.
SERIES_R = 50.0
Z0_INPUT = 75.0
Z0_OUTPUT = 50.0
ASYMMETRIC_Z0_DECK = f"""* Series resistor, unequal reference impedances
V1 p1 0 DC 0 AC 1 portnum 1 z0 {Z0_INPUT:g}
R1 p1 p2 {SERIES_R:g}
V2 p2 0 DC 0 AC 0 portnum 2 z0 {Z0_OUTPUT:g}
.end
"""

# Three ports, non-uniform impedances, and a reactive path so the admittance
# matrix is complex and frequency-dependent. L1 reaches port 3 through R4
# rather than bridging two port sources directly, which would short two ideal
# voltage sources and make the DC operating point singular.
THREE_PORT_DECK = """* Three-port, non-uniform reference impedances
V1 p1 0 DC 0 AC 1 portnum 1 z0 50
V2 p2 0 DC 0 AC 0 portnum 2 z0 75
V3 p3 0 DC 0 AC 0 portnum 3 z0 50
R1 p1 mid 25
R2 p2 mid 40
R3 p3 mid 60
C1 mid 0 1n
L1 p1 lx 1u
R4 lx p3 10
.end
"""

THREE_PORT_FREQUENCIES = [1.0e6, 3.16e6, 1.0e7, 3.16e7, 1.0e8]


def test_reference_impedance_scaling_is_load_bearing() -> None:
    """Pins the normalization convention against a derived closed form.

    For a series resistance R bridging two ports with real reference impedances
    z1 and z2, write a = z1/R, b = z2/R, c = sqrt(z1*z2)/R. From
    S = (I - DYD)(I + DYD)^-1 with Y = [[1, -1], [-1, 1]]/R and
    D = diag(sqrt(z1), sqrt(z2)), and using c**2 = a*b:

        S11 = (1 + b - a) / (1 + a + b)
        S22 = (1 + a - b) / (1 + a + b)
        S12 = S21 = 2c / (1 + a + b)

    The off-diagonal term is the load-bearing one. Dropping D -- computing
    (I - ZY)(I + ZY)^-1 and stopping -- yields 2b/(1 + a + b) instead, which
    for these values differs in the first decimal place.
    """
    result = rspice.Engine().run_s_parameters(
        rspice.Netlist.parse(ASYMMETRIC_Z0_DECK), [1.0e6]
    )

    a = Z0_INPUT / SERIES_R
    b = Z0_OUTPUT / SERIES_R
    c = np.sqrt(Z0_INPUT * Z0_OUTPUT) / SERIES_R
    total = 1.0 + a + b

    assert result.reference_impedances.tolist() == [Z0_INPUT, Z0_OUTPUT]
    assert result.s(1, 1)[0] == pytest.approx((1.0 + b - a) / total, abs=1e-12)
    assert result.s(2, 2)[0] == pytest.approx((1.0 + a - b) / total, abs=1e-12)
    assert result.s(2, 1)[0] == pytest.approx(2.0 * c / total, abs=1e-12)
    assert result.s(1, 2)[0] == pytest.approx(2.0 * c / total, abs=1e-12)

    # The value an unnormalized implementation would produce, stated
    # explicitly so the distinction cannot be lost in a future edit.
    unnormalized_s21 = 2.0 * b / total
    assert abs(2.0 * c / total - unnormalized_s21) > 0.1
    assert result.s(2, 1)[0] != pytest.approx(unnormalized_s21, abs=1e-6)


def test_reciprocal_network_has_symmetric_s_matrix() -> None:
    """Reciprocity holds across unequal reference impedances.

    Under the power-wave convention both S and its transpose are functions of
    DYD, which is symmetric whenever Y is. Asymmetry is therefore *not* a valid
    signal that the D scaling was applied -- a natural but wrong way to test
    this, which is why the closed form above is used instead.
    """
    result = rspice.Engine().run_s_parameters(
        rspice.Netlist.parse(THREE_PORT_DECK), THREE_PORT_FREQUENCIES
    )

    assert result.reference_impedances.tolist() == [50.0, 75.0, 50.0]
    for output_port in range(1, 4):
        for input_port in range(output_port + 1, 4):
            assert np.allclose(
                result.s(output_port, input_port),
                result.s(input_port, output_port),
                rtol=1e-12,
                atol=1e-15,
            ), f"S{output_port}{input_port} != S{input_port}{output_port}"
