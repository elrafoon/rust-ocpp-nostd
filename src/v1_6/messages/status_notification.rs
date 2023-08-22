use crate::v1_6::types::{ChargePointErrorCode, ChargePointStatus};

use chrono::{DateTime, Utc};
#[cfg(feature = "std")]
use validator::Validate;

/// This contains the field definition of the StatusNotification.req PDU sent by the Charge Point to the Central System. See also Status Notification
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest<'a> {
    /// Required. The id of the connector for which the status is reported. Id '0' (zero) is used if the status is for the Charge Point main controller.
    pub connector_id: u64,
    /// Required. This contains the error code reported by the Charge Point.
    pub error_code: ChargePointErrorCode, // IdToken, should this be a type?
    /// Optional. Additional free format information related to the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature="std", validate(length(min = 1, max = 50)))]
    pub info: Option<&'a str>,
    /// Required. This contains the current status of the Charge Point.
    pub status: ChargePointStatus,
    /// Optional. The time for which the status is reported. If absent time of receipt of the message will be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    /// Optional. This identifies the vendor-specific implementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature="std", validate(length(min = 1, max = 255)))]
    pub vendor_id: Option<&'a str>,
    /// Optional. This contains the vendor-specific error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature="std", validate(length(min = 1, max = 50)))]
    pub vendor_error_code: Option<&'a str>,
}

/// This contains the field definition of the StartTransaction.conf PDU sent by the Central System to the Charge Point in response to a StartTransaction.req PDU. See also Start Transaction
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationResponse {
    // This contains the field definition of the StatusNotification.conf PDU sent by the Central System to the Charge Point in response to an StatusNotification.req PDU. See also Status Notification No fields are defined.
}
