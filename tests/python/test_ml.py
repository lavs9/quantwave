"""Tests for QuantWave Machine Learning module (quantwave.ml).

Verifies Python bindings for:
- Kolmogorov-Arnold Networks (KanNetwork, KanIndicator, symbolic extraction)
- Technical Indicator Networks (DifferentiableEma, DifferentiableRsi, DifferentiableMacd)
"""

import pytest
import quantwave as qw
from quantwave.ml import (
    KanNetwork,
    KanIndicator,
    DifferentiableEma,
    DifferentiableRsi,
    DifferentiableMacd,
    extract_symbolic_formula,
    extract_feature_importance,
)


def test_kan_network_construction_and_forward():
    net = KanNetwork(layers=[3, 6, 1], grid_size=5, spline_order=3)
    assert net.in_features() == 3
    assert net.out_features() == 1
    assert net.num_layers() == 2

    inputs = [0.2, -0.4, 0.6]
    out = net.forward(inputs)
    assert len(out) == 1
    assert isinstance(out[0], float)

    # Batch forward
    batch_inputs = [
        [0.1, 0.2, 0.3],
        [-0.5, 0.0, 0.5],
        [0.9, -0.9, 0.1],
    ]
    batch_out = net.forward_batch(batch_inputs)
    assert len(batch_out) == 3
    for row in batch_out:
        assert len(row) == 1


def test_kan_network_dimension_mismatch():
    with pytest.raises(ValueError, match="layers must have at least 2 dimensions"):
        KanNetwork(layers=[5])

    net = KanNetwork(layers=[2, 4, 1])
    with pytest.raises(ValueError, match="Dimension mismatch"):
        net.forward([1.0, 2.0, 3.0])  # expected 2, got 3


def test_kan_network_serialization():
    net = KanNetwork(layers=[2, 4, 1])
    inputs = [0.33, -0.77]
    expected_out = net.forward(inputs)

    json_str = net.to_json()
    assert isinstance(json_str, str)
    assert len(json_str) > 100

    loaded = KanNetwork.from_json(json_str)
    assert loaded.in_features() == 2
    assert loaded.out_features() == 1
    assert loaded.forward(inputs) == expected_out


def test_kan_symbolic_extraction():
    net = KanNetwork(layers=[3, 4, 1])
    feature_names = ["rsi", "macd", "volatility"]

    formula = net.extract_symbolic_formula(feature_names)
    assert isinstance(formula, str)
    assert "rsi" in formula or "macd" in formula

    importance = net.extract_feature_importance(feature_names)
    assert len(importance) == 3
    names = [t["feature_name"] for t in importance]
    assert set(names) == set(feature_names)

    total_rel = sum(t["relative_importance"] for t in importance)
    assert total_rel == pytest.approx(1.0, abs=1e-5)

    # Module-level functions
    assert extract_symbolic_formula(net, feature_names) == formula
    assert extract_feature_importance(net, feature_names) == importance


def test_kan_indicator_streaming():
    net = KanNetwork(layers=[2, 4, 1])
    indicator = KanIndicator(net)
    assert indicator.in_features() == 2

    # Streaming single vectors
    v1 = indicator.next([0.1, -0.2])
    v2 = indicator.update([0.3, 0.4])
    assert isinstance(v1, float)
    assert isinstance(v2, float)

    # Parity with KanNetwork.forward
    expected_v1 = net.forward([0.1, -0.2])[0]
    assert v1 == expected_v1

    # Batch evaluation
    batch_vals = indicator.batch([[0.1, -0.2], [0.3, 0.4]])
    assert len(batch_vals) == 2
    assert batch_vals[0] == expected_v1

    # 1D scalar network
    scalar_net = KanNetwork(layers=[1, 3, 1])
    scalar_ind = KanIndicator(scalar_net)
    val = scalar_ind.next(0.75)
    assert isinstance(val, float)
    assert val == scalar_net.forward([0.75])[0]


def test_differentiable_ema():
    ema = DifferentiableEma(span=10.0)
    assert ema.span == 10.0
    assert ema.alpha == pytest.approx(2.0 / 11.0)
    assert ema.bars_consumed == 0

    val0, grad0 = ema.step(100.0)
    assert val0 == 100.0
    assert grad0 == 0.0
    assert ema.bars_consumed == 1

    val1, grad1 = ema.step(105.0)
    assert val1 > 100.0
    assert grad1 < 0.0  # higher span means slower EMA -> lower value

    # Update span (online SGD step)
    old_span = ema.span
    new_span = ema.update_span(learning_rate=0.5, loss_gradient=1.0)
    assert new_span != old_span

    # Batch pass
    ema.reset()
    assert ema.bars_consumed == 0
    prices = [100.0, 102.0, 101.0, 104.0, 103.0]
    vals, grads = ema.batch(prices)
    assert len(vals) == len(prices)
    assert len(grads) == len(prices)
    assert vals[0] == 100.0


def test_differentiable_rsi():
    rsi = DifferentiableRsi(span=14.0, overbought=70.0, oversold=30.0, steepness=0.1)
    assert rsi.span == 14.0
    assert rsi.overbought == 70.0
    assert rsi.oversold == 30.0

    val0, sig0, grad0 = rsi.step(100.0)
    assert val0 == 50.0  # neutral initial
    assert grad0 == 0.0

    # Sequential price increases -> RSI rises
    prices = [100.0, 102.0, 104.0, 106.0, 108.0]
    for p in prices:
        val, sig, grad = rsi.step(p)

    assert val > 60.0
    assert -1.0 <= sig <= 1.0

    # Batch pass
    rsi.reset()
    rsi_vals, sig_vals, grad_vals = rsi.batch(prices)
    assert len(rsi_vals) == len(prices)
    assert len(sig_vals) == len(prices)
    assert len(grad_vals) == len(prices)


def test_differentiable_macd():
    macd = DifferentiableMacd(fast_span=12.0, slow_span=26.0, signal_span=9.0)
    assert macd.fast_span == 12.0
    assert macd.slow_span == 26.0
    assert macd.signal_span == 9.0

    line0, sig0, hist0, g_fast, g_slow, g_sig = macd.step(100.0)
    assert line0 == 0.0
    assert sig0 == 0.0
    assert hist0 == 0.0

    prices = [100.0, 102.0, 105.0, 107.0, 110.0]
    for p in prices:
        line, sig, hist, g_fast, g_slow, g_sig = macd.step(p)

    # In rising market, fast EMA > slow EMA -> MACD line > 0
    assert line > 0.0

    # Batch pass
    macd.reset()
    lines, sigs, hists = macd.batch(prices)
    assert len(lines) == len(prices)
    assert len(sigs) == len(prices)
    assert len(hists) == len(prices)


def test_tin_to_kan_pipeline():
    """Validates end-to-end 2-stage TIN (feature extraction) -> KAN (signal generation) pipeline."""
    rsi = DifferentiableRsi(span=14.0)
    macd = DifferentiableMacd(fast_span=12.0, slow_span=26.0, signal_span=9.0)
    kan = KanNetwork(layers=[2, 4, 1])

    prices = [100.0, 101.5, 99.8, 102.4, 103.8, 101.2, 104.5]
    for p in prices:
        r_val, _, _ = rsi.step(p)
        m_val, _, _, _, _, _ = macd.step(p)

        # Normalize features for KAN input [-1.0, 1.0]
        norm_rsi = (r_val - 50.0) / 50.0
        norm_macd = max(min(m_val / 5.0, 1.0), -1.0)

        signal = kan.forward([norm_rsi, norm_macd])[0]
        assert isinstance(signal, float)


def test_pytorch_learnable_ema():
    """Validates PyTorch autograd backpropagation through DifferentiableEma."""
    try:
        import torch
        from quantwave.ml import LearnableEma
    except (ImportError, AttributeError):
        pytest.skip("PyTorch not installed")

    prices = torch.tensor([100.0, 102.0, 101.0, 104.0, 105.0], requires_grad=True)
    target = torch.tensor([100.0, 101.0, 101.5, 103.0, 104.0])

    model = LearnableEma(initial_span=10.0)
    optimizer = torch.optim.SGD(model.parameters(), lr=0.1)

    initial_span = model.span.item()
    out = model(prices)
    loss = (out - target).pow(2).mean()
    loss.backward()

    assert model.span.grad is not None
    assert prices.grad is not None
    assert torch.isfinite(model.span.grad)
    assert torch.isfinite(prices.grad).all()

    optimizer.step()
    assert model.span.item() != initial_span


def test_pytorch_learnable_rsi():
    """Validates PyTorch autograd backpropagation through DifferentiableRsi."""
    try:
        import torch
        from quantwave.ml import LearnableRsi
    except (ImportError, AttributeError):
        pytest.skip("PyTorch not installed")

    prices = torch.tensor([100.0, 102.0, 105.0, 103.0, 107.0])
    target_rsi = torch.tensor([50.0, 60.0, 70.0, 65.0, 75.0])

    model = LearnableRsi(initial_span=14.0)
    optimizer = torch.optim.SGD(model.parameters(), lr=0.5)

    initial_span = model.span.item()
    out_rsi, out_sig = model(prices)
    loss = (out_rsi - target_rsi).pow(2).mean()
    loss.backward()

    assert model.span.grad is not None
    assert torch.isfinite(model.span.grad)

    optimizer.step()
    assert model.span.item() != initial_span

