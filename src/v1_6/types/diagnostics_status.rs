/// Status in DiagnosticsStatusNotificationRequest
#[derive(serde::Serialize, serde::Deserialize, minicbor::Encode, minicbor::Decode, Debug, Clone, PartialEq, Default)]
#[cbor(index_only)]
pub enum DiagnosticsStatus {
    /// Charge Point is not performing diagnostics related tasks. Status Idle SHALL only be used as in a DiagnosticsStatusNotification.req that was triggered by a TriggerMessage.req
    #[default]
    #[b(0)]
    Idle,
    /// Diagnostics information has been uploaded.
    #[b(1)]
    Uploaded,
    /// Uploading of diagnostics failed.
    #[b(2)]
    UploadFailed,
    /// File is being uploaded.
    #[b(3)]
    Uploading,
}
