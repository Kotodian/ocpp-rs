#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum APNAuthenticationEnumType {
    CHAP,
    NONE,
    PAP,
    AUTO,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AttributeEnumType {
    Actual,
    Target,
    MinSet,
    MaxSet,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AuthorizationStatusEnumType {
    Accepeted,
    Blocked,
    ConcurrentTx,
    Expired,
    Invalid,
    NoCredit,
    NotAllowedTypeEVSE,
    NotAtThisLocation,
    NotAtThisTime,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AuthorizeCertificateStatusEnumType {
    Accepted,
    SignatureError,
    CertificateExpired,
    NoCertificateAvailable,
    CertChainError,
    CertificateRevoked,
    ContractCanceled,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BootReasonEnumType {
    ApplicationReset,
    FirmwareUpdate,
    LocalReset,
    PowerUp,
    RemoteReset,
    ScheduledReset,
    Triggered,
    Unknown,
    Watchdog,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CancelReservationStatusEnumType {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CertificateActionEnumType {
    Install,
    Update,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CertificateSignedStatusEnumType {
    Accpeted,
    Rejected,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum CertificateSigningUseEnumType {
    ChargingStationCertificate,
    V2GCertificate,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChangeAvailabilityStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargingLimitSourceEnumType {
    EMS,
    Other,
    SO,
    CSO,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargingProfileKindEnumType {
    Absolute,
    Recurring,
    Relative,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargingProfilePurposeEnumType {
    ChargingStationExternalConstraints,
    ChargingStationMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargingProfileStatusEnumType {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargingRateUnitEnumType {
    W,
    A,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargingStateEnumType {
    Charging,
    EVConnected,
    SuspendedEV,
    SuspendedEVSE,
    Idle,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClearCacheStatusEnumType {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClearChargingProfileStatusEnumType {
    Accepted,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClearMessageStatusEnumType {
    Accepted,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClearMonitoringStatusEnumType {
    Accepted,
    Rejected,
    NotFound,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ComponentCriterionEnumType {
    Active,
    Available,
    Enable,
    Problem,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ConnectorEnumType {
    #[serde(rename = "cCCS1")]
    CCCS1,
    #[serde(rename = "cCCS2")]
    CCCS2,
    #[serde(rename = "cG105")]
    CG105,
    #[serde(rename = "cTesla")]
    CTesla,
    #[serde(rename = "cType1")]
    CType1,
    #[serde(rename = "cType2")]
    CType2,
    #[serde(rename = "s309-1P-16A")]
    S3091P16A,
    #[serde(rename = "s309-1P-32A")]
    S3091P32A,
    #[serde(rename = "s309-3P-16A")]
    S3093P16A,
    #[serde(rename = "s309-3P-32A")]
    S3093P32A,
    SBS1361,
    #[serde(rename = "sCEE-7-7")]
    SCEE77,
    #[serde(rename = "sType2")]
    SType2,
    #[serde(rename = "sType2")]
    SType3,
    Other1PhMax16A,
    Other1PhOver16A,
    Other3Ph,
    Pan,
    #[serde(rename = "wInductive")]
    WInductive,
    #[serde(rename = "wResonant")]
    WResonant,
    Undetermined,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ConnectorStatusEnumType {
    Available,
    Occuiped,
    Reserved,
    Unavailable,
    Faulted,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum CostKindEnumType {
    CarbonDioxideEmission,
    RelativePricePercentage,
    RenewableGenerationPercentage,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum CustomerInformationStatusEnumType {
    Accepted,
    Rejected,
    Invalid,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum DataEnumType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "dateTime")]
    DateTime,
    #[serde(rename = "boolean")]
    Boolean,
    OptionList,
    SequenceList,
    MemberList,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum DataTransferStatusEnumType {
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum DeleteCertificateStatusEnumType {
    Accepted,
    Failed,
    NotFound,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum DisplayMessageStatusEnumType {
    Accepted,
    NotSupportedMessageFormat,
    Rejected,
    NotSupportedPriority,
    NotSupportedState,
    UnknownTransaction,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
pub enum EnergyTransferModeEnumType {
    DC,
    #[serde(rename = "AC_single_phase")]
    ACSinglePhase,
    #[serde(rename = "AC_two_phase")]
    ACTwoPhase,
    #[serde(rename = "AC_three_phase")]
    ACThreePhase,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum EventNotificationEnumType {
    HardWiredNotification,
    HardWiredMonitor,
    PreconfiguredMonitor,
    CustomMonitor,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum EventTriggerEnumType {
    Alerting,
    Delta,
    Periodic,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum FirmwareStatusEnumType {
    Downloaded,
    DownloadFailed,
    Downloading,
    DownloadScheduled,
    DownloadPaused,
    Idle,
    InstallationFailed,
    Installing,
    Installed,
    InstallRebooting,
    InstallScheduled,
    InstallVerificationFailed,
    InvalidSignature,
    SignatureVerified,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GenericDeviceModelStatusEnumType {
    Accepted,
    Rejected,
    NotSupported,
    EmptyResultSet,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GenericStatusEnumType {
    Accepted,
    Rejected,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GetCertificateStatusEnumType {
    Accepted,
    Failed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GetCertificateIdUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    CSMSRootCertificate,
    V2GCertificateChain,
    ManufacturerRootCertificate,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GetChargingProfileStatusEnumType {
    Accepted,
    NoProfiles,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GetDisplayMessagesStatusEnumType {
    Accepted,
    Unknown,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum GetInstalledCertificateStatusEnumType {
    Accepted,
    NotFound,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GetVariableStatusEnumType {
    Accepted,
    Rejected,
    UnknownComponent,
    UnknownVariable,
    NotSupportedAttributeType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum HashAlgorithmEnumType {
    SHA256,
    SHA384,
    SHA512,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum IdTokenEnumType {
    Central,
    #[serde(rename = "eMAID")]
    EMAID,
    ISO14443,
    ISO15693,
    KeyCode,
    Local,
    MacAddress,
    NoAuthorization,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum InstallCertificateStatusEnumType {
    Accepted,
    Rejected,
    Failed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum InstallCertificateUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    CSMSRootCertificate,
    ManufacturerRootCertificate,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Iso15118EVCertificateStatusEnumType {
    Accepted,
    Failed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum LocationEnumType {
    Body,
    Cable,
    EV,
    Inlet,
    Outlet,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum LogEnumType {
    DiagnosticsLog,
    SecurityLog,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum LogStatusEnumType {
    Accepted,
    Rejected,
    AcceptedCanceled,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum MeasurandEnumType {
    #[serde(rename = "Current.Export")]
    CurrentExport,
    #[serde(rename = "Current.Import")]
    CurrentImport,
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    #[serde(rename = "Energy.Active.Net")]
    EnergyActiveNet,
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    #[serde(rename = "Energy.Reactive.Net")]
    EnergyReactiveNet,
    #[serde(rename = "Energy.Apparent.Net")]
    EnergyApparentNet,
    #[serde(rename = "Energy.Apparent.Import")]
    EnergyApparentImport,
    #[serde(rename = "Energy.Apparent.Export")]
    EnergyApparentExport,
    #[serde(rename = "Frequency")]
    Frequency,
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    #[serde(rename = "SoC")]
    SoC,
    #[serde(rename = "Voltage")]
    Voltage,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MessageFormatEnumType {
    ASCII,
    HTML,
    URI,
    UTF8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MessagePriorityEnumType {
    AlwaysFront,
    InFront,
    NormalCycle,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MessageStateEnumType {
    Charging,
    Faulted,
    Idle,
    Unavailable,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MessageTriggerEnumType {
    BootNotification,
    LogStatusNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    SignChargingStationCertificate,
    SignV2GCertificate,
    StatusNotification,
    TransactionEvent,
    SignCombinedCertificate,
    PublishFirmwareStatusNotification,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MonitorEnumType {
    UpperThreshold,
    LowerThreshold,
    DeltaPeriodic,
    PeriodicClockAligned,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MonitoringBaseEnumType {
    All,
    FactoryDefault,
    HardWiredOnly,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MonitoringCriterionEnumType {
    ThresholdMonitoring,
    DeltaMonitoring,
    PeriodicMonitoring,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MutabilityEnumType {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum NotifyEVChargingNeedsStatusEnumType {
    Accepted,
    Rejected,
    Processing,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum OCPPInterfaceEnumType {
    Wired0,
    Wired1,
    Wired2,
    Wired3,
    Wireless0,
    Wireless1,
    Wireless2,
    Wireless3,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum OCPPTransportEnumType {
    JSON,
    SOAP,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum OCPPVersionEnumType {
    OCPP12,
    OCPP15,
    OCPP16,
    OCPP20,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum OperationalStatusEnumType {
    Inoperative,
    Operative,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum PhaseEnumType {
    L1,
    L2,
    L3,
    N,
    #[serde(rename = "L1-N")]
    L1N,
    #[serde(rename = "L2-N")]
    L2N,
    #[serde(rename = "L3-N")]
    L3N,
    #[serde(rename = "L1-L2")]
    L1L2,
    #[serde(rename = "L2-L3")]
    L2L3,
    #[serde(rename = "L3-L1")]
    L3L1,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum PublishFirmwareStatusEnumType {
    Idle,
    DownloadScheduled,
    Downloading,
    Downloaded,
    Published,
    DownloadFailed,
    DownloadPaused,
    InvalidChecksum,
    ChecksumVerified,
    PublishFailed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ReadingContextEnumType {
    #[serde(rename = "Interruption.Begin")]
    InterruptionBegin,
    #[serde(rename = "Interruption.End")]
    InterruptionEnd,
    Other,
    #[serde(rename = "Sample.Clock")]
    SampleClock,
    #[serde(rename = "Sample.Periodic")]
    SamplePeriodic,
    #[serde(rename = "Transaction.Begin")]
    TransactionBegin,
    #[serde(rename = "Transaction.End")]
    TransactionEnd,
    Trigger,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ReasonEnumType {
    DeAuthorized,
    EmergencyStop,
    EnergyLimitReached,
    EVDisconnected,
    GroundFault,
    ImmediateReset,
    Local,
    LocalOutOfCredit,
    MasterPass,
    Other,
    OvercurrentFault,
    PowerLoss,
    PowerQuality,
    Reboot,
    Remote,
    SOCLimitReached,
    StoppedByEV,
    TimeLimitReached,
    Timeout,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum RecurrencyKindEnumType {
    Daily,
    Weekly,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum RegistrationStatusEnumType {
    Accepted,
    Pending,
    Rejected,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ReportBaseEnumType {
    ConfigurationInventory,
    FullInventory,
    SummaryInventory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum RequestStartStopStatusEnumType {
    Accepted,
    Rejected,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ReservationUpdateStatusEnumType {
    Expired,
    Removed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ReserveNowStatusEnumType {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ResetEnumType {
    Immediate,
    OnIdle,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ResetStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SendLocalListStatusEnumType {
    Accepted,
    Failed,
    VersionMismatch,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SetMonitoringStatusEnumType {
    Accepted,
    UnknownComponent,
    UnknownVariable,
    UnsupportedMonitorType,
    Rejected,
    Duplicate,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SetNetworkProfileStatusEnumType {
    Accepted,
    Rejected,
    Failed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SetVariableStatusEnumType {
    Accepted,
    Rejected,
    UnknownComponent,
    UnknownVariable,
    NotSupportedAttributeType,
    RebootRequired,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum TransactionEventEnumType {
    Ended,
    Started,
    Updated,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum TriggerMessageStatusEnumType {
    Accepted,
    Rejected,
    NotImplemented,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum TriggerReasonEnumType {
    Authorized,
    CablePluggedIn,
    ChargingRateChanged,
    ChargingStateChanged,
    Deauthorized,
    EnergyLimitReached,
    EVCommunicationLost,
    EVConnectTimeout,
    MeterValueClock,
    MeterValuePeriodic,
    TimeLimitReached,
    Trigger,
    UnlockCommand,
    StopAuthorized,
    EVDeparted,
    EVDetected,
    RemoteStop,
    RemoteStart,
    AbnormalCondition,
    SignedDataReceived,
    ResetCommand,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UnlockStatusEnumType {
    Unlocked,
    UnlockFailed,
    OngoingAuthorizedTransaction,
    UnknownConnector,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UnpublishFirmwareStatusEnumType {
    DownloadOngoing,
    NoFirmware,
    Unpublished,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UpdateEnumType {
    Differential,
    Full,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UpdateFirmwareStatusEnumType {
    Accepted,
    Rejected,
    AcceptedCanceled,
    InvalidCertificate,
    RevokedCertificate,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UploadLogStatusEnumType {
    BadMessage,
    Idle,
    NotSupportedOperation,
    PermissionDenied,
    Uploaded,
    UploadFailure,
    Uploading,
    AcceptedCanceled,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum VPNEnumType {
    IKEv2,
    IPSec,
    L2TP,
    PPTP,
}
