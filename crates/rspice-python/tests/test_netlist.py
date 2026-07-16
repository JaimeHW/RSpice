"""Netlist parsing semantics and introspection."""

import pathlib

import pytest

import rspice


class TestParseStatementSemantics:
    def test_parse_diagnostics_are_public_typed_objects(self):
        netlist = rspice.Netlist.parse("* diagnostic\nR1 1 0\n.end")
        assert len(netlist.diagnostics) == 1
        diagnostic = netlist.diagnostics[0]
        assert isinstance(diagnostic, rspice.ParseDiagnostic)
        assert type(diagnostic).__module__ == "rspice"
        assert diagnostic.line == 2
        assert diagnostic.severity == "warning"
        assert diagnostic.code == "xyce_resistor_missing_value"
        assert diagnostic.message

    def test_titleless_content_parses_all_elements(self):
        netlist = rspice.Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")
        assert netlist.num_elements == 2

    def test_leading_comment_becomes_title(self):
        netlist = rspice.Netlist.parse("* My circuit\nV1 1 0 10\nR1 1 0 1k\n.end")
        assert "My circuit" in netlist.title
        assert netlist.num_elements == 2

    def test_single_statement_no_end(self):
        netlist = rspice.Netlist.parse("V1 1 0 10")
        assert netlist.num_elements == 1

    def test_first_line_typo_raises_instead_of_becoming_title(self):
        # Historically "Vfoo 1 0 banana" was silently consumed as a title,
        # yielding an empty circuit. parse() must surface the error.
        with pytest.raises(rspice.ParseError) as exc_info:
            rspice.Netlist.parse("Vfoo 1 0 banana\n.end")
        assert exc_info.value.kind
        assert hasattr(exc_info.value, "line")
        assert hasattr(exc_info.value, "detail")

    def test_statement_like_title_is_not_eaten_as_device(self):
        # A title like "C3 Amplifier Design 2024" must not become a 2024 F
        # capacitor. In parse(), titles are comment lines.
        netlist = rspice.Netlist.parse(
            "* C3 Amplifier Design 2024\nV1 1 0 10\nR1 1 0 1k\n.end"
        )
        assert netlist.num_elements == 2

    def test_blank_leading_lines_are_tolerated(self):
        netlist = rspice.Netlist.parse("\n\n  \nV1 1 0 10\nR1 1 0 1k\n.end")
        assert netlist.num_elements == 2


class TestParseSpice:
    def test_first_line_is_always_title(self):
        netlist = rspice.Netlist.parse_spice("My Amplifier\nV1 1 0 10\nR1 1 0 1k\n.end")
        assert netlist.num_elements == 2
        assert "My Amplifier" in netlist.title

    def test_statement_looking_title_is_still_title(self):
        netlist = rspice.Netlist.parse_spice(
            "C3 Amplifier Design 2024\nV1 1 0 10\nR1 1 0 1k\n.end"
        )
        assert netlist.num_elements == 2


class TestParseFile:
    def test_parse_file_accepts_pathlike(self, tmp_path: pathlib.Path):
        deck = tmp_path / "divider.sp"
        deck.write_text("File divider\nV1 1 0 10\nR1 1 0 1k\n.end\n")
        netlist = rspice.Netlist.parse_file(deck)
        assert netlist.num_elements == 2

    def test_parse_file_missing_raises(self, tmp_path: pathlib.Path):
        with pytest.raises(rspice.ParseError):
            rspice.Netlist.parse_file(tmp_path / "nope.sp")

    def test_parse_with_includes(self, tmp_path: pathlib.Path):
        (tmp_path / "load.inc").write_text("R2 out 0 1k\n")
        netlist = rspice.Netlist.parse_with_includes(
            "V1 in 0 10\nR1 in out 1k\n.include load.inc\n.end", tmp_path
        )
        assert netlist.num_elements == 3

    def test_missing_ends_exposes_included_source_provenance(
        self, tmp_path: pathlib.Path
    ):
        child = tmp_path / "missing.ends"
        child.write_text(".subckt testsub a b\nR1 a b 1\n")
        deck = tmp_path / "deck.cir"
        deck.write_text("missing ends\n.include missing.ends\n.end\n")

        with pytest.raises(rspice.ParseError) as exc_info:
            rspice.Netlist.parse_file(deck)

        error = exc_info.value
        assert error.kind == "missing_subcircuit_ends"
        assert error.line == 1
        assert pathlib.Path(error.source).resolve() == child.resolve()
        assert error.detected_line == 3
        assert pathlib.Path(error.detected_source).resolve() == child.resolve()
        assert error.boundary == "end_of_source"
        assert error.authored_name == "testsub"
        assert error.canonical_name == "TESTSUB"
        assert error.qualified_name == "TESTSUB"
        assert error.detail == "TESTSUB"

    def test_initcond_duplicate_exposes_typed_primary_and_related_origins(self):
        with pytest.raises(rspice.ParseError) as exc_info:
            rspice.Netlist.parse_spice(
                "duplicate initcond\n"
                ".INITCOND C1 IC=1\n"
                ".INITCOND malformed second card\n"
                "C1 1 0 1u\n"
                ".END\n"
            )

        error = exc_info.value
        assert error.kind == "device_initial_condition_duplicate_directive"
        assert error.category == "device_initial_condition"
        assert error.line == 3
        assert error.primary_line == 3
        assert error.source is None
        assert error.primary_source is None
        assert error.related_line == 2
        assert error.related_source is None
        assert error.device is None
        assert error.requested_path is None

    def test_initcond_source_failure_exposes_requested_path_and_source(
        self, tmp_path: pathlib.Path, monkeypatch: pytest.MonkeyPatch
    ):
        deck = tmp_path / "missing-initcond.cir"
        deck.write_text(
            "missing initcond\n"
            ".INITCOND FILE absent-initcond.dat\n"
            "C1 1 0 1u\n"
            ".END\n"
        )
        monkeypatch.chdir(tmp_path)

        with pytest.raises(rspice.ParseError) as exc_info:
            rspice.Netlist.parse_file(deck)

        error = exc_info.value
        assert error.kind == "device_initial_condition_source_unavailable"
        assert error.category == "device_initial_condition"
        assert error.line == 2
        assert error.primary_line == 2
        assert pathlib.Path(error.source).resolve() == deck.resolve()
        assert pathlib.Path(error.primary_source).resolve() == deck.resolve()
        assert error.requested_path == "absent-initcond.dat"


class TestIntrospection:
    def test_counts_and_names(self):
        netlist = rspice.Netlist.parse(
            """* Introspection target
.param rval=1k
V1 in 0 10
R1 in out {rval}
R2 out 0 1k
.model dmod D(IS=1e-14)
.tran 1u 1m
.meas tran vmax MAX V(out)
.end
"""
        )
        assert netlist.num_elements == 3
        assert netlist.element_names == ["V1", "R1", "R2"]
        assert netlist.num_models == 1
        assert [n.upper() for n in netlist.model_names] == ["DMOD"]
        assert netlist.num_analyses == 1
        assert netlist.analyses[0].startswith(".tran")
        assert netlist.num_measurements == 1
        assert [n.lower() for n in netlist.measurement_names] == ["vmax"]

    def test_repr_mentions_measurements(self):
        netlist = rspice.Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")
        assert "measurements=0" in repr(netlist)
