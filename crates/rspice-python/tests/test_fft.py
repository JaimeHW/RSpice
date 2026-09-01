"""Typed transient `.FFT` access and lossless persistence contracts."""

import pickle

import numpy as np
import pytest

import rspice


FFT_DECK = """V1 out 0 SIN(0 1 1k)
R1 out 0 1k
.save I(V1)
.options fft fft_mode=1 fft_accurate=0 fftout=1
.tran 1u 1m
.fft V(out) np=128 format=unorm window=hann freq=1k fmin=1k fmax=10k
.fft {2*V(out)} np=128 format=norm window=rect
.fft I(V1) np=128 format=unorm window=hamming
.end
"""


@pytest.fixture(scope="module")
def fft_netlist():
    return rspice.Netlist.parse(FFT_DECK)


def round_trip(value):
    return pickle.loads(pickle.dumps(value, pickle.HIGHEST_PROTOCOL))


def assert_harmonic_equal(actual, expected):
    assert actual.rank == expected.rank
    assert actual.bin == expected.bin
    assert actual.frequency == expected.frequency
    assert actual.magnitude == expected.magnitude
    assert actual.magnitude_db == expected.magnitude_db
    assert actual.phase_degrees == expected.phase_degrees


def assert_fft_equal(actual, expected):
    for name in (
        "source_kind",
        "source",
        "output_name",
        "physical_type",
        "start_time",
        "stop_time",
        "sample_interval",
        "point_count",
        "accurate_sampling",
        "format",
        "mode",
        "window",
        "window_name",
        "alpha",
        "coherent_gain",
        "frequency_resolution",
        "fundamental_bin",
        "minimum_metric_bin",
        "maximum_metric_bin",
    ):
        assert getattr(actual, name) == getattr(expected, name), name

    np.testing.assert_array_equal(actual.frequencies, expected.frequencies)
    np.testing.assert_array_equal(actual.complex_bins, expected.complex_bins)
    np.testing.assert_array_equal(actual.magnitudes, expected.magnitudes)
    np.testing.assert_array_equal(actual.phases_degrees, expected.phases_degrees)
    assert len(actual.bins) == len(expected.bins)
    for actual_bin, expected_bin in zip(actual.bins, expected.bins):
        assert actual_bin.index == expected_bin.index
        assert actual_bin.frequency == expected_bin.frequency
        assert actual_bin.real == expected_bin.real
        assert actual_bin.imaginary == expected_bin.imaginary
        assert complex(actual_bin.value) == complex(expected_bin.value)
        assert actual_bin.magnitude == expected_bin.magnitude
        assert actual_bin.phase_degrees == expected_bin.phase_degrees

    assert (actual.metrics is None) == (expected.metrics is None)
    if actual.metrics is not None:
        for name in (
            "fundamental_magnitude",
            "thd_ratio",
            "thd_db",
            "sndr_db",
            "enob_bits",
            "snr_db",
            "sfdr_db",
            "sfdr_spur_bin",
            "sfdr_spur_frequency",
        ):
            assert getattr(actual.metrics, name) == getattr(expected.metrics, name), name
        assert len(actual.metrics.largest_harmonics) == len(
            expected.metrics.largest_harmonics
        )
        for actual_harmonic, expected_harmonic in zip(
            actual.metrics.largest_harmonics,
            expected.metrics.largest_harmonics,
        ):
            assert_harmonic_equal(actual_harmonic, expected_harmonic)


def test_full_and_compressed_results_expose_identical_typed_fft(engine, fft_netlist):
    full = engine.run_tran(fft_netlist, stop_time=1e-3, max_step=1e-6)
    compressed = engine.run_tran_compressed(
        fft_netlist, stop_time=1e-3, max_step=1e-6
    )

    assert [fft.source_kind for fft in full.fft_results] == [
        "probe",
        "expression",
        "probe",
    ]
    assert [fft.physical_type for fft in full.fft_results] == [
        "voltage",
        "parameter",
        "current",
    ]
    assert [fft.output_name for fft in full.fft_results] == [
        "V(OUT)",
        "{2*V(out)}",
        "I(V1)",
    ]
    assert full.fft(0).mode == "spectre_compatible"
    assert full.fft(0).window == "hann"
    assert full.fft(0).format == "unnormalized"
    assert full.fft(0).point_count == 128
    assert len(full.fft(0).bins) == 65
    assert full.fft(0).metrics is not None
    assert full.fft(0).metrics.largest_harmonics
    with pytest.raises(IndexError, match="FFT result index"):
        full.fft(3)
    with pytest.raises(IndexError, match="FFT bin index"):
        full.fft(0).bin(65)

    assert len(compressed.fft_results) == len(full.fft_results)
    for compressed_fft, full_fft in zip(compressed.fft_results, full.fft_results):
        assert_fft_equal(compressed_fft, full_fft)


@pytest.mark.parametrize("compressed", [False, True])
def test_transient_pickle_round_trip_preserves_complete_fft_exactly(
    engine, fft_netlist, compressed
):
    if compressed:
        original = engine.run_tran_compressed(
            fft_netlist, stop_time=1e-3, max_step=1e-6
        )
    else:
        original = engine.run_tran(fft_netlist, stop_time=1e-3, max_step=1e-6)

    restored = round_trip(original)
    assert len(restored.fft_results) == len(original.fft_results) == 3
    for restored_fft, original_fft in zip(
        restored.fft_results, original.fft_results
    ):
        assert_fft_equal(restored_fft, original_fft)

    # FFT products and their typed children remain independently persistable.
    assert_fft_equal(round_trip(original.fft(0)), original.fft(0))
    assert round_trip(original.fft(0).bin(1)).real == original.fft(0).bin(1).real
    restored_metrics = round_trip(original.fft(0).metrics)
    assert restored_metrics.thd_ratio == original.fft(0).metrics.thd_ratio
    assert_harmonic_equal(
        round_trip(original.fft(0).metrics.largest_harmonics[0]),
        original.fft(0).metrics.largest_harmonics[0],
    )


@pytest.mark.parametrize("compressed", [False, True])
def test_transient_pickle_rejects_legacy_and_future_fft_state(
    engine, fft_netlist, compressed
):
    if compressed:
        original = engine.run_tran_compressed(
            fft_netlist, stop_time=1e-3, max_step=1e-6
        )
    else:
        original = engine.run_tran(fft_netlist, stop_time=1e-3, max_step=1e-6)
    unpickler, state = original.__reduce__()
    fft_index = -2 if compressed else -1

    legacy_state = list(state)
    legacy_state[fft_index] = None
    with pytest.raises(ValueError, match="legacy transient pickle"):
        unpickler(*legacy_state)

    future_state = list(state)
    future_state[fft_index] = (999, state[fft_index][1])
    with pytest.raises(ValueError, match="unsupported transient FFT pickle state version"):
        unpickler(*future_state)


def test_fft_pickle_rejects_malformed_enum_and_bin_shape(engine, fft_netlist):
    fft = engine.run_tran(fft_netlist, stop_time=1e-3, max_step=1e-6).fft(0)
    unpickler, args = fft.__reduce__()
    state = args[0]

    source, sampling, configuration, axes, bins, metrics = state
    bad_configuration = (
        "invented-format",
        configuration[1],
        configuration[2],
        configuration[3],
        configuration[4],
        configuration[5],
    )
    with pytest.raises(ValueError, match="unknown transient FFT format"):
        unpickler((source, sampling, bad_configuration, axes, bins, metrics))

    with pytest.raises(ValueError, match="has 64 bins, expected 65"):
        unpickler((source, sampling, configuration, axes, bins[:-1], metrics))
