"""Tests for Rust -> Python metadata codegen (quantwave-iqq7)."""

import quantwave as qw


def test_generated_metadata_registry_populated():
    names = qw.indicators()
    assert len(names) >= 200, f"expected 200+ indicators from Rust codegen, got {len(names)}"


def test_hand_override_wins_for_rsi():
    meta = qw.metadata("rsi")
    assert meta is not None
    assert meta.warmup_bars == 14
    assert "close" in meta.data_inputs


def test_sr_monitor_from_overlay():
    meta = qw.metadata("sr_monitor")
    assert meta is not None
    assert meta.warmup_bars == 0
    assert "interaction_count" in meta.outputs