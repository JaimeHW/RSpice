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


def test_engine_run_executes_sp_directive():
    report = rspice.Engine().run(rspice.Netlist.parse(DECK))
    assert isinstance(report.s_parameters, rspice.SParameterResult)
    assert [record.kind for record in report.records] == ["sp"]
    assert not report.records[0].skipped


def test_sp_noise_flag_is_never_silently_ignored():
    deck = DECK.replace(".sp lin 3 1k 100k", ".sp lin 3 1k 100k 1")
    report = rspice.Engine().run(rspice.Netlist.parse(deck))
    assert report.s_parameters is not None
    skipped = [record for record in report.records if record.skipped]
    assert [record.kind for record in skipped] == ["sp_noise"]
    assert "port-current noise-correlation matrix" in skipped[0].reason
    assert not report.all_passed
    with pytest.raises(rspice.MeasurementError, match="port-current"):
        report.assert_passed()


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
