use chrono::DateTime;
use chrono::Utc;

use super::charging_schedule_type::ChargingScheduleType;
use crate::v2_0_1::enumerations::charging_profile_kind_enum_type::ChargingProfileKindEnumType;
use crate::v2_0_1::enumerations::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;
use crate::v2_0_1::enumerations::recurrency_kind_enum_type::RecurrencyKindEnumType;
use crate::Vec;

#[cfg(feature = "std")]
use validator::Validate;
/// A ChargingProfile consists of ChargingSchedule, describing the amount of power or current that can be delivered per time interval
/// ChargingProfileType is used by: RequestStartTransactionRequest , SetChargingProfileRequest , ReportChargingProfilesRequest
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileType<'a,
    const N_SALES_TARIFF_ENTRIES: usize, const N_TARIFF_CONSUMPTION_COSTS: usize,
    const N_COSTS_PER_TARIFF_CONS_COST: usize, const N_CHARGING_SCHEDULE_PERIODS: usize,
    const N_CHARGING_SCHEDULES: usize>
{
    /// Required. Id of ChargingProfile.
    pub id: i64,
    /// Required. Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values. Lowest level is 0.
    pub stack_level: i64,
    /// Required. Defines the purpose of the schedule transferred by this profile
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,
    /// Required. Indicates the kind of schedule.
    pub charging_profile_kind: ChargingProfileKindEnumType,
    /// Optional. Indicates the start point of a recurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,
    /// Optional. Point in time at which the profile starts to be valid. If absent, the profile is valid as soon as it is received by the Charging Station
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,
    /// Optional. Point in time at which the profile stops to be valid. If absent, the profile is valid until it is replaced by another profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTime<Utc>>,
    /// Optional. SHALL only be included if ChargingProfilePurpose is set to TxProfile. The transactionId is used to match the profile to a specific transaction
    #[cfg_attr(feature="std", validate(length(min = 0, max = 36)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<&'a str>,
    /// Required. Schedule that contains limits for the available power or current over time. In order to support ISO 15118 schedule negotiation, it supports at most three schedules with associated tariff to choose from
    pub charging_schedule: Vec<ChargingScheduleType<'a, N_SALES_TARIFF_ENTRIES, N_TARIFF_CONSUMPTION_COSTS, N_COSTS_PER_TARIFF_CONS_COST, N_CHARGING_SCHEDULE_PERIODS>, N_CHARGING_SCHEDULES>,
}
