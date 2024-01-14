use super::{SampledValue, DateTime};
use crate::Vec;

/// Collection of one or more sampled values in MeterValues.req and StopTransaction.req. All sampled values in a MeterValue are sampled at the same point in time.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[derive(minicbor::Encode, minicbor::Decode)]
#[serde(rename_all = "camelCase")]
pub struct MeterValue<'a, const N_SAMPLED_VALUES: usize> {
    /// Required. Timestamp for measured value(s).
    #[b(0)]
    pub timestamp: DateTime,
    /// Required. One or more measured values
    #[serde(borrow)]
    #[b(1)]
    pub sampled_value: Vec<SampledValue<'a>, N_SAMPLED_VALUES>,
}
