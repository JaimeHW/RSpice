"""Pickle and copy support.

Multiprocessing is how most callers parallelize on the GIL-enabled
interpreters this package supports, so netlists, configurations, and results
have to survive a process boundary intact.
"""

import copy
import pickle

import numpy as np
import pytest

import rspice

DECK = """* Parametric divider
.param rval=1k
V1 in 0 10
R1 in out {rval}
R2 out 0 1k
.op
.end
"""


def round_trip(obj, protocol=pickle.HIGHEST_PROTOCOL):
    return pickle.loads(pickle.dumps(obj, protocol))


class TestEnums:
    @pytest.mark.parametrize(
        "member",
        [
            rspice.DampingStrategy.NONE,
            rspice.DampingStrategy.LINE_SEARCH,
            rspice.DampingStrategy.VOLTAGE_LIMITING,
            rspice.DampingStrategy.BANK_ROSE,
            rspice.DampingStrategy.COMBINED,
            rspice.IntegrationMethod.BACKWARD_EULER,
            rspice.IntegrationMethod.TRAPEZOIDAL,
            rspice.IntegrationMethod.GEAR2,
            rspice.IntegrationMethod.TRAP_GEAR,
        ],
    )
    def test_members_round_trip(self, member):
        assert round_trip(member) == member
        assert copy.deepcopy(member) == member


class TestConfiguration:
    def test_bypass_config(self):
        original = rspice.BypassConfig(enabled=True, reltol=2e-3, abstol=1e-9)
        restored = round_trip(original)
        assert restored == original
        assert restored.enabled and restored.reltol == 2e-3

    def test_convergence_config(self):
        original = rspice.ConvergenceConfig.robust()
        restored = round_trip(original)
        assert restored == original
        assert restored.damping_strategy == original.damping_strategy
        assert restored.charge_abstol == original.charge_abstol

    def test_resource_limits(self):
        original = rspice.ResourceLimits(max_batch_runs=77, max_netlist_bytes=4096)
        restored = round_trip(original)
        assert restored == original
        assert restored.max_batch_runs == 77

    def test_unlimited_resource_limits(self):
        assert round_trip(rspice.ResourceLimits.unlimited()) == (
            rspice.ResourceLimits.unlimited()
        )

    def test_simulation_config_carries_nested_objects(self):
        original = rspice.SimulationConfig(
            tolerance=1e-11,
            temperature=350.0,
            integration_method=rspice.IntegrationMethod.GEAR2,
            convergence=rspice.ConvergenceConfig.robust(),
            bypass=rspice.BypassConfig(enabled=True, reltol=2e-3),
            resource_limits=rspice.ResourceLimits(max_batch_runs=77),
        )
        restored = round_trip(original)

        assert restored == original
        assert restored.temperature == 350.0
        assert restored.integration_method == rspice.IntegrationMethod.GEAR2
        assert restored.convergence == original.convergence
        assert restored.bypass.reltol == 2e-3
        assert restored.resource_limits.max_batch_runs == 77

    def test_max_timestep_infinity_survives(self):
        original = rspice.SimulationConfig(max_timestep=float("inf"))
        assert round_trip(original).max_timestep == float("inf")

    def test_deepcopy_is_independent(self):
        original = rspice.BypassConfig(enabled=False)
        clone = copy.deepcopy(original)
        clone.enabled = True
        assert not original.enabled

    def test_engine_accepts_an_unpickled_config(self):
        config = rspice.SimulationConfig(tolerance=1e-10, temperature=310.0)
        engine = rspice.Engine(round_trip(config))
        assert engine.config.temperature == 310.0


class TestNetlist:
    def test_round_trip_preserves_structure(self):
        original = rspice.Netlist.parse(DECK)
        restored = round_trip(original)

        assert restored.title == original.title
        assert restored.num_elements == original.num_elements
        assert restored.element_names == original.element_names
        assert restored.analyses == original.analyses
        assert restored.source == original.source

    def test_unpickled_netlist_simulates_identically(self, engine):
        original = rspice.Netlist.parse(DECK)
        restored = round_trip(original)
        assert engine.run_dc_op(restored).voltage("out") == pytest.approx(
            engine.run_dc_op(original).voltage("out")
        )

    def test_parse_spice_semantics_survive(self):
        original = rspice.Netlist.parse_spice("My Amplifier\nV1 in 0 10\nR1 in 0 1k\n.end")
        restored = round_trip(original)
        assert restored.title == "My Amplifier"
        assert restored.num_elements == original.num_elements

    def test_resource_policy_is_carried(self):
        limits = rspice.ResourceLimits(max_netlist_bytes=4096)
        restored = round_trip(rspice.Netlist.parse(DECK, resource_limits=limits))
        # The policy is reapplied on unpickling, so a deck that no longer fits
        # would fail rather than silently parsing under the defaults.
        assert restored.num_elements == 3

    def test_source_path_is_carried(self, tmp_path):
        deck = tmp_path / "deck.cir"
        deck.write_text(DECK)
        original = rspice.Netlist.parse_file(deck)
        restored = round_trip(original)
        assert restored.source_path == original.source_path

    @pytest.mark.parametrize("protocol", range(2, pickle.HIGHEST_PROTOCOL + 1))
    def test_every_supported_protocol(self, protocol):
        original = rspice.Netlist.parse(DECK)
        assert round_trip(original, protocol).num_elements == original.num_elements

    def test_deepcopy(self):
        original = rspice.Netlist.parse(DECK)
        assert copy.deepcopy(original).element_names == original.element_names


ANALYSIS_DECK = """* Every result kind from one deck
.param rval=1k
V1 in 0 AC 1 SIN(0 1 1k)
R1 in out {rval}
C1 out 0 1u
.end
"""


@pytest.fixture(scope="module")
def analysis_netlist():
    return rspice.Netlist.parse(ANALYSIS_DECK)


class TestResults:
    """Every result a worker can return survives a process boundary."""

    def test_simulation_result(self, engine, analysis_netlist):
        original = engine.run_dc_op(analysis_netlist)
        restored = round_trip(original)

        assert restored.voltage("out") == original.voltage("out")
        assert restored.node_names == original.node_names
        assert restored.branch_names == original.branch_names
        assert restored.branch_current("V1") == original.branch_current("V1")

    def test_transient_result_including_derived_analyses(self, engine, analysis_netlist):
        original = engine.run_tran(analysis_netlist, stop_time=2e-3, max_step=2e-5)
        restored = round_trip(original)

        assert restored.num_points == original.num_points
        np.testing.assert_allclose(restored.time, original.time)
        np.testing.assert_allclose(
            restored.voltage_waveform("out"), original.voltage_waveform("out")
        )
        np.testing.assert_allclose(
            restored.branch_current_waveform("V1"),
            original.branch_current_waveform("V1"),
        )
        # Derived analyses must still work off the rebuilt result.
        assert (
            restored.fourier("out", 1e3).thd_percent
            == original.fourier("out", 1e3).thd_percent
        )
        assert restored.to_csv() == original.to_csv()

    def test_ac_result(self, engine, analysis_netlist):
        original = engine.run_ac_sweep(analysis_netlist, "dec", 5, 10.0, 1e4)
        restored = round_trip(original)

        np.testing.assert_allclose(restored.frequencies, original.frequencies)
        np.testing.assert_allclose(
            restored.voltage_complex("out"), original.voltage_complex("out")
        )
        np.testing.assert_allclose(
            restored.branch_current_complex("V1"),
            original.branch_current_complex("V1"),
        )
        assert restored.to_csv() == original.to_csv()

    def test_dc_sweep_result(self, engine, analysis_netlist):
        original = engine.run_dc_sweep(analysis_netlist, "V1", 0, 5, 1)
        restored = round_trip(original)

        assert len(restored) == len(original)
        np.testing.assert_allclose(restored.sweep_values, original.sweep_values)
        np.testing.assert_allclose(
            restored.voltage_array("out"), original.voltage_array("out")
        )
        assert restored.export_columns == original.export_columns

    def test_noise_results_keep_their_contributors(self, engine, analysis_netlist):
        original = engine.run_noise(analysis_netlist, "out", [1e3, 1e4])
        restored = round_trip(original)

        assert len(restored) == len(original)
        for left, right in zip(restored, original):
            assert left.frequency == right.frequency
            assert left.output_noise_density == right.output_noise_density
            assert [c.device_name for c in left.contributions] == [
                c.device_name for c in right.contributions
            ]

    def test_transfer_function_result(self, engine, analysis_netlist):
        original = engine.run_transfer_function(analysis_netlist, "out", "V1")
        restored = round_trip(original)
        assert restored.gain == original.gain
        assert restored.input_impedance == original.input_impedance

    def test_pole_zero_result(self, engine, analysis_netlist):
        original = engine.run_pz(analysis_netlist, "in", "out")
        restored = round_trip(original)

        assert restored.num_poles == original.num_poles
        assert restored.dc_gain == original.dc_gain
        np.testing.assert_allclose(restored.poles_array, original.poles_array)

    def test_fourier_result(self, engine, analysis_netlist):
        tran = engine.run_tran(analysis_netlist, stop_time=2e-3, max_step=2e-5)
        original = tran.fourier("out", 1e3)
        restored = round_trip(original)

        assert restored.thd_percent == original.thd_percent
        assert restored.dc_component == original.dc_component
        assert [h.magnitude for h in restored.harmonics] == [
            h.magnitude for h in original.harmonics
        ]

    def test_monte_carlo_result(self, engine, analysis_netlist):
        original = engine.run_monte_carlo(analysis_netlist, 24, seed=3)
        restored = round_trip(original)

        assert restored.variable_names == original.variable_names
        assert restored.num_runs == original.num_runs
        np.testing.assert_allclose(
            restored["V(OUT)"].samples, original["V(OUT)"].samples
        )
        assert restored["V(OUT)"].histogram == original["V(OUT)"].histogram

    def test_monte_carlo_payload_is_deterministic(self, engine, analysis_netlist):
        original = engine.run_monte_carlo(analysis_netlist, 16, seed=5)
        assert pickle.dumps(original) == pickle.dumps(original)

    def test_dc_sensitivity_result(self, engine, analysis_netlist):
        original = engine.run_sensitivity_dc_complete(analysis_netlist, "out")
        restored = round_trip(original)

        assert restored.vector_names == original.vector_names
        assert restored.output_value == original.output_value

    def test_ac_sensitivity_result(self, engine, analysis_netlist):
        original = engine.run_sensitivity_ac_complete(analysis_netlist, "out", [1e3])
        restored = round_trip(original)

        assert restored.vector_names == original.vector_names
        np.testing.assert_allclose(restored.output_complex, original.output_complex)
        left = restored.get(restored.vector_names[0])
        right = original.get(original.vector_names[0])
        np.testing.assert_allclose(left.absolute, right.absolute)
        np.testing.assert_allclose(left.db, right.db)

    def test_measurements_and_records(self, engine):
        deck = ANALYSIS_DECK.replace(
            ".end", ".tran 2u 2m\n.meas tran vmax MAX V(out)\n.end"
        )
        report = engine.run(rspice.Netlist.parse(deck))

        measurement = report.measurement("vmax")
        restored = round_trip(measurement)
        assert restored.value == measurement.value
        assert restored.passed == measurement.passed
        assert restored.analysis == measurement.analysis

        record = round_trip(report.records[0])
        assert record.kind == report.records[0].kind
        assert record.skipped == report.records[0].skipped


def _simulate_in_worker(payload):
    """Run one operating point in a spawned process.

    Defined at module scope because Windows spawns rather than forks, so the
    worker has to be importable by name.
    """
    netlist, config = payload
    return rspice.Engine(config).run_dc_op(netlist)


class TestProcessPool:
    """The reason pickling matters: parallelising across processes."""

    def test_netlist_and_result_cross_a_process_boundary(self):
        import multiprocessing

        netlist = rspice.Netlist.parse(DECK)
        config = rspice.SimulationConfig(tolerance=1e-10)

        context = multiprocessing.get_context("spawn")
        with context.Pool(1) as pool:
            result = pool.apply_async(
                _simulate_in_worker, ((netlist, config),)
            ).get(timeout=300)

        assert result.voltage("out") == pytest.approx(5.0)
        assert result.node_names == rspice.Engine().run_dc_op(netlist).node_names


RF_DECK = """* Driven RC for periodic analyses
V1 in 0 SIN(0 1 1meg)
R1 in out 1k
C1 out 0 100p
.end
"""

# A diode mixer, not the RC, is what makes the periodic state worth carrying:
# it spreads energy across harmonics, couples the PAC sidebands so the
# conversion matrix is not merely diagonal, and makes HB record a Norton
# continuation limitation. On the RC every one of those is empty or zero and
# an equality assertion would hold vacuously.
MIXER_DECK = """* Diode mixer
V1 lo 0 SIN(0 0.8 1meg)
R1 lo mid 1k
D1 mid out DM
R2 out 0 2k
C1 out 0 50p
.model DM D(IS=1e-14 N=1 CJO=1p TT=1n)
.end
"""

DISTORTION_DECK = """* Distortion oracle
V1 out 0 DC 0.5 DISTOF1 1m 0
D1 out 0 DM
.model DM D(IS=1e-12 N=1 CJO=0 TT=0)
.end
"""

RF_FUNDAMENTAL = 1e6


@pytest.fixture(scope="module")
def rf_netlist():
    return rspice.Netlist.parse(RF_DECK)


@pytest.fixture(scope="module")
def mixer_netlist():
    return rspice.Netlist.parse(MIXER_DECK)


@pytest.fixture(scope="module")
def distortion_netlist():
    return rspice.Netlist.parse(DISTORTION_DECK)


class TestPeriodicResults:
    """Periodic and RF results a worker can return."""

    def test_pss_result(self, engine, mixer_netlist):
        original = engine.run_pss(mixer_netlist, RF_FUNDAMENTAL, harmonics=5)
        # Guard against a vacuous comparison: a flat orbit would compare equal
        # no matter what the reduction dropped.
        assert original.num_points > 1
        assert original.peak_to_peak("out") > 1e-3
        assert original.thd_percent("out") > 1.0

        restored = round_trip(original)
        for name in (
            "frequency",
            "period",
            "iterations",
            "residual_norm",
            "is_stable",
            "period_detected",
            "num_nodes",
            "num_points",
            "num_harmonics",
            "node_names",
        ):
            assert getattr(restored, name) == getattr(original, name), name
        np.testing.assert_array_equal(restored.time, original.time)
        np.testing.assert_array_equal(
            restored.harmonic_frequencies, original.harmonic_frequencies
        )
        np.testing.assert_array_equal(
            restored.voltage_waveform("out"), original.voltage_waveform("out")
        )
        # Derived quantities recompute from the rebuilt orbit, so they check
        # the waveforms and the period together.
        np.testing.assert_array_equal(
            restored.harmonic_coefficients("out"),
            original.harmonic_coefficients("out"),
        )
        assert restored.thd_percent("out") == original.thd_percent("out")
        assert restored.dc("out") == original.dc("out")
        assert restored.peak_to_peak("out") == original.peak_to_peak("out")
        assert repr(restored) == repr(original)

    def test_pss_floquet_multipliers_travel(self):
        """No driven solve fills this field, so exercise it directly.

        `PssResult.floquet_multipliers` is populated only by the oscillator
        path, but the reduction has to carry it whenever it is non-empty.
        """
        multipliers = [(1.0, 0.0), (0.25, -0.5), (-0.125, 0.75)]
        original = rspice.PssResult._unpickle(
            (1e-6, 1e6, 3, 1e-12, True),
            [0.0, 5e-7],
            [[0.0, 1.0]],
            ["OUT"],
            multipliers,
            (5, 3, 1e-12, 1e-6, True),
        )
        assert len(original.floquet_multipliers) == 3

        restored = round_trip(original)
        np.testing.assert_array_equal(
            restored.floquet_multipliers,
            np.array([complex(re, im) for re, im in multipliers]),
        )
        assert restored.period_detected is True
        assert restored.num_harmonics == 5
        np.testing.assert_array_equal(restored.voltage_waveform("OUT"), [0.0, 1.0])

    def test_hb_result(self, engine, mixer_netlist):
        original = engine.run_hb(mixer_netlist, RF_FUNDAMENTAL, harmonics=5)
        assert original.converged
        # Every harmonic carries energy here, so a dropped spectrum shows up.
        assert np.count_nonzero(original.coefficients("out")) > 3
        assert original.thd_percent("out") > 1.0

        restored = round_trip(original)
        for name in (
            "converged",
            "iterations",
            "residual_norm",
            "fundamental_frequency",
            "num_harmonics",
            "solve_time_seconds",
            "is_valid",
            "node_names",
        ):
            assert getattr(restored, name) == getattr(original, name), name
        np.testing.assert_array_equal(
            restored.harmonic_frequencies, original.harmonic_frequencies
        )
        np.testing.assert_array_equal(
            restored.coefficients("out"), original.coefficients("out")
        )
        np.testing.assert_array_equal(
            restored.phase_degrees("out"), original.phase_degrees("out")
        )
        assert restored.dc("out") == original.dc("out")
        assert restored.rms("out") == original.rms("out")
        assert restored.thd_percent("out") == original.thd_percent("out")
        assert repr(restored) == repr(original)

    def test_hb_carries_the_spectra_is_valid_depends_on(self, engine, mixer_netlist):
        """`is_valid` is a finiteness test over spectra with no accessor.

        Dropping them would leave `is_valid` reading `True` on a rebuilt
        result whose original had a non-finite reactive current, so the
        reduction carries them even though nothing else reads them.
        """
        original = engine.run_hb(mixer_netlist, RF_FUNDAMENTAL, harmonics=5)
        state = original.__reduce__()[1]
        reactive_spectra, limitations = state[6], state[7]

        assert reactive_spectra, "the mixer's capacitor should be retained"
        device, kind, voltages, currents, dc_exact = reactive_spectra[0]
        assert device == "C1"
        assert kind == "capacitor"
        assert len(voltages) == len(currents) == 6
        assert dc_exact is True
        assert limitations == ["nonlinear_voltage_sources_use_norton_equivalent"]

        assert round_trip(original).is_valid == original.is_valid

    @pytest.mark.parametrize(
        ("field", "value", "message"),
        [
            (6, [("C1", "flux-capacitor", [], [], True)], "reactive element kind"),
            (7, ["time-travel-not-retained"], "continuation limitation"),
        ],
    )
    def test_an_unknown_hb_tag_is_rejected(self, field, value, message):
        state = [
            (True, 1, 0.0, 1e6, 1, 0.0),
            [],
            [],
            [],
            [],
            [],
            [],
            [],
        ]
        state[field] = value
        with pytest.raises(ValueError, match=message):
            rspice.HbResult._unpickle(*state)

    def test_pac_result(self, engine, mixer_netlist):
        original = engine.run_pac(
            mixer_netlist, RF_FUNDAMENTAL, 1e3, 1e5, 4, "V1", "out", sideband_max=2
        )
        # The conversion matrix is only worth carrying if it mixes: on a
        # linear circuit every off-diagonal entry is zero.
        off_diagonal = [
            np.count_nonzero(original.conversion_gain(source, output))
            for source in original.sidebands
            for output in original.sidebands
            if source != output
        ]
        assert min(off_diagonal) > 0, "expected every sideband pair to couple"

        restored = round_trip(original)
        for name in (
            "fundamental_frequency",
            "sideband_min",
            "sideband_max",
            "sidebands",
            "node_names",
            "input_source",
            "output_node",
            "converged",
        ):
            assert getattr(restored, name) == getattr(original, name), name
        np.testing.assert_array_equal(restored.frequencies, original.frequencies)
        for sideband in original.sidebands:
            assert np.count_nonzero(original.voltage("out", sideband))
            np.testing.assert_array_equal(
                restored.voltage("out", sideband),
                original.voltage("out", sideband),
                err_msg=f"sideband {sideband}",
            )
        # The conversion matrix is a separate structure from the per-sideband
        # spectra and has to survive independently of them.
        for output in original.sidebands:
            for source in original.sidebands:
                np.testing.assert_array_equal(
                    restored.conversion_gain(source, output),
                    original.conversion_gain(source, output),
                    err_msg=f"{source} -> {output}",
                )
                np.testing.assert_array_equal(
                    restored.conversion_gain_db(source, output),
                    original.conversion_gain_db(source, output),
                    err_msg=f"{source} -> {output} in dB",
                )
        assert repr(restored) == repr(original)


class TestCompressedTransientResult:
    def test_waveforms_and_interpolation_survive(self, engine, analysis_netlist):
        original = engine.run_tran_compressed(
            analysis_netlist, stop_time=2e-3, max_step=2e-5
        )
        assert original.num_points > 2
        assert original.input_points >= original.num_points

        restored = round_trip(original)
        for name in (
            "num_nodes",
            "num_points",
            "input_points",
            "compression_ratio",
            "node_names",
        ):
            assert getattr(restored, name) == getattr(original, name), name
        np.testing.assert_array_equal(restored.time, original.time)
        np.testing.assert_array_equal(
            restored.voltage_waveform("out"), original.voltage_waveform("out")
        )
        # Interpolation and resampling read the retained points, not a cache.
        assert restored.voltage_at("out", 1e-3) == original.voltage_at("out", 1e-3)
        for restored_axis, original_axis in zip(
            restored.resample("out", 32), original.resample("out", 32)
        ):
            np.testing.assert_array_equal(restored_axis, original_axis)
        assert repr(restored) == repr(original)


class TestDistortionResult:
    def test_harmonic_products_survive(self, engine, distortion_netlist):
        original = engine.run_distortion(distortion_netlist, [1e3, 2e3])
        assert original.available_products == ["2f1", "3f1"]

        restored = round_trip(original)
        assert restored.is_two_tone == original.is_two_tone
        assert restored.f2_frequency == original.f2_frequency
        assert restored.f2_over_f1 == original.f2_over_f1
        assert restored.num_points == original.num_points
        assert restored.node_names == original.node_names
        assert restored.branch_names == original.branch_names
        assert restored.available_products == original.available_products
        np.testing.assert_array_equal(
            restored.f1_frequencies, original.f1_frequencies
        )
        np.testing.assert_array_equal(
            restored.fundamental_f1.voltage_complex("out"),
            original.fundamental_f1.voltage_complex("out"),
        )
        for product in original.available_products:
            assert np.count_nonzero(original.product(product).voltage_complex("out"))
            np.testing.assert_array_equal(
                restored.product(product).voltage_complex("out"),
                original.product(product).voltage_complex("out"),
                err_msg=product,
            )
            np.testing.assert_array_equal(
                restored.voltage_db_relative(product, "out"),
                original.voltage_db_relative(product, "out"),
                err_msg=product,
            )
        assert repr(restored) == repr(original)

    def test_two_tone_products_survive(self, engine):
        deck = DISTORTION_DECK.replace("DISTOF1 1m 0", "DISTOF1 1m 0 DISTOF2 2m 0")
        original = engine.run_distortion(
            rspice.Netlist.parse(deck), [1e3, 2e3], f2_over_f1=0.9
        )
        assert original.available_products == ["f1+f2", "f1-f2", "2f1-f2"]

        restored = round_trip(original)
        assert restored.is_two_tone
        assert restored.f2_over_f1 == original.f2_over_f1
        assert restored.f2_frequency == original.f2_frequency
        assert restored.available_products == original.available_products
        np.testing.assert_array_equal(
            restored.fundamental_f2.voltage_complex("out"),
            original.fundamental_f2.voltage_complex("out"),
        )
        for product in original.available_products:
            np.testing.assert_array_equal(
                restored.product(product).voltage_complex("out"),
                original.product(product).voltage_complex("out"),
                err_msg=product,
            )

    def test_an_unknown_product_label_is_rejected(self):
        with pytest.raises(ValueError, match="unknown distortion product"):
            rspice.DistortionResult._unpickle(
                None, [1e3], [], None, [("nonsense", [])], [], []
            )


class TestRfDeepCopy:
    """`copy.deepcopy` goes through the same reduction as pickle."""

    def test_periodic_results_deep_copy(self, engine, mixer_netlist):
        pss = engine.run_pss(mixer_netlist, RF_FUNDAMENTAL, harmonics=5)
        hb = engine.run_hb(mixer_netlist, RF_FUNDAMENTAL, harmonics=5)

        assert copy.deepcopy(pss).thd_percent("out") == pss.thd_percent("out")
        assert copy.deepcopy(hb).dc("out") == hb.dc("out")


def _analyse_in_worker(payload):
    """Run a PSS solve in a spawned process and return the result object."""
    netlist, fundamental = payload
    return rspice.Engine().run_pss(netlist, fundamental, harmonics=5)


class TestRfProcessPool:
    def test_a_pss_result_crosses_a_process_boundary(self):
        import multiprocessing

        netlist = rspice.Netlist.parse(MIXER_DECK)
        context = multiprocessing.get_context("spawn")
        with context.Pool(1) as pool:
            result = pool.apply_async(
                _analyse_in_worker, ((netlist, RF_FUNDAMENTAL),)
            ).get(timeout=300)

        local = rspice.Engine().run_pss(netlist, RF_FUNDAMENTAL, harmonics=5)
        assert result.frequency == pytest.approx(local.frequency, rel=1e-9)
        np.testing.assert_allclose(
            result.voltage_waveform("out"), local.voltage_waveform("out")
        )
        assert result.thd_percent("out") == pytest.approx(
            local.thd_percent("out"), rel=1e-9
        )
