use crate::v1_6::types::DiagnosticsStatus;

#[derive(serde::Serialize, serde::Deserialize, minicbor::Encode, minicbor::Decode, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsStatusNotificationRequest {
    #[b(0)]
    pub status: DiagnosticsStatus,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsStatusNotificationResponse {}
