"""Bit-exact reference output for the S-parameter path.

The N-port Y-to-S conversion and the two-port noise-parameter derivation are
moving out of the binding layer into `rspice_core`. The formula is not changing,
only its address, so the post-move results must be *identical*, not merely
close. These goldens are stored as `float.hex()` strings and compared with `==`
for exactly that reason: a tolerance-based check would hide a normalization
that silently reverted to the uniform-impedance form.

Deliberate coverage that the closed-form tests in `test_s_parameters.py` cannot
provide:

- **Non-uniform reference impedances.** With every port at 50 ohms the
  `D^-1 (I - ZY)(I + ZY)^-1 D` normalization is indistinguishable from dropping
  `D` entirely, because every `sqrt(Z0_j / Z0_i)` scale factor is 1. The 3-port
  deck uses 50/75/50 so the scaling is load-bearing.
- **Complex, frequency-dependent Y.** A purely resistive network gives a real
  admittance matrix and never exercises complex pivoting in the inverse. Both
  decks below carry reactive elements.
- **A noise reference impedance that is not 50 ohms**, so the `y0` term in the
  noise derivation cannot be confused with a constant.

Regenerating after a *deliberate* numeric change:

    RSPICE_UPDATE_SPARAM_GOLDEN=1 python -m pytest tests/test_sparameter_golden.py

Any diff in the resulting file is a physics change and must be justified as one.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

import numpy as np
import pytest

import rspice

GOLDEN = Path(__file__).with_name("sparameter_golden.json")

# Three ports, non-uniform reference impedances, and a reactive path so the
# admittance matrix is complex and frequency-dependent. L1 reaches port 3
# through R4 rather than bridging two port sources directly, which would short
# two ideal voltage sources together and make the DC operating point singular.
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

# Two ports with a 75-ohm input reference so the noise derivation's `y0` term
# is exercised at something other than the 50-ohm default.
NOISE_DECK = """* Two-port series resistor, 75-ohm input reference
V1 p1 0 DC 0 AC 1 portnum 1 z0 75
R1 p1 p2 50
C1 p1 0 10p
V2 p2 0 DC 0 AC 0 portnum 2 z0 50
.end
"""

NOISE_FREQUENCIES = [1.0e6, 1.0e7, 1.0e8]

# A bare series resistor between two ports with unequal reference impedances.
# Purely resistive and reciprocal, so the S-matrix has an exact closed form
# (derived in `test_reference_impedance_scaling_is_load_bearing`) that pins the
# normalization convention rather than merely the implementation's own output.
SERIES_R = 50.0
Z0_INPUT = 75.0
Z0_OUTPUT = 50.0
ASYMMETRIC_Z0_DECK = f"""* Series resistor, unequal reference impedances
V1 p1 0 DC 0 AC 1 portnum 1 z0 {Z0_INPUT:g}
R1 p1 p2 {SERIES_R:g}
V2 p2 0 DC 0 AC 0 portnum 2 z0 {Z0_OUTPUT:g}
.end
"""


def _hex_array(values: Any) -> list[str]:
    """Encode floats losslessly, including NaN and infinities."""
    array = np.asarray(values)
    if np.iscomplexobj(array):
        return [f"{value.real.hex()}|{value.imag.hex()}" for value in array.ravel()]
    return [float(value).hex() for value in array.ravel()]


def _capture_matrix(result: rspice.SParameterResult) -> dict[str, list[str]]:
    matrix: dict[str, list[str]] = {}
    for output_port in range(1, result.num_ports + 1):
        for input_port in range(1, result.num_ports + 1):
            matrix[f"s{output_port}{input_port}"] = _hex_array(
                result.s(output_port, input_port)
            )
    return matrix


def _capture_optional(result: rspice.SParameterResult, name: str) -> list[str] | None:
    values = getattr(result, name)
    if values is None:
        return None
    if values.dtype == np.bool_:
        return [bool(value) for value in values.ravel()]
    return _hex_array(values)


NOISE_SERIES = (
    "noise_resistance",
    "noise_factor",
    "noise_figure_db",
    "minimum_noise_factor",
    "minimum_noise_figure_db",
    "optimum_source_reflection",
    "noise_parameters_valid",
)

# Routed through core's SMatrix bridge rather than the binding's own maths.
# Captured so the consolidation cannot disturb them by accident.
TWO_PORT_DERIVED = (
    "k_factor",
    "mu_factor",
    "mu_prime",
    "delta",
    "unconditionally_stable",
    "max_available_gain_db",
    "max_stable_gain_db",
    "mason_unilateral_gain_db",
    "transducer_gain_db",
    "reverse_isolation_db",
)


def _capture(result: rspice.SParameterResult) -> dict[str, Any]:
    captured: dict[str, Any] = {
        "num_ports": result.num_ports,
        "num_points": result.num_points,
        "port_names": list(result.port_names),
        "reference_impedances": _hex_array(result.reference_impedances),
        "frequencies": _hex_array(result.frequencies),
        "matrix": _capture_matrix(result),
        "has_noise": result.has_noise,
        "has_two_port_noise_parameters": result.has_two_port_noise_parameters,
        "has_two_port_stability": result.has_two_port_stability,
    }
    if result.has_noise:
        captured["noise_temperature"] = float(result.noise_temperature).hex()
        captured["cy"] = {
            f"cy{output}{inp}": _hex_array(result.cy(output, inp))
            for output in range(1, result.num_ports + 1)
            for inp in range(1, result.num_ports + 1)
        }
        for name in NOISE_SERIES:
            captured[name] = _capture_optional(result, name)
    for name in TWO_PORT_DERIVED:
        captured[name] = _capture_optional(result, name)
    return captured


def build_golden() -> dict[str, Any]:
    engine = rspice.Engine()
    three_port = engine.run_s_parameters(
        rspice.Netlist.parse(THREE_PORT_DECK), THREE_PORT_FREQUENCIES
    )
    noise = engine.run_s_parameters(
        rspice.Netlist.parse(NOISE_DECK), NOISE_FREQUENCIES, do_noise=True
    )
    asymmetric = engine.run_s_parameters(
        rspice.Netlist.parse(ASYMMETRIC_Z0_DECK), [1.0e6]
    )
    return {
        "three_port_non_uniform_z0": _capture(three_port),
        "two_port_noise_75_ohm_input": _capture(noise),
        "two_port_asymmetric_z0_closed_form": _capture(asymmetric),
    }


def test_sparameter_results_are_bit_identical_to_golden() -> None:
    actual = build_golden()

    if os.environ.get("RSPICE_UPDATE_SPARAM_GOLDEN"):
        GOLDEN.write_text(json.dumps(actual, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        pytest.skip("sparameter_golden.json regenerated; review the diff before committing")

    assert GOLDEN.exists(), (
        "tests/sparameter_golden.json is missing; regenerate with "
        "RSPICE_UPDATE_SPARAM_GOLDEN=1 python -m pytest tests/test_sparameter_golden.py"
    )
    expected = json.loads(GOLDEN.read_text(encoding="utf-8"))

    for case in expected:
        assert case in actual, f"golden case {case} no longer produced"
        for key, expected_value in expected[case].items():
            assert actual[case][key] == expected_value, (
                f"{case}.{key} changed; the S-parameter path is not bit-identical"
            )


def test_reference_impedance_scaling_is_load_bearing() -> None:
    """Guards the guard, against a closed form rather than the code's own output.

    A golden captured from the implementation cannot tell you the
    implementation is right — only that it has not changed. This pins the
    convention itself.

    For a series resistance R bridging two ports with real reference
    impedances z1 and z2, write a = z1/R, b = z2/R, c = sqrt(z1*z2)/R. From
    S = (I - DYD)(I + DYD)^-1 with Y = [[1, -1], [-1, 1]]/R and
    D = diag(sqrt(z1), sqrt(z2)), and using c**2 = a*b:

        S11 = (1 + b - a) / (1 + a + b)
        S22 = (1 + a - b) / (1 + a + b)
        S12 = S21 = 2c / (1 + a + b)

    The off-diagonal term is the load-bearing one. Dropping D — that is,
    computing (I - ZY)(I + ZY)^-1 and stopping — yields 2b/(1 + a + b)
    instead, which for these values differs in the first decimal place. Note
    that S is symmetric here even though z1 != z2: under the power-wave
    convention a reciprocal network always has a symmetric S-matrix, because
    S and its transpose are both functions of DYD, which is symmetric whenever
    Y is. Asymmetry is therefore *not* a valid signal that D was applied.
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
    """Reciprocity holds across unequal reference impedances (see above)."""
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
