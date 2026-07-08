//! Live execution bridge contract (quantwave-cr6v.16 / quantwave-53tj).
//!
//! Nautilus Trader is **LGPL-3.0** — not embedded in quantwave-backtest.
//! This module defines a clean-room event export trait for a future adapter crate.

use crate::{Bar, StrategySignal};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors from live bridge adapters.
#[derive(Error, Debug)]
pub enum LiveBridgeError {
    #[error("bridge not connected")]
    NotConnected,

    #[error("bridge publish failed: {0}")]
    PublishFailed(String),
}

/// One exportable decision event for external live engines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveSignalEvent {
    pub ts: DateTime<Utc>,
    pub symbol: Option<String>,
    pub exposure: f64,
    pub metadata: Option<std::collections::HashMap<String, f64>>,
}

impl LiveSignalEvent {
    pub fn from_bar_signal(bar: &Bar, symbol: Option<String>, signal: &StrategySignal) -> Self {
        Self {
            ts: bar.ts,
            symbol,
            exposure: signal.exposure,
            metadata: signal.metadata.clone(),
        }
    }
}

/// Trait implemented by future live adapters (e.g. a separate `quantwave-nautilus` crate).
pub trait LiveBridge: Send {
    fn connect(&mut self) -> Result<(), LiveBridgeError>;
    fn publish(&mut self, event: &LiveSignalEvent) -> Result<(), LiveBridgeError>;
    fn disconnect(&mut self) -> Result<(), LiveBridgeError>;
}

/// In-memory stub for tests and notebooks — records events, no network I/O.
#[derive(Debug, Default)]
pub struct RecordingLiveBridge {
    connected: bool,
    pub events: Vec<LiveSignalEvent>,
}

impl RecordingLiveBridge {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LiveBridge for RecordingLiveBridge {
    fn connect(&mut self) -> Result<(), LiveBridgeError> {
        self.connected = true;
        Ok(())
    }

    fn publish(&mut self, event: &LiveSignalEvent) -> Result<(), LiveBridgeError> {
        if !self.connected {
            return Err(LiveBridgeError::NotConnected);
        }
        self.events.push(event.clone());
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), LiveBridgeError> {
        self.connected = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording_live_bridge_records_events() {
        let mut bridge = RecordingLiveBridge::new();
        bridge.connect().unwrap();
        let bar = Bar {
            ts: Utc::now(),
            close: 101.5,
            high: None,
            low: None,
        };
        let signal = StrategySignal {
            exposure: 1.0,
            metadata: None,
        };
        let event = LiveSignalEvent::from_bar_signal(&bar, Some("SPY".into()), &signal);
        bridge.publish(&event).unwrap();
        assert_eq!(bridge.events.len(), 1);
        assert_eq!(bridge.events[0].exposure, 1.0);
        bridge.disconnect().unwrap();
    }

    #[test]
    fn test_live_bridge_not_connected_errors() {
        let mut bridge = RecordingLiveBridge::new();
        let event = LiveSignalEvent {
            ts: Utc::now(),
            symbol: None,
            exposure: 0.0,
            metadata: None,
        };
        assert!(matches!(
            bridge.publish(&event),
            Err(LiveBridgeError::NotConnected)
        ));
    }
}
