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
            engine.run_tran(netlist, stop_time=5e-3, max_step=5e-8)
        finally:
            stop.set()
            thread.join()
        # If the GIL were held for the whole simulation the ticker would
        # barely run. Require steady progress.
        assert len(ticks) >= 10

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
        def fire_sigint():
            time.sleep(0.5)
            os.kill(os.getpid(), signal.SIGINT)

        killer = threading.Thread(target=fire_sigint)
        start = time.monotonic()
        killer.start()
        try:
            with pytest.raises(KeyboardInterrupt):
                engine.run_tran(netlist, stop_time=50.0, max_step=1e-7)
        finally:
            killer.join()
        elapsed = time.monotonic() - start
        # Cancellation must be prompt — well under the full run time.
        assert elapsed < 10.0

    def test_keyboard_interrupt_cancels_dc_sweep(self, engine):
        # A million-point sweep, cancelled shortly after start.
        netlist = rspice.Netlist.parse(
            "* big sweep\nV1 in 0 10\nR1 in out 1k\nR2 out 0 1k\n.end"
        )

        def fire_sigint():
            time.sleep(0.3)
            os.kill(os.getpid(), signal.SIGINT)

        killer = threading.Thread(target=fire_sigint)
        start = time.monotonic()
        killer.start()
        try:
            with pytest.raises(KeyboardInterrupt):
                engine.run_dc_sweep(netlist, "V1", 0.0, 10.0, 1e-5)
        finally:
            killer.join()
        assert time.monotonic() - start < 10.0
