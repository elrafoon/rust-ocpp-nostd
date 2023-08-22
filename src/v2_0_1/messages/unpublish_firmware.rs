use crate::v2_0_1::enumerations::unpublish_firmware_status_enum_type::UnpublishFirmwareStatusEnumType;

/// This contains the field definition of the UnpublishFirmwareRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareRequest<'a> {
    pub checksum: &'a str,
}

/// This contains the field definition of the UnpublishFirmwareResponse PDU sent by the Charging Station to the CSMS in response to a UnpublishFirmwareRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareResponse {
    pub status: UnpublishFirmwareStatusEnumType,
}
