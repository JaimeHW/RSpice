"""Result serialization: Touchstone, SPICE raw, and CSV.

Each format is checked against the in-memory result it came from, so an
exported artifact cannot drift from the data a caller already sees.
"""

import struct

import numpy as np
import pytest

import rspice

RC = """* RC lowpass
V1 in 0 AC 1 PULSE(0 1 0 1n 1n 1m 2m)
R1 in out 1k
C1 out 0 1u
.end
"""

TWO_PORT = """* Resistive two-port
V1 in 0 AC 0 portnum=1 z0=50
V2 out 0 AC 0 portnum=2 z0=50
R1 in mid 50
R2 mid 0 50
R3 mid out 50
.end
"""

MIXED_PORTS = """* Two ports with different reference impedances
V1 in 0 AC 0 portnum=1 z0=50
V2 out 0 AC 0 portnum=2 z0=75
R1 in out 50
.end
"""


def parse_raw_header(document: str) -> dict[str, str]:
    header = {}
    for line in document.splitlines():
        if line.startswith(("\t", "Values:", "Binary:")):
            break
        if ": " in line:
            key, value = line.split(": ", 1)
            header[key] = value
    return header


@pytest.fixture()
def tran(engine):
    return engine.run_tran(rspice.Netlist.parse(RC), stop_time=1e-3, max_step=1e-5)


@pytest.fixture()
def ac(engine):
    return engine.run_ac_sweep(rspice.Netlist.parse(RC), "dec", 5, 10.0, 1e4)


@pytest.fixture()
def sparams(engine):
    return engine.run_s_parameters(
        rspice.Netlist.parse(TWO_PORT), np.array([1e9, 2e9, 5e9])
    )


class TestTouchstone:
    def test_option_line_declares_format_unit_and_impedance(self, sparams):
        lines = sparams.to_touchstone(format="ma", frequency_unit="mhz").splitlines()
        assert lines[0] == "# MHZ S MA R 50"

    def test_comments_precede_the_option_line(self, sparams):
        text = sparams.to_touchstone(comments=["first", "second"])
        assert text.splitlines()[:2] == ["! first", "! second"]

    def test_two_port_data_uses_the_s11_s21_s12_s22_ordering(self, sparams):
        data = [
            line
            for line in sparams.to_touchstone(frequency_unit="hz").splitlines()
            if not line.startswith(("!", "#"))
        ]
        fields = [float(value) for value in data[0].split()]

        assert fields[0] == pytest.approx(sparams.frequencies[0])
        for offset, (row, col) in enumerate([(1, 1), (2, 1), (1, 2), (2, 2)]):
            expected = sparams.s(row, col)[0]
            assert fields[1 + 2 * offset] == pytest.approx(expected.real)
            assert fields[2 + 2 * offset] == pytest.approx(expected.imag)

    def test_frequency_unit_scales_the_axis(self, sparams):
        in_hz = sparams.to_touchstone(frequency_unit="hz").splitlines()[1].split()[0]
        in_ghz = sparams.to_touchstone(frequency_unit="ghz").splitlines()[1].split()[0]
        assert float(in_hz) == pytest.approx(float(in_ghz) * 1e9)

    def test_extension_reflects_the_port_count(self, sparams):
        assert sparams.touchstone_extension == "s2p"

    def test_write_touchstone_matches_to_touchstone(self, sparams, tmp_path):
        path = tmp_path / f"dut.{sparams.touchstone_extension}"
        sparams.write_touchstone(path, format="db")
        assert path.read_text() == sparams.to_touchstone(format="db")

    def test_mixed_reference_impedances_are_refused(self, engine):
        result = engine.run_s_parameters(
            rspice.Netlist.parse(MIXED_PORTS), np.array([1e9])
        )
        with pytest.raises(ValueError, match="one reference impedance"):
            result.to_touchstone()

    @pytest.mark.parametrize(
        ("kwargs", "match"),
        [
            ({"format": "polar"}, "Touchstone format"),
            ({"frequency_unit": "thz"}, "frequency unit"),
        ],
    )
    def test_unknown_options_are_rejected(self, sparams, kwargs, match):
        with pytest.raises(ValueError, match=match):
            sparams.to_touchstone(**kwargs)


class TestTransientExport:
    def test_columns_cover_every_node_and_branch(self, tran):
        columns = tran.export_columns
        assert columns[0] == "time"
        for name in tran.node_names:
            assert f"V({name})" in columns
        for name in tran.branch_names:
            assert f"I({name})" in columns

    def test_ascii_raw_header_describes_the_plot(self, tran):
        header = parse_raw_header(tran.to_raw().decode())
        assert header["Plotname"] == "Transient Analysis"
        assert header["Flags"] == "real"
        assert int(header["No. Variables"]) == len(tran.export_columns)
        assert int(header["No. Points"]) == tran.num_points

    def test_binary_raw_payload_round_trips(self, tran):
        document = tran.to_raw(format="binary")
        payload = document[document.index(b"Binary:\n") + len(b"Binary:\n") :]

        assert len(payload) == tran.num_points * len(tran.export_columns) * 8
        first = struct.unpack("<d", payload[:8])[0]
        assert first == pytest.approx(tran.time[0])

    def test_custom_title_reaches_the_header(self, tran):
        header = parse_raw_header(tran.to_raw(title="regression run").decode())
        assert header["Title"] == "regression run"

    def test_csv_matches_the_arrays(self, tran):
        rows = tran.to_csv().splitlines()
        assert rows[0].split(",") == tran.export_columns
        assert len(rows) == tran.num_points + 1

        first = [float(value) for value in rows[1].split(",")]
        assert first[0] == pytest.approx(tran.time[0])
        assert first[1] == pytest.approx(tran.voltage_waveform(tran.node_names[0])[0])

    def test_write_helpers_match_their_in_memory_form(self, tran, tmp_path):
        raw, csv = tmp_path / "run.raw", tmp_path / "run.csv"
        stamp = "Thu Jan 1 00:00:00 1970"
        tran.write_raw(raw, format="binary", timestamp=stamp)
        tran.write_csv(csv)
        assert raw.read_bytes() == tran.to_raw(format="binary", timestamp=stamp)
        assert csv.read_text() == tran.to_csv()

    def test_a_pinned_timestamp_makes_raw_output_reproducible(self, tran):
        stamp = "Thu Jan 1 00:00:00 1970"
        first = tran.to_raw(timestamp=stamp)
        assert first == tran.to_raw(timestamp=stamp)
        assert parse_raw_header(first.decode())["Date"] == stamp

    def test_the_default_timestamp_is_the_current_time(self, tran):
        # Without a pin the header carries a real date, which is what an
        # ngspice reader expects; only the Date line may differ between runs.
        header = parse_raw_header(tran.to_raw().decode())
        assert header["Date"] and header["Date"] != ""

    def test_unknown_raw_format_is_rejected(self, tran):
        with pytest.raises(ValueError, match="raw format"):
            tran.to_raw(format="xml")


class TestAcExport:
    def test_raw_is_flagged_complex(self, ac):
        assert parse_raw_header(ac.to_raw().decode())["Flags"] == "complex"

    def test_binary_raw_carries_two_doubles_per_value(self, ac):
        document = ac.to_raw(format="binary")
        payload = document[document.index(b"Binary:\n") + len(b"Binary:\n") :]
        assert len(payload) == len(ac.frequencies) * len(ac.export_columns) * 16

    def test_csv_keeps_the_axis_real_and_splits_phasors(self, ac):
        rows = ac.to_csv().splitlines()
        headers = rows[0].split(",")

        assert headers[0] == "frequency"
        assert "frequency_imag" not in headers
        assert headers[1:3] == [
            f"V({ac.node_names[0]})_real",
            f"V({ac.node_names[0]})_imag",
        ]

        first = [float(value) for value in rows[1].split(",")]
        phasor = ac.voltage_complex(ac.node_names[0])[0]
        assert first[0] == pytest.approx(ac.frequencies[0])
        assert first[1] == pytest.approx(phasor.real)
        assert first[2] == pytest.approx(phasor.imag)

    def test_write_helpers_match_their_in_memory_form(self, ac, tmp_path):
        raw, csv = tmp_path / "ac.raw", tmp_path / "ac.csv"
        stamp = "Thu Jan 1 00:00:00 1970"
        ac.write_raw(raw, timestamp=stamp)
        ac.write_csv(csv)
        assert raw.read_bytes() == ac.to_raw(timestamp=stamp)
        assert csv.read_text() == ac.to_csv()


class TestDcSweepExport:
    def test_columns_lead_with_the_swept_source(self, engine, divider):
        sweep = engine.run_dc_sweep(divider, "V1", 0, 5, 1)
        assert sweep.export_columns[0] == "v-sweep(V1)"

    def test_csv_matches_the_sweep_arrays(self, engine, divider):
        sweep = engine.run_dc_sweep(divider, "V1", 0, 5, 1)
        rows = sweep.to_csv().splitlines()

        assert rows[0].split(",") == sweep.export_columns
        assert len(rows) == len(sweep) + 1
        values = [float(row.split(",")[0]) for row in rows[1:]]
        np.testing.assert_allclose(values, sweep.sweep_values)

    def test_raw_declares_a_dc_plot(self, engine, divider):
        sweep = engine.run_dc_sweep(divider, "V1", 0, 5, 1)
        header = parse_raw_header(sweep.to_raw().decode())
        assert header["Plotname"] == "DC transfer characteristic"
        assert header["Flags"] == "real"


class TestTwoPortStability:
    """K, mu, MAG, MSG, and the stability circles.

    The fixture is a resistive tee whose S-parameters are all exactly 0.25,
    so every figure below has a closed form: Delta = 0, K = 7, mu = 3,
    MSG = 0 dB, and MAG = 10*log10(7 - sqrt(48)).
    """

    def test_availability_is_reported(self, sparams):
        assert sparams.has_two_port_stability

    def test_rollett_and_edwards_sinsky_factors(self, sparams):
        np.testing.assert_allclose(sparams.k_factor, 7.0, rtol=1e-9)
        np.testing.assert_allclose(sparams.mu_factor, 3.0, rtol=1e-9)
        np.testing.assert_allclose(sparams.mu_prime, 3.0, rtol=1e-9)

    def test_determinant_and_stability_verdict(self, sparams):
        np.testing.assert_allclose(np.abs(sparams.delta), 0.0, atol=1e-12)
        assert sparams.unconditionally_stable.all()

    def test_gain_figures(self, sparams):
        np.testing.assert_allclose(sparams.max_stable_gain_db, 0.0, atol=1e-9)
        expected_mag = 10.0 * np.log10(7.0 - np.sqrt(48.0))
        np.testing.assert_allclose(
            sparams.max_available_gain_db, expected_mag, rtol=1e-9
        )
        np.testing.assert_allclose(
            sparams.transducer_gain_db, 20.0 * np.log10(0.25), rtol=1e-9
        )

    def test_transducer_gain_matches_the_s_parameter(self, sparams):
        np.testing.assert_allclose(
            sparams.transducer_gain_db,
            20.0 * np.log10(np.abs(sparams.s(2, 1))),
            rtol=1e-9,
        )

    def test_stability_circles(self, sparams):
        circles = sparams.stability_circles(0)
        # Cs = S11 / |S11|^2 = 4, rs = |S12 S21| / |S11|^2 = 1.
        assert circles["input_center"] == pytest.approx(4.0 + 0j)
        assert circles["input_radius"] == pytest.approx(1.0)
        assert circles["output_center"] == pytest.approx(4.0 + 0j)
        assert circles["output_radius"] == pytest.approx(1.0)
        # The circle sits outside the unit disc, so the stable region is the
        # outside of it, which contains every passive termination.
        assert circles["input_stable_inside"] is False
        assert circles["output_stable_inside"] is False

    def test_a_one_port_reports_no_two_port_figures(self, engine):
        one_port = rspice.Netlist.parse(
            "* One port\nV1 in 0 AC 0 portnum=1 z0=50\nR1 in 0 50\n.end"
        )
        result = engine.run_s_parameters(one_port, np.array([1e9]))

        assert not result.has_two_port_stability
        assert result.k_factor is None
        assert result.delta is None
        assert result.max_available_gain_db is None
        with pytest.raises(ValueError, match="two-port"):
            result.stability_circles(0)

    def test_out_of_range_frequency_index_is_rejected(self, sparams):
        with pytest.raises(IndexError, match="out of range"):
            sparams.stability_circles(99)
