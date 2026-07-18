"""GIL release, parallel engines, and Ctrl-C cancellation."""

import signal
import sys
import threading
import time

import pytest

import rspice

LONG_RC = """* Long transient workload
V1 in 0 SIN(0 1 1k)
R1 in out 1k
C1 out 0 1u
.end
"""

NONLINEAR_DC_SWEEP = """* Nonlinear DC cancellation workload
V1 in 0 0
R1 in out 1k
D1 out 0 dmod
.model dmod D(IS=1e-14 N=1 RS=10)
.end
"""

STB_LOOP = """* Loop stability cancellation workload
E1 eo 0 ctrl 0 -1000
VPROBE eo x 0
R1 x ctrl 1k
C1 ctrl 0 159.154943091895n
.end
"""


def start_sigint_timer(delay, done):
    def fire_sigint():
        if not done.wait(delay):
            # `raise_signal` is portable across POSIX and Windows and is
            # delivered to Python's main thread when the binding polls signal
            # handlers while waiting for the simulation worker.
            signal.raise_signal(signal.SIGINT)

    killer = threading.Thread(target=fire_sigint)
    killer.start()
    return killer


def wait_until(predicate, timeout=10.0):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if predicate():
            return True
        time.sleep(0.001)
    return predicate()


def cancellation_engine():
    """Use an explicit unbounded policy for deliberately enormous workloads."""
    config = rspice.SimulationConfig(resource_limits=rspice.ResourceLimits.unlimited())
    return rspice.Engine(config)


class TestGilRelease:
    def test_main_thread_stays_live_during_simulation(self, engine):
        netlist = rspice.Netlist.parse(LONG_RC)
        ticks = []
        stop = threading.Event()

        def ticker():
            while not stop.is_set():
                ticks.append(time.monotonic())
                time.sleep(0.005)

        thread = threading.Thread(target=ticker)
        thread.start()
        try:
            start = time.monotonic()
            engine.run_tran(netlist, stop_time=2e-2, max_step=5e-8)
            end = time.monotonic()
        finally:
            stop.set()
            thread.join()
        # If the GIL were held for the whole simulation the ticker would
        # block on it and could not record a single tick inside the
        # simulation window (sleep granularity on loaded CI runners makes
        # any fixed tick-count assertion flaky, so count window ticks only).
        during = [t for t in ticks if start < t < end]
        assert len(during) >= 3, (
            f"only {len(during)} ticks during the {end - start:.3f}s "
            f"simulation window — GIL appears to be held"
        )

    def test_parallel_engines(self):
        netlists = [rspice.Netlist.parse(LONG_RC) for _ in range(4)]
        results = [None] * 4
        errors = []

        def work(i):
            try:
                engine = rspice.Engine()
                results[i] = engine.run_tran(netlists[i], 1e-3, max_step=1e-6)
            except Exception as exc:  # pragma: no cover
                errors.append(exc)

        threads = [threading.Thread(target=work, args=(i,)) for i in range(4)]
        start = time.monotonic()
        for t in threads:
            t.start()
        for t in threads:
            t.join()
        elapsed = time.monotonic() - start
        assert not errors
        assert all(r is not None and r.num_points > 10 for r in results)
        assert elapsed < 60

    def test_one_engine_and_netlist_are_safe_to_share(self):
        engine = rspice.Engine()
        netlist = rspice.Netlist.parse("V1 in 0 1\nR1 in 0 1k\n.end\n")
        barrier = threading.Barrier(8)
        results = [None] * 8
        errors = []

        def work(index):
            try:
                barrier.wait(timeout=10)
                result = engine.run_dc_op(netlist)
                # Exercise scalar and NumPy access against independently
                # produced results while all threads remain active.
                results[index] = (result.voltage("in"), result.node_voltages)
            except BaseException as exc:  # pragma: no cover - diagnostic path
                errors.append(exc)

        threads = [threading.Thread(target=work, args=(index,)) for index in range(8)]
        for thread in threads:
            thread.start()
        for thread in threads:
            thread.join(timeout=30)

        assert all(not thread.is_alive() for thread in threads)
        assert not errors
        assert all(value == pytest.approx(1.0) for value, _ in results)
        assert all(array.flags.owndata for _, array in results)

    @pytest.mark.skipif(
        not hasattr(sys, "_is_gil_enabled") or sys._is_gil_enabled(),
        reason="requires free-threaded CPython with the GIL disabled",
    )
    def test_free_threaded_import_keeps_gil_disabled(self):
        assert not sys._is_gil_enabled()


class TestCancellation:
    def test_engine_cancel_stops_all_active_calls(self):
        engine = cancellation_engine()
        netlist = rspice.Netlist.parse(LONG_RC)
        errors = []

        def work():
            try:
                engine.run_tran(netlist, stop_time=50.0, max_step=1e-7)
            except BaseException as exc:  # cancellation is the expected result
                errors.append(exc)

        threads = [threading.Thread(target=work) for _ in range(2)]
        for thread in threads:
            thread.start()

        assert wait_until(lambda: engine.active_run_count == 2)
        assert engine.is_running
        assert engine.progress is None  # ambiguous with concurrent calls
        assert engine.cancel() == 2

        for thread in threads:
            thread.join(timeout=10)
        assert all(not thread.is_alive() for thread in threads)
        assert len(errors) == 2
        assert all(isinstance(error, rspice.CancelledError) for error in errors)
        assert all(error.kind == "aborted" for error in errors)
        assert not engine.is_running
        assert engine.active_run_count == 0
        assert engine.cancel() == 0

        # Cancellation is scoped to calls that were active at cancel time;
        # it must not poison the Engine for subsequent work.
        result = engine.run_dc_op(rspice.Netlist.parse("V1 in 0 1\nR1 in 0 1k\n.end\n"))
        assert result.voltage("in") == pytest.approx(1.0)

    def test_single_active_run_reports_progress(self):
        engine = cancellation_engine()
        netlist = rspice.Netlist.parse(LONG_RC)
        errors = []

        def work():
            try:
                engine.run_tran(netlist, stop_time=50.0, max_step=1e-7)
            except BaseException as exc:
                errors.append(exc)

        thread = threading.Thread(target=work)
        thread.start()
        assert wait_until(lambda: engine.active_run_count == 1)
        assert wait_until(lambda: engine.progress is not None)
        assert 0.0 <= engine.progress <= 1.0
        assert engine.cancel() == 1
        thread.join(timeout=10)

        assert not thread.is_alive()
        assert len(errors) == 1
        assert isinstance(errors[0], rspice.CancelledError)
        assert engine.progress is None

    def test_keyboard_interrupt_cancels_transient(self):
        engine = cancellation_engine()
        netlist = rspice.Netlist.parse(LONG_RC)

        # This workload runs for minutes if not cancelled.
        done = threading.Event()
        killer = start_sigint_timer(0.5, done)
        start = time.monotonic()
        try:
            with pytest.raises(KeyboardInterrupt):
                engine.run_tran(netlist, stop_time=50.0, max_step=1e-7)
        finally:
            done.set()
            killer.join()
        elapsed = time.monotonic() - start
        # Cancellation must be prompt — well under the full run time.
        assert elapsed < 10.0

    def test_keyboard_interrupt_cancels_dc_sweep(self):
        engine = cancellation_engine()
        # A two-million-point nonlinear sweep, cancelled shortly after start.
        netlist = rspice.Netlist.parse(NONLINEAR_DC_SWEEP)

        done = threading.Event()
        killer = start_sigint_timer(0.05, done)
        start = time.monotonic()
        try:
            with pytest.raises(KeyboardInterrupt):
                engine.run_dc_sweep(netlist, "V1", 0.0, 10.0, 5e-6)
        finally:
            done.set()
            killer.join()
        assert time.monotonic() - start < 10.0

    def test_keyboard_interrupt_cancels_pss(self):
        engine = cancellation_engine()
        # A moderately wide RC ladder makes the stabilization solve long
        # enough that the signal cannot race with normal completion on fast
        # release builds.
        lines = ["V1 n0 0 SIN(0 1 1k)"]
        for index in range(80):
            input_node = "n0" if index == 0 else f"n{index}"
            output_node = f"n{index + 1}"
            lines.extend(
                (
                    f"R{index} {input_node} {output_node} 1k",
                    f"C{index} {output_node} 0 1n",
                )
            )
        lines.append(".end")
        netlist = rspice.Netlist.parse("\n".join(lines))

        done = threading.Event()
        killer = start_sigint_timer(0.05, done)
        start = time.monotonic()
        try:
            with pytest.raises(KeyboardInterrupt):
                engine.run_pss(
                    netlist,
                    1e3,
                    tstab=20.0,
                    max_iterations=10,
                    points_per_period=64,
                )
        finally:
            done.set()
            killer.join()
        assert time.monotonic() - start < 10.0

    def test_keyboard_interrupt_cancels_ac_sweep(self):
        engine = cancellation_engine()
        lines = ["V1 n0 0 AC 1"]
        for index in range(120):
            input_node = "n0" if index == 0 else f"n{index}"
            output_node = f"n{index + 1}"
            lines.extend(
                (
                    f"R{index} {input_node} {output_node} 1k",
                    f"C{index} {output_node} 0 1n",
                )
            )
        lines.append(".end")
        netlist = rspice.Netlist.parse("\n".join(lines))
        frequencies = [1.0e3] * 50_000

        done = threading.Event()
        killer = start_sigint_timer(0.01, done)
        start = time.monotonic()
        try:
            with pytest.raises(KeyboardInterrupt):
                engine.run_ac(netlist, frequencies)
        finally:
            done.set()
            killer.join()
        assert time.monotonic() - start < 10.0

    def test_keyboard_interrupt_cancels_stb_sweep(self):
        engine = cancellation_engine()
        netlist = rspice.Netlist.parse(STB_LOOP)

        done = threading.Event()
        killer = start_sigint_timer(0.01, done)
        start = time.monotonic()
        try:
            with pytest.raises(KeyboardInterrupt):
                engine.run_stb(
                    netlist,
                    "VPROBE",
                    variation="lin",
                    points=200_000,
                    start_freq=10.0,
                    stop_freq=10e6,
                )
        finally:
            done.set()
            killer.join()
        assert time.monotonic() - start < 10.0
