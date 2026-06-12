"""Netlist parsing semantics and introspection."""

import pathlib

import pytest

import rspice


class TestParseStatementSemantics:
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
        with pytest.raises(rspice.ParseError):
            rspice.Netlist.parse("Vfoo 1 0 banana\n.end")

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
