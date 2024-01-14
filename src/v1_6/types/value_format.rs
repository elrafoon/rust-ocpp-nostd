/// Format that specifies how the value element in SampledValue is to be interpreted.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[derive(minicbor::Encode, minicbor::Decode)]
pub enum ValueFormat {
    /// Data is to be interpreted as integer/decimal numeric data.
    #[default]
    #[b(0)]
    Raw,
    /// Data is represented as a signed binary data block, encoded as hex data.
    #[b(1)]
    SignedData,
}
