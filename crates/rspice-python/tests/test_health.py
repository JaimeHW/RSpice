"""Backend readiness contract."""

import pytest

import rspice


def test_engine_health_check_exercises_parser_and_solver(engine):
    report = engine.health_check()

    assert isinstance(report, rspice.HealthReport)
    assert report.status == "ready"
    assert report.ready is True
    assert report.duration_seconds >= 0.0
    assert report.element_count == 2
    assert report.node_count == 1
    assert report.branch_count == 1
    assert report.output_voltage == pytest.approx(1.0, abs=1e-12)
