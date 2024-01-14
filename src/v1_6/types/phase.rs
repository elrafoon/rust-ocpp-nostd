/// Phase as used in SampledValue. Phase specifies how a measured value is to be interpreted. Please note that not all values of Phase are applicable to all Measurands.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
#[derive(minicbor::Encode, minicbor::Decode)]
pub enum Phase {
    #[default]
    #[b(0)]
    L1,
    #[b(1)]
    L2,
    #[b(2)]
    L3,
    #[b(3)]
    N,
    #[serde(rename = "L1-N")]
    #[b(4)]
    L1N,
    #[serde(rename = "L2-N")]
    #[b(5)]
    L2N,
    #[serde(rename = "L3-N")]
    #[b(6)]
    L3N,
    #[serde(rename = "L1-L2")]
    #[b(7)]
    L1L2,
    #[serde(rename = "L2-L3")]
    #[b(8)]
    L2L3,
    #[serde(rename = "L3-L1")]
    #[b(9)]
    L3L1,
}
