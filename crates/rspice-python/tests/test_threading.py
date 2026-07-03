"""GIL release, parallel engines, and Ctrl-C cancellation."""

import os
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


def start_sigint_timer(delay, done):
    def fire_sigint():
        if not done.wait(delay):
            os.kill(os.getpid(), signal.SIGINT)

    killer = threading.Thread(target=fire_sigint)
    killer.start()
    return killer


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


@pytest.mark.skipif(
    sys.platform == "win32", reason="POSIX signal delivery required"
)
class TestCancellation:
    def test_keyboard_interrupt_cancels_transient(self, engine):
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

    def test_keyboard_interrupt_cancels_dc_sweep(self, engine):
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
