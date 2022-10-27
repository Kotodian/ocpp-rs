use std::{fmt::Display, str::FromStr};

use ocpp_json::v2_0_1::payloads::{
    AuthorizeRequest, AuthorizeResponse, BootNotificationRequest, BootNotificationResponse,
    CancelReservationRequest, CancelReservationResponse, CertificateSignedRequest,
    CertificateSignedResponse, ChangeAvailabilityRequest, ChangeAvailabilityResponse,
    ClearCacheRequest, ClearCacheResponse, ClearChargingProfileRequest,
    ClearChargingProfileResponse, ClearDisplayMessageRequest, ClearDisplayMessageResponse,
    ClearVariableMonitoringRequest, ClearVariableMonitoringResponse, ClearedChargingLimitRequest,
    ClearedChargingLimitResponse, CostUpdatedRequest, CostUpdatedResponse,
    CustomerInformationRequest, CustomerInformationResponse, DataTransferRequest,
    DataTransferResponse, DeleteCertificateRequest, DeleteCertificateResponse,
    FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
    Get15118EVCertificateRequest, Get15118EVCertificateResponse, GetBaseReportRequest,
    GetBaseReportResponse, GetCertificateStatusRequest, GetCertificateStatusResponse,
    GetChargingProfilesRequest, GetChargingProfilesResponse, GetCompositeScheduleRequest,
    GetCompositeScheduleResponse, GetDisplayMessagesRequest, GetDisplayMessagesResponse,
    GetInstalledCertificateIdsRequest, GetInstalledCertificateIdsResponse,
    GetLocalListVersionRequest, GetLocalListVersionResponse, GetLogRequest, GetLogResponse,
    GetMonitoringReportRequest, GetMonitoringReportResponse, GetReportRequest, GetReportResponse,
    GetTransactionStatusRequest, GetTransactionStatusResponse, GetVariablesRequest,
    GetVariablesResponse, HeartbeatRequest, HeartbeatResponse, InstallCertificateRequest,
    InstallCertificateResponse, LogStatusNotificationRequest, LogStatusNotificationResponse,
    MeterValuesRequest, MeterValuesResponse, NotifyChargingLimitRequest,
    NotifyChargingLimitResponse, NotifyCustomerInformationRequest,
    NotifyCustomerInformationResponse, NotifyDisplayMessagesRequest, NotifyDisplayMessagesResponse,
    NotifyEVChargingNeedsRequest, NotifyEVChargingNeedsResponse, NotifyEVChargingScheduleRequest,
    NotifyEVChargingScheduleResponse, NotifyEventRequest, NotifyEventResponse,
    NotifyMonitoringReportRequest, NotifyMonitoringReportResponse, NotifyReportRequest,
    NotifyReportResponse, PublishFirmwareRequest, PublishFirmwareResponse,
    PublishFirmwareStatusNotificationRequest, PublishFirmwareStatusNotificationResponse,
    ReportChargingProfilesRequest, ReportChargingProfilesResponse, RequestStartTransactionRequest,
    RequestStartTransactionResponse, RequestStopTransactionRequest, RequestStopTransactionResponse,
    ReservationStatusUpdateRequest, ReservationStatusUpdateResponse, ReserveNowRequest,
    ReserveNowResponse, ResetRequest, ResetResponse, SecurityEventNotificationRequest,
    SecurityEventNotificationResponse, SendLocalListRequest, SendLocalListResponse,
    SetChargingProfileRequest, SetChargingProfileResponse, SetDisplayMessageRequest,
    SetDisplayMessageResponse, SetMonitoringBaseRequest, SetMonitoringBaseResponse,
    SetMonitoringLevelRequest, SetMonitoringLevelResponse, SetNetworkProfileRequest,
    SetNetworkProfileResponse, SetVariableMonitoringRequest, SetVariableMonitoringResponse,
    SetVariablesRequest, SetVariablesResponse, SignCertificateRequest, SignCertificateResponse,
    StatusNotificationRequest, StatusNotificationResponse, TransactionEventRequest,
    TransactionEventResponse, TriggerMessageRequest, TriggerMessageResponse,
    UnlockConnectorRequest, UnlockConnectorResponse, UnpublishFirmwareRequest,
    UnpublishFirmwareResponse, UpdateFirmwareRequest, UpdateFirmwareResponse,
};
use serde_json::Value;
use strum_macros::Display;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MessageType {
    /// OCPP Call
    Call(usize, String, String, Value),
    /// OCPP Result
    CallResult(usize, String, Value),
    /// OCPP Error
    CallError(usize, String, String, String, Value),
}

use crate::feature;

feature!(
    Authorize,
    AuthorizeFeature,
    AuthorizeRequest,
    AuthorizeResponse
);

feature!(
    BootNotification,
    BootNotificationFeature,
    BootNotificationRequest,
    BootNotificationResponse
);

feature!(
    CancelReservation,
    CancelReservationFeature,
    CancelReservationRequest,
    CancelReservationResponse
);

feature!(
    CertificateSigned,
    CertificateSignedFeature,
    CertificateSignedRequest,
    CertificateSignedResponse
);

feature!(
    ChangeAvailability,
    ChangeAvailabilityFeature,
    ChangeAvailabilityRequest,
    ChangeAvailabilityResponse
);

feature!(
    ClearCache,
    ClearCacheFeature,
    ClearCacheRequest,
    ClearCacheResponse
);

feature!(
    ClearChargingProfile,
    ClearChargingProfileFeature,
    ClearChargingProfileRequest,
    ClearChargingProfileResponse
);

feature!(
    ClearDisplayMessage,
    ClearDisplayMessageFeature,
    ClearDisplayMessageRequest,
    ClearDisplayMessageResponse
);

feature!(
    ClearedChargingLimit,
    ClearedChargingLimitFeature,
    ClearedChargingLimitRequest,
    ClearedChargingLimitResponse
);

feature!(
    ClearVariableMonitoring,
    ClearVariableMonitoringFeature,
    ClearVariableMonitoringRequest,
    ClearVariableMonitoringResponse
);
feature!(
    CostUpdated,
    CostUpdatedFeature,
    CostUpdatedRequest,
    CostUpdatedResponse
);
feature!(
    CustomerInformation,
    CustomerInformationFeature,
    CustomerInformationRequest,
    CustomerInformationResponse
);

feature!(
    DataTransfer,
    DataTransferFeature,
    DataTransferRequest,
    DataTransferResponse
);

feature!(
    DeleteCertificate,
    DeleteCertificateFeature,
    DeleteCertificateRequest,
    DeleteCertificateResponse
);
feature!(
    FirmwareStatusNotification,
    FirmwareStatusNotificationFeature,
    FirmwareStatusNotificationRequest,
    FirmwareStatusNotificationResponse
);

feature!(
    Get15118EVCertificate,
    Get15118EVCertificateFeature,
    Get15118EVCertificateRequest,
    Get15118EVCertificateResponse
);

feature!(
    GetBaseReport,
    GetBaseReportFeature,
    GetBaseReportRequest,
    GetBaseReportResponse
);

feature!(
    GetCertificateStatus,
    GetCertificateStatusFeature,
    GetCertificateStatusRequest,
    GetCertificateStatusResponse
);
feature!(
    GetChargingProfiles,
    GetChargingProfilesFeature,
    GetChargingProfilesRequest,
    GetChargingProfilesResponse
);
feature!(
    GetCompositeSchedule,
    GetCompositeScheduleFeature,
    GetCompositeScheduleRequest,
    GetCompositeScheduleResponse
);
feature!(
    GetDisplayMessage,
    GetDisplayMessagesFeature,
    GetDisplayMessagesRequest,
    GetDisplayMessagesResponse
);
feature!(
    GetInstalledCertificateIds,
    GetInstalledCertificateIdsFeature,
    GetInstalledCertificateIdsRequest,
    GetInstalledCertificateIdsResponse
);
feature!(GetLog, GetLogFeature, GetLogRequest, GetLogResponse);

feature!(
    GetLocalListVersion,
    GetLocalListVersionFeature,
    GetLocalListVersionRequest,
    GetLocalListVersionResponse
);
feature!(
    GetMonitoringReport,
    GetMonitoringReportFeature,
    GetMonitoringReportRequest,
    GetMonitoringReportResponse
);
feature!(
    GetReport,
    GetReportFeature,
    GetReportRequest,
    GetReportResponse
);
feature!(
    GetTransactionStatus,
    GetTransactionStatusFeature,
    GetTransactionStatusRequest,
    GetTransactionStatusResponse
);
feature!(
    GetVariables,
    GetVariablesFeature,
    GetVariablesRequest,
    GetVariablesResponse
);

feature!(
    Heartbeat,
    HeartbeatFeature,
    HeartbeatRequest,
    HeartbeatResponse
);

feature!(
    InstallCertificate,
    InstallCertificateFeature,
    InstallCertificateRequest,
    InstallCertificateResponse
);
feature!(
    LogStatusNotification,
    LogStatusNotificationFeature,
    LogStatusNotificationRequest,
    LogStatusNotificationResponse
);
feature!(
    MeterValues,
    MeterValuesFeature,
    MeterValuesRequest,
    MeterValuesResponse
);

feature!(
    NotifyChargingLimit,
    NotifyChargingLimitFeature,
    NotifyChargingLimitRequest,
    NotifyChargingLimitResponse
);

feature!(
    NotifyCustomerInformation,
    NotifyCustomerInformationFeature,
    NotifyCustomerInformationRequest,
    NotifyCustomerInformationResponse
);

feature!(
    NotifyDisplayMessages,
    NotifyDisplayMessagesFeature,
    NotifyDisplayMessagesRequest,
    NotifyDisplayMessagesResponse
);

feature!(
    NotifyEVChargingNeeds,
    NotifyEVChargingNeedsFeature,
    NotifyEVChargingNeedsRequest,
    NotifyEVChargingNeedsResponse
);
feature!(
    NotifyEVChargingSchedule,
    NotifyEVChargingScheduleFeature,
    NotifyEVChargingScheduleRequest,
    NotifyEVChargingScheduleResponse
);

feature!(
    NotifyEvent,
    NotifyEventFeature,
    NotifyEventRequest,
    NotifyEventResponse
);

feature!(
    NotifyMonitoringReport,
    NotifyMonitoringReportFeature,
    NotifyMonitoringReportRequest,
    NotifyMonitoringReportResponse
);

feature!(
    NotifyReport,
    NotifyReportFeature,
    NotifyReportRequest,
    NotifyReportResponse
);

feature!(
    PublishFirmware,
    PublishFirmwareFeature,
    PublishFirmwareRequest,
    PublishFirmwareResponse
);

feature!(
    PublishFirmwareStatusNotification,
    PublishFirmwareStatusNotificationFeature,
    PublishFirmwareStatusNotificationRequest,
    PublishFirmwareStatusNotificationResponse
);

feature!(
    ReportChargingProfiles,
    ReportChargingProfilesFeature,
    ReportChargingProfilesRequest,
    ReportChargingProfilesResponse
);

feature!(
    RequestStartTransaction,
    RequestStartTransactionFeature,
    RequestStartTransactionRequest,
    RequestStartTransactionResponse
);

feature!(
    RequestStopTransaction,
    RequestStopTransactionFeature,
    RequestStopTransactionRequest,
    RequestStopTransactionResponse
);

feature!(
    ReservationStatusUpdate,
    ReservationStatusUpdateFeature,
    ReservationStatusUpdateRequest,
    ReservationStatusUpdateResponse
);

feature!(
    ReserveNow,
    ReserveNowFeature,
    ReserveNowRequest,
    ReserveNowResponse
);

feature!(Reset, ResetFeature, ResetRequest, ResetResponse);

feature!(
    SecurityEventNotification,
    SecurityEventNotificationFeature,
    SecurityEventNotificationRequest,
    SecurityEventNotificationResponse
);

feature!(
    SendLocalList,
    SendLocalListFeature,
    SendLocalListRequest,
    SendLocalListResponse
);

feature!(
    SetChargingProfile,
    SetChargingProfileFeature,
    SetChargingProfileRequest,
    SetChargingProfileResponse
);

feature!(
    SetDisplayMessage,
    SetDisplayMessageFeature,
    SetDisplayMessageRequest,
    SetDisplayMessageResponse
);
feature!(
    SetMonitoringBase,
    SetMonitoringBaseFeature,
    SetMonitoringBaseRequest,
    SetMonitoringBaseResponse
);

feature!(
    SetMonitoringLevel,
    SetMonitoringLevelFeature,
    SetMonitoringLevelRequest,
    SetMonitoringLevelResponse
);
feature!(
    SetNetworkProfile,
    SetNetworkProfileFeature,
    SetNetworkProfileRequest,
    SetNetworkProfileResponse
);
feature!(
    SetVariableMonitoring,
    SetVariableMonitoringFeature,
    SetVariableMonitoringRequest,
    SetVariableMonitoringResponse
);
feature!(
    SetVariables,
    SetVariablesFeature,
    SetVariablesRequest,
    SetVariablesResponse
);
feature!(
    SignCertificate,
    SignCertificateFeature,
    SignCertificateRequest,
    SignCertificateResponse
);
feature!(
    StatusNotification,
    StatusNotificationFeature,
    StatusNotificationRequest,
    StatusNotificationResponse
);
feature!(
    TransactionEvent,
    TransactionEventFeature,
    TransactionEventRequest,
    TransactionEventResponse
);
feature!(
    TriggerMessage,
    TriggerMessageFeature,
    TriggerMessageRequest,
    TriggerMessageResponse
);
feature!(
    UnlockConnector,
    UnlockConnectorFeature,
    UnlockConnectorRequest,
    UnlockConnectorResponse
);
feature!(
    UnpublishFirmware,
    UnpublishFirmwareFeature,
    UnpublishFirmwareRequest,
    UnpublishFirmwareResponse
);
feature!(
    UpdateFirmware,
    UpdateFirmwareFeature,
    UpdateFirmwareRequest,
    UpdateFirmwareResponse
);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Action {
    Authorize,
    BootNotification,
    CancelReservation,
    CertificateSigned,
    ChangeAvailability,
    ClearCache,
    ClearChargingProfile,
    ClearDisplayMessage,
    ClearedChargingLimit,
    ClearVariableMonitoring,
    CostUpdated,
    CustomerInformation,
    DataTransfer,
    DeleteCertificate,
    FirmwareStatusNotification,
    Get15118EVCertificate,
    GetBaseReport,
    GetCertificateStatus,
    GetChargingProfile,
    GetCompositeSchedule,
    GetDisplayMessage,
    GetInstalledCertificateIds,
    GetLocalListVersion,
    GetLog,
    GetMonitoringReport,
    GetReport,
    GetTransactionStatus,
    GetVariables,
    Heartbeat,
    InstallCertificate,
    LogStatusNotification,
    MeterValues,
    NotifyChargingLimit,
    NotifyCustomerInformation,
    NotifyDisplayMessages,
    NotifyEVChargingNeeds,
    NotifyEVChargingSchedule,
    NotifyEvent,
    NotifyMonitoringReport,
    NotifyReport,
    PublishFirmware,
    PublishFirmwareStatusNotification,
    ReportChargingProfiles,
    RequestStartTransaction,
    RequestStopTransaction,
    ReservationStatusUpdate,
    ReserveNow,
    Reset,
    SecurityEventNotification,
    SendLocalList,
    SetChargingProfile,
    SetDisplayMessage,
    SetMonitoringBase,
    SetMonitoringLevel,
    SetNetworkProfile,
    SetVariableMonitoring,
    SetVariables,
    SignCertificate,
    StatusNotification,
    TransactionEvent,
    TriggerMessage,
    UnlockConnector,
    UnpublishFirmware,
    UpdateFirmware,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "Authorize" => Ok(Action::Authorize),
            "BootNotification" => Ok(Action::BootNotification),
            "CancelReservation" => Ok(Action::CancelReservation),
            "CertificateSigned" => Ok(Action::CertificateSigned),
            "ChangeAvailability" => Ok(Action::ChangeAvailability),
            "ClearCache" => Ok(Action::ClearCache),
            "ClearChargingProfile" => Ok(Action::ClearChargingProfile),
            "ClearDisplayMessage" => Ok(Action::ClearDisplayMessage),
            "ClearedChargingLimit" => Ok(Action::ClearedChargingLimit),
            "ClearVariableMonitoring" => Ok(Action::ClearVariableMonitoring),
            "CostUpdated" => Ok(Action::CostUpdated),
            "CustomerInformation" => Ok(Action::CustomerInformation),
            "DataTransfer" => Ok(Action::DataTransfer),
            "DeleteCertificate" => Ok(Action::DeleteCertificate),
            "FirmwareStatusNotification" => Ok(Action::FirmwareStatusNotification),
            "Get15118EVCertificate" => Ok(Action::Get15118EVCertificate),
            "GetBaseReport" => Ok(Action::GetBaseReport),
            "GetCertificateStatus" => Ok(Action::GetCertificateStatus),
            "GetChargingProfile" => Ok(Action::GetChargingProfile),
            "GetCompositeSchedule" => Ok(Action::GetCompositeSchedule),
            "GetDisplayMessage" => Ok(Action::GetDisplayMessage),
            "GetInstalledCertificateIds" => Ok(Action::GetInstalledCertificateIds),
            "GetLocalListVersion" => Ok(Action::GetLocalListVersion),
            "GetLog" => Ok(Action::GetLog),
            "GetMonitoringReport" => Ok(Action::GetMonitoringReport),
            "GetReport" => Ok(Action::GetReport),
            "GetTransactionStatus" => Ok(Action::GetTransactionStatus),
            "GetVariables" => Ok(Action::GetVariables),
            "Heartbeat" => Ok(Action::Heartbeat),
            "InstallCertificate" => Ok(Action::InstallCertificate),
            "LogStatusNotification" => Ok(Action::LogStatusNotification),
            "MeterValues" => Ok(Action::MeterValues),
            "NotifyChargingLimit" => Ok(Action::NotifyChargingLimit),
            "NotifyCustomerInformation" => Ok(Action::NotifyCustomerInformation),
            "NotifyDisplayMessages" => Ok(Action::NotifyDisplayMessages),
            "NotifyEVChargingNeeds" => Ok(Action::NotifyEVChargingNeeds),
            "NotifyEVChargingSchedule" => Ok(Action::NotifyEVChargingSchedule),
            "NotifyEvent" => Ok(Action::NotifyEvent),
            "NotifyMonitoringReport" => Ok(Action::NotifyMonitoringReport),
            "NotifyReport" => Ok(Action::NotifyReport),
            "PublishFirmware" => Ok(Action::PublishFirmware),
            "PublishFirmwareStatusNotification" => Ok(Action::PublishFirmwareStatusNotification),
            "ReportChargingProfiles" => Ok(Action::ReportChargingProfiles),
            "RequestStartTransaction" => Ok(Action::RequestStartTransaction),
            "RequestStopTransaction" => Ok(Action::RequestStopTransaction),
            "ReservationStatusUpdate" => Ok(Action::ReservationStatusUpdate),
            "ReserveNow" => Ok(Action::ReserveNow),
            "Reset" => Ok(Action::Reset),
            "SecurityEventNotification" => Ok(Action::SecurityEventNotification),
            "SendLocalList" => Ok(Action::SendLocalList),
            "SetChargingProfile" => Ok(Action::SetChargingProfile),
            "SetDisplayMessage" => Ok(Action::SetDisplayMessage),
            "SetMonitoringBase" => Ok(Action::SetMonitoringBase),
            "SetMonitoringLevel" => Ok(Action::SetMonitoringLevel),
            "SetNetworkProfile" => Ok(Action::SetNetworkProfile),
            "SetVariableMonitoring" => Ok(Action::SetVariableMonitoring),
            "SetVariables" => Ok(Action::SetVariables),
            "SignCertificate" => Ok(Action::SignCertificate),
            "StatusNotification" => Ok(Action::StatusNotification),
            "TransactionEvent" => Ok(Action::TransactionEvent),
            "TriggerMessage" => Ok(Action::TriggerMessage),
            "UnlockConnector" => Ok(Action::UnlockConnector),
            "UnpublishFirmware" => Ok(Action::UnpublishFirmware),
            "UpdateFirmware" => Ok(Action::UpdateFirmware),
            _ => Err(()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Payload {
    Authorize(AuthorizeFeature),
    BootNotification(BootNotificationFeature),
    CancelReservation(CancelReservationFeature),
    CertificateSigned(CertificateSignedFeature),
    ChangeAvailability(ChangeAvailabilityFeature),
    ClearCache(ClearCacheFeature),
    ClearChargingProfile(ClearChargingProfileFeature),
    ClearDisplayMessage(ClearDisplayMessageFeature),
    ClearedChargingLimit(ClearedChargingLimitFeature),
    ClearVariableMonitoring(ClearVariableMonitoringFeature),
    CostUpdated(CostUpdatedFeature),
    CustomerInformation(CustomerInformationFeature),
    DataTransfer(DataTransferFeature),
    DeleteCertificate(DeleteCertificateFeature),
    FirmwareStatusNotification(FirmwareStatusNotificationFeature),
    Get15118EVCertificate(Get15118EVCertificateFeature),
    GetBaseReport(GetBaseReportFeature),
    GetCertificateStatus(GetCertificateStatusFeature),
    GetChargingProfiles(GetChargingProfilesFeature),
    GetCompositeSchedule(GetCompositeScheduleFeature),
    GetDisplayMessage(GetDisplayMessagesFeature),
    GetInstalledCertificateIds(GetInstalledCertificateIdsFeature),
    GetLocalListVersion(GetLocalListVersionFeature),
    GetLog(GetLogFeature),
    GetMonitoringReport(GetMonitoringReportFeature),
    GetReport(GetReportFeature),
    GetTransactionStatus(GetTransactionStatusFeature),
    GetVariables(GetVariablesFeature),
    Heartbeat(HeartbeatFeature),
    InstallCertificate(InstallCertificateFeature),
    LogStatusNotification(LogStatusNotificationFeature),
    MeterValues(MeterValuesFeature),
    NotifyChargingLimit(NotifyChargingLimitFeature),
    NotifyCustomerInformation(NotifyCustomerInformationFeature),
    NotifyDisplayMessages(NotifyDisplayMessagesFeature),
    NotifyEVChargingNeeds(NotifyEVChargingNeedsFeature),
    NotifyEVChargingSchedule(NotifyEVChargingScheduleFeature),
    NotifyEvent(NotifyEventFeature),
    NotifyMonitoringReport(NotifyMonitoringReportFeature),
    NotifyReport(NotifyReportFeature),
    PublishFirmware(PublishFirmwareFeature),
    PublishFirmwareStatusNotification(PublishFirmwareStatusNotificationFeature),
    ReportChargingProfiles(ReportChargingProfilesFeature),
    RequestStartTransaction(RequestStartTransactionFeature),
    RequestStopTransaction(RequestStopTransactionFeature),
    ReservationStatusUpdate(ReservationStatusUpdateFeature),
    ReserveNow(ReserveNowFeature),
    Reset(ResetFeature),
    SecurityEventNotification(SecurityEventNotificationFeature),
    SendLocalList(SendLocalListFeature),
    SetChargingProfile(SetChargingProfileFeature),
    SetDisplayMessage(SetDisplayMessageFeature),
    SetMonitoringBase(SetMonitoringBaseFeature),
    SetMonitoringLevel(SetMonitoringLevelFeature),
    SetNetworkProfile(SetNetworkProfileFeature),
    SetVariableMonitoring(SetVariableMonitoringFeature),
    SetVariables(SetVariablesFeature),
    SignCertificate(SignCertificateFeature),
    StatusNotification(StatusNotificationFeature),
    TransactionEvent(TransactionEventFeature),
    TriggerMessage(TriggerMessageFeature),
    UnlockConnector(UnlockConnectorFeature),
    UnpublishFirmware(UnpublishFirmwareFeature),
    UpdateFirmware(UpdateFirmwareFeature),
}
