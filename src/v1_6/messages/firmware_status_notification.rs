use crate::v1_6::types::FirmwareStatus;

#[derive(serde::Serialize, serde::Deserialize, minicbor::Encode, minicbor::Decode, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    #[b(0)]
    pub status: FirmwareStatus,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {}
