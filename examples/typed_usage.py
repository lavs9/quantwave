"""Typed usage sample for pyright/mypy (quantwave-5ipk.3)."""

from __future__ import annotations

import quantwave as qw


def main() -> None:
    names = qw.indicators()
    assert "rsi" in names
    meta = qw.metadata("rsi")
    assert meta is not None
    cls = qw.streaming_class("rsi")
    assert cls is not None
    _ = qw.ta.rsi


if __name__ == "__main__":
    main()