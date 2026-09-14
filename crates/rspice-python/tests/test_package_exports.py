"""The extension and maturin's star-import package expose the same result types."""

import rspice


def test_current_impulse_type_survives_package_star_import():
    exports = {}
    exec("from rspice import *", exports)
    assert exports["CurrentImpulseTrace"] is rspice.CurrentImpulseTrace
