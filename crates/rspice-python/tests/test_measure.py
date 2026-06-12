"""The verification workflow: Engine.run, .MEAS evaluation, RunReport."""

import math

import pytest

import rspice

RC_MEAS = """* RC step with measurements, tau = 100us
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
R1 in out 1k
C1 out 0 100n
.tran 1u 1m
.meas tran t_half FIND TIME WHEN V(out)=0.5
.meas tran v_final MAX V(out)
.meas tran trise TRIG V(out) VAL=0.1 RISE=1 TARG V(out) VAL=0.9 RISE=1
.end
"""

SINE_MEAS = """* Sine statistics
V1 in 0 SIN(0 1 1k)
R1 in out 1k
R2 out 0 1k
.tran 2u 10m
.meas tran v_rms RMS V(out) FROM=0 TO=10m
.meas tran v_avg AVG V(out) FROM=0 TO=10m
.meas tran v_pp PP V(out)
.meas tran i_rms RMS I(V1) FROM=0 TO=10m
.end
"""

DC_MEAS = """* Swept divider with DC measurement
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.dc V1 0 10 0.5
.meas dc vout_max MAX V(out)
.meas dc vout_at_5 FIND V(out) WHEN V(in)=5
.end
"""

TAU = 1e-4


class TestRunTransientMeasurements:
    def test_measurements_match_rc_theory(self, engine):
        netlist = rspice.Netlist.parse(RC_MEAS)
        report = engine.run(netlist)
        report.assert_passed()
        assert report.all_passed
        assert report.num_measurements == 3

        t_half = report.measurement("t_half")
        assert t_half.passed
        assert t_half.value == pytest.approx(TAU * math.log(2.0), rel=0.02)

        v_final = report.measurement("v_final")
        assert v_final.value == pytest.approx(1.0, abs=1e-3)

        trise = report.measurement("trise")
        assert trise.value == pytest.approx(TAU * math.log(9.0), rel=0.02)

    def test_report_carries_results(self, engine):
        netlist = rspice.Netlist.parse(RC_MEAS)
        report = engine.run(netlist)
        assert report.tran is not None
        assert report.tran.num_points > 100
        assert report.op is None
        assert report.ac is None
        assert report.analyses_run == ["tran"]
        assert report.skipped == []
        assert "all_passed=true" in repr(report).lower()

    def test_measurement_lookup_case_insensitive(self, engine):
        netlist = rspice.Netlist.parse(RC_MEAS)
        report = engine.run(netlist)
        assert report.measurement("T_HALF") is not None
        assert report.measurement("nope") is None

    def test_float_conversion(self, engine):
        netlist = rspice.Netlist.parse(RC_MEAS)
        report = engine.run(netlist)
        assert float(report.measurement("v_final")) == pytest.approx(1.0, abs=1e-3)


class TestSineMeasurements:
    def test_rms_avg_pp_and_current(self, engine):
        netlist = rspice.Netlist.parse(SINE_MEAS)
        report = engine.run(netlist)
        report.assert_passed()

        # V(out) is a 0.5 V amplitude sine.
        assert report.measurement("v_rms").value == pytest.approx(
            0.5 / math.sqrt(2), rel=0.02
        )
        assert abs(report.measurement("v_avg").value) < 5e-3
        assert report.measurement("v_pp").value == pytest.approx(1.0, rel=0.02)
        # Source current: 1 V amplitude over 2 kΩ.
        assert report.measurement("i_rms").value == pytest.approx(
            (1.0 / 2000.0) / math.sqrt(2), rel=0.02
        )


class TestDcMeasurements:
    def test_dc_sweep_measurements(self, engine):
        netlist = rspice.Netlist.parse(DC_MEAS)
        report = engine.run(netlist)
        report.assert_passed()
        assert report.dc is not None
        assert report.measurement("vout_max").value == pytest.approx(5.0, abs=1e-6)
        assert report.measurement("vout_at_5").value == pytest.approx(2.5, abs=1e-6)


class TestVerificationDiscipline:
    def test_assert_passed_raises_without_measurements(self, engine, divider):
        netlist = rspice.Netlist.parse(
            "* op only\nV1 in 0 10\nR1 in out 1k\nR2 out 0 1k\n.op\n.end"
        )
        report = engine.run(netlist)
        assert report.num_measurements == 0
        assert report.all_passed  # vacuously
        with pytest.raises(rspice.MeasurementError):
            report.assert_passed()

    def test_unrunnable_measurement_fails_loudly(self, engine):
        # A TRAN measurement with no .tran in the deck must FAIL, not vanish.
        netlist = rspice.Netlist.parse(
            """* op-only deck with orphan tran measurement
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.op
.meas tran vmax MAX V(out)
.end
"""
        )
        report = engine.run(netlist)
        assert report.num_measurements == 1
        m = report.measurements[0]
        assert not m.passed
        assert "tran" in m.error.lower()
        assert not report.all_passed
        assert len(report.failures) == 1
        with pytest.raises(rspice.MeasurementError) as exc_info:
            report.assert_passed()
        assert "vmax" in str(exc_info.value).lower()

    def test_failed_when_condition_reports_error(self, engine):
        # V(out) never reaches 5 V in this deck: FIND ... WHEN must fail.
        netlist = rspice.Netlist.parse(
            """* RC that never crosses 5 V
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
R1 in out 1k
C1 out 0 100n
.tran 1u 1m
.meas tran t_cross FIND V(out) WHEN V(out)=5
.end
"""
        )
        report = engine.run(netlist)
        m = report.measurement("t_cross")
        assert not m.passed
        assert m.value is None
        assert m.error
        with pytest.raises(rspice.MeasurementError):
            report.assert_passed()
        with pytest.raises(ValueError):
            float(m)

    def test_unsupported_directives_are_recorded_not_dropped(self, engine):
        netlist = rspice.Netlist.parse(
            """* deck with an unsupported directive
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.op
.temp 27 50 80
.end
"""
        )
        report = engine.run(netlist)
        assert report.analyses_run == ["op"]
        skipped = report.skipped
        assert len(skipped) == 1
        assert skipped[0].kind == "temp"
        assert skipped[0].reason


class TestRunMultipleAnalyses:
    def test_op_ac_tran_in_one_deck(self, engine):
        netlist = rspice.Netlist.parse(
            """* combined deck
V1 in 0 AC 1 SIN(0 1 1k)
R1 in out 1k
C1 out 0 1u
.op
.ac dec 10 1 100k
.tran 10u 2m
.end
"""
        )
        report = engine.run(netlist)
        assert report.analyses_run == ["op", "ac", "tran"]
        assert report.op is not None
        assert report.ac is not None
        assert report.tran is not None
        assert report.ac.num_frequencies == 51

    def test_four_directive(self, engine):
        netlist = rspice.Netlist.parse(
            """* THD deck
V1 in 0 SIN(0 1 1k)
R1 in out 1k
R2 out 0 1k
.tran 2u 10m
.four 1k V(out)
.end
"""
        )
        report = engine.run(netlist)
        assert "four" in report.analyses_run
        assert len(report.fourier) == 1
        assert report.fourier[0].thd_percent < 1.0


class TestMeasureStandalone:
    def test_measure_against_existing_tran(self, engine):
        netlist = rspice.Netlist.parse(RC_MEAS)
        tran = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
        measurements = engine.measure(netlist, tran)
        assert len(measurements) == 3
        by_name = {m.name.lower(): m for m in measurements}
        assert by_name["v_final"].value == pytest.approx(1.0, abs=1e-3)

    def test_measure_against_existing_sweep(self, engine):
        netlist = rspice.Netlist.parse(DC_MEAS)
        sweep = engine.run_dc_sweep(netlist, "V1", 0.0, 10.0, 0.5)
        measurements = engine.measure(netlist, sweep)
        by_name = {m.name.lower(): m for m in measurements}
        assert by_name["vout_max"].value == pytest.approx(5.0, abs=1e-6)

    def test_measure_rejects_wrong_type(self, engine, divider):
        with pytest.raises(TypeError):
            engine.measure(divider, "not a result")
