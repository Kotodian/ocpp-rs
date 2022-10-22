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

feature!(AuthorizeFeature, AuthorizeRequest, AuthorizeResponse);
feature!(
    BootNotificationFeature,
    BootNotificationRequest,
    BootNotificationResponse
);
feature!(
    CancelReservationFeature,
    CancelReservationRequest,
    CancelReservationResponse
);
feature!(
    CertificateSignedFeature,
    CertificateSignedRequest,
    CertificateSignedResponse
);
feature!(
    ChangeAvailabilityFeature,
    ChangeAvailabilityRequest,
    ChangeAvailabilityResponse
);
feature!(ClearCacheFeature, ClearCacheRequest, ClearCacheResponse);
feature!(
    ClearChargingProfileFeature,
    ClearChargingProfileRequest,
    ClearChargingProfileResponse
);
feature!(
    ClearDisplayMessageFeature,
    ClearDisplayMessageRequest,
    ClearDisplayMessageResponse
);
feature!(
    ClearedChargingLimitFeature,
    ClearedChargingLimitRequest,
    ClearedChargingLimitResponse
);
feature!(
    ClearVariableMonitoringFeature,
    ClearVariableMonitoringRequest,
    ClearVariableMonitoringResponse
);
feature!(CostUpdatedFeature, CostUpdatedRequest, CostUpdatedResponse);
feature!(
    CustomerInformationFeature,
    CustomerInformationRequest,
    CustomerInformationResponse
);
feature!(
    DataTransferFeature,
    DataTransferRequest,
    DataTransferResponse
);
feature!(
    DeleteCertificateFeature,
    DeleteCertificateRequest,
    DeleteCertificateResponse
);
feature!(
    FirmwareStatusNotificationFeature,
    FirmwareStatusNotificationRequest,
    FirmwareStatusNotificationResponse
);
feature!(
    Get15118EVCertificateFeature,
    Get15118EVCertificateRequest,
    Get15118EVCertificateResponse
);
feature!(
    GetBaseReportFeature,
    GetBaseReportRequest,
    GetBaseReportResponse
);
feature!(
    GetCertificateStatusFeature,
    GetCertificateStatusRequest,
    GetCertificateStatusResponse
);
feature!(
    GetChargingProfilesFeature,
    GetChargingProfilesRequest,
    GetChargingProfilesResponse
);
feature!(
    GetCompositeScheduleFeature,
    GetCompositeScheduleRequest,
    GetCompositeScheduleResponse
);
feature!(
    GetDisplayMessagesFeature,
    GetDisplayMessagesRequest,
    GetDisplayMessagesResponse
);
feature!(
    GetInstalledCertificateIdsFeature,
    GetInstalledCertificateIdsRequest,
    GetInstalledCertificateIdsResponse
);
feature!(GetLogFeature, GetLogRequest, GetLogResponse);
feature!(
    GetLocalListVersionFeature,
    GetLocalListVersionRequest,
    GetLocalListVersionResponse
);
feature!(
    GetMonitoringReportFeature,
    GetMonitoringReportRequest,
    GetMonitoringReportResponse
);
feature!(GetReportFeature, GetReportRequest, GetReportResponse);
feature!(
    GetTransactionStatusFeature,
    GetTransactionStatusRequest,
    GetTransactionStatusResponse
);
feature!(
    GetVariablesFeature,
    GetVariablesRequest,
    GetVariablesResponse
);

feature!(HeartbeatFeature, HeartbeatRequest, HeartbeatResponse);
feature!(
    InstallCertificateFeature,
    InstallCertificateRequest,
    InstallCertificateResponse
);
feature!(
    LogStatusNotificationFeature,
    LogStatusNotificationRequest,
    LogStatusNotificationResponse
);
feature!(MeterValuesFeature, MeterValuesRequest, MeterValuesResponse);
feature!(
    NotifyChargingLimitFeature,
    NotifyChargingLimitRequest,
    NotifyChargingLimitResponse
);
feature!(
    NotifyCustomerInformationFeature,
    NotifyCustomerInformationRequest,
    NotifyCustomerInformationResponse
);
feature!(
    NotifyDisplayMessagesFeature,
    NotifyDisplayMessagesRequest,
    NotifyDisplayMessagesResponse
);

feature!(
    NotifyEVChargingNeedsFeature,
    NotifyEVChargingNeedsRequest,
    NotifyEVChargingNeedsResponse
);
feature!(
    NotifyEVChargingScheduleFeature,
    NotifyEVChargingScheduleRequest,
    NotifyEVChargingScheduleResponse
);
feature!(NotifyEventFeature, NotifyEventRequest, NotifyEventResponse);
feature!(
    NotifyMonitoringReportFeature,
    NotifyMonitoringReportRequest,
    NotifyMonitoringReportResponse
);
feature!(
    NotifyReportFeature,
    NotifyReportRequest,
    NotifyReportResponse
);

feature!(
    PublishFirmwareFeature,
    PublishFirmwareRequest,
    PublishFirmwareResponse
);
feature!(
    PublishFirmwareStatusNotificationFeature,
    PublishFirmwareStatusNotificationRequest,
    PublishFirmwareStatusNotificationResponse
);
feature!(
    ReportChargingProfilesFeature,
    ReportChargingProfilesRequest,
    ReportChargingProfilesResponse
);
feature!(
    RequestStartTransactionFeature,
    RequestStartTransactionRequest,
    RequestStartTransactionResponse
);
feature!(
    RequestStopTransactionFeature,
    RequestStopTransactionRequest,
    RequestStopTransactionResponse
);
feature!(
    ReservationStatusUpdateFeature,
    ReservationStatusUpdateRequest,
    ReservationStatusUpdateResponse
);
feature!(ReserveNowFeature, ReserveNowRequest, ReserveNowResponse);
feature!(ResetFeature, ResetRequest, ResetResponse);
feature!(
    SecurityEventNotificationFeature,
    SecurityEventNotificationRequest,
    SecurityEventNotificationResponse
);
feature!(
    SendLocalListFeature,
    SendLocalListRequest,
    SendLocalListResponse
);
feature!(
    SetChargingProfileFeature,
    SetChargingProfileRequest,
    SetChargingProfileResponse
);
feature!(
    SetDisplayMessageFeature,
    SetDisplayMessageRequest,
    SetDisplayMessageResponse
);
feature!(
    SetMonitoringBaseFeature,
    SetMonitoringBaseRequest,
    SetMonitoringBaseResponse
);
feature!(
    SetMonitoringLevelFeature,
    SetMonitoringLevelRequest,
    SetMonitoringLevelResponse
);
feature!(
    SetNetworkProfileFeature,
    SetNetworkProfileRequest,
    SetNetworkProfileResponse
);
feature!(
    SetVariableMonitoringFeature,
    SetVariableMonitoringRequest,
    SetVariableMonitoringResponse
);
feature!(
    SetVariablesFeature,
    SetVariablesRequest,
    SetVariablesResponse
);
feature!(
    SignCertificateFeature,
    SignCertificateRequest,
    SignCertificateResponse
);
feature!(
    StatusNotificationFeature,
    StatusNotificationRequest,
    StatusNotificationResponse
);
feature!(
    TransactionEventFeature,
    TransactionEventRequest,
    TransactionEventResponse
);
feature!(
    TriggerMessageFeature,
    TriggerMessageRequest,
    TriggerMessageResponse
);
feature!(
    UnlockConnectorFeature,
    UnlockConnectorRequest,
    UnlockConnectorResponse
);
feature!(
    UnpublishFirmwareFeature,
    UnpublishFirmwareRequest,
    UnpublishFirmwareResponse
);
feature!(
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
    GetChargingProfile(GetChargingProfilesFeature),
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
