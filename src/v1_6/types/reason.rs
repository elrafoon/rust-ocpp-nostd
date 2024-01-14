/// Reason for stopping a transaction in StopTransactionRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[derive(minicbor::Encode, minicbor::Decode)]
pub enum Reason {
    /// The transaction was stopped because of the authorization status in a StartTransaction.conf
    #[b(0)]
    DeAuthorized,
    /// Emergency stop button was used.
    #[b(1)]
    EmergencyStop,
    /// disconnecting of cable, vehicle moved away from inductive charge unit.
    #[b(2)]
    EVDisconnected,
    /// A hard reset command was received.
    #[b(3)]
    HardReset,
    /// Stopped locally on request of the user at the Charge Point. This is a regular termination of a transaction. Examples: presenting an RFID tag, pressing a button to stop.
    #[b(4)]
    Local,
    /// Any other reason.
    #[default]
    #[b(5)]
    Other,
    /// Complete loss of power.
    #[b(6)]
    PowerLoss,
    /// A locally initiated reset/reboot occurred. (for instance watchdog kicked in)
    #[b(7)]
    Reboot,
    /// Stopped remotely on request of the user. This is a regular termination of a transaction. Examples: termination using a smartphone app, exceeding a (non local) prepaid credit.
    #[b(8)]
    Remote,
    /// A soft reset command was received.
    #[b(9)]
    SoftReset,
    /// Central System sent an Unlock Connector command.
    #[b(10)]
    UnlockCommand,
}
