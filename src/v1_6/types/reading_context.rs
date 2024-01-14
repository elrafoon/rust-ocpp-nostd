/// Values of the context field of a value in SampledValue.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[derive(minicbor::Encode, minicbor::Decode)]
pub enum ReadingContext {
    /// Value taken at start of interruption.
    #[serde(rename = "Interruption.Begin")]
    #[b(0)]
    InterruptionBegin,
    /// Value taken when resuming after interruption.
    #[serde(rename = "Interruption.End")]
    #[b(1)]
    InterruptionEnd,
    /// Value for any other situations.
    #[default]
    #[b(2)]
    Other,
    /// Value taken at clock aligned interval.
    #[serde(rename = "Sample.Clock")]
    #[b(3)]
    SampleClock,
    /// Value taken as periodic sample relative to start time of transaction.
    #[serde(rename = "Sample.Periodic")]
    #[b(4)]
    SamplePeriodic,
    /// Value taken at start of transaction.
    #[serde(rename = "Transaction.Begin")]
    #[b(5)]
    TransactionBegin,
    /// Value taken at end of transaction.
    #[serde(rename = "Transaction.End")]
    #[b(6)]
    TransactionEnd,
    /// Value taken in response to a TriggerMessage.req
    #[b(7)]
    Trigger,
}
