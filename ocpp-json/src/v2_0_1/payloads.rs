use std::fmt;

use chrono::{DateTime, Utc};
use validator::Validate;

use super::{
    data_types::{
        AuthorizationData, CertificateHashDataChainType, CertificateHashDataType,
        ChargingLimitType, ChargingNeedsType, ChargingProfileCriterionType, ChargingProfileType,
        ChargingScheduleType, ChargingStationType, ClearChargingProfileType,
        ClearMonitoringResultType, ComponentVariableType, CompositeScheduleType, EVSEType,
        EventDataType, FirmwareType, GetVariableDataType, GetVariableResultType, IdTokenInfoType,
        IdTokenType, LogParametersType, MessageContentType, MessageInfoType, MeterValueType,
        MonitoringDataType, NetworkConnectionProfileType, OCSPRequestDataType, ReportDataType,
        SetVariableDataType, SetVariableResultType, StatusInfoType, TransactionType, SetMonitoringResultType, SetMonitoringDataType,
    },
    enums::{
        AuthorizeCertificateStatusEnumType, BootReasonEnumType, CancelReservationStatusEnumType,
        CertificateActionEnumType, CertificateSignedStatusEnumType, CertificateSigningUseEnumType,
        ChangeAvailabilityStatusEnumType, ChargingLimitSourceEnumType,
        ChargingProfileStatusEnumType, ChargingRateUnitEnumType, ClearCacheStatusEnumType,
        ClearChargingProfileStatusEnumType, ClearMessageStatusEnumType, ComponentCriterionEnumType,
        ConnectorEnumType, ConnectorStatusEnumType, CustomerInformationStatusEnumType,
        DataTransferStatusEnumType, DeleteCertificateStatusEnumType, DisplayMessageStatusEnumType,
        FirmwareStatusEnumType, GenericDeviceModelStatusEnumType, GenericStatusEnumType,
        GetCertificateIdUseEnumType, GetCertificateStatusEnumType,
        GetChargingProfileStatusEnumType, GetDisplayMessagesStatusEnumType,
        InstallCertificateStatusEnumType, InstallCertificateUseEnumType,
        Iso15118EVCertificateStatusEnumType, LogEnumType, LogStatusEnumType,
        MessagePriorityEnumType, MessageStateEnumType, MessageTriggerEnumType,
        MonitoringBaseEnumType, MonitoringCriterionEnumType, NotifyEVChargingNeedsStatusEnumType,
        OperationalStatusEnumType, PublishFirmwareStatusEnumType, RegistrationStatusEnumType,
        ReportBaseEnumType, RequestStartStopStatusEnumType, ReservationUpdateStatusEnumType,
        ReserveNowStatusEnumType, ResetEnumType, ResetStatusEnumType, SendLocalListStatusEnumType,
        SetNetworkProfileStatusEnumType, TransactionEventEnumType, TriggerMessageStatusEnumType,
        TriggerReasonEnumType, UnlockStatusEnumType, UnpublishFirmwareStatusEnumType,
        UpdateEnumType, UpdateFirmwareStatusEnumType, UploadLogStatusEnumType,
    },
};

#[derive(serde::Serialize, serde::Deserialize, Validate, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    #[validate(length(min = 0, max = 5500))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    pub id_token: IdTokenType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,

    pub id_token_info: IdTokenInfoType,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub reason: BootReasonEnumType,

    pub charging_station: ChargingStationType,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    pub current_time: DateTime<Utc>,

    pub interval: u64,

    pub status: RegistrationStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl fmt::Display for BootNotificationRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for BootNotificationResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationRequest {
    pub reservation_id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationResponse {
    pub status: CancelReservationStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedRequest {
    #[validate(length(min = 0, max = 10000))]
    pub certificate_chain: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedResponse {
    pub status: CertificateSignedStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    pub operational_status: OperationalStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    pub status: ChangeAvailabilityStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheRequest {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheResponse {
    pub status: ClearCacheStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_criteria: Option<ClearChargingProfileType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileResponse {
    pub status: ClearChargingProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageRequest {
    pub id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageResponse {
    pub status: ClearMessageStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringRequest {
    pub id: Vec<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringResponse {
    pub clear_monitoring_result: Vec<ClearMonitoringResultType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitRequest {
    pub charging_limit_source: ChargingLimitSourceEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitResponse {}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedRequest {
    pub total_cost: f64,

    #[validate(length(min = 0, max = 36))]
    pub transaction_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationRequest {
    pub request_id: i64,

    pub report: bool,

    pub clear: bool,

    #[validate(length(min = 0, max = 64))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_certificate: Option<CertificateHashDataType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationResponse {
    pub status: CustomerInformationStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    #[validate(length(min = 0, max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    pub data: String,

    #[validate(length(min = 0, max = 255))]
    pub vendor_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    pub status: DataTransferStatusEnumType,

    pub data: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateRequest {
    pub certificate_hash_data: CertificateHashDataType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateResponse {
    pub status: DeleteCertificateStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateRequest {
    #[validate(length(min = 0, max = 50))]
    #[serde(rename = "iso15118SchemaVersion")]
    pub iso_15118_schema_version: String,

    pub action: CertificateActionEnumType,

    #[validate(length(min = 0, max = 5600))]
    pub exi_request: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateResponse {
    pub status: Iso15118EVCertificateStatusEnumType,

    pub exi_response: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportRequest {
    pub request_id: i64,

    pub report_base: ReportBaseEnumType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportResponse {
    pub status: GenericDeviceModelStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusRequest {
    pub ocsp_request_data: OCSPRequestDataType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusResponse {
    pub status: GetCertificateStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_result: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesRequest {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,

    pub charging_profile: ChargingProfileCriterionType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesResponse {
    pub status: GetChargingProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    pub duration: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,

    pub evse_id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CompositeScheduleType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<i64>>,

    pub request_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<MessagePriorityEnumType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesResponse {
    pub status: GetDisplayMessagesStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<Vec<GetCertificateIdUseEnumType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsResponse {
    pub status: GetDisplayMessagesStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data_chain: Option<Vec<CertificateHashDataChainType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    pub version_number: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRequest {
    pub log_type: LogEnumType,

    pub request_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,

    pub log: LogParametersType,
}

/// GetLogResponse, sent by the Charging Station to the CSMS in response to a GetLogRequest.
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLogResponse {
    pub status: LogStatusEnumType,

    #[validate(length(min = 0, max = 255))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportRequest {
    pub request_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_criteria: Option<Vec<MonitoringCriterionEnumType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<Vec<ComponentVariableType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportResponse {
    pub status: GenericDeviceModelStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
    pub request_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 4))]
    pub component_criteria: Option<Vec<ComponentCriterionEnumType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<Vec<ComponentVariableType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetReportResponse {
    pub status: GenericDeviceModelStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_indicator: Option<bool>,

    pub messages_in_queue: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest {
    pub get_variable_data: Vec<GetVariableDataType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesResponse {
    pub get_variable_result: Vec<GetVariableResultType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    pub current_time: DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateRequest {
    pub certificate_type: InstallCertificateUseEnumType,

    #[validate(length(min = 0, max = 5500))]
    pub certificate: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateResponse {
    pub status: InstallCertificateStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationRequest {
    pub status: UploadLogStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    pub evse_id: i64,
    pub meter_value: Vec<MeterValueType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,

    pub charging_limit: ChargingLimitType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<Vec<ChargingScheduleType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationRequest {
    pub data: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    pub seq_no: i64,

    pub generated_at: DateTime<Utc>,

    pub request_id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesRequest {
    pub request_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_info: Option<Vec<MessageInfoType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_schedule_tuples: Option<i64>,

    pub evse_id: i64,

    pub charging_needs: ChargingNeedsType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsResponse {
    pub status: NotifyEVChargingNeedsStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleRequest {
    pub time_base: DateTime<Utc>,

    pub evse_id: i64,

    pub charging_schedule: ChargingScheduleType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventRequest {
    pub generated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    pub seq_no: i64,

    pub event_data: Vec<EventDataType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportRequest {
    pub request_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    pub seq_no: i64,

    pub generated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<Vec<MonitoringDataType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportRequest {
    pub request_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    pub seq_no: i64,

    pub generated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_data: Option<Vec<ReportDataType>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareRequest {
    pub location: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,

    pub checksum: String,

    pub request_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationRequest {
    pub status: PublishFirmwareStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesRequest {
    pub request_id: i64,

    pub charging_limit_source: ChargingLimitSourceEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    pub evse_id: i64,

    pub charging_profile: Vec<ChargingProfileType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,

    pub remote_start_id: i64,

    pub id_token: IdTokenType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfileType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionResponse {
    pub status: RequestStartStopStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionRequest {
    pub transaction_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionResponse {
    pub status: RequestStartStopStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateRequest {
    pub reservation_id: i64,

    pub reservation_update_status: ReservationUpdateStatusEnumType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowRequest {
    pub id: i64,

    pub expiry_date_time: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<ConnectorEnumType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,

    pub id_token: IdTokenType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowResponse {
    pub status: ReserveNowStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    #[serde(rename = "type")]
    pub request_type: ResetEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetResponse {
    pub status: ResetStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationRequest {
    #[serde(rename = "type")]
    pub kind: String,

    pub timestamp: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListRequest {
    pub version_number: i64,

    pub update_type: UpdateEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListResponse {
    pub status: SendLocalListStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    pub evse_id: i64,

    pub charging_profile: ChargingProfileType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileResponse {
    pub status: ChargingProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageRequest {
    pub message: MessageInfoType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageResponse {
    pub status: DisplayMessageStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseRequest {
    pub monitoring_base: MonitoringBaseEnumType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseResponse {
    pub status: GenericDeviceModelStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelRequest {
    pub severity: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileRequest {
    pub configuration_slot: i64,

    pub connection_data: NetworkConnectionProfileType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileResponse {
    pub status: SetNetworkProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    pub set_monitoring_data: Vec<SetMonitoringDataType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringResponse {
    pub set_monitoring_result: Vec<SetMonitoringResultType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest {
    pub set_variable_data: Vec<SetVariableDataType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesResponse {
    pub set_variable_result: Vec<SetVariableResultType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateRequest {
    pub csr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    pub timestamp: DateTime<Utc>,

    pub connector_status: ConnectorStatusEnumType,

    pub evse_id: i64,

    pub connector_id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationResponse {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventRequest {
    pub event_type: TransactionEventEnumType,
    pub timestamp: DateTime<Utc>,
    pub trigger_reason: TriggerReasonEnumType,
    pub seq_no: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_phases_used: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cable_max_current: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i64>,

    pub transaction_info: TransactionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_value: Option<Vec<MeterValueType>>,
}

/// This contains the field definition of the TransactionEventResponse PDU sent by the CSMS to the Charging Station in response to a TransactionEventRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message: Option<MessageContentType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageRequest {
    pub requested_message: MessageTriggerEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageResponse {
    pub status: TriggerMessageStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorRequest {
    pub evse_id: i64,

    pub connector_id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorResponse {
    pub status: UnlockStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareRequest {
    pub checksum: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareResponse {
    pub status: UnpublishFirmwareStatusEnumType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,

    pub request_id: i64,

    pub firmware: FirmwareType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareResponse {
    pub status: UpdateFirmwareStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
