import numpy as np
import pytest

import rspice


DECK = """
V1 p1 0 DC 0 AC 1 portnum 1 z0 50
R1 p1 p2 50
V2 p2 0 DC 0 AC 0 portnum 2 z0 50
.sp lin 3 1k 100k
.end
"""


def test_two_port_series_resistor_matches_closed_form():
    netlist = rspice.Netlist.parse(DECK)
    result = rspice.Engine().run_s_parameters(netlist, [1.0e3, 1.0e5])

    assert isinstance(result, rspice.SParameterResult)
    assert result.num_ports == 2
    assert result.num_points == 2
    assert result.port_names == ["V1", "V2"]
    assert result.reference_impedances.tolist() == [50.0, 50.0]
    assert np.allclose(result.s(1, 1), 1.0 / 3.0, rtol=0.0, atol=1e-12)
    assert np.allclose(result.s(2, 1), 2.0 / 3.0, rtol=0.0, atol=1e-12)
    assert np.allclose(result.s(1, 2), result.s(2, 1), rtol=0.0, atol=1e-12)
    assert np.allclose(result.s(2, 2), result.s(1, 1), rtol=0.0, atol=1e-12)
    assert result.s(1, 1).dtype == np.complex128
    with pytest.raises(IndexError):
        result.s(0, 1)
    with pytest.raises(IndexError):
        result.s(1, 3)
    assert not result.has_noise
    assert result.noise_temperature is None
    assert result.noise_resistance is None
    with pytest.raises(ValueError, match="do_noise=True"):
        result.cy(1, 1)


def test_engine_run_executes_sp_directive():
    report = rspice.Engine().run(rspice.Netlist.parse(DECK))
    assert isinstance(report.s_parameters, rspice.SParameterResult)
    assert [record.kind for record in report.records] == ["sp"]
    assert not report.records[0].skipped


def test_sp_noise_directive_computes_cy_and_two_port_noise_parameters():
    deck = DECK.replace(".sp lin 3 1k 100k", ".sp lin 3 1k 100k 1")
    report = rspice.Engine().run(rspice.Netlist.parse(deck))
    result = report.s_parameters
    assert result is not None
    assert [record.kind for record in report.records] == ["sp", "sp_noise"]
    assert not any(record.skipped for record in report.records)
    _assert_series_resistor_noise(result, 50.0)


def test_direct_sp_noise_api_computes_standard_noise_outputs():
    result = rspice.Engine().run_s_parameters(
        rspice.Netlist.parse(DECK), [1.0e3, 1.0e5], do_noise=True
    )
    _assert_series_resistor_noise(result, 50.0)


def _assert_series_resistor_noise(result, resistance):
    assert result.has_noise
    assert result.has_two_port_noise_parameters
    assert result.noise_temperature == pytest.approx(300.15, rel=0.0, abs=1e-12)
    expected_cy = 4.0 * 1.380649e-23 * result.noise_temperature / resistance
    assert np.allclose(result.cy(1, 1), expected_cy, rtol=1e-11, atol=0.0)
    assert np.allclose(result.cy(2, 2), expected_cy, rtol=1e-11, atol=0.0)
    assert np.allclose(result.cy(1, 2), -expected_cy, rtol=1e-11, atol=0.0)
    assert np.array_equal(result.cy(2, 1), np.conjugate(result.cy(1, 2)))
    assert np.allclose(result.noise_resistance, resistance, rtol=1e-11, atol=0.0)
    assert np.allclose(result.noise_factor, 2.0, rtol=1e-11, atol=0.0)
    assert np.allclose(result.minimum_noise_factor, 1.0, rtol=0.0, atol=1e-11)
    assert np.allclose(result.noise_figure_db, 10.0 * np.log10(2.0), atol=1e-11)
    assert np.allclose(result.minimum_noise_figure_db, 0.0, atol=1e-11)
    assert np.allclose(result.optimum_source_reflection, 1.0 + 0.0j, atol=1e-11)
    assert np.all(result.noise_parameters_valid)
    assert np.array_equal(result.rn, result.noise_resistance)
    assert np.array_equal(result.nf, result.noise_figure_db)
    assert np.array_equal(result.nfmin, result.minimum_noise_figure_db)
    assert np.array_equal(result.sopt, result.optimum_source_reflection)


@pytest.mark.parametrize(
    "deck, message",
    [
        ("V1 out 0 AC 1\nR1 out 0 50\n.end", "annotated"),
        (
            "V1 a 0 AC 1 portnum 2 z0 50\nR1 a 0 50\n.end",
            "dense and unique",
        ),
    ],
)
def test_invalid_port_definitions_are_rejected(deck, message):
    with pytest.raises(ValueError, match=message):
        rspice.Engine().run_s_parameters(rspice.Netlist.parse(deck), [1.0e3])


def test_nonpositive_port_impedance_is_rejected_during_parsing():
    with pytest.raises(rspice.ParseError, match="positive impedance"):
        rspice.Netlist.parse("V1 a 0 AC 1 portnum 1 z0 -1\nR1 a 0 50\n.end")


def test_sparameter_result_has_public_module_identity():
    assert rspice.SParameterResult.__module__ == "rspice"
