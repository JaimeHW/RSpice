"""Periodic operating-point reuse and envelope continuation.

PAC and PNoise both linearize around a PSS solution, and a transient can
start from a converged orbit instead of integrating the settling interval
again. Both shortcuts have to produce the same answers as the direct calls.
"""

import numpy as np
import pytest

import rspice

# A driven RC network is enough to exercise the plumbing: the periodic
# solution is analytic, so a wrong continuation shows up immediately.
DRIVEN = """* Driven RC
V1 in 0 SIN(0 1 1meg)
R1 in out 1k
C1 out 0 100p
.end
"""

FUNDAMENTAL = 1e6


@pytest.fixture(scope="module")
def driven():
    return rspice.Netlist.parse(DRIVEN)


class TestOperatingPointReuse:
    def test_operating_point_reports_the_converged_orbit(self, engine, driven):
        operating_point = engine.run_pss_operating_point(driven, FUNDAMENTAL)

        assert operating_point.frequency == pytest.approx(FUNDAMENTAL, rel=1e-6)
        assert operating_point.period == pytest.approx(1.0 / FUNDAMENTAL, rel=1e-6)
        assert operating_point.iterations >= 1
        assert repr(operating_point).startswith("PssOperatingPoint(")

    def test_pac_reuse_matches_a_standalone_solve(self, engine, driven):
        direct = engine.run_pac(driven, FUNDAMENTAL, 1e3, 1e5, 5, "V1", "out")
        operating_point = engine.run_pss_operating_point(driven, FUNDAMENTAL)
        reused = engine.run_pac(
            driven, FUNDAMENTAL, 1e3, 1e5, 5, "V1", "out", pss=operating_point
        )

        np.testing.assert_allclose(reused.frequencies, direct.frequencies)
        np.testing.assert_allclose(
            reused.voltage("out", 0), direct.voltage("out", 0), rtol=1e-6, atol=1e-15
        )

    def test_pnoise_reuse_matches_a_standalone_solve(self, engine, driven):
        offsets = [1e3, 1e4]
        direct = engine.run_pnoise(driven, FUNDAMENTAL, offsets, "out")
        operating_point = engine.run_pss_operating_point(driven, FUNDAMENTAL)
        reused = engine.run_pnoise(
            driven, FUNDAMENTAL, offsets, "out", pss=operating_point
        )

        np.testing.assert_allclose(reused.frequencies, direct.frequencies)
        np.testing.assert_allclose(
            reused.output_noise, direct.output_noise, rtol=1e-6, atol=1e-30
        )

    def test_one_operating_point_serves_many_analyses(self, engine, driven):
        operating_point = engine.run_pss_operating_point(driven, FUNDAMENTAL)
        first = engine.run_pac(
            driven, FUNDAMENTAL, 1e3, 1e4, 3, "V1", "out", pss=operating_point
        )
        second = engine.run_pnoise(
            driven, FUNDAMENTAL, [1e3], "out", pss=operating_point
        )
        assert len(first.frequencies) == 3
        assert len(second.frequencies) == 1


class TestPssContinuation:
    def test_continuation_returns_a_result_and_a_state(self, engine, driven):
        result, state = engine.run_pss_continuation(driven, FUNDAMENTAL)

        assert result.frequency == pytest.approx(FUNDAMENTAL, rel=1e-6)
        assert state.period == pytest.approx(1.0 / FUNDAMENTAL, rel=1e-6)
        assert np.isfinite(state.time_origin)
        assert repr(state).startswith("PssContinuationState(")

    def test_a_transient_continues_from_the_orbit(self, engine, driven):
        _, state = engine.run_pss_continuation(driven, FUNDAMENTAL)
        duration = 5.0 / FUNDAMENTAL
        tran, checkpoint = engine.run_tran_from_pss(
            driven, state, duration, max_step=duration / 200
        )

        assert tran.num_points > 10
        assert tran.stop_time == pytest.approx(
            state.time_origin + duration, rel=1e-6
        )
        assert checkpoint.time == pytest.approx(tran.stop_time, rel=1e-6)
        # The continued run starts on the orbit, so it is already periodic:
        # no settling transient remains to decay.
        waveform = tran.voltage_waveform("out")
        first_half = np.max(np.abs(waveform[: len(waveform) // 2]))
        second_half = np.max(np.abs(waveform[len(waveform) // 2 :]))
        assert second_half == pytest.approx(first_half, rel=0.05)

    def test_frozen_sources_are_accepted(self, engine, driven):
        _, state = engine.run_pss_continuation(
            driven, FUNDAMENTAL, frozen_sources=["V1"]
        )
        assert state.period == pytest.approx(1.0 / FUNDAMENTAL, rel=1e-6)

    @pytest.mark.parametrize("duration", [0.0, -1.0, float("nan"), float("inf")])
    def test_a_bad_duration_is_rejected(self, engine, driven, duration):
        _, state = engine.run_pss_continuation(driven, FUNDAMENTAL)
        with pytest.raises(ValueError, match="duration"):
            engine.run_tran_from_pss(driven, state, duration)

    def test_a_bad_max_step_is_rejected(self, engine, driven):
        _, state = engine.run_pss_continuation(driven, FUNDAMENTAL)
        with pytest.raises(ValueError, match="max_step"):
            engine.run_tran_from_pss(driven, state, 1e-6, max_step=0.0)


class TestHbEnvelope:
    def test_envelope_returns_a_result_and_a_state(self, engine, driven):
        result, state = engine.run_hb_envelope(driven, FUNDAMENTAL, harmonics=5)

        assert result.fundamental_frequency == pytest.approx(FUNDAMENTAL)
        assert state.fundamental_frequency == pytest.approx(FUNDAMENTAL)
        assert state.frozen_sources == []
        assert repr(state).startswith("HbEnvelopeState(")

    def test_frozen_sources_are_retained_on_the_state(self, engine, driven):
        _, state = engine.run_hb_envelope(
            driven, FUNDAMENTAL, harmonics=5, frozen_sources=["V1"]
        )
        assert state.frozen_sources == ["V1"]

    def test_a_transient_continues_from_the_envelope(self, engine, driven):
        _, state = engine.run_hb_envelope(driven, FUNDAMENTAL, harmonics=5)
        duration = 5.0 / FUNDAMENTAL
        tran, checkpoint = engine.run_tran_from_hb_envelope(
            driven, state, duration, max_step=duration / 200
        )

        assert tran.num_points > 10
        assert checkpoint.time == pytest.approx(tran.stop_time, rel=1e-6)

    def test_a_bad_duration_is_rejected(self, engine, driven):
        _, state = engine.run_hb_envelope(driven, FUNDAMENTAL, harmonics=5)
        with pytest.raises(ValueError, match="duration"):
            engine.run_tran_from_hb_envelope(driven, state, -1.0)
