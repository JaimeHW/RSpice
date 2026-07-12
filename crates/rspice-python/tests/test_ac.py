"""AC analysis: name access, error discipline, sweeps, branch currents."""

import math

import numpy as np
import pytest

import rspice


AC_DATA_DECK = """
V1 in 0 AC 1
R1 in out 1k
C1 out 0 159.154943091895p
.AC DATA=pts
.DATA pts
+ FREQ
+ 1
+ 1k
+ 1meg
.ENDDATA
.END
"""

FC = 1.0 / (2 * math.pi * 1e3 * 1e-6)  # RC corner: 159.155 Hz


class TestAcBasics:
    def test_lowpass_rolloff(self, engine, rc_lowpass):
        freqs = np.logspace(0, 5, 11)
        ac = engine.run_ac(rc_lowpass, freqs.tolist())
        mag = ac.voltage_magnitude("out")
        assert mag[0] == pytest.approx(1.0, abs=1e-3)
        assert mag[-1] < 2e-3
        np.testing.assert_allclose(ac.frequencies, freqs)
        assert ac.num_frequencies == 11

    def test_accepts_numpy_array_frequencies(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, np.logspace(0, 4, 5))
        assert ac.num_frequencies == 5

    def test_magnitude_at_corner_is_3db(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [FC])
        assert ac.voltage_db("out")[0] == pytest.approx(-3.0103, abs=0.01)
        assert ac.voltage_phase_degrees("out")[0] == pytest.approx(-45.0, abs=0.1)

    def test_complex_phasors(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [FC])
        h = ac.voltage_complex("out")
        assert h.dtype == np.complex128
        expected = 1.0 / (1.0 + 1j)
        assert h[0] == pytest.approx(expected, abs=1e-3)

    def test_name_and_index_agree(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0, 100.0])
        names = ac.node_names
        assert "OUT" in [n.upper() for n in names]
        idx = [n.upper() for n in names].index("OUT") + 1
        np.testing.assert_allclose(
            ac.voltage_magnitude("out"), ac.voltage_magnitude(idx)
        )

    def test_ground_is_zero(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        assert ac.voltage_magnitude(0)[0] == 0.0
        assert ac.voltage_magnitude("gnd")[0] == 0.0


class TestAcErrorDiscipline:
    def test_out_of_range_node_raises_indexerror(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        with pytest.raises(IndexError):
            ac.voltage_magnitude(99)
        with pytest.raises(IndexError):
            ac.voltage_db(99)
        with pytest.raises(IndexError):
            ac.voltage_complex(99)

    def test_unknown_node_name_raises_keyerror(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        with pytest.raises(KeyError):
            ac.voltage_magnitude("nonexistent")

    def test_bad_freq_index_raises_indexerror(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        with pytest.raises(IndexError):
            ac.magnitude_at(999, "out")
        with pytest.raises(IndexError):
            ac.phase_at(999, "out")
        assert ac.magnitude_at(0, "out") > 0.99

    def test_empty_frequencies_raise_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac(rc_lowpass, [])

    def test_negative_frequency_raises_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac(rc_lowpass, [10.0, -5.0])

    def test_non_finite_frequency_raises_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac(rc_lowpass, [10.0, float("nan")])


class TestAcBranchCurrents:
    def test_source_current_magnitude(self, engine, rc_lowpass):
        # Far above fc the capacitor is a short: |I| ≈ 1 V / 1 kΩ.
        ac = engine.run_ac(rc_lowpass, [1e6])
        assert "V1" in [n.upper() for n in ac.branch_names]
        i_mag = ac.branch_current_magnitude("V1")
        assert i_mag[0] == pytest.approx(1e-3, rel=1e-2)
        i_cplx = ac.branch_current_complex("V1")
        assert i_cplx.dtype == np.complex128
        assert abs(i_cplx[0]) == pytest.approx(i_mag[0], rel=1e-12)

    def test_unknown_branch_raises_keyerror(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        with pytest.raises(KeyError):
            ac.branch_current_complex("V99")


class TestAcSweeps:
    def test_run_ac_sweep_decade(self, engine, rc_lowpass):
        ac = engine.run_ac_sweep(rc_lowpass, "dec", 10, 1.0, 1e5)
        freqs = ac.frequencies
        assert freqs[0] == pytest.approx(1.0)
        assert freqs[-1] == pytest.approx(1e5, rel=1e-6)
        # 5 decades x 10 points + endpoint
        assert len(freqs) == 51

    def test_ac_frequencies_matches_run_ac_sweep(self, engine, rc_lowpass):
        freqs = rspice.ac_frequencies("dec", 10, 1.0, 1e5)
        ac = engine.run_ac_sweep(rc_lowpass, "dec", 10, 1.0, 1e5)
        np.testing.assert_allclose(ac.frequencies, freqs)

    def test_linear_sweep(self, engine, rc_lowpass):
        ac = engine.run_ac_sweep(rc_lowpass, "lin", 11, 100.0, 200.0)
        np.testing.assert_allclose(ac.frequencies, np.linspace(100.0, 200.0, 11))

    def test_invalid_variation_raises_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac_sweep(rc_lowpass, "log", 10, 1.0, 1e5)

    def test_invalid_sweep_range_raises_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac_sweep(rc_lowpass, "lin", 2, 200.0, 100.0)

    def test_ac_frequencies_validates(self):
        with pytest.raises(ValueError):
            rspice.ac_frequencies("nope", 10, 1.0, 1e5)


class TestAcData:
    def test_named_data_table_frequency_grid(self, engine):
        netlist = rspice.Netlist.parse(AC_DATA_DECK)
        result = engine.run_ac_data(netlist, "PTS")
        assert result.frequencies.tolist() == [1.0, 1.0e3, 1.0e6]

    def test_engine_run_executes_ac_data_directive(self, engine):
        report = engine.run(rspice.Netlist.parse(AC_DATA_DECK))
        assert report.ac is not None
        assert report.ac.frequencies.tolist() == [1.0, 1.0e3, 1.0e6]
        assert any(record.kind == "ac_data" and not record.skipped for record in report.records)

    def test_missing_data_table_is_rejected(self, engine, rc_lowpass):
        with pytest.raises(ValueError, match="not found"):
            engine.run_ac_data(rc_lowpass, "missing")
