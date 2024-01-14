/// Allowable values of the optional "location" field of a value element in SampledValue.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[derive(minicbor::Encode, minicbor::Decode)]
pub enum Location {
    /// Measurement inside body of Charge Point (e.g. Temperature)
    #[b(0)]
    Body,
    ///Measurement taken from cable between EV and Charge Point
    #[b(1)]
    Cable,
    ///Measurement taken by EV
    #[serde(rename = "EV")]
    #[b(2)]
    Ev,
    ///Measurement at network (“grid”) inlet connection
    #[b(3)]
    Inlet,
    ///Measurement at a Connector. Default value
    #[default]
    #[b(4)]
    Outlet,
}
