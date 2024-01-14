/// Status in TriggerMessageResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[derive(minicbor::Encode, minicbor::Decode)]
pub enum UnitOfMeasure {
    /// Watt-hours (energy). Default.
    #[default]
    #[b(0)]
    Wh,
    /// kiloWatt-hours (energy).
    #[serde(rename = "kWh")]
    #[b(1)]
    KWh,
    /// Var-hours (reactive energy).
    #[serde(rename = "varh")]
    #[b(2)]
    Varh,
    /// kilovar-hours (reactive energy).
    #[serde(rename = "kvarh")]
    #[b(3)]
    Kvarh,
    /// Watts (power).
    #[b(4)]
    W,
    /// kilowatts (power).
    #[serde(rename = "kW")]
    #[b(5)]
    Kw,
    /// VoltAmpere (apparent power).
    #[serde(rename = "VA")]
    #[b(6)]
    Va,
    /// kiloVolt Ampere (apparent power).
    #[serde(rename = "kVA")]
    #[b(7)]
    Kva,
    /// Vars (reactive power).
    #[serde(rename = "var")]
    #[b(8)]
    Var,
    /// kilovars (reactive power).
    #[serde(rename = "kvar")]
    #[b(9)]
    Kvar,
    /// Amperes (current).
    #[b(10)]
    A,
    /// Voltage (r.m.s. AC).
    #[b(11)]
    V,
    /// Degrees (temperature).
    #[b(12)]
    Celsius,
    /// Degrees (temperature).
    #[b(13)]
    Fahrenheit,
    /// Degrees Kelvin (temperature).
    #[b(14)]
    K,
    /// Percentage.
    #[b(15)]
    Percent,
}
