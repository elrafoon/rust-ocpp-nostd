/// Status of a firmware download as reported in FirmwareStatusNotificationRequest
#[derive(serde::Serialize, serde::Deserialize, minicbor::Encode, minicbor::Decode, Debug, Clone, PartialEq, Default)]
pub enum FirmwareStatus {
    /// New firmware has been downloaded by Charge Point.
    #[b(0)]
    Downloaded,
    /// Charge point failed to download firmware.
    #[b(1)]
    DownloadFailed,
    /// Firmware is being downloaded.
    #[b(2)]
    Downloading,
    /// Charge Point is not performing firmware update related tasks. Status Idle SHALL only be used as in a FirmwareStatusNotificationRequest that was triggered by a TriggerMessageRequest
    #[default]
    #[b(3)]
    Idle,
    /// Installation of new firmware has failed.
    #[b(4)]
    InstallationFailed,
    /// Firmware is being installed.
    #[b(5)]
    Installing,
    /// New firmware has successfully been installed in charge point.
    #[b(6)]
    Installed,
}
