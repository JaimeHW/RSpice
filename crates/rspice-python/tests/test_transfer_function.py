"""Small-signal transfer function (`.TF`)."""

import math

import pytest

import rspice



class TestTransferFunction:
    def test_divider_gain_and_impedances(self, engine, divider):
        tf = engine.run_transfer_function(divider, "out", "V1")
        assert tf.gain == pytest.approx(0.5, rel=1e-6)
        assert tf.input_impedance == pytest.approx(2000.0, rel=1e-6)
        # Thevenin output impedance: 1k || 1k = 500.
        assert tf.output_impedance == pytest.approx(500.0, rel=1e-6)
        assert tf.gain_db == pytest.approx(20 * math.log10(0.5), rel=1e-6)

    def test_unknown_source_raises(self, engine, divider):
        with pytest.raises(rspice.SimulationError):
            engine.run_transfer_function(divider, "out", "V99")
