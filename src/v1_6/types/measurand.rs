/// Allowable values of the optional "measurand" field of a Value element, as used in MeterValuesRequest and StopTransaction.req messages. Default value of "measurand" is always "Energy.Active.Import.Register"
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[derive(minicbor::Encode, minicbor::Decode)]
pub enum Measurand {
    ///Instantaneous current flow from EV
    #[serde(rename = "Current.Export")]
    #[b(0)]
    CurrentExport,
    /// Instantaneous current flow to EV
    #[serde(rename = "Current.Import")]
    #[b(1)]
    CurrentImport,
    /// Maximum current offered to EV
    #[serde(rename = "Current.Offered")]
    #[b(2)]
    CurrentOffered,
    /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
    #[serde(rename = "Energy.Active.Export.Register")]
    #[b(3)]
    EnergyActiveExportRegister,
    /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
    #[default]
    #[serde(rename = "Energy.Active.Import.Register")]
    #[b(4)]
    EnergyActiveImportRegister,
    ///  Numerical value read from the "reactive electrical energy" (VARh or kVARh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
    #[serde(rename = "Energy.Reactive.Export.Register")]
    #[b(5)]
    EnergyReactiveExportRegister,
    /// Numerical value read from the "reactive electrical energy" (VARh or kVARh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
    #[serde(rename = "Energy.Reactive.Import.Register")]
    #[b(6)]
    EnergyReactiveImportRegister,
    /// Absolute amount of "active electrical energy" (Wh or kWh) exported (to the grid) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Active.Export.Interval")]
    #[b(7)]
    EnergyActiveExportInterval,
    /// Absolute amount of "active electrical energy" (Wh or kWh) imported (from the grid supply) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Active.Import.Interval")]
    #[b(8)]
    EnergyActiveImportInterval,
    /// Absolute amount of "reactive electrical energy" (VARh or kVARh) exported (to the grid) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    #[b(9)]
    EnergyReactiveExportInterval,
    ///  Absolute amount of "reactive electrical energy" (VARh or kVARh) imported (from the grid supply) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    #[b(10)]
    EnergyReactiveImportInterval,
    /// Instantaneous reading of powerline frequency. NOTE: OCPP 1.6 does not have a UnitOfMeasure for frequency, the UnitOfMeasure for any SampledValue with measurand: Frequency is Hertz.
    #[b(11)]
    Frequency,
    /// Instantaneous active power exported by EV. (W or kW)
    #[serde(rename = "Power.Active.Export")]
    #[b(12)]
    PowerActiveExport,
    /// Instantaneous active power imported by EV. (W or kW)
    #[serde(rename = "Power.Active.Import")]
    #[b(13)]
    PowerActiveImport,
    /// Instantaneous power factor of total energy flow
    #[serde(rename = "Power.Factor")]
    #[b(14)]
    PowerFactor,
    /// Maximum power offered to EV
    #[serde(rename = "Power.Offered")]
    #[b(15)]
    PowerOffered,
    /// Instantaneous reactive power exported by EV. (var or kvar)
    #[serde(rename = "Power.Reactive.Export")]
    #[b(16)]
    PowerReactiveExport,
    /// Instantaneous reactive power imported by EV. (var or kvar)
    #[serde(rename = "Power.Reactive.Import")]
    #[b(17)]
    PowerReactiveImport,
    /// Fan speed in RPM
    #[serde(rename = "RPM")]
    #[b(18)]
    Rpm,
    /// State of charge of charging vehicle in percentage
    #[b(19)]
    SoC,
    /// Temperature reading inside Charge Point.
    #[b(20)]
    Temperature,
    /// Instantaneous AC RMS supply voltage
    #[b(21)]
    Voltage,
}
