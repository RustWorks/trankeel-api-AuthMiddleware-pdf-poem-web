use crate::Amount;
use crate::DateTime;

// # Types

/// Synthesis of collected rents.
pub struct Summary {
    pub since: DateTime,
    pub until: DateTime,
    //
    pub amount_expected: Amount,
    pub amount_received: Amount,
    pub amount_settled: Amount,
    pub amount_partial: Amount,
    pub amount_pending: Amount,
    //
    pub n_expected: u32,
    pub n_received: u32,
    pub n_settled: u32,
    pub n_partial: u32,
    pub n_pending: u32,
    //
    pub ratio_expected: f64,
    pub ratio_received: f64,
    pub ratio_settled: f64,
    pub ratio_partial: f64,
    pub ratio_pending: f64,
    //
    pub variation_expected: f64,
    pub variation_received: f64,
    pub variation_settled: f64,
    pub variation_partial: f64,
    pub variation_pending: f64,
    //
    pub payment_rate: f64,
    pub occupation_rate: f64,
}

impl Default for Summary {
    fn default() -> Self {
        Self {
            since: DateTime::default(),
            until: DateTime::default(),
            amount_expected: Default::default(),
            amount_received: Default::default(),
            amount_settled: Default::default(),
            amount_partial: Default::default(),
            amount_pending: Default::default(),
            n_expected: Default::default(),
            n_received: Default::default(),
            n_settled: Default::default(),
            n_partial: Default::default(),
            n_pending: Default::default(),
            ratio_expected: Default::default(),
            ratio_received: Default::default(),
            ratio_settled: Default::default(),
            ratio_partial: Default::default(),
            ratio_pending: Default::default(),
            variation_expected: Default::default(),
            variation_received: Default::default(),
            variation_settled: Default::default(),
            variation_partial: Default::default(),
            variation_pending: Default::default(),
            payment_rate: Default::default(),
            occupation_rate: Default::default(),
        }
    }
}
