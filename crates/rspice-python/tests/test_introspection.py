"""Netlist introspection and parameter override.

A netlist is the unit of work an automation script passes around, so it has
to be inspectable and derivable without falling back to string-editing the
deck.
"""

import pickle

import pytest

import rspice

DECK = """* Amp with parameters
.param rval=1k cval=1u
V1 in 0 10
R1 in out {rval}
C1 out 0 {cval}
M1 d g s b NMOS W=2u L=180n
* M1 is here to be introspected, not driven. Every node still needs a DC path
* to ground for an operating point to exist, and a 1T tie supplies one without
* conducting or reaching `out`.
Rd d 0 1T
Rg g 0 1T
Rs s 0 1T
Rb b 0 1T
.model NMOS NMOS(LEVEL=1 VTO=0.5)
.subckt cell a b
.param rval=99
Rin a b {rval}
.ends
Xc1 out 0 cell
.end
"""


@pytest.fixture()
def netlist():
    return rspice.Netlist.parse(DECK)


class TestElements:
    def test_every_instance_is_listed_in_deck_order(self, netlist):
        assert [element.name for element in netlist.elements] == netlist.element_names

    def test_passive_carries_value_and_nodes(self, netlist):
        resistor = netlist.element("R1")
        assert resistor.kind == "Resistor"
        assert resistor.nodes == ["IN", "OUT"]
        assert resistor.value == pytest.approx(1000.0)
        assert resistor.model is None

    def test_capacitor_value_resolves_the_parameter(self, netlist):
        assert netlist.element("C1").value == pytest.approx(1e-6)

    def test_modelled_device_carries_model_and_instance_parameters(self, netlist):
        mosfet = netlist.element("M1")
        assert mosfet.kind == "Mosfet"
        assert mosfet.model == "NMOS"
        assert mosfet.nodes == ["D", "G", "S", "B"]
        assert mosfet.instance_params["W"] == pytest.approx(2e-6)
        assert mosfet.instance_params["L"] == pytest.approx(180e-9)

    def test_subcircuit_instance_is_reported(self, netlist):
        assert netlist.element("Xc1").kind == "Subcircuit"

    def test_lookup_is_case_insensitive(self, netlist):
        assert netlist.element("r1").name == netlist.element("R1").name

    def test_unknown_element_raises(self, netlist):
        with pytest.raises(KeyError, match="nope"):
            netlist.element("nope")

    def test_elements_are_a_read_only_projection(self, netlist):
        element = netlist.element("R1")
        assert repr(element).startswith("Element(name='R1'")
        with pytest.raises(AttributeError):
            element.value = 5.0


class TestNodes:
    def test_authored_nodes_are_listed_without_ground(self, netlist):
        assert netlist.node_names == ["B", "D", "G", "IN", "OUT", "S"]

    def test_ground_is_excluded(self):
        simple = rspice.Netlist.parse("V1 a 0 1\nR1 a gnd 1k\n.end")
        assert simple.node_names == ["A"]


class TestParameters:
    def test_parameters_expose_canonical_names(self, netlist):
        assert netlist.parameters["RVAL"] == pytest.approx(1000.0)
        assert netlist.parameters["CVAL"] == pytest.approx(1e-6)

    def test_lookup_is_case_insensitive(self, netlist):
        assert netlist.parameter("rval") == netlist.parameter("RVAL")

    def test_unknown_parameter_raises(self, netlist):
        with pytest.raises(KeyError, match="nope"):
            netlist.parameter("nope")


class TestWithParameters:
    def test_override_changes_the_simulated_result(self, engine, netlist):
        # At DC the capacitor is open, so `out` is loaded only by the 99 ohm
        # resistor inside the subcircuit: V(out) = 10 * 99 / (rval + 99).
        for rval, expected in ((1e3, 10 * 99 / 1099), (4e3, 10 * 99 / 4099)):
            derived = netlist.with_parameters({"rval": rval})
            assert derived.parameter("rval") == pytest.approx(rval)
            assert engine.run_dc_op(derived).voltage("out") == pytest.approx(
                expected, rel=1e-6
            )

    def test_the_original_netlist_is_untouched(self, netlist):
        netlist.with_parameters({"rval": 9e3})
        assert netlist.parameter("rval") == pytest.approx(1000.0)

    def test_subcircuit_scoped_parameters_are_left_alone(self, netlist):
        derived = netlist.with_parameters({"rval": 4e3})
        assert ".param rval=99" in derived.source

    def test_a_new_name_gets_its_own_card(self, netlist):
        derived = netlist.with_parameters({"brandnew": 42.0})
        assert derived.parameter("brandnew") == pytest.approx(42.0)

    def test_multiple_overrides_apply_together(self, netlist):
        derived = netlist.with_parameters({"rval": 2e3, "cval": 2e-6})
        assert derived.parameter("rval") == pytest.approx(2e3)
        assert derived.parameter("cval") == pytest.approx(2e-6)

    def test_rewrite_is_deterministic(self, netlist):
        first = netlist.with_parameters({"rval": 2e3, "cval": 2e-6}).source
        second = netlist.with_parameters({"cval": 2e-6, "rval": 2e3}).source
        assert first == second

    def test_derived_netlist_is_picklable(self, netlist):
        derived = netlist.with_parameters({"rval": 4e3})
        assert pickle.loads(pickle.dumps(derived)).parameter("rval") == pytest.approx(
            4e3
        )

    def test_empty_mapping_is_rejected(self, netlist):
        with pytest.raises(ValueError, match="at least one parameter"):
            netlist.with_parameters({})

    @pytest.mark.parametrize("value", [float("nan"), float("inf"), float("-inf")])
    def test_non_finite_values_are_rejected(self, netlist, value):
        with pytest.raises(ValueError, match="finite"):
            netlist.with_parameters({"rval": value})

    def test_brace_expressions_survive_the_rewrite(self, engine):
        deck = """* Expression parameters
.param base=1k scale='2*base'
V1 in 0 10
R1 in out {base}
R2 out 0 {scale}
.end
"""
        netlist = rspice.Netlist.parse(deck)
        derived = netlist.with_parameters({"base": 2e3})
        assert derived.parameter("base") == pytest.approx(2e3)
        # The dependent expression is re-evaluated against the new base.
        assert derived.parameter("scale") == pytest.approx(4e3)
