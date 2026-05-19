use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SmaKwargs {
    pub period: usize,
}

// Polars Expression Plugins were removed because they required PyO3, 
// which was causing version conflicts. The Python package now uses UniFFI exclusively.
