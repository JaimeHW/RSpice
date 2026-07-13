"""Third-order harmonic and two-tone Volterra distortion contracts."""

import math

import numpy as np
import pytest

import rspice


K_BOLTZMANN = 1.380649e-23
Q_ELECTRON = 1.602176634e-19
TEMP_REFERENCE = 300.15
VT = TEMP_REFERENCE * K_BOLTZMANN / Q_ELECTRON


HARMONIC_DIODE = """* harmonic distortion oracle
V1 out 0 DC 0.5 DISTOF1 1m 0
D1 out 0 DM
.model DM D(IS=1e-12 N=1 CJO=0 TT=0)
.end
"""


TWO_TONE_DIODE = """* two-tone distortion oracle
V1 out 0 DC 0.5 DISTOF1 1m 0 DISTOF2 2m 0
D1 out 0 DM
.model DM D(IS=1e-12 N=1 CJO=0 TT=0)
.end
"""


def _diode_current() -> float:
    return 1e-12 * math.exp(0.5 / VT)


def test_harmonic_products_are_actual_phasors(engine) -> None:
    result = engine.run_distortion(
        rspice.Netlist.parse(HARMONIC_DIODE), [1e3, 2e3]
    )

    assert isinstance(result, rspice.DistortionResult)
    assert not result.is_two_tone
    assert result.f2_frequency is None
    assert result.f2_over_f1 is None
    assert result.available_products == ["2f1", "3f1"]
    np.testing.assert_allclose(result.f1_frequencies, [1e3, 2e3])
    np.testing.assert_allclose(result.fundamental_f1.frequencies, [1e3, 2e3])
    np.testing.assert_allclose(
        result.fundamental_f1.voltage_complex("out"), [1e-3, 1e-3]
    )

    second = result.product("2f1")
    third = result.product("HD3")
    np.testing.assert_allclose(second.frequencies, [2e3, 4e3])
    np.testing.assert_allclose(third.frequencies, [3e3, 6e3])

    expected_second = _diode_current() * 1e-6 / (4.0 * VT**2)
    expected_third = _diode_current() * 1e-9 / (24.0 * VT**3)
    np.testing.assert_allclose(
        np.abs(second.branch_current_complex("V1")),
        expected_second,
        rtol=2e-5,
    )
    np.testing.assert_allclose(
        np.abs(third.branch_current_complex("V1")),
        expected_third,
        rtol=2e-3,
    )

    fundamental_current = _diode_current() * 1e-3 / VT
    np.testing.assert_allclose(
        result.branch_current_ratio("2f1", "V1"),
        expected_second / fundamental_current,
        rtol=2e-5,
    )
    np.testing.assert_allclose(
        result.branch_current_db_relative("2f1", "V1"),
        20.0 * np.log10(expected_second / fundamental_current),
        rtol=2e-5,
    )


def test_two_tone_products_and_fixed_f2_contract(engine) -> None:
    result = engine.run_distortion(
        rspice.Netlist.parse(TWO_TONE_DIODE), [1e3, 2e3], f2_over_f1=0.9
    )

    assert result.is_two_tone
    assert result.f2_frequency == pytest.approx(900.0)
    assert result.f2_over_f1 == pytest.approx(0.9)
    assert result.available_products == ["f1+f2", "f1-f2", "2f1-f2"]
    assert result.fundamental_f2 is not None
    np.testing.assert_allclose(result.fundamental_f2.frequencies, [900.0, 900.0])
    np.testing.assert_allclose(result.product("sum").frequencies, [1900.0, 2900.0])
    np.testing.assert_allclose(
        result.product("difference").frequencies, [100.0, 1100.0]
    )
    np.testing.assert_allclose(result.product("im3").frequencies, [1100.0, 3100.0])

    expected_im2 = _diode_current() * 1e-3 * 2e-3 / (2.0 * VT**2)
    expected_im3 = _diode_current() * (1e-3) ** 2 * 2e-3 / (8.0 * VT**3)
    for product in ("f1+f2", "f1-f2"):
        np.testing.assert_allclose(
            np.abs(result.product(product).branch_current_complex("V1")),
            expected_im2,
            rtol=2e-5,
        )
    np.testing.assert_allclose(
        np.abs(result.product("2f1-f2").branch_current_complex("V1")),
        expected_im3,
        rtol=2e-3,
    )


def test_deck_run_executes_disto_and_returns_result(engine) -> None:
    deck = HARMONIC_DIODE.replace(".end", ".disto lin 3 1k 2k\n.end")
    report = engine.run(rspice.Netlist.parse(deck))

    assert report.distortion is not None
    assert report.distortion.num_points == 3
    assert [record.kind for record in report.records] == ["disto"]
    assert not report.records[0].skipped
    assert report.skipped == []


def test_sweep_api_and_result_error_discipline(engine) -> None:
    netlist = rspice.Netlist.parse(HARMONIC_DIODE)
    result = engine.run_distortion_sweep(netlist, "lin", 3, 1e3, 2e3)
    np.testing.assert_allclose(result.f1_frequencies, [1e3, 1.5e3, 2e3])

    with pytest.raises(ValueError, match="not available"):
        result.product("f1+f2")
    with pytest.raises(ValueError, match="unknown distortion product"):
        result.product("not-a-product")
    with pytest.raises(KeyError):
        result.voltage_ratio("2f1", "missing")
    with pytest.raises(KeyError):
        result.branch_current_ratio("2f1", "missing")
    with pytest.raises(IndexError):
        result.voltage_ratio("2f1", 999)


@pytest.mark.parametrize(
    ("frequencies", "ratio"),
    [([], None), ([0.0], None), ([float("nan")], None), ([1e3], 0.0), ([1e3], 1.0)],
)
def test_invalid_arguments_raise_value_error(engine, frequencies, ratio) -> None:
    with pytest.raises(ValueError):
        engine.run_distortion(
            rspice.Netlist.parse(HARMONIC_DIODE), frequencies, f2_over_f1=ratio
        )


def test_two_tone_requires_distof2_and_positive_difference_frequencies(engine) -> None:
    with pytest.raises(rspice.SimulationError, match="DISTOF2"):
        engine.run_distortion(
            rspice.Netlist.parse(HARMONIC_DIODE), [1e3], f2_over_f1=0.9
        )
    with pytest.raises(ValueError, match="greater than the fixed F2"):
        engine.run_distortion(
            rspice.Netlist.parse(TWO_TONE_DIODE),
            [1e3, 500.0],
            f2_over_f1=0.9,
        )


def test_numpy_exports_are_owned_copies(engine) -> None:
    result = engine.run_distortion(rspice.Netlist.parse(HARMONIC_DIODE), [1e3])
    first = result.f1_frequencies
    assert first.flags.owndata
    first[0] = 123.0
    assert result.f1_frequencies[0] == pytest.approx(1e3)
