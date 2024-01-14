/// Charge Point status reported in StatusNotification.req.
#[derive(serde::Serialize, serde::Deserialize, minicbor::Encode, minicbor::Decode, Debug, Clone, PartialEq, Default)]
pub enum ChargePointErrorCode {
    /// Failure to lock or unlock connector.
    #[b(0)]
    ConnectorLockFailure,
    /// Communication failure with the vehicle, might be Mode 3 or other communication protocol problem. This is not a real error in the sense that the Charge Point doesn’t need to go to the faulted state. Instead, it should go to the SuspendedEVSE state.
    #[b(1)]
    EVCommunicationError,
    /// Ground fault circuit interrupter has been activated.
    #[b(2)]
    GroundFailure,
    /// Temperature inside Charge Point is too high.
    #[b(3)]
    HighTemperature,
    /// Error in internal hard- or software component.
    #[b(4)]
    InternalError,
    /// The authorization information received from the Central System is in conflict with the LocalAuthorizationList.
    #[b(5)]
    LocalListConflict,
    /// No error to report.
    #[default]
    #[b(6)]
    NoError,
    /// Other type of error. More information in vendorErrorCode.
    #[b(7)]
    OtherError,
    /// Over current protection device has tripped.
    #[b(8)]
    OverCurrentFailure,
    /// Voltage has risen above an acceptable level.
    #[b(9)]
    OverVoltage,
    /// Failure to read electrical/energy/power meter.
    #[b(10)]
    PowerMeterFailure,
    /// Failure to control power switch.
    #[b(11)]
    PowerSwitchFailure,
    /// Failure with idTag reader.
    #[b(12)]
    ReaderFailure,
    /// Unable to perform a reset.
    #[b(13)]
    ResetFailure,
    /// Voltage has dropped below an acceptable level.
    #[b(14)]
    UnderVoltage,
    /// Wireless communication device reports a weak signal.
    #[b(15)]
    WeakSignal,
}
