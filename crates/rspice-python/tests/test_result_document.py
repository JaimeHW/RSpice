"""The shared result document, exposed uniformly on every result family.

Every family in this binding answers `signals()`, `scalars()`,
`device_observables()` and `document()` over the one
`rspice-analysis-result` document the CLI, the WASM build and the engine
adapter publish. These tests are the proof that the surface has no per-family
subset: the family corpus below is checked exhaustively, so a family that
stops answering — or a new one that never starts — fails here.
"""

import pickle

import numpy as np
import pytest

import rspice

DIVIDER = """* Voltage divider
V1 in 0 DC 10
R1 in out 1k
R2 out 0 1k
"""

RC = """* RC lowpass
V1 in 0 DC 0 AC 1 SIN(0 1 1k)
R1 in out 1k
C1 out 0 1u
"""

TWO_PORT = """* Resistive pad
V1 p1 0 AC 1 portnum=1 z0=50
V2 p2 0 AC 0 portnum=2 z0=50
RA p1 mid 25
RB mid 0 50
RC mid p2 25
"""

RF = """* RF one-pole
V1 in 0 SIN(0 1 1G)
R1 in out 1k
C1 out 0 1p
"""

DISTORTION_DIODE = """* harmonic distortion
V1 out 0 DC 0.5 DISTOF1 1m 0
D1 out 0 DM
.model DM D(IS=1e-12 N=1 CJO=0 TT=0)
"""


def parse(deck: str) -> rspice.Netlist:
    return rspice.Netlist.parse(deck + ".end\n")


def report_for(deck: str) -> rspice.RunReport:
    return rspice.Engine().run(parse(deck), continue_on_error=False)


def deck_results():
    """One live result per family this binding publishes, with its result kind.

    The `.MEAS`-free decks are the smallest ones that make each family run;
    the point of the corpus is coverage of the families, not of the circuits.
    """
    op = report_for(DIVIDER + ".op\n")
    dc = report_for(DIVIDER + ".dc V1 0 10 5\n")
    tran = report_for(RC + ".tran 10u 1m\n")
    ac = report_for(RC + ".ac dec 3 10 10k\n")
    noise = report_for(RC + ".noise V(out) V1 dec 2 100 1k\n")
    tf = report_for(DIVIDER + ".tf V(out) V1\n")
    stb = report_for(
        "* loop\nE1 eo 0 ctrl 0 -1000\nVPROBE eo x 0\nR1 x ctrl 1k\n"
        "C1 ctrl 0 159.154943091895n\n.stb dec 2 10 10meg probe=vprobe\n"
    )
    pz = report_for(RC + ".pz in 0 out 0 vol pz\n")
    sens = report_for(DIVIDER + ".sens V(out)\n")
    sens_ac = report_for(RC + ".sens V(out) ac dec 2 10 1k\n")
    disto = report_for(DISTORTION_DIODE + ".disto dec 2 100 1k\n")
    sp = report_for(TWO_PORT + ".sp lin 2 1meg 2meg\n")
    mc = report_for(DIVIDER + ".mc 3 SEED 7 GAUSS 0.01\n")
    hb = report_for(RF + ".hb 1g\n")
    pss = report_for(RF + ".pss fund=1g\n")
    pac = report_for(RF + ".pss fund=1g\n.pac dec 2 1k 10k input=v1 out=v(out)\n")
    pnoise = report_for(RF + ".pss fund=1g\n.pnoise dec 2 1k 10k out=v(out)\n")
    envelope = report_for(RF + ".hb 1g\n.envelope tstop=1n\n")
    four = report_for(RC + ".tran 10u 4m\n.four 1k v(out)\n")
    fft = report_for(RC + ".tran 10u 1m\n.fft v(out) np=8 freq=1k\n")

    return [
        ("op", op.op),
        ("dc", dc.dc),
        ("tran", tran.tran),
        ("ac", ac.ac),
        ("noise", noise.noise[0]),
        ("tf", tf.tf),
        ("stb", stb.stb),
        ("pole-zero", pz.pz),
        ("sensitivity", sens.sensitivity),
        ("sensitivity", sens_ac.sensitivity_ac),
        ("distortion", disto.distortion),
        ("sp", sp.s_parameters),
        ("monte-carlo", mc.monte_carlo),
        ("hb", hb.hb),
        ("pss", pss.pss),
        ("pac", pac.pac),
        ("pnoise", pnoise.pnoise),
        ("envelope", envelope.envelope),
        ("fourier", four.fourier[0]),
        ("fft", fft.tran.fft_results[0]),
    ]


@pytest.fixture(scope="module")
def results():
    return deck_results()


def test_every_family_publishes_the_shared_document(results):
    """No family is exempt: each answers all four accessors over its document."""
    for kind, result in results:
        document = result.document()
        assert document["schema"] == "rspice-analysis-result", kind
        # Version 2 is what this build writes: a transient payload may carry
        # digital bus declarations, which a version-1 document could not.
        assert document["schemaVersion"] == 2, kind
        assert document["resultKind"] == kind, kind
        assert document["analysis"]["tag"], kind

        descriptors = result.signals()
        assert isinstance(descriptors, list), kind
        for descriptor in descriptors:
            assert isinstance(descriptor, rspice.SignalDescriptor), kind
            assert descriptor.name, kind
            assert descriptor.kind in {
                "voltage",
                "current",
                "device_observable",
                "scalar",
                "digital",
            }, (kind, descriptor.kind)
            assert descriptor.availability in {
                "available",
                "not_projected",
                "absent_at_coordinate",
            }, kind
            assert descriptor.owner_kind in {"node", "branch", "device", "analysis"}, kind
            assert (descriptor.owner is None) == (descriptor.owner_kind == "analysis"), kind
            assert descriptor.point_count == document["pointCount"], kind

        for scalar in result.scalars():
            assert isinstance(scalar, rspice.ResultScalar), kind
            assert scalar.name, kind
            if scalar.representation == "unavailable":
                assert scalar.value is None, kind
                assert scalar.unavailable_reason, kind

        for observable in result.device_observables():
            assert isinstance(observable, rspice.DeviceObservable), kind
            assert observable.device and observable.parameter, kind
            assert len(observable.validity) == len(observable), kind


def test_the_document_is_the_same_inventory_the_accessors_report(results):
    """The typed accessors and the JSON view are two views of one document."""
    for kind, result in results:
        document = result.document()
        assert [descriptor.name for descriptor in result.signals()] == [
            signal["descriptor"]["canonicalName"] for signal in document["signals"]
        ], kind
        assert [scalar.name for scalar in result.scalars()] == [
            scalar["name"] for scalar in document["scalars"]
        ], kind
        published = [
            (state["deviceName"], parameter["name"])
            for state in document["deviceStates"]
            for parameter in state["parameters"]
        ]
        assert [
            (observable.device, observable.parameter)
            for observable in result.device_observables()
        ] == published, kind


def test_a_device_observable_reports_absence_rather_than_a_zero():
    """An operating point publishes its device inventory with typed presence."""
    report = report_for(
        "* one diode\nV1 in 0 DC 0.7\nR1 in a 100\nD1 a 0 DMOD\n"
        ".model DMOD D(IS=1e-14)\n.op\n"
    )
    observables = report.op.device_observables()
    assert observables, "the operating point captured a device report"
    by_device = {observable.device for observable in observables}
    assert "d1" in by_device or "D1" in by_device
    for observable in observables:
        values = observable.values
        assert values.dtype == np.float64
        assert len(observable.validity) == values.size
        for present, value in zip(observable.validity, values):
            assert present == (not np.isnan(value))


def test_an_unbounded_margin_is_a_determination_not_a_number():
    """A loop with no phase crossover has no gain margin, and says so."""
    report = report_for(
        "* single pole loop\nE1 eo 0 ctrl 0 -1000\nVPROBE eo x 0\nR1 x ctrl 1k\n"
        "C1 ctrl 0 159.154943091895n\n.stb dec 2 10 10meg probe=vprobe\n"
    )
    margins = {scalar.name: scalar for scalar in report.stb.scalars()}
    margin = margins["gain_margin_db"]
    assert margin.representation == "unavailable"
    assert margin.value is None
    assert margin.unavailable_reason in {
        "positive_infinity",
        "negative_infinity",
        "no_crossover",
    }


def test_each_authored_card_publishes_under_its_own_identity():
    """A deck with two cards of one family does not report both as `-001`."""
    report = report_for(RC + ".ac dec 2 10 100\n.ac dec 2 100 1k\n")
    assert [result.document()["analysis"]["tag"] for result in report.all_ac] == [
        "ac-001",
        "ac-002",
    ]


def test_a_coordinate_result_names_the_coordinate_it_was_solved_at():
    """A `.STEP` sweep publishes one document per coordinate, each placed."""
    report = report_for(
        "* stepped divider\n.param rval=1k\nV1 in 0 DC 10\nR1 in out {rval}\n"
        "R2 out 0 1k\n.step param rval list 1k 2k\n.op\n"
    )
    documents = [result.document() for result in report.all_op]
    assert len(documents) == 2
    labels = [document["coordinate"]["label"] for document in documents]
    assert len(set(labels)) == 2
    for document in documents:
        assert document["analysis"]["tag"] == "op-001"


def test_a_pickled_result_says_its_document_did_not_survive_the_round_trip():
    """Pickled state is this binding's projection, not the core evidence."""
    report = report_for(RC + ".ac dec 3 10 10k\n")
    restored = pickle.loads(pickle.dumps(report.ac))
    # The projection itself round-trips exactly; only the document does not.
    assert restored.frequencies.tolist() == report.ac.frequencies.tolist()
    for accessor in ("signals", "scalars", "device_observables", "document"):
        with pytest.raises(rspice.RSpiceNotImplementedError) as excinfo:
            getattr(restored, accessor)()
        assert "pickled state" in str(excinfo.value)


def test_a_convenience_call_publishes_the_single_analysis_identity():
    """A direct call runs exactly one card, which is that family's first."""
    result = rspice.Engine().run_ac(parse(RC), [10.0, 100.0, 1000.0])
    assert result.document()["analysis"]["tag"] == "ac-001"
    assert [descriptor.kind for descriptor in result.signals()] != []
