use crate::v2_0_1::enums::MessageFormatEnumType;
use crate::v2_0_1::validator::validate_identifier_string;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ACChargingParametersType {
    pub enegery_amount: i64,
    pub ev_min_current: i64,
    pub ev_max_current: i64,
    pub ev_max_voltage: i64,
}

use chrono::{DateTime, Utc};
use validator::Validate;

use super::enums::{
    APNAuthenticationEnumType, AttributeEnumType, AuthorizationStatusEnumType,
    ChargingLimitSourceEnumType, ChargingProfileKindEnumType, ChargingProfilePurposeEnumType,
    ChargingRateUnitEnumType, ChargingStateEnumType, ClearMonitoringStatusEnumType,
    CostKindEnumType, DataEnumType, EnergyTransferModeEnumType, EventNotificationEnumType,
    EventTriggerEnumType, GetCertificateIdUseEnumType, GetVariableStatusEnumType,
    HashAlgorithmEnumType, IdTokenEnumType, LocationEnumType, MeasurandEnumType,
    MessagePriorityEnumType, MessageStateEnumType, MonitorEnumType, MutabilityEnumType,
    OCPPInterfaceEnumType, OCPPTransportEnumType, OCPPVersionEnumType, PhaseEnumType,
    ReadingContextEnumType, ReasonEnumType, RecurrencyKindEnumType, SetMonitoringStatusEnumType,
    SetVariableStatusEnumType, VPNEnumType,
};

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType {
    #[validate(length(min = 0, max = 36), custom = "validate_identifier_string")]
    pub additional_id_token: String,

    #[validate(length(min = 0, max = 50))]
    #[serde(rename = "type")]
    pub t: String,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct APNType {
    #[validate(length(min = 0, max = 512))]
    pub apn: String,

    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn_user_name: Option<String>,

    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn_password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_pin: Option<i64>,

    #[validate(length(min = 0, max = 6))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefered_network: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_only_prefered_network: Option<bool>,

    pub apn_authentication: APNAuthenticationEnumType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,

    pub id_token: Option<IdTokenType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataChainType {
    pub certificate_type: GetCertificateIdUseEnumType,

    pub certificate_hash_data: CertificateHashDataType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataType {
    pub hash_algorithm: HashAlgorithmEnumType,

    #[validate(length(min = 0, max = 128))]
    pub issuer_name_hash: String,

    #[validate(length(min = 0, max = 128))]
    pub issuer_key_hash: String,

    #[validate(length(min = 0, max = 40))]
    pub serial_number: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingLimitType {
    pub charging_limit_source: ChargingLimitSourceEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grid_critical: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingNeedsType {
    pub requested_energy_transfer: EnergyTransferModeEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<DateTime<Utc>>,

    #[serde(rename = "acChargingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_charging_parameters: Option<ACChargingParametersType>,

    #[serde(rename = "dcChargingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charging_parameters: Option<DCChargingParametersType>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DCChargingParametersType {
    pub ev_max_current: i64,
    pub ev_max_voltage: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_power: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub state_of_charge: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_capacity: Option<i64>,

    #[serde(rename = "fullSoC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub full_soc: Option<u8>,

    #[serde(rename = "bulkSoC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub bulk_soc: Option<u8>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileCriterionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_limit_source: Option<Vec<ChargingLimitSourceEnumType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileType {
    pub id: i64,

    pub stack_level: i64,

    pub charging_profile_purpose: ChargingProfilePurposeEnumType,

    pub charging_profile_kind: ChargingProfileKindEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTime<Utc>>,

    #[validate(length(min = 0, max = 36))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,

    pub charging_schedule: Vec<ChargingScheduleType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleType {
    pub id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    pub charging_rate_unit: ChargingRateUnitEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_rate: Option<f64>,

    #[validate(length(min = 1))]
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff: Option<SalesTariffType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriodType {
    pub start_period: i64,

    pub limit: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_to_use: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    #[validate(length(min = 0, max = 25))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    #[validate(length(min = 0, max = 20))]
    pub model: String,

    #[validate(length(min = 0, max = 50))]
    pub vendor_name: String,

    #[validate(length(min = 0, max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<ModemType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearMonitoringResultType {
    pub status: ClearMonitoringStatusEnumType,

    pub id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentType {
    /// Name of the component. Name should be taken from the list of standardized component names whenever possible. Case Insensitive. strongly advised to use Camel Case
    #[validate(length(min = 0, max = 50))]
    pub name: String,
    /// Name of instance in case the component exists as multiple instances. Case Insensitive. strongly advised to use Camel Case
    #[validate(length(min = 0, max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Specifies the EVSE when component is located at EVSE level, also specifies the connector when component is located at Connector level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVariableType {
    /// Required. Component for which a report of Variable is requested.
    pub component: ComponentType,
    /// Optional. Variable(s) for which the report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>,
}

/// CompositeScheduleType is used by: GetCompositeScheduleResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompositeScheduleType {
    pub evse_id: i64,
    pub duration: i64,
    pub schedule_start: DateTime<Utc>,
    pub charging_rate_unit: ChargingRateUnitEnumType,
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EVSEType {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventDataType {
    pub event_id: i64,
    pub timestamp: DateTime<Utc>,
    pub trigger: EventTriggerEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<i64>,
    pub actual_value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_monitoring_id: Option<i64>,
    pub event_notification_type: EventNotificationEnumType,
    pub component: ComponentType,
    pub variable: VariableType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionCostType {
    pub start_value: i64,
    pub cost: Vec<CostType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CostType {
    pub cost_kind: CostKindEnumType,
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_multiplier: Option<i8>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareType {
    pub location: String,
    pub retrieve_date_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    pub component: ComponentType,
    pub variable: VariableType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableResultType {
    pub attribute_status: GetVariableStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 2500))]
    pub attribute_value: Option<String>,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType {
    pub remote_location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_timestamp: Option<DateTime<Utc>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
    pub status: AuthorizationStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expire_date_time: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_message: Option<MessageContentType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType {
    pub id_token: String,
    #[serde(rename = "type")]
    pub t: IdTokenEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Vec<AdditionalInfoType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MessageContentType {
    pub format: MessageFormatEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    pub id: i64,
    pub priority: MessagePriorityEnumType,
    pub state: MessageStateEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub message: MessageContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<ComponentType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType {
    pub timestamp: DateTime<Utc>,
    pub sampled_value: Vec<SampledValueType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    pub hash_algorithm: HashAlgorithmEnumType,
    #[validate(length(min = 0, max = 128))]
    pub issuer_name_hash: String,
    #[validate(length(min = 0, max = 128))]
    pub issuer_key_hash: String,
    #[validate(length(min = 0, max = 40))]
    pub serial_number: String,
    #[validate(length(min = 0, max = 512))]
    #[serde(rename = "responderURL")]
    pub responder_url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RelativeTimeIntervalType {
    pub start: i64,
    pub duration: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_attribute: Vec<VariableAttributeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_characteristics: Option<VariableCharacteristicsType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SampledValueType {
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContextEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<MeasurandEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<PhaseEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_meter_value: Option<SignedMeterValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<UnitOfMeasureType>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_e_price_levels: Option<i64>,

    #[validate(length(min = 1, max = 1024))]
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_price_level: Option<u64>,
    pub relative_time_interval: RelativeTimeIntervalType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,
    pub value: f64,
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,
    pub severity: u8,
    pub component: ComponentType,
    pub variable: VariableType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringResultType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub status: SetMonitoringStatusEnumType,
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,
    pub severity: u8,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    pub attribute_value: String,
    pub component: ComponentType,
    pub variable: VariableType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResultType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    pub attribute_status: SetVariableStatusEnumType,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedMeterValueType {
    pub signed_meter_data: String,
    pub signing_method: String,
    pub encoding_method: String,
    pub public_key: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoType {
    pub reason_code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_state: Option<ChargingStateEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spent_charging: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<ReasonEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_start_id: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_monitoring: Vec<VariableMonitoringType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConnectionProfileType {
    pub ocpp_version: OCPPVersionEnumType,
    pub ocpp_transport: OCPPTransportEnumType,
    pub ocpp_csms_url: String,
    pub message_timeout: i64,
    pub security_profile: i64,
    pub ocpp_interface: OCPPInterfaceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<VPNType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn: Option<APNType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnitOfMeasureType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VariableAttributeType {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AttributeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutability: Option<MutabilityEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub data_type: DataEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_list: Option<String>,
    pub supports_monitoring: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoringType {
    pub id: i64,
    pub transaction: bool,
    pub value: f64,
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,
    pub severity: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableType {
    #[validate(length(min = 0, max = 50))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 50))]
    pub instance: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VPNType {
    pub server: String,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub password: String,
    pub key: String,
    #[serde(rename = "type")]
    pub kind: VPNEnumType,
}
