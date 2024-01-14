#[cfg(feature = "std")]
use validator::Validate;

use crate::v1_6::types::DataTransferStatus;

#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, minicbor::Encode, minicbor::Decode, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest<'a> {
    #[serde(rename = "vendorId")]
    #[b(0)]
    pub vendor_string: &'a str,
    #[cfg_attr(feature="std", validate(length(min = 1, max = 50)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[b(1)]
    pub message_id: Option<&'a str>,
    #[cfg_attr(feature="std", validate(length(min = 1, max = 255)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[b(2)]
    pub data: Option<&'a str>,
}

#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse<'a> {
    /// Required. This indicates the success or failure of the data transfer.
    pub status: DataTransferStatus,
    /// Optional. Data in response to request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<&'a str>,
}
