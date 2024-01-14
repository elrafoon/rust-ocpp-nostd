use chrono::{DateTime, Utc};
#[cfg(feature = "std")]
use validator::Validate;

#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, minicbor::Encode, minicbor::Decode, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {}

#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    /// # From OCPP Specification
    /// Required. This contains the current time of the Central System.
    pub current_time: DateTime<Utc>,
}
