"""Transient analysis: waveforms, branch currents, Fourier, validation."""

import math

import numpy as np
import pytest

import rspice

RC_STEP = """* RC step response, tau = 100us
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
R1 in out 1k
C1 out 0 100n
.end
"""

SINE = """* 1 kHz sine into a divider
V1 in 0 SIN(0 1 1k)
R1 in out 1k
R2 out 0 1k
.end
"""


class TestTransient:
    def test_run_honors_each_selected_tran_startup_mode(self, engine):
        netlist = rspice.Netlist.parse(
            """* mixed selected transient startup modes
V1 in 0 5
R1 in out 1k
C1 out 0 1u
.tran 10u 1m
.tran 10u 1m uic
.end
"""
        )
        report = engine.run(netlist)
        assert len(report.all_tran) == 2
        ordinary, uic = report.all_tran
        assert ordinary.voltage_waveform("out")[0] == pytest.approx(5.0, abs=1e-9)
        assert uic.voltage_waveform("out")[0] == pytest.approx(0.0, abs=1e-12)

    def test_start_time_clips_output_but_preserves_alignment(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        tran = engine.run_tran(
            netlist, stop_time=1e-3, max_step=1e-6, start_time=5e-4
        )
        assert tran.time[0] >= 5e-4
        assert tran.time[-1] == pytest.approx(1e-3, rel=1e-3)
        assert len(tran.voltage_waveform("out")) == tran.num_points
        assert len(tran.branch_current_waveform("V1")) == tran.num_points

    @pytest.mark.parametrize("start_time", [-1.0, 1e-3, 2e-3, math.inf, math.nan])
    def test_invalid_start_time_raises_valueerror(self, engine, start_time):
        netlist = rspice.Netlist.parse(RC_STEP)
        with pytest.raises(ValueError, match="start_time"):
            engine.run_tran(netlist, stop_time=1e-3, start_time=start_time)

    def test_rc_step_settles_to_one(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        tran = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
        v_out = tran.voltage_waveform("out")
        assert isinstance(v_out, np.ndarray)
        assert len(v_out) == tran.num_points
        assert v_out[-1] == pytest.approx(1.0, abs=1e-3)
        assert tran.stop_time == pytest.approx(1e-3, rel=1e-3)

    def test_rc_time_constant(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        tran = engine.run_tran(netlist, stop_time=5e-4, max_step=5e-7)
        time = tran.time
        v_out = tran.voltage_waveform("out")
        # Value at t = tau should be 1 - 1/e.
        idx = int(np.searchsorted(time, 1e-4))
        assert v_out[idx] == pytest.approx(1.0 - math.exp(-1.0), abs=0.02)

    def test_ground_waveform_is_zero(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        assert float(np.max(np.abs(tran.voltage_waveform(0)))) == 0.0
        assert float(np.max(np.abs(tran.voltage_waveform("gnd")))) == 0.0

    def test_voltage_at(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        assert tran.voltage_at(0, 0) == 0.0
        with pytest.raises(IndexError):
            tran.voltage_at(1, 10**9)
        with pytest.raises(IndexError):
            tran.voltage_at(99, 0)

    def test_unknown_node_raises_keyerror(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        with pytest.raises(KeyError):
            tran.voltage_waveform("nonexistent")


class TestTransientBranchCurrents:
    def test_source_current_waveform(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        tran = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
        assert "V1" in [n.upper() for n in tran.branch_names]
        i_v1 = tran.branch_current_waveform("V1")
        assert len(i_v1) == tran.num_points
        # After settling no current flows; at t=0+ the full step appears
        # across R: |I| ≈ 1 V / 1 kΩ.
        assert abs(i_v1[-1]) < 5e-5
        assert np.max(np.abs(i_v1)) == pytest.approx(1e-3, rel=0.05)

    def test_unknown_branch_raises_keyerror(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        with pytest.raises(KeyError):
            tran.branch_current_waveform("L99")


class TestTransientDeviceParameters:
    def test_saved_device_operating_point_waveforms(self, engine):
        netlist = rspice.Netlist.parse(
            """* MOS transient operating parameters
VDD d 0 5
VG g 0 PULSE(0 3 1u 100n 100n 4u 10u)
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS (LEVEL=1 VTO=1 KP=100u)
.save @M1[gm] @M1[id]
.end
"""
        )
        result = engine.run_tran(netlist, stop_time=10e-6, max_step=100e-9)
        assert {
            "@m1[gm]",
            "@m1[id]",
        } <= {name.lower() for name in result.device_parameter_names}
        gm = result.device_parameter_waveform("m1", "GM")
        drain_current = result.device_parameter_waveform("M1", "id")
        assert gm.shape == drain_current.shape == result.time.shape
        assert np.nanmax(gm) > 0.0
        assert np.nanmax(drain_current) > 0.0
        with pytest.raises(KeyError, match="add it to .SAVE"):
            result.device_parameter_waveform("M1", "missing")

        compressed = engine.run_tran_compressed(
            netlist,
            stop_time=10e-6,
            max_step=100e-9,
            abs_tol=1e-8,
            rel_tol=1e-4,
        )
        assert {
            "@m1[gm]",
            "@m1[id]",
        } <= {name.lower() for name in compressed.device_parameter_names}
        compressed_gm = compressed.device_parameter_waveform("m1", "GM")
        assert compressed_gm.shape == compressed.time.shape
        reconstructed = np.array(
            [compressed.device_parameter_at("M1", "gm", float(time)) for time in result.time]
        )
        tolerance = 1e-8 + 1e-4 * np.abs(gm)
        assert np.all(np.abs(reconstructed - gm) <= tolerance * (1.0 + 1e-10))
        with pytest.raises(KeyError, match="add it to .SAVE"):
            compressed.device_parameter_waveform("M1", "missing")


class TestTransientValidation:
    def test_zero_stop_time_raises(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=0.0)

    def test_negative_stop_time_raises(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=-1.0)

    def test_non_finite_stop_time_raises(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=float("inf"))

    def test_invalid_max_step_raises(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=1e-3, max_step=-1.0)
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=1e-3, max_step=0.0)


class TestTransientCheckpoint:
    def test_unresumable_checkpoint_is_refused_during_run_preflight(self, engine):
        netlist = rspice.Netlist.parse(
            """* checkpoint capability blocker
V1 in 0 1
B1 out 0 V={SDT(V(in))}
R1 out 0 1k
.end
"""
        )
        with pytest.raises(
            rspice.SimulationError,
            match="checkpoint capability preflight failed.*behavioral-source accepted SDT state",
        ):
            engine.run_tran_checkpointed(
                netlist, stop_time=10e-9, max_step=1e-9
            )

    def test_segmented_resume_matches_uninterrupted_final_state(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        full = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
        first, checkpoint = engine.run_tran_checkpointed(
            netlist, stop_time=5e-4, max_step=1e-6
        )
        resumed, final_checkpoint = engine.resume_tran(
            netlist, checkpoint, stop_time=1e-3, max_step=1e-6
        )

        assert isinstance(checkpoint, rspice.TransientCheckpoint)
        assert checkpoint.time == pytest.approx(5e-4, rel=0.0, abs=1e-15)
        assert final_checkpoint.time == pytest.approx(1e-3, rel=0.0, abs=1e-15)
        assert first.time[-1] == pytest.approx(checkpoint.time)
        assert resumed.time[0] == pytest.approx(checkpoint.time)
        assert resumed.voltage_waveform("out")[-1] == pytest.approx(
            full.voltage_waveform("out")[-1], rel=1e-8, abs=1e-12
        )

    def test_checkpoint_round_trips_and_enforces_netlist_identity(
        self, engine, tmp_path
    ):
        netlist = rspice.Netlist.parse(RC_STEP)
        _, checkpoint = engine.run_tran_checkpointed(
            netlist, stop_time=2e-4, max_step=1e-6
        )
        path = tmp_path / "state.rspice-checkpoint"
        checkpoint.save(path)
        loaded = rspice.TransientCheckpoint.load(path)
        assert loaded.time == checkpoint.time
        assert loaded.netlist_fingerprint == checkpoint.netlist_fingerprint
        assert np.array_equal(loaded.solution, checkpoint.solution)

        other = rspice.Netlist.parse("V1 out 0 2\nR1 out 0 1k\n.end")
        with pytest.raises(rspice.SimulationError, match="different netlist"):
            engine.resume_tran(other, loaded, stop_time=3e-4, max_step=1e-6)

    def test_resume_requires_later_stop(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        _, checkpoint = engine.run_tran_checkpointed(
            netlist, stop_time=2e-4, max_step=1e-6
        )
        with pytest.raises(ValueError):
            engine.resume_tran(netlist, checkpoint, stop_time=checkpoint.time)


class TestCompressedTransient:
    def test_compressed_waveform_reduces_storage_and_resamples(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        full = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
        compressed = engine.run_tran_compressed(
            netlist,
            stop_time=1e-3,
            max_step=1e-6,
            abs_tol=1e-6,
            rel_tol=1e-3,
        )

        assert isinstance(compressed, rspice.CompressedTransientResult)
        assert compressed.input_points == full.num_points
        assert compressed.num_points < compressed.input_points
        assert compressed.compression_ratio > 1.0
        assert compressed.step_sizes.shape == compressed.time.shape
        assert np.all(compressed.step_sizes >= 0.0)
        assert compressed.voltage_waveform("out").shape == compressed.time.shape
        reconstructed = np.array(
            [compressed.voltage_at("out", float(time)) for time in full.time]
        )
        assert np.max(np.abs(reconstructed - full.voltage_waveform("out"))) < 2e-3
        time, values = compressed.resample("out", 101)
        assert time.shape == values.shape == (101,)
        assert time[0] == pytest.approx(compressed.time[0])
        assert time[-1] == pytest.approx(compressed.time[-1])
        ground_time, ground = compressed.resample("0", 11)
        assert ground_time.shape == ground.shape == (11,)
        assert np.all(ground == 0.0)
        assert compressed.branch_names == full.branch_names
        for branch_name in compressed.branch_names:
            retained_current = compressed.branch_current_waveform(branch_name)
            assert retained_current.shape == compressed.time.shape
            reconstructed_current = np.array(
                [
                    compressed.branch_current_at(branch_name, float(time))
                    for time in full.time
                ]
            )
            full_current = full.branch_current_waveform(branch_name)
            tolerance = 1e-6 + 1e-3 * np.abs(full_current)
            assert np.all(
                np.abs(reconstructed_current - full_current)
                <= tolerance * (1.0 + 1e-10)
            )
        with pytest.raises(KeyError):
            compressed.branch_current_waveform("L99")

    def test_compression_arguments_are_validated(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_tran_compressed(rc_lowpass, 1e-3, abs_tol=-1.0)
        with pytest.raises(ValueError):
            engine.run_tran_compressed(rc_lowpass, 1e-3, rel_tol=float("nan"))
        with pytest.raises(ValueError):
            engine.run_tran_compressed(rc_lowpass, 1e-3, max_interval=-1.0)

    def test_typed_device_store_waveform_is_retained_and_interpolated(self, engine):
        netlist = rspice.Netlist.parse(
            """* TEAM memristor typed store trace
V1 in 0 0.2
.model mrm1 memristor level=2 ron=50 roff=1k
YMEMRISTOR mr1 in 0 mrm1 ivrelation=1
.tran 1n 4n
.end
"""
        )
        compressed = engine.run_tran_compressed(
            netlist,
            stop_time=4e-9,
            max_step=1e-9,
            abs_tol=1e-8,
            rel_tol=1e-4,
        )
        assert compressed.store_names == ["YMEMRISTOR!MR1:R"]
        resistance = compressed.store_waveform("ymemristor!mr1:r")
        assert resistance.shape == compressed.time.shape
        assert compressed.store_at("YMEMRISTOR!MR1:R", 2e-9) > 0.0
        with pytest.raises(KeyError, match="unknown device-store trace"):
            compressed.store_waveform("missing")


class TestFourier:
    def test_pure_sine_has_low_thd(self, engine):
        netlist = rspice.Netlist.parse(SINE)
        tran = engine.run_tran(netlist, stop_time=10e-3, max_step=2e-6)
        four = tran.fourier("out", fundamental=1e3)
        assert four.fundamental_magnitude == pytest.approx(0.5, rel=0.02)
        assert four.thd_percent < 1.0
        assert abs(four.dc_component) < 0.01
        assert len(four.harmonics) >= 5
        assert four.harmonics[0].n == 1
        assert four.harmonics[0].frequency == pytest.approx(1e3, rel=1e-6)
        assert len(four.magnitudes) == len(four.harmonics)

    def test_fundamental_phase_units(self, engine):
        # v(t) = A sin(wt) decomposes with a1=0, b1=A, so the fundamental
        # phase is atan2(-b, a) = -90 degrees = -pi/2 radians. Pins the
        # radians/degrees convention (core reports degrees; the binding
        # normalizes to radians with a *_degrees helper).
        netlist = rspice.Netlist.parse(SINE)
        tran = engine.run_tran(netlist, stop_time=10e-3, max_step=2e-6)
        fund = tran.fourier("out", fundamental=1e3).harmonics[0]
        assert fund.phase == pytest.approx(-math.pi / 2, abs=0.02)
        assert fund.phase_degrees == pytest.approx(-90.0, abs=1.0)

    def test_fourier_validation(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        with pytest.raises(ValueError):
            tran.fourier("out", fundamental=-1.0)
        with pytest.raises(ValueError):
            tran.fourier("out", fundamental=1e3, num_harmonics=0)
        with pytest.raises(ValueError, match="shorter than the required Fourier window"):
            tran.fourier("out", fundamental=1e3)
        with pytest.raises(KeyError):
            tran.fourier("nonexistent", fundamental=1e3)


XSPICE_EVENTS = """* an ADC bridge, a DAC bridge back, and a real event observer
v1 in 0 pulse(0 5 0 1n 1n 5n 10n)
abridge1 [in] [d] adc
adac [d] [out] dac
aobs out rnode obs
rout out 0 1k
.model adc adc_bridge(in_low=1 in_high=4)
.model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)
.model obs v_to_real(gain=2)
.end
"""

# Every spelling `DigitalEvent.state` and `.strength` may carry. These are the
# shared result document's own tags, which is what makes a history read the
# same through the accessor, through `document()` and through a pickle.
DIGITAL_STATES = {
    "zero",
    "one",
    "unknown",
    "zero_resistive",
    "one_resistive",
    "unknown_resistive",
    "zero_high_z",
    "one_high_z",
    "unknown_high_z",
    "high_z",
}
DIGITAL_STRENGTHS = {"strong", "resistive", "high_z", "undetermined"}


@pytest.fixture(scope="module")
def event_transient():
    netlist = rspice.Netlist.parse(XSPICE_EVENTS)
    return rspice.Engine().run_tran(netlist, stop_time=20e-9, max_step=1e-9)


class TestDigitalEvents:
    def test_every_event_node_answers_with_typed_rows(self, event_transient):
        nodes = event_transient.digital_nodes()
        assert nodes, "the ADC bridge drives a digital event node"
        for node in nodes:
            events = event_transient.digital_events(node)
            assert events, node
            times = [event.time_s for event in events]
            assert times == sorted(times), node
            for event in events:
                assert isinstance(event, rspice.DigitalEvent)
                assert event.state in DIGITAL_STATES
                assert event.strength in DIGITAL_STRENGTHS
                assert 0 <= event.code <= 12
                assert repr(event).startswith("DigitalEvent(")

    def test_the_rows_are_the_documents_own_history(self, event_transient):
        """The typed accessor and the JSON view are two views of one history."""
        traces = event_transient.document()["payload"]["digitalTraces"]
        assert [trace["nodeName"] for trace in traces] == event_transient.digital_nodes()
        for trace in traces:
            events = event_transient.digital_events(trace["nodeName"])
            assert [event.time_s for event in events] == [
                point["time"] for point in trace["points"]
            ]
            assert [event.state for event in events] == [
                point["state"] for point in trace["points"]
            ]
            assert [event.strength for event in events] == [
                point["strength"] for point in trace["points"]
            ]

    def test_a_node_name_resolves_the_way_a_deck_node_name_does(self, event_transient):
        node = event_transient.digital_nodes()[0]
        shouted = [
            (event.time_s, event.state, event.strength, event.code)
            for event in event_transient.digital_events(node.upper())
        ]
        exact = [
            (event.time_s, event.state, event.strength, event.code)
            for event in event_transient.digital_events(node)
        ]
        assert shouted == exact

    def test_an_unknown_event_node_says_how_one_goes_missing(self, event_transient):
        with pytest.raises(KeyError, match="ESAVE"):
            event_transient.digital_events("not_a_node")

    def test_the_history_survives_a_pickle_because_the_labels_are_its_own(
        self, event_transient
    ):
        import pickle

        restored = pickle.loads(pickle.dumps(event_transient))
        assert restored.digital_nodes() == event_transient.digital_nodes()
        for node in event_transient.digital_nodes():
            assert [
                (event.time_s, event.state, event.strength, event.code)
                for event in restored.digital_events(node)
            ] == [
                (event.time_s, event.state, event.strength, event.code)
                for event in event_transient.digital_events(node)
            ]


class TestDigitalVocabulary:
    """One digital state has one spelling on this surface.

    `CompressedTransientResult.digital_trace` used to answer in core's other
    spelling, which hyphenates where the shared document underscores, so the
    same committed value read `zero-resistive` through one container and
    `zero_resistive` through the other.
    """

    def test_both_transient_containers_spell_one_state_one_way(self, event_transient):
        netlist = rspice.Netlist.parse(XSPICE_EVENTS)
        compressed = rspice.Engine().run_tran_compressed(
            netlist, stop_time=20e-9, max_step=1e-9
        )
        assert compressed.digital_trace_names == event_transient.digital_nodes()
        for node in compressed.digital_trace_names:
            rows = compressed.digital_trace(node)
            assert rows == [
                (event.time_s, event.state, event.strength)
                for event in event_transient.digital_events(node)
            ], node
            for _, state, strength in rows:
                assert state in DIGITAL_STATES, state
                assert strength in DIGITAL_STRENGTHS, strength

    def test_a_compressed_result_answers_the_same_real_rows(self, event_transient):
        netlist = rspice.Netlist.parse(XSPICE_EVENTS)
        compressed = rspice.Engine().run_tran_compressed(
            netlist, stop_time=20e-9, max_step=1e-9
        )
        assert compressed.real_trace_names == event_transient.real_trace_names
        for node in compressed.real_trace_names:
            assert compressed.real_trace(node) == event_transient.real_trace(node)


class TestRealEventNodes:
    def test_every_real_event_node_answers_with_its_committed_rows(
        self, event_transient
    ):
        names = event_transient.real_trace_names
        assert names, "the v_to_real observer drives a real event node"
        traces = event_transient.document()["payload"]["realTraces"]
        assert [trace["nodeName"] for trace in traces] == names
        for trace in traces:
            rows = event_transient.real_trace(trace["nodeName"])
            assert rows == [(point["time"], point["value"]) for point in trace["points"]]
            assert [time for time, _ in rows] == sorted(time for time, _ in rows)

    def test_a_real_node_name_resolves_the_way_a_deck_node_name_does(
        self, event_transient
    ):
        node = event_transient.real_trace_names[0]
        assert event_transient.real_trace(node.upper()) == event_transient.real_trace(
            node
        )

    def test_an_unknown_real_event_node_is_refused_by_name(self, event_transient):
        with pytest.raises(KeyError, match="not_a_node"):
            event_transient.real_trace("not_a_node")


# The declared word, its members, and the events the pair produce. No deck this
# build can run declares a bus: the mixed Verilog-AMS boundary that does needs
# the `veriloga` feature, which the Python extension does not build with. The
# declaration therefore arrives the way a loaded file's does, through the
# versioned pickle state and its own decoder.
#
# The histories are the ones BUS-L2's `output [1:0] count` records on a `#5`
# grid, so the word counts 00 01 10 11 and the dump below is that lane's own
# oracle.
BUS_NAME = "x1.count"
BUS_MEMBERS = ["COUNT#1", "COUNT#0"]
BUS_HISTORIES = [
    (
        "COUNT#1",
        [(0.0, "zero", "strong"), (1.0e-8, "one", "strong")],
    ),
    (
        "COUNT#0",
        [
            (0.0, "zero", "strong"),
            (5.0e-9, "one", "strong"),
            (1.0e-8, "zero", "strong"),
            (1.5e-8, "one", "strong"),
        ],
    ),
]
BUS_EVENTS = [(0.0, "00"), (5.0e-9, "01"), (1.0e-8, "10"), (1.5e-8, "11")]
BUS_DECLARATION = (BUS_NAME, 1, 0, BUS_MEMBERS, "engine")


def _with_event_state(result, event_state):
    """Rebuild `result` with a different event state, through `_unpickle`."""
    unpickler, state = result.__reduce__()
    return unpickler(*state[:-1], event_state)


@pytest.fixture(scope="module")
def bus_transient(event_transient):
    version = event_transient.__reduce__()[1][-1][0]
    # No real event node, so this result's dump is exactly the browser
    # binding's for the same declaration - which is what the two surfaces are
    # compared on.
    return _with_event_state(
        event_transient,
        (version, BUS_HISTORIES, [], [BUS_DECLARATION]),
    )


class TestDigitalBuses:
    def test_a_run_without_a_bus_declares_none(self, event_transient):
        assert event_transient.digital_buses() == []

    def test_a_declaration_carries_its_range_and_its_members_in_order(
        self, bus_transient
    ):
        buses = bus_transient.digital_buses()
        assert len(buses) == 1
        bus = buses[0]
        assert isinstance(bus, rspice.DigitalBus)
        assert (bus.name, bus.msb, bus.lsb) == (BUS_NAME, 1, 0)
        assert bus.members == BUS_MEMBERS
        assert bus.source == "engine"
        assert repr(bus).startswith("DigitalBus(")

    def test_a_bus_event_is_every_member_change_with_the_whole_word(
        self, bus_transient
    ):
        events = bus_transient.bus_events(BUS_NAME)
        assert [(event.time_s, event.value) for event in events] == BUS_EVENTS
        for event in events:
            assert isinstance(event, rspice.BusEvent)
            assert len(event.bits) == len(BUS_MEMBERS)
            assert repr(event).startswith("BusEvent(")
        # The word is the members' held values, MSB first, as event codes.
        assert events[-1].bits == [1, 1]

    def test_a_bus_name_resolves_the_way_a_deck_node_name_does(self, bus_transient):
        shouted = [
            (event.time_s, event.value) for event in bus_transient.bus_events("X1.COUNT")
        ]
        assert shouted == BUS_EVENTS

    def test_an_undeclared_bus_lists_what_is_declared(self, bus_transient):
        with pytest.raises(KeyError, match=BUS_NAME):
            bus_transient.bus_events("data")

    def test_the_declaration_survives_a_pickle(self, bus_transient):
        import pickle

        restored = pickle.loads(pickle.dumps(bus_transient))
        original = bus_transient.digital_buses()[0]
        rebuilt = restored.digital_buses()[0]
        assert (rebuilt.name, rebuilt.msb, rebuilt.lsb) == (
            original.name,
            original.msb,
            original.lsb,
        )
        assert rebuilt.members == original.members
        assert rebuilt.source == original.source
        assert [
            (event.time_s, event.value, event.bits)
            for event in restored.bus_events(BUS_NAME)
        ] == [
            (event.time_s, event.value, event.bits)
            for event in bus_transient.bus_events(BUS_NAME)
        ]

    def test_a_state_written_before_the_bus_contract_restores_with_no_bus(
        self, event_transient
    ):
        """A version-1 state is three fields and carries no table.

        It is read rather than refused: nothing that could write one could
        declare a bus, so an empty table is what it says.
        """
        _, digital, real, _ = event_transient.__reduce__()[1][-1]
        restored = _with_event_state(event_transient, (1, digital, real))
        assert restored.digital_buses() == []
        assert restored.digital_nodes() == event_transient.digital_nodes()

    def test_a_version_that_contradicts_the_state_shape_is_refused(
        self, event_transient
    ):
        version, digital, real, buses = event_transient.__reduce__()[1][-1]
        with pytest.raises(ValueError, match="4-field state"):
            _with_event_state(event_transient, (1, digital, real, buses))
        with pytest.raises(ValueError, match="3-field state"):
            _with_event_state(event_transient, (version, digital, real))

    def test_a_pickled_bus_whose_member_has_no_trace_is_refused(self, event_transient):
        version, _, real, _ = event_transient.__reduce__()[1][-1]
        with pytest.raises(ValueError, match="cannot carry"):
            _with_event_state(
                event_transient,
                (
                    version,
                    BUS_HISTORIES[:1],
                    real,
                    [BUS_DECLARATION],
                ),
            )

    def test_a_pickled_bus_with_an_unknown_declarer_is_refused(self, event_transient):
        version, _, real, _ = event_transient.__reduce__()[1][-1]
        with pytest.raises(ValueError, match="guessed"):
            _with_event_state(
                event_transient,
                (
                    version,
                    BUS_HISTORIES,
                    real,
                    [(BUS_NAME, 1, 0, BUS_MEMBERS, "guessed")],
                ),
            )


class TestValueChangeDump:
    def test_a_declared_bus_is_dumped_as_one_vector(self, bus_transient):
        dump = bus_transient.to_vcd()
        assert f"$var wire 2 ! {BUS_NAME} [1:0] $end" in dump
        # The members are in the vector, bit for bit, so they are not declared
        # a second time as scalars.
        for member in BUS_MEMBERS:
            assert member not in dump
        assert "b00 !" in dump
        assert "b01 !" in dump
        assert "b10 !" in dump

    def test_the_dump_declares_every_event_node_under_one_scope(self, event_transient):
        dump = event_transient.to_vcd()
        assert "$scope module events $end" in dump
        assert "$timescale" in dump
        assert "$enddefinitions $end" in dump
        for node in event_transient.digital_nodes():
            assert f" {node} $end" in dump
        # A real event node is a `real` variable, which no table format carries.
        assert "$var real 64 " in dump

    def test_write_vcd_publishes_exactly_what_to_vcd_returns(
        self, event_transient, tmp_path
    ):
        path = tmp_path / "run.vcd"
        event_transient.write_vcd(path)
        assert path.read_text(encoding="ascii") == event_transient.to_vcd()

    def test_a_run_with_no_event_history_is_refused_rather_than_dumped_empty(
        self, engine, rc_lowpass
    ):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        assert tran.digital_nodes() == []
        with pytest.raises(ValueError, match="declare no signal"):
            tran.to_vcd()
