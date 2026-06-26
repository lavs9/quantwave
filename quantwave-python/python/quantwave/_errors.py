"""
Public exception hierarchy for quantwave (quantwave-1x2z).

Exception contract
------------------
* ``QuantwaveError`` — base for all library-raised errors you should catch.
* ``IndicatorNotFoundError`` — unknown indicator name in discovery/parity APIs.
* ``InvalidParameterError`` — bad params (negative period, missing required key).
* ``ParityError`` — batch vs streaming mismatch (subclass of AssertionError for
  pytest compatibility, but also QuantwaveError for unified handling).
* ``StreamingError`` — streaming path failed to run.
* ``InternalError`` — native FFI/uniffi failure (re-exported from ``__init__``).

Typical usage::

    import quantwave as qw

    try:
        qw.assert_parity("rsi", {"period": 14}, closes)
    except qw.ParityError as e:
        ...  # batch/stream diverged
    except qw.IndicatorNotFoundError:
        ...  # bad indicator name
    except qw.QuantwaveError:
        ...  # any other quantwave-specific error
"""


class QuantwaveError(Exception):
    """Base exception for quantwave errors."""


class IndicatorNotFoundError(QuantwaveError, ValueError):
    """Raised when an indicator name is not registered or has no streaming class."""


class InvalidParameterError(QuantwaveError, ValueError):
    """Raised when indicator parameters are invalid or incomplete."""


class StreamingError(QuantwaveError, RuntimeError):
    """Raised when the streaming execution path fails."""


class ParityError(QuantwaveError, AssertionError):
    """Raised when batch and streaming outputs disagree beyond tolerance."""