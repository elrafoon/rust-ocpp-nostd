use crate::v1_6::types::{IdTagInfo, DateTime};

#[cfg(feature = "std")]
use validator::Validate;

/// This section contains the field definition of the StartTransaction.req PDU sent by the Charge Point to the Central System. See also Start Transaction
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, minicbor::Encode, minicbor::Decode, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionRequest<'a> {
    /// Required. This identifies which connector of the Charge Point is used.
    #[b(0)]
    pub connector_id: u64,
    /// Required. This contains the identifier for which a transaction has to be started.
    #[cfg_attr(feature="std", validate(length(min = 1, max = 20)))]
    #[b(1)]
    pub id_tag: &'a str, // IdToken, should this be a type?
    /// Required. This contains the meter value in Wh for the connector at start of the transaction.
    #[b(2)]
    pub meter_start: i64,
    /// Optional. This contains the id of the reservation that terminates as a result of this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[b(3)]
    pub reservation_id: Option<i64>,
    /// Required. This contains the date and time on which the transaction is started.
    #[b(4)]
    pub timestamp: DateTime,
}

/// This contains the field definition of the StartTransaction.conf PDU sent by the Central System to the Charge Point in response to a StartTransaction.req PDU. See also Start Transaction
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionResponse<'a> {
    /// Required. This contains information about authorization status, expiry and parent id
    #[serde(borrow)]
    pub id_tag_info: IdTagInfo<'a>,
    /// Required. This contains the transaction id supplied by the Central System.
    pub transaction_id: i64,
}
