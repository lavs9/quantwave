"""Machine Learning, Kolmogorov-Arnold Networks (KAN), and Technical Indicator Networks (TIN).

Provides Python access to QuantWave's high-performance Rust neural architectures:
- KAN (Kolmogorov-Arnold Networks): B-spline parameterized edge activations and symbolic formula extraction.
- TIN (Technical Indicator Networks): Differentiable technical indicators with analytical parameter gradients.
- PyTorch Autograd Bridge: End-to-end differentiable indicators for PyTorch nn.Module models.
"""

from __future__ import annotations

from typing import Any, List, Sequence, Tuple, Union
import warnings

try:
    from quantwave import _ml  # type: ignore
    _ml_available = True
except (ImportError, AttributeError) as _e:
    _ml = None
    _ml_available = False
    warnings.warn(f"quantwave._ml extension unavailable: {_e}")

if _ml_available:
    KanNetwork = _ml.KanNetwork
    KanIndicator = _ml.KanIndicator
    DifferentiableEma = _ml.DifferentiableEma
    DifferentiableRsi = _ml.DifferentiableRsi
    DifferentiableMacd = _ml.DifferentiableMacd
    extract_symbolic_formula = _ml.py_extract_symbolic_formula
    extract_feature_importance = _ml.py_extract_feature_importance
else:
    # Fallback placeholders when extension is not built
    class _Dummy:
        def __init__(self, *args: Any, **kwargs: Any) -> None:
            raise RuntimeError(
                "quantwave._ml compiled extension is not available. "
                "Ensure quantwave wheel is installed with native binaries."
            )

    KanNetwork = _Dummy  # type: ignore
    KanIndicator = _Dummy  # type: ignore
    DifferentiableEma = _Dummy  # type: ignore
    DifferentiableRsi = _Dummy  # type: ignore
    DifferentiableMacd = _Dummy  # type: ignore

    def extract_symbolic_formula(*args: Any, **kwargs: Any) -> str:
        raise RuntimeError("quantwave._ml compiled extension is not available.")

    def extract_feature_importance(*args: Any, **kwargs: Any) -> list[dict[str, Any]]:
        raise RuntimeError("quantwave._ml compiled extension is not available.")

__all__ = [
    "KanNetwork",
    "KanIndicator",
    "DifferentiableEma",
    "DifferentiableRsi",
    "DifferentiableMacd",
    "extract_symbolic_formula",
    "extract_feature_importance",
]

# ---------------------------------------------------------------------------
# Optional PyTorch Autograd Bridge
# ---------------------------------------------------------------------------
try:
    import torch
    import torch.nn as nn
    _has_torch = True
except ImportError:
    _has_torch = False

if _has_torch and _ml_available:

    class DiffEmaFunction(torch.autograd.Function):
        """Autograd Function for DifferentiableEma using Rust analytical gradients."""

        @staticmethod
        def forward(ctx: Any, prices: torch.Tensor, span: torch.Tensor) -> torch.Tensor:
            span_val = float(span.detach().item())
            prices_list = prices.detach().cpu().to(torch.float64).numpy().tolist()

            ema = DifferentiableEma(span=span_val)
            smoothed_vals, grad_spans = ema.batch(prices_list)

            ctx.save_for_backward(prices, span)
            ctx.grad_spans = torch.tensor(grad_spans, dtype=prices.dtype, device=prices.device)
            ctx.alpha = 2.0 / (span_val + 1.0)

            return torch.tensor(smoothed_vals, dtype=prices.dtype, device=prices.device)

        @staticmethod
        def backward(ctx: Any, grad_output: torch.Tensor) -> Tuple[torch.Tensor, torch.Tensor]:
            prices, span = ctx.saved_tensors
            grad_spans = ctx.grad_spans
            alpha = ctx.alpha

            # Gradient w.r.t span: sum_t (dL_t * dY_t / dTau)
            grad_span = (grad_output * grad_spans).sum()

            # Gradient w.r.t input prices: reverse IIR adjoint filtering
            dL = grad_output.detach().cpu().to(torch.float64).numpy()
            n = len(dL)
            adj = 0.0
            grad_x = [0.0] * n
            for t in range(n - 1, -1, -1):
                adj = dL[t] + (1.0 - alpha) * adj
                grad_x[t] = alpha * adj

            grad_prices = torch.tensor(grad_x, dtype=prices.dtype, device=prices.device)
            return grad_prices, grad_span

    class LearnableEma(nn.Module):
        """PyTorch nn.Module wrapping DifferentiableEma with a learnable span parameter."""

        def __init__(
            self,
            initial_span: float = 10.0,
            min_span: float = 1.0,
            max_span: float = 100.0,
        ) -> None:
            super().__init__()
            self.span = nn.Parameter(torch.tensor(float(initial_span)))
            self.min_span = min_span
            self.max_span = max_span

        def forward(self, prices: torch.Tensor) -> torch.Tensor:
            clamped_span = torch.clamp(self.span, self.min_span, self.max_span)
            return DiffEmaFunction.apply(prices, clamped_span)

    class DiffRsiFunction(torch.autograd.Function):
        """Autograd Function for DifferentiableRsi using Rust analytical gradients."""

        @staticmethod
        def forward(
            ctx: Any,
            prices: torch.Tensor,
            span: torch.Tensor,
            overbought: float = 70.0,
            oversold: float = 30.0,
            steepness: float = 0.1,
        ) -> Tuple[torch.Tensor, torch.Tensor]:
            span_val = float(span.detach().item())
            prices_list = prices.detach().cpu().to(torch.float64).numpy().tolist()

            rsi = DifferentiableRsi(
                span=span_val,
                overbought=overbought,
                oversold=oversold,
                steepness=steepness,
            )
            rsi_vals, sig_vals, grad_spans = rsi.batch(prices_list)

            ctx.save_for_backward(prices, span)
            ctx.grad_spans = torch.tensor(grad_spans, dtype=prices.dtype, device=prices.device)

            out_rsi = torch.tensor(rsi_vals, dtype=prices.dtype, device=prices.device)
            out_sig = torch.tensor(sig_vals, dtype=prices.dtype, device=prices.device)
            return out_rsi, out_sig

        @staticmethod
        def backward(
            ctx: Any, grad_rsi: torch.Tensor, grad_sig: torch.Tensor
        ) -> Tuple[None, torch.Tensor, None, None, None]:
            _prices, _span = ctx.saved_tensors
            grad_spans = ctx.grad_spans
            grad_span = (grad_rsi * grad_spans).sum()
            return None, grad_span, None, None, None

    class LearnableRsi(nn.Module):
        """PyTorch nn.Module wrapping DifferentiableRsi with a learnable span parameter."""

        def __init__(
            self,
            initial_span: float = 14.0,
            overbought: float = 70.0,
            oversold: float = 30.0,
            steepness: float = 0.1,
            min_span: float = 2.0,
            max_span: float = 100.0,
        ) -> None:
            super().__init__()
            self.span = nn.Parameter(torch.tensor(float(initial_span)))
            self.overbought = overbought
            self.oversold = oversold
            self.steepness = steepness
            self.min_span = min_span
            self.max_span = max_span

        def forward(self, prices: torch.Tensor) -> Tuple[torch.Tensor, torch.Tensor]:
            clamped_span = torch.clamp(self.span, self.min_span, self.max_span)
            return DiffRsiFunction.apply(
                prices, clamped_span, self.overbought, self.oversold, self.steepness
            )

    __all__.extend([
        "DiffEmaFunction",
        "LearnableEma",
        "DiffRsiFunction",
        "LearnableRsi",
    ])
